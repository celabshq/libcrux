//! # P-384 ECDH
//!
//! This module provides ECDH operations over the mathematical objects
//! defined in [`curve`].
//!
//! Public keys are validated on deserialization and then stored in affine
//! coordinates to ensure we can never have the point at infinity for a
//! public key. Validity requirements for public keys are:
//! 0. Their encoding is either
//!
//! - SEC1 uncompressed: 97 bytes where the byte at index 0 is `0x04`
//!   and the bytes at indices 1..49 and 49..97 encode the affine X and Y
//!   coordinates of the public key in big-endian byte order, in
//!   particular the encoded coordinates are less than the P-384 field
//!   modulus, or
//! - SEC1 compressed: 49 bytes where the byte at index 0 is either
//!   `0x02` or `0x03` and the bytes at indices 1..49 encode the affine X
//!   coordinate of the public key in big-endian byte order, in
//!   particular the encoded coordinate is less than the P-384 field modulus.
//! 1. The encoded coordinates correspond to a valid public key, in
//!    particular
//!    - The encoded or reconstructed point coordinates correspond to a
//!      point on the P-384. We enforce this by either validating the curve
//!      equation if both coordinates are given or it holds by
//!      construction if a valid Y coordinate can be reconstructed from
//!      the X coordinate and the parity of Y encoded in the first byte of
//!      the compressed encoding.
//!    - They do not represent the point at infinity O. This holds by
//!      construction for deserialized points, since the point at infinity
//!      cannot be represented in affine coordinates.
//!
//! Private keys are stored as big endian byte arrays, instead of
//! converting them to the internal field element type. This makes avoids
//! unnecessary conversions before scalar multiplication. Validity
//! requirements for private keys are:
//! 0. They are field elements encoded as 48-byte big-endian arrays.
//! 1. They are non-zero.
//! 2. They are strictly less than the curve order.
//!
//!These requirements are checked on deserialization.
//!
//! When a public key is derived from a valid private key it is done by
//! scalar multiplication of the generator G with private key, after which
//! the result in projective coordinates is transformed into affine
//! coordinates. This is always possible, since P-384 is of prime order
//! and therefore we know that we can only have kG = O, iff k is the group
//! order which would make it an invalid private key.
//!
//! Shared secrets are the result of a scalar multiplication between a
//! valid public key and a valid private key. We don't need to worry about
//! the shared secret being the point at infinity, since if we have kP = O
//! for some scalar k and point P, it must either hold that k = 0, which
//! would make k an invalid private key or P = 0, which would make P an
//! invalid public key.
use crate::{
    constants::{COMPRESSED_POINT_LEN, FP_NUM_BYTES, UNCOMPRESSED_POINT_LEN},
    curve::{affine::AffinePoint, ProjectivePoint},
    EcdhError as Error,
};

pub(crate) mod private;

use private::PrivateKey;

/// A P-384 shared secret.
pub struct SharedSecret([u8; FP_NUM_BYTES]);

impl AsRef<[u8; FP_NUM_BYTES]> for SharedSecret {
    fn as_ref(&self) -> &[u8; FP_NUM_BYTES] {
        &self.0
    }
}

/// An ECDH public key.
#[derive(Clone, Copy)]
pub struct PublicKey(AffinePoint);

impl PublicKey {
    /// Derives an ECDH shared secret from the public key and a scalar.
    pub fn ecdh(&self, private_key: &PrivateKey) -> SharedSecret {
        let pk_projective = ProjectivePoint::from(self.0);

        // This can never be the point at infinity, since the public
        // key is not the point at infinity and the private key is a
        // non-zero scalar less than the group order.
        let ecdh_projective = pk_projective.scalar_mul(&private_key.0);
        assert!(!ecdh_projective.is_point_at_infinity());

        SharedSecret(ecdh_projective.to_affine_non_inf().x.to_be_bytes())
    }
    /// Read the SEC1 compressed encoding of a public key from the input
    /// buffer.
    ///
    /// Returns an error if the buffer does not contain a valid encoding of a
    /// point on P-384.
    pub fn from_uncompressed(uncompressed_bytes: &[u8]) -> Result<Self, Error> {
        let p_affine = AffinePoint::from_uncompressed(uncompressed_bytes)
            .map_err(|_| Error::InvalidPublicKey)?;

        if p_affine.validate() {
            Ok(Self(p_affine))
        } else {
            Err(Error::InvalidPublicKey)
        }
    }

    /// Read the SEC1 compressed encoding of a public key from the input
    /// buffer.
    ///
    /// Returns an error if the buffer does not contain a valid encoding of a
    /// point on P-384.
    pub fn from_compressed(compressed_bytes: &[u8]) -> Result<Self, Error> {
        let p_affine =
            AffinePoint::from_compressed(compressed_bytes).map_err(|_| Error::InvalidPublicKey)?;

        Ok(Self(p_affine))
    }

    /// Write the SEC1 uncompressed encoding of the public key into the
    /// provided buffer `out`.
    pub fn to_uncompressed(self, out: &mut [u8; UNCOMPRESSED_POINT_LEN]) {
        self.0.to_uncompressed(out);
    }

    /// Write the SEC1 compressed encoding of the public key into the provided
    /// buffer `out`.
    pub fn to_compressed(self, out: &mut [u8; COMPRESSED_POINT_LEN]) {
        self.0.to_compressed(out);
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;

    /// Attempt decoding if input length matches either compressed or
    /// uncompressed encoding lengths.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            COMPRESSED_POINT_LEN => Self::from_compressed(value),
            UNCOMPRESSED_POINT_LEN => Self::from_uncompressed(value),
            _ => Err(Error::InvalidPublicKey),
        }
    }
}

impl From<&PrivateKey> for PublicKey {
    /// For a given private key `x`, the corresponding public key is
    /// `xG`, where `G` is the group generator.
    fn from(value: &PrivateKey) -> Self {
        // This can never be the point at infinity, since the
        // generator is not the point at infinity, and the private
        // key is a non-zero scalar less than the group order.
        let pk_projective = ProjectivePoint::generator().scalar_mul(&value.0);

        assert!(!pk_projective.is_point_at_infinity());
        Self(pk_projective.to_affine_non_inf())
    }
}
