//! gRPC service implementations.

use tonic::{Request, Response, Status};

use crate::application::{AuthService, BlogService};
use crate::constants::{DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::infrastructure::jwt;

/// Generated protobuf types and service traits.
pub mod proto {
    tonic::include_proto!("blog");
}

use proto::auth_service_server::AuthService as GrpcAuthServiceTrait;
use proto::blog_service_server::BlogService as GrpcBlogServiceTrait;

// ============================================================================
// Auth Service Implementation
// ============================================================================

/// gRPC implementation of AuthService.
pub struct GrpcAuthService {
    auth_service: AuthService,
}

impl GrpcAuthService {
    /// Creates a new GrpcAuthService.
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }
}

#[tonic::async_trait]
impl GrpcAuthServiceTrait for GrpcAuthService {
    async fn register(
        &self,
        request: Request<proto::RegisterRequest>,
    ) -> Result<Response<proto::AuthResponse>, Status> {
        let req = request.into_inner();

        let shared_req = blog_shared::RegisterRequest {
            username: req.username,
            email: req.email,
            password: req.password,
        };

        let result = self
            .auth_service
            .register(shared_req)
            .await
            .map_err(app_error_to_status)?;

        Ok(Response::new(proto::AuthResponse {
            token: result.token,
            user: Some(user_dto_to_proto(&result.user)),
        }))
    }

    async fn login(
        &self,
        request: Request<proto::LoginRequest>,
    ) -> Result<Response<proto::AuthResponse>, Status> {
        let req = request.into_inner();

        let shared_req = blog_shared::LoginRequest {
            username: req.username,
            password: req.password,
        };

        let result = self
            .auth_service
            .login(shared_req)
            .await
            .map_err(app_error_to_status)?;

        Ok(Response::new(proto::AuthResponse {
            token: result.token,
            user: Some(user_dto_to_proto(&result.user)),
        }))
    }
}

// ============================================================================
// Blog Service Implementation
// ============================================================================

/// gRPC implementation of BlogService.
pub struct GrpcBlogService {
    blog_service: BlogService,
    jwt_secret: String,
}

impl GrpcBlogService {
    /// Creates a new GrpcBlogService.
    pub fn new(blog_service: BlogService, jwt_secret: String) -> Self {
        Self {
            blog_service,
            jwt_secret,
        }
    }

    /// Validates a JWT token and returns the user ID.
    fn validate_token(&self, token: &str) -> Result<i64, Status> {
        let claims = jwt::validate_token(token, &self.jwt_secret)
            .map_err(|_| Status::unauthenticated("Invalid token"))?;
        Ok(claims.sub)
    }
}

#[tonic::async_trait]
impl GrpcBlogServiceTrait for GrpcBlogService {
    async fn create_post(
        &self,
        request: Request<proto::CreatePostRequest>,
    ) -> Result<Response<proto::PostResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.validate_token(&req.token)?;

        let shared_req = blog_shared::CreatePostRequest {
            title: req.title,
            content: req.content,
        };

        let post = self
            .blog_service
            .create_post(user_id, shared_req)
            .await
            .map_err(app_error_to_status)?;

        Ok(Response::new(proto::PostResponse {
            post: Some(post_dto_to_proto(&post)),
        }))
    }

    async fn get_post(
        &self,
        request: Request<proto::GetPostRequest>,
    ) -> Result<Response<proto::PostResponse>, Status> {
        let req = request.into_inner();

        let post = self
            .blog_service
            .get_post(req.id)
            .await
            .map_err(app_error_to_status)?;

        Ok(Response::new(proto::PostResponse {
            post: Some(post_dto_to_proto(&post)),
        }))
    }

    async fn list_posts(
        &self,
        request: Request<proto::ListPostsRequest>,
    ) -> Result<Response<proto::ListPostsResponse>, Status> {
        let req = request.into_inner();
        let limit = if req.limit > 0 {
            req.limit
        } else {
            DEFAULT_LIMIT
        };
        let offset = if req.offset >= 0 {
            req.offset
        } else {
            DEFAULT_OFFSET
        };

        let result = self
            .blog_service
            .list_posts(limit, offset)
            .await
            .map_err(app_error_to_status)?;

        let posts = result.posts.iter().map(post_dto_to_proto).collect();

        Ok(Response::new(proto::ListPostsResponse {
            posts,
            total: result.total,
        }))
    }

    async fn update_post(
        &self,
        request: Request<proto::UpdatePostRequest>,
    ) -> Result<Response<proto::PostResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.validate_token(&req.token)?;

        let shared_req = blog_shared::UpdatePostRequest {
            title: req.title,
            content: req.content,
        };

        let post = self
            .blog_service
            .update_post(req.id, user_id, shared_req)
            .await
            .map_err(app_error_to_status)?;

        Ok(Response::new(proto::PostResponse {
            post: Some(post_dto_to_proto(&post)),
        }))
    }

    async fn delete_post(
        &self,
        request: Request<proto::DeletePostRequest>,
    ) -> Result<Response<proto::Empty>, Status> {
        let req = request.into_inner();
        let user_id = self.validate_token(&req.token)?;

        self.blog_service
            .delete_post(req.id, user_id)
            .await
            .map_err(app_error_to_status)?;

        Ok(Response::new(proto::Empty {}))
    }
}

// ============================================================================
// Conversion Helpers
// ============================================================================

/// Converts AppError to gRPC Status.
fn app_error_to_status(err: crate::domain::AppError) -> Status {
    use crate::domain::AppError;

    match err {
        AppError::UserNotFound | AppError::PostNotFound => Status::not_found(err.to_string()),
        AppError::InvalidCredentials => Status::unauthenticated(err.to_string()),
        AppError::Forbidden => Status::permission_denied(err.to_string()),
        AppError::UsernameExists | AppError::EmailExists | AppError::Validation(_) => {
            Status::invalid_argument(err.to_string())
        }
        _ => Status::internal("Internal server error"),
    }
}

/// Converts UserDto to proto User.
fn user_dto_to_proto(user: &blog_shared::UserDto) -> proto::User {
    proto::User {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        created_at: user.created_at.to_rfc3339(),
    }
}

/// Converts PostDto to proto Post.
fn post_dto_to_proto(post: &blog_shared::PostDto) -> proto::Post {
    proto::Post {
        id: post.id,
        title: post.title.clone(),
        content: post.content.clone(),
        author_id: post.author_id,
        author_username: post.author_username.clone(),
        created_at: post.created_at.to_rfc3339(),
        updated_at: post.updated_at.to_rfc3339(),
    }
}
