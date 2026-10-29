use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct EcbPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,

    /// Encryption only.
    #[serde(default, with = "hex::serde")]
    pub pt: Vec<u8>,

    /// Decryption only.
    #[serde(default, with = "hex::serde")]
    pub ct: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct EcbPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"` or `"MCT"`.
    pub testType: String,
    pub direction: String,
    /// Key length in bits.
    pub keyLen: usize,
    pub tests: Vec<EcbPrompt>,
}

/// One of the 100 outer iterations of the MCT.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct EcbMctResult {
    #[serde(with = "upper_hex")]
    pub key: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub pt: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub ct: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct EcbResult {
    pub tcId: usize,

    /// AFT only.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ct: Option<Vec<u8>>,

    /// MCT only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resultsArray: Vec<EcbMctResult>,
}

pub type ResultEcbTestGroup = TestGroupResults<EcbResult>;

impl_test_case!(EcbPrompt, EcbResult);
impl_test_group!(EcbPromptTestGroup => EcbPrompt);
