use libcrux_kem::{
    key_gen,
    Algorithm::{X25519MlKem768Draft00, XWingKemDraft06},
    Error, MlKem768PrivateKey, MlKem768PublicKey, PrivateKey, PublicKey,
};

const X25519_KEY_LEN: usize = 32;

#[test]
fn hybrid_private_key_decode_rejects_invalid_lengths() {
    const MLKEM_KEY_LEN: usize = MlKem768PrivateKey::len();
    const KEY_LEN: usize = MLKEM_KEY_LEN + X25519_KEY_LEN;

    for len in [
        0,
        1,
        MLKEM_KEY_LEN - 1,
        MLKEM_KEY_LEN,
        KEY_LEN - 1,
        KEY_LEN + 1,
    ] {
        let encoded = vec![0; len];
        assert!(
            matches!(
                PrivateKey::decode(X25519MlKem768Draft00, &encoded),
                Err(Error::InvalidPrivateKey)
            ),
            "accepted an invalid {len}-byte private key"
        );
    }
}

#[test]
fn hybrid_public_key_decode_rejects_invalid_lengths() {
    const MLKEM_KEY_LEN: usize = MlKem768PublicKey::len();
    const KEY_LEN: usize = MLKEM_KEY_LEN + X25519_KEY_LEN;

    for algorithm in [X25519MlKem768Draft00, XWingKemDraft06] {
        for len in [
            0,
            1,
            MLKEM_KEY_LEN - 1,
            MLKEM_KEY_LEN,
            KEY_LEN - 1,
            KEY_LEN + 1,
        ] {
            let encoded = vec![0; len];
            assert!(
                matches!(
                    PublicKey::decode(algorithm, &encoded),
                    Err(Error::InvalidPublicKey)
                ),
                "{algorithm:?} accepted an invalid {len}-byte public key"
            );
        }
    }
}

#[test]
fn hybrid_keys_round_trip_decoding() {
    let mut rng = rand::rng();

    for algorithm in [X25519MlKem768Draft00, XWingKemDraft06] {
        let (private_key, public_key) = key_gen(algorithm, &mut rng).unwrap();

        assert!(PrivateKey::decode(algorithm, &private_key.encode()).is_ok());
        assert!(PublicKey::decode(algorithm, &public_key.encode()).is_ok());
    }
}
