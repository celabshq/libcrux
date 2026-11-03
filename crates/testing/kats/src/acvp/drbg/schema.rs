use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

/// An operation on the DRBG after instantiation.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct OtherInput {
    /// `"reSeed"` or `"generate"`.
    pub intendedUse: String,

    #[serde(with = "hex::serde")]
    pub additionalInput: Vec<u8>,

    /// Empty for `generate` without prediction resistance.
    #[serde(with = "hex::serde")]
    pub entropyInput: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DrbgPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub entropyInput: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub nonce: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub persoString: Vec<u8>,

    pub otherInput: Vec<OtherInput>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DrbgPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    /// The hash function, e.g. `"SHA2-256"`.
    pub mode: String,
    pub derFunc: bool,
    pub reSeed: bool,
    pub predResistance: bool,
    /// Lengths in bits.
    pub entropyInputLen: usize,
    pub nonceLen: usize,
    pub persoStringLen: usize,
    pub additionalInputLen: usize,
    pub returnedBitsLen: usize,
    pub tests: Vec<DrbgPrompt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DrbgResult {
    pub tcId: usize,

    #[serde(with = "upper_hex")]
    pub returnedBits: Vec<u8>,
}

pub type ResultDrbgTestGroup = TestGroupResults<DrbgResult>;

impl_test_case!(DrbgPrompt, DrbgResult);
impl_test_group!(DrbgPromptTestGroup => DrbgPrompt);
