mod util;

use libcrux_ecdsa::p256::{self, PrivateKey, PublicKey};

use crate::util::*;

/// Private keys whose public keys have an `X` starting with `0x02` and `0x03`,
/// so that the first 33 bytes of `X || Y` are also a valid compressed point.
const SK_HEX: [&str; 2] = [
    "42000000000000000000000000000000000000000000000000000000000001AA",
    "4200000000000000000000000000000000000000000000000000000000000106",
];

fn private_keys() -> [PrivateKey; 2] {
    SK_HEX.map(|sk| PrivateKey::try_from(&hex_str_to_array::<[u8; 32]>(sk)).unwrap())
}

fn raw(pk: &PublicKey) -> [u8; 64] {
    *AsRef::<[u8; 64]>::as_ref(pk)
}

/// The encodings of `pk`: uncompressed, compressed and `X || Y`.
fn encodings(pk: &[u8; 64]) -> [Vec<u8>; 3] {
    let mut uncompressed = vec![0x04];
    uncompressed.extend_from_slice(pk);
    let mut compressed = vec![0x02 | (pk[63] & 1)];
    compressed.extend_from_slice(&pk[..32]);
    [uncompressed, compressed, pk.to_vec()]
}

#[test]
fn raw_public_key() {
    for (sk, prefix) in private_keys().iter().zip([0x02, 0x03]) {
        let pk = raw(&sk.public_key().unwrap());
        assert_eq!(pk[0], prefix);
        assert!(p256::compressed_to_coordinates(&pk[..33]).is_ok());

        // `X || Y` must not be read as a compressed point.
        assert_eq!(raw(&PublicKey::try_from(&pk).unwrap()), pk);
    }
}

#[test]
fn all_encodings() {
    for sk in private_keys() {
        let pk = raw(&sk.public_key().unwrap());

        for encoded in encodings(&pk) {
            let parsed = PublicKey::try_from(encoded.as_slice()).unwrap();
            assert_eq!(raw(&parsed), pk);
        }
    }
}

#[test]
fn wrong_length() {
    let [sk, _] = private_keys();
    let pk = raw(&sk.public_key().unwrap());

    // Each encoding with a trailing byte.
    let too_long = encodings(&pk).map(|mut encoded| {
        encoded.push(0);
        encoded
    });

    for encoded in too_long.iter().map(Vec::as_slice).chain([&pk[..63], &[]]) {
        assert!(
            PublicKey::try_from(encoded).is_err(),
            "accepted a public key of {} bytes",
            encoded.len()
        );
    }

    assert!(p256::uncompressed_to_coordinates(&too_long[0]).is_err());
    assert!(p256::compressed_to_coordinates(&too_long[1]).is_err());
}
