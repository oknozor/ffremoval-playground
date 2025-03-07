use std::fs;
use crate::manager::InMemoryFeatureManager;

mod config;
mod manager;

fn main() {
    let result = fs::read_to_string("tto").unwrap();
    let manager = serde_json::from_str::<InMemoryFeatureManager>(&result).unwrap();
config::do_stuff(manager);
}
