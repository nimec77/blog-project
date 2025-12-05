//! JWT token handling.

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::constants::JWT_EXPIRY_HOURS;
use crate::domain::AppError;

/// JWT claims structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID).
    pub sub: i64,
    /// Expiration time (Unix timestamp).
    pub exp: usize,
}

/// Creates a JWT token for the given user ID.
pub fn create_token(user_id: i64, secret: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now() + chrono::Duration::hours(JWT_EXPIRY_HOURS);
    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::Jwt)
}

/// Validates a JWT token and returns the claims.
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(AppError::Jwt)?;

    Ok(token_data.claims)
}
