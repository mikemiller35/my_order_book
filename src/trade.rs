use crate::{OrderId, Price, Quantity};

#[derive(Debug, Clone, PartialEq)]
pub struct TradeInfo {
    pub order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
}

impl TradeInfo {
    pub fn new(order_id: OrderId, price: Price, quantity: Quantity) -> Self {
        Self {
            order_id,
            price,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trade {
    pub bid_info: TradeInfo,
    pub ask_info: TradeInfo,
}

impl Trade {
    pub fn new(bid_info: TradeInfo, ask_info: TradeInfo) -> Self {
        Self {
            bid_info,
            ask_info,
        }
    }
}

