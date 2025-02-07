use axum::{extract::{Path, Query, State}, Json};

use crate::app::state::AppState;

pub async fn get_chapters(
    State(app_state): State<AppState>,
    Path(params): Path<()>
) -> Result<Json<()>, axum::response::Response> {
    Ok(Json(()))
}

pub async fn get_audio_chapter(
    State(app_state): State<AppState>,
    Path(params): Path<()>
) -> Result<Json<()>, axum::response::Response> {
    Ok(Json(()))
}