use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BibleQueryParams {
    pub language: Option<String>,
    pub version: Option<String>,
}