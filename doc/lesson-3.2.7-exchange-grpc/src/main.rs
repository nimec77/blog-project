mod server;

// Сгенерированный код из proto
pub mod exchange {
    tonic::include_proto!("exchange");
}

use server::ExchangeServiceImpl;
use exchange::exchange_service_server::ExchangeServiceServer;
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    let addr = "127.0.0.1:50051".parse()?;
    let service = ExchangeServiceImpl::new();
    
    info!("🚀 Exchange gRPC server starting on {}", addr);
    
    Server::builder()
        .add_service(ExchangeServiceServer::new(service))
        .serve(addr)
        .await?;
    
    Ok(())
}
