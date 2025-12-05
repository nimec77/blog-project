//! Database connection and pool management.

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tracing::info;

use crate::constants::DB_MAX_CONNECTIONS;

/// Create a SQLite connection pool and run migrations.
pub async fn create_pool(database_url: &str) -> SqlitePool {
    info!(url = %database_url, "Connecting to database");

    let pool = SqlitePoolOptions::new()
        .max_connections(DB_MAX_CONNECTIONS)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Database migrations completed");

    pool
}
