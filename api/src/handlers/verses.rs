use std::i32;

use axum::extract::{Path, Query, State};
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::http::params::verses::{VerseByNumberPathParams, VersesPathParams, VersesQueryParams};
use crate::models::sql::bible;

pub async fn get_verses(
    State(app_state): State<AppState>,
    Query(query): Query<VersesQueryParams>,
    Path(path): Path<VersesPathParams>,
) -> Result<String, axum::response::Response> {
    let db_pool: &PgPool = &app_state.db_pool;

    let start = query.start.unwrap_or(1);
    let end = query.end.unwrap_or(i32::MAX);

    if start > end {
        return Err(
            axum::response::Response::builder()
                .status(400)
                .body("Invalid range".into())
                .unwrap(),
        );
    }

    let rows: Vec<bible::Verse> = sqlx::query_as(
        "SELECT * FROM verses 
            WHERE bible_id = $1 
            AND book_num = $2 
            AND chapter_num = $3" 
    )
        .bind(path.bible_id)
        .bind(path.book_num)
        .bind(path.chapter_num)
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

pub async fn get_verse_by_number(
    State(app_state): State<AppState>,
    Path(path): Path<VerseByNumberPathParams>,
) -> Result<String, axum::response::Response> {
    let db_pool: &PgPool = &app_state.db_pool;

    let rows: Vec<bible::Verse> = sqlx::query_as(
        "SELECT * FROM verses 
            WHERE bible_id = $1 
            AND book_num = $2 
            AND chapter_num = $3 
            AND verse_num = $4"
    )
        .bind(path.bible_id)
        .bind(path.book_num)
        .bind(path.chapter_num)
        .bind(path.verse_num)
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