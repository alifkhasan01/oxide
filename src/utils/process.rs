#![allow(dead_code)]

use std::process::Command;
use anyhow::{Context, Result};
use console::style;

pub struct CommandRunner;

impl CommandRunner {
    pub fn run(program: &str, args: &[&str]) -> Result<()> {
        let status = Command::new(program)
            .args(args)
            .status()
            .context(format!("Failed to execute {} {}", program, args.join(" ")))?;

        if !status.success() {
            anyhow::bail!(
                "{} Command failed with exit code: {:?}",
                style("✗").red(),
                status.code()
            );
        }

        Ok(())
    }

    pub fn run_with_output(program: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(program)
            .args(args)
            .output()
            .context(format!("Failed to execute {} {}", program, args.join(" ")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "{} Command failed: {}",
                style("✗").red(),
                stderr.trim()
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn which(command: &str) -> Result<String> {
        which::which(command)
            .context(format!("{} not found in PATH", command))
            .map(|p| p.to_string_lossy().to_string())
    }
}
