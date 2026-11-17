use ctgrind_test::{classify, declassify, SecretRng};
use libcrux_ecdsa::p256::{self, PrivateKey};
use std::hint::black_box;

#[inline(never)]
fn test_sign() {
    let sk = PrivateKey::try_from(&black_box([0x42u8; 32])).unwrap();
    let sk_bytes: &[u8; 32] = sk.as_ref();
    classify(sk_bytes);

    // The nonce comes from the RNG, which marks it as secret.
    let sig = p256::rand::sign(
        libcrux_sha2::Algorithm::Sha256,
        b"test message",
        &sk,
        &mut SecretRng(1),
    )
    .unwrap();
    let (r, s) = sig.as_bytes();
    declassify(r);
    declassify(s);
    println!("ecdsa p256 signature: {:02x?}", &r[..4]);
}

#[inline(never)]
fn test_public_key() {
    let sk = PrivateKey::try_from(&black_box([0x42u8; 32])).unwrap();
    let sk_bytes: &[u8; 32] = sk.as_ref();
    classify(sk_bytes);

    let pk = sk.public_key().unwrap();
    declassify(&pk.0);
    println!("ecdsa p256 public key: {:02x?}", &pk.0[..4]);
}

pub fn main() {
    test_sign();
    test_public_key();
}
