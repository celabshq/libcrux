use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenPrompt {
    pub tcId: usize,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KeyGenPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    /// E.g. `"Curve25519"`.
    pub curve: String,
    pub tests: Vec<KeyGenPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub privateKey: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub publicKey: Vec<u8>,
}

pub type ResultKeyGenTestGroup = TestGroupResults<KeyGenResult>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyVerPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub publicKey: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KeyVerPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub curve: String,
    pub tests: Vec<KeyVerPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyVerResult {
    pub tcId: usize,
    pub testPassed: bool,
}

pub type ResultKeyVerTestGroup = TestGroupResults<KeyVerResult>;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SscPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub publicServer: Vec<u8>,

    /// Internal projection only.
    #[serde(default, with = "hex::serde")]
    pub privateIut: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SscPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub curve: String,
    pub tests: Vec<SscPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SscResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub publicIut: Vec<u8>,

    /// The shared secret.
    #[serde(with = "upper_hex")]
    pub z: Vec<u8>,
}

pub type ResultSscTestGroup = TestGroupResults<SscResult>;

impl_test_case!(
    KeyGenPrompt,
    KeyGenResult,
    KeyVerPrompt,
    KeyVerResult,
    SscPrompt,
    SscResult,
);
impl_test_group!(
    KeyGenPromptTestGroup => KeyGenPrompt,
    KeyVerPromptTestGroup => KeyVerPrompt,
    SscPromptTestGroup => SscPrompt,
);
