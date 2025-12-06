//! Authentication middleware and extractors.

use std::future::{Future, Ready, ready};
use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, dev::Payload, web};

use crate::domain::AppError;
use crate::infrastructure::jwt;

/// Authenticated user extracted from JWT token in Authorization header.
///
/// Use this extractor in handlers that require authentication:
/// ```ignore
/// async fn create_post(auth: AuthenticatedUser, ...) -> Result<impl Responder, AppError> {
///     let user_id = auth.user_id;
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// The authenticated user's ID.
    pub user_id: i64,
}

/// Wrapper for JWT secret to use as app data.
#[derive(Clone)]
pub struct JwtSecret(pub String);

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(extract_user(req))
    }
}

/// Extracts the authenticated user from the request.
fn extract_user(req: &HttpRequest) -> Result<AuthenticatedUser, AppError> {
    // Extract token from Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::InvalidCredentials)?;

    // Get JWT secret from app data
    let jwt_secret = req
        .app_data::<web::Data<JwtSecret>>()
        .ok_or_else(|| AppError::Internal("JWT secret not configured".into()))?;

    // Validate token and extract claims
    let claims = jwt::validate_token(token, &jwt_secret.0)?;

    Ok(AuthenticatedUser {
        user_id: claims.sub,
    })
}

/// Optional authentication extractor.
///
/// Use this when authentication is optional (e.g., public endpoints that
/// behave differently for authenticated users).
#[derive(Debug, Clone)]
pub struct OptionalUser(pub Option<AuthenticatedUser>);

impl FromRequest for OptionalUser {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let user = extract_user(req).ok();
        Box::pin(async move { Ok(OptionalUser(user)) })
    }
}
