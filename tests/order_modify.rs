use my_order_book::{OrderModify, Side, OrderType};

#[test]
fn test_order_modify_creation() {
    let order_modify = OrderModify::new(42, Side::Buy, 150, 25);
    
    assert_eq!(order_modify.get_order_id(), 42);
    assert_eq!(order_modify.get_side(), Side::Buy);
    assert_eq!(order_modify.get_price(), 150);
    assert_eq!(order_modify.get_quantity(), 25);
}

#[test]
fn test_order_modify_getters() {
    let buy_modify = OrderModify::new(1, Side::Buy, 100, 10);
    
    assert_eq!(buy_modify.get_order_id(), 1);
    assert_eq!(buy_modify.get_side(), Side::Buy);
    assert_eq!(buy_modify.get_price(), 100);
    assert_eq!(buy_modify.get_quantity(), 10);
    
    let sell_modify = OrderModify::new(99, Side::Sell, 200, 5);
    
    assert_eq!(sell_modify.get_order_id(), 99);
    assert_eq!(sell_modify.get_side(), Side::Sell);
    assert_eq!(sell_modify.get_price(), 200);
    assert_eq!(sell_modify.get_quantity(), 5);
}

#[test]
fn test_order_modify_to_order_limit() {
    let order_modify = OrderModify::new(123, Side::Buy, 175, 30);
    let order = order_modify.to_order(OrderType::Limit);
    
    assert_eq!(order.get_order_id(), 123);
    assert_eq!(order.get_side(), Side::Buy);
    assert_eq!(order.get_order_type(), OrderType::Limit);
    assert_eq!(order.get_price(), 175);
    assert_eq!(order.get_initial_quantity(), 30);
    assert_eq!(order.get_remaining_quantity(), 30);
    assert_eq!(order.filled_quantity, 0);
    assert!(!order.is_filled());
}

#[test]
fn test_order_modify_to_order_market() {
    let order_modify = OrderModify::new(456, Side::Sell, 0, 15);
    let order = order_modify.to_order(OrderType::Market);
    
    assert_eq!(order.get_order_id(), 456);
    assert_eq!(order.get_side(), Side::Sell);
    assert_eq!(order.get_order_type(), OrderType::Market);
    assert_eq!(order.get_price(), 0);
    assert_eq!(order.get_initial_quantity(), 15);
}

#[test]
fn test_order_modify_to_order_fill_or_kill() {
    let order_modify = OrderModify::new(789, Side::Buy, 95, 20);
    let order = order_modify.to_order(OrderType::FillOrKill);
    
    assert_eq!(order.get_order_id(), 789);
    assert_eq!(order.get_side(), Side::Buy);
    assert_eq!(order.get_order_type(), OrderType::FillOrKill);
    assert_eq!(order.get_price(), 95);
    assert_eq!(order.get_initial_quantity(), 20);
}

#[test]
fn test_order_modify_to_order_good_till_cancel() {
    let order_modify = OrderModify::new(321, Side::Sell, 110, 8);
    let order = order_modify.to_order(OrderType::GoodTillCancel);
    
    assert_eq!(order.get_order_id(), 321);
    assert_eq!(order.get_side(), Side::Sell);
    assert_eq!(order.get_order_type(), OrderType::GoodTillCancel);
    assert_eq!(order.get_price(), 110);
    assert_eq!(order.get_initial_quantity(), 8);
}

#[test]
fn test_order_modify_clone_and_equality() {
    let order_modify1 = OrderModify::new(555, Side::Buy, 125, 12);
    let order_modify2 = order_modify1.clone();
    
    assert_eq!(order_modify1, order_modify2);
    assert_eq!(order_modify1.get_order_id(), order_modify2.get_order_id());
    assert_eq!(order_modify1.get_side(), order_modify2.get_side());
    assert_eq!(order_modify1.get_price(), order_modify2.get_price());
    assert_eq!(order_modify1.get_quantity(), order_modify2.get_quantity());
}

#[test]
fn test_order_modify_inequality() {
    let order_modify1 = OrderModify::new(1, Side::Buy, 100, 10);
    let order_modify2 = OrderModify::new(2, Side::Buy, 100, 10);
    let order_modify3 = OrderModify::new(1, Side::Sell, 100, 10);
    let order_modify4 = OrderModify::new(1, Side::Buy, 105, 10);
    let order_modify5 = OrderModify::new(1, Side::Buy, 100, 15);
    
    assert_ne!(order_modify1, order_modify2); // Different order ID
    assert_ne!(order_modify1, order_modify3); // Different side
    assert_ne!(order_modify1, order_modify4); // Different price
    assert_ne!(order_modify1, order_modify5); // Different quantity
}

#[test]
fn test_order_modify_debug_format() {
    let order_modify = OrderModify::new(777, Side::Buy, 88, 44);
    let debug_str = format!("{:?}", order_modify);
    
    assert!(debug_str.contains("OrderModify"));
    assert!(debug_str.contains("777"));
    assert!(debug_str.contains("Buy"));
    assert!(debug_str.contains("88"));
    assert!(debug_str.contains("44"));
}

#[test]
fn test_order_modify_with_zero_values() {
    let order_modify = OrderModify::new(0, Side::Buy, 0, 0);
    
    assert_eq!(order_modify.get_order_id(), 0);
    assert_eq!(order_modify.get_side(), Side::Buy);
    assert_eq!(order_modify.get_price(), 0);
    assert_eq!(order_modify.get_quantity(), 0);
    
    let order = order_modify.to_order(OrderType::Market);
    assert_eq!(order.get_order_id(), 0);
    assert_eq!(order.get_price(), 0);
    assert_eq!(order.get_initial_quantity(), 0);
}

#[test]
fn test_order_modify_with_max_values() {
    let order_modify = OrderModify::new(u64::MAX, Side::Sell, u32::MAX, u32::MAX);
    
    assert_eq!(order_modify.get_order_id(), u64::MAX);
    assert_eq!(order_modify.get_side(), Side::Sell);
    assert_eq!(order_modify.get_price(), u32::MAX);
    assert_eq!(order_modify.get_quantity(), u32::MAX);
    
    let order = order_modify.to_order(OrderType::Limit);
    assert_eq!(order.get_order_id(), u64::MAX);
    assert_eq!(order.get_price(), u32::MAX);
    assert_eq!(order.get_initial_quantity(), u32::MAX);
}
