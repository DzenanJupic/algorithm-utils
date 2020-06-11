use std::fmt;

use crate::GeneralErrorKind;

#[derive(Debug, Copy, Clone)]
pub enum BrokerErrorKind {
    ConnectionFailed,
    CouldNotLogin,
    CouldNotLogout,
    NoSuchOrder,
    NoSuchPosition,
    Other,
    TimeOut,
}

impl fmt::Display for BrokerErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl std::error::Error for BrokerErrorKind {}

impl GeneralErrorKind for BrokerErrorKind {}
