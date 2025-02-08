use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetBibleChaptersRes {
    pub num_chapters: i64,
}
