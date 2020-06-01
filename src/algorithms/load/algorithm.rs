use std::ffi::OsStr;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

use chrono::Duration;
use libloading::Library;

use crate::{AlgorithmInterface, Derivative, Error, ErrorKind, Instruction, Position, Price, TradingErrorKind};

/// The AlgorithmRegistration type represents a mutable pointer to an instance of
/// AlgorithmRegistration
///
/// This pointer is needed to load the static instance created by export_algorithm!
type AlgorithmRegistration = *mut crate::AlgorithmRegistration;

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

    min_data_length: u64,
    max_data_length: u64,

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
    pub fn load<P: AsRef<OsStr>>(path: &P) -> Result<Self, Error<ErrorKind>> {
        let os_path = std::fs::canonicalize(path.as_ref().to_os_string())?;

        if !os_path.exists() {
            return Err(Error::new(
                "Could not find the supplied path".to_string(),
                ErrorKind::IO,
            ))
        }

        let lib = Library::new(path)?;

        let algorithm_registration = unsafe {
            lib
                .get::<AlgorithmRegistration>(b"ALGORITHM_REGISTRATION\0")?
                .read()
        };

        // check that the rustc and the algorithm-utils version of the provided algorithm is
        // the same as the rustc and the algorithm-utils version of this crate.
        // This should make sure that no undefined behaviour occurs when loading the algorithm.
        // To pass this test the algorithm just needs to use the same version of rustc and
        // algorithm-utils like trading-desk used when it was compiled
        if algorithm_registration.rustc_version != crate::RUSTC_VERSION
            || algorithm_registration.utils_version != crate::UTILS_VERSION {
            return Err(Error::new(
                format!(
                    "The algorithm `{}` has a mismatched version\n\
                    Algorithm version: [{}/{}]\nUtils version: [{}/{}]\n\
                    Please update either trading-desk or the algorithm",
                    algorithm_registration.name,
                    algorithm_registration.rustc_version,
                    algorithm_registration.utils_version,
                    crate::RUSTC_VERSION,
                    crate::UTILS_VERSION
                ),
                ErrorKind::MisMatchedVersion,
            ));
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
    }

    #[inline]
    pub const fn name(&self) -> &'static str { &self.name }
    #[inline]
    pub const fn description(&self) -> &'static str { &self.description }
    #[inline]
    pub const fn min_data_length(&self) -> u64 { self.min_data_length }
    #[inline]
    pub const fn max_data_length(&self) -> u64 { self.max_data_length }
    #[inline]
    pub const fn path(&self) -> &PathBuf { &self.path }
}

impl AlgorithmInterface for Algorithm {
    #[inline]
    fn init(&mut self, derivative: &Derivative, time_steps: Duration) -> Result<(), Error<TradingErrorKind>> {
        self.algorithm_box.init(derivative, time_steps)
    }

    #[inline]
    fn collect_prices(&mut self, prices: &[Price]) -> Result<(), Error<TradingErrorKind>> {
        self.algorithm_box.collect_prices(prices)
    }

    #[inline]
    fn algorithm(&mut self, positions: &[Position], prices: &[Price]) -> Result<&[Instruction], Error<TradingErrorKind>> {
        self.algorithm_box.algorithm(positions, prices)
    }

    #[inline]
    fn shutdown(&mut self, positions: &[Position], prices: &[Price]) -> Result<&[Instruction], Error<TradingErrorKind>> {
        self.algorithm_box.shutdown(positions, prices)
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
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
