//! Ported from ../../sha3/tests/compare_ref.rs, the byte-level spec's suite.
//! Same vectors, second consumer: here they run against the bit-level spec
//! through its byte wrappers (`bytes::*`, i.e. `h2b` in front and `b2h`
//! behind).
/// Compare our SHA3 implementation against the reference libcrux-sha3 crate.

#[test]
fn sha3_256_vs_reference() {
    let mut ref_digest = [0u8; 32];
    libcrux_sha3::portable::sha256(&mut ref_digest, b"");
    let our_digest = hacspec_sha3_pedantic::bytes::sha3_256(b"");
    assert_eq!(ref_digest, our_digest);
}

#[test]
fn shake128_abc_vs_reference() {
    let mut ref_out = [0u8; 32];
    libcrux_sha3::portable::shake128(&mut ref_out, b"abc");
    let our_out = hacspec_sha3_pedantic::bytes::shake128(b"abc", 32);
    eprintln!("SHAKE128 ref: {:02x?}", ref_out.to_vec());
    eprintln!("SHAKE128 our: {:02x?}", our_out.to_vec());
    assert_eq!(&ref_out[..], &our_out[..]);
}

#[test]
fn shake256_abc_vs_reference() {
    let mut ref_out = [0u8; 32];
    libcrux_sha3::portable::shake256(&mut ref_out, b"abc");
    let our_out = hacspec_sha3_pedantic::bytes::shake256(b"abc", 32);
    eprintln!("SHAKE256 ref: {:02x?}", ref_out.to_vec());
    eprintln!("SHAKE256 our: {:02x?}", our_out.to_vec());
    assert_eq!(&ref_out[..], &our_out[..]);
}
