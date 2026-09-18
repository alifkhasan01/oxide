use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{
    create::CreateCommand,
    dev::DevCommand,
    build::BuildCommand,
    test::TestCommand,
    check::CheckCommand,
    generate::GenerateCommand,
    add::AddCommand,
    services::ServicesCommand,
    completion::CompletionCommand,
    doctor::DoctorCommand,
    upgrade::UpgradeCommand,
};

#[derive(Parser)]
#[command(
    name = "oxide",
    about = "A modern Rust CLI for generating, developing, and managing backend projects",
    version,
    long_about = "Oxide is a Rust-based CLI designed to simplify backend development by providing a unified workflow for creating projects, selecting technology stacks, generating resources, managing development services, and running common development tasks."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new backend project
    Create(CreateCommand),

    /// Generate a backend resource
    #[command(alias = "g")]
    Generate(GenerateCommand),

    /// Add a feature to an existing project
    Add(AddCommand),

    /// Start the development server
    Dev(DevCommand),

    /// Build the project
    Build(BuildCommand),

    /// Run tests
    Test(TestCommand),

    /// Run checks (cargo check, fmt, clippy)
    Check(CheckCommand),

    /// Manage development services
    Services(ServicesCommand),

    /// Generate shell completion scripts
    Completion(CompletionCommand),

    /// Check environment and project health
    Doctor(DoctorCommand),

    /// Upgrade project to latest Oxide version
    Upgrade(UpgradeCommand),
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Create(cmd) => cmd.execute(),
            Commands::Generate(cmd) => cmd.execute(),
            Commands::Add(cmd) => cmd.execute(),
            Commands::Dev(cmd) => cmd.execute(),
            Commands::Build(cmd) => cmd.execute(),
            Commands::Test(cmd) => cmd.execute(),
            Commands::Check(cmd) => cmd.execute(),
            Commands::Services(cmd) => cmd.execute(),
            Commands::Completion(cmd) => cmd.execute(),
            Commands::Doctor(cmd) => cmd.execute(),
            Commands::Upgrade(cmd) => cmd.execute(),
        }
    }
}
