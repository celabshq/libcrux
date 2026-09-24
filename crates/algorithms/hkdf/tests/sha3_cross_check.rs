//! No official test vectors exist for HKDF-SHA3 (RFC 5869 only covers SHA-1/SHA-256;
//! Wycheproof's `hkdf_*_test.json` files stop at SHA-512; RFC 9688 registers the
//! HKDF-with-SHA3-* algorithm identifiers but publishes no test vectors). So this
//! cross-checks `libcrux_hkdf`'s SHA3 output against the independent RustCrypto
//! `hkdf` + `sha3` crates.

use hkdf::Hkdf;
use libcrux_hkdf::Algorithm;
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

struct TestCase {
    ikm: &'static [u8],
    salt: &'static [u8],
    info: &'static [u8],
    len: usize,
}

const CASES: &[TestCase] = &[
    // Empty salt.
    TestCase {
        ikm: b"input key material",
        salt: b"",
        info: b"context info",
        len: 32,
    },
    // Empty info.
    TestCase {
        ikm: b"input key material",
        salt: b"some salt",
        info: b"",
        len: 32,
    },
    // Output length exactly one hash block (single HMAC call in expand).
    TestCase {
        ikm: b"input key material",
        salt: b"some salt",
        info: b"context info",
        len: 28,
    },
    // Output length crossing a multi-block expand boundary.
    TestCase {
        ikm: b"a longer piece of input key material to exercise the extract step",
        salt: b"some salt",
        info: b"context info",
        len: 200,
    },
    // Zero-length output.
    TestCase {
        ikm: b"input key material",
        salt: b"some salt",
        info: b"context info",
        len: 0,
    },
];

macro_rules! cross_check {
    ($fn_name:ident, $algo:expr, $reference:ty) => {
        #[test]
        fn $fn_name() {
            for case in CASES {
                let mut libcrux_okm = vec![0u8; case.len];
                libcrux_hkdf::hkdf(
                    $algo,
                    libcrux_okm.as_mut_slice().classify_ref_mut(),
                    case.salt.classify_ref(),
                    case.ikm.classify_ref(),
                    case.info,
                )
                .unwrap();

                let mut reference_okm = vec![0u8; case.len];
                let hk = Hkdf::<$reference>::new(Some(case.salt), case.ikm);
                hk.expand(case.info, &mut reference_okm).unwrap();

                assert_eq!(libcrux_okm, reference_okm);
            }
        }
    };
}

cross_check!(sha3_224_matches_rustcrypto, Algorithm::Sha3_224, Sha3_224);
cross_check!(sha3_256_matches_rustcrypto, Algorithm::Sha3_256, Sha3_256);
cross_check!(sha3_384_matches_rustcrypto, Algorithm::Sha3_384, Sha3_384);
cross_check!(sha3_512_matches_rustcrypto, Algorithm::Sha3_512, Sha3_512);
