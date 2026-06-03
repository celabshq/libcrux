// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::mlkem_schema;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: mlkem_schema = serde_json::from_str(&json).unwrap();
// }

pub use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

pub type Notes = std::collections::HashMap<Flag, NotesEntry>;

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemTests {
    pub algorithm: String,

    pub schema: String,

    pub number_of_tests: i64,

    pub notes: Notes,

    pub test_groups: Vec<TestGroup>,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum ParameterSet {
    #[serde(rename = "ML-KEM-512")]
    MlKem512,
    #[serde(rename = "ML-KEM-768")]
    MlKem768,
    #[serde(rename = "ML-KEM-1024")]
    MlKem1024,
}

/// Test group for keygen and/or decaps
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemTestGroup {
    pub source: Source,

    pub parameter_set: ParameterSet,

    pub tests: Vec<MlKemTest>,
}

/// Test group for encaps
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemEncapsTestGroup {
    pub source: Source,

    pub parameter_set: ParameterSet,

    pub tests: Vec<MlKemEncapsTest>,
}

/// Test group for decaps
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemDecapsValidationTestGroup {
    pub source: Source,

    pub parameter_set: ParameterSet,

    pub tests: Vec<MlKemDecapsValidationTest>,
}

/// Test for encaps
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TestGroup {
    #[serde(rename = "MLKEMTest")]
    MlKemTestGroup(MlKemTestGroup),
    #[serde(rename = "MLKEMEncapsTest")]
    MlKemEncapsTestGroup(MlKemEncapsTestGroup),
    #[serde(rename = "MLKEMDecapsValidationTest")]
    MlKemDecapsValidationTestGroup(MlKemDecapsValidationTestGroup),
}

/// Test for keygen and/or decaps
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemTest {
    /// Identifier of the test case
    pub tc_id: i64,

    pub flags: Vec<Flag>,

    /// The d || z seed
    #[serde(with = "hex::serde")]
    pub seed: Vec<u8>,

    /// The encapsulation key derived from the seed
    #[serde(rename = "ek")]
    // is empty for some MlKemResult::Invalid tests
    // Option<Vec<u8>> does not work with hex::serde
    #[serde(default)]
    #[serde(with = "hex::serde")]
    pub encapsulation_key: Vec<u8>,

    /// An input ciphertext
    #[serde(rename = "c")]
    #[serde(with = "hex::serde")]
    pub ciphertext: Vec<u8>,

    /// The output shared secret
    #[serde(rename = "K")]
    #[serde(with = "hex::serde")]
    pub shared_secret: Vec<u8>,

    /// Test result
    pub result: TestResult,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemEncapsTest {
    /// Identifier of the test case
    pub tc_id: i64,

    pub flags: Vec<Flag>,

    /// The encapsulation key
    #[serde(rename = "ek")]
    #[serde(with = "hex::serde")]
    pub encapsulation_key: Vec<u8>,

    /// The ML-KEM.Encaps_internal m input
    #[serde(with = "hex::serde")]
    pub m: Vec<u8>,

    /// The output ciphertext
    #[serde(rename = "c")]
    #[serde(with = "hex::serde")]
    pub ciphertext: Vec<u8>,

    /// The output shared secret
    #[serde(rename = "K")]
    #[serde(with = "hex::serde")]
    pub shared_secret: Vec<u8>,

    /// Test result
    pub result: TestResult,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemDecapsValidationTest {
    /// Identifier of the test case
    pub tc_id: i64,

    pub flags: Vec<Flag>,

    /// The decapsulation key
    #[serde(rename = "dk")]
    #[serde(with = "hex::serde")]
    pub decapsulation_key: Vec<u8>,

    /// The output ciphertext
    #[serde(rename = "c")]
    #[serde(with = "hex::serde")]
    pub ciphertext: Vec<u8>,

    /// Test result
    pub result: TestResult,
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Flag {
    ModulusOverflow,
    Strcmp,
    IncorrectCiphertextLength,
    IncorrectDecapsulationKeyLength,
    InvalidDecapsulationKey,
}
