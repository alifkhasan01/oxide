#![allow(dead_code)]

use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    Unknown,
}

impl Platform {
    pub fn detect() -> Self {
        match env::consts::OS {
            "linux" => Platform::Linux,
            "macos" => Platform::MacOS,
            "windows" => Platform::Windows,
            _ => Platform::Unknown,
        }
    }

    pub fn is_unix(&self) -> bool {
        matches!(self, Platform::Linux | Platform::MacOS)
    }

    pub fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }

    pub fn path_separator(&self) -> &str {
        if self.is_windows() {
            "\\"
        } else {
            "/"
        }
    }

    pub fn exe_extension(&self) -> &str {
        if self.is_windows() {
            ".exe"
        } else {
            ""
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOS => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
            Platform::Unknown => write!(f, "Unknown"),
        }
    }
}
