use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::error::Error;

pub fn handle_error(err: Box<dyn Error>) -> Response {
    match err {
        _ => {
            // Unhandled error
            tracing::error!("Unhandled error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub trait ServiceErrorExt<T> {
    fn into_axum_response(self) -> Result<T, Response>;
}

impl<T> ServiceErrorExt<T> for Result<T, Box<dyn Error>> {
    fn into_axum_response(self) -> Result<T, Response> {
        self.map_err(|err| handle_error(err))
    }
}
