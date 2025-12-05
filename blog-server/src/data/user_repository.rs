//! User repository for database operations.

use sqlx::SqlitePool;

use crate::domain::{AppError, User};

/// Repository for user-related database operations.
#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    /// Creates a new UserRepository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Finds a user by username.
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id!", username, email, password_hash, created_at as "created_at: _"
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Finds a user by email.
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id!", username, email, password_hash, created_at as "created_at: _"
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Creates a new user.
    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let now = chrono::Utc::now();
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, created_at)
            VALUES (?, ?, ?, ?)
            RETURNING id as "id!", username, email, password_hash, created_at as "created_at: _"
            "#,
            username,
            email,
            password_hash,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
