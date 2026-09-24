#![no_main]

use libcrux_p384::derive_ecdh;
use libfuzzer_sys::fuzz_target;

// First 48 bytes are treated as the private key scalar, the rest as the
// peer's SEC1-encoded public key.
fuzz_target!(|data: &[u8]| {
    if data.len() < 48 {
        return;
    }
    let (sk_bytes, pk_bytes) = data.split_at(48);
    let _ = derive_ecdh(sk_bytes, pk_bytes);
});
