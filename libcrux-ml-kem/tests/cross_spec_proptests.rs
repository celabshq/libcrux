//! Property-based cross-spec tests: random seeds in × byte equality out.
//!
//! Complements the deterministic table in `cross_spec.rs`. 32 cases per
//! variant by default — keep the per-variant budget under a few seconds so
//! the suite stays usable on every `cargo test`. Tune via
//! `PROPTEST_CASES` env var for deeper smoke runs.
//!
//! Unlucky-seed handling: `hacspec_ml_kem::generate_keypair` can return
//! `Err(BadRejectionSamplingRandomnessError)`. We `prop_assume!` past such
//! seeds.

use proptest::prelude::*;

use hacspec_ml_kem::{
    self as spec, ML_KEM_1024, ML_KEM_1024_CT_SIZE, ML_KEM_1024_DK_PKE_SIZE, ML_KEM_1024_DK_SIZE,
    ML_KEM_1024_EK_SIZE, ML_KEM_1024_J_INPUT_SIZE, ML_KEM_1024_U_SIZE, ML_KEM_1024_V_SIZE,
    ML_KEM_512, ML_KEM_512_CT_SIZE, ML_KEM_512_DK_PKE_SIZE, ML_KEM_512_DK_SIZE, ML_KEM_512_EK_SIZE,
    ML_KEM_512_J_INPUT_SIZE, ML_KEM_512_U_SIZE, ML_KEM_512_V_SIZE, ML_KEM_768, ML_KEM_768_CT_SIZE,
    ML_KEM_768_DK_PKE_SIZE, ML_KEM_768_DK_SIZE, ML_KEM_768_EK_SIZE, ML_KEM_768_J_INPUT_SIZE,
    ML_KEM_768_U_SIZE, ML_KEM_768_V_SIZE,
};

/// Strategy: arbitrary 64-byte arrays for KeyGen randomness.
fn keygen_seed_strategy() -> impl Strategy<Value = [u8; 64]> {
    proptest::collection::vec(any::<u8>(), 64..=64).prop_map(|v| {
        let mut a = [0u8; 64];
        a.copy_from_slice(&v);
        a
    })
}

/// Strategy: arbitrary 32-byte arrays for Encaps randomness.
fn encaps_seed_strategy() -> impl Strategy<Value = [u8; 32]> {
    proptest::collection::vec(any::<u8>(), 32..=32).prop_map(|v| {
        let mut a = [0u8; 32];
        a.copy_from_slice(&v);
        a
    })
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 32,
        .. ProptestConfig::default()
    })]

    /// ML-KEM-512: spec ↔ impl agreement across KeyGen → Encaps → Decaps.
    #[cfg(feature = "mlkem512")]
    #[test]
    fn mlkem512_roundtrip_matches_spec(
        keygen_seed in keygen_seed_strategy(),
        encaps_seed in encaps_seed_strategy(),
    ) {
        let spec_kp = spec::generate_keypair::<
            2, ML_KEM_512_EK_SIZE, ML_KEM_512_DK_SIZE, ML_KEM_512_DK_PKE_SIZE,
        >(&ML_KEM_512, &keygen_seed);

        // Skip seeds the spec rejects.
        prop_assume!(spec_kp.is_ok());
        let (spec_ek, spec_dk) = spec_kp.unwrap();

        let kp = libcrux_ml_kem::mlkem512::generate_key_pair(keygen_seed);

        prop_assert_eq!(&kp.public_key().as_slice()[..], &spec_ek[..]);
        prop_assert_eq!(&kp.private_key().as_slice()[..], &spec_dk[..]);

        let (spec_ss, spec_ct) = spec::encapsulate::<
            2, ML_KEM_512_EK_SIZE, ML_KEM_512_U_SIZE, ML_KEM_512_V_SIZE, ML_KEM_512_CT_SIZE,
        >(&ML_KEM_512, &spec_ek, &encaps_seed)
            .expect("encaps on a successfully-generated key should not fail");

        let (impl_ct, impl_ss_e) =
            libcrux_ml_kem::mlkem512::encapsulate(kp.public_key(), encaps_seed);
        prop_assert_eq!(impl_ct.as_ref(), &spec_ct[..]);
        prop_assert_eq!(&impl_ss_e[..], &spec_ss[..]);

        let spec_ss_d = spec::decapsulate::<
            2, ML_KEM_512_EK_SIZE, ML_KEM_512_DK_SIZE, ML_KEM_512_DK_PKE_SIZE,
            ML_KEM_512_U_SIZE, ML_KEM_512_V_SIZE, ML_KEM_512_CT_SIZE, ML_KEM_512_J_INPUT_SIZE,
        >(&ML_KEM_512, &spec_dk, &spec_ct)
            .expect("spec decaps on a valid (ek, ct) should not fail");

        let impl_ss_d = libcrux_ml_kem::mlkem512::decapsulate(kp.private_key(), &impl_ct);
        prop_assert_eq!(&impl_ss_d[..], &spec_ss_d[..]);
    }

    /// ML-KEM-768: spec ↔ impl agreement across KeyGen → Encaps → Decaps.
    #[cfg(feature = "mlkem768")]
    #[test]
    fn mlkem768_roundtrip_matches_spec(
        keygen_seed in keygen_seed_strategy(),
        encaps_seed in encaps_seed_strategy(),
    ) {
        let spec_kp = spec::generate_keypair::<
            3, ML_KEM_768_EK_SIZE, ML_KEM_768_DK_SIZE, ML_KEM_768_DK_PKE_SIZE,
        >(&ML_KEM_768, &keygen_seed);
        prop_assume!(spec_kp.is_ok());
        let (spec_ek, spec_dk) = spec_kp.unwrap();

        let kp = libcrux_ml_kem::mlkem768::generate_key_pair(keygen_seed);

        prop_assert_eq!(&kp.public_key().as_slice()[..], &spec_ek[..]);
        prop_assert_eq!(&kp.private_key().as_slice()[..], &spec_dk[..]);

        let (spec_ss, spec_ct) = spec::encapsulate::<
            3, ML_KEM_768_EK_SIZE, ML_KEM_768_U_SIZE, ML_KEM_768_V_SIZE, ML_KEM_768_CT_SIZE,
        >(&ML_KEM_768, &spec_ek, &encaps_seed)
            .expect("encaps on a successfully-generated key should not fail");

        let (impl_ct, impl_ss_e) =
            libcrux_ml_kem::mlkem768::encapsulate(kp.public_key(), encaps_seed);
        prop_assert_eq!(impl_ct.as_ref(), &spec_ct[..]);
        prop_assert_eq!(&impl_ss_e[..], &spec_ss[..]);

        let spec_ss_d = spec::decapsulate::<
            3, ML_KEM_768_EK_SIZE, ML_KEM_768_DK_SIZE, ML_KEM_768_DK_PKE_SIZE,
            ML_KEM_768_U_SIZE, ML_KEM_768_V_SIZE, ML_KEM_768_CT_SIZE, ML_KEM_768_J_INPUT_SIZE,
        >(&ML_KEM_768, &spec_dk, &spec_ct)
            .expect("spec decaps on a valid (ek, ct) should not fail");

        let impl_ss_d = libcrux_ml_kem::mlkem768::decapsulate(kp.private_key(), &impl_ct);
        prop_assert_eq!(&impl_ss_d[..], &spec_ss_d[..]);
    }

    /// ML-KEM-1024: spec ↔ impl agreement across KeyGen → Encaps → Decaps.
    #[cfg(feature = "mlkem1024")]
    #[test]
    fn mlkem1024_roundtrip_matches_spec(
        keygen_seed in keygen_seed_strategy(),
        encaps_seed in encaps_seed_strategy(),
    ) {
        let spec_kp = spec::generate_keypair::<
            4, ML_KEM_1024_EK_SIZE, ML_KEM_1024_DK_SIZE, ML_KEM_1024_DK_PKE_SIZE,
        >(&ML_KEM_1024, &keygen_seed);
        prop_assume!(spec_kp.is_ok());
        let (spec_ek, spec_dk) = spec_kp.unwrap();

        let kp = libcrux_ml_kem::mlkem1024::generate_key_pair(keygen_seed);

        prop_assert_eq!(&kp.public_key().as_slice()[..], &spec_ek[..]);
        prop_assert_eq!(&kp.private_key().as_slice()[..], &spec_dk[..]);

        let (spec_ss, spec_ct) = spec::encapsulate::<
            4, ML_KEM_1024_EK_SIZE, ML_KEM_1024_U_SIZE, ML_KEM_1024_V_SIZE, ML_KEM_1024_CT_SIZE,
        >(&ML_KEM_1024, &spec_ek, &encaps_seed)
            .expect("encaps on a successfully-generated key should not fail");

        let (impl_ct, impl_ss_e) =
            libcrux_ml_kem::mlkem1024::encapsulate(kp.public_key(), encaps_seed);
        prop_assert_eq!(impl_ct.as_ref(), &spec_ct[..]);
        prop_assert_eq!(&impl_ss_e[..], &spec_ss[..]);

        let spec_ss_d = spec::decapsulate::<
            4, ML_KEM_1024_EK_SIZE, ML_KEM_1024_DK_SIZE, ML_KEM_1024_DK_PKE_SIZE,
            ML_KEM_1024_U_SIZE, ML_KEM_1024_V_SIZE, ML_KEM_1024_CT_SIZE, ML_KEM_1024_J_INPUT_SIZE,
        >(&ML_KEM_1024, &spec_dk, &spec_ct)
            .expect("spec decaps on a valid (ek, ct) should not fail");

        let impl_ss_d = libcrux_ml_kem::mlkem1024::decapsulate(kp.private_key(), &impl_ct);
        prop_assert_eq!(&impl_ss_d[..], &spec_ss_d[..]);
    }
}
