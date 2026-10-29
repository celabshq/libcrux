use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CcmPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub iv: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub aad: Vec<u8>,

    /// Encryption only.
    #[serde(default, with = "hex::serde")]
    pub pt: Vec<u8>,

    /// Decryption only: the ciphertext followed by the tag.
    #[serde(default, with = "hex::serde")]
    pub ct: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct CcmPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    /// `"encrypt"` or `"decrypt"`.
    pub direction: String,
    /// Key length in bits.
    pub keyLen: usize,
    /// IV (nonce) length in bits.
    pub ivLen: usize,
    /// Payload length in bits.
    pub payloadLen: usize,
    /// AAD length in bits.
    pub aadLen: usize,
    /// Tag length in bits.
    pub tagLen: usize,
    pub tests: Vec<CcmPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CcmResult {
    pub tcId: usize,

    /// Encryption only: the ciphertext followed by the tag.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ct: Option<Vec<u8>>,

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

pub type ResultCcmTestGroup = TestGroupResults<CcmResult>;

impl_test_case!(CcmPrompt, CcmResult);
impl_test_group!(CcmPromptTestGroup => CcmPrompt);
