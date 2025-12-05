//! Database connection and pool management.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tracing::info;

use crate::constants::DB_MAX_CONNECTIONS;

/// Creates a SQLite connection pool.
pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    info!(url = %database_url, "Connecting to database");

    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(DB_MAX_CONNECTIONS)
        .connect_with(options)
        .await?;

    info!("Connected to database");
    Ok(pool)
}

/// Runs database migrations.
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Running database migrations");
    sqlx::migrate!().run(pool).await?;
    info!("Database migrations completed");
    Ok(())
}
