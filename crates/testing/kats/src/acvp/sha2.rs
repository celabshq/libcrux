//! ACVP SHA-2 Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! SHA2-224/256/384/512 (revision 1.0): AFT with byte-oriented messages of up to 16384 bits,
//! MCT (`mctVersion` "standard") and LDT.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::sha2::HashTests;
//!
//! let tests = HashTests::sha2_256();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub use super::hash::{schema, HashTests};

impl HashTests {
    pub fn sha2_224() -> Self {
        load!("sha2-1_0", "sha2-224")
    }

    pub fn sha2_256() -> Self {
        load!("sha2-1_0", "sha2-256")
    }

    pub fn sha2_384() -> Self {
        load!("sha2-1_0", "sha2-384")
    }

    pub fn sha2_512() -> Self {
        load!("sha2-1_0", "sha2-512")
    }
}
