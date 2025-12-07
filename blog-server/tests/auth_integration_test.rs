//! Integration tests for authentication endpoints.

mod common;

use std::sync::Arc;

use actix_web::{App, test, web};
use blog_shared::{AuthResponse, LoginRequest, RegisterRequest, UserDto};

use blog_server::application::AuthService;
use blog_server::data::UserRepository;
use blog_server::presentation::JwtSecret;
use blog_server::presentation::http_handlers::api_routes;

use common::{TEST_JWT_SECRET, setup_test_db};

/// Test user registration creates a new user.
#[tokio::test]
async fn test_register_creates_user() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(auth_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let req = RegisterRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "secret123".to_string(),
    };

    let resp = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&req)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 201);

    let auth_resp: AuthResponse = test::read_body_json(resp).await;
    assert!(!auth_resp.token.is_empty());
    assert_eq!(auth_resp.user.username, "testuser");
    assert_eq!(auth_resp.user.email, "test@example.com");
}

/// Test registration fails with duplicate username.
#[tokio::test]
async fn test_register_duplicate_username_fails() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(auth_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let req = RegisterRequest {
        username: "duplicate".to_string(),
        email: "first@example.com".to_string(),
        password: "secret123".to_string(),
    };

    // First registration should succeed
    let resp = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 201);

    // Second registration with same username should fail
    let req2 = RegisterRequest {
        username: "duplicate".to_string(),
        email: "second@example.com".to_string(),
        password: "secret456".to_string(),
    };

    let resp2 = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&req2)
        .send_request(&app)
        .await;

    assert!(resp2.status().is_client_error());
}

/// Test successful login.
#[tokio::test]
async fn test_login_success() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    // Register user first
    let register_req = RegisterRequest {
        username: "loginuser".to_string(),
        email: "login@example.com".to_string(),
        password: "secret123".to_string(),
    };

    test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_req)
        .send_request(&app)
        .await;

    // Now login
    let login_req = LoginRequest {
        username: "loginuser".to_string(),
        password: "secret123".to_string(),
    };

    let resp = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 200);

    let auth_resp: AuthResponse = test::read_body_json(resp).await;
    assert!(!auth_resp.token.is_empty());
    assert_eq!(auth_resp.user.username, "loginuser");
}

/// Test login fails with invalid credentials.
#[tokio::test]
async fn test_login_invalid_credentials_fails() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    // Register user
    let register_req = RegisterRequest {
        username: "validuser".to_string(),
        email: "valid@example.com".to_string(),
        password: "correctpassword".to_string(),
    };

    test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_req)
        .send_request(&app)
        .await;

    // Try to login with wrong password
    let login_req = LoginRequest {
        username: "validuser".to_string(),
        password: "wrongpassword".to_string(),
    };

    let resp = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .send_request(&app)
        .await;

    assert!(resp.status().is_client_error());
}

/// Test /auth/me endpoint with valid token.
#[tokio::test]
async fn test_get_me_with_valid_token() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service.clone()))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    // Register user
    let register_req = RegisterRequest {
        username: "meuser".to_string(),
        email: "me@example.com".to_string(),
        password: "secret123".to_string(),
    };

    let resp = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_req)
        .send_request(&app)
        .await;

    let auth_resp: AuthResponse = test::read_body_json(resp).await;
    let token = auth_resp.token;

    // Call /auth/me with token
    let resp = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 200);

    let user: UserDto = test::read_body_json(resp).await;
    assert_eq!(user.username, "meuser");
}

/// Test /auth/me endpoint fails without token.
#[tokio::test]
async fn test_get_me_without_token_fails() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    // Call /auth/me without token
    let resp = test::TestRequest::get()
        .uri("/api/auth/me")
        .send_request(&app)
        .await;

    assert!(resp.status().is_client_error());
}
