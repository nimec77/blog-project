//! HTTP request handlers.

use actix_web::{HttpResponse, Responder, Scope, get, post, web};
use blog_shared::{LoginRequest, RegisterRequest};

use crate::application::AuthService;
use crate::domain::AppError;

/// Creates public routes scope (no authentication required).
pub fn public_routes() -> Scope {
    web::scope("")
        .service(health)
        .service(register)
        .service(login)
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
