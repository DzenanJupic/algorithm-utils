use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Duration, Local};

use crate::{PositionType, Price, RelativePrice, StockExchange};

/// for a documentation of the order types:  https://www.investopedia.com/investing/basics-trading-stock-know-your-orders/
#[derive(Clone, Debug, PartialEq)]
pub enum Order {
    Normal(OrderData),
    OneCancelsTheOther(Vec<OrderData>),
    AllOrNone(Vec<OrderData>),
    ImmediateOrCancel(OrderData),
    FillOrKill(Vec<OrderData>),
}

impl Order {
    pub fn has_id(&self, id: u64) -> bool {
        use Order::*;
        match self {
            Normal(order_data) => order_data.id == id,
            OneCancelsTheOther(data) => data
                .iter()
                .find(|order_data| order_data.id == id)
                .is_some(),
            AllOrNone(data) => data
                .iter()
                .find(|order_data| order_data.id == id)
                .is_some(),
            ImmediateOrCancel(order_data) => order_data.id == id,
            FillOrKill(data) => data
                .iter()
                .find(|order_data| order_data.id == id)
                .is_some(),
        }
    }

    pub fn get(&self, id: u64) -> Option<&OrderData> {
        use Order::*;
        match self {
            Normal(order_data) => {
                if order_data.id == id {
                    Some(order_data)
                } else {
                    None
                }
            }
            OneCancelsTheOther(data) => data
                .iter()
                .find(|order_data| order_data.id == id),
            AllOrNone(data) => data
                .iter()
                .find(|order_data| order_data.id == id),
            ImmediateOrCancel(order_data) => {
                if order_data.id == id {
                    Some(order_data)
                } else {
                    None
                }
            }
            FillOrKill(data) => data
                .iter()
                .find(|order_data| order_data.id == id),
        }
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut OrderData> {
        use Order::*;
        match self {
            Normal(order_data) => {
                if order_data.id == id {
                    Some(order_data)
                } else {
                    None
                }
            }
            OneCancelsTheOther(data) => data
                .iter_mut()
                .find(|order_data| order_data.id == id),
            AllOrNone(data) => data
                .iter_mut()
                .find(|order_data| order_data.id == id),
            ImmediateOrCancel(order_data) => {
                if order_data.id == id {
                    Some(order_data)
                } else {
                    None
                }
            }
            FillOrKill(data) => data
                .iter_mut()
                .find(|order_data| order_data.id == id),
        }
    }
}

/// The OrderData of an Order
///
/// #### Fields:
/// * __id__: A unique id that makes it easy to identify an order. Note: Since many brokers provide
/// strings instead of u64 the id is always the hash of the provided raw_id.
/// * __raw_id__: A unique id that makes it easy to identify an order. This id is usually provided
/// by the broker
/// * __stock_exchange__: The stock exchange on which the order will be executed.
/// todo
#[derive(Clone, Debug, PartialEq)]
pub struct OrderData {
    id: u64,

    stock_exchange: StockExchange,
    pieces: u64,

    order_type: OrderType,
    position_type: PositionType,

    take_profit: TakeProfit,
    stop_loss: StopLoss,

    moment: OrderMoment,
    validity: OrderValidity,
}

impl OrderData {
    pub fn id(&self) -> u64 { self.id }
    pub fn stock_exchange(&self) -> &StockExchange { &self.stock_exchange }
    pub fn pieces(&self) -> u64 { self.pieces }
    pub fn order_type(&self) -> &OrderType { &self.order_type }
    pub fn position_type(&self) -> &PositionType { &self.position_type }
    pub fn take_profit(&self) -> &TakeProfit { &self.take_profit }
    pub fn stop_loss(&self) -> &StopLoss { &self.stop_loss }
    pub fn moment(&self) -> &OrderMoment { &self.moment }
    pub fn validity(&self) -> &OrderValidity { &self.validity }

    pub fn update_take_profit(&mut self, take_profit: TakeProfit) { self.take_profit = take_profit }
    pub fn update_stop_loss(&mut self, stop_loss: StopLoss) { self.stop_loss = stop_loss }
    pub fn update_moment(&mut self, order_moment: OrderMoment) { self.moment = order_moment }
    pub fn update_validity(&mut self, order_validity: OrderValidity) { self.validity = order_validity }

    pub fn hash_raw_id(raw_id: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        raw_id.hash(&mut hasher);
        hasher.finish()
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrderType {
    MarketOrder,
    LimitOrder(Price),
    StopOrder(Price),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TakeProfit {
    Absolute(Price),
    Relative(RelativePrice),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopLoss {
    Absolute(Price),
    Relative(RelativePrice),
    Trailing(RelativePrice),
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderMoment {
    Instant,
    Planed(DateTime<Local>),
}

impl OrderMoment {
    pub fn as_duration(&self) -> Duration {
        use OrderMoment::*;
        match self {
            Instant => Duration::zero(),
            Planed(date_time) => *date_time - Local::now()
        }
    }

    pub fn is_now_or_passed(&self) -> bool {
        self.as_duration() <= Duration::zero()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderValidity {
    Now,
    OneDay,
    OneWeek,
    OneMonth,
    OneYear,
    Forever,
}

impl OrderValidity {
    pub fn as_duration(&self) -> Duration {
        use OrderValidity::*;
        match self {
            Now => Duration::zero(),
            OneDay => Duration::days(1),
            OneWeek => Duration::weeks(1),
            OneMonth => Duration::days(30),
            OneYear => Duration::days(365),
            Forever => Duration::max_value()
        }
    }

    pub fn is_valid(&self, start: &DateTime<Local>) -> bool {
        let duration = self.as_duration();
        *start + duration < Local::now()
    }
}
