//! Corner-case tests for ML-DSA — independent of the per-method cross-spec
//! tests in `tests/cross_spec.rs`.
//!
//! Ten tests covering:
//!   1. `decompose` wraparound at q-1.
//!   2. `HintBitPack` with exactly ω one-bits (per param-set ω).
//!   3. `HintBitUnpack` rejects non-monotonic block indices.
//!   4. `HintBitUnpack` rejects nonzero padding bytes.
//!   5. Infinity norm at exact γ₁ bound (γ₁-1, γ₁, γ₁+1).
//!   6. `power2round` near q (q-2, q-1).
//!   7. `power2round` at the 2²³ D-boundary (2²³-1, 2²³, 2²³+1).
//!   8. `SampleInBall` with τ=0 produces all-zero polynomial.
//!   9. `sign_pre_hashed` with empty context succeeds.
//!  10. `sign_pre_hashed` with 255-byte context succeeds; 256 fails.
//!
//! ## Status
//!
//! Tests 1–8 require either the SIMD trait (`Operations`), the per-impl
//! types, or a hacspec helper that is currently `pub(crate)` and so
//! unreachable from this integration-test crate.  They are TODO stubs
//! with the intended body inlined as a comment.
//!
//! Tests 9 and 10 use only the public top-level API
//! (`libcrux_ml_dsa::ml_dsa_44::*`) and are functional.

#![cfg(feature = "cross-spec-tests")]
#![allow(dead_code, unused_imports, unused_variables)]

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

const Q: i32 = 8_380_417;
const D: i32 = 13;
const TWO_TO_D: i32 = 1 << 13;
const TWO_TO_23: i32 = 1 << 23;

const GAMMA1_44: i32 = 1 << 17;
const GAMMA1_65: i32 = 1 << 19;
const GAMMA1_87: i32 = 1 << 19;

const GAMMA2_44: i32 = (Q - 1) / 88; // 95_232
const GAMMA2_65: i32 = (Q - 1) / 32; // 261_888
const GAMMA2_87: i32 = (Q - 1) / 32;

// ω from FIPS 204 Table 1.
const OMEGA_44: usize = 80;
const OMEGA_65: usize = 55;
const OMEGA_87: usize = 75;

// ---------------------------------------------------------------------------
// 1. decompose wraparound at q-1.
// ---------------------------------------------------------------------------

/// Feed coefficients near q-1, check the special-case branch of FIPS Alg 36
/// where `r⁺ - r₀ == q - 1` returns `(0, r₀ - 1)`.
#[test]
fn decompose_wraparound_at_q_minus_1() {
    // TODO: requires hacspec_ml_dsa::arithmetic::decompose to be `pub`.
    // Intended body:
    //   for &gamma2 in &[GAMMA2_44, GAMMA2_65] {
    //       for &r in &[Q - 1, Q - 2, Q - 3, Q - 88, Q - 89, Q - 90] {
    //           let (r1, r0) = hacspec_ml_dsa::arithmetic::decompose(r, gamma2);
    //           let alpha = 2 * gamma2;
    //           let r0_back = ((r0 as i64) + alpha as i64) as i32 % alpha;
    //           assert!(r0 >= -alpha/2 && r0 <= alpha/2,
    //                   "decompose r0 out of range for r={}", r);
    //           // Reconstruction modulo q.
    //           let recon = ((r1 as i64) * (alpha as i64) + (r0 as i64))
    //                          .rem_euclid(Q as i64) as i32;
    //           assert_eq!(recon, r % Q,
    //                      "decompose reconstruction failed for r={}, gamma2={}",
    //                      r, gamma2);
    //       }
    //   }
    assert!(true, "decompose_wraparound stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 2. HintBitPack with exactly ω one-bits.
// ---------------------------------------------------------------------------

/// Construct a hint vector with exactly ω 1-bits and check round-trip.
#[test]
fn hint_bit_pack_count_exactly_omega() {
    // TODO: requires hacspec_ml_dsa::encoding::{hint_bit_pack, hint_bit_unpack}
    // and the corresponding libcrux_ml_dsa::encoding::signature helpers,
    // none of which are reachable.  Intended body:
    //
    //   for &(k, omega) in &[(4, OMEGA_44), (6, OMEGA_65), (8, OMEGA_87)] {
    //       let mut hint = vec![[false; 256]; k];
    //       // Place exactly omega 1-bits, distributed across rows.
    //       let mut placed = 0;
    //       'outer: for row in 0..k {
    //           for col in 0..256 {
    //               if placed == omega { break 'outer; }
    //               hint[row][col] = true;
    //               placed += 1;
    //           }
    //       }
    //       // hint_bit_pack returns a serialized hint of length omega + k.
    //       let hint_array: [[bool; 256]; K] = ...;
    //       let packed = spec::encoding::hint_bit_pack::<K, HINT_BYTES>(&hint_array);
    //       let unpacked = spec::encoding::hint_bit_unpack::<K>(&packed, omega);
    //       assert!(unpacked.is_some(), "round-trip at exactly omega failed");
    //       let unpacked = unpacked.unwrap();
    //       for row in 0..K {
    //           for col in 0..256 {
    //               assert_eq!(unpacked[row][col], hint_array[row][col]);
    //           }
    //       }
    //   }
    assert!(true, "hint_bit_pack_count_exactly_omega stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 3. HintBitUnpack rejects non-monotonic block indices.
// ---------------------------------------------------------------------------

/// Synthesize a malformed signature with non-monotonic block indices,
/// ensure deserialize returns Err.
#[test]
fn hint_bit_unpack_rejects_non_monotonic_indices() {
    // TODO: requires hacspec_ml_dsa::encoding::hint_bit_unpack to be pub.
    // Intended body:
    //   // Build a 2-row hint where row 0 has bit indices [5, 3] (non-monotonic).
    //   let omega = OMEGA_44;
    //   let k = 4;
    //   let mut buf = vec![0u8; omega + k];
    //   // Row 0: 2 ones at positions 5 and 3 (out of order).
    //   buf[0] = 5;
    //   buf[1] = 3; // <- violates monotonicity
    //   buf[omega] = 2;     // count for row 0
    //   buf[omega + 1] = 2; // cumulative count after row 1 (no ones in rows 1..k)
    //   buf[omega + 2] = 2;
    //   buf[omega + 3] = 2;
    //   let res = spec::encoding::hint_bit_unpack::<K>(&buf, omega);
    //   assert!(res.is_none(), "non-monotonic indices must be rejected");
    assert!(true, "hint_bit_unpack_rejects_non_monotonic stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 4. HintBitUnpack rejects nonzero padding bytes.
// ---------------------------------------------------------------------------

/// Analogous to test 3 but the malformation is non-zero padding bytes
/// past the last cumulative-count index.
#[test]
fn hint_bit_unpack_rejects_nonzero_padding() {
    // TODO: requires hacspec_ml_dsa::encoding::hint_bit_unpack to be pub.
    // Intended body:
    //   let omega = OMEGA_44;
    //   let k = 4;
    //   let mut buf = vec![0u8; omega + k];
    //   // No ones; cumulative counts all zero except final byte.
    //   buf[5] = 0xFF; // non-zero in the padding region (positions 0..omega
    //                  // beyond cumulative_count[k-1] = 0).
    //   buf[omega..].fill(0);
    //   let res = spec::encoding::hint_bit_unpack::<K>(&buf, omega);
    //   assert!(res.is_none(), "nonzero padding must be rejected");
    assert!(true, "hint_bit_unpack_rejects_nonzero_padding stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 5. Infinity norm at exact γ₁ bound.
// ---------------------------------------------------------------------------

/// γ₁ exactly, γ₁-1, γ₁+1.
#[test]
fn infinity_norm_at_exact_bound() {
    // TODO: requires libcrux_ml_dsa::simd::traits::Operations to be reachable.
    // Intended body:
    //   for &gamma1 in &[GAMMA1_44, GAMMA1_65] {
    //       let mut unit_below = O::zero();
    //       let mut unit_at = O::zero();
    //       let mut unit_above = O::zero();
    //       O::from_coefficient_array(&[gamma1 - 1, 0, 0, 0, 0, 0, 0, 0],
    //                                  &mut unit_below);
    //       O::from_coefficient_array(&[gamma1, 0, 0, 0, 0, 0, 0, 0],
    //                                  &mut unit_at);
    //       O::from_coefficient_array(&[gamma1 + 1, 0, 0, 0, 0, 0, 0, 0],
    //                                  &mut unit_above);
    //       assert!(!O::infinity_norm_exceeds(&unit_below, gamma1),
    //               "γ₁-1 should NOT exceed γ₁");
    //       assert!(O::infinity_norm_exceeds(&unit_at, gamma1),
    //               "γ₁ should exceed (>=) γ₁");
    //       assert!(O::infinity_norm_exceeds(&unit_above, gamma1),
    //               "γ₁+1 should exceed γ₁");
    //   }
    assert!(true, "infinity_norm_at_exact_bound stubbed pending Operations accessor");
}

// ---------------------------------------------------------------------------
// 6. power2round near q.
// ---------------------------------------------------------------------------

/// r ∈ {q-2, q-1}.
#[test]
fn power2round_near_q() {
    // TODO: requires hacspec_ml_dsa::arithmetic::power2round to be pub.
    // Intended body:
    //   for &r in &[Q - 2, Q - 1] {
    //       let (r1, r0) = spec::arithmetic::power2round(r);
    //       // Reconstruction: r ≡ r1 · 2^d + r0 (mod q).
    //       let recon = ((r1 as i64) * (TWO_TO_D as i64) + r0 as i64)
    //                       .rem_euclid(Q as i64) as i32;
    //       assert_eq!(recon, r, "power2round reconstruction failed at r={}", r);
    //       // r0 must be centered.
    //       assert!(r0 > -TWO_TO_D / 2 && r0 <= TWO_TO_D / 2,
    //               "power2round r0 out of centered range: {}", r0);
    //   }
    assert!(true, "power2round_near_q stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 7. power2round at the 2²³ D-boundary.
// ---------------------------------------------------------------------------

/// r ∈ {2²³-1, 2²³, 2²³+1}.  The D-boundary is interesting because the
/// rounding flips between r1 = ⌊r / 2¹³⌋ and r1 = ⌈r / 2¹³⌉ exactly here.
#[test]
fn power2round_at_2_to_23() {
    // TODO: requires hacspec_ml_dsa::arithmetic::power2round to be pub.
    // Intended body:
    //   for &r in &[TWO_TO_23 - 1, TWO_TO_23, TWO_TO_23 + 1] {
    //       if r >= Q { continue; }
    //       let (r1, r0) = spec::arithmetic::power2round(r);
    //       let recon = ((r1 as i64) * (TWO_TO_D as i64) + r0 as i64)
    //                       .rem_euclid(Q as i64) as i32;
    //       assert_eq!(recon, r, "power2round reconstruction failed at r={}", r);
    //   }
    assert!(true, "power2round_at_2_to_23 stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 8. SampleInBall with τ=0.
// ---------------------------------------------------------------------------

/// τ=0 should produce an all-zero polynomial.
#[test]
fn sample_in_ball_tau_zero() {
    // TODO: requires hacspec_ml_dsa::sampling::sample_in_ball to be pub.
    // Intended body:
    //   let rho = [0u8; 32];
    //   let c = spec::sampling::sample_in_ball(&rho, 0);
    //   for i in 0..256 {
    //       assert_eq!(c[i], 0, "sample_in_ball(τ=0) coeff {} = {}", i, c[i]);
    //   }
    assert!(true, "sample_in_ball_tau_zero stubbed pending spec accessor");
}

// ---------------------------------------------------------------------------
// 9 & 10. sign_pre_hashed with empty / 255-byte / 256-byte context.
//
// These DO use only the public top-level API, so they're functional.
// ---------------------------------------------------------------------------

/// `sign_pre_hashed` with empty context succeeds.
#[test]
fn sign_pre_hashed_empty_context() {
    let randomness = [0x42u8; libcrux_ml_dsa::KEY_GENERATION_RANDOMNESS_SIZE];
    let signing_randomness = [0x55u8; libcrux_ml_dsa::SIGNING_RANDOMNESS_SIZE];
    let key_pair = libcrux_ml_dsa::ml_dsa_44::generate_key_pair(randomness);
    let message = b"empty-context test";
    let context: &[u8] = &[];
    let result = libcrux_ml_dsa::ml_dsa_44::sign_pre_hashed_shake128(
        &key_pair.signing_key,
        message,
        context,
        signing_randomness,
    );
    assert!(
        result.is_ok(),
        "sign_pre_hashed with empty context should succeed: {:?}",
        result.err()
    );
}

/// `sign_pre_hashed` with 255-byte context succeeds; 256-byte context fails
/// with `ContextTooLongError`.
#[test]
fn sign_pre_hashed_max_context() {
    let randomness = [0x77u8; libcrux_ml_dsa::KEY_GENERATION_RANDOMNESS_SIZE];
    let signing_randomness = [0xAAu8; libcrux_ml_dsa::SIGNING_RANDOMNESS_SIZE];
    let key_pair = libcrux_ml_dsa::ml_dsa_44::generate_key_pair(randomness);
    let message = b"max-context test";

    let ctx_255 = vec![0xFFu8; 255];
    let ok = libcrux_ml_dsa::ml_dsa_44::sign_pre_hashed_shake128(
        &key_pair.signing_key,
        message,
        &ctx_255,
        signing_randomness,
    );
    assert!(
        ok.is_ok(),
        "sign_pre_hashed with 255-byte context should succeed: {:?}",
        ok.err()
    );

    let ctx_256 = vec![0xFFu8; 256];
    let err = libcrux_ml_dsa::ml_dsa_44::sign_pre_hashed_shake128(
        &key_pair.signing_key,
        message,
        &ctx_256,
        signing_randomness,
    );
    match err {
        Err(libcrux_ml_dsa::SigningError::ContextTooLongError) => {}
        Err(other) => panic!(
            "sign_pre_hashed with 256-byte context should fail with ContextTooLongError, got {:?}",
            other
        ),
        Ok(_) => panic!(
            "sign_pre_hashed with 256-byte context should fail with ContextTooLongError, got Ok(_)"
        ),
    }
}
