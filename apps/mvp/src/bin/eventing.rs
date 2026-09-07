//! Runs one bounded Mulac event-consumer batch.

use clap::Parser;
use kernel::io::ReservableEventSpec;
use mvp::assembly::boot;
use std::error::Error;
use std::num::NonZeroUsize;

#[derive(Parser)]
#[command(about = "Run one bounded Mulac event-consumer batch")]
struct Args {
    /// How many events to reserve and process in one run
    #[arg(default_value = "1")]
    batch_size: NonZeroUsize,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let _guard = mvp::logging::init();
    let args = Args::parse();

    let batch_size = args.batch_size.get();

    let kernel = boot()?;

    match kernel.event_consumer().consume(&ReservableEventSpec::new(batch_size)) {
        Ok(processed) => {
            println!("processed {processed:?} event batch result");
            Ok(())
        }
        Err(errors) => {
            for error in &errors {
                tracing::error!("event error: {error}");
            }
            Err(format!("{} event errors", errors.len()).into())
        }
    }
}
