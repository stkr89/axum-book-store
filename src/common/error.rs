use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub enum APIError {
    ServerError(String),
    BadRequest(String),
    Unauthorized(String),
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        match self {
            APIError::ServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            APIError::BadRequest(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            APIError::Unauthorized(e) => (StatusCode::UNAUTHORIZED, e).into_response(),
        }
    }
}