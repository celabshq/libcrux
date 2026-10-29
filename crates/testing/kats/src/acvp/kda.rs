//! ACVP SP 800-56Cr2 KDA Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server>, as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`. The JSON files were
//! minified with `jq -c`.
//!
//! The registrations cover the two-step KDF with HMAC-SHA2-{256,384,512} and
//! HMAC-SHA3-{224,256,384,512}, a feedback mode expansion with a 32-bit counter placed
//! between the iteration variable `K(i-1)` and the fixed info, default and random salts,
//! empty and non-empty IVs, as well as multi-expansion tests. Each registration uses a
//! single output length `l`, so there is one set of vectors per `l`.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::kda::{schema::TwoStepPrompt, TwoStepFeedbackHmacTests};
//!
//! for tests in TwoStepFeedbackHmacTests::load_all() {
//!     for test_group in tests.prompts.testGroups {
//!         for test in test_group.tests {
//!             match test {
//!                 TwoStepPrompt::Single(test) => {
//!                     // retrieve the expected result for that test
//!                     let expected_result =
//!                         tests.results.find_expected_result(test_group.tgId, test.tcId);
//!                 }
//!                 TwoStepPrompt::MultiExpansion(test) => { /* ... */ }
//!             }
//!         }
//!     }
//! }
//! ```

pub mod hkdf_schema;
pub mod schema;

/// HKDF (`KDA` `HKDF` `Sp800-56Cr2`) with SHA2-256/384/512 and SHA3-224/256/384/512, default
/// and random salts, and the fixed info `uPartyInfo||vPartyInfo||l`, without multi-expansion
/// tests and hybrid shared secrets.
///
/// Generated with the `GenValAppRunner` from the `registration.json` next to the `prompt.json`.
pub struct HkdfTests {
    pub prompts: super::schema_common::Prompts<hkdf_schema::HkdfPromptTestGroup>,
    pub results: super::schema_common::Results<hkdf_schema::ResultHkdfTestGroup>,
}

impl HkdfTests {
    pub fn sha2_sha3() -> Self {
        load!("kda-hkdf-sp800-56cr2", "sha2-sha3")
    }
}

/// Two-step KDF using HMAC, with a feedback mode expansion and a 32-bit counter placed
/// between the iteration variable `K(i-1)` and the fixed info.
pub struct TwoStepFeedbackHmacTests {
    pub prompts: super::schema_common::Prompts<schema::TwoStepPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultTwoStepTestGroup>,
}

impl TwoStepFeedbackHmacTests {
    /// Load the test vectors for all output lengths `l`.
    pub fn load_all() -> Vec<Self> {
        vec![
            load!(
                "kda-twostep-sp800-56cr2",
                "feedback-hmac-ctr32-before-fixed-l136"
            ),
            load!(
                "kda-twostep-sp800-56cr2",
                "feedback-hmac-ctr32-before-fixed-l512"
            ),
            load!(
                "kda-twostep-sp800-56cr2",
                "feedback-hmac-ctr32-before-fixed-l1000"
            ),
        ]
    }

    /// Load the test vectors matching the use of the two-step KDF in 3KEM.
    ///
    /// The fixed info is `label || 0x00 || uPartyInfo || vPartyInfo || [L]32`, matching 3KEM's
    /// `label || 0x00 || context || [L]32`, with an empty IV and no multi-expansion:
    /// - `3kem-combine-sha3-256`: HMAC-SHA3-256, random salt, `z` of 640 bits, `l` = 256
    ///   (`KEM.combine` of the default suite).
    /// - `3kem-dhkdf-combine-sha3-384`: HMAC-SHA3-384, default and random salt, `z` of 384 and
    ///   640 bits, `l` = 384 (`DH_KDF`, and `KEM.combine` of the level 5 suite).
    /// - `3kem-nho-sha3-256` and `3kem-nho-sha3-384`: HMAC-SHA3-256/384, random salt, `z` of
    ///   256/384 bits, `l` = 1536 (`NHO.finalize`; `l` is a managed parameter in 3KEM).
    pub fn load_3kem() -> Vec<Self> {
        vec![
            load!("kda-twostep-sp800-56cr2", "3kem-combine-sha3-256"),
            load!("kda-twostep-sp800-56cr2", "3kem-dhkdf-combine-sha3-384"),
            load!("kda-twostep-sp800-56cr2", "3kem-nho-sha3-256"),
            load!("kda-twostep-sp800-56cr2", "3kem-nho-sha3-384"),
        ]
    }
}
