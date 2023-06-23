use std::error::Error;

use rust::run;

fn main() -> Result<(), Box<dyn Error>> {
    let args: std::env::Args = std::env::args();
    run(args)?;
    Ok(())
}