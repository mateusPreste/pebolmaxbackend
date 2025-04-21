use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use serde_json::json;

#[derive(Debug)]
pub enum DefaultError {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    Internal(String),
}

impl IntoResponse for DefaultError {
    fn into_response(self) -> Response {
        match self {
            DefaultError::NotFound(msg) =>
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Not Found", "message": msg })),
                ).into_response(),

            DefaultError::BadRequest(msg) =>
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Bad Request", "message": msg })),
                ).into_response(),

            DefaultError::Conflict(msg) =>
                (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "Conflict", "message": msg })),
                ).into_response(),

            DefaultError::Internal(msg) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal Server Error", "message": msg })),
                ).into_response(),
        }
    }
}
