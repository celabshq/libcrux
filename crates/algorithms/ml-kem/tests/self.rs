use libcrux_ml_kem::{MlKemCiphertext, MlKemPrivateKey};
use libcrux_sha3::shake256;
use rand::{rng, rngs::SysRng, TryRng};

const SHARED_SECRET_SIZE: usize = 32;

fn random_array<const L: usize>() -> [u8; L] {
    let mut rng = SysRng;
    let mut seed = [0; L];
    rng.try_fill_bytes(&mut seed).unwrap();
    seed
}

macro_rules! impl_consistency {
    ($name:ident, $key_gen:expr, $encaps:expr, $decaps:expr) => {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[test]
        fn $name() {
            let randomness = random_array();
            let key_pair = $key_gen(randomness);
            let randomness = random_array();
            let (ciphertext, shared_secret) = $encaps(key_pair.public_key(), randomness);
            let shared_secret_decapsulated = $decaps(key_pair.private_key(), &ciphertext);
            assert_eq!(
                shared_secret, shared_secret_decapsulated,
                "lhs: shared_secret, rhs: shared_secret_decapsulated"
            );

            // If the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }
    };
}

macro_rules! impl_consistency_unpacked {
    ($name:ident, $modp:path) => {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[test]
        fn $name() {
            use $modp as p;

            let randomness = random_array();

            // Generate unpacked key
            let key_pair_unpacked = p::unpacked::generate_key_pair(randomness);

            // Generate regular key
            let key_pair = p::generate_key_pair(randomness);

            // Ensure the two keys are the same
            let serialized_public_key =
                p::unpacked::key_pair_serialized_public_key(&key_pair_unpacked);
            assert_eq!(
                key_pair.public_key().as_slice(),
                serialized_public_key.as_slice()
            );
            let mut re_unpacked_public_key = Default::default();
            p::unpacked::unpacked_public_key(key_pair.public_key(), &mut re_unpacked_public_key);
            let mut serialized_public_key = Default::default();
            p::unpacked::serialized_public_key(&re_unpacked_public_key, &mut serialized_public_key);
            assert_eq!(
                serialized_public_key.as_slice(),
                key_pair.public_key().as_slice()
            );
            let mut serialized_private_key = Default::default();
            p::unpacked::key_pair_serialized_private_key_mut(
                &key_pair_unpacked,
                &mut serialized_private_key,
            );
            assert_eq!(
                serialized_private_key.as_slice(),
                key_pair.private_key().as_slice()
            );

            // Unpacked key from the serialized private key
            let mut new_kp = Default::default();
            p::unpacked::key_pair_from_private_mut(&serialized_private_key, &mut new_kp);

            let randomness = random_array();
            let (ciphertext, shared_secret) = p::encapsulate(key_pair.public_key(), randomness);
            let (ciphertext_unpacked, shared_secret_unpacked) =
                p::unpacked::encapsulate(&key_pair_unpacked.public_key, randomness);
            assert_eq!(
                shared_secret, shared_secret_unpacked,
                "lhs: shared_secret, rhs: shared_secret_unpacked"
            );
            assert_eq!(
                ciphertext.as_slice(),
                ciphertext_unpacked.as_slice(),
                "lhs: ciphertext, rhs: ciphertext_unpacked"
            );

            // Check with re-assembled new_kp
            let (ciphertext_unpacked, shared_secret_unpacked) =
                p::unpacked::encapsulate(&new_kp.public_key, randomness);
            assert_eq!(
                shared_secret, shared_secret_unpacked,
                "lhs: shared_secret, rhs: shared_secret_unpacked"
            );
            assert_eq!(
                ciphertext.as_slice(),
                ciphertext_unpacked.as_slice(),
                "lhs: ciphertext, rhs: ciphertext_unpacked"
            );

            let shared_secret_decapsulated =
                p::unpacked::decapsulate(&key_pair_unpacked, &ciphertext);
            let shared_secret = p::decapsulate(key_pair.private_key(), &ciphertext);
            assert_eq!(
                shared_secret_unpacked, shared_secret_decapsulated,
                "lhs: shared_secret_unpacked, rhs: shared_secret_decapsulated"
            );
            assert_eq!(
                shared_secret, shared_secret_decapsulated,
                "lhs: shared_secret, rhs: shared_secret_decapsulated"
            );

            // Check with re-assembled new_kp
            let shared_secret_decapsulated = p::unpacked::decapsulate(&new_kp, &ciphertext);
            assert_eq!(
                shared_secret_unpacked, shared_secret_decapsulated,
                "lhs: shared_secret_unpacked, rhs: shared_secret_decapsulated"
            );

            // If the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }
    };
}

#[cfg(feature = "incremental")]
macro_rules! impl_consistency_incremental {
    ($name:ident, $modp:path) => {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg(feature = "alloc")]
        #[test]
        fn $name() {
            use $modp::{incremental::*, portable::unpacked};

            let key_gen_randomness = random_array();

            // Generate key pair.
            // Alloc (platform dependent keys)
            let key_pair = alloc::generate_key_pair(key_gen_randomness);
            // Serialized keys
            let mut key_pair_bytes = [0u8; key_pair_len()];
            generate_key_pair(key_gen_randomness, &mut key_pair_bytes).unwrap();
            // Compressed, serialized keys
            let key_pair_bytes_compressed = KeyPairCompressedBytes::from_seed(key_gen_randomness);

            // Get pk1 and pk2 to send out.
            let mut pk1_bytes = [0u8; 64];
            key_pair.pk1_bytes(&mut pk1_bytes).unwrap();

            let mut pk2_bytes = [0u8; pk2_len()];
            key_pair.pk2_bytes(&mut pk2_bytes);

            // Check that the keys are the same
            assert_eq!(pk1_bytes, pk1(&key_pair_bytes));
            assert_eq!(pk2_bytes, pk2(&key_pair_bytes));
            // Same for the compressed key pair
            assert_eq!(&pk1_bytes, key_pair_bytes_compressed.pk1());
            assert_eq!(&pk2_bytes, key_pair_bytes_compressed.pk2());

            // The other party encapsulates to pk1 ...
            let encaps_randomness = random_array();
            let (ct1, ct2, shared_secret) = {
                let pk1 = PublicKey1::try_from(&pk1_bytes as &[u8]).unwrap();
                let (ct1, state, dyn_ss) = alloc::encapsulate1(&pk1, encaps_randomness);
                debug_assert_eq!(ct1.value.len(), Ciphertext1::len());

                // encaps1 with serialized state
                let mut serialized_state = [0u8; encaps_state_len()];
                let mut shared_secret_serialized = [0u8; SHARED_SECRET_SIZE];
                let ct12 = encapsulate1(
                    &pk1_bytes,
                    encaps_randomness,
                    &mut serialized_state,
                    &mut shared_secret_serialized,
                )
                .unwrap();
                assert_eq!(ct1.value, ct12.value);

                // Check the public key for consistency.
                assert!(validate_pk(&pk1, &pk2_bytes).is_ok());

                // ... and then to pk2.
                // pk2 is passed in as bytes because the deserializaiton is runtime
                // platform dependent.
                let ct2 = alloc::encapsulate2(state.as_ref(), &pk2_bytes).unwrap();
                debug_assert_eq!(ct2.value.len(), Ciphertext2::len());

                // encaps2 with serialized state
                let ct22 = encapsulate2(&serialized_state, &pk2_bytes);
                assert_eq!(ct2.value, ct22.value);

                assert_eq!(dyn_ss, shared_secret_serialized);
                (ct1, ct2, dyn_ss)
            };

            // The initiator decapsulates the two ciphertexts.
            // Alloc (platform dependent keys)
            let shared_secret_decaps = alloc::decapsulate(key_pair.as_ref(), &ct1, &ct2);
            // Serialized key
            let shared_secret_decaps2 =
                decapsulate_incremental_key(&key_pair_bytes, &ct1, &ct2).unwrap();
            // Compressed, serialized key
            let sk = key_pair_bytes_compressed.sk();
            let shared_secret_decaps3 = decapsulate_compressed_key(sk, &ct1, &ct2);

            // Check the shared secret.
            assert_eq!(shared_secret_decaps, shared_secret);
            assert_eq!(shared_secret_decaps2, shared_secret);
            assert_eq!(shared_secret_decaps3, shared_secret);

            // Compute comparison shared secrets and ciphertexts with the other APIs
            let key_pair = unpacked::generate_key_pair(key_gen_randomness);
            let (ciphertext_unpacked, shared_secret_unpacked) =
                unpacked::encapsulate(&key_pair.public_key, encaps_randomness);

            // Check c1 and c2
            let mut combined_ct = vec![0u8; ciphertext_unpacked.as_ref().len()];
            combined_ct[0..ct1.value.len()].copy_from_slice(&ct1.value);
            combined_ct[ct1.value.len()..].copy_from_slice(&ct2.value);
            assert_eq!(
                ciphertext_unpacked.as_ref(),
                &combined_ct,
                "Invalid ciphertexts"
            );

            // Check API consistency
            assert_eq!(
                shared_secret_unpacked, shared_secret,
                "Invalid encaps shared secret"
            );
        }
    };
}

#[cfg(all(feature = "mlkem512", feature = "incremental"))]
impl_consistency_incremental!(consistency_incremental_512, libcrux_ml_kem::mlkem512);

#[cfg(all(feature = "mlkem768", feature = "incremental"))]
impl_consistency_incremental!(consistency_incremental_768, libcrux_ml_kem::mlkem768);

#[cfg(all(feature = "mlkem1024", feature = "incremental"))]
impl_consistency_incremental!(consistency_incremental_1024, libcrux_ml_kem::mlkem1024);

fn modify_ciphertext<const LEN: usize>(ciphertext: MlKemCiphertext<LEN>) -> MlKemCiphertext<LEN> {
    let mut raw_ciphertext = [0u8; LEN];
    raw_ciphertext.copy_from_slice(ciphertext.as_ref());

    let mut random_u32: usize = rng().try_next_u32().unwrap().try_into().unwrap();

    let mut random_byte: u8 = (random_u32 & 0xFF) as u8;
    if random_byte == 0 {
        random_byte += 1;
    }
    random_u32 >>= 8;

    let position = random_u32 % MlKemCiphertext::<LEN>::len();
    raw_ciphertext[position] ^= random_byte;

    let ciphertext: [u8; LEN] = raw_ciphertext[0..MlKemCiphertext::<LEN>::len()]
        .try_into()
        .unwrap();

    ciphertext.into()
}

macro_rules! impl_modified_ciphertext {
    ($name:ident, $key_gen:expr, $encaps:expr, $decaps:expr) => {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[test]
        fn $name() {
            let randomness = random_array();
            let key_pair = $key_gen(randomness);
            let randomness = random_array();
            let (ciphertext, shared_secret) = $encaps(key_pair.public_key(), randomness);

            let ciphertext = modify_ciphertext(ciphertext);
            let shared_secret_decapsulated = $decaps(key_pair.private_key(), &ciphertext);

            assert_ne!(shared_secret, shared_secret_decapsulated);

            // if the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }
    };
}

fn modify_secret_key<const LEN: usize>(
    secret_key: &MlKemPrivateKey<LEN>,
    modify_implicit_rejection_value: bool,
) -> MlKemPrivateKey<LEN> {
    let mut raw_secret_key = [0u8; LEN];
    raw_secret_key.copy_from_slice(secret_key.as_slice());

    let mut random_u32: usize = rng().try_next_u32().unwrap().try_into().unwrap();

    let mut random_byte: u8 = (random_u32 & 0xFF) as u8;
    if random_byte == 0 {
        random_byte += 1;
    }
    random_u32 >>= 8;

    let position = if modify_implicit_rejection_value {
        (MlKemPrivateKey::<LEN>::len() - SHARED_SECRET_SIZE) + (random_u32 % SHARED_SECRET_SIZE)
    } else {
        random_u32 % (MlKemPrivateKey::<LEN>::len() - SHARED_SECRET_SIZE)
    };

    raw_secret_key[position] ^= random_byte;

    let secret_key: [u8; LEN] = raw_secret_key[0..MlKemPrivateKey::<LEN>::len()]
        .try_into()
        .unwrap();

    secret_key.into()
}

fn compute_implicit_rejection_shared_secret<const CLEN: usize, const LEN: usize>(
    ciphertext: MlKemCiphertext<CLEN>,
    secret_key: MlKemPrivateKey<LEN>,
) -> [u8; SHARED_SECRET_SIZE] {
    let mut to_hash = secret_key[MlKemPrivateKey::<LEN>::len() - SHARED_SECRET_SIZE..].to_vec();
    to_hash.extend_from_slice(ciphertext.as_ref());

    shake256(&to_hash)
}

macro_rules! impl_modified_secret_key {
    ($name:ident, $key_gen:expr, $encaps:expr, $decaps:expr) => {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[test]
        fn $name() {
            let randomness = random_array();
            let key_pair = $key_gen(randomness);
            let randomness = random_array();
            let (ciphertext, _) = $encaps(key_pair.public_key(), randomness);

            let secret_key = modify_secret_key(key_pair.private_key(), false);
            let shared_secret_decapsulated = $decaps(&secret_key, &ciphertext);

            assert_eq!(
                shared_secret_decapsulated,
                compute_implicit_rejection_shared_secret(ciphertext, secret_key)
            );

            // if the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }
    };
}

macro_rules! impl_modified_ciphertext_and_implicit_rejection_value {
    ($name:ident, $key_gen:expr, $encaps:expr, $decaps:expr) => {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[test]
        fn $name() {
            let randomness = random_array();
            let key_pair = $key_gen(randomness);
            let randomness = random_array();
            let (ciphertext, _) = $encaps(key_pair.public_key(), randomness);

            let ciphertext = modify_ciphertext(ciphertext);
            let shared_secret_decapsulated = $decaps(key_pair.private_key(), &ciphertext);

            let secret_key = modify_secret_key(key_pair.private_key(), true);
            let shared_secret_decapsulated_ms = $decaps(&secret_key, &ciphertext);

            assert_ne!(shared_secret_decapsulated, shared_secret_decapsulated_ms);

            assert_eq!(
                shared_secret_decapsulated_ms,
                compute_implicit_rejection_shared_secret(ciphertext, secret_key)
            );

            // if the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }
    };
}

#[cfg(feature = "mlkem512")]
impl_consistency!(
    consistency_512,
    libcrux_ml_kem::mlkem512::generate_key_pair,
    libcrux_ml_kem::mlkem512::encapsulate,
    libcrux_ml_kem::mlkem512::decapsulate
);
#[cfg(feature = "mlkem768")]
impl_consistency!(
    consistency_768,
    libcrux_ml_kem::mlkem768::generate_key_pair,
    libcrux_ml_kem::mlkem768::encapsulate,
    libcrux_ml_kem::mlkem768::decapsulate
);
#[cfg(feature = "mlkem1024")]
impl_consistency!(
    consistency_1024,
    libcrux_ml_kem::mlkem1024::generate_key_pair,
    libcrux_ml_kem::mlkem1024::encapsulate,
    libcrux_ml_kem::mlkem1024::decapsulate
);

#[cfg(all(feature = "mlkem512"))]
impl_consistency_unpacked!(
    consistency_unpacked_512_portable,
    libcrux_ml_kem::mlkem512::portable
);

#[cfg(all(feature = "mlkem512", feature = "simd128",))]
impl_consistency_unpacked!(
    consistency_unpacked_512_neon,
    libcrux_ml_kem::mlkem512::neon
);

#[cfg(all(feature = "mlkem512", feature = "simd256",))]
impl_consistency_unpacked!(
    consistency_unpacked_512_avx2,
    libcrux_ml_kem::mlkem512::avx2
);

#[cfg(all(feature = "mlkem1024"))]
impl_consistency_unpacked!(
    consistency_unpacked_1024_portable,
    libcrux_ml_kem::mlkem1024::portable
);

#[cfg(all(feature = "mlkem1024", feature = "simd128",))]
impl_consistency_unpacked!(
    consistency_unpacked_1024_neon,
    libcrux_ml_kem::mlkem1024::neon
);

#[cfg(all(feature = "mlkem1024", feature = "simd256",))]
impl_consistency_unpacked!(
    consistency_unpacked_1024_avx2,
    libcrux_ml_kem::mlkem1024::avx2
);

#[cfg(all(feature = "mlkem768",))]
impl_consistency_unpacked!(
    consistency_unpacked_768_portable,
    libcrux_ml_kem::mlkem768::portable
);

#[cfg(all(feature = "mlkem768", feature = "simd128",))]
impl_consistency_unpacked!(
    consistency_unpacked_768_neon,
    libcrux_ml_kem::mlkem768::neon
);

#[cfg(all(feature = "mlkem768", feature = "simd256",))]
impl_consistency_unpacked!(
    consistency_unpacked_768_avx2,
    libcrux_ml_kem::mlkem768::avx2
);

#[cfg(feature = "mlkem512")]
impl_modified_ciphertext!(
    modified_ciphertext_512,
    libcrux_ml_kem::mlkem512::generate_key_pair,
    libcrux_ml_kem::mlkem512::encapsulate,
    libcrux_ml_kem::mlkem512::decapsulate
);
#[cfg(feature = "mlkem768")]
impl_modified_ciphertext!(
    modified_ciphertext_768,
    libcrux_ml_kem::mlkem768::generate_key_pair,
    libcrux_ml_kem::mlkem768::encapsulate,
    libcrux_ml_kem::mlkem768::decapsulate
);
#[cfg(feature = "mlkem1024")]
impl_modified_ciphertext!(
    modified_ciphertext_1024,
    libcrux_ml_kem::mlkem1024::generate_key_pair,
    libcrux_ml_kem::mlkem1024::encapsulate,
    libcrux_ml_kem::mlkem1024::decapsulate
);
#[cfg(feature = "mlkem512")]
impl_modified_secret_key!(
    modified_secret_key_512,
    libcrux_ml_kem::mlkem512::generate_key_pair,
    libcrux_ml_kem::mlkem512::encapsulate,
    libcrux_ml_kem::mlkem512::decapsulate
);
#[cfg(feature = "mlkem768")]
impl_modified_secret_key!(
    modified_secret_key_768,
    libcrux_ml_kem::mlkem768::generate_key_pair,
    libcrux_ml_kem::mlkem768::encapsulate,
    libcrux_ml_kem::mlkem768::decapsulate
);
#[cfg(feature = "mlkem1024")]
impl_modified_secret_key!(
    modified_secret_key_1024,
    libcrux_ml_kem::mlkem1024::generate_key_pair,
    libcrux_ml_kem::mlkem1024::encapsulate,
    libcrux_ml_kem::mlkem1024::decapsulate
);

#[cfg(feature = "mlkem512")]
impl_modified_ciphertext_and_implicit_rejection_value!(
    modified_ciphertext_and_implicit_rejection_value_512,
    libcrux_ml_kem::mlkem512::generate_key_pair,
    libcrux_ml_kem::mlkem512::encapsulate,
    libcrux_ml_kem::mlkem512::decapsulate
);
#[cfg(feature = "mlkem768")]
impl_modified_ciphertext_and_implicit_rejection_value!(
    modified_ciphertext_and_implicit_rejection_value_768,
    libcrux_ml_kem::mlkem768::generate_key_pair,
    libcrux_ml_kem::mlkem768::encapsulate,
    libcrux_ml_kem::mlkem768::decapsulate
);
#[cfg(feature = "mlkem1024")]
impl_modified_ciphertext_and_implicit_rejection_value!(
    modified_ciphertext_and_implicit_rejection_value_1024,
    libcrux_ml_kem::mlkem1024::generate_key_pair,
    libcrux_ml_kem::mlkem1024::encapsulate,
    libcrux_ml_kem::mlkem1024::decapsulate
);
