mod util;
use libcrux_ecdsa::{
    p256::{self, PublicKey},
    DigestAlgorithm,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use util::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct P256TestVector {
    algorithm: String,
    generatorVersion: String,
    numberOfTests: usize,
    notes: Option<Value>, // text notes (might not be present), keys correspond to flags
    header: Vec<Value>,   // not used
    testGroups: Vec<TestGroup>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct TestGroup {
    key: Key,
    keyDer: String,
    keyPem: String,
    sha: String,
    r#type: String,
    tests: Vec<Test>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct Key {
    curve: String,
    r#type: String,
    keySize: usize,
    uncompressed: String,
    wx: String,
    wy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct Test {
    tcId: usize,
    comment: String,
    msg: String,
    sig: String,
    result: String,
    flags: Vec<String>,
}

impl ReadFromFile for P256TestVector {}

fn make_fixed_length(b: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let b_len = if b.len() >= 32 { 32 } else { b.len() };
    for i in 0..b_len {
        out[31 - i] = b[b.len() - 1 - i];
    }
    out
}

// A very simple ASN1 parser for ecdsa signatures.
fn decode_signature(sig: &[u8]) -> p256::Signature {
    let mut index = 0;
    let (seq, seq_len) = (sig[index], sig[index + 1] as usize);
    assert_eq!(0x30, seq);
    assert_eq!(seq_len, sig.len() - 2);
    index += 2;

    let (x_int, x_int_len) = (sig[index], sig[index + 1] as usize);
    assert_eq!(0x02, x_int);
    assert!(index + x_int_len + 2 < sig.len());
    index += 2;
    let r = &sig[index..index + x_int_len];
    index += x_int_len;

    let (y_int, y_int_len) = (sig[index], sig[index + 1] as usize);
    assert_eq!(0x02, y_int);
    assert!(index + y_int_len + 2 == sig.len());
    index += 2;
    let s = &sig[index..index + y_int_len as usize];
    index += y_int_len;
    assert_eq!(sig.len(), index);

    p256::Signature::from_raw(make_fixed_length(r), make_fixed_length(s))
}

#[allow(non_snake_case)]
#[test]
fn test_wycheproof() {
    let tests: P256TestVector = P256TestVector::from_file("tests/ecdsa_secp256r1_sha256_test.json");
    // TODO: add SHA512 tests

    assert_eq!(tests.algorithm, "ECDSA");

    let num_tests = tests.numberOfTests;
    let mut tests_run = 0;
    let mut tests_skipped = 0;

    for testGroup in tests.testGroups.iter() {
        assert_eq!(testGroup.key.curve, "secp256r1");
        assert_eq!(testGroup.key.r#type, "EcPublicKey");
        assert_eq!(testGroup.r#type, "EcdsaVerify");

        assert_eq!(testGroup.sha, "SHA-256");

        let pk = hex_str_to_bytes(&testGroup.key.uncompressed);
        let pk = PublicKey::try_from(pk.as_slice()).unwrap();

        for test in testGroup.tests.iter() {
            println!("Test {:?}: {:?}", test.tcId, test.comment);

            let valid = test.result.eq("valid") || test.result.eq("acceptable");
            let hash = DigestAlgorithm::Sha256;

            // Skip invalid for now
            if !valid {
                tests_skipped += 1;
                continue;
            }

            let msg = hex_str_to_bytes(&test.msg);
            let sig = hex_str_to_bytes(&test.sig);

            // The signature is ASN.1 encoded.
            let signature = decode_signature(&sig);

            match p256::verify(hash, &msg, &signature, &pk) {
                Ok(_) => {
                    assert!(valid);
                }
                Err(e) => {
                    println!("Error case ({:?}", e);
                    assert!(!valid);
                }
            }

            tests_run += 1;
        }
    }
    // Check that we ran all tests.
    println!(
        "Ran {} out of {} tests and skipped {}.",
        tests_run, num_tests, tests_skipped
    );
    assert_eq!(num_tests - tests_skipped, tests_run);
}
