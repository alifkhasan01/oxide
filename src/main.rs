mod cli;
mod commands;
mod core;
mod stacks;
mod config;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.execute()
}
