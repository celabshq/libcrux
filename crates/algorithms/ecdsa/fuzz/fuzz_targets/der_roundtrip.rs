//! Fuzz the P-256 DER encoder against its own decoder.
//!
//! The first 64 bytes of the input are used directly as the raw `(r, s)`
//! scalars (including degenerate values like all-zero or all-0xff, and
//! values with the high bit set, which need extra sign-byte handling in the
//! encoder). Encoding a signature and decoding it back must always succeed
//! and reproduce the original `(r, s)`, and the encoding must never exceed
//! the encoder's own stated maximum size, without panicking or triggering
//! UB for any input.
#![no_main]

use libcrux_ecdsa::p256::{Signature, DER_SIGNATURE_MAX_LEN};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 64 {
        return;
    }

    let mut r = [0u8; 32];
    let mut s = [0u8; 32];
    r.copy_from_slice(&data[0..32]);
    s.copy_from_slice(&data[32..64]);

    let sig = Signature::from_raw(r, s);
    let der = sig.to_der();
    assert!(
        der.len() <= DER_SIGNATURE_MAX_LEN,
        "DER encoding exceeded its stated maximum"
    );

    let decoded = Signature::from_der(&der).expect("a freshly encoded signature must decode");
    assert_eq!(
        decoded.as_bytes(),
        sig.as_bytes(),
        "DER round-trip did not preserve (r, s)"
    );
});
