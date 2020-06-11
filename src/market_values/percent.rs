use std::fmt::Debug;

use crate::MarketValue;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Percent(pub(crate) f64);

impl Percent {
    pub fn from_decimal(decimal: f64) -> Self {
        Self(decimal / 100.0)
    }

    pub fn growth<P: MarketValue>(old: P, new: P) -> Self {
        Self((*new / *old) - 1.0)
    }
}

impl MarketValue for Percent {
    fn new(value: f64) -> Self {
        Self(value)
    }

    fn one() -> Self {
        Self(0.01)
    }

    fn minus_one() -> Self {
        Self(-0.01)
    }

    fn one_hundred() -> Self {
        Self(1.0)
    }

    fn minus_one_hundred() -> Self {
        Self(1.0)
    }
}

impl_ops!(Percent, no_percent);
