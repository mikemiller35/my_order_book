use my_order_book::{OrderBook, Side, OrderType};

fn main() {
    let mut order_book = OrderBook::new();

    // Do some stuff - create overlapping prices so they can trade
    let (buy_order_id, trades1) = order_book.add_order(Side::Buy, OrderType::Limit, 105, 10);
    let (sell_order_id, trades2) = order_book.add_order(Side::Sell, OrderType::Limit, 100, 5);

    println!("Created buy order {} and sell order {}", buy_order_id, sell_order_id);
    println!("Trades from first order: {:?}", trades1);
    println!("Trades from second order: {:?}", trades2);
}
