//! Fuzz an arbitrary sequence of `reseed` and `generate` calls on a DRBG
//! instantiated with fuzzer-controlled entropy.
//!
//! This target exercises the full state machine and interactions between
//! operations, including the reseed counter tracking.
//!
//! Input layout:
//!   [0]      algorithm selector: 0 → SHA-256, 1 → SHA-384, 2 → SHA-512
//!   [1]      entropy_len for `new()` (0–255)
//!   [2]      nonce_len   for `new()` (0–255)
//!   [3..]    operations stream (parsed below)
//!
//! Operations stream: each operation is a tagged record.
//!   Tag 0x00  — reseed
//!     [1]  entropy_len (0–127)
//!     [2]  ai_len      (0–127)
//!     [3..3+entropy_len+ai_len]  entropy || additional_input
//!
//!   Tag 0x01  — generate
//!     [1..3]  output_len (little-endian u16)
//!     [4]     ai_len (0–127)
//!     [5..5+ai_len]  additional_input
//!
//! Any other tag byte terminates the stream.
//!
//! Checks:
//!   - No panic or undefined behaviour for any input.
//!   - `ReseedRequired` is only returned when `needs_reseed()` is true.
//!   - After a successful reseed, `needs_reseed()` is false.
//!   - `reseed_counter()` is always in [1, RESEED_INTERVAL + 1].
#![no_main]

use libcrux_hmac_drbg::{
    GenerateError, HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512, ReseedError, RESEED_INTERVAL,
};
use libfuzzer_sys::fuzz_target;

/// A minimal cursor over a byte slice.
struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    fn take(&mut self, n: usize) -> &'a [u8] {
        let end = (self.pos + n).min(self.data.len());
        let slice = &self.data[self.pos..end];
        self.pos = end;
        slice
    }
    fn byte(&mut self) -> Option<u8> {
        let b = self.data.get(self.pos)?;
        self.pos += 1;
        Some(*b)
    }
    fn u16_le(&mut self) -> Option<u16> {
        let lo = self.byte()?;
        let hi = self.byte()?;
        Some(u16::from_le_bytes([lo, hi]))
    }
}

macro_rules! run_lifecycle {
    ($drbg_new:expr, $data:expr) => {{
        let mut cur = Cursor::new($data);

        let entropy_len = cur.byte().unwrap_or(0) as usize;
        let nonce_len = cur.byte().unwrap_or(0) as usize;
        let entropy = cur.take(entropy_len);
        let nonce = cur.take(nonce_len);

        let mut drbg = match $drbg_new(entropy, nonce) {
            Ok(d) => d,
            Err(_) => return, // seed too large — skip
        };

        // Bound iterations so the fuzzer doesn't get stuck in long loops.
        const MAX_OPS: usize = 32;

        for _ in 0..MAX_OPS {
            let tag = match cur.byte() {
                Some(b) => b,
                None => break,
            };

            match tag & 0x01 {
                // Reseed
                0x00 => {
                    let ent_len = (cur.byte().unwrap_or(0) as usize) & 0x7f;
                    let ai_len = (cur.byte().unwrap_or(0) as usize) & 0x7f;
                    let ent = cur.take(ent_len);
                    let ai = cur.take(ai_len);

                    match drbg.reseed(ent, ai) {
                        Ok(()) => {
                            assert!(!drbg.needs_reseed());
                            assert_eq!(drbg.reseed_counter(), 1);
                        }
                        Err(ReseedError::InputTooLarge) => {} // expected for large inputs

                        // there are more variants if the health tests feature is set on
                        // the drbg crate, so we need the exception here
                        #[allow(unreachable_patterns)]
                        Err(e) => panic!("unexpected reseed error: {e:?}"),
                    }
                }

                // Generate
                _ => {
                    let output_len = cur.u16_le().unwrap_or(0) as usize;
                    let ai_len = (cur.byte().unwrap_or(0) as usize) & 0x7f;
                    let ai = cur.take(ai_len);
                    let additional_input = if ai.is_empty() { &[] } else { ai };

                    let needs_reseed_before = drbg.needs_reseed();
                    let counter_before = drbg.reseed_counter();
                    let mut out = vec![0u8; output_len];

                    match drbg.generate(&mut out, additional_input) {
                        Ok(()) => {
                            assert!(!needs_reseed_before);
                            assert_eq!(drbg.reseed_counter(), counter_before + 1);
                        }
                        Err(GenerateError::ReseedRequired) => {
                            assert!(needs_reseed_before);
                            // Counter must not have changed.
                            assert_eq!(drbg.reseed_counter(), counter_before);
                        }
                        Err(GenerateError::RequestInvalid) => {
                            // output_len == 0 or > MAX_GENERATE_BYTES: counter unchanged.
                            assert_eq!(drbg.reseed_counter(), counter_before);
                        }
                        Err(e) => panic!("unexpected generate error: {e:?}"),
                    }

                    assert!(drbg.reseed_counter() <= RESEED_INTERVAL + 1);
                }
            }
        }
    }};
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }

    let alg = data[0] % 3;
    let rest = &data[1..];

    match alg {
        0 => run_lifecycle!(
            |ent: &[u8], non: &[u8]| HmacDrbgSha256::new(ent, non, &[]),
            rest
        ),
        1 => run_lifecycle!(
            |ent: &[u8], non: &[u8]| HmacDrbgSha384::new(ent, non, &[]),
            rest
        ),
        _ => run_lifecycle!(
            |ent: &[u8], non: &[u8]| HmacDrbgSha512::new(ent, non, &[]),
            rest
        ),
    }
});
