//! Trusted-base proof utilities for libcrux-ml-dsa F* verification.
//!
//! Staging area for facts about MODELED PRIMITIVES that are true but not (yet)
//! provable within ml-dsa's F* dependency closure. These are `assume val`s —
//! part of the trusted base, like the intrinsic models themselves — collected
//! here (rather than duplicated at the use sites) and tagged with the place they
//! should eventually be upstreamed to and discharged.
//!
//! UPSTREAM TARGETS:
//!  * `lemma_movemask_ps_bound` -> core-models. Give the abstract
//!    `Libcrux_core_models.Core_arch.X86.Avx.e_mm256_movemask_ps'` val the
//!    refinement `r: i32{v r >= 0 /\ v r < 256}` (it is an 8-lane sign-bit mask;
//!    justified by `Int_vec.Lemmas`, where movemask == sum of `2^i` over the set
//!    lanes). It is not provable here because `Int_vec.Lemmas` (which pulls in
//!    `Tactics.Circuits`) is outside ml-dsa's F* dependency closure.
//!  * `lemma_count_ones_nibble` -> hax-lib `Rust_primitives.Arithmetic`.
//!    Strengthen `count_ones_i32`'s spec (or add a general `count_ones_lt_pow2`):
//!    `v x < pow2 n ==> v (count_ones x) <= n`. The current spec only bounds the
//!    result by `<= 32`, with no relationship to the value.

// The lemmas are emitted as standalone F* `assume val`s into
// `Libcrux_ml_dsa.Proof_utils`; the marker below just gives hax an item to hang
// the module on (the whole module is `#[cfg(hax)]`, so it has no runtime form).
#[hax_lib::fstar::before(
    r#"
assume
val lemma_movemask_ps_bound (a: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256))
    : Lemma
      (ensures
        v (Libcrux_intrinsics.Avx2.mm256_movemask_ps a) >= 0 /\
        v (Libcrux_intrinsics.Avx2.mm256_movemask_ps a) < 256)

assume
val lemma_count_ones_nibble (x: i32)
    : Lemma (requires v x >= 0 /\ v x < 16)
      (ensures v (Core_models.Num.impl_i32__count_ones x) <= 4)
"#
)]
pub(crate) fn proof_utils_module_marker() -> bool {
    true
}
