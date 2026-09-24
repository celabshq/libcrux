//! # Affine Coordinate Representation
//!
//! We don't use affine coordinates for curve operations, just to store
//! points that must not be the point at infinity, such as valid ECDH
//! public keys.

use core::ops::Neg;

use crate::{
    constants::{
        COMPRESSED_POINT_ID, COMPRESSED_POINT_ID_ODD, COMPRESSED_POINT_LEN, UNCOMPRESSED_POINT_ID,
        UNCOMPRESSED_POINT_LEN,
    },
    field::Fp,
    InternalError,
};

/// A point on P-384 in affine coordinates.
#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct AffinePoint {
    pub(crate) x: Fp,
    pub(crate) y: Fp,
}

impl AffinePoint {
    /// Read a SEC1 uncompressed public key encoding.
    ///
    /// This function validates the correctness of the encoding, but *does
    /// not* validate the curve point itself. For checking curve membership,
    /// use `AffinePoint::validate`.
    ///
    /// The uncompressed encoding of a curve point P is a 97-byte slice starting with
    /// byte `0x04`, followed by 48-byte encodings of affine point
    /// coordinates x_P and y_P:
    ///
    ///   P_uncompressed = `0x04` || X || Y
    ///
    /// where X = FE2OS(x_P) and Y = FE2OS(y_P) are the encodings of curve
    /// coordinates as octet strings.
    pub(crate) fn from_uncompressed(uncompressed_bytes: &[u8]) -> Result<Self, InternalError> {
        if uncompressed_bytes.len() != UNCOMPRESSED_POINT_LEN
            || uncompressed_bytes[0] != UNCOMPRESSED_POINT_ID
        {
            return Err(InternalError::Uncompressed);
        }

        const OFFSET: usize = 1;
        let x_bytes = &uncompressed_bytes[OFFSET..UNCOMPRESSED_POINT_LEN / 2 + OFFSET];
        let y_bytes = &uncompressed_bytes[UNCOMPRESSED_POINT_LEN / 2 + OFFSET..];

        let x = Fp::from_be_bytes(x_bytes.try_into().expect("x_bytes is 48 bytes long"))
            .map_err(|_| InternalError::Uncompressed)?;
        let y = Fp::from_be_bytes(y_bytes.try_into().expect("y_bytes is 48 bytes long"))
            .map_err(|_| InternalError::Uncompressed)?;

        Ok(AffinePoint { x, y })
    }

    /// Read the SEC1 compressed encoding of a public key from the input
    /// buffer.
    ///
    /// Returns an error if the buffer does not contain a valid encoding of a
    /// point on P-384.
    ///
    /// A valid encoding has the form `y_P || FE2OS(X)`, where `y_P` is one byte
    /// with value either `0x02` or `0x03` and `FE2OS(X)` is the 48-byte
    /// encoding of the X coordinate of the candidate point.
    ///
    /// Curve membership is tested by computing the right-hand side of the
    /// short Weierstrass equation Y' = X^3 + aX + b. The point is on the
    /// curve, if Y' is a square. If Y' is non-zero, which of two possible
    /// points was encoded is determined from `y_P`.
    pub fn from_compressed(compressed_bytes: &[u8]) -> Result<Self, InternalError> {
        if compressed_bytes.len() != COMPRESSED_POINT_LEN {
            return Err(InternalError::Compressed);
        }
        if !(compressed_bytes[0] == COMPRESSED_POINT_ID
            || compressed_bytes[0] == COMPRESSED_POINT_ID_ODD)
        {
            return Err(InternalError::Compressed);
        }

        let expect_odd = compressed_bytes[0] == COMPRESSED_POINT_ID_ODD;

        let x = Fp::from_be_bytes(
            &compressed_bytes[1..]
                .try_into()
                .expect("remainder of `compressed_bytes` is 48 bytes long"),
        )
        .map_err(|_| InternalError::Compressed)?;

        let weierstrass_lhs = x.weierstrass_rhs();

        // Note that there are no solutions for X^3 + aX + b = 0 in a
        // prime order field, so we don't have to consider the case
        // that Y = 0.
        // See e.g. https://crypto.stackexchange.com/a/108242 for a
        // simple proof of the above claim.
        assert!(!weierstrass_lhs.is_zero());
        if let Some(y) = weierstrass_lhs.sqrt() {
            if y.is_odd() == expect_odd {
                Ok(AffinePoint { x, y })
            } else {
                Ok(AffinePoint { x, y: y.neg() })
            }
        } else {
            Err(InternalError::Compressed)
        }
    }

    /// Write the SEC1 uncompressed encoding of the affine point into the
    /// provided buffer `out`.
    pub(crate) fn to_uncompressed(self, out: &mut [u8; UNCOMPRESSED_POINT_LEN]) {
        out[0] = UNCOMPRESSED_POINT_ID;
        out[1..49].copy_from_slice(&self.x.to_be_bytes());
        out[49..].copy_from_slice(&self.y.to_be_bytes());
    }

    /// Write the SEC1 compressed encoding of the affine point into the provided
    /// buffer `out`.
    pub(crate) fn to_compressed(self, out: &mut [u8; COMPRESSED_POINT_LEN]) {
        if self.y.is_odd() {
            out[0] = COMPRESSED_POINT_ID_ODD;
        } else {
            out[0] = COMPRESSED_POINT_ID;
        }
        out[1..COMPRESSED_POINT_LEN].copy_from_slice(&self.x.to_be_bytes());
    }

    /// Validate the curve equation.
    ///
    /// That is, check whether Y^2 = X^3 + aX + b.
    pub(crate) fn validate(&self) -> bool {
        let y_squared = self.y.square();
        let rhs = self.x.weierstrass_rhs();

        (y_squared - rhs).is_zero()
    }
}
