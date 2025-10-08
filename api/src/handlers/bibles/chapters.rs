use crate::app::state::AppState;
use crate::models::http::params::bibles::chapters::ChaptersPathParams;
use crate::models::http::response::bibles::chapters::GetBibleChaptersRes;
use crate::models::sql::bible;
use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;

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
    let db_client: &PgPool = &app_state.db_client;

    let rows: Vec<bible::Count> = sqlx::query_as(
        "SELECT count(distinct chapter) FROM verses WHERE bible_id = $1 AND book = $2",
    )
    .bind(params.bible_id)
    .bind(params.book_num)
    .fetch_all(db_client)
    .await
    .map_err(|err| {
        axum::response::Response::builder()
            .status(500)
            .body(format!("Database query failed: {err}").into())
            .expect("axum response builder failed")
    })?;

    let first_row = rows.into_iter().next().ok_or_else(|| {
        axum::response::Response::builder()
            .status(404)
            .body("Verse not found".into())
            .expect("axum response builder failed")
    })?;

    Ok(Json(GetBibleChaptersRes {
        num_chapters: first_row.count,
    }))
}
