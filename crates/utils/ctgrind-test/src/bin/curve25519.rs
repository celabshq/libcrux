use ctgrind_test::{classify, declassify};
use std::hint::black_box;

#[inline(never)]
fn test_secret_to_public() {
    let sk = black_box([0x42u8; 32]);
    classify(&sk);

    let mut pk = [0u8; 32];
    libcrux_curve25519::secret_to_public(&mut pk, &sk);
    declassify(&pk);
    println!("x25519 public key: {:02x?}", &pk[..4]);
}

/// The hacl `ecdh` returns the all-zero check as a bool, without branching on it.
#[inline(never)]
fn test_ecdh() {
    let their_sk = black_box([0x24u8; 32]);
    let mut their_pk = [0u8; 32];
    libcrux_curve25519::secret_to_public(&mut their_pk, &their_sk);

    let sk = black_box([0x42u8; 32]);
    classify(&sk);

    let mut ss = [0u8; 32];
    let ok = libcrux_curve25519::hacl::ecdh(&mut ss, &sk, &their_pk);
    declassify(&ok);
    declassify(&ss);
    println!("x25519 ecdh ({ok}): {:02x?}", &ss[..4]);
}

pub fn main() {
    test_secret_to_public();
    test_ecdh();
}
