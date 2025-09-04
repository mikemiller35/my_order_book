//! Core types used throughout the order book system.

/// Unique identifier for orders
pub type OrderId = u64;

/// Multiple order IDs
pub type OrderIds = Vec<OrderId>;

/// Price representation
pub type Price = u32;

/// Quantity of shares/units
pub type Quantity = u32;

/// Side of the market (buy or sell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

/// Type of order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    /// Fill and Kill - execute immediately, cancel remainder
    FillAndKill,
    /// Fill or Kill - execute completely or cancel entirely
    FillOrKill,
    /// Good Till Cancel - remains active until cancelled
    GoodTillCancel,
    /// Good For Day - remains active until end of trading day
    GoodForDay,
    /// Limit order - execute at specified price or better
    Limit,
    /// Market order - execute immediately at best available price
    Market,
    /// Stop order - becomes market order when stop price is reached
    Stop,
    /// Stop limit order - becomes limit order when stop price is reached
    StopLimit,
}

/// Information about a price level in the order book
#[derive(Debug, Clone, PartialEq)]
pub struct LevelInfo {
    pub price: Price,
    pub quantity: Quantity,
}

/// Status of an order after being submitted to the order book
#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    /// Order was accepted and placed in the book
    Accepted,
    /// Order was accepted and immediately executed (fully or partially)
    Executed,
    /// Order was rejected - market order with no liquidity on the other side
    RejectedNoLiquidity,
    /// Order was rejected - Fill-and-Kill order couldn't match
    RejectedFillAndKillNoMatch,
    /// Order was rejected - Fill-or-Kill order couldn't be fully filled
    RejectedFillOrKillPartialFill,
    /// Order was rejected - duplicate order ID
    RejectedDuplicateId,
}

impl OrderStatus {
    pub fn message(&self) -> &'static str {
        match self {
            OrderStatus::Accepted => "Order accepted and placed in order book",
            OrderStatus::Executed => "Order executed successfully",
            OrderStatus::RejectedNoLiquidity => "Market order rejected: no liquidity available on opposite side",
            OrderStatus::RejectedFillAndKillNoMatch => "Fill-and-Kill order rejected: no matching orders available",
            OrderStatus::RejectedFillOrKillPartialFill => "Fill-or-Kill order rejected: insufficient liquidity for complete fill",
            OrderStatus::RejectedDuplicateId => "Order rejected: duplicate order ID",
        }
    }
}

/// Result of submitting an order to the order book
#[derive(Debug, Clone, PartialEq)]
pub struct OrderResult {
    pub order_id: OrderId,
    pub status: OrderStatus,
    pub trades: crate::trade::Trades,
}

impl OrderResult {
    pub fn new(order_id: OrderId, status: OrderStatus, trades: crate::trade::Trades) -> Self {
        Self {
            order_id,
            status,
            trades,
        }
    }
}
