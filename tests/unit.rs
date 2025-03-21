#[cfg(test)]
mod tests {
    use High_Performance_Trading_Engine::order_book::{Order, OrderBook, OrderSide, OrderType};

    // Helper function to create a test order
    fn create_test_order(
        side: OrderSide,
        order_type: OrderType,
        price: u64,
        quantity: u64,
    ) -> Order {
        Order {
            id: 0, // Will be set by OrderBook
            side,
            order_type,
            price,
            quantity,
            timestamp: 0,
            trader_id: "test_trader".to_string(),
        }
    }

    #[test]
    fn test_place_market_buy_order() {
        let mut order_book = OrderBook::new();

        // Create some sell orders
        let sell_order1 = create_test_order(OrderSide::Sell, OrderType::Market, 100, 50);
        let sell_order2 = create_test_order(OrderSide::Sell, OrderType::Market, 100, 30);

        // Place sell orders
        order_book.place_order(sell_order1).unwrap();
        order_book.place_order(sell_order2).unwrap();

        // Create a buy market order
        let buy_order = create_test_order(OrderSide::Buy, OrderType::Market, 100, 70);

        // Place buy order
        let result = order_book.place_order(buy_order);

        // Assert order was processed successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_place_limit_buy_order() {
        let mut order_book = OrderBook::new();

        // Create some sell orders
        let sell_order1 =
            create_test_order(OrderSide::Sell, OrderType::Limit { price: 100 }, 100, 50);
        let sell_order2 =
            create_test_order(OrderSide::Sell, OrderType::Limit { price: 100 }, 100, 30);

        // Place sell orders
        order_book.place_order(sell_order1).unwrap();
        order_book.place_order(sell_order2).unwrap();

        // Create a buy limit order
        let buy_order = create_test_order(OrderSide::Buy, OrderType::Limit { price: 110 }, 110, 70);

        // Place buy order
        let result = order_book.place_order(buy_order);

        // Assert order was processed successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_zero_quantity_order() {
        let mut order_book = OrderBook::new();

        // Create an order with zero quantity
        let order = create_test_order(OrderSide::Buy, OrderType::Market, 100, 0);

        // Place order should return an error
        let result = order_book.place_order(order);

        // Assert error was returned
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot place order with zero quantity");
    }

    #[test]
    fn test_partial_fill() {
        let mut order_book = OrderBook::new();

        // Create a sell order
        let sell_order =
            create_test_order(OrderSide::Sell, OrderType::Limit { price: 100 }, 100, 50);

        // Place sell order
        order_book.place_order(sell_order).unwrap();

        // Create a buy order larger than sell order
        let buy_order = create_test_order(OrderSide::Buy, OrderType::Market, 100, 70);

        // Place buy order
        let result = order_book.place_order(buy_order);

        // Assert order was processed successfully
        assert!(result.is_ok());
    }
}
