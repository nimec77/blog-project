//! Application error types.

use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

/// Application-level errors.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("User not found")]
    UserNotFound,

    #[error("Post not found")]
    PostNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Forbidden")]
    Forbidden,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::UserNotFound | AppError::PostNotFound => {
                HttpResponse::NotFound().json(serde_json::json!({"error": self.to_string()}))
            }
            AppError::InvalidCredentials => {
                HttpResponse::Unauthorized().json(serde_json::json!({"error": self.to_string()}))
            }
            AppError::Forbidden => {
                HttpResponse::Forbidden().json(serde_json::json!({"error": self.to_string()}))
            }
            AppError::Database(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Internal server error"})),
        }
    }
}
