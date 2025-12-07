//! Blog CLI entry point.

mod commands;
mod constants;

use blog_client::{BlogClient, ClientError};
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use constants::{DEFAULT_GRPC_URL, DEFAULT_HTTP_URL};

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
    commands::execute(&mut client, cli.command).await
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
