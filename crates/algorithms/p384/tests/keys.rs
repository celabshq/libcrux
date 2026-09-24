use libcrux_p384::{PrivateKey, PublicKey};

// The private key `1`, whose corresponding public key is the standard NIST
// P-384 base point G (SEC2 / FIPS 186-4), independently of anything computed
// by this crate.
const SK: &str = "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
const PK_UNCOMPRESSED: &str = "04aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab73617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f";

#[test]
fn public_key_derivation_matches_known_vector() {
    let sk_bytes = hex::decode(SK).unwrap();
    let sk = PrivateKey::try_from(sk_bytes.as_slice()).unwrap();

    // Exercises From<&PrivateKey> for PublicKey (generator scalar-mult),
    // the one path neither `wycheproof.rs` nor the `generate()` test calls.
    let pk = PublicKey::from(&sk);

    let mut uncompressed = [0u8; 97];
    pk.to_uncompressed(&mut uncompressed);
    assert_eq!(hex::encode(uncompressed), PK_UNCOMPRESSED);
}
