//! Post data transfer objects.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Post data transfer object with author info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDto {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub author_username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list of posts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostListResponse {
    pub posts: Vec<PostDto>,
    pub total: i64,
}
