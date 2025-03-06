use crate::FEATURE_X_ENABLED;

pub fn feature_x() {
    if FEATURE_X_ENABLED {
        println!("Feature X is enabled!");
    } else {
        println!("Feature X is disabled.");
    }
}