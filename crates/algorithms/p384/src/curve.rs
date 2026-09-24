//! # Curve Operations
//!
//! Points on P-384 in affine representation are pairs `(X, Y)` of elements
//! of the base prime field Fp (defined in [`field`], with some
//! more convenience functions defined in [`field_ops`]), which satisfy
//! the short Weierstrass equation `Y^2 = X^3 + aX + b`, where `a` and `b`
//! are curve parameters (defined in [`constants`]).
//!
//! There is no affine representation of the point at infinity, the
//! additive identity element of P-384, so group operations are performed
//! in projective coordinates instead. An point `(X, Y)` in affine coordinates is
//! transformed to projective coordinates as `(X, Y, 1)`. The point at
//! infinity is represented by any point in projective coordinates with `Z
//! = 0`. A point in projective coordinates `(X,Y,Z)` is transformed back
//! to affine coordinates as `(X/Z, Y/Z)`, normalizing the `X` and `Y`
//! coordinates by `Z`. This means that we cannot apply this
//! transformation to the point at infinity.

use crate::{
    constants::{
        FP_NUM_BYTES, FP_NUM_LIMBS, FP_ONE, FP_ZERO, NIST_P384_B, NIST_P384_GX, NIST_P384_GY,
    },
    field::Fp,
};

pub(crate) mod affine;

use affine::AffinePoint;

/// A point on P-384 in projective coordinates.
#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct ProjectivePoint {
    x: Fp,
    y: Fp,
    z: Fp,
}

impl ProjectivePoint {
    #[inline]
    /// Implements exception-free projective point doubling for prime order
    /// short Weierstrass curves.
    /// Commented numbers indicate steps in algorithm 6 from
    ///
    /// Joost Renes, Craig Costello, and Lejla Batina. 2016. Complete Addition
    /// Formulas for Prime Order Elliptic Curves. In Proceedings, Part I, of
    /// the 35th Annual International Conference on Advances in Cryptology ---
    /// EUROCRYPT 2016 - Volume 9665. Springer-Verlag, Berlin, Heidelberg,
    /// 403–428. [link][1],[preprint][2]
    ///
    /// [1]: https://dl.acm.org/doi/10.5555/3081770.3081786
    /// [2]: https://eprint.iacr.org/2015/1060
    fn double(&self) -> Self {
        let ProjectivePoint { x, y, z } = &self;

        let mut t0 = x.square(); // 1.
        let t1 = y.square(); // 2.
        let mut t2 = z.square(); // 3.

        let mut t3 = x * y; // 4.
        t3 = t3 + t3; // 5.
        let mut z3 = x * z; // 6.

        z3 = z3 + z3; // 7.
        let mut y3 = t2 * NIST_P384_B; // 8.
        y3 -= z3; // 9.

        let mut x3 = y3 + y3; // 10.
        y3 += x3; // 11.
        x3 = t1 - y3; // 12.

        y3 += t1; // 13.
        y3 *= x3; // 14.
        x3 *= t3; // 15.

        t3 = t2 + t2; // 16.
        t2 += t3; // 17.
        z3 *= NIST_P384_B; // 18.

        z3 -= t2; // 19.
        z3 -= t0; // 20.
        t3 = z3 + z3; // 21.

        z3 += t3; // 22.
        t3 = t0 + t0; // 23.
        t0 += t3; // 24.

        t0 -= t2; // 25.
        t0 *= z3; // 26.
        y3 += t0; // 27.

        t0 = y * z; // 28.
        t0 = t0 + t0; // 29.
        z3 *= t0; // 30.

        x3 -= z3; // 31.
        z3 = t0 * t1; // 32.
        z3 = z3 + z3; // 33.

        z3 = z3 + z3; // 34.

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Complete point addition for prime order short Weierstrass
    /// curves with a = -3.
    /// Commented numbers indicate steps in algorithm 4 from
    ///
    /// Joost Renes, Craig Costello, and Lejla Batina. 2016. Complete Addition
    /// Formulas for Prime Order Elliptic Curves. In Proceedings, Part I, of
    /// the 35th Annual International Conference on Advances in Cryptology ---
    /// EUROCRYPT 2016 - Volume 9665. Springer-Verlag, Berlin, Heidelberg,
    /// 403–428. [link][1],[preprint][2]
    ///
    /// [1]: https://dl.acm.org/doi/10.5555/3081770.3081786
    /// [2]: https://eprint.iacr.org/2015/1060
    #[inline]
    fn add(&self, other: &Self) -> Self {
        let ProjectivePoint {
            x: x1,
            y: y1,
            z: z1,
        } = &self;

        let ProjectivePoint {
            x: x2,
            y: y2,
            z: z2,
        } = other;

        let mut t0 = x1 * x2; // 1.
        let mut t1 = y1 * y2; // 2.
        let mut t2 = z1 * z2; // 3.

        let mut t3 = x1 + y1; // 4.
        let mut t4 = x2 + y2; // 5.
        t3 *= t4; // 6.

        t4 = t0 + t1; // 7.
        t3 -= t4; // 8.
        t4 = y1 + z1; // 9.

        let mut x3 = y2 + z2; // 10.
        t4 *= x3; // 11.
        x3 = t1 + t2; // 12.

        t4 -= x3; // 13.
        x3 = x1 + z1; // 14.
        let mut y3 = x2 + z2; // 15.

        x3 *= y3; // 16.
        y3 = t0 + t2; // 17.
        y3 = x3 - y3; // 18.

        let mut z3 = t2 * NIST_P384_B; // 19.
        x3 = y3 - z3; // 20.
        z3 = x3 + x3; // 21.

        x3 += z3; // 22.
        z3 = t1 - x3; // 23.
        x3 += t1; // 24.

        y3 *= NIST_P384_B; // 25.
        t1 = t2 + t2; // 26.
        t2 += t1; // 27.

        y3 -= t2; // 28.
        y3 -= t0; // 29.
        t1 = y3 + y3; // 30.

        y3 += t1; // 31.
        t1 = t0 + t0; // 32.
        t0 += t1; // 33.

        t0 -= t2; // 34.
        t1 = t4 * y3; // 35.
        t2 = t0 * y3; // 36.

        y3 = x3 * z3; // 37.
        y3 += t2; // 38.
        x3 *= t3; // 39.

        x3 -= t1; // 40.
        z3 *= t4; // 41.
        t1 = t3 * t0; // 42.

        z3 += t1; // 43.

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Any point with Z = 0 represents the point at infinity.
    pub(crate) fn is_point_at_infinity(&self) -> bool {
        self.z.is_zero()
    }

    /// Returns the group generator of P-384.
    pub const fn generator() -> Self {
        ProjectivePoint {
            x: NIST_P384_GX,
            y: NIST_P384_GY,
            z: FP_ONE,
        }
    }

    /// Returns the canonical additive identity for P-384, i.e. the point at infinity.
    const fn identity() -> Self {
        ProjectivePoint {
            x: FP_ZERO,
            y: FP_ONE,
            z: FP_ZERO,
        }
    }

    #[inline]
    /// Perform a scalar multiplication of the point and a given
    /// scalar, represented as a big endian byte array.
    ///
    /// Uses a simple Montgomery ladder internally.
    pub(crate) fn scalar_mul(&self, scalar: &[u8; FP_NUM_BYTES]) -> Self {
        let mut r0 = Self::identity();
        let mut r1 = *self;
        let mut swap: u64 = 0;

        for byte in scalar {
            for i in (0..8).rev() {
                let bit = ((byte >> i) & 1) as u64;
                swap ^= bit;
                cswap(swap, &mut r0, &mut r1);
                swap = bit;
                r1 = r0.add(&r1);
                r0 = r0.double();
            }
        }

        cswap(swap, &mut r0, &mut r1);
        r0
    }

    /// Attempt to convert the projective point to affine coordinates.
    ///
    /// This is not possible, if the input point is the point at
    /// infinity.
    #[cfg(test)]
    pub(crate) fn to_affine(self) -> Option<AffinePoint> {
        if self.is_point_at_infinity() {
            return None;
        }

        Some(self.to_affine_non_inf())
    }

    /// Convert a projective point that is not the point at infinity to
    /// affine coordinates.
    ///
    /// **CAUTION:** It is the caller's responsibility to ensure that the
    /// input to this function is not the point at infinity. If the
    /// input encodes the point at infinity, the output of this
    /// function is undefined.
    pub(crate) fn to_affine_non_inf(self) -> AffinePoint {
        let z_inv = self.z.inv();

        let x = self.x * z_inv;
        let y = self.y * z_inv;

        AffinePoint { x, y }
    }
}

impl From<AffinePoint> for ProjectivePoint {
    /// A canonical projective point for a given affine point (X, Y) is the
    /// point (X, Y, 1).
    fn from(value: AffinePoint) -> Self {
        ProjectivePoint {
            x: value.x,
            y: value.y,
            z: FP_ONE,
        }
    }
}
/// Constant-time conditional swap: if `swap` is 1, exchanges `a` and `b` in
/// place (branch-free, via limb-wise XOR-mask) with no data-dependent memory
/// access pattern beyond what's inherent to touching both points every call.
#[inline]
fn cswap(swap: u64, a: &mut ProjectivePoint, b: &mut ProjectivePoint) {
    let mask = 0u64.wrapping_sub(swap); // swap in {0,1} -> mask is all-0s or all-1s
    for i in 0..FP_NUM_LIMBS {
        let t = mask & (a.x.0[i] ^ b.x.0[i]);
        a.x.0[i] ^= t;
        b.x.0[i] ^= t;
        let t = mask & (a.y.0[i] ^ b.y.0[i]);
        a.y.0[i] ^= t;
        b.y.0[i] ^= t;
        let t = mask & (a.z.0[i] ^ b.z.0[i]);
        a.z.0[i] ^= t;
        b.z.0[i] ^= t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annihilation() {
        use crate::constants::NIST_P384_CURVE_ORDER_BE_BYTES;

        let g = ProjectivePoint::generator();
        let mut p_minus_one = NIST_P384_CURVE_ORDER_BE_BYTES;
        p_minus_one[47] -= 1;

        let mut p_minus_two = NIST_P384_CURVE_ORDER_BE_BYTES;
        p_minus_two[47] -= 2;

        let g_p_minus_one = g.scalar_mul(&p_minus_one);
        let g_p_minus_two = g.scalar_mul(&p_minus_two);

        let split_g_p_minus_one = g_p_minus_two.add(&g);
        let split_identity = g_p_minus_one.add(&g);

        let identity = g.scalar_mul(&NIST_P384_CURVE_ORDER_BE_BYTES);

        assert!(split_identity.is_point_at_infinity());
        assert!(identity.is_point_at_infinity());

        assert_eq!(
            g_p_minus_one.to_affine().unwrap(),
            split_g_p_minus_one.to_affine().unwrap()
        );
        assert_ne!(g_p_minus_one, g);
        assert!(!g_p_minus_one.is_point_at_infinity());
    }
}
