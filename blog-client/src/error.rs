//! Client library errors.

use thiserror::Error;

/// Errors that can occur when using the blog client.
#[derive(Debug, Error)]
pub enum ClientError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// gRPC request failed.
    #[error("gRPC request failed: {0}")]
    Grpc(#[from] tonic::Status),

    /// Invalid server URL provided.
    #[error("Invalid server URL: {0}")]
    InvalidUrl(String),

    /// Operation requires authentication but no token is set.
    #[error("Not authenticated")]
    NotAuthenticated,

    /// Server returned an error response.
    #[error("Server error ({status}): {message}")]
    Server { status: u16, message: String },

    /// Failed to deserialize server response.
    #[error("Deserialization failed: {0}")]
    Deserialization(#[from] serde_json::Error),
}
