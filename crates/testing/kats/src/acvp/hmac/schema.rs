use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct HmacPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,
    /// Length of `key` in bits.
    pub keyLen: usize,

    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,
    /// Length of `msg` in bits.
    pub msgLen: usize,

    /// Length of the MAC in bits.
    pub macLen: usize,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct HmacPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub tests: Vec<HmacPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct HmacResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub mac: Vec<u8>,
}

pub type ResultHmacTestGroup = TestGroupResults<HmacResult>;

impl_test_case!(HmacPrompt, HmacResult);
impl_test_group!(HmacPromptTestGroup => HmacPrompt);
