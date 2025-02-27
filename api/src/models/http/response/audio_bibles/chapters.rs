use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetAudioChaptersRes {
    pub num_chapters: i64,
}