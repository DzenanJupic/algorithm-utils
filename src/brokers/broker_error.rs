use std::fmt;
use std::fmt::Formatter;

use crate::TradingError;

#[derive(Debug, Clone)]
pub struct BrokerError {
    msg: String,
    kind: BrokerErrorKind,
}

impl std::error::Error for BrokerError {}

impl TradingError for BrokerError {}

impl fmt::Display for BrokerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "BrokerError ({:?}):\n\
            \t{}",
            self.kind,
            self.msg
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BrokerErrorKind {
    ConnectionFailed,
    CouldNotLogin,
    CouldNotLogout,
    NoSuchPosition,
    Other,
    TimeOut,
}