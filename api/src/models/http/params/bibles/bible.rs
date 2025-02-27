use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
#[into_params(parameter_in = Query)]
pub struct BibleQueryParams {
    pub language: Option<String>,
    pub version: Option<String>,
}