use axum::routing::Route;
use tower::{timeout::TimeoutLayer, Layer};
use std::{convert::Infallible, time::Duration};

pub fn get_timeout_layer(timeout_seconds: u64) -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(timeout_seconds))
}
