use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChaptersPathParams {
    pub audio_bible_id: i32,
    pub book_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct AudioChapterPathParams {
    pub audio_bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
}