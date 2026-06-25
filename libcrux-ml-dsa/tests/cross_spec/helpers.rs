//! Shared helpers for cross-spec tests: deterministic RNG seeding, random
//! coefficient/polynomial generators, and (eventually) lane <-> scalar
//! adapters between the libcrux SIMD impls and a flat `[i32; 256]`.
//!
//! ## Accessibility note
//!
//! The `from_simd_unit` / `to_simd_unit` / `from_polynomial` / `to_polynomial`
//! adapters are commented out below: they require either
//! `libcrux_ml_dsa::simd::traits::Operations` (currently `pub(crate)`)
//! or `libcrux_ml_dsa::polynomial::PolynomialRingElement` (also
//! `pub(crate)`) to be reachable from external integration tests.
//!
//! To unblock, re-export under a `test-utils` (or `cross-spec-tests`)
//! feature in `libcrux-ml-dsa/src/lib.rs`:
//!
//! ```rust,ignore
//! #[cfg(feature = "test-utils")]
//! pub mod simd_test {
//!     pub use crate::simd::traits::{Operations, COEFFICIENTS_IN_SIMD_UNIT,
//!                                    SIMD_UNITS_IN_RING_ELEMENT};
//!     pub use crate::simd::portable::PortableSIMDUnit;
//!     #[cfg(feature = "simd256")]
//!     pub use crate::simd::avx2::AVX2SIMDUnit;
//! }
//! #[cfg(feature = "test-utils")]
//! pub mod polynomial_test { pub use crate::polynomial::PolynomialRingElement; }
//! ```
//!
//! Once this re-export lands, swap the `// use libcrux_ml_dsa::...` lines
//! below for the actual `use` statement and delete the TODO blocks.

#![allow(dead_code, unused_imports)]

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

// FIPS 204 q.
pub const Q: i32 = 8_380_417;
// Per-SIMD-unit lane count.
pub const LANES: usize = 8;
// Per-polynomial coefficient count.
pub const N: usize = 256;
// SIMD units per polynomial: 256 / 8.
pub const SIMD_UNITS: usize = 32;
// R⁻¹ mod q where R = 2³² (Montgomery constant).
pub const R_INV: i64 = 8_265_825;
// 2^D where D = 13.
pub const TWO_D: i32 = 1 << 13;

/// Deterministic ChaCha20 RNG seeded with `seed`.  All tests that fuzz
/// must use this so failures are reproducible.
pub fn seeded_rng(seed: u64) -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(seed)
}

/// Sample a uniform i32 in `[0, max)`.
pub fn random_coefficient(rng: &mut impl Rng, max: i32) -> i32 {
    assert!(max > 0);
    rng.random_range(0..max)
}

/// Sample a uniform i32 in `[-bound, bound]`.
pub fn random_signed_coefficient(rng: &mut impl Rng, bound: i32) -> i32 {
    assert!(bound > 0);
    rng.random_range(-bound..=bound)
}

/// Sample 8 coefficients in `[0, max)` for a single SIMD unit.
pub fn random_simd_unit_coeffs(rng: &mut impl Rng, max: i32) -> [i32; LANES] {
    let mut out = [0i32; LANES];
    for v in out.iter_mut() {
        *v = random_coefficient(rng, max);
    }
    out
}

/// Sample 256 coefficients in `[0, max)` for a polynomial.
pub fn random_polynomial_coeffs(rng: &mut impl Rng, max: i32) -> [i32; N] {
    let mut out = [0i32; N];
    for v in out.iter_mut() {
        *v = random_coefficient(rng, max);
    }
    out
}

/// Sample 8 *signed* coefficients in `[-bound, bound]` for a single SIMD unit.
pub fn random_simd_unit_signed(rng: &mut impl Rng, bound: i32) -> [i32; LANES] {
    let mut out = [0i32; LANES];
    for v in out.iter_mut() {
        *v = random_signed_coefficient(rng, bound);
    }
    out
}

/// Sample 256 *signed* coefficients in `[-bound, bound]` for a polynomial.
pub fn random_polynomial_signed(rng: &mut impl Rng, bound: i32) -> [i32; N] {
    let mut out = [0i32; N];
    for v in out.iter_mut() {
        *v = random_signed_coefficient(rng, bound);
    }
    out
}

// ---------------------------------------------------------------------------
// SIMD <-> scalar adapters (TODO — gated on accessibility of `Operations`).
// ---------------------------------------------------------------------------
//
// The following adapters are blocked on:
//
// 1. `libcrux_ml_dsa::simd::traits::Operations` becoming reachable
//    (currently `pub(crate)`).
// 2. The concrete impl types `PortableSIMDUnit` and `AVX2SIMDUnit`
//    becoming reachable.
// 3. `libcrux_ml_dsa::polynomial::PolynomialRingElement` becoming
//    reachable for the polynomial-level adapters.
//
// Sketch of the intended API:
//
// ```rust,ignore
// use libcrux_ml_dsa::simd::traits::Operations;
//
// pub fn from_simd_unit<O: Operations>(unit: &O) -> [i32; LANES] {
//     let mut out = [0i32; LANES];
//     O::to_coefficient_array(unit, &mut out);
//     out
// }
//
// pub fn to_simd_unit<O: Operations>(coeffs: &[i32; LANES]) -> O {
//     let mut out = O::zero();
//     O::from_coefficient_array(coeffs, &mut out);
//     out
// }
//
// pub fn from_polynomial<O: Operations>(
//     p: &libcrux_ml_dsa::polynomial::PolynomialRingElement<O>,
// ) -> [i32; N] {
//     p.to_i32_array()
// }
//
// pub fn to_polynomial<O: Operations>(
//     coeffs: &[i32; N],
// ) -> libcrux_ml_dsa::polynomial::PolynomialRingElement<O> {
//     let mut out = libcrux_ml_dsa::polynomial::PolynomialRingElement::<O>::zero();
//     libcrux_ml_dsa::polynomial::PolynomialRingElement::<O>::from_i32_array(coeffs, &mut out);
//     out
// }
// ```

/// Reduce a value to `[0, q)`.  Spec helper for cross-checks where the
/// hacspec mod_q function is unreachable.
pub fn mod_q_local(a: i64) -> i32 {
    let r = (a % Q as i64) as i32;
    if r < 0 {
        r + Q
    } else {
        r
    }
}

/// Centered modular reduction: result in `(-m/2, m/2]`.
pub fn mod_pm_local(a: i32, m: i32) -> i32 {
    assert!(m > 0);
    let a64 = a as i64;
    let m64 = m as i64;
    let r = ((a64 % m64) + m64) % m64;
    let r = r as i32;
    if r > m / 2 {
        r - m
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_is_deterministic() {
        let mut a = seeded_rng(42);
        let mut b = seeded_rng(42);
        let xa = random_coefficient(&mut a, 1000);
        let xb = random_coefficient(&mut b, 1000);
        assert_eq!(xa, xb, "ChaCha20 with the same seed must be deterministic");
    }

    #[test]
    fn signed_range_respected() {
        let mut rng = seeded_rng(7);
        for _ in 0..100 {
            let v = random_signed_coefficient(&mut rng, 100);
            assert!(v >= -100 && v <= 100);
        }
    }

    #[test]
    fn unsigned_range_respected() {
        let mut rng = seeded_rng(11);
        for _ in 0..100 {
            let v = random_coefficient(&mut rng, Q);
            assert!(v >= 0 && v < Q);
        }
    }

    #[test]
    fn mod_q_local_normalizes() {
        assert_eq!(mod_q_local(0), 0);
        assert_eq!(mod_q_local(Q as i64 - 1), Q - 1);
        assert_eq!(mod_q_local(Q as i64), 0);
        assert_eq!(mod_q_local(-1), Q - 1);
    }

    #[test]
    fn mod_pm_local_centers() {
        assert_eq!(mod_pm_local(0, 8), 0);
        assert_eq!(mod_pm_local(4, 8), 4);
        assert_eq!(mod_pm_local(5, 8), -3);
        assert_eq!(mod_pm_local(-3, 8), -3);
    }
}
