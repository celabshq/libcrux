//! The schema of the `KDA` `HKDF` vector sets.
//!
//! The test cases and results are those of the two-step KDA, see [`super::schema`].

use serde::Deserialize;

pub use super::schema::{
    IterationParameter, ResultTwoStepTestGroup as ResultHkdfTestGroup, SingleExpansionPrompt,
};
use crate::acvp::schema_common::upper_hex;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct HkdfConfiguration {
    pub kdfType: String,
    /// Length of the derived keying material in bits.
    pub l: usize,
    /// Length of the salt in bits.
    pub saltLen: usize,
    pub saltMethod: String,
    pub fixedInfoPattern: String,
    pub fixedInfoEncoding: String,
    /// The hash function of the HMAC, e.g. `"SHA2-256"`.
    pub hmacAlg: String,
}

/// A test group without multi-expansion tests.
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct HkdfPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"` or `"VAL"`.
    pub testType: String,
    /// Length of the shared secret `z` in bits.
    pub zLength: usize,
    pub usesHybridSharedSecret: bool,
    pub multiExpansion: bool,
    pub kdfConfiguration: HkdfConfiguration,
    pub tests: Vec<SingleExpansionPrompt>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct HkdfMultiExpansionConfiguration {
    pub kdfType: String,
    /// Length of the derived keying material in bits.
    pub l: usize,
    /// Length of the salt in bits.
    pub saltLen: usize,
    pub saltMethod: String,
    /// The hash function of the HMAC, e.g. `"SHA2-256"`.
    pub hmacAlg: String,
}

/// One extraction with `salt` from `z` (and `t`), then one expansion per iteration.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct HkdfMultiExpansionParameter {
    pub kdfType: String,
    pub hmacAlg: String,

    #[serde(with = "hex::serde")]
    pub salt: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub z: Vec<u8>,

    /// The auxiliary shared secret, appended to `z`; only with a hybrid shared secret.
    #[serde(default, with = "hex::serde")]
    pub t: Vec<u8>,

    pub iterationParameters: Vec<IterationParameter>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct HkdfMultiExpansionPrompt {
    pub tcId: usize,
    pub kdfMultiExpansionParameter: HkdfMultiExpansionParameter,

    /// Only present for `VAL` tests.
    #[serde(default, with = "upper_hex::option_vec")]
    pub dkms: Option<Vec<Vec<u8>>>,
}

/// A test group with multi-expansion tests.
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct HkdfMultiExpansionPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"` or `"VAL"`.
    pub testType: String,
    /// Length of the shared secret `z` in bits.
    pub zLength: usize,
    pub usesHybridSharedSecret: bool,
    pub multiExpansion: bool,
    pub kdfMultiExpansionConfiguration: HkdfMultiExpansionConfiguration,
    pub tests: Vec<HkdfMultiExpansionPrompt>,
}

/// A test group with or without multi-expansion tests.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum HkdfAnyPromptTestGroup {
    Single(HkdfPromptTestGroup),
    MultiExpansion(HkdfMultiExpansionPromptTestGroup),
}

impl_test_group!(HkdfPromptTestGroup => SingleExpansionPrompt);
impl_test_case!(HkdfMultiExpansionPrompt);
impl_test_group!(HkdfMultiExpansionPromptTestGroup => HkdfMultiExpansionPrompt);
