use super::super::schema_common::*;
use serde::{Deserialize, Deserializer};

/// Deserialize an optional hex string.
fn opt_hex<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error> {
    Option::<String>::deserialize(deserializer)?
        .map(|s| hex::decode(s).map_err(serde::de::Error::custom))
        .transpose()
}

/// Deserialize an optional list of hex strings.
fn opt_hex_vec<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<Vec<u8>>>, D::Error> {
    Option::<Vec<String>>::deserialize(deserializer)?
        .map(|v| {
            v.into_iter()
                .map(|s| hex::decode(s).map_err(serde::de::Error::custom))
                .collect()
        })
        .transpose()
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TwoStepConfiguration {
    pub kdfType: String,
    /// Length of the derived keying material in bits.
    pub l: usize,
    /// Length of the salt in bits.
    pub saltLen: usize,
    pub saltMethod: String,
    pub fixedInfoPattern: String,
    pub fixedInfoEncoding: String,
    pub kdfMode: String,
    pub macMode: String,
    pub counterLocation: String,
    /// Length of the counter in bits.
    pub counterLen: usize,
    /// Length of the IV in bits.
    pub ivLen: usize,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TwoStepMultiExpansionConfiguration {
    pub kdfType: String,
    /// Length of the derived keying material in bits.
    pub l: usize,
    /// Length of the salt in bits.
    pub saltLen: usize,
    pub saltMethod: String,
    pub kdfMode: String,
    pub macMode: String,
    pub counterLocation: String,
    /// Length of the counter in bits.
    pub counterLen: usize,
    /// Length of the IV in bits.
    pub ivLen: usize,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KdfParameter {
    pub kdfType: String,

    #[serde(with = "hex::serde")]
    pub salt: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub z: Vec<u8>,

    /// Length of the derived keying material in bits.
    pub l: usize,

    /// Omitted in the JSON when empty.
    #[serde(default, with = "hex::serde")]
    pub iv: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PartyInfo {
    #[serde(with = "hex::serde")]
    pub partyId: Vec<u8>,

    #[serde(default, deserialize_with = "opt_hex")]
    pub ephemeralData: Option<Vec<u8>>,
}

impl PartyInfo {
    /// The `uPartyInfo` or `vPartyInfo` fixed info piece, i.e. `partyId || ephemeralData`.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = self.partyId.clone();
        out.extend_from_slice(self.ephemeralData.as_deref().unwrap_or_default());
        out
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct IterationParameter {
    /// Length of the derived keying material in bits.
    pub l: usize,

    #[serde(with = "hex::serde")]
    pub fixedInfo: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KdfMultiExpansionParameter {
    pub kdfType: String,
    pub kdfMode: String,
    pub macMode: String,
    pub counterLocation: String,
    pub counterLen: usize,

    #[serde(with = "hex::serde")]
    pub salt: Vec<u8>,

    /// Omitted in the JSON when empty.
    #[serde(default, with = "hex::serde")]
    pub iv: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub z: Vec<u8>,

    pub iterationParameters: Vec<IterationParameter>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SingleExpansionPrompt {
    pub tcId: usize,
    pub kdfParameter: KdfParameter,
    pub fixedInfoPartyU: PartyInfo,
    pub fixedInfoPartyV: PartyInfo,

    /// Only present for `VAL` tests.
    #[serde(default, deserialize_with = "opt_hex")]
    pub dkm: Option<Vec<u8>>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct MultiExpansionPrompt {
    pub tcId: usize,
    pub kdfMultiExpansionParameter: KdfMultiExpansionParameter,

    /// Only present for `VAL` tests.
    #[serde(default, deserialize_with = "opt_hex_vec")]
    pub dkms: Option<Vec<Vec<u8>>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TwoStepPrompt {
    Single(SingleExpansionPrompt),
    MultiExpansion(MultiExpansionPrompt),
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TwoStepPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    /// Length of the shared secret `z` in bits.
    pub zLength: usize,
    pub usesHybridSharedSecret: bool,
    pub multiExpansion: bool,
    /// Present iff `multiExpansion` is `false`.
    pub kdfConfiguration: Option<TwoStepConfiguration>,
    /// Present iff `multiExpansion` is `true`.
    pub kdfMultiExpansionConfiguration: Option<TwoStepMultiExpansionConfiguration>,
    pub tests: Vec<TwoStepPrompt>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TwoStepResult {
    pub tcId: usize,

    /// Expected result of single expansion `AFT` tests.
    #[serde(default, deserialize_with = "opt_hex")]
    pub dkm: Option<Vec<u8>>,

    /// Expected result of multi-expansion `AFT` tests.
    #[serde(default, deserialize_with = "opt_hex_vec")]
    pub dkms: Option<Vec<Vec<u8>>>,

    /// Expected result of `VAL` tests.
    pub testPassed: Option<bool>,
}

pub type ResultTwoStepTestGroup = TestGroupResults<TwoStepResult>;

impl TestResult for TwoStepResult {
    fn tc_id(&self) -> usize {
        self.tcId
    }
}
