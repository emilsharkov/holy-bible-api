use std::error::Error;

use axum::{extract::{Request, State}, http::Extensions, middleware::Next, response::Response};
use hyper::{HeaderMap, StatusCode};

use crate::{app::state::AppState, config::settings::RedisSettings};


pub async fn rate_limiter(
    State(state): State<AppState>,
    headers: HeaderMap,
    extensions: Extensions,
    request: Request,
    next: Next,
    settings: u16
) -> Result<Response,StatusCode> {
    Err(StatusCode::TOO_MANY_REQUESTS)
}


// async fn is_rate_limited() -> Result<bool, Error>{
//     Ok(false)
// }