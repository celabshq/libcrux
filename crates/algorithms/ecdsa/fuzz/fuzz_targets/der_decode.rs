//! Fuzz the P-256 DER decoder with arbitrary, likely-malformed input.
//!
//! `Signature::from_der` must never panic or exhibit UB on any input,
//! whether malformed or a well-formed `ECDSA-Sig-Value`. When decoding
//! succeeds, re-encoding the result and decoding it again must reproduce the
//! same `(r, s)`, i.e. `from_der` accepts a canonical superset of what
//! `to_der` produces.
#![no_main]

use libcrux_ecdsa::p256::{Signature, DER_SIGNATURE_MAX_LEN};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(sig) = Signature::from_der(data) else {
        return;
    };

    let der = sig.to_der();
    assert!(
        der.len() <= DER_SIGNATURE_MAX_LEN,
        "DER encoding exceeded its stated maximum"
    );

    let reparsed = Signature::from_der(&der).expect("re-encoding a decoded signature must decode");
    assert_eq!(
        reparsed.as_bytes(),
        sig.as_bytes(),
        "decode -> encode -> decode did not preserve (r, s)"
    );
});
