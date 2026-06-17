pub use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

pub type Notes = std::collections::HashMap<Flag, NotesEntry>;

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KmacTests {
    pub algorithm: String,

    pub schema: String,

    pub number_of_tests: i64,

    pub notes: Notes,

    pub test_groups: Vec<KmacTestGroup>,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KmacTestGroup {
    pub source: Source,

    /// Key size in bits
    pub key_size: u32,

    /// Tag size in bits
    pub tag_size: u32,

    pub tests: Vec<KmacTest>,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KmacTest {
    /// Identifier of the test case
    pub tc_id: i64,

    pub comment: String,

    pub flags: Vec<Flag>,

    /// The KMAC key
    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,

    /// The input message
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,

    /// The expected MAC tag
    #[serde(with = "hex::serde")]
    pub tag: Vec<u8>,

    /// Test result
    pub result: KmacResult,
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Flag {
    Pseudorandom,
    ModifiedTag,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KmacResult {
    Valid,
    Invalid,
    Acceptable,
}
