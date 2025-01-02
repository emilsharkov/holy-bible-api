use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookQueryParams {
    testament: Option<String>,
}