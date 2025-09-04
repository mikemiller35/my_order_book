use std::collections::{BTreeMap, HashMap};
use crate::{
    order::{Order, Orders},
    order_modify::OrderModify,
    trade::{Trade, Trades,  TradeInfo},
    types::{LevelInfo, OrderId, OrderIds, OrderType, Price, Quantity, Side, OrderStatus, OrderResult},
};

type MatchedTrade = (OrderId, Price, OrderId, Price, Quantity, bool, bool);
type MatchedTrades = Vec<MatchedTrade>;

#[derive(Debug, Clone, PartialEq)]
pub struct OrderbookLevelInfos {
    pub bids: Vec<LevelInfo>,
    pub asks: Vec<LevelInfo>,
}

impl OrderbookLevelInfos {
    pub fn new(bids: Vec<LevelInfo>, asks: Vec<LevelInfo>) -> Self {
        Self { bids, asks }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct LevelData {
    quantity: Quantity,
    count: Quantity,
}

impl LevelData {
    fn new() -> Self {
        Self {
            quantity: 0,
            count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LevelAction {
    Add,
    Remove,
    Match,
}

#[derive(Debug, Default)]
pub struct OrderBook {
    pub bids: BTreeMap<Price, Orders>,
    pub asks: BTreeMap<Price, Orders>,
    pub orders: HashMap<OrderId, Order>,
    data: HashMap<Price, LevelData>,
    next_order_id: OrderId,
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    fn update_level_data(&mut self, price: Price, quantity: Quantity, action: LevelAction) {
        let data: &mut LevelData = self.data.entry(price).or_insert_with(LevelData::new);

        match action {
            LevelAction::Add => {
                data.count += 1;
                data.quantity += quantity;
            }
            LevelAction::Remove => {
                data.count = data.count.saturating_sub(1);
                data.quantity = data.quantity.saturating_sub(quantity);
            }
            LevelAction::Match => {
                data.quantity = data.quantity.saturating_sub(quantity);
            }
        }

        if data.count == 0 {
            self.data.remove(&price);
        }
    }

    fn on_order_added(&mut self, order: &Order) {
        self.update_level_data(order.get_price(), order.get_initial_quantity(), LevelAction::Add);
    }

    fn on_order_cancelled(&mut self, order: &Order) {
        self.update_level_data(order.get_price(), order.get_remaining_quantity(), LevelAction::Remove);
    }

    fn on_order_matched(&mut self, price: Price, quantity: Quantity, is_fully_filled: bool) {
        let action: LevelAction = if is_fully_filled {
            LevelAction::Remove
        } else {
            LevelAction::Match
        };
        self.update_level_data(price, quantity, action);
    }

    fn can_match(&self, side: Side, price: Price) -> bool {
        match side {
            Side::Buy => {
                if self.asks.is_empty() {
                    return false;
                }
                let best_ask: Price = *self.asks.keys().next().unwrap();
                price >= best_ask
            }
            Side::Sell => {
                if self.bids.is_empty() {
                    return false;
                }
                let best_bid: Price = *self.bids.keys().next_back().unwrap();
                price <= best_bid
            }
        }
    }

    fn can_fully_fill(&self, side: Side, price: Price, mut quantity: Quantity) -> bool {
        if !self.can_match(side, price) {
            return false;
        }

        let threshold: Option<Price> = match side {
            Side::Buy => {
                if let Some(ask_price) = self.asks.keys().next() {
                    Some(*ask_price)
                } else {
                    return false;
                }
            }
            Side::Sell => {
                if let Some(bid_price) = self.bids.keys().next_back() {
                    Some(*bid_price)
                } else {
                    return false;
                }
            }
        };

        for (level_price, level_data) in &self.data {
            if let Some(threshold_price) = threshold {
                let skip_level: bool = match side {
                    Side::Buy => threshold_price > *level_price,
                    Side::Sell => threshold_price < *level_price,
                };
                if skip_level {
                    continue;
                }
            }

            let price_check: bool = match side {
                Side::Buy => *level_price > price,
                Side::Sell => *level_price < price,
            };
            if price_check {
                continue;
            }

            if quantity <= level_data.quantity {
                return true;
            }

            quantity = quantity.saturating_sub(level_data.quantity);
        }

        false
    }

    fn match_orders(&mut self) -> Trades {
        let mut trades: Trades = Vec::new();

        loop {
            if self.bids.is_empty() || self.asks.is_empty() {
                break;
            }

            let (bid_price, ask_price) = {
                let bid_price = *self.bids.keys().next_back().unwrap();
                let ask_price = *self.asks.keys().next().unwrap();
                (bid_price, ask_price)
            };

            if bid_price < ask_price {
                break;
            }

            let mut matched_trades: MatchedTrades = Vec::new();
            let mut bid_orders_to_remove: OrderIds = Vec::new();
            let mut ask_orders_to_remove: OrderIds = Vec::new();
            let mut bid_orders_to_update: Orders = Vec::new();
            let mut ask_orders_to_update: Orders = Vec::new();

            // Process matching within this price level
            {
                let bid_orders: &mut Orders = self.bids.get_mut(&bid_price).unwrap();
                let ask_orders: &mut Orders = self.asks.get_mut(&ask_price).unwrap();

                while !bid_orders.is_empty() && !ask_orders.is_empty() {
                    let mut bid: Order = bid_orders[0].clone();
                    let mut ask: Order = ask_orders[0].clone();

                    let quantity: Quantity = std::cmp::min(bid.get_remaining_quantity(), ask.get_remaining_quantity());

                    bid.fill(quantity);
                    ask.fill(quantity);

                    let bid_filled: bool = bid.is_filled();
                    let ask_filled: bool = ask.is_filled();

                    matched_trades.push((bid.get_order_id(), bid.get_price(), ask.get_order_id(), ask.get_price(), quantity, bid_filled, ask_filled));

                    if bid_filled {
                        bid_orders_to_remove.push(bid.get_order_id());
                        bid_orders.remove(0);
                    } else {
                        bid_orders[0] = bid.clone();
                        bid_orders_to_update.push(bid);
                    }

                    if ask_filled {
                        ask_orders_to_remove.push(ask.get_order_id());
                        ask_orders.remove(0);
                    } else {
                        ask_orders[0] = ask.clone();
                        ask_orders_to_update.push(ask);
                    }
                }
            }

            for order_id in bid_orders_to_remove {
                self.orders.remove(&order_id);
            }
            for order_id in ask_orders_to_remove {
                self.orders.remove(&order_id);
            }
            for order in bid_orders_to_update {
                self.orders.insert(order.get_order_id(), order);
            }
            for order in ask_orders_to_update {
                self.orders.insert(order.get_order_id(), order);
            }

            for (bid_id, bid_price, ask_id, ask_price, quantity, bid_filled, ask_filled) in matched_trades {
                trades.push(Trade::new(
                    TradeInfo::new(bid_id, bid_price, quantity),
                    TradeInfo::new(ask_id, ask_price, quantity),
                ));

                self.on_order_matched(bid_price, quantity, bid_filled);
                self.on_order_matched(ask_price, quantity, ask_filled);
            }

            if self.bids.get(&bid_price).map_or(true, |orders: &Orders| orders.is_empty()) {
                self.bids.remove(&bid_price);
                self.data.remove(&bid_price);
            }

            if self.asks.get(&ask_price).map_or(true, |orders:&Orders| orders.is_empty()) {
                self.asks.remove(&ask_price);
                self.data.remove(&ask_price);
            }
        }

        let mut orders_to_cancel: OrderIds = Vec::new();

        if !self.bids.is_empty() {
            let best_bid_price: Price = *self.bids.keys().next_back().unwrap();
            if let Some(bid_orders) = self.bids.get(&best_bid_price) {
                if let Some(order) = bid_orders.first() {
                    if order.get_order_type() == OrderType::FillAndKill {
                        orders_to_cancel.push(order.get_order_id());
                    }
                }
            }
        }

        if !self.asks.is_empty() {
            let best_ask_price: Price = *self.asks.keys().next().unwrap();
            if let Some(ask_orders) = self.asks.get(&best_ask_price) {
                if let Some(order) = ask_orders.first() {
                    if order.get_order_type() == OrderType::FillAndKill {
                        orders_to_cancel.push(order.get_order_id());
                    }
                }
            }
        }

        for order_id in orders_to_cancel {
            self.cancel_order_internal(order_id);
        }

        trades
    }

    pub fn add_order(&mut self, side: Side, order_type: OrderType, price: Price, quantity: Quantity) -> (OrderId, Trades) {
        let order_id: OrderId = self.next_order_id;
        self.next_order_id += 1;

        let mut order: Order = Order::new(order_id, side, order_type, price, quantity);

        if self.orders.contains_key(&order_id) {
            return (order_id, Vec::new());
        }

        if order_type == OrderType::Market {
            match side {
                Side::Buy => {
                    if !self.asks.is_empty() {
                        let worst_ask: Price = *self.asks.keys().next_back().unwrap();
                        order.to_good_till_cancel(worst_ask);
                    } else {
                        return (order_id, Vec::new());
                    }
                }
                Side::Sell => {
                    if !self.bids.is_empty() {
                        let worst_bid: Price = *self.bids.keys().next().unwrap();
                        order.to_good_till_cancel(worst_bid);
                    } else {
                        return (order_id, Vec::new());
                    }
                }
            }
        }

        if order_type == OrderType::FillAndKill && !self.can_match(side, order.get_price()) {
            return (order_id, Vec::new());
        }

        if order_type == OrderType::FillOrKill && !self.can_fully_fill(side, order.get_price(), order.get_initial_quantity()) {
            return (order_id, Vec::new());
        }

        match side {
            Side::Buy => {
                self.bids.entry(order.get_price()).or_insert_with(Vec::new).push(order.clone());
            }
            Side::Sell => {
                self.asks.entry(order.get_price()).or_insert_with(Vec::new).push(order.clone());
            }
        }

        self.orders.insert(order_id, order.clone());
        self.on_order_added(&order);

        let trades: Trades = self.match_orders();
        (order_id, trades)
    }

    pub fn add_order_with_status(&mut self, side: Side, order_type: OrderType, price: Price, quantity: Quantity) -> OrderResult {
        let order_id = self.next_order_id;
        self.next_order_id += 1;

        let mut order: Order = Order::new(order_id, side, order_type, price, quantity);

        if self.orders.contains_key(&order_id) {
            return OrderResult::new(order_id, OrderStatus::RejectedDuplicateId, Vec::new());
        }

        if order_type == OrderType::Market {
            match side {
                Side::Buy => {
                    if !self.asks.is_empty() {
                        let worst_ask: Price = *self.asks.keys().next_back().unwrap();
                        order.to_good_till_cancel(worst_ask);
                    } else {
                        return OrderResult::new(order_id, OrderStatus::RejectedNoLiquidity, Vec::new());
                    }
                }
                Side::Sell => {
                    if !self.bids.is_empty() {
                        let worst_bid: Price = *self.bids.keys().next().unwrap();
                        order.to_good_till_cancel(worst_bid);
                    } else {
                        return OrderResult::new(order_id, OrderStatus::RejectedNoLiquidity, Vec::new());
                    }
                }
            }
        }

        if order_type == OrderType::FillAndKill && !self.can_match(side, order.get_price()) {
            return OrderResult::new(order_id, OrderStatus::RejectedFillAndKillNoMatch, Vec::new());
        }

        if order_type == OrderType::FillOrKill && !self.can_fully_fill(side, order.get_price(), order.get_initial_quantity()) {
            return OrderResult::new(order_id, OrderStatus::RejectedFillOrKillPartialFill, Vec::new());
        }

        match side {
            Side::Buy => {
                self.bids.entry(order.get_price()).or_insert_with(Vec::new).push(order.clone());
            }
            Side::Sell => {
                self.asks.entry(order.get_price()).or_insert_with(Vec::new).push(order.clone());
            }
        }

        self.orders.insert(order_id, order.clone());
        self.on_order_added(&order);

        let trades: Trades = self.match_orders();

        let status: OrderStatus = if trades.is_empty() {
            OrderStatus::Accepted
        } else {
            OrderStatus::Executed
        };

        OrderResult::new(order_id, status, trades)
    }

    pub fn cancel_orders(&mut self, order_ids: OrderIds) {
        for order_id in order_ids {
            self.cancel_order_internal(order_id);
        }
    }

    pub fn cancel_order_internal(&mut self, order_id: OrderId) {
        if !self.orders.contains_key(&order_id) {
            return;
        }

        let order: Order = self.orders.remove(&order_id).unwrap();
        match order.side {
            Side::Buy => {
                let price: Price = order.price;
                let orders: &mut Orders = self.bids.get_mut(&price).unwrap();
                orders.retain(|o: &Order| o.id != order_id);
                if orders.is_empty() {
                    self.bids.remove(&price);
                }
            }
            Side::Sell => {
                let price: Price = order.price;
                let orders: &mut Orders = self.asks.get_mut(&price).unwrap();
                orders.retain(|o: &Order| o.id != order_id);
                if orders.is_empty() {
                    self.asks.remove(&price);
                }
            }
        }

        self.on_order_cancelled(&order);
    }

    pub fn cancel_order(&mut self, order_id: OrderId) {
        self.cancel_order_internal(order_id);
    }

    pub fn modify_order(&mut self, order_modify: OrderModify) -> Trades {
        let order_type: OrderType = {
            if let Some(existing_order) = self.orders.get(&order_modify.get_order_id()) {
                existing_order.get_order_type()
            } else {
                return Vec::new();
            }
        };

        self.cancel_order(order_modify.get_order_id());
        let (_, trades) = self.add_order(
            order_modify.get_side(),
            order_type,
            order_modify.get_price(),
            order_modify.get_quantity(),
        );
        trades
    }

    pub fn size(&self) -> usize {
        self.orders.len()
    }

    pub fn get_order_infos(&self) -> OrderbookLevelInfos {
        let mut bid_infos = Vec::new();
        let mut ask_infos = Vec::new();

        for (price, orders) in self.bids.iter().rev() {
            let total_quantity: Quantity = orders.iter()
                .map(|order: &Order| order.get_remaining_quantity())
                .sum();
            bid_infos.push(LevelInfo {
                price: *price,
                quantity: total_quantity,
            });
        }

        for (price, orders) in &self.asks {
            let total_quantity: u32 = orders.iter()
                .map(|order: &Order| order.get_remaining_quantity())
                .sum();
            ask_infos.push(LevelInfo {
                price: *price,
                quantity: total_quantity,
            });
        }

        OrderbookLevelInfos::new(bid_infos, ask_infos)
    }

    pub fn get_best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    pub fn get_best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    /// Prune Good For Day orders
    pub fn prune_good_for_day_orders(&mut self) {
        let mut orders_to_cancel = Vec::new();

        for (order_id, order) in &self.orders {
            if order.get_order_type() == OrderType::GoodForDay {
                // In a for real implementation, check if it's past market close time
                // For now, we just collect them for potential cancellation
                orders_to_cancel.push(*order_id);
            }
        }
        // Only cancel if past market close
        // self.cancel_orders(&orders_to_cancel);
    }
}