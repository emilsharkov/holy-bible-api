use axum::extract::State;
use sqlx::{FromRow, PgPool};
use tracing::{event, Level};
use crate::app::state::AppState;
use crate::models::sql::bible;

pub async fn get_bibles(State(app_state): State<AppState>) -> Result<String, axum::response::Response> {
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
