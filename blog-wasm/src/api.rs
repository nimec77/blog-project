//! HTTP client for the blog API.

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};

use blog_shared::{
    AuthResponse, CreatePostRequest, LoginRequest, PostDto, PostListResponse, RegisterRequest,
    UpdatePostRequest,
};

use crate::constants::{API_BASE_URL, TOKEN_STORAGE_KEY};

/// API client error.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Loads token from localStorage.
pub fn get_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_STORAGE_KEY).ok()
}

/// Saves token to localStorage.
pub fn set_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_STORAGE_KEY, token);
}

/// Clears token from localStorage.
pub fn clear_token() {
    LocalStorage::delete(TOKEN_STORAGE_KEY);
}

/// Checks if user is authenticated.
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

/// Registers a new user.
pub async fn register(req: RegisterRequest) -> Result<AuthResponse, ApiError> {
    let url = format!("{}/api/auth/register", API_BASE_URL);
    let response = Request::post(&url)
        .json(&req)
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?;

    handle_response(response).await
}

/// Logs in an existing user.
pub async fn login(req: LoginRequest) -> Result<AuthResponse, ApiError> {
    let url = format!("{}/api/auth/login", API_BASE_URL);
    let response = Request::post(&url)
        .json(&req)
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?;

    handle_response(response).await
}

/// Creates a new post.
pub async fn create_post(req: CreatePostRequest) -> Result<PostDto, ApiError> {
    let url = format!("{}/api/posts", API_BASE_URL);
    let token = get_token().ok_or(ApiError {
        message: "Not authenticated".into(),
    })?;

    let response = Request::post(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?;

    handle_response(response).await
}

/// Gets a post by ID.
pub async fn get_post(id: i64) -> Result<PostDto, ApiError> {
    let url = format!("{}/api/posts/{}", API_BASE_URL, id);
    let response = Request::get(&url).send().await.map_err(|e| ApiError {
        message: e.to_string(),
    })?;

    handle_response(response).await
}

/// Lists posts with pagination.
pub async fn list_posts(limit: i64, offset: i64) -> Result<PostListResponse, ApiError> {
    let url = format!(
        "{}/api/posts?limit={}&offset={}",
        API_BASE_URL, limit, offset
    );
    let response = Request::get(&url).send().await.map_err(|e| ApiError {
        message: e.to_string(),
    })?;

    handle_response(response).await
}

/// Updates a post.
pub async fn update_post(id: i64, req: UpdatePostRequest) -> Result<PostDto, ApiError> {
    let url = format!("{}/api/posts/{}", API_BASE_URL, id);
    let token = get_token().ok_or(ApiError {
        message: "Not authenticated".into(),
    })?;

    let response = Request::put(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?;

    handle_response(response).await
}

/// Deletes a post.
pub async fn delete_post(id: i64) -> Result<(), ApiError> {
    let url = format!("{}/api/posts/{}", API_BASE_URL, id);
    let token = get_token().ok_or(ApiError {
        message: "Not authenticated".into(),
    })?;

    let response = Request::delete(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?;

    if response.ok() {
        Ok(())
    } else {
        let text = response.text().await.unwrap_or_default();
        Err(ApiError { message: text })
    }
}

/// Handles API response.
async fn handle_response<T: serde::de::DeserializeOwned>(
    response: gloo_net::http::Response,
) -> Result<T, ApiError> {
    if response.ok() {
        response.json().await.map_err(|e| ApiError {
            message: e.to_string(),
        })
    } else {
        let text = response.text().await.unwrap_or_default();
        Err(ApiError { message: text })
    }
}
