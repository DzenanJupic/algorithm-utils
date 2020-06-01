#![feature(new_uninit)]

pub use algorithms::*;
pub use banks::*;
pub use brokers::*;
pub use derivative::*;
pub use error::*;
pub use export::*;
pub use instruction::*;
pub use order::*;
pub use position::*;
pub use stock_exchange::*;

pub mod algorithms;
pub mod banks;
pub mod brokers;
pub mod derivative;
pub mod error;
pub mod instruction;
pub mod order;
pub mod position;
pub mod stock_exchange;

pub type Price = f64;
pub type Percent = f64;
pub type Points = f64;

pub const UTILS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
