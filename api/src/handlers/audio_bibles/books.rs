use axum::{extract::{Path, State}, response::Response, Json};

use crate::{app::state::AppState, models::{http::{params::audio_bibles::books::BookPathParams, response::audio_bibles::books::GetBooksRes}, sql::audio_bibles}};

pub async fn get_books(
    State(app_state): State<AppState>,
    Path(params): Path<BookPathParams>
) -> Result<Json<GetBooksRes>, axum::response::Response> {
    let db_client = &*app_state.db_client;
    let rows: Vec<audio_bibles::Count> = sqlx::query_as(
    "SELECT COUNT(distinct book) 
        FROM audio_bible_chapters 
        WHERE audio_bible_id = $1"
    )
        .bind(params.audio_bible_id)
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;
    
    let num_books = rows
        .into_iter()
        .next()
        .ok_or_else(|| {
            Response::builder()
                .status(404)
                .body("Audio Bible not found".into())
                .expect("axum response builder failed")
        })?
        .count;

    Ok(Json(GetBooksRes { num_books }))
}