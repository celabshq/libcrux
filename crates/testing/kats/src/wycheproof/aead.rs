use serde::{Deserialize, Serialize};

use super::schema_common::TestResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSet {
    pub number_of_tests: usize,
    pub test_groups: Vec<TestGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestGroup {
    pub iv_size: usize,
    pub key_size: usize,
    pub tag_size: usize,
    pub tests: Vec<Test>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    pub tc_id: usize,
    pub flags: Vec<String>,
    pub comment: String,
    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,
    #[serde(rename = "iv", with = "hex::serde")]
    pub nonce: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub aad: Vec<u8>,
    #[serde(rename = "msg", with = "hex::serde")]
    pub pt: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub ct: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub tag: Vec<u8>,
    pub result: TestResult,
}

impl TestSet {
    pub fn load_chacha20_poly1305() -> Self {
        let data = include_str!("../../wycheproof/chacha20_poly1305_test.json");
        serde_json::from_str(data).expect("could not deserialize chacha20_poly1305 KAT file")
    }

    pub fn load_xchacha20_poly1305() -> Self {
        let data = include_str!("../../wycheproof/xchacha20_poly1305_test.json");
        serde_json::from_str(data).expect("could not deserialize xchacha20_poly1305 KAT file")
    }
}
