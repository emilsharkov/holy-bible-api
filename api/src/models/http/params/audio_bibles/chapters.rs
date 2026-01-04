use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct ChaptersPath {
    pub audio_bible_id: i32,
    pub book_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct AudioChapterPath {
    pub audio_bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
}
