use std::fmt;
use std::fmt::Debug;
use crate::algorithms::export::AlgorithmInterface;

#[derive(Clone, Debug)]
pub struct AlgorithmRegistration {
    pub rustc_version: &'static str,
    pub utils_version: &'static str,

    pub name: &'static str,
    pub description: &'static str,

    pub min_data_length: DataLength,
    pub max_data_length: DataLength,

    pub initial_algorithm_state_fn: unsafe extern fn() -> Box<dyn AlgorithmInterface>,
}

#[derive(Copy, Clone, Debug)]
pub enum DataLength {
    Fixed(usize),
    Variable
}

impl fmt::Display for DataLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?}",
            self
        )
    }
}
