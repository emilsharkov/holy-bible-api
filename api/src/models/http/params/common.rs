use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Query parameters for filtering bibles or audio bibles by language and version
#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
#[into_params(parameter_in = Query)]
pub struct BibleQuery {
    pub language: Option<String>,
    pub version: Option<String>,
}

