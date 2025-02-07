use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersesQueryParams {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Deserialize)]
pub struct VersesPathParams {
    pub bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
}

#[derive(Deserialize)]
pub struct VerseByNumberPathParams {
    pub bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
    pub verse_num: i32,
}
