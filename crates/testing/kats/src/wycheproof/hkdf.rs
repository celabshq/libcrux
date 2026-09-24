//! Wycheproof HKDF Known Answer Tests
//!
//! The JSON files were taken from <https://github.com/C2SP/wycheproof>, as of commit
//! [cd136e97040de0842c3a198670b1c5e4f423c940](https://github.com/C2SP/wycheproof/tree/cd136e97040de0842c3a198670b1c5e4f423c940).
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::wycheproof::hkdf::{HashAlgorithm, HkdfTests};
//!
//! let tests = HkdfTests::load(HashAlgorithm::Sha256);
//!
//! for test_group in &tests.test_groups {
//!     for test in &test_group.tests {
//!         // use test.ikm, test.salt, test.info, test.size, test.okm, test.result ...
//!     }
//! }
//! ```

pub mod schema;

pub use schema::*;

/// The hash function used inside HKDF.
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

macro_rules! impl_load {
    ($name:ident, $file:literal) => {
        impl HkdfTests {
            fn $name() -> Self {
                let data: &str =
                    include_str!(concat!("../../wycheproof/hkdf_", $file, "_test.json"));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }
        }
    };
}

impl_load!(load_sha256, "sha256");
impl_load!(load_sha384, "sha384");
impl_load!(load_sha512, "sha512");

impl HkdfTests {
    /// Load the [`HkdfTests`] for the given [`HashAlgorithm`].
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
        HkdfTests::load(HashAlgorithm::Sha256);
        HkdfTests::load(HashAlgorithm::Sha384);
        HkdfTests::load(HashAlgorithm::Sha512);
    }
}
