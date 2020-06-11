use chrono::{DateTime, Local};

use crate::Order;

pub struct Position {
    pub id: String,
    pub bought: DateTime<Local>,
    pub order: Order,
}

impl From<Order> for Position {
    fn from(_: Order) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionType {
    LongCall,
    LongPut,
    ShortCall,
    ShortPut,
}
