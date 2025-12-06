//! Post repository for database operations.

use sqlx::SqlitePool;

use crate::domain::{AppError, Post};

/// Repository for post-related database operations.
#[derive(Clone)]
pub struct PostRepository {
    pool: SqlitePool,
}

impl PostRepository {
    /// Creates a new PostRepository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new post.
    pub async fn create(
        &self,
        title: &str,
        content: &str,
        author_id: i64,
    ) -> Result<Post, AppError> {
        let now = chrono::Utc::now();
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, author_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id as "id!", title, content, author_id, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            title,
            content,
            author_id,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    /// Finds a post by ID.
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id as "id!", title, content, author_id, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM posts
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    /// Lists posts with pagination, ordered by created_at descending.
    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id as "id!", title, content, author_id, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM posts
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    /// Counts total posts.
    pub async fn count(&self) -> Result<i64, AppError> {
        let result = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count: i64" FROM posts"#)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    /// Updates a post. Only provided fields are updated.
    pub async fn update(
        &self,
        id: i64,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Post, AppError> {
        let now = chrono::Utc::now();

        // Get current post to preserve unchanged fields
        let current = self.find_by_id(id).await?.ok_or(AppError::PostNotFound)?;

        let new_title = title.unwrap_or(&current.title);
        let new_content = content.unwrap_or(&current.content);

        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = ?, content = ?, updated_at = ?
            WHERE id = ?
            RETURNING id as "id!", title, content, author_id, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            new_title,
            new_content,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    /// Deletes a post by ID.
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::PostNotFound);
        }

        Ok(())
    }

    /// Finds the author username for a given author_id.
    pub async fn find_author_username(&self, author_id: i64) -> Result<String, AppError> {
        let result = sqlx::query_scalar!(
            r#"SELECT username as "username!" FROM users WHERE id = ?"#,
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;

        result.ok_or(AppError::UserNotFound)
    }
}
