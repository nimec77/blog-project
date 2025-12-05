//! Blog server entry point.

use actix_web::{App, HttpResponse, HttpServer, web};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod constants;
mod domain;
mod infrastructure;

use infrastructure::{config::Config, database};

/// Health check endpoint.
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load config
    let config = Config::from_env();
    let pool = database::create_pool(&config.database_url).await;

    info!(port = config.http_port, "Starting HTTP server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
    })
    .bind(("0.0.0.0", config.http_port))?
    .run()
    .await
}
