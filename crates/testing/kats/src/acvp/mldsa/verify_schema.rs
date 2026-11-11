use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigVerPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub parameterSet: String,
    pub preHash: Option<String>,
    pub signatureInterface: String,
    #[serde(default)]
    pub externalMu: bool,
    pub tests: Vec<SigVerTest>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigVerTest {
    pub tcId: usize,
    #[serde(default, with = "hex::serde")]
    pub message: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub mu: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub signature: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub pk: Vec<u8>,
    #[serde(with = "hex::serde", default)]
    pub context: Vec<u8>,
    #[serde(default)]
    pub hashAlg: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigVerResult {
    pub tcId: usize,
    pub testPassed: bool,
}

pub type ResultSigVerTestGroup = TestGroupResults<SigVerResult>;
impl_test_case!(SigVerTest, SigVerResult);
impl_test_group!(SigVerPromptTestGroup => SigVerTest);
