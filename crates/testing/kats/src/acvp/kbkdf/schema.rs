use super::super::schema_common::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KbkdfPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub keyIn: Vec<u8>,

    #[serde(with = "hex::serde")]
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
    /// Length of the counter in bits.
    pub counterLength: usize,
    pub counterLocation: String,
    pub zeroLengthIv: bool,
    pub tests: Vec<KbkdfPrompt>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KbkdfResult {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub fixedData: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub keyOut: Vec<u8>,
}

pub type ResultKbkdfTestGroup = TestGroupResults<KbkdfResult>;

impl TestResult for KbkdfResult {
    fn tc_id(&self) -> usize {
        self.tcId
    }
}
