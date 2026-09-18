use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct DoctorCommand {
    /// Check project health
    #[arg(long)]
    pub project: bool,
}

impl DoctorCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style("Oxide Doctor\n").cyan().bold());

        if self.project {
            self.check_project()?;
        } else {
            self.check_environment()?;
        }

        Ok(())
    }

    fn check_environment(&self) -> Result<()> {
        println!("{}", style("Checking environment...\n").cyan());

        let mut issues = Vec::new();

        // Check Rust
        match std::process::Command::new("rustc").arg("--version").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  {} Rust: {}", style("✓").green(), version);
            }
            Err(_) => {
                println!("  {} Rust: {}", style("✗").red(), "Not found");
                issues.push("Rust is not installed. Install from https://rustup.rs/");
            }
        }

        // Check Cargo
        match std::process::Command::new("cargo").arg("--version").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  {} Cargo: {}", style("✓").green(), version);
            }
            Err(_) => {
                println!("  {} Cargo: {}", style("✗").red(), "Not found");
                issues.push("Cargo is not installed. It should come with Rust.");
            }
        }

        // Check Docker
        match std::process::Command::new("docker").arg("--version").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  {} Docker: {}", style("✓").green(), version);
            }
            Err(_) => {
                println!("  {} Docker: {} (optional)", style("⚠").yellow(), "Not found");
                issues.push("Docker is not installed. Required for service management.");
            }
        }

        // Check Git
        match std::process::Command::new("git").arg("--version").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  {} Git: {}", style("✓").green(), version);
            }
            Err(_) => {
                println!("  {} Git: {} (optional)", style("⚠").yellow(), "Not found");
            }
        }

        println!();
        if issues.is_empty() {
            println!("{}", style("✓ All checks passed!").green().bold());
        } else {
            println!("{}", style("Issues found:").yellow().bold());
            for issue in &issues {
                println!("  • {}", issue);
            }
        }

        Ok(())
    }

    fn check_project(&self) -> Result<()> {
        if !std::path::Path::new("nexus.toml").exists() {
            anyhow::bail!("Not an oxide project. Run this command in a project created with 'oxide create'.");
        }

        println!("{}", style("Checking project...\n").cyan());

        let mut issues = Vec::new();

        // Read nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;

        let config: toml::Value = toml::from_str(&nexus_toml)
            .context("Failed to parse nexus.toml")?;

        let project_name = config["project"]["name"].as_str().unwrap_or("unknown");
        let framework = config["project"]["framework"].as_str().unwrap_or("unknown");
        let db_provider = config["database"]["provider"].as_str().unwrap_or("none");
        let auth_provider = config["auth"]["provider"].as_str().unwrap_or("none");
        let cache_provider = config["cache"]["provider"].as_str().unwrap_or("none");

        println!("  {} Project: {}", style("✓").green(), project_name);
        println!("  {} Framework: {}", style("✓").green(), framework);
        println!("  {} Database: {}", style("✓").green(), db_provider);
        println!("  {} Auth: {}", style("✓").green(), auth_provider);
        println!("  {} Cache: {}", style("✓").green(), cache_provider);

        // Check required files
        let required_files = vec![
            "Cargo.toml",
            "src/main.rs",
            "src/state.rs",
            "src/routes/mod.rs",
            ".env",
        ];

        println!("\n{}", style("Checking files:").cyan());
        for file in &required_files {
            if std::path::Path::new(file).exists() {
                println!("  {} {}", style("✓").green(), file);
            } else {
                println!("  {} {}", style("✗").red(), file);
                issues.push(format!("Missing required file: {}", file));
            }
        }

        // Check database-specific files
        if db_provider != "none" {
            let db_files = vec!["src/database.rs", "migrations/"];
            println!("\n{}", style("Checking database files:").cyan());
            for file in &db_files {
                if std::path::Path::new(file).exists() {
                    println!("  {} {}", style("✓").green(), file);
                } else {
                    println!("  {} {}", style("✗").red(), file);
                    issues.push(format!("Missing database file: {}", file));
                }
            }
        }

        // Check auth-specific files
        if auth_provider == "jwt" {
            let auth_files = vec!["src/middleware/auth.rs"];
            println!("\n{}", style("Checking auth files:").cyan());
            for file in &auth_files {
                if std::path::Path::new(file).exists() {
                    println!("  {} {}", style("✓").green(), file);
                } else {
                    println!("  {} {}", style("✗").red(), file);
                    issues.push(format!("Missing auth file: {}", file));
                }
            }
        }

        // Check API features
        let api_cors = config.get("api").and_then(|api| api.get("cors")).and_then(|v| v.as_bool()).unwrap_or(false);
        let api_validation = config.get("api").and_then(|api| api.get("validation")).and_then(|v| v.as_bool()).unwrap_or(false);
        let api_openapi = config.get("api").and_then(|api| api.get("openapi")).and_then(|v| v.as_bool()).unwrap_or(false);

        if api_cors || api_validation || api_openapi {
            println!("\n{}", style("Checking API features:").cyan());
            if api_cors {
                if std::path::Path::new("src/middleware/cors.rs").exists() {
                    println!("  {} CORS", style("✓").green());
                } else {
                    println!("  {} CORS", style("✗").red());
                    issues.push("CORS enabled but middleware not found".to_string());
                }
            }
            if api_validation {
                if std::path::Path::new("src/validators/mod.rs").exists() {
                    println!("  {} Validation", style("✓").green());
                } else {
                    println!("  {} Validation", style("✗").red());
                    issues.push("Validation enabled but validators not found".to_string());
                }
            }
            if api_openapi {
                if std::path::Path::new("src/docs.rs").exists() {
                    println!("  {} OpenAPI", style("✓").green());
                } else {
                    println!("  {} OpenAPI", style("✗").red());
                    issues.push("OpenAPI enabled but docs not found".to_string());
                }
            }
        }

        println!();
        if issues.is_empty() {
            println!("{}", style("✓ Project looks healthy!").green().bold());
        } else {
            println!("{}", style("Issues found:").yellow().bold());
            for issue in &issues {
                println!("  • {}", issue);
            }
        }

        Ok(())
    }
}
