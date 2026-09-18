#![allow(dead_code)]

use crate::core::stack::Dependency;

pub fn postgres_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "sqlx",
            version: "0.8",
            features: &["runtime-tokio", "tls-rustls", "postgres", "chrono", "uuid"],
        },
        Dependency {
            name: "chrono",
            version: "0.4",
            features: &["serde"],
        },
        Dependency {
            name: "uuid",
            version: "1",
            features: &["v4", "serde"],
        },
    ]
}

pub fn mysql_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "sqlx",
            version: "0.8",
            features: &["runtime-tokio", "tls-rustls", "mysql", "chrono", "uuid"],
        },
        Dependency {
            name: "chrono",
            version: "0.4",
            features: &["serde"],
        },
        Dependency {
            name: "uuid",
            version: "1",
            features: &["v4", "serde"],
        },
    ]
}

pub fn sqlite_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "sqlx",
            version: "0.8",
            features: &["runtime-tokio", "tls-rustls", "sqlite", "chrono", "uuid"],
        },
        Dependency {
            name: "chrono",
            version: "0.4",
            features: &["serde"],
        },
        Dependency {
            name: "uuid",
            version: "1",
            features: &["v4", "serde"],
        },
    ]
}

pub fn postgres_config(db_name: &str) -> String {
    format!(
        r#"use sqlx::postgres::{db_name}Pool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {{
    sqlx::postgres::PgPool::connect(database_url).await
}}
"#
    )
}
