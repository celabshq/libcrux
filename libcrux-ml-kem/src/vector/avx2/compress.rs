use crate::vector::FIELD_MODULUS;

use super::*;

// Multiply the 32-bit numbers contained in |lhs| and |rhs|, and store only
// the upper 32 bits of the resulting product.
// This implementation was taken from:
// https://ei1333.github.io/library/math/combinatorics/vectorize-mod-int.hpp.html
//
// TODO: Optimize this implementation if performance numbers suggest doing so.
#[inline(always)]
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
#[hax_lib::requires(fstar!(r#"v $COEFFICIENT_BITS >= 0 /\ v $COEFFICIENT_BITS < bits i32_inttype /\
    range (v ((mk_i32 1) <<! $COEFFICIENT_BITS) - 1) i32_inttype"#))]
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
    mm256_permute4x64_epi64::<0b11_01_10_00>(compressed)
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
