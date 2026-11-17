use ctgrind_test::{classify, declassify};
use std::hint::black_box;

#[inline(never)]
fn test_sha2() {
    // Longer than one block, to also cover the multi-block path.
    let message = black_box([0x42u8; 200]);
    classify(&message);

    let d224 = libcrux_sha2::sha224(&message);
    let d256 = libcrux_sha2::sha256(&message);
    let d384 = libcrux_sha2::sha384(&message);
    let d512 = libcrux_sha2::sha512(&message);
    declassify(&d224);
    declassify(&d256);
    declassify(&d384);
    declassify(&d512);
    println!("sha224: {:02x?}", &d224[..4]);
    println!("sha256: {:02x?}", &d256[..4]);
    println!("sha384: {:02x?}", &d384[..4]);
    println!("sha512: {:02x?}", &d512[..4]);
}

pub fn main() {
    test_sha2();
}
