use crate::core::stack::Dependency;

pub fn dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "axum",
            version: "0.8",
            features: &[],
        },
        Dependency {
            name: "tokio",
            version: "1",
            features: &["full"],
        },
        Dependency {
            name: "tower",
            version: "0.5",
            features: &[],
        },
        Dependency {
            name: "tower-http",
            version: "0.6",
            features: &["cors", "trace"],
        },
        Dependency {
            name: "serde",
            version: "1",
            features: &["derive"],
        },
        Dependency {
            name: "serde_json",
            version: "1",
            features: &[],
        },
        Dependency {
            name: "tracing",
            version: "0.1",
            features: &[],
        },
        Dependency {
            name: "tracing-subscriber",
            version: "0.3",
            features: &["env-filter"],
        },
        Dependency {
            name: "dotenvy",
            version: "0.15",
            features: &[],
        },
        Dependency {
            name: "anyhow",
            version: "1",
            features: &[],
        },
        Dependency {
            name: "config",
            version: "0.14",
            features: &[],
        },
    ]
}
