//! Structs based on
//! [`schemas/mac_test_schema_v1.json`](https://github.com/C2SP/wycheproof/blob/cd136e97040de0842c3a198670b1c5e4f423c940/schemas/mac_test_schema_v1.json)

pub use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

pub type Notes = std::collections::HashMap<Flag, NotesEntry>;

/// Top-level HMAC test suite (one per hash variant).
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacTests {
    /// The primitive tested, e.g. `"HMACSHA256"`.
    pub algorithm: String,

    /// Schema identifier.
    pub schema: String,

    /// Total number of test cases across all groups.
    pub number_of_tests: i64,

    pub notes: Notes,

    pub test_groups: Vec<TestGroup>,
}

/// A group of HMAC tests sharing the same key size and tag size.
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestGroup {
    #[serde(rename = "type")]
    pub test_group_type: Type,

    pub source: Source,

    /// Key size in bits.
    pub key_size: u32,

    /// Tag (output) size in bits.
    pub tag_size: u32,

    pub tests: Vec<Test>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MacTest")]
    MacTest,
}

/// A single HMAC test case.
#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    /// Identifier of the test case.
    pub tc_id: i64,

    /// A brief description of the test case.
    pub comment: String,

    /// The HMAC key.
    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,

    /// The message to authenticate.
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,

    /// The expected (or modified) MAC tag.
    #[serde(with = "hex::serde")]
    pub tag: Vec<u8>,

    /// Test result.
    pub result: TestResult,

    /// A list of flags.
    pub flags: Vec<Flag>,
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Flag {
    /// Correctness test with pseudorandom inputs.
    Pseudorandom,
    /// Tag was modified; verification must reject it.
    ModifiedTag,
}
