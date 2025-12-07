//! gRPC client for the blog API.

use chrono::{DateTime, Utc};
use serde::de::Error as _;

use blog_shared::{
    AuthResponse, CreatePostRequest, LoginRequest, PostDto, PostListResponse, RegisterRequest,
    UpdatePostRequest, UserDto,
};

use crate::ClientError;

/// Generated protobuf types and client stubs.
pub mod proto {
    tonic::include_proto!("blog");
}

use proto::{auth_service_client::AuthServiceClient, blog_service_client::BlogServiceClient};

/// gRPC client for the blog API.
#[derive(Clone)]
pub struct GrpcClient {
    auth_client: AuthServiceClient<tonic::transport::Channel>,
    blog_client: BlogServiceClient<tonic::transport::Channel>,
    token: Option<String>,
}

impl GrpcClient {
    /// Connects to the gRPC server.
    pub async fn connect(addr: &str) -> Result<Self, ClientError> {
        let channel = tonic::transport::Channel::from_shared(addr.to_string())
            .map_err(|e| ClientError::InvalidUrl(e.to_string()))?
            .connect()
            .await
            .map_err(|e| ClientError::Grpc(tonic::Status::from_error(Box::new(e))))?;

        Ok(Self {
            auth_client: AuthServiceClient::new(channel.clone()),
            blog_client: BlogServiceClient::new(channel),
            token: None,
        })
    }

    /// Sets the authentication token.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Clears the authentication token.
    pub fn clear_token(&mut self) {
        self.token = None;
    }

    /// Returns the current token if set.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Registers a new user.
    pub async fn register(&mut self, req: RegisterRequest) -> Result<AuthResponse, ClientError> {
        let request = proto::RegisterRequest {
            username: req.username,
            email: req.email,
            password: req.password,
        };

        let response = self.auth_client.register(request).await?;
        Self::convert_auth_response(response.into_inner())
    }

    /// Logs in an existing user.
    pub async fn login(&mut self, req: LoginRequest) -> Result<AuthResponse, ClientError> {
        let request = proto::LoginRequest {
            username: req.username,
            password: req.password,
        };

        let response = self.auth_client.login(request).await?;
        Self::convert_auth_response(response.into_inner())
    }

    /// Creates a new post (requires authentication).
    pub async fn create_post(&mut self, req: CreatePostRequest) -> Result<PostDto, ClientError> {
        let token = self.token.clone().ok_or(ClientError::NotAuthenticated)?;
        let request = proto::CreatePostRequest {
            token,
            title: req.title,
            content: req.content,
        };

        let response = self.blog_client.create_post(request).await?;
        Self::convert_post(response.into_inner().post.unwrap())
    }

    /// Gets a post by ID.
    pub async fn get_post(&mut self, id: i64) -> Result<PostDto, ClientError> {
        let request = proto::GetPostRequest { id };
        let response = self.blog_client.get_post(request).await?;
        Self::convert_post(response.into_inner().post.unwrap())
    }

    /// Lists posts with pagination.
    pub async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<PostListResponse, ClientError> {
        let request = proto::ListPostsRequest { limit, offset };
        let response = self.blog_client.list_posts(request).await?;
        let inner = response.into_inner();

        let posts = inner
            .posts
            .into_iter()
            .map(Self::convert_post)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PostListResponse {
            posts,
            total: inner.total,
        })
    }

    /// Updates a post (author only).
    pub async fn update_post(
        &mut self,
        id: i64,
        req: UpdatePostRequest,
    ) -> Result<PostDto, ClientError> {
        let token = self.token.clone().ok_or(ClientError::NotAuthenticated)?;
        let request = proto::UpdatePostRequest {
            token,
            id,
            title: req.title,
            content: req.content,
        };

        let response = self.blog_client.update_post(request).await?;
        Self::convert_post(response.into_inner().post.unwrap())
    }

    /// Deletes a post (author only).
    pub async fn delete_post(&mut self, id: i64) -> Result<(), ClientError> {
        let token = self.token.clone().ok_or(ClientError::NotAuthenticated)?;
        let request = proto::DeletePostRequest { token, id };
        self.blog_client.delete_post(request).await?;
        Ok(())
    }

    /// Converts proto AuthResponse to shared AuthResponse.
    fn convert_auth_response(response: proto::AuthResponse) -> Result<AuthResponse, ClientError> {
        let user = response.user.unwrap();
        Ok(AuthResponse {
            token: response.token,
            user: UserDto {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: Self::parse_datetime(&user.created_at)?,
            },
        })
    }

    /// Converts proto Post to shared PostDto.
    fn convert_post(post: proto::Post) -> Result<PostDto, ClientError> {
        Ok(PostDto {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            author_username: post.author_username,
            created_at: Self::parse_datetime(&post.created_at)?,
            updated_at: Self::parse_datetime(&post.updated_at)?,
        })
    }

    /// Parses ISO 8601 datetime string.
    fn parse_datetime(s: &str) -> Result<DateTime<Utc>, ClientError> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| ClientError::Deserialization(serde_json::Error::custom(e.to_string())))
    }
}
