use serde::Deserialize;
use serde_json;
use std::{fs::File, io::BufReader, path::Path};

use libcrux_sha3::*;

#[derive(Deserialize)]
struct MlKemNISTKAT {
    #[serde(with = "hex::serde")]
    key_generation_seed: [u8; 64],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_public_key: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_secret_key: [u8; 32],

    #[serde(with = "hex::serde")]
    encapsulation_seed: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_ciphertext: [u8; 32],

    #[serde(with = "hex::serde")]
    shared_secret: [u8; 32],
}

macro_rules! impl_nist_known_answer_tests {
    ($name:ident, $variant:literal, $parameter_set:literal, $module:path) => {
        #[test]
        fn $name() {
            use $module::*;

            let katfile_path = Path::new("tests")
                .join("kats")
            .join(format!("nistkats_{}_{}.json", $variant, $parameter_set));
            let katfile = File::open(katfile_path).expect("Could not open KAT file.");
            let reader = BufReader::new(katfile);

            let nist_kats: Vec<MlKemNISTKAT> =
                serde_json::from_reader(reader).expect("Could not deserialize KAT file.");

            for kat in nist_kats {
                let key_pair = generate_key_pair(kat.key_generation_seed);

                assert!(validate_public_key(key_pair.public_key()));

                #[cfg(not(feature = "kyber"))]
                {
                    let unpacked_keys = unpacked::generate_key_pair(kat.key_generation_seed);

                    let pk = unpacked::key_pair_serialized_public_key(&unpacked_keys);
                    let sk = unpacked::key_pair_serialized_private_key(&unpacked_keys);

                    let public_key_hash = sha256(pk.as_slice());
                    let secret_key_hash = sha256(sk.as_slice());

                    assert_eq!(public_key_hash, kat.sha3_256_hash_of_public_key, "lhs: computed public key hash, rhs: hash from kat");
                    assert_eq!(secret_key_hash, kat.sha3_256_hash_of_secret_key, "lhs: computed secret key hash, rhs: hash from kat");
                }

                let public_key_hash = sha256(key_pair.pk());
                eprintln!("pk hash: {}", hex::encode(public_key_hash));
                let secret_key_hash = sha256(key_pair.sk());

                assert_eq!(public_key_hash, kat.sha3_256_hash_of_public_key, "lhs: computed public key hash, rhs: hash from kat");
                assert_eq!(secret_key_hash, kat.sha3_256_hash_of_secret_key, "lhs: computed secret key hash, rhs: hash from kat");

                let (ciphertext, shared_secret) =
                encapsulate(key_pair.public_key(), kat.encapsulation_seed);
                let ciphertext_hash = sha256(ciphertext.as_ref());

                assert_eq!(ciphertext_hash, kat.sha3_256_hash_of_ciphertext, "lhs: computed ciphertext hash, rhs: hash from akt");
                assert_eq!(shared_secret.as_ref(), kat.shared_secret, "lhs: computed shared secret from encapsulate, rhs: shared secret from kat");


                assert!(validate_private_key(key_pair.private_key(), &ciphertext));

                let shared_secret_from_decapsulate =
                decapsulate(key_pair.private_key(), &ciphertext);
                assert_eq!(shared_secret_from_decapsulate, shared_secret.as_ref(), "lhs: shared secret computed via decapsulation, rhs: shared secret computed via encapsulation");
            }
        }
    };
}

#[cfg(all(feature = "mlkem512"))]
impl_nist_known_answer_tests!(
    mlkem512_nist_kats_portable,
    "mlkem",
    512,
    libcrux_ml_kem::mlkem512::portable
);

#[cfg(all(feature = "mlkem768"))]
impl_nist_known_answer_tests!(
    mlkem768_nist_kats_portable,
    "mlkem",
    768,
    libcrux_ml_kem::mlkem768::portable
);

#[cfg(all(feature = "mlkem1024"))]
impl_nist_known_answer_tests!(
    mlkem1024_nist_kats_portable,
    "mlkem",
    1024,
    libcrux_ml_kem::mlkem1024::portable
);

#[cfg(all(feature = "mlkem512", feature = "kyber"))]
impl_nist_known_answer_tests!(
    kyber512_nist_kats_portable,
    "kyber",
    512,
    libcrux_ml_kem::kyber512
);

#[cfg(all(feature = "mlkem768", feature = "kyber"))]
impl_nist_known_answer_tests!(
    kyber768_nist_kats_portable,
    "kyber",
    768,
    libcrux_ml_kem::kyber768
);

#[cfg(all(feature = "mlkem1024", feature = "kyber"))]
impl_nist_known_answer_tests!(
    kyber1024_nist_kats_portable,
    "kyber",
    1024,
    libcrux_ml_kem::kyber1024
);

// Testing multiplexing APIs

macro_rules! impl_kats {
    ($name:ident, $variant:literal, $parameter_set:literal, $module:path) => {
        #[test]
        fn $name() {
            use $module::*;

            let katfile_path = Path::new("tests")
                .join("kats")
            .join(format!("nistkats_{}_{}.json", $variant, $parameter_set));
            let katfile = File::open(katfile_path).expect("Could not open KAT file.");
            let reader = BufReader::new(katfile);

            let nist_kats: Vec<MlKemNISTKAT> =
                serde_json::from_reader(reader).expect("Could not deserialize KAT file.");

            for kat in nist_kats {
                let key_pair = generate_key_pair(kat.key_generation_seed);
                #[cfg(all(feature = "alloc", feature = "incremental"))]
                let incremental_key_pair = incremental::alloc::generate_key_pair(kat.key_generation_seed);

                assert!(validate_public_key(key_pair.public_key()));

                let public_key_hash = sha256(key_pair.pk());
                let secret_key_hash = sha256(key_pair.sk());

                assert_eq!(public_key_hash, kat.sha3_256_hash_of_public_key, "lhs: computed public key hash, rhs: hash from kat");
                assert_eq!(secret_key_hash, kat.sha3_256_hash_of_secret_key, "lhs: computed secret key hash, rhs: hash from kat");

                // Encapsulate
                let (ciphertext, shared_secret) =
                encapsulate(key_pair.public_key(), kat.encapsulation_seed);
                let ciphertext_hash = sha256(ciphertext.as_ref());

                assert_eq!(ciphertext_hash, kat.sha3_256_hash_of_ciphertext, "lhs: computed ciphertext hash, rhs: hash from akt");
                assert_eq!(shared_secret.as_ref(), kat.shared_secret, "lhs: computed shared secret from encapsulate, rhs: shared secret from kat");

                // Decapsulate
                assert!(validate_private_key(key_pair.private_key(), &ciphertext));

                let shared_secret_from_decapsulate =
                decapsulate(key_pair.private_key(), &ciphertext);
                assert_eq!(shared_secret_from_decapsulate, shared_secret.as_ref(), "lhs: shared secret computed via decapsulation, rhs: shared secret computed via encapsulation");

                // Incremental encapsulate
                #[cfg(all(feature = "alloc", feature = "incremental"))]
                {
                    let mut pk1_bytes = [0u8; 64];
                    incremental_key_pair.pk1_bytes(&mut pk1_bytes).unwrap();

                    let mut pk2_bytes = [0u8; incremental::pk2_len()];
                    incremental_key_pair.pk2_bytes(&mut pk2_bytes);

                    let (ct1, ct2, incremental_shared_secret) = {
                        let pk1 = incremental::PublicKey1::try_from(&pk1_bytes as &[u8]).unwrap();
                        let (ct1, state, ss) = incremental::alloc::encapsulate1(&pk1,  kat.encapsulation_seed);

                        assert!(incremental::validate_pk(&pk1, &pk2_bytes).is_ok());

                        // ... and then to pk2.
                        // pk2 is passed in as bytes because the deserializaiton is runtime
                        // platform dependent.
                        let ct2 = incremental::alloc::encapsulate2(state.as_ref(), &pk2_bytes).unwrap();

                        (ct1, ct2, ss)
                    };

                    let incremental_shared_secret_decaps = incremental::alloc::decapsulate(incremental_key_pair.as_ref(), &ct1, &ct2);
                    assert_eq!(incremental_shared_secret, shared_secret.as_ref());
                    assert_eq!(incremental_shared_secret_decaps, shared_secret.as_ref());
                }
            }
        }
    };
}

#[cfg(all(feature = "mlkem512"))]
impl_kats!(mlkem512_nist_kats, "mlkem", 512, libcrux_ml_kem::mlkem512);

#[cfg(all(feature = "mlkem768"))]
impl_kats!(mlkem768_nist_kat, "mlkem", 768, libcrux_ml_kem::mlkem768);

#[cfg(all(feature = "mlkem1024"))]
impl_kats!(
    mlkem1024_nist_kats,
    "mlkem",
    1024,
    libcrux_ml_kem::mlkem1024
);
