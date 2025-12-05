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

    #[error("Username already exists")]
    UsernameExists,

    #[error("Email already exists")]
    EmailExists,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error")]
    PasswordHash,

    #[error("Internal error: {0}")]
    Internal(String),
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
            AppError::UsernameExists | AppError::EmailExists | AppError::Validation(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({"error": self.to_string()}))
            }
            AppError::Config(_)
            | AppError::Database(_)
            | AppError::Jwt(_)
            | AppError::PasswordHash
            | AppError::Internal(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Internal server error"})),
        }
    }
}
