use chrono::Duration;

use crate::*;

pub trait AlgorithmInterface {
    /// The `init` function will be called exactly once before trading begins.
    /// This is the only time you'll get to know the derivative that will be traded
    /// and the time steps it will be traded in.
    /// If this information is important to you just save it in your algorithm struct.
    #[allow(unused)]
    fn init(&mut self, deposit: &Deposit, derivative: &Derivative, time_steps: Duration) -> Result<(), TradingError> { Ok(()) }

    /// The `load_data` function will be called while the amount of market_values is less then
    /// Self::min_prices()
    /// You can use it to pre calculate some values (for example moving averages).
    /// It's not possible to give instructions here.
    #[allow(unused)]
    fn collect_prices(&mut self, prices: &[Price]) -> Result<(), TradingError> { Ok(()) }

    /// The `algorithm` function will be called in user defined time steps and is the
    /// hart of your algorithm. Here you can buy or sell derivatives and make money!
    /// Please note that your calculations shouldn't take longer then the time step
    /// defined by the user. If so the algorithm will be shutdown and instructions
    /// have no effect.
    fn algorithm(&mut self, deposit: &Deposit, prices: &[Price], instructions: &mut Vec<Instruction>) -> Result<(), TradingError>;

    /// The `shutdown` function will be called at the end, when the user decides to stop
    /// trading. It's meant to clean things up. Please note that you can't buy anything
    /// here.
    /// If any positions remain open after `shutdown` returned they will be handled
    /// according to the users preferences.
    #[allow(unused)]
    fn shutdown(&mut self, deposit: &Deposit, prices: &[Price], instructions: &mut Vec<Instruction>) -> Result<(), TradingError> { Ok(()) }
}
