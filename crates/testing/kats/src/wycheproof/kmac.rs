//! Wycheproof KMAC Known Answer Tests
//!
//! The JSON files for KMAC were taken from <https://github.com/google/wycheproof>.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::wycheproof::kmac::{Algorithm, KmacTests};
//!
//! // load the tests for KMAC128
//! let tests = KmacTests::load(Algorithm::Kmac128);
//!
//! for test_group in &tests.test_groups {
//!     for test in &test_group.tests {
//!         // ...
//!     }
//! }
//! ```

pub mod schema;

pub use schema::*;

/// KMAC algorithm variants
pub enum Algorithm {
    Kmac128,
    Kmac256,
}

impl KmacTests {
    fn kmac128() -> Self {
        let data: &str = include_str!("../../wycheproof/kmac128_no_customization_test.json");
        serde_json::from_str(data).expect("Could not deserialize KAT file.")
    }

    fn kmac256() -> Self {
        let data: &str = include_str!("../../wycheproof/kmac256_no_customization_test.json");
        serde_json::from_str(data).expect("Could not deserialize KAT file.")
    }

    /// Load the [`KmacTests`] for the given [`Algorithm`].
    pub fn load(algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::Kmac128 => Self::kmac128(),
            Algorithm::Kmac256 => Self::kmac256(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // test that data is loaded successfully
    #[test]
    fn test_load() {
        KmacTests::load(Algorithm::Kmac128);
        KmacTests::load(Algorithm::Kmac256);
    }
}
