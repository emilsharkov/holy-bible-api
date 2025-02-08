use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize,ToSchema)]
pub struct AudioBible {
    pub audio_bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(Serialize,ToSchema)]
pub struct GetAudioBiblesRes {
    pub audio_bibles: Vec<AudioBible>,
}