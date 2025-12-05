//! Shared types for the blog platform.
//!
//! This crate contains DTOs shared between server, client, and CLI.

mod auth;
pub mod constants;
mod post;
mod request;
mod user;

pub use auth::{AuthResponse, LoginRequest, RegisterRequest};
pub use post::{PostDto, PostListResponse};
pub use request::{CreatePostRequest, UpdatePostRequest};
pub use user::UserDto;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_user_dto_serialization() {
        let user = UserDto {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("testuser"));

        let parsed: UserDto = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.username, "testuser");
    }

    #[test]
    fn test_post_dto_serialization() {
        let post = PostDto {
            id: 1,
            title: "Test Post".to_string(),
            content: "Content".to_string(),
            author_id: 42,
            author_username: "author".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&post).unwrap();
        let parsed: PostDto = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.title, "Test Post");
        assert_eq!(parsed.author_id, 42);
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt.token.here".to_string(),
            user: UserDto {
                id: 1,
                username: "user".to_string(),
                email: "user@example.com".to_string(),
                created_at: Utc::now(),
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("jwt.token.here"));
    }
}
