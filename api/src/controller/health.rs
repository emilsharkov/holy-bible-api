use crate::app::state::AppState;
use axum::{routing::get, Json, Router};

pub fn get_health_route() -> Router<AppState> {
    Router::new().route("/health", get(get_health))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, body = String)
    )
)]
pub async fn get_health() -> Result<Json<String>, axum::response::Response> {
    Ok(Json("Healthy!".to_string()))
}
