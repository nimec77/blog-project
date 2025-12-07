//! Blog client library.
//!
//! Provides HTTP and gRPC clients for the blog API.

mod error;
mod grpc_client;
mod http_client;

pub use error::ClientError;
pub use grpc_client::GrpcClient;
pub use http_client::HttpClient;

use blog_shared::{
    AuthResponse, CreatePostRequest, LoginRequest, PostDto, PostListResponse, RegisterRequest,
    UpdatePostRequest,
};

/// Unified blog client supporting both HTTP and gRPC transports.
pub enum BlogClient {
    /// HTTP client variant.
    Http(HttpClient),
    /// gRPC client variant.
    Grpc(Box<GrpcClient>),
}

impl BlogClient {
    /// Creates a new HTTP client.
    pub fn http(base_url: &str) -> Self {
        Self::Http(HttpClient::new(base_url))
    }

    /// Creates a new gRPC client.
    pub async fn grpc(addr: &str) -> Result<Self, ClientError> {
        Ok(Self::Grpc(Box::new(GrpcClient::connect(addr).await?)))
    }

    /// Sets the authentication token.
    pub fn set_token(&mut self, token: String) {
        match self {
            Self::Http(client) => client.set_token(token),
            Self::Grpc(client) => client.set_token(token),
        }
    }

    /// Clears the authentication token.
    pub fn clear_token(&mut self) {
        match self {
            Self::Http(client) => client.clear_token(),
            Self::Grpc(client) => client.clear_token(),
        }
    }

    /// Returns the current token if set.
    pub fn token(&self) -> Option<&str> {
        match self {
            Self::Http(client) => client.token(),
            Self::Grpc(client) => client.token(),
        }
    }

    /// Registers a new user.
    pub async fn register(&mut self, req: RegisterRequest) -> Result<AuthResponse, ClientError> {
        match self {
            Self::Http(client) => client.register(req).await,
            Self::Grpc(client) => client.register(req).await,
        }
    }

    /// Logs in an existing user.
    pub async fn login(&mut self, req: LoginRequest) -> Result<AuthResponse, ClientError> {
        match self {
            Self::Http(client) => client.login(req).await,
            Self::Grpc(client) => client.login(req).await,
        }
    }

    /// Creates a new post (requires authentication).
    pub async fn create_post(&mut self, req: CreatePostRequest) -> Result<PostDto, ClientError> {
        match self {
            Self::Http(client) => client.create_post(req).await,
            Self::Grpc(client) => client.create_post(req).await,
        }
    }

    /// Gets a post by ID.
    pub async fn get_post(&mut self, id: i64) -> Result<PostDto, ClientError> {
        match self {
            Self::Http(client) => client.get_post(id).await,
            Self::Grpc(client) => client.get_post(id).await,
        }
    }

    /// Lists posts with pagination.
    pub async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<PostListResponse, ClientError> {
        match self {
            Self::Http(client) => client.list_posts(limit, offset).await,
            Self::Grpc(client) => client.list_posts(limit, offset).await,
        }
    }

    /// Updates a post (author only).
    pub async fn update_post(
        &mut self,
        id: i64,
        req: UpdatePostRequest,
    ) -> Result<PostDto, ClientError> {
        match self {
            Self::Http(client) => client.update_post(id, req).await,
            Self::Grpc(client) => client.update_post(id, req).await,
        }
    }

    /// Deletes a post (author only).
    pub async fn delete_post(&mut self, id: i64) -> Result<(), ClientError> {
        match self {
            Self::Http(client) => client.delete_post(id).await,
            Self::Grpc(client) => client.delete_post(id).await,
        }
    }
}
