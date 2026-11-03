//! ACVP SHA-3 and SHAKE Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`. The registrations
//! only use byte-oriented message and output lengths.
//!
//! - SHA3-224/256/384/512 (revision 2.0): AFT, MCT (`mctVersion` "standard") and LDT.
//! - SHAKE-128/256 (revision 1.0): AFT, MCT and VOT, with output lengths of 128 to 4096 bits.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::sha3::HashTests;
//!
//! let tests = HashTests::sha3_256();
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
    pub fn sha3_224() -> Self {
        load!("sha3-2_0", "sha3-224")
    }

    pub fn sha3_256() -> Self {
        load!("sha3-2_0", "sha3-256")
    }

    pub fn sha3_384() -> Self {
        load!("sha3-2_0", "sha3-384")
    }

    pub fn sha3_512() -> Self {
        load!("sha3-2_0", "sha3-512")
    }

    pub fn shake128() -> Self {
        load!("shake-1_0", "shake-128")
    }

    pub fn shake256() -> Self {
        load!("shake-1_0", "shake-256")
    }
}
