use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema)]
#[derive(IntoParams)]
#[derive(Deserialize, Serialize)]
pub struct BibleQueryParams {
    #[param(in = Query)]
    pub language: Option<String>,
    
    #[param(in = Query)]
    pub version: Option<String>,
}