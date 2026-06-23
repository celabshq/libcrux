//! Cross-spec equivalence tests between `libcrux-ml-kem` and `hacspec_ml_kem`.
//!
//! These tests assert byte-identical agreement between the optimized impl and
//! the FIPS-203 hacspec reference on the IND-CCA surface (KeyGen / Encaps /
//! Decaps) at all three parameter sets.

use hacspec_ml_kem::{
    self as spec, ML_KEM_1024, ML_KEM_1024_CT_SIZE, ML_KEM_1024_DK_PKE_SIZE, ML_KEM_1024_DK_SIZE,
    ML_KEM_1024_EK_SIZE, ML_KEM_1024_J_INPUT_SIZE, ML_KEM_1024_U_SIZE, ML_KEM_1024_V_SIZE,
    ML_KEM_512, ML_KEM_512_CT_SIZE, ML_KEM_512_DK_PKE_SIZE, ML_KEM_512_DK_SIZE, ML_KEM_512_EK_SIZE,
    ML_KEM_512_J_INPUT_SIZE, ML_KEM_512_U_SIZE, ML_KEM_512_V_SIZE, ML_KEM_768, ML_KEM_768_CT_SIZE,
    ML_KEM_768_DK_PKE_SIZE, ML_KEM_768_DK_SIZE, ML_KEM_768_EK_SIZE, ML_KEM_768_J_INPUT_SIZE,
    ML_KEM_768_U_SIZE, ML_KEM_768_V_SIZE,
};

// ---------------------------------------------------------------------------
// Const-generic size assertions — fail fast on any drift between the impl's
// hard-coded sizes (per FIPS-203 §8 table) and the spec's exported constants.
// ---------------------------------------------------------------------------

#[cfg(feature = "mlkem512")]
#[test]
fn size_constants_agree_512() {
    use libcrux_ml_kem::mlkem512::*;
    assert_eq!(MlKem512PublicKey::len(), ML_KEM_512_EK_SIZE);
    assert_eq!(MlKem512PrivateKey::len(), ML_KEM_512_DK_SIZE);
    assert_eq!(MlKem512Ciphertext::len(), ML_KEM_512_CT_SIZE);
}

#[cfg(feature = "mlkem768")]
#[test]
fn size_constants_agree_768() {
    use libcrux_ml_kem::mlkem768::*;
    assert_eq!(MlKem768PublicKey::len(), ML_KEM_768_EK_SIZE);
    assert_eq!(MlKem768PrivateKey::len(), ML_KEM_768_DK_SIZE);
    assert_eq!(MlKem768Ciphertext::len(), ML_KEM_768_CT_SIZE);
}

#[cfg(feature = "mlkem1024")]
#[test]
fn size_constants_agree_1024() {
    use libcrux_ml_kem::mlkem1024::*;
    assert_eq!(MlKem1024PublicKey::len(), ML_KEM_1024_EK_SIZE);
    assert_eq!(MlKem1024PrivateKey::len(), ML_KEM_1024_DK_SIZE);
    assert_eq!(MlKem1024Ciphertext::len(), ML_KEM_1024_CT_SIZE);
}

// ---------------------------------------------------------------------------
// Seed tables — deterministic vectors exercised by every variant. The last
// entry in each is the first NIST ML-KEM KAT seed (identical across the
// 512/768/1024 KAT files), so the cross-comparison also runs on a known NIST
// vector. (Agreement with the published KAT outputs themselves is checked in
// `nistkats.rs`.)
// ---------------------------------------------------------------------------

const KEYGEN_SEEDS: &[[u8; 64]] = &[
    [0x00; 64],
    [0xFF; 64],
    [0xAA; 64],
    {
        let mut s = [0u8; 64];
        let mut i = 0usize;
        while i < 64 {
            s[i] = i as u8;
            i += 1;
        }
        s
    },
    // First NIST ML-KEM KAT keygen seed.
    [
        0x7c, 0x99, 0x35, 0xa0, 0xb0, 0x76, 0x94, 0xaa, 0x0c, 0x6d, 0x10, 0xe4, 0xdb, 0x6b, 0x1a,
        0xdd, 0x2f, 0xd8, 0x1a, 0x25, 0xcc, 0xb1, 0x48, 0x03, 0x2d, 0xcd, 0x73, 0x99, 0x36, 0x73,
        0x7f, 0x2d, 0x86, 0x26, 0xed, 0x79, 0xd4, 0x51, 0x14, 0x08, 0x00, 0xe0, 0x3b, 0x59, 0xb9,
        0x56, 0xf8, 0x21, 0x0e, 0x55, 0x60, 0x67, 0x40, 0x7d, 0x13, 0xdc, 0x90, 0xfa, 0x9e, 0x8b,
        0x87, 0x2b, 0xfb, 0x8f,
    ],
];
const ENCAPS_SEEDS: &[[u8; 32]] = &[
    [0x00; 32],
    [0xFF; 32],
    [0xCD; 32],
    // First NIST ML-KEM KAT encaps seed.
    [
        0x14, 0x7c, 0x03, 0xf7, 0xa5, 0xbe, 0xbb, 0xa4, 0x06, 0xc8, 0xfa, 0xe1, 0x87, 0x4d, 0x7f,
        0x13, 0xc8, 0x0e, 0xfe, 0x79, 0xa3, 0xa9, 0xa8, 0x74, 0xcc, 0x09, 0xfe, 0x76, 0xf6, 0x99,
        0x76, 0x15,
    ],
];

// ---------------------------------------------------------------------------
// IND-CCA byte-level cross-comparison (impl vs spec).
//
// Generated per parameter set. Each variant gets:
//   1. keygen_matches_spec       — pk, sk byte-equal.
//   2. encapsulate_matches_spec  — ct, ss byte-equal on a fresh keypair.
//   3. full_roundtrip_matches_spec — encaps then decaps; impl ss matches
//                                      spec ss on both sides.
//   4. decaps_rejection_matches_spec — flipping a ct byte triggers implicit
//                                      rejection; impl ss matches spec ss
//                                      (i.e., both produce J(s, c')).
// ---------------------------------------------------------------------------

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
                for (i, randomness) in KEYGEN_SEEDS.iter().enumerate() {
                    let (spec_ek, spec_dk) =
                        spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, randomness)
                            .unwrap_or_else(|_| {
                                panic!(
                                    concat!(
                                        stringify!($mod_name),
                                        " spec keygen failed for seed index {}; ",
                                        "consider replacing KEYGEN_SEEDS[{}]"
                                    ),
                                    i, i
                                )
                            });

                    let kp = impl_mod::generate_key_pair(*randomness);

                    assert_eq!(
                        &kp.public_key().as_slice()[..],
                        &spec_ek[..],
                        concat!(stringify!($mod_name), " ek mismatch (seed idx={})"),
                        i
                    );
                    assert_eq!(
                        &kp.private_key().as_slice()[..],
                        &spec_dk[..],
                        concat!(stringify!($mod_name), " dk mismatch (seed idx={})"),
                        i
                    );
                }
            }

            #[test]
            fn encapsulate_matches_spec() {
                let keygen_rand = [1u8; 64];
                let (spec_ek, _) =
                    spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, &keygen_rand)
                        .expect("spec keygen failed");
                let kp = impl_mod::generate_key_pair(keygen_rand);

                for (i, encaps_rand) in ENCAPS_SEEDS.iter().enumerate() {
                    let (spec_ss, spec_ct) =
                        spec::encapsulate::<$k, $ek, $u, $v, $ct>(&$params, &spec_ek, encaps_rand)
                            .expect("spec encaps failed");

                    let (impl_ct, impl_ss) = impl_mod::encapsulate(kp.public_key(), *encaps_rand);

                    assert_eq!(
                        impl_ct.as_ref(),
                        &spec_ct[..],
                        concat!(stringify!($mod_name), " ct mismatch (encaps idx={})"),
                        i
                    );
                    assert_eq!(
                        &impl_ss[..],
                        &spec_ss[..],
                        concat!(stringify!($mod_name), " ss mismatch (encaps idx={})"),
                        i
                    );
                }
            }

            #[test]
            fn full_roundtrip_matches_spec() {
                for (i, randomness) in KEYGEN_SEEDS.iter().enumerate() {
                    let encaps_rand = ENCAPS_SEEDS[i % ENCAPS_SEEDS.len()];

                    let (spec_ek, spec_dk) =
                        spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, randomness)
                            .unwrap_or_else(|_| {
                                panic!(
                                    concat!(
                                        stringify!($mod_name),
                                        " spec keygen failed for seed idx {}"
                                    ),
                                    i
                                )
                            });
                    let (spec_ss, spec_ct) =
                        spec::encapsulate::<$k, $ek, $u, $v, $ct>(&$params, &spec_ek, &encaps_rand)
                            .expect("spec encaps failed");
                    let spec_ss_d = spec::decapsulate::<$k, $ek, $dk, $dk_pke, $u, $v, $ct, $j>(
                        &$params, &spec_dk, &spec_ct,
                    )
                    .expect("spec decaps failed");

                    let kp = impl_mod::generate_key_pair(*randomness);
                    let (impl_ct, impl_ss_e) = impl_mod::encapsulate(kp.public_key(), encaps_rand);
                    let impl_ss_d = impl_mod::decapsulate(kp.private_key(), &impl_ct);

                    assert_eq!(impl_ct.as_ref(), &spec_ct[..], "ct (seed idx={})", i);
                    assert_eq!(&impl_ss_e[..], &spec_ss[..], "encaps ss");
                    assert_eq!(&impl_ss_d[..], &spec_ss_d[..], "decaps ss");
                    assert_eq!(
                        &impl_ss_e[..],
                        &impl_ss_d[..],
                        "impl encaps/decaps ss differ"
                    );
                }
            }

            #[test]
            fn decaps_rejection_matches_spec() {
                // Implicit rejection: tampered ct should produce J(s, c').
                // Both impl and spec implement the FIPS-203 implicit rejection;
                // they should agree byte-for-byte.
                let keygen_rand = KEYGEN_SEEDS[1]; // 0xFF seed
                let encaps_rand = ENCAPS_SEEDS[0];

                let (spec_ek, spec_dk) =
                    spec::generate_keypair::<$k, $ek, $dk, $dk_pke>(&$params, &keygen_rand)
                        .expect("spec keygen failed");
                let (_spec_ss, spec_ct) =
                    spec::encapsulate::<$k, $ek, $u, $v, $ct>(&$params, &spec_ek, &encaps_rand)
                        .expect("spec encaps failed");

                let kp = impl_mod::generate_key_pair(keygen_rand);

                // Tamper one byte of the ciphertext.
                let mut tampered_ct: [u8; $ct] = spec_ct;
                tampered_ct[7] ^= 0xA5;

                let spec_ss_rej = spec::decapsulate::<$k, $ek, $dk, $dk_pke, $u, $v, $ct, $j>(
                    &$params,
                    &spec_dk,
                    &tampered_ct,
                )
                .expect("spec decaps (rejection branch) failed");

                let impl_tampered_ct = tampered_ct.into();
                let impl_ss_rej = impl_mod::decapsulate(kp.private_key(), &impl_tampered_ct);

                assert_eq!(
                    &impl_ss_rej[..],
                    &spec_ss_rej[..],
                    concat!(stringify!($mod_name), " implicit rejection ss mismatch")
                );
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
    ML_KEM_512_CT_SIZE,
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
    ML_KEM_768_CT_SIZE,
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
    ML_KEM_1024_CT_SIZE,
    { ML_KEM_1024_J_INPUT_SIZE }
);
