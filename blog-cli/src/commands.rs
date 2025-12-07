//! Command execution logic.

use blog_client::{BlogClient, ClientError};
use blog_shared::{CreatePostRequest, LoginRequest, RegisterRequest, UpdatePostRequest};

use crate::Commands;

/// Executes the given command using the provided client.
/// Returns the token if login/register succeeded (for persistence).
pub async fn execute(
    client: &mut BlogClient,
    command: Commands,
) -> Result<Option<String>, ClientError> {
    match command {
        Commands::Register {
            username,
            email,
            password,
        } => {
            let req = RegisterRequest {
                username,
                email,
                password,
            };
            let response = client.register(req).await?;
            println!("✅ Registered successfully!");
            println!(
                "User: {} (ID: {})",
                response.user.username, response.user.id
            );
            println!("Token saved to ~/.blog_token");
            Ok(Some(response.token))
        }
        Commands::Login { username, password } => {
            let req = LoginRequest { username, password };
            let response = client.login(req).await?;
            println!("✅ Logged in successfully!");
            println!(
                "User: {} (ID: {})",
                response.user.username, response.user.id
            );
            println!("Token saved to ~/.blog_token");
            Ok(Some(response.token))
        }
        Commands::Create { title, content } => {
            let req = CreatePostRequest { title, content };
            let post = client.create_post(req).await?;
            println!("✅ Post created!");
            println!("ID: {}", post.id);
            println!("Title: {}", post.title);
            Ok(None)
        }
        Commands::Get { id } => {
            let post = client.get_post(id).await?;
            println!("📝 Post #{}", post.id);
            println!("Title: {}", post.title);
            println!("Content: {}", post.content);
            println!("Author: {} (ID: {})", post.author_username, post.author_id);
            println!("Created: {}", post.created_at);
            println!("Updated: {}", post.updated_at);
            Ok(None)
        }
        Commands::List { limit, offset } => {
            let response = client.list_posts(limit, offset).await?;
            println!("📚 Posts ({} total):", response.total);
            for post in response.posts {
                println!("  [{}] {} by {}", post.id, post.title, post.author_username);
            }
            Ok(None)
        }
        Commands::Update { id, title, content } => {
            let req = UpdatePostRequest { title, content };
            let post = client.update_post(id, req).await?;
            println!("✅ Post updated!");
            println!("ID: {}", post.id);
            println!("Title: {}", post.title);
            Ok(None)
        }
        Commands::Delete { id } => {
            client.delete_post(id).await?;
            println!("✅ Post {} deleted!", id);
            Ok(None)
        }
    }
}
