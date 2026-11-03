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
    pub curve: String,
    pub secretGenerationMode: String,
    pub tests: Vec<KeyGenPrompt>,
}

/// A key pair `(d, Q)`. Integers are big-endian.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub d: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub qx: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub qy: Vec<u8>,
}

pub type ResultKeyGenTestGroup = TestGroupResults<KeyGenResult>;

/// A public key `Q` to validate. Coordinates are big-endian.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyVerPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub qx: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub qy: Vec<u8>,
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
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub message: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigGenPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub curve: String,
    /// The hash function, e.g. `"SHA2-256"`.
    pub hashAlg: String,
    /// Whether the message is the already hashed message.
    pub componentTest: bool,
    /// `"SP800-106"` if the message is to be randomized before hashing.
    pub conformance: Option<String>,
    pub tests: Vec<SigGenPrompt>,
}

/// A signature `(r, s)`. Integers are big-endian.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub r: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub s: Vec<u8>,
}

/// The results of a test group, with the public key `Q` of the IUT for the group.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenResultTestGroup {
    pub tgId: usize,

    #[serde(with = "upper_hex")]
    pub qx: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub qy: Vec<u8>,

    pub tests: Vec<SigGenResult>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigVerPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub message: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub qx: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub qy: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub r: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub s: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigVerPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub curve: String,
    /// The hash function, e.g. `"SHA2-256"`.
    pub hashAlg: String,
    pub tests: Vec<SigVerPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigVerResult {
    pub tcId: usize,
    pub testPassed: bool,
}

pub type ResultSigGenTestGroup = SigGenResultTestGroup;
pub type ResultSigVerTestGroup = TestGroupResults<SigVerResult>;

impl_test_case!(
    KeyGenPrompt,
    KeyGenResult,
    KeyVerPrompt,
    KeyVerResult,
    SigGenPrompt,
    SigGenResult,
    SigVerPrompt,
    SigVerResult,
);
impl_test_group!(
    KeyGenPromptTestGroup => KeyGenPrompt,
    KeyVerPromptTestGroup => KeyVerPrompt,
    SigGenPromptTestGroup => SigGenPrompt,
    SigGenResultTestGroup => SigGenResult,
    SigVerPromptTestGroup => SigVerPrompt,
);
