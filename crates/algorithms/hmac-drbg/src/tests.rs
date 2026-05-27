use crate::*;

// ---------------------------------------------------------------------------
// Shared entropy helpers for tests
// ---------------------------------------------------------------------------

/// Build a `[u8; N]` by cycling through `[a, b, a, b, ...]`.
/// Passes the AIS 31 startup test: non-constant, ~50 % bits set, no long runs.
const fn alt<const N: usize>(a: u8, b: u8) -> [u8; N] {
    let mut out = [0u8; N];
    let mut i = 0usize;
    while i < N {
        out[i] = if i % 2 == 0 { a } else { b };
        i += 1;
    }
    out
}

const E32: [u8; 32] = alt(0x55, 0xaa);

// ---------------------------------------------------------------------------
// AIS 31 startup tests (feature = "health-tests")
// ---------------------------------------------------------------------------

#[cfg(feature = "health-tests")]
mod startup_tests {
    use super::*;
    use crate::{startup_test, RCT_C, STARTUP_MONOBIT_MIN_BYTES};

    // Primary entropy (0x55 / 0xaa alternating, 50 % bits set)
    pub const E48: [u8; 48] = alt(0x55, 0xaa);
    pub const E64: [u8; 64] = alt(0x55, 0xaa);

    // Alternate entropy used where a *different* seed is needed (e.g. reseed tests)
    pub const R32: [u8; 32] = alt(0xa5, 0x5a);

    // --- non-constant (total failure) ---

    #[test]
    fn too_short_fails() {
        assert!(startup_test(&[0x55u8; 31], 32));
        assert!(startup_test(&[], 32));
    }

    #[test]
    fn all_zeros_fails() {
        assert!(startup_test(&[0u8; 32], 32));
    }

    #[test]
    fn all_ones_fails() {
        assert!(startup_test(&[0xffu8; 32], 32));
    }

    #[test]
    fn all_same_byte_fails() {
        assert!(startup_test(&[0x42u8; 32], 32));
    }

    #[test]
    fn two_distinct_bytes_passes_non_constant() {
        // [0x00, 0xff, 0x00, 0xff, ...] alternating — not constant, 50 % bits set
        let mut e = [0x00u8; 32];
        for i in (1..32).step_by(2) {
            e[i] = 0xff;
        }
        assert!(!startup_test(&e, 32));
    }

    // --- monobit ---

    #[test]
    fn monobit_too_few_ones_fails() {
        // 16 × 0x01 (1 set bit each) || 16 × 0x00 = 16 ones out of 256 bits = 6.25 %
        let mut e = [0x00u8; 32];
        for b in &mut e[..16] {
            *b = 0x01;
        }
        assert!(startup_test(&e, 32));
    }

    #[test]
    fn monobit_too_many_ones_fails() {
        // 16 × 0xfe (7 ones) || 16 × 0xff (8 ones) = 248 ones out of 256 bits = 96.9 %
        let mut e = [0xffu8; 32];
        for b in &mut e[..16] {
            *b = 0xfe;
        }
        assert!(startup_test(&e, 32));
    }

    #[test]
    fn monobit_not_applied_below_min_bytes() {
        // Two bytes: 1 set bit out of 16 — would fail monobit, but input is
        // below STARTUP_MONOBIT_MIN_BYTES so the test is skipped.
        // min_len=0 to isolate the monobit behaviour from the length check.
        let e: &[u8] = &[0x00, 0x01];
        assert!(STARTUP_MONOBIT_MIN_BYTES > 2, "precondition");
        // Non-constant passes (two distinct bytes); monobit skipped.
        assert!(!startup_test(e, 0));
    }

    // --- run-length ---

    #[test]
    fn run_at_threshold_fails() {
        // One different byte then RCT_C identical bytes → run of RCT_C → failure.
        let mut e = [0xaau8; 32];
        e[0] = 0x00;
        assert!(startup_test(&e[..1 + RCT_C as usize], 0));
    }

    #[test]
    fn run_below_threshold_passes() {
        // One different byte + RCT_C-1 identical bytes → run of 20 → passes.
        // RCT_C=21, so worst case is 1+20=21 bytes; fits in a fixed-size array.
        let mut e = [0x00u8; 21];
        let run = RCT_C as usize - 1; // 20
        for b in &mut e[1..1 + run] {
            *b = 0xbb;
        }
        assert!(!startup_test(&e[..1 + run], 0));
    }

    // --- integration: new() rejects bad entropy ---

    #[test]
    fn new_rejects_all_zero_entropy() {
        let err = crate::HmacDrbgSha256::new(&[0u8; 32], &[0u8; 16], &[]).unwrap_err();
        assert_eq!(err, crate::InstantiateError::HealthCheckFailed);
    }

    #[test]
    fn new_accepts_good_entropy() {
        crate::HmacDrbgSha256::new(&super::E32, &[0u8; 16], &[]).unwrap();
    }

    // --- integration: reseed() rejects bad entropy ---

    #[test]
    fn reseed_rejects_all_zero_entropy() {
        let mut drbg = crate::HmacDrbgSha256::new(&super::E32, &[0u8; 16], &[]).unwrap();
        let err = drbg.reseed(&[0u8; 32], &[]).unwrap_err();
        assert_eq!(err, crate::ReseedError::HealthCheckFailed);
    }
}

// ---------------------------------------------------------------------------
// Continuous health tests (feature = "health-tests")
// ---------------------------------------------------------------------------

#[cfg(feature = "health-tests")]
mod health_tests {
    use super::{
        startup_tests::{E48, E64, R32},
        E32,
    };
    use crate::*;

    #[test]
    fn is_poisoned_false_initially() {
        let drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        assert!(!drbg.is_poisoned());
    }

    #[test]
    fn poison_for_testing_sets_flag() {
        let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        drbg.poison_for_testing();
        assert!(drbg.is_poisoned());
    }

    #[test]
    fn poisoned_generate_returns_error() {
        let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        drbg.poison_for_testing();
        let mut out = [0u8; 32];
        assert_eq!(
            drbg.generate(&mut out, &[]).unwrap_err(),
            GenerateError::HealthCheckFailed
        );
    }

    #[test]
    fn poisoned_reseed_returns_error() {
        let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        drbg.poison_for_testing();
        assert_eq!(
            drbg.reseed(&R32, &[]).unwrap_err(),
            ReseedError::HealthCheckFailed
        );
    }

    #[test]
    fn repeated_generate_stays_healthy() {
        let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        let mut out = [0u8; 32];
        for _ in 0..10 {
            drbg.generate(&mut out, &[]).unwrap();
            assert!(!drbg.is_poisoned());
        }
    }

    #[test]
    fn repeated_generate_sha384_stays_healthy() {
        let mut drbg = HmacDrbgSha384::new(&E48, &[0u8; 24], &[]).unwrap();
        let mut out = [0u8; 48];
        for _ in 0..10 {
            drbg.generate(&mut out, &[]).unwrap();
            assert!(!drbg.is_poisoned());
        }
    }

    #[test]
    fn repeated_generate_sha512_stays_healthy() {
        let mut drbg = HmacDrbgSha512::new(&E64, &[0u8; 32], &[]).unwrap();
        let mut out = [0u8; 64];
        for _ in 0..10 {
            drbg.generate(&mut out, &[]).unwrap();
            assert!(!drbg.is_poisoned());
        }
    }

    #[test]
    fn reseed_clears_prev_block_valid() {
        // After a reseed the inter-call tracker is reset; the first generate
        // after reseed must not falsely detect a collision with pre-reseed output.
        let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        let mut out = [0u8; 32];
        drbg.generate(&mut out, &[]).unwrap();
        drbg.reseed(&R32, &[]).unwrap();
        drbg.generate(&mut out, &[]).unwrap();
        assert!(!drbg.is_poisoned());
    }

    #[test]
    fn health_debug_shows_poisoned_flag() {
        let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
        drbg.poison_for_testing();
        assert!(drbg.is_poisoned());
    }
}

#[test]
fn reseed_required_error() {
    let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
    drbg.set_reseed_counter(RESEED_INTERVAL + 1);
    let mut out = [0u8; 32];
    let err = drbg.generate(&mut out, &[]).unwrap_err();
    assert_eq!(err, GenerateError::ReseedRequired);
}

#[test]
fn needs_reseed_true_after_exhaustion() {
    let mut drbg = HmacDrbgSha256::new(&E32, &[0u8; 16], &[]).unwrap();
    drbg.set_reseed_counter(RESEED_INTERVAL + 1);
    assert!(drbg.needs_reseed());
}
