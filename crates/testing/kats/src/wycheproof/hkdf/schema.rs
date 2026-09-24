//! Structs based on
//! [`schemas/hkdf_test_schema_v1.json`](https://github.com/C2SP/wycheproof/blob/cd136e97040de0842c3a198670b1c5e4f423c940/schemas/hkdf_test_schema_v1.json)

pub use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

pub type Notes = std::collections::HashMap<Flag, NotesEntry>;

/// Top-level HKDF test suite (one per hash variant).
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HkdfTests {
    /// The primitive tested, e.g. `"HKDF-SHA-256"`.
    pub algorithm: String,

    /// Schema identifier.
    pub schema: String,

    /// Total number of test cases across all groups.
    pub number_of_tests: i64,

    pub notes: Notes,

    pub test_groups: Vec<TestGroup>,
}

/// A group of HKDF tests sharing the same key size.
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestGroup {
    #[serde(rename = "type")]
    pub test_group_type: Type,

    pub source: Source,

    /// Key (input keying material) size in bits.
    pub key_size: u32,

    pub tests: Vec<Test>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "HkdfTest")]
    HkdfTest,
}

/// A single HKDF test case.
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    /// Identifier of the test case.
    pub tc_id: i64,

    /// A brief description of the test case.
    pub comment: String,

    /// The input keying material.
    #[serde(with = "hex::serde")]
    pub ikm: Vec<u8>,

    /// The salt.
    #[serde(with = "hex::serde")]
    pub salt: Vec<u8>,

    /// The context and application specific information.
    #[serde(with = "hex::serde")]
    pub info: Vec<u8>,

    /// The requested output keying material length, in bytes.
    pub size: usize,

    /// The expected output keying material.
    #[serde(with = "hex::serde")]
    pub okm: Vec<u8>,

    /// Test result.
    pub result: TestResult,

    /// A list of flags.
    pub flags: Vec<Flag>,
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Flag {
    /// Normal case test vector.
    Normal,
    /// Multiple outputs with the same ikm, salt, and info but different sizes;
    /// the shorter outputs must be a prefix of the longer ones.
    OutputCollision,
    /// The requested output size is the maximal size allowed by the spec.
    MaximalOutputSize,
    /// The salt is empty.
    EmptySalt,
    /// The requested output size is larger than allowed by the spec.
    SizeTooLarge,
}
