//! Integration tests for the `rand` trait implementations on [`HmacDrbg`].
//!
//! Requires the `rand` feature.  Run with:
//! ```text
//! cargo test -p libcrux-drbg --features rand
//! ```
#![cfg(feature = "rand")]

use libcrux_hmac_drbg::{HmacDrbgSeed, HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512};
use rand::{SeedableRng, TryCryptoRng, TryRng};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Valid alternating-byte entropy (0x55 / 0xaa). Passes AIS 31 startup tests.
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

// ---------------------------------------------------------------------------
// TryRng — try_next_u32 / try_next_u64 / try_fill_bytes
// ---------------------------------------------------------------------------

#[test]
fn try_next_u32_returns_ok() {
    let mut drbg = make_sha256();
    drbg.try_next_u32().unwrap();
}

#[test]
fn try_next_u64_returns_ok() {
    let mut drbg = make_sha256();
    drbg.try_next_u64().unwrap();
}

#[test]
fn try_next_u32_sha384() {
    let mut drbg = make_sha384();
    drbg.try_next_u32().unwrap();
}

#[test]
fn try_next_u32_sha512() {
    let mut drbg = make_sha512();
    drbg.try_next_u32().unwrap();
}

#[test]
fn try_fill_bytes_small() {
    let mut drbg = make_sha256();
    let mut buf = [0u8; 16];
    drbg.try_fill_bytes(&mut buf).unwrap();
    // Check that the buffer was actually written (not all zeros).
    assert_ne!(buf, [0u8; 16]);
}

#[test]
fn try_fill_bytes_exact_max_generate() {
    // MAX_GENERATE_BYTES = 65_536 — exactly one chunk, no splitting needed.
    let mut drbg = make_sha256();
    let mut buf = vec![0u8; libcrux_hmac_drbg::MAX_GENERATE_BYTES];
    drbg.try_fill_bytes(&mut buf).unwrap();
}

#[test]
fn try_fill_bytes_larger_than_max_generate() {
    // try_fill_bytes must split the request into chunks of MAX_GENERATE_BYTES
    // and must NOT return RequestTooLarge.
    let mut drbg = make_sha256();
    let mut buf = vec![0u8; libcrux_hmac_drbg::MAX_GENERATE_BYTES + 1];
    drbg.try_fill_bytes(&mut buf).unwrap();
}

#[test]
fn try_fill_bytes_two_chunks() {
    // 2 × MAX_GENERATE_BYTES — two separate generate() calls internally.
    let mut drbg = make_sha256();
    let mut buf = vec![0u8; libcrux_hmac_drbg::MAX_GENERATE_BYTES * 2];
    drbg.try_fill_bytes(&mut buf).unwrap();
}

#[test]
fn try_fill_bytes_is_deterministic() {
    // Two identically seeded DRBGs should produce identical output.
    let mut a = make_sha256();
    let mut b = make_sha256();
    let mut out_a = [0u8; 64];
    let mut out_b = [0u8; 64];
    a.try_fill_bytes(&mut out_a).unwrap();
    b.try_fill_bytes(&mut out_b).unwrap();
    assert_eq!(out_a, out_b);
}

// ---------------------------------------------------------------------------
// TryCryptoRng — marker trait bound check
// ---------------------------------------------------------------------------

/// Accept any `TryCryptoRng` and return a random u64.
fn need_crypto_rng<R: TryCryptoRng>(rng: &mut R) -> u64 {
    rng.try_next_u64().unwrap()
}

#[test]
fn hmac_drbg_satisfies_try_crypto_rng_sha256() {
    let mut drbg = make_sha256();
    need_crypto_rng(&mut drbg);
}

#[test]
fn hmac_drbg_satisfies_try_crypto_rng_sha384() {
    let mut drbg = make_sha384();
    need_crypto_rng(&mut drbg);
}

#[test]
fn hmac_drbg_satisfies_try_crypto_rng_sha512() {
    let mut drbg = make_sha512();
    need_crypto_rng(&mut drbg);
}

// ---------------------------------------------------------------------------
// SeedableRng — from_seed / seed type
// ---------------------------------------------------------------------------

#[test]
fn from_seed_sha256_produces_output() {
    let seed = HmacDrbgSeed::<32>([0x42u8; 32]);
    let mut drbg = HmacDrbgSha256::from_seed(seed);
    let mut out = [0u8; 32];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn from_seed_sha384_produces_output() {
    let seed = HmacDrbgSeed::<48>([0x42u8; 48]);
    let mut drbg = HmacDrbgSha384::from_seed(seed);
    let mut out = [0u8; 48];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn from_seed_sha512_produces_output() {
    let seed = HmacDrbgSeed::<64>([0x42u8; 64]);
    let mut drbg = HmacDrbgSha512::from_seed(seed);
    let mut out = [0u8; 64];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn from_seed_is_deterministic() {
    // Same seed → same output.
    let seed_a = HmacDrbgSeed::<32>([0x7fu8; 32]);
    let seed_b = HmacDrbgSeed::<32>([0x7fu8; 32]);
    let mut a = HmacDrbgSha256::from_seed(seed_a);
    let mut b = HmacDrbgSha256::from_seed(seed_b);
    let mut out_a = [0u8; 64];
    let mut out_b = [0u8; 64];
    a.try_fill_bytes(&mut out_a).unwrap();
    b.try_fill_bytes(&mut out_b).unwrap();
    assert_eq!(out_a, out_b);
}

#[test]
fn from_seed_different_seeds_differ() {
    let mut a = HmacDrbgSha256::from_seed(HmacDrbgSeed::<32>([0x11u8; 32]));
    let mut b = HmacDrbgSha256::from_seed(HmacDrbgSeed::<32>([0x22u8; 32]));
    let mut out_a = [0u8; 32];
    let mut out_b = [0u8; 32];
    a.try_fill_bytes(&mut out_a).unwrap();
    b.try_fill_bytes(&mut out_b).unwrap();
    assert_ne!(out_a, out_b);
}

#[test]
fn from_seed_all_zero_is_accepted() {
    // SeedableRng is a deterministic interface — all-zero seeds are valid.
    let seed = HmacDrbgSeed::<32>([0u8; 32]);
    let mut drbg = HmacDrbgSha256::from_seed(seed);
    let mut out = [0u8; 32];
    drbg.try_fill_bytes(&mut out).unwrap();
}

// ---------------------------------------------------------------------------
// SeedableRng — from_rng
// ---------------------------------------------------------------------------

#[test]
fn from_rng_sha256_produces_output() {
    use rand::{rand_core::UnwrapErr, rngs::SysRng, TryRng as _};
    let mut drbg = HmacDrbgSha256::from_rng(&mut UnwrapErr(SysRng));
    let mut out = [0u8; 32];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn from_rng_sha384_produces_output() {
    use rand::{rand_core::UnwrapErr, rngs::SysRng, TryRng as _};
    let mut drbg = HmacDrbgSha384::from_rng(&mut UnwrapErr(SysRng));
    let mut out = [0u8; 48];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn from_rng_sha512_produces_output() {
    use rand::{rand_core::UnwrapErr, rngs::SysRng, TryRng as _};
    let mut drbg = HmacDrbgSha512::from_rng(&mut UnwrapErr(SysRng));
    let mut out = [0u8; 64];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn from_rng_two_calls_differ() {
    // Two separate from_rng calls draw fresh OS entropy → different DRBG state.
    use rand::{rand_core::UnwrapErr, rngs::SysRng, TryRng as _};
    let mut a = HmacDrbgSha256::from_rng(&mut UnwrapErr(SysRng));
    let mut b = HmacDrbgSha256::from_rng(&mut UnwrapErr(SysRng));
    let mut out_a = [0u8; 32];
    let mut out_b = [0u8; 32];
    a.try_fill_bytes(&mut out_a).unwrap();
    b.try_fill_bytes(&mut out_b).unwrap();
    // With 256 bits of OS entropy each, collision probability is negligible.
    assert_ne!(out_a, out_b);
}

// ---------------------------------------------------------------------------
// SeedableRng — try_from_rng
// ---------------------------------------------------------------------------

#[test]
fn try_from_rng_sha256_produces_output() {
    let mut drbg = HmacDrbgSha256::try_from_rng(&mut rand::rngs::SysRng).unwrap();
    let mut out = [0u8; 32];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn try_from_rng_sha384_produces_output() {
    let mut drbg = HmacDrbgSha384::try_from_rng(&mut rand::rngs::SysRng).unwrap();
    let mut out = [0u8; 48];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn try_from_rng_sha512_produces_output() {
    let mut drbg = HmacDrbgSha512::try_from_rng(&mut rand::rngs::SysRng).unwrap();
    let mut out = [0u8; 64];
    drbg.try_fill_bytes(&mut out).unwrap();
}

#[test]
fn try_from_rng_two_calls_differ() {
    // Two separate try_from_rng calls draw fresh OS entropy → different outputs.
    let mut a = HmacDrbgSha256::try_from_rng(&mut rand::rngs::SysRng).unwrap();
    let mut b = HmacDrbgSha256::try_from_rng(&mut rand::rngs::SysRng).unwrap();
    let mut out_a = [0u8; 32];
    let mut out_b = [0u8; 32];
    a.try_fill_bytes(&mut out_a).unwrap();
    b.try_fill_bytes(&mut out_b).unwrap();
    assert_ne!(out_a, out_b);
}

#[derive(Debug, PartialEq)]
struct TestError(&'static str);

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl core::error::Error for TestError {}

#[test]
fn try_from_rng_propagates_rng_error() {
    use rand::TryRng;

    /// A RNG that always fails after N bytes.
    struct FailAfter(usize);
    impl TryRng for FailAfter {
        type Error = TestError;

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Err(TestError("injected failure"))
        }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Err(TestError("injected failure"))
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            if dst.len() > self.0 {
                return Err(TestError("injected failure"));
            }
            self.0 -= dst.len();
            dst.fill(0x55);
            Ok(())
        }
    }

    // Fails on the second try_fill_bytes call (the nonce), not the first.
    // FailAfter(32) allows the entropy fill to succeed but rejects the nonce.
    let mut rng = FailAfter(32);
    let result = HmacDrbgSha256::try_from_rng(&mut rng);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, "injected failure");
}

#[test]
fn seed_default_is_all_zeros() {
    let seed = HmacDrbgSeed::<32>::default();
    assert_eq!(seed.0, [0u8; 32]);
}

#[test]
fn seed_as_ref_as_mut() {
    let mut seed = HmacDrbgSeed::<32>([0x55u8; 32]);
    assert_eq!(seed.as_ref(), &[0x55u8; 32]);
    seed.as_mut()[0] = 0xff;
    assert_eq!(seed.0[0], 0xff);
}
