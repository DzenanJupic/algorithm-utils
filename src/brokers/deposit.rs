use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{Currency, MarketValue, Order, OrderData, Position, Price, Transaction};

pub struct Deposit {
    id: u64,
    raw_id: String,

    currency: Currency,
    transactions: Vec<Transaction>,
    balance: Price,

    orders: Vec<Order>,
    positions: Vec<Position>,
}

impl Deposit {
    pub fn empty(raw_id: String, currency: Currency) -> Self {
        let mut hasher = DefaultHasher::new();
        raw_id.hash(&mut hasher);
        let id = hasher.finish();

        Self {
            id,
            raw_id,
            currency,
            transactions: Vec::new(),
            balance: Price::zero(),
            orders: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub const fn id(&self) -> u64 { self.id }
    pub const fn currency(&self) -> Currency { self.currency }
    pub fn transactions(&self) -> &Vec<Transaction> { &self.transactions }
    pub fn orders(&self) -> &Vec<Order> { &self.orders }
    pub fn positions(&self) -> &Vec<Position> { &self.positions }

    pub fn order_id_exists(&self, id: u64) -> bool {
        self.orders
            .iter()
            .find(|order| order.has_id(id))
            .is_some()
    }

    pub fn add_order(&mut self, order: Order) { self.orders.push(order); }

    pub fn change_order(&mut self, order_id: u64) -> Option<&mut OrderData> {
        for order in self.orders.iter_mut() {
            let order_option = order.find_order_mut(order_id);
            if order_option.is_some() {
                return order_option;
            }
        }
        None
    }
}