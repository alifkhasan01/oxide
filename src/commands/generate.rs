use std::path::Path;
use anyhow::Result;
use clap::{Args, Subcommand};
use console::style;

use crate::core::resource::ResourceGenerator;

#[derive(Args)]
pub struct GenerateCommand {
    #[command(subcommand)]
    pub resource: GenerateResource,
}

#[derive(Subcommand)]
pub enum GenerateResource {
    /// Generate a backend resource
    Resource {
        /// Resource name
        name: String,

        /// Generate CRUD operations
        #[arg(long)]
        crud: bool,
    },
}

impl GenerateCommand {
    pub fn execute(&self) -> Result<()> {
        match &self.resource {
            GenerateResource::Resource { name, crud } => {
                // Check if we're in an oxide project
                if !std::path::Path::new("nexus.toml").exists() {
                    anyhow::bail!(
                        "Not an oxide project. Run this command in a project created with 'oxide create'."
                    );
                }

                ResourceGenerator::generate(Path::new("."), name, *crud)?;

                println!("\n{}", style("✓ Resource generated successfully!").green().bold());
                println!("\nNext steps:");
                println!("  Add routes to src/routes/{}.rs", name);
                println!("  Implement handlers in src/handlers/{}.rs", name);

                Ok(())
            }
        }
    }
}
