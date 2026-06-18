pub use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

pub type Notes = std::collections::HashMap<Flag, NotesEntry>;
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Verify tests for ML-DSA
pub struct MlDsaVerifyTests {
    /// the primitive tested in the test file
    pub algorithm: String,

    pub generator_version: String,

    /// additional documentation
    pub header: Vec<String>,

    pub notes: Notes,

    /// the number of test vectors in this test
    pub number_of_tests: i64,

    pub schema: String,

    pub test_groups: Vec<TestGroup>,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundaryCondition {
    pub bug_type: String,

    pub description: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestGroup {
    #[serde(rename = "type")]
    pub test_group_type: Type,

    /// Encoded ML-DSA public key
    #[serde(with = "hex::serde")]
    pub public_key: Vec<u8>,

    pub tests: Vec<Test>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MlDsaVerify")]
    MlDsaVerify,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    /// Identifier of the test case
    pub tc_id: i64,

    /// A brief description of the test case
    pub comment: String,

    /// The message to verify
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,

    /// [optional] The additional context string (empty if not provided)
    #[serde(default, with = "hex::serde")]
    pub ctx: Vec<u8>,

    /// The encoded signature
    #[serde(with = "hex::serde")]
    pub sig: Vec<u8>,

    /// Test result
    pub result: TestResult,

    /// A list of flags
    pub flags: Vec<Flag>,
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Flag {
    BoundaryCondition,
    IncorrectPublicKeyLength,
    IncorrectSignatureLength,
    InvalidHintsEncoding,
    InvalidPrivateKey,
    InvalidContext,
    ManySteps,
    ModifiedSignature,
    ValidSignature,
    ZeroPublicKey,
    InfinityNormViolation,
    InvalidSignature,
}
