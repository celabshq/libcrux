use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub z: [u8; 32],

    #[serde(with = "hex::serde")]
    pub d: [u8; 32],
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KeyGenPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub parameterSet: String,
    pub tests: Vec<KeyGenPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub ek: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub dk: Vec<u8>,
}

pub type ResultKeyGenTestGroup = TestGroupResults<KeyGenResult>;

impl_test_case!(KeyGenPrompt, KeyGenResult);
impl_test_group!(KeyGenPromptTestGroup => KeyGenPrompt);
