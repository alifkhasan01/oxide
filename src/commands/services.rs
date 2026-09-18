use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use console::style;

#[derive(Args)]
pub struct ServicesCommand {
    #[command(subcommand)]
    pub action: ServicesAction,
}

#[derive(Subcommand)]
pub enum ServicesAction {
    /// Start development services
    Up,

    /// Stop development services
    Down,

    /// Show service status
    Status,
}

impl ServicesCommand {
    pub fn execute(&self) -> Result<()> {
        match &self.action {
            ServicesAction::Up => {
                println!("{}", style("Starting development services...\n").cyan());

                // Check if we're in an oxide project
                if !std::path::Path::new("nexus.toml").exists() {
                    anyhow::bail!(
                        "Not an oxide project. Run this command in a project created with 'oxide create'."
                    );
                }

                // Read nexus.toml to check which services are needed
                let nexus_toml = std::fs::read_to_string("nexus.toml")
                    .context("Failed to read nexus.toml")?;

                let config: toml::Value = toml::from_str(&nexus_toml)
                    .context("Failed to parse nexus.toml")?;

                let db_provider = config["database"]["provider"]
                    .as_str()
                    .unwrap_or("none");

                let cache_provider = config["cache"]["provider"]
                    .as_str()
                    .unwrap_or("none");

                // Check if docker is available
                let docker_available = std::process::Command::new("docker")
                    .arg("--version")
                    .output()
                    .is_ok();

                if !docker_available {
                    println!("{}", style("⚠ Docker is not installed or not in PATH").yellow());
                    println!("Please install Docker to use service management.");
                    println!("https://docs.docker.com/get-docker/");
                    return Ok(());
                }

                // Generate docker-compose.yml if it doesn't exist
                if !std::path::Path::new("docker-compose.yml").exists() {
                    self.generate_docker_compose(db_provider, cache_provider)?;
                }

                // Start services
                println!("Starting services...");

                if db_provider != "none" {
                    let service_name = match db_provider {
                        "postgres" => "postgres",
                        "mysql" => "mysql",
                        "sqlite" => "sqlite",
                        _ => "database",
                    };

                    println!("  {} Starting {}...", style("→").cyan(), service_name);
                    let status = std::process::Command::new("docker")
                        .args(["compose", "up", "-d", service_name])
                        .status()
                        .context("Failed to start database service")?;

                    if status.success() {
                        println!("  {} {} is ready", style("✓").green(), service_name);
                    } else {
                        println!("  {} Failed to start {}", style("✗").red(), service_name);
                    }
                }

                if cache_provider == "redis" {
                    println!("  {} Starting redis...", style("→").cyan());
                    let status = std::process::Command::new("docker")
                        .args(["compose", "up", "-d", "redis"])
                        .status()
                        .context("Failed to start redis service")?;

                    if status.success() {
                        println!("  {} redis is ready", style("✓").green());
                    } else {
                        println!("  {} Failed to start redis", style("✗").red());
                    }
                }

                println!("\n{}", style("✓ Services are ready!").green().bold());

                Ok(())
            }
            ServicesAction::Down => {
                println!("{}", style("Stopping development services...\n").cyan());

                if !std::path::Path::new("docker-compose.yml").exists() {
                    println!("{}", style("No docker-compose.yml found.").yellow());
                    return Ok(());
                }

                let status = std::process::Command::new("docker")
                    .args(["compose", "down"])
                    .status()
                    .context("Failed to stop services")?;

                if status.success() {
                    println!("{}", style("✓ Services stopped.").green());
                } else {
                    println!("{}", style("✗ Failed to stop services.").red());
                }

                Ok(())
            }
            ServicesAction::Status => {
                println!("{}", style("Checking service status...\n").cyan());

                if !std::path::Path::new("docker-compose.yml").exists() {
                    println!("{}", style("No docker-compose.yml found.").yellow());
                    return Ok(());
                }

                std::process::Command::new("docker")
                    .args(["compose", "ps"])
                    .status()
                    .context("Failed to check service status")?;

                Ok(())
            }
        }
    }

    fn generate_docker_compose(&self, db_provider: &str, cache_provider: &str) -> Result<()> {
        let mut compose = String::from("version: '3.8'\n\nservices:\n");

        match db_provider {
            "postgres" => {
                compose.push_str(
                    r#"  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: my_database
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
"#
                );
            }
            "mysql" => {
                compose.push_str(
                    r#"  mysql:
    image: mysql:8.0
    environment:
      MYSQL_ROOT_PASSWORD: rootpassword
      MYSQL_DATABASE: my_database
      MYSQL_USER: user
      MYSQL_PASSWORD: password
    ports:
      - "3306:3306"
    volumes:
      - mysql_data:/var/lib/mysql
"#
                );
            }
            "sqlite" => {
                // SQLite doesn't need a service
            }
            _ => {}
        }

        if cache_provider == "redis" {
            compose.push_str(
                r#"  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
"#
            );
        }

        // Add volumes section
        compose.push_str("\nvolumes:\n");
        if db_provider == "postgres" {
            compose.push_str("  postgres_data:\n");
        } else if db_provider == "mysql" {
            compose.push_str("  mysql_data:\n");
        }
        if cache_provider == "redis" {
            compose.push_str("  redis_data:\n");
        }

        std::fs::write("docker-compose.yml", compose)
            .context("Failed to write docker-compose.yml")?;

        println!("  {} Generated docker-compose.yml", style("✓").green());

        Ok(())
    }
}
