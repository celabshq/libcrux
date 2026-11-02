use ctgrind_test::{classify, declassify};
use std::hint::black_box;

#[inline(never)]
fn test_kmac() {
    let key = black_box([0x42u8; 32]);
    let data = black_box([0x17u8; 200]);
    classify(&key);
    classify(&data);

    let mut tag128 = [0u8; 32];
    libcrux_kmac::kmac_128(&mut tag128, &key, &data, b"custom");
    declassify(&tag128);
    println!("kmac128: {:02x?}", &tag128[..4]);

    let mut tag256 = [0u8; 64];
    libcrux_kmac::kmac_256(&mut tag256, &key, &data, b"custom");
    declassify(&tag256);
    println!("kmac256: {:02x?}", &tag256[..4]);
}

pub fn main() {
    test_kmac();
}
