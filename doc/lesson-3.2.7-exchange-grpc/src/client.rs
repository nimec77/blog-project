use crate::exchange::exchange_service_client::ExchangeServiceClient;
use crate::exchange::*;
use tonic::Request;

pub struct ExchangeClient {
    client: ExchangeServiceClient<tonic::transport::Channel>,
}

impl ExchangeClient {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ExchangeServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
    
    pub async fn create_order(
        &mut self,
        user_id: String,
        symbol: String,
        order_type: OrderType,
        price: f64,
        quantity: i64,
    ) -> Result<Order, Box<dyn std::error::Error>> {
        let request = Request::new(CreateOrderRequest {
            user_id,
            symbol,
            r#type: order_type as i32,
            price,
            quantity,
        });
        
        let response = self.client.create_order(request).await?;
        let order = response.into_inner().order.ok_or("Order not found in response")?;
        Ok(order)
    }
    
    pub async fn get_balance(
        &mut self,
        user_id: String,
        currency: String,
    ) -> Result<BalanceResponse, Box<dyn std::error::Error>> {
        let request = Request::new(GetBalanceRequest { user_id, currency });
        let response = self.client.get_balance(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn get_active_orders(
        &mut self,
        user_id: String,
        symbol: String,
    ) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
        let request = Request::new(GetActiveOrdersRequest { user_id, symbol });
        let response = self.client.get_active_orders(request).await?;
        Ok(response.into_inner().orders)
    }
    
    pub async fn stream_quotes(
        &mut self,
        symbols: Vec<String>,
    ) -> Result<tonic::Streaming<Quote>, Box<dyn std::error::Error>> {
        let request = Request::new(StreamQuotesRequest { symbols });
        let response = self.client.stream_quotes(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn cancel_order(
        &mut self,
        user_id: String,
        order_id: i64,
    ) -> Result<CancelOrderResponse, Box<dyn std::error::Error>> {
        let request = Request::new(CancelOrderRequest { user_id, order_id });
        let response = self.client.cancel_order(request).await?;
        Ok(response.into_inner())
    }
}

