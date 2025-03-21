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

    pub fn place_order(&mut self, mut order: Order) -> Result<(), &'static str> {
        if order.quantity == 0 {
            return Err("Cannot place order with zero quantity");
        }

        order.id = self.generate_order_id();
        self.process_order(order);

        Ok(())
    }

    fn process_order(&mut self, mut order: Order) {
        match order.order_type {
            OrderType::Market => self.match_order(&mut order, None),
            OrderType::Limit { price: limit_price } => {
                self.match_order(&mut order, Some(limit_price))
            }
        }
    }

    fn match_order(&mut self, order: &mut Order, limit_price: Option<u64>) {
        let opposite_side = match order.side {
            OrderSide::Buy => &mut self.sell_side,
            OrderSide::Sell => &mut self.buy_side,
        };

        while order.quantity > 0 && !opposite_side.is_empty() {
            let matching_price = *opposite_side.keys().next().unwrap();

            // Limit price check with different logic for buy and sell
            match (order.side, limit_price) {
                (OrderSide::Buy, Some(limit_price)) => {
                    // For buy order, only match if matching price is less than or equal to limit price
                    if matching_price > limit_price {
                        break;
                    }
                }
                (OrderSide::Sell, Some(limit_price)) => {
                    // For sell order, only match if matching price is greater than or equal to limit price
                    if matching_price < limit_price {
                        break;
                    }
                }
                _ => {} // No limit price check for market orders
            }

            if let Some(matching_orders) = opposite_side.get_mut(&matching_price) {
                if let Some(mut matching_order) = matching_orders.pop_front() {
                    if matching_order.quantity > order.quantity {
                        // Partial fill of matching order
                        matching_order.quantity -= order.quantity;
                        order.quantity = 0;

                        // Put partially filled matching order back
                        matching_orders.push_front(matching_order);
                    } else {
                        // Full fill of matching order
                        let filled_quantity = matching_order.quantity;

                        // Reduce original order quantity
                        order.quantity -= filled_quantity;

                        // Remove price level if no orders left
                        if matching_orders.is_empty() {
                            opposite_side.remove(&matching_price);
                        }
                    }
                }
            }
        }

        // Place remaining order if not fully filled
        if order.quantity > 0 {
            self.place_remaining_order(order.clone());
        }
    }

    fn place_remaining_order(&mut self, order: Order) {
        let book_side = match order.side {
            OrderSide::Buy => &mut self.buy_side,
            OrderSide::Sell => &mut self.sell_side,
        };

        book_side
            .entry(order.price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }
}
