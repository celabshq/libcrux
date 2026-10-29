//! The elliptic curve operations used by KAS-ECC-SSC and ECDSA, on big-endian byte strings.

use libcrux_ecdh::Algorithm;
use libcrux_kats::acvp::ec::{pad, raw_point, uncompressed_point};

/// A key pair `(d, x, y)`.
type KeyPair = (Vec<u8>, Vec<u8>, Vec<u8>);

/// A point `(x, y)`.
type Point = (Vec<u8>, Vec<u8>);

/// `ecdh(d, x, y)`.
type Ecdh = fn(&[u8], &[u8], &[u8]) -> Option<Vec<u8>>;

pub(crate) struct Curve {
    /// Length of a field element and a scalar in bytes.
    pub(crate) field_len: usize,
    /// A fresh key pair `(d, x, y)` for key agreement.
    pub(crate) generate: fn() -> KeyPair,
    /// A fresh key pair `(d, x, y)` from the ECDSA key generation.
    pub(crate) ecdsa_generate: fn() -> KeyPair,
    /// The public key `(x, y)` of `d`, or `None` if `d` is invalid.
    pub(crate) public_key: fn(&[u8]) -> Option<Point>,
    /// Whether `(x, y)` is a valid public key.
    pub(crate) validate: fn(&[u8], &[u8]) -> bool,
    /// The shared secret `Z`, the x-coordinate of `d * (x, y)`, or `None` if an input is invalid.
    pub(crate) ecdh: Ecdh,
}

mod p256 {
    use super::*;

    const LEN: usize = 32;

    fn split(xy: &[u8]) -> Point {
        (xy[..LEN].to_vec(), xy[LEN..].to_vec())
    }

    pub(super) fn generate() -> KeyPair {
        let (d, xy) = libcrux_ecdh::key_gen(Algorithm::P256, &mut rand::rng()).unwrap();
        let (x, y) = split(&xy);
        (d, x, y)
    }

    pub(super) fn ecdsa_generate() -> KeyPair {
        let (d, q) = libcrux_ecdsa::p256::rand::generate_key_pair(&mut rand::rng()).unwrap();
        let d: &[u8] = d.as_ref();
        let (x, y) = split(&q.0);
        (d.to_vec(), x, y)
    }

    pub(super) fn public_key(d: &[u8]) -> Option<Point> {
        let xy = libcrux_ecdh::secret_to_public(Algorithm::P256, pad(d, LEN)?).ok()?;
        Some(split(&xy))
    }

    pub(super) fn validate(x: &[u8], y: &[u8]) -> bool {
        raw_point(x, y, LEN)
            .is_some_and(|q| libcrux_ecdsa::p256::PublicKey::try_from(q.as_slice()).is_ok())
    }

    /// [`libcrux_ecdh::derive`] returns the shared point `X || Y`.
    pub(super) fn ecdh(d: &[u8], x: &[u8], y: &[u8]) -> Option<Vec<u8>> {
        let point =
            libcrux_ecdh::derive(Algorithm::P256, raw_point(x, y, LEN)?, pad(d, LEN)?).ok()?;
        Some(point[..LEN].to_vec())
    }
}

mod p384 {
    use libcrux_p384::{PrivateKey, PublicKey};

    use super::*;

    const LEN: usize = 48;

    fn coordinates(pk: PublicKey) -> Point {
        let mut q = [0u8; 2 * LEN + 1];
        pk.to_uncompressed(&mut q);
        (q[1..1 + LEN].to_vec(), q[1 + LEN..].to_vec())
    }

    /// [`PrivateKey::generate`] implements FIPS 186-5 A.2.2.
    pub(super) fn generate() -> KeyPair {
        let sk = PrivateKey::generate(&mut rand::rng()).unwrap();
        let (x, y) = coordinates(PublicKey::from(&sk));
        (sk.as_ref().to_vec(), x, y)
    }

    pub(super) fn public_key(d: &[u8]) -> Option<Point> {
        let sk = PrivateKey::try_from(pad(d, LEN)?.as_slice()).ok()?;
        Some(coordinates(PublicKey::from(&sk)))
    }

    pub(super) fn validate(x: &[u8], y: &[u8]) -> bool {
        uncompressed_point(x, y, LEN).is_some_and(|q| PublicKey::from_uncompressed(&q).is_ok())
    }

    pub(super) fn ecdh(d: &[u8], x: &[u8], y: &[u8]) -> Option<Vec<u8>> {
        let z = libcrux_p384::derive_ecdh(&pad(d, LEN)?, &uncompressed_point(x, y, LEN)?).ok()?;
        Some(z.as_ref().to_vec())
    }
}

/// The curve `name`, or `None` for a curve that ACVP tests but libcrux does not implement.
pub(crate) fn curve(name: &str) -> Option<Curve> {
    Some(match name {
        "P-256" => Curve {
            field_len: 32,
            generate: p256::generate,
            ecdsa_generate: p256::ecdsa_generate,
            public_key: p256::public_key,
            validate: p256::validate,
            ecdh: p256::ecdh,
        },
        "P-384" => Curve {
            field_len: 48,
            generate: p384::generate,
            ecdsa_generate: p384::generate,
            public_key: p384::public_key,
            validate: p384::validate,
            ecdh: p384::ecdh,
        },
        "P-192" | "P-224" | "P-521" | "B-163" | "B-233" | "B-283" | "B-409" | "B-571" | "K-163"
        | "K-233" | "K-283" | "K-409" | "K-571" => return None,
        c => panic!("unexpected curve {c}"),
    })
}
