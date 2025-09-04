use my_order_book::{Trade, TradeInfo};

#[test]
fn test_trade_info_creation() {
    let trade_info = TradeInfo::new(42, 150, 25);
    
    assert_eq!(trade_info.order_id, 42);
    assert_eq!(trade_info.price, 150);
    assert_eq!(trade_info.quantity, 25);
}

#[test]
fn test_trade_info_with_zero_values() {
    let trade_info = TradeInfo::new(0, 0, 0);
    
    assert_eq!(trade_info.order_id, 0);
    assert_eq!(trade_info.price, 0);
    assert_eq!(trade_info.quantity, 0);
}

#[test]
fn test_trade_info_with_max_values() {
    let trade_info = TradeInfo::new(u64::MAX, u32::MAX, u32::MAX);
    
    assert_eq!(trade_info.order_id, u64::MAX);
    assert_eq!(trade_info.price, u32::MAX);
    assert_eq!(trade_info.quantity, u32::MAX);
}

#[test]
fn test_trade_info_clone_and_equality() {
    let trade_info1 = TradeInfo::new(123, 100, 10);
    let trade_info2 = trade_info1.clone();
    
    assert_eq!(trade_info1, trade_info2);
    assert_eq!(trade_info1.order_id, trade_info2.order_id);
    assert_eq!(trade_info1.price, trade_info2.price);
    assert_eq!(trade_info1.quantity, trade_info2.quantity);
}

#[test]
fn test_trade_info_inequality() {
    let trade_info1 = TradeInfo::new(1, 100, 10);
    let trade_info2 = TradeInfo::new(2, 100, 10);
    let trade_info3 = TradeInfo::new(1, 105, 10);
    let trade_info4 = TradeInfo::new(1, 100, 15);
    
    assert_ne!(trade_info1, trade_info2); // Different order ID
    assert_ne!(trade_info1, trade_info3); // Different price
    assert_ne!(trade_info1, trade_info4); // Different quantity
}

#[test]
fn test_trade_info_debug_format() {
    let trade_info = TradeInfo::new(777, 88, 44);
    let debug_str = format!("{:?}", trade_info);
    
    assert!(debug_str.contains("TradeInfo"));
    assert!(debug_str.contains("777"));
    assert!(debug_str.contains("88"));
    assert!(debug_str.contains("44"));
}

#[test]
fn test_trade_creation() {
    let bid_info = TradeInfo::new(1, 100, 10);
    let ask_info = TradeInfo::new(2, 100, 10);
    let trade = Trade::new(bid_info.clone(), ask_info.clone());
    
    assert_eq!(trade.bid_info, bid_info);
    assert_eq!(trade.ask_info, ask_info);
    assert_eq!(trade.bid_info.order_id, 1);
    assert_eq!(trade.ask_info.order_id, 2);
    assert_eq!(trade.bid_info.price, 100);
    assert_eq!(trade.ask_info.price, 100);
    assert_eq!(trade.bid_info.quantity, 10);
    assert_eq!(trade.ask_info.quantity, 10);
}

#[test]
fn test_trade_with_different_prices() {
    let bid_info = TradeInfo::new(1, 105, 8);
    let ask_info = TradeInfo::new(2, 100, 8);
    let trade = Trade::new(bid_info, ask_info);
    
    assert_eq!(trade.bid_info.price, 105);
    assert_eq!(trade.ask_info.price, 100);
    assert_eq!(trade.bid_info.quantity, 8);
    assert_eq!(trade.ask_info.quantity, 8);
}

#[test]
fn test_trade_with_different_quantities() {
    let bid_info = TradeInfo::new(1, 100, 15);
    let ask_info = TradeInfo::new(2, 100, 5);
    let trade = Trade::new(bid_info, ask_info);
    
    assert_eq!(trade.bid_info.quantity, 15);
    assert_eq!(trade.ask_info.quantity, 5);
}

#[test]
fn test_trade_clone_and_equality() {
    let bid_info = TradeInfo::new(1, 100, 10);
    let ask_info = TradeInfo::new(2, 100, 10);
    let trade1 = Trade::new(bid_info, ask_info);
    let trade2 = trade1.clone();
    
    assert_eq!(trade1, trade2);
    assert_eq!(trade1.bid_info, trade2.bid_info);
    assert_eq!(trade1.ask_info, trade2.ask_info);
}

#[test]
fn test_trade_inequality() {
    let bid_info1 = TradeInfo::new(1, 100, 10);
    let ask_info1 = TradeInfo::new(2, 100, 10);
    let bid_info2 = TradeInfo::new(3, 100, 10);
    let ask_info2 = TradeInfo::new(4, 100, 10);
    
    let trade1 = Trade::new(bid_info1, ask_info1);
    let trade2 = Trade::new(bid_info2, ask_info2);
    
    assert_ne!(trade1, trade2);
}

#[test]
fn test_trade_debug_format() {
    let bid_info = TradeInfo::new(123, 95, 20);
    let ask_info = TradeInfo::new(456, 95, 20);
    let trade = Trade::new(bid_info, ask_info);
    let debug_str = format!("{:?}", trade);
    
    assert!(debug_str.contains("Trade"));
    assert!(debug_str.contains("bid_info"));
    assert!(debug_str.contains("ask_info"));
    assert!(debug_str.contains("123"));
    assert!(debug_str.contains("456"));
    assert!(debug_str.contains("95"));
    assert!(debug_str.contains("20"));
}

#[test]
fn test_trade_with_zero_quantities() {
    let bid_info = TradeInfo::new(1, 100, 0);
    let ask_info = TradeInfo::new(2, 100, 0);
    let trade = Trade::new(bid_info, ask_info);
    
    assert_eq!(trade.bid_info.quantity, 0);
    assert_eq!(trade.ask_info.quantity, 0);
}

#[test]
fn test_trade_with_large_values() {
    let bid_info = TradeInfo::new(u64::MAX - 1, u32::MAX - 1, u32::MAX - 1);
    let ask_info = TradeInfo::new(u64::MAX, u32::MAX, u32::MAX);
    let trade = Trade::new(bid_info, ask_info);
    
    assert_eq!(trade.bid_info.order_id, u64::MAX - 1);
    assert_eq!(trade.ask_info.order_id, u64::MAX);
    assert_eq!(trade.bid_info.price, u32::MAX - 1);
    assert_eq!(trade.ask_info.price, u32::MAX);
    assert_eq!(trade.bid_info.quantity, u32::MAX - 1);
    assert_eq!(trade.ask_info.quantity, u32::MAX);
}

#[test]
fn test_realistic_trade_scenario() {
    // Simulate a realistic trade: buyer pays 101, seller gets 100
    let bid_info = TradeInfo::new(12345, 101, 50);
    let ask_info = TradeInfo::new(67890, 100, 50);
    let trade = Trade::new(bid_info, ask_info);
    
    assert_eq!(trade.bid_info.order_id, 12345);
    assert_eq!(trade.ask_info.order_id, 67890);
    assert_eq!(trade.bid_info.price, 101);
    assert_eq!(trade.ask_info.price, 100);
    assert_eq!(trade.bid_info.quantity, 50);
    assert_eq!(trade.ask_info.quantity, 50);
}
