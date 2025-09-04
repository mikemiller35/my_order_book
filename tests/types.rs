use my_order_book::{Side, OrderType, LevelInfo};

#[test]
fn test_side_enum_values() {
    let buy = Side::Buy;
    let sell = Side::Sell;
    
    assert_eq!(buy, Side::Buy);
    assert_eq!(sell, Side::Sell);
    assert_ne!(buy, sell);
}

#[test]
fn test_side_clone_and_copy() {
    let buy = Side::Buy;
    let buy_clone = buy.clone();
    let buy_copy = buy;
    
    assert_eq!(buy, buy_clone);
    assert_eq!(buy, buy_copy);
    assert_eq!(buy_clone, buy_copy);
}

#[test]
fn test_side_debug_format() {
    let buy = Side::Buy;
    let sell = Side::Sell;
    
    let buy_debug = format!("{:?}", buy);
    let sell_debug = format!("{:?}", sell);
    
    assert_eq!(buy_debug, "Buy");
    assert_eq!(sell_debug, "Sell");
}

#[test]
fn test_side_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert(Side::Buy, "buy_orders");
    map.insert(Side::Sell, "sell_orders");
    
    assert_eq!(map.get(&Side::Buy), Some(&"buy_orders"));
    assert_eq!(map.get(&Side::Sell), Some(&"sell_orders"));
}

#[test]
fn test_order_type_enum_values() {
    let limit = OrderType::Limit;
    let market = OrderType::Market;
    let fok = OrderType::FillOrKill;
    let fak = OrderType::FillAndKill;
    let gtc = OrderType::GoodTillCancel;
    let gfd = OrderType::GoodForDay;
    let stop = OrderType::Stop;
    let stop_limit = OrderType::StopLimit;
    
    assert_eq!(limit, OrderType::Limit);
    assert_eq!(market, OrderType::Market);
    assert_eq!(fok, OrderType::FillOrKill);
    assert_eq!(fak, OrderType::FillAndKill);
    assert_eq!(gtc, OrderType::GoodTillCancel);
    assert_eq!(gfd, OrderType::GoodForDay);
    assert_eq!(stop, OrderType::Stop);
    assert_eq!(stop_limit, OrderType::StopLimit);
}

#[test]
fn test_order_type_inequality() {
    assert_ne!(OrderType::Limit, OrderType::Market);
    assert_ne!(OrderType::FillOrKill, OrderType::FillAndKill);
    assert_ne!(OrderType::GoodTillCancel, OrderType::GoodForDay);
    assert_ne!(OrderType::Stop, OrderType::StopLimit);
}

#[test]
fn test_order_type_clone_and_copy() {
    let limit = OrderType::Limit;
    let limit_clone = limit.clone();
    let limit_copy = limit;
    
    assert_eq!(limit, limit_clone);
    assert_eq!(limit, limit_copy);
    assert_eq!(limit_clone, limit_copy);
}

#[test]
fn test_order_type_debug_format() {
    let order_types = [
        (OrderType::Limit, "Limit"),
        (OrderType::Market, "Market"),
        (OrderType::FillOrKill, "FillOrKill"),
        (OrderType::FillAndKill, "FillAndKill"),
        (OrderType::GoodTillCancel, "GoodTillCancel"),
        (OrderType::GoodForDay, "GoodForDay"),
        (OrderType::Stop, "Stop"),
        (OrderType::StopLimit, "StopLimit"),
    ];
    
    for (order_type, expected_debug) in order_types.iter() {
        let debug_str = format!("{:?}", order_type);
        assert_eq!(debug_str, *expected_debug);
    }
}

#[test]
fn test_level_info_creation() {
    let level_info = LevelInfo {
        price: 100,
        quantity: 50,
    };
    
    assert_eq!(level_info.price, 100);
    assert_eq!(level_info.quantity, 50);
}

#[test]
fn test_level_info_with_zero_values() {
    let level_info = LevelInfo {
        price: 0,
        quantity: 0,
    };
    
    assert_eq!(level_info.price, 0);
    assert_eq!(level_info.quantity, 0);
}

#[test]
fn test_level_info_with_max_values() {
    let level_info = LevelInfo {
        price: u32::MAX,
        quantity: u32::MAX,
    };
    
    assert_eq!(level_info.price, u32::MAX);
    assert_eq!(level_info.quantity, u32::MAX);
}

#[test]
fn test_level_info_clone_and_equality() {
    let level_info1 = LevelInfo {
        price: 150,
        quantity: 25,
    };
    let level_info2 = level_info1.clone();
    
    assert_eq!(level_info1, level_info2);
    assert_eq!(level_info1.price, level_info2.price);
    assert_eq!(level_info1.quantity, level_info2.quantity);
}

#[test]
fn test_level_info_inequality() {
    let level_info1 = LevelInfo { price: 100, quantity: 10 };
    let level_info2 = LevelInfo { price: 105, quantity: 10 };
    let level_info3 = LevelInfo { price: 100, quantity: 15 };
    
    assert_ne!(level_info1, level_info2); // Different price
    assert_ne!(level_info1, level_info3); // Different quantity
}

#[test]
fn test_level_info_debug_format() {
    let level_info = LevelInfo {
        price: 123,
        quantity: 456,
    };
    let debug_str = format!("{:?}", level_info);
    
    assert!(debug_str.contains("LevelInfo"));
    assert!(debug_str.contains("123"));
    assert!(debug_str.contains("456"));
}

#[test]
fn test_type_aliases() {
    // Test that type aliases work correctly
    let order_id: my_order_book::OrderId = 12345u64;
    let price: my_order_book::Price = 100u32;
    let quantity: my_order_book::Quantity = 50u32;
    
    assert_eq!(order_id, 12345u64);
    assert_eq!(price, 100u32);
    assert_eq!(quantity, 50u32);
}

#[test]
fn test_order_ids_vec() {
    let order_ids: my_order_book::types::OrderIds = vec![1, 2, 3, 4, 5];
    
    assert_eq!(order_ids.len(), 5);
    assert_eq!(order_ids[0], 1);
    assert_eq!(order_ids[4], 5);
}

#[test]
fn test_realistic_level_info_scenario() {
    // Simulate realistic market data
    let bid_level = LevelInfo {
        price: 9950, // $99.50 in cents
        quantity: 1000,
    };
    
    let ask_level = LevelInfo {
        price: 10000, // $100.00 in cents
        quantity: 500,
    };
    
    assert_eq!(bid_level.price, 9950);
    assert_eq!(bid_level.quantity, 1000);
    assert_eq!(ask_level.price, 10000);
    assert_eq!(ask_level.quantity, 500);
    
    // Verify spread
    assert_eq!(ask_level.price - bid_level.price, 50); // 50 cent spread
}

#[test]
fn test_side_pattern_matching() {
    fn side_to_string(side: Side) -> &'static str {
        match side {
            Side::Buy => "buy",
            Side::Sell => "sell",
        }
    }
    
    assert_eq!(side_to_string(Side::Buy), "buy");
    assert_eq!(side_to_string(Side::Sell), "sell");
}

#[test]
fn test_order_type_pattern_matching() {
    fn is_immediate_order(order_type: OrderType) -> bool {
        match order_type {
            OrderType::Market | OrderType::FillOrKill | OrderType::FillAndKill => true,
            _ => false,
        }
    }
    
    assert!(is_immediate_order(OrderType::Market));
    assert!(is_immediate_order(OrderType::FillOrKill));
    assert!(is_immediate_order(OrderType::FillAndKill));
    assert!(!is_immediate_order(OrderType::Limit));
    assert!(!is_immediate_order(OrderType::GoodTillCancel));
}
