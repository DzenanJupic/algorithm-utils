use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Local};

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    id: u64,

    time: DateTime<Local>,
    buy_price: Price,

    stock_exchange: StockExchange,
    pieces: u64,

    position_type: PositionType,

    take_profit: TakeProfit,
    stop_loss: StopLoss,
}

impl Position {
    pub fn from_order_data(
        id: u64,
        time: DateTime<Local>,
        buy_price: Price,
        order_data: &OrderData,
    ) -> Self {
        Self {
            id,
            time,
            buy_price,
            stock_exchange: *order_data.stock_exchange(),
            pieces: order_data.pieces(),
            position_type: *order_data.position_type(),
            take_profit: *order_data.take_profit(),
            stop_loss: *order_data.stop_loss(),
        }
    }

    pub fn hash_raw_id(raw_id: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        raw_id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn time(&self) -> &DateTime<Local> { &self.time }
    pub fn buy_price(&self) -> &Price { &self.buy_price }
    pub fn pieces(&self) -> u64 { self.pieces }
    pub fn position_type(&self) -> &PositionType { &self.position_type }
    pub fn take_profit(&self) -> &TakeProfit { &self.take_profit }
    pub fn stop_loss(&self) -> &StopLoss { &self.stop_loss }

    pub fn has_id(&self, id: u64) -> bool {
        self.id == id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionType {
    LongCall,
    LongPut,
    ShortCall,
    ShortPut,
}
