use axum::extract::{Path, Query, State};
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::http::params::books::BookQueryParams;
use crate::models::sql::bible;

pub async fn get_books(
    State(app_state): State<AppState>,
    Path(bible_id): Path<i32>,
) -> Result<String, axum::response::Response> {
    let db_pool: &PgPool = &app_state.db_pool;

    let rows: Vec<bible::Verse> = sqlx::query_as("SELECT * FROM verses WHERE bible_id = $1")
        .bind(bible_id)
        .fetch_all(db_pool)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .unwrap()
        })?;

    let result = rows
        .into_iter()
        .map(|bible| format!("{}: {}", bible.bible_id, bible.verse))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(result)
}