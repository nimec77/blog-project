//! Command execution logic.

use blog_client::{BlogClient, ClientError};
use blog_shared::{LoginRequest, RegisterRequest};

use crate::Commands;

/// Executes the given command using the provided client.
pub async fn execute(client: &mut BlogClient, command: Commands) -> Result<(), ClientError> {
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
            println!("Token: {}", response.token);
        }
        Commands::Login { username, password } => {
            let req = LoginRequest { username, password };
            let response = client.login(req).await?;
            println!("✅ Logged in successfully!");
            println!(
                "User: {} (ID: {})",
                response.user.username, response.user.id
            );
            println!("Token: {}", response.token);
        }
        _ => {
            println!("Command not implemented yet");
        }
    }
    Ok(())
}
