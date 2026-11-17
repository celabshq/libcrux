//! Regression test: ML-KEM private keys must be validated before
//! decapsulation (FIPS 203, Algorithm 21 "ML-KEM.Decaps" input check,
//! Section 7.3 item 3).
//!
//! An ML-KEM decapsulation key is `dk_pke || ek || H(ek) ||
//! z`. Decapsulation must reject a key where the stored `H(ek)` does
//! not match the embedded `ek`.  In `libcrux-ml-kem` callers are
//! expected to call the key validation functions themselves, but
//! `libcrux-kem` does not provide them. Therefore key validation
//! should happen automatically before every decapsulation via
//! `libcrux-kem`.

use libcrux_kem::{
    key_gen, Algorithm, Error, MlKem1024PrivateKey, MlKem512PrivateKey, MlKem768PrivateKey,
    PrivateKey,
};

/// Rank `k` of each ML-KEM parameter set. In the encoded private key `ek`
/// occupies bytes `384k..768k + 32` and `H(ek)` bytes `768k + 32..768k + 64`.
const CASES: [(Algorithm, usize, usize); 4] = [
    (Algorithm::MlKem512, 2, MlKem512PrivateKey::len()),
    (Algorithm::MlKem768, 3, MlKem768PrivateKey::len()),
    (Algorithm::MlKem1024, 4, MlKem1024PrivateKey::len()),
    // The ML-KEM key comes first in the encoded hybrid key, followed by the
    // 32-byte X25519 key, so the ML-KEM offsets are unchanged.
    (
        Algorithm::X25519MlKem768Draft00,
        3,
        MlKem768PrivateKey::len(),
    ),
];

#[test]
fn decapsulate_rejects_private_key_with_wrong_hash() {
    let mut rng = rand::rng();

    for (algorithm, k, mlkem_sk_len) in CASES {
        let (sk, pk) = key_gen(algorithm, &mut rng).unwrap();
        let (ss, ct) = pk.encapsulate(&mut rng).unwrap();

        let encoded = sk.encode();
        let ek = 384 * k..768 * k + 32;
        let h_ek = 768 * k + 32..768 * k + 64;
        assert_eq!(h_ek.end + 32, mlkem_sk_len, "{algorithm:?}: wrong layout");

        // The unmodified key still decapsulates correctly.
        let decoded = PrivateKey::decode(algorithm, &encoded).unwrap();
        assert_eq!(
            ct.decapsulate(&decoded).unwrap().encode(),
            ss.encode(),
            "{algorithm:?}: valid private key was not accepted"
        );

        // Flip a bit in the stored hash, and in the embedded public key.
        for (region, index) in [("H(ek)", h_ek.start), ("ek", ek.start)] {
            let mut tampered = encoded.clone();
            tampered[index] ^= 1;
            let tampered = PrivateKey::decode(algorithm, &tampered).unwrap();

            assert!(
                matches!(ct.decapsulate(&tampered), Err(Error::InvalidPrivateKey)),
                "{algorithm:?}: decapsulated with a private key whose {region} was modified"
            );
        }
    }
}
