//! User domain entity.

use chrono::{DateTime, Utc};

/// User entity with password hash (internal use only).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
