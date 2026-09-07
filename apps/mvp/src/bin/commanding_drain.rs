//! Drains the Mulac command queue with a harvester + worker pool.
//!
//! A single harvester thread feeds a bounded channel (capacity `batch * 2`) with work permits,
//! and a pool of worker threads each reserve-and-process one command per permit. Processing stops
//! once `max` commands have been handled or the queue is drained.
//!
//! The kernel's `CommandConsumer::consume` reserves and processes together (and doesn't expose the
//! dispatcher), so workers self-consume; reservation uses `FOR UPDATE SKIP LOCKED`, making N
//! concurrent `consume(1)` calls process N distinct commands safely.

use clap::Parser;
use kernel::io::ReservableCommandSpec;
use mvp::assembly::io::boot;
use std::error::Error;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser)]
#[command(about = "Drain the command queue with a harvester + worker pool, up to a maximum")]
struct Args {
    /// How many commands to pace per cycle; the channel capacity is `batch * 2`
    batch: NonZeroUsize,

    /// Maximum number of commands to process across all workers
    max: NonZeroUsize,

    /// Number of worker threads
    #[arg(long, default_value = "60")]
    workers: NonZeroUsize,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let _guard = mvp::logging::init();
    let args = Args::parse();

    let batch = args.batch.get();
    let max = args.max.get();
    let workers = args.workers.get();

    let kernel = boot()?;
    let consumer = kernel.command_consumer();

    // Bounded channel gives backpressure so the harvester can't outrun the workers.
    // Note: any worker that sees an empty reservation sets `drained`, which stops the
    // rest. Under FOR UPDATE SKIP LOCKED a worker can see nothing while others still
    // hold rows, so the pool can stop slightly before the queue is truly empty; the
    // next run picks up the remainder.
    let (tx, rx) = sync_channel::<()>((batch * 2).max(1));
    let rx = Arc::new(Mutex::new(rx));

    let drained = Arc::new(AtomicBool::new(false));
    let processed = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));

    println!("Starting commands (batch {batch}, max {max}, workers {workers})");

    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let consumer = consumer.clone();
        let rx = rx.clone();
        let drained = drained.clone();
        let processed = processed.clone();
        let error_count = error_count.clone();

        handles.push(thread::spawn(move || {
            loop {
                if drained.load(Ordering::Relaxed) {
                    break;
                }

                // Hold the receiver lock only for the recv itself.
                let permit = rx.lock().expect("receiver lock").recv();
                if permit.is_err() {
                    break; // channel closed by the harvester
                }

                match consumer.consume(&ReservableCommandSpec::new(1)) {
                    Ok(0) => drained.store(true, Ordering::Relaxed), // queue empty
                    Ok(count) => {
                        processed.fetch_add(count, Ordering::Relaxed);
                    }
                    Err(errors) => {
                        for error in &errors {
                            tracing::error!("command error: {error}");
                        }
                        error_count.fetch_add(1, Ordering::Relaxed);
                        processed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }));
    }

    // Harvester: feed up to `max` permits, stopping early once the queue is drained.
    for _ in 0..max {
        if drained.load(Ordering::Relaxed) {
            break;
        }
        if tx.send(()).is_err() {
            break; // all workers gone
        }
    }
    drop(tx); // close the channel so workers exit once it is empty

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }

    let processed = processed.load(Ordering::Relaxed);
    let errors = error_count.load(Ordering::Relaxed);
    println!("processed {processed} commands (batch {batch}, max {max}, workers {workers})");

    if errors > 0 {
        return Err(format!("{errors} command batches reported errors").into());
    }

    Ok(())
}
