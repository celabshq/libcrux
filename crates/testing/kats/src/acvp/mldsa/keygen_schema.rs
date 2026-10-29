use serde::{Deserialize, Serialize};

use super::super::schema_common::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KeyGenPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub seed: [u8; 32],
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
    pub pk: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub sk: Vec<u8>,
}

pub type ResultKeyGenTestGroup = TestGroupResults<KeyGenResult>;

impl_test_case!(KeyGenPrompt, KeyGenResult);
impl_test_group!(KeyGenPromptTestGroup => KeyGenPrompt);
