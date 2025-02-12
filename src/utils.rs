use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current time.
pub fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}
