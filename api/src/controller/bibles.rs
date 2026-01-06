use axum::{extract::Path, extract::Query, extract::State, routing::get, Json, Router};

use crate::{
    app::state::AppState,
    controller::error::ServiceErrorExt,
    models::http::{
        params::{
            bibles::{
                books::BookPath,
                chapters::ChaptersPath,
                verses::{RandomBibleVersePath, VerseByNumberPath, VerseOfTheDayPath, VersesPath, VersesQuery},
            },
            common::BibleQuery,
        },
        response::{
            bibles::{
                bibles::Bible,
                verses::BibleVerse
            },
            common::{BooksCountResponse, ChaptersCountResponse},
        },
    },
};
use std::i32;

pub fn get_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/bibles", get(get_bibles))
        .route("/bibles/{bible_id}/books", get(get_bible_books))
        .route(
            "/bibles/{bible_id}/books/{book_num}/chapters",
            get(get_bible_chapters),
        )
        .route(
            "/bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses",
            get(get_bible_verses),
        )
        .route(
            "/bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses/{verse_num}",
            get(get_bible_verse_by_number),
        )
        .route(
            "/bibles/{bible_id}/random",
            get(get_random_bible_verse),
        )
        .route(
            "/bibles/{bible_id}/verse-of-the-day",
            get(get_verse_of_the_day),
        )
}

#[utoipa::path(
    get,
    path = "/bibles",
    params(BibleQuery),
    responses(
        (status = 200, body = Vec<Bible>)
    )
)]
pub async fn get_bibles(
    State(app_state): State<AppState>,
    Query(params): Query<BibleQuery>,
) -> Result<Json<Vec<Bible>>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let bibles = bible_service
        .get_bibles(params.language, params.version)
        .await
        .into_axum_response()?;
    Ok(Json(bibles))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books",
    params(BookPath),
    responses(
        (status = 200, body = BooksCountResponse)
    )
)]
pub async fn get_bible_books(
    State(app_state): State<AppState>,
    Path(params): Path<BookPath>,
) -> Result<Json<BooksCountResponse>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let num_books = bible_service
        .get_bible_books(params.bible_id)
        .await
        .into_axum_response()?;
    Ok(Json(BooksCountResponse { num_books }))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books/{book_num}/chapters",
    params(ChaptersPath),
    responses(
        (status = 200, body = ChaptersCountResponse)
    )
)]
pub async fn get_bible_chapters(
    State(app_state): State<AppState>,
    Path(params): Path<ChaptersPath>,
) -> Result<Json<ChaptersCountResponse>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let num_chapters = bible_service
        .get_bible_chapters(params.bible_id, params.book_num)
        .await
        .into_axum_response()?;
    Ok(Json(ChaptersCountResponse { num_chapters }))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses",
    params(VersesQuery, VersesPath),
    responses(
        (status = 200, body = Vec<BibleVerse>)
    )
)]
pub async fn get_bible_verses(
    State(app_state): State<AppState>,
    Query(query): Query<VersesQuery>,
    Path(path): Path<VersesPath>,
) -> Result<Json<Vec<BibleVerse>>, axum::response::Response> {
    let start = query.start.unwrap_or(1);
    let end = query.end.unwrap_or(i32::MAX);

    if start > end {
        return Err(axum::response::Response::builder()
            .status(400)
            .body("Invalid range".into())
            .expect("axum response builder failed"));
    }

    let bible_service = &app_state.bible_service;
    let verses = bible_service
        .get_bible_verses(path.bible_id, path.book_num, path.chapter_num, start, end)
        .await
        .into_axum_response()?;
    Ok(Json(verses))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses/{verse_num}",
    params(VerseByNumberPath),
    responses(
        (status = 200, body = BibleVerse)
    )
)]
pub async fn get_bible_verse_by_number(
    State(app_state): State<AppState>,
    Path(path): Path<VerseByNumberPath>,
) -> Result<Json<BibleVerse>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let verse = bible_service
        .get_bible_verse_by_number(
            path.bible_id,
            path.book_num,
            path.chapter_num,
            path.verse_num,
        )
        .await
        .into_axum_response()?;
    Ok(Json(verse))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/random",
    params(RandomBibleVersePath),
    responses(
        (status = 200, body = BibleVerse)
    )
)]
pub async fn get_random_bible_verse(
    State(app_state): State<AppState>,
    Path(path): Path<RandomBibleVersePath>,
) -> Result<Json<BibleVerse>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let verse = bible_service
        .get_random_bible_verse(path.bible_id, None)
        .await
        .into_axum_response()?;
    Ok(Json(verse))
}

// verse of the day
#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/verse-of-the-day",
    params(VerseOfTheDayPath),
    responses(
        (status = 200, body = BibleVerse)
    )
)]
pub async fn get_verse_of_the_day(
    State(app_state): State<AppState>,
    Path(path): Path<VerseOfTheDayPath>,
) -> Result<Json<BibleVerse>, axum::response::Response> {
    // Generate seed based on current date (days since epoch) to ensure same verse for the day
    let days_since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() / 86400; // Convert to days
    
    let bible_service = &app_state.bible_service;
    let verse = bible_service
        .get_random_bible_verse(path.bible_id, Some(days_since_epoch as f64))
        .await
        .into_axum_response()?;
    Ok(Json(verse))
}