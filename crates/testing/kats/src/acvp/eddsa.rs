//! ACVP EdDSA (revision 1.0) Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! The registrations cover Ed25519 in pure mode (no pre-hash, no context):
//! - keyGen: generated as a sample. The IUT generates the key pairs, so the expected results only
//!   contain example key pairs `(d, q)`.
//! - sigGen: AFT and bit flip tests (BFT). The IUT chooses a key pair per group, so the set is
//!   generated as a sample, and the `internalProjection.json` with the private keys is included
//!   as [`SigGenTests::projection`].
//! - sigVer.
//!
//! keyVer (public key validation) is not covered, because libcrux has no API to validate an
//! Ed25519 public key.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::eddsa::SigVerTests;
//!
//! let tests = SigVerTests::ed25519();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod schema;

use super::schema_common::{Prompts, Results};

pub struct KeyGenTests {
    pub prompts: Prompts<schema::KeyGenPromptTestGroup>,
    pub results: Results<schema::ResultKeyGenTestGroup>,
}

impl KeyGenTests {
    pub fn ed25519() -> Self {
        load!("eddsa-1_0", "keygen-ed25519")
    }
}

pub struct SigGenTests {
    pub prompts: Prompts<schema::SigGenPromptTestGroup>,
    pub results: Results<schema::ResultSigGenTestGroup>,
    /// The test groups including the private key `d` of each group.
    pub projection: Prompts<schema::SigGenPromptTestGroup>,
}

impl SigGenTests {
    pub fn ed25519() -> Self {
        load!("eddsa-1_0", "siggen-ed25519", projection)
    }

    /// The test group `tgId`, including the private key `d`.
    #[allow(non_snake_case)]
    pub fn find_projection(&self, tgId: usize) -> &schema::SigGenPromptTestGroup {
        self.projection.find_group(tgId)
    }
}

pub struct SigVerTests {
    pub prompts: Prompts<schema::SigVerPromptTestGroup>,
    pub results: Results<schema::ResultSigVerTestGroup>,
}

impl SigVerTests {
    pub fn ed25519() -> Self {
        load!("eddsa-1_0", "sigver-ed25519")
    }
}
