//! Regression test: ML-KEM public keys must be validated before encapsulation
//! (FIPS 203, Algorithm 20 "ML-KEM.Encaps" input check, Section 7.2 item 2).
//!
//! `PublicKey::decode` rejects keys that fail the modulus check, but
//! keys can also be built directly (the variants and fields are
//! public) or through `tls_codec`. In `libcrux-ml-kem` callers are
//! expected to call the key validation functions themselves, but
//! `libcrux-kem` does not provide them. Therefore key validation
//! should happen automatically before every encapsulation via
//! `libcrux-kem`.

use libcrux_kem::{
    key_gen, Algorithm, Error, MlKem1024PublicKey, MlKem512PublicKey, MlKem768PublicKey, PublicKey,
};
use tls_codec::{Deserialize, Serialize};

const ALGORITHMS: [Algorithm; 5] = [
    Algorithm::MlKem512,
    Algorithm::MlKem768,
    Algorithm::MlKem1024,
    Algorithm::X25519MlKem768Draft00,
    Algorithm::XWingKemDraft06,
];

/// All bytes `0xff` decode to the coefficient 4095, which is not reduced
/// modulo q = 3329, so the modulus check fails.
fn invalid<T: for<'a> TryFrom<&'a [u8]>>(len: usize) -> T {
    T::try_from(&vec![0xff; len]).ok().unwrap()
}

/// Replace the ML-KEM part of `pk` with an invalid key.
fn break_mlkem_part(pk: &mut PublicKey) {
    match pk {
        PublicKey::MlKem512(k) => *k = invalid(MlKem512PublicKey::len()),
        PublicKey::MlKem768(k) => *k = invalid(MlKem768PublicKey::len()),
        PublicKey::MlKem1024(k) => *k = invalid(MlKem1024PublicKey::len()),
        PublicKey::X25519MlKem768Draft00(k) => k.mlkem = invalid(MlKem768PublicKey::len()),
        PublicKey::XWingKemDraft06(k) => k.pk_m = invalid(MlKem768PublicKey::len()),
        PublicKey::X25519(_) | PublicKey::P256(_) => unreachable!(),
    }
}

fn tls_round_trip(pk: &PublicKey) -> PublicKey {
    let bytes = pk.tls_serialize_detached().unwrap();
    PublicKey::tls_deserialize_exact(&bytes).unwrap()
}

/// The seed length `encapsulate_derand` expects.
fn derand_seed(algorithm: Algorithm) -> Vec<u8> {
    match algorithm {
        Algorithm::X25519MlKem768Draft00 | Algorithm::XWingKemDraft06 => vec![7; 64],
        _ => vec![7; 32],
    }
}

#[test]
fn encapsulate_rejects_invalid_mlkem_public_key() {
    let mut rng = rand::rng();

    for algorithm in ALGORITHMS {
        let (_, mut pk) = key_gen(algorithm, &mut rng).unwrap();
        let seed = derand_seed(algorithm);

        // Valid keys still encapsulate, also after a TLS round trip.
        for (path, pk) in [("direct", &pk), ("tls_codec", &tls_round_trip(&pk))] {
            assert!(
                pk.encapsulate(&mut rng).is_ok(),
                "{algorithm:?} ({path}): encapsulate rejected a valid key"
            );
            assert!(
                pk.encapsulate_derand(&seed).is_ok(),
                "{algorithm:?} ({path}): encapsulate_derand rejected a valid key"
            );
        }

        break_mlkem_part(&mut pk);
        assert!(
            matches!(
                PublicKey::decode(algorithm, &pk.encode()),
                Err(Error::InvalidPublicKey)
            ),
            "{algorithm:?}: decode accepted the invalid key"
        );

        for (path, pk) in [("direct", &pk), ("tls_codec", &tls_round_trip(&pk))] {
            assert!(
                matches!(pk.encapsulate(&mut rng), Err(Error::InvalidPublicKey)),
                "{algorithm:?} ({path}): encapsulate accepted an invalid key"
            );
            assert!(
                matches!(pk.encapsulate_derand(&seed), Err(Error::InvalidPublicKey)),
                "{algorithm:?} ({path}): encapsulate_derand accepted an invalid key"
            );
        }
    }
}
