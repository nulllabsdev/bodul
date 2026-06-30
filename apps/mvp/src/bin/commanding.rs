//! Runs one bounded Mulac command-consumer batch.

use clap::Parser;
use kernel::io::ReservableCommandSpec;
use mvp::assembly::io::boot;
use std::error::Error;
use std::num::NonZeroUsize;

#[derive(Parser)]
#[command(about = "Run one bounded Mulac command-consumer batch")]
struct Args {
    /// How many commands to reserve and process in one run
    #[arg(default_value = "1")]
    batch_size: NonZeroUsize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let batch_size = args.batch_size.get();

    let kernel = boot()?;

    match kernel
        .command_consumer()
        .consume(&ReservableCommandSpec::new(batch_size))
    {
        Ok(processed) => {
            println!("processed {processed:?} command batch result");
            Ok(())
        }
        Err(errors) => {
            for error in &errors {
                eprintln!("command error: {error}");
            }
            Err(format!("{} command errors", errors.len()).into())
        }
    }
}
