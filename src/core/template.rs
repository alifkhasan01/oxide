#![allow(dead_code)]

use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};

pub struct TemplateEngine {
    variables: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();
        for (key, value) in &self.variables {
            let placeholder = format!("{{{{ {} }}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }

    pub fn render_file(&self, template_path: &Path, output_path: &Path) -> Result<()> {
        let template = std::fs::read_to_string(template_path)
            .context(format!("Failed to read template: {}", template_path.display()))?;

        let rendered = self.render(&template);

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {}", parent.display()))?;
        }

        std::fs::write(output_path, rendered)
            .context(format!("Failed to write file: {}", output_path.display()))?;

        Ok(())
    }
}
