use serde::Deserialize;

pub trait FeatureManager {
    fn is_enabled(&self, flag: &str) -> bool;
}

#[derive(Deserialize)]
pub struct InMemoryFeatureManager {
    features: Vec<String>,
}

impl FeatureManager for InMemoryFeatureManager {
    fn is_enabled(&self, flag: &str) -> bool {
        self.features.iter().find(|f| *f == flag).is_some()
    }
}