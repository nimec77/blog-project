//! Blog CLI entry point.

mod commands;
mod constants;

use std::fs;
use std::path::PathBuf;

use blog_client::{BlogClient, ClientError};
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use constants::{DEFAULT_GRPC_URL, DEFAULT_HTTP_URL, TOKEN_FILE};

/// Blog platform CLI client.
#[derive(Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI client for the blog platform", long_about = None)]
pub struct Cli {
    /// Use gRPC transport instead of HTTP.
    #[arg(long, global = true)]
    pub grpc: bool,

    /// Server URL (HTTP: http://localhost:8080, gRPC: http://localhost:50051).
    #[arg(long, global = true)]
    pub server: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Register a new user.
    Register {
        /// Username for the new account.
        #[arg(long)]
        username: String,
        /// Email address.
        #[arg(long)]
        email: String,
        /// Password.
        #[arg(long)]
        password: String,
    },
    /// Login to an existing account.
    Login {
        /// Username.
        #[arg(long)]
        username: String,
        /// Password.
        #[arg(long)]
        password: String,
    },
    /// Create a new post.
    Create {
        /// Post title.
        #[arg(long)]
        title: String,
        /// Post content.
        #[arg(long)]
        content: String,
    },
    /// Get a post by ID.
    Get {
        /// Post ID.
        #[arg(long)]
        id: i64,
    },
    /// List all posts.
    List {
        /// Maximum number of posts to return.
        #[arg(long, default_value = "10")]
        limit: i64,
        /// Number of posts to skip.
        #[arg(long, default_value = "0")]
        offset: i64,
    },
    /// Update a post.
    Update {
        /// Post ID.
        #[arg(long)]
        id: i64,
        /// New title (optional).
        #[arg(long)]
        title: Option<String>,
        /// New content (optional).
        #[arg(long)]
        content: Option<String>,
    },
    /// Delete a post.
    Delete {
        /// Post ID.
        #[arg(long)]
        id: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let mut client = create_client(&cli).await?;

    // Load saved token
    if let Some(token) = load_token() {
        client.set_token(token);
    }

    // Execute command and save token if returned
    if let Some(token) = commands::execute(&mut client, cli.command).await?
        && let Err(e) = save_token(&token)
    {
        eprintln!("Warning: Failed to save token: {}", e);
    }

    Ok(())
}

/// Creates a client based on CLI flags.
async fn create_client(cli: &Cli) -> Result<BlogClient, ClientError> {
    if cli.grpc {
        let url = cli.server.as_deref().unwrap_or(DEFAULT_GRPC_URL);
        BlogClient::grpc(url).await
    } else {
        let url = cli.server.as_deref().unwrap_or(DEFAULT_HTTP_URL);
        Ok(BlogClient::http(url))
    }
}

/// Returns the token file path.
fn token_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(TOKEN_FILE))
}

/// Loads token from file if it exists.
fn load_token() -> Option<String> {
    let path = token_path()?;
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Saves token to file.
fn save_token(token: &str) -> std::io::Result<()> {
    if let Some(path) = token_path() {
        fs::write(path, token)?;
    }
    Ok(())
}
