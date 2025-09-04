use my_order_book::{OrderBook, OrderModify, Side, OrderType, OrderStatus};

#[test]
fn can_add_order_and_query_bbo() {
    let mut ob = OrderBook::new();
    let (id, trades) = ob.add_order(Side::Buy, OrderType::Limit, 100, 5);
    assert_eq!(id, 0);
    assert!(trades.is_empty());
    assert_eq!(ob.get_best_bid(), Some(100));
    assert_eq!(ob.get_best_ask(), None);
}

#[test]
fn test_basic_order_matching() {
    let mut ob = OrderBook::new();
    
    // Add non-crossing orders
    let (buy_id, trades1) = ob.add_order(Side::Buy, OrderType::Limit, 100, 10);
    let (sell_id, trades2) = ob.add_order(Side::Sell, OrderType::Limit, 105, 5);
    
    assert_eq!(buy_id, 0);
    assert_eq!(sell_id, 1);
    assert!(trades1.is_empty());
    assert!(trades2.is_empty());
    
    // Add crossing order
    let (crossing_id, trades3) = ob.add_order(Side::Buy, OrderType::Limit, 105, 3);
    assert_eq!(crossing_id, 2);
    assert_eq!(trades3.len(), 1);
    
    let trade = &trades3[0];
    assert_eq!(trade.bid_info.order_id, 2);
    assert_eq!(trade.ask_info.order_id, 1);
    assert_eq!(trade.bid_info.quantity, 3);
    assert_eq!(trade.ask_info.quantity, 3);
}

#[test]
fn test_market_orders() {
    let mut ob = OrderBook::new();
    
    // Add limit order first
    let (_, _) = ob.add_order(Side::Sell, OrderType::Limit, 105, 10);
    
    // Add market buy order - should convert to limit at worst ask
    let (market_id, trades) = ob.add_order(Side::Buy, OrderType::Market, 0, 5);
    assert_eq!(market_id, 1);
    assert_eq!(trades.len(), 1);
    
    let trade = &trades[0];
    assert_eq!(trade.bid_info.quantity, 5);
    assert_eq!(trade.ask_info.quantity, 5);
}

#[test]
fn test_fill_or_kill() {
    let mut ob = OrderBook::new();
    
    // Add partial liquidity
    let (_, _) = ob.add_order(Side::Sell, OrderType::Limit, 105, 5);
    
    // Try Fill or Kill for more than available - should be rejected
    let (fok_id, trades) = ob.add_order(Side::Buy, OrderType::FillOrKill, 105, 10);
    assert_eq!(fok_id, 1);
    assert!(trades.is_empty());
    assert_eq!(ob.size(), 1); // Only the original sell order remains
    
    // Try Fill or Kill for exact amount - should work
    let (fok_id2, trades2) = ob.add_order(Side::Buy, OrderType::FillOrKill, 105, 5);
    assert_eq!(fok_id2, 2);
    assert_eq!(trades2.len(), 1);
    assert_eq!(ob.size(), 0); // Both orders should be filled
}

#[test]
fn test_order_modification() {
    let mut ob = OrderBook::new();
    
    // Add an order
    let (order_id, _) = ob.add_order(Side::Buy, OrderType::Limit, 100, 10);
    assert_eq!(ob.size(), 1);
    
    // Modify the order
    let order_modify = OrderModify::new(order_id, Side::Buy, 101, 15);
    let trades = ob.modify_order(order_modify);
    assert!(trades.is_empty());
    assert_eq!(ob.size(), 1);
    assert_eq!(ob.get_best_bid(), Some(101));
}

#[test]
fn test_order_cancellation() {
    let mut ob = OrderBook::new();
    
    // Add orders
    let (buy_id, _) = ob.add_order(Side::Buy, OrderType::Limit, 100, 10);
    let (sell_id, _) = ob.add_order(Side::Sell, OrderType::Limit, 105, 5);
    assert_eq!(ob.size(), 2);
    
    // Cancel buy order
    ob.cancel_order(buy_id);
    assert_eq!(ob.size(), 1);
    assert_eq!(ob.get_best_bid(), None);
    assert_eq!(ob.get_best_ask(), Some(105));
    
    // Cancel sell order
    ob.cancel_order(sell_id);
    assert_eq!(ob.size(), 0);
    assert_eq!(ob.get_best_ask(), None);
}

#[test]
fn test_order_book_levels() {
    let mut ob = OrderBook::new();
    
    // Add multiple orders at different levels
    let (_, _) = ob.add_order(Side::Buy, OrderType::Limit, 100, 10);
    let (_, _) = ob.add_order(Side::Buy, OrderType::Limit, 99, 5);
    let (_, _) = ob.add_order(Side::Sell, OrderType::Limit, 105, 8);
    let (_, _) = ob.add_order(Side::Sell, OrderType::Limit, 106, 3);
    
    let level_infos = ob.get_order_infos();
    
    // Check bid levels (should be sorted by price descending)
    assert_eq!(level_infos.bids.len(), 2);
    assert_eq!(level_infos.bids[0].price, 100);
    assert_eq!(level_infos.bids[0].quantity, 10);
    assert_eq!(level_infos.bids[1].price, 99);
    assert_eq!(level_infos.bids[1].quantity, 5);
    
    // Check ask levels (should be sorted by price ascending)
    assert_eq!(level_infos.asks.len(), 2);
    assert_eq!(level_infos.asks[0].price, 105);
    assert_eq!(level_infos.asks[0].quantity, 8);
    assert_eq!(level_infos.asks[1].price, 106);
    assert_eq!(level_infos.asks[1].quantity, 3);
}

#[test]
fn test_partial_fills() {
    let mut ob = OrderBook::new();
    
    // Add large sell order
    let (_sell_id, _) = ob.add_order(Side::Sell, OrderType::Limit, 105, 20);
    
    // Add smaller buy order - should partially fill
    let (_buy_id, trades) = ob.add_order(Side::Buy, OrderType::Limit, 105, 8);
    
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].bid_info.quantity, 8);
    assert_eq!(trades[0].ask_info.quantity, 8);
    
    // Check remaining quantities
    let level_infos = ob.get_order_infos();
    assert_eq!(level_infos.asks[0].quantity, 12); // 20 - 8 = 12 remaining
    assert!(level_infos.bids.is_empty()); // Buy order should be fully filled
}

#[test]
fn test_order_status_accepted() {
    let mut ob = OrderBook::new();

    // Non-crossing orders should be accepted
    let result1 = ob.add_order_with_status(Side::Buy, OrderType::Limit, 100, 10);
    assert_eq!(result1.status, OrderStatus::Accepted);
    assert_eq!(result1.order_id, 0);
    assert!(result1.trades.is_empty());

    let result2 = ob.add_order_with_status(Side::Sell, OrderType::Limit, 105, 5);
    assert_eq!(result2.status, OrderStatus::Accepted);
    assert_eq!(result2.order_id, 1);
    assert!(result2.trades.is_empty());
}

#[test]
fn test_order_status_executed() {
    let mut ob = OrderBook::new();

    // Add a sell order first
    let result1 = ob.add_order_with_status(Side::Sell, OrderType::Limit, 100, 10);
    assert_eq!(result1.status, OrderStatus::Accepted);

    // Add crossing buy order - should execute
    let result2 = ob.add_order_with_status(Side::Buy, OrderType::Limit, 100, 5);
    assert_eq!(result2.status, OrderStatus::Executed);
    assert_eq!(result2.trades.len(), 1);
    assert_eq!(result2.trades[0].bid_info.order_id, 1);
    assert_eq!(result2.trades[0].ask_info.order_id, 0);
}

#[test]
fn test_order_status_rejected_no_liquidity() {
    let mut ob = OrderBook::new();

    // Market buy with no asks should be rejected
    let result = ob.add_order_with_status(Side::Buy, OrderType::Market, 0, 10);
    assert_eq!(result.status, OrderStatus::RejectedNoLiquidity);
    assert!(result.trades.is_empty());
    assert_eq!(ob.size(), 0);

    // Market sell with no bids should be rejected
    let result2 = ob.add_order_with_status(Side::Sell, OrderType::Market, 0, 10);
    assert_eq!(result2.status, OrderStatus::RejectedNoLiquidity);
    assert!(result2.trades.is_empty());
    assert_eq!(ob.size(), 0);
}

#[test]
fn test_order_status_rejected_fill_and_kill_no_match() {
    let mut ob = OrderBook::new();

    // Add a sell order at 105
    ob.add_order_with_status(Side::Sell, OrderType::Limit, 105, 10);

    // Fill-and-Kill buy at 100 (below ask) should be rejected
    let result = ob.add_order_with_status(Side::Buy, OrderType::FillAndKill, 100, 5);
    assert_eq!(result.status, OrderStatus::RejectedFillAndKillNoMatch);
    assert!(result.trades.is_empty());
    assert_eq!(ob.size(), 1); // Only the original sell order remains
}

#[test]
fn test_order_status_rejected_fill_or_kill_partial_fill() {
    let mut ob = OrderBook::new();

    // Add partial liquidity - only 5 shares available
    ob.add_order_with_status(Side::Sell, OrderType::Limit, 100, 5);

    // Fill-or-Kill for 10 shares (more than available) should be rejected
    let result = ob.add_order_with_status(Side::Buy, OrderType::FillOrKill, 100, 10);
    assert_eq!(result.status, OrderStatus::RejectedFillOrKillPartialFill);
    assert!(result.trades.is_empty());
    assert_eq!(ob.size(), 1); // Only the original sell order remains

    // Fill-or-Kill for exact amount should work
    let result2 = ob.add_order_with_status(Side::Buy, OrderType::FillOrKill, 100, 5);
    assert_eq!(result2.status, OrderStatus::Executed);
    assert_eq!(result2.trades.len(), 1);
    assert_eq!(ob.size(), 0); // Both orders should be filled
}

#[test]
fn test_order_status_messages() {
    // Test that all status messages are meaningful
    assert_eq!(OrderStatus::Accepted.message(), "Order accepted and placed in order book");
    assert_eq!(OrderStatus::Executed.message(), "Order executed successfully");
    assert_eq!(OrderStatus::RejectedNoLiquidity.message(), "Market order rejected: no liquidity available on opposite side");
    assert_eq!(OrderStatus::RejectedFillAndKillNoMatch.message(), "Fill-and-Kill order rejected: no matching orders available");
    assert_eq!(OrderStatus::RejectedFillOrKillPartialFill.message(), "Fill-or-Kill order rejected: insufficient liquidity for complete fill");
    assert_eq!(OrderStatus::RejectedDuplicateId.message(), "Order rejected: duplicate order ID");
}

#[test]
fn test_mixed_order_methods() {
    let mut ob = OrderBook::new();

    // Test that both old and new methods work together
    let (old_id, old_trades) = ob.add_order(Side::Buy, OrderType::Limit, 100, 10);
    let new_result = ob.add_order_with_status(Side::Sell, OrderType::Limit, 105, 5);

    assert_eq!(old_id, 0);
    assert!(old_trades.is_empty());
    assert_eq!(new_result.order_id, 1);
    assert_eq!(new_result.status, OrderStatus::Accepted);
    assert!(new_result.trades.is_empty());

    // Add crossing order with new method
    let crossing_result = ob.add_order_with_status(Side::Buy, OrderType::Limit, 106, 3);
    assert_eq!(crossing_result.status, OrderStatus::Executed);
    assert_eq!(crossing_result.trades.len(), 1);
}
