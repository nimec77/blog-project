use exchange_grpc::client::ExchangeClient;
use exchange_grpc::exchange::OrderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    let mut client = ExchangeClient::connect("http://127.0.0.1:50051".to_string()).await?;
    
    let user_id = "user_123".to_string();
    
    // Создание ордера на покупку
    println!("Creating buy order...");
    let order = client
        .create_order(
            user_id.clone(),
            "BTC".to_string(),
            OrderType::Buy,
            50000.0,
            100,
        )
        .await?;
    println!("Created order: id={}, symbol={}, type={:?}, price={}, qty={}", 
        order.id, order.symbol, order.r#type, order.price, order.quantity);
    
    // Получение баланса
    println!("\nGetting balance...");
    let balance = client
        .get_balance(user_id.clone(), "USD".to_string())
        .await?;
    println!("Balance: user={}, currency={}, balance={}", 
        balance.user_id, balance.currency, balance.balance);
    
    // Получение активных ордеров
    println!("\nGetting active orders...");
    let orders = client
        .get_active_orders(user_id.clone(), "".to_string())
        .await?;
    println!("Active orders: {}", orders.len());
    for order in &orders {
        println!("  - Order {}: {} {} @ {}", order.id, order.symbol, order.quantity, order.price);
    }
    
    // Поток котировок
    println!("\nStreaming quotes (5 messages)...");
    let mut stream = client
        .stream_quotes(vec!["BTC".to_string(), "ETH".to_string()])
        .await?;
    
    for _ in 0..5 {
        match stream.message().await? {
            Some(quote) => {
                println!(
                    "Quote: {} | bid={:.2} ask={:.2} last={:.2} volume={}",
                    quote.symbol, quote.bid, quote.ask, quote.last, quote.volume
                );
            }
            None => break,
        }
    }
    
    // Отмена ордера
    if !orders.is_empty() {
        println!("\nCancelling order...");
        let cancel_response = client
            .cancel_order(user_id.clone(), orders[0].id)
            .await?;
        println!("Cancel response: success={}, message={}", 
            cancel_response.success, cancel_response.message);
    }
    
    Ok(())
}

