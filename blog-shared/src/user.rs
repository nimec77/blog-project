//! User data transfer objects.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User data transfer object (no password_hash exposed).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
