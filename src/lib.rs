pub mod algorithms;
pub mod error;

pub const UTILS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub mod prelude {}

pub fn hash_raw_id(raw_id: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    raw_id.hash(&mut hasher);
    hasher.finish()
}
