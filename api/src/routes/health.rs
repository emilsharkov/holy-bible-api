use crate::{app::state::AppState, handlers::health::health::get_health};
use axum::{routing::get, Router};

pub fn get_health_route() -> Router<AppState> {
    Router::new().route("/health", get(get_health))
}
