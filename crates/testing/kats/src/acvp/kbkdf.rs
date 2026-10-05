//! ACVP SP 800-108 KBKDF Known Answer Tests
//!
//! The JSON files were taken from <https://github.com/usnistgov/ACVP-Server>
//! (`gen-val/json-files/KDF-1.0`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d).
//!
//! Only a subset of the test groups is included. The files were filtered with
//! ```sh
//! F='.kdfMode=="feedback" and (.macMode|startswith("HMAC")) and .counterLength==32 and .counterLocation=="before fixed data"'
//! jq -c "[.testGroups[]|select($F)|.tgId]" prompt.json > ids.json
//! jq --slurpfile ids ids.json '.testGroups |= map(select(.tgId as $t | $ids[0]|index($t)))' \
//!     prompt.json > feedback-hmac-ctr32-before-fixed/prompt.json
//! jq --slurpfile ids ids.json '.testGroups |= map(select(.tgId as $t | $ids[0]|index($t)))' \
//!     expectedResults.json > feedback-hmac-ctr32-before-fixed/expectedResults.json
//! ```
//!
//! Note that `keyOutLength` is given in **bits** and is not necessarily a multiple of 8.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::kbkdf::FeedbackHmacTests;
//!
//! let tests = FeedbackHmacTests::load();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod schema;

/// HMAC-based feedback mode KBKDF with a 32-bit counter placed between the
/// iteration variable `K(i-1)` and the fixed data.
pub struct FeedbackHmacTests {
    pub prompts: super::schema_common::Prompts<schema::KbkdfPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultKbkdfTestGroup>,
}

super::impl_tests!(
    FeedbackHmacTests,
    "kdf-1_0",
    "feedback-hmac-ctr32-before-fixed"
);
