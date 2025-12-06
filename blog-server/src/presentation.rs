//! Presentation layer: HTTP handlers and routes.

pub mod grpc_service;
mod http_handlers;
mod middleware;

pub use http_handlers::api_routes;
pub use middleware::{AuthenticatedUser, JwtSecret, OptionalUser};
