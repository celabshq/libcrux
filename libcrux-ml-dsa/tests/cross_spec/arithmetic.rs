//! Cross-spec tests for the arithmetic methods of the `Operations` trait.
//!
//! Each test runs 100 deterministic ChaCha20 iterations and compares the
//! impl's per-lane output against the corresponding `hacspec_ml_dsa`
//! function.

#![allow(dead_code, unused_imports, unused_variables)]

use super::helpers::*;
use hacspec_ml_dsa as spec;
#[cfg(feature = "simd256")]
use libcrux_ml_dsa::test_utils::simd::AVX2SIMDUnit;
use libcrux_ml_dsa::test_utils::simd::Operations;
use libcrux_ml_dsa::test_utils::simd::PortableSIMDUnit;
use rand::{Rng, RngExt};

// Per-test iteration count.
const ITERATIONS: usize = 100;

// γ₂ values from FIPS 204 Table 1.
const GAMMA2_88: i32 = (Q - 1) / 88; // 95_232  (ML-DSA-44)
const GAMMA2_32: i32 = (Q - 1) / 32; // 261_888 (ML-DSA-65, ML-DSA-87)

// β = τ·η bounds (largest β across ML-DSA-44/65/87).
const BETA_44: i32 = 78; // τ=39, η=2
const BETA_65: i32 = 196; // τ=49, η=4
const BETA_87: i32 = 120; // τ=60, η=2

// γ₁ = 2¹⁷ for ML-DSA-44, 2¹⁹ for ML-DSA-65/87.
const GAMMA1_17: i32 = 1 << 17;
const GAMMA1_19: i32 = 1 << 19;

// ---------------------------------------------------------------------------
// Generic test functions (parameterized over an `Operations` impl).
// ---------------------------------------------------------------------------

/// Cross-check `Operations::add` against per-lane `(a + b) mod q`.
///
/// The trait `add` is element-wise i32 add without reduction, so the
/// reference is plain wrapping i32 addition (the precondition keeps the
/// inputs bounded so wrap can't occur).
pub fn test_add_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0xADD0);
    for _ in 0..ITERATIONS {
        let a = random_simd_unit_signed(&mut rng, Q - 1);
        let b = random_simd_unit_signed(&mut rng, Q - 1);
        let mut expected = [0i32; LANES];
        for i in 0..LANES {
            expected[i] = a[i].wrapping_add(b[i]);
        }
        let mut lhs = to_simd_unit::<O>(&a);
        let rhs = to_simd_unit::<O>(&b);
        O::add(&mut lhs, &rhs);
        let got = from_simd_unit::<O>(&lhs);
        assert_eq!(got, expected, "add mismatch");
    }
}

/// Cross-check `Operations::subtract` against per-lane `(a - b) mod q`.
pub fn test_subtract_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x5_B0);
    for _ in 0..ITERATIONS {
        let a = random_simd_unit_signed(&mut rng, Q - 1);
        let b = random_simd_unit_signed(&mut rng, Q - 1);
        let mut expected = [0i32; LANES];
        for i in 0..LANES {
            expected[i] = a[i].wrapping_sub(b[i]);
        }
        let mut lhs = to_simd_unit::<O>(&a);
        let rhs = to_simd_unit::<O>(&b);
        O::subtract(&mut lhs, &rhs);
        let got = from_simd_unit::<O>(&lhs);
        assert_eq!(got, expected, "subtract mismatch");
    }
}

/// Cross-check `Operations::infinity_norm_exceeds` against
/// `max_i |x_i| >= bound`.
pub fn test_infinity_norm_exceeds_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x1F00);
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_signed(&mut rng, GAMMA1_19);
        let bound = random_coefficient(&mut rng, GAMMA1_19);
        let max_abs = coeffs
            .iter()
            .map(|x| x.unsigned_abs() as i32)
            .max()
            .unwrap();
        let expected = max_abs >= bound;
        let unit = to_simd_unit::<O>(&coeffs);
        let got = O::infinity_norm_exceeds(&unit, bound);
        assert_eq!(got, expected, "infinity_norm_exceeds mismatch");
    }
}

/// Cross-check `Operations::decompose` against per-lane
/// `hacspec_ml_dsa::arithmetic::decompose`, including the wraparound
/// case where `r⁺ - r₀ == q - 1`.
pub fn test_decompose_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0xDEC0);
    for &gamma2 in &[GAMMA2_88, GAMMA2_32] {
        for _ in 0..ITERATIONS {
            // Mix of generic random values and near-q values to hit the
            // `r_plus - r0 == q - 1` corner.
            let coeffs = if rng.random::<bool>() {
                random_simd_unit_coeffs(&mut rng, Q)
            } else {
                let mut c = [0i32; LANES];
                for v in c.iter_mut() {
                    *v = Q - 1 - random_coefficient(&mut rng, 100);
                }
                c
            };
            let input = to_simd_unit::<O>(&coeffs);
            let mut low = O::zero();
            let mut high = O::zero();
            O::decompose(gamma2, &input, &mut low, &mut high);
            let got_low = from_simd_unit::<O>(&low);
            let got_high = from_simd_unit::<O>(&high);
            for i in 0..LANES {
                // Spec `decompose` returns (r1, r0); the impl writes r0 to
                // `low` and r1 to `high`.
                let (r1, r0) = spec::arithmetic::decompose(coeffs[i], gamma2);
                assert_eq!(got_low[i], r0, "decompose low (r0) mismatch");
                assert_eq!(got_high[i], r1, "decompose high (r1) mismatch");
            }
        }
    }
}

/// Cross-check `Operations::compute_hint` against the per-lane hint formula.
///
/// The impl's `compute_hint` operates on already-decomposed `(low, high)`
/// values (it is the `ComputeHint`/`MakeHint` step in the `(r0, r1)` domain),
/// which is a different formulation from `hacspec`'s `make_hint(z, r)` that
/// re-derives the decomposition internally.  We therefore cross-check against
/// the documented scalar reference `compute_one_hint`:
///
///   hint = 1  iff  low > γ₂  ∨  low < −γ₂  ∨  (low == −γ₂ ∧ high ≠ 0)
///
/// plus the returned set-bit popcount.
pub fn test_compute_hint_matches_spec<O: Operations>() {
    fn compute_one_hint(low: i32, high: i32, gamma2: i32) -> bool {
        low > gamma2 || low < -gamma2 || (low == -gamma2 && high != 0)
    }
    let mut rng = seeded_rng(0xC417);
    for &gamma2 in &[GAMMA2_88, GAMMA2_32] {
        for _ in 0..ITERATIONS {
            let low = random_simd_unit_signed(&mut rng, gamma2);
            let high = random_simd_unit_coeffs(&mut rng, Q);
            let low_unit = to_simd_unit::<O>(&low);
            let high_unit = to_simd_unit::<O>(&high);
            let mut hint = O::zero();
            let count = O::compute_hint(&low_unit, &high_unit, gamma2, &mut hint);
            let hint_arr = from_simd_unit::<O>(&hint);

            let mut expected_count = 0usize;
            for i in 0..LANES {
                let h = compute_one_hint(low[i], high[i], gamma2);
                assert_eq!(hint_arr[i] != 0, h, "hint bit mismatch lane {}", i);
                if h {
                    expected_count += 1;
                }
            }
            assert_eq!(count, expected_count, "hint popcount mismatch");
        }
    }
}

/// Cross-check `Operations::use_hint` against per-lane
/// `hacspec_ml_dsa::arithmetic::use_hint`.
pub fn test_use_hint_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x05E1);
    for &gamma2 in &[GAMMA2_88, GAMMA2_32] {
        for _ in 0..ITERATIONS {
            let r = random_simd_unit_coeffs(&mut rng, Q);
            let hint: [i32; LANES] = core::array::from_fn(|_| rng.random_range(0..2));
            let unit = to_simd_unit::<O>(&r);
            let mut hint_unit = to_simd_unit::<O>(&hint);
            O::use_hint(gamma2, &unit, &mut hint_unit);
            let got = from_simd_unit::<O>(&hint_unit);
            for i in 0..LANES {
                let expected = spec::arithmetic::use_hint(hint[i] != 0, r[i], gamma2);
                assert_eq!(got[i], expected, "use_hint mismatch");
            }
        }
    }
}

/// Cross-check `Operations::power2round` against per-lane
/// `hacspec_ml_dsa::arithmetic::power2round`.
pub fn test_power2round_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x9024);
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_coeffs(&mut rng, Q);
        let mut t0 = to_simd_unit::<O>(&coeffs);
        let mut t1 = O::zero();
        O::power2round(&mut t0, &mut t1);
        let got_t0 = from_simd_unit::<O>(&t0);
        let got_t1 = from_simd_unit::<O>(&t1);
        for i in 0..LANES {
            // Spec `power2round` returns (r1, r0); the impl writes r0 to `t0`
            // and r1 to `t1`.
            let (r1, r0) = spec::arithmetic::power2round(coeffs[i]);
            assert_eq!(got_t1[i], r1, "power2round t1 mismatch");
            assert_eq!(got_t0[i], r0, "power2round t0 mismatch");
        }
    }
}

/// Cross-check `Operations::montgomery_multiply` against per-lane
/// `mod_q(a · b · R⁻¹)`.
pub fn test_montgomery_multiply_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0xA0A1);
    for _ in 0..ITERATIONS {
        let a = random_simd_unit_signed(&mut rng, Q - 1);
        let b = random_simd_unit_signed(&mut rng, Q - 1);
        let mut expected_mod_q = [0i32; LANES];
        for i in 0..LANES {
            // a · b · R⁻¹ mod q. `hacspec_ml_dsa` has no Montgomery helper
            // (it works in the standard domain), so the reference is the
            // local mod-q reduction. Reduce the product first to stay in i64.
            let prod_mod_q = mod_q_local((a[i] as i64) * (b[i] as i64));
            expected_mod_q[i] = mod_q_local((prod_mod_q as i64) * R_INV);
        }
        let mut lhs = to_simd_unit::<O>(&a);
        let rhs = to_simd_unit::<O>(&b);
        O::montgomery_multiply(&mut lhs, &rhs);
        let got = from_simd_unit::<O>(&lhs);
        for i in 0..LANES {
            // The trait post-condition is mod-q equivalence, not equality.
            assert_eq!(
                mod_q_local(got[i] as i64),
                expected_mod_q[i],
                "montgomery_multiply mismatch"
            );
        }
    }
}

/// Cross-check `Operations::shift_left_then_reduce::<13>` against per-lane
/// `mod_q(a · 2¹³)` (precondition `0 ≤ a ≤ 261_631`).
pub fn test_shift_left_then_reduce_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x5414);
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_coeffs(&mut rng, 261_631 + 1);
        let mut expected = [0i32; LANES];
        for i in 0..LANES {
            expected[i] = mod_q_local((coeffs[i] as i64) << 13);
        }
        let mut unit = to_simd_unit::<O>(&coeffs);
        O::shift_left_then_reduce::<13>(&mut unit);
        let got = from_simd_unit::<O>(&unit);
        for i in 0..LANES {
            // The trait post-condition is mod-q equivalence, not equality.
            assert_eq!(
                mod_q_local(got[i] as i64),
                expected[i],
                "shift_left_then_reduce mismatch"
            );
        }
    }
}

/// Cross-check `Operations::barrett_reduce_simd_unit` against `mod_q`.
///
/// The trait has no whole-polynomial `reduce`; the reduction primitive is
/// `barrett_reduce_simd_unit`, applied here to each of the 32 SIMD units.
/// Inputs are kept within the Barrett input bound (`|x| ≤ 2_143_289_343`).
/// The result is a representative in `[-(q-1), q-1]` (not canonical `[0, q)`),
/// so the check is mod-q equivalence only.
pub fn test_reduce_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x4ED0);
    for _ in 0..ITERATIONS {
        let coeffs = random_polynomial_signed(&mut rng, i32::MAX / 2);
        let mut units = to_polynomial_units::<O>(&coeffs);
        for unit in units.iter_mut() {
            O::barrett_reduce_simd_unit(unit);
        }
        let got = from_polynomial_units::<O>(&units);
        for i in 0..N {
            assert_eq!(
                mod_q_local(got[i] as i64),
                mod_q_local(coeffs[i] as i64),
                "barrett_reduce mod_q mismatch coeff {}",
                i
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Concrete instantiations.
// ---------------------------------------------------------------------------

#[test]
fn test_add_portable_matches_spec() {
    test_add_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_subtract_portable_matches_spec() {
    test_subtract_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_infinity_norm_exceeds_portable_matches_spec() {
    test_infinity_norm_exceeds_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_decompose_portable_matches_spec() {
    test_decompose_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_compute_hint_portable_matches_spec() {
    test_compute_hint_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_use_hint_portable_matches_spec() {
    test_use_hint_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_power2round_portable_matches_spec() {
    test_power2round_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_montgomery_multiply_portable_matches_spec() {
    test_montgomery_multiply_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_shift_left_then_reduce_portable_matches_spec() {
    test_shift_left_then_reduce_matches_spec::<PortableSIMDUnit>();
}

#[test]
fn test_reduce_portable_matches_spec() {
    test_reduce_matches_spec::<PortableSIMDUnit>();
}

// AVX2 variants — gated on feature so the tests only run when the AVX2
// impl is in scope.
#[cfg(feature = "simd256")]
mod avx2 {
    use super::*;

    #[test]
    fn test_add_avx2_matches_spec() {
        test_add_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_subtract_avx2_matches_spec() {
        test_subtract_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_infinity_norm_exceeds_avx2_matches_spec() {
        test_infinity_norm_exceeds_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_decompose_avx2_matches_spec() {
        test_decompose_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_compute_hint_avx2_matches_spec() {
        test_compute_hint_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_use_hint_avx2_matches_spec() {
        test_use_hint_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_power2round_avx2_matches_spec() {
        test_power2round_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_montgomery_multiply_avx2_matches_spec() {
        test_montgomery_multiply_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_shift_left_then_reduce_avx2_matches_spec() {
        test_shift_left_then_reduce_matches_spec::<AVX2SIMDUnit>();
    }

    #[test]
    fn test_reduce_avx2_matches_spec() {
        test_reduce_matches_spec::<AVX2SIMDUnit>();
    }
}
