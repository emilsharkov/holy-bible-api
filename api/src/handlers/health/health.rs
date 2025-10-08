use axum::Json;

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
