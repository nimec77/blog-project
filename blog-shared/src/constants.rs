//! Shared constants for the blog platform.

// Environment variable names
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_JWT_SECRET: &str = "JWT_SECRET";
pub const ENV_HTTP_PORT: &str = "HTTP_PORT";
pub const ENV_GRPC_PORT: &str = "GRPC_PORT";

// Default values
pub const DEFAULT_HTTP_PORT: u16 = 8080;
pub const DEFAULT_GRPC_PORT: u16 = 50051;
