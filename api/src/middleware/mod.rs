pub mod rate_limiter;
pub mod timeout;
pub mod trace;

use axum::error_handling::HandleErrorLayer;
use axum::Router;
use tracing::error;
use crate::config::settings::{MiddlewareSettings, Settings};

use std::time::Duration;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use axum::http::StatusCode;
use tower::BoxError;


pub fn get_middleware_stack(
    router: axum::Router,
    settings: &Settings,
) -> Router {
    router
    .layer(
        ServiceBuilder::new()
        
            .layer(trace::get_trace_layer())
            .layer(rate_limiter::get_rate_limiter_layer(
                settings.middleware_settings.rate_limiter_per_second,
                settings.middleware_settings.rate_limiter_burst_size,
            ))
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                error!("some error");
                StatusCode::REQUEST_TIMEOUT
            }))
            
    )
}
