use crate::FEATURE_X_ENABLED;
use crate::manager::{FeatureManager, InMemoryFeatureManager};

pub fn feature_x(manager: InMemoryFeatureManager) {
    if manager.is_enabled(FEATURE_X_ENABLED) {
        println!("Feature X is enabled!");
    } else {
        println!("Feature X is disabled.");
    }
}