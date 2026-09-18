#![allow(dead_code)]

use crate::core::stack::Dependency;

pub fn jwt_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "jsonwebtoken",
            version: "9",
            features: &[],
        },
        Dependency {
            name: "argon2",
            version: "0.5",
            features: &["std"],
        },
        Dependency {
            name: "rand",
            version: "0.8",
            features: &["std"],
        },
    ]
}

pub fn session_dependencies() -> Vec<Dependency> {
    // TODO: Implement session authentication
    vec![]
}
