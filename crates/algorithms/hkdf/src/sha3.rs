//! Generic HKDF extract/expand built on [`libcrux_hmac::HmacState`], used for the SHA3
//! variants (no HACL SHA3-HMAC exists to generate `extract`/`expand` from, unlike the
//! SHA2 variants in `src/hacl.rs`).
//!
//! Unlike the code in `hacl.rs` this code is not extracted from verified HACL*/F*
//! sources, so it is structured to be checked step by step against the verified
//! `expand_sha2_256`/`extract_sha2_256` in `src/hacl.rs`:
//!
//! - `extract_generic` is exactly `extract_sha2_256`: a single `HMAC(salt, ikm)` call.
//! - `expand_generic` computes the same `T(i) = HMAC(PRK, T(i-1) || info || i)` chain as
//!   `expand_sha2_256`, using the same starting counter (`1`) and increment (`+1` per
//!   round). `expand_sha2_256` special-cases the first round (`i == 0`) by omitting the
//!   (empty) `T(-1)` prefix, and handles a final partial block separately after the loop
//!   over full blocks. Here that's expressed as a single loop where the first round
//!   naturally has `t_len == 0` (nothing to prepend, matching the `i == 0` case) and
//!   every round copies only `min(LEN, len - written)` bytes out (naturally producing a
//!   truncated final block without a separate branch). `len == 0` performs zero HMAC
//!   calls, matching `expand_sha2_256`'s `n == 0 && n * tlen < len == false`.
//!
//! Function signatures intentionally match the HACL functions in `src/hacl.rs` so the
//! same `impl_hkdf!` macro invocation can call either backend.
//!
//! Every function here is `pub(crate)`: they are pure internal glue for `impl_hkdf!`,
//! never meant to be called directly, and their `.expect()`s below rely on their sole
//! caller (`extract_arrayref`/`expand_arrayref` in `hkdf.rs`) having already validated
//! `salt`/`ikm`/`prk`/`info` via `checked_u32(...)?` before invoking them — the same
//! trust boundary the HACL `extract_sha2_*`/`expand_sha2_*` functions rely on, which
//! similarly assume pre-validated lengths and aren't `pub(crate)` only because `hacl.rs`
//! is machine-generated code we don't hand-edit.

use libcrux_hmac::{HmacSha3_224, HmacSha3_256, HmacSha3_384, HmacSha3_512, HmacState};

fn extract_generic<const LEN: usize, H: HmacState<LEN>>(
    prk: &mut [u8],
    salt: &[u8],
    _salt_len: u32, // Unused, only here for hacl.rs compatibility.
    ikm: &[u8],
    _ikm_len: u32, // Unused, only here for hacl.rs compatibility.
) {
    // Caller (`extract_arrayref` in `hkdf.rs`) already checked `salt`/`ikm`
    // fit in a `u32`, the same trust boundary the HACL `extract_sha2_*`
    // functions rely on.
    // These debug asserts are just for documentation.
    debug_assert_eq!(salt.len(), _salt_len as usize);
    debug_assert_eq!(ikm.len(), _ikm_len as usize);

    let mut h = H::new(salt).expect("salt too long");
    h.update(ikm).expect("ikm too long");
    h.finalize((&mut prk[..LEN]).try_into().unwrap());
}

fn expand_generic<const LEN: usize, H: HmacState<LEN>>(
    okm: &mut [u8],
    prk: &[u8],
    _prk_len: u32,
    info: &[u8],
    _info_len: u32,
    len: u32,
) {
    // Caller (`expand_arrayref` in `hkdf.rs`) already checked `prk`/`info`
    // fit in a `u32`, the same trust boundary the HACL `expand_sha2_*`
    // functions rely on.
    // These debug asserts are just for documentation.
    debug_assert_eq!(info.len(), _info_len as usize);
    debug_assert_eq!(prk.len(), _prk_len as usize);

    let len = len as usize;
    let mut t = [0u8; LEN];
    let mut t_len = 0_usize;
    let mut counter: u8 = 1;
    let mut written = 0_usize;

    while written < len {
        let mut h = H::new(prk).expect("prk too long");
        h.update(&t[..t_len]).unwrap();
        h.update(info).expect("info too long");
        h.update(&[counter]).unwrap();
        h.finalize(&mut t);
        t_len = LEN;

        let take = core::cmp::min(LEN, len - written);
        okm[written..written + take].copy_from_slice(&t[..take]);
        written += take;
        counter = counter.wrapping_add(1);
    }
}

pub(crate) fn extract_sha3_224(
    prk: &mut [u8],
    salt: &[u8],
    salt_len: u32,
    ikm: &[u8],
    ikm_len: u32,
) {
    extract_generic::<28, HmacSha3_224>(prk, salt, salt_len, ikm, ikm_len)
}

pub(crate) fn expand_sha3_224(
    okm: &mut [u8],
    prk: &[u8],
    prk_len: u32,
    info: &[u8],
    info_len: u32,
    len: u32,
) {
    expand_generic::<28, HmacSha3_224>(okm, prk, prk_len, info, info_len, len)
}

pub(crate) fn extract_sha3_256(
    prk: &mut [u8],
    salt: &[u8],
    salt_len: u32,
    ikm: &[u8],
    ikm_len: u32,
) {
    extract_generic::<32, HmacSha3_256>(prk, salt, salt_len, ikm, ikm_len)
}

pub(crate) fn expand_sha3_256(
    okm: &mut [u8],
    prk: &[u8],
    prk_len: u32,
    info: &[u8],
    info_len: u32,
    len: u32,
) {
    expand_generic::<32, HmacSha3_256>(okm, prk, prk_len, info, info_len, len)
}

pub(crate) fn extract_sha3_384(
    prk: &mut [u8],
    salt: &[u8],
    salt_len: u32,
    ikm: &[u8],
    ikm_len: u32,
) {
    extract_generic::<48, HmacSha3_384>(prk, salt, salt_len, ikm, ikm_len)
}

pub(crate) fn expand_sha3_384(
    okm: &mut [u8],
    prk: &[u8],
    prk_len: u32,
    info: &[u8],
    info_len: u32,
    len: u32,
) {
    expand_generic::<48, HmacSha3_384>(okm, prk, prk_len, info, info_len, len)
}

pub(crate) fn extract_sha3_512(
    prk: &mut [u8],
    salt: &[u8],
    salt_len: u32,
    ikm: &[u8],
    ikm_len: u32,
) {
    extract_generic::<64, HmacSha3_512>(prk, salt, salt_len, ikm, ikm_len)
}

pub(crate) fn expand_sha3_512(
    okm: &mut [u8],
    prk: &[u8],
    prk_len: u32,
    info: &[u8],
    info_len: u32,
    len: u32,
) {
    expand_generic::<64, HmacSha3_512>(okm, prk, prk_len, info, info_len, len)
}

#[cfg(test)]
mod tests {
    //! This tests the generic loop in isolation from SHA3: instantiate it with the
    //! already-verified `HmacSha256` and compare against the official RFC 5869 SHA-256
    //! test vectors. This catches a bug in the loop restructuring itself, independent of
    //! whether HMAC-SHA3 is correct (which `tests/sha3_cross_check.rs` covers).
    //!
    //! This test requires alloc.
    extern crate alloc;

    use alloc::{vec, vec::Vec};

    use libcrux_hmac::HmacSha256;

    use super::*;

    fn run(
        ikm: &[u8],
        salt: &[u8],
        info: &[u8],
        l: usize,
        expected_prk: &[u8],
        expected_okm: &[u8],
    ) {
        let mut prk = [0u8; 32];
        extract_generic::<32, HmacSha256>(&mut prk, salt, salt.len() as u32, ikm, ikm.len() as u32);
        assert_eq!(prk, expected_prk);

        let mut okm = vec![0u8; l];
        expand_generic::<32, HmacSha256>(&mut okm, &prk, 32, info, info.len() as u32, l as u32);
        assert_eq!(okm, expected_okm);
    }

    #[test]
    fn rfc5869_test_case_1() {
        run(
            &[0x0b; 22],
            &hex_bytes("000102030405060708090a0b0c"),
            &hex_bytes("f0f1f2f3f4f5f6f7f8f9"),
            42,
            &hex_bytes("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5"),
            &hex_bytes(
                "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865",
            ),
        );
    }

    #[test]
    fn rfc5869_test_case_2_long_inputs_multi_block_expand() {
        let ikm = hex_bytes(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\
             202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f\
             404142434445464748494a4b4c4d4e4f",
        );
        let salt = hex_bytes(
            "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f\
             808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f\
             a0a1a2a3a4a5a6a7a8a9aaabacadaeaf",
        );
        let info = hex_bytes(
            "b0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
             d0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeef\
             f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
        );
        let expected_prk =
            hex_bytes("06a6b88c5853361a06104c9ceb35b45cef760014904671014a193f40c15fc244");
        let expected_okm = hex_bytes(
            "b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c5\
             9045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71cc\
             30c58179ec3e87c14c01d5c1f3434f1d87",
        );
        run(&ikm, &salt, &info, 82, &expected_prk, &expected_okm);
    }

    #[test]
    fn rfc5869_test_case_3_empty_salt_and_info() {
        run(
            &[0x0b; 22],
            &[],
            &[],
            42,
            &hex_bytes("19ef24a32c717b167f33a91d6f648bdf96596776afdb6377ac434c1c293ccb04"),
            &hex_bytes(
                "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8",
            ),
        );
    }

    fn hex_bytes(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }
}
