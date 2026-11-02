use ctgrind_test::{classify, declassify};
use libcrux_hmac::Algorithm;
use std::hint::black_box;

#[inline(never)]
fn test_hmac(alg: Algorithm) {
    let key = black_box([0x42u8; 32]);
    let data = black_box([0x17u8; 200]);
    classify(&key);
    classify(&data);

    let mut tag = [0u8; 64];
    libcrux_hmac::hmac(alg, &key, &data, &mut tag);
    declassify(&tag);
    println!("hmac {alg:?}: {:02x?}", &tag[..4]);
}

pub fn main() {
    for alg in [
        Algorithm::Sha256,
        Algorithm::Sha384,
        Algorithm::Sha512,
        Algorithm::Sha3_224,
        Algorithm::Sha3_256,
        Algorithm::Sha3_384,
        Algorithm::Sha3_512,
    ] {
        test_hmac(alg);
    }
}
