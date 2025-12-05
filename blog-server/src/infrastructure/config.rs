//! Application configuration.

use std::env;

use blog_shared::constants::{
    DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT, ENV_DATABASE_URL, ENV_GRPC_PORT, ENV_HTTP_PORT,
    ENV_JWT_SECRET,
};

/// Application configuration loaded from environment.
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub http_port: u16,
    pub grpc_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var(ENV_DATABASE_URL)
                .unwrap_or_else(|_| panic!("{ENV_DATABASE_URL} is required")),
            jwt_secret: env::var(ENV_JWT_SECRET)
                .unwrap_or_else(|_| panic!("{ENV_JWT_SECRET} is required")),
            http_port: env::var(ENV_HTTP_PORT)
                .unwrap_or_else(|_| DEFAULT_HTTP_PORT.to_string())
                .parse()
                .unwrap_or_else(|_| panic!("{ENV_HTTP_PORT} must be a number")),
            grpc_port: env::var(ENV_GRPC_PORT)
                .unwrap_or_else(|_| DEFAULT_GRPC_PORT.to_string())
                .parse()
                .unwrap_or_else(|_| panic!("{ENV_GRPC_PORT} must be a number")),
        }
    }
}
