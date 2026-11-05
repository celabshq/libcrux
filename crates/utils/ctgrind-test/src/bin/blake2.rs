use ctgrind_test::{classify, declassify};
use libcrux_blake2::{Blake2bBuilder, Blake2sBuilder};
use std::hint::black_box;

#[inline(never)]
fn test_blake2b() {
    let key = black_box([0x42u8; 64]);
    let message = black_box([0x17u8; 200]);
    classify(&key);
    classify(&message);

    let mut hasher = Blake2bBuilder::new_keyed_const(&key).build_const_digest_len::<64>();
    hasher.update(&message).unwrap();
    let mut digest = [0u8; 64];
    hasher.finalize(&mut digest);
    declassify(&digest);
    println!("blake2b: {:02x?}", &digest[..4]);
}

#[inline(never)]
fn test_blake2s() {
    let key = black_box([0x42u8; 32]);
    let message = black_box([0x17u8; 200]);
    classify(&key);
    classify(&message);

    let mut hasher = Blake2sBuilder::new_keyed_const(&key).build_const_digest_len::<32>();
    hasher.update(&message).unwrap();
    let mut digest = [0u8; 32];
    hasher.finalize(&mut digest);
    declassify(&digest);
    println!("blake2s: {:02x?}", &digest[..4]);
}

pub fn main() {
    test_blake2b();
    test_blake2s();
}
