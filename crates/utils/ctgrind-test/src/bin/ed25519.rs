use ctgrind_test::{classify, declassify};
use std::hint::black_box;

#[inline(never)]
fn test_secret_to_public() {
    let sk = black_box([0x42u8; 32]);
    classify(&sk);

    let mut pk = [0u8; 32];
    libcrux_ed25519::secret_to_public(&mut pk, &sk);
    declassify(&pk);
    println!("ed25519 public key: {:02x?}", &pk[..4]);
}

#[inline(never)]
fn test_sign() {
    let sk = black_box([0x42u8; 32]);
    let msg = black_box(*b"test message");
    classify(&sk);

    let sig = libcrux_ed25519::sign(&msg, &sk).unwrap();
    declassify(&sig);
    println!("ed25519 signature: {:02x?}", &sig[..4]);
}

pub fn main() {
    test_secret_to_public();
    test_sign();
}
