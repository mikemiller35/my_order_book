use my_order_book::{OrderBook, Side, OrderType};

fn main() {
    let mut order_book = OrderBook::new();

    // Create non-overlapping orders first (no immediate trades)
    let result1 = order_book.add_order_with_status(Side::Buy, OrderType::Limit, 100, 10);
    let result2 = order_book.add_order_with_status(Side::Sell, OrderType::Limit, 105, 8);

    println!("=== Initial Orders ===");
    println!("Buy order {}: {} - {}", result1.order_id, result1.status.message(),
             if result1.trades.is_empty() { "No trades" } else { &format!("{} trades", result1.trades.len()) });
    println!("Sell order {}: {} - {}", result2.order_id, result2.status.message(),
             if result2.trades.is_empty() { "No trades" } else { &format!("{} trades", result2.trades.len()) });

    // Now add a crossing order that will create trades
    println!("\n=== Adding Crossing Order ===");
    let result3 = order_book.add_order_with_status(Side::Buy, OrderType::Limit, 106, 5);

    println!("Crossing buy order {}: {} - {}", result3.order_id, result3.status.message(),
             if result3.trades.is_empty() { "No trades" } else { &format!("{} trades", result3.trades.len()) });

    if !result3.trades.is_empty() {
        println!("Trade details: {:?}", result3.trades);
    }

    // Show the current best bid/ask
    println!("\n=== Order Book State ===");
    println!("Best bid: {:?}", order_book.get_best_bid());
    println!("Best ask: {:?}", order_book.get_best_ask());

    // Demonstrate rejection scenarios
    println!("\n=== Demonstrating Order Rejections ===");

    // Try a market buy with no asks
    let mut empty_book = OrderBook::new();
    let market_result = empty_book.add_order_with_status(Side::Buy, OrderType::Market, 0, 10);
    println!("Market buy with no liquidity: {}", market_result.status.message());

    // Try Fill-or-Kill that can't be fully filled
    let fok_result = order_book.add_order_with_status(Side::Buy, OrderType::FillOrKill, 110, 100);
    println!("Fill-or-Kill for 100 shares: {}", fok_result.status.message());
}
