use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<sqlx::PgPool>,
}
