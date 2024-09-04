mod block;
mod blockchain;
mod errors;
mod cli;
mod transaction;

use crate::errors::Result;
use crate::cli::Cli;
fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
