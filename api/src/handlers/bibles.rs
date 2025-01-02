use axum::extract::{Path, Query, State};
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::http::params::bible::{BibleQueryParams, BibleSearchQueryParams};
use crate::models::sql::bible;

pub async fn get_bibles(
    State(app_state): State<AppState>,
    Query(params): Query<BibleQueryParams>
) -> Result<String, axum::response::Response> {
    let db_pool: &PgPool = &app_state.db_pool;

    let rows: Vec<bible::Bible> = sqlx::query_as("SELECT * FROM bible")
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
        .map(|bible| format!("{}: {} {}", bible.bible_id, bible.language, bible.version.unwrap()))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(result)
}

pub async fn get_verse_by_search(
    State(app_state): State<AppState>,
    Path(bible_id): Path<i32>,
    Query(params): Query<BibleSearchQueryParams> 
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
        .map(|verse| format!("{}: {}", verse.bible_id, verse.verse))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(result)
}