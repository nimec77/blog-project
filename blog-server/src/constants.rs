//! Server-specific constants.

/// Maximum number of database connections in the pool.
pub const DB_MAX_CONNECTIONS: u32 = 5;

/// JWT token expiry in hours.
pub const JWT_EXPIRY_HOURS: i64 = 24;

/// Default pagination limit for list endpoints.
pub const DEFAULT_LIMIT: i64 = 10;

/// Default pagination offset for list endpoints.
pub const DEFAULT_OFFSET: i64 = 0;

/// Allowed CORS origin for WASM frontend.
pub const CORS_ALLOWED_ORIGIN: &str = "http://127.0.0.1:8081";
