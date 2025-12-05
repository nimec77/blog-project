use tonic::{Request, Response, Status};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use tokio_stream;
use rand::{Rng, SeedableRng};

use crate::exchange::exchange_service_server::ExchangeService;
use crate::exchange::*;

// In-memory хранилище для демонстрации
#[derive(Clone)]
pub struct ExchangeState {
    orders: Arc<RwLock<HashMap<i64, Order>>>,
    balances: Arc<RwLock<HashMap<String, HashMap<String, f64>>>>, // user_id -> currency -> balance
    next_order_id: Arc<RwLock<i64>>,
}

impl ExchangeState {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
            balances: Arc::new(RwLock::new(HashMap::new())),
            next_order_id: Arc::new(RwLock::new(1)),
        }
    }
}

#[derive(Clone)]
pub struct ExchangeServiceImpl {
    state: ExchangeState,
}

impl ExchangeServiceImpl {
    pub fn new() -> Self {
        Self {
            state: ExchangeState::new(),
        }
    }
}

#[tonic::async_trait]
impl ExchangeService for ExchangeServiceImpl {
    async fn create_order(
        &self,
        request: Request<CreateOrderRequest>,
    ) -> Result<Response<OrderResponse>, Status> {
        let req = request.into_inner();
        
        // Валидация
        if req.price <= 0.0 {
            return Err(Status::invalid_argument("Price must be positive"));
        }
        if req.quantity <= 0 {
            return Err(Status::invalid_argument("Quantity must be positive"));
        }
        if req.symbol.is_empty() {
            return Err(Status::invalid_argument("Symbol cannot be empty"));
        }
        
        // Генерация ID ордера
        let order_id = {
            let mut id = self.state.next_order_id.write().await;
            let current = *id;
            *id += 1;
            current
        };
        
        // Создание ордера
        let order = Order {
            id: order_id,
            user_id: req.user_id.clone(),
            symbol: req.symbol.clone(),
            r#type: req.r#type,
            price: req.price,
            quantity: req.quantity,
            filled_quantity: 0,
            status: OrderStatus::Pending as i32,
            created_at: Utc::now().timestamp(),
        };
        
        // Сохранение ордера
        {
            let mut orders = self.state.orders.write().await;
            orders.insert(order_id, order.clone());
        }
        
        tracing::info!(
            "Created order: id={}, user={}, symbol={}, type={:?}, price={}, qty={}",
            order_id,
            req.user_id,
            req.symbol,
            req.r#type,
            req.price,
            req.quantity
        );
        
        Ok(Response::new(OrderResponse { order: Some(order) }))
    }
    
    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<BalanceResponse>, Status> {
        let req = request.into_inner();
        
        let balances = self.state.balances.read().await;
        let user_balances = balances.get(&req.user_id);
        
        let balance = user_balances
            .and_then(|b| b.get(&req.currency))
            .copied()
            .unwrap_or(0.0);
        
        Ok(Response::new(BalanceResponse {
            user_id: req.user_id,
            currency: req.currency,
            balance,
        }))
    }
    
    async fn get_active_orders(
        &self,
        request: Request<GetActiveOrdersRequest>,
    ) -> Result<Response<ActiveOrdersResponse>, Status> {
        let req = request.into_inner();
        
        let orders = self.state.orders.read().await;
        let active_orders: Vec<Order> = orders
            .values()
            .filter(|o| {
                o.user_id == req.user_id
                    && o.status == OrderStatus::Pending as i32
                    && (req.symbol.is_empty() || o.symbol == req.symbol)
            })
            .cloned()
            .collect();
        
        Ok(Response::new(ActiveOrdersResponse {
            orders: active_orders,
        }))
    }
    
    type StreamQuotesStream = tokio_stream::wrappers::ReceiverStream<Result<Quote, Status>>;
    
    async fn stream_quotes(
        &self,
        request: Request<StreamQuotesRequest>,
    ) -> Result<Response<Self::StreamQuotesStream>, Status> {
        let req = request.into_inner();
        
        if req.symbols.is_empty() {
            return Err(Status::invalid_argument("At least one symbol required"));
        }
        
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        
        // Запускаем задачу для отправки котировок
        let symbols = req.symbols.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                
                for symbol in &symbols {
                    // Генерируем случайные котировки для демонстрации
                    let base_price = match symbol.as_str() {
                        "BTC" => 50000.0,
                        "ETH" => 3000.0,
                        "USD" => 1.0,
                        _ => 100.0,
                    };
                    
                    // Используем простой генератор на основе времени для избежания проблем с Send
                    let timestamp = Utc::now().timestamp();
                    let seed = (timestamp as u64) ^ (symbol.len() as u64);
                    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
                    let variation = (rng.gen::<f64>() - 0.5) * 0.02; // ±1%
                    let bid = base_price * (1.0 + variation);
                    let ask = bid * 1.001; // спред 0.1%
                    let last = (bid + ask) / 2.0;
                    
                    let quote = Quote {
                        symbol: symbol.clone(),
                        bid,
                        ask,
                        last,
                        volume: rng.gen_range(0..1000000),
                        timestamp,
                    };
                    
                    if tx.send(Ok(quote)).await.is_err() {
                        // Клиент отключился
                        return;
                    }
                }
            }
        });
        
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
    
    async fn cancel_order(
        &self,
        request: Request<CancelOrderRequest>,
    ) -> Result<Response<CancelOrderResponse>, Status> {
        let req = request.into_inner();
        
        let mut orders = self.state.orders.write().await;
        
        match orders.get_mut(&req.order_id) {
            Some(order) => {
                if order.user_id != req.user_id {
                    return Err(Status::permission_denied("Order belongs to another user"));
                }
                
                if order.status != OrderStatus::Pending as i32 {
                    return Err(Status::failed_precondition(
                        "Only pending orders can be cancelled",
                    ));
                }
                
                order.status = OrderStatus::Cancelled as i32;
                
                tracing::info!("Cancelled order: id={}, user={}", req.order_id, req.user_id);
                
                Ok(Response::new(CancelOrderResponse {
                    success: true,
                    message: "Order cancelled successfully".to_string(),
                }))
            }
            None => Err(Status::not_found("Order not found")),
        }
    }
}

