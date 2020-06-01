pub use self::algorithm_interface::*;
pub use self::algorithm_registration::*;

pub mod algorithm_interface;
pub mod algorithm_registration;

#[macro_export]
macro_rules! export_algorithm {
    ($name:literal, $description:literal, $algorithm:expr) => (export_algorithm!($name, $description, 0, 0, $algorithm));
    ($name:literal, $description:literal, $min_data_length:literal, $max_data_length:literal, $algorithm:expr) => {
        #[doc(hidden)]
        pub extern fn initial_algorithm_state() -> ::std::boxed::Box<dyn AlgorithmInterface> {
            ::std::boxed::Box::new($algorithm)
        }

        #[doc(hidden)]
        #[no_mangle]
        // needs to be static
        pub static ALGORITHM_REGISTRATION: $crate::AlgorithmRegistration = $crate::AlgorithmRegistration {
            rustc_version: $crate::RUSTC_VERSION,
            utils_version: $crate::UTILS_VERSION,

            name: $name,
            description: $description,

            min_data_length: $min_data_length,
            max_data_length: $max_data_length,

            initial_algorithm_state_fn: initial_algorithm_state,
        };
    };
}
