//! Blog server entry point.

use std::net::SocketAddr;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server as GrpcServer;
use tonic_reflection::server::Builder as ReflectionBuilder;
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
use presentation::grpc_service::proto::auth_service_server::AuthServiceServer;
use presentation::grpc_service::proto::blog_service_server::BlogServiceServer;
use presentation::grpc_service::{GrpcAuthService, GrpcBlogService};
use presentation::{JwtSecret, api_routes};

/// File descriptor set for gRPC reflection.
const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("blog_descriptor");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Clone services for gRPC
    let grpc_auth_service = GrpcAuthService::new(auth_service.clone());
    let grpc_blog_service = GrpcBlogService::new(blog_service.clone(), config.jwt_secret.clone());

    // gRPC server address
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", config.grpc_port).parse()?;

    // Create reflection service for gRPC
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // Bind gRPC listener first to log when ready
    let grpc_listener = TcpListener::bind(&grpc_addr).await?;
    info!(port = config.grpc_port, "gRPC server listening");

    // Start gRPC server with the listener
    let grpc_server = GrpcServer::builder()
        .add_service(AuthServiceServer::new(grpc_auth_service))
        .add_service(BlogServiceServer::new(grpc_blog_service))
        .add_service(reflection_service)
        .serve_with_incoming(TcpListenerStream::new(grpc_listener));

    // Start HTTP server
    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jwt_secret.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .service(web::scope("/api").service(api_routes()))
    })
    .bind(("0.0.0.0", config.http_port))?;

    info!(port = config.http_port, "HTTP server listening");

    let http_server = http_server.run();

    // Run both servers concurrently
    tokio::select! {
        result = http_server => {
            result?;
        }
        result = grpc_server => {
            result?;
        }
    }

    Ok(())
}
