use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KbkdfPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub keyIn: Vec<u8>,

    /// Not present in counter mode.
    #[serde(default, with = "hex::serde")]
    pub iv: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KbkdfPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub kdfMode: String,
    pub macMode: String,
    /// Length of the derived key in bits.
    pub keyOutLength: usize,
    /// Length of the counter in bits; not present without a counter.
    #[serde(default)]
    pub counterLength: usize,
    /// Not present without a counter.
    #[serde(default)]
    pub counterLocation: String,
    /// Not present in counter mode.
    #[serde(default)]
    pub zeroLengthIv: bool,
    pub tests: Vec<KbkdfPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KbkdfResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub fixedData: Vec<u8>,

    #[serde(with = "upper_hex")]
    pub keyOut: Vec<u8>,
}

pub type ResultKbkdfTestGroup = TestGroupResults<KbkdfResult>;

impl_test_case!(KbkdfPrompt, KbkdfResult);
impl_test_group!(KbkdfPromptTestGroup => KbkdfPrompt);
