use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct DevCommand {
    /// Port to run on
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Enable hot reload
    #[arg(long)]
    pub watch: bool,
}

impl DevCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style("Starting development server...\n").cyan());

        // TODO: Implement dev server with cargo-watch
        // For now, just run cargo run
        println!("Running: cargo run");

        std::process::Command::new("cargo")
            .arg("run")
            .status()
            .context("Failed to execute cargo run")?;

        Ok(())
    }
}
