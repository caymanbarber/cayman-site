use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[allow(dead_code)]
pub enum AppError {
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, message).into_response()
    }
}
