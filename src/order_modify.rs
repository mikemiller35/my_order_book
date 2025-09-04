use crate::{Order, OrderId, OrderType, Price, Quantity, Side};

#[derive(Debug, Clone, PartialEq)]
pub struct OrderModify {
    pub order_id: OrderId,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
}

impl OrderModify {
    pub fn new(order_id: OrderId, side: Side, price: Price, quantity: Quantity) -> Self {
        Self {
            order_id,
            side,
            price,
            quantity,
        }
    }

    pub fn get_order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_price(&self) -> Price {
        self.price
    }

    pub fn get_quantity(&self) -> Quantity {
        self.quantity
    }

    /// Convert OrderModify to a new Order
    pub fn to_order(&self, order_type: OrderType) -> Order {
        Order::new(self.order_id, self.side, order_type, self.price, self.quantity)
    }
}
