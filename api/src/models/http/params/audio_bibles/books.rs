use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookPathParams {
    pub audio_bible_id: i32,
}