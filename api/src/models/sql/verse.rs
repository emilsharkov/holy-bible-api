use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Verse {
    pub bible_id: i32,
    pub testament: String,
    pub book: i32,
    pub chapter: i32,
    pub verse: i32,
    pub text: String,
}