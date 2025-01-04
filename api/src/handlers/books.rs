use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::app::state::AppState;
use crate::models::sql::bible;
use crate::models::http::response;
use crate::models::http::params::books::BookPathParams;

pub async fn get_books(
    State(app_state): State<AppState>,
    Path(params): Path<BookPathParams>,
) -> Result<Json<response::books::GetBooksRes>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let rows: Vec<bible::Count> = sqlx::query_as(
        "SELECT count(*) FROM verses WHERE bible_id = $1"
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
        response::books::GetBooksRes {
            num_books: first_row.count,
        }
    ))
}