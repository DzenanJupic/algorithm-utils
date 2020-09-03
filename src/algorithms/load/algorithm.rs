use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

use chrono::Duration;
use libloading::Library;
use crate::algorithms::export::{AlgorithmInterface, DataLength, algorithm_registration};
use crate::error::GeneralError;


/// The AlgorithmRegistration type represents a mutable pointer to an instance of
/// AlgorithmRegistration
///
/// This pointer is needed to load the static instance created by export_algorithm!
type AlgorithmRegistration = *mut algorithm_registration::AlgorithmRegistration;

/// a wrapper around an extern AlgorithmInterface
///
/// This wrapper provides convenient access to a Box<dyn AlgorithmInterface.
/// This dynamic AlgorithmInterface usually is a dynamically loaded library that contains
/// an algorithm.
/// The _lib field is for the borrow checker to keep the library alive as long as an instance
/// of it is used.
pub struct Algorithm {
    name: &'static str,
    description: &'static str,

    min_data_length: DataLength,
    max_data_length: DataLength,

    path: PathBuf,
    algorithm_box: Box<dyn AlgorithmInterface>,
    _lib: Library,
}

impl Algorithm {
    /// loads an algorithm by path
    ///
    /// this function loads an algorithm from a dynamically loaded library.
    /// To return Ok:
    ///     * the path needs to be valid
    ///     * the provided library needs to contain a static variable called `ALGORITHM_REGISTRATION`
    ///     * This variable needs to contain an instance of the AlgorithmRegistration struct
    pub fn load<P: AsRef<OsStr>>(path: &P) -> Result<Self, GeneralError> {
        let os_path = std::fs::canonicalize(path.as_ref().to_os_string())?;

        if !os_path.exists() {
            println!("Could not find the supplied path ({:?})", path.as_ref());
            return Err(GeneralError::IO);
        }

        let lib = Library::new(path)?;

        std::panic::catch_unwind(|| {
            let algorithm_registration = unsafe {
                lib
                    .get::<AlgorithmRegistration>(b"ALGORITHM_REGISTRATION\0")?
                    .read()
            };


            // check that the rustc and the trading-utils version of the provided algorithm is
            // the same as the rustc and the trading-utils version of this crate.
            // This should make sure that no undefined behaviour occurs when loading the algorithm.
            // To pass this test the algorithm just needs to use the same version of rustc and
            // trading-utils like trading-desk used when it was compiled
            if algorithm_registration.rustc_version != crate::RUSTC_VERSION
                || algorithm_registration.utils_version != crate::UTILS_VERSION {
                println!(
                    "The algorithm `{}` has a mismatched version\n\
                    Algorithm version: [{}/{}]\nUtils version: [{}/{}]\n\
                    Please update either trading-desk or the algorithm",
                    algorithm_registration.name,
                    algorithm_registration.rustc_version,
                    algorithm_registration.utils_version,
                    crate::RUSTC_VERSION,
                    crate::UTILS_VERSION
                );
                panic!();
            }

            if let DataLength::Fixed(max) = algorithm_registration.max_data_length {
                if let DataLength::Fixed(min) = algorithm_registration.min_data_length {
                    if max > min {
                        println!(
                            "The algorithm `{}` is not configured correctly\n\
                            (min_data_length: {}, max_data_length: {})\n\
                            (max_data_length needs to be greater or equal to min_data_length or DataLength::Variable)",
                            algorithm_registration.name,
                            min, max,
                        );
                        panic!();
                    }
                } else if max == 0 {
                    println!(
                        "The algorithm `{}` is not configured correctly\n\
                        (max_data_length needs to be greater then 0)",
                        algorithm_registration.name,
                    );
                    panic!();
                }
            }

            let algorithm_box = unsafe { (algorithm_registration.initial_algorithm_state_fn)() };

            Ok(Self {
                name: algorithm_registration.name,
                description: algorithm_registration.description,
                min_data_length: algorithm_registration.min_data_length,
                max_data_length: algorithm_registration.max_data_length,
                path: os_path,
                algorithm_box,
                _lib: lib,
            })
        }).unwrap_or(Err(GeneralError::LibLoading))
    }

    pub fn name(&self) -> &'static str { &self.name }
    pub fn description(&self) -> &'static str { &self.description }
    pub fn min_data_length(&self) -> DataLength { self.min_data_length }
    pub fn max_data_length(&self) -> DataLength { self.max_data_length }
    pub fn path(&self) -> &PathBuf { &self.path }
}

impl AlgorithmInterface for Algorithm {
    fn init(&mut self, deposit: &Deposit, derivative: &Derivative, time_steps: Duration) -> Result<(), TradingError> {
        self.algorithm_box.init(deposit, derivative, time_steps)
    }

    fn collect_prices(&mut self, prices: &[Price]) -> Result<(), TradingError> {
        self.algorithm_box.collect_prices(prices)
    }

    fn algorithm(&mut self, deposit: &Deposit, prices: &[Price], instructions: &mut Vec<Instruction>) -> Result<(), TradingError> {
        self.algorithm_box.algorithm(deposit, prices, instructions)
    }

    fn shutdown(&mut self, deposit: &Deposit, prices: &[Price], instructions: &mut Vec<Instruction>) -> Result<(), TradingError> {
        self.algorithm_box.shutdown(deposit, prices, instructions)
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} ({:?})\n\n\
            minimal data length: {}\n\
            maximal data length: {}\n\n\
            {}",
            self.name, self.path,
            self.min_data_length,
            self.max_data_length,
            self.description
        )
    }
}
