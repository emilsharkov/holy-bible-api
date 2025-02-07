use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AudioBibleQueryParams {
    pub language: Option<String>,
    pub version: Option<String>,
}