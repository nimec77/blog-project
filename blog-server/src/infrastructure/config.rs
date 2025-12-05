//! Application configuration.

use std::env;

use blog_shared::constants::{
    DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT, ENV_DATABASE_URL, ENV_GRPC_PORT, ENV_HTTP_PORT,
    ENV_JWT_SECRET,
};

use crate::domain::AppError;

/// Application configuration loaded from environment.
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub http_port: u16,
    pub grpc_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self, AppError> {
        // Try workspace root first, then blog-server subdirectory
        dotenvy::dotenv()
            .or_else(|_| dotenvy::from_filename("blog-server/.env"))
            .ok();

        let database_url = env::var(ENV_DATABASE_URL)
            .map_err(|_| AppError::Config(format!("{ENV_DATABASE_URL} must be set")))?;

        let jwt_secret = env::var(ENV_JWT_SECRET)
            .map_err(|_| AppError::Config(format!("{ENV_JWT_SECRET} must be set")))?;

        let http_port = env::var(ENV_HTTP_PORT)
            .unwrap_or_else(|_| DEFAULT_HTTP_PORT.to_string())
            .parse()
            .map_err(|_| AppError::Config(format!("{ENV_HTTP_PORT} must be a number")))?;

        let grpc_port = env::var(ENV_GRPC_PORT)
            .unwrap_or_else(|_| DEFAULT_GRPC_PORT.to_string())
            .parse()
            .map_err(|_| AppError::Config(format!("{ENV_GRPC_PORT} must be a number")))?;

        Ok(Self {
            database_url,
            jwt_secret,
            http_port,
            grpc_port,
        })
    }
}
