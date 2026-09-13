//! This module provides a specification-friendly bit vector type.
use super::bit::{Bit, MachineInteger};
use super::funarr::*;

use std::fmt::Formatter;

// This is required due to some hax-lib inconsistencies with versus without `cfg(hax)`.
#[cfg(hax)]
use hax_lib::{int, ToInt};

// TODO: this module uses `u128/i128` as mathematic integers. We should use `hax_lib::int` or bigint.

/// A fixed-size bit vector type.
///
/// `BitVec<N>` is a specification-friendly, fixed-length bit vector that internally
/// stores an array of [`Bit`] values, where each `Bit` represents a single binary digit (0 or 1).
///
/// This type provides several utility methods for constructing and converting bit vectors:
///
/// The [`Debug`] implementation for `BitVec` pretty-prints the bits in groups of eight,
/// making the bit pattern more human-readable. The type also implements indexing,
/// allowing for easy access to individual bits.
#[hax_lib::fstar::before("noeq")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitVec<const N: u64>(FunArray<N, Bit>);

/// Pretty prints a bit slice by group of 8
#[hax_lib::exclude]
fn bit_slice_to_string(bits: &[Bit]) -> String {
    bits.iter()
        .map(|bit| match bit {
            Bit::Zero => '0',
            Bit::One => '1',
        })
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|bits| bits.iter().collect::<String>())
        .map(|s| format!("{s} "))
        .collect::<String>()
        .trim()
        .into()
}

#[hax_lib::exclude]
impl<const N: u64> core::fmt::Debug for BitVec<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", bit_slice_to_string(&self.0.as_vec()))
    }
}

#[hax_lib::attributes]
impl<const N: u64> core::ops::Index<u64> for BitVec<N> {
    type Output = Bit;
    #[requires(index < N)]
    fn index(&self, index: u64) -> &Self::Output {
        self.0.get(index)
    }
}

/// Convert a bit slice into an unsigned number.
#[hax_lib::exclude]
fn u128_int_from_bit_slice(bits: &[Bit]) -> u128 {
    bits.iter()
        .enumerate()
        .map(|(i, bit)| u128::from(*bit) << i)
        .sum::<u128>()
}

/// Convert a bit slice into a machine integer of type `T`.
#[hax_lib::exclude]
fn int_from_bit_slice<T: TryFrom<i128> + MachineInteger + Copy>(bits: &[Bit]) -> T {
    debug_assert!(bits.len() <= T::bits() as usize);
    let result = if T::SIGNED {
        let is_negative = matches!(bits[T::bits() as usize - 1], Bit::One);
        let s = u128_int_from_bit_slice(&bits[0..T::bits() as usize - 1]) as i128;
        if is_negative {
            s + (-2i128).pow(T::bits() - 1)
        } else {
            s
        }
    } else {
        u128_int_from_bit_slice(bits) as i128
    };
    let Ok(n) = result.try_into() else {
        // Conversion must succeed as `result` is guaranteed to be in range due to the bit-length check.
        unreachable!()
    };
    n
}

#[libcrux_macros::trusted(replace, "trusted-extern: BitVec::from_fn F* model (bit-vector primitive)")]
#[hax_lib::fstar::replace(
    r#"
let ${BitVec::<0>::from_fn::<fn(u64)->Bit>}
    (v_N: u64)
    (#_v_F: Type0)
    (f: (i: u64 {v i < v v_N}) -> $:{Bit})
    : t_BitVec v_N =
    ${BitVec::<0>}(${FunArray::<0,()>::from_fn::<fn(u64)->()>} v_N #$:{Bit} #(u64 -> $:{Bit}) f)
"#
)]
const _: () = ();

macro_rules! impl_pointwise {
    ($n:literal, $($i:literal)*) => {
        impl BitVec<$n> {
            pub fn pointwise(self) -> Self {
                Self::from_fn(|i| match i {
                    $($i => self[$i],)*
                    _ => unreachable!(),
                })
            }
        }
    };
}

impl_pointwise!(128, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127);
impl_pointwise!(256, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138 139 140 141 142 143 144 145 146 147 148 149 150 151 152 153 154 155 156 157 158 159 160 161 162 163 164 165 166 167 168 169 170 171 172 173 174 175 176 177 178 179 180 181 182 183 184 185 186 187 188 189 190 191 192 193 194 195 196 197 198 199 200 201 202 203 204 205 206 207 208 209 210 211 212 213 214 215 216 217 218 219 220 221 222 223 224 225 226 227 228 229 230 231 232 233 234 235 236 237 238 239 240 241 242 243 244 245 246 247 248 249 250 251 252 253 254 255);

/// An F* attribute that indiquates a rewritting lemma should be applied
pub const REWRITE_RULE: () = {};

#[hax_lib::exclude]
impl<const N: u64> BitVec<N> {
    /// Constructor for BitVec. `BitVec::<N>::from_fn` constructs a bitvector out of a function that takes usizes smaller than `N` and produces bits.
    pub fn from_fn<F: Fn(u64) -> Bit>(f: F) -> Self {
        Self(FunArray::from_fn(f))
    }
    /// Convert a slice of machine integers where only the `d` least significant bits are relevant.
    pub fn from_slice<T: Into<i128> + MachineInteger + Copy>(x: &[T], d: u64) -> Self {
        Self::from_fn(|i| Bit::of_int::<T>(x[(i / d) as usize], (i % d) as u32))
    }

    /// Construct a BitVec out of a machine integer.
    pub fn from_int<T: Into<i128> + MachineInteger + Copy>(n: T) -> Self {
        Self::from_slice::<T>(&[n], T::bits() as u64)
    }

    /// Convert a BitVec into a machine integer of type `T`.
    pub fn to_int<T: TryFrom<i128> + MachineInteger + Copy>(self) -> T {
        int_from_bit_slice(&self.0.as_vec())
    }

    /// Convert a BitVec into a vector of machine integers of type `T`.
    pub fn to_vec<T: TryFrom<i128> + MachineInteger + Copy>(&self) -> Vec<T> {
        self.0
            .as_vec()
            .chunks(T::bits() as usize)
            .map(int_from_bit_slice)
            .collect()
    }

    /// Generate a random BitVec.
    pub fn rand() -> Self {
        use rand::prelude::*;
        let random_source: Vec<_> = {
            let mut rng = rand::rng();
            (0..N).map(|_| rng.random::<bool>()).collect()
        };
        Self::from_fn(|i| random_source[i as usize].into())
    }
}

#[libcrux_macros::trusted(replace, "hax-limitation: F*-native functional-extensionality lemma (proof machinery)")]
#[hax_lib::fstar::replace(
    r#"
open FStar.FunctionalExtensionality

let extensionality' (#a: Type) (#b: Type) (f g: FStar.FunctionalExtensionality.(a ^-> b))
  : Lemma (ensures (FStar.FunctionalExtensionality.feq f g <==> f == g))
  = ()

let mark_to_normalize #t (x: t): t = x

open FStar.Tactics.V2
#push-options "--z3rlimit 80 --admit_smt_queries true"
let bitvec_rewrite_lemma_128 (x: $:{BitVec<128>})
: Lemma (x == mark_to_normalize (${BitVec::<128>::pointwise} x)) =
    let a = x._0._0 in
    let b = (${BitVec::<128>::pointwise} x)._0._0 in
    assert_norm (FStar.FunctionalExtensionality.feq a b);
    extensionality' a b

let bitvec_rewrite_lemma_256 (x: $:{BitVec<256>})
: Lemma (x == mark_to_normalize (${BitVec::<256>::pointwise} x)) =
    let a = x._0._0 in
    let b = (${BitVec::<256>::pointwise} x)._0._0 in
    assert_norm (FStar.FunctionalExtensionality.feq a b);
    extensionality' a b
#pop-options

let bitvec_postprocess_norm_aux (): Tac unit = with_compat_pre_core 1 (fun () ->
    let debug_mode = ext_enabled "debug_bv_postprocess_rewrite" in
    let crate = match cur_module () with | crate::_ -> crate | _ -> fail "Empty module name" in
    // Remove indirections
    norm [primops; iota; delta_namespace [crate; "Libcrux_intrinsics"]; zeta_full];
    // Rewrite call chains
    let lemmas = FStar.List.Tot.map (fun f -> pack_ln (FStar.Stubs.Reflection.V2.Data.Tv_FVar f)) (lookup_attr (`${REWRITE_RULE}) (top_env ())) in
    l_to_r lemmas;
    /// Get rid of casts
    norm [primops; iota; delta_namespace ["Rust_primitives"; "Prims.pow2"]; zeta_full];
    if debug_mode then print ("[postprocess_rewrite_helper] lemmas = " ^ term_to_string (quote lemmas));

    l_to_r [`bitvec_rewrite_lemma_128; `bitvec_rewrite_lemma_256];

    let round _: Tac unit =
        if debug_mode then dump "[postprocess_rewrite_helper] Rewrote goal";
        // Normalize as much as possible
        norm [primops; iota; delta_namespace ["Core"; crate; "Core_models"; "Libcrux_intrinsics"; "FStar.FunctionalExtensionality"; "Rust_primitives"]; zeta_full];
        if debug_mode then print ("[postprocess_rewrite_helper] first norm done");
        // Compute the last bits
        // compute ();
        // if debug_mode then dump ("[postprocess_rewrite_helper] compute done");
        // Force full normalization
        norm [primops; iota; delta; unascribe; zeta_full];
        if debug_mode then dump "[postprocess_rewrite_helper] after full normalization";
        // Solves the goal `<normalized body> == ?u`
        trefl ()
    in

    ctrl_rewrite BottomUp (fun t ->
        let f, args = collect_app t in
        let matches = match inspect f with | Tv_UInst f _ | Tv_FVar f -> (inspect_fv f) = explode_qn (`%mark_to_normalize) | _ -> false in
        let has_two_args = match args with | [_; _] -> true | _ -> false in
        (matches && has_two_args, Continue)
    ) round;

    // Solves the goal `<normalized body> == ?u`
    trefl ()
)

let ${bitvec_postprocess_norm} (): Tac unit =
    if lax_on ()
    then trefl () // don't bother rewritting the goal
    else bitvec_postprocess_norm_aux ()
"#
)]
/// This function is useful only for verification in F*.
/// Used with `postprocess_rewrite`, this tactic:
///  1. Applies a series of rewrite rules (the lemmas marked with `REWRITE_RULE`)
///  2. Normalizes, bottom-up, every sub-expressions typed `BitVec<_>` inside the body of a function.
/// This tactic should be used on expressions that compute a _static_ permutation of bits.
pub fn bitvec_postprocess_norm() {}

#[hax_lib::attributes]
impl<const N: u64> BitVec<N> {
    #[hax_lib::requires(CHUNK > 0 && CHUNK.to_int() * SHIFTS.to_int() == N.to_int())]
    pub fn chunked_shift<const CHUNK: u64, const SHIFTS: u64>(
        self,
        shl: FunArray<SHIFTS, i128>,
    ) -> BitVec<N> {
        // TODO: this inner method is because of https://github.com/cryspen/hax-evit/issues/29
        #[hax_lib::fstar::options("--z3rlimit 50 --split_queries always")]
        #[hax_lib::requires(CHUNK > 0 && CHUNK.to_int() * SHIFTS.to_int() == N.to_int())]
        fn chunked_shift<const N: u64, const CHUNK: u64, const SHIFTS: u64>(
            bitvec: BitVec<N>,
            shl: FunArray<SHIFTS, i128>,
        ) -> BitVec<N> {
            BitVec::from_fn(|i| {
                let nth_bit = i % CHUNK;
                let nth_chunk = i / CHUNK;
                hax_lib::assert_prop!(nth_chunk.to_int() <= SHIFTS.to_int() - int!(1));
                hax_lib::assert_prop!(
                    nth_chunk.to_int() * CHUNK.to_int()
                        <= (SHIFTS.to_int() - int!(1)) * CHUNK.to_int()
                );
                let shift: i128 = if nth_chunk < SHIFTS {
                    shl[nth_chunk]
                } else {
                    0
                };
                let local_index = (nth_bit as i128).wrapping_sub(shift);
                if local_index < CHUNK as i128 && local_index >= 0 {
                    let local_index = local_index as u64;
                    hax_lib::assert_prop!(
                        nth_chunk.to_int() * CHUNK.to_int() + local_index.to_int()
                            < SHIFTS.to_int() * CHUNK.to_int()
                    );
                    bitvec[nth_chunk * CHUNK + local_index]
                } else {
                    Bit::Zero
                }
            })
        }
        chunked_shift::<N, CHUNK, SHIFTS>(self, shl)
    }

    /// Folds over the array, accumulating a result.
    ///
    /// # Arguments
    /// * `init` - The initial value of the accumulator.
    /// * `f` - A function combining the accumulator and each element.
    pub fn fold<A>(&self, init: A, f: fn(A, Bit) -> A) -> A {
        self.0.fold(init, f)
    }
}

pub mod int_vec_interp {
    //! This module defines interpretation for bit vectors as vectors of machine integers of various size and signedness.
    use super::*;

    /// An F* attribute that marks an item as being an interpretation lemma.
    #[allow(dead_code)]
    #[hax_lib::fstar::before("irreducible")]
    pub const SIMPLIFICATION_LEMMA: () = ();

    /// Derives interpretations functions, simplification lemmas and type
    /// synonyms.
    macro_rules! interpretations {
        ($n:literal; $($name:ident [$ty:ty; $m:literal]),*) => {
            $(
                #[doc = concat!(stringify!($ty), " vectors of size ", stringify!($m))]
                #[allow(non_camel_case_types)]
                pub type $name = FunArray<$m, $ty>;
                pastey::paste! {
                    const _: ()  = {
                        // Concretized to real F* defs via the module-level
                        // `fstar::replace` block below (excluded here so the
                        // hand-written defs are the sole extraction).
                        #[hax_lib::exclude]
                        impl BitVec<$n> {
                            #[doc = concat!("Conversion from ", stringify!($ty), " vectors of size ", stringify!($m), "to  bit vectors of size ", stringify!($n))]
                            pub fn [< from_ $name >](iv: $name) -> BitVec<$n> {
                                let vec: Vec<$ty> = iv.as_vec();
                                Self::from_slice(&vec[..], <$ty>::bits() as u64)
                            }
                            #[doc = concat!("Conversion from bit vectors of size ", stringify!($n), " to ", stringify!($ty), " vectors of size ", stringify!($m))]
                            pub fn [< to_ $name >](bv: BitVec<$n>) -> $name {
                                let vec: Vec<$ty> = bv.to_vec();
                                $name::from_fn(|i| vec[i as usize])
                            }
                        }

                        #[cfg(test)]
                        impl From<BitVec<$n>> for $name {
                            fn from(bv: BitVec<$n>) -> Self {
                                BitVec::[< to_ $name >](bv)
                            }
                        }
                        #[cfg(test)]
                        impl From<$name> for BitVec<$n> {
                            fn from(iv: $name) -> Self {
                                BitVec::[< from_ $name >](iv)
                            }
                        }
                    };
                }
            )*
        };
    }

    // Defines the types `i32x8` and `i64x4`, and define intepretations function
    // (`From` instances) from/to those types from/to bit vectors.
    //
    // We will need more such interpreations in the future to handle more avx2
    // intrinsics (e.g. `_mm256_add_epi16` works on 16 bits integers, not on i32
    // or i64).
    interpretations!(256; i32x8 [i32; 8], i64x4 [i64; 4], i16x16 [i16; 16], i128x2 [i128; 2], i8x32 [i8; 32],
		     u32x8 [u32; 8], u64x4 [u64; 4], u16x16 [u16; 16], u8x32 [u8; 32]);
    interpretations!(128; i32x4 [i32; 4], i64x2 [i64; 2], i16x8 [i16; 8], i128x1 [i128; 1], i8x16 [i8; 16],
		     u32x4 [u32; 4], u64x2 [u64; 2], u16x8 [u16; 8], u8x16 [u8; 16]);
    // 64-bit interpretations — needed for ARM NEON's `*_t` half-vectors:
    // int32x2_t = i32x2, int16x4_t = i16x4, int8x8_t = i8x8, etc.
    interpretations!(64; i32x2 [i32; 2], i16x4 [i16; 4], i8x8 [i8; 8], i64x1 [i64; 1],
		     u32x2 [u32; 2], u16x4 [u16; 4], u8x8 [u8; 8], u64x1 [u64; 1]);

    /// Concrete lane-view conversions (bit-slice + two's-complement) with a
    /// single generic round-trip lemma `lemma_conv_rt` (SMTPat). Gives the
    /// macro conversions real F* definitions rather than opaque stubs.
    #[libcrux_macros::trusted(replace, "trusted-extern: BitVec concrete lane-view conversions (bit-vector primitive)")]
    #[hax_lib::fstar::replace(
        r#"
(* ============================================================
   Concrete lane-view conversions for core-models BitVec.
   Generic base-2 / two's-complement codec + generic tiling
   (from_iv / to_iv) + a single generic round-trip lemma.
   Each per-width conversion below is a thin instantiation.
   (Grandfathered-OK primitive modeling for the BitVec base.)
   ============================================================ *)

let bval (b: Libcrux_core_models.Abstractions.Bit.t_Bit) : n: nat{n < 2} =
  match b with
  | Libcrux_core_models.Abstractions.Bit.Bit_Zero  -> 0
  | Libcrux_core_models.Abstractions.Bit.Bit_One  -> 1

let ebit (m: nat) (b: nat) : Libcrux_core_models.Abstractions.Bit.t_Bit =
  if (m / pow2 b) % 2 = 1
  then Libcrux_core_models.Abstractions.Bit.Bit_One
  else Libcrux_core_models.Abstractions.Bit.Bit_Zero

let rec dsum2 (f: nat -> Libcrux_core_models.Abstractions.Bit.t_Bit) (off: nat) (n: nat)
    : Tot nat (decreases n) =
  if n = 0 then 0 else bval (f off) + 2 * dsum2 f (off + 1) (n - 1)

let lemA2 (m: nat) (off: nat) : Lemma (m / pow2 (off + 1) == (m / pow2 off) / 2) =
  FStar.Math.Lemmas.pow2_plus off 1;
  FStar.Math.Lemmas.division_multiplication_lemma m (pow2 off) 2

let lemB (m: nat) (k: pos) : Lemma ((m % 2) + 2 * ((m / 2) % k) == m % (2 * k)) =
  FStar.Math.Lemmas.lemma_div_mod m 2;
  FStar.Math.Lemmas.lemma_div_mod (m / 2) k;
  FStar.Math.Lemmas.lemma_mod_plus (m % 2 + 2 * ((m / 2) % k)) ((m / 2) / k) (2 * k);
  FStar.Math.Lemmas.small_mod (m % 2 + 2 * ((m / 2) % k)) (2 * k)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 60"
let rec lemma_dsum2 (f: nat -> Libcrux_core_models.Abstractions.Bit.t_Bit) (m: nat) (off: nat) (n: nat)
      (h: (b: nat) -> Lemma (bval (f b) == (m / pow2 b) % 2))
    : Lemma (ensures dsum2 f off n == (m / pow2 off) % pow2 n) (decreases n) =
  if n = 0
  then (assert_norm (pow2 0 == 1))
  else
    (lemma_dsum2 f m (off + 1) (n - 1) h;
      h off;
      lemA2 m off;
      lemB (m / pow2 off) (pow2 (n - 1));
      FStar.Math.Lemmas.pow2_plus 1 (n - 1))

let rec dsum2_bound (f: nat -> Libcrux_core_models.Abstractions.Bit.t_Bit) (off n: nat)
    : Lemma (ensures dsum2 f off n < pow2 n) (decreases n) =
  if n = 0 then () else (dsum2_bound f (off + 1) (n - 1); FStar.Math.Lemmas.pow2_plus 1 (n - 1))
#pop-options

let ebit_bit (m: nat) (b: nat) : Lemma (bval (ebit m b) == (m / pow2 b) % 2) = ()

let bits_ge1 (t: Rust_primitives.Integers.inttype)
    : Lemma (Rust_primitives.Integers.bits t >= 8) = ()

let bits_le128 (t: Rust_primitives.Integers.inttype)
    : Lemma (Rust_primitives.Integers.bits t <= 128) = ()

let div_lt (a b c: nat) : Lemma (requires c >= 1 /\ b >= 1 /\ a < b * c) (ensures a / c < b) =
  FStar.Math.Lemmas.lemma_div_plus (c - 1) (b - 1) c;
  FStar.Math.Lemmas.small_div (c - 1) c;
  FStar.Math.Lemmas.lemma_div_le a (b * c - 1) c

let pow2_split (n: nat)
    : Lemma (requires n >= 1) (ensures pow2 n == 2 * pow2 (n - 1)) =
  FStar.Math.Lemmas.pow2_plus 1 (n - 1)

let encode_bit (t: Rust_primitives.Integers.inttype) (x: Rust_primitives.Integers.int_t t) (b: nat)
    : Libcrux_core_models.Abstractions.Bit.t_Bit =
  ebit ((Rust_primitives.Integers.v x) % pow2 (Rust_primitives.Integers.bits t)) b

let tc_of_u (t: Rust_primitives.Integers.inttype) (u: nat) : int =
  if Rust_primitives.Integers.signed t && u >= pow2 (Rust_primitives.Integers.bits t - 1)
  then u - pow2 (Rust_primitives.Integers.bits t)
  else u

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_tc_of_u_rt (t: Rust_primitives.Integers.inttype) (x: Rust_primitives.Integers.int_t t)
    : Lemma (tc_of_u t ((Rust_primitives.Integers.v x) % pow2 (Rust_primitives.Integers.bits t)) ==
        Rust_primitives.Integers.v x) =
  bits_ge1 t;
  pow2_split (Rust_primitives.Integers.bits t);
  let n = Rust_primitives.Integers.bits t in
  let vx = Rust_primitives.Integers.v x in
  if Rust_primitives.Integers.signed t
  then
    (if vx >= 0
      then FStar.Math.Lemmas.small_mod vx (pow2 n)
      else
        (FStar.Math.Lemmas.lemma_mod_plus vx 1 (pow2 n);
          FStar.Math.Lemmas.small_mod (vx + pow2 n) (pow2 n)))
  else FStar.Math.Lemmas.small_mod vx (pow2 n)

let lemma_encode_dsum2 (t: Rust_primitives.Integers.inttype) (x: Rust_primitives.Integers.int_t t)
    : Lemma (dsum2 (fun b -> encode_bit t x b) 0 (Rust_primitives.Integers.bits t) ==
        (Rust_primitives.Integers.v x) % pow2 (Rust_primitives.Integers.bits t)) =
  let n = Rust_primitives.Integers.bits t in
  let u = (Rust_primitives.Integers.v x) % pow2 n in
  FStar.Math.Lemmas.lemma_mod_lt (Rust_primitives.Integers.v x) (pow2 n);
  lemma_dsum2 (fun b -> encode_bit t x b) u 0 n (fun b -> ebit_bit u b);
  assert_norm (pow2 0 == 1);
  FStar.Math.Lemmas.small_mod u (pow2 n)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 100"
let lemma_tc_range (t: Rust_primitives.Integers.inttype) (u: nat)
    : Lemma (requires u < pow2 (Rust_primitives.Integers.bits t))
      (ensures Rust_primitives.Integers.range (tc_of_u t u) t) =
  bits_ge1 t;
  pow2_split (Rust_primitives.Integers.bits t)

let decode_lane (t: Rust_primitives.Integers.inttype) (f: nat -> Libcrux_core_models.Abstractions.Bit.t_Bit)
    : Rust_primitives.Integers.int_t t =
  bits_ge1 t;
  dsum2_bound f 0 (Rust_primitives.Integers.bits t);
  let u = dsum2 f 0 (Rust_primitives.Integers.bits t) in
  lemma_tc_range t u;
  Rust_primitives.Integers.mk_int #t (tc_of_u t u)

let lemma_decode_encode (t: Rust_primitives.Integers.inttype) (x: Rust_primitives.Integers.int_t t)
    : Lemma (decode_lane t (fun b -> encode_bit t x b) == x) =
  lemma_encode_dsum2 t x;
  lemma_tc_of_u_rt t x
#pop-options

let lane_reader (n: u64) (w: nat) (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n) (i: u64) (b: nat)
    : Libcrux_core_models.Abstractions.Bit.t_Bit =
  if b < w && w * v i + b < v n
  then
    Libcrux_core_models.Abstractions.Funarr.impl_5__get n
      #Libcrux_core_models.Abstractions.Bit.t_Bit bv._0 (mk_u64 (w * v i + b))
  else Libcrux_core_models.Abstractions.Bit.Bit_Zero

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
[@@ "opaque_to_smt"]
let from_iv
      (t: Rust_primitives.Integers.inttype)
      (n m: u64)
      (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray m (Rust_primitives.Integers.int_t t))
    : Prims.Pure (Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
      (requires v n == v m * Rust_primitives.Integers.bits t)
      (ensures fun _ -> True) =
  bits_ge1 t;
  bits_le128 t;
  let w:u64 = mk_u64 (Rust_primitives.Integers.bits t) in
  Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn n
    #(u64 -> Libcrux_core_models.Abstractions.Bit.t_Bit)
    (fun (j: u64{v j < v n}) ->
        div_lt (v j) (v m) (v w);
        let lane_idx:u64 = j /! w in
        encode_bit t
          (Libcrux_core_models.Abstractions.Funarr.impl_5__get m #(Rust_primitives.Integers.int_t t) iv lane_idx)
          (v (j %! w)))

[@@ "opaque_to_smt"]
let to_iv
      (t: Rust_primitives.Integers.inttype)
      (n m: u64)
      (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
    : Prims.Pure (Libcrux_core_models.Abstractions.Funarr.t_FunArray m (Rust_primitives.Integers.int_t t))
      (requires v n == v m * Rust_primitives.Integers.bits t)
      (ensures fun _ -> True) =
  bits_ge1 t;
  Libcrux_core_models.Abstractions.Funarr.impl_5__from_fn m
    #(Rust_primitives.Integers.int_t t)
    #(u64 -> Rust_primitives.Integers.int_t t)
    (fun (i: u64{v i < v m}) -> decode_lane t (lane_reader n (Rust_primitives.Integers.bits t) bv i))
#pop-options

let funarr_ext
      (m: u64)
      (#elem: Type0)
      (a b: Libcrux_core_models.Abstractions.Funarr.t_FunArray m elem)
      (h: (i: u64{v i < v m}) -> Lemma (a._0 i == b._0 i))
    : Lemma (a == b) =
  introduce forall (i: u64{v i < v m}). a._0 i == b._0 i with (h i);
  assert (FStar.FunctionalExtensionality.feq a._0 b._0);
  Libcrux_core_models.Abstractions.Bitvec.extensionality' a._0 b._0

#push-options "--fuel 1 --z3rlimit 200"
let reader_h_gen
      (t: Rust_primitives.Integers.inttype)
      (n m: u64)
      (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray m (Rust_primitives.Integers.int_t t))
      (i: u64{v i < v m})
      (b: nat)
    : Lemma (requires v n == v m * Rust_primitives.Integers.bits t)
      (ensures
        bval (lane_reader n (Rust_primitives.Integers.bits t) (from_iv t n m iv) i b) ==
        (((v (Libcrux_core_models.Abstractions.Funarr.impl_5__get m #(Rust_primitives.Integers.int_t t) iv i)) %
            pow2 (Rust_primitives.Integers.bits t)) /
          pow2 b) %
        2) =
  reveal_opaque (`%from_iv) from_iv;
  bits_ge1 t;
  let w = Rust_primitives.Integers.bits t in
  let iv_i = Libcrux_core_models.Abstractions.Funarr.impl_5__get m #(Rust_primitives.Integers.int_t t) iv i in
  let u = (v iv_i) % pow2 w in
  if b < w
  then
    (FStar.Math.Lemmas.lemma_div_plus b (v i) w;
      FStar.Math.Lemmas.lemma_mod_plus b (v i) w;
      FStar.Math.Lemmas.small_div b w;
      FStar.Math.Lemmas.small_mod b w;
      ebit_bit u b)
  else
    (FStar.Math.Lemmas.pow2_le_compat b w;
      FStar.Math.Lemmas.small_div u (pow2 b))
#pop-options

#push-options "--fuel 1 --z3rlimit 200"
let my_lane_gen
      (t: Rust_primitives.Integers.inttype)
      (n m: u64)
      (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray m (Rust_primitives.Integers.int_t t))
      (i: u64{v i < v m})
    : Lemma (requires v n == v m * Rust_primitives.Integers.bits t)
      (ensures
        (to_iv t n m (from_iv t n m iv))._0 i ==
        Libcrux_core_models.Abstractions.Funarr.impl_5__get m #(Rust_primitives.Integers.int_t t) iv i) =
  reveal_opaque (`%to_iv) to_iv;
  bits_ge1 t;
  let w = Rust_primitives.Integers.bits t in
  let bv = from_iv t n m iv in
  let iv_i = Libcrux_core_models.Abstractions.Funarr.impl_5__get m #(Rust_primitives.Integers.int_t t) iv i in
  let u = (v iv_i) % pow2 w in
  assert_norm (pow2 0 == 1);
  dsum2_bound (lane_reader n w bv i) 0 w;
  let aux (b: nat) : Lemma (bval (lane_reader n w bv i b) == (u / pow2 b) % 2) =
    reader_h_gen t n m iv i b
  in
  lemma_dsum2 (lane_reader n w bv i) u 0 w aux;
  FStar.Math.Lemmas.lemma_mod_lt (v iv_i) (pow2 w);
  FStar.Math.Lemmas.small_mod u (pow2 w);
  lemma_tc_of_u_rt t iv_i
#pop-options

#push-options "--fuel 1 --z3rlimit 200"
let lemma_conv_rt
      (t: Rust_primitives.Integers.inttype)
      (n m: u64)
      (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray m (Rust_primitives.Integers.int_t t))
    : Lemma (requires v n == v m * Rust_primitives.Integers.bits t)
      (ensures to_iv t n m (from_iv t n m iv) == iv)
      [SMTPat (to_iv t n m (from_iv t n m iv))] =
  funarr_ext m #(Rust_primitives.Integers.int_t t)
    (to_iv t n m (from_iv t n m iv)) iv (my_lane_gen t n m iv)
#pop-options

(* ---- per-width conversions (thin instantiations) ---- *)
let ${BitVec::<256>::from_i32x8} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) i32) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) iv
let ${BitVec::<256>::to_i32x8} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) i32 =
  to_iv Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) bv
let ${BitVec::<256>::from_i64x4} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) i64) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.I64 (mk_u64 256) (mk_u64 4) iv
let ${BitVec::<256>::to_i64x4} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) i64 =
  to_iv Rust_primitives.Integers.I64 (mk_u64 256) (mk_u64 4) bv
let ${BitVec::<256>::from_i16x16} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) i16) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.I16 (mk_u64 256) (mk_u64 16) iv
let ${BitVec::<256>::to_i16x16} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) i16 =
  to_iv Rust_primitives.Integers.I16 (mk_u64 256) (mk_u64 16) bv
let ${BitVec::<256>::from_i128x2} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) i128) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.I128 (mk_u64 256) (mk_u64 2) iv
let ${BitVec::<256>::to_i128x2} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) i128 =
  to_iv Rust_primitives.Integers.I128 (mk_u64 256) (mk_u64 2) bv
let ${BitVec::<256>::from_i8x32} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 32) i8) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.I8 (mk_u64 256) (mk_u64 32) iv
let ${BitVec::<256>::to_i8x32} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 32) i8 =
  to_iv Rust_primitives.Integers.I8 (mk_u64 256) (mk_u64 32) bv
let ${BitVec::<256>::from_u32x8} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) u32) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.U32 (mk_u64 256) (mk_u64 8) iv
let ${BitVec::<256>::to_u32x8} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) u32 =
  to_iv Rust_primitives.Integers.U32 (mk_u64 256) (mk_u64 8) bv
let ${BitVec::<256>::from_u64x4} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) u64) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.U64 (mk_u64 256) (mk_u64 4) iv
let ${BitVec::<256>::to_u64x4} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) u64 =
  to_iv Rust_primitives.Integers.U64 (mk_u64 256) (mk_u64 4) bv
let ${BitVec::<256>::from_u16x16} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) u16) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.U16 (mk_u64 256) (mk_u64 16) iv
let ${BitVec::<256>::to_u16x16} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) u16 =
  to_iv Rust_primitives.Integers.U16 (mk_u64 256) (mk_u64 16) bv
let ${BitVec::<256>::from_u8x32} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 32) u8) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
  from_iv Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32) iv
let ${BitVec::<256>::to_u8x32} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 32) u8 =
  to_iv Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32) bv
let ${BitVec::<128>::from_i32x4} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) i32) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.I32 (mk_u64 128) (mk_u64 4) iv
let ${BitVec::<128>::to_i32x4} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) i32 =
  to_iv Rust_primitives.Integers.I32 (mk_u64 128) (mk_u64 4) bv
let ${BitVec::<128>::from_i64x2} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) i64) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.I64 (mk_u64 128) (mk_u64 2) iv
let ${BitVec::<128>::to_i64x2} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) i64 =
  to_iv Rust_primitives.Integers.I64 (mk_u64 128) (mk_u64 2) bv
let ${BitVec::<128>::from_i16x8} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) i16) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.I16 (mk_u64 128) (mk_u64 8) iv
let ${BitVec::<128>::to_i16x8} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) i16 =
  to_iv Rust_primitives.Integers.I16 (mk_u64 128) (mk_u64 8) bv
let ${BitVec::<128>::from_i128x1} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 1) i128) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.I128 (mk_u64 128) (mk_u64 1) iv
let ${BitVec::<128>::to_i128x1} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 1) i128 =
  to_iv Rust_primitives.Integers.I128 (mk_u64 128) (mk_u64 1) bv
let ${BitVec::<128>::from_i8x16} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) i8) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.I8 (mk_u64 128) (mk_u64 16) iv
let ${BitVec::<128>::to_i8x16} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) i8 =
  to_iv Rust_primitives.Integers.I8 (mk_u64 128) (mk_u64 16) bv
let ${BitVec::<128>::from_u32x4} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) u32) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.U32 (mk_u64 128) (mk_u64 4) iv
let ${BitVec::<128>::to_u32x4} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) u32 =
  to_iv Rust_primitives.Integers.U32 (mk_u64 128) (mk_u64 4) bv
let ${BitVec::<128>::from_u64x2} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) u64) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.U64 (mk_u64 128) (mk_u64 2) iv
let ${BitVec::<128>::to_u64x2} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) u64 =
  to_iv Rust_primitives.Integers.U64 (mk_u64 128) (mk_u64 2) bv
let ${BitVec::<128>::from_u16x8} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) u16) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.U16 (mk_u64 128) (mk_u64 8) iv
let ${BitVec::<128>::to_u16x8} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) u16 =
  to_iv Rust_primitives.Integers.U16 (mk_u64 128) (mk_u64 8) bv
let ${BitVec::<128>::from_u8x16} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) u8) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128) =
  from_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) iv
let ${BitVec::<128>::to_u8x16} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 16) u8 =
  to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) bv
let ${BitVec::<64>::from_i32x2} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) i32) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.I32 (mk_u64 64) (mk_u64 2) iv
let ${BitVec::<64>::to_i32x2} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) i32 =
  to_iv Rust_primitives.Integers.I32 (mk_u64 64) (mk_u64 2) bv
let ${BitVec::<64>::from_i16x4} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) i16) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.I16 (mk_u64 64) (mk_u64 4) iv
let ${BitVec::<64>::to_i16x4} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) i16 =
  to_iv Rust_primitives.Integers.I16 (mk_u64 64) (mk_u64 4) bv
let ${BitVec::<64>::from_i8x8} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) i8) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.I8 (mk_u64 64) (mk_u64 8) iv
let ${BitVec::<64>::to_i8x8} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) i8 =
  to_iv Rust_primitives.Integers.I8 (mk_u64 64) (mk_u64 8) bv
let ${BitVec::<64>::from_i64x1} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 1) i64) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.I64 (mk_u64 64) (mk_u64 1) iv
let ${BitVec::<64>::to_i64x1} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 1) i64 =
  to_iv Rust_primitives.Integers.I64 (mk_u64 64) (mk_u64 1) bv
let ${BitVec::<64>::from_u32x2} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) u32) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.U32 (mk_u64 64) (mk_u64 2) iv
let ${BitVec::<64>::to_u32x2} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 2) u32 =
  to_iv Rust_primitives.Integers.U32 (mk_u64 64) (mk_u64 2) bv
let ${BitVec::<64>::from_u16x4} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) u16) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.U16 (mk_u64 64) (mk_u64 4) iv
let ${BitVec::<64>::to_u16x4} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 4) u16 =
  to_iv Rust_primitives.Integers.U16 (mk_u64 64) (mk_u64 4) bv
let ${BitVec::<64>::from_u8x8} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) u8) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.U8 (mk_u64 64) (mk_u64 8) iv
let ${BitVec::<64>::to_u8x8} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 8) u8 =
  to_iv Rust_primitives.Integers.U8 (mk_u64 64) (mk_u64 8) bv
let ${BitVec::<64>::from_u64x1} (iv: Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 1) u64) : Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64) =
  from_iv Rust_primitives.Integers.U64 (mk_u64 64) (mk_u64 1) iv
let ${BitVec::<64>::to_u64x1} (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 64)) : Libcrux_core_models.Abstractions.Funarr.t_FunArray (mk_u64 1) u64 =
  to_iv Rust_primitives.Integers.U64 (mk_u64 64) (mk_u64 1) bv
"#
    )]
    const _: () = ();

    impl i64x4 {
        pub fn into_i32x8(self) -> i32x8 {
            i32x8::from_fn(|i| {
                let value = *self.get(i / 2);
                (if i % 2 == 0 { value } else { value >> 32 }) as i32
            })
        }
    }

    impl i32x8 {
        pub fn into_i64x4(self) -> i64x4 {
            i64x4::from_fn(|i| {
                let low = *self.get(2 * i) as u32 as u64;
                let high = *self.get(2 * i + 1) as i32 as i64;
                (high << 32) | low as i64
            })
        }
    }

    impl From<i64x4> for i32x8 {
        fn from(vec: i64x4) -> Self {
            vec.into_i32x8()
        }
    }

    /// Lemma stating that converting an `i64x4` vector to a `BitVec<256>` and then into an `i32x8`
    /// yields the same result as directly converting the `i64x4` into an `i32x8`.
    #[hax_lib::fstar::before("[@@ $SIMPLIFICATION_LEMMA ]")]
    #[hax_lib::opaque]
    #[hax_lib::lemma]
    pub fn lemma_rewrite_i64x4_bv_i32x8(
        bv: i64x4,
    ) -> Proof<{ hax_lib::eq(BitVec::to_i32x8(BitVec::from_i64x4(bv)), bv.into_i32x8()) }> {
    }

    /// Lemma stating that converting an `i64x4` vector to a `BitVec<256>` and then into an `i32x8`
    /// yields the same result as directly converting the `i64x4` into an `i32x8`.
    #[hax_lib::fstar::before("[@@ $SIMPLIFICATION_LEMMA ]")]
    #[hax_lib::opaque]
    #[hax_lib::lemma]
    pub fn lemma_rewrite_i32x8_bv_i64x4(
        bv: i32x8,
    ) -> Proof<{ hax_lib::eq(BitVec::to_i64x4(BitVec::from_i32x8(bv)), bv.into_i64x4()) }> {
    }

    /// Normalize `from` calls that convert from one type to itself
    #[libcrux_macros::trusted(replace, "hax-limitation: F*-native SIMPLIFICATION_LEMMA rewrite (proof machinery)")]
    #[hax_lib::fstar::replace(
        r#"
        [@@ $SIMPLIFICATION_LEMMA ]
        let lemma (t: Type) (i: Core.Convert.t_From t t) (x: t)
            : Lemma (Core.Convert.f_from #t #t #i x == (norm [primops; iota; delta; zeta] i.f_from) x)
            = ()
    "#
    )]
    const _: () = ();

    #[cfg(test)]
    mod direct_convertions_tests {
        use super::*;
        use crate::helpers::test::HasRandom;

        #[test]
        fn into_i32x8() {
            for _ in 0..10000 {
                let x: i64x4 = i64x4::random();
                let y = x.into_i32x8();
                assert_eq!(BitVec::from_i64x4(x), BitVec::from_i32x8(y));
            }
        }
        #[test]
        fn into_i64x4() {
            let x: i32x8 = i32x8::random();
            let y = x.into_i64x4();
            assert_eq!(BitVec::from_i32x8(x), BitVec::from_i64x4(y));
        }
    }
}
