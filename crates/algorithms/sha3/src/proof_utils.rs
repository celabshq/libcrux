use hax_lib::{forall, implies, Prop};

/// Checks if all slices in an array have the same length.
pub(crate) fn slices_same_len<const N: usize>(slices: &[&[u8]; N]) -> Prop {
    forall(|i: usize| implies(i < N, slices[0].len() == slices[i].len()))
}

pub(crate) fn valid_rate(rate: usize) -> bool {
    // This is could be changed to checking against the specific valid rates
    // corresponding to: SHA3-512, SHA3-384, SHA3-256/SHAKE256, SHA3-224, SHAKE128
    // rate == 72 || rate == 104 || rate == 136 || rate == 144 || rate == 168
    rate != 0 && rate <= 200 && rate % 8 == 0 && (rate % 32 == 8 || rate % 32 == 16)
}

/// XOF state invariant: validates that buffer length and rate are valid.
pub(crate) fn keccak_xof_state_inv(rate: usize, buf_len: usize) -> bool {
    valid_rate(rate) && buf_len <= rate
}

pub(crate) use lemmas::{lemma_div_mul_mod, lemma_mul_succ_le};

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
}
