//! HTTP request handlers.

use actix_web::{HttpResponse, Responder, Scope, delete, get, post, put, web};
use blog_shared::{CreatePostRequest, LoginRequest, RegisterRequest, UpdatePostRequest};
use serde::Deserialize;

use crate::application::{AuthService, BlogService};
use crate::domain::AppError;
use crate::presentation::middleware::AuthenticatedUser;

/// Creates all API routes.
pub fn api_routes() -> Scope {
    web::scope("")
        // Health
        .service(health)
        // Auth (public)
        .service(register)
        .service(login)
        // Posts (mixed: list/get are public, create/update/delete require auth)
        .service(list_posts)
        .service(get_post)
        .service(create_post)
        .service(update_post)
        .service(delete_post)
}

/// Health check endpoint.
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// Handles user registration.
#[post("/auth/register")]
async fn register(
    service: web::Data<AuthService>,
    payload: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let response = service.register(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(response))
}

/// Handles user login.
#[post("/auth/login")]
async fn login(
    service: web::Data<AuthService>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let response = service.login(payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

/// Query parameters for listing posts.
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

const DEFAULT_LIMIT: i64 = 10;
const DEFAULT_OFFSET: i64 = 0;

/// Lists posts with pagination (public).
#[get("/posts")]
async fn list_posts(
    service: web::Data<BlogService>,
    query: web::Query<ListPostsQuery>,
) -> Result<impl Responder, AppError> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
    let offset = query.offset.unwrap_or(DEFAULT_OFFSET);
    let response = service.list_posts(limit, offset).await?;
    Ok(HttpResponse::Ok().json(response))
}

/// Gets a single post by ID (public).
#[get("/posts/{id}")]
async fn get_post(
    service: web::Data<BlogService>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let post = service.get_post(id).await?;
    Ok(HttpResponse::Ok().json(post))
}

/// Creates a new post (requires authentication).
#[post("/posts")]
async fn create_post(
    auth: AuthenticatedUser,
    service: web::Data<BlogService>,
    payload: web::Json<CreatePostRequest>,
) -> Result<impl Responder, AppError> {
    let post = service
        .create_post(auth.user_id, payload.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(post))
}

/// Updates a post (author only).
#[put("/posts/{id}")]
async fn update_post(
    auth: AuthenticatedUser,
    service: web::Data<BlogService>,
    path: web::Path<i64>,
    payload: web::Json<UpdatePostRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let post = service
        .update_post(id, auth.user_id, payload.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(post))
}

/// Deletes a post (author only).
#[delete("/posts/{id}")]
async fn delete_post(
    auth: AuthenticatedUser,
    service: web::Data<BlogService>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    service.delete_post(id, auth.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
