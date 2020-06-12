use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Local};

use crate::*;

pub struct Deposit {
    id: u64,

    currency: Currency,
    cash: Price,

    orders: Vec<Order>,
    positions: Vec<Position>,
}

impl Deposit {
    pub fn empty(id: u64, currency: Currency) -> Self {
        Self {
            id,
            currency,
            cash: Price::zero(),
            orders: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub fn hash_raw_id(raw_id: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        raw_id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn currency(&self) -> &Currency { &self.currency }
    pub fn cash(&self) -> &Price { &self.cash }
    pub fn orders(&self) -> &Vec<Order> { &self.orders }
    pub fn positions(&self) -> &Vec<Position> { &self.positions }

    pub fn order_exists(&self, id: u64) -> bool {
        self.get_order(id).is_some()
    }

    fn get_order_index(&self, id: u64) -> Option<usize> {
        self.orders
            .iter()
            .position(
                |order| order.has_id(id)
            )
    }

    pub fn get_order(&self, id: u64) -> Option<&Order> {
        self.orders
            .iter()
            .find(
                |order| order.has_id(id)
            )
    }

    pub fn place_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn edit_order<F: Fn(&mut Order)>(&mut self, id: u64, change_fn: F) -> Result<(), BrokerErrorKind> {
        let order = self.orders
                        .iter_mut()
                        .find(
                            |order| order.has_id(id)
                        )
                        .ok_or(BrokerErrorKind::NoSuchOrder)?;
        Ok(change_fn(order))
    }

    pub fn execute_order(
        &mut self,
        order_id: u64,
        position_id: u64,
        buy_price: Price,
        time: DateTime<Local>,
    ) -> Result<Order, BrokerErrorKind> {
        match self.get_order_index(order_id) {
            Some(index) => {
                let order = self.orders.swap_remove(index);
                let order_data = order.get(order_id).unwrap();

                self.positions.push(Position::from_order_data(
                    position_id,
                    time,
                    buy_price,
                    order_data,
                ));
                self.cash -= buy_price;

                Ok(order)
            }
            None => Err(BrokerErrorKind::NoSuchOrder)
        }
    }

    pub fn delete_order(&mut self, id: u64) -> Result<Order, BrokerErrorKind> {
        match self.get_order_index(id) {
            Some(index) => Ok(self.orders.swap_remove(index)),
            None => Err(BrokerErrorKind::NoSuchOrder)
        }
    }

    pub fn position_exists(&self, id: u64) -> bool {
        self.get_position(id).is_some()
    }

    fn get_position_index(&self, id: u64) -> Option<usize> {
        self.positions
            .iter()
            .position(
                |position| position.has_id(id)
            )
    }

    pub fn get_position(&self, id: u64) -> Option<&Position> {
        self.positions
            .iter()
            .find(
                |position| position.has_id(id)
            )
    }

    pub fn close_position(&mut self, id: u64, sell_price: Price) -> Result<Position, BrokerErrorKind> {
        match self.get_position_index(id) {
            Some(index) => {
                self.cash += sell_price;
                Ok(self.positions.swap_remove(index))
            }
            None => Err(BrokerErrorKind::NoSuchPosition)
        }
    }
}
