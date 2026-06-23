//! Pins down `keccak == squeeze ∘ absorb`, exercised across the
//! SHA-3 / SHAKE rates and delimiters.

use hacspec_sha3::sponge::{absorb, keccak, squeeze};

fn check<const OUT: usize>(rate: usize, delim: u8, msg: &[u8]) {
    let via_keccak = keccak::<OUT>(rate, delim, msg);
    let via_split = squeeze::<OUT>(absorb(rate, delim, msg), rate);
    assert_eq!(
        via_keccak,
        via_split,
        "keccak != squeeze(absorb) for rate={rate}, delim={delim:#x}, msg.len()={}",
        msg.len()
    );
}

#[test]
fn keccak_equals_squeeze_of_absorb() {
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
    check::<64>(136, 0x1f, short);
    check::<300>(136, 0x1f, &empty);
    check::<300>(136, 0x1f, short);
    check::<300>(136, 0x1f, &long);
}
