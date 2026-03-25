#[cfg(hax)]
use hax_lib::{self, constructors::from_bool, int::ToInt};

#[cfg(hax)]
use crate::proof_utils::{slices_same_len, valid_rate};

// XXX: These should be default functions on `KeccakItem`, but hax doesn't
//      support that yet. cryspen/hax#888
#[hax_lib::requires(i < 5 && j < 5)]
#[inline(always)]
pub(crate) fn get_ij<const N: usize, T: KeccakItem<N>>(arr: &[T; 25], i: usize, j: usize) -> &T {
    &arr[5 * j + i]
}

#[hax_lib::requires(i < 5 && j < 5)]
#[inline(always)]
pub(crate) fn set_ij<const N: usize, T: KeccakItem<N>>(
    arr: &mut [T; 25],
    i: usize,
    j: usize,
    value: T,
) {
    arr[5 * j + i] = value;
}

/// A Keccak Item for multiplexing arithmetic implementations.
#[hax_lib::attributes]
pub(crate) trait KeccakItem<const N: usize>: Clone + Copy {
    /// Zero
    #[hax_lib::requires(true)]
    fn zero() -> Self;

    /// `a ^ b ^ c ^ d ^ e`
    #[hax_lib::requires(true)]
    fn xor5(a: Self, b: Self, c: Self, d: Self, e: Self) -> Self;

    /// `a ^ (b <<< 1)``
    #[hax_lib::requires(true)]
    fn rotate_left1_and_xor(a: Self, b: Self) -> Self;

    /// `(a ^ b) <<< LEFT`
    #[hax_lib::requires(
        LEFT.to_int() + RIGHT.to_int() == 64.to_int() &&
        RIGHT > 0 &&
        RIGHT < 64
    )]
    fn xor_and_rotate<const LEFT: i32, const RIGHT: i32>(a: Self, b: Self) -> Self;

    /// `a ^ (b & !c)`
    #[hax_lib::requires(true)]
    fn and_not_xor(a: Self, b: Self, c: Self) -> Self;

    /// `a ^ c`
    #[hax_lib::requires(true)]
    fn xor_constant(a: Self, c: u64) -> Self;

    /// `a ^ b`
    #[hax_lib::requires(true)]
    fn xor(a: Self, b: Self) -> Self;
}

/// Trait to load new bytes into the state.
#[hax_lib::attributes]
pub(crate) trait Absorb<const N: usize> {
    /// Absorb a block
    #[hax_lib::requires(
      from_bool(
        N != 0 &&
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= input[0].len().to_int()
      ).and(
        slices_same_len(input)
      )
    )]
    fn load_block<const RATE: usize>(&mut self, input: &[&[u8]; N], start: usize);

    /// Absorb the last block (may be partial)
    #[hax_lib::requires(
      from_bool(
        N != 0 &&
        valid_rate(RATE) &&
        len < RATE &&
        start.to_int() + len.to_int() <= input[0].len().to_int()
      ).and(
        slices_same_len(input)
      )
    )]
    fn load_last<const RATE: usize, const DELIMITER: u8>(
        &mut self,
        input: &[&[u8]; N],
        start: usize,
        len: usize,
    );
}

// HAX does not support the &[mut &[u8]; N] Type. Because of that we have to
// implement the Squeeze trait for each N separately. This complicates the
// implementation in other files as well. Refactor when HAX supports it.
// https://github.com/cryspen/hax/issues/420

/// Trait to squeeze bytes out of the state.
///
/// Note that this is implemented for each platform (1, 2, 4) because hax can't
/// handle the mutability required for a generic implementation.
///
/// Store blocks `N = 1`
#[hax_lib::fstar::replace(
    interface, "
class t_Squeeze (v_Self: Type0) (v_T: Type0) = {
  // TODO: This super variable is problematic and makes typecheck fail
  // https://github.com/cryspen/hax/issues/1554
  // [@@@ FStar.Tactics.Typeclasses.no_method]_super_18390613159176269294:t_KeccakItem v_T (mk_usize 1);
  f_squeeze_pre:v_RATE: usize -> self_: v_Self -> out: t_Slice u8 -> start: usize -> len: usize
    -> pred:
      Type0
        { Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
          ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) <=
          (Rust_primitives.Hax.Int.from_machine (Core.Slice.impl__len #u8 out <: usize)
            <:
            Hax_lib.Int.t_Int) ==>
          pred };
  f_squeeze_post:
      v_RATE: usize ->
      self_: v_Self ->
      out: t_Slice u8 ->
      start: usize ->
      len: usize ->
      out_future: t_Slice u8
    -> pred:
      Type0
        { pred ==>
          (Core.Slice.impl__len #u8 out_future <: usize) =. (Core.Slice.impl__len #u8 out <: usize)
        };
  f_squeeze:v_RATE: usize -> x0: v_Self -> x1: t_Slice u8 -> x2: usize -> x3: usize
    -> Prims.Pure (t_Slice u8)
        (f_squeeze_pre v_RATE x0 x1 x2 x3)
        (fun result -> f_squeeze_post v_RATE x0 x1 x2 x3 result)
}

// TODO: See above
// [@@ FStar.Tactics.Typeclasses.tcinstance]
// let _ = fun (v_Self:Type0) (v_T:Type0) {|i: t_Squeeze v_Self v_T|} -> i._super_18390613159176269294
"
)]
#[hax_lib::attributes]
pub(crate) trait Squeeze<T: KeccakItem<1>> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        len <= RATE &&
        start.to_int() + len.to_int() <= out.len().to_int()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    fn squeeze<const RATE: usize>(&self, out: &mut [u8], start: usize, len: usize);
}

// Renaming the squeeze functions of the Squeeze2 and Squeeze4 Trait is currently
// necessary because F* will not find the correct function, pre and post conditions otherwise.
// Check the following issue: https://github.com/cryspen/hax/issues/1595

/// Trait to squeeze bytes out of the state.
///
/// Note that this is implemented for each platform (1, 2, 4) because hax can't
/// handle the mutability required for a generic implementation.
///
/// Store blocks `N = 2`
#[cfg(feature = "simd128")]
pub(crate) trait Squeeze2<T: KeccakItem<2>> {
    fn squeeze2<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        start: usize,
        len: usize,
    );
}

/// Trait to squeeze bytes out of the state.
///
/// Note that this is implemented for each platform (1, 2, 4) because hax can't
/// handle the mutability required for a generic implementation.
///
/// Store blocks `N = 4`
#[cfg(feature = "simd256")]
pub(crate) trait Squeeze4<T: KeccakItem<4>> {
    fn squeeze4<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
        start: usize,
        len: usize,
    );
}
