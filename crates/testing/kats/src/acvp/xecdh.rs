//! ACVP XECDH (RFC 7748) Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! The registrations cover Curve25519 (X25519):
//! - keyGen: generated as a sample. The IUT generates the key pairs, so the expected results only
//!   contain example key pairs.
//! - keyVer: public key validation, which for X25519 only checks the length.
//! - SSC: shared secret computation. The IUT chooses its key pair, so the set is generated as a
//!   sample, and the `internalProjection.json` with the private keys is included as
//!   [`SscTests::projection`].
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::xecdh::KeyVerTests;
//!
//! let tests = KeyVerTests::curve25519();
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
    pub fn curve25519() -> Self {
        load!("xecdh-rfc7748", "keygen-curve25519")
    }
}

pub struct KeyVerTests {
    pub prompts: Prompts<schema::KeyVerPromptTestGroup>,
    pub results: Results<schema::ResultKeyVerTestGroup>,
}

impl KeyVerTests {
    pub fn curve25519() -> Self {
        load!("xecdh-rfc7748", "keyver-curve25519")
    }
}

pub struct SscTests {
    pub prompts: Prompts<schema::SscPromptTestGroup>,
    pub results: Results<schema::ResultSscTestGroup>,
    /// The tests including the private key of the IUT.
    pub projection: Prompts<schema::SscPromptTestGroup>,
}

impl SscTests {
    pub fn curve25519() -> Self {
        load!("xecdh-rfc7748", "ssc-curve25519", projection)
    }

    /// The test case `tcId` of the test group `tgId`, including the IUT private key.
    #[allow(non_snake_case)]
    pub fn find_projection(&self, tgId: usize, tcId: usize) -> &schema::SscPrompt {
        self.projection.find_test(tgId, tcId)
    }
}
