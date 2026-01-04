use std::time::Duration;

use axum::http::{HeaderName, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use crate::config::settings::CorsSettings;

pub fn get_cors_layer(settings: &CorsSettings) -> CorsLayer {
    let allowed_methods = [
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ];

    let allowed_headers = [
        HeaderName::from_static("content-type"),
        HeaderName::from_static("authorization"),
        HeaderName::from_static("accept"),
        HeaderName::from_static("x-requested-with"),
    ];

    let mut cors_layer = CorsLayer::new()
        .allow_methods(allowed_methods)
        .allow_headers(allowed_headers)
        .max_age(Duration::from_secs(60 * 60));

    if settings.allow_any_origin {
        // Cannot use credentials with wildcard origin
        cors_layer = cors_layer.allow_origin(Any);
    } else if let Some(origins) = build_origin_list(&settings.allowed_origins) {
        // Can use credentials with explicit origins
        cors_layer = cors_layer
            .allow_origin(AllowOrigin::list(origins))
            .allow_credentials(true);
    }

    cors_layer
}

fn build_origin_list(origins: &[String]) -> Option<Vec<HeaderValue>> {
    let parsed_origins: Vec<_> = origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin.trim()).ok())
        .collect();

    if parsed_origins.is_empty() {
        None
    } else {
        Some(parsed_origins)
    }
}
