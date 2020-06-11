use crate::{BrokerCapability, BrokerErrorKind, Deposit, Error, Order, Position, StockExchange};

pub trait BrokerInterface {
    const NAME: &'static str;
    const CAPABILITIES: &'static [BrokerCapability];
    const STOCK_EXCHANGES: &'static [StockExchange];

    fn login(&mut self) -> Result<(), Error<BrokerErrorKind>>;
    fn logout(&mut self) -> Result<(), Error<BrokerErrorKind>>;
    fn is_logged_in(&self) -> bool;

    fn all_deposits(&self) -> Result<Vec<Deposit>, Error<BrokerErrorKind>>;
    fn update_deposit_transactions(&self, deposit: &mut Deposit) -> Result<(), Error<BrokerErrorKind>>;
    fn update_deposit_balance(&self, deposit: &mut Deposit) -> Result<(), Error<BrokerErrorKind>>;

    fn all_orders(&self) -> Result<&[Order], Error<BrokerErrorKind>>;
    fn get_order(&self) -> Result<&Order, Error<BrokerErrorKind>>;
    fn change_order(&self) -> Result<(), Error<BrokerErrorKind>>;
    fn delete_order(&self) -> Result<Order, Error<BrokerErrorKind>>;
    fn update_deposit_orders(&self, deposit: &mut Deposit) -> Result<(), Error<BrokerErrorKind>>;

    fn all_positions(&self) -> Result<&[Position], Error<BrokerErrorKind>>;
    fn get_positions(&self) -> Result<&Position, Error<BrokerErrorKind>>;
    fn update_deposit_positions(&self, deposit: &mut Deposit) -> Result<(), Error<BrokerErrorKind>>;

    fn buy(&self, deposit: &mut Deposit, order: u64) -> Result<&Position, Error<BrokerErrorKind>>;
    fn sell(&self, deposit: &mut Deposit, position: u64) -> Result<Position, Error<BrokerErrorKind>>;
}