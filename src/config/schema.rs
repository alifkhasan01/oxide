#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub framework: String,
    pub database: String,
    pub auth: String,
    pub cache: String,
    pub api_docs: String,
}

impl ProjectConfig {
    pub fn to_nexus_toml(&self) -> String {
        format!(
            r#"[project]
name = "{}"
version = "0.1.0"
framework = "{}"
edition = "2021"

[database]
provider = "{}"

[auth]
provider = "{}"

[cache]
provider = "{}"

[tool.oxide]
version = "{}""#,
            self.name,
            self.framework.to_lowercase(),
            self.database.to_lowercase(),
            self.auth.to_lowercase(),
            self.cache.to_lowercase(),
            env!("CARGO_PKG_VERSION")
        )
    }
}
