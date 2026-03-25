#![no_main]

use libcrux_aes::aes_ccm_128::portable::PortableAesCcm128;
use libcrux_traits::aead::slice::Aead;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 + 12 + 5 + 16 {
        // We want at least a key, nonce, aad, and tag.
        return;
    }

    let key = &data[0..16];
    let nonce = &data[16..16 + 12];
    let aad = &data[16 + 12..16 + 12 + 5];
    let tag = &data[16 + 12 + 5..16 + 12 + 5 + 16];
    let ctxt = &data[16 + 12 + 5 + 16..];

    let mut ptxt = vec![0u8; ctxt.len()];

    // Decryption will mostly fail due to tag mismatch; we're checking for panics/crashes.
    let _ = PortableAesCcm128::decrypt(&mut ptxt, key, nonce, aad, ctxt, tag);
});
