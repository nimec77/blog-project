//! Blog client library.
//!
//! Provides HTTP and gRPC clients for the blog API.

mod error;
mod http_client;

pub use error::ClientError;
pub use http_client::HttpClient;
