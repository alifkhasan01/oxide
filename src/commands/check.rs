use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct CheckCommand {
    /// Skip cargo check
    #[arg(long)]
    pub skip_check: bool,

    /// Skip cargo fmt check
    #[arg(long)]
    pub skip_fmt: bool,

    /// Skip cargo clippy
    #[arg(long)]
    pub skip_clippy: bool,
}

impl CheckCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style("Running checks...\n").cyan());

        let mut failed = false;

        // cargo check
        if !self.skip_check {
            println!("{}", style("Running cargo check...").dim());
            let status = std::process::Command::new("cargo")
                .arg("check")
                .status()
                .context("Failed to execute cargo check")?;

            if !status.success() {
                println!("{}", style("✗ cargo check failed").red());
                failed = true;
            } else {
                println!("{}", style("✓ cargo check passed").green());
            }
        }

        // cargo fmt --check
        if !self.skip_fmt {
            println!("\n{}", style("Running cargo fmt --check...").dim());
            let status = std::process::Command::new("cargo")
                .args(["fmt", "--check"])
                .status()
                .context("Failed to execute cargo fmt")?;

            if !status.success() {
                println!("{}", style("✗ cargo fmt check failed").red());
                failed = true;
            } else {
                println!("{}", style("✓ cargo fmt check passed").green());
            }
        }

        // cargo clippy
        if !self.skip_clippy {
            println!("\n{}", style("Running cargo clippy...").dim());
            let status = std::process::Command::new("cargo")
                .args(["clippy", "--", "-D", "warnings"])
                .status()
                .context("Failed to execute cargo clippy")?;

            if !status.success() {
                println!("{}", style("✗ cargo clippy failed").red());
                failed = true;
            } else {
                println!("{}", style("✓ cargo clippy passed").green());
            }
        }

        println!();
        if failed {
            anyhow::bail!("Some checks failed");
        } else {
            println!("{}", style("✓ All checks passed!").green().bold());
        }

        Ok(())
    }
}
