use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct AudioBible {
    pub audio_bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(Debug, FromRow)]
pub struct Count {
    pub count: i64,
}