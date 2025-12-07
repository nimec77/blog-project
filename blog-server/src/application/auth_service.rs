//! Authentication service.

use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use blog_shared::{AuthResponse, LoginRequest, RegisterRequest, UserDto};
use tracing::{info, instrument};

use crate::data::UserRepository;
use crate::domain::AppError;
use crate::infrastructure::jwt;

/// Service for authentication operations.
#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<UserRepository>,
    jwt_secret: String,
}

impl AuthService {
    /// Creates a new AuthService.
    pub fn new(user_repo: Arc<UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    /// Registers a new user.
    #[instrument(skip(self, req), fields(username = %req.username, email = %req.email))]
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        // Check if username exists
        if self
            .user_repo
            .find_by_username(&req.username)
            .await?
            .is_some()
        {
            return Err(AppError::UsernameExists);
        }

        // Check if email exists
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::EmailExists);
        }

        // Hash password
        let password_hash = hash_password(&req.password)?;

        // Create user
        let user = self
            .user_repo
            .create(&req.username, &req.email, &password_hash)
            .await?;

        // Generate token
        let token = jwt::create_token(user.id, &self.jwt_secret)?;

        info!(user_id = user.id, "User registered");

        Ok(AuthResponse {
            token,
            user: user_to_dto(&user),
        })
    }

    /// Gets a user by ID (for session restoration).
    #[instrument(skip(self))]
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<UserDto, AppError> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::UserNotFound)?;

        Ok(user_to_dto(&user))
    }

    /// Logs in an existing user.
    #[instrument(skip(self, req), fields(username = %req.username))]
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        // Find user
        let user = self
            .user_repo
            .find_by_username(&req.username)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        verify_password(&req.password, &user.password_hash)?;

        // Generate token
        let token = jwt::create_token(user.id, &self.jwt_secret)?;

        info!(user_id = user.id, "User logged in");

        Ok(AuthResponse {
            token,
            user: user_to_dto(&user),
        })
    }
}

/// Hashes a password using Argon2.
fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::PasswordHash)
}

/// Verifies a password against a hash.
fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::PasswordHash)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)
}

/// Converts a User domain entity to UserDto.
fn user_to_dto(user: &crate::domain::User) -> UserDto {
    UserDto {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        created_at: user.created_at,
    }
}
