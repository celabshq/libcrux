//! Invalid-input tests for HKDF-SHA3: the error paths in `extract`/`expand` that don't
//! depend on Wycheproof vectors (there are none for SHA3, see `tests/wycheproof.rs` and
//! `tests/sha3_cross_check.rs`). A `prk` output buffer shorter than the hash length, an
//! `expand` `prk` input shorter than the hash length, and a requested output length
//! longer than `255 * hash_len`. Exercised through both the dynamic (`Algorithm`) and
//! typed (`Hkdf<Sha3_*>`) APIs.

use libcrux_hkdf::{expand, extract, Algorithm, ExpandError, ExtractError, Hkdf};
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};

macro_rules! invalid_input_tests {
    ($mod_name:ident, $algo:expr, $hash_len:literal, $typed:ty) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn extract_prk_buffer_too_short_dynamic() {
                let mut prk = vec![0u8; $hash_len - 1];
                let result = extract(
                    $algo,
                    prk.as_mut_slice().classify_ref_mut(),
                    b"salt".classify_ref(),
                    b"ikm".classify_ref(),
                );
                assert_eq!(result, Err(ExtractError::PrkTooShort));
            }

            #[test]
            fn extract_prk_buffer_too_short_typed() {
                let mut prk = vec![0u8; $hash_len - 1];
                let result = <$typed>::extract(
                    prk.as_mut_slice().classify_ref_mut(),
                    b"salt".classify_ref(),
                    b"ikm".classify_ref(),
                );
                assert_eq!(result, Err(ExtractError::PrkTooShort));
            }

            #[test]
            fn expand_prk_too_short_dynamic() {
                let prk = vec![0u8; $hash_len - 1];
                let mut okm = vec![0u8; 16];
                let result = expand(
                    $algo,
                    okm.as_mut_slice().classify_ref_mut(),
                    prk.as_slice().classify_ref(),
                    b"info",
                );
                assert_eq!(result, Err(ExpandError::PrkTooShort));
            }

            #[test]
            fn expand_prk_too_short_typed() {
                let prk = vec![0u8; $hash_len - 1];
                let mut okm = vec![0u8; 16];
                let result = <$typed>::expand(
                    okm.as_mut_slice().classify_ref_mut(),
                    prk.as_slice().classify_ref(),
                    b"info",
                );
                assert_eq!(result, Err(ExpandError::PrkTooShort));
            }

            #[test]
            fn expand_output_too_long_dynamic() {
                let prk = vec![0u8; $hash_len];
                let mut okm = vec![0u8; 255 * $hash_len + 1];
                let result = expand(
                    $algo,
                    okm.as_mut_slice().classify_ref_mut(),
                    prk.as_slice().classify_ref(),
                    b"info",
                );
                assert_eq!(result, Err(ExpandError::OutputTooLong));
            }

            #[test]
            fn expand_output_too_long_typed() {
                let prk = vec![0u8; $hash_len];
                let mut okm = vec![0u8; 255 * $hash_len + 1];
                let result = <$typed>::expand(
                    okm.as_mut_slice().classify_ref_mut(),
                    prk.as_slice().classify_ref(),
                    b"info",
                );
                assert_eq!(result, Err(ExpandError::OutputTooLong));
            }

            #[test]
            fn expand_output_at_maximum_allowed_size_succeeds() {
                // Boundary check: exactly `255 * hash_len` is still allowed; only a
                // strictly greater length is rejected.
                let prk = vec![0u8; $hash_len];
                let mut okm = vec![0u8; 255 * $hash_len];
                let result = expand(
                    $algo,
                    okm.as_mut_slice().classify_ref_mut(),
                    prk.as_slice().classify_ref(),
                    b"info",
                );
                assert!(result.is_ok());
            }
        }
    };
}

invalid_input_tests!(
    sha3_224,
    Algorithm::Sha3_224,
    28,
    Hkdf<libcrux_hkdf::Sha3_224>
);
invalid_input_tests!(
    sha3_256,
    Algorithm::Sha3_256,
    32,
    Hkdf<libcrux_hkdf::Sha3_256>
);
invalid_input_tests!(
    sha3_384,
    Algorithm::Sha3_384,
    48,
    Hkdf<libcrux_hkdf::Sha3_384>
);
invalid_input_tests!(
    sha3_512,
    Algorithm::Sha3_512,
    64,
    Hkdf<libcrux_hkdf::Sha3_512>
);
