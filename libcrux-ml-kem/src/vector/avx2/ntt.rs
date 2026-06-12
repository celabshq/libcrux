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

// Per-lane lifting lemmas: confine the map2/createi reasoning to these two
// once-proven bodies (verify at module-default rlimit, no cascade), so the
// *_sums helpers' lane foralls reason over clean `+.`/`mul_mod` equalities.
// NO SMTPat — global triggers regressed the forward ntt_layer_1/2 leaves
// ("incomplete quantifiers" under 0.3.7); these are called explicitly from
// the forall-aux inside lemma_inv_l2_sums instead (createi paid once, here).
let lemma_get_lane_add (a b: FI.t_Vec256) (i:nat{i < 16}) : Lemma
  (ensures FI.get_lane (FI.mm256_add_epi16 a b) i == (FI.get_lane a i) +. (FI.get_lane b i))
  = ()

let lemma_get_lane_mullo (a b: FI.t_Vec256) (i:nat{i < 16}) : Lemma
  (ensures FI.get_lane (FI.mm256_mullo_epi16 a b) i == mul_mod (FI.get_lane a i) (FI.get_lane b i))
  = ()

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

// Forward-NTT layer-2: same mod-add-distributivity recipe as forward layer-1,
// with shuffle_epi32 controls 238/68 and len-4 pairs.  Reuses FI/FS/FA +
// lemma_modadd from the forward-layer-1 before-block (earlier in this file);
// fwd2_-prefixed names avoid collision.
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let fwd2_shuffle_238 (vv: FI.t_Vec256) : Lemma
  (ensures (let r = FI.mm256_shuffle_epi32 (mk_i32 238) vv in
     FI.get_lane r 0 == FI.get_lane vv 4 /\ FI.get_lane r 1 == FI.get_lane vv 5 /\
     FI.get_lane r 2 == FI.get_lane vv 6 /\ FI.get_lane r 3 == FI.get_lane vv 7 /\
     FI.get_lane r 4 == FI.get_lane vv 4 /\ FI.get_lane r 5 == FI.get_lane vv 5 /\
     FI.get_lane r 6 == FI.get_lane vv 6 /\ FI.get_lane r 7 == FI.get_lane vv 7 /\
     FI.get_lane r 8 == FI.get_lane vv 12 /\ FI.get_lane r 9 == FI.get_lane vv 13 /\
     FI.get_lane r 10 == FI.get_lane vv 14 /\ FI.get_lane r 11 == FI.get_lane vv 15 /\
     FI.get_lane r 12 == FI.get_lane vv 12 /\ FI.get_lane r 13 == FI.get_lane vv 13 /\
     FI.get_lane r 14 == FI.get_lane vv 14 /\ FI.get_lane r 15 == FI.get_lane vv 15))
  = admit ()

let fwd2_shuffle_68 (vv: FI.t_Vec256) : Lemma
  (ensures (let r = FI.mm256_shuffle_epi32 (mk_i32 68) vv in
     FI.get_lane r 0 == FI.get_lane vv 0 /\ FI.get_lane r 1 == FI.get_lane vv 1 /\
     FI.get_lane r 2 == FI.get_lane vv 2 /\ FI.get_lane r 3 == FI.get_lane vv 3 /\
     FI.get_lane r 4 == FI.get_lane vv 0 /\ FI.get_lane r 5 == FI.get_lane vv 1 /\
     FI.get_lane r 6 == FI.get_lane vv 2 /\ FI.get_lane r 7 == FI.get_lane vv 3 /\
     FI.get_lane r 8 == FI.get_lane vv 8 /\ FI.get_lane r 9 == FI.get_lane vv 9 /\
     FI.get_lane r 10 == FI.get_lane vv 10 /\ FI.get_lane r 11 == FI.get_lane vv 11 /\
     FI.get_lane r 12 == FI.get_lane vv 8 /\ FI.get_lane r 13 == FI.get_lane vv 9 /\
     FI.get_lane r 14 == FI.get_lane vv 10 /\ FI.get_lane r 15 == FI.get_lane vv 11))
  = admit ()

let fwd2_shuffle_preserves_bound (c: i32) (vv: FI.t_Vec256) (b: nat) : Lemma
  (requires FS.is_i16b_array b (FI.vec256_as_i16x16 vv))
  (ensures FS.is_i16b_array b (FI.vec256_as_i16x16 (FI.mm256_shuffle_epi32 c vv)))
  = admit ()

#push-options "--z3rlimit 400 --split_queries always"
let lemma_fwd_l2_add (lhs rhs result: FI.t_Vec256) : Lemma
  (requires
     result == FI.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (6*3328) (FI.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (FI.vec256_as_i16x16 rhs))
  (ensures
     FS.is_i16b_array (7*3328) (FI.vec256_as_i16x16 result) /\
     (forall (i:nat). i < 16 ==>
        v (FI.get_lane result i) == v (FI.get_lane lhs i) + v (FI.get_lane rhs i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l2_resultv (vector lhs rhs result: FI.t_Vec256) : Lemma
  (requires
     result == FI.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (6*3328) (FI.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (FI.vec256_as_i16x16 rhs) /\
     FI.get_lane lhs 0 == FI.get_lane vector 0 /\ FI.get_lane lhs 1 == FI.get_lane vector 1 /\
     FI.get_lane lhs 2 == FI.get_lane vector 2 /\ FI.get_lane lhs 3 == FI.get_lane vector 3 /\
     FI.get_lane lhs 4 == FI.get_lane vector 0 /\ FI.get_lane lhs 5 == FI.get_lane vector 1 /\
     FI.get_lane lhs 6 == FI.get_lane vector 2 /\ FI.get_lane lhs 7 == FI.get_lane vector 3 /\
     FI.get_lane lhs 8 == FI.get_lane vector 8 /\ FI.get_lane lhs 9 == FI.get_lane vector 9 /\
     FI.get_lane lhs 10 == FI.get_lane vector 10 /\ FI.get_lane lhs 11 == FI.get_lane vector 11 /\
     FI.get_lane lhs 12 == FI.get_lane vector 8 /\ FI.get_lane lhs 13 == FI.get_lane vector 9 /\
     FI.get_lane lhs 14 == FI.get_lane vector 10 /\ FI.get_lane lhs 15 == FI.get_lane vector 11)
  (ensures
     FS.is_i16b_array (7*3328) (FI.vec256_as_i16x16 result) /\
     v (FI.get_lane result 0)  == v (FI.get_lane vector 0)  + v (FI.get_lane rhs 0) /\
     v (FI.get_lane result 1)  == v (FI.get_lane vector 1)  + v (FI.get_lane rhs 1) /\
     v (FI.get_lane result 2)  == v (FI.get_lane vector 2)  + v (FI.get_lane rhs 2) /\
     v (FI.get_lane result 3)  == v (FI.get_lane vector 3)  + v (FI.get_lane rhs 3) /\
     v (FI.get_lane result 4)  == v (FI.get_lane vector 0)  + v (FI.get_lane rhs 4) /\
     v (FI.get_lane result 5)  == v (FI.get_lane vector 1)  + v (FI.get_lane rhs 5) /\
     v (FI.get_lane result 6)  == v (FI.get_lane vector 2)  + v (FI.get_lane rhs 6) /\
     v (FI.get_lane result 7)  == v (FI.get_lane vector 3)  + v (FI.get_lane rhs 7) /\
     v (FI.get_lane result 8)  == v (FI.get_lane vector 8)  + v (FI.get_lane rhs 8) /\
     v (FI.get_lane result 9)  == v (FI.get_lane vector 9)  + v (FI.get_lane rhs 9) /\
     v (FI.get_lane result 10) == v (FI.get_lane vector 10) + v (FI.get_lane rhs 10) /\
     v (FI.get_lane result 11) == v (FI.get_lane vector 11) + v (FI.get_lane rhs 11) /\
     v (FI.get_lane result 12) == v (FI.get_lane vector 8)  + v (FI.get_lane rhs 12) /\
     v (FI.get_lane result 13) == v (FI.get_lane vector 9)  + v (FI.get_lane rhs 13) /\
     v (FI.get_lane result 14) == v (FI.get_lane vector 10) + v (FI.get_lane rhs 14) /\
     v (FI.get_lane result 15) == v (FI.get_lane vector 11) + v (FI.get_lane rhs 15))
  = lemma_fwd_l2_add lhs rhs result
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l2_post
    (vec rhs zetas result: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
  : Lemma
    (requires
      v (Seq.index result 0)  == v (Seq.index vec 0)  + v (Seq.index rhs 0) /\
      v (Seq.index result 1)  == v (Seq.index vec 1)  + v (Seq.index rhs 1) /\
      v (Seq.index result 2)  == v (Seq.index vec 2)  + v (Seq.index rhs 2) /\
      v (Seq.index result 3)  == v (Seq.index vec 3)  + v (Seq.index rhs 3) /\
      v (Seq.index result 4)  == v (Seq.index vec 0)  + v (Seq.index rhs 4) /\
      v (Seq.index result 5)  == v (Seq.index vec 1)  + v (Seq.index rhs 5) /\
      v (Seq.index result 6)  == v (Seq.index vec 2)  + v (Seq.index rhs 6) /\
      v (Seq.index result 7)  == v (Seq.index vec 3)  + v (Seq.index rhs 7) /\
      v (Seq.index result 8)  == v (Seq.index vec 8)  + v (Seq.index rhs 8) /\
      v (Seq.index result 9)  == v (Seq.index vec 9)  + v (Seq.index rhs 9) /\
      v (Seq.index result 10) == v (Seq.index vec 10) + v (Seq.index rhs 10) /\
      v (Seq.index result 11) == v (Seq.index vec 11) + v (Seq.index rhs 11) /\
      v (Seq.index result 12) == v (Seq.index vec 8)  + v (Seq.index rhs 12) /\
      v (Seq.index result 13) == v (Seq.index vec 9)  + v (Seq.index rhs 13) /\
      v (Seq.index result 14) == v (Seq.index vec 10) + v (Seq.index rhs 14) /\
      v (Seq.index result 15) == v (Seq.index vec 11) + v (Seq.index rhs 15) /\
      v (Seq.index rhs 0)  % 3329 == (v (Seq.index vec 4)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 1)  % 3329 == (v (Seq.index vec 5)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 2)  % 3329 == (v (Seq.index vec 6)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 3)  % 3329 == (v (Seq.index vec 7)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 4)  % 3329 == (v (Seq.index vec 4)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 5)  % 3329 == (v (Seq.index vec 5)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 6)  % 3329 == (v (Seq.index vec 6)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 7)  % 3329 == (v (Seq.index vec 7)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 8)  % 3329 == (v (Seq.index vec 12) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 9)  % 3329 == (v (Seq.index vec 13) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 10) % 3329 == (v (Seq.index vec 14) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 11) % 3329 == (v (Seq.index vec 15) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 12) % 3329 == (v (Seq.index vec 12) * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 13) % 3329 == (v (Seq.index vec 13) * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 14) % 3329 == (v (Seq.index vec 14) * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 15) % 3329 == (v (Seq.index vec 15) * (- v zeta1) * 169) % 3329 /\
      FS.is_i16b_array (7*3328) result)
    (ensures
      FS.is_i16b_array (7*3328) result /\
      FS.ntt_layer_2_butterfly_post vec result zeta0 zeta1)
  =
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 0)) (v (Seq.index vec 4) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 1)) (v (Seq.index vec 5) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 2)) (v (Seq.index rhs 2)) (v (Seq.index vec 6) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 3)) (v (Seq.index rhs 3)) (v (Seq.index vec 7) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 4)) (v (Seq.index vec 4) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 5)) (v (Seq.index vec 5) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 2)) (v (Seq.index rhs 6)) (v (Seq.index vec 6) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 3)) (v (Seq.index rhs 7)) (v (Seq.index vec 7) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 8)) (v (Seq.index vec 12) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 9)) (v (Seq.index vec 13) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 10)) (v (Seq.index rhs 10)) (v (Seq.index vec 14) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 11)) (v (Seq.index rhs 11)) (v (Seq.index vec 15) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 12)) (v (Seq.index vec 12) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 13)) (v (Seq.index vec 13) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 10)) (v (Seq.index rhs 14)) (v (Seq.index vec 14) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 11)) (v (Seq.index rhs 15)) (v (Seq.index vec 15) * (- v zeta1) * 169);
  reveal_opaque (`%FS.ntt_layer_2_butterfly_post)
    (FS.ntt_layer_2_butterfly_post vec)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b_array (6*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    Spec.Utils.ntt_layer_2_butterfly_post
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) zeta0 zeta1"#))]
pub(crate) fn ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let zetas = mm256_set_epi16(
        -zeta1, -zeta1, -zeta1, -zeta1, zeta1, zeta1, zeta1, zeta1, -zeta0, -zeta0, -zeta0, -zeta0,
        zeta0, zeta0, zeta0, zeta0,
    );
    hax_lib::fstar!(
        r#"assert (Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas}))"#
    );

    let rhs0 = mm256_shuffle_epi32::<0b11_10_11_10>(vector);
    hax_lib::fstar!(
        r#"fwd2_shuffle_238 ${vector};
           fwd2_shuffle_preserves_bound (mk_i32 238) ${vector} (6*3328)"#
    );
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs0, zetas);

    let lhs = mm256_shuffle_epi32::<0b01_00_01_00>(vector);
    hax_lib::fstar!(
        r#"fwd2_shuffle_68 ${vector};
           fwd2_shuffle_preserves_bound (mk_i32 68) ${vector} (6*3328)"#
    );

    let result = mm256_add_epi16(lhs, rhs);
    hax_lib::fstar!(
        r#"lemma_fwd_l2_resultv ${vector} ${lhs} ${rhs} ${result};
           assert (v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 0) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 1) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 2) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 3) == v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 4) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 5) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 6) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 7) == - v zeta0 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 8) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 9) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 10) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 11) == v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 12) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 13) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 14) == - v zeta1 /\
                   v (Libcrux_intrinsics.Avx2_extract.get_lane ${zetas} 15) == - v zeta1);
           lemma_fwd_l2_post
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${rhs})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${zetas})
             (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
             zeta0 zeta1"#
    );
    result
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
  = let aux (i: nat {i < 8})
      : Lemma (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                            (Libcrux_intrinsics.Avx2_extract.mm256_castsi256_si128 v)) i ==
               Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) i) =
      let a = Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                           (Libcrux_intrinsics.Avx2_extract.mm256_castsi256_si128 v)) i in
      let b = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) i in
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
        Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec128_as_i16x8_lemma
          (Libcrux_intrinsics.Avx2_extract.mm256_castsi256_si128 v) 16 k;
        Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 k;
        assert (k / 16 == i);
        assert (k % 16 == nthv);
        assert (Libcrux_intrinsics.Avx2_extract.mm256_castsi256_si128 v k == v k)
      in
      Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in
    Classical.forall_intro aux

let lemma_mm256_extracti128_si256_1 (v: Libcrux_intrinsics.Avx2_extract.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                 (Libcrux_intrinsics.Avx2_extract.mm256_extracti128_si256 (mk_i32 1) v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) (i + 8)))
  = let aux (i: nat {i < 8})
      : Lemma (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                            (Libcrux_intrinsics.Avx2_extract.mm256_extracti128_si256 (mk_i32 1) v)) i ==
               Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) (i + 8)) =
      let a = Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8
                           (Libcrux_intrinsics.Avx2_extract.mm256_extracti128_si256 (mk_i32 1) v)) i in
      let b = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 v) (i + 8) in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        FStar.Math.Lemmas.lemma_mult_le_left 16 (i + 8) 15;
        let k : nat = 16 * i + nthv in
        let k' : nat = 16 * (i + 8) + nthv in
        assert (k < 128);
        assert (k' < 256);
        assert (k' == k + 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        FStar.Math.Lemmas.lemma_div_plus nthv (i + 8) 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv (i + 8) 16;
        Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec128_as_i16x8_lemma
          (Libcrux_intrinsics.Avx2_extract.mm256_extracti128_si256 (mk_i32 1) v) 16 k;
        Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 k';
        assert (k / 16 == i);
        assert (k % 16 == nthv);
        assert (k' / 16 == i + 8);
        assert (k' % 16 == nthv);
        assert (Libcrux_intrinsics.Avx2_extract.mm256_extracti128_si256 (mk_i32 1) v k == v (k + 128))
      in
      Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in
    Classical.forall_intro aux

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

// Createi-free under 0.3.7: a per-lane forall-aux calls lemma_get_lane_mullo /
// lemma_get_lane_add (above) explicitly, paying the map2/createi reasoning once
// inside those two bodies (no SMTPat — see the note at their definition); with
// the per-lane bounds in hand, Z3's native i16 model lifts `mul_mod`/`+.` to
// integer `*`/`+` (same as lemma_inv_l1_sums). Classical.forall_intro assembles
// the lane forall; the array bound follows per-lane.
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
  = let aux (i:nat{i < 16}) : Lemma
        (ensures
           ZS.is_intb (2*3328) (v (ZI.get_lane sum i)) /\
           v (ZI.get_lane sum i) ==
             v (ZI.get_lane lhs i) + v (ZI.get_lane rhs0 i) * v (ZI.get_lane mult i)) =
      // per-lane mullo/add facts (createi confined to these two bodies, no SMTPat):
      lemma_get_lane_mullo rhs0 mult i;   // get_lane rhs i == mul_mod (get_lane rhs0 i) (get_lane mult i)
      lemma_get_lane_add lhs rhs i;       // get_lane sum i == get_lane lhs i +. get_lane rhs i
      // per-lane bounds: get_lane _ i == Seq.index (vec256_as_i16x16 _) i (definitional),
      // so the requires' is_i16b_array foralls instantiate here; with these bounds Z3's
      // native i16 model lifts mul_mod / +. to integer * / + (same as lemma_inv_l1_sums).
      assert (ZS.is_intb 3328 (v (ZI.get_lane lhs i)));
      assert (ZS.is_intb 3328 (v (ZI.get_lane rhs0 i)));
      // re-assert the mult requires-forall: under --split_queries always this
      // disjunction-bodied forall is otherwise pruned from this sub-query's
      // hypotheses, so the instantiation at `i` would not fire.
      assert (forall (j:nat). j < 16 ==> (v (ZI.get_lane mult j) == 1 \/ v (ZI.get_lane mult j) == -1));
      assert (v (ZI.get_lane mult i) == 1 \/ v (ZI.get_lane mult i) == -1)
    in
    Classical.forall_intro aux
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
  = lemma_inv_l2_sums lhs rhs0 mult rhs sum;
    // re-assert l2_sums' post forall to pull it into each per-lane sub-query
    // under --split_queries always (else the lane instantiations do not fire on 0.3.7).
    assert (forall (i:nat). i < 16 ==>
        v (ZI.get_lane sum i) ==
          v (ZI.get_lane lhs i) + v (ZI.get_lane rhs0 i) * v (ZI.get_lane mult i))
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
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"
// ─────────────────────────────────────────────────────────────────────────
// ntt_multiply lemma library — ground-literal architecture, v12.
//
// Per-lane admitted axioms (quantifier-free); helpers and one main Lemma
// (plain-implication encoding — large Pure bodies bury hypotheses behind a
// unit-refinement chain Z3 cannot thread).  The grouping shuffle mask is
// threaded everywhere as a FREE parameter `m` with `m == <inline mask>`
// as a requires: BitVec closure terms admit no first-order congruence, so
// the mask equality must only ever be PROPAGATED as a hypothesis, never
// re-derived; the function instantiates `m := shuffle_with`, putting every
// conclusion directly on its own spine terms.  Cross-validated against a
// bit-exact simulation (2000 random trials).
// ─────────────────────────────────────────────────────────────────────────

(* Evens/odds grouping shuffle at the concrete mask (passed as `m`):
   out half-lane k takes input half-lane sigma(k), sigma = [0,2,4,6,1,3,5,7]. *)
let lemma_nttmul_shuffle_group_lane (vv m: ZI.t_Vec256) (k: nat{k < 16}) : Lemma
  (requires m ==
      (ZI.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)))
  (ensures ZI.get_lane (ZI.mm256_shuffle_epi8 vv m) k ==
   ZI.get_lane vv (if k < 4 then 2*k
                   else if k < 8 then 2*(k-4)+1
                   else if k < 12 then 2*(k-8)+8
                   else 2*(k-12)+9))
  = admit ()

(* Adjacent-pair swap (mask passed as `m`): out lane k = in lane (k xor 1). *)
let lemma_nttmul_swap_lane (vv m: ZI.t_Vec256) (k: nat{k < 16}) : Lemma
  (requires m ==
      (ZI.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)))
  (ensures ZI.get_lane (ZI.mm256_shuffle_epi8 vv m) k ==
   ZI.get_lane vv (if k % 2 = 0 then k+1 else k-1))
  = admit ()

(* 64-bit qword permute, control 0xD8 = [q0, q2, q1, q3]. *)
let lemma_nttmul_permute_d8_lane (vv: ZI.t_Vec256) (k: nat{k < 16}) : Lemma
  (ZI.get_lane (ZI.mm256_permute4x64_epi64 (mk_i32 216) vv) k ==
   ZI.get_lane vv (if k < 4 then k else if k < 8 then k+4 else if k < 12 then k-4 else k))
  = admit ()

let lemma_nttmul_cast_lane (vv: ZI.t_Vec256) (j: nat{j < 8}) : Lemma
  (ZI.get_lane128 (ZI.mm256_castsi256_si128 vv) j == ZI.get_lane vv j)
  = lemma_mm256_castsi256_si128 vv

let lemma_nttmul_extract1_lane (vv: ZI.t_Vec256) (j: nat{j < 8}) : Lemma
  (ZI.get_lane128 (ZI.mm256_extracti128_si256 (mk_i32 1) vv) j == ZI.get_lane vv (j + 8))
  = lemma_mm256_extracti128_si256_1 vv

let lemma_nttmul_cvt_lane (x: ZI.t_Vec128) (j: nat{j < 8}) : Lemma
  (ZA.lane32 (ZI.mm256_cvtepi16_epi32 x) j == v (ZI.get_lane128 x j))
  = admit ()

let lemma_nttmul_mullo32_lane (a b: ZI.t_Vec256) (bnd_a bnd_b: nat) (j: nat{j < 8}) : Lemma
  (requires bnd_a * bnd_b < pow2 31 /\
            Spec.Utils.is_intb bnd_a (ZA.lane32 a j) /\
            Spec.Utils.is_intb bnd_b (ZA.lane32 b j))
  (ensures ZA.lane32 (ZI.mm256_mullo_epi32 a b) j == ZA.lane32 a j * ZA.lane32 b j /\
           Spec.Utils.is_intb (bnd_a * bnd_b) (ZA.lane32 (ZI.mm256_mullo_epi32 a b) j))
  = admit ()

let lemma_nttmul_add32_lane (a b: ZI.t_Vec256) (bnd_a bnd_b: nat) (j: nat{j < 8}) : Lemma
  (requires bnd_a + bnd_b < pow2 31 /\
            Spec.Utils.is_intb bnd_a (ZA.lane32 a j) /\
            Spec.Utils.is_intb bnd_b (ZA.lane32 b j))
  (ensures ZA.lane32 (ZI.mm256_add_epi32 a b) j == ZA.lane32 a j + ZA.lane32 b j /\
           Spec.Utils.is_intb (bnd_a + bnd_b) (ZA.lane32 (ZI.mm256_add_epi32 a b) j))
  = admit ()

let lemma_nttmul_madd_lane (a b: ZI.t_Vec256) (bnd_a bnd_b: nat) (j: nat{j < 8}) : Lemma
  (requires 2 * (bnd_a * bnd_b) < pow2 31 /\
            Spec.Utils.is_i16b bnd_a (ZI.get_lane a (2*j)) /\
            Spec.Utils.is_i16b bnd_a (ZI.get_lane a (2*j+1)) /\
            Spec.Utils.is_i16b bnd_b (ZI.get_lane b (2*j)) /\
            Spec.Utils.is_i16b bnd_b (ZI.get_lane b (2*j+1)))
  (ensures ZA.lane32 (ZI.mm256_madd_epi16 a b) j ==
             v (ZI.get_lane a (2*j)) * v (ZI.get_lane b (2*j)) +
             v (ZI.get_lane a (2*j+1)) * v (ZI.get_lane b (2*j+1)) /\
           Spec.Utils.is_intb (2 * (bnd_a * bnd_b)) (ZA.lane32 (ZI.mm256_madd_epi16 a b) j))
  = admit ()

let lemma_nttmul_set32_lane (e7 e6 e5 e4 e3 e2 e1 e0: i32) (j: nat{j < 8}) : Lemma
  (ZA.lane32 (ZI.mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0) j ==
   v (if j = 0 then e0 else if j = 1 then e1 else if j = 2 then e2
      else if j = 3 then e3 else if j = 4 then e4 else if j = 5 then e5
      else if j = 6 then e6 else e7))
  = admit ()

let lemma_nttmul_slli16_lane (vv: ZI.t_Vec256) (k: nat{k < 16}) : Lemma
  (ZI.get_lane (ZI.mm256_slli_epi32 (mk_i32 16) vv) k ==
   (if k % 2 = 0 then mk_i16 0 else ZI.get_lane vv (k-1)))
  = admit ()

let lemma_nttmul_blend_aa_lane (a b: ZI.t_Vec256) (k: nat{k < 16}) : Lemma
  (ZI.get_lane (ZI.mm256_blend_epi16 (mk_i32 170) a b) k ==
   (if k % 2 = 0 then ZI.get_lane a k else ZI.get_lane b k))
  = admit ()

#push-options "--z3rlimit 200"
let lemma_nttmul_even_chain (p r z ab: int) : Lemma
  (requires r % 3329 == (ab * 169) % 3329)
  (ensures ((p + r * z) * 169) % 3329 == ((p + ab * z * 169) * 169) % 3329)
  = calc (==) {
      ((p + r * z) * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (p + r * z) 169 3329 }
      ((p + r * z) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_add_distr p (r * z) 3329 }
      ((p + (r * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l r z 3329 }
      ((p + (r % 3329 * z) % 3329) % 3329 * 169) % 3329;
      (==) { () }
      ((p + ((ab * 169) % 3329 * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (ab * 169) z 3329 }
      ((p + (ab * 169 * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_add_distr p (ab * 169 * z) 3329 }
      ((p + ab * 169 * z) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (p + ab * 169 * z) 169 3329 }
      ((p + ab * 169 * z) * 169) % 3329;
      (==) { assert (ab * 169 * z == ab * z * 169) }
      ((p + ab * z * 169) * 169) % 3329;
    }
#pop-options

#push-options "--z3rlimit 400 --split_queries always"

let lemma_nttmul_prep_evens (orig m sh2 ev: ZI.t_Vec256) (ev128: ZI.t_Vec128) : Lemma
  (requires
     m ==
      (ZI.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 1) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 3) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 5) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 7) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 9) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 11) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 13) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 15) /\
     sh2 == ZI.mm256_permute4x64_epi64 (mk_i32 216) (ZI.mm256_shuffle_epi8 orig m) /\
     ev128 == ZI.mm256_castsi256_si128 sh2 /\
     ev == ZI.mm256_cvtepi16_epi32 ev128)
  (ensures
     ZA.lane32 ev 0 == v (ZI.get_lane orig 0) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 0) /\
     ZA.lane32 ev 1 == v (ZI.get_lane orig 2) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 1) /\
     ZA.lane32 ev 2 == v (ZI.get_lane orig 4) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 2) /\
     ZA.lane32 ev 3 == v (ZI.get_lane orig 6) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 3) /\
     ZA.lane32 ev 4 == v (ZI.get_lane orig 8) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 4) /\
     ZA.lane32 ev 5 == v (ZI.get_lane orig 10) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 5) /\
     ZA.lane32 ev 6 == v (ZI.get_lane orig 12) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 6) /\
     ZA.lane32 ev 7 == v (ZI.get_lane orig 14) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 7))
  = let sh1 = ZI.mm256_shuffle_epi8 orig m in
    lemma_nttmul_cvt_lane ev128 0;
    lemma_nttmul_cast_lane sh2 0;
    lemma_nttmul_cvt_lane ev128 1;
    lemma_nttmul_cast_lane sh2 1;
    lemma_nttmul_cvt_lane ev128 2;
    lemma_nttmul_cast_lane sh2 2;
    lemma_nttmul_cvt_lane ev128 3;
    lemma_nttmul_cast_lane sh2 3;
    lemma_nttmul_cvt_lane ev128 4;
    lemma_nttmul_cast_lane sh2 4;
    lemma_nttmul_cvt_lane ev128 5;
    lemma_nttmul_cast_lane sh2 5;
    lemma_nttmul_cvt_lane ev128 6;
    lemma_nttmul_cast_lane sh2 6;
    lemma_nttmul_cvt_lane ev128 7;
    lemma_nttmul_cast_lane sh2 7;
    lemma_nttmul_permute_d8_lane sh1 0;
    lemma_nttmul_permute_d8_lane sh1 1;
    lemma_nttmul_permute_d8_lane sh1 2;
    lemma_nttmul_permute_d8_lane sh1 3;
    lemma_nttmul_permute_d8_lane sh1 4;
    lemma_nttmul_permute_d8_lane sh1 5;
    lemma_nttmul_permute_d8_lane sh1 6;
    lemma_nttmul_permute_d8_lane sh1 7;
    lemma_nttmul_permute_d8_lane sh1 8;
    lemma_nttmul_permute_d8_lane sh1 9;
    lemma_nttmul_permute_d8_lane sh1 10;
    lemma_nttmul_permute_d8_lane sh1 11;
    lemma_nttmul_permute_d8_lane sh1 12;
    lemma_nttmul_permute_d8_lane sh1 13;
    lemma_nttmul_permute_d8_lane sh1 14;
    lemma_nttmul_permute_d8_lane sh1 15;
    lemma_nttmul_shuffle_group_lane orig m 0;
    lemma_nttmul_shuffle_group_lane orig m 1;
    lemma_nttmul_shuffle_group_lane orig m 2;
    lemma_nttmul_shuffle_group_lane orig m 3;
    lemma_nttmul_shuffle_group_lane orig m 4;
    lemma_nttmul_shuffle_group_lane orig m 5;
    lemma_nttmul_shuffle_group_lane orig m 6;
    lemma_nttmul_shuffle_group_lane orig m 7;
    lemma_nttmul_shuffle_group_lane orig m 8;
    lemma_nttmul_shuffle_group_lane orig m 9;
    lemma_nttmul_shuffle_group_lane orig m 10;
    lemma_nttmul_shuffle_group_lane orig m 11;
    lemma_nttmul_shuffle_group_lane orig m 12;
    lemma_nttmul_shuffle_group_lane orig m 13;
    lemma_nttmul_shuffle_group_lane orig m 14;
    lemma_nttmul_shuffle_group_lane orig m 15;
    ()

let lemma_nttmul_prep_odds (orig m sh2 ev: ZI.t_Vec256) (ev128: ZI.t_Vec128) : Lemma
  (requires
     m ==
      (ZI.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 1) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 3) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 5) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 7) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 9) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 11) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 13) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 15) /\
     sh2 == ZI.mm256_permute4x64_epi64 (mk_i32 216) (ZI.mm256_shuffle_epi8 orig m) /\
     ev128 == ZI.mm256_extracti128_si256 (mk_i32 1) sh2 /\
     ev == ZI.mm256_cvtepi16_epi32 ev128)
  (ensures
     ZA.lane32 ev 0 == v (ZI.get_lane orig 1) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 0) /\
     ZA.lane32 ev 1 == v (ZI.get_lane orig 3) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 1) /\
     ZA.lane32 ev 2 == v (ZI.get_lane orig 5) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 2) /\
     ZA.lane32 ev 3 == v (ZI.get_lane orig 7) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 3) /\
     ZA.lane32 ev 4 == v (ZI.get_lane orig 9) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 4) /\
     ZA.lane32 ev 5 == v (ZI.get_lane orig 11) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 5) /\
     ZA.lane32 ev 6 == v (ZI.get_lane orig 13) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 6) /\
     ZA.lane32 ev 7 == v (ZI.get_lane orig 15) /\
     Spec.Utils.is_intb 3328 (ZA.lane32 ev 7))
  = let sh1 = ZI.mm256_shuffle_epi8 orig m in
    lemma_nttmul_cvt_lane ev128 0;
    lemma_nttmul_extract1_lane sh2 0;
    lemma_nttmul_cvt_lane ev128 1;
    lemma_nttmul_extract1_lane sh2 1;
    lemma_nttmul_cvt_lane ev128 2;
    lemma_nttmul_extract1_lane sh2 2;
    lemma_nttmul_cvt_lane ev128 3;
    lemma_nttmul_extract1_lane sh2 3;
    lemma_nttmul_cvt_lane ev128 4;
    lemma_nttmul_extract1_lane sh2 4;
    lemma_nttmul_cvt_lane ev128 5;
    lemma_nttmul_extract1_lane sh2 5;
    lemma_nttmul_cvt_lane ev128 6;
    lemma_nttmul_extract1_lane sh2 6;
    lemma_nttmul_cvt_lane ev128 7;
    lemma_nttmul_extract1_lane sh2 7;
    lemma_nttmul_permute_d8_lane sh1 0;
    lemma_nttmul_permute_d8_lane sh1 1;
    lemma_nttmul_permute_d8_lane sh1 2;
    lemma_nttmul_permute_d8_lane sh1 3;
    lemma_nttmul_permute_d8_lane sh1 4;
    lemma_nttmul_permute_d8_lane sh1 5;
    lemma_nttmul_permute_d8_lane sh1 6;
    lemma_nttmul_permute_d8_lane sh1 7;
    lemma_nttmul_permute_d8_lane sh1 8;
    lemma_nttmul_permute_d8_lane sh1 9;
    lemma_nttmul_permute_d8_lane sh1 10;
    lemma_nttmul_permute_d8_lane sh1 11;
    lemma_nttmul_permute_d8_lane sh1 12;
    lemma_nttmul_permute_d8_lane sh1 13;
    lemma_nttmul_permute_d8_lane sh1 14;
    lemma_nttmul_permute_d8_lane sh1 15;
    lemma_nttmul_shuffle_group_lane orig m 0;
    lemma_nttmul_shuffle_group_lane orig m 1;
    lemma_nttmul_shuffle_group_lane orig m 2;
    lemma_nttmul_shuffle_group_lane orig m 3;
    lemma_nttmul_shuffle_group_lane orig m 4;
    lemma_nttmul_shuffle_group_lane orig m 5;
    lemma_nttmul_shuffle_group_lane orig m 6;
    lemma_nttmul_shuffle_group_lane orig m 7;
    lemma_nttmul_shuffle_group_lane orig m 8;
    lemma_nttmul_shuffle_group_lane orig m 9;
    lemma_nttmul_shuffle_group_lane orig m 10;
    lemma_nttmul_shuffle_group_lane orig m 11;
    lemma_nttmul_shuffle_group_lane orig m 12;
    lemma_nttmul_shuffle_group_lane orig m 13;
    lemma_nttmul_shuffle_group_lane orig m 14;
    lemma_nttmul_shuffle_group_lane orig m 15;
    ()

let lemma_nttmul_swap_facts (orig m sw: ZI.t_Vec256) : Lemma
  (requires
     m ==
      (ZI.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 1) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 3) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 5) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 7) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 9) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 11) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 13) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane orig 15) /\
     sw == ZI.mm256_shuffle_epi8 orig m)
  (ensures
     ZI.get_lane sw 0 == ZI.get_lane orig 1 /\
     ZI.get_lane sw 1 == ZI.get_lane orig 0 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 1) /\
     ZI.get_lane sw 2 == ZI.get_lane orig 3 /\
     ZI.get_lane sw 3 == ZI.get_lane orig 2 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 3) /\
     ZI.get_lane sw 4 == ZI.get_lane orig 5 /\
     ZI.get_lane sw 5 == ZI.get_lane orig 4 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 5) /\
     ZI.get_lane sw 6 == ZI.get_lane orig 7 /\
     ZI.get_lane sw 7 == ZI.get_lane orig 6 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 7) /\
     ZI.get_lane sw 8 == ZI.get_lane orig 9 /\
     ZI.get_lane sw 9 == ZI.get_lane orig 8 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 9) /\
     ZI.get_lane sw 10 == ZI.get_lane orig 11 /\
     ZI.get_lane sw 11 == ZI.get_lane orig 10 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 11) /\
     ZI.get_lane sw 12 == ZI.get_lane orig 13 /\
     ZI.get_lane sw 13 == ZI.get_lane orig 12 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 13) /\
     ZI.get_lane sw 14 == ZI.get_lane orig 15 /\
     ZI.get_lane sw 15 == ZI.get_lane orig 14 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane sw 15))
  =
    lemma_nttmul_swap_lane orig m 0;
    lemma_nttmul_swap_lane orig m 1;
    lemma_nttmul_swap_lane orig m 2;
    lemma_nttmul_swap_lane orig m 3;
    lemma_nttmul_swap_lane orig m 4;
    lemma_nttmul_swap_lane orig m 5;
    lemma_nttmul_swap_lane orig m 6;
    lemma_nttmul_swap_lane orig m 7;
    lemma_nttmul_swap_lane orig m 8;
    lemma_nttmul_swap_lane orig m 9;
    lemma_nttmul_swap_lane orig m 10;
    lemma_nttmul_swap_lane orig m 11;
    lemma_nttmul_swap_lane orig m 12;
    lemma_nttmul_swap_lane orig m 13;
    lemma_nttmul_swap_lane orig m 14;
    lemma_nttmul_swap_lane orig m 15;
    ()

let lemma_nttmul_out_bounds (pl prsh: ZI.t_Vec256) : Lemma
  (requires
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane pl 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 1) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 3) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 5) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 7) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 9) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 11) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 13) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane prsh 15))
  (ensures ZS.is_i16b_array 3328
     (ZI.vec256_as_i16x16 (ZI.mm256_blend_epi16 (mk_i32 170) pl prsh)))
  = let aux (k: nat{k < 16}) : Lemma
      (Spec.Utils.is_i16b 3328 (ZI.get_lane (ZI.mm256_blend_epi16 (mk_i32 170) pl prsh) k)) =
      lemma_nttmul_blend_aa_lane pl prsh k;
      (if k = 0 then () else if k = 1 then () else if k = 2 then () else if k = 3 then () else if k = 4 then () else if k = 5 then () else if k = 6 then () else if k = 7 then () else if k = 8 then () else if k = 9 then () else if k = 10 then () else if k = 11 then () else if k = 12 then () else if k = 13 then () else if k = 14 then () else ())
    in
    Classical.forall_intro aux

#pop-options

(* The whole functional proof.  `m` is the grouping mask as a free var
   (instantiated with the function's `shuffle_with`); all conclusions land
   on the function's own spine terms. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_nttmul_main (m ms lhs rhs: ZI.t_Vec256) (zeta0 zeta1 zeta2 zeta3: i16) : Lemma
  (requires
     ms ==
      (ZI.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)) /\
     m ==
      (ZI.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
     Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 1) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 3) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 5) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 7) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 9) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 11) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 13) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane lhs 15) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 0) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 1) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 2) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 3) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 4) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 5) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 6) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 7) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 8) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 9) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 10) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 11) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 12) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 13) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 14) /\
     Spec.Utils.is_i16b 3328 (ZI.get_lane rhs 15))
  (ensures
     (let lhs_grouped = ZI.mm256_permute4x64_epi64 (mk_i32 216) (ZI.mm256_shuffle_epi8 lhs m) in
      let rhs_grouped = ZI.mm256_permute4x64_epi64 (mk_i32 216) (ZI.mm256_shuffle_epi8 rhs m) in
      let left = ZI.mm256_mullo_epi32
                   (ZI.mm256_cvtepi16_epi32 (ZI.mm256_castsi256_si128 lhs_grouped))
                   (ZI.mm256_cvtepi16_epi32 (ZI.mm256_castsi256_si128 rhs_grouped)) in
      let odd_products = ZI.mm256_mullo_epi32
                   (ZI.mm256_cvtepi16_epi32 (ZI.mm256_extracti128_si256 (mk_i32 1) lhs_grouped))
                   (ZI.mm256_cvtepi16_epi32 (ZI.mm256_extracti128_si256 (mk_i32 1) rhs_grouped)) in
      let odd_products_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s odd_products in
      let right = ZI.mm256_mullo_epi32 odd_products_reduced
        (ZI.mm256_set_epi32 (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32)) in
      let products_left_raw = ZI.mm256_add_epi32 left right in
      let products_left = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_left_raw in
      let rhs_adjacent_swapped = ZI.mm256_shuffle_epi8 rhs ms in
      let products_right_raw = ZI.mm256_madd_epi16 lhs rhs_adjacent_swapped in
      let products_right_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_right_raw in
      let products_right = ZI.mm256_slli_epi32 (mk_i32 16) products_right_reduced in
      let out = ZI.mm256_blend_epi16 (mk_i32 170) products_left products_right in
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 7) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 7) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 7) /\
      ZS.is_i16b_array 3328 (ZI.vec256_as_i16x16 out) /\
      Spec.Utils.ntt_multiply_butterfly_post
        (ZI.vec256_as_i16x16 lhs) (ZI.vec256_as_i16x16 rhs)
        (ZI.vec256_as_i16x16 out) zeta0 zeta1 zeta2 zeta3))
  = assert_norm (pow2 15 == 32768);
    assert_norm (pow2 31 == 2147483648);
    let lhs_grouped = ZI.mm256_permute4x64_epi64 (mk_i32 216) (ZI.mm256_shuffle_epi8 lhs m) in
    let rhs_grouped = ZI.mm256_permute4x64_epi64 (mk_i32 216) (ZI.mm256_shuffle_epi8 rhs m) in
    let lhs_ev128 = ZI.mm256_castsi256_si128 lhs_grouped in
    let lhs_evens = ZI.mm256_cvtepi16_epi32 lhs_ev128 in
    let lhs_od128 = ZI.mm256_extracti128_si256 (mk_i32 1) lhs_grouped in
    let lhs_odds = ZI.mm256_cvtepi16_epi32 lhs_od128 in
    let rhs_ev128 = ZI.mm256_castsi256_si128 rhs_grouped in
    let rhs_evens = ZI.mm256_cvtepi16_epi32 rhs_ev128 in
    let rhs_od128 = ZI.mm256_extracti128_si256 (mk_i32 1) rhs_grouped in
    let rhs_odds = ZI.mm256_cvtepi16_epi32 rhs_od128 in
    lemma_nttmul_prep_evens lhs m lhs_grouped lhs_evens lhs_ev128;
    lemma_nttmul_prep_odds lhs m lhs_grouped lhs_odds lhs_od128;
    lemma_nttmul_prep_evens rhs m rhs_grouped rhs_evens rhs_ev128;
    lemma_nttmul_prep_odds rhs m rhs_grouped rhs_odds rhs_od128;
    let left = ZI.mm256_mullo_epi32 lhs_evens rhs_evens in
    let odd_products = ZI.mm256_mullo_epi32 lhs_odds rhs_odds in
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 0;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 1;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 2;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 3;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 4;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 5;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 6;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 3328 3328 7;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 0;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 1;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 2;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 3;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 4;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 5;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 6;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 3328 3328 7;
    let odd_products_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s odd_products in
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 0) * v (ZI.get_lane rhs 0))
      (v (ZI.get_lane odd_products_reduced 0)) (v zeta0)
      (v (ZI.get_lane lhs 1) * v (ZI.get_lane rhs 1));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 2) * v (ZI.get_lane rhs 2))
      (v (ZI.get_lane odd_products_reduced 2)) (- (v zeta0))
      (v (ZI.get_lane lhs 3) * v (ZI.get_lane rhs 3));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 4) * v (ZI.get_lane rhs 4))
      (v (ZI.get_lane odd_products_reduced 4)) (v zeta1)
      (v (ZI.get_lane lhs 5) * v (ZI.get_lane rhs 5));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 6) * v (ZI.get_lane rhs 6))
      (v (ZI.get_lane odd_products_reduced 6)) (- (v zeta1))
      (v (ZI.get_lane lhs 7) * v (ZI.get_lane rhs 7));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 8) * v (ZI.get_lane rhs 8))
      (v (ZI.get_lane odd_products_reduced 8)) (v zeta2)
      (v (ZI.get_lane lhs 9) * v (ZI.get_lane rhs 9));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 10) * v (ZI.get_lane rhs 10))
      (v (ZI.get_lane odd_products_reduced 10)) (- (v zeta2))
      (v (ZI.get_lane lhs 11) * v (ZI.get_lane rhs 11));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 12) * v (ZI.get_lane rhs 12))
      (v (ZI.get_lane odd_products_reduced 12)) (v zeta3)
      (v (ZI.get_lane lhs 13) * v (ZI.get_lane rhs 13));
    lemma_nttmul_even_chain
      (v (ZI.get_lane lhs 14) * v (ZI.get_lane rhs 14))
      (v (ZI.get_lane odd_products_reduced 14)) (- (v zeta3))
      (v (ZI.get_lane lhs 15) * v (ZI.get_lane rhs 15));
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 0;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 1;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 2;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 3;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 4;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 5;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 6;
    lemma_nttmul_set32_lane
      (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32) 7;
    let zv = 
      (ZI.mm256_set_epi32 (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32)) in
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 0;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 1;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 2;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 3;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 4;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 5;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 6;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 7;
    let right = ZI.mm256_mullo_epi32 odd_products_reduced zv in
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 0;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 1;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 2;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 3;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 4;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 5;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 6;
    lemma_nttmul_add32_lane left right (3328 * 3328) (3328 * 1664) 7;
    let products_left_raw = ZI.mm256_add_epi32 left right in
    let products_left = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_left_raw in
    assert (v (ZI.get_lane products_left 0) % 3329 ==
      ((v (ZI.get_lane lhs 0) * v (ZI.get_lane rhs 0) +
        v (ZI.get_lane lhs 1) * v (ZI.get_lane rhs 1) * (v zeta0) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 2) % 3329 ==
      ((v (ZI.get_lane lhs 2) * v (ZI.get_lane rhs 2) +
        v (ZI.get_lane lhs 3) * v (ZI.get_lane rhs 3) * (- (v zeta0)) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 4) % 3329 ==
      ((v (ZI.get_lane lhs 4) * v (ZI.get_lane rhs 4) +
        v (ZI.get_lane lhs 5) * v (ZI.get_lane rhs 5) * (v zeta1) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 6) % 3329 ==
      ((v (ZI.get_lane lhs 6) * v (ZI.get_lane rhs 6) +
        v (ZI.get_lane lhs 7) * v (ZI.get_lane rhs 7) * (- (v zeta1)) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 8) % 3329 ==
      ((v (ZI.get_lane lhs 8) * v (ZI.get_lane rhs 8) +
        v (ZI.get_lane lhs 9) * v (ZI.get_lane rhs 9) * (v zeta2) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 10) % 3329 ==
      ((v (ZI.get_lane lhs 10) * v (ZI.get_lane rhs 10) +
        v (ZI.get_lane lhs 11) * v (ZI.get_lane rhs 11) * (- (v zeta2)) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 12) % 3329 ==
      ((v (ZI.get_lane lhs 12) * v (ZI.get_lane rhs 12) +
        v (ZI.get_lane lhs 13) * v (ZI.get_lane rhs 13) * (v zeta3) * 169) * 169) % 3329);
    assert (v (ZI.get_lane products_left 14) % 3329 ==
      ((v (ZI.get_lane lhs 14) * v (ZI.get_lane rhs 14) +
        v (ZI.get_lane lhs 15) * v (ZI.get_lane rhs 15) * (- (v zeta3)) * 169) * 169) % 3329);
    let rhs_adjacent_swapped = ZI.mm256_shuffle_epi8 rhs ms in
    lemma_nttmul_swap_facts rhs ms rhs_adjacent_swapped;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 0;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 1;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 2;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 3;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 4;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 5;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 6;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 3328 3328 7;
    let products_right_raw = ZI.mm256_madd_epi16 lhs rhs_adjacent_swapped in
    let products_right_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_right_raw in
    assert (v (ZI.get_lane products_right_reduced 0) % 3329 ==
      ((v (ZI.get_lane lhs 0) * v (ZI.get_lane rhs 1) +
        v (ZI.get_lane lhs 1) * v (ZI.get_lane rhs 0)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 2) % 3329 ==
      ((v (ZI.get_lane lhs 2) * v (ZI.get_lane rhs 3) +
        v (ZI.get_lane lhs 3) * v (ZI.get_lane rhs 2)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 4) % 3329 ==
      ((v (ZI.get_lane lhs 4) * v (ZI.get_lane rhs 5) +
        v (ZI.get_lane lhs 5) * v (ZI.get_lane rhs 4)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 6) % 3329 ==
      ((v (ZI.get_lane lhs 6) * v (ZI.get_lane rhs 7) +
        v (ZI.get_lane lhs 7) * v (ZI.get_lane rhs 6)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 8) % 3329 ==
      ((v (ZI.get_lane lhs 8) * v (ZI.get_lane rhs 9) +
        v (ZI.get_lane lhs 9) * v (ZI.get_lane rhs 8)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 10) % 3329 ==
      ((v (ZI.get_lane lhs 10) * v (ZI.get_lane rhs 11) +
        v (ZI.get_lane lhs 11) * v (ZI.get_lane rhs 10)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 12) % 3329 ==
      ((v (ZI.get_lane lhs 12) * v (ZI.get_lane rhs 13) +
        v (ZI.get_lane lhs 13) * v (ZI.get_lane rhs 12)) * 169) % 3329);
    assert (v (ZI.get_lane products_right_reduced 14) % 3329 ==
      ((v (ZI.get_lane lhs 14) * v (ZI.get_lane rhs 15) +
        v (ZI.get_lane lhs 15) * v (ZI.get_lane rhs 14)) * 169) % 3329);
    lemma_nttmul_slli16_lane products_right_reduced 1;
    lemma_nttmul_slli16_lane products_right_reduced 3;
    lemma_nttmul_slli16_lane products_right_reduced 5;
    lemma_nttmul_slli16_lane products_right_reduced 7;
    lemma_nttmul_slli16_lane products_right_reduced 9;
    lemma_nttmul_slli16_lane products_right_reduced 11;
    lemma_nttmul_slli16_lane products_right_reduced 13;
    lemma_nttmul_slli16_lane products_right_reduced 15;
    let products_right = ZI.mm256_slli_epi32 (mk_i32 16) products_right_reduced in
    lemma_nttmul_blend_aa_lane products_left products_right 0;
    lemma_nttmul_blend_aa_lane products_left products_right 1;
    lemma_nttmul_blend_aa_lane products_left products_right 2;
    lemma_nttmul_blend_aa_lane products_left products_right 3;
    lemma_nttmul_blend_aa_lane products_left products_right 4;
    lemma_nttmul_blend_aa_lane products_left products_right 5;
    lemma_nttmul_blend_aa_lane products_left products_right 6;
    lemma_nttmul_blend_aa_lane products_left products_right 7;
    lemma_nttmul_blend_aa_lane products_left products_right 8;
    lemma_nttmul_blend_aa_lane products_left products_right 9;
    lemma_nttmul_blend_aa_lane products_left products_right 10;
    lemma_nttmul_blend_aa_lane products_left products_right 11;
    lemma_nttmul_blend_aa_lane products_left products_right 12;
    lemma_nttmul_blend_aa_lane products_left products_right 13;
    lemma_nttmul_blend_aa_lane products_left products_right 14;
    lemma_nttmul_blend_aa_lane products_left products_right 15;
    lemma_nttmul_out_bounds products_left products_right;
    let out = ZI.mm256_blend_epi16 (mk_i32 170) products_left products_right in
    reveal_opaque (`%Spec.Utils.ntt_multiply_butterfly_post)
      (Spec.Utils.ntt_multiply_butterfly_post
        (ZI.vec256_as_i16x16 lhs) (ZI.vec256_as_i16x16 rhs)
        (ZI.vec256_as_i16x16 out) zeta0 zeta1 zeta2 zeta3)
#pop-options
"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
    Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 0) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 1) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 2) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 3) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 4) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 5) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 6) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 7) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 8) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 9) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 10) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 11) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 12) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 13) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 14) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${lhs} 15) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 0) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 1) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 2) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 3) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 4) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 5) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 6) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 7) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 8) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 9) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 10) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 11) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 12) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 13) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 14) /\
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_extract.get_lane ${rhs} 15)"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
    Spec.Utils.ntt_multiply_butterfly_post
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${lhs})
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${rhs})
      (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
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
    let right = mm256_mullo_epi32(
        odd_products_reduced,
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

    let products_left_raw = mm256_add_epi32(left, right);
    let products_left = arithmetic::montgomery_reduce_i32s(products_left_raw);

    // Compute the second term of the product
    let swap_with = mm256_set_epi8(
        13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8, 11, 10, 5,
        4, 7, 6, 1, 0, 3, 2,
    );
    let rhs_adjacent_swapped = mm256_shuffle_epi8(rhs, swap_with);
    let products_right_raw = mm256_madd_epi16(lhs, rhs_adjacent_swapped);
    let products_right_reduced = arithmetic::montgomery_reduce_i32s(products_right_raw);
    let products_right = mm256_slli_epi32::<16>(products_right_reduced);

    // Combine them into one vector
    let result = mm256_blend_epi16::<0b1_0_1_0_1_0_1_0>(products_left, products_right);
    hax_lib::fstar!(
        r#"lemma_nttmul_main ${shuffle_with} ${swap_with} ${lhs} ${rhs} zeta0 zeta1 zeta2 zeta3;
        assert (ZS.is_i16b_array 3328 (ZI.vec256_as_i16x16 ${result}));
        assert (Spec.Utils.ntt_multiply_butterfly_post
          (ZI.vec256_as_i16x16 ${lhs}) (ZI.vec256_as_i16x16 ${rhs})
          (ZI.vec256_as_i16x16 ${result}) zeta0 zeta1 zeta2 zeta3)"#
    );
    result
}
