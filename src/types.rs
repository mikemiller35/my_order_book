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
