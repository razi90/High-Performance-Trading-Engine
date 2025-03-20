use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

use super::{Order, OrderSide, OrderType, TradeRecorder};

pub struct OrderBook {
    buy_side: BTreeMap<u64, VecDeque<Order>>,
    sell_side: BTreeMap<u64, VecDeque<Order>>,
    next_order_id: Arc<Mutex<u64>>,
    trade_recorder: Option<TradeRecorder>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_side: BTreeMap::new(),
            sell_side: BTreeMap::new(),
            next_order_id: Arc::new(Mutex::new(1)),
            trade_recorder: None,
        }
    }

    pub fn set_trade_recorder(&mut self, recorder: TradeRecorder) {
        self.trade_recorder = Some(recorder);
    }

    fn generate_order_id(&self) -> u64 {
        let mut id = self.next_order_id.lock().unwrap();
        let next_id = *id;
        *id += 1;
        next_id
    }

    fn place_order(&mut self, mut order: Order) -> Result<(), &'static str> {
        if order.quantity == 0 {
            return Err("Cannot place order with zero quantity");
        }

        order.id = self.generate_order_id();

        match order.side {
            OrderSide::BUY => self.process_buy_order(order),
            OrderSide::SELL => self.process_sell_order(order),
        }

        Ok(())
    }

    fn process_buy_order(&mut self, mut order: Order) {
        match order.order_type {
            OrderType::MARKET => self.match_order(&mut order, None),
            OrderType::LIMIT { price: limit_price } => {
                self.match_order(&mut order, Some(limit_price))
            }
        }
    }

    fn match_order(&mut self, order: &mut Order, limit_price: Option<u64>) {
        while order.quantity > 0 && !self.sell_side.is_empty() {
            let lowest_sell_price = *self.sell_side.keys().next().unwrap();

            // Check limit price if applicable
            if let Some(price_limit) = limit_price {
                if lowest_sell_price > price_limit {
                    break;
                }
            }

            if let Some(sell_orders) = self.sell_side.get_mut(&lowest_sell_price) {
                if let Some(mut sell_order) = sell_orders.pop_front() {
                    if sell_order.quantity > order.quantity {
                        // Partial fill of sell order
                        sell_order.quantity -= order.quantity;
                        order.quantity = 0;

                        // Put partially filled sell order back
                        sell_orders.push_front(sell_order);
                    } else {
                        // Full fill of sell order
                        let filled_quantity = sell_order.quantity;

                        // Reduce buy order quantity
                        order.quantity -= filled_quantity;

                        // Remove price level if no orders left
                        if sell_orders.is_empty() {
                            self.sell_side.remove(&lowest_sell_price);
                        }
                    }
                }
            }
        }

        // Place remaining order if not fully filled
        if order.quantity > 0 {
            self.place_remaining_buy_order(order.clone());
        }
    }

    fn process_sell_order(&mut self, order: Order) {}

    fn place_remaining_buy_order(&mut self, order: Order) {
        self.buy_side
            .entry(order.price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }

    fn place_remaining_sell_order(&mut self, order: Order) {
        self.sell_side
            .entry(order.price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }
}
