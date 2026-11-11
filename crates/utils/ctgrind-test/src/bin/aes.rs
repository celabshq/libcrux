//! AES-GCM and AES-CCM with a secret key. Each mode is tested with the multiplexed implementation,
//! which uses the hardware implementation where available, and with the portable one.

use ctgrind_test::{classify, declassify};
use libcrux_aes::{Aead as _, AeadConsts};
use std::hint::black_box;

macro_rules! test_aead {
    ($name:ident, $ty:ty, $algo:expr) => {
        #[inline(never)]
        fn $name() {
            let algo = $algo;
            let key_bytes = black_box([0x42u8; 32]);
            let key_bytes = &key_bytes[..<$ty as AeadConsts>::KEY_LEN];
            let nonce_bytes = black_box([0x24u8; 16]);
            let msg = black_box([0x71u8; 100]);
            classify(key_bytes);
            classify(&msg);

            let key = algo.new_key(key_bytes).unwrap();
            let nonce = algo
                .new_nonce(&nonce_bytes[..<$ty as AeadConsts>::NONCE_LEN])
                .unwrap();
            let mut ct = [0u8; 100];
            let mut tag_bytes = [0u8; 16];
            let tag = algo
                .new_tag_mut(&mut tag_bytes[..<$ty as AeadConsts>::TAG_LEN])
                .unwrap();
            key.encrypt(&mut ct, tag, nonce, b"aad", &msg).unwrap();
            declassify(&ct);
            declassify(&tag_bytes);

            let tag = algo
                .new_tag(&tag_bytes[..<$ty as AeadConsts>::TAG_LEN])
                .unwrap();
            let mut pt = [0u8; 100];
            let ok = key.decrypt(&mut pt, nonce, b"aad", &ct, tag).is_ok();
            declassify(&pt);
            println!("{}: decrypt {ok}, ct {:02x?}", stringify!($name), &ct[..4]);
        }
    };
}

test_aead!(gcm128, libcrux_aes::AesGcm128, libcrux_aes::AesGcm128);
test_aead!(
    gcm128_portable,
    libcrux_aes::aes_gcm_128::portable::PortableAesGcm128,
    libcrux_aes::aes_gcm_128::portable::PortableAesGcm128
);
test_aead!(gcm256, libcrux_aes::AesGcm256, libcrux_aes::AesGcm256);
test_aead!(
    gcm256_portable,
    libcrux_aes::aes_gcm_256::portable::PortableAesGcm256,
    libcrux_aes::aes_gcm_256::portable::PortableAesGcm256
);
test_aead!(ccm128, libcrux_aes::AesCcm128, libcrux_aes::AesCcm128);
test_aead!(
    ccm128_portable,
    libcrux_aes::aes_ccm_128::portable::PortableAesCcm128,
    libcrux_aes::aes_ccm_128::portable::PortableAesCcm128
);
test_aead!(ccm256, libcrux_aes::AesCcm256, libcrux_aes::AesCcm256);
test_aead!(
    ccm256_portable,
    libcrux_aes::aes_ccm_256::portable::PortableAesCcm256,
    libcrux_aes::aes_ccm_256::portable::PortableAesCcm256
);

pub fn main() {
    gcm128();
    gcm128_portable();
    gcm256();
    gcm256_portable();
    ccm128();
    ccm128_portable();
    ccm256();
    ccm256_portable();
}
