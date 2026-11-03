//! ACVP HMAC Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! The registrations cover HMAC-SHA2-256/384/512 and HMAC-SHA3-224/256/384/512 (revision 2.0)
//! with byte-oriented key and message lengths, and untruncated MACs only.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::hmac::HmacTests;
//!
//! let tests = HmacTests::hmac_sha3_256();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod schema;

pub struct HmacTests {
    pub prompts: super::schema_common::Prompts<schema::HmacPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultHmacTestGroup>,
}

impl HmacTests {
    pub fn hmac_sha2_256() -> Self {
        load!("hmac-sha2-2_0", "hmac-sha2-256")
    }

    pub fn hmac_sha2_384() -> Self {
        load!("hmac-sha2-2_0", "hmac-sha2-384")
    }

    pub fn hmac_sha2_512() -> Self {
        load!("hmac-sha2-2_0", "hmac-sha2-512")
    }

    pub fn hmac_sha3_224() -> Self {
        load!("hmac-sha3-2_0", "hmac-sha3-224")
    }

    pub fn hmac_sha3_256() -> Self {
        load!("hmac-sha3-2_0", "hmac-sha3-256")
    }

    pub fn hmac_sha3_384() -> Self {
        load!("hmac-sha3-2_0", "hmac-sha3-384")
    }

    pub fn hmac_sha3_512() -> Self {
        load!("hmac-sha3-2_0", "hmac-sha3-512")
    }
}
