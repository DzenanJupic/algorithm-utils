use crate::AlgorithmInterface;

#[derive(Clone, Debug)]
pub struct AlgorithmRegistration {
    pub rustc_version: &'static str,
    pub utils_version: &'static str,

    pub name: &'static str,
    pub description: &'static str,

    pub min_data_length: u64,
    pub max_data_length: u64,

    pub initial_algorithm_state_fn: unsafe extern fn() -> Box<dyn AlgorithmInterface>,
}
