use tower::timeout::TimeoutLayer;
use std::time::Duration;

pub fn get_timeout_layer(timeout_seconds: u64) -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(timeout_seconds))
}
