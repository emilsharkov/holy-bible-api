use axum::{extract::Path, extract::Query, extract::State, routing::get, Json, Router};

use crate::{
    app::state::AppState,
    controller::error::ServiceErrorExt,
    models::http::{
        params::bibles::{
            bible::BibleQueryParams,
            books::BookPathParams,
            chapters::ChaptersPathParams,
            verses::{VerseByNumberPathParams, VersesPathParams, VersesQueryParams},
        },
        response::bibles::{
            bibles::GetBiblesRes,
            books::GetBibleBooksRes,
            chapters::GetBibleChaptersRes,
            verses::{BibleVerse, GetBibleVersesRes},
        },
    },
};
use std::i32;

pub fn get_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/bibles", get(get_bibles))
        .route("/bibles/:bible_id/books", get(get_bible_books))
        .route(
            "/bibles/:bible_id/books/:book_num/chapters",
            get(get_bible_chapters),
        )
        .route(
            "/bibles/:bible_id/books/:book_num/chapters/:chapter_num/verses",
            get(get_bible_verses),
        )
        .route(
            "/bibles/:bible_id/books/:book_num/chapters/:chapter_num/verses/:verse_num",
            get(get_bible_verse_by_number),
        )
}

#[utoipa::path(
    get,
    path = "/bibles",
    params(BibleQueryParams),
    responses(
        (status = 200, body = GetBiblesRes)
    )
)]
pub async fn get_bibles(
    State(app_state): State<AppState>,
    Query(params): Query<BibleQueryParams>,
) -> Result<Json<GetBiblesRes>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let bibles = bible_service
        .get_bibles(params.language, params.version)
        .await
        .into_axum_response()?;
    Ok(Json(GetBiblesRes { bibles }))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books",
    params(BookPathParams),
    responses(
        (status = 200, body = GetBibleBooksRes)
    )
)]
pub async fn get_bible_books(
    State(app_state): State<AppState>,
    Path(params): Path<BookPathParams>,
) -> Result<Json<GetBibleBooksRes>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let num_books = bible_service
        .get_bible_books(params.bible_id)
        .await
        .into_axum_response()?;
    Ok(Json(GetBibleBooksRes { num_books }))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books/{book_num}/chapters",
    params(ChaptersPathParams),
    responses(
        (status = 200, body = GetBibleChaptersRes)
    )
)]
pub async fn get_bible_chapters(
    State(app_state): State<AppState>,
    Path(params): Path<ChaptersPathParams>,
) -> Result<Json<GetBibleChaptersRes>, axum::response::Response> {
    let bible_service = &app_state.bible_service;
    let num_chapters = bible_service
        .get_bible_chapters(params.bible_id, params.book_num)
        .await
        .into_axum_response()?;
    Ok(Json(GetBibleChaptersRes { num_chapters }))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses",
    params(VersesQueryParams, VersesPathParams),
    responses(
        (status = 200, body = GetBibleVersesRes)
    )
)]
pub async fn get_bible_verses(
    State(app_state): State<AppState>,
    Query(query): Query<VersesQueryParams>,
    Path(path): Path<VersesPathParams>,
) -> Result<Json<GetBibleVersesRes>, axum::response::Response> {
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
    Ok(Json(GetBibleVersesRes { verses }))
}

#[utoipa::path(
    get,
    path = "/bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses/{verse_num}",
    params(VerseByNumberPathParams),
    responses(
        (status = 200, body = BibleVerse)
    )
)]
pub async fn get_bible_verse_by_number(
    State(app_state): State<AppState>,
    Path(path): Path<VerseByNumberPathParams>,
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
