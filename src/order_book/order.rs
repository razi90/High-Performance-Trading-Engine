#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    BUY,
    SELL,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    MARKET,
    LIMIT { price: u64 },
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u128,
    pub trader_id: String,
    pub limit_price: u64,
}
