use axum::extract::State;
use sqlx::{FromRow, PgPool};

use crate::app::state::AppState;

#[derive(Debug, FromRow)]
struct Bible {
    id: i32,
    name: String,
}

pub async fn get_bibles(State(app_state): State<AppState>) -> Result<String, axum::response::Response> {
    let db_pool: &PgPool = &app_state.db_pool;

    let rows: Vec<Bible> = sqlx::query_as("SELECT id, name FROM bibles")
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
        .map(|bible| format!("{}: {}", bible.id, bible.name))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(result)
}
