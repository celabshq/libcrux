//! ACVP KAS-ECC-SSC (SP 800-56Ar3) Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! The registrations cover P-256 and P-384 with the `ephemeralUnified` and `onePassDh` schemes, both
//! roles, and no hashing of the shared secret `z`.
//!
//! In the AFT the implementation under test (IUT) generates its own key pair, so the expected
//! results only contain its public key and `z`. The vector set is therefore generated as a
//! sample, and the `internalProjection.json` is included as [`KasEccSscTests::projection`],
//! which also contains the IUT private keys.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::kas_ecc::KasEccSscTests;
//!
//! let tests = KasEccSscTests::p384();
//!
//! for test_group in &tests.prompts.testGroups {
//!     for test in &test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!         // retrieve the test including the IUT private key
//!         let projection = tests.find_projection(test_group.tgId, test.tcId);
//!         // the key of the IUT for its role in the scheme
//!         let d = projection.iut_private_key(test_group.iut_ephemeral());
//!     }
//! }
//! ```

pub mod schema;

use super::schema_common::{Prompts, Results};

pub struct KasEccSscTests {
    pub prompts: Prompts<schema::KasPromptTestGroup>,
    pub results: Results<schema::ResultKasTestGroup>,
    pub projection: Prompts<schema::KasPromptTestGroup>,
}

impl KasEccSscTests {
    pub fn p256() -> Self {
        load!("kas-ecc-ssc-sp800-56ar3", "p-256", projection)
    }

    pub fn p384() -> Self {
        load!("kas-ecc-ssc-sp800-56ar3", "p-384", projection)
    }

    /// The test case `tcId` of the test group `tgId`, including the IUT private keys.
    #[allow(non_snake_case)]
    pub fn find_projection(&self, tgId: usize, tcId: usize) -> &schema::KasPrompt {
        self.projection.find_test(tgId, tcId)
    }
}
