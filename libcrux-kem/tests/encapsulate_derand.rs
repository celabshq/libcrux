use libcrux_kem::{
    key_gen,
    Algorithm::{X25519MlKem768Draft00, XWingKemDraft06},
    Error,
};

// `PublicKey::encapsulate_derand` should not panic on a too-short seed for the
// two hybrid algorithms (raw `seed[0..32]`/`&seed[32..]` slicing before any
// length check).
// The return an `Err(Error::KeyGen)`.
#[test]
fn hybrid_encapsulate_derand_rejects_short_seed() {
    let mut rng = rand::rng();

    for algorithm in [X25519MlKem768Draft00, XWingKemDraft06] {
        let (_, public_key) = key_gen(algorithm, &mut rng).unwrap();

        for len in [0, 1, 32, 63] {
            let seed = vec![0u8; len];
            assert!(
                matches!(public_key.encapsulate_derand(&seed), Err(Error::KeyGen)),
                "{algorithm:?} accepted a {len}-byte seed instead of rejecting it"
            );
        }

        // A correctly-sized (64-byte) seed still works.
        let seed = vec![0u8; 64];
        assert!(public_key.encapsulate_derand(&seed).is_ok());
    }
}
