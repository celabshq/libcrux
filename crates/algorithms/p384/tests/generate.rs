#![cfg(feature = "rand")]

use libcrux_p384::{PrivateKey, PublicKey};

#[test]
fn generate_round_trips_through_ecdh() {
    let mut rng = rand::rng();

    for _ in 0..100 {
        let sk_a = PrivateKey::generate(&mut rng).unwrap();
        let sk_b = PrivateKey::generate(&mut rng).unwrap();
        let pk_a = PublicKey::from(&sk_a);
        let pk_b = PublicKey::from(&sk_b);

        // Generated keys must be valid and produce a shared secret both ways.
        let secret_a = pk_b.ecdh(&sk_a);
        let secret_b = pk_a.ecdh(&sk_b);
        assert_eq!(secret_a.as_ref(), secret_b.as_ref());

        // Public keys should round-trip through (de)serialization.
        let mut uncompressed_a = [0u8; 97];
        pk_a.to_uncompressed(&mut uncompressed_a);
        let decoded = PublicKey::try_from(uncompressed_a.as_slice()).unwrap();
        let mut re_encoded = [0u8; 97];
        decoded.to_uncompressed(&mut re_encoded);
        assert_eq!(re_encoded, uncompressed_a);

        // Two independently generated keys should not collide.
        let mut uncompressed_b = [0u8; 97];
        pk_b.to_uncompressed(&mut uncompressed_b);
        assert_ne!(uncompressed_a, uncompressed_b);
    }
}
