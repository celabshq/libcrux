use crate::vector::FIELD_MODULUS;

use super::*;

// Multiply the 32-bit numbers contained in |lhs| and |rhs|, and store only
// the upper 32 bits of the resulting product.
// This implementation was taken from:
// https://ei1333.github.io/library/math/combinatorics/vectorize-mod-int.hpp.html
//
// TODO: Optimize this implementation if performance numbers suggest doing so.
#[inline(always)]
#[hax_lib::fstar::before(r#"module Iavx = Libcrux_intrinsics.Avx2_extract"#)]
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
#[hax_lib::fstar::before(
    r#"
(* AGENT C2: closed via `lemma_mm256_xor_si256` axiom (sibling of
   `lemma_mm256_and_si256`).  Strengthens the per-lane xor characterization. *)
let lemma_mm256_xor_si256_lane (lhs rhs: Libcrux_intrinsics.Avx2_extract.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 16 ==>
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
                 (Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 lhs rhs)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 lhs) i ^.
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 rhs) i))
  = Libcrux_intrinsics.Avx2_extract.lemma_mm256_xor_si256 lhs rhs

(* AGENT C2: closed via `lemma_mm256_srli_epi16` axiom.  Specialises the
   per-lane logical right-shift characterization to SHIFT = 15 (sign bit
   extraction). *)
let lemma_mm256_srli_epi16_15 (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 16 ==>
    v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
                    (Libcrux_intrinsics.Avx2_extract.mm256_srli_epi16 (mk_i32 15) vec)) i) ==
    (if v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 vec) i) < 0
     then 1 else 0)))
  = Libcrux_intrinsics.Avx2_extract.lemma_mm256_srli_epi16 (mk_i32 15) vec;
    let view = Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 vec in
    let view_shifted = Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
                         (Libcrux_intrinsics.Avx2_extract.mm256_srli_epi16 (mk_i32 15) vec) in
    introduce forall (i: nat). i < 16 ==>
      v (Seq.index view_shifted i) ==
      (if v (Seq.index view i) < 0 then 1 else 0)
    with begin
      if i < 16 then begin
        let x = Seq.index view i in
        if v x < 0 then begin
          assert (v (cast x <: u16) == v x + pow2 16);
          assert (v ((cast x <: u16) >>! mk_i32 15) == (v x + pow2 16) / pow2 15);
          assert ((v x + pow2 16) / pow2 15 == 1)
        end else begin
          assert (v (cast x <: u16) == v x);
          assert (v ((cast x <: u16) >>! mk_i32 15) == v x / pow2 15);
          assert (v x / pow2 15 == 0)
        end
      end
    end

(* >>! 15 on i16 (arithmetic shift) is sign extension: -1 if negative, else 0 *)
let lemma_i16_arith_shr_15 (x: i16) : Lemma
  (ensures v (x >>! mk_i32 15) == (if v x < 0 then -1 else 0))
  [SMTPat (x >>! mk_i32 15)]
  = ()

(* xor of an i16 with all-ones (-1) is bitwise NOT, i.e. (-x - 1).
   xor with all-zeros is identity.  Proved via Rust_primitives.Integers
   logxor_lemma + lognot_lemma (covers a ^ ones == lognot a and
   v (lognot a) == -1 - v a on signed types). *)
let lemma_i16_xor_neg1 (x: i16) : Lemma
  (ensures v (x ^. mk_i16 (-1)) == -(v x) - 1)
  [SMTPat (x ^. mk_i16 (-1))]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 (-1));
    Rust_primitives.Integers.lognot_lemma x

let lemma_i16_xor_zero (x: i16) : Lemma
  (ensures v (x ^. mk_i16 0) == v x)
  [SMTPat (x ^. mk_i16 0)]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 0)

(* P1: per-lane conditional-not.  When the mask m is all-ones (v m = -1) it
   flips x to lognot x = -x-1; when all-zeros (v m = 0) it is identity.
   Mirror of the portable shifted_to_positive xor reasoning. *)
let lemma_xor_cond_not (m x: i16) : Lemma
  (requires v m == (if v x < 0 then -1 else 0))
  (ensures v (m ^. x) == (if v x < 0 then - (v x) - 1 else v x))
  = Rust_primitives.Integers.mk_int_v_lemma m;
    Rust_primitives.Integers.logxor_lemma x m;
    Rust_primitives.Integers.lognot_lemma x

(* P0: the compress-1 integer identity, a mirror of the portable
   compress_message_coefficient final case-split.  Pure arithmetic fact:
   for a field element vec_i in [0,3328], floor((vec_i*4+3329)/6658) is
   0 / 1 / 2 on the three ranges [0,832] / [833,2496] / [2497,3328], whose
   parities are 0 / 1 / 0 = (if 833<=vec_i<=2496 then 1 else 0). *)
#push-options "--z3rlimit 200"
let lemma_compress_message_identity (vec_i: int) : Lemma
  (requires vec_i >= 0 /\ vec_i < 3329)
  (ensures ((vec_i * 4 + 3329) / 6658) % 2 == (if 833 <= vec_i && vec_i <= 2496 then 1 else 0))
  = assert (vec_i < 833 ==> (vec_i * 4 + 3329) >= 3329 /\ (vec_i * 4 + 3329) < 6658);
    assert (vec_i < 833 ==> (vec_i * 4 + 3329) / 6658 == 0);
    assert (vec_i < 833 ==> ((vec_i * 4 + 3329) / 6658) % 2 == 0);
    assert ((vec_i >= 833 && vec_i <= 2496) ==> (vec_i * 4 + 3329) >= 6658 /\ (vec_i * 4 + 3329) < 13316);
    assert ((vec_i >= 833 && vec_i <= 2496) ==> (vec_i * 4 + 3329) / 6658 == 1);
    assert ((vec_i >= 833 && vec_i <= 2496) ==> ((vec_i * 4 + 3329) / 6658) % 2 == 1);
    assert (vec_i > 2496 ==> (vec_i * 4 + 3329) >= 13316 /\ (vec_i * 4 + 3329) < 19974);
    assert (vec_i > 2496 ==> (vec_i * 4 + 3329) / 6658 == 2);
    assert (vec_i > 2496 ==> ((vec_i * 4 + 3329) / 6658) % 2 == 0)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall (i: nat). i < 16 ==>
    v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) >= 0 /\
    v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) < 3329"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) in
     let res_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) i) in
     res_i >= 0 /\ res_i < 2 /\
     res_i == ((vec_i * 4 + 3329) / 6658) % 2)"#))]
pub(crate) fn compress_message_coefficient(vector: Vec256) -> Vec256 {
    let field_modulus_halved = mm256_set1_epi16((FIELD_MODULUS - 1) / 2);
    let field_modulus_quartered = mm256_set1_epi16((FIELD_MODULUS - 1) / 4);

    let shifted = mm256_sub_epi16(field_modulus_halved, vector);
    hax_lib::fstar!(
        r#"assert (forall (i: nat). i < 16 ==>
            v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted}) i) ==
            1664 - v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i))"#
    );

    let mask = mm256_srai_epi16::<15>(shifted);
    hax_lib::fstar!(
        r#"assert (forall (i: nat). i < 16 ==>
            v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${mask}) i) ==
            (if v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted}) i) < 0
             then -1 else 0))"#
    );

    let shifted_to_positive = mm256_xor_si256(mask, shifted);
    hax_lib::fstar!(
        r#"lemma_mm256_xor_si256_lane ${mask} ${shifted};
           introduce forall (i: nat). i < 16 ==>
             v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted_to_positive}) i) ==
             (let s = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted}) i) in
              if s < 0 then -s - 1 else s)
           with begin
             if i < 16 then
               lemma_xor_cond_not (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${mask}) i)
                                  (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted}) i)
           end"#
    );

    let shifted_to_positive_in_range =
        mm256_sub_epi16(shifted_to_positive, field_modulus_quartered);
    hax_lib::fstar!(
        r#"introduce forall (i: nat). i < 16 ==>
             v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted_to_positive_in_range}) i) ==
             (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) in
              if vec_i <= 1664 then 832 - vec_i else vec_i - 2497)
           with begin
             if i < 16 then begin
               let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) in
               let s = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted}) i) in
               assert (s == 1664 - vec_i);
               assert (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${shifted_to_positive}) i) ==
                       (if s < 0 then -s - 1 else s));
               assert (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${field_modulus_quartered}) i) == 832)
             end
           end"#
    );

    let result = mm256_srli_epi16::<15>(shifted_to_positive_in_range);
    hax_lib::fstar!(
        r#"lemma_mm256_srli_epi16_15 ${shifted_to_positive_in_range};
           assert (forall (i: nat). i < 16 ==>
             v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) i) ==
             (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) in
              if 833 <= vec_i && vec_i <= 2496 then 1 else 0));
           introduce forall (i: nat). i < 16 ==>
             (let vec_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) in
              ((vec_i * 4 + 3329) / 6658) % 2 ==
              (if 833 <= vec_i && vec_i <= 2496 then 1 else 0))
           with begin
             if i < 16 then
               lemma_compress_message_identity
                 (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i))
           end"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(
    r##"
(* ───────────────────────────────────────────────────────────────────────
   mulhi composite lemma: lane j of `mulhi_mm256_epi32 lhs rhs` is the high
   32 bits of the unsigned 32x32 product of lane j of lhs and rhs.  Proven
   from the (validated) mul_epu32 / shuffle_epi32 / unpack{lo,hi}_epi32 /
   unpackhi_epi64 lane axioms — NOT itself an axiom.
   ─────────────────────────────────────────────────────────────────────── *)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_mulhi_hi32 (p: Iavx.t_Vec256) (i: nat{i < 4}) (bigp: int)
  : Lemma
    (requires Iavx.lane64u p i == bigp /\ 0 <= bigp /\ bigp / 4294967296 < 2147483648)
    (ensures Iavx.lane32 p (2 * i + 1) == bigp / 4294967296)
  = let lo = Iavx.lane32 p (2 * i) in
    let hiv = Iavx.lane32 p (2 * i + 1) in
    assert (Iavx.lane64u p i == (lo % 4294967296) + 4294967296 * (hiv % 4294967296));
    FStar.Math.Lemmas.lemma_div_plus (lo % 4294967296) (hiv % 4294967296) 4294967296;
    FStar.Math.Lemmas.lemma_div_mod hiv 4294967296

let lemma_shuffle245_even (vec: Iavx.t_Vec256) (i: nat{i < 4})
  : Lemma (ensures Iavx.lane32 (Iavx.mm256_shuffle_epi32 (mk_i32 245) vec) (2 * i)
                   == Iavx.lane32 vec (2 * i + 1))
  = match i with
    | 0 -> reveal_opaque (`%Iavx.shuffle32_src) (Iavx.shuffle32_src (mk_i32 245) 0)
    | 1 -> reveal_opaque (`%Iavx.shuffle32_src) (Iavx.shuffle32_src (mk_i32 245) 2)
    | 2 -> reveal_opaque (`%Iavx.shuffle32_src) (Iavx.shuffle32_src (mk_i32 245) 4)
    | _ -> reveal_opaque (`%Iavx.shuffle32_src) (Iavx.shuffle32_src (mk_i32 245) 6)
#pop-options

(* Ground per-lane fact lemmas, each isolating ONE intrinsic axiom (minimal
   context, ~ms each).  The mulhi assembly below cites these and EXCLUDES the
   quantified intrinsic posts (`--using_facts_from -...`) so the 7 coexisting
   lane-foralls (mul_epu32 ×2, shuffle ×2, unpack ×3) cannot cross-saturate. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let mul_epu32_lane_nn (a b: Iavx.t_Vec256) (i: nat{i < 4})
  : Lemma (requires 0 <= Iavx.lane32 a (2 * i) /\ 0 <= Iavx.lane32 b (2 * i))
          (ensures Iavx.lane64u (Iavx.mm256_mul_epu32 a b) i ==
                   Iavx.lane32 a (2 * i) * Iavx.lane32 b (2 * i)) = ()
let unpacklo_lane (a b: Iavx.t_Vec256) (k: nat{k < 8})
  : Lemma (Iavx.lane32 (Iavx.mm256_unpacklo_epi32 a b) k ==
           (match k with | 0 -> Iavx.lane32 a 0 | 1 -> Iavx.lane32 b 0
            | 2 -> Iavx.lane32 a 1 | 3 -> Iavx.lane32 b 1 | 4 -> Iavx.lane32 a 4
            | 5 -> Iavx.lane32 b 4 | 6 -> Iavx.lane32 a 5 | _ -> Iavx.lane32 b 5)) = ()
let unpackhi_lane (a b: Iavx.t_Vec256) (k: nat{k < 8})
  : Lemma (Iavx.lane32 (Iavx.mm256_unpackhi_epi32 a b) k ==
           (match k with | 0 -> Iavx.lane32 a 2 | 1 -> Iavx.lane32 b 2
            | 2 -> Iavx.lane32 a 3 | 3 -> Iavx.lane32 b 3 | 4 -> Iavx.lane32 a 6
            | 5 -> Iavx.lane32 b 6 | 6 -> Iavx.lane32 a 7 | _ -> Iavx.lane32 b 7)) = ()
let unpackhi64_lane (a b: Iavx.t_Vec256) (j: nat{j < 8})
  : Lemma (Iavx.lane32 (Iavx.mm256_unpackhi_epi64 a b) j ==
           (match j with | 0 -> Iavx.lane32 a 2 | 1 -> Iavx.lane32 a 3
            | 2 -> Iavx.lane32 b 2 | 3 -> Iavx.lane32 b 3 | 4 -> Iavx.lane32 a 6
            | 5 -> Iavx.lane32 a 7 | 6 -> Iavx.lane32 b 6 | _ -> Iavx.lane32 b 7)) = ()
#pop-options

(* One single-lane lemma per result lane (no in-body match — the 4/8-way match
   combines all branch contexts into one VC and saturates).  Each is the proven
   `test_lane0` shape: cite the ground per-lane facts, exclude the quantified
   intrinsic posts.  ~ms each. *)
unfold let mulhi_l_pre (lhs rhs: Iavx.t_Vec256) (j: nat{j < 8}) : prop =
  0 <= Iavx.lane32 lhs j /\ 0 <= Iavx.lane32 rhs j /\
  (Iavx.lane32 lhs j * Iavx.lane32 rhs j) / 4294967296 < 2147483648
unfold let mulhi_l_post (lhs rhs: Iavx.t_Vec256) (j: nat{j < 8}) : prop =
  Iavx.lane32 (mulhi_mm256_epi32 lhs rhs) j == (Iavx.lane32 lhs j * Iavx.lane32 rhs j) / 4294967296

#push-options "--fuel 1 --ifuel 1 --z3rlimit 50 --using_facts_from '* -Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 -Libcrux_intrinsics.Avx2_extract.mm256_mul_epu32 -Libcrux_intrinsics.Avx2_extract.mm256_shuffle_epi32'"
let lemma_mulhi_l0 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 0) (ensures mulhi_l_post lhs rhs 0)
  = let prod02 = Iavx.mm256_mul_epu32 lhs rhs in
    let prod13 = Iavx.mm256_mul_epu32 (Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs) (Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 0; lemma_mulhi_hi32 prod02 0 (Iavx.lane32 lhs 0 * Iavx.lane32 rhs 0);
    unpacklo_lane prod02 prod13 2; unpackhi64_lane ulo uhi 0
let lemma_mulhi_l2 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 2) (ensures mulhi_l_post lhs rhs 2)
  = let prod02 = Iavx.mm256_mul_epu32 lhs rhs in
    let prod13 = Iavx.mm256_mul_epu32 (Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs) (Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 1; lemma_mulhi_hi32 prod02 1 (Iavx.lane32 lhs 2 * Iavx.lane32 rhs 2);
    unpackhi_lane prod02 prod13 2; unpackhi64_lane ulo uhi 2
let lemma_mulhi_l4 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 4) (ensures mulhi_l_post lhs rhs 4)
  = let prod02 = Iavx.mm256_mul_epu32 lhs rhs in
    let prod13 = Iavx.mm256_mul_epu32 (Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs) (Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 2; lemma_mulhi_hi32 prod02 2 (Iavx.lane32 lhs 4 * Iavx.lane32 rhs 4);
    unpacklo_lane prod02 prod13 6; unpackhi64_lane ulo uhi 4
let lemma_mulhi_l6 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 6) (ensures mulhi_l_post lhs rhs 6)
  = let prod02 = Iavx.mm256_mul_epu32 lhs rhs in
    let prod13 = Iavx.mm256_mul_epu32 (Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs) (Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs) in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    mul_epu32_lane_nn lhs rhs 3; lemma_mulhi_hi32 prod02 3 (Iavx.lane32 lhs 6 * Iavx.lane32 rhs 6);
    unpackhi_lane prod02 prod13 6; unpackhi64_lane ulo uhi 6
let lemma_mulhi_l1 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 1) (ensures mulhi_l_post lhs rhs 1)
  = let shl = Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Iavx.mm256_mul_epu32 lhs rhs in let prod13 = Iavx.mm256_mul_epu32 shl shr in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 0; lemma_shuffle245_even rhs 0;
    mul_epu32_lane_nn shl shr 0; lemma_mulhi_hi32 prod13 0 (Iavx.lane32 lhs 1 * Iavx.lane32 rhs 1);
    unpacklo_lane prod02 prod13 3; unpackhi64_lane ulo uhi 1
let lemma_mulhi_l3 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 3) (ensures mulhi_l_post lhs rhs 3)
  = let shl = Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Iavx.mm256_mul_epu32 lhs rhs in let prod13 = Iavx.mm256_mul_epu32 shl shr in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 1; lemma_shuffle245_even rhs 1;
    mul_epu32_lane_nn shl shr 1; lemma_mulhi_hi32 prod13 1 (Iavx.lane32 lhs 3 * Iavx.lane32 rhs 3);
    unpackhi_lane prod02 prod13 3; unpackhi64_lane ulo uhi 3
let lemma_mulhi_l5 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 5) (ensures mulhi_l_post lhs rhs 5)
  = let shl = Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Iavx.mm256_mul_epu32 lhs rhs in let prod13 = Iavx.mm256_mul_epu32 shl shr in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 2; lemma_shuffle245_even rhs 2;
    mul_epu32_lane_nn shl shr 2; lemma_mulhi_hi32 prod13 2 (Iavx.lane32 lhs 5 * Iavx.lane32 rhs 5);
    unpacklo_lane prod02 prod13 7; unpackhi64_lane ulo uhi 5
let lemma_mulhi_l7 (lhs rhs: Iavx.t_Vec256) : Lemma (requires mulhi_l_pre lhs rhs 7) (ensures mulhi_l_post lhs rhs 7)
  = let shl = Iavx.mm256_shuffle_epi32 (mk_i32 245) lhs in let shr = Iavx.mm256_shuffle_epi32 (mk_i32 245) rhs in
    let prod02 = Iavx.mm256_mul_epu32 lhs rhs in let prod13 = Iavx.mm256_mul_epu32 shl shr in
    let ulo = Iavx.mm256_unpacklo_epi32 prod02 prod13 in let uhi = Iavx.mm256_unpackhi_epi32 prod02 prod13 in
    lemma_shuffle245_even lhs 3; lemma_shuffle245_even rhs 3;
    mul_epu32_lane_nn shl shr 3; lemma_mulhi_hi32 prod13 3 (Iavx.lane32 lhs 7 * Iavx.lane32 rhs 7);
    unpackhi_lane prod02 prod13 7; unpackhi64_lane ulo uhi 7
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_mulhi_mm256_epi32 (lhs rhs: Iavx.t_Vec256) (j: nat{j < 8})
  : Lemma (requires mulhi_l_pre lhs rhs j) (ensures mulhi_l_post lhs rhs j)
  = match j with
    | 0 -> lemma_mulhi_l0 lhs rhs | 1 -> lemma_mulhi_l1 lhs rhs
    | 2 -> lemma_mulhi_l2 lhs rhs | 3 -> lemma_mulhi_l3 lhs rhs
    | 4 -> lemma_mulhi_l4 lhs rhs | 5 -> lemma_mulhi_l5 lhs rhs
    | 6 -> lemma_mulhi_l6 lhs rhs | _ -> lemma_mulhi_l7 lhs rhs
#pop-options

(* ───────────────────────────────────────────────────────────────────────
   d-bit compress body: ground per-stage lane lemmas + a per-half spine lemma.
   ─────────────────────────────────────────────────────────────────────── *)

(* a non-negative small lane pins its two i16 lanes *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 60"
let lemma_lane32_to_i16 (a: Iavx.t_Vec256) (j: nat{j < 8}) (av: nat)
  : Lemma (requires Iavx.lane32 a j == av /\ av < 32768)
          (ensures v (Iavx.get_lane a (2 * j)) == av /\ v (Iavx.get_lane a (2 * j + 1)) == 0) = ()
#pop-options

(* @%-into-i32 is the identity on a non-negative value below 2^31 — proved on an
   ABSTRACT int so the nonlinear product it is applied to never enters the @% VC. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 40"
let lemma_atpercent_id (p: int)
  : Lemma (requires 0 <= p /\ p < 2147483648) (ensures p @% 4294967296 == p)
  = FStar.Math.Lemmas.small_mod p 4294967296
#pop-options

(* per-stage ground facts (one intrinsic axiom each) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let cvtepi_lane_nn (c0: Iavx.t_Vec128) (j: nat{j < 8})
  : Lemma (requires 0 <= v (Iavx.get_lane128 c0 j) /\ v (Iavx.get_lane128 c0 j) < 3329)
          (ensures Iavx.lane32 (Iavx.mm256_cvtepi16_epi32 c0) j == v (Iavx.get_lane128 c0 j)) = ()
let slli_lane_nowrap (c1: Iavx.t_Vec256) (cb: i32) (j: nat{j < 8})
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
                    0 <= Iavx.lane32 c1 j /\ Iavx.lane32 c1 j < 3329)
          (ensures Iavx.lane32 (Iavx.mm256_slli_epi32 cb c1) j == Iavx.lane32 c1 j * pow2 (v cb))
  = assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb);
    FStar.Math.Lemmas.lemma_mult_le_left (Iavx.lane32 c1 j) (pow2 (v cb)) 2048;
    FStar.Math.Lemmas.lemma_mult_le_right 2048 (Iavx.lane32 c1 j) 3328;
    lemma_atpercent_id (Iavx.lane32 c1 j * pow2 (v cb))
let add_lane_1664 (c2 fmh: Iavx.t_Vec256) (j: nat{j < 8})
  : Lemma (requires Iavx.lane32 fmh j == 1664 /\ 0 <= Iavx.lane32 c2 j /\ Iavx.lane32 c2 j < 6815745)
          (ensures Iavx.lane32 (Iavx.mm256_add_epi32 c2 fmh) j == Iavx.lane32 c2 j + 1664) = ()
let srli3_lane (c4: Iavx.t_Vec256) (j: nat{j < 8})
  : Lemma (requires 0 <= Iavx.lane32 c4 j /\ Iavx.lane32 c4 j < 2147483648)
          (ensures Iavx.lane32 (Iavx.mm256_srli_epi32 (mk_i32 3) c4) j == Iavx.lane32 c4 j / 8) = ()
#pop-options

(* AND with a broadcast (2^dd - 1) mask reduces a small non-negative lane mod 2^dd *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_and_mask_lane (c5 mask: Iavx.t_Vec256) (j: nat{j < 8}) (cval: nat) (dd: nat)
  : Lemma (requires 0 < dd /\ dd <= 11 /\ Iavx.lane32 c5 j == cval /\ cval < 32768 /\
                    Iavx.get_lane mask (2 * j) == mk_i16 (pow2 dd - 1) /\
                    Iavx.get_lane mask (2 * j + 1) == mk_i16 0)
          (ensures Iavx.lane32 (Iavx.mm256_and_si256 c5 mask) j == cval % pow2 dd)
  = lemma_lane32_to_i16 c5 j cval;
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    Rust_primitives.Integers.logand_mask_lemma (mk_i16 cval) dd;
    assert (mk_i16 (pow2 dd - 1) ==
            Rust_primitives.Integers.sub #i16_inttype (mk_i16 (pow2 dd)) (mk_i16 1));
    Rust_primitives.Integers.logand_lemma (mk_i16 0) (mk_i16 0)
#pop-options

(* The per-half spine, symbolic in lane j: the cvtepi/slli/add/mulhi/srli/and
   chain computes the Barrett `((x*2^d+1664)*10321340)>>35 & (2^d-1)`.  Constant
   vectors fmh/cf/mask are passed in with their lane facts so the recomputed
   spine matches the body's exact `set1` expressions definitionally. *)
(* nonlinear bounds, proven in CLEAN context (no SIMD terms) so the heavy
   half-lemma below only consumes them as ground facts. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 60"
let lemma_compress_nn_bounds (xv dd: nat)
  : Lemma (requires xv < 3329 /\ (dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11))
          (ensures (let nn = xv * pow2 dd + 1664 in
                    nn <= 6817408 /\
                    (nn * 10321340) / pow2 32 < 2147483648 /\
                    (nn * 10321340) / pow2 35 < 32768))
  = assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    FStar.Math.Lemmas.lemma_mult_le_right (pow2 dd) xv 3328;
    FStar.Math.Lemmas.lemma_mult_le_left 3328 (pow2 dd) 2048;
    let nn = xv * pow2 dd + 1664 in
    FStar.Math.Lemmas.lemma_mult_le_right 10321340 nn 6817408;
    assert_norm (6817408 * 10321340 == 70364785886720);
    assert_norm (pow2 47 == 140737488355328);
    assert (nn * 10321340 < pow2 47);
    FStar.Math.Lemmas.lemma_div_lt (nn * 10321340) 47 32;
    FStar.Math.Lemmas.lemma_div_lt (nn * 10321340) 47 35;
    assert_norm (pow2 15 == 32768); assert_norm (pow2 12 == 4096);
    assert_norm (pow2 31 == 2147483648)
#pop-options

(* Exclude lane32's DEFINITION too: keep it an atomic uninterpreted term so the
   products `lane32 c3 j * lane32 cf j` etc. don't unfold into get_lane arithmetic
   (which is what saturates Z3 nonlinearly).  All lane32 equalities the chain needs
   are supplied as facts by the per-stage lemmas + the clean bounds helper. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Libcrux_intrinsics.Avx2_extract.lane32 -Libcrux_intrinsics.Avx2_extract.mm256_cvtepi16_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_add_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_srli_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_mul_epu32 -Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 -Libcrux_intrinsics.Avx2_extract.mm256_shuffle_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_set1_epi32'"
let lemma_compress_half (c0: Iavx.t_Vec128) (cb: i32) (fmh cf mask: Iavx.t_Vec256) (j: nat{j < 8})
  : Lemma
    (requires
       (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
       0 <= v (Iavx.get_lane128 c0 j) /\ v (Iavx.get_lane128 c0 j) < 3329 /\
       Iavx.lane32 fmh j == 1664 /\ Iavx.lane32 cf j == 10321340 /\
       Iavx.get_lane mask (2 * j) == mk_i16 (pow2 (v cb) - 1) /\
       Iavx.get_lane mask (2 * j + 1) == mk_i16 0)
    (ensures
       (let c1 = Iavx.mm256_cvtepi16_epi32 c0 in
        let c2 = Iavx.mm256_slli_epi32 cb c1 in
        let c3 = Iavx.mm256_add_epi32 c2 fmh in
        let c4 = mulhi_mm256_epi32 c3 cf in
        let c5 = Iavx.mm256_srli_epi32 (mk_i32 3) c4 in
        let c6 = Iavx.mm256_and_si256 c5 mask in
        let xv = v (Iavx.get_lane128 c0 j) in
        let dd = v cb in
        0 <= Iavx.lane32 c6 j /\ Iavx.lane32 c6 j < pow2 dd /\
        Iavx.lane32 c6 j == (((xv * pow2 dd + 1664) * 10321340) / pow2 35) % pow2 dd))
  = let dd = v cb in
    let xv = v (Iavx.get_lane128 c0 j) in
    let c1 = Iavx.mm256_cvtepi16_epi32 c0 in
    let c2 = Iavx.mm256_slli_epi32 cb c1 in
    let c3 = Iavx.mm256_add_epi32 c2 fmh in
    let c4 = mulhi_mm256_epi32 c3 cf in
    let c5 = Iavx.mm256_srli_epi32 (mk_i32 3) c4 in
    let nn = xv * pow2 dd + 1664 in
    assert_norm (pow2 32 == 4294967296); assert_norm (pow2 35 == 34359738368);
    lemma_compress_nn_bounds xv dd;     (* ground: nn<=6817408, nn*K/2^32<=32767, nn*K/2^35<=4095 *)
    cvtepi_lane_nn c0 j;                 (* lane32 c1 j == xv *)
    slli_lane_nowrap c1 cb j;            (* lane32 c2 j == xv*2^dd *)
    add_lane_1664 c2 fmh j;             (* lane32 c3 j == nn *)
    assert (Iavx.lane32 c3 j == nn);
    assert (Iavx.lane32 c3 j * Iavx.lane32 cf j == nn * 10321340);
    lemma_mulhi_mm256_epi32 c3 cf j;    (* lane32 c4 j == (nn*10321340)/2^32 *)
    assert (Iavx.lane32 c4 j == (nn * 10321340) / pow2 32);
    srli3_lane c4 j;                    (* lane32 c5 j == ((nn*10321340)/2^32)/8 *)
    FStar.Math.Lemmas.division_multiplication_lemma (nn * 10321340) 4294967296 8;
    assert (Iavx.lane32 c5 j == (nn * 10321340) / pow2 35);
    lemma_and_mask_lane c5 mask j ((nn * 10321340) / pow2 35) dd
#pop-options

(* pack + permute<0b11_01_10_00=0xD8>: ground per-lane facts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let packs_lane (a b: Iavx.t_Vec256) (k: nat{k < 16})
  : Lemma (Iavx.get_lane (Iavx.mm256_packs_epi32 a b) k ==
           (if k < 4 then Iavx.sat_i16 (Iavx.lane32 a k)
            else if k < 8 then Iavx.sat_i16 (Iavx.lane32 b (k - 4))
            else if k < 12 then Iavx.sat_i16 (Iavx.lane32 a (k - 4))
            else Iavx.sat_i16 (Iavx.lane32 b (k - 8)))) = ()
let permute_lane_0xD8 (vec: Iavx.t_Vec256) (k: nat{k < 16})
  : Lemma (Iavx.get_lane (Iavx.mm256_permute4x64_epi64 (mk_i32 216) vec) k ==
           Iavx.get_lane vec (4 * Iavx.permute64_src (mk_i32 216) (k / 4) + k % 4)) = ()
#pop-options

(* result.[i] of permute<0xD8>(packs cl6 ch6) == lane32 of cl6 (i<8) / ch6 (i-8),
   when those lanes are < 2^15 (so the i16 saturation is the identity). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 60 --using_facts_from '* -Libcrux_intrinsics.Avx2_extract.lane32 -Libcrux_intrinsics.Avx2_extract.mm256_packs_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_permute4x64_epi64'"
let lemma_result_lane (cl6 ch6: Iavx.t_Vec256) (i: nat{i < 16})
  : Lemma
    (requires (forall (k: nat). k < 8 ==>
                 0 <= Iavx.lane32 cl6 k /\ Iavx.lane32 cl6 k < 32768 /\
                 0 <= Iavx.lane32 ch6 k /\ Iavx.lane32 ch6 k < 32768))
    (ensures v (Iavx.get_lane (Iavx.mm256_permute4x64_epi64 (mk_i32 216)
                                 (Iavx.mm256_packs_epi32 cl6 ch6)) i) ==
             (if i < 8 then Iavx.lane32 cl6 i else Iavx.lane32 ch6 (i - 8)))
  = let packed = Iavx.mm256_packs_epi32 cl6 ch6 in
    permute_lane_0xD8 packed i;
    let r0 = reveal_opaque (`%Iavx.permute64_src) (Iavx.permute64_src (mk_i32 216) 0) in
    let r1 = reveal_opaque (`%Iavx.permute64_src) (Iavx.permute64_src (mk_i32 216) 1) in
    let r2 = reveal_opaque (`%Iavx.permute64_src) (Iavx.permute64_src (mk_i32 216) 2) in
    let r3 = reveal_opaque (`%Iavx.permute64_src) (Iavx.permute64_src (mk_i32 216) 3) in
    (match i with
     | 0 -> packs_lane cl6 ch6 0  | 1 -> packs_lane cl6 ch6 1
     | 2 -> packs_lane cl6 ch6 2  | 3 -> packs_lane cl6 ch6 3
     | 4 -> packs_lane cl6 ch6 8  | 5 -> packs_lane cl6 ch6 9
     | 6 -> packs_lane cl6 ch6 10 | 7 -> packs_lane cl6 ch6 11
     | 8 -> packs_lane cl6 ch6 4  | 9 -> packs_lane cl6 ch6 5
     | 10 -> packs_lane cl6 ch6 6 | 11 -> packs_lane cl6 ch6 7
     | 12 -> packs_lane cl6 ch6 12 | 13 -> packs_lane cl6 ch6 13
     | 14 -> packs_lane cl6 ch6 14 | _ -> packs_lane cl6 ch6 15)
#pop-options

(* set1_epi32 lane facts: lane32 == the broadcast constant; for a < 2^16
   constant, the per-i16 decomposition (low = constant, high = 0). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80 --split_queries always"
let set1_lane32 (c: i32) (j: nat{j < 8})
  : Lemma (Iavx.lane32 (Iavx.mm256_set1_epi32 c) j == v c) = ()
let set1_mask_i16 (c: i32) (dd: nat{0 < dd /\ dd <= 11}) (j: nat{j < 8})
  : Lemma (requires v c == pow2 dd - 1)
          (ensures Iavx.get_lane (Iavx.mm256_set1_epi32 c) (2 * j) == mk_i16 (pow2 dd - 1) /\
                   Iavx.get_lane (Iavx.mm256_set1_epi32 c) (2 * j + 1) == mk_i16 0)
  = assert_norm (pow2 11 == 2048); FStar.Math.Lemmas.pow2_le_compat 11 dd
(* the d-bit mask constant `(1 <<! cb) - 1` has value `2^d - 1` (clean context so
   the `@%.` in shift_left_positive_lemma evaluates). *)
let lemma_mask_val (cb: i32)
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11))
          (ensures v ((mk_i32 1 <<! cb <: i32) -! mk_i32 1) == pow2 (v cb) - 1)
  = assert_norm (pow2 11 == 2048); assert_norm (pow2 31 == 2147483648);
    assert_norm (pow2 32 == 4294967296);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb)
#pop-options

(* local copies of the cast bridges (Vector.Avx2.Ntt has the originals, but it
   is not a dependency of this module). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let compress_castsi256_lemma (vc: Iavx.t_Vec256)
  : Lemma (ensures (forall (i: nat). i < 8 ==>
            Seq.index (Iavx.vec128_as_i16x8 (Iavx.mm256_castsi256_si128 vc)) i ==
            Seq.index (Iavx.vec256_as_i16x16 vc) i))
  = let aux (i: nat{i < 8})
      : Lemma (Seq.index (Iavx.vec128_as_i16x8 (Iavx.mm256_castsi256_si128 vc)) i ==
               Seq.index (Iavx.vec256_as_i16x16 vc) i) =
      let a = Seq.index (Iavx.vec128_as_i16x8 (Iavx.mm256_castsi256_si128 vc)) i in
      let b = Seq.index (Iavx.vec256_as_i16x16 vc) i in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        let k : nat = 16 * i + nthv in
        assert (k < 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        Iavx.bit_vec_of_int_t_array_vec128_as_i16x8_lemma (Iavx.mm256_castsi256_si128 vc) 16 k;
        Iavx.bit_vec_of_int_t_array_vec256_as_i16x16_lemma vc 16 k;
        assert (k / 16 == i); assert (k % 16 == nthv);
        assert (Iavx.mm256_castsi256_si128 vc k == vc k)
      in Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in Classical.forall_intro aux

let compress_extracti128_lemma (vc: Iavx.t_Vec256)
  : Lemma (ensures (forall (i: nat). i < 8 ==>
            Seq.index (Iavx.vec128_as_i16x8 (Iavx.mm256_extracti128_si256 (mk_i32 1) vc)) i ==
            Seq.index (Iavx.vec256_as_i16x16 vc) (i + 8)))
  = let aux (i: nat{i < 8})
      : Lemma (Seq.index (Iavx.vec128_as_i16x8 (Iavx.mm256_extracti128_si256 (mk_i32 1) vc)) i ==
               Seq.index (Iavx.vec256_as_i16x16 vc) (i + 8)) =
      let a = Seq.index (Iavx.vec128_as_i16x8 (Iavx.mm256_extracti128_si256 (mk_i32 1) vc)) i in
      let b = Seq.index (Iavx.vec256_as_i16x16 vc) (i + 8) in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        FStar.Math.Lemmas.lemma_mult_le_left 16 (i + 8) 15;
        let k : nat = 16 * i + nthv in
        let k' : nat = 16 * (i + 8) + nthv in
        assert (k < 128); assert (k' < 256); assert (k' == k + 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        FStar.Math.Lemmas.lemma_div_plus nthv (i + 8) 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv (i + 8) 16;
        Iavx.bit_vec_of_int_t_array_vec128_as_i16x8_lemma (Iavx.mm256_extracti128_si256 (mk_i32 1) vc) 16 k;
        Iavx.bit_vec_of_int_t_array_vec256_as_i16x16_lemma vc 16 k';
        assert (k / 16 == i); assert (k % 16 == nthv);
        assert (k' / 16 == i + 8); assert (k' % 16 == nthv);
        assert (Iavx.mm256_extracti128_si256 (mk_i32 1) vc k == vc (k + 128))
      in Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in Classical.forall_intro aux
#pop-options
"##
)]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always --using_facts_from '* -Libcrux_intrinsics.Avx2_extract.lane32 -Libcrux_intrinsics.Avx2_extract.mm256_cvtepi16_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_add_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_srli_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_mul_epu32 -Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 -Libcrux_intrinsics.Avx2_extract.mm256_shuffle_epi32'")]
#[hax_lib::requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/ v $COEFFICIENT_BITS == 5 \/
    v $COEFFICIENT_BITS == 10 \/ v $COEFFICIENT_BITS == 11) /\
    range (v ((mk_i32 1) <<! $COEFFICIENT_BITS) - 1) i32_inttype /\
    (forall (i: nat). i < 16 ==>
      0 <= v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) /\
      v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) < 3329)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let ri = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) i) in
     let vi = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) in
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
    hax_lib::fstar!(
        r#"
  let dd = v v_COEFFICIENT_BITS in
  assert_norm (pow2 11 == 2048);
  FStar.Math.Lemmas.pow2_le_compat 11 dd;
  compress_castsi256_lemma vector;
  compress_extracti128_lemma vector;
  let aux_low (j: nat{j < 8})
    : Lemma (0 <= Iavx.lane32 compressed_low j /\ Iavx.lane32 compressed_low j < pow2 dd /\
             Iavx.lane32 compressed_low j ==
             (((v (Seq.index (Iavx.vec256_as_i16x16 vector) j) * pow2 dd + 1664) * 10321340) / pow2 35)
             % pow2 dd) =
    set1_lane32 (((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1) /!
                 mk_i32 2) j;
    set1_lane32 (mk_i32 10321340) j;
    assert_norm (v ((((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1)
                     <: i32) /! mk_i32 2) == 1664);
    lemma_mask_val v_COEFFICIENT_BITS;
    set1_mask_i16 ((mk_i32 1 <<! v_COEFFICIENT_BITS <: i32) -! mk_i32 1) dd j;
    lemma_compress_half (Iavx.mm256_castsi256_si128 vector) v_COEFFICIENT_BITS field_modulus_halved
      compression_factor coefficient_bits_mask j
  in
  Classical.forall_intro aux_low;
  let aux_high (j: nat{j < 8})
    : Lemma (0 <= Iavx.lane32 compressed_high j /\ Iavx.lane32 compressed_high j < pow2 dd /\
             Iavx.lane32 compressed_high j ==
             (((v (Seq.index (Iavx.vec256_as_i16x16 vector) (j + 8)) * pow2 dd + 1664) * 10321340)
              / pow2 35) % pow2 dd) =
    set1_lane32 (((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1) /!
                 mk_i32 2) j;
    set1_lane32 (mk_i32 10321340) j;
    assert_norm (v ((((cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) -! mk_i32 1)
                     <: i32) /! mk_i32 2) == 1664);
    lemma_mask_val v_COEFFICIENT_BITS;
    set1_mask_i16 ((mk_i32 1 <<! v_COEFFICIENT_BITS <: i32) -! mk_i32 1) dd j;
    lemma_compress_half (Iavx.mm256_extracti128_si256 (mk_i32 1) vector) v_COEFFICIENT_BITS
      field_modulus_halved compression_factor coefficient_bits_mask j
  in
  Classical.forall_intro aux_high;
  assert (forall (k: nat). k < 8 ==>
            0 <= Iavx.lane32 compressed_low k /\ Iavx.lane32 compressed_low k < 32768 /\
            0 <= Iavx.lane32 compressed_high k /\ Iavx.lane32 compressed_high k < 32768);
  let aux_res (i: nat{i < 16})
    : Lemma (let ri = v (Seq.index (Iavx.vec256_as_i16x16 result) i) in
             let vi = v (Seq.index (Iavx.vec256_as_i16x16 vector) i) in
             0 <= ri /\ ri < pow2 dd /\
             ri == (((vi * pow2 dd + 1664) * 10321340) / pow2 35) % pow2 dd) =
    lemma_result_lane compressed_low compressed_high i;
    if i < 8 then
      assert (v (Seq.index (Iavx.vec256_as_i16x16 result) i) == Iavx.lane32 compressed_low i)
    else
      assert (v (Seq.index (Iavx.vec256_as_i16x16 result) i) == Iavx.lane32 compressed_high (i - 8))
  in
  Classical.forall_intro aux_res
"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"forall i. let x = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i in
                                      (x == mk_i16 0 \/ x == mk_i16 1)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    (let res_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result) i) in
     let a_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i) in
     (res_i == 0 \/ res_i == 1665) /\ res_i == (2 * a_i * 3329 + 2) / 4)"#))]
pub fn decompress_1(a: Vec256) -> Vec256 {
    let z = mm256_setzero_si256();

    hax_lib::fstar!(
        r#"
        assert(Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $z == Seq.create 16 (mk_i16 0));
        assert(forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $z) i == mk_i16 0);
        assert(forall i. let x = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i in 
                                      ((0 - v x) == 0 \/ (0 - v x) == -1));
        assert(forall i. i < 16 ==>
                        Spec.Utils.is_intb (pow2 15 - 1) 
                        (0 - v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i)))
    "#
    );

    let s = arithmetic::sub(z, a);

    hax_lib::fstar!(
        r#"assert(forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $s) i == mk_i16 0 \/ 
                            Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $s) i == mk_i16 (-1))"#
    );

    let result = arithmetic::bitwise_and_with_constant(s, 1665);

    hax_lib::fstar!(
        r#"Rust_primitives.Integers.logand_lemma (mk_i16 1665) (mk_i16 1665);
           introduce forall (i: nat). i < 16 ==>
             (let res_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result) i) in
              let a_i = v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i) in
              (res_i == 0 \/ res_i == 1665) /\ res_i == (2 * a_i * 3329 + 2) / 4)
           with begin
             if i < 16 then begin
               let si = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $s) i in
               let ai = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i in
               assert (v si == - v ai);
               assert (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result) i ==
                       (si &. mk_i16 1665))
             end
           end"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"v $COEFFICIENT_BITS >= 0 /\ v $COEFFICIENT_BITS < bits i32_inttype"#))]
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
    mm256_permute4x64_epi64::<0b11_01_10_00>(compressed)
}
