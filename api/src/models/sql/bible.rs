use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Bible {
    pub bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}