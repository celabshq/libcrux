use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct GcmPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,

    /// Omitted if the IV is generated internally.
    #[serde(default, with = "hex::serde")]
    pub iv: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub aad: Vec<u8>,

    /// Encryption only.
    #[serde(default, with = "hex::serde")]
    pub pt: Vec<u8>,

    /// Decryption only.
    #[serde(default, with = "hex::serde")]
    pub ct: Vec<u8>,

    /// Decryption only.
    #[serde(default, with = "hex::serde")]
    pub tag: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct GcmPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    /// `"encrypt"` or `"decrypt"`.
    pub direction: String,
    /// Key length in bits.
    pub keyLen: usize,
    /// IV length in bits.
    pub ivLen: usize,
    /// `"external"` or `"internal"`.
    pub ivGen: String,
    /// The SP 800-38D construction of an internally generated IV, `"8.2.1"` or `"8.2.2"`.
    /// Not present in newer vector sets with an external IV.
    pub ivGenMode: Option<String>,
    /// Payload length in bits.
    pub payloadLen: usize,
    /// AAD length in bits.
    pub aadLen: usize,
    /// Tag length in bits.
    pub tagLen: usize,
    pub tests: Vec<GcmPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct GcmResult {
    pub tcId: usize,

    /// Encryption only.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ct: Option<Vec<u8>>,

    /// Encryption only.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag: Option<Vec<u8>>,

    /// Successful decryption only.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pt: Option<Vec<u8>>,

    /// Decryption only; `false` if the tag does not verify.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub testPassed: Option<bool>,
}

pub type ResultGcmTestGroup = TestGroupResults<GcmResult>;

impl_test_case!(GcmPrompt, GcmResult);
impl_test_group!(GcmPromptTestGroup => GcmPrompt);
