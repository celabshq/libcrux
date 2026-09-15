use super::*;

// Lemma library relocated to proofs/fstar/spec/Libcrux_ml_kem.Vector.Avx2.Ntt_theory.fst.
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views
module FS = Spec.Utils
module FA = Libcrux_ml_kem.Vector.Avx2.Arithmetic
open Libcrux_ml_kem.Vector.Avx2.Ntt_theory
"#
)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (8*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    Spec.Utils.ntt_layer_1_butterfly_post
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) zeta0 zeta1 zeta2 zeta3"#))]
pub(crate) fn ntt_layer_1_step(
    vector: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    let zetas = mm256_set_epi16(
        -zeta3, -zeta3, zeta3, zeta3, -zeta2, -zeta2, zeta2, zeta2, -zeta1, -zeta1, zeta1, zeta1,
        -zeta0, -zeta0, zeta0, zeta0,
    );
    proof!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas}))"#
    );

    let rhs0 = mm256_shuffle_epi32::<0b11_11_01_01>(vector);
    proof!(
        r#"fwd_shuffle_245 ${vector};
           fwd_shuffle_preserves_bound (mk_i32 245) ${vector} (7*3328)"#
    );
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs0, zetas);

    let lhs = mm256_shuffle_epi32::<0b10_10_00_00>(vector);
    proof!(
        r#"fwd_shuffle_160 ${vector};
           fwd_shuffle_preserves_bound (mk_i32 160) ${vector} (7*3328)"#
    );

    let result = mm256_add_epi16(lhs, rhs);
    proof!(
        r#"lemma_fwd_l1_resultv ${vector} ${lhs} ${rhs} ${result};
           assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 0) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 1) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 2) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 3) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 4) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 5) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 6) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 7) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 8) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 9) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 10) == - v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 11) == - v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 12) == v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 13) == v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 14) == - v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 15) == - v zeta3);
           lemma_fwd_l1_post
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${rhs})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result})
             zeta0 zeta1 zeta2 zeta3"#
    );
    result
}

// Forward-NTT layer-2: same mod-add-distributivity recipe as forward layer-1,
// with shuffle_epi32 controls 238/68 and len-4 pairs.  Reuses FI/FS/FA +
// lemma_modadd from the forward-layer-1 before-block (earlier in this file);
// fwd2_-prefixed names avoid collision.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b_array (6*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    Spec.Utils.ntt_layer_2_butterfly_post
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) zeta0 zeta1"#))]
pub(crate) fn ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let zetas = mm256_set_epi16(
        -zeta1, -zeta1, -zeta1, -zeta1, zeta1, zeta1, zeta1, zeta1, -zeta0, -zeta0, -zeta0, -zeta0,
        zeta0, zeta0, zeta0, zeta0,
    );
    proof!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas}))"#
    );

    let rhs0 = mm256_shuffle_epi32::<0b11_10_11_10>(vector);
    proof!(
        r#"fwd2_shuffle_238 ${vector};
           fwd2_shuffle_preserves_bound (mk_i32 238) ${vector} (6*3328)"#
    );
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs0, zetas);

    let lhs = mm256_shuffle_epi32::<0b01_00_01_00>(vector);
    proof!(
        r#"fwd2_shuffle_68 ${vector};
           fwd2_shuffle_preserves_bound (mk_i32 68) ${vector} (6*3328)"#
    );

    let result = mm256_add_epi16(lhs, rhs);
    proof!(
        r#"lemma_fwd_l2_resultv ${vector} ${lhs} ${rhs} ${result};
           assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 0) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 1) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 2) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 3) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 4) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 5) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 6) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 7) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 8) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 9) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 10) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 11) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 12) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 13) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 14) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 15) == - v zeta1);
           lemma_fwd_l2_post
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${rhs})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result})
             zeta0 zeta1"#
    );
    result
}

// Lemma library relocated to proofs/fstar/spec/Libcrux_ml_kem.Vector.Avx2.Ntt_theory.fst.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
    Spec.Utils.is_i16b_array (5*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (6*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    (forall (i:nat). i < 8 ==>
       v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) i) % 3329 ==
         (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) +
          v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) (i+8)) * v zeta * 169) % 3329 /\
       v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) (i+8)) % 3329 ==
         (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i) -
          v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) (i+8)) * v zeta * 169) % 3329)
"#))]
pub(crate) fn ntt_layer_3_step(vector: Vec256, zeta: i16) -> Vec256 {
    let rhs = mm256_extracti128_si256::<1>(vector);
    proof!(r#"lemma_mm256_extracti128_si256_1 ${vector}"#);
    // Now: forall i<8. get_lane128 rhs i = get_lane vector (i+8)

    let zetas_v128 = mm_set1_epi16(zeta);
    // Post: vec128_as_i16x8 zetas_v128 == Spec.Utils.create (sz 8) zeta
    // Pre for mont_mul: is_i16b_array 1664 zetas_v128 (since |zeta| <= 1664)
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${zetas_v128} i) == v zeta);
           assert (Spec.Utils.is_i16b_array 1664
                     (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 ${zetas_v128}))"#
    );

    let rhs = arithmetic::montgomery_multiply_m128i_by_constants(rhs, zetas_v128);
    // Post: is_i16b_array 3328 rhs /\
    //   forall i<8. v(get_lane128 rhs i) % 3329 ==
    //                  (v(get_lane vector (i+8)) * v zeta * 169) % 3329
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${rhs} i) % 3329 ==
                (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) (i + 8))
                  * v zeta * 169) % 3329)"#
    );

    let lhs = mm256_castsi256_si128(vector);
    proof!(r#"lemma_mm256_castsi256_si128 ${vector}"#);
    // Now: forall i<8. get_lane128 lhs i = get_lane vector i

    let lower_coefficients = mm_add_epi16(lhs, rhs);
    // Post: vec128_as_i16x8 lower == map2 (+.) ...
    // Use lemma_add_i_128 (SMTPat) to lift +. to +.
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lower_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lhs} i) +
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${rhs} i));
           assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lower_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) i) +
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${rhs} i))"#
    );

    let upper_coefficients = mm_sub_epi16(lhs, rhs);
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${upper_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lhs} i) -
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${rhs} i));
           assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${upper_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) i) -
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${rhs} i))"#
    );

    let combined_lo = mm256_castsi128_si256(lower_coefficients);
    proof!(r#"lemma_mm256_castsi128_si256_lo ${lower_coefficients}"#);

    let combined = mm256_inserti128_si256::<1>(combined_lo, upper_coefficients);
    proof!(r#"lemma_mm256_inserti128_si256_1 ${combined_lo} ${upper_coefficients}"#);
    // Final: forall i<8. combined[i] = lower[i], combined[i+8] = upper[i]
    proof!(
        r#"
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${combined}) i ==
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lower_coefficients} i);
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${combined}) (i + 8) ==
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${upper_coefficients} i)"#
    );
    combined
}

// The all-literal `mult` selector of `inv_ntt_layer_1_step`, isolated in its own
// tiny function.  Each of its sixteen `mk_i16 (+/-1)` literals carries a trivial
// `Integers.range` well-formedness check.  Inside `inv_ntt_layer_1_step` those
// checks SATURATE at the full rlimit: `--ext context_pruning` keeps only the
// fact-ids it deems relevant per split sub-query, and the shuffle / permute /
// sums / montgomery / barrett / blend machinery crowds the basic numeral axioms
// out of the relevant set.  Here the context is one op plus one lemma, so it does
// not.  (Factoring the whole pre-`zetas` HALF instead does NOT work — measured: it
// merely moves the same saturation from the consumer into the producer.)
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views
module ZS = Spec.Utils
module ZA = Libcrux_ml_kem.Vector.Avx2.Arithmetic
"#
)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::ensures(|mult| fstar!(r#"
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 0) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 1) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 2) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 3) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 4) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 5) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 6) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 7) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 8) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 9) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 10) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 11) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 12) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 13) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 14) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 15) == -1"#))]
fn inv_ntt_layer_1_mult() -> Vec256 {
    let mult = mm256_set_epi16(-1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1);
    proof!(
        r#"lemma_mm256_set_epi16_lanes (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 1) (mk_i16 1) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 1) (mk_i16 1) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 1) (mk_i16 1) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 1) (mk_i16 1)"#
    );
    mult
}

// Lemma library relocated to proofs/fstar/spec/Libcrux_ml_kem.Vector.Avx2.Ntt_theory.fst.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    Spec.Utils.inv_ntt_layer_1_butterfly_post
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) zeta0 zeta1 zeta2 zeta3"#))]
pub(crate) fn inv_ntt_layer_1_step(
    vector: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    let lhs = mm256_shuffle_epi32::<0b11_11_01_01>(vector);
    proof!(
        r#"lemma_shuffle_245 ${vector};
           lemma_shuffle_preserves_bound (mk_i32 245) ${vector} (4*3328)"#
    );

    let rhs0 = mm256_shuffle_epi32::<0b10_10_00_00>(vector);
    proof!(
        r#"lemma_shuffle_160 ${vector};
           lemma_shuffle_preserves_bound (mk_i32 160) ${vector} (4*3328)"#
    );

    let mult = inv_ntt_layer_1_mult();
    let rhs = mm256_mullo_epi16(rhs0, mult);

    let sum = mm256_add_epi16(lhs, rhs);
    proof!(r#"lemma_inv_l1_sums_v ${vector} ${lhs} ${rhs0} ${mult} ${rhs} ${sum}"#);

    let zetas = mm256_set_epi16(
        zeta3, zeta3, 0, 0, zeta2, zeta2, 0, 0, zeta1, zeta1, 0, 0, zeta0, zeta0, 0, 0,
    );
    let sum_times_zetas = arithmetic::montgomery_multiply_by_constants(sum, zetas);

    let sum_reduced = arithmetic::barrett_reduce(sum);

    let result = mm256_blend_epi16::<0b1_1_0_0_1_1_0_0>(sum_reduced, sum_times_zetas);
    proof!(
        r#"assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 2) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 3) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 6) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 7) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 10) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 11) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 14) == v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 15) == v zeta3);
           lemma_blend_204 ${sum_reduced} ${sum_times_zetas};
           lemma_inv_l1_post
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${sum})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${sum_reduced})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${sum_times_zetas})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result})
             zeta0 zeta1 zeta2 zeta3"#
    );
    result
}

// The all-literal `mult` selector of `inv_ntt_layer_2_step` — same
// pruning-starvation isolation as `inv_ntt_layer_1_mult`, see its comment.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::ensures(|mult| fstar!(r#"
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 0) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 1) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 2) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 3) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 4) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 5) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 6) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 7) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 8) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 9) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 10) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 11) == 1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 12) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 13) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 14) == -1 /\
    v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${mult} 15) == -1"#))]
fn inv_ntt_layer_2_mult() -> Vec256 {
    let mult = mm256_set_epi16(-1, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1, 1, 1, 1, 1);
    proof!(
        r#"lemma_mm256_set_epi16_lanes (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 1) (mk_i16 1) (mk_i16 1) (mk_i16 1) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 (-1)) (mk_i16 1) (mk_i16 1) (mk_i16 1) (mk_i16 1)"#
    );
    mult
}

// Lemma library relocated to proofs/fstar/spec/Libcrux_ml_kem.Vector.Avx2.Ntt_theory.fst.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (2*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    Spec.Utils.inv_ntt_layer_2_butterfly_post
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) zeta0 zeta1"#))]
pub(crate) fn inv_ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let lhs = mm256_permute4x64_epi64::<0b11_11_01_01>(vector);
    proof!(
        r#"lemma_permute_245 ${vector};
           lemma_permute_preserves_bound (mk_i32 245) ${vector} 3328"#
    );

    let rhs0 = mm256_permute4x64_epi64::<0b10_10_00_00>(vector);
    proof!(
        r#"lemma_permute_160 ${vector};
           lemma_permute_preserves_bound (mk_i32 160) ${vector} 3328"#
    );

    let mult = inv_ntt_layer_2_mult();
    let rhs = mm256_mullo_epi16(rhs0, mult);

    let sum = mm256_add_epi16(lhs, rhs);
    proof!(r#"lemma_inv_l2_sums_v ${vector} ${lhs} ${rhs0} ${mult} ${rhs} ${sum}"#);

    let zetas = mm256_set_epi16(
        zeta1, zeta1, zeta1, zeta1, 0, 0, 0, 0, zeta0, zeta0, zeta0, zeta0, 0, 0, 0, 0,
    );
    let sum_times_zetas = arithmetic::montgomery_multiply_by_constants(sum, zetas);

    let result = mm256_blend_epi16::<0b1_1_1_1_0_0_0_0>(sum, sum_times_zetas);
    proof!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas}));
           assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 4) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 5) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 6) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 7) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 12) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 13) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 14) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${zetas} 15) == v zeta1);
           lemma_blend_240 ${sum} ${sum_times_zetas};
           lemma_inv_l2_post
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${sum})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${sum_times_zetas})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result})
             zeta0 zeta1"#
    );
    result
}

#[inline(always)]
// `inv_ntt_layer_3_step` had ONE sub-query that only ever passed via hint REPLAY: once a
// dependency digest changes (here, two additions to the canonical `Intrinsics_views`), the
// replay fails and the query saturates COLD at the full rlimit 400 (measured: 334 s).  A
// saturating query cannot be re-recorded — it produces no hint by construction — so it has
// to be made fast-stable cold.  `#restart-solver` gives this declaration a fresh z3 per
// sub-query, so the 50-odd preceding split sub-queries cannot pollute the solver state it
// runs in (same fix, same shape, as the ml-dsa `Simd.Avx2.Invntt` decls).
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
    Spec.Utils.is_i16b_array (2*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    (forall (i:nat). i < 8 ==>
       v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) i) % 3329 ==
         (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) (i+8)) +
          v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i)) % 3329 /\
       v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) (i+8)) % 3329 ==
         ((v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) (i+8)) -
           v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}) i))
          * v zeta * 169) % 3329)
"#))]
pub(crate) fn inv_ntt_layer_3_step(vector: Vec256, zeta: i16) -> Vec256 {
    let lhs = mm256_extracti128_si256::<1>(vector);
    proof!(r#"lemma_mm256_extracti128_si256_1 ${vector}"#);
    // forall i<8. get_lane128 lhs i = get_lane vector (i+8)

    let rhs = mm256_castsi256_si128(vector);
    proof!(r#"lemma_mm256_castsi256_si128 ${vector}"#);
    // forall i<8. get_lane128 rhs i = get_lane vector i

    let lower_coefficients = mm_add_epi16(lhs, rhs);
    // mm_add_epi16 post + lemma_add_i_128 (SMTPat) lift +. → +
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lower_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) (i + 8)) +
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) i))"#
    );

    let upper_coefficients = mm_sub_epi16(lhs, rhs);
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${upper_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) (i + 8)) -
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) i))"#
    );

    let zetas_v128 = mm_set1_epi16(zeta);
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${zetas_v128} i) == v zeta);
           assert (Spec.Utils.is_i16b_array 1664
                     (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 ${zetas_v128}))"#
    );

    let upper_coefficients =
        arithmetic::montgomery_multiply_m128i_by_constants(upper_coefficients, zetas_v128);
    // Post: is_i16b_array 3328 upper_coefficients /\
    //   forall i<8. v(upper[i]) % 3329 ==
    //               (v(vec[i+8]) - v(vec[i])) * v zeta * 169 % 3329
    proof!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${upper_coefficients} i) % 3329 ==
                ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) (i + 8)) -
                  v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${vector}) i))
                 * v zeta * 169) % 3329)"#
    );

    let combined_lo = mm256_castsi128_si256(lower_coefficients);
    proof!(r#"lemma_mm256_castsi128_si256_lo ${lower_coefficients}"#);

    let combined = mm256_inserti128_si256::<1>(combined_lo, upper_coefficients);
    proof!(r#"lemma_mm256_inserti128_si256_1 ${combined_lo} ${upper_coefficients}"#);
    // forall i<8. combined[i]   = lower[i]
    //             combined[i+8] = upper[i]
    proof!(
        r#"
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${combined}) i ==
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${lower_coefficients} i);
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (${combined}) (i + 8) ==
                Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 ${upper_coefficients} i)"#
    );
    combined
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
    Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 0) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 1) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 2) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 3) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 4) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 5) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 6) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 7) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 8) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 9) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 10) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 11) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 12) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 13) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 14) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${lhs} 15) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 0) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 1) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 2) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 3) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 4) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 5) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 6) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 7) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 8) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 9) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 10) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 11) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 12) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 13) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 14) /\
    Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane ${rhs} 15)"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) /\
    Spec.Utils.ntt_multiply_butterfly_post
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${lhs})
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${rhs})
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result})
      zeta0 zeta1 zeta2 zeta3"#))]
pub(crate) fn ntt_multiply(
    lhs: Vec256,
    rhs: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    // Compute the first term of the product
    let shuffle_with = mm256_set_epi8(
        15, 14, 11, 10, 7, 6, 3, 2, 13, 12, 9, 8, 5, 4, 1, 0, 15, 14, 11, 10, 7, 6, 3, 2, 13, 12,
        9, 8, 5, 4, 1, 0,
    );
    const PERMUTE_WITH: i32 = 0b11_01_10_00;

    // Prepare the left hand side
    let lhs_shuffled = mm256_shuffle_epi8(lhs, shuffle_with);
    let lhs_grouped = mm256_permute4x64_epi64::<{ PERMUTE_WITH }>(lhs_shuffled);

    let lhs_evens_128_ = mm256_castsi256_si128(lhs_grouped);
    let lhs_evens = mm256_cvtepi16_epi32(lhs_evens_128_);

    let lhs_odds_128_ = mm256_extracti128_si256::<1>(lhs_grouped);
    let lhs_odds = mm256_cvtepi16_epi32(lhs_odds_128_);

    // Prepare the right hand side
    let rhs_shuffled = mm256_shuffle_epi8(rhs, shuffle_with);
    let rhs_grouped = mm256_permute4x64_epi64::<{ PERMUTE_WITH }>(rhs_shuffled);

    let rhs_evens_128_ = mm256_castsi256_si128(rhs_grouped);
    let rhs_evens = mm256_cvtepi16_epi32(rhs_evens_128_);

    let rhs_odds_128_ = mm256_extracti128_si256::<1>(rhs_grouped);
    let rhs_odds = mm256_cvtepi16_epi32(rhs_odds_128_);

    // Start operating with them
    let left = mm256_mullo_epi32(lhs_evens, rhs_evens);

    let odd_products = mm256_mullo_epi32(lhs_odds, rhs_odds);
    let odd_products_reduced = arithmetic::montgomery_reduce_i32s(odd_products);
    // Naming the zeta multiplier vector (a no-op vs inlining it in `right`) lets the
    // functional proof reference it directly; rebuilding `set_epi32(neg ..)` inside the
    // proof block would re-trigger the `neg` subtyping in the impl's heavy VC.
    let zeta_multipliers = mm256_set_epi32(
        -(zeta3 as i32),
        zeta3 as i32,
        -(zeta2 as i32),
        zeta2 as i32,
        -(zeta1 as i32),
        zeta1 as i32,
        -(zeta0 as i32),
        zeta0 as i32,
    );
    let right = mm256_mullo_epi32(odd_products_reduced, zeta_multipliers);

    let products_left_raw = mm256_add_epi32(left, right);
    let products_left = arithmetic::montgomery_reduce_i32s(products_left_raw);

    // Compute the second term of the product
    let swap_with = mm256_set_epi8(
        13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8, 11, 10, 5, 4,
        7, 6, 1, 0, 3, 2,
    );
    let rhs_adjacent_swapped = mm256_shuffle_epi8(rhs, swap_with);
    let products_right_raw = mm256_madd_epi16(lhs, rhs_adjacent_swapped);
    let products_right_reduced = arithmetic::montgomery_reduce_i32s(products_right_raw);
    let products_right = mm256_slli_epi32::<16>(products_right_reduced);

    // Combine them into one vector
    let result = mm256_blend_epi16::<0b1_0_1_0_1_0_1_0>(products_left, products_right);
    proof!(
        r#"lemma_nttmul_zv zeta0 zeta1 zeta2 zeta3 ${zeta_multipliers};
        lemma_nttmul_main ${shuffle_with} ${swap_with} ${lhs} ${rhs} ${zeta_multipliers} zeta0 zeta1 zeta2 zeta3;
        assert (ZS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}));
        assert (Spec.Utils.ntt_multiply_butterfly_post
          (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${lhs}) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${rhs})
          (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}) zeta0 zeta1 zeta2 zeta3)"#
    );
    result
}
