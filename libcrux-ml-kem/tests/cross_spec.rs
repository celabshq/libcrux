//! Cross-comparison tests between the ML-KEM implementation and specification.
//!
//! These tests verify that the implementation produces byte-identical outputs
//! to the specification for all ML-KEM operations, using deterministic randomness.

use hacspec_ml_kem::{
    self as spec, ML_KEM_1024, ML_KEM_1024_CT_SIZE, ML_KEM_1024_DK_PKE_SIZE, ML_KEM_1024_DK_SIZE,
    ML_KEM_1024_EK_SIZE, ML_KEM_1024_J_INPUT_SIZE, ML_KEM_1024_U_SIZE, ML_KEM_1024_V_SIZE,
    ML_KEM_512, ML_KEM_512_CT_SIZE, ML_KEM_512_DK_PKE_SIZE, ML_KEM_512_DK_SIZE, ML_KEM_512_EK_SIZE,
    ML_KEM_512_J_INPUT_SIZE, ML_KEM_512_U_SIZE, ML_KEM_512_V_SIZE, ML_KEM_768, ML_KEM_768_CT_SIZE,
    ML_KEM_768_DK_PKE_SIZE, ML_KEM_768_DK_SIZE, ML_KEM_768_EK_SIZE, ML_KEM_768_J_INPUT_SIZE,
    ML_KEM_768_U_SIZE, ML_KEM_768_V_SIZE,
};

// ---------------------------------------------------------------------------
// Layer 0: Hash function agreement
// ---------------------------------------------------------------------------

/// Verify that the spec's SHA3 (hacspec_sha3) and the impl's SHA3 (libcrux_sha3)
/// produce identical outputs. This is a prerequisite for all other tests.
mod hash_agreement {
    #[test]
    fn sha3_512() {
        for input in [
            &b"test input for G"[..],
            &[0u8; 0][..],
            &[0xAB; 64][..],
            &[0xFF; 33][..],
        ] {
            let spec_out = hacspec_sha3::sha3_512(input);
            let impl_out = libcrux_sha3::sha512(input);
            assert_eq!(
                spec_out,
                impl_out,
                "SHA3-512 mismatch for input len={}",
                input.len()
            );
        }
    }

    #[test]
    fn sha3_256() {
        for input in [&b"test input for H"[..], &[0u8; 0][..], &[0xAB; 64][..]] {
            let spec_out = hacspec_sha3::sha3_256(input);
            let impl_out = libcrux_sha3::sha256(input);
            assert_eq!(
                spec_out,
                impl_out,
                "SHA3-256 mismatch for input len={}",
                input.len()
            );
        }
    }

    #[test]
    fn shake256() {
        for input in [&b"test PRF"[..], &[0u8; 33][..], &[0xFF; 34][..]] {
            let spec_out: [u8; 128] = hacspec_sha3::shake256(input);
            let impl_out: [u8; 128] = libcrux_sha3::shake256(input);
            assert_eq!(
                spec_out,
                impl_out,
                "SHAKE256 mismatch for input len={}",
                input.len()
            );
        }
    }

    #[test]
    fn shake128() {
        for input in [&b"test XOF"[..], &[0u8; 34][..], &[0xFF; 34][..]] {
            let spec_out: [u8; 840] = hacspec_sha3::shake128(input);
            let impl_out: [u8; 840] = libcrux_sha3::shake128(input);
            assert_eq!(
                spec_out,
                impl_out,
                "SHAKE128 mismatch for input len={}",
                input.len()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Layer 1: Serialization / Compression (spec-side unit tests)
// ---------------------------------------------------------------------------

mod serialization {
    use hacspec_ml_kem::compress::*;
    use hacspec_ml_kem::parameters::*;
    use hacspec_ml_kem::serialize::*;

    /// byte_encode then byte_decode is identity for d < 12.
    #[test]
    fn byte_encode_decode_roundtrip() {
        for d in [1usize, 4, 5, 10, 11] {
            let poly: Polynomial = createi(|i| FieldElement::new((i as u16 * 7 + 3) % (1u16 << d)));
            match d {
                1 => {
                    let encoded = byte_encode::<32, 256>(poly, 1);
                    let decoded = byte_decode::<32, 256>(&encoded, 1);
                    assert_eq!(poly, decoded, "roundtrip failed for d=1");
                }
                4 => {
                    let encoded = byte_encode::<128, 1024>(poly, 4);
                    let decoded = byte_decode::<128, 1024>(&encoded, 4);
                    assert_eq!(poly, decoded, "roundtrip failed for d=4");
                }
                5 => {
                    let encoded = byte_encode::<160, 1280>(poly, 5);
                    let decoded = byte_decode::<160, 1280>(&encoded, 5);
                    assert_eq!(poly, decoded, "roundtrip failed for d=5");
                }
                10 => {
                    let encoded = byte_encode::<320, 2560>(poly, 10);
                    let decoded = byte_decode::<320, 2560>(&encoded, 10);
                    assert_eq!(poly, decoded, "roundtrip failed for d=10");
                }
                11 => {
                    let encoded = byte_encode::<352, 2816>(poly, 11);
                    let decoded = byte_decode::<352, 2816>(&encoded, 11);
                    assert_eq!(poly, decoded, "roundtrip failed for d=11");
                }
                _ => unreachable!(),
            }
        }
    }

    /// byte_encode(12) then byte_decode(12) reduces mod q.
    #[test]
    fn byte_encode_decode_12_roundtrip() {
        let poly: Polynomial = createi(|i| FieldElement::new((i as u16 * 13) % FIELD_MODULUS));
        let encoded = byte_encode::<384, 3072>(poly, 12);
        let decoded = byte_decode::<384, 3072>(&encoded, 12);
        assert_eq!(poly, decoded);
    }

    /// compress then decompress roundtrip preserves identity on decompressed values.
    #[test]
    fn compress_decompress_roundtrip() {
        for d in [1usize, 4, 5, 10, 11] {
            let upper = 1u16 << d;
            let poly: Polynomial = createi(|i| FieldElement::new((i as u16) % upper));
            let compressed = compress(poly, d);
            let decompressed = decompress(compressed, d);
            let recompressed = compress(decompressed, d);
            assert_eq!(
                compressed, recompressed,
                "compress(decompress(x)) != x for d={d}"
            );
        }
    }

    /// compress_then_serialize_message roundtrip.
    #[test]
    fn message_serialize_roundtrip() {
        let msg_bytes = [0xABu8; 32];
        let poly = deserialize_then_decompress_message(&msg_bytes);
        let reencoded = compress_then_serialize_message(poly);
        assert_eq!(msg_bytes, reencoded);
    }
}

// ---------------------------------------------------------------------------
// Layer 2: NTT (spec-side unit tests)
// ---------------------------------------------------------------------------

mod ntt_tests {
    use hacspec_ml_kem::invert_ntt::ntt_inverse;
    use hacspec_ml_kem::ntt::*;
    use hacspec_ml_kem::parameters::*;

    /// NTT then inverse NTT is identity.
    #[test]
    fn ntt_roundtrip() {
        for seed in [0u16, 42, 1000] {
            let poly: Polynomial = createi(|i| {
                FieldElement::new(((i as u16).wrapping_mul(seed).wrapping_add(7)) % FIELD_MODULUS)
            });
            let ntt_poly = vector_ntt([poly])[0];
            let recovered = ntt_inverse(ntt_poly);
            assert_eq!(poly, recovered, "NTT roundtrip failed for seed={seed}");
        }
    }

    /// NTT multiplication corresponds to polynomial multiplication.
    /// Test: NTT(1) * NTT(1) should yield NTT(1) since 1*1=1 in R_q.
    #[test]
    fn ntt_multiply_one_times_one() {
        let mut one = [FieldElement::new(0); 256];
        one[0] = FieldElement::new(1);

        let one_ntt = vector_ntt([one])[0];
        let product_ntt = multiply_ntts(&one_ntt, &one_ntt);
        let product = ntt_inverse(product_ntt);

        assert_eq!(product, one, "1 * 1 should equal 1");
    }

    /// NTT multiply: 1 * X = X.
    #[test]
    fn ntt_multiply_one_times_x() {
        let mut one = [FieldElement::new(0); 256];
        one[0] = FieldElement::new(1);
        let mut x = [FieldElement::new(0); 256];
        x[1] = FieldElement::new(1);

        let one_ntt = vector_ntt([one])[0];
        let x_ntt = vector_ntt([x])[0];
        let product_ntt = multiply_ntts(&one_ntt, &x_ntt);
        let product = ntt_inverse(product_ntt);

        assert_eq!(product, x, "1 * X should equal X");
    }
}

// ---------------------------------------------------------------------------
// Layer 3: Sampling (spec-side unit tests)
// ---------------------------------------------------------------------------

mod sampling_tests {
    use hacspec_ml_kem::parameters::*;
    use hacspec_ml_kem::sampling::*;

    /// CBD with eta=2: all coefficients should be in {0, 1, 2, 3327, 3328}.
    #[test]
    fn cbd_eta2_range() {
        let bytes = [0x55u8; 128]; // deterministic pattern
        let poly = sample_poly_cbd::<128, 1024>(2, &bytes);
        for (i, coeff) in poly.iter().enumerate() {
            assert!(
                coeff.val <= 2 || coeff.val >= FIELD_MODULUS - 2,
                "CBD eta=2 coefficient {} out of range: {}",
                i,
                coeff.val
            );
        }
    }

    /// CBD with eta=3: all coefficients should be in {0,1,2,3, 3326,3327,3328}.
    #[test]
    fn cbd_eta3_range() {
        let bytes = [0xAAu8; 192]; // deterministic pattern
        let poly = sample_poly_cbd::<192, 1536>(3, &bytes);
        for (i, coeff) in poly.iter().enumerate() {
            assert!(
                coeff.val <= 3 || coeff.val >= FIELD_MODULUS - 3,
                "CBD eta=3 coefficient {} out of range: {}",
                i,
                coeff.val
            );
        }
    }

    /// Rejection sampling: all-zero input should produce all-zero polynomial.
    #[test]
    fn rejection_sampling_zeros() {
        let bytes = [0u8; 840];
        let poly = sample_ntt::<70, 560, 840, 6720>(bytes).unwrap();
        for coeff in poly.iter() {
            assert_eq!(coeff.val, 0);
        }
    }
}

// ---------------------------------------------------------------------------
// Layer 4: IND-CPA (spec-side unit tests via byte comparison)
// ---------------------------------------------------------------------------

mod ind_cpa_tests {
    use hacspec_ml_kem::{self as spec, *};

    /// IND-CPA encrypt then decrypt roundtrip (spec-only, verifying spec correctness).
    #[test]
    fn spec_encrypt_decrypt_roundtrip() {
        let seed = [42u8; 32];
        let (ek, dk) = spec::ind_cpa::generate_keypair::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
        >(&ML_KEM_768, &seed)
        .unwrap();

        let message = [0xABu8; 32];
        let randomness = [0xCDu8; 32];
        let ct = spec::ind_cpa::encrypt::<
            3,
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
        >(&ML_KEM_768, &ek, &message, &randomness)
        .unwrap();

        let decrypted = spec::ind_cpa::decrypt::<3>(&ML_KEM_768, &dk, &ct);
        assert_eq!(decrypted, message);
    }
}

// ---------------------------------------------------------------------------
// Layers 5-6: IND-CCA byte-level cross-comparison (impl vs spec)
// ---------------------------------------------------------------------------

/// Macro to generate cross-spec tests for a given ML-KEM parameter set.
macro_rules! cross_spec_tests {
    (
        $mod_name:ident,
        $feature:literal,
        $impl_mod:path,
        $k:literal,
        $params:expr,
        $ek:expr, $dk:expr, $dk_pke:expr,
        $u:expr, $v:expr, $ct:expr, $j:expr
    ) => {
        #[cfg(feature = $feature)]
        mod $mod_name {
            use super::*;
            use $impl_mod as impl_mod;

            #[test]
            fn keygen_matches_spec() {
                for seed_byte in [0u8, 42, 0xFF] {
                    let randomness = [seed_byte; 64];

                    let (spec_ek, spec_dk) =
                        spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, &randomness)
                            .expect("spec keygen failed");

                    let (impl_sk, impl_pk) = impl_mod::generate_key_pair(randomness).into_parts();

                    assert_eq!(
                        impl_pk.as_slice(),
                        &spec_ek[..],
                        concat!(stringify!($mod_name), " ek mismatch (seed_byte={})"),
                        seed_byte
                    );
                    assert_eq!(
                        impl_sk.as_slice(),
                        &spec_dk[..],
                        concat!(stringify!($mod_name), " dk mismatch (seed_byte={})"),
                        seed_byte
                    );
                }
            }

            #[test]
            fn encapsulate_matches_spec() {
                let keygen_rand = [1u8; 64];
                let encaps_rand = [0xABu8; 32];

                let (spec_ek, _) =
                    spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, &keygen_rand)
                        .expect("spec keygen failed");

                let (_impl_sk, impl_pk) = impl_mod::generate_key_pair(keygen_rand).into_parts();

                let (spec_ss, spec_ct) =
                    spec::encapsulate::<$k, $ek, $u, $v, $ct>(&$params, &spec_ek, &encaps_rand)
                        .expect("spec encaps failed");

                let (impl_ct, impl_ss) = impl_mod::encapsulate(&impl_pk, encaps_rand);

                assert_eq!(
                    impl_ct.as_ref(),
                    &spec_ct[..],
                    concat!(stringify!($mod_name), " ciphertext mismatch")
                );
                assert_eq!(
                    &impl_ss[..],
                    &spec_ss[..],
                    concat!(stringify!($mod_name), " shared secret mismatch")
                );
            }

            #[test]
            fn full_roundtrip_matches_spec() {
                for seed_byte in [0u8, 42, 0xFF] {
                    let keygen_rand = [seed_byte; 64];
                    let encaps_rand = [seed_byte.wrapping_add(1); 32];

                    // Spec
                    let (spec_ek, spec_dk) =
                        spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, &keygen_rand)
                            .expect("spec keygen failed");

                    let (spec_ss, spec_ct) =
                        spec::encapsulate::<$k, $ek, $u, $v, $ct>(&$params, &spec_ek, &encaps_rand)
                            .expect("spec encaps failed");

                    let spec_ss_decaps =
                        spec::decapsulate::<$k, $ek, $dk, $dk_pke, $u, $v, $ct, $j>(
                            &$params, &spec_dk, &spec_ct,
                        )
                        .expect("spec decaps failed");

                    // Impl
                    let (impl_sk, impl_pk) = impl_mod::generate_key_pair(keygen_rand).into_parts();
                    let (impl_ct, impl_ss_encaps) = impl_mod::encapsulate(&impl_pk, encaps_rand);
                    let impl_ss_decaps = impl_mod::decapsulate(&impl_sk, &impl_ct);

                    // Cross-compare
                    assert_eq!(
                        impl_ct.as_ref(),
                        &spec_ct[..],
                        concat!(stringify!($mod_name), " ct mismatch (seed_byte={})"),
                        seed_byte
                    );
                    assert_eq!(
                        &impl_ss_encaps[..],
                        &spec_ss[..],
                        concat!(stringify!($mod_name), " encaps ss mismatch (seed_byte={})"),
                        seed_byte
                    );
                    assert_eq!(
                        &impl_ss_decaps[..],
                        &spec_ss_decaps[..],
                        concat!(stringify!($mod_name), " decaps ss mismatch (seed_byte={})"),
                        seed_byte
                    );
                    assert_eq!(
                        &impl_ss_encaps[..],
                        &impl_ss_decaps[..],
                        concat!(
                            stringify!($mod_name),
                            " encaps/decaps ss differ (seed_byte={})"
                        ),
                        seed_byte
                    );
                }
            }
        }
    };
}

cross_spec_tests!(
    mlkem512_cross,
    "mlkem512",
    libcrux_ml_kem::mlkem512,
    2,
    ML_KEM_512,
    { ML_KEM_512_EK_SIZE },
    { ML_KEM_512_DK_SIZE },
    { ML_KEM_512_DK_PKE_SIZE },
    { ML_KEM_512_U_SIZE },
    { ML_KEM_512_V_SIZE },
    { ML_KEM_512_CT_SIZE },
    { ML_KEM_512_J_INPUT_SIZE }
);

cross_spec_tests!(
    mlkem768_cross,
    "mlkem768",
    libcrux_ml_kem::mlkem768,
    3,
    ML_KEM_768,
    { ML_KEM_768_EK_SIZE },
    { ML_KEM_768_DK_SIZE },
    { ML_KEM_768_DK_PKE_SIZE },
    { ML_KEM_768_U_SIZE },
    { ML_KEM_768_V_SIZE },
    { ML_KEM_768_CT_SIZE },
    { ML_KEM_768_J_INPUT_SIZE }
);

cross_spec_tests!(
    mlkem1024_cross,
    "mlkem1024",
    libcrux_ml_kem::mlkem1024,
    4,
    ML_KEM_1024,
    { ML_KEM_1024_EK_SIZE },
    { ML_KEM_1024_DK_SIZE },
    { ML_KEM_1024_DK_PKE_SIZE },
    { ML_KEM_1024_U_SIZE },
    { ML_KEM_1024_V_SIZE },
    { ML_KEM_1024_CT_SIZE },
    { ML_KEM_1024_J_INPUT_SIZE }
);
