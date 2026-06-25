/// NIST Known Answer Tests for ML-DSA.
///
/// Uses KAT files from libcrux-ml-dsa (same repo), comparing SHA3-256 hashes
/// of generated keys and signatures against expected values.
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

use hacspec_ml_dsa::{
    keygen_internal, sign_internal, verify_internal, MlDsaParams, ML_DSA_44, ML_DSA_44_C_TILDE_LEN,
    ML_DSA_44_PK_SIZE, ML_DSA_44_SIG_SIZE, ML_DSA_44_SK_SIZE, ML_DSA_44_W1_SIZE, ML_DSA_65,
    ML_DSA_65_C_TILDE_LEN, ML_DSA_65_PK_SIZE, ML_DSA_65_SIG_SIZE, ML_DSA_65_SK_SIZE,
    ML_DSA_65_W1_SIZE, ML_DSA_87, ML_DSA_87_C_TILDE_LEN, ML_DSA_87_PK_SIZE, ML_DSA_87_SIG_SIZE,
    ML_DSA_87_SK_SIZE, ML_DSA_87_W1_SIZE,
};

#[derive(Debug, Deserialize)]
struct MlDsaNISTKAT {
    #[serde(with = "hex::serde")]
    key_generation_seed: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_verification_key: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_signing_key: [u8; 32],

    message: String,

    #[serde(with = "hex::serde")]
    signing_randomness: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_signature: [u8; 32],
}

/// Format M' for non-pre-hashed signing with empty context.
/// FIPS 204, Algorithm 2, line 5: M' = IntegerToBytes(0,1) || IntegerToBytes(|ctx|,1) || ctx || M
fn format_m_prime(message: &[u8]) -> Vec<u8> {
    let mut m_prime = vec![0x00u8, 0x00u8]; // mode=0, ctx_len=0
    m_prime.extend_from_slice(message);
    m_prime
}

fn load_kats(parameter_set: usize) -> Vec<MlDsaNISTKAT> {
    let katfile_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../libcrux-ml-dsa/tests/kats")
        .join(format!("nistkats-{}.json", parameter_set));
    let katfile = File::open(&katfile_path)
        .unwrap_or_else(|e| panic!("Could not open KAT file {:?}: {}", katfile_path, e));
    let reader = BufReader::new(katfile);
    serde_json::from_reader(reader).expect("Could not deserialize KAT file.")
}

fn run_keygen_kats<const K: usize, const L: usize, const PK_SIZE: usize, const SK_SIZE: usize>(
    params: &MlDsaParams,
    parameter_set: usize,
) {
    let kats = load_kats(parameter_set);
    for (i, kat) in kats.iter().enumerate() {
        let (pk, sk) = keygen_internal::<K, L, PK_SIZE, SK_SIZE>(&kat.key_generation_seed, params);

        let pk_hash: [u8; 32] = hacspec_sha3::sha3_256(&pk);
        assert_eq!(
            pk_hash, kat.sha3_256_hash_of_verification_key,
            "KAT {i}: pk hash mismatch"
        );

        let sk_hash: [u8; 32] = hacspec_sha3::sha3_256(&sk);
        assert_eq!(
            sk_hash, kat.sha3_256_hash_of_signing_key,
            "KAT {i}: sk hash mismatch"
        );
    }
}

fn run_sign_verify_kats<
    const K: usize,
    const L: usize,
    const PK_SIZE: usize,
    const SK_SIZE: usize,
    const SIG_SIZE: usize,
    const W1_BYTES: usize,
    const C_TILDE_LEN: usize,
>(
    params: &MlDsaParams,
    parameter_set: usize,
) {
    let kats = load_kats(parameter_set);
    for (i, kat) in kats.iter().enumerate() {
        let (pk, sk) = keygen_internal::<K, L, PK_SIZE, SK_SIZE>(&kat.key_generation_seed, params);
        let message = hex::decode(&kat.message).expect("Hex-decoding message failed");
        let m_prime = format_m_prime(&message);

        let sigma = sign_internal::<K, L, SIG_SIZE, W1_BYTES, C_TILDE_LEN>(
            &sk,
            &m_prime,
            &kat.signing_randomness,
            params,
        )
        .expect("KAT {i}: signing failed");

        let sig_hash: [u8; 32] = hacspec_sha3::sha3_256(&sigma);
        assert_eq!(
            sig_hash, kat.sha3_256_hash_of_signature,
            "KAT {i}: signature hash mismatch"
        );

        verify_internal::<K, L, C_TILDE_LEN, W1_BYTES>(&pk, &m_prime, &sigma, params)
            .unwrap_or_else(|e| panic!("KAT {i}: verification failed: {e:?}"));
    }
}

#[test]
fn keygen_44() {
    run_keygen_kats::<4, 4, ML_DSA_44_PK_SIZE, ML_DSA_44_SK_SIZE>(&ML_DSA_44, 44);
}

#[test]
fn keygen_65() {
    run_keygen_kats::<6, 5, ML_DSA_65_PK_SIZE, ML_DSA_65_SK_SIZE>(&ML_DSA_65, 65);
}

#[test]
fn keygen_87() {
    run_keygen_kats::<8, 7, ML_DSA_87_PK_SIZE, ML_DSA_87_SK_SIZE>(&ML_DSA_87, 87);
}

#[test]
fn sign_verify_44() {
    run_sign_verify_kats::<
        4,
        4,
        ML_DSA_44_PK_SIZE,
        ML_DSA_44_SK_SIZE,
        ML_DSA_44_SIG_SIZE,
        ML_DSA_44_W1_SIZE,
        ML_DSA_44_C_TILDE_LEN,
    >(&ML_DSA_44, 44);
}

#[test]
fn sign_verify_65() {
    run_sign_verify_kats::<
        6,
        5,
        ML_DSA_65_PK_SIZE,
        ML_DSA_65_SK_SIZE,
        ML_DSA_65_SIG_SIZE,
        ML_DSA_65_W1_SIZE,
        ML_DSA_65_C_TILDE_LEN,
    >(&ML_DSA_65, 65);
}

#[test]
fn sign_verify_87() {
    run_sign_verify_kats::<
        8,
        7,
        ML_DSA_87_PK_SIZE,
        ML_DSA_87_SK_SIZE,
        ML_DSA_87_SIG_SIZE,
        ML_DSA_87_W1_SIZE,
        ML_DSA_87_C_TILDE_LEN,
    >(&ML_DSA_87, 87);
}
