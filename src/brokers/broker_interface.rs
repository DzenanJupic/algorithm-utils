use crate::{Order, Position};

pub trait BrokerInterface {
    type ACCOUNT;

    fn capabilities(&self);

    fn login(&mut self);
    fn logout(&mut self);

    fn all_accounts(&self);
    fn account_balance(&self, account: Self::ACCOUNT);
    fn account_transactions(&self);

    fn all_deposits(&self);
    fn deposit_balance(&self);
    fn deposit_transactions(&self);

    fn all_orders(&self);
    fn get_order(&self);
    fn change_order(&self);
    fn delete_order(&self);

    fn all_positions(&self);
    fn get_positions(&self);

    fn buy(&self, order: Order);
    fn sell(&self, position: Position);
}