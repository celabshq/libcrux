use ctgrind_test::{classify, declassify};
use libcrux_hmac::HmacSha256;
use std::hint::black_box;

#[inline(never)]
fn test_two_step() {
    let shared_secret = black_box([0x42u8; 32]);
    let salt = black_box([0x24u8; 32]);
    let iv = black_box([0x11u8; 32]);
    classify(&shared_secret);

    let mut out = [0u8; 100];
    libcrux_nist_kdf::two_step::kdf::<32, HmacSha256>(
        &mut out,
        &shared_secret,
        &salt,
        &iv,
        &[b"info"],
    )
    .unwrap();
    declassify(&out);
    println!("two-step kdf: {:02x?}", &out[..4]);
}

#[inline(never)]
fn test_feedback() {
    let k_in = black_box([0x42u8; 32]);
    let iv = black_box([0x11u8; 32]);
    classify(&k_in);

    let mut out = [0u8; 100];
    libcrux_nist_kdf::feedback::kdf::<32, HmacSha256>(&mut out, &k_in, &iv, &[b"info"]).unwrap();
    declassify(&out);
    println!("feedback kdf: {:02x?}", &out[..4]);
}

pub fn main() {
    test_two_step();
    test_feedback();
}
