//! This module defines classification and declassification over secret integers
//! These implementations are meant to be used when feature `check-secret-independence` is set
use super::classify_secret::*;
use crate::mem_requests::{ct_classify, ct_declassify};
use crate::traits::*;
use core::ops::*;
use core::ptr;

pub type I8 = Secret<i8>;
pub type U8 = Secret<u8>;
pub type I16 = Secret<i16>;
pub type U16 = Secret<u16>;
pub type I32 = Secret<i32>;
pub type U32 = Secret<u32>;
pub type I64 = Secret<i64>;
pub type U64 = Secret<u64>;
pub type I128 = Secret<i128>;
pub type U128 = Secret<u128>;

// Construct a secret integer
pub fn secret<T>(x: T) -> Secret<T> {
    ct_classify(&x);
    Secret(x)
}

// Classify a scalar
impl<T: Scalar> Classify for T {
    type Classified = Secret<T>;
    fn classify(self) -> Secret<Self> {
        secret(self)
    }
}

// Declassify a scalar
impl<T: Scalar> Declassify for Secret<T> {
    type Declassified = T;
    fn declassify(self) -> T {
        ct_declassify(&self);
        self.0
    }
}

// Classify a reference to a scalar
impl<'a, T: Scalar> ClassifyRef for &'a T {
    type ClassifiedRef = &'a Secret<T>;
    fn classify_ref(self) -> &'a Secret<T> {
        ct_classify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret values is the same
        unsafe { &*ptr::from_ref(self).cast::<Secret<T>>() }
    }
}

// Declassify a reference to a scalar
impl<'a, T: Scalar> DeclassifyRef for &'a Secret<T> {
    type DeclassifiedRef = &'a T;
    fn declassify_ref(self) -> &'a T {
        ct_declassify(self);
        // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
        //       the memory representation of the public and secret values is the same
        unsafe { &*ptr::from_ref(self).cast::<T>() }
    }
}

/// Classify a mutable reference to a slice
pub fn classify_mut_slice<T: Scalar>(x: &mut [T]) -> &mut [Secret<T>] {
    ct_classify(x);
    // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
    //       the memory representation of the public and secret slices is the same
    unsafe { core::slice::from_raw_parts_mut(x.as_mut_ptr().cast::<Secret<T>>(), x.len()) }
}

/// Declassify a mutable reference to a slice
pub fn declassify_mut_slice<T: Scalar>(x: &mut [Secret<T>]) -> &mut [T] {
    ct_declassify(x);
    // SAFETY: this is safe since the `Secret` type is `repr(transparent)`, so
    //       the memory representation of the public and secret slices is the same
    unsafe { core::slice::from_raw_parts_mut(x.as_mut_ptr().cast::<T>(), x.len()) }
}

// We define a series of operations that are safe over secret values
// Notably, one cannot extract the inner value from a secret, one cannot
// cast a secret into a public value, and one cannot divide or mod a secret
// The absence of those operations enforces our secret independence discipline.

// Add secret values
impl<T: Add, V: Into<Secret<T>>> Add<V> for Secret<T> {
    type Output = Secret<T::Output>;
    fn add(self, rhs: V) -> Self::Output {
        self.0.add(rhs.into().0).into()
    }
}

// Subtract secret values
impl<T: Sub, V: Into<Secret<T>>> Sub<V> for Secret<T> {
    type Output = Secret<T::Output>;
    fn sub(self, rhs: V) -> Self::Output {
        self.0.sub(rhs.into().0).into()
    }
}

// Negate secret values
impl<T: Neg> Neg for Secret<T> {
    type Output = Secret<T::Output>;

    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}

// Multiply secret values
impl<T: Mul, V: Into<Secret<T>>> Mul<V> for Secret<T> {
    type Output = Secret<T::Output>;
    fn mul(self, rhs: V) -> Self::Output {
        self.0.mul(rhs.into().0).into()
    }
}

// Bitwise Xor of secret values
impl<T: BitXor, V: Into<Secret<T>>> BitXor<V> for Secret<T> {
    type Output = Secret<T::Output>;
    fn bitxor(self, rhs: V) -> Self::Output {
        self.0.bitxor(rhs.into().0).into()
    }
}

// Bitwise Or of secret values
impl<T: BitOr, V: Into<Secret<T>>> BitOr<V> for Secret<T> {
    type Output = Secret<T::Output>;
    fn bitor(self, rhs: V) -> Self::Output {
        self.0.bitor(rhs.into().0).into()
    }
}

// Bitwise And of secret values
impl<T: BitAnd, V: Into<Secret<T>>> BitAnd<V> for Secret<T> {
    type Output = Secret<T::Output>;
    fn bitand(self, rhs: V) -> Self::Output {
        self.0.bitand(rhs.into().0).into()
    }
}

// Bitwise Not of secret values
impl<T: Not> Not for Secret<T> {
    type Output = Secret<T::Output>;
    fn not(self) -> Self::Output {
        self.0.not().into()
    }
}

// Shift-left of secret values
// Note: the number of bits we shift the value by is not secret,
//       since some implementations may leak this value
impl<U, T: Shl<U>> Shl<U> for Secret<T>
where
    T::Output: Into<T>,
{
    type Output = Secret<T>;
    fn shl(self, rhs: U) -> Self::Output {
        (self.0.shl(rhs).into()).into()
    }
}

// Shift-right of secret values
// Note: the number of bits we shift the value by is not secret,
//       since some implementations may leak this value
impl<U, T: Shr<U>> Shr<U> for Secret<T>
where
    T::Output: Into<T>,
{
    type Output = Secret<T>;
    fn shr(self, rhs: U) -> Self::Output {
        (self.0.shr(rhs).into()).into()
    }
}

// += over secret values
impl<T: AddAssign, V: Into<Secret<T>>> AddAssign<V> for Secret<T> {
    fn add_assign(&mut self, rhs: V) {
        self.0 += rhs.into().0
    }
}

// -= over secret values
impl<T: SubAssign, V: Into<Secret<T>>> SubAssign<V> for Secret<T> {
    fn sub_assign(&mut self, rhs: V) {
        self.0 -= rhs.into().0
    }
}

// *= over secret values
impl<T: MulAssign, V: Into<Secret<T>>> MulAssign<V> for Secret<T> {
    fn mul_assign(&mut self, rhs: V) {
        self.0 *= rhs.into().0
    }
}

// ^= over secret values
impl<T: BitXorAssign, V: Into<Secret<T>>> BitXorAssign<V> for Secret<T> {
    fn bitxor_assign(&mut self, rhs: V) {
        self.0 ^= rhs.into().0;
    }
}

// |= over secret values
impl<T: BitOrAssign, V: Into<Secret<T>>> BitOrAssign<V> for Secret<T> {
    fn bitor_assign(&mut self, rhs: V) {
        self.0 |= rhs.into().0;
    }
}

// &= over secret values
impl<T: BitAndAssign, V: Into<Secret<T>>> BitAndAssign<V> for Secret<T> {
    fn bitand_assign(&mut self, rhs: V) {
        self.0 &= rhs.into().0;
    }
}

// >>= over secret values
impl<U, T: ShrAssign<U>> ShrAssign<U> for Secret<T> {
    fn shr_assign(&mut self, rhs: U) {
        self.0 >>= rhs;
    }
}

// <<= over secret values
impl<U, T: ShlAssign<U>> ShlAssign<U> for Secret<T> {
    fn shl_assign(&mut self, rhs: U) {
        self.0 <<= rhs;
    }
}

// Implement Intops for secret integers
macro_rules! impl_int_ops {
    ($t:ty) => {
        impl IntOps for Secret<$t> {
            fn wrapping_add<T: Into<Secret<$t>>>(self, rhs: T) -> Self {
                self.declassify()
                    .wrapping_add(rhs.into().declassify())
                    .classify()
            }
            fn wrapping_sub<T: Into<Secret<$t>>>(self, rhs: T) -> Self {
                self.declassify()
                    .wrapping_sub(rhs.into().declassify())
                    .classify()
            }
            fn wrapping_mul<T: Into<Secret<$t>>>(self, rhs: T) -> Self {
                self.declassify()
                    .wrapping_mul(rhs.into().declassify())
                    .classify()
            }
            fn wrapping_neg(self) -> Self {
                self.declassify().wrapping_neg().classify()
            }
            fn rotate_left(self, rhs: u32) -> Self {
                self.declassify().rotate_left(rhs).classify()
            }
            fn rotate_right(self, rhs: u32) -> Self {
                self.declassify().rotate_right(rhs).classify()
            }
        }
    };
}
impl_int_ops!(u8);
impl_int_ops!(u16);
impl_int_ops!(u32);
impl_int_ops!(u64);
impl_int_ops!(u128);
impl_int_ops!(i8);
impl_int_ops!(i16);
impl_int_ops!(i32);
impl_int_ops!(i64);
impl_int_ops!(i128);

// Implement EncodingOps for secret integers
macro_rules! impl_encode_ops {
    ($t:ty, $N:literal) => {
        impl EncodeOps<U8, $N> for Secret<$t> {
            fn to_le_bytes(&self) -> [U8; $N] {
                self.0.to_le_bytes().classify()
            }
            fn to_be_bytes(&self) -> [U8; $N] {
                self.0.to_be_bytes().classify()
            }
            fn from_le_bytes(x: [U8; $N]) -> Self {
                <$t>::from_le_bytes(x.declassify()).classify()
            }
            fn from_be_bytes(x: [U8; $N]) -> Self {
                <$t>::from_be_bytes(x.declassify()).classify()
            }
        }
    };
}
impl_encode_ops!(u8, 1);
impl_encode_ops!(u16, 2);
impl_encode_ops!(u32, 4);
impl_encode_ops!(u64, 8);
impl_encode_ops!(u128, 16);
impl_encode_ops!(i8, 1);
impl_encode_ops!(i16, 2);
impl_encode_ops!(i32, 4);
impl_encode_ops!(i64, 8);
impl_encode_ops!(i128, 16);
