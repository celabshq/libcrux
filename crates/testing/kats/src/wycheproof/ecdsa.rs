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
    #[serde(rename = "publicKey")]
    pub key: EcdsaPublicKey,
    pub public_key_der: String,
    pub tests: Vec<Test>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EcdsaPublicKey {
    /// Uncompressed public key bytes (04 || X || Y)
    #[serde(rename = "uncompressed", with = "hex::serde")]
    pub key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    pub tc_id: usize,
    pub flags: Vec<String>,
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub sig: Vec<u8>,
    pub result: TestResult,
}

impl TestSet {
    pub fn load_secp256r1_sha256() -> Self {
        let data = include_str!("../../wycheproof/ecdsa_secp256r1_sha256_test.json");
        serde_json::from_str(data).expect("could not deserialize ecdsa_secp256r1_sha256 KAT file")
    }

    pub fn load_secp256r1_sha512() -> Self {
        let data = include_str!("../../wycheproof/ecdsa_secp256r1_sha512_test.json");
        serde_json::from_str(data).expect("could not deserialize ecdsa_secp256r1_sha512 KAT file")
    }
}
