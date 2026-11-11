use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KmacPrompt {
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

    /// The customization string, if `hexCustomization` is `false`.
    #[serde(default)]
    pub customization: String,
    /// The customization string, if `hexCustomization` is `true`.
    #[serde(default, with = "hex::serde")]
    pub customizationHex: Vec<u8>,

    /// The MAC to verify; MVT only.
    #[serde(default, with = "hex::serde")]
    pub mac: Vec<u8>,
}

impl KmacPrompt {
    /// The customization string as bytes, guessed from which of `customization` and
    /// `customizationHex` is non-empty.
    ///
    /// Prefer [`KmacPromptTestGroup::customization`], which uses the `hexCustomization` of the
    /// test group.
    pub fn customization_bytes(&self) -> &[u8] {
        if self.customizationHex.is_empty() {
            self.customization.as_bytes()
        } else {
            &self.customizationHex
        }
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KmacPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"` (MAC generation) or `"MVT"` (MAC verification).
    pub testType: String,
    pub xof: bool,
    pub hexCustomization: bool,
    pub tests: Vec<KmacPrompt>,
}

impl KmacPromptTestGroup {
    /// The customization string of `test` as bytes, according to `hexCustomization`.
    pub fn customization<'a>(&self, test: &'a KmacPrompt) -> &'a [u8] {
        if self.hexCustomization {
            assert!(test.customization.is_empty(), "tcId {}", test.tcId);
            &test.customizationHex
        } else {
            assert!(test.customizationHex.is_empty(), "tcId {}", test.tcId);
            test.customization.as_bytes()
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KmacResult {
    pub tcId: usize,

    /// AFT only.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac: Option<Vec<u8>>,

    /// MVT only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub testPassed: Option<bool>,
}

pub type ResultKmacTestGroup = TestGroupResults<KmacResult>;

impl_test_case!(KmacPrompt, KmacResult);
impl_test_group!(KmacPromptTestGroup => KmacPrompt);
