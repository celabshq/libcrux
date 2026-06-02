use super::*;

// ─────────────────────────────────────────────────────────────────────────
// Forward-NTT layer-1 lemma library.  Same within-128-bit-lane shuffle_epi32
// architecture as the inverse layers, but the "add" operand is a montgomery
// residue (only known mod 3329), so the post-helper (`lemma_fwd_l1_post`)
// bridges via mod-add-distributivity (`lemma_modadd`); the subtraction in
// `ntt_spec` comes from the negated zetas entries.  `fwd_shuffle_*` /
// `lemma_fwd_l1_*` are this layer's copies (distinct names so they don't
// collide with the inverse-layer before-blocks later in the file).  FI/FS/FA
// are this file's module abbreviations (defined here, the first fn).
// ─────────────────────────────────────────────────────────────────────────
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
module FI = Libcrux_intrinsics.Avx2_extract
module FS = Spec.Utils
module FA = Libcrux_ml_kem.Vector.Avx2.Arithmetic

let fwd_shuffle_245 (vv: FI.t_Vec256) : Lemma
  (ensures (let r = FI.mm256_shuffle_epi32 (mk_i32 245) vv in
     FI.get_lane r 0 == FI.get_lane vv 2 /\ FI.get_lane r 1 == FI.get_lane vv 3 /\
     FI.get_lane r 2 == FI.get_lane vv 2 /\ FI.get_lane r 3 == FI.get_lane vv 3 /\
     FI.get_lane r 4 == FI.get_lane vv 6 /\ FI.get_lane r 5 == FI.get_lane vv 7 /\
     FI.get_lane r 6 == FI.get_lane vv 6 /\ FI.get_lane r 7 == FI.get_lane vv 7 /\
     FI.get_lane r 8 == FI.get_lane vv 10 /\ FI.get_lane r 9 == FI.get_lane vv 11 /\
     FI.get_lane r 10 == FI.get_lane vv 10 /\ FI.get_lane r 11 == FI.get_lane vv 11 /\
     FI.get_lane r 12 == FI.get_lane vv 14 /\ FI.get_lane r 13 == FI.get_lane vv 15 /\
     FI.get_lane r 14 == FI.get_lane vv 14 /\ FI.get_lane r 15 == FI.get_lane vv 15))
  = admit ()

let fwd_shuffle_160 (vv: FI.t_Vec256) : Lemma
  (ensures (let r = FI.mm256_shuffle_epi32 (mk_i32 160) vv in
     FI.get_lane r 0 == FI.get_lane vv 0 /\ FI.get_lane r 1 == FI.get_lane vv 1 /\
     FI.get_lane r 2 == FI.get_lane vv 0 /\ FI.get_lane r 3 == FI.get_lane vv 1 /\
     FI.get_lane r 4 == FI.get_lane vv 4 /\ FI.get_lane r 5 == FI.get_lane vv 5 /\
     FI.get_lane r 6 == FI.get_lane vv 4 /\ FI.get_lane r 7 == FI.get_lane vv 5 /\
     FI.get_lane r 8 == FI.get_lane vv 8 /\ FI.get_lane r 9 == FI.get_lane vv 9 /\
     FI.get_lane r 10 == FI.get_lane vv 8 /\ FI.get_lane r 11 == FI.get_lane vv 9 /\
     FI.get_lane r 12 == FI.get_lane vv 12 /\ FI.get_lane r 13 == FI.get_lane vv 13 /\
     FI.get_lane r 14 == FI.get_lane vv 12 /\ FI.get_lane r 15 == FI.get_lane vv 13))
  = admit ()

let fwd_shuffle_preserves_bound (c: i32) (vv: FI.t_Vec256) (b: nat) : Lemma
  (requires FS.is_i16b_array b (FI.vec256_as_i16x16 vv))
  (ensures FS.is_i16b_array b (FI.vec256_as_i16x16 (FI.mm256_shuffle_epi32 c vv)))
  = admit ()

#push-options "--z3rlimit 400 --split_queries always"
let lemma_fwd_l1_add (lhs rhs result: FI.t_Vec256) : Lemma
  (requires
     result == FI.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (7*3328) (FI.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (FI.vec256_as_i16x16 rhs))
  (ensures
     FS.is_i16b_array (8*3328) (FI.vec256_as_i16x16 result) /\
     (forall (i:nat). i < 16 ==>
        v (FI.get_lane result i) == v (FI.get_lane lhs i) + v (FI.get_lane rhs i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l1_resultv (vector lhs rhs result: FI.t_Vec256) : Lemma
  (requires
     result == FI.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (7*3328) (FI.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (FI.vec256_as_i16x16 rhs) /\
     FI.get_lane lhs 0 == FI.get_lane vector 0 /\ FI.get_lane lhs 1 == FI.get_lane vector 1 /\
     FI.get_lane lhs 2 == FI.get_lane vector 0 /\ FI.get_lane lhs 3 == FI.get_lane vector 1 /\
     FI.get_lane lhs 4 == FI.get_lane vector 4 /\ FI.get_lane lhs 5 == FI.get_lane vector 5 /\
     FI.get_lane lhs 6 == FI.get_lane vector 4 /\ FI.get_lane lhs 7 == FI.get_lane vector 5 /\
     FI.get_lane lhs 8 == FI.get_lane vector 8 /\ FI.get_lane lhs 9 == FI.get_lane vector 9 /\
     FI.get_lane lhs 10 == FI.get_lane vector 8 /\ FI.get_lane lhs 11 == FI.get_lane vector 9 /\
     FI.get_lane lhs 12 == FI.get_lane vector 12 /\ FI.get_lane lhs 13 == FI.get_lane vector 13 /\
     FI.get_lane lhs 14 == FI.get_lane vector 12 /\ FI.get_lane lhs 15 == FI.get_lane vector 13)
  (ensures
     FS.is_i16b_array (8*3328) (FI.vec256_as_i16x16 result) /\
     v (FI.get_lane result 0)  == v (FI.get_lane vector 0)  + v (FI.get_lane rhs 0) /\
     v (FI.get_lane result 1)  == v (FI.get_lane vector 1)  + v (FI.get_lane rhs 1) /\
     v (FI.get_lane result 2)  == v (FI.get_lane vector 0)  + v (FI.get_lane rhs 2) /\
     v (FI.get_lane result 3)  == v (FI.get_lane vector 1)  + v (FI.get_lane rhs 3) /\
     v (FI.get_lane result 4)  == v (FI.get_lane vector 4)  + v (FI.get_lane rhs 4) /\
     v (FI.get_lane result 5)  == v (FI.get_lane vector 5)  + v (FI.get_lane rhs 5) /\
     v (FI.get_lane result 6)  == v (FI.get_lane vector 4)  + v (FI.get_lane rhs 6) /\
     v (FI.get_lane result 7)  == v (FI.get_lane vector 5)  + v (FI.get_lane rhs 7) /\
     v (FI.get_lane result 8)  == v (FI.get_lane vector 8)  + v (FI.get_lane rhs 8) /\
     v (FI.get_lane result 9)  == v (FI.get_lane vector 9)  + v (FI.get_lane rhs 9) /\
     v (FI.get_lane result 10) == v (FI.get_lane vector 8)  + v (FI.get_lane rhs 10) /\
     v (FI.get_lane result 11) == v (FI.get_lane vector 9)  + v (FI.get_lane rhs 11) /\
     v (FI.get_lane result 12) == v (FI.get_lane vector 12) + v (FI.get_lane rhs 12) /\
     v (FI.get_lane result 13) == v (FI.get_lane vector 13) + v (FI.get_lane rhs 13) /\
     v (FI.get_lane result 14) == v (FI.get_lane vector 12) + v (FI.get_lane rhs 14) /\
     v (FI.get_lane result 15) == v (FI.get_lane vector 13) + v (FI.get_lane rhs 15))
  = lemma_fwd_l1_add lhs rhs result
#pop-options

let lemma_modadd (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a + r) % 3329 == (a + x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_add_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_add_distr a x 3329

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l1_post
    (vec rhs zetas result: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
  : Lemma
    (requires
      v (Seq.index result 0)  == v (Seq.index vec 0)  + v (Seq.index rhs 0) /\
      v (Seq.index result 1)  == v (Seq.index vec 1)  + v (Seq.index rhs 1) /\
      v (Seq.index result 2)  == v (Seq.index vec 0)  + v (Seq.index rhs 2) /\
      v (Seq.index result 3)  == v (Seq.index vec 1)  + v (Seq.index rhs 3) /\
      v (Seq.index result 4)  == v (Seq.index vec 4)  + v (Seq.index rhs 4) /\
      v (Seq.index result 5)  == v (Seq.index vec 5)  + v (Seq.index rhs 5) /\
      v (Seq.index result 6)  == v (Seq.index vec 4)  + v (Seq.index rhs 6) /\
      v (Seq.index result 7)  == v (Seq.index vec 5)  + v (Seq.index rhs 7) /\
      v (Seq.index result 8)  == v (Seq.index vec 8)  + v (Seq.index rhs 8) /\
      v (Seq.index result 9)  == v (Seq.index vec 9)  + v (Seq.index rhs 9) /\
      v (Seq.index result 10) == v (Seq.index vec 8)  + v (Seq.index rhs 10) /\
      v (Seq.index result 11) == v (Seq.index vec 9)  + v (Seq.index rhs 11) /\
      v (Seq.index result 12) == v (Seq.index vec 12) + v (Seq.index rhs 12) /\
      v (Seq.index result 13) == v (Seq.index vec 13) + v (Seq.index rhs 13) /\
      v (Seq.index result 14) == v (Seq.index vec 12) + v (Seq.index rhs 14) /\
      v (Seq.index result 15) == v (Seq.index vec 13) + v (Seq.index rhs 15) /\
      v (Seq.index rhs 0)  % 3329 == (v (Seq.index vec 2)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 1)  % 3329 == (v (Seq.index vec 3)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 2)  % 3329 == (v (Seq.index vec 2)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 3)  % 3329 == (v (Seq.index vec 3)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 4)  % 3329 == (v (Seq.index vec 6)  * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 5)  % 3329 == (v (Seq.index vec 7)  * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 6)  % 3329 == (v (Seq.index vec 6)  * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 7)  % 3329 == (v (Seq.index vec 7)  * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 8)  % 3329 == (v (Seq.index vec 10) * v zeta2 * 169) % 3329 /\
      v (Seq.index rhs 9)  % 3329 == (v (Seq.index vec 11) * v zeta2 * 169) % 3329 /\
      v (Seq.index rhs 10) % 3329 == (v (Seq.index vec 10) * (- v zeta2) * 169) % 3329 /\
      v (Seq.index rhs 11) % 3329 == (v (Seq.index vec 11) * (- v zeta2) * 169) % 3329 /\
      v (Seq.index rhs 12) % 3329 == (v (Seq.index vec 14) * v zeta3 * 169) % 3329 /\
      v (Seq.index rhs 13) % 3329 == (v (Seq.index vec 15) * v zeta3 * 169) % 3329 /\
      v (Seq.index rhs 14) % 3329 == (v (Seq.index vec 14) * (- v zeta3) * 169) % 3329 /\
      v (Seq.index rhs 15) % 3329 == (v (Seq.index vec 15) * (- v zeta3) * 169) % 3329 /\
      FS.is_i16b_array (8*3328) result)
    (ensures
      FS.is_i16b_array (8*3328) result /\
      FS.ntt_layer_1_butterfly_post vec result zeta0 zeta1 zeta2 zeta3)
  =
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 0)) (v (Seq.index vec 2) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 1)) (v (Seq.index vec 3) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 2)) (v (Seq.index vec 2) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 3)) (v (Seq.index vec 3) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 4)) (v (Seq.index rhs 4)) (v (Seq.index vec 6) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 5)) (v (Seq.index rhs 5)) (v (Seq.index vec 7) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 4)) (v (Seq.index rhs 6)) (v (Seq.index vec 6) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 5)) (v (Seq.index rhs 7)) (v (Seq.index vec 7) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 8)) (v (Seq.index vec 10) * v zeta2 * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 9)) (v (Seq.index vec 11) * v zeta2 * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 10)) (v (Seq.index vec 10) * (- v zeta2) * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 11)) (v (Seq.index vec 11) * (- v zeta2) * 169);
  lemma_modadd (v (Seq.index vec 12)) (v (Seq.index rhs 12)) (v (Seq.index vec 14) * v zeta3 * 169);
  lemma_modadd (v (Seq.index vec 13)) (v (Seq.index rhs 13)) (v (Seq.index vec 15) * v zeta3 * 169);
  lemma_modadd (v (Seq.index vec 12)) (v (Seq.index rhs 14)) (v (Seq.index vec 14) * (- v zeta3) * 169);
  lemma_modadd (v (Seq.index vec 13)) (v (Seq.index rhs 15)) (v (Seq.index vec 15) * (- v zeta3) * 169);
  reveal_opaque (`%FS.ntt_layer_1_butterfly_post)
    (FS.ntt_layer_1_butterfly_post vec)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (8*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    Spec.Utils.ntt_layer_1_butterfly_post
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) zeta0 zeta1 zeta2 zeta3"#))]
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
    hax_lib::fstar!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas}))"#
    );

    let rhs0 = mm256_shuffle_epi32::<0b11_11_01_01>(vector);
    hax_lib::fstar!(
        r#"fwd_shuffle_245 ${vector};
           fwd_shuffle_preserves_bound (mk_i32 245) ${vector} (7*3328)"#
    );
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs0, zetas);

    let lhs = mm256_shuffle_epi32::<0b10_10_00_00>(vector);
    hax_lib::fstar!(
        r#"fwd_shuffle_160 ${vector};
           fwd_shuffle_preserves_bound (mk_i32 160) ${vector} (7*3328)"#
    );

    let result = mm256_add_epi16(lhs, rhs);
    hax_lib::fstar!(
        r#"lemma_fwd_l1_resultv ${vector} ${lhs} ${rhs} ${result};
           assert (v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 0) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 1) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 2) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 3) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 4) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 5) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 6) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 7) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 8) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 9) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 10) == - v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 11) == - v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 12) == v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 13) == v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 14) == - v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 15) == - v zeta3);
           lemma_fwd_l1_post
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${rhs})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
             zeta0 zeta1 zeta2 zeta3"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1"#))]
pub(crate) fn ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let zetas = mm256_set_epi16(
        -zeta1, -zeta1, -zeta1, -zeta1, zeta1, zeta1, zeta1, zeta1, -zeta0, -zeta0, -zeta0, -zeta0,
        zeta0, zeta0, zeta0, zeta0,
    );

    let rhs = mm256_shuffle_epi32::<0b11_10_11_10>(vector);
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs, zetas);

    let lhs = mm256_shuffle_epi32::<0b01_00_01_00>(vector);

    mm256_add_epi16(lhs, rhs)
}

// ─────────────────────────────────────────────────────────────────────────
// Generic SIMD lane lemmas.  These bridge bit-level Vec256/Vec128 ops to
// their i16-array views.  The four mm256_* lemmas admit the abstract
// `vec256_as_i16x16` / `vec128_as_i16x8` definition (declared `val` in
// `Avx2_extract.fsti`); the two mm_* add/sub lemmas hold trivially.
// ─────────────────────────────────────────────────────────────────────────
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let lemma_mm256_castsi256_si128 (v: Libcrux_intrinsics.Avx2_extract.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                 (Libcrux_intrinsics.Avx2_extract.mm256_castsi256_si128 v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) i))
  = admit ()

let lemma_mm256_extracti128_si256_1 (v: Libcrux_intrinsics.Avx2_extract.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                 (Libcrux_intrinsics.Avx2_extract.mm256_extracti128_si256 (mk_i32 1) v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) (i + 8)))
  = admit ()

let lemma_mm256_castsi128_si256_lo (v: Libcrux_intrinsics.Avx2_extract.t_Vec128) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
                 (Libcrux_intrinsics.Avx2_extract.mm256_castsi128_si256 v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 v) i))
  = admit ()

let lemma_mm256_inserti128_si256_1
    (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    (b: Libcrux_intrinsics.Avx2_extract.t_Vec128) : Lemma
  (ensures
    (forall (i: nat). i < 8 ==>
      Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
                   (Libcrux_intrinsics.Avx2_extract.mm256_inserti128_si256 (mk_i32 1) a b)) i ==
      Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 a) i) /\
    (forall (i: nat). i < 8 ==>
      Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
                   (Libcrux_intrinsics.Avx2_extract.mm256_inserti128_si256 (mk_i32 1) a b)) (i + 8) ==
      Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 b) i))
  = admit ()

let lemma_add_i_128
    (lhs rhs: Libcrux_intrinsics.Avx2_extract.t_Vec128) (i: nat) : Lemma
  (requires i < 8 /\ Spec.Utils.is_intb (pow2 15 - 1)
                       (v (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i) +
                        v (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i)))
  (ensures v (add_mod (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i)) ==
           v (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i) +
           v (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i))
  [SMTPat (v (add_mod (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i)))]
  = ()

let lemma_sub_i_128
    (lhs rhs: Libcrux_intrinsics.Avx2_extract.t_Vec128) (i: nat) : Lemma
  (requires i < 8 /\ Spec.Utils.is_intb (pow2 15 - 1)
                       (v (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i) -
                        v (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i)))
  (ensures v (sub_mod (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i)) ==
           v (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i) -
           v (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i))
  [SMTPat (v (sub_mod (Libcrux_intrinsics.Avx2_extract.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_extract.get_lane128 rhs i)))]
  = ()
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
    Spec.Utils.is_i16b_array (5*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (6*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    (forall (i:nat). i < 8 ==>
       v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) i) % 3329 ==
         (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) +
          v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) (i+8)) * v zeta * 169) % 3329 /\
       v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) (i+8)) % 3329 ==
         (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i) -
          v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) (i+8)) * v zeta * 169) % 3329)
"#))]
pub(crate) fn ntt_layer_3_step(vector: Vec256, zeta: i16) -> Vec256 {
    let rhs = mm256_extracti128_si256::<1>(vector);
    hax_lib::fstar!(r#"lemma_mm256_extracti128_si256_1 ${vector}"#);
    // Now: forall i<8. get_lane128 rhs i = get_lane vector (i+8)

    let zetas_v128 = mm_set1_epi16(zeta);
    // Post: vec128_as_i16x8 zetas_v128 == Spec.Utils.create (sz 8) zeta
    // Pre for mont_mul: is_i16b_array 1664 zetas_v128 (since |zeta| <= 1664)
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${zetas_v128} i) == v zeta);
           assert (Spec.Utils.is_i16b_array 1664
                     (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 ${zetas_v128}))"#
    );

    let rhs = arithmetic::montgomery_multiply_m128i_by_constants(rhs, zetas_v128);
    // Post: is_i16b_array 3328 rhs /\
    //   forall i<8. v(get_lane128 rhs i) % 3329 ==
    //                  (v(get_lane vector (i+8)) * v zeta * 169) % 3329
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${rhs} i) % 3329 ==
                (v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) (i + 8))
                  * v zeta * 169) % 3329)"#
    );

    let lhs = mm256_castsi256_si128(vector);
    hax_lib::fstar!(r#"lemma_mm256_castsi256_si128 ${vector}"#);
    // Now: forall i<8. get_lane128 lhs i = get_lane vector i

    let lower_coefficients = mm_add_epi16(lhs, rhs);
    // Post: vec128_as_i16x8 lower == map2 (+.) ...
    // Use lemma_add_i_128 (SMTPat) to lift +. to +.
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${lower_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${lhs} i) +
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${rhs} i));
           assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${lower_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) i) +
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${rhs} i))"#
    );

    let upper_coefficients = mm_sub_epi16(lhs, rhs);
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${upper_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${lhs} i) -
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${rhs} i));
           assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${upper_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) i) -
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${rhs} i))"#
    );

    let combined_lo = mm256_castsi128_si256(lower_coefficients);
    hax_lib::fstar!(r#"lemma_mm256_castsi128_si256_lo ${lower_coefficients}"#);

    let combined = mm256_inserti128_si256::<1>(combined_lo, upper_coefficients);
    hax_lib::fstar!(r#"lemma_mm256_inserti128_si256_1 ${combined_lo} ${upper_coefficients}"#);
    // Final: forall i<8. combined[i] = lower[i], combined[i+8] = upper[i]
    hax_lib::fstar!(
        r#"
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_extract.get_lane (${combined}) i ==
                Libcrux_intrinsics.Avx2_extract.get_lane128 ${lower_coefficients} i);
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_extract.get_lane (${combined}) (i + 8) ==
                Libcrux_intrinsics.Avx2_extract.get_lane128 ${upper_coefficients} i)"#
    );
    combined
}

// ─────────────────────────────────────────────────────────────────────────
// Inverse-NTT layer-1 lemma library.  The leaf uses within-128-bit-lane
// `mm256_shuffle_epi32` + `mm256_mullo_epi16` + `mm256_blend_epi16`, none of
// which have an F* functional model — `lemma_shuffle_{245,160}` /
// `lemma_shuffle_preserves_bound` / `lemma_blend_204` axiomatize their per-lane
// behaviour (analogous to the layer-3 `mm256_*` lemmas above).  The map2/createi
// cost of the mullo+add chain is isolated to `lemma_inv_l1_sums` (paid ONCE,
// uniform forall); `lemma_inv_l1_sums_v` substitutes the shuffle/mult facts to
// express the sum lanes in terms of `vector`; `lemma_inv_l1_post` derives the
// trait `inv_ntt_layer_1_butterfly_post` over plain i16 arrays (no `mm256`
// terms, hence cascade-free).
// ─────────────────────────────────────────────────────────────────────────
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
module ZI = Libcrux_intrinsics.Avx2_extract
module ZS = Spec.Utils
module ZA = Libcrux_ml_kem.Vector.Avx2.Arithmetic

let lemma_shuffle_245 (vv: ZI.t_Vec256) : Lemma
  (ensures (let r = ZI.mm256_shuffle_epi32 (mk_i32 245) vv in
     ZI.get_lane r 0 == ZI.get_lane vv 2 /\ ZI.get_lane r 1 == ZI.get_lane vv 3 /\
     ZI.get_lane r 2 == ZI.get_lane vv 2 /\ ZI.get_lane r 3 == ZI.get_lane vv 3 /\
     ZI.get_lane r 4 == ZI.get_lane vv 6 /\ ZI.get_lane r 5 == ZI.get_lane vv 7 /\
     ZI.get_lane r 6 == ZI.get_lane vv 6 /\ ZI.get_lane r 7 == ZI.get_lane vv 7 /\
     ZI.get_lane r 8 == ZI.get_lane vv 10 /\ ZI.get_lane r 9 == ZI.get_lane vv 11 /\
     ZI.get_lane r 10 == ZI.get_lane vv 10 /\ ZI.get_lane r 11 == ZI.get_lane vv 11 /\
     ZI.get_lane r 12 == ZI.get_lane vv 14 /\ ZI.get_lane r 13 == ZI.get_lane vv 15 /\
     ZI.get_lane r 14 == ZI.get_lane vv 14 /\ ZI.get_lane r 15 == ZI.get_lane vv 15))
  = admit ()

let lemma_shuffle_160 (vv: ZI.t_Vec256) : Lemma
  (ensures (let r = ZI.mm256_shuffle_epi32 (mk_i32 160) vv in
     ZI.get_lane r 0 == ZI.get_lane vv 0 /\ ZI.get_lane r 1 == ZI.get_lane vv 1 /\
     ZI.get_lane r 2 == ZI.get_lane vv 0 /\ ZI.get_lane r 3 == ZI.get_lane vv 1 /\
     ZI.get_lane r 4 == ZI.get_lane vv 4 /\ ZI.get_lane r 5 == ZI.get_lane vv 5 /\
     ZI.get_lane r 6 == ZI.get_lane vv 4 /\ ZI.get_lane r 7 == ZI.get_lane vv 5 /\
     ZI.get_lane r 8 == ZI.get_lane vv 8 /\ ZI.get_lane r 9 == ZI.get_lane vv 9 /\
     ZI.get_lane r 10 == ZI.get_lane vv 8 /\ ZI.get_lane r 11 == ZI.get_lane vv 9 /\
     ZI.get_lane r 12 == ZI.get_lane vv 12 /\ ZI.get_lane r 13 == ZI.get_lane vv 13 /\
     ZI.get_lane r 14 == ZI.get_lane vv 12 /\ ZI.get_lane r 15 == ZI.get_lane vv 13))
  = admit ()

let lemma_shuffle_preserves_bound (c: i32) (vv: ZI.t_Vec256) (b: nat) : Lemma
  (requires ZS.is_i16b_array b (ZI.vec256_as_i16x16 vv))
  (ensures ZS.is_i16b_array b (ZI.vec256_as_i16x16 (ZI.mm256_shuffle_epi32 c vv)))
  = admit ()

let lemma_blend_204 (a b: ZI.t_Vec256) : Lemma
  (ensures (let r = ZI.mm256_blend_epi16 (mk_i32 204) a b in
     ZI.get_lane r 0 == ZI.get_lane a 0 /\ ZI.get_lane r 1 == ZI.get_lane a 1 /\
     ZI.get_lane r 2 == ZI.get_lane b 2 /\ ZI.get_lane r 3 == ZI.get_lane b 3 /\
     ZI.get_lane r 4 == ZI.get_lane a 4 /\ ZI.get_lane r 5 == ZI.get_lane a 5 /\
     ZI.get_lane r 6 == ZI.get_lane b 6 /\ ZI.get_lane r 7 == ZI.get_lane b 7 /\
     ZI.get_lane r 8 == ZI.get_lane a 8 /\ ZI.get_lane r 9 == ZI.get_lane a 9 /\
     ZI.get_lane r 10 == ZI.get_lane b 10 /\ ZI.get_lane r 11 == ZI.get_lane b 11 /\
     ZI.get_lane r 12 == ZI.get_lane a 12 /\ ZI.get_lane r 13 == ZI.get_lane a 13 /\
     ZI.get_lane r 14 == ZI.get_lane b 14 /\ ZI.get_lane r 15 == ZI.get_lane b 15))
  = admit ()

#push-options "--z3rlimit 400 --split_queries always"
let lemma_inv_l1_sums (lhs rhs0 mult rhs sum: ZI.t_Vec256) : Lemma
  (requires
     rhs == ZI.mm256_mullo_epi16 rhs0 mult /\
     sum == ZI.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array (4*3328) (ZI.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array (4*3328) (ZI.vec256_as_i16x16 rhs0) /\
     (forall (i:nat). i < 16 ==> (v (ZI.get_lane mult i) == 1 \/ v (ZI.get_lane mult i) == -1)))
  (ensures
     ZS.is_i16b_array 28296 (ZI.vec256_as_i16x16 sum) /\
     (forall (i:nat). i < 16 ==>
        v (ZI.get_lane sum i) ==
          v (ZI.get_lane lhs i) + v (ZI.get_lane rhs0 i) * v (ZI.get_lane mult i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_inv_l1_sums_v (vector lhs rhs0 mult rhs sum: ZI.t_Vec256) : Lemma
  (requires
     rhs == ZI.mm256_mullo_epi16 rhs0 mult /\
     sum == ZI.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array (4*3328) (ZI.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array (4*3328) (ZI.vec256_as_i16x16 rhs0) /\
     v (ZI.get_lane mult 0) == 1 /\ v (ZI.get_lane mult 1) == 1 /\
     v (ZI.get_lane mult 2) == -1 /\ v (ZI.get_lane mult 3) == -1 /\
     v (ZI.get_lane mult 4) == 1 /\ v (ZI.get_lane mult 5) == 1 /\
     v (ZI.get_lane mult 6) == -1 /\ v (ZI.get_lane mult 7) == -1 /\
     v (ZI.get_lane mult 8) == 1 /\ v (ZI.get_lane mult 9) == 1 /\
     v (ZI.get_lane mult 10) == -1 /\ v (ZI.get_lane mult 11) == -1 /\
     v (ZI.get_lane mult 12) == 1 /\ v (ZI.get_lane mult 13) == 1 /\
     v (ZI.get_lane mult 14) == -1 /\ v (ZI.get_lane mult 15) == -1 /\
     ZI.get_lane lhs 0 == ZI.get_lane vector 2 /\ ZI.get_lane lhs 1 == ZI.get_lane vector 3 /\
     ZI.get_lane lhs 2 == ZI.get_lane vector 2 /\ ZI.get_lane lhs 3 == ZI.get_lane vector 3 /\
     ZI.get_lane lhs 4 == ZI.get_lane vector 6 /\ ZI.get_lane lhs 5 == ZI.get_lane vector 7 /\
     ZI.get_lane lhs 6 == ZI.get_lane vector 6 /\ ZI.get_lane lhs 7 == ZI.get_lane vector 7 /\
     ZI.get_lane lhs 8 == ZI.get_lane vector 10 /\ ZI.get_lane lhs 9 == ZI.get_lane vector 11 /\
     ZI.get_lane lhs 10 == ZI.get_lane vector 10 /\ ZI.get_lane lhs 11 == ZI.get_lane vector 11 /\
     ZI.get_lane lhs 12 == ZI.get_lane vector 14 /\ ZI.get_lane lhs 13 == ZI.get_lane vector 15 /\
     ZI.get_lane lhs 14 == ZI.get_lane vector 14 /\ ZI.get_lane lhs 15 == ZI.get_lane vector 15 /\
     ZI.get_lane rhs0 0 == ZI.get_lane vector 0 /\ ZI.get_lane rhs0 1 == ZI.get_lane vector 1 /\
     ZI.get_lane rhs0 2 == ZI.get_lane vector 0 /\ ZI.get_lane rhs0 3 == ZI.get_lane vector 1 /\
     ZI.get_lane rhs0 4 == ZI.get_lane vector 4 /\ ZI.get_lane rhs0 5 == ZI.get_lane vector 5 /\
     ZI.get_lane rhs0 6 == ZI.get_lane vector 4 /\ ZI.get_lane rhs0 7 == ZI.get_lane vector 5 /\
     ZI.get_lane rhs0 8 == ZI.get_lane vector 8 /\ ZI.get_lane rhs0 9 == ZI.get_lane vector 9 /\
     ZI.get_lane rhs0 10 == ZI.get_lane vector 8 /\ ZI.get_lane rhs0 11 == ZI.get_lane vector 9 /\
     ZI.get_lane rhs0 12 == ZI.get_lane vector 12 /\ ZI.get_lane rhs0 13 == ZI.get_lane vector 13 /\
     ZI.get_lane rhs0 14 == ZI.get_lane vector 12 /\ ZI.get_lane rhs0 15 == ZI.get_lane vector 13)
  (ensures
     ZS.is_i16b_array 28296 (ZI.vec256_as_i16x16 sum) /\
     v (ZI.get_lane sum 0)  == v (ZI.get_lane vector 2)  + v (ZI.get_lane vector 0) /\
     v (ZI.get_lane sum 1)  == v (ZI.get_lane vector 3)  + v (ZI.get_lane vector 1) /\
     v (ZI.get_lane sum 2)  == v (ZI.get_lane vector 2)  - v (ZI.get_lane vector 0) /\
     v (ZI.get_lane sum 3)  == v (ZI.get_lane vector 3)  - v (ZI.get_lane vector 1) /\
     v (ZI.get_lane sum 4)  == v (ZI.get_lane vector 6)  + v (ZI.get_lane vector 4) /\
     v (ZI.get_lane sum 5)  == v (ZI.get_lane vector 7)  + v (ZI.get_lane vector 5) /\
     v (ZI.get_lane sum 6)  == v (ZI.get_lane vector 6)  - v (ZI.get_lane vector 4) /\
     v (ZI.get_lane sum 7)  == v (ZI.get_lane vector 7)  - v (ZI.get_lane vector 5) /\
     v (ZI.get_lane sum 8)  == v (ZI.get_lane vector 10) + v (ZI.get_lane vector 8) /\
     v (ZI.get_lane sum 9)  == v (ZI.get_lane vector 11) + v (ZI.get_lane vector 9) /\
     v (ZI.get_lane sum 10) == v (ZI.get_lane vector 10) - v (ZI.get_lane vector 8) /\
     v (ZI.get_lane sum 11) == v (ZI.get_lane vector 11) - v (ZI.get_lane vector 9) /\
     v (ZI.get_lane sum 12) == v (ZI.get_lane vector 14) + v (ZI.get_lane vector 12) /\
     v (ZI.get_lane sum 13) == v (ZI.get_lane vector 15) + v (ZI.get_lane vector 13) /\
     v (ZI.get_lane sum 14) == v (ZI.get_lane vector 14) - v (ZI.get_lane vector 12) /\
     v (ZI.get_lane sum 15) == v (ZI.get_lane vector 15) - v (ZI.get_lane vector 13))
  = lemma_inv_l1_sums lhs rhs0 mult rhs sum
#pop-options

#push-options "--z3rlimit 200 --split_queries always"
let lemma_inv_l1_post
    (vec sum sbar stz zetas res: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
  : Lemma
    (requires
      v (Seq.index sum 0)  == v (Seq.index vec 2)  + v (Seq.index vec 0) /\
      v (Seq.index sum 1)  == v (Seq.index vec 3)  + v (Seq.index vec 1) /\
      v (Seq.index sum 2)  == v (Seq.index vec 2)  - v (Seq.index vec 0) /\
      v (Seq.index sum 3)  == v (Seq.index vec 3)  - v (Seq.index vec 1) /\
      v (Seq.index sum 4)  == v (Seq.index vec 6)  + v (Seq.index vec 4) /\
      v (Seq.index sum 5)  == v (Seq.index vec 7)  + v (Seq.index vec 5) /\
      v (Seq.index sum 6)  == v (Seq.index vec 6)  - v (Seq.index vec 4) /\
      v (Seq.index sum 7)  == v (Seq.index vec 7)  - v (Seq.index vec 5) /\
      v (Seq.index sum 8)  == v (Seq.index vec 10) + v (Seq.index vec 8) /\
      v (Seq.index sum 9)  == v (Seq.index vec 11) + v (Seq.index vec 9) /\
      v (Seq.index sum 10) == v (Seq.index vec 10) - v (Seq.index vec 8) /\
      v (Seq.index sum 11) == v (Seq.index vec 11) - v (Seq.index vec 9) /\
      v (Seq.index sum 12) == v (Seq.index vec 14) + v (Seq.index vec 12) /\
      v (Seq.index sum 13) == v (Seq.index vec 15) + v (Seq.index vec 13) /\
      v (Seq.index sum 14) == v (Seq.index vec 14) - v (Seq.index vec 12) /\
      v (Seq.index sum 15) == v (Seq.index vec 15) - v (Seq.index vec 13) /\
      (forall (i:nat). i < 16 ==> v (Seq.index sbar i) % 3329 == v (Seq.index sum i) % 3329) /\
      (forall (i:nat). i < 16 ==>
         v (Seq.index stz i) % 3329 == (v (Seq.index sum i) * v (Seq.index zetas i) * 169) % 3329) /\
      v (Seq.index zetas 2) == v zeta0 /\ v (Seq.index zetas 3) == v zeta0 /\
      v (Seq.index zetas 6) == v zeta1 /\ v (Seq.index zetas 7) == v zeta1 /\
      v (Seq.index zetas 10) == v zeta2 /\ v (Seq.index zetas 11) == v zeta2 /\
      v (Seq.index zetas 14) == v zeta3 /\ v (Seq.index zetas 15) == v zeta3 /\
      Seq.index res 0 == Seq.index sbar 0 /\ Seq.index res 1 == Seq.index sbar 1 /\
      Seq.index res 2 == Seq.index stz 2 /\ Seq.index res 3 == Seq.index stz 3 /\
      Seq.index res 4 == Seq.index sbar 4 /\ Seq.index res 5 == Seq.index sbar 5 /\
      Seq.index res 6 == Seq.index stz 6 /\ Seq.index res 7 == Seq.index stz 7 /\
      Seq.index res 8 == Seq.index sbar 8 /\ Seq.index res 9 == Seq.index sbar 9 /\
      Seq.index res 10 == Seq.index stz 10 /\ Seq.index res 11 == Seq.index stz 11 /\
      Seq.index res 12 == Seq.index sbar 12 /\ Seq.index res 13 == Seq.index sbar 13 /\
      Seq.index res 14 == Seq.index stz 14 /\ Seq.index res 15 == Seq.index stz 15 /\
      ZS.is_i16b_array 3328 sbar /\ ZS.is_i16b_array 3328 stz)
    (ensures
      ZS.is_i16b_array 3328 res /\
      ZS.inv_ntt_layer_1_butterfly_post vec res zeta0 zeta1 zeta2 zeta3)
  =
  reveal_opaque (`%ZS.inv_ntt_layer_1_butterfly_post)
    (ZS.inv_ntt_layer_1_butterfly_post vec)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    Spec.Utils.inv_ntt_layer_1_butterfly_post
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) zeta0 zeta1 zeta2 zeta3"#))]
pub(crate) fn inv_ntt_layer_1_step(
    vector: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    let lhs = mm256_shuffle_epi32::<0b11_11_01_01>(vector);
    hax_lib::fstar!(
        r#"lemma_shuffle_245 ${vector};
           lemma_shuffle_preserves_bound (mk_i32 245) ${vector} (4*3328)"#
    );

    let rhs0 = mm256_shuffle_epi32::<0b10_10_00_00>(vector);
    hax_lib::fstar!(
        r#"lemma_shuffle_160 ${vector};
           lemma_shuffle_preserves_bound (mk_i32 160) ${vector} (4*3328)"#
    );

    let mult = mm256_set_epi16(-1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1);
    let rhs = mm256_mullo_epi16(rhs0, mult);

    let sum = mm256_add_epi16(lhs, rhs);
    hax_lib::fstar!(r#"lemma_inv_l1_sums_v ${vector} ${lhs} ${rhs0} ${mult} ${rhs} ${sum}"#);

    let zetas = mm256_set_epi16(
        zeta3, zeta3, 0, 0, zeta2, zeta2, 0, 0, zeta1, zeta1, 0, 0, zeta0, zeta0, 0, 0,
    );
    let sum_times_zetas = arithmetic::montgomery_multiply_by_constants(sum, zetas);

    let sum_reduced = arithmetic::barrett_reduce(sum);

    let result = mm256_blend_epi16::<0b1_1_0_0_1_1_0_0>(sum_reduced, sum_times_zetas);
    hax_lib::fstar!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas}));
           assert (v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 2) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 3) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 6) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 7) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 10) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 11) == v zeta2 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 14) == v zeta3 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 15) == v zeta3);
           lemma_blend_204 ${sum_reduced} ${sum_times_zetas};
           lemma_inv_l1_post
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${sum})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${sum_reduced})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${sum_times_zetas})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
             zeta0 zeta1 zeta2 zeta3"#
    );
    result
}

// ─────────────────────────────────────────────────────────────────────────
// Inverse-NTT layer-2 lemma library — same architecture as layer-1, with
// cross-128-bit-lane `mm256_permute4x64_epi64` (modelless: axiomatized per
// control by `lemma_permute_{245,160}`) and NO Barrett (the "add" outputs are
// the raw sum, bounded `2*3328`).  `lemma_inv_l2_sums` pays map2/createi once;
// `lemma_inv_l2_post` derives `inv_ntt_layer_2_butterfly_post` over plain i16
// arrays.
// ─────────────────────────────────────────────────────────────────────────
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let lemma_permute_245 (vv: ZI.t_Vec256) : Lemma
  (ensures (let r = ZI.mm256_permute4x64_epi64 (mk_i32 245) vv in
     ZI.get_lane r 0 == ZI.get_lane vv 4 /\ ZI.get_lane r 1 == ZI.get_lane vv 5 /\
     ZI.get_lane r 2 == ZI.get_lane vv 6 /\ ZI.get_lane r 3 == ZI.get_lane vv 7 /\
     ZI.get_lane r 4 == ZI.get_lane vv 4 /\ ZI.get_lane r 5 == ZI.get_lane vv 5 /\
     ZI.get_lane r 6 == ZI.get_lane vv 6 /\ ZI.get_lane r 7 == ZI.get_lane vv 7 /\
     ZI.get_lane r 8 == ZI.get_lane vv 12 /\ ZI.get_lane r 9 == ZI.get_lane vv 13 /\
     ZI.get_lane r 10 == ZI.get_lane vv 14 /\ ZI.get_lane r 11 == ZI.get_lane vv 15 /\
     ZI.get_lane r 12 == ZI.get_lane vv 12 /\ ZI.get_lane r 13 == ZI.get_lane vv 13 /\
     ZI.get_lane r 14 == ZI.get_lane vv 14 /\ ZI.get_lane r 15 == ZI.get_lane vv 15))
  = admit ()

let lemma_permute_160 (vv: ZI.t_Vec256) : Lemma
  (ensures (let r = ZI.mm256_permute4x64_epi64 (mk_i32 160) vv in
     ZI.get_lane r 0 == ZI.get_lane vv 0 /\ ZI.get_lane r 1 == ZI.get_lane vv 1 /\
     ZI.get_lane r 2 == ZI.get_lane vv 2 /\ ZI.get_lane r 3 == ZI.get_lane vv 3 /\
     ZI.get_lane r 4 == ZI.get_lane vv 0 /\ ZI.get_lane r 5 == ZI.get_lane vv 1 /\
     ZI.get_lane r 6 == ZI.get_lane vv 2 /\ ZI.get_lane r 7 == ZI.get_lane vv 3 /\
     ZI.get_lane r 8 == ZI.get_lane vv 8 /\ ZI.get_lane r 9 == ZI.get_lane vv 9 /\
     ZI.get_lane r 10 == ZI.get_lane vv 10 /\ ZI.get_lane r 11 == ZI.get_lane vv 11 /\
     ZI.get_lane r 12 == ZI.get_lane vv 8 /\ ZI.get_lane r 13 == ZI.get_lane vv 9 /\
     ZI.get_lane r 14 == ZI.get_lane vv 10 /\ ZI.get_lane r 15 == ZI.get_lane vv 11))
  = admit ()

let lemma_permute_preserves_bound (c: i32) (vv: ZI.t_Vec256) (b: nat) : Lemma
  (requires ZS.is_i16b_array b (ZI.vec256_as_i16x16 vv))
  (ensures ZS.is_i16b_array b (ZI.vec256_as_i16x16 (ZI.mm256_permute4x64_epi64 c vv)))
  = admit ()

let lemma_blend_240 (a b: ZI.t_Vec256) : Lemma
  (ensures (let r = ZI.mm256_blend_epi16 (mk_i32 240) a b in
     ZI.get_lane r 0 == ZI.get_lane a 0 /\ ZI.get_lane r 1 == ZI.get_lane a 1 /\
     ZI.get_lane r 2 == ZI.get_lane a 2 /\ ZI.get_lane r 3 == ZI.get_lane a 3 /\
     ZI.get_lane r 4 == ZI.get_lane b 4 /\ ZI.get_lane r 5 == ZI.get_lane b 5 /\
     ZI.get_lane r 6 == ZI.get_lane b 6 /\ ZI.get_lane r 7 == ZI.get_lane b 7 /\
     ZI.get_lane r 8 == ZI.get_lane a 8 /\ ZI.get_lane r 9 == ZI.get_lane a 9 /\
     ZI.get_lane r 10 == ZI.get_lane a 10 /\ ZI.get_lane r 11 == ZI.get_lane a 11 /\
     ZI.get_lane r 12 == ZI.get_lane b 12 /\ ZI.get_lane r 13 == ZI.get_lane b 13 /\
     ZI.get_lane r 14 == ZI.get_lane b 14 /\ ZI.get_lane r 15 == ZI.get_lane b 15))
  = admit ()

#push-options "--z3rlimit 400 --split_queries always"
let lemma_inv_l2_sums (lhs rhs0 mult rhs sum: ZI.t_Vec256) : Lemma
  (requires
     rhs == ZI.mm256_mullo_epi16 rhs0 mult /\
     sum == ZI.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array 3328 (ZI.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array 3328 (ZI.vec256_as_i16x16 rhs0) /\
     (forall (i:nat). i < 16 ==> (v (ZI.get_lane mult i) == 1 \/ v (ZI.get_lane mult i) == -1)))
  (ensures
     ZS.is_i16b_array (2*3328) (ZI.vec256_as_i16x16 sum) /\
     (forall (i:nat). i < 16 ==>
        v (ZI.get_lane sum i) ==
          v (ZI.get_lane lhs i) + v (ZI.get_lane rhs0 i) * v (ZI.get_lane mult i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_inv_l2_sums_v (vector lhs rhs0 mult rhs sum: ZI.t_Vec256) : Lemma
  (requires
     rhs == ZI.mm256_mullo_epi16 rhs0 mult /\
     sum == ZI.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array 3328 (ZI.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array 3328 (ZI.vec256_as_i16x16 rhs0) /\
     v (ZI.get_lane mult 0) == 1 /\ v (ZI.get_lane mult 1) == 1 /\
     v (ZI.get_lane mult 2) == 1 /\ v (ZI.get_lane mult 3) == 1 /\
     v (ZI.get_lane mult 4) == -1 /\ v (ZI.get_lane mult 5) == -1 /\
     v (ZI.get_lane mult 6) == -1 /\ v (ZI.get_lane mult 7) == -1 /\
     v (ZI.get_lane mult 8) == 1 /\ v (ZI.get_lane mult 9) == 1 /\
     v (ZI.get_lane mult 10) == 1 /\ v (ZI.get_lane mult 11) == 1 /\
     v (ZI.get_lane mult 12) == -1 /\ v (ZI.get_lane mult 13) == -1 /\
     v (ZI.get_lane mult 14) == -1 /\ v (ZI.get_lane mult 15) == -1 /\
     ZI.get_lane lhs 0 == ZI.get_lane vector 4 /\ ZI.get_lane lhs 1 == ZI.get_lane vector 5 /\
     ZI.get_lane lhs 2 == ZI.get_lane vector 6 /\ ZI.get_lane lhs 3 == ZI.get_lane vector 7 /\
     ZI.get_lane lhs 4 == ZI.get_lane vector 4 /\ ZI.get_lane lhs 5 == ZI.get_lane vector 5 /\
     ZI.get_lane lhs 6 == ZI.get_lane vector 6 /\ ZI.get_lane lhs 7 == ZI.get_lane vector 7 /\
     ZI.get_lane lhs 8 == ZI.get_lane vector 12 /\ ZI.get_lane lhs 9 == ZI.get_lane vector 13 /\
     ZI.get_lane lhs 10 == ZI.get_lane vector 14 /\ ZI.get_lane lhs 11 == ZI.get_lane vector 15 /\
     ZI.get_lane lhs 12 == ZI.get_lane vector 12 /\ ZI.get_lane lhs 13 == ZI.get_lane vector 13 /\
     ZI.get_lane lhs 14 == ZI.get_lane vector 14 /\ ZI.get_lane lhs 15 == ZI.get_lane vector 15 /\
     ZI.get_lane rhs0 0 == ZI.get_lane vector 0 /\ ZI.get_lane rhs0 1 == ZI.get_lane vector 1 /\
     ZI.get_lane rhs0 2 == ZI.get_lane vector 2 /\ ZI.get_lane rhs0 3 == ZI.get_lane vector 3 /\
     ZI.get_lane rhs0 4 == ZI.get_lane vector 0 /\ ZI.get_lane rhs0 5 == ZI.get_lane vector 1 /\
     ZI.get_lane rhs0 6 == ZI.get_lane vector 2 /\ ZI.get_lane rhs0 7 == ZI.get_lane vector 3 /\
     ZI.get_lane rhs0 8 == ZI.get_lane vector 8 /\ ZI.get_lane rhs0 9 == ZI.get_lane vector 9 /\
     ZI.get_lane rhs0 10 == ZI.get_lane vector 10 /\ ZI.get_lane rhs0 11 == ZI.get_lane vector 11 /\
     ZI.get_lane rhs0 12 == ZI.get_lane vector 8 /\ ZI.get_lane rhs0 13 == ZI.get_lane vector 9 /\
     ZI.get_lane rhs0 14 == ZI.get_lane vector 10 /\ ZI.get_lane rhs0 15 == ZI.get_lane vector 11)
  (ensures
     ZS.is_i16b_array (2*3328) (ZI.vec256_as_i16x16 sum) /\
     v (ZI.get_lane sum 0)  == v (ZI.get_lane vector 4)  + v (ZI.get_lane vector 0) /\
     v (ZI.get_lane sum 1)  == v (ZI.get_lane vector 5)  + v (ZI.get_lane vector 1) /\
     v (ZI.get_lane sum 2)  == v (ZI.get_lane vector 6)  + v (ZI.get_lane vector 2) /\
     v (ZI.get_lane sum 3)  == v (ZI.get_lane vector 7)  + v (ZI.get_lane vector 3) /\
     v (ZI.get_lane sum 4)  == v (ZI.get_lane vector 4)  - v (ZI.get_lane vector 0) /\
     v (ZI.get_lane sum 5)  == v (ZI.get_lane vector 5)  - v (ZI.get_lane vector 1) /\
     v (ZI.get_lane sum 6)  == v (ZI.get_lane vector 6)  - v (ZI.get_lane vector 2) /\
     v (ZI.get_lane sum 7)  == v (ZI.get_lane vector 7)  - v (ZI.get_lane vector 3) /\
     v (ZI.get_lane sum 8)  == v (ZI.get_lane vector 12) + v (ZI.get_lane vector 8) /\
     v (ZI.get_lane sum 9)  == v (ZI.get_lane vector 13) + v (ZI.get_lane vector 9) /\
     v (ZI.get_lane sum 10) == v (ZI.get_lane vector 14) + v (ZI.get_lane vector 10) /\
     v (ZI.get_lane sum 11) == v (ZI.get_lane vector 15) + v (ZI.get_lane vector 11) /\
     v (ZI.get_lane sum 12) == v (ZI.get_lane vector 12) - v (ZI.get_lane vector 8) /\
     v (ZI.get_lane sum 13) == v (ZI.get_lane vector 13) - v (ZI.get_lane vector 9) /\
     v (ZI.get_lane sum 14) == v (ZI.get_lane vector 14) - v (ZI.get_lane vector 10) /\
     v (ZI.get_lane sum 15) == v (ZI.get_lane vector 15) - v (ZI.get_lane vector 11))
  = lemma_inv_l2_sums lhs rhs0 mult rhs sum
#pop-options

#push-options "--z3rlimit 200 --split_queries always"
let lemma_inv_l2_post
    (vec sum stz zetas res: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
  : Lemma
    (requires
      v (Seq.index sum 0)  == v (Seq.index vec 4)  + v (Seq.index vec 0) /\
      v (Seq.index sum 1)  == v (Seq.index vec 5)  + v (Seq.index vec 1) /\
      v (Seq.index sum 2)  == v (Seq.index vec 6)  + v (Seq.index vec 2) /\
      v (Seq.index sum 3)  == v (Seq.index vec 7)  + v (Seq.index vec 3) /\
      v (Seq.index sum 4)  == v (Seq.index vec 4)  - v (Seq.index vec 0) /\
      v (Seq.index sum 5)  == v (Seq.index vec 5)  - v (Seq.index vec 1) /\
      v (Seq.index sum 6)  == v (Seq.index vec 6)  - v (Seq.index vec 2) /\
      v (Seq.index sum 7)  == v (Seq.index vec 7)  - v (Seq.index vec 3) /\
      v (Seq.index sum 8)  == v (Seq.index vec 12) + v (Seq.index vec 8) /\
      v (Seq.index sum 9)  == v (Seq.index vec 13) + v (Seq.index vec 9) /\
      v (Seq.index sum 10) == v (Seq.index vec 14) + v (Seq.index vec 10) /\
      v (Seq.index sum 11) == v (Seq.index vec 15) + v (Seq.index vec 11) /\
      v (Seq.index sum 12) == v (Seq.index vec 12) - v (Seq.index vec 8) /\
      v (Seq.index sum 13) == v (Seq.index vec 13) - v (Seq.index vec 9) /\
      v (Seq.index sum 14) == v (Seq.index vec 14) - v (Seq.index vec 10) /\
      v (Seq.index sum 15) == v (Seq.index vec 15) - v (Seq.index vec 11) /\
      (forall (i:nat). i < 16 ==>
         v (Seq.index stz i) % 3329 == (v (Seq.index sum i) * v (Seq.index zetas i) * 169) % 3329) /\
      v (Seq.index zetas 4) == v zeta0 /\ v (Seq.index zetas 5) == v zeta0 /\
      v (Seq.index zetas 6) == v zeta0 /\ v (Seq.index zetas 7) == v zeta0 /\
      v (Seq.index zetas 12) == v zeta1 /\ v (Seq.index zetas 13) == v zeta1 /\
      v (Seq.index zetas 14) == v zeta1 /\ v (Seq.index zetas 15) == v zeta1 /\
      Seq.index res 0 == Seq.index sum 0 /\ Seq.index res 1 == Seq.index sum 1 /\
      Seq.index res 2 == Seq.index sum 2 /\ Seq.index res 3 == Seq.index sum 3 /\
      Seq.index res 4 == Seq.index stz 4 /\ Seq.index res 5 == Seq.index stz 5 /\
      Seq.index res 6 == Seq.index stz 6 /\ Seq.index res 7 == Seq.index stz 7 /\
      Seq.index res 8 == Seq.index sum 8 /\ Seq.index res 9 == Seq.index sum 9 /\
      Seq.index res 10 == Seq.index sum 10 /\ Seq.index res 11 == Seq.index sum 11 /\
      Seq.index res 12 == Seq.index stz 12 /\ Seq.index res 13 == Seq.index stz 13 /\
      Seq.index res 14 == Seq.index stz 14 /\ Seq.index res 15 == Seq.index stz 15 /\
      ZS.is_i16b_array (2*3328) sum /\ ZS.is_i16b_array 3328 stz)
    (ensures
      ZS.is_i16b_array (2*3328) res /\
      ZS.inv_ntt_layer_2_butterfly_post vec res zeta0 zeta1)
  =
  reveal_opaque (`%ZS.inv_ntt_layer_2_butterfly_post)
    (ZS.inv_ntt_layer_2_butterfly_post vec)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (2*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    Spec.Utils.inv_ntt_layer_2_butterfly_post
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) zeta0 zeta1"#))]
pub(crate) fn inv_ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let lhs = mm256_permute4x64_epi64::<0b11_11_01_01>(vector);
    hax_lib::fstar!(
        r#"lemma_permute_245 ${vector};
           lemma_permute_preserves_bound (mk_i32 245) ${vector} 3328"#
    );

    let rhs0 = mm256_permute4x64_epi64::<0b10_10_00_00>(vector);
    hax_lib::fstar!(
        r#"lemma_permute_160 ${vector};
           lemma_permute_preserves_bound (mk_i32 160) ${vector} 3328"#
    );

    let mult = mm256_set_epi16(-1, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1, 1, 1, 1, 1);
    let rhs = mm256_mullo_epi16(rhs0, mult);

    let sum = mm256_add_epi16(lhs, rhs);
    hax_lib::fstar!(r#"lemma_inv_l2_sums_v ${vector} ${lhs} ${rhs0} ${mult} ${rhs} ${sum}"#);

    let zetas = mm256_set_epi16(
        zeta1, zeta1, zeta1, zeta1, 0, 0, 0, 0, zeta0, zeta0, zeta0, zeta0, 0, 0, 0, 0,
    );
    let sum_times_zetas = arithmetic::montgomery_multiply_by_constants(sum, zetas);

    let result = mm256_blend_epi16::<0b1_1_1_1_0_0_0_0>(sum, sum_times_zetas);
    hax_lib::fstar!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas}));
           assert (v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 4) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 5) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 6) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 7) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 12) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 13) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 14) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 15) == v zeta1);
           lemma_blend_240 ${sum} ${sum_times_zetas};
           lemma_inv_l2_post
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${sum})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${sum_times_zetas})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
             zeta0 zeta1"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
    Spec.Utils.is_i16b_array (2*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    (forall (i:nat). i < 8 ==>
       v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) i) % 3329 ==
         (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) (i+8)) +
          v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i)) % 3329 /\
       v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) (i+8)) % 3329 ==
         ((v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) (i+8)) -
           v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector}) i))
          * v zeta * 169) % 3329)
"#))]
pub(crate) fn inv_ntt_layer_3_step(vector: Vec256, zeta: i16) -> Vec256 {
    let lhs = mm256_extracti128_si256::<1>(vector);
    hax_lib::fstar!(r#"lemma_mm256_extracti128_si256_1 ${vector}"#);
    // forall i<8. get_lane128 lhs i = get_lane vector (i+8)

    let rhs = mm256_castsi256_si128(vector);
    hax_lib::fstar!(r#"lemma_mm256_castsi256_si128 ${vector}"#);
    // forall i<8. get_lane128 rhs i = get_lane vector i

    let lower_coefficients = mm_add_epi16(lhs, rhs);
    // mm_add_epi16 post + lemma_add_i_128 (SMTPat) lift +. → +
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${lower_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) (i + 8)) +
                v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) i))"#
    );

    let upper_coefficients = mm_sub_epi16(lhs, rhs);
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${upper_coefficients} i) ==
                v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) (i + 8)) -
                v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) i))"#
    );

    let zetas_v128 = mm_set1_epi16(zeta);
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${zetas_v128} i) == v zeta);
           assert (Spec.Utils.is_i16b_array 1664
                     (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 ${zetas_v128}))"#
    );

    let upper_coefficients =
        arithmetic::montgomery_multiply_m128i_by_constants(upper_coefficients, zetas_v128);
    // Post: is_i16b_array 3328 upper_coefficients /\
    //   forall i<8. v(upper[i]) % 3329 ==
    //               (v(vec[i+8]) - v(vec[i])) * v zeta * 169 % 3329
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 8 ==>
                v (Libcrux_intrinsics.Avx2_extract.get_lane128 ${upper_coefficients} i) % 3329 ==
                ((v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) (i + 8)) -
                  v (Libcrux_intrinsics.Avx2_extract.get_lane (${vector}) i))
                 * v zeta * 169) % 3329)"#
    );

    let combined_lo = mm256_castsi128_si256(lower_coefficients);
    hax_lib::fstar!(r#"lemma_mm256_castsi128_si256_lo ${lower_coefficients}"#);

    let combined = mm256_inserti128_si256::<1>(combined_lo, upper_coefficients);
    hax_lib::fstar!(r#"lemma_mm256_inserti128_si256_1 ${combined_lo} ${upper_coefficients}"#);
    // forall i<8. combined[i]   = lower[i]
    //             combined[i+8] = upper[i]
    hax_lib::fstar!(
        r#"
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_extract.get_lane (${combined}) i ==
                Libcrux_intrinsics.Avx2_extract.get_lane128 ${lower_coefficients} i);
        assert (forall (i:nat). i < 8 ==>
                Libcrux_intrinsics.Avx2_extract.get_lane (${combined}) (i + 8) ==
                Libcrux_intrinsics.Avx2_extract.get_lane128 ${upper_coefficients} i)"#
    );
    combined
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3"#))]
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
    let lhs_shuffled = mm256_permute4x64_epi64::<{ PERMUTE_WITH }>(lhs_shuffled);

    let lhs_evens = mm256_castsi256_si128(lhs_shuffled);
    let lhs_evens = mm256_cvtepi16_epi32(lhs_evens);

    let lhs_odds = mm256_extracti128_si256::<1>(lhs_shuffled);
    let lhs_odds = mm256_cvtepi16_epi32(lhs_odds);

    // Prepare the right hand side
    let rhs_shuffled = mm256_shuffle_epi8(rhs, shuffle_with);
    let rhs_shuffled = mm256_permute4x64_epi64::<{ PERMUTE_WITH }>(rhs_shuffled);

    let rhs_evens = mm256_castsi256_si128(rhs_shuffled);
    let rhs_evens = mm256_cvtepi16_epi32(rhs_evens);

    let rhs_odds = mm256_extracti128_si256::<1>(rhs_shuffled);
    let rhs_odds = mm256_cvtepi16_epi32(rhs_odds);

    // Start operating with them
    let left = mm256_mullo_epi32(lhs_evens, rhs_evens);

    let right = mm256_mullo_epi32(lhs_odds, rhs_odds);
    let right = arithmetic::montgomery_reduce_i32s(right);
    let right = mm256_mullo_epi32(
        right,
        mm256_set_epi32(
            -(zeta3 as i32),
            zeta3 as i32,
            -(zeta2 as i32),
            zeta2 as i32,
            -(zeta1 as i32),
            zeta1 as i32,
            -(zeta0 as i32),
            zeta0 as i32,
        ),
    );

    let products_left = mm256_add_epi32(left, right);
    let products_left = arithmetic::montgomery_reduce_i32s(products_left);

    // Compute the second term of the product
    let rhs_adjacent_swapped = mm256_shuffle_epi8(
        rhs,
        mm256_set_epi8(
            13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8, 11, 10, 5,
            4, 7, 6, 1, 0, 3, 2,
        ),
    );
    let products_right = mm256_madd_epi16(lhs, rhs_adjacent_swapped);
    let products_right = arithmetic::montgomery_reduce_i32s(products_right);
    let products_right = mm256_slli_epi32::<16>(products_right);

    // Combine them into one vector
    mm256_blend_epi16::<0b1_0_1_0_1_0_1_0>(products_left, products_right)
}
