//! Server-specific constants.

/// Maximum number of database connections in the pool.
pub const DB_MAX_CONNECTIONS: u32 = 5;

/// JWT token expiry in hours.
pub const JWT_EXPIRY_HOURS: i64 = 24;
