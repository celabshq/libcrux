//! Cross-spec tests for the encoding methods of the `Operations` trait.
//!
//! Six round-trip tests + one direct cross-check against the hacspec
//! `simple_bit_pack` / `bit_pack` helpers.  Each round-trip drives the
//! impl through `serialize ∘ deserialize` (or vice versa) and checks
//! agreement with the hacspec reference.

#![allow(dead_code, unused_imports, unused_variables)]

use super::helpers::*;
use hacspec_ml_dsa as spec;
#[cfg(feature = "simd256")]
use libcrux_ml_dsa::test_utils::simd::AVX2SIMDUnit;
use libcrux_ml_dsa::test_utils::simd::Operations;
use libcrux_ml_dsa::test_utils::simd::PortableSIMDUnit;
use libcrux_ml_dsa::test_utils::Eta;
use rand::Rng;

const ITERATIONS: usize = 100;

// γ₁ exponents from FIPS 204 Table 1.
const GAMMA1_EXPONENTS: &[usize] = &[17, 19];
// γ₁ values: 2^exp.
const GAMMA1_17: i32 = 1 << 17;
const GAMMA1_19: i32 = 1 << 19;
// Commitment widths: 4 bits for γ₂ = 261_888, 6 bits for γ₂ = 95_232.
const COMMITMENT_WIDTHS: &[usize] = &[4, 6];
// η values.
const ETAS: &[usize] = &[2, 4];
// t0: 13-bit signed, range [-2¹², 2¹²].
const T0_HALF: i32 = 1 << 12;
// t1: 10-bit unsigned.
const T1_BOUND: i32 = 1 << 10;

/// γ₁ serialize ∘ deserialize round-trip for both γ₁ exponents.
pub fn test_gamma1_serialize_deserialize_roundtrip<O: Operations>() {
    let mut rng = seeded_rng(0xC101);
    for &gamma1_exponent in GAMMA1_EXPONENTS {
        let gamma1: i32 = 1 << gamma1_exponent;
        let bits_per_coeff = gamma1_exponent + 1;
        let bytes_per_unit = LANES * bits_per_coeff / 8;
        for _ in 0..ITERATIONS {
            // Keep strictly inside the packable range [-(γ₁-1), γ₁-1] so the
            // (γ₁ - coeff) packing fits in `bits_per_coeff` bits without
            // truncation (coeff = -γ₁ would overflow).
            let coeffs = random_simd_unit_signed(&mut rng, gamma1 - 1);
            let unit = to_simd_unit::<O>(&coeffs);
            let mut buf = vec![0u8; bytes_per_unit];
            O::gamma1_serialize(&unit, &mut buf, gamma1_exponent);
            let mut round = O::zero();
            O::gamma1_deserialize(&buf, &mut round, gamma1_exponent);
            let got = from_simd_unit::<O>(&round);
            assert_eq!(got, coeffs, "gamma1 roundtrip failed");
        }
    }
}

/// commitment serialize (4-bit and 6-bit widths) cross-checked against
/// `hacspec_ml_dsa::encoding::simple_bit_pack`.
///
/// There is no `commitment_deserialize` on the trait, so instead of a
/// round-trip we build a full 256-coefficient polynomial from 32 SIMD-unit
/// serializations and compare the concatenated bytes against the spec's
/// `simple_bit_pack` (whose bit-width is `bitlen(b) = width`).
pub fn test_commitment_serialize_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0xC0A0);
    for &width in COMMITMENT_WIDTHS {
        let max = 1i32 << width; // coeffs in [0, 2^width)
        let bytes_per_unit = LANES * width / 8;
        for _ in 0..ITERATIONS {
            let poly = random_polynomial_coeffs(&mut rng, max);
            let mut buf = vec![0u8; SIMD_UNITS * bytes_per_unit];
            for u in 0..SIMD_UNITS {
                let mut lane = [0i32; LANES];
                lane.copy_from_slice(&poly[u * LANES..(u + 1) * LANES]);
                let unit = to_simd_unit::<O>(&lane);
                O::commitment_serialize(
                    &unit,
                    &mut buf[u * bytes_per_unit..(u + 1) * bytes_per_unit],
                );
            }
            // bitlen(b) == width  ⟺  b = 2^width - 1.
            let b = (max - 1) as usize;
            let spec_buf: Vec<u8> = match width {
                4 => spec::encoding::simple_bit_pack::<{ 32 * 4 }>(&poly, b).to_vec(),
                6 => spec::encoding::simple_bit_pack::<{ 32 * 6 }>(&poly, b).to_vec(),
                _ => unreachable!(),
            };
            assert_eq!(
                &buf[..],
                &spec_buf[..],
                "commitment vs simple_bit_pack mismatch (width {})",
                width
            );
        }
    }
}

/// error serialize ∘ deserialize round-trip for η ∈ {2, 4}.
pub fn test_error_serialize_deserialize_roundtrip<O: Operations>() {
    let mut rng = seeded_rng(0xE220);
    for &eta in ETAS {
        let bits = if eta == 2 { 3 } else { 4 };
        let bytes_per_unit = LANES * bits / 8;
        for _ in 0..ITERATIONS {
            let coeffs = random_simd_unit_signed(&mut rng, eta as i32);
            let eta_enum = if eta == 2 { Eta::Two } else { Eta::Four };
            let unit = to_simd_unit::<O>(&coeffs);
            let mut buf = vec![0u8; bytes_per_unit];
            O::error_serialize(eta_enum, &unit, &mut buf);
            let mut round = O::zero();
            O::error_deserialize(eta_enum, &buf, &mut round);
            let got = from_simd_unit::<O>(&round);
            assert_eq!(got, coeffs, "error roundtrip mismatch");
        }
    }
}

/// t0 serialize ∘ deserialize round-trip (13-bit signed).
pub fn test_t0_serialize_deserialize_roundtrip<O: Operations>() {
    let mut rng = seeded_rng(0x7000);
    let bytes_per_unit = LANES * 13 / 8; // = 13
    for _ in 0..ITERATIONS {
        // t0 packs (2¹² - coeff) in 13 bits; keep strictly inside
        // [-(2¹²-1), 2¹²-1] so the value fits without truncation.
        let coeffs = random_simd_unit_signed(&mut rng, T0_HALF - 1);
        let unit = to_simd_unit::<O>(&coeffs);
        let mut buf = vec![0u8; bytes_per_unit];
        O::t0_serialize(&unit, &mut buf);
        let mut round = O::zero();
        O::t0_deserialize(&buf, &mut round);
        let got = from_simd_unit::<O>(&round);
        assert_eq!(got, coeffs, "t0 roundtrip mismatch");
    }
}

/// t1 serialize ∘ deserialize round-trip (10-bit unsigned).
pub fn test_t1_serialize_deserialize_roundtrip<O: Operations>() {
    let mut rng = seeded_rng(0x7101);
    let bytes_per_unit = LANES * 10 / 8; // = 10
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_coeffs(&mut rng, T1_BOUND);
        let unit = to_simd_unit::<O>(&coeffs);
        let mut buf = vec![0u8; bytes_per_unit];
        O::t1_serialize(&unit, &mut buf);
        let mut round = O::zero();
        O::t1_deserialize(&buf, &mut round);
        let got = from_simd_unit::<O>(&round);
        assert_eq!(got, coeffs, "t1 roundtrip mismatch");
    }
}

/// Direct cross-check: γ₁ serialize matches `hacspec_ml_dsa::encoding::bit_pack`.
///
/// Builds a full 256-coefficient polynomial via 32 invocations of
/// `gamma1_serialize` on consecutive SIMD units, then compares the
/// concatenated bytes against the spec's `bit_pack(w, gamma1, gamma1 - 1)`.
pub fn test_gamma1_serialize_matches_spec_bit_pack<O: Operations>() {
    let mut rng = seeded_rng(0xC1BC);
    for &gamma1_exponent in GAMMA1_EXPONENTS {
        let gamma1: i32 = 1 << gamma1_exponent;
        let bits_per_coeff = gamma1_exponent + 1;
        let total_bytes = N * bits_per_coeff / 8;
        let bytes_per_unit = LANES * bits_per_coeff / 8;
        for _ in 0..ITERATIONS {
            // Inside the packable range [-(γ₁-1), γ₁-1] (see roundtrip note).
            let poly = random_polynomial_signed(&mut rng, gamma1 - 1);
            let mut buf = vec![0u8; total_bytes];
            for u in 0..SIMD_UNITS {
                let mut unit_coeffs = [0i32; LANES];
                unit_coeffs.copy_from_slice(&poly[u * LANES..(u + 1) * LANES]);
                let unit = to_simd_unit::<O>(&unit_coeffs);
                O::gamma1_serialize(
                    &unit,
                    &mut buf[u * bytes_per_unit..(u + 1) * bytes_per_unit],
                    gamma1_exponent,
                );
            }
            // γ₁ packing is bit_pack with a = γ₁ - 1, b = γ₁.
            let spec_buf: Vec<u8> = match gamma1_exponent {
                17 => spec::encoding::bit_pack::<{ 32 * 18 }>(
                    &poly,
                    (gamma1 - 1) as usize,
                    gamma1 as usize,
                )
                .to_vec(),
                19 => spec::encoding::bit_pack::<{ 32 * 20 }>(
                    &poly,
                    (gamma1 - 1) as usize,
                    gamma1 as usize,
                )
                .to_vec(),
                _ => unreachable!(),
            };
            assert_eq!(&buf[..], &spec_buf[..], "gamma1 bit_pack mismatch");
        }
    }
}

// ---------------------------------------------------------------------------
// Concrete instantiations.
// ---------------------------------------------------------------------------

#[test]
fn gamma1_serialize_deserialize_roundtrip_portable() {
    test_gamma1_serialize_deserialize_roundtrip::<PortableSIMDUnit>();
}

#[test]
fn commitment_serialize_matches_spec_portable() {
    test_commitment_serialize_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn error_serialize_deserialize_roundtrip_portable() {
    test_error_serialize_deserialize_roundtrip::<PortableSIMDUnit>();
}

#[test]
fn t0_serialize_deserialize_roundtrip_portable() {
    test_t0_serialize_deserialize_roundtrip::<PortableSIMDUnit>();
}

#[test]
fn t1_serialize_deserialize_roundtrip_portable() {
    test_t1_serialize_deserialize_roundtrip::<PortableSIMDUnit>();
}

#[test]
fn gamma1_serialize_matches_spec_bit_pack_portable() {
    test_gamma1_serialize_matches_spec_bit_pack::<PortableSIMDUnit>();
}

#[cfg(feature = "simd256")]
mod avx2 {
    use super::*;

    #[test]
    fn gamma1_serialize_deserialize_roundtrip_avx2() {
        test_gamma1_serialize_deserialize_roundtrip::<AVX2SIMDUnit>();
    }

    #[test]
    fn commitment_serialize_matches_spec_avx2() {
        test_commitment_serialize_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn error_serialize_deserialize_roundtrip_avx2() {
        test_error_serialize_deserialize_roundtrip::<AVX2SIMDUnit>();
    }

    #[test]
    fn t0_serialize_deserialize_roundtrip_avx2() {
        test_t0_serialize_deserialize_roundtrip::<AVX2SIMDUnit>();
    }

    #[test]
    fn t1_serialize_deserialize_roundtrip_avx2() {
        test_t1_serialize_deserialize_roundtrip::<AVX2SIMDUnit>();
    }

    #[test]
    fn gamma1_serialize_matches_spec_bit_pack_avx2() {
        test_gamma1_serialize_matches_spec_bit_pack::<AVX2SIMDUnit>();
    }
}
