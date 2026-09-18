use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct UpgradeCommand {
    /// Check for updates without applying
    #[arg(long)]
    pub check: bool,
}

impl UpgradeCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style("Oxide Upgrade\n").cyan().bold());

        if !std::path::Path::new("nexus.toml").exists() {
            anyhow::bail!("Not an oxide project. Run this command in a project created with 'oxide create'.");
        }

        // Read nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;

        let config: toml::Value = toml::from_str(&nexus_toml)
            .context("Failed to parse nexus.toml")?;

        let current_version = config["tool"]["oxide"]["version"]
            .as_str()
            .unwrap_or("0.0.0");

        let latest_version = env!("CARGO_PKG_VERSION");

        println!("  Current version: {}", current_version);
        println!("  Latest version:  {}", latest_version);

        if current_version == latest_version {
            println!("\n{}", style("✓ Project is up to date!").green().bold());
            return Ok(());
        }

        if self.check {
            println!("\n{}", style("Update available!").yellow().bold());
            println!("Run 'oxide upgrade' to update your project.");
            return Ok(());
        }

        println!("\n{}", style("Upgrading project...").cyan());

        // Update nexus.toml
        let nexus_toml = nexus_toml.replace(
            &format!("version = \"{}\"", current_version),
            &format!("version = \"{}\"", latest_version)
        );
        std::fs::write("nexus.toml", nexus_toml)
            .context("Failed to write nexus.toml")?;

        println!("  {} Updated nexus.toml", style("✓").green());

        // Check for new features and suggest adding them
        self.suggest_new_features()?;

        println!("\n{}", style("✓ Project upgraded successfully!").green().bold());
        println!("\nNote: Some changes may require manual review.");
        println!("Check your project files for any necessary updates.");

        Ok(())
    }

    fn suggest_new_features(&self) -> Result<()> {
        println!("\n{}", style("Checking for new features...").cyan());

        let features = vec![
            ("cors", "Add CORS support: oxide add cors"),
            ("validation", "Add request validation: oxide add validation"),
            ("openapi", "Add OpenAPI docs: oxide add openapi"),
            ("websocket", "Add WebSocket support: oxide add websocket"),
            ("ratelimit", "Add rate limiting: oxide add ratelimit"),
        ];

        let mut suggestions = Vec::new();

        for (feature, message) in &features {
            if !self.has_feature(feature) {
                suggestions.push(message.to_string());
            }
        }

        if !suggestions.is_empty() {
            println!("\n{}", style("Suggested features to add:").yellow());
            for suggestion in &suggestions {
                println!("  • {}", suggestion);
            }
        } else {
            println!("  {} All recommended features are enabled", style("✓").green());
        }

        Ok(())
    }

    fn has_feature(&self, feature: &str) -> bool {
        if let Ok(nexus_toml) = std::fs::read_to_string("nexus.toml") {
            if let Ok(config) = nexus_toml.parse::<toml::Value>() {
                match feature {
                    "cors" => config.get("api")
                        .and_then(|api| api.get("cors"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    "validation" => config.get("api")
                        .and_then(|api| api.get("validation"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    "openapi" => config.get("api")
                        .and_then(|api| api.get("openapi"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    "websocket" => config.get("api")
                        .and_then(|api| api.get("websocket"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    "ratelimit" => config.get("api")
                        .and_then(|api| api.get("ratelimit"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}
