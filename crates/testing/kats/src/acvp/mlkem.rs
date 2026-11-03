//! ACVP ML-KEM Known Answer Tests
//!
//! The JSON files for ML-KEM were taken from <https://github.com/usnistgov/ACVP-Server>, as of
//! commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d)
//! (release v1.1.0.43). The encap-decap files are from `ML-KEM-encapDecap-FIPS203-tr1`.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::mlkem::{
//!     KeyGenTests, EncapDecapTests, encap_decap_schema::EncapDecapTestPrompts
//! };
//!
//! // keygen tests
//! let keygen_tests = KeyGenTests::load();
//!
//! for test_group in keygen_tests.prompts.testGroups {
//!     for test in test_group.tests {
//!        // retrieve the expected result for that test
//!         let expected_result
//!             = keygen_tests.results.find_expected_result(test_group.tgId, test.tcId);
//!    }
//! }
//!
//! // encap-decap tests
//! let encap_decap_tests = EncapDecapTests::load();
//!
//! for test_group in encap_decap_tests.prompts.testGroups {
//!     match test_group.tests {
//!         // encapsulation
//!         EncapDecapTestPrompts::EncapTests { tests } => {
//!             for test in tests {
//!                 // retrieve the expected result for that test
//!                 let expected_result
//!                     = encap_decap_tests.results
//!                         .find_expected_result(test_group.tgId, test.tcId);
//!             }
//!
//!         },
//!         // decapsulation
//!         EncapDecapTestPrompts::DecapTests { tests } => { /* ... */ },
//!         // encapsulationKeyCheck
//!         EncapDecapTestPrompts::EncapKeyCheck { tests } => { /* ... */ },
//!         // decapsulationKeyCheck
//!         EncapDecapTestPrompts::DecapKeyCheck { tests } => { /* ... */ },
//!     }
//! }
//!
//! ```

pub mod encap_decap_schema;
pub mod keygen_schema;

pub struct KeyGenTests {
    pub prompts: super::schema_common::Prompts<keygen_schema::KeyGenPromptTestGroup>,
    pub results: super::schema_common::Results<keygen_schema::ResultKeyGenTestGroup>,
}

pub struct EncapDecapTests {
    pub prompts: super::schema_common::Prompts<encap_decap_schema::EncapDecapPromptTestGroup>,
    pub results: super::schema_common::Results<encap_decap_schema::ResultEncapDecapTestGroup>,
}

impl KeyGenTests {
    pub fn load() -> Self {
        load!("mlkem-1_1_0_43", "keygen")
    }
}

impl EncapDecapTests {
    pub fn load() -> Self {
        load!("mlkem-1_1_0_43", "encap-decap")
    }
}
