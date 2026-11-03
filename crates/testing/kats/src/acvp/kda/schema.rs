use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

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

    /// The auxiliary shared secret, appended to `z`; only with a hybrid shared secret.
    #[serde(default, with = "hex::serde")]
    pub t: Vec<u8>,

    /// Length of the derived keying material in bits.
    pub l: usize,

    /// Omitted in the JSON when empty.
    #[serde(default, with = "hex::serde")]
    pub iv: Vec<u8>,

    /// Only present if the `fixedInfoPattern` contains `label`.
    #[serde(default, with = "hex::serde")]
    pub label: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PartyInfo {
    #[serde(with = "hex::serde")]
    pub partyId: Vec<u8>,

    #[serde(default, with = "upper_hex::option")]
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

    /// The auxiliary shared secret, appended to `z`; only with a hybrid shared secret.
    #[serde(default, with = "hex::serde")]
    pub t: Vec<u8>,

    pub iterationParameters: Vec<IterationParameter>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SingleExpansionPrompt {
    pub tcId: usize,
    pub kdfParameter: KdfParameter,
    pub fixedInfoPartyU: PartyInfo,
    pub fixedInfoPartyV: PartyInfo,

    /// Only present for `VAL` tests.
    #[serde(default, with = "upper_hex::option")]
    pub dkm: Option<Vec<u8>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct MultiExpansionPrompt {
    pub tcId: usize,
    pub kdfMultiExpansionParameter: KdfMultiExpansionParameter,

    /// Only present for `VAL` tests.
    #[serde(default, with = "upper_hex::option_vec")]
    pub dkms: Option<Vec<Vec<u8>>>,
}

impl SingleExpansionPrompt {
    /// The fixed info according to the registered `fixedInfoPattern`, with `l` as a 32-bit
    /// big-endian integer.
    pub fn fixed_info(&self, pattern: &str) -> Vec<u8> {
        let mut out = Vec::new();
        for piece in pattern.split("||") {
            match piece {
                "uPartyInfo" => out.extend(self.fixedInfoPartyU.encode()),
                "vPartyInfo" => out.extend(self.fixedInfoPartyV.encode()),
                "label" => out.extend(&self.kdfParameter.label),
                "l" => out.extend(u32::try_from(self.kdfParameter.l).unwrap().to_be_bytes()),
                literal => {
                    let hex = literal
                        .strip_prefix("literal[")
                        .and_then(|l| l.strip_suffix(']'))
                        .unwrap_or_else(|| panic!("unexpected fixedInfoPattern piece {literal}"));
                    out.extend(hex::decode(hex).unwrap());
                }
            }
        }
        out
    }
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

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct TwoStepResult {
    pub tcId: usize,

    /// Expected result of single expansion `AFT` tests.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dkm: Option<Vec<u8>>,

    /// Expected result of multi-expansion `AFT` tests.
    #[serde(
        default,
        with = "upper_hex::option_vec",
        skip_serializing_if = "Option::is_none"
    )]
    pub dkms: Option<Vec<Vec<u8>>>,

    /// Expected result of `VAL` tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub testPassed: Option<bool>,
}

pub type ResultTwoStepTestGroup = TestGroupResults<TwoStepResult>;

impl TestCase for TwoStepPrompt {
    fn tc_id(&self) -> usize {
        match self {
            Self::Single(test) => test.tcId,
            Self::MultiExpansion(test) => test.tcId,
        }
    }
}

impl_test_case!(SingleExpansionPrompt, MultiExpansionPrompt, TwoStepResult);
impl_test_group!(TwoStepPromptTestGroup => TwoStepPrompt);
