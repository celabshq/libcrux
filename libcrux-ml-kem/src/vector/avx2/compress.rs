use crate::vector::FIELD_MODULUS;

use super::*;

// Multiply the 32-bit numbers contained in |lhs| and |rhs|, and store only
// the upper 32 bits of the resulting product.
// This implementation was taken from:
// https://ei1333.github.io/library/math/combinatorics/vectorize-mod-int.hpp.html
//
// TODO: Optimize this implementation if performance numbers suggest doing so.
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views
open Libcrux_ml_kem.Vector.Avx2.Compress_theory
"#
)]
fn mulhi_mm256_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    let prod02 = mm256_mul_epu32(lhs, rhs);
    let prod13 = mm256_mul_epu32(
        mm256_shuffle_epi32::<0b11_11_01_01>(lhs),
        mm256_shuffle_epi32::<0b11_11_01_01>(rhs),
    );

    mm256_unpackhi_epi64(
        mm256_unpacklo_epi32(prod02, prod13),
        mm256_unpackhi_epi32(prod02, prod13),
    )
}

// ─────────────────────────────────────────────────────────────────────────
// Generic SIMD lane lemmas for compress_message_coefficient.  These bridge
// bit-level Vec256 ops to their per-lane i16 views.  Closed via
// per-lane axioms `lemma_mm256_xor_si256` and `lemma_mm256_srli_epi16`
// added to `Libcrux_intrinsics.Avx2_extract` (siblings of
// `lemma_mm256_and_si256`).
// ─────────────────────────────────────────────────────────────────────────
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall (i: nat). i < 16 ==>
    v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) >= 0 /\
    v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) < 3329"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
     let res_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) i) in
     res_i >= 0 /\ res_i < 2 /\
     res_i == ((vec_i * 4 + 3329) / 6658) % 2)"#))]
pub(crate) fn compress_message_coefficient(vector: Vec256) -> Vec256 {
    let field_modulus_halved = mm256_set1_epi16((FIELD_MODULUS - 1) / 2);
    let field_modulus_quartered = mm256_set1_epi16((FIELD_MODULUS - 1) / 4);

    let shifted = mm256_sub_epi16(field_modulus_halved, vector);
    proof!(
        r#"assert (forall (i: nat). i < 16 ==>
            v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted}) i) ==
            1664 - v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i))"#
    );

    let mask = mm256_srai_epi16::<15>(shifted);
    proof!(
        r#"assert (forall (i: nat). i < 16 ==>
            v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${mask}) i) ==
            (if v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted}) i) < 0
             then -1 else 0))"#
    );

    let shifted_to_positive = mm256_xor_si256(mask, shifted);
    proof!(
        r#"lemma_mm256_xor_si256_lane ${mask} ${shifted};
           introduce forall (i: nat). i < 16 ==>
             v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted_to_positive}) i) ==
             (let s = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted}) i) in
              if s < 0 then -s - 1 else s)
           with begin
             if i < 16 then
               lemma_xor_cond_not (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${mask}) i)
                                  (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted}) i)
           end"#
    );

    let shifted_to_positive_in_range =
        mm256_sub_epi16(shifted_to_positive, field_modulus_quartered);
    proof!(
        r#"introduce forall (i: nat). i < 16 ==>
             v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted_to_positive_in_range}) i) ==
             (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
              if vec_i <= 1664 then 832 - vec_i else vec_i - 2497)
           with begin
             if i < 16 then begin
               let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
               let s = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted}) i) in
               assert (s == 1664 - vec_i);
               assert (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${shifted_to_positive}) i) ==
                       (if s < 0 then -s - 1 else s));
               assert (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${field_modulus_quartered}) i) == 832)
             end
           end"#
    );

    let result = mm256_srli_epi16::<15>(shifted_to_positive_in_range);
    proof!(
        r#"lemma_mm256_srli_epi16_15 ${shifted_to_positive_in_range};
           assert (forall (i: nat). i < 16 ==>
             v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) i) ==
             (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
              if 833 <= vec_i && vec_i <= 2496 then 1 else 0));
           introduce forall (i: nat). i < 16 ==>
             (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
              ((vec_i * 4 + 3329) / 6658) % 2 ==
              (if 833 <= vec_i && vec_i <= 2496 then 1 else 0))
           with begin
             if i < 16 then
               lemma_compress_message_identity
                 (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i))
           end"#
    );
    result
}

#[inline(always)]
// The mulhi_l_* / lemma_mulhi_mm256_epi32 / lemma_compress_half cluster cites this
// module's own mulhi_mm256_epi32 fn; a companion would create an F* module cycle
// (Error 308, documented in the Vector.Avx2.Compress_theory companion header).
// proof-residence: locked(module-cycle) — cites own mulhi_mm256_epi32 fn
#[hax_lib::fstar::before(
    r#"
(* One single-lane lemma per result lane (no in-body match — the 4/8-way match
   combines all branch contexts into one VC and saturates).  Each is the proven
   `test_lane0` shape: cite the ground per-lane facts, exclude the quantified
   intrinsic posts.  ~ms each. *)
unfold let mulhi_l_pre (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) : prop =
  0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs j /\ 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs j /\
  (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs j * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs j) / 4294967296 < 2147483648
unfold let mulhi_l_post (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) : prop =
  Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (mulhi_mm256_epi32 lhs rhs) j == (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs j * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs j) / 4294967296

#push-options "--fuel 1 --ifuel 1 --z3rlimit 50 --using_facts_from '* -Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 -Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 -Libcrux_intrinsics.Avx2.mm256_unpackhi_epi64 -Libcrux_intrinsics.Avx2.mm256_mul_epu32 -Libcrux_intrinsics.Avx2.mm256_shuffle_epi32'"
let lemma_mulhi_l0 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 0) (ensures mulhi_l_post lhs rhs 0)
  = let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in
    let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 0; lemma_mulhi_hi32 prod02 0 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 0 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 0);
    unpacklo_lane prod02 prod13 2; unpackhi64_lane ulo uhi 0
let lemma_mulhi_l2 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 2) (ensures mulhi_l_post lhs rhs 2)
  = let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in
    let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 1; lemma_mulhi_hi32 prod02 1 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 2 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 2);
    unpackhi_lane prod02 prod13 2; unpackhi64_lane ulo uhi 2
let lemma_mulhi_l4 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 4) (ensures mulhi_l_post lhs rhs 4)
  = let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in
    let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 2; lemma_mulhi_hi32 prod02 2 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 4 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 4);
    unpacklo_lane prod02 prod13 6; unpackhi64_lane ulo uhi 4
let lemma_mulhi_l6 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 6) (ensures mulhi_l_post lhs rhs 6)
  = let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in
    let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 3; lemma_mulhi_hi32 prod02 3 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 6 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 6);
    unpackhi_lane prod02 prod13 6; unpackhi64_lane ulo uhi 6
let lemma_mulhi_l1 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 1) (ensures mulhi_l_post lhs rhs 1)
  = let shl = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 shl shr in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 0; lemma_shuffle245_even rhs 0;
    mul_epu32_lane_nn shl shr 0; lemma_mulhi_hi32 prod13 0 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 1 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 1);
    unpacklo_lane prod02 prod13 3; unpackhi64_lane ulo uhi 1
let lemma_mulhi_l3 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 3) (ensures mulhi_l_post lhs rhs 3)
  = let shl = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 shl shr in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 1; lemma_shuffle245_even rhs 1;
    mul_epu32_lane_nn shl shr 1; lemma_mulhi_hi32 prod13 1 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 3 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 3);
    unpackhi_lane prod02 prod13 3; unpackhi64_lane ulo uhi 3
let lemma_mulhi_l5 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 5) (ensures mulhi_l_post lhs rhs 5)
  = let shl = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 shl shr in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 2; lemma_shuffle245_even rhs 2;
    mul_epu32_lane_nn shl shr 2; lemma_mulhi_hi32 prod13 2 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 5 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 5);
    unpacklo_lane prod02 prod13 7; unpackhi64_lane ulo uhi 5
let lemma_mulhi_l7 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 7) (ensures mulhi_l_post lhs rhs 7)
  = let shl = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 lhs rhs in let prod13 = Libcrux_intrinsics.Avx2.mm256_mul_epu32 shl shr in
    let ulo = Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 3; lemma_shuffle245_even rhs 3;
    mul_epu32_lane_nn shl shr 3; lemma_mulhi_hi32 prod13 3 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 lhs 7 * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 rhs 7);
    unpackhi_lane prod02 prod13 7; unpackhi64_lane ulo uhi 7
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_mulhi_mm256_epi32 (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (requires mulhi_l_pre lhs rhs j) (ensures mulhi_l_post lhs rhs j)
  = match j with
    | 0 -> lemma_mulhi_l0 lhs rhs | 1 -> lemma_mulhi_l1 lhs rhs
    | 2 -> lemma_mulhi_l2 lhs rhs | 3 -> lemma_mulhi_l3 lhs rhs
    | 4 -> lemma_mulhi_l4 lhs rhs | 5 -> lemma_mulhi_l5 lhs rhs
    | 6 -> lemma_mulhi_l6 lhs rhs | _ -> lemma_mulhi_l7 lhs rhs
#pop-options

(* Exclude lane32's DEFINITION too: keep it an atomic uninterpreted term so the
   products `lane32 c3 j * lane32 cf j` etc. don't unfold into get_lane arithmetic
   (which is what saturates Z3 nonlinearly).  All lane32 equalities the chain needs
   are supplied as facts by the per-stage lemmas + the clean bounds helper. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lane32 -Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 -Libcrux_intrinsics.Avx2.mm256_slli_epi32 -Libcrux_intrinsics.Avx2.mm256_add_epi32 -Libcrux_intrinsics.Avx2.mm256_srli_epi32 -Libcrux_intrinsics.Avx2.mm256_mul_epu32 -Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 -Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 -Libcrux_intrinsics.Avx2.mm256_unpackhi_epi64 -Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 -Libcrux_intrinsics.Avx2.mm256_set1_epi32'"
let lemma_compress_half (c0: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) (cb: i32) (fmh cf mask: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma
    (requires
       (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
       0 <= v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) < 3329 /\
       Libcrux_intrinsics.Avx2_ml_kem_views.lane32 fmh j == 1664 /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 cf j == 10321340 /\
       Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mask (2 * j) == mk_i16 (pow2 (v cb) - 1) /\
       Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mask (2 * j + 1) == mk_i16 0)
    (ensures
       (let c1 = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 c0 in
        let c2 = Libcrux_intrinsics.Avx2.mm256_slli_epi32 cb c1 in
        let c3 = Libcrux_intrinsics.Avx2.mm256_add_epi32 c2 fmh in
        let c4 = mulhi_mm256_epi32 c3 cf in
        let c5 = Libcrux_intrinsics.Avx2.mm256_srli_epi32 (mk_i32 3) c4 in
        let c6 = Libcrux_intrinsics.Avx2.mm256_and_si256 c5 mask in
        let xv = v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) in
        let dd = v cb in
        0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c6 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c6 j < pow2 dd /\
        Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c6 j == (((xv * pow2 dd + 1664) * 10321340) / pow2 35) % pow2 dd))
  = let dd = v cb in
    let xv = v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) in
    let c1 = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 c0 in
    let c2 = Libcrux_intrinsics.Avx2.mm256_slli_epi32 cb c1 in
    let c3 = Libcrux_intrinsics.Avx2.mm256_add_epi32 c2 fmh in
    let c4 = mulhi_mm256_epi32 c3 cf in
    let c5 = Libcrux_intrinsics.Avx2.mm256_srli_epi32 (mk_i32 3) c4 in
    let nn = xv * pow2 dd + 1664 in
    assert_norm (pow2 32 == 4294967296); assert_norm (pow2 35 == 34359738368);
    lemma_compress_nn_bounds xv dd;     (* ground: nn<=6817408, nn*K/2^32<=32767, nn*K/2^35<=4095 *)
    cvtepi_lane_nn c0 j;                 (* lane32 c1 j == xv *)
    slli_lane_nowrap c1 cb j;            (* lane32 c2 j == xv*2^dd *)
    add_lane_1664 c2 fmh j;             (* lane32 c3 j == nn *)
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c3 j == nn);
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c3 j * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 cf j == nn * 10321340);
    lemma_mulhi_mm256_epi32 c3 cf j;    (* lane32 c4 j == (nn*10321340)/2^32 *)
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c4 j == (nn * 10321340) / pow2 32);
    srli3_lane c4 j;                    (* lane32 c5 j == ((nn*10321340)/2^32)/8 *)
    FStar.Math.Lemmas.division_multiplication_lemma (nn * 10321340) 4294967296 8;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c5 j == (nn * 10321340) / pow2 35);
    lemma_and_mask_lane c5 mask j ((nn * 10321340) / pow2 35) dd
#pop-options
"#
)]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lane32 -Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 -Libcrux_intrinsics.Avx2.mm256_slli_epi32 -Libcrux_intrinsics.Avx2.mm256_add_epi32 -Libcrux_intrinsics.Avx2.mm256_srli_epi32 -Libcrux_intrinsics.Avx2.mm256_mul_epu32 -Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 -Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 -Libcrux_intrinsics.Avx2.mm256_unpackhi_epi64 -Libcrux_intrinsics.Avx2.mm256_shuffle_epi32'")]
#[hax_lib::requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/ v $COEFFICIENT_BITS == 5 \/
    v $COEFFICIENT_BITS == 10 \/ v $COEFFICIENT_BITS == 11) /\
    range (v ((mk_i32 1) <<! $COEFFICIENT_BITS) - 1) i32_inttype /\
    (forall (i: nat). i < 16 ==>
      0 <= v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) /\
      v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) < 3329)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let ri = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) i) in
     let vi = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
     0 <= ri /\ ri < pow2 (v $COEFFICIENT_BITS) /\
     ri == (((vi * pow2 (v $COEFFICIENT_BITS) + 1664) * 10321340) / pow2 35)
           % pow2 (v $COEFFICIENT_BITS))"#))]
pub(crate) fn compress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    vector: Vec256,
) -> Vec256 {
    let field_modulus_halved = mm256_set1_epi32(((FIELD_MODULUS as i32) - 1) / 2);
    let compression_factor = mm256_set1_epi32(10_321_340);
    let coefficient_bits_mask = mm256_set1_epi32((1 << COEFFICIENT_BITS) - 1);

    // ---- Compress the first 8 coefficients ----

    // Take the bottom 128 bits, i.e. the first 8 16-bit coefficients
    let coefficients_low = mm256_castsi256_si128(vector);

    // If:
    //
    // coefficients_low[0:15] = A
    // coefficients_low[16:31] = B
    // coefficients_low[32:63] = C
    // and so on ...
    //
    // after this step:
    //
    // coefficients_low[0:31] = A
    // coefficients_low[32:63] = B
    // and so on ...
    let coefficients_low = mm256_cvtepi16_epi32(coefficients_low);

    let compressed_low = mm256_slli_epi32::<{ COEFFICIENT_BITS }>(coefficients_low);
    let compressed_low = mm256_add_epi32(compressed_low, field_modulus_halved);

    let compressed_low = mulhi_mm256_epi32(compressed_low, compression_factor);

    // Due to the mulhi_mm256_epi32 we've already shifted right by 32 bits, we
    // just need to shift right by 35 - 32 = 3 more.
    let compressed_low = mm256_srli_epi32::<3>(compressed_low);

    let compressed_low = mm256_and_si256(compressed_low, coefficient_bits_mask);

    // ---- Compress the next 8 coefficients ----

    // Take the upper 128 bits, i.e. the next 8 16-bit coefficients
    let coefficients_high = mm256_extracti128_si256::<1>(vector);
    let coefficients_high = mm256_cvtepi16_epi32(coefficients_high);

    let compressed_high = mm256_slli_epi32::<{ COEFFICIENT_BITS }>(coefficients_high);
    let compressed_high = mm256_add_epi32(compressed_high, field_modulus_halved);

    let compressed_high = mulhi_mm256_epi32(compressed_high, compression_factor);
    let compressed_high = mm256_srli_epi32::<3>(compressed_high);
    let compressed_high = mm256_and_si256(compressed_high, coefficient_bits_mask);

    // Combining them, and grouping each set of 64-bits, this function results
    // in:
    //
    // 0: low low low low | 1: high high high high | 2: low low low low | 3: high high high high
    //
    // where each |low| and |high| is a 16-bit element
    let compressed = mm256_packs_epi32(compressed_low, compressed_high);

    // To be in the right order, we need to move the |low|s above in position 2 to
    // position 1 and the |high|s in position 1 to position 2, and leave the
    // rest unchanged.
    let result = mm256_permute4x64_epi64::<0b11_01_10_00>(compressed);
    proof!(
        r#"
  let dd = v v_COEFFICIENT_BITS in
  assert_norm (pow2 11 == 2048);
  FStar.Math.Lemmas.pow2_le_compat 11 dd;
  compress_castsi256_lemma vector;
  compress_extracti128_lemma vector;
  let aux_low (j: nat{j < 8})
    : Lemma (0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_low j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_low j < pow2 dd /\
             Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_low j ==
             (((v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vector) j) * pow2 dd + 1664) * 10321340) / pow2 35)
             % pow2 dd) =
    set1_lane32 (((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1) /!
                 mk_i32 2) j;
    set1_lane32 (mk_i32 10321340) j;
    assert_norm (v ((((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1)
                     <: i32) /! mk_i32 2) == 1664);
    lemma_mask_val v_COEFFICIENT_BITS;
    set1_mask_i16 ((mk_i32 1 <<! v_COEFFICIENT_BITS <: i32) -! mk_i32 1) dd j;
    lemma_compress_half (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vector) v_COEFFICIENT_BITS field_modulus_halved
      compression_factor coefficient_bits_mask j
  in
  Classical.forall_intro aux_low;
  let aux_high (j: nat{j < 8})
    : Lemma (0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_high j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_high j < pow2 dd /\
             Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_high j ==
             (((v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vector) (j + 8)) * pow2 dd + 1664) * 10321340)
              / pow2 35) % pow2 dd) =
    set1_lane32 (((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1) /!
                 mk_i32 2) j;
    set1_lane32 (mk_i32 10321340) j;
    assert_norm (v ((((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1)
                     <: i32) /! mk_i32 2) == 1664);
    lemma_mask_val v_COEFFICIENT_BITS;
    set1_mask_i16 ((mk_i32 1 <<! v_COEFFICIENT_BITS <: i32) -! mk_i32 1) dd j;
    lemma_compress_half (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vector) v_COEFFICIENT_BITS
      field_modulus_halved compression_factor coefficient_bits_mask j
  in
  Classical.forall_intro aux_high;
  assert (forall (k: nat). k < 8 ==>
            0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_low k /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_low k < 32768 /\
            0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_high k /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_high k < 32768);
  let aux_res (i: nat{i < 16})
    : Lemma (let ri = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) i) in
             let vi = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vector) i) in
             0 <= ri /\ ri < pow2 dd /\
             ri == (((vi * pow2 dd + 1664) * 10321340) / pow2 35) % pow2 dd) =
    lemma_result_lane compressed_low compressed_high i;
    if i < 8 then
      assert (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) i) == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_low i)
    else
      assert (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) i) == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 compressed_high (i - 8))
  in
  Classical.forall_intro aux_res
"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"forall i. let x = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $a) i in
                                      (x == mk_i16 0 \/ x == mk_i16 1)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let res_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $result) i) in
     let a_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $a) i) in
     (res_i == 0 \/ res_i == 1665) /\ res_i == (2 * a_i * 3329 + 2) / 4)"#))]
pub fn decompress_1(a: Vec256) -> Vec256 {
    let z = mm256_setzero_si256();

    proof!(
        r#"
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_setzero_si256 ();
        assert(Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $z == Seq.create 16 (mk_i16 0));
        assert(forall i. Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $z) i == mk_i16 0);
        assert(forall i. let x = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $a) i in 
                                      ((0 - v x) == 0 \/ (0 - v x) == -1));
        assert(forall i. i < 16 ==>
                        Spec.Utils.is_intb (pow2 15 - 1) 
                        (0 - v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $a) i)))
    "#
    );

    let s = arithmetic::sub(z, a);

    proof!(
        r#"assert(forall i. Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $s) i == mk_i16 0 \/ 
                            Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $s) i == mk_i16 (-1))"#
    );

    let result = arithmetic::bitwise_and_with_constant(s, 1665);

    proof!(
        r#"Rust_primitives.Integers.logand_lemma (mk_i16 1665) (mk_i16 1665);
           introduce forall (i: nat). i < 16 ==>
             (let res_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $result) i) in
              let a_i = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $a) i) in
              (res_i == 0 \/ res_i == 1665) /\ res_i == (2 * a_i * 3329 + 2) / 4)
           with begin
             if i < 16 then begin
               let si = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $s) i in
               let ai = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $a) i in
               assert (v si == - v ai);
               assert (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 $result) i ==
                       (si &. mk_i16 1665))
             end
           end"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lane32 -Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 -Libcrux_intrinsics.Avx2.mm256_mullo_epi32 -Libcrux_intrinsics.Avx2.mm256_slli_epi32 -Libcrux_intrinsics.Avx2.mm256_add_epi32 -Libcrux_intrinsics.Avx2.mm256_srli_epi32 -Libcrux_intrinsics.Avx2.mm256_set1_epi32'")]
#[hax_lib::requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/ v $COEFFICIENT_BITS == 5 \/
    v $COEFFICIENT_BITS == 10 \/ v $COEFFICIENT_BITS == 11) /\
    (forall (i: nat). i < 16 ==>
      0 <= v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) /\
      v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) <
      pow2 (v $COEFFICIENT_BITS))"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let ri = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) i) in
     let vi = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) in
     0 <= ri /\ ri < 3329 /\
     ri == (2 * vi * 3329 + pow2 (v $COEFFICIENT_BITS)) / (pow2 (v $COEFFICIENT_BITS) * 2))"#))]
pub(crate) fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    vector: Vec256,
) -> Vec256 {
    let field_modulus = mm256_set1_epi32(FIELD_MODULUS as i32);
    let two_pow_coefficient_bits = mm256_set1_epi32(1 << COEFFICIENT_BITS);

    // ---- Compress the first 8 coefficients ----
    let coefficients_low = mm256_castsi256_si128(vector);
    let coefficients_low = mm256_cvtepi16_epi32(coefficients_low);

    let decompressed_low = mm256_mullo_epi32(coefficients_low, field_modulus);
    let decompressed_low = mm256_slli_epi32::<1>(decompressed_low);
    let decompressed_low = mm256_add_epi32(decompressed_low, two_pow_coefficient_bits);

    // We can't shift in one go by (COEFFICIENT_BITS + 1) due to the lack
    // of support for const generic expressions.
    let decompressed_low = mm256_srli_epi32::<{ COEFFICIENT_BITS }>(decompressed_low);
    let decompressed_low = mm256_srli_epi32::<1>(decompressed_low);

    // ---- Compress the next 8 coefficients ----
    let coefficients_high = mm256_extracti128_si256::<1>(vector);
    let coefficients_high = mm256_cvtepi16_epi32(coefficients_high);

    let decompressed_high = mm256_mullo_epi32(coefficients_high, field_modulus);
    let decompressed_high = mm256_slli_epi32::<1>(decompressed_high);
    let decompressed_high = mm256_add_epi32(decompressed_high, two_pow_coefficient_bits);

    // We can't shift in one go by (COEFFICIENT_BITS + 1) due to the lack
    // of support for const generic expressions.
    let decompressed_high = mm256_srli_epi32::<{ COEFFICIENT_BITS }>(decompressed_high);
    let decompressed_high = mm256_srli_epi32::<1>(decompressed_high);

    // Combining them, and grouping each set of 64-bits, this function results
    // in:
    //
    // 0: low low low low | 1: high high high high | 2: low low low low | 3: high high high high
    //
    // where each |low| and |high| is a 16-bit element
    let compressed = mm256_packs_epi32(decompressed_low, decompressed_high);

    // To be in the right order, we need to move the |low|s above in position 2 to
    // position 1 and the |high|s in position 1 to position 2, and leave the
    // rest unchanged.
    let result = mm256_permute4x64_epi64::<0b11_01_10_00>(compressed);
    proof!(
        r#"
  let dd = v v_COEFFICIENT_BITS in
  assert_norm (pow2 11 == 2048);
  FStar.Math.Lemmas.pow2_le_compat 11 dd;
  compress_castsi256_lemma vector;
  compress_extracti128_lemma vector;
  let aux_low (j: nat{j < 8})
    : Lemma (0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_low j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_low j < 3329 /\
             Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_low j ==
             (2 * v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vector) j) * 3329 + pow2 dd) / (pow2 dd * 2)) =
    set1_lane32 (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) j;
    set1_lane32 (mk_i32 1 <<! v_COEFFICIENT_BITS <: i32) j;
    assert_norm (v (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) == 3329);
    lemma_twopow_val v_COEFFICIENT_BITS;
    lemma_decompress_half (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vector) v_COEFFICIENT_BITS field_modulus
      two_pow_coefficient_bits j
  in
  Classical.forall_intro aux_low;
  let aux_high (j: nat{j < 8})
    : Lemma (0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_high j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_high j < 3329 /\
             Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_high j ==
             (2 * v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vector) (j + 8)) * 3329 + pow2 dd)
             / (pow2 dd * 2)) =
    set1_lane32 (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) j;
    set1_lane32 (mk_i32 1 <<! v_COEFFICIENT_BITS <: i32) j;
    assert_norm (v (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) == 3329);
    lemma_twopow_val v_COEFFICIENT_BITS;
    lemma_decompress_half (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vector) v_COEFFICIENT_BITS
      field_modulus two_pow_coefficient_bits j
  in
  Classical.forall_intro aux_high;
  assert (forall (k: nat). k < 8 ==>
            0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_low k /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_low k < 32768 /\
            0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_high k /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_high k < 32768);
  let aux_res (i: nat{i < 16})
    : Lemma (let ri = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) i) in
             let vi = v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vector) i) in
             0 <= ri /\ ri < 3329 /\
             ri == (2 * vi * 3329 + pow2 dd) / (pow2 dd * 2)) =
    lemma_result_lane decompressed_low decompressed_high i;
    if i < 8 then
      assert (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) i) == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_low i)
    else
      assert (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) i) == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 decompressed_high (i - 8))
  in
  Classical.forall_intro aux_res
"#
    );
    result
}
