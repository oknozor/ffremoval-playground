pub static FEATURE_X_ENABLED: bool = true;  // This is the flag we want to remove later

pub fn feature_x() {
    if FEATURE_X_ENABLED {
        println!("Feature X is enabled!");
    } else {
        println!("Feature X is disabled.");
    }
}