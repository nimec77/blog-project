//! Domain layer: entities and business logic.

mod error;
mod post;
mod user;

pub use error::AppError;
pub use post::Post;
pub use user::User;
