pub use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

pub type Notes = std::collections::HashMap<Flag, NotesEntry>;

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    /// Identifier of the test case
    pub tc_id: i64,

    /// A brief description of the test case
    pub comment: String,

    /// The message to sign
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,

    /// [optional] The additional context string (empty if not provided)
    #[serde(default)]
    #[serde(with = "hex::serde")]
    pub ctx: Vec<u8>,

    /// The encoded signature (empty in case of expected failure)
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
    IncorrectPrivateKeyLength,
    InvalidPrivateKey,
    InvalidContext,
    ManySteps,
    ValidSignature,
}
