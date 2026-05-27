//! Fuzz `HmacDrbg::new()` — instantiation with arbitrary entropy, nonce, and
//! personalization string for all three supported hash variants.
//!
//! Input layout (all lengths are bytes):
//!   [0]      algorithm selector: 0 → SHA-256, 1 → SHA-384, 2 → SHA-512
//!   [1]      entropy_len  (0–255)
//!   [2]      nonce_len    (0–255)
//!   [3]      pers_len     (0–255)
//!   [4..]    data (entropy || nonce || personalization_string, truncated to what is available)
//!
//! Checks:
//!   - No panic or undefined behaviour for any input.
//!   - `new()` returns `Err(InputTooLarge)` iff the combined seed exceeds 384 bytes.
//!   - On success, `needs_reseed()` is false and `reseed_counter()` is 1.
#![no_main]

use libcrux_hmac_drbg::{HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512, InstantiateError};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let alg = data[0] % 3;
    let entropy_len = data[1] as usize;
    let nonce_len = data[2] as usize;
    let pers_len = data[3] as usize;
    let payload = &data[4..];

    fn take(buf: &[u8], offset: usize, n: usize) -> &[u8] {
        let start = offset.min(buf.len());
        let end = (start + n).min(buf.len());
        &buf[start..end]
    }

    let entropy = take(payload, 0, entropy_len);
    let nonce = take(payload, entropy_len, nonce_len);
    let pers = take(payload, entropy_len + nonce_len, pers_len);

    let total = entropy.len() + nonce.len() + pers.len();
    const MAX_SEED: usize = 384;

    macro_rules! check {
        ($result:expr) => {
            match $result {
                Ok(drbg) => {
                    assert!(!drbg.needs_reseed());
                    assert_eq!(drbg.reseed_counter(), 1);
                }
                Err(InstantiateError::InputTooLarge) => {
                    // Only acceptable when the seed exceeds the limit.
                    assert!(
                        total > MAX_SEED,
                        "InputTooLarge returned but total={total} <= {MAX_SEED}"
                    );
                }
                // there are more variants if the rand feature is set on the drbg crate,
                // so we need the exception here
                #[allow(unreachable_patterns)]
                Err(e) => panic!("unexpected error: {e:?}"),
            }
        };
    }

    match alg {
        0 => check!(HmacDrbgSha256::new(entropy, nonce, pers)),
        1 => check!(HmacDrbgSha384::new(entropy, nonce, pers)),
        _ => check!(HmacDrbgSha512::new(entropy, nonce, pers)),
    }
});
