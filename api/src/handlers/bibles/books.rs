use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::http::response::bibles::books::GetBibleBooksRes;
use crate::models::sql::bible;
use crate::models::http::params::bibles::books::BookPathParams;

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
    let db_client: &PgPool = &app_state.db_client;

    let rows: Vec<bible::Count> = sqlx::query_as(
        "SELECT count(distinct book) FROM verses WHERE bible_id = $1"
    )
        .bind(params.bible_id)
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

    Ok(Json(
        GetBibleBooksRes {
            num_books: first_row.count,
        }
    ))
}