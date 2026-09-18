use anyhow::Result;
use crate::config::schema::ProjectConfig;
use crate::stacks::{axum, database, auth, redis};

pub struct Stack {
    pub dependencies: Vec<Dependency>,
}

pub struct Dependency {
    pub name: &'static str,
    pub version: &'static str,
    pub features: &'static [&'static str],
}

impl Stack {
    pub fn dependencies(&self) -> &[Dependency] {
        &self.dependencies
    }
}

pub struct StackResolver;

impl StackResolver {
    pub fn resolve(config: &ProjectConfig) -> Result<Stack> {
        let mut dependencies = Vec::new();

        // Add framework dependencies
        match config.framework.as_str() {
            "Axum" => dependencies.extend(axum::dependencies()),
            "Actix Web" => {
                // TODO: Implement Actix support
                anyhow::bail!("Actix Web support will be implemented in future versions");
            }
            "Rocket" => {
                // TODO: Implement Rocket support
                anyhow::bail!("Rocket support will be implemented in future versions");
            }
            _ => {
                anyhow::bail!("Unknown framework: {}", config.framework);
            }
        }

        // Add database dependencies
        match config.database.as_str() {
            "PostgreSQL" => dependencies.extend(database::postgres_dependencies()),
            "MySQL" => dependencies.extend(database::mysql_dependencies()),
            "SQLite" => dependencies.extend(database::sqlite_dependencies()),
            "None" => {}
            _ => {
                anyhow::bail!("Unknown database: {}", config.database);
            }
        }

        // Add authentication dependencies
        match config.auth.as_str() {
            "JWT" => dependencies.extend(auth::jwt_dependencies()),
            "Session" => {
                // TODO: Implement Session support
                anyhow::bail!("Session authentication will be implemented in future versions");
            }
            "None" => {}
            _ => {
                anyhow::bail!("Unknown auth method: {}", config.auth);
            }
        }

        // Add cache dependencies
        match config.cache.as_str() {
            "Redis" => dependencies.extend(redis::dependencies()),
            "None" => {}
            _ => {
                anyhow::bail!("Unknown cache: {}", config.cache);
            }
        }

        Ok(Stack { dependencies })
    }
}
