use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct ChaptersPathParams {
    pub audio_bible_id: i32,
    pub book_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct AudioChapterPathParams {
    pub audio_bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
}
