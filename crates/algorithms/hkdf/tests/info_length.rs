//! Tests for the upper bound on the length of `info` in HKDF expand.
//!
//! HACL SHA-2 expand computes the length of an internal buffer as
//! `hash_len + info_len + 1` in a wrapping `u32`. For `info_len`
//! within `hash_len + 1` of `u32::MAX` this overflows and leads to a
//! panic on slicing a buffer. `info` lengths above `u32::MAX -
//! hash_len - 1` must instead be rejected with `ArgumentTooLong`, for
//! every algorithm and every public entry point.
//!
//! The `info` buffers here are allocated zeroed, which on Linux and
//! macOS maps pages lazily.  The rejection path never reads them, so
//! the tests don't actually use ~4 GiB of memory.  They are
//! restricted to those 64-bit platforms because elsewhere the
//! allocation may be committed eagerly or not be possible at all.
#![cfg(all(
    target_pointer_width = "64",
    any(target_os = "linux", target_os = "macos")
))]

use libcrux_hkdf::{expand, hkdf, Algorithm, ArrayReferenceExpandError, ExpandError, Hkdf};
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};

/// One byte more than any `info` length the tests below use.
fn big_info() -> Vec<u8> {
    vec![0u8; u32::MAX as usize + 1]
}

macro_rules! info_length_tests {
    ($mod_name:ident, $algo:expr, $hash_len:literal, $typed:ty) => {
        mod $mod_name {
            use super::*;

            const MAX_INFO_LEN: usize = u32::MAX as usize - $hash_len - 1;

            /// Lengths that must be rejected: the smallest one over the limit, the top of the
            /// former panicking range, and the smallest one that doesn't fit in a `u32`.
            const TOO_LONG: [usize; 3] =
                [MAX_INFO_LEN + 1, u32::MAX as usize, u32::MAX as usize + 1];

            #[test]
            fn info_too_long_is_rejected() {
                let big = big_info();
                let prk = [0u8; $hash_len];
                let mut okm = [0u8; $hash_len];

                for len in TOO_LONG {
                    let info = &big[..len];

                    let result = expand(
                        $algo,
                        okm.as_mut_slice().classify_ref_mut(),
                        prk.as_slice().classify_ref(),
                        info,
                    );
                    assert_eq!(
                        result,
                        Err(ExpandError::ArgumentTooLong),
                        "expand, len {len}"
                    );

                    let result = hkdf(
                        $algo,
                        okm.as_mut_slice().classify_ref_mut(),
                        b"salt".classify_ref(),
                        b"ikm".classify_ref(),
                        info,
                    );
                    assert_eq!(result, Err(ExpandError::ArgumentTooLong), "hkdf, len {len}");

                    let result = <$typed>::expand(
                        okm.as_mut_slice().classify_ref_mut(),
                        prk.as_slice().classify_ref(),
                        info,
                    );
                    assert_eq!(
                        result,
                        Err(ExpandError::ArgumentTooLong),
                        "typed expand, len {len}"
                    );

                    let result = <$typed>::hkdf(
                        okm.as_mut_slice().classify_ref_mut(),
                        b"salt".classify_ref(),
                        b"ikm".classify_ref(),
                        info,
                    );
                    assert_eq!(
                        result,
                        Err(ExpandError::ArgumentTooLong),
                        "typed hkdf, len {len}"
                    );

                    let result = <$typed>::expand_arrayref(
                        okm.as_mut_slice().classify_ref_mut(),
                        prk.classify_ref(),
                        info,
                    );
                    assert_eq!(
                        result,
                        Err(ArrayReferenceExpandError::ArgumentTooLong),
                        "typed expand_arrayref, len {len}"
                    );

                    let result = libcrux_hkdf::$mod_name::expand_arrayref(
                        okm.as_mut_slice().classify_ref_mut(),
                        prk.classify_ref(),
                        info,
                    );
                    assert_eq!(
                        result,
                        Err(ArrayReferenceExpandError::ArgumentTooLong),
                        "expand_arrayref, len {len}"
                    );
                }
            }

            /// The largest allowed `info` length is still accepted. This hashes ~4 GiB, so it
            /// is ignored by default; run it with
            /// `cargo test --release -p libcrux-hkdf --test info_length -- --ignored`.
            #[test]
            #[ignore]
            fn max_info_len_is_accepted() {
                let info = vec![0u8; MAX_INFO_LEN];
                let prk = [0u8; $hash_len];
                let mut okm = [0u8; $hash_len];

                let result = expand(
                    $algo,
                    okm.as_mut_slice().classify_ref_mut(),
                    prk.as_slice().classify_ref(),
                    &info,
                );
                assert_eq!(result, Ok(()));
            }
        }
    };
}

info_length_tests!(
    sha2_256,
    Algorithm::Sha256,
    32,
    Hkdf<libcrux_hkdf::Sha2_256>
);
info_length_tests!(
    sha2_384,
    Algorithm::Sha384,
    48,
    Hkdf<libcrux_hkdf::Sha2_384>
);
info_length_tests!(
    sha2_512,
    Algorithm::Sha512,
    64,
    Hkdf<libcrux_hkdf::Sha2_512>
);
info_length_tests!(
    sha3_224,
    Algorithm::Sha3_224,
    28,
    Hkdf<libcrux_hkdf::Sha3_224>
);
info_length_tests!(
    sha3_256,
    Algorithm::Sha3_256,
    32,
    Hkdf<libcrux_hkdf::Sha3_256>
);
info_length_tests!(
    sha3_384,
    Algorithm::Sha3_384,
    48,
    Hkdf<libcrux_hkdf::Sha3_384>
);
info_length_tests!(
    sha3_512,
    Algorithm::Sha3_512,
    64,
    Hkdf<libcrux_hkdf::Sha3_512>
);
