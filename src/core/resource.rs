#![allow(dead_code)]

use std::path::Path;
use anyhow::{Context, Result};
use console::style;

pub struct ResourceGenerator;

impl ResourceGenerator {
    pub fn generate(project_path: &Path, name: &str, crud: bool) -> Result<()> {
        println!("{}", style(format!("Generating resource '{}'...\n", name)).cyan());

        // Generate model
        Self::generate_model(project_path, name)?;

        // Generate handler
        Self::generate_handler(project_path, name)?;

        // Generate service
        Self::generate_service(project_path, name)?;

        // Generate repository
        Self::generate_repository(project_path, name)?;

        // Generate routes
        Self::generate_routes(project_path, name, crud)?;

        println!("  {} Model", style("✓").green());
        println!("  {} Handler", style("✓").green());
        println!("  {} Service", style("✓").green());
        println!("  {} Repository", style("✓").green());
        println!("  {} Routes", style("✓").green());

        Ok(())
    }

    fn generate_model(project_path: &Path, name: &str) -> Result<()> {
        let struct_name = to_pascal_case(name);
        let model_rs = format!(
            r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {struct_name} {{
    pub id: i32,
    // Add your fields here
}}

impl {struct_name} {{
    pub fn new(id: i32) -> Self {{
        Self {{ id }}
    }}
}}
"#
        );

        std::fs::write(project_path.join(format!("src/models/{}.rs", name)), model_rs)
            .context("Failed to write model file")?;

        Ok(())
    }

    fn generate_handler(project_path: &Path, name: &str) -> Result<()> {
        let handler_rs = format!(
            r#"use axum::{{extract::State, Json}};
use crate::error::AppError;
use crate::state::AppState;

pub async fn list(State(_state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, AppError> {{
    // TODO: Implement list handler
    Ok(Json(vec![]))
}}

pub async fn get(State(_state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {{
    // TODO: Implement get handler
    Ok(Json(serde_json::json!({{}})))
}}

pub async fn create(State(_state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {{
    // TODO: Implement create handler
    Ok(Json(serde_json::json!({{}})))
}}

pub async fn update(State(_state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {{
    // TODO: Implement update handler
    Ok(Json(serde_json::json!({{}})))
}}

pub async fn delete(State(_state): State<AppState>) -> Result<(), AppError> {{
    // TODO: Implement delete handler
    Ok(())
}}
"#
        );

        std::fs::write(project_path.join(format!("src/handlers/{}.rs", name)), handler_rs)
            .context("Failed to write handler file")?;

        Ok(())
    }

    fn generate_service(project_path: &Path, name: &str) -> Result<()> {
        let struct_name = to_pascal_case(name);
        let service_rs = format!(
            r#"use crate::error::AppError;

pub struct {struct_name}Service;

impl {struct_name}Service {{
    pub fn new() -> Self {{
        Self
    }}

    // Add your service methods here
}}
"#
        );

        std::fs::write(project_path.join(format!("src/services/{}.rs", name)), service_rs)
            .context("Failed to write service file")?;

        Ok(())
    }

    fn generate_repository(project_path: &Path, name: &str) -> Result<()> {
        let struct_name = to_pascal_case(name);
        let repository_rs = format!(
            r#"use crate::error::AppError;

pub struct {struct_name}Repository;

impl {struct_name}Repository {{
    pub fn new() -> Self {{
        Self
    }}

    // Add your repository methods here
}}
"#
        );

        std::fs::write(project_path.join(format!("src/repositories/{}.rs", name)), repository_rs)
            .context("Failed to write repository file")?;

        Ok(())
    }

    fn generate_routes(project_path: &Path, name: &str, crud: bool) -> Result<()> {
        let routes_rs = if crud {
            format!(
                r#"use axum::{{Router, routing::{{get, post, put, delete}}}};
use crate::handlers::{name};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {{
    Router::new()
        .route("/{name}s", get({name}::list).post({name}::create))
        .route("/{name}s/:id", get({name}::get).put({name}::update).delete({name}::delete))
        .with_state(state)
}}
"#
            )
        } else {
            format!(
                r#"use axum::Router;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {{
    Router::new()
        .with_state(state)
}}
"#
            )
        };

        std::fs::write(project_path.join(format!("src/routes/{}.rs", name)), routes_rs)
            .context("Failed to write routes file")?;

        Ok(())
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let rest: String = chars.collect::<String>().to_lowercase();
                    format!("{}{}", first.to_uppercase(), rest)
                }
            }
        })
        .collect()
}
