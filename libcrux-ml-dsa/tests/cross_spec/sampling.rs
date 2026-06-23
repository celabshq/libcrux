//! Cross-spec tests for the rejection-sampling methods of the `Operations`
//! trait.
//!
//! Three tests, one per rejection-sampling variant on the trait:
//!
//!   - `rejection_sample_less_than_field_modulus` (3-byte → 23-bit candidate)
//!   - `rejection_sample_less_than_eta_equals_2` (half-byte → η=2 candidate)
//!   - `rejection_sample_less_than_eta_equals_4` (half-byte → η=4 candidate)
//!
//! Each test feeds a fixed-length deterministic ChaCha20 byte buffer to the
//! impl, captures the returned coefficients, and re-derives the expected
//! sequence per byte using the corresponding hacspec helper
//! (`Encoding::coeff_from_three_bytes` or `Encoding::coeff_from_half_byte`).

#![allow(dead_code, unused_imports, unused_variables)]

use super::helpers::*;
use hacspec_ml_dsa as spec;
#[cfg(feature = "simd256")]
use libcrux_ml_dsa::test_utils::simd::AVX2SIMDUnit;
use libcrux_ml_dsa::test_utils::simd::Operations;
use libcrux_ml_dsa::test_utils::simd::PortableSIMDUnit;
use rand::{Rng, RngExt};

const ITERATIONS: usize = 100;

/// Cross-check `rejection_sample_less_than_field_modulus`:  feed a 24-byte
/// buffer (worst case = 8 candidates × 3 bytes), compare against per-byte
/// `coeff_from_three_bytes`.
pub fn rejection_sample_field_modulus_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x5A1F);
    for _ in 0..ITERATIONS {
        let mut bytes = [0u8; 24];
        rng.fill(&mut bytes);
        let mut out = [0i32; 8];
        let n = O::rejection_sample_less_than_field_modulus(&bytes, &mut out);
        // Re-derive expected via the per-3-byte helper.
        let mut expected: Vec<i32> = Vec::new();
        for chunk in bytes.chunks(3) {
            if chunk.len() < 3 {
                break;
            }
            if let Some(c) = spec::encoding::coeff_from_three_bytes(chunk[0], chunk[1], chunk[2]) {
                expected.push(c);
            }
        }
        assert_eq!(
            n,
            expected.len(),
            "rejection_sample(field_modulus) count mismatch"
        );
        for (i, &c) in expected.iter().enumerate() {
            assert_eq!(
                out[i], c,
                "rejection_sample(field_modulus) coeff {} mismatch",
                i
            );
        }
    }
}

/// Cross-check `rejection_sample_less_than_eta_equals_2`: feed a 4-byte
/// buffer, compare against per-half-byte `coeff_from_half_byte(b, 2)`.
pub fn rejection_sample_eta_2_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x5A02);
    for _ in 0..ITERATIONS {
        let mut bytes = [0u8; 4];
        rng.fill(&mut bytes);
        let mut out = [0i32; 8];
        let n = O::rejection_sample_less_than_eta_equals_2(&bytes, &mut out);
        let mut expected: Vec<i32> = Vec::new();
        for &b in &bytes {
            let lo = b & 0x0F;
            let hi = (b >> 4) & 0x0F;
            for half in [lo, hi] {
                if let Some(c) = spec::encoding::coeff_from_half_byte(half, 2) {
                    expected.push(c);
                }
            }
        }
        assert_eq!(n, expected.len(), "rejection_sample(eta=2) count mismatch");
        for (i, &c) in expected.iter().enumerate() {
            assert_eq!(out[i], c, "rejection_sample(eta=2) coeff {} mismatch", i);
        }
    }
}

/// Cross-check `rejection_sample_less_than_eta_equals_4`: feed a 4-byte
/// buffer, compare against per-half-byte `coeff_from_half_byte(b, 4)`.
pub fn rejection_sample_eta_4_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x5A04);
    for _ in 0..ITERATIONS {
        let mut bytes = [0u8; 4];
        rng.fill(&mut bytes);
        let mut out = [0i32; 8];
        let n = O::rejection_sample_less_than_eta_equals_4(&bytes, &mut out);
        let mut expected: Vec<i32> = Vec::new();
        for &b in &bytes {
            let lo = b & 0x0F;
            let hi = (b >> 4) & 0x0F;
            for half in [lo, hi] {
                if let Some(c) = spec::encoding::coeff_from_half_byte(half, 4) {
                    expected.push(c);
                }
            }
        }
        assert_eq!(n, expected.len(), "rejection_sample(eta=4) count mismatch");
        for (i, &c) in expected.iter().enumerate() {
            assert_eq!(out[i], c, "rejection_sample(eta=4) coeff {} mismatch", i);
        }
    }
}

// ---------------------------------------------------------------------------
// Concrete instantiations.
// ---------------------------------------------------------------------------

#[test]
fn rejection_sample_field_modulus_portable() {
    rejection_sample_field_modulus_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn rejection_sample_eta_2_portable() {
    rejection_sample_eta_2_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn rejection_sample_eta_4_portable() {
    rejection_sample_eta_4_matches_spec::<PortableSIMDUnit>();
}

#[cfg(feature = "simd256")]
mod avx2 {
    use super::*;

    #[test]
    fn rejection_sample_field_modulus_avx2() {
        rejection_sample_field_modulus_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn rejection_sample_eta_2_avx2() {
        rejection_sample_eta_2_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn rejection_sample_eta_4_avx2() {
        rejection_sample_eta_4_matches_spec::<AVX2SIMDUnit>();
    }
}
