use anyhow::{Context, Result};
use clap::Args;
use console::style;

#[derive(Args)]
pub struct AddCommand {
    /// Feature to add (postgres, mysql, sqlite, redis, auth, cors, validation, openapi, etc.)
    pub feature: String,

    /// Run in dry-run mode (show what would be modified)
    #[arg(long)]
    pub dry_run: bool,
}

impl AddCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", style(format!("Adding '{}'...\n", self.feature)).cyan());

        // Check if we're in an oxide project
        if !std::path::Path::new("nexus.toml").exists() {
            anyhow::bail!(
                "Not an oxide project. Run this command in a project created with 'oxide create'."
            );
        }

        // Read nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;

        // Parse the configuration
        let config: toml::Value = toml::from_str(&nexus_toml)
            .context("Failed to parse nexus.toml")?;

        let project_name = config["project"]["name"]
            .as_str()
            .unwrap_or("unknown");

        // Check if feature is already enabled
        let current_db = config["database"]["provider"].as_str().unwrap_or("none");
        let current_auth = config["auth"]["provider"].as_str().unwrap_or("none");
        let current_cache = config["cache"]["provider"].as_str().unwrap_or("none");
        let current_cors = config.get("api").and_then(|api| api.get("cors")).and_then(|v| v.as_bool()).unwrap_or(false);
        let current_validation = config.get("api").and_then(|api| api.get("validation")).and_then(|v| v.as_bool()).unwrap_or(false);
        let current_openapi = config.get("api").and_then(|api| api.get("openapi")).and_then(|v| v.as_bool()).unwrap_or(false);

        match self.feature.to_lowercase().as_str() {
            "postgres" | "postgresql" => {
                if current_db == "postgres" {
                    println!("{}", style("PostgreSQL is already enabled.").yellow());
                    return Ok(());
                }
                self.add_postgres(project_name)?;
            }
            "mysql" => {
                if current_db == "mysql" {
                    println!("{}", style("MySQL is already enabled.").yellow());
                    return Ok(());
                }
                self.add_mysql(project_name)?;
            }
            "sqlite" => {
                if current_db == "sqlite" {
                    println!("{}", style("SQLite is already enabled.").yellow());
                    return Ok(());
                }
                self.add_sqlite(project_name)?;
            }
            "redis" => {
                if current_cache == "redis" {
                    println!("{}", style("Redis is already enabled.").yellow());
                    return Ok(());
                }
                self.add_redis()?;
            }
            "auth" | "jwt" => {
                if current_auth == "jwt" {
                    println!("{}", style("JWT authentication is already enabled.").yellow());
                    return Ok(());
                }
                self.add_jwt()?;
            }
            "cors" => {
                if current_cors {
                    println!("{}", style("CORS is already enabled.").yellow());
                    return Ok(());
                }
                self.add_cors()?;
            }
            "validation" | "validate" => {
                if current_validation {
                    println!("{}", style("Validation is already enabled.").yellow());
                    return Ok(());
                }
                self.add_validation()?;
            }
            "openapi" | "swagger" | "docs" => {
                if current_openapi {
                    println!("{}", style("OpenAPI is already enabled.").yellow());
                    return Ok(());
                }
                self.add_openapi()?;
            }
            "websocket" | "ws" => {
                self.add_websocket()?;
            }
            "ratelimit" | "rate-limit" => {
                self.add_ratelimit()?;
            }
            _ => {
                anyhow::bail!("Unknown feature: {}. Available features: postgres, mysql, sqlite, redis, auth, cors, validation, openapi, websocket, ratelimit", self.feature);
            }
        }

        println!("\n{}", style(format!("✓ '{}' has been added!", self.feature)).green().bold());

        Ok(())
    }

    fn add_dependency_to_cargo_toml(&self, deps: &[(&str, &str, &[&str])]) -> Result<()> {
        let cargo_toml = std::fs::read_to_string("Cargo.toml")
            .context("Failed to read Cargo.toml")?;

        let mut lines: Vec<String> = cargo_toml.lines().map(String::from).collect();
        let mut insert_pos = None;

        // Find [dependencies] section
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "[dependencies]" {
                insert_pos = Some(i + 1);
                break;
            }
        }

        if let Some(pos) = insert_pos {
            // Check if dependencies already exist
            let existing_deps: Vec<String> = lines[pos..].iter()
                .take_while(|l| !l.trim().is_empty() && !l.trim().starts_with('['))
                .map(|l| l.trim().to_string())
                .collect();

            for (name, version, features) in deps {
                let dep_name = name.to_string();
                if existing_deps.iter().any(|d| d.starts_with(&format!("{} ", dep_name)) || d.starts_with(&format!("{}=", dep_name))) {
                    continue; // Skip if already exists
                }

                let dep_str = if features.is_empty() {
                    format!("{} = \"{}\"", name, version)
                } else {
                    let features_str = features.iter()
                        .map(|f| format!("\"{}\"", f))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{} = {{ version = \"{}\", features = [{}] }}", name, version, features_str)
                };

                lines.insert(pos, dep_str);
            }
        }

        std::fs::write("Cargo.toml", lines.join("\n"))
            .context("Failed to write Cargo.toml")?;

        Ok(())
    }

    fn update_nexus_toml(&self, section: &str, key: &str, value: &str) -> Result<()> {
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;

        // Find the section and update the value
        let mut lines: Vec<String> = nexus_toml.lines().map(String::from).collect();
        let mut in_section = false;

        for line in &mut lines {
            if line.trim() == format!("[{}]", section) {
                in_section = true;
                continue;
            }

            if in_section && line.trim().starts_with('[') {
                break;
            }

            if in_section && line.trim().starts_with(key) {
                *line = format!("{} = \"{}\"", key, value);
                break;
            }
        }

        // If section doesn't exist, add it
        if !lines.iter().any(|l| l.trim() == format!("[{}]", section)) {
            lines.push(format!("\n[{}]", section));
            lines.push(format!("{} = \"{}\"", key, value));
        }

        std::fs::write("nexus.toml", lines.join("\n"))
            .context("Failed to write nexus.toml")?;

        Ok(())
    }

    fn update_env_example(&self, key: &str, value: &str) -> Result<()> {
        let mut env_content = std::fs::read_to_string(".env.example")
            .unwrap_or_else(|_| "# Server configuration\nAPP_SERVER_HOST=127.0.0.1\nAPP_SERVER_PORT=3000\n\n# Logging\nRUST_LOG=info\n".to_string());

        if !env_content.contains(key) {
            env_content.push_str(&format!("\n# {}\n{}={}\n", key, key, value));
        }
        std::fs::write(".env.example", &env_content)
            .context("Failed to write .env.example")?;

        Ok(())
    }

    fn add_module_to_main_rs(&self, module_decl: &str) -> Result<()> {
        let main_rs = std::fs::read_to_string("src/main.rs")
            .context("Failed to read main.rs")?;

        if !main_rs.contains(module_decl) {
            let main_rs = main_rs.replace(
                "mod state;",
                &format!("mod state;\n{}", module_decl)
            );
            std::fs::write("src/main.rs", main_rs)
                .context("Failed to write main.rs")?;
        }

        Ok(())
    }

    fn add_postgres(&self, project_name: &str) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  src/database.rs");
            println!("  src/state.rs");
            println!("  .env.example");
            println!("  migrations/");
            return Ok(());
        }

        println!("Adding PostgreSQL support...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("sqlx", "0.8", &["runtime-tokio", "tls-rustls", "postgres", "chrono", "uuid"]),
            ("chrono", "0.4", &["serde"]),
            ("uuid", "1", &["v4", "serde"]),
        ])?;

        // Update nexus.toml
        self.update_nexus_toml("database", "provider", "postgres")?;

        // Create database.rs
        let db_rs = r#"use sqlx::postgres::PgPool;
use anyhow::{Context, Result};

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = sqlx::postgres::PgPool::connect(database_url)
        .await
        .context("Failed to connect to PostgreSQL")?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}
"#;
        std::fs::write("src/database.rs", db_rs)
            .context("Failed to write database.rs")?;

        // Add database module to main.rs
        self.add_module_to_main_rs("mod database;")?;

        // Update state.rs
        let state_rs = r#"use anyhow::Result;
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let db = crate::database::create_pool(&database_url).await?;

        Ok(Self { db })
    }
}
"#;
        std::fs::write("src/state.rs", state_rs)
            .context("Failed to write state.rs")?;

        // Update .env.example
        let db_url = format!("postgres://user:password@localhost:5432/{}", project_name.replace('-', "_"));
        self.update_env_example("DATABASE_URL", &db_url)?;

        // Create migrations directory
        std::fs::create_dir_all("migrations")
            .context("Failed to create migrations directory")?;

        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let migration_dir = format!("migrations/{}_init", timestamp);
        std::fs::create_dir_all(&migration_dir)
            .context("Failed to create migration directory")?;

        std::fs::write(format!("{}/up.sql", migration_dir), "-- Add your initial migration here\n")
            .context("Failed to write up.sql")?;
        std::fs::write(format!("{}/down.sql", migration_dir), "-- Reverse migration\n")
            .context("Failed to write down.sql")?;

        println!("  {} postgres dependency", style("✓").green());
        println!("  {} Database configuration", style("✓").green());
        println!("  {} Database module", style("✓").green());
        println!("  {} State with database pool", style("✓").green());
        println!("  {} Environment configuration", style("✓").green());
        println!("  {} Migrations directory", style("✓").green());

        Ok(())
    }

    fn add_mysql(&self, project_name: &str) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  src/database.rs");
            println!("  src/state.rs");
            println!("  .env.example");
            println!("  migrations/");
            return Ok(());
        }

        println!("Adding MySQL support...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("sqlx", "0.8", &["runtime-tokio", "tls-rustls", "mysql", "chrono", "uuid"]),
            ("chrono", "0.4", &["serde"]),
            ("uuid", "1", &["v4", "serde"]),
        ])?;

        // Update nexus.toml
        self.update_nexus_toml("database", "provider", "mysql")?;

        // Create database.rs
        let db_rs = r#"use sqlx::mysql::MySqlPool;
use anyhow::{Context, Result};

pub async fn create_pool(database_url: &str) -> Result<MySqlPool> {
    let pool = sqlx::mysql::MySqlPool::connect(database_url)
        .await
        .context("Failed to connect to MySQL")?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}
"#;
        std::fs::write("src/database.rs", db_rs)
            .context("Failed to write database.rs")?;

        // Add database module to main.rs
        self.add_module_to_main_rs("mod database;")?;

        // Update state.rs
        let state_rs = r#"use anyhow::Result;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let db = crate::database::create_pool(&database_url).await?;

        Ok(Self { db })
    }
}
"#;
        std::fs::write("src/state.rs", state_rs)
            .context("Failed to write state.rs")?;

        // Update .env.example
        let db_url = format!("mysql://user:password@localhost:3306/{}", project_name.replace('-', "_"));
        self.update_env_example("DATABASE_URL", &db_url)?;

        // Create migrations directory
        std::fs::create_dir_all("migrations")
            .context("Failed to create migrations directory")?;

        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let migration_dir = format!("migrations/{}_init", timestamp);
        std::fs::create_dir_all(&migration_dir)
            .context("Failed to create migration directory")?;

        std::fs::write(format!("{}/up.sql", migration_dir), "-- Add your initial migration here\n")
            .context("Failed to write up.sql")?;
        std::fs::write(format!("{}/down.sql", migration_dir), "-- Reverse migration\n")
            .context("Failed to write down.sql")?;

        println!("  {} mysql dependency", style("✓").green());
        println!("  {} Database configuration", style("✓").green());
        println!("  {} Database module", style("✓").green());
        println!("  {} State with database pool", style("✓").green());
        println!("  {} Environment configuration", style("✓").green());
        println!("  {} Migrations directory", style("✓").green());

        Ok(())
    }

    fn add_sqlite(&self, _project_name: &str) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  src/database.rs");
            println!("  src/state.rs");
            println!("  .env.example");
            println!("  migrations/");
            return Ok(());
        }

        println!("Adding SQLite support...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("sqlx", "0.8", &["runtime-tokio", "tls-rustls", "sqlite", "chrono", "uuid"]),
            ("chrono", "0.4", &["serde"]),
            ("uuid", "1", &["v4", "serde"]),
        ])?;

        // Update nexus.toml
        self.update_nexus_toml("database", "provider", "sqlite")?;

        // Create database.rs
        let db_rs = r#"use sqlx::sqlite::SqlitePool;
use anyhow::{Context, Result};

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = sqlx::sqlite::SqlitePool::connect(database_url)
        .await
        .context("Failed to connect to SQLite")?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}
"#;
        std::fs::write("src/database.rs", db_rs)
            .context("Failed to write database.rs")?;

        // Add database module to main.rs
        self.add_module_to_main_rs("mod database;")?;

        // Update state.rs
        let state_rs = r#"use anyhow::Result;
use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite::memory:".to_string());

        let db = crate::database::create_pool(&database_url).await?;

        Ok(Self { db })
    }
}
"#;
        std::fs::write("src/state.rs", state_rs)
            .context("Failed to write state.rs")?;

        // Update .env.example
        self.update_env_example("DATABASE_URL", "sqlite::memory:")?;

        // Create migrations directory
        std::fs::create_dir_all("migrations")
            .context("Failed to create migrations directory")?;

        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let migration_dir = format!("migrations/{}_init", timestamp);
        std::fs::create_dir_all(&migration_dir)
            .context("Failed to create migration directory")?;

        std::fs::write(format!("{}/up.sql", migration_dir), "-- Add your initial migration here\n")
            .context("Failed to write up.sql")?;
        std::fs::write(format!("{}/down.sql", migration_dir), "-- Reverse migration\n")
            .context("Failed to write down.sql")?;

        println!("  {} sqlite dependency", style("✓").green());
        println!("  {} Database configuration", style("✓").green());
        println!("  {} Database module", style("✓").green());
        println!("  {} State with database pool", style("✓").green());
        println!("  {} Environment configuration", style("✓").green());
        println!("  {} Migrations directory", style("✓").green());

        Ok(())
    }

    fn add_redis(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  nexus.toml");
            println!("  .env.example");
            return Ok(());
        }

        println!("Adding Redis support...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("redis", "0.26", &["tokio-comp", "connection-manager"]),
        ])?;

        // Update nexus.toml
        self.update_nexus_toml("cache", "provider", "redis")?;

        // Update .env.example
        self.update_env_example("REDIS_URL", "redis://localhost:6379")?;

        println!("  {} redis dependency", style("✓").green());
        println!("  {} Configuration", style("✓").green());
        println!("  {} Environment configuration", style("✓").green());

        Ok(())
    }

    fn add_jwt(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  nexus.toml");
            println!("  .env.example");
            println!("  src/middleware/auth.rs");
            return Ok(());
        }

        println!("Adding JWT authentication...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("jsonwebtoken", "9", &[]),
            ("argon2", "0.5", &["std"]),
            ("rand", "0.8", &[]),
        ])?;

        // Update nexus.toml
        self.update_nexus_toml("auth", "provider", "jwt")?;

        // Update .env.example
        self.update_env_example("JWT_SECRET", "your-secret-key-here")?;
        self.update_env_example("JWT_EXPIRATION", "3600")?;

        // Create auth middleware
        let auth_rs = r#"use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            let secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string());

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            );

            match token_data {
                Ok(_) => Ok(next.run(req).await),
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn create_token(sub: &str) -> Result<String, StatusCode> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret".to_string());
    let expiration = std::env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "3600".to_string())
        .parse::<usize>()
        .unwrap_or(3600);

    let claims = Claims {
        sub: sub.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
"#;
        std::fs::write("src/middleware/auth.rs", auth_rs)
            .context("Failed to write auth middleware")?;

        // Update middleware/mod.rs
        let mod_rs = r#"pub mod auth;
"#;
        std::fs::write("src/middleware/mod.rs", mod_rs)
            .context("Failed to write middleware/mod.rs")?;

        println!("  {} jsonwebtoken dependency", style("✓").green());
        println!("  {} argon2 dependency", style("✓").green());
        println!("  {} rand dependency", style("✓").green());
        println!("  {} Configuration", style("✓").green());
        println!("  {} Environment configuration", style("✓").green());
        println!("  {} Auth middleware", style("✓").green());

        Ok(())
    }

    fn add_cors(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  nexus.toml");
            println!("  src/middleware/cors.rs");
            println!("  src/routes/mod.rs");
            return Ok(());
        }

        println!("Adding CORS support...");

        // Update nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;
        let nexus_toml = if nexus_toml.contains("[api]") {
            nexus_toml.replace("[api]\n", "[api]\ncors = true\n")
        } else {
            format!("{}\n[api]\ncors = true\n", nexus_toml)
        };
        std::fs::write("nexus.toml", nexus_toml)
            .context("Failed to write nexus.toml")?;

        // Create cors middleware
        let cors_rs = r#"use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
"#;
        std::fs::write("src/middleware/cors.rs", cors_rs)
            .context("Failed to write cors middleware")?;

        // Update middleware/mod.rs
        let mod_rs = if std::path::Path::new("src/middleware/auth.rs").exists() {
            r#"pub mod auth;
pub mod cors;
"#
        } else {
            r#"pub mod cors;
"#
        };
        std::fs::write("src/middleware/mod.rs", mod_rs)
            .context("Failed to write middleware/mod.rs")?;

        // Update routes/mod.rs to use CORS
        let routes_rs = r#"use crate::middleware::cors::cors_layer;
use crate::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .layer(cors_layer())
        .with_state(state)
}
"#;
        std::fs::write("src/routes/mod.rs", routes_rs)
            .context("Failed to write routes/mod.rs")?;

        println!("  {} Configuration", style("✓").green());
        println!("  {} CORS middleware", style("✓").green());
        println!("  {} Routes updated", style("✓").green());

        Ok(())
    }

    fn add_validation(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  nexus.toml");
            println!("  src/validators/");
            return Ok(());
        }

        println!("Adding validation support...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("validator", "0.18", &["derive"]),
            ("garde", "0.18", &[]),
        ])?;

        // Update nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;
        let nexus_toml = if nexus_toml.contains("[api]") {
            nexus_toml.replace("[api]\n", "[api]\nvalidation = true\n")
        } else {
            format!("{}\n[api]\nvalidation = true\n", nexus_toml)
        };
        std::fs::write("nexus.toml", nexus_toml)
            .context("Failed to write nexus.toml")?;

        // Create validators directory
        std::fs::create_dir_all("src/validators")
            .context("Failed to create validators directory")?;

        // Create validation example
        let validators_mod_rs = r#"use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

pub async fn validate_request<T: Validate + serde::de::DeserializeOwned>(
    Json(payload): Json<T>,
) -> Result<T, (StatusCode, Json<serde_json::Value>)> {
    payload.validate().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": {
                    "message": e,
                    "code": 400,
                }
            })),
        )
    })?;

    Ok(payload)
}
"#;
        std::fs::write("src/validators/mod.rs", validators_mod_rs)
            .context("Failed to write validators/mod.rs")?;

        println!("  {} validator dependency", style("✓").green());
        println!("  {} garde dependency", style("✓").green());
        println!("  {} Configuration", style("✓").green());
        println!("  {} Validators module", style("✓").green());

        Ok(())
    }

    fn add_openapi(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  nexus.toml");
            println!("  src/docs.rs");
            return Ok(());
        }

        println!("Adding OpenAPI documentation...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("utoipa", "4", &["axum_extras"]),
            ("utoipa-swagger-ui", "7", &["axum"]),
        ])?;

        // Update nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;
        let nexus_toml = if nexus_toml.contains("[api]") {
            nexus_toml.replace("[api]\n", "[api]\nopenapi = true\n")
        } else {
            format!("{}\n[api]\nopenapi = true\n", nexus_toml)
        };
        std::fs::write("nexus.toml", nexus_toml)
            .context("Failed to write nexus.toml")?;

        // Create docs module
        let docs_rs = r#"use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "My API",
        version = "0.1.0",
        description = "A backend API generated by Oxide",
    ),
    paths(
        // Add your API paths here
        // crate::handlers::user::list,
        // crate::handlers::user::get,
        // crate::handlers::user::create,
    ),
    components(
        // Add your API components here
        // crate::models::user::User,
    ),
    tags(
        (name = "user", description = "User management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn swagger_routes() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/:tail*.rs")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
"#;
        std::fs::write("src/docs.rs", docs_rs)
            .context("Failed to write docs.rs")?;

        // Update routes/mod.rs to include Swagger
        let routes_rs = r#"use crate::middleware::cors::cors_layer;
use crate::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router {
    let mut app = Router::new();

    // Add Swagger UI if OpenAPI is enabled
    #[cfg(feature = "openapi")]
    {
        app = app.merge(crate::docs::swagger_routes());
    }

    app.layer(cors_layer())
        .with_state(state)
}
"#;
        std::fs::write("src/routes/mod.rs", routes_rs)
            .context("Failed to write routes/mod.rs")?;

        println!("  {} utoipa dependency", style("✓").green());
        println!("  {} utoipa-swagger-ui dependency", style("✓").green());
        println!("  {} Configuration", style("✓").green());
        println!("  {} Docs module", style("✓").green());
        println!("  {} Swagger UI routes", style("✓").green());

        Ok(())
    }

    fn add_websocket(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  nexus.toml");
            println!("  src/handlers/ws.rs");
            return Ok(());
        }

        println!("Adding WebSocket support...");

        // Update nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;
        let nexus_toml = if nexus_toml.contains("[api]") {
            nexus_toml.replace("[api]\n", "[api]\nwebsocket = true\n")
        } else {
            format!("{}\n[api]\nwebsocket = true\n", nexus_toml)
        };
        std::fs::write("nexus.toml", nexus_toml)
            .context("Failed to write nexus.toml")?;

        // Create WebSocket handler
        let ws_rs = r#"use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<crate::state::AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, _state: crate::state::AppState) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                // Echo back the message
                if sender.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}
"#;
        std::fs::write("src/handlers/ws.rs", ws_rs)
            .context("Failed to write WebSocket handler")?;

        println!("  {} Configuration", style("✓").green());
        println!("  {} WebSocket handler", style("✓").green());

        Ok(())
    }

    fn add_ratelimit(&self) -> Result<()> {
        if self.dry_run {
            println!("{}", style("Dry run mode - no files will be modified").yellow());
            println!("\nWould modify:");
            println!("  Cargo.toml");
            println!("  nexus.toml");
            println!("  src/middleware/ratelimit.rs");
            return Ok(());
        }

        println!("Adding rate limiting support...");

        // Add dependencies
        self.add_dependency_to_cargo_toml(&[
            ("governor", "0.7", &[]),
            ("dashmap", "5", &[]),
        ])?;

        // Update nexus.toml
        let nexus_toml = std::fs::read_to_string("nexus.toml")
            .context("Failed to read nexus.toml")?;
        let nexus_toml = if nexus_toml.contains("[api]") {
            nexus_toml.replace("[api]\n", "[api]\nratelimit = true\n")
        } else {
            format!("{}\n[api]\nratelimit = true\n", nexus_toml)
        };
        std::fs::write("nexus.toml", nexus_toml)
            .context("Failed to write nexus.toml")?;

        // Create rate limiter middleware
        let ratelimit_rs = r#"use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;

pub struct RateLimiterState {
    pub limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimiterState {
    pub fn new() -> Self {
        let limiter = Arc::new(RateLimiter::direct(
            Quota::per_second(NonZeroU32::new(10).unwrap()),
        ));

        Self { limiter }
    }
}

pub async fn rate_limit_middleware(
    State(state): State<RateLimiterState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if state.limiter.check().is_err() {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}
"#;
        std::fs::write("src/middleware/ratelimit.rs", ratelimit_rs)
            .context("Failed to write rate limiter middleware")?;

        // Update middleware/mod.rs
        let mod_rs = if std::path::Path::new("src/middleware/auth.rs").exists() {
            if std::path::Path::new("src/middleware/cors.rs").exists() {
                r#"pub mod auth;
pub mod cors;
pub mod ratelimit;
"#
            } else {
                r#"pub mod auth;
pub mod ratelimit;
"#
            }
        } else if std::path::Path::new("src/middleware/cors.rs").exists() {
            r#"pub mod cors;
pub mod ratelimit;
"#
        } else {
            r#"pub mod ratelimit;
"#
        };
        std::fs::write("src/middleware/mod.rs", mod_rs)
            .context("Failed to write middleware/mod.rs")?;

        println!("  {} governor dependency", style("✓").green());
        println!("  {} dashmap dependency", style("✓").green());
        println!("  {} Configuration", style("✓").green());
        println!("  {} Rate limiter middleware", style("✓").green());

        Ok(())
    }
}
