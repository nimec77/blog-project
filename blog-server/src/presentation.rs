//! Presentation layer: HTTP handlers and routes.

mod http_handlers;
mod middleware;

pub use http_handlers::{protected_routes, public_routes};
pub use middleware::{AuthenticatedUser, JwtSecret, OptionalUser};
