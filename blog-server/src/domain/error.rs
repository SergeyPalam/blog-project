use actix_web::{App, HttpResponse, error::ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User already exists: {0}")]
    AlreadyExists(String),
    #[error("User unauthorized: {0}")]
    Unauthorized(String),
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Post not found: {0}")]
    PostNotFound(String),
    #[error("Internal server error: {0}")]
    
    InternalError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            AppError::AlreadyExists(_) => {
                StatusCode::CONFLICT
            }
            AppError::Unauthorized(_) => {
                StatusCode::UNAUTHORIZED
            }
            AppError::UserNotFound(_) => {
                StatusCode::NOT_FOUND
            }
            AppError::PostNotFound(_) => {
                StatusCode::NOT_FOUND
            }
            AppError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16()
        }))
    }
}