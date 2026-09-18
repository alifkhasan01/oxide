#![allow(dead_code)]

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .context(format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    std::fs::write(path, content)
        .context(format!("Failed to write file: {}", path.display()))?;
    Ok(())
}

pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    ensure_dir(dst)?;

    for entry in std::fs::read_dir(src)
        .context(format!("Failed to read directory: {}", src.display()))?
    {
        let entry = entry.context("Failed to read directory entry")?;
        let file_type = entry.file_type().context("Failed to get file type")?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)
                .context(format!("Failed to copy file: {}", src_path.display()))?;
        }
    }

    Ok(())
}

pub fn project_root() -> Result<PathBuf> {
    let mut path = std::env::current_dir().context("Failed to get current directory")?;
    loop {
        if path.join("Cargo.toml").exists() {
            return Ok(path);
        }
        if !path.pop() {
            anyhow::bail!("Not in a Rust project directory");
        }
    }
}
