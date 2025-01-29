use axum::{routing::get, Json, Router};
use crate::app::state::AppState;

pub fn get_health_route() -> Router<AppState> {
    Router::new().route(
        "/health",
        get(|| async {
            Json("Hello, health!")
        }),
    )
}
