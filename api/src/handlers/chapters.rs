use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::http::params::chapters::ChaptersPathParams;
use crate::models::sql::bible;
use crate::models::http::response;

pub async fn get_chapters(
    State(app_state): State<AppState>,
    Path(params): Path<ChaptersPathParams>,
) -> Result<Json<response::chapters::GetChaptersRes>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let rows: Vec<bible::Count> = sqlx::query_as(
        "SELECT count(*) FROM verses WHERE bible_id = $1 AND book_num = $2"
    )
        .bind(params.bible_id)
        .bind(params.book_num)
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .unwrap()
        })?;

    let first_row = rows.into_iter().next().ok_or_else(|| {
        axum::response::Response::builder()
            .status(404)
            .body("Verse not found".into())
            .unwrap()
    })?;

    Ok(Json(
        response::chapters::GetChaptersRes {
            num_chapters: first_row.count,
        }
    ))
}