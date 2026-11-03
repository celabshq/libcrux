//! ACVP HMAC_DRBG (SP 800-90Ar1) Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! The registrations cover SHA2-256, SHA2-384 and SHA2-512, with reseeding, without prediction
//! resistance, 256-bit entropy input, 128-bit nonce, and empty or 256-bit personalization
//! strings and additional inputs.
//!
//! Each test case instantiates the DRBG and then performs the operations in
//! [`schema::DrbgPrompt::otherInput`] in order. The expected result is the output of the last
//! `generate` call.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::drbg::HmacDrbgTests;
//!
//! let tests = HmacDrbgTests::sha2_256_sha2_384();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod schema;

pub struct HmacDrbgTests {
    pub prompts: super::schema_common::Prompts<schema::DrbgPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultDrbgTestGroup>,
}

impl HmacDrbgTests {
    pub fn sha2_256_sha2_384() -> Self {
        load!("hmac-drbg-sp800-90ar1", "sha2-256-sha2-384")
    }

    pub fn sha2_512() -> Self {
        load!("hmac-drbg-sp800-90ar1", "sha2-512")
    }
}
