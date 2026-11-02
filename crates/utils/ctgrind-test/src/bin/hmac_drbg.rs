use ctgrind_test::{classify, declassify};
use libcrux_hmac_drbg::{HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512};
use std::hint::black_box;

macro_rules! test_drbg {
    ($name:ident, $drbg:ty) => {
        #[inline(never)]
        fn $name() {
            let entropy = black_box([0x42u8; 64]);
            let reseed_entropy = black_box([0x43u8; 64]);
            let nonce = black_box([0x24u8; 32]);
            classify(&entropy);
            classify(&reseed_entropy);

            let mut drbg = <$drbg>::new(&entropy, &nonce, b"personalization").unwrap();
            let mut out = [0u8; 100];
            drbg.generate(&mut out, b"").unwrap();
            drbg.reseed(&reseed_entropy, b"").unwrap();
            drbg.generate(&mut out, b"additional").unwrap();
            declassify(&out);
            println!("{}: {:02x?}", stringify!($drbg), &out[..4]);
        }
    };
}

test_drbg!(test_sha256, HmacDrbgSha256);
test_drbg!(test_sha384, HmacDrbgSha384);
test_drbg!(test_sha512, HmacDrbgSha512);

pub fn main() {
    test_sha256();
    test_sha384();
    test_sha512();
}
