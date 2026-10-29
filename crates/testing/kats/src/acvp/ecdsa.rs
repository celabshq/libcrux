//! ACVP ECDSA (FIPS 186-5) Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! - keyGen: P-256 and P-384, secret generation mode `"testing candidates"` (FIPS 186-5, A.2.2). The IUT
//!   generates the key pairs, so the prompt only contains test case ids. The set is generated as
//!   a sample, so the expected results contain example key pairs `(d, Q)`.
//! - keyVer: P-256 and P-384, public key validation.
//! - sigGen: P-256 with SHA2-256/384/512. The IUT generates a key pair per test group and
//!   randomized signatures, so the (sample) expected results only contain example signatures.
//! - sigVer: P-256 with SHA2-256/384/512.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::ecdsa::KeyVerTests;
//!
//! let tests = KeyVerTests::p384();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod schema;

pub struct KeyGenTests {
    pub prompts: super::schema_common::Prompts<schema::KeyGenPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultKeyGenTestGroup>,
}

impl KeyGenTests {
    pub fn p256() -> Self {
        load!("ecdsa-keygen-fips186-5", "p-256")
    }

    pub fn p384() -> Self {
        load!("ecdsa-keygen-fips186-5", "p-384")
    }
}

pub struct KeyVerTests {
    pub prompts: super::schema_common::Prompts<schema::KeyVerPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultKeyVerTestGroup>,
}

impl KeyVerTests {
    pub fn p256() -> Self {
        load!("ecdsa-keyver-fips186-5", "p-256")
    }

    pub fn p384() -> Self {
        load!("ecdsa-keyver-fips186-5", "p-384")
    }
}

pub struct SigGenTests {
    pub prompts: super::schema_common::Prompts<schema::SigGenPromptTestGroup>,
    pub results: super::schema_common::Results<schema::SigGenResultTestGroup>,
}

impl SigGenTests {
    pub fn p256() -> Self {
        load!("ecdsa-siggen-fips186-5", "p-256")
    }
}

pub struct SigVerTests {
    pub prompts: super::schema_common::Prompts<schema::SigVerPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultSigVerTestGroup>,
}

impl SigVerTests {
    pub fn p256() -> Self {
        load!("ecdsa-sigver-fips186-5", "p-256")
    }
}
