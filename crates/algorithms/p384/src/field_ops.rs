//! More ergonomic field operations
//!
//! We assume the underlying operations from `field` are constant-time.
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
    constants::{
        fp_from_be_bytes, FP_NUM_BYTES, FP_NUM_LIMBS, FP_ONE, FP_ZERO, NIST_P384_A, NIST_P384_B,
        NIST_P384_P_BE_BYTES, NIST_P384_P_PLUS_1_OVER_4_BE_BYTES,
    },
    field::{
        fiat_p384_to_bytes, fp_add, fp_from_montgomery, fp_inv, fp_mul, fp_nonzero, fp_opp,
        fp_square, fp_sub, Fp, FpRaw,
    },
    util::be_bytes_lt,
    InternalError,
};

impl Add<&Fp> for &Fp {
    type Output = Fp;

    #[inline]
    fn add(self, rhs: &Fp) -> Self::Output {
        let mut out = Fp::new();
        fp_add(&mut out, self, rhs);
        out
    }
}

impl Add<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn add(self, rhs: Fp) -> Self::Output {
        let mut out = Fp::new();
        fp_add(&mut out, &self, &rhs);
        out
    }
}

impl Mul<&Fp> for &Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: &Fp) -> Self::Output {
        let mut out = Fp::new();
        fp_mul(&mut out, self, rhs);
        out
    }
}

impl Mul<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: Fp) -> Self::Output {
        let mut out = Fp::new();
        fp_mul(&mut out, &self, &rhs);
        out
    }
}

impl Sub<&Fp> for &Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, rhs: &Fp) -> Self::Output {
        let mut out = Fp::new();
        fp_sub(&mut out, self, rhs);
        out
    }
}

impl Sub<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, rhs: Fp) -> Self::Output {
        let mut out = Fp::new();
        fp_sub(&mut out, &self, &rhs);
        out
    }
}

impl Neg for &Fp {
    type Output = Fp;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut out = Fp::new();
        fp_opp(&mut out, self);
        out
    }
}
impl AddAssign for Fp {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let mut tmp = Fp::new();
        fp_add(&mut tmp, self, &rhs);
        *self = tmp;
    }
}

impl SubAssign for Fp {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let mut tmp = Fp::new();
        fp_sub(&mut tmp, self, &rhs);
        *self = tmp;
    }
}

impl MulAssign for Fp {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let mut tmp = Fp::new();
        fp_mul(&mut tmp, self, &rhs);
        *self = tmp;
    }
}

impl Fp {
    #[inline]
    pub(crate) const fn new() -> Self {
        FP_ZERO
    }

    #[inline]
    /// Checks whether the right-most bit of the standard form of
    /// `self` as big-endian bytes is set, i.e. whether
    /// `from_montgomery(self) % 2 == 1`.
    pub(crate) fn is_odd(&self) -> bool {
        // Reading the parity bit directly from the raw (pre-serialization)
        // limb array was tried here and measured no difference in the ECDH
        // benchmark, so this keeps the simpler byte-serialization form.
        let serialized = self.to_be_bytes();
        serialized[serialized.len() - 1] & 1 == 1
    }

    #[inline]
    /// There is a separate `fp_square` function in the vendored code
    /// that might be more efficient than using `fp_mul`.
    pub(crate) fn square(&self) -> Self {
        let mut out = Fp::new();
        fp_square(&mut out, self);
        out
    }

    /// Parse a Montgomery standard form field element from big-endian bytes.
    ///
    /// Returns [`InternalError`] if the encoded integer is unreduced,
    /// i.e. larger than the field modulus.
    #[inline]
    pub(crate) fn from_be_bytes(bytes: &[u8; FP_NUM_BYTES]) -> Result<Self, InternalError> {
        if !core::hint::black_box(be_bytes_lt(bytes, &NIST_P384_P_BE_BYTES)) {
            return Err(InternalError::FieldElement);
        }
        Ok(fp_from_be_bytes(bytes))
    }

    /// Converts a Montgomery-domain field element to big-endian
    /// bytes.
    #[inline]
    pub(crate) fn to_be_bytes(self) -> [u8; FP_NUM_BYTES] {
        let mut raw = FpRaw::new();
        fp_from_montgomery(&mut raw, &self);
        let mut le = [0u8; FP_NUM_BYTES];
        fiat_p384_to_bytes(&mut le, &raw.0);
        le.reverse();
        le
    }

    #[inline]
    pub(crate) fn inv(&self) -> Self {
        let mut out = Fp::new();
        fp_inv(&mut out, self);
        out
    }

    #[inline]
    /// We assume this is a constant-time test.
    pub(crate) fn is_zero(&self) -> bool {
        !fp_nonzero(self)
    }

    #[inline]
    /// Attempts to compute a square root of `self` by raising it to
    /// (p+1)/4 mod p using square-and-mul. This is safe, since we
    /// only branch on the bits of the public constant (p+1)/4, not on
    /// any bits of `self`.
    ///
    /// If the result squares to the original input, we have confirmed
    /// the input was a quadratic residue and can return its square
    /// root.
    pub(crate) fn sqrt(&self) -> Option<Self> {
        let mut acc = FP_ONE;

        for byte in NIST_P384_P_PLUS_1_OVER_4_BE_BYTES {
            for bit in (0..8).rev() {
                acc = acc.square();

                if (byte >> bit) & 1 == 1 {
                    acc = &acc * self;
                }
            }
        }

        // This is safe, assuming the underlying operations `fp_sub`
        // and `fp_nonzero` are constant-time.
        if (&acc.square() - self).is_zero() {
            Some(acc)
        } else {
            None
        }
    }

    #[inline]
    /// Computes the right-hand side of the short Weierstrass equation
    /// for P-384, i.e. X^3 + aX + b.
    pub(crate) fn weierstrass_rhs(&self) -> Self {
        // Computing a*X via additions (since a = -3) instead of a full
        // multiplication was tried here and measured no difference in the
        // ECDH benchmark, so this keeps the simpler multiplication form.
        let x_cubed = &self.square() * self;
        let ax = self * &NIST_P384_A;
        (x_cubed + ax) + NIST_P384_B
    }
}

impl FpRaw {
    /// Create a new standard form field element.
    ///
    /// Only need this because we can't implement `Default` as `const`.
    pub(crate) const fn new() -> Self {
        Self([0u64; FP_NUM_LIMBS])
    }
}
