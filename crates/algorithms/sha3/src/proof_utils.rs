use hax_lib::{forall, implies, Prop};
#[cfg(hax)]
use hax_lib::prop::*;

/// Checks if all slices in an array have the same length.
pub(crate) fn slices_same_len<const N: usize>(slices: &[&[u8]; N]) -> Prop {
    forall(|i: usize| implies(i < N, slices[0].len() == slices[i].len()))
}

/// `modifies_range a fa lo hi`: `fa` has the same length as `a` and is
/// equal to `a` at every index outside the half-open range `[lo, hi)`.
/// Opaque so that composition proofs manipulate it via the frame
/// lemmas (`lemma_modifies_range_union`) without unfolding the byte
/// content — the central trick for the SHA-3 store-block composition.
#[cfg(hax)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
pub(crate) fn modifies_range(a: &[u8], fa: &[u8], lo: usize, hi: usize) -> Prop {
    forall(|k: usize| implies(k < a.len() && k < fa.len() && (k < lo || k >= hi), a[k] == fa[k]))
        & (a.len() == fa.len()).to_prop()
}

/// Frame composition: a modification of `[lo,mid)` followed by a
/// modification of `[mid,hi)` is a modification of `[lo,hi)`. The only
/// lemma the outer loop / composer needs about `modifies_range`; both
/// `modifies_range` occurrences are revealed here so callers never do.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_modifies_range_union (a b c: t_Slice u8) (lo mid hi: usize)
  : Lemma
    (requires
      modifies_range a b lo mid /\ modifies_range b c mid hi /\
      v lo <= v mid /\ v mid <= v hi)
    (ensures modifies_range a c lo hi)
  = reveal_opaque (`%modifies_range) modifies_range
"#
)]
pub(crate) fn lemma_modifies_range_union(
    _a: &[u8],
    _b: &[u8],
    _c: &[u8],
    _lo: usize,
    _mid: usize,
    _hi: usize,
) {
}

/// Reflexivity: a sequence trivially modifies itself on any range
/// (it equals itself everywhere). Used to seed loop-invariant base
/// cases. Reveals `modifies_range`.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_modifies_range_refl (a: t_Slice u8) (lo hi: usize)
  : Lemma (ensures modifies_range a a lo hi)
  = reveal_opaque (`%modifies_range) modifies_range
"#
)]
pub(crate) fn lemma_modifies_range_refl(_a: &[u8], _lo: usize, _hi: usize) {}

/// Introduce `modifies_range` over an arbitrary range from the explicit
/// frame fact (equal length + equal outside `[lo,hi)`). Reveals
/// `modifies_range`; lets leaf producers package their frame without
/// revealing in their own body.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_modifies_range_intro (a b: t_Slice u8) (lo hi: usize)
  : Lemma
    (requires
      Seq.length a == Seq.length b /\
      (forall (k: nat). (k < Seq.length a /\ (k < v lo \/ k >= v hi)) ==>
        Seq.index b k == Seq.index a k))
    (ensures modifies_range a b lo hi)
  = reveal_opaque (`%modifies_range) modifies_range
"#
)]
pub(crate) fn lemma_modifies_range_intro(_a: &[u8], _b: &[u8], _lo: usize, _hi: usize) {}

pub(crate) fn valid_rate(rate: usize) -> bool {
    // This is could be changed to checking against the specific valid rates
    // corresponding to: SHA3-512, SHA3-384, SHA3-256/SHAKE256, SHA3-224, SHAKE128
    // rate == 72 || rate == 104 || rate == 136 || rate == 144 || rate == 168
    rate > 32 && rate < 200 && rate % 8 == 0 && (rate % 32 == 8 || rate % 32 == 16)
}

/// XOF state invariant: validates that buffer length and rate are valid.
pub(crate) fn keccak_xof_state_inv(rate: usize, buf_len: usize) -> bool {
    valid_rate(rate) && buf_len <= rate
}

pub(crate) use lemmas::{lemma_div_mul_mod, lemma_mul_succ_le, lemma_shl_xor_shr_is_rotate_left};

mod lemmas {
    //! F* verification lemmas for SHA3/Keccak implementation.
    //!
    //! This module contains lemmas used for F* verification across
    //! the libcrux-sha3 crate. These lemmas are only used during
    //! formal verification with hax and F*, and have no runtime behavior.

    /// Lemma proving the division-multiplication-modulo identity.
    ///
    /// For any `a`, `b` with `b != 0`,
    /// proves that `(a / b) * b + (a % b) = a`.
    ///
    /// This is used in the `squeeze` function to verify correct buffer indexing
    /// when squeezing multiple blocks.
    #[hax_lib::fstar::replace(
        r#"
let lemma_div_mul_mod (a b: usize)
    : Lemma
        (requires b <>. mk_usize 0)
        (ensures (a /! b) *! b +! (a %! b) =. a)
    = ()
"#
    )]
    pub(crate) fn lemma_div_mul_mod(_a: usize, _b: usize) {}

    /// Lemma proving multiplication bounds for successive elements.
    ///
    /// For any `k < n`,
    /// proves that `k * d + d ≤ n * d`.
    ///
    /// This is used in the `keccak` functions to verify that block iterations
    /// stay within bounds.
    #[hax_lib::fstar::replace(
        r#"
let rec lemma_mul_succ_le (k n d: usize)
  : Lemma
    (requires (v k) < (v n))
    (ensures (v k) * (v d) + (v d) <= (v n) * (v d))
    (decreases (v n)) =
  if v n = 0 then ()
  else if v k = v n - 1 then ()
  else lemma_mul_succ_le k (n -! mk_usize 1) d
"#
    )]
    pub(crate) fn lemma_mul_succ_le(_k: usize, _n: usize, _d: usize) {}

    /// Bridge lemma: `(x <<! LEFT) ^. (x >>! RIGHT) ≡ rotate_left x LEFT`
    /// when `LEFT + RIGHT == 64` and `0 < RIGHT < 64`.
    ///
    /// Used by `EquivImplSpec.Keccakf.Avx2.avx2_lc_rotate_left1_and_xor`
    /// (and friends) to bridge per-lane shift+xor SMTPats to the
    /// `Core_models.Num.impl_u64__rotate_left` form used in the spec.
    ///
    /// As of cryspen/hax integer-lemmas branch,
    /// `Core_models.Num.impl_u64__rotate_left` is concretely defined
    /// as a delegation to `Rust_primitives.Integers.rotate_left_u`
    /// — `(x <<! n) ^. (x >>! (64 - n))` — making this lemma a real
    /// proof via per-bit reasoning + bit-extensionality.
    #[hax_lib::fstar::replace(
        r#"
let lemma_shl_xor_shr_is_rotate_left (x: u64) (v_LEFT v_RIGHT: i32)
  : Lemma
      (requires
        v v_LEFT >= 0 /\ v v_LEFT < 64 /\
        v v_RIGHT > 0 /\ v v_RIGHT < 64 /\
        v v_LEFT + v v_RIGHT == 64)
      (ensures
        ((x <<! v_LEFT) ^. (x >>! v_RIGHT)) ==
        Core_models.Num.impl_u64__rotate_left x (cast (v_LEFT <: i32) <: u32))
  = let lhs = (x <<! v_LEFT) ^. (x >>! v_RIGHT) in
    let n: u32 = cast (v_LEFT <: i32) in
    let rhs = Core_models.Num.impl_u64__rotate_left x n in
    let aux (i: usize {Rust_primitives.Integers.v i < 64})
      : Lemma (Rust_primitives.Integers.get_bit lhs i ==
               Rust_primitives.Integers.get_bit rhs i) =
      Rust_primitives.Integers.lemma_rotate_left_u_get_bit
        #Rust_primitives.Integers.u64_inttype x n i
    in
    FStar.Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits lhs rhs
"#
    )]
    pub(crate) fn lemma_shl_xor_shr_is_rotate_left(_x: u64, _left: i32, _right: i32) {}
}
