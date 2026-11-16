mod test_util;
use libcrux_ecdh::{self, key_gen, Algorithm, Error};
use rand::rngs::SysRng;
use rand_core::UnwrapErr;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use test_util::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn derive() {
    let _ = pretty_env_logger::try_init();

    let mut rng = UnwrapErr(SysRng);

    let (private_a, public_a) = key_gen(Algorithm::X25519, &mut rng).unwrap();
    let (private_b, public_b) = key_gen(Algorithm::X25519, &mut rng).unwrap();

    let shared_a = libcrux_ecdh::derive(Algorithm::X25519, &public_b, &private_a).unwrap();
    let shared_b = libcrux_ecdh::derive(Algorithm::X25519, &public_a, &private_b).unwrap();
    assert_eq!(shared_a, shared_b);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct X25519TestVector {
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
    curve: String,
    r#type: String,
    tests: Vec<Test>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct Test {
    tcId: usize,
    comment: String,
    public: String,
    private: String,
    shared: String,
    result: String,
    flags: Vec<String>,
}

impl ReadFromFile for X25519TestVector {}

#[allow(non_snake_case)]
#[test]
fn wycheproof() {
    let tests: X25519TestVector = X25519TestVector::from_file("tests/wycheproof/x25519_test.json");

    assert_eq!(tests.algorithm, "XDH");

    let num_tests = tests.numberOfTests;
    let mut tests_run = 0;

    for testGroup in tests.testGroups.iter() {
        assert_eq!(testGroup.curve, "curve25519");
        assert_eq!(testGroup.r#type, "XdhComp");
        for test in testGroup.tests.iter() {
            let valid = test.result.eq("valid") || test.result.eq("acceptable");
            // Mark some test cases as invalid because HACL doesn't allow them
            let valid = match test.comment.as_str() {
                "public key = 0" => false,
                "public key = 1" => false,
                "public key with low order" => false,
                "public key = \
                 57896044618658097711785492504343953926634992332820282019728792003956564819949" => {
                    false
                }
                "public key = \
                 57896044618658097711785492504343953926634992332820282019728792003956564819950" => {
                    false
                }
                "public key = \
                 57896044618658097711785492504343953926634992332820282019728792003956564819968" => {
                    false
                }
                "public key = \
                 57896044618658097711785492504343953926634992332820282019728792003956564819969" => {
                    false
                }
                "special case public key" => {
                    (test.flags.contains(&"Twist".to_owned()) && test.tcId != 154)
                        || test.tcId == 120
                        || test.tcId == 122
                        || test.tcId == 123
                        || test.tcId == 125
                        || test.tcId == 128
                        || test.tcId == 131
                        || test.tcId == 132
                        || test.tcId == 134
                        || test.tcId == 135
                        || test.tcId == 137
                        || test.tcId == 138
                        || test.tcId == 141
                        || test.tcId == 142
                        || test.tcId == 143
                        || test.tcId == 144
                        || test.tcId == 145
                        || test.tcId == 146
                        || test.tcId == 149
                        || test.tcId == 150
                        || test.tcId == 151
                        || test.tcId == 152
                        || test.tcId == 153
                }
                "D = 0 in multiplication by 2" => false,
                _ => valid,
            };
            println!("Test {:?}: {:?}", test.tcId, test.comment);
            let public = hex_str_to_bytes(&test.public);
            let private = hex_str_to_bytes(&test.private);
            let shared = hex_str_to_bytes(&test.shared);

            match libcrux_ecdh::derive(Algorithm::X25519, &public, &private) {
                Ok(r) => {
                    assert!(valid);
                    assert_eq!(r[..], shared[..]);
                }
                Err(e) => {
                    assert!(!valid);
                    assert!(matches!(e, Error::Custom(_)));
                }
            }
            tests_run += 1;
        }
    }
    // Check that we ran all tests.
    println!("Ran {} out of {} tests.", tests_run, num_tests);
    assert_eq!(num_tests, tests_run);
}
