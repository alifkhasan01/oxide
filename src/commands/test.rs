use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct TestCommand {
    /// Run tests in release mode
    #[arg(short, long)]
    pub release: bool,

    /// Test name pattern to filter
    #[arg(short, long)]
    pub name: Option<String>,
}

impl TestCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style("Running tests...\n").cyan());

        let mut args = vec!["test"];
        if self.release {
            args.push("--release");
        }
        if let Some(ref name) = self.name {
            args.push(name);
        }

        std::process::Command::new("cargo")
            .args(&args)
            .status()
            .context("Failed to execute cargo test")?;

        println!("\n{}", style("✓ Tests completed!").green());
        Ok(())
    }
}
