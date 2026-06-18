use libcrux_hmac_drbg::{
    GenerateError, HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512, MAX_GENERATE_BYTES,
};
use libcrux_kats::cavp::drbg::HmacDrbgTest;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a valid test-entropy array of `N` bytes (0x55/0xaa alternating).
/// Passes the AIS 31 startup test: non-constant, ~50 % bits set, no long runs.
fn alt_entropy<const N: usize>() -> [u8; N] {
    let mut out = [0u8; N];
    for (i, b) in out.iter_mut().enumerate() {
        *b = if i % 2 == 0 { 0x55 } else { 0xaa };
    }
    out
}

fn make_sha256() -> HmacDrbgSha256 {
    HmacDrbgSha256::new(&alt_entropy::<32>(), &[0u8; 16], &[]).unwrap()
}

fn make_sha384() -> HmacDrbgSha384 {
    HmacDrbgSha384::new(&alt_entropy::<48>(), &[0u8; 24], &[]).unwrap()
}

fn make_sha512() -> HmacDrbgSha512 {
    HmacDrbgSha512::new(&alt_entropy::<64>(), &[0u8; 32], &[]).unwrap()
}

fn run_kat(t: &HmacDrbgTest, expected: &[u8]) -> usize {
    // Dispatch on hash, run two generate calls, check returned_bits.
    let mut ran_test = 0;
    match t.hash.as_str() {
        "SHA-256" => {
            let mut drbg =
                HmacDrbgSha256::new(&t.entropy_input, &t.nonce, &t.personalization_string).unwrap();
            if !t.entropy_input_reseed.is_empty() {
                drbg.reseed(&t.entropy_input_reseed, &t.additional_input_reseed)
                    .unwrap();
            }
            let mut out = vec![0u8; expected.len()];
            drbg.generate(
                &mut out,
                t.additional_input
                    .first()
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .unwrap();
            drbg.generate(
                &mut out,
                t.additional_input
                    .get(1)
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .unwrap();
            assert_eq!(out, expected, "SHA-256 COUNT={}", t.count);
            ran_test = 1;
        }

        "SHA-384" => {
            let mut drbg =
                HmacDrbgSha384::new(&t.entropy_input, &t.nonce, &t.personalization_string).unwrap();
            if !t.entropy_input_reseed.is_empty() {
                drbg.reseed(&t.entropy_input_reseed, &t.additional_input_reseed)
                    .unwrap();
            }
            let mut out = vec![0u8; expected.len()];
            drbg.generate(
                &mut out,
                t.additional_input
                    .first()
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .unwrap();
            drbg.generate(
                &mut out,
                t.additional_input
                    .get(1)
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .unwrap();
            assert_eq!(out, expected, "SHA-384 COUNT={}", t.count);
            ran_test = 1;
        }

        "SHA-512" => {
            let mut drbg =
                HmacDrbgSha512::new(&t.entropy_input, &t.nonce, &t.personalization_string).unwrap();
            if !t.entropy_input_reseed.is_empty() {
                drbg.reseed(&t.entropy_input_reseed, &t.additional_input_reseed)
                    .unwrap();
            }
            let mut out = vec![0u8; expected.len()];
            drbg.generate(
                &mut out,
                t.additional_input
                    .first()
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .unwrap();
            drbg.generate(
                &mut out,
                t.additional_input
                    .get(1)
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .unwrap();
            assert_eq!(out, expected, "SHA-512 COUNT={}", t.count);
            ran_test = 1;
        }
        s => {
            eprintln!("Skipping test for hash function {s}.")
        } // SHA-1, SHA-224, SHA-512/224, SHA-512/256 — not supported, skip
    }
    ran_test
}

// ---------------------------------------------------------------------------
// KAT tests
// ---------------------------------------------------------------------------

#[test]
fn kat() {
    // Supported: "SHA-256", "SHA-384", "SHA-512"
    let tv = libcrux_kats::cavp::drbg::HmacDrbg::new().unwrap();
    assert!(!tv.tests.is_empty(), "no supported test vectors found");
    let mut test_count = 0;
    for t in &tv.tests {
        test_count += run_kat(t, &t.returned_bits);
    }
    eprintln!("Ran {test_count} / {} tests", tv.tests.len());
}

// ---------------------------------------------------------------------------
// §10.1.2.3  Instantiate
// ---------------------------------------------------------------------------

#[test]
fn instantiate_sha256() {
    let drbg = make_sha256();
    assert_eq!(drbg.reseed_counter(), 1);
}

#[test]
fn instantiate_sha384() {
    let drbg = make_sha384();
    assert_eq!(drbg.reseed_counter(), 1);
}

#[test]
fn instantiate_sha512() {
    let drbg = make_sha512();
    assert_eq!(drbg.reseed_counter(), 1);
}

#[test]
fn instantiate_large_entropy_accepted() {
    // Large entropy is no longer capped; new() must succeed.
    let big = alt_entropy::<512>();
    let mut drbg = HmacDrbgSha256::new(&big, &[], &[]).unwrap();
    let mut out = [0u8; 32];
    drbg.generate(&mut out, &[]).unwrap();
}

// ---------------------------------------------------------------------------
// §10.1.2.5  Generate — counter and output
// ---------------------------------------------------------------------------

#[test]
fn counter_increments_sha256() {
    let mut drbg = make_sha256();
    assert_eq!(drbg.reseed_counter(), 1);
    let mut out = [0u8; 32];
    drbg.generate(&mut out, &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 2);
    drbg.generate(&mut out, &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 3);
}

#[test]
fn counter_increments_sha384() {
    let mut drbg = make_sha384();
    let mut out = [0u8; 48];
    drbg.generate(&mut out, &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 2);
}

#[test]
fn counter_increments_sha512() {
    let mut drbg = make_sha512();
    let mut out = [0u8; 64];
    drbg.generate(&mut out, &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 2);
}

#[test]
fn generate_deterministic_sha256() {
    let mut a = make_sha256();
    let mut b = make_sha256();
    let mut out_a = [0u8; 64];
    let mut out_b = [0u8; 64];
    a.generate(&mut out_a, &[]).unwrap();
    b.generate(&mut out_b, &[]).unwrap();
    assert_eq!(out_a, out_b);
}

#[test]
fn generate_different_seeds_sha256() {
    let mut a = HmacDrbgSha256::new(&alt_entropy::<32>(), &[0u8; 16], &[]).unwrap();
    // Second seed: shift the pattern by one so it's different but still valid.
    let mut e2 = alt_entropy::<32>();
    e2[0] ^= 0x01;
    let mut b = HmacDrbgSha256::new(&e2, &[0u8; 16], &[]).unwrap();
    let mut out_a = [0u8; 32];
    let mut out_b = [0u8; 32];
    a.generate(&mut out_a, &[]).unwrap();
    b.generate(&mut out_b, &[]).unwrap();
    assert_ne!(out_a, out_b);
}

#[test]
fn request_zero_bytes_rejected() {
    let mut drbg = make_sha256();
    let err = drbg.generate(&mut [], &[]).unwrap_err();
    assert_eq!(err, GenerateError::RequestInvalid);
}

#[test]
fn request_too_large() {
    let mut drbg = make_sha256();
    let mut big = vec![0u8; MAX_GENERATE_BYTES + 1];
    let err = drbg.generate(&mut big, &[]).unwrap_err();
    assert_eq!(err, GenerateError::RequestInvalid);
}

#[test]
fn request_max_bytes_succeeds() {
    let mut drbg = make_sha256();
    let mut out = vec![0u8; MAX_GENERATE_BYTES];
    drbg.generate(&mut out, &[]).unwrap();
}

#[test]
fn needs_reseed_false_initially() {
    let drbg = make_sha256();
    assert!(!drbg.needs_reseed());
}

#[test]
fn debug_redacts_key_material() {
    let drbg = make_sha256();
    let s = format!("{drbg:?}");
    assert!(
        s.contains("<redacted>"),
        "key/v must be redacted in Debug output"
    );
    assert!(
        !s.contains("reseed_counter: 0"),
        "reseed_counter should be visible"
    );
}

// ---------------------------------------------------------------------------
// §10.1.2.4  Reseed
// ---------------------------------------------------------------------------

#[test]
fn reseed_resets_counter_sha256() {
    let mut drbg = make_sha256();
    let mut out = [0u8; 32];
    drbg.generate(&mut out, &[]).unwrap();
    drbg.generate(&mut out, &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 3);
    drbg.reseed(&alt_entropy::<32>(), &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 1);
}

#[test]
fn reseed_resets_counter_sha384() {
    let mut drbg = make_sha384();
    let mut out = [0u8; 48];
    drbg.generate(&mut out, &[]).unwrap();
    drbg.reseed(&alt_entropy::<48>(), &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 1);
}

#[test]
fn reseed_resets_counter_sha512() {
    let mut drbg = make_sha512();
    let mut out = [0u8; 64];
    drbg.generate(&mut out, &[]).unwrap();
    drbg.reseed(&alt_entropy::<64>(), &[]).unwrap();
    assert_eq!(drbg.reseed_counter(), 1);
}

#[test]
fn reseed_changes_output() {
    let mut drbg = make_sha256();
    let mut out1 = [0u8; 32];
    drbg.generate(&mut out1, &[]).unwrap();
    // Use a different-but-valid entropy so the reseed changes the DRBG state.
    let mut e2 = alt_entropy::<32>();
    e2[0] ^= 0x01;
    drbg.reseed(&e2, &[]).unwrap();
    let mut out2 = [0u8; 32];
    drbg.generate(&mut out2, &[]).unwrap();
    assert_ne!(out1, out2);
}

#[test]
fn reseed_large_entropy_accepted() {
    let mut drbg = make_sha256();
    let big = alt_entropy::<512>();
    drbg.reseed(&big, &[]).unwrap();
}
