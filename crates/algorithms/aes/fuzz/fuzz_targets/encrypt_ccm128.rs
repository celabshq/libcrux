#![no_main]

use libcrux_aes::aes_ccm_128::portable::PortableAesCcm128;
use libcrux_traits::aead::slice::Aead;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 + 12 + 7 {
        // We want at least a key, nonce, and a few input bytes.
        return;
    }

    let key = &data[0..16];
    let nonce = &data[16..16 + 12];
    let aad = &data[16 + 12..16 + 12 + 5];

    let mut ctxt = vec![0u8; data.len()];
    let mut tag_bytes = [0u8; 16];

    PortableAesCcm128::encrypt(&mut ctxt, &mut tag_bytes, key, nonce, aad, data).unwrap();

    let mut roundtrip = vec![0u8; data.len()];
    PortableAesCcm128::decrypt(&mut roundtrip, key, nonce, aad, &ctxt, &tag_bytes).unwrap();
    assert_eq!(data, roundtrip.as_slice());
});
