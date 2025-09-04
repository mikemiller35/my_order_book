use crate::{OrderId, Price, Quantity, Side, OrderType};

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Price,
    pub quantity: Quantity,
    pub filled_quantity: Quantity,
}

impl Order {
    pub fn new(id: OrderId, side: Side, order_type: OrderType, price: Price, quantity: Quantity) -> Self {
        Self {
            id,
            side,
            order_type,
            price,
            quantity,
            filled_quantity: 0,
        }
    }

    pub fn fill(&mut self, quantity: Quantity) {
        self.filled_quantity += quantity;
        if self.filled_quantity > self.quantity {
            self.filled_quantity = self.quantity;
        }
    }

    pub fn is_filled(&self) -> bool {
        self.filled_quantity >= self.quantity
    }

    pub fn get_remaining_quantity(&self) -> Quantity {
        self.quantity.saturating_sub(self.filled_quantity)
    }

    pub fn get_initial_quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn get_order_id(&self) -> OrderId {
        self.id
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_price(&self) -> Price {
        self.price
    }

    pub fn get_order_type(&self) -> OrderType {
        self.order_type
    }

    /// Convert market order to good till cancel with specified price
    pub fn to_good_till_cancel(&mut self, price: Price) {
        self.price = price;
        self.order_type = OrderType::GoodTillCancel;
    }
}

pub type Orders = Vec<Order>;
