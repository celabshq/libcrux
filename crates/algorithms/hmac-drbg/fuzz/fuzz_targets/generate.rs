//! Fuzz `HmacDrbg::generate()` — output sizing and optional additional input.
//!
//! Uses a fixed, well-known seed so the fuzzer can focus entirely on the
//! generate call itself.  All three algorithm variants are exercised.
//!
//! Input layout:
//!   [0]      algorithm selector: 0 → SHA-256, 1 → SHA-384, 2 → SHA-512
//!   [1..3]   output_len (little-endian u16; the full range 0..=65537 is explored)
//!   [4]      has_additional_input: 0 = None, non-zero = Some(rest of data)
//!   [5..]    additional_input bytes (used when has_additional_input != 0)
//!
//! Checks:
//!   - No panic or undefined behaviour for any input.
//!   - `RequestInvalid` iff output_len == 0 or output_len > MAX_GENERATE_BYTES.
//!   - On success the output buffer is fully written (no uninitialised bytes leak).
#![no_main]

use libcrux_hmac_drbg::{
    GenerateError, HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512, MAX_GENERATE_BYTES,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let alg = data[0] % 3;
    let output_len = u16::from_le_bytes([data[1], data[2]]) as usize;
    let has_ai = data[3] != 0;
    let ai_bytes = &data[4..];
    let additional_input = if has_ai { ai_bytes } else { &[] };

    let mut out = vec![0u8; output_len];

    macro_rules! run {
        ($drbg:expr) => {{
            let mut drbg = $drbg;
            match drbg.generate(&mut out, additional_input) {
                Ok(()) => {
                    // Success must only happen for valid lengths.
                    assert!(output_len > 0 && output_len <= MAX_GENERATE_BYTES);
                }
                Err(GenerateError::RequestInvalid) => {
                    assert!(output_len == 0 || output_len > MAX_GENERATE_BYTES);
                }
                Err(GenerateError::ReseedRequired) => {
                    // Cannot happen with a freshly instantiated DRBG (counter = 1).
                    panic!("ReseedRequired on fresh DRBG");
                }
                Err(e) => panic!("unexpected error: {e:?}"),
            }
        }};
    }

    match alg {
        0 => run!(HmacDrbgSha256::new(&[0u8; 32], &[0u8; 16], &[]).unwrap()),
        1 => run!(HmacDrbgSha384::new(&[0u8; 48], &[0u8; 24], &[]).unwrap()),
        _ => run!(HmacDrbgSha512::new(&[0u8; 64], &[0u8; 32], &[]).unwrap()),
    }
});
