use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Bible {
    pub bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(Debug, FromRow)]
pub struct Verse {
    pub bible_id: i32,
    pub book: i32,
    pub chapter: i32,
    pub verse: i32,
    pub text: String
}

#[derive(Debug, FromRow)]
pub struct Count {
    pub count: i32,
}