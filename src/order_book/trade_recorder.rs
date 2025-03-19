pub struct TradeRecorder {
    // trades: Vec<Trade>,
}

impl TradeRecorder {
    pub fn new() -> Self {
        TradeRecorder {
            // trades: Vec::new()
        }
    }
    pub fn record_trade(
        &mut self,
        buy_order_id: u64,
        sell_order_id: u64,
        quantity: u64,
        price: u64,
    ) {
        // TBD
    }
}
