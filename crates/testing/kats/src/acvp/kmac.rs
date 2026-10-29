//! ACVP KMAC (SP 800-185) Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! The registrations cover KMAC-128 and KMAC-256 (revision 1.0) without XOF, with byte-oriented
//! lengths, keys of at least the security strength, and customization strings given as text
//! and as hex.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::kmac::KmacTests;
//!
//! let tests = KmacTests::kmac_128();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod schema;

pub struct KmacTests {
    pub prompts: super::schema_common::Prompts<schema::KmacPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultKmacTestGroup>,
}

impl KmacTests {
    pub fn kmac_128() -> Self {
        load!("kmac-1_0", "kmac-128")
    }

    pub fn kmac_256() -> Self {
        load!("kmac-1_0", "kmac-256")
    }
}
