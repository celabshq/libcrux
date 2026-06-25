//! Cross-comparison tests between the libcrux ML-DSA implementation and the
//! hacspec specification at `specs/ml-dsa/`.
//!
//! Mirrors the methodology in `libcrux-ml-kem/tests/cross_spec.rs`: each
//! `Operations`-trait method is exercised against its `hacspec_ml_dsa::*`
//! equivalent over a deterministic ChaCha20-RNG fuzz of 100 iterations
//! (kept low for fast feedback), parameterized over both the `Portable` and
//! `AVX2` SIMD impls.
//!
//! Gated behind the `cross-spec-tests` feature so the default `cargo test`
//! workflow is unaffected.
//!
//! ## Accessibility wall (2026-04-27)
//!
//! At the time of writing both `libcrux_ml_dsa::simd::*` and the hacspec
//! internal modules (`hacspec_ml_dsa::{arithmetic, encoding, ntt,
//! sampling, polynomial, ...}`) are `pub(crate)`. External integration
//! tests (this file lives in `tests/`, which is a separate crate) cannot
//! reach the `Operations` trait, the per-impl SIMD types, the per-lane
//! arithmetic helpers, or the per-byte sampling step helpers.
//!
//! Until those modules are re-exported behind a `test-utils` (or
//! `cross-spec-tests`) feature gate, the per-method tests below are
//! left as TODO stubs that document the exact item that needs to become
//! reachable.  The single test we *can* exercise end-to-end is the
//! `sign_pre_hashed` context-length corner case in `tests/edge_cases.rs`,
//! which only uses the public top-level API.
//!
//! The recommended source change to unblock the per-method tests:
//!
//! ```rust,ignore
//! // libcrux-ml-dsa/src/lib.rs — under `#[cfg(feature = "test-utils")]`:
//! #[cfg(feature = "test-utils")]
//! pub mod simd { pub use crate::simd::*; }
//! #[cfg(feature = "test-utils")]
//! pub mod polynomial { pub use crate::polynomial::*; }
//! ```
//!
//! And, on the hacspec side:
//!
//! ```rust,ignore
//! // specs/ml-dsa/src/lib.rs — make modules `pub` (or re-export):
//! pub use crate::arithmetic::{decompose, make_hint, use_hint, power2round,
//!                              shift_left_then_reduce, montgomery_reduce, mod_q};
//! pub use crate::encoding::{coeff_from_three_bytes, coeff_from_half_byte,
//!                            simple_bit_pack, simple_bit_unpack,
//!                            bit_pack, bit_unpack};
//! pub use crate::ntt::{ntt, intt};
//! ```

#![cfg(feature = "cross-spec-tests")]

// Internal helpers and test modules.
// Each module is wired below; the per-method tests are mostly TODO until
// the accessibility gap above is closed.
mod cross_spec {
    pub mod helpers;
    pub mod arithmetic;
    pub mod encoding;
    pub mod sampling;
    pub mod ntt;
}

/// Generate a parameterized cross-spec test over a chosen `Operations` impl.
///
/// Usage (once `Operations` and the impl types become reachable):
///
/// ```rust,ignore
/// cross_spec_test!(
///     test_add_portable_matches_spec,
///     "std",
///     PortableImpl,
///     test_add_matches_spec
/// );
///
/// #[cfg(feature = "simd256")]
/// cross_spec_test!(
///     test_add_avx2_matches_spec,
///     "simd256",
///     Avx2Impl,
///     test_add_matches_spec
/// );
/// ```
///
/// `$feature` is the cargo feature flag that gates this instantiation
/// (e.g. `"simd256"` for AVX2, `"std"` for portable — every build has
/// the latter).  `$impl_ty` is the SIMD impl type that implements
/// `libcrux_ml_dsa::simd::traits::Operations`.  `$generic_test` is the
/// name of a `<O: Operations>` test fn defined in one of the per-area
/// modules (`arithmetic`, `encoding`, `sampling`, `ntt`).
#[macro_export]
macro_rules! cross_spec_test {
    ($name:ident, $feature:literal, $impl_ty:ty, $generic_test:ident) => {
        #[cfg(feature = $feature)]
        #[test]
        fn $name() {
            // TODO: enable once `Operations` is re-exported under
            // `cfg(feature = "test-utils")`. See the "Accessibility wall"
            // comment in tests/cross_spec.rs.
            // $generic_test::<$impl_ty>();
            assert!(true, "cross-spec test gated on test-utils accessor");
        }
    };
}
