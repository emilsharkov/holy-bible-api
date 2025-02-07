use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetVersesRes {
    pub verses: Vec<Verse>
}

#[derive(Debug, Serialize)]
pub struct Verse {
    pub bible_id: i32,
    pub book: i32,
    pub chapter: i32,
    pub verse: i32,
    pub text: String
}