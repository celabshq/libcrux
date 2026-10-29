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

/// A RNG that returns the given 48-byte blocks in order.
struct FixedRng(Vec<[u8; 48]>);

impl rand::TryRng for FixedRng {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        unimplemented!()
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        unimplemented!()
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        dst.copy_from_slice(&self.0.remove(0));
        Ok(())
    }
}

impl rand::TryCryptoRng for FixedRng {}

/// The P-384 curve order n minus `k`, big-endian.
fn order_minus(k: u8) -> [u8; 48] {
    let mut n: [u8; 48] = hex::decode(
        "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973",
    )
    .unwrap()
    .try_into()
    .unwrap();
    n[47] -= k;
    n
}

fn generate_from(candidates: Vec<[u8; 48]>) -> Option<[u8; 48]> {
    PrivateKey::generate(&mut FixedRng(candidates))
        .ok()
        .map(|sk| *sk.as_ref())
}

/// FIPS 186-5, A.2.2: reject `c > n - 2`, otherwise `d = c + 1`.
#[test]
fn generate_is_fips_186_5_rejection_sampling() {
    let mut one = [0u8; 48];
    one[47] = 1;
    assert_eq!(generate_from(vec![[0u8; 48]]), Some(one));

    assert_eq!(generate_from(vec![order_minus(2)]), Some(order_minus(1)));

    // n - 1 and anything above is rejected.
    let mut five = [0u8; 48];
    five[47] = 5;
    let mut six = [0u8; 48];
    six[47] = 6;
    assert_eq!(generate_from(vec![order_minus(1), five]), Some(six));
    assert_eq!(generate_from(vec![[0xff; 48], five]), Some(six));

    // Carry propagation.
    let mut c = [0u8; 48];
    c[46] = 0x01;
    c[47] = 0xff;
    let mut d = [0u8; 48];
    d[46] = 0x02;
    assert_eq!(generate_from(vec![c]), Some(d));

    // Give up after 5 rejected candidates.
    assert_eq!(generate_from(vec![[0xff; 48]; 5]), None);
}
