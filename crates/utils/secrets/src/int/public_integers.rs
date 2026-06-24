/// This crate defines classification and declassification over public integers
/// All functions and types here are transparent (identities) and have no performance impact
use crate::traits::*;
pub type I8 = i8;
pub type U8 = u8;
pub type I16 = i16;
pub type U16 = u16;
pub type I32 = i32;
pub type U32 = u32;
pub type I64 = i64;
pub type U64 = u64;
#[cfg(not(eurydice))]
pub type I128 = i128;
#[cfg(not(eurydice))]
pub type U128 = u128;

/// Construct a public integer (identity)
#[inline(always)]
pub fn secret<T: Scalar>(x: T) -> T {
    x
}
