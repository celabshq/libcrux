use serde::{Deserialize, Serialize};

use super::schema_common::TestResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSet {
    pub number_of_tests: usize,
    pub test_groups: Vec<TestGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestGroup {
    pub tests: Vec<Test>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    pub tc_id: usize,
    pub flags: Vec<String>,
    #[serde(rename = "public", with = "hex::serde")]
    pub public_key: Vec<u8>,
    #[serde(rename = "private", with = "hex::serde")]
    pub private_key: Vec<u8>,
    #[serde(rename = "shared", with = "hex::serde")]
    pub shared_secret: Vec<u8>,
    pub result: TestResult,
}

impl TestSet {
    pub fn load_secp256r1_ecpoint() -> Self {
        let data = include_str!("../../wycheproof/ecdh_secp256r1_ecpoint_test.json");
        serde_json::from_str(data).expect("could not deserialize ecdh_secp256r1_ecpoint KAT file")
    }
}
