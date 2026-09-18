use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct BuildCommand {
    /// Build in release mode
    #[arg(short, long)]
    pub release: bool,
}

impl BuildCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style("Building project...\n").cyan());

        let mut args = vec!["build"];
        if self.release {
            args.push("--release");
        }

        std::process::Command::new("cargo")
            .args(&args)
            .status()
            .context("Failed to execute cargo build")?;

        println!("\n{}", style("✓ Build completed!").green());
        Ok(())
    }
}
