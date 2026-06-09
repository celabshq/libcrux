//! Wycheproof HMAC Known Answer Tests
//!
//! The JSON files were taken from <https://github.com/C2SP/wycheproof>, as of commit
//! [cd136e97040de0842c3a198670b1c5e4f423c940](https://github.com/C2SP/wycheproof/tree/cd136e97040de0842c3a198670b1c5e4f423c940).
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::wycheproof::hmac::{HashAlgorithm, HmacTests};
//!
//! let tests = HmacTests::load(HashAlgorithm::Sha256);
//!
//! for test_group in &tests.test_groups {
//!     for test in &test_group.tests {
//!         // use test.key, test.msg, test.tag, test.result ...
//!     }
//! }
//! ```

pub mod schema;

pub use schema::*;

/// The hash function used inside HMAC.
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

macro_rules! impl_load {
    ($name:ident, $file:literal) => {
        impl HmacTests {
            fn $name() -> Self {
                let data: &str =
                    include_str!(concat!("../../wycheproof/hmac_", $file, "_test.json"));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }
        }
    };
}

impl_load!(load_sha256, "sha256");
impl_load!(load_sha384, "sha384");
impl_load!(load_sha512, "sha512");

impl HmacTests {
    /// Load the [`HmacTests`] for the given [`HashAlgorithm`].
    pub fn load(algorithm: HashAlgorithm) -> Self {
        match algorithm {
            HashAlgorithm::Sha256 => Self::load_sha256(),
            HashAlgorithm::Sha384 => Self::load_sha384(),
            HashAlgorithm::Sha512 => Self::load_sha512(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load() {
        HmacTests::load(HashAlgorithm::Sha256);
        HmacTests::load(HashAlgorithm::Sha384);
        HmacTests::load(HashAlgorithm::Sha512);
    }
}
