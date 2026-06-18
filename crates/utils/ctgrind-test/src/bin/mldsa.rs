use libcrux_ml_dsa::{ml_dsa_44, ml_dsa_65, ml_dsa_87};
use std::hint::black_box;

#[inline(never)]
fn test_sign_mldsa_44() {
    let key_pair = ml_dsa_44::generate_key_pair(black_box([0u8; 32]));
    let sk = &key_pair.signing_key;

    let message = b"test message";
    let context = b"";
    let sig = ml_dsa_44::sign(sk, message, context, black_box([0u8; 32])).unwrap();

    println!("mldsa44 signature: {:02x?}", &sig.as_slice()[..4]);
}

#[inline(never)]
fn test_sign_mldsa_65() {
    let key_pair = ml_dsa_65::generate_key_pair(black_box([0u8; 32]));
    let sk = &key_pair.signing_key;

    let message = b"test message";
    let context = b"";
    let sig = ml_dsa_65::sign(sk, message, context, black_box([0u8; 32])).unwrap();

    println!("mldsa65 signature: {:02x?}", &sig.as_slice()[..4]);
}

#[inline(never)]
fn test_sign_mldsa_87() {
    let key_pair = ml_dsa_87::generate_key_pair(black_box([0u8; 32]));
    let sk = &key_pair.signing_key;

    let message = b"test message";
    let context = b"";
    let sig = ml_dsa_87::sign(sk, message, context, black_box([0u8; 32])).unwrap();

    println!("mldsa87 signature: {:02x?}", &sig.as_slice()[..4]);
}

// XXX test other parameter configurations

pub fn main() {
    #[cfg(not(valgrind_ct_test))]
    {
        compile_error!(
            "ctgrind_test mldsa binary needs to be compiled with '--cfg valgrind_ct_test'"
        );
    }
    test_sign_mldsa_44();
    test_sign_mldsa_65();
    test_sign_mldsa_87();
}
