use crate::app::state::AppState;
use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use hyper::StatusCode;
use redis::Commands;
use std::{
    any::type_name_of_val,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::error;

pub async fn rate_limiter(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
    request_limit_per_hour: u16,
) -> Result<Response, StatusCode> {
    let redis_client = state.redis_client;
    let ip = addr.ip().to_string();
    match is_rate_limited((*redis_client).clone(), ip, request_limit_per_hour).await {
        Ok(false) => Ok(next.run(request).await),
        Ok(true) => Err(StatusCode::TOO_MANY_REQUESTS),
        Err(e) => {
            error!("Error type: {}", type_name_of_val(&e));
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn is_rate_limited(
    redis_client: redis::Client,
    ip: String,
    request_limit_per_hour: u16,
) -> Result<bool, redis::RedisError> {
    let mut conn = redis_client.get_connection()?;
    let current_window = get_current_window()?;
    let bucket_key = format!("rate_limit:{ip}:{current_window}");

    let current_request_count: u64 = conn.get(&bucket_key).unwrap_or(0);
    if current_request_count >= u64::from(request_limit_per_hour) {
        return Ok(true);
    }

    let _: () = conn.incr(&bucket_key, 1)?;
    let _: () = conn.expire(&bucket_key, 3600)?;
    Ok(false)
}

fn get_current_window() -> Result<u64, redis::RedisError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() / 3600)
        .map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::Client,
                "Failed to calculate the current window in hours",
                format!("{e:?}"),
            ))
        })
}
