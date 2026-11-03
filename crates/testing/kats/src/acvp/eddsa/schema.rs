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
    /// E.g. `"ED-25519"`.
    pub curve: String,
    pub tests: Vec<KeyGenPrompt>,
}

/// A key pair: the private key `d` and the encoded public key `q`.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub d: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub q: Vec<u8>,
}

pub type ResultKeyGenTestGroup = TestGroupResults<KeyGenResult>;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigGenPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub message: Vec<u8>,

    /// Only for pre-hash and Ed448; empty otherwise.
    #[serde(default, with = "hex::serde")]
    pub context: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigGenPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"` or `"BFT"` (bit flip test).
    pub testType: String,
    pub curve: String,
    pub preHash: bool,

    /// The private key of the IUT; internal projection only.
    #[serde(default, with = "hex::serde")]
    pub d: Vec<u8>,

    /// The public key of the IUT; internal projection only.
    #[serde(default, with = "hex::serde")]
    pub q: Vec<u8>,

    pub tests: Vec<SigGenPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub signature: Vec<u8>,
}

/// The results of a test group, with the public key `q` of the IUT for the group.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenResultTestGroup {
    pub tgId: usize,

    #[serde(with = "upper_hex")]
    pub q: Vec<u8>,

    pub tests: Vec<SigGenResult>,
}

pub type ResultSigGenTestGroup = SigGenResultTestGroup;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigVerPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub message: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub q: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub signature: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigVerPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub curve: String,
    pub preHash: bool,
    pub tests: Vec<SigVerPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigVerResult {
    pub tcId: usize,
    pub testPassed: bool,
}

pub type ResultSigVerTestGroup = TestGroupResults<SigVerResult>;

impl_test_case!(
    KeyGenPrompt,
    KeyGenResult,
    SigGenPrompt,
    SigGenResult,
    SigVerPrompt,
    SigVerResult,
);
impl_test_group!(
    KeyGenPromptTestGroup => KeyGenPrompt,
    SigGenPromptTestGroup => SigGenPrompt,
    SigGenResultTestGroup => SigGenResult,
    SigVerPromptTestGroup => SigVerPrompt,
);
