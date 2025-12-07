//! Common test utilities.

use sqlx::SqlitePool;

use blog_server::infrastructure::database;

/// Creates an in-memory SQLite database for testing.
pub async fn setup_test_db() -> SqlitePool {
    let pool = database::create_pool("sqlite::memory:")
        .await
        .expect("failed to create test database");
    database::run_migrations(&pool)
        .await
        .expect("failed to run migrations");
    pool
}

/// Test JWT secret for integration tests.
pub const TEST_JWT_SECRET: &str =
    "test-secret-key-for-integration-tests-minimum-32-characters-long";
