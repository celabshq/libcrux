//! The hacl P-256 functions return the private key (and nonce) validity as a bool, without
//! branching on it.

use ctgrind_test::{classify, declassify};
use std::hint::black_box;

#[inline(never)]
fn test_validate_private_key() {
    let sk = black_box([0x42u8; 32]);
    classify(&sk);

    let ok = libcrux_p256::validate_private_key(&sk);
    declassify(&ok);
    println!("p256 validate_private_key: {ok}");
}

#[inline(never)]
fn test_dh_initiator() {
    let sk = black_box([0x42u8; 32]);
    classify(&sk);

    let mut pk = [0u8; 64];
    let ok = libcrux_p256::dh_initiator(&mut pk, &sk);
    declassify(&ok);
    declassify(&pk);
    println!("p256 dh_initiator ({ok}): {:02x?}", &pk[..4]);
}

#[inline(never)]
fn test_dh_responder() {
    let their_sk = black_box([0x24u8; 32]);
    let mut their_pk = [0u8; 64];
    assert!(libcrux_p256::dh_initiator(&mut their_pk, &their_sk));

    let sk = black_box([0x42u8; 32]);
    classify(&sk);

    let mut ss = [0u8; 64];
    let ok = libcrux_p256::dh_responder(&mut ss, &their_pk, &sk);
    declassify(&ok);
    declassify(&ss);
    println!("p256 dh_responder ({ok}): {:02x?}", &ss[..4]);
}

#[inline(never)]
fn test_ecdsa_sign() {
    let sk = black_box([0x42u8; 32]);
    let nonce = black_box([0x24u8; 32]);
    let msg = black_box(*b"test message");
    classify(&sk);
    classify(&nonce);

    let mut sig = [0u8; 64];
    let ok = libcrux_p256::ecdsa_sign_p256_sha2(&mut sig, msg.len() as u32, &msg, &sk, &nonce);
    declassify(&ok);
    declassify(&sig);
    println!("p256 ecdsa signature ({ok}): {:02x?}", &sig[..4]);
}

pub fn main() {
    test_validate_private_key();
    test_dh_initiator();
    test_dh_responder();
    test_ecdsa_sign();
}
