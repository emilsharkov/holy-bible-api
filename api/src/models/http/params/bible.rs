use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BibleQueryParams {
    pub language: Option<String>,
    pub version: Option<String>,
}
