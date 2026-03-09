#![no_main]

use libcrux_aes::aes_ccm_256::short_tag::portable::PortableAesCcm256ShortTag;
use libcrux_traits::aead::slice::Aead;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 + 12 + 5 + 8 {
        // We want at least a key, nonce, aad, and tag.
        return;
    }

    let key = &data[0..32];
    let nonce = &data[32..32 + 12];
    let aad = &data[32 + 12..32 + 12 + 5];
    let tag = &data[32 + 12 + 5..32 + 12 + 5 + 8];
    let ctxt = &data[32 + 12 + 5 + 8..];

    let mut ptxt = vec![0u8; ctxt.len()];

    // Decryption will mostly fail due to tag mismatch; we're checking for panics/crashes.
    let _ = PortableAesCcm256ShortTag::decrypt(&mut ptxt, key, nonce, aad, ctxt, tag);
});
