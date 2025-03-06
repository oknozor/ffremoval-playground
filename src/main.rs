use std::fs;
use crate::manager::InMemoryFeatureManager;

mod config;
mod manager;

pub static FEATURE_X_ENABLED: &str = "feature_x_enabled";

fn main() {
    let result = fs::read_to_string("tto").unwrap();
    let manager = serde_json::from_str::<InMemoryFeatureManager>(&result).unwrap();
    config::feature_x(manager);
}

