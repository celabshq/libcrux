/// Wycheproof test vectors for ML-DSA sign (with seed) and verify.
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

use hacspec_ml_dsa::{
    keygen_internal, pk_size, sig_size, sign_internal, verify_internal, MlDsaParams, ML_DSA_44,
    ML_DSA_44_C_TILDE_LEN, ML_DSA_44_PK_SIZE, ML_DSA_44_SIG_SIZE, ML_DSA_44_SK_SIZE,
    ML_DSA_44_W1_SIZE, ML_DSA_65, ML_DSA_65_C_TILDE_LEN, ML_DSA_65_PK_SIZE, ML_DSA_65_SIG_SIZE,
    ML_DSA_65_SK_SIZE, ML_DSA_65_W1_SIZE, ML_DSA_87, ML_DSA_87_C_TILDE_LEN, ML_DSA_87_PK_SIZE,
    ML_DSA_87_SIG_SIZE, ML_DSA_87_SK_SIZE, ML_DSA_87_W1_SIZE,
};

// ======================================================================
// Serde types for sign-with-seed tests
// ======================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignSeedFile {
    test_groups: Vec<SignSeedGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignSeedGroup {
    #[serde(with = "hex::serde")]
    private_seed: [u8; 32],
    tests: Vec<SignTest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignTest {
    tc_id: i64,
    comment: String,
    #[serde(with = "hex::serde")]
    msg: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    ctx: Vec<u8>,
    #[serde(with = "hex::serde")]
    sig: Vec<u8>,
    result: String,
    #[serde(default)]
    flags: Vec<String>,
}

// ======================================================================
// Serde types for verify tests
// ======================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VerifyFile {
    test_groups: Vec<VerifyGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VerifyGroup {
    #[serde(with = "hex::serde")]
    public_key: Vec<u8>,
    tests: Vec<VerifyTest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VerifyTest {
    tc_id: i64,
    comment: String,
    #[serde(with = "hex::serde")]
    msg: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    ctx: Vec<u8>,
    #[serde(with = "hex::serde")]
    sig: Vec<u8>,
    result: String,
    #[serde(default)]
    flags: Vec<String>,
}

// ======================================================================
// Helpers
// ======================================================================

/// Format M' = IntegerToBytes(0,1) || IntegerToBytes(|ctx|,1) || ctx || M
fn format_m_prime(msg: &[u8], ctx: &[u8]) -> Vec<u8> {
    let mut m_prime = vec![0x00u8];
    m_prime.push(ctx.len() as u8);
    m_prime.extend_from_slice(ctx);
    m_prime.extend_from_slice(msg);
    m_prime
}

fn wycheproof_path(filename: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/testing/kats/wycheproof")
        .join(filename)
}

// ======================================================================
// Sign with seed tests
// ======================================================================

fn run_sign_seed_tests<
    const K: usize,
    const L: usize,
    const PK_SIZE: usize,
    const SK_SIZE: usize,
    const SIG_SIZE: usize,
    const W1_BYTES: usize,
    const C_TILDE_LEN: usize,
>(
    params: &MlDsaParams,
    filename: &str,
) {
    let file = File::open(wycheproof_path(filename))
        .unwrap_or_else(|e| panic!("Could not open {}: {}", filename, e));
    let data: SignSeedFile =
        serde_json::from_reader(BufReader::new(file)).expect("Could not deserialize");

    let signing_randomness = [0u8; 32];

    for group in &data.test_groups {
        let (pk, sk) = keygen_internal::<K, L, PK_SIZE, SK_SIZE>(&group.private_seed, params);

        for test in &group.tests {
            if test.ctx.len() > 255 {
                assert_eq!(
                    test.result, "invalid",
                    "tc_id={}: long ctx should be invalid",
                    test.tc_id
                );
                continue;
            }

            let m_prime = format_m_prime(&test.msg, &test.ctx);
            let sigma = sign_internal::<K, L, SIG_SIZE, W1_BYTES, C_TILDE_LEN>(
                &sk,
                &m_prime,
                &signing_randomness,
                params,
            );

            if test.result == "valid" {
                let sigma = sigma.expect("signing should succeed for valid test");
                assert_eq!(
                    sigma.as_slice(),
                    test.sig.as_slice(),
                    "tc_id={}: signature mismatch ({})",
                    test.tc_id,
                    test.comment
                );
                verify_internal::<K, L, C_TILDE_LEN, W1_BYTES>(&pk, &m_prime, &sigma, params)
                    .unwrap_or_else(|e| {
                        panic!(
                            "tc_id={}: verify failed for valid signature: {e:?}",
                            test.tc_id
                        )
                    });
            }
        }
    }
}

#[test]
fn wycheproof_sign_seed_44() {
    run_sign_seed_tests::<
        4,
        4,
        ML_DSA_44_PK_SIZE,
        ML_DSA_44_SK_SIZE,
        ML_DSA_44_SIG_SIZE,
        ML_DSA_44_W1_SIZE,
        ML_DSA_44_C_TILDE_LEN,
    >(&ML_DSA_44, "mldsa_44_sign_seed_test.json");
}

#[test]
fn wycheproof_sign_seed_65() {
    run_sign_seed_tests::<
        6,
        5,
        ML_DSA_65_PK_SIZE,
        ML_DSA_65_SK_SIZE,
        ML_DSA_65_SIG_SIZE,
        ML_DSA_65_W1_SIZE,
        ML_DSA_65_C_TILDE_LEN,
    >(&ML_DSA_65, "mldsa_65_sign_seed_test.json");
}

#[test]
fn wycheproof_sign_seed_87() {
    run_sign_seed_tests::<
        8,
        7,
        ML_DSA_87_PK_SIZE,
        ML_DSA_87_SK_SIZE,
        ML_DSA_87_SIG_SIZE,
        ML_DSA_87_W1_SIZE,
        ML_DSA_87_C_TILDE_LEN,
    >(&ML_DSA_87, "mldsa_87_sign_seed_test.json");
}

// ======================================================================
// Verify tests
// ======================================================================

fn run_verify_tests<
    const K: usize,
    const L: usize,
    const C_TILDE_LEN: usize,
    const W1_BYTES: usize,
>(
    params: &MlDsaParams,
    filename: &str,
) {
    let file = File::open(wycheproof_path(filename))
        .unwrap_or_else(|e| panic!("Could not open {}: {}", filename, e));
    let data: VerifyFile =
        serde_json::from_reader(BufReader::new(file)).expect("Could not deserialize");

    for group in &data.test_groups {
        let pk = &group.public_key;

        if pk.len() != pk_size(params) {
            for test in &group.tests {
                assert_eq!(
                    test.result, "invalid",
                    "tc_id={}: wrong pk size should be invalid",
                    test.tc_id
                );
            }
            continue;
        }

        for test in &group.tests {
            if test.sig.len() != sig_size(params) {
                assert_eq!(
                    test.result, "invalid",
                    "tc_id={}: wrong sig size should be invalid",
                    test.tc_id
                );
                continue;
            }

            if test.ctx.len() > 255 {
                assert_eq!(
                    test.result, "invalid",
                    "tc_id={}: long ctx should be invalid",
                    test.tc_id
                );
                continue;
            }

            let m_prime = format_m_prime(&test.msg, &test.ctx);
            let valid = verify_internal::<K, L, C_TILDE_LEN, W1_BYTES>(
                pk, &m_prime, &test.sig, params,
            )
            .is_ok();

            if test.result == "valid" {
                assert!(
                    valid,
                    "tc_id={}: expected valid but verify failed ({})",
                    test.tc_id, test.comment
                );
            } else {
                assert!(
                    !valid,
                    "tc_id={}: expected invalid but verify passed ({})",
                    test.tc_id, test.comment
                );
            }
        }
    }
}

#[test]
fn wycheproof_verify_44() {
    run_verify_tests::<4, 4, ML_DSA_44_C_TILDE_LEN, ML_DSA_44_W1_SIZE>(
        &ML_DSA_44,
        "mldsa_44_verify_test.json",
    );
}

#[test]
fn wycheproof_verify_65() {
    run_verify_tests::<6, 5, ML_DSA_65_C_TILDE_LEN, ML_DSA_65_W1_SIZE>(
        &ML_DSA_65,
        "mldsa_65_verify_test.json",
    );
}

#[test]
fn wycheproof_verify_87() {
    run_verify_tests::<8, 7, ML_DSA_87_C_TILDE_LEN, ML_DSA_87_W1_SIZE>(
        &ML_DSA_87,
        "mldsa_87_verify_test.json",
    );
}
