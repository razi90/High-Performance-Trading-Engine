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
            OrderType::MARKET => {
                while order.quantity > 0 && !self.sell_side.is_empty() {
                    let lowest_sell_price = *self.sell_side.keys().next().unwrap();

                    if let Some(sell_orders) = self.sell_side.get_mut(&lowest_sell_price) {
                        if let Some(mut sell_order) = sell_orders.pop_front() {
                            if sell_order.quantity > order.quantity {
                                sell_order.quantity -= order.quantity;

                                order.quantity = 0;

                                sell_orders.push_front(sell_order);
                            } else {
                                let filled_quantity = sell_order.quantity;

                                order.quantity -= filled_quantity;

                                if sell_orders.is_empty() {
                                    self.sell_side.remove(&lowest_sell_price);
                                }
                            }
                        }
                    }
                }

                if order.quantity > 0 {
                    self.place_remaining_buy_order(order);
                }
            }

            OrderType::LIMIT { price: limit_price } => {
                while order.quantity > 0 && !self.sell_side.is_empty() {
                    let lowest_sell_price = self.sell_side.keys().next().cloned();
                }
            }
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
