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

pub mod schema;

/// Two-step KDF using HMAC, with a feedback mode expansion and a 32-bit counter placed
/// between the iteration variable `K(i-1)` and the fixed info.
pub struct TwoStepFeedbackHmacTests {
    pub prompts: super::schema_common::Prompts<schema::TwoStepPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultTwoStepTestGroup>,
}

macro_rules! variant {
    ($variant:literal) => {
        (
            include_str!(concat!(
                "../../acvp/kda-twostep-sp800-56cr2/",
                $variant,
                "/prompt.json"
            )),
            include_str!(concat!(
                "../../acvp/kda-twostep-sp800-56cr2/",
                $variant,
                "/expectedResults.json"
            )),
        )
    };
}

impl TwoStepFeedbackHmacTests {
    /// Load the test vectors for all output lengths `l`.
    pub fn load_all() -> Vec<Self> {
        [
            variant!("feedback-hmac-ctr32-before-fixed-l136"),
            variant!("feedback-hmac-ctr32-before-fixed-l512"),
            variant!("feedback-hmac-ctr32-before-fixed-l1000"),
        ]
        .into_iter()
        .map(|(prompts, results)| Self {
            prompts: serde_json::from_str(prompts).expect("Could not deserialize KAT file."),
            results: serde_json::from_str(results).expect("Could not deserialize KAT file."),
        })
        .collect()
    }
}
