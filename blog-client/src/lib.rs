//! Blog client library.
//!
//! Provides HTTP and gRPC clients for the blog API.

mod error;
mod grpc_client;
mod http_client;

pub use error::ClientError;
pub use grpc_client::GrpcClient;
pub use http_client::HttpClient;
