use hacspec_ml_kem::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct NistKat {
    key_generation_seed: String,
    sha3_256_hash_of_public_key: String,
    sha3_256_hash_of_secret_key: String,
    encapsulation_seed: String,
    sha3_256_hash_of_ciphertext: String,
    shared_secret: String,
}

fn keygen_with_rank(params: &MlKemParams, randomness: &[u8; 64]) -> (Vec<u8>, Vec<u8>) {
    match params.rank {
        2 => {
            let (ek, dk) = generate_keypair::<
                2,
                { ML_KEM_512_EK_SIZE },
                { ML_KEM_512_DK_SIZE },
                { ML_KEM_512_DK_PKE_SIZE },
            >(params, randomness)
            .unwrap();
            (ek.to_vec(), dk.to_vec())
        }
        3 => {
            let (ek, dk) = generate_keypair::<
                3,
                { ML_KEM_768_EK_SIZE },
                { ML_KEM_768_DK_SIZE },
                { ML_KEM_768_DK_PKE_SIZE },
            >(params, randomness)
            .unwrap();
            (ek.to_vec(), dk.to_vec())
        }
        4 => {
            let (ek, dk) = generate_keypair::<
                4,
                { ML_KEM_1024_EK_SIZE },
                { ML_KEM_1024_DK_SIZE },
                { ML_KEM_1024_DK_PKE_SIZE },
            >(params, randomness)
            .unwrap();
            (ek.to_vec(), dk.to_vec())
        }
        _ => panic!("unsupported rank"),
    }
}

fn encaps_with_rank(params: &MlKemParams, ek: &[u8], m: &[u8; 32]) -> ([u8; 32], Vec<u8>) {
    match params.rank {
        2 => {
            let ek_arr: &[u8; ML_KEM_512_EK_SIZE] = ek.try_into().unwrap();
            let (k, c) = encapsulate::<
                2,
                { ML_KEM_512_EK_SIZE },
                { ML_KEM_512_U_SIZE },
                { ML_KEM_512_V_SIZE },
                { ML_KEM_512_CT_SIZE },
            >(params, ek_arr, m)
            .unwrap();
            (k, c.to_vec())
        }
        3 => {
            let ek_arr: &[u8; ML_KEM_768_EK_SIZE] = ek.try_into().unwrap();
            let (k, c) = encapsulate::<
                3,
                { ML_KEM_768_EK_SIZE },
                { ML_KEM_768_U_SIZE },
                { ML_KEM_768_V_SIZE },
                { ML_KEM_768_CT_SIZE },
            >(params, ek_arr, m)
            .unwrap();
            (k, c.to_vec())
        }
        4 => {
            let ek_arr: &[u8; ML_KEM_1024_EK_SIZE] = ek.try_into().unwrap();
            let (k, c) = encapsulate::<
                4,
                { ML_KEM_1024_EK_SIZE },
                { ML_KEM_1024_U_SIZE },
                { ML_KEM_1024_V_SIZE },
                { ML_KEM_1024_CT_SIZE },
            >(params, ek_arr, m)
            .unwrap();
            (k, c.to_vec())
        }
        _ => panic!("unsupported rank"),
    }
}

fn decaps_with_rank(params: &MlKemParams, dk: &[u8], c: &[u8]) -> [u8; 32] {
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
            .unwrap()
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
            .unwrap()
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
            .unwrap()
        }
        _ => panic!("unsupported rank"),
    }
}

fn sha3_256(data: &[u8]) -> [u8; 32] {
    hacspec_sha3::sha3_256(data)
}

fn run_nist_kats(path: &str, params: &MlKemParams) {
    let data = std::fs::read_to_string(path).unwrap();
    let kats: Vec<NistKat> = serde_json::from_str(&data).unwrap();

    for (i, kat) in kats.iter().enumerate() {
        let seed: [u8; 64] = hex::decode(&kat.key_generation_seed)
            .unwrap()
            .try_into()
            .unwrap();

        // ML-KEM.KeyGen(d || z)
        let (ek, dk) = keygen_with_rank(params, &seed);

        // Verify public key hash
        let ek_hash = sha3_256(&ek);
        let expected_ek_hash: [u8; 32] = hex::decode(&kat.sha3_256_hash_of_public_key)
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(
            ek_hash, expected_ek_hash,
            "kat {}: public key hash mismatch",
            i
        );

        // Verify secret key hash
        let dk_hash = sha3_256(&dk);
        let expected_dk_hash: [u8; 32] = hex::decode(&kat.sha3_256_hash_of_secret_key)
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(
            dk_hash, expected_dk_hash,
            "kat {}: secret key hash mismatch",
            i
        );

        // ML-KEM.Encaps(ek, m)
        let m: [u8; 32] = hex::decode(&kat.encapsulation_seed)
            .unwrap()
            .try_into()
            .unwrap();
        let (shared_secret, ciphertext) = encaps_with_rank(params, &ek, &m);

        // Verify ciphertext hash
        let ct_hash = sha3_256(&ciphertext);
        let expected_ct_hash: [u8; 32] = hex::decode(&kat.sha3_256_hash_of_ciphertext)
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(
            ct_hash, expected_ct_hash,
            "kat {}: ciphertext hash mismatch",
            i
        );

        // Verify shared secret
        let expected_ss: [u8; 32] = hex::decode(&kat.shared_secret).unwrap().try_into().unwrap();
        assert_eq!(
            shared_secret, expected_ss,
            "kat {}: shared secret mismatch",
            i
        );

        // Verify decapsulation round-trip
        let decaps_ss = decaps_with_rank(params, &dk, &ciphertext);
        assert_eq!(
            decaps_ss, shared_secret,
            "kat {}: decaps shared secret mismatch",
            i
        );
    }
}

#[test]
fn nist_kats_mlkem_512() {
    run_nist_kats(
        "../../libcrux-ml-kem/tests/kats/nistkats_mlkem_512.json",
        &ML_KEM_512,
    );
}

#[test]
fn nist_kats_mlkem_768() {
    run_nist_kats(
        "../../libcrux-ml-kem/tests/kats/nistkats_mlkem_768.json",
        &ML_KEM_768,
    );
}

#[test]
fn nist_kats_mlkem_1024() {
    run_nist_kats(
        "../../libcrux-ml-kem/tests/kats/nistkats_mlkem_1024.json",
        &ML_KEM_1024,
    );
}
