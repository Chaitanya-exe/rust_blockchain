mod block;
mod blockchain;
mod errors;
mod cli;

use crate::errors::Result;
use crate::cli::Cli;
fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
