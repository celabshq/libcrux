//! The bit-level counterpart of ../../sha3/tests/sponge_decomposition.rs.
//!
//! That test pins `keccak == squeeze ∘ absorb` across the SHA-3 / SHAKE rates
//! and delimiters. This spec has no such split to pin: Algorithm 8 is one
//! function, with absorbing as Step 6 and squeezing as Steps 7-10. What is
//! worth pinning here instead is the seam the two specs meet at, over exactly
//! the same matrix of rates, delimiters and output lengths: the neighbour
//! folds the domain suffix and the leading padding bit into one delimiter byte
//! (`0x06`, `0x1f`), whereas here the suffix is a bit string of Sec. 6 and the
//! padding comes from `pad10*1`. Agreeing on every rate is what says the two
//! conventions describe the same padding.

use hacspec_sha3_pedantic::bits::{b2h, concat, h2b_full};
use hacspec_sha3_pedantic::sha3::{HASH_SUFFIX, XOF_SUFFIX};
use hacspec_sha3_pedantic::sponge::keccak_c;

const B: usize = 1600;

/// `KECCAK[c]` on `M || suffix`, where the neighbour passes `delim` instead.
///
/// The neighbour's `rate` is in bytes (144 for SHA3-224); `KECCAK[c]` is
/// parameterised by the capacity in bits, `c = 1600 - 8·rate`.
fn via_bits(rate_bytes: usize, suffix: &[bool], msg: &[u8], out_bytes: usize) -> Vec<u8> {
    let n = concat(&h2b_full(msg), suffix);
    b2h(&keccak_c(B - 8 * rate_bytes, &n, 8 * out_bytes))
}

fn check<const OUT: usize>(rate: usize, delim: u8, msg: &[u8]) {
    let suffix: &[bool] = match delim {
        0x06 => &HASH_SUFFIX, // `01`, Sec. 6.1
        0x1f => &XOF_SUFFIX,  // `1111`, Sec. 6.2
        _ => panic!("unknown delimiter"),
    };
    let via_keccak = hacspec_sha3::sponge::keccak::<OUT>(rate, delim, msg);
    assert_eq!(
        via_bits(rate, suffix, msg, OUT),
        via_keccak.to_vec(),
        "rate={rate}, delim={delim:#x}, msg.len()={}, out={OUT}",
        msg.len()
    );
}

#[test]
fn keccak_agrees_across_rates_and_delimiters() {
    let empty: [u8; 0] = [];
    let short = b"hello world";
    let long: Vec<u8> = (0u8..200).collect();

    // SHA3-224: rate=144, delim=0x06, out=28
    check::<28>(144, 0x06, &empty);
    check::<28>(144, 0x06, short);
    check::<28>(144, 0x06, &long);
    // SHA3-256: rate=136, delim=0x06, out=32
    check::<32>(136, 0x06, &empty);
    check::<32>(136, 0x06, short);
    check::<32>(136, 0x06, &long);
    // SHA3-384: rate=104, delim=0x06, out=48
    check::<48>(104, 0x06, &empty);
    check::<48>(104, 0x06, short);
    check::<48>(104, 0x06, &long);
    // SHA3-512: rate=72, delim=0x06, out=64
    check::<64>(72, 0x06, &empty);
    check::<64>(72, 0x06, short);
    check::<64>(72, 0x06, &long);
    // SHAKE128: rate=168, delim=0x1f — short and long output exercise the squeeze loop.
    check::<16>(168, 0x1f, &empty);
    check::<16>(168, 0x1f, short);
    check::<16>(168, 0x1f, &long);
    check::<200>(168, 0x1f, &empty);
    check::<200>(168, 0x1f, short);
    check::<200>(168, 0x1f, &long);
    // SHAKE256: rate=136, delim=0x1f.
    check::<64>(136, 0x1f, &empty);
    check::<64>(136, 0x1f, short);
    check::<300>(136, 0x1f, &empty);
    check::<300>(136, 0x1f, short);
    check::<300>(136, 0x1f, &long);
}
