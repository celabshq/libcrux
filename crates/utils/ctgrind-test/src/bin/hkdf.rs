use ctgrind_test::{classify, declassify};
use libcrux_hkdf::Algorithm;
use std::hint::black_box;

#[inline(never)]
fn test_hkdf(alg: Algorithm) {
    let salt = black_box([0x24u8; 32]);
    let ikm = black_box([0x42u8; 32]);
    classify(&ikm);

    let mut prk = [0u8; 64];
    let prk_len = match alg {
        Algorithm::Sha256 | Algorithm::Sha3_256 => 32,
        Algorithm::Sha384 | Algorithm::Sha3_384 => 48,
        Algorithm::Sha512 | Algorithm::Sha3_512 => 64,
        Algorithm::Sha3_224 => 28,
    };
    libcrux_hkdf::extract(alg, &mut prk[..prk_len], &salt, &ikm).unwrap();

    let mut okm = [0u8; 100];
    libcrux_hkdf::expand(alg, &mut okm, &prk[..prk_len], b"info").unwrap();
    declassify(&okm);
    println!("hkdf {alg:?}: {:02x?}", &okm[..4]);
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
        test_hkdf(alg);
    }
}
