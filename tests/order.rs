use my_order_book::{Order, Side, OrderType};

#[test]
fn test_order_creation() {
    let order = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    
    assert_eq!(order.get_order_id(), 1);
    assert_eq!(order.get_side(), Side::Buy);
    assert_eq!(order.get_order_type(), OrderType::Limit);
    assert_eq!(order.get_price(), 100);
    assert_eq!(order.get_initial_quantity(), 10);
    assert_eq!(order.get_remaining_quantity(), 10);
    assert_eq!(order.filled_quantity, 0);
    assert!(!order.is_filled());
}

#[test]
fn test_order_getters() {
    let buy_order = Order::new(42, Side::Buy, OrderType::Limit, 150, 25);
    
    assert_eq!(buy_order.get_order_id(), 42);
    assert_eq!(buy_order.get_side(), Side::Buy);
    assert_eq!(buy_order.get_order_type(), OrderType::Limit);
    assert_eq!(buy_order.get_price(), 150);
    assert_eq!(buy_order.get_initial_quantity(), 25);
    
    let sell_order = Order::new(99, Side::Sell, OrderType::Market, 0, 5);
    
    assert_eq!(sell_order.get_order_id(), 99);
    assert_eq!(sell_order.get_side(), Side::Sell);
    assert_eq!(sell_order.get_order_type(), OrderType::Market);
    assert_eq!(sell_order.get_price(), 0);
    assert_eq!(sell_order.get_initial_quantity(), 5);
}

#[test]
fn test_order_fill() {
    let mut order = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    
    // Initially not filled
    assert!(!order.is_filled());
    assert_eq!(order.get_remaining_quantity(), 10);
    
    // Partial fill
    order.fill(3);
    assert!(!order.is_filled());
    assert_eq!(order.filled_quantity, 3);
    assert_eq!(order.get_remaining_quantity(), 7);
    
    // Another partial fill
    order.fill(4);
    assert!(!order.is_filled());
    assert_eq!(order.filled_quantity, 7);
    assert_eq!(order.get_remaining_quantity(), 3);
    
    // Complete fill
    order.fill(3);
    assert!(order.is_filled());
    assert_eq!(order.filled_quantity, 10);
    assert_eq!(order.get_remaining_quantity(), 0);
}

#[test]
fn test_order_overfill_protection() {
    let mut order = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    
    // Try to fill more than the order quantity
    order.fill(15);
    
    // Should be capped at the order quantity
    assert!(order.is_filled());
    assert_eq!(order.filled_quantity, 10);
    assert_eq!(order.get_remaining_quantity(), 0);
}

#[test]
fn test_order_multiple_fills_with_overfill() {
    let mut order = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    
    // Partial fill
    order.fill(7);
    assert_eq!(order.filled_quantity, 7);
    assert_eq!(order.get_remaining_quantity(), 3);
    
    // Try to fill more than remaining
    order.fill(5);
    
    // Should be capped at total order quantity
    assert!(order.is_filled());
    assert_eq!(order.filled_quantity, 10);
    assert_eq!(order.get_remaining_quantity(), 0);
}

#[test]
fn test_to_good_till_cancel() {
    let mut market_order = Order::new(1, Side::Buy, OrderType::Market, 0, 10);
    
    assert_eq!(market_order.get_order_type(), OrderType::Market);
    assert_eq!(market_order.get_price(), 0);
    
    // Convert to GTC with specific price
    market_order.to_good_till_cancel(105);
    
    assert_eq!(market_order.get_order_type(), OrderType::GoodTillCancel);
    assert_eq!(market_order.get_price(), 105);
    
    // Other properties should remain unchanged
    assert_eq!(market_order.get_order_id(), 1);
    assert_eq!(market_order.get_side(), Side::Buy);
    assert_eq!(market_order.get_initial_quantity(), 10);
}

#[test]
fn test_order_types() {
    let limit_order = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    assert_eq!(limit_order.get_order_type(), OrderType::Limit);
    
    let market_order = Order::new(2, Side::Sell, OrderType::Market, 0, 5);
    assert_eq!(market_order.get_order_type(), OrderType::Market);
    
    let fok_order = Order::new(3, Side::Buy, OrderType::FillOrKill, 95, 15);
    assert_eq!(fok_order.get_order_type(), OrderType::FillOrKill);
    
    let gtc_order = Order::new(4, Side::Sell, OrderType::GoodTillCancel, 110, 8);
    assert_eq!(gtc_order.get_order_type(), OrderType::GoodTillCancel);
}

#[test]
fn test_order_sides() {
    let buy_order = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    assert_eq!(buy_order.get_side(), Side::Buy);
    
    let sell_order = Order::new(2, Side::Sell, OrderType::Limit, 105, 5);
    assert_eq!(sell_order.get_side(), Side::Sell);
}

#[test]
fn test_order_clone_and_equality() {
    let order1 = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    let order2 = order1.clone();
    
    assert_eq!(order1, order2);
    assert_eq!(order1.get_order_id(), order2.get_order_id());
    assert_eq!(order1.get_side(), order2.get_side());
    assert_eq!(order1.get_price(), order2.get_price());
    assert_eq!(order1.get_initial_quantity(), order2.get_initial_quantity());
}

#[test]
fn test_order_inequality_after_fill() {
    let mut order1 = Order::new(1, Side::Buy, OrderType::Limit, 100, 10);
    let order2 = order1.clone();
    
    // Initially equal
    assert_eq!(order1, order2);
    
    // After filling, they should be different
    order1.fill(5);
    assert_ne!(order1, order2);
    assert_ne!(order1.filled_quantity, order2.filled_quantity);
}
