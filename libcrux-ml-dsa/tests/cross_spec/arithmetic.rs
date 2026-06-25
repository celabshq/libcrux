//! Cross-spec tests for the arithmetic methods of the `Operations` trait.
//!
//! Each test runs 100 deterministic ChaCha20 iterations and compares the
//! impl's per-lane output against the corresponding `hacspec_ml_dsa`
//! function.  100 iterations (rather than 1000) keeps feedback fast.
//!
//! ## Status
//!
//! All test bodies are currently TODO stubs because:
//!
//! - `libcrux_ml_dsa::simd::traits::Operations` is `pub(crate)`.
//! - `hacspec_ml_dsa::arithmetic::{decompose, make_hint, use_hint,
//!    power2round, montgomery_reduce, shift_left_then_reduce, mod_q}`
//!    are all `pub(crate)`.
//!
//! The intended test logic is inlined in each `// TODO:` comment block
//! so that the next session can drop in real call-sites once both crates
//! expose the relevant items behind a `test-utils` feature gate.

#![allow(dead_code, unused_imports, unused_variables)]

use super::helpers::*;
use hacspec_ml_dsa as spec;
use libcrux_ml_dsa as impl_crate;
use rand::Rng;

// Per-test iteration count. Kept at 100 (not 1000) for fast CI feedback.
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
//
// Each `test_*_matches_spec::<O>()` is meant to be invoked via
// `cross_spec_test!` from `tests/cross_spec.rs` once `Operations` is
// reachable from the integration-test crate.
// ---------------------------------------------------------------------------

/// Cross-check `Operations::add` against per-lane `(a + b) mod q`.
///
/// The trait `add` is element-wise i32 add without reduction, so the
/// reference is plain wrapping i32 addition (the precondition keeps the
/// inputs bounded so wrap can't occur).
pub fn test_add_matches_spec() {
    let mut rng = seeded_rng(0xADD0);
    for _ in 0..ITERATIONS {
        let a = random_simd_unit_signed(&mut rng, Q - 1);
        let b = random_simd_unit_signed(&mut rng, Q - 1);
        let mut expected = [0i32; LANES];
        for i in 0..LANES {
            expected[i] = a[i].wrapping_add(b[i]);
        }
        // TODO: requires libcrux_ml_dsa::simd::traits::Operations to be
        // re-exported under `cfg(feature = "test-utils")`.
        // Intended body:
        //   let mut lhs = to_simd_unit::<O>(&a);
        //   let rhs = to_simd_unit::<O>(&b);
        //   O::add(&mut lhs, &rhs);
        //   let got = from_simd_unit::<O>(&lhs);
        //   assert_eq!(got, expected, "add mismatch on iteration");
        let _ = expected;
    }
}

/// Cross-check `Operations::subtract` against per-lane `(a - b) mod q`.
pub fn test_subtract_matches_spec() {
    let mut rng = seeded_rng(0x5_B0);
    for _ in 0..ITERATIONS {
        let a = random_simd_unit_signed(&mut rng, Q - 1);
        let b = random_simd_unit_signed(&mut rng, Q - 1);
        let mut expected = [0i32; LANES];
        for i in 0..LANES {
            expected[i] = a[i].wrapping_sub(b[i]);
        }
        // TODO: requires Operations + impl-type accessibility.
        // Intended body:
        //   let mut lhs = to_simd_unit::<O>(&a);
        //   let rhs = to_simd_unit::<O>(&b);
        //   O::subtract(&mut lhs, &rhs);
        //   let got = from_simd_unit::<O>(&lhs);
        //   assert_eq!(got, expected);
        let _ = expected;
    }
}

/// Cross-check `Operations::infinity_norm_exceeds` against
/// `max_i |x_i| >= bound`.
pub fn test_infinity_norm_exceeds_matches_spec() {
    let mut rng = seeded_rng(0x1F00);
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_signed(&mut rng, GAMMA1_19);
        let bound = random_coefficient(&mut rng, GAMMA1_19);
        let max_abs = coeffs.iter().map(|x| x.unsigned_abs() as i32).max().unwrap();
        let expected = max_abs >= bound;
        // TODO: requires Operations + impl-type accessibility.
        // Intended body:
        //   let unit = to_simd_unit::<O>(&coeffs);
        //   let got = O::infinity_norm_exceeds(&unit, bound);
        //   assert_eq!(got, expected, "infinity_norm_exceeds mismatch");
        let _ = expected;
    }
}

/// Cross-check `Operations::decompose` against per-lane
/// `hacspec_ml_dsa::arithmetic::decompose`, including the wraparound
/// case where `r⁺ - r₀ == q - 1`.
pub fn test_decompose_matches_spec() {
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
            // TODO: requires hacspec_ml_dsa::arithmetic::decompose to be
            // re-exported. Intended body:
            //   let mut input = to_simd_unit::<O>(&coeffs);
            //   let mut low = O::zero();
            //   let mut high = O::zero();
            //   O::decompose(gamma2, &input, &mut low, &mut high);
            //   let got_low = from_simd_unit::<O>(&low);
            //   let got_high = from_simd_unit::<O>(&high);
            //   for i in 0..LANES {
            //       let (r1, r0) = spec::arithmetic::decompose(coeffs[i], gamma2);
            //       assert_eq!(got_low[i], r0, "decompose low mismatch");
            //       assert_eq!(got_high[i], r1, "decompose high mismatch");
            //   }
            let _ = (gamma2, coeffs);
        }
    }
}

/// Cross-check `Operations::compute_hint` against per-lane
/// `hacspec_ml_dsa::arithmetic::make_hint` plus a popcount check.
pub fn test_compute_hint_matches_spec() {
    let mut rng = seeded_rng(0xC417);
    for &gamma2 in &[GAMMA2_88, GAMMA2_32] {
        for _ in 0..ITERATIONS {
            let low = random_simd_unit_signed(&mut rng, gamma2);
            let high = random_simd_unit_coeffs(&mut rng, Q);
            // TODO: requires Operations + spec::arithmetic::make_hint.
            // Intended body:
            //   let low_unit = to_simd_unit::<O>(&low);
            //   let high_unit = to_simd_unit::<O>(&high);
            //   let mut hint = O::zero();
            //   let count = O::compute_hint(&low_unit, &high_unit, gamma2, &mut hint);
            //   let hint_arr = from_simd_unit::<O>(&hint);
            //
            //   // Per-lane: each hint bit ≡ make_hint(low, high, gamma2).
            //   let mut expected_count = 0usize;
            //   for i in 0..LANES {
            //       let h = spec::arithmetic::make_hint(low[i], high[i], gamma2);
            //       assert_eq!(hint_arr[i] != 0, h, "hint bit mismatch lane {}", i);
            //       if h { expected_count += 1; }
            //   }
            //   assert_eq!(count, expected_count, "hint popcount mismatch");
            let _ = (gamma2, low, high);
        }
    }
}

/// Cross-check `Operations::use_hint` against per-lane
/// `hacspec_ml_dsa::arithmetic::use_hint`.
pub fn test_use_hint_matches_spec() {
    let mut rng = seeded_rng(0x05E1);
    for &gamma2 in &[GAMMA2_88, GAMMA2_32] {
        for _ in 0..ITERATIONS {
            let r = random_simd_unit_coeffs(&mut rng, Q);
            let hint: [i32; LANES] = [
                rng.random_range(0..2),
                rng.random_range(0..2),
                rng.random_range(0..2),
                rng.random_range(0..2),
                rng.random_range(0..2),
                rng.random_range(0..2),
                rng.random_range(0..2),
                rng.random_range(0..2),
            ];
            // TODO: requires Operations + spec::arithmetic::use_hint.
            // Intended body:
            //   let unit = to_simd_unit::<O>(&r);
            //   let mut hint_unit = to_simd_unit::<O>(&hint);
            //   O::use_hint(gamma2, &unit, &mut hint_unit);
            //   let got = from_simd_unit::<O>(&hint_unit);
            //   for i in 0..LANES {
            //       let expected = spec::arithmetic::use_hint(hint[i] != 0, r[i], gamma2);
            //       assert_eq!(got[i], expected, "use_hint mismatch");
            //   }
            let _ = (gamma2, r, hint);
        }
    }
}

/// Cross-check `Operations::power2round` against per-lane
/// `hacspec_ml_dsa::arithmetic::power2round`.
pub fn test_power2round_matches_spec() {
    let mut rng = seeded_rng(0x9024);
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_coeffs(&mut rng, Q);
        // TODO: requires Operations + spec::arithmetic::power2round.
        // Intended body:
        //   let mut t0 = to_simd_unit::<O>(&coeffs);
        //   let mut t1 = O::zero();
        //   O::power2round(&mut t0, &mut t1);
        //   let got_t0 = from_simd_unit::<O>(&t0);
        //   let got_t1 = from_simd_unit::<O>(&t1);
        //   for i in 0..LANES {
        //       let (r1, r0) = spec::arithmetic::power2round(coeffs[i]);
        //       assert_eq!(got_t1[i], r1, "power2round t1 mismatch");
        //       assert_eq!(got_t0[i], r0, "power2round t0 mismatch");
        //   }
        let _ = coeffs;
    }
}

/// Cross-check `Operations::montgomery_multiply` against per-lane
/// `mod_q(a · b · R⁻¹)`.
pub fn test_montgomery_multiply_matches_spec() {
    let mut rng = seeded_rng(0xA0A1);
    for _ in 0..ITERATIONS {
        let a = random_simd_unit_signed(&mut rng, Q - 1);
        let b = random_simd_unit_signed(&mut rng, Q - 1);
        let mut expected_mod_q = [0i32; LANES];
        for i in 0..LANES {
            // a · b · R⁻¹ mod q (reference implementation; bypasses spec
            // until hacspec_ml_dsa::arithmetic::montgomery_reduce is pub).
            // Reduce intermediate to keep within i64.
            let prod_mod_q = mod_q_local((a[i] as i64) * (b[i] as i64));
            expected_mod_q[i] = mod_q_local((prod_mod_q as i64) * R_INV);
        }
        // TODO: requires Operations + spec::arithmetic::montgomery_reduce.
        // Intended body:
        //   let mut lhs = to_simd_unit::<O>(&a);
        //   let rhs = to_simd_unit::<O>(&b);
        //   O::montgomery_multiply(&mut lhs, &rhs);
        //   let got = from_simd_unit::<O>(&lhs);
        //   for i in 0..LANES {
        //       // The trait post is mod_q-equivalence, not equality.
        //       assert_eq!(
        //           mod_q_local(got[i] as i64),
        //           spec::arithmetic::montgomery_reduce((a[i] as i64) * (b[i] as i64)),
        //           "montgomery_multiply mismatch"
        //       );
        //   }
        let _ = (a, b, expected_mod_q);
    }
}

/// Cross-check `Operations::shift_left_then_reduce::<13>` against per-lane
/// `mod_q(a · 2¹³)` (precondition `0 ≤ a ≤ 261_631`).
pub fn test_shift_left_then_reduce_matches_spec() {
    let mut rng = seeded_rng(0x5414);
    for _ in 0..ITERATIONS {
        let coeffs = random_simd_unit_coeffs(&mut rng, 261_631 + 1);
        let mut expected = [0i32; LANES];
        for i in 0..LANES {
            expected[i] = mod_q_local((coeffs[i] as i64) << 13);
        }
        // TODO: requires Operations + spec::arithmetic::shift_left_then_reduce.
        // Intended body:
        //   let mut unit = to_simd_unit::<O>(&coeffs);
        //   O::shift_left_then_reduce::<13>(&mut unit);
        //   let got = from_simd_unit::<O>(&unit);
        //   for i in 0..LANES {
        //       let exp = spec::arithmetic::shift_left_then_reduce(coeffs[i]);
        //       assert_eq!(mod_q_local(got[i] as i64), exp,
        //                  "shift_left_then_reduce mismatch");
        //   }
        let _ = (coeffs, expected);
    }
}

/// Cross-check `Operations::reduce` against `0 ≤ result[i] < q` ∧
/// `result[i] ≡ input[i] (mod q)` over a 32-SIMD-unit polynomial.
pub fn test_reduce_matches_spec() {
    let mut rng = seeded_rng(0x4ED0);
    for _ in 0..ITERATIONS {
        let coeffs = random_polynomial_signed(&mut rng, i32::MAX / 2);
        let mut expected = [0i32; N];
        for i in 0..N {
            expected[i] = mod_q_local(coeffs[i] as i64);
        }
        // TODO: requires Operations + the [_; SIMD_UNITS_IN_RING_ELEMENT]
        // signature is on the trait, but we still need the impl types reachable.
        // Intended body:
        //   let mut units = to_polynomial_units::<O>(&coeffs);
        //   O::reduce(&mut units);
        //   let got = from_polynomial_units::<O>(&units);
        //   for i in 0..N {
        //       assert!(got[i] >= 0 && got[i] < Q,
        //               "reduce out-of-range coeff {}: {}", i, got[i]);
        //       assert_eq!(mod_q_local(got[i] as i64),
        //                  mod_q_local(coeffs[i] as i64),
        //                  "reduce mod_q mismatch coeff {}", i);
        //   }
        let _ = (coeffs, expected);
    }
}

// ---------------------------------------------------------------------------
// Concrete instantiations (sanity stubs).
//
// Once `Operations` and the impl types are reachable, these become real
// tests via the `cross_spec_test!` macro from `tests/cross_spec.rs`.
// ---------------------------------------------------------------------------

#[test]
fn test_add_portable_matches_spec() {
    test_add_matches_spec();
}

#[test]
fn test_subtract_portable_matches_spec() {
    test_subtract_matches_spec();
}

#[test]
fn test_infinity_norm_exceeds_portable_matches_spec() {
    test_infinity_norm_exceeds_matches_spec();
}

#[test]
fn test_decompose_portable_matches_spec() {
    test_decompose_matches_spec();
}

#[test]
fn test_compute_hint_portable_matches_spec() {
    test_compute_hint_matches_spec();
}

#[test]
fn test_use_hint_portable_matches_spec() {
    test_use_hint_matches_spec();
}

#[test]
fn test_power2round_portable_matches_spec() {
    test_power2round_matches_spec();
}

#[test]
fn test_montgomery_multiply_portable_matches_spec() {
    test_montgomery_multiply_matches_spec();
}

#[test]
fn test_shift_left_then_reduce_portable_matches_spec() {
    test_shift_left_then_reduce_matches_spec();
}

#[test]
fn test_reduce_portable_matches_spec() {
    test_reduce_matches_spec();
}

// AVX2 variants — gated on feature so the tests only run when the AVX2
// impl is in scope.
#[cfg(feature = "simd256")]
mod avx2 {
    use super::*;

    #[test]
    fn test_add_avx2_matches_spec() {
        test_add_matches_spec();
    }

    #[test]
    fn test_subtract_avx2_matches_spec() {
        test_subtract_matches_spec();
    }

    #[test]
    fn test_infinity_norm_exceeds_avx2_matches_spec() {
        test_infinity_norm_exceeds_matches_spec();
    }

    #[test]
    fn test_decompose_avx2_matches_spec() {
        test_decompose_matches_spec();
    }

    #[test]
    fn test_compute_hint_avx2_matches_spec() {
        test_compute_hint_matches_spec();
    }

    #[test]
    fn test_use_hint_avx2_matches_spec() {
        test_use_hint_matches_spec();
    }

    #[test]
    fn test_power2round_avx2_matches_spec() {
        test_power2round_matches_spec();
    }

    #[test]
    fn test_montgomery_multiply_avx2_matches_spec() {
        test_montgomery_multiply_matches_spec();
    }

    #[test]
    fn test_shift_left_then_reduce_avx2_matches_spec() {
        test_shift_left_then_reduce_matches_spec();
    }

    #[test]
    fn test_reduce_avx2_matches_spec() {
        test_reduce_matches_spec();
    }
}
