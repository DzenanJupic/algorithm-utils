use std::fmt::Debug;

use crate::{Currency, MarketValue, Percent, Points, RelativePoints};

pub type PriceWithCurrency = (Price, Currency);
pub type RelativePrice = Price;

/// A decimal Price without a currency
///
/// This price struct is the basic representation of a price that implements the basic features
/// required when working with market_values.
///
/// There's no currency linked to this price since algorithms usually don't care about currency.
/// So be careful not to mix different currency's when working with this price struct.
/// If you really need a currency have a look at the `PriceWithCurrency` type.
///
/// ## Percent calculations
/// When working with market_values and percentages it's important to be aware how different operations
/// are implemented. If you need a custom behaviour you can always dereference both the price and
/// percentage to a float.
///
/// #### Addition
/// When you add a percentage to a price the price will be increased by the persentage of itself.
/// ```
///# use trading_utils::{Price, Percent};
/// let addition = Price(100.0) + Percent(0.05);
/// assert_eq!(addition, Price(105.0));
///```
/// #### Subtraction
/// When you subtract a percentage of a price the price will be decreased by the percentage of itself.
/// ```
/// # use trading_utils::{Price, Percent};
/// let subtraction = Price(100.0) - Percent(0.05);
/// assert_eq!(subtraction, Price(95.0));
/// ```
/// #### Multiplication
/// When you multiplie a price and a percentage the price will be the percentage of itself.
/// ```
/// # use trading_utils::{Price, Percent};
/// let multiplication = Price(100.0) * Percent(0.05);
/// assert_eq!(multiplication, Price(5.0));
/// ```
/// #### Division
/// When you devide a price by a percentage the price will be the price devided by the percentage.
/// ```
/// # use trading_utils::{Price, Percent};
/// let division = Price(100.0) / Percent(0.05);
/// assert_eq!(division, Price(2_000.0));
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Price(pub(crate) f64);

impl Price {
    pub fn from_relative_price(base_price: Price, relative_price: RelativePrice) -> Self {
        base_price + relative_price
    }

    pub fn from_points(price_per_point: Price, points: Points) -> Self {
        price_per_point * *points
    }

    pub fn from_relative_points(base_price: Price, price_per_point: Price, relative_points: RelativePoints) -> Self {
        base_price + price_per_point * *relative_points
    }
}


impl MarketValue for Price {
    fn new(value: f64) -> Self {
        Self(value)
    }
}

impl_ops!(Price);
