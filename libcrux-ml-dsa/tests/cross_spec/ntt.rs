//! Cross-spec tests for the NTT methods of the `Operations` trait.
//!
//! Two tests:
//!
//!   - `ntt_then_invert_ntt_roundtrip`: random polynomial p with all
//!     coefficients in `[0, q-1]`; check that
//!     `invert_ntt_montgomery(ntt(p)) ≡ p (mod q)` per coefficient.
//!   - `ntt_matches_spec`: apply trait `ntt` to a random polynomial,
//!     compare flat-256 against `hacspec_ml_dsa::ntt::ntt`.
//!
//! ## Status
//!
//! Both bodies are TODO stubs because:
//!
//!   - `libcrux_ml_dsa::simd::traits::Operations` is `pub(crate)`.
//!   - `libcrux_ml_dsa::polynomial::PolynomialRingElement` is `pub(crate)`.
//!   - `hacspec_ml_dsa::ntt::ntt` (and `intt`) is `pub(crate)`.
//!
//! Once both are exposed via `test-utils`, drop the body in each
//! `// TODO:` comment.

#![allow(dead_code, unused_imports, unused_variables)]

use super::helpers::*;
use hacspec_ml_dsa as spec;
use libcrux_ml_dsa as impl_crate;
use rand::Rng;

const ITERATIONS: usize = 100;

/// `invert_ntt_montgomery(ntt(p)) ≡ p (mod q)` per coefficient.
///
/// Note: the Montgomery exit absorbs an extra R⁻¹ factor in the impl,
/// so the equality is `mod_q`-equivalent rather than equal.  In the
/// hacspec, `intt` already produces standard-domain output; the
/// libcrux `invert_ntt_montgomery` exits via Montgomery so the
/// coefficient values may differ by a Montgomery factor.  The check
/// below is mod-q equivalence.
pub fn ntt_then_invert_ntt_roundtrip() {
    let mut rng = seeded_rng(0x4711);
    for _ in 0..ITERATIONS {
        let p = random_polynomial_coeffs(&mut rng, Q);
        // TODO: requires Operations + PolynomialRingElement accessibility.
        // Intended body:
        //   let mut units = to_polynomial_units::<O>(&p);
        //   O::ntt(&mut units);
        //   O::invert_ntt_montgomery(&mut units);
        //   let got = from_polynomial_units::<O>(&units);
        //   for i in 0..N {
        //       assert_eq!(
        //           mod_q_local(got[i] as i64),
        //           mod_q_local(p[i] as i64),
        //           "ntt-intt mismatch at coeff {}: got {}, expected {}",
        //           i, got[i], p[i]
        //       );
        //   }
        let _ = p;
    }
}

/// `Operations::ntt(p)` flat-256 matches `hacspec_ml_dsa::ntt::ntt(p)`
/// (mod q per coefficient).
pub fn ntt_matches_spec() {
    let mut rng = seeded_rng(0x4774);
    for _ in 0..ITERATIONS {
        let p = random_polynomial_coeffs(&mut rng, Q);
        // TODO: requires Operations + PolynomialRingElement + spec::ntt::ntt.
        // Intended body:
        //   let mut units = to_polynomial_units::<O>(&p);
        //   O::ntt(&mut units);
        //   let got = from_polynomial_units::<O>(&units);
        //   let expected = spec::ntt::ntt(p);
        //   for i in 0..N {
        //       assert_eq!(
        //           mod_q_local(got[i] as i64),
        //           mod_q_local(expected[i] as i64),
        //           "ntt vs spec mismatch at coeff {}: got {}, expected {}",
        //           i, got[i], expected[i]
        //       );
        //   }
        let _ = p;
    }
}

// ---------------------------------------------------------------------------
// Concrete instantiations.
// ---------------------------------------------------------------------------

#[test]
fn ntt_then_invert_ntt_roundtrip_portable() {
    ntt_then_invert_ntt_roundtrip();
}

#[test]
fn ntt_matches_spec_portable() {
    ntt_matches_spec();
}

#[cfg(feature = "simd256")]
mod avx2 {
    use super::*;

    #[test]
    fn ntt_then_invert_ntt_roundtrip_avx2() {
        ntt_then_invert_ntt_roundtrip();
    }

    #[test]
    fn ntt_matches_spec_avx2() {
        ntt_matches_spec();
    }
}
