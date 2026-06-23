//! Cross-spec tests for the NTT methods of the `Operations` trait.
//!
//! Two tests:
//!
//!   - `ntt_then_invert_ntt_roundtrip`: random polynomial p with all
//!     coefficients in `[0, q-1]`; check that
//!     `invert_ntt_montgomery(ntt(p)) ≡ p (mod q)` per coefficient.
//!   - `ntt_matches_spec`: apply trait `ntt` to a random polynomial,
//!     compare flat-256 against `hacspec_ml_dsa::ntt::ntt`.

#![allow(dead_code, unused_imports, unused_variables)]

use super::helpers::*;
use hacspec_ml_dsa as spec;
#[cfg(feature = "simd256")]
use libcrux_ml_dsa::test_utils::simd::AVX2SIMDUnit;
use libcrux_ml_dsa::test_utils::simd::Operations;
use libcrux_ml_dsa::test_utils::simd::PortableSIMDUnit;
use rand::Rng;

const ITERATIONS: usize = 100;

// The impl's `ntt` requires inputs bounded by `NTT_BASE_BOUND = (q-1)/2`.
const NTT_INPUT_BOUND: i32 = (Q - 1) / 2;

// R = 2³² mod q, the Montgomery constant.  `invert_ntt_montgomery` exits in
// the Montgomery domain, so `invert_ntt_montgomery(ntt(p)) ≡ p · R (mod q)`.
const R_MOD_Q: i64 = 4_193_792;

/// `invert_ntt_montgomery(ntt(p)) ≡ p (mod q)` per coefficient.
///
/// Note: the Montgomery exit absorbs an extra R⁻¹ factor in the impl,
/// so the equality is `mod_q`-equivalent rather than equal.  In the
/// hacspec, `intt` already produces standard-domain output; the
/// libcrux `invert_ntt_montgomery` exits via Montgomery so the
/// coefficient values may differ by a Montgomery factor.  The check
/// below is mod-q equivalence.
pub fn ntt_then_invert_ntt_roundtrip<O: Operations>() {
    let mut rng = seeded_rng(0x4711);
    for _ in 0..ITERATIONS {
        let p = random_polynomial_coeffs(&mut rng, NTT_INPUT_BOUND);
        let mut units = to_polynomial_units::<O>(&p);
        O::ntt(&mut units);
        O::invert_ntt_montgomery(&mut units);
        let got = from_polynomial_units::<O>(&units);
        for i in 0..N {
            assert_eq!(
                mod_q_local(got[i] as i64),
                mod_q_local(p[i] as i64 * R_MOD_Q),
                "ntt-intt mismatch at coeff {}: got {}, expected p·R = {}",
                i,
                got[i],
                mod_q_local(p[i] as i64 * R_MOD_Q)
            );
        }
    }
}

/// `Operations::ntt(p)` flat-256 matches `hacspec_ml_dsa::ntt::ntt(p)`
/// (mod q per coefficient).
pub fn ntt_matches_spec<O: Operations>() {
    let mut rng = seeded_rng(0x4774);
    for _ in 0..ITERATIONS {
        let p = random_polynomial_coeffs(&mut rng, NTT_INPUT_BOUND);
        let mut units = to_polynomial_units::<O>(&p);
        O::ntt(&mut units);
        let got = from_polynomial_units::<O>(&units);
        let expected = spec::ntt::ntt(p);
        for i in 0..N {
            assert_eq!(
                mod_q_local(got[i] as i64),
                mod_q_local(expected[i] as i64),
                "ntt vs spec mismatch at coeff {}: got {}, expected {}",
                i,
                got[i],
                expected[i]
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Concrete instantiations.
// ---------------------------------------------------------------------------

#[test]
fn ntt_then_invert_ntt_roundtrip_portable() {
    ntt_then_invert_ntt_roundtrip::<PortableSIMDUnit>();
}

#[test]
fn ntt_matches_spec_portable() {
    ntt_matches_spec::<PortableSIMDUnit>();
}

#[cfg(feature = "simd256")]
mod avx2 {
    use super::*;

    #[test]
    fn ntt_then_invert_ntt_roundtrip_avx2() {
        ntt_then_invert_ntt_roundtrip::<AVX2SIMDUnit>();
    }

    #[test]
    fn ntt_matches_spec_avx2() {
        ntt_matches_spec::<AVX2SIMDUnit>();
    }
}
