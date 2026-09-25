#![cfg(feature = "zeroize")]

use libcrux_ml_dsa::{MLDSAKeyPair, MLDSASigningKey, MLDSAVerificationKey};
use zeroize::Zeroize;

const SK_SIZE: usize = 64;
const VK_SIZE: usize = 32;

#[test]
fn signing_key_zeroize_clears_value() {
    let mut key = MLDSASigningKey::<SK_SIZE>::new([0xAA; SK_SIZE]);
    assert_eq!(key.as_slice(), &[0xAA; SK_SIZE]);

    key.zeroize();

    assert_eq!(key.as_slice(), &[0u8; SK_SIZE]);
}

#[test]
fn keypair_zeroize_clears_signing_key_only() {
    let mut keypair = MLDSAKeyPair::<VK_SIZE, SK_SIZE> {
        signing_key: MLDSASigningKey::<SK_SIZE>::new([0xAA; SK_SIZE]),
        verification_key: MLDSAVerificationKey::<VK_SIZE>::new([0xBB; VK_SIZE]),
    };
    assert_eq!(keypair.signing_key.as_slice(), &[0xAA; SK_SIZE]);
    assert_eq!(keypair.verification_key.as_slice(), &[0xBB; VK_SIZE]);

    keypair.zeroize();

    assert_eq!(keypair.signing_key.as_slice(), &[0u8; SK_SIZE]);
    assert_eq!(keypair.verification_key.as_slice(), &[0xBB; VK_SIZE]);
}
