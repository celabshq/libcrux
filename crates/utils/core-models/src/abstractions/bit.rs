//! # Bit Manipulation and Machine Integer Utilities
//!
//! This module provides utilities for working with individual bits and machine integer types.
//! It defines a [`Bit`] enum to represent a single bit (`0` or `1`) along with convenient
//! conversion implementations between `Bit`, [`bool`], and various primitive integer types.
//!
//! In addition, the module introduces the [`MachineInteger`] trait which abstracts over
//! integer types, providing associated constants:
//!
//! - `BITS`: The size of the integer type in bits.
//! - `SIGNED`: A flag indicating whether the type is signed.
//!
//! The [`Bit`] type includes methods for extracting the value of a specific bit from an integer.
//! For example, [`Bit::of_int`] returns the bit at a given position for a provided integer,
//! handling both positive and negative values (assuming a two's complement representation).
//!
//! # Examples
//!
//! ```rust
//! use libcrux_core_models::abstractions::bit::{Bit, MachineInteger};
//!
//! // Extract the 3rd bit (0-indexed) from an integer.
//! let bit = Bit::of_int(42, 2);
//! println!("The extracted bit is: {:?}", bit);
//!
//! // Convert Bit to a primitive integer type.
//! let num: u8 = bit.into();
//! println!("As an integer: {}", num);
//! ```
//!
//! [`bool`]: https://doc.rust-lang.org/std/primitive.bool.html
//! [`Bit::of_int`]: enum.Bit.html#method.of_int

/// Represent a bit: `0` or `1`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Bit {
    Zero,
    One,
}

macro_rules! generate_from_bit_impls {
    ($($ty:ident),*) => {
        $(impl From<Bit> for $ty {
            fn from(bit: Bit) -> Self {
                bool::from(bit) as $ty
            }
        })*
    };
}
generate_from_bit_impls!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl From<Bit> for bool {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl From<bool> for Bit {
    fn from(b: bool) -> Bit {
        match b {
            false => Bit::Zero,
            true => Bit::One,
        }
    }
}

/// A trait for types that represent machine integers.
#[hax_lib::attributes]
pub trait MachineInteger {
    /// The size of this integer type in bits.
    #[hax_lib::requires(true)]
    #[hax_lib::ensures(|bits| bits >= 8)]
    fn bits() -> u32;

    /// The signedness of this integer type.
    const SIGNED: bool;
}

macro_rules! generate_machine_integer_impls {
    ($($ty:ident),*) => {
        $(#[hax_lib::exclude]impl MachineInteger for $ty {
            fn bits() -> u32 { $ty::BITS }
            #[allow(unused_comparisons)]
            const SIGNED: bool = $ty::MIN < 0;
        })*
    };
}
generate_machine_integer_impls!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[hax_lib::fstar::replace(
    r"
instance impl_MachineInteger_poly (t: inttype): t_MachineInteger (int_t t) =
  { f_bits = (fun () -> mk_u32 (bits t));
    f_bits_pre = (fun () -> True);
    f_bits_post = (fun () r -> r == mk_u32 (bits t));
    f_SIGNED = signed t }
"
)]
const _: () = {};

#[hax_lib::exclude]
impl Bit {
    fn of_raw_int(x: u128, nth: u32) -> Self {
        if x / 2u128.pow(nth) % 2 == 1 {
            Self::One
        } else {
            Self::Zero
        }
    }

    pub fn of_int<T: Into<i128> + MachineInteger>(x: T, nth: u32) -> Bit {
        let x: i128 = x.into();
        if x >= 0 {
            Self::of_raw_int(x as u128, nth)
        } else {
            Self::of_raw_int((2i128.pow(T::bits()) + x) as u128, nth)
        }
    }
}
