//! Post domain entity.

use chrono::{DateTime, Utc};

/// Post entity.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
