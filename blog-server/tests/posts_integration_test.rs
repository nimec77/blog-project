//! Integration tests for posts endpoints.

mod common;

use std::sync::Arc;

use actix_web::{App, test, web};
use blog_shared::{
    AuthResponse, CreatePostRequest, PostDto, PostListResponse, RegisterRequest, UpdatePostRequest,
};

use blog_server::application::{AuthService, BlogService};
use blog_server::data::{PostRepository, UserRepository};
use blog_server::presentation::JwtSecret;
use blog_server::presentation::http_handlers::api_routes;

use common::{TEST_JWT_SECRET, setup_test_db};

/// Macro to register a user and get their token.
macro_rules! register_user {
    ($app:expr, $username:expr, $email:expr, $password:expr) => {{
        let req = RegisterRequest {
            username: $username.to_string(),
            email: $email.to_string(),
            password: $password.to_string(),
        };

        let resp = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(&req)
            .send_request($app)
            .await;

        let auth_resp: AuthResponse = test::read_body_json(resp).await;
        auth_resp.token
    }};
}

/// Test listing posts when database is empty.
#[tokio::test]
async fn test_list_posts_empty() {
    let pool = setup_test_db().await;
    let post_repo = Arc::new(PostRepository::new(pool));
    let blog_service = BlogService::new(Arc::clone(&post_repo));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let resp = test::TestRequest::get()
        .uri("/api/posts")
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 200);

    let list_resp: PostListResponse = test::read_body_json(resp).await;
    assert_eq!(list_resp.posts.len(), 0);
    assert_eq!(list_resp.total, 0);
}

/// Test creating a post requires authentication.
#[tokio::test]
async fn test_create_post_requires_auth() {
    let pool = setup_test_db().await;
    let post_repo = Arc::new(PostRepository::new(pool));
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let req = CreatePostRequest {
        title: "Test Post".to_string(),
        content: "Test content".to_string(),
    };

    // Try to create post without token
    let resp = test::TestRequest::post()
        .uri("/api/posts")
        .set_json(&req)
        .send_request(&app)
        .await;

    assert!(resp.status().is_client_error());
}

/// Test successfully creating a post with authentication.
#[tokio::test]
async fn test_create_post_success() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let token = register_user!(&app, "postauthor", "author@example.com", "secret123");

    let req = CreatePostRequest {
        title: "My First Post".to_string(),
        content: "This is the content of my first post.".to_string(),
    };

    let resp = test::TestRequest::post()
        .uri("/api/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&req)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 201);

    let post: PostDto = test::read_body_json(resp).await;
    assert_eq!(post.title, "My First Post");
    assert_eq!(post.content, "This is the content of my first post.");
    assert_eq!(post.author_username, "postauthor");
}

/// Test getting a post by ID.
#[tokio::test]
async fn test_get_post_by_id() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let token = register_user!(&app, "getpostuser", "getpost@example.com", "secret123");

    // Create a post
    let create_req = CreatePostRequest {
        title: "Post to Get".to_string(),
        content: "Content to retrieve".to_string(),
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_req)
        .send_request(&app)
        .await;

    let created_post: PostDto = test::read_body_json(create_resp).await;

    // Get the post by ID
    let get_resp = test::TestRequest::get()
        .uri(&format!("/api/posts/{}", created_post.id))
        .send_request(&app)
        .await;

    assert_eq!(get_resp.status(), 200);

    let retrieved_post: PostDto = test::read_body_json(get_resp).await;
    assert_eq!(retrieved_post.id, created_post.id);
    assert_eq!(retrieved_post.title, "Post to Get");
}

/// Test updating a post by the author.
#[tokio::test]
async fn test_update_post_by_author() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let token = register_user!(&app, "updateauthor", "update@example.com", "secret123");

    // Create a post
    let create_req = CreatePostRequest {
        title: "Original Title".to_string(),
        content: "Original content".to_string(),
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_req)
        .send_request(&app)
        .await;

    let created_post: PostDto = test::read_body_json(create_resp).await;

    // Update the post
    let update_req = UpdatePostRequest {
        title: Some("Updated Title".to_string()),
        content: None,
    };

    let update_resp = test::TestRequest::put()
        .uri(&format!("/api/posts/{}", created_post.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_req)
        .send_request(&app)
        .await;

    assert_eq!(update_resp.status(), 200);

    let updated_post: PostDto = test::read_body_json(update_resp).await;
    assert_eq!(updated_post.title, "Updated Title");
    assert_eq!(updated_post.content, "Original content");
}

/// Test updating a post by a non-author fails.
#[tokio::test]
async fn test_update_post_by_non_author_fails() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let author_token = register_user!(&app, "postowner", "owner@example.com", "secret123");
    let other_token = register_user!(&app, "otherperson", "other@example.com", "secret456");

    // Author creates a post
    let create_req = CreatePostRequest {
        title: "Owner's Post".to_string(),
        content: "This is my post".to_string(),
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/posts")
        .insert_header(("Authorization", format!("Bearer {}", author_token)))
        .set_json(&create_req)
        .send_request(&app)
        .await;

    let created_post: PostDto = test::read_body_json(create_resp).await;

    // Other user tries to update it
    let update_req = UpdatePostRequest {
        title: Some("Hacked Title".to_string()),
        content: None,
    };

    let update_resp = test::TestRequest::put()
        .uri(&format!("/api/posts/{}", created_post.id))
        .insert_header(("Authorization", format!("Bearer {}", other_token)))
        .set_json(&update_req)
        .send_request(&app)
        .await;

    assert_eq!(update_resp.status(), 403);
}

/// Test deleting a post by the author.
#[tokio::test]
async fn test_delete_post_by_author() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let token = register_user!(&app, "deleteuser", "delete@example.com", "secret123");

    // Create a post
    let create_req = CreatePostRequest {
        title: "Post to Delete".to_string(),
        content: "Will be deleted".to_string(),
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_req)
        .send_request(&app)
        .await;

    let created_post: PostDto = test::read_body_json(create_resp).await;

    // Delete the post
    let delete_resp = test::TestRequest::delete()
        .uri(&format!("/api/posts/{}", created_post.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_request(&app)
        .await;

    assert_eq!(delete_resp.status(), 204);

    // Verify post is gone
    let get_resp = test::TestRequest::get()
        .uri(&format!("/api/posts/{}", created_post.id))
        .send_request(&app)
        .await;

    assert!(get_resp.status().is_client_error());
}

/// Test deleting a post by a non-author fails.
#[tokio::test]
async fn test_delete_post_by_non_author_fails() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let author_token = register_user!(&app, "delowner", "delowner@example.com", "secret123");
    let other_token = register_user!(&app, "delother", "delother@example.com", "secret456");

    // Author creates a post
    let create_req = CreatePostRequest {
        title: "Protected Post".to_string(),
        content: "Cannot be deleted by others".to_string(),
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/posts")
        .insert_header(("Authorization", format!("Bearer {}", author_token)))
        .set_json(&create_req)
        .send_request(&app)
        .await;

    let created_post: PostDto = test::read_body_json(create_resp).await;

    // Other user tries to delete it
    let delete_resp = test::TestRequest::delete()
        .uri(&format!("/api/posts/{}", created_post.id))
        .insert_header(("Authorization", format!("Bearer {}", other_token)))
        .send_request(&app)
        .await;

    assert_eq!(delete_resp.status(), 403);
}

/// Test listing posts with pagination.
#[tokio::test]
async fn test_list_posts_pagination() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool));
    let auth_service = AuthService::new(Arc::clone(&user_repo), TEST_JWT_SECRET.to_string());
    let blog_service = BlogService::new(Arc::clone(&post_repo));
    let jwt_secret = JwtSecret(TEST_JWT_SECRET.to_string());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_secret))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .service(web::scope("/api").service(api_routes())),
    )
    .await;

    let token = register_user!(&app, "pagination", "pagination@example.com", "secret123");

    // Create 5 posts
    for i in 1..=5 {
        let req = CreatePostRequest {
            title: format!("Post {}", i),
            content: format!("Content {}", i),
        };

        test::TestRequest::post()
            .uri("/api/posts")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&req)
            .send_request(&app)
            .await;
    }

    // Get first 3 posts
    let resp = test::TestRequest::get()
        .uri("/api/posts?limit=3&offset=0")
        .send_request(&app)
        .await;

    let list_resp: PostListResponse = test::read_body_json(resp).await;
    assert_eq!(list_resp.posts.len(), 3);
    assert_eq!(list_resp.total, 5);

    // Get next 2 posts
    let resp = test::TestRequest::get()
        .uri("/api/posts?limit=3&offset=3")
        .send_request(&app)
        .await;

    let list_resp: PostListResponse = test::read_body_json(resp).await;
    assert_eq!(list_resp.posts.len(), 2);
    assert_eq!(list_resp.total, 5);
}
