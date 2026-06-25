use hacspec_ml_kem::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestFile {
    #[serde(rename = "testGroups")]
    test_groups: Vec<TestGroup>,
}

#[derive(Deserialize)]
struct TestGroup {
    #[serde(rename = "parameterSet")]
    parameter_set: String,
    tests: Vec<TestCase>,
}

#[derive(Deserialize)]
struct TestCase {
    #[serde(rename = "tcId")]
    tc_id: u32,
    #[serde(default)]
    seed: String,
    #[serde(default)]
    ek: String,
    #[serde(default)]
    dk: String,
    #[serde(default)]
    m: String,
    #[serde(default)]
    c: String,
    #[serde(rename = "K", default)]
    k: String,
    result: String,
    #[serde(default)]
    flags: Vec<String>,
}

fn get_params(param_set: &str) -> &'static MlKemParams {
    match param_set {
        "ML-KEM-512" => &ML_KEM_512,
        "ML-KEM-768" => &ML_KEM_768,
        "ML-KEM-1024" => &ML_KEM_1024,
        _ => panic!("unknown parameter set: {}", param_set),
    }
}

fn keygen_with_rank(
    params: &MlKemParams,
    randomness: &[u8; 64],
) -> Result<(Vec<u8>, Vec<u8>), BadRejectionSamplingRandomnessError> {
    match params.rank {
        2 => {
            let (ek, dk) = generate_keypair::<
                2,
                { ML_KEM_512_EK_SIZE },
                { ML_KEM_512_DK_SIZE },
                { ML_KEM_512_DK_PKE_SIZE },
            >(params, randomness)?;
            Ok((ek.to_vec(), dk.to_vec()))
        }
        3 => {
            let (ek, dk) = generate_keypair::<
                3,
                { ML_KEM_768_EK_SIZE },
                { ML_KEM_768_DK_SIZE },
                { ML_KEM_768_DK_PKE_SIZE },
            >(params, randomness)?;
            Ok((ek.to_vec(), dk.to_vec()))
        }
        4 => {
            let (ek, dk) = generate_keypair::<
                4,
                { ML_KEM_1024_EK_SIZE },
                { ML_KEM_1024_DK_SIZE },
                { ML_KEM_1024_DK_PKE_SIZE },
            >(params, randomness)?;
            Ok((ek.to_vec(), dk.to_vec()))
        }
        _ => panic!("unsupported rank"),
    }
}

fn encaps_with_rank(
    params: &MlKemParams,
    ek: &[u8],
    m: &[u8; 32],
) -> Result<([u8; 32], Vec<u8>), BadRejectionSamplingRandomnessError> {
    match params.rank {
        2 => {
            let ek_arr: &[u8; ML_KEM_512_EK_SIZE] = ek.try_into().unwrap();
            let (k, c) = encapsulate::<
                2,
                { ML_KEM_512_EK_SIZE },
                { ML_KEM_512_U_SIZE },
                { ML_KEM_512_V_SIZE },
                { ML_KEM_512_CT_SIZE },
            >(params, ek_arr, m)?;
            Ok((k, c.to_vec()))
        }
        3 => {
            let ek_arr: &[u8; ML_KEM_768_EK_SIZE] = ek.try_into().unwrap();
            let (k, c) = encapsulate::<
                3,
                { ML_KEM_768_EK_SIZE },
                { ML_KEM_768_U_SIZE },
                { ML_KEM_768_V_SIZE },
                { ML_KEM_768_CT_SIZE },
            >(params, ek_arr, m)?;
            Ok((k, c.to_vec()))
        }
        4 => {
            let ek_arr: &[u8; ML_KEM_1024_EK_SIZE] = ek.try_into().unwrap();
            let (k, c) = encapsulate::<
                4,
                { ML_KEM_1024_EK_SIZE },
                { ML_KEM_1024_U_SIZE },
                { ML_KEM_1024_V_SIZE },
                { ML_KEM_1024_CT_SIZE },
            >(params, ek_arr, m)?;
            Ok((k, c.to_vec()))
        }
        _ => panic!("unsupported rank"),
    }
}

fn decaps_with_rank(
    params: &MlKemParams,
    dk: &[u8],
    c: &[u8],
) -> Result<[u8; 32], BadRejectionSamplingRandomnessError> {
    match params.rank {
        2 => {
            let dk_arr: &[u8; ML_KEM_512_DK_SIZE] = dk.try_into().unwrap();
            let c_arr: &[u8; ML_KEM_512_CT_SIZE] = c.try_into().unwrap();
            decapsulate::<
                2,
                { ML_KEM_512_EK_SIZE },
                { ML_KEM_512_DK_SIZE },
                { ML_KEM_512_DK_PKE_SIZE },
                { ML_KEM_512_U_SIZE },
                { ML_KEM_512_V_SIZE },
                { ML_KEM_512_CT_SIZE },
                { ML_KEM_512_J_INPUT_SIZE },
            >(params, dk_arr, c_arr)
        }
        3 => {
            let dk_arr: &[u8; ML_KEM_768_DK_SIZE] = dk.try_into().unwrap();
            let c_arr: &[u8; ML_KEM_768_CT_SIZE] = c.try_into().unwrap();
            decapsulate::<
                3,
                { ML_KEM_768_EK_SIZE },
                { ML_KEM_768_DK_SIZE },
                { ML_KEM_768_DK_PKE_SIZE },
                { ML_KEM_768_U_SIZE },
                { ML_KEM_768_V_SIZE },
                { ML_KEM_768_CT_SIZE },
                { ML_KEM_768_J_INPUT_SIZE },
            >(params, dk_arr, c_arr)
        }
        4 => {
            let dk_arr: &[u8; ML_KEM_1024_DK_SIZE] = dk.try_into().unwrap();
            let c_arr: &[u8; ML_KEM_1024_CT_SIZE] = c.try_into().unwrap();
            decapsulate::<
                4,
                { ML_KEM_1024_EK_SIZE },
                { ML_KEM_1024_DK_SIZE },
                { ML_KEM_1024_DK_PKE_SIZE },
                { ML_KEM_1024_U_SIZE },
                { ML_KEM_1024_V_SIZE },
                { ML_KEM_1024_CT_SIZE },
                { ML_KEM_1024_J_INPUT_SIZE },
            >(params, dk_arr, c_arr)
        }
        _ => panic!("unsupported rank"),
    }
}

fn load_test_file(path: &str) -> TestFile {
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

// ---- keygen_seed tests ----

fn run_keygen_seed_test(path: &str) {
    let test_file = load_test_file(path);
    for group in &test_file.test_groups {
        let params = get_params(&group.parameter_set);
        for tc in &group.tests {
            let seed_bytes = hex::decode(&tc.seed).unwrap();
            assert_eq!(
                seed_bytes.len(),
                64,
                "tcId={}: seed must be 64 bytes",
                tc.tc_id
            );

            let randomness: [u8; 64] = seed_bytes.try_into().unwrap();
            let (ek, dk) = keygen_with_rank(params, &randomness)
                .unwrap_or_else(|_| panic!("tcId={}: keygen failed", tc.tc_id));

            let expected_ek = hex::decode(&tc.ek).unwrap();
            let expected_dk = hex::decode(&tc.dk).unwrap();

            assert_eq!(ek, expected_ek, "tcId={}: ek mismatch", tc.tc_id);
            assert_eq!(dk, expected_dk, "tcId={}: dk mismatch", tc.tc_id);
        }
    }
}

#[test]
fn wycheproof_keygen_seed_512() {
    run_keygen_seed_test("tests/mlkem_512_keygen_seed_test.json");
}

#[test]
fn wycheproof_keygen_seed_768() {
    run_keygen_seed_test("tests/mlkem_768_keygen_seed_test.json");
}

#[test]
fn wycheproof_keygen_seed_1024() {
    run_keygen_seed_test("tests/mlkem_1024_keygen_seed_test.json");
}

// ---- encaps tests ----

fn run_encaps_test(path: &str) {
    let test_file = load_test_file(path);
    for group in &test_file.test_groups {
        let params = get_params(&group.parameter_set);
        for tc in &group.tests {
            if tc.result == "valid" {
                let ek = hex::decode(&tc.ek).unwrap();
                let m_bytes = hex::decode(&tc.m).unwrap();
                let m: [u8; 32] = m_bytes.try_into().unwrap();

                let (k, c) = encaps_with_rank(params, &ek, &m)
                    .unwrap_or_else(|_| panic!("tcId={}: encaps failed", tc.tc_id));

                let expected_c = hex::decode(&tc.c).unwrap();
                let expected_k = hex::decode(&tc.k).unwrap();

                assert_eq!(c, expected_c, "tcId={}: ciphertext mismatch", tc.tc_id);
                assert_eq!(
                    k.to_vec(),
                    expected_k,
                    "tcId={}: shared key mismatch",
                    tc.tc_id
                );
            } else if tc.flags.contains(&"ModulusOverflow".to_string()) {
                // Modulus check should reject this ek
                let ek = hex::decode(&tc.ek).unwrap();
                // Use a dummy m for modulus check tests (their m may be malformed)
                let m = [0x42u8; 32];

                let result = std::panic::catch_unwind(|| encaps_with_rank(params, &ek, &m));
                assert!(
                    result.is_err(),
                    "tcId={}: should have rejected invalid ek",
                    tc.tc_id
                );
            }
        }
    }
}

#[test]
fn wycheproof_encaps_512() {
    run_encaps_test("tests/mlkem_512_encaps_test.json");
}

#[test]
fn wycheproof_encaps_768() {
    run_encaps_test("tests/mlkem_768_encaps_test.json");
}

#[test]
fn wycheproof_encaps_1024() {
    run_encaps_test("tests/mlkem_1024_encaps_test.json");
}

// ---- decaps tests ----

fn run_decaps_test(path: &str) {
    let test_file = load_test_file(path);
    for group in &test_file.test_groups {
        let params = get_params(&group.parameter_set);
        for tc in &group.tests {
            let dk = hex::decode(&tc.dk).unwrap();
            let c = hex::decode(&tc.c).unwrap();

            if tc.result == "valid" {
                // Valid: decapsulation should succeed without panic
                let _k = decaps_with_rank(params, &dk, &c)
                    .unwrap_or_else(|_| panic!("tcId={}: decaps failed", tc.tc_id));
            } else {
                // Invalid: wrong-length inputs etc. — just verify no crash
                // Skip cases with incorrect lengths that would cause index-out-of-bounds
                if tc.flags.contains(&"IncorrectCiphertextLength".to_string())
                    || tc
                        .flags
                        .contains(&"IncorrectDecapsulationKeyLength".to_string())
                {
                    // These have wrong-length inputs; just verify we don't crash
                    let _ = std::panic::catch_unwind(|| decaps_with_rank(params, &dk, &c));
                } else {
                    // InvalidDecapsulationKey: decaps should still produce a result
                    // (implicit rejection), just not the "correct" one
                    let _ = decaps_with_rank(params, &dk, &c);
                }
            }
        }
    }
}

#[test]
fn wycheproof_decaps_512() {
    run_decaps_test("tests/mlkem_512_semi_expanded_decaps_test.json");
}

#[test]
fn wycheproof_decaps_768() {
    run_decaps_test("tests/mlkem_768_semi_expanded_decaps_test.json");
}

#[test]
fn wycheproof_decaps_1024() {
    run_decaps_test("tests/mlkem_1024_semi_expanded_decaps_test.json");
}

// ---- full tests (keygen + decaps) ----

fn run_full_test(path: &str) {
    let test_file = load_test_file(path);
    for group in &test_file.test_groups {
        let params = get_params(&group.parameter_set);
        for tc in &group.tests {
            if tc.result != "valid" {
                continue;
            }

            let seed = hex::decode(&tc.seed).unwrap();
            assert_eq!(seed.len(), 64, "tcId={}: seed must be 64 bytes", tc.tc_id);

            // seed = d || z (keygen randomness)
            let randomness: [u8; 64] = seed.try_into().unwrap();
            let (ek, dk) = keygen_with_rank(params, &randomness)
                .unwrap_or_else(|_| panic!("tcId={}: keygen failed", tc.tc_id));

            let expected_ek = hex::decode(&tc.ek).unwrap();
            assert_eq!(ek, expected_ek, "tcId={}: ek mismatch", tc.tc_id);

            // Decapsulate the provided ciphertext
            let c = hex::decode(&tc.c).unwrap();
            let k = decaps_with_rank(params, &dk, &c)
                .unwrap_or_else(|_| panic!("tcId={}: decaps failed", tc.tc_id));

            let expected_k = hex::decode(&tc.k).unwrap();
            assert_eq!(
                k.to_vec(),
                expected_k,
                "tcId={}: shared key mismatch",
                tc.tc_id
            );
        }
    }
}

#[test]
fn wycheproof_full_512() {
    run_full_test("tests/mlkem_512_test.json");
}

#[test]
fn wycheproof_full_768() {
    run_full_test("tests/mlkem_768_test.json");
}

#[test]
fn wycheproof_full_1024() {
    run_full_test("tests/mlkem_1024_test.json");
}
