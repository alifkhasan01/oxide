use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Args;
use console::style;

use crate::core::generator::ProjectGenerator;
use crate::core::stack::StackResolver;
use crate::config::schema::ProjectConfig;

#[derive(Args)]
pub struct CreateCommand {
    /// Project name
    pub name: String,

    /// Framework to use
    #[arg(long, value_enum)]
    pub framework: Option<FrameworkArg>,

    /// Database to use
    #[arg(long, value_enum)]
    pub database: Option<DatabaseArg>,

    /// Authentication method
    #[arg(long, value_enum)]
    pub auth: Option<AuthArg>,

    /// Cache to use
    #[arg(long, value_enum)]
    pub cache: Option<CacheArg>,

    /// Run in non-interactive mode
    #[arg(long)]
    pub non_interactive: bool,
}

#[derive(clap::ValueEnum, Clone)]
pub enum FrameworkArg {
    Axum,
    Actix,
    Rocket,
}

#[derive(clap::ValueEnum, Clone)]
pub enum DatabaseArg {
    Postgres,
    Mysql,
    Sqlite,
    None,
}

#[derive(clap::ValueEnum, Clone)]
pub enum AuthArg {
    Jwt,
    Session,
    None,
}

#[derive(clap::ValueEnum, Clone)]
pub enum CacheArg {
    Redis,
    None,
}

impl CreateCommand {
    pub fn execute(&self) -> Result<()> {
        println!("\n{}", style("Oxide").bold().cyan());
        println!("{}", style("Rust Backend Generator\n").dim());

        // Validate project name
        let project_path = PathBuf::from(&self.name);
        if project_path.exists() {
            anyhow::bail!(
                "Directory '{}' already exists. Please choose a different name.",
                self.name
            );
        }

        // Get configuration (interactive or from args)
        let config = if self.non_interactive {
            self.get_config_from_args()?
        } else {
            self.interactive_prompt()?
        };

        // Resolve stack
        let stack = StackResolver::resolve(&config)?;

        // Generate project
        println!("{}", style("Creating project...\n").cyan());

        let generator = ProjectGenerator::new(config.clone());
        generator.generate(&project_path, &stack)?;

        // Print success message
        println!("\n{}", style("✓ Project created successfully!\n").green().bold());
        println!("{}", style("Next steps:").cyan());
        println!("  cd {}", self.name);
        println!("  oxide dev\n");

        Ok(())
    }

    fn interactive_prompt(&self) -> Result<ProjectConfig> {
        use dialoguer::{Select, theme::ColorfulTheme};

        let theme = ColorfulTheme::default();

        // Framework selection
        let frameworks = vec!["Axum", "Actix Web", "Rocket"];
        let framework_idx = Select::with_theme(&theme)
            .with_prompt("Backend framework")
            .items(&frameworks)
            .default(0)
            .interact()
            .context("Failed to read framework selection")?;

        // Database selection
        let databases = vec!["PostgreSQL", "MySQL", "SQLite", "None"];
        let database_idx = Select::with_theme(&theme)
            .with_prompt("Database")
            .items(&databases)
            .default(0)
            .interact()
            .context("Failed to read database selection")?;

        // Auth selection
        let auth_methods = vec!["None", "JWT", "Session"];
        let auth_idx = Select::with_theme(&theme)
            .with_prompt("Authentication")
            .items(&auth_methods)
            .default(0)
            .interact()
            .context("Failed to read auth selection")?;

        // Cache selection
        let cache_options = vec!["None", "Redis"];
        let cache_idx = Select::with_theme(&theme)
            .with_prompt("Cache")
            .items(&cache_options)
            .default(0)
            .interact()
            .context("Failed to read cache selection")?;

        // API documentation selection
        let doc_options = vec!["None", "OpenAPI"];
        let doc_idx = Select::with_theme(&theme)
            .with_prompt("API documentation")
            .items(&doc_options)
            .default(0)
            .interact()
            .context("Failed to read documentation selection")?;

        Ok(ProjectConfig {
            name: self.name.clone(),
            framework: frameworks[framework_idx].to_string(),
            database: databases[database_idx].to_string(),
            auth: auth_methods[auth_idx].to_string(),
            cache: cache_options[cache_idx].to_string(),
            api_docs: doc_options[doc_idx].to_string(),
        })
    }

    fn get_config_from_args(&self) -> Result<ProjectConfig> {
        let framework = match &self.framework {
            Some(FrameworkArg::Axum) => "Axum".to_string(),
            Some(FrameworkArg::Actix) => "Actix Web".to_string(),
            Some(FrameworkArg::Rocket) => "Rocket".to_string(),
            None => "Axum".to_string(), // Default
        };

        let database = match &self.database {
            Some(DatabaseArg::Postgres) => "PostgreSQL".to_string(),
            Some(DatabaseArg::Mysql) => "MySQL".to_string(),
            Some(DatabaseArg::Sqlite) => "SQLite".to_string(),
            Some(DatabaseArg::None) | None => "None".to_string(),
        };

        let auth = match &self.auth {
            Some(AuthArg::Jwt) => "JWT".to_string(),
            Some(AuthArg::Session) => "Session".to_string(),
            Some(AuthArg::None) | None => "None".to_string(),
        };

        let cache = match &self.cache {
            Some(CacheArg::Redis) => "Redis".to_string(),
            Some(CacheArg::None) | None => "None".to_string(),
        };

        Ok(ProjectConfig {
            name: self.name.clone(),
            framework,
            database,
            auth,
            cache,
            api_docs: "None".to_string(),
        })
    }
}
