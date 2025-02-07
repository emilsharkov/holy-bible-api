use serde::Serialize;

#[derive(Serialize)]
pub struct AudioBible {
    pub audio_bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(Serialize)]
pub struct GetAudioBibleRes {
    pub audio_bibles: Vec<AudioBible>,
}