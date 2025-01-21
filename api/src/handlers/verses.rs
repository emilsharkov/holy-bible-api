use std::i32;

use axum::extract::{Path, Query, State};
use axum::Json;
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::http::params::verses::{VerseByNumberPathParams, VersesPathParams, VersesQueryParams};
use crate::models::sql::bible;
use crate::models::http::response;

pub async fn get_verses(
    State(app_state): State<AppState>,
    Query(query): Query<VersesQueryParams>,
    Path(path): Path<VersesPathParams>,
) -> Result<Json<response::verses::GetVersesRes>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let start = query.start.unwrap_or(1);
    let end = query.end.unwrap_or(i32::MAX);

    if start > end {
        return Err(
            axum::response::Response::builder()
                .status(400)
                .body("Invalid range".into())
                .expect("axum response builder failed"),
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
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;

    let result = rows
        .into_iter()
        .map(|bible| -> response::verses::Verse {
            response::verses::Verse {
                bible_id: bible.bible_id,
                book: bible.book,
                chapter: bible.chapter,
                verse: bible.verse,
                text: bible.text,
            }
        })
        .collect::<Vec<response::verses::Verse>>();

    Ok(Json(
        response::verses::GetVersesRes {
            verses: result,
        }
    ))
}

pub async fn get_verse_by_number(
    State(app_state): State<AppState>,
    Path(path): Path<VerseByNumberPathParams>,
) -> Result<Json<response::verses::Verse>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

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
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;

    let first_row = rows.into_iter().next().ok_or_else(|| {
        axum::response::Response::builder()
            .status(404)
            .body("Verse not found".into())
            .expect("axum response builder failed")
    })?;

    Ok(Json(response::verses::Verse {
        bible_id: first_row.bible_id,
        book: first_row.book,
        chapter: first_row.chapter,
        verse: first_row.verse,
        text: first_row.text.clone(),
    }))
}