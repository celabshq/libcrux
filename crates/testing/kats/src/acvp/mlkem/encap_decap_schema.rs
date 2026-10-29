use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct EncapPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub ek: Vec<u8>,

    #[serde(with = "hex::serde")]
    pub m: [u8; 32],
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DecapPrompt {
    pub tcId: usize,

    #[serde(with = "hex::serde")]
    pub c: Vec<u8>,

    #[serde(flatten)]
    pub key: DecapKey,
}

/// The decapsulation key, either in expanded form or as the key generation seed,
/// depending on the `keyFormat` of the test group, see [`DecapKey::key_format`].
#[derive(Deserialize)]
#[serde(untagged)]
pub enum DecapKey {
    Expanded {
        #[serde(with = "hex::serde")]
        dk: Vec<u8>,
    },
    Seed {
        #[serde(with = "hex::serde")]
        d: [u8; 32],
        #[serde(with = "hex::serde")]
        z: [u8; 32],
    },
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncapKeyCheckPrompt {
    pub tcId: usize,
    #[serde(with = "hex::serde")]
    pub ek: Vec<u8>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecapKeyCheckPrompt {
    pub tcId: usize,
    #[serde(with = "hex::serde")]
    pub dk: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct EncapDecapPromptTestGroup {
    pub tgId: usize,
    pub testType: String,
    pub parameterSet: String,
    /// `"expanded"` or `"seed"` for decapsulation groups since `FIPS203-tr1`.
    pub keyFormat: Option<String>,
    #[serde(flatten)]
    pub tests: EncapDecapTestPrompts,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
#[serde(tag = "function")]
pub enum EncapDecapTestPrompts {
    #[serde(rename(deserialize = "encapsulation"))]
    EncapTests { tests: Vec<EncapPrompt> },
    #[serde(rename(deserialize = "decapsulation"))]
    DecapTests { tests: Vec<DecapPrompt> },
    #[serde(rename(deserialize = "encapsulationKeyCheck"))]
    EncapKeyCheck { tests: Vec<EncapKeyCheckPrompt> },
    #[serde(rename(deserialize = "decapsulationKeyCheck"))]
    DecapKeyCheck { tests: Vec<DecapKeyCheckPrompt> },
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
#[allow(non_snake_case)]
pub enum EncapDecapResult {
    EncapResult {
        tcId: usize,
        #[serde(with = "upper_hex")]
        c: Vec<u8>,
        #[serde(with = "upper_hex")]
        k: Vec<u8>,
    },
    DecapResult {
        tcId: usize,
        #[serde(with = "upper_hex")]
        k: Vec<u8>,
    },
    KeyCheckResult {
        tcId: usize,
        testPassed: bool,
    },
}

impl TestResult for EncapDecapResult {
    fn tc_id(&self) -> usize {
        match self {
            Self::EncapResult { tcId, .. } => *tcId,
            Self::DecapResult { tcId, .. } => *tcId,
            Self::KeyCheckResult { tcId, .. } => *tcId,
        }
    }
}

pub type ResultEncapDecapTestGroup = TestGroupResults<EncapDecapResult>;

impl DecapKey {
    /// The `keyFormat` of this key, `"expanded"` or `"seed"`.
    pub fn key_format(&self) -> &'static str {
        match self {
            Self::Expanded { .. } => "expanded",
            Self::Seed { .. } => "seed",
        }
    }
}

impl EncapDecapTestPrompts {
    /// The `function` of the test group.
    pub fn function(&self) -> &'static str {
        match self {
            Self::EncapTests { .. } => "encapsulation",
            Self::DecapTests { .. } => "decapsulation",
            Self::EncapKeyCheck { .. } => "encapsulationKeyCheck",
            Self::DecapKeyCheck { .. } => "decapsulationKeyCheck",
        }
    }
}

impl_test_case!(
    EncapPrompt,
    DecapPrompt,
    EncapKeyCheckPrompt,
    DecapKeyCheckPrompt
);

impl TestIds for EncapDecapPromptTestGroup {
    fn test_ids(&self) -> Vec<(usize, usize)> {
        let tc_ids: Vec<usize> = match &self.tests {
            EncapDecapTestPrompts::EncapTests { tests } => tests.iter().map(|t| t.tcId).collect(),
            EncapDecapTestPrompts::DecapTests { tests } => tests.iter().map(|t| t.tcId).collect(),
            EncapDecapTestPrompts::EncapKeyCheck { tests } => {
                tests.iter().map(|t| t.tcId).collect()
            }
            EncapDecapTestPrompts::DecapKeyCheck { tests } => {
                tests.iter().map(|t| t.tcId).collect()
            }
        };
        tc_ids.into_iter().map(|tc_id| (self.tgId, tc_id)).collect()
    }
}
