//! HTTP client for the blog API.

use reqwest::Client;

use blog_shared::{
    AuthResponse, CreatePostRequest, LoginRequest, PostDto, PostListResponse, RegisterRequest,
    UpdatePostRequest,
};

use crate::ClientError;

/// HTTP client for the blog API.
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl HttpClient {
    /// Creates a new HTTP client.
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            token: None,
        }
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
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, ClientError> {
        let url = format!("{}/api/auth/register", self.base_url);
        let response = self.client.post(&url).json(&req).send().await?;
        self.handle_response(response).await
    }

    /// Logs in an existing user.
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, ClientError> {
        let url = format!("{}/api/auth/login", self.base_url);
        let response = self.client.post(&url).json(&req).send().await?;
        self.handle_response(response).await
    }

    /// Creates a new post (requires authentication).
    pub async fn create_post(&self, req: CreatePostRequest) -> Result<PostDto, ClientError> {
        let url = format!("{}/api/posts", self.base_url);
        let response = self
            .authorized_request(self.client.post(&url))?
            .json(&req)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Gets a post by ID.
    pub async fn get_post(&self, id: i64) -> Result<PostDto, ClientError> {
        let url = format!("{}/api/posts/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Lists posts with pagination.
    pub async fn list_posts(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<PostListResponse, ClientError> {
        let url = format!(
            "{}/api/posts?limit={}&offset={}",
            self.base_url, limit, offset
        );
        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Updates a post (author only).
    pub async fn update_post(
        &self,
        id: i64,
        req: UpdatePostRequest,
    ) -> Result<PostDto, ClientError> {
        let url = format!("{}/api/posts/{}", self.base_url, id);
        let response = self
            .authorized_request(self.client.put(&url))?
            .json(&req)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Deletes a post (author only).
    pub async fn delete_post(&self, id: i64) -> Result<(), ClientError> {
        let url = format!("{}/api/posts/{}", self.base_url, id);
        let response = self
            .authorized_request(self.client.delete(&url))?
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(ClientError::Server { status, message })
        }
    }

    /// Adds authorization header to a request builder.
    fn authorized_request(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ClientError> {
        let token = self.token.as_ref().ok_or(ClientError::NotAuthenticated)?;
        Ok(builder.header("Authorization", format!("Bearer {}", token)))
    }

    /// Handles response, extracting JSON or error.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ClientError> {
        if response.status().is_success() {
            let body = response.text().await?;
            Ok(serde_json::from_str(&body)?)
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(ClientError::Server { status, message })
        }
    }
}
