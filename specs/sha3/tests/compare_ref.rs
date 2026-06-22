/// Compare our SHA3 implementation against the reference libcrux-sha3 crate.

#[test]
fn sha3_256_vs_reference() {
    let mut ref_digest = [0u8; 32];
    libcrux_sha3::portable::sha256(&mut ref_digest, b"");
    let our_digest = hacspec_sha3::sha3_256(b"");
    assert_eq!(ref_digest, our_digest);
}

#[test]
fn shake128_abc_vs_reference() {
    let mut ref_out = [0u8; 32];
    libcrux_sha3::portable::shake128(&mut ref_out, b"abc");
    let our_out = hacspec_sha3::shake128::<32>(b"abc");
    eprintln!("SHAKE128 ref: {:02x?}", ref_out.to_vec());
    eprintln!("SHAKE128 our: {:02x?}", our_out.to_vec());
    assert_eq!(ref_out, our_out);
}

#[test]
fn shake256_abc_vs_reference() {
    let mut ref_out = [0u8; 32];
    libcrux_sha3::portable::shake256(&mut ref_out, b"abc");
    let our_out = hacspec_sha3::shake256::<32>(b"abc");
    eprintln!("SHAKE256 ref: {:02x?}", ref_out.to_vec());
    eprintln!("SHAKE256 our: {:02x?}", our_out.to_vec());
    assert_eq!(ref_out, our_out);
}
