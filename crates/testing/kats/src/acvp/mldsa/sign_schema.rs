use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Randomness(#[serde(with = "hex::serde")] pub [u8; 32]);

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SigGenPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub parameterSet: String,
    pub deterministic: bool,
    pub signatureInterface: String,
    pub preHash: Option<String>,
    #[serde(default)]
    pub externalMu: bool,
    /// `"expanded"` or `"seed"` since `FIPS204-tr1`.
    pub keyFormat: Option<String>,
    pub tests: Vec<SigGenTest>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenTest {
    pub tcId: usize,
    /// The expanded signing key; empty if the key is given as `seed`.
    #[serde(default, with = "hex::serde")]
    pub sk: Vec<u8>,
    /// The key generation seed, for `keyFormat` `"seed"`.
    #[serde(default, with = "hex::serde")]
    pub seed: Vec<u8>,
    #[serde(default)]
    pub rnd: Option<Randomness>,
    #[serde(default, with = "hex::serde")]
    pub message: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub mu: Vec<u8>,
    #[serde(with = "hex::serde", default)]
    pub context: Vec<u8>,
    #[serde(default)]
    pub hashAlg: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SigGenResult {
    pub tcId: usize,
    #[serde(with = "upper_hex")]
    pub signature: Vec<u8>,
}

pub type ResultSigGenTestGroup = TestGroupResults<SigGenResult>;

impl_test_case!(SigGenTest, SigGenResult);
impl_test_group!(SigGenPromptTestGroup => SigGenTest);
