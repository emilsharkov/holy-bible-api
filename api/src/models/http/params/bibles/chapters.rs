use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct ChaptersPath {
    pub bible_id: i32,
    pub book_num: i32,
}
