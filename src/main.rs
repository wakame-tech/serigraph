pub mod decomp_cycles;
pub mod readers;
pub mod serialize;
pub mod utils;

use anyhow::Result;
use readers::obsidian::cli;

fn main() -> Result<()> {
    cli()
}
