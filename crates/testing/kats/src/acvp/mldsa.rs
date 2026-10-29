//! ACVP ML-DSA Known Answer Tests
//!
//! The JSON files for ML-DSA were taken from <https://github.com/usnistgov/ACVP-Server>, as of
//! commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d)
//! (release v1.1.0.43). `siggen-tr1` is `ML-DSA-sigGen-FIPS204-tr1`.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::mldsa::{KeyGenTests, SigGenTests, SigVerTests};
//!
//! // keygen tests
//! let keygen_tests = KeyGenTests::load();
//!
//! for test_group in keygen_tests.prompts.testGroups {
//!    for test in test_group.tests {
//!        // retrieve the expected result for that test
//!         let expected_result
//!             = keygen_tests.results.find_expected_result(test_group.tgId, test.tcId);
//!    }
//! }
//!
//! // siggen tests
//! let siggen_tests = SigGenTests::load();
//!
//! for test_group in siggen_tests.prompts.testGroups {
//!    for test in test_group.tests {
//!        // retrieve the expected result for that test
//!         let expected_result
//!             = siggen_tests.results.find_expected_result(test_group.tgId, test.tcId);
//!    }
//! }
//!
//! // sigver tests
//! let sigver_tests = SigVerTests::load();
//!
//! for test_group in sigver_tests.prompts.testGroups {
//!    for test in test_group.tests {
//!        // retrieve the expected result for that test
//!         let expected_result
//!             = sigver_tests.results.find_expected_result(test_group.tgId, test.tcId);
//!    }
//! }
//! ```
pub mod keygen_schema;
pub mod sign_schema;
pub mod verify_schema;

pub struct KeyGenTests {
    pub prompts: super::schema_common::Prompts<keygen_schema::KeyGenPromptTestGroup>,
    pub results: super::schema_common::Results<keygen_schema::ResultKeyGenTestGroup>,
}

pub struct SigGenTests {
    pub prompts: super::schema_common::Prompts<sign_schema::SigGenPromptTestGroup>,
    pub results: super::schema_common::Results<sign_schema::ResultSigGenTestGroup>,
}
pub struct SigVerTests {
    pub prompts: super::schema_common::Prompts<verify_schema::SigVerPromptTestGroup>,
    pub results: super::schema_common::Results<verify_schema::ResultSigVerTestGroup>,
}

impl KeyGenTests {
    pub fn load() -> Self {
        load!("mldsa-1_1_0_43", "keygen")
    }
}

impl SigGenTests {
    pub fn load() -> Self {
        load!("mldsa-1_1_0_43", "siggen")
    }

    /// The `ML-DSA-sigGen-FIPS204-tr1` vector set, which also contains signing keys given as
    /// the key generation seed (`keyFormat` `"seed"`).
    pub fn load_tr1() -> Self {
        load!("mldsa-1_1_0_43", "siggen-tr1")
    }
}

impl SigVerTests {
    pub fn load() -> Self {
        load!("mldsa-1_1_0_43", "sigver")
    }
}
