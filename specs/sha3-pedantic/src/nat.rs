//! The nonnegative integers the Standard counts with.
//!
//! FIPS 202 counts the way mathematics counts. `len(M)`, the `d` of
//! `KECCAK[c](N, d)`, the `j` of `pad10*1` and the block index of Algorithm 8
//! are nonnegative integers, and the Standard bounds none of them. A machine
//! word is the wrong shape for that — not because the numbers are large, but
//! because a word puts a bound on `len(M)` into the *specification*, where the
//! Standard has none, and then every proof written against the specification
//! has to carry that bound as a precondition. A `u64` length is how
//! `MAX_INPUT_LEN` came to stand in libcrux-iot's contracts, and a `usize`
//! length is how the same bound used to move with the target word size.
//!
//! So lengths are a [`Nat`]: opaque to the extraction, and modelled in
//! `Assumptions/TypesExternal.lean` as Lean's `Nat`, which does not stop.
//! Addition and multiplication are total there, so `8 * len(M)` and
//! `len(N) + len(pad)` are unconditional and no bound on the input reaches the
//! contracts. What stays partial is only what is partial in the mathematics or
//! at the boundary: `a - b` where `b > a`, division by zero, and handing a
//! value back down to the fixed-width layer as a `usize`.
//!
//! ## The `u128` underneath
//!
//! The model is exact on every state this crate can reach, and the argument is
//! the allocator's, not the Standard's. The bits counted here are held in a
//! `Vec<u8>`, which is at most `isize::MAX` bytes, so the longest [`BitStr`]
//! that can exist has fewer than `2^67` bits on any target; `pad10*1` adds at
//! most `r + 1 < 2^11` more, and the sponge's cursor `i * r` stays below the
//! length of the padded string. Against `2^128` that is sixty bits of
//! headroom, and it is headroom over what can be *allocated*, not over what
//! someone might pass. A `u64` could not make that argument: `8 * isize::MAX`
//! overflows one on a 64-bit target, which is why the bound had to be stated
//! rather than derived.
//!
//! Where a `u128` could nonetheless overflow, the operation is written
//! `checked_*` and panics: in the one place the model and the implementation
//! could part company, the implementation stops rather than wrapping, in
//! release as well as debug.
//!
//! [`BitStr`]: crate::bits::BitStr

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Rem, Sub};

/// A nonnegative integer — a length, a bit position, an output size.
///
/// Opaque to the extraction: the Lean model is `Nat`, with no upper bound at
/// all. The `u128` is how it is realised in Rust, and is not what the proofs
/// reason about.
#[cfg_attr(hax, hax_lib::opaque)]
#[cfg_attr(not(hax), derive(Debug))]
pub struct Nat(pub(crate) u128);

// `Clone` and `Copy` are hand-written for the same reason `BitStr`'s `Clone`
// is: a derive is an impl whose body reads the representation, which the
// extraction must not see. `Copy` has no body to hide, but it is written out
// next to `Clone` rather than derived so that the pair reads as one decision.
impl Clone for Nat {
    #[cfg_attr(hax, hax_lib::opaque)]
    fn clone(&self) -> Nat {
        Nat(self.0)
    }
}

impl Copy for Nat {}

impl Nat {
    /// The integer `x`.
    ///
    /// Takes a `u64` because every literal the Standard writes — a rate, a
    /// digest length, the `2` of Algorithm 9 — is small. Lengths that come
    /// from the machine arrive through [`Nat::from_usize`] instead.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn new(x: u64) -> Nat {
        Nat(x as u128)
    }

    /// The integer `x`, from a count the machine made: a byte count, a slice
    /// length, the width `b`.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn from_usize(x: usize) -> Nat {
        Nat(x as u128)
    }

    /// Back down to the fixed-width layer.
    ///
    /// Partial, and the only partiality here that is about a machine rather
    /// than about arithmetic: the fixed-width layer works on `b`-bit strings
    /// for a `b` in Table 1, so every call site hands it a rate or a width,
    /// never a length that came from the input.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn to_usize(self) -> usize {
        assert!(
            self.0 <= usize::MAX as u128,
            "value too large to hand to the fixed-width layer"
        );
        self.0 as usize
    }
}

impl Add for Nat {
    type Output = Nat;

    /// Total in the model. `checked_add` so that the implementation stops
    /// rather than wrapping past it; see the module docs for why it cannot.
    #[cfg_attr(hax, hax_lib::opaque)]
    fn add(self, other: Nat) -> Nat {
        Nat(self.0.checked_add(other.0).expect("Nat: sum too large"))
    }
}

impl Sub for Nat {
    type Output = Nat;

    /// Partial, and partial in the model too: the nonnegative integers are not
    /// closed under subtraction.
    #[cfg_attr(hax, hax_lib::opaque)]
    fn sub(self, other: Nat) -> Nat {
        Nat(self
            .0
            .checked_sub(other.0)
            .expect("Nat: difference below zero"))
    }
}

impl Mul for Nat {
    type Output = Nat;

    /// Total in the model — this is `8 * len`, the multiplication that used to
    /// carry the bound.
    #[cfg_attr(hax, hax_lib::opaque)]
    fn mul(self, other: Nat) -> Nat {
        Nat(self.0.checked_mul(other.0).expect("Nat: product too large"))
    }
}

impl Div for Nat {
    type Output = Nat;

    /// Truncating division, partial at zero as division is.
    #[cfg_attr(hax, hax_lib::opaque)]
    fn div(self, other: Nat) -> Nat {
        assert!(other.0 != 0, "Nat: division by zero");
        Nat(self.0 / other.0)
    }
}

impl Rem for Nat {
    type Output = Nat;

    /// `a mod b`, partial at zero as division is.
    #[cfg_attr(hax, hax_lib::opaque)]
    fn rem(self, other: Nat) -> Nat {
        assert!(other.0 != 0, "Nat: division by zero");
        Nat(self.0 % other.0)
    }
}

// The comparisons are written out rather than derived, and `PartialOrd`
// overrides `lt`, `le`, `gt` and `ge` rather than leaving them to their
// defaults: `<` and `<=` are what the Standard's loops and the sponge's
// "if `d ≤ |Z|`" are written with, so each has to be an operation the
// extraction can name and the model can define.
impl PartialEq for Nat {
    #[cfg_attr(hax, hax_lib::opaque)]
    fn eq(&self, other: &Nat) -> bool {
        self.0 == other.0
    }

    #[cfg_attr(hax, hax_lib::opaque)]
    fn ne(&self, other: &Nat) -> bool {
        self.0 != other.0
    }
}

impl Eq for Nat {}

impl PartialOrd for Nat {
    #[cfg_attr(hax, hax_lib::opaque)]
    fn partial_cmp(&self, other: &Nat) -> Option<Ordering> {
        Some(if self.0 < other.0 {
            Ordering::Less
        } else if self.0 == other.0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        })
    }

    #[cfg_attr(hax, hax_lib::opaque)]
    fn lt(&self, other: &Nat) -> bool {
        self.0 < other.0
    }

    #[cfg_attr(hax, hax_lib::opaque)]
    fn le(&self, other: &Nat) -> bool {
        self.0 <= other.0
    }

    #[cfg_attr(hax, hax_lib::opaque)]
    fn gt(&self, other: &Nat) -> bool {
        self.0 > other.0
    }

    #[cfg_attr(hax, hax_lib::opaque)]
    fn ge(&self, other: &Nat) -> bool {
        self.0 >= other.0
    }
}
