//! Blog server entry point.

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod application;
mod constants;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use application::{AuthService, BlogService};
use data::{PostRepository, UserRepository};
use infrastructure::{config::Config, database};
use presentation::{JwtSecret, api_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load config
    let config = Config::from_env().expect("invalid configuration");

    // Create database pool and run migrations
    let pool = database::create_pool(&config.database_url)
        .await
        .expect("failed to connect to database");
    database::run_migrations(&pool)
        .await
        .expect("failed to run migrations");

    // Create repositories
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool.clone()));

    // Create services
    let auth_service = AuthService::new(Arc::clone(&user_repo), config.jwt_secret.clone());
    let blog_service = BlogService::new(Arc::clone(&post_repo));

    // JWT secret for auth middleware
    let jwt_secret = JwtSecret(config.jwt_secret.clone());

    info!(port = config.http_port, "Starting HTTP server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jwt_secret.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .service(web::scope("/api").service(api_routes()))
    })
    .bind(("0.0.0.0", config.http_port))?
    .run()
    .await
}
