use crate::core::stack::Dependency;

pub fn dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "redis",
            version: "0.26",
            features: &["tokio-comp", "connection-manager"],
        },
    ]
}
