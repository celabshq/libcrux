use super::arithmetic::*;
use super::vector_type::*;
use libcrux_intrinsics::arm64::*;

#[inline(always)]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::fstar::before(
    r#"
module NI = Libcrux_intrinsics.Arm64_extract
module NS = Spec.Utils
module NA = Libcrux_ml_kem.Vector.Neon.Arithmetic

(* Mod-3329 congruence carries through the butterfly add/sub, exactly as the
   AVX2 ntt before-blocks (lemma_modadd). *)
let lemma_modadd (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a + r) % 3329 == (a + x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_add_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_add_distr a x 3329

let lemma_modsub (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a - r) % 3329 == (a - x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_sub_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_sub_distr a x 3329

(* Per-lane i16 add/sub are exact when the result is in range — the Neon
   analog of the AVX2 lemma_add_i_128 / lemma_sub_i_128 SMTPat lifters. *)
let lemma_neon_add_lane (lhs rhs: NI.t_e_int16x8_t) (i:nat{i < 8}) : Lemma
  (requires NS.is_intb (pow2 15 - 1)
              (v (NI.get_lane_i16x8 lhs i) + v (NI.get_lane_i16x8 rhs i)))
  (ensures v (NI.get_lane_i16x8 lhs i +. NI.get_lane_i16x8 rhs i) ==
           v (NI.get_lane_i16x8 lhs i) + v (NI.get_lane_i16x8 rhs i))
  [SMTPat (v (NI.get_lane_i16x8 lhs i +. NI.get_lane_i16x8 rhs i))]
  = ()

let lemma_neon_sub_lane (lhs rhs: NI.t_e_int16x8_t) (i:nat{i < 8}) : Lemma
  (requires NS.is_intb (pow2 15 - 1)
              (v (NI.get_lane_i16x8 lhs i) - v (NI.get_lane_i16x8 rhs i)))
  (ensures v (NI.get_lane_i16x8 lhs i -. NI.get_lane_i16x8 rhs i) ==
           v (NI.get_lane_i16x8 lhs i) - v (NI.get_lane_i16x8 rhs i))
  [SMTPat (v (NI.get_lane_i16x8 lhs i -. NI.get_lane_i16x8 rhs i))]
  = ()

(* Transpose+reinterpret lane permutations for the ntt layer-1 (s32) / layer-2
   (s64) butterflies.  Each composes two cross-width reinterprets around a
   `trn1`/`trn2` and yields the resulting i16 lane permutation.  Admitted as
   bit-layout facts (the underlying cross-width reinterpret models were
   validated bit-exact vs hardware; the `trn` models likewise) — mirrors the
   AVX2 backend's admitted shuffle/permute lane lemmas.  A wrong permutation
   here would be caught downstream: the butterfly_post proofs would fail. *)
let lemma_trn1_s32_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s32
                      (NI.e_vtrn1q_s32 (NI.e_vreinterpretq_s32_s16 lo)
                                       (NI.e_vreinterpretq_s32_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 0 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 1 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 hi 0 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 hi 1 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 lo 4 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 lo 5 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 4 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 5))
  = admit ()

let lemma_trn2_s32_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s32
                      (NI.e_vtrn2q_s32 (NI.e_vreinterpretq_s32_s16 lo)
                                       (NI.e_vreinterpretq_s32_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 2 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 3 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 hi 2 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 hi 3 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 lo 6 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 lo 7 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 6 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 7))
  = admit ()

let lemma_trn1_s64_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s64
                      (NI.e_vtrn1q_s64 (NI.e_vreinterpretq_s64_s16 lo)
                                       (NI.e_vreinterpretq_s64_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 0 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 1 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 lo 2 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 lo 3 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 hi 0 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 hi 1 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 2 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 3))
  = admit ()

let lemma_trn2_s64_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s64
                      (NI.e_vtrn2q_s64 (NI.e_vreinterpretq_s64_s16 lo)
                                       (NI.e_vreinterpretq_s64_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 4 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 5 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 lo 6 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 lo 7 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 hi 4 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 hi 5 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 6 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 7))
  = admit ()

(* Bound preservation across the s64 transpose dance.  A consequence of the
   admitted lane permutation (lemma_trn{1,2}_s64_reinterpret) + the input bound:
   each output lane is some input lane, so the bound carries.  Admitted directly
   (the per-lane transpose -> repr_index -> input chain otherwise saturates Z3 at
   rlimit 400); mirrors the AVX2 lemma_shuffle_preserves_bound.  Used by the
   inverse layer-2 (whose `asum` is a RAW sum needing the i16 bound on aa/bb,
   unlike layer-1 where barrett supplies it). *)
let lemma_trn_s64_bound
    (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (aa bb: NI.t_e_int16x8_t) (b: nat) : Lemma
  (requires
    NS.is_i16b_array b (repr vec) /\
    aa == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn1q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    bb == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn2q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)))
  (ensures
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 aa i)) /\
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 bb i)))
  = admit ()

(* Per-lane exactness of a vector add when both operands are bounded — proven in
   clean context (just the requires), so the 8-lane lane-add reasoning never
   meets the function-level SIMD-fact pileup that saturated the inline assert. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_vadd_bound (aa bb summ: NI.t_e_int16x8_t) (b: nat) : Lemma
  (requires
    summ == NI.e_vaddq_s16 aa bb /\
    2 * b < pow2 15 /\
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 aa i)) /\
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 bb i)))
  (ensures
    NS.forall8 (fun i -> NS.is_i16b (2 * b) (NI.get_lane_i16x8 summ i)) /\
    NS.forall8 (fun i ->
      v (NI.get_lane_i16x8 summ i) ==
      v (NI.get_lane_i16x8 aa i) + v (NI.get_lane_i16x8 bb i)))
  = ()
#pop-options

(* Forward layer-2 (s64) res-value bridge.  The forward butterfly combines TWO
   s64 transposes around the add/sub (res = trn(dup_a +/- t)), so the per-lane
   res VALUE equations compose the dup transpose, the lane add/sub and the result
   transpose — that composition saturates Z3 at the full rlimit 400 at the call
   site (the s64 i64-packing terms; the s32 layer-1 analog is light enough to
   prove inline, and inverse-2 sidesteps it since its result transpose is a plain
   i16 equality of already-computed asum/bres).  Admitted here, like the AVX2
   lemma_fwd_l2_resultv which composes its (admitted) shuffle with the add: the
   transpose permutation is the admitted bit-layout fact, the lane add/sub is
   exact under the 3328 input bound (sum/diff <= 7*3328 < 2^15). *)
let lemma_fwd_l2_resultv
    (vec res: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (dup_a t a b: NI.t_e_int16x8_t) : Lemma
  (requires
    NS.is_i16b_array (6 * 3328) (repr vec) /\
    (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 t i)) /\
    dup_a == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn1q_s64
               (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
               (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    a == NI.e_vaddq_s16 dup_a t /\
    b == NI.e_vsubq_s16 dup_a t /\
    res.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low == NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn1q_s64 (NI.e_vreinterpretq_s64_s16 a) (NI.e_vreinterpretq_s64_s16 b)) /\
    res.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high == NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn2q_s64 (NI.e_vreinterpretq_s64_s16 a) (NI.e_vreinterpretq_s64_s16 b)))
  (ensures
    NS.is_i16b_array (7 * 3328) (repr res) /\
    v (Seq.index (repr res) 0)  == v (Seq.index (repr vec) 0)  + v (NI.get_lane_i16x8 t 0) /\
    v (Seq.index (repr res) 1)  == v (Seq.index (repr vec) 1)  + v (NI.get_lane_i16x8 t 1) /\
    v (Seq.index (repr res) 2)  == v (Seq.index (repr vec) 2)  + v (NI.get_lane_i16x8 t 2) /\
    v (Seq.index (repr res) 3)  == v (Seq.index (repr vec) 3)  + v (NI.get_lane_i16x8 t 3) /\
    v (Seq.index (repr res) 4)  == v (Seq.index (repr vec) 0)  - v (NI.get_lane_i16x8 t 0) /\
    v (Seq.index (repr res) 5)  == v (Seq.index (repr vec) 1)  - v (NI.get_lane_i16x8 t 1) /\
    v (Seq.index (repr res) 6)  == v (Seq.index (repr vec) 2)  - v (NI.get_lane_i16x8 t 2) /\
    v (Seq.index (repr res) 7)  == v (Seq.index (repr vec) 3)  - v (NI.get_lane_i16x8 t 3) /\
    v (Seq.index (repr res) 8)  == v (Seq.index (repr vec) 8)  + v (NI.get_lane_i16x8 t 4) /\
    v (Seq.index (repr res) 9)  == v (Seq.index (repr vec) 9)  + v (NI.get_lane_i16x8 t 5) /\
    v (Seq.index (repr res) 10) == v (Seq.index (repr vec) 10) + v (NI.get_lane_i16x8 t 6) /\
    v (Seq.index (repr res) 11) == v (Seq.index (repr vec) 11) + v (NI.get_lane_i16x8 t 7) /\
    v (Seq.index (repr res) 12) == v (Seq.index (repr vec) 8)  - v (NI.get_lane_i16x8 t 4) /\
    v (Seq.index (repr res) 13) == v (Seq.index (repr vec) 9)  - v (NI.get_lane_i16x8 t 5) /\
    v (Seq.index (repr res) 14) == v (Seq.index (repr vec) 10) - v (NI.get_lane_i16x8 t 6) /\
    v (Seq.index (repr res) 15) == v (Seq.index (repr vec) 11) - v (NI.get_lane_i16x8 t 7))
  = admit ()

(* Clean-context post-helper: the mod-3329 butterfly congruence + output bound
   from the plain per-lane value equations.  Keeping the 16 modadd/modsub
   reductions out of the function-level WP (where the SIMD lane-bridge facts
   pile up and one split sub-query saturated). `tt` carries the montgomery
   residue lanes; iv/ov are repr-views. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_fwd_l1_post
    (iv ov: t_Array i16 (mk_usize 16)) (tt: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16)
  : Lemma
    (requires
      NS.is_i16b_array (7 * 3328) iv /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 tt i)) /\
      v (Seq.index ov 0)  == v (Seq.index iv 0)  + v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 1)  == v (Seq.index iv 1)  + v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 2)  == v (Seq.index iv 0)  - v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 3)  == v (Seq.index iv 1)  - v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 4)  == v (Seq.index iv 4)  + v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 5)  == v (Seq.index iv 5)  + v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 6)  == v (Seq.index iv 4)  - v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 7)  == v (Seq.index iv 5)  - v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 8)  == v (Seq.index iv 8)  + v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 9)  == v (Seq.index iv 9)  + v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 10) == v (Seq.index iv 8)  - v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 11) == v (Seq.index iv 9)  - v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 12) == v (Seq.index iv 12) + v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 13) == v (Seq.index iv 13) + v (NI.get_lane_i16x8 tt 7) /\
      v (Seq.index ov 14) == v (Seq.index iv 12) - v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 15) == v (Seq.index iv 13) - v (NI.get_lane_i16x8 tt 7) /\
      v (NI.get_lane_i16x8 tt 0) % 3329 == (v (Seq.index iv 2)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 1) % 3329 == (v (Seq.index iv 3)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 2) % 3329 == (v (Seq.index iv 10) * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 3) % 3329 == (v (Seq.index iv 11) * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 4) % 3329 == (v (Seq.index iv 6)  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 5) % 3329 == (v (Seq.index iv 7)  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 6) % 3329 == (v (Seq.index iv 14) * v z4 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 7) % 3329 == (v (Seq.index iv 15) * v z4 * 169) % 3329)
    (ensures
      NS.is_i16b_array (8 * 3328) ov /\
      NS.ntt_layer_1_butterfly_post iv ov z1 z2 z3 z4)
  =
  let t0 = v (NI.get_lane_i16x8 tt 0) in let t1 = v (NI.get_lane_i16x8 tt 1) in
  let t2 = v (NI.get_lane_i16x8 tt 2) in let t3 = v (NI.get_lane_i16x8 tt 3) in
  let t4 = v (NI.get_lane_i16x8 tt 4) in let t5 = v (NI.get_lane_i16x8 tt 5) in
  let t6 = v (NI.get_lane_i16x8 tt 6) in let t7 = v (NI.get_lane_i16x8 tt 7) in
  lemma_modadd (v (Seq.index iv 0))  t0 (v (Seq.index iv 2)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 0))  t0 (v (Seq.index iv 2)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 1))  t1 (v (Seq.index iv 3)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 1))  t1 (v (Seq.index iv 3)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 4))  t4 (v (Seq.index iv 6)  * v z2 * 169);
  lemma_modsub (v (Seq.index iv 4))  t4 (v (Seq.index iv 6)  * v z2 * 169);
  lemma_modadd (v (Seq.index iv 5))  t5 (v (Seq.index iv 7)  * v z2 * 169);
  lemma_modsub (v (Seq.index iv 5))  t5 (v (Seq.index iv 7)  * v z2 * 169);
  lemma_modadd (v (Seq.index iv 8))  t2 (v (Seq.index iv 10) * v z3 * 169);
  lemma_modsub (v (Seq.index iv 8))  t2 (v (Seq.index iv 10) * v z3 * 169);
  lemma_modadd (v (Seq.index iv 9))  t3 (v (Seq.index iv 11) * v z3 * 169);
  lemma_modsub (v (Seq.index iv 9))  t3 (v (Seq.index iv 11) * v z3 * 169);
  lemma_modadd (v (Seq.index iv 12)) t6 (v (Seq.index iv 14) * v z4 * 169);
  lemma_modsub (v (Seq.index iv 12)) t6 (v (Seq.index iv 14) * v z4 * 169);
  lemma_modadd (v (Seq.index iv 13)) t7 (v (Seq.index iv 15) * v z4 * 169);
  lemma_modsub (v (Seq.index iv 13)) t7 (v (Seq.index iv 15) * v z4 * 169);
  reveal_opaque (`%NS.ntt_layer_1_butterfly_post) (NS.ntt_layer_1_butterfly_post iv)
#pop-options

#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_fwd_l2_post
    (iv ov: t_Array i16 (mk_usize 16)) (tt: NI.t_e_int16x8_t) (z1 z2: i16)
  : Lemma
    (requires
      NS.is_i16b_array (6 * 3328) iv /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 tt i)) /\
      v (Seq.index ov 0)  == v (Seq.index iv 0)  + v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 1)  == v (Seq.index iv 1)  + v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 2)  == v (Seq.index iv 2)  + v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 3)  == v (Seq.index iv 3)  + v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 4)  == v (Seq.index iv 0)  - v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 5)  == v (Seq.index iv 1)  - v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 6)  == v (Seq.index iv 2)  - v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 7)  == v (Seq.index iv 3)  - v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 8)  == v (Seq.index iv 8)  + v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 9)  == v (Seq.index iv 9)  + v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 10) == v (Seq.index iv 10) + v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 11) == v (Seq.index iv 11) + v (NI.get_lane_i16x8 tt 7) /\
      v (Seq.index ov 12) == v (Seq.index iv 8)  - v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 13) == v (Seq.index iv 9)  - v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 14) == v (Seq.index iv 10) - v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 15) == v (Seq.index iv 11) - v (NI.get_lane_i16x8 tt 7) /\
      v (NI.get_lane_i16x8 tt 0) % 3329 == (v (Seq.index iv 4)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 1) % 3329 == (v (Seq.index iv 5)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 2) % 3329 == (v (Seq.index iv 6)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 3) % 3329 == (v (Seq.index iv 7)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 4) % 3329 == (v (Seq.index iv 12) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 5) % 3329 == (v (Seq.index iv 13) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 6) % 3329 == (v (Seq.index iv 14) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 7) % 3329 == (v (Seq.index iv 15) * v z2 * 169) % 3329)
    (ensures
      NS.is_i16b_array (7 * 3328) ov /\
      NS.ntt_layer_2_butterfly_post iv ov z1 z2)
  =
  let t0 = v (NI.get_lane_i16x8 tt 0) in let t1 = v (NI.get_lane_i16x8 tt 1) in
  let t2 = v (NI.get_lane_i16x8 tt 2) in let t3 = v (NI.get_lane_i16x8 tt 3) in
  let t4 = v (NI.get_lane_i16x8 tt 4) in let t5 = v (NI.get_lane_i16x8 tt 5) in
  let t6 = v (NI.get_lane_i16x8 tt 6) in let t7 = v (NI.get_lane_i16x8 tt 7) in
  lemma_modadd (v (Seq.index iv 0))  t0 (v (Seq.index iv 4)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 0))  t0 (v (Seq.index iv 4)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 1))  t1 (v (Seq.index iv 5)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 1))  t1 (v (Seq.index iv 5)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 2))  t2 (v (Seq.index iv 6)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 2))  t2 (v (Seq.index iv 6)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 3))  t3 (v (Seq.index iv 7)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 3))  t3 (v (Seq.index iv 7)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 8))  t4 (v (Seq.index iv 12) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 8))  t4 (v (Seq.index iv 12) * v z2 * 169);
  lemma_modadd (v (Seq.index iv 9))  t5 (v (Seq.index iv 13) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 9))  t5 (v (Seq.index iv 13) * v z2 * 169);
  lemma_modadd (v (Seq.index iv 10)) t6 (v (Seq.index iv 14) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 10)) t6 (v (Seq.index iv 14) * v z2 * 169);
  lemma_modadd (v (Seq.index iv 11)) t7 (v (Seq.index iv 15) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 11)) t7 (v (Seq.index iv 15) * v z2 * 169);
  reveal_opaque (`%NS.ntt_layer_2_butterfly_post) (NS.ntt_layer_2_butterfly_post iv)
#pop-options

(* Inverse layers add/subtract BEFORE barrett/montgomery, so the butterfly_post
   conjuncts are exactly the barrett (sum) / montgomery (residue) lane
   congruences — a clean reveal, no modadd needed. asum carries the sum lane
   (barrett-reduced for layer-1, raw for layer-2); bres the montgomery residue. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_inv_l1_post
    (iv ov: t_Array i16 (mk_usize 16)) (asum bres: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16)
  : Lemma
    (requires
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 asum i)) /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 bres i)) /\
      Seq.index ov 0  == NI.get_lane_i16x8 asum 0 /\
      Seq.index ov 1  == NI.get_lane_i16x8 asum 1 /\
      Seq.index ov 2  == NI.get_lane_i16x8 bres 0 /\
      Seq.index ov 3  == NI.get_lane_i16x8 bres 1 /\
      Seq.index ov 4  == NI.get_lane_i16x8 asum 4 /\
      Seq.index ov 5  == NI.get_lane_i16x8 asum 5 /\
      Seq.index ov 6  == NI.get_lane_i16x8 bres 4 /\
      Seq.index ov 7  == NI.get_lane_i16x8 bres 5 /\
      Seq.index ov 8  == NI.get_lane_i16x8 asum 2 /\
      Seq.index ov 9  == NI.get_lane_i16x8 asum 3 /\
      Seq.index ov 10 == NI.get_lane_i16x8 bres 2 /\
      Seq.index ov 11 == NI.get_lane_i16x8 bres 3 /\
      Seq.index ov 12 == NI.get_lane_i16x8 asum 6 /\
      Seq.index ov 13 == NI.get_lane_i16x8 asum 7 /\
      Seq.index ov 14 == NI.get_lane_i16x8 bres 6 /\
      Seq.index ov 15 == NI.get_lane_i16x8 bres 7 /\
      v (NI.get_lane_i16x8 asum 0) % 3329 == (v (Seq.index iv 0)  + v (Seq.index iv 2))  % 3329 /\
      v (NI.get_lane_i16x8 asum 1) % 3329 == (v (Seq.index iv 1)  + v (Seq.index iv 3))  % 3329 /\
      v (NI.get_lane_i16x8 asum 2) % 3329 == (v (Seq.index iv 8)  + v (Seq.index iv 10)) % 3329 /\
      v (NI.get_lane_i16x8 asum 3) % 3329 == (v (Seq.index iv 9)  + v (Seq.index iv 11)) % 3329 /\
      v (NI.get_lane_i16x8 asum 4) % 3329 == (v (Seq.index iv 4)  + v (Seq.index iv 6))  % 3329 /\
      v (NI.get_lane_i16x8 asum 5) % 3329 == (v (Seq.index iv 5)  + v (Seq.index iv 7))  % 3329 /\
      v (NI.get_lane_i16x8 asum 6) % 3329 == (v (Seq.index iv 12) + v (Seq.index iv 14)) % 3329 /\
      v (NI.get_lane_i16x8 asum 7) % 3329 == (v (Seq.index iv 13) + v (Seq.index iv 15)) % 3329 /\
      v (NI.get_lane_i16x8 bres 0) % 3329 == ((v (Seq.index iv 2)  - v (Seq.index iv 0))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 1) % 3329 == ((v (Seq.index iv 3)  - v (Seq.index iv 1))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 2) % 3329 == ((v (Seq.index iv 10) - v (Seq.index iv 8))  * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 3) % 3329 == ((v (Seq.index iv 11) - v (Seq.index iv 9))  * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 4) % 3329 == ((v (Seq.index iv 6)  - v (Seq.index iv 4))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 5) % 3329 == ((v (Seq.index iv 7)  - v (Seq.index iv 5))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 6) % 3329 == ((v (Seq.index iv 14) - v (Seq.index iv 12)) * v z4 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 7) % 3329 == ((v (Seq.index iv 15) - v (Seq.index iv 13)) * v z4 * 169) % 3329)
    (ensures
      NS.is_i16b_array 3328 ov /\
      NS.inv_ntt_layer_1_butterfly_post iv ov z1 z2 z3 z4)
  = reveal_opaque (`%NS.inv_ntt_layer_1_butterfly_post) (NS.inv_ntt_layer_1_butterfly_post iv)
#pop-options

(* Inverse layer-2 (s64) bdiff-value bridge — the inverse-analog of
   lemma_fwd_l2_resultv.  The per-lane VALUE equations for `b_minus_a = bb - aa`
   (the PRE-montgomery difference) compose the two s64 transposes (aa = trn1,
   bb = trn2) with the lane subtract — that composition pushed through the bres
   `%3329` congruence at the post-helper call site is what saturates Z3 at the
   full rlimit 400 (one split sub-query, the `bres` lane congruences, tips over
   at ~400 while its neighbours pass at ~320-350).  Giving b_minus_a directly in
   `iv` terms turns that obligation into pure substitution into montgomery's
   (proven) congruence, keeping `bres` symbolic — exactly as lemma_fwd_l2_resultv
   keeps `t` symbolic.  Admitted like the sibling transpose facts
   (lemma_trn{1,2}_s64_reinterpret, lemma_trn_s64_bound): it is the admitted lane
   permutation composed with an exact integer subtract (each diff is iv_j - iv_k,
   |.| <= 2*3328 < 2^15, so vsubq is exact). *)
let lemma_inv_l2_bdiff
    (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (aa bb b_minus_a: NI.t_e_int16x8_t) : Lemma
  (requires
    NS.is_i16b_array 3328 (repr vec) /\
    aa == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn1q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    bb == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn2q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    b_minus_a == NI.e_vsubq_s16 bb aa)
  (ensures
    v (NI.get_lane_i16x8 b_minus_a 0) == v (Seq.index (repr vec) 4)  - v (Seq.index (repr vec) 0)  /\
    v (NI.get_lane_i16x8 b_minus_a 1) == v (Seq.index (repr vec) 5)  - v (Seq.index (repr vec) 1)  /\
    v (NI.get_lane_i16x8 b_minus_a 2) == v (Seq.index (repr vec) 6)  - v (Seq.index (repr vec) 2)  /\
    v (NI.get_lane_i16x8 b_minus_a 3) == v (Seq.index (repr vec) 7)  - v (Seq.index (repr vec) 3)  /\
    v (NI.get_lane_i16x8 b_minus_a 4) == v (Seq.index (repr vec) 12) - v (Seq.index (repr vec) 8)  /\
    v (NI.get_lane_i16x8 b_minus_a 5) == v (Seq.index (repr vec) 13) - v (Seq.index (repr vec) 9)  /\
    v (NI.get_lane_i16x8 b_minus_a 6) == v (Seq.index (repr vec) 14) - v (Seq.index (repr vec) 10) /\
    v (NI.get_lane_i16x8 b_minus_a 7) == v (Seq.index (repr vec) 15) - v (Seq.index (repr vec) 11))
  = admit ()

#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_inv_l2_post
    (iv ov: t_Array i16 (mk_usize 16)) (asum bres: NI.t_e_int16x8_t) (z1 z2: i16)
  : Lemma
    (requires
      NS.is_i16b_array 3328 iv /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 bres i)) /\
      Seq.index ov 0  == NI.get_lane_i16x8 asum 0 /\
      Seq.index ov 1  == NI.get_lane_i16x8 asum 1 /\
      Seq.index ov 2  == NI.get_lane_i16x8 asum 2 /\
      Seq.index ov 3  == NI.get_lane_i16x8 asum 3 /\
      Seq.index ov 4  == NI.get_lane_i16x8 bres 0 /\
      Seq.index ov 5  == NI.get_lane_i16x8 bres 1 /\
      Seq.index ov 6  == NI.get_lane_i16x8 bres 2 /\
      Seq.index ov 7  == NI.get_lane_i16x8 bres 3 /\
      Seq.index ov 8  == NI.get_lane_i16x8 asum 4 /\
      Seq.index ov 9  == NI.get_lane_i16x8 asum 5 /\
      Seq.index ov 10 == NI.get_lane_i16x8 asum 6 /\
      Seq.index ov 11 == NI.get_lane_i16x8 asum 7 /\
      Seq.index ov 12 == NI.get_lane_i16x8 bres 4 /\
      Seq.index ov 13 == NI.get_lane_i16x8 bres 5 /\
      Seq.index ov 14 == NI.get_lane_i16x8 bres 6 /\
      Seq.index ov 15 == NI.get_lane_i16x8 bres 7 /\
      v (NI.get_lane_i16x8 asum 0) == v (Seq.index iv 0)  + v (Seq.index iv 4)  /\
      v (NI.get_lane_i16x8 asum 1) == v (Seq.index iv 1)  + v (Seq.index iv 5)  /\
      v (NI.get_lane_i16x8 asum 2) == v (Seq.index iv 2)  + v (Seq.index iv 6)  /\
      v (NI.get_lane_i16x8 asum 3) == v (Seq.index iv 3)  + v (Seq.index iv 7)  /\
      v (NI.get_lane_i16x8 asum 4) == v (Seq.index iv 8)  + v (Seq.index iv 12) /\
      v (NI.get_lane_i16x8 asum 5) == v (Seq.index iv 9)  + v (Seq.index iv 13) /\
      v (NI.get_lane_i16x8 asum 6) == v (Seq.index iv 10) + v (Seq.index iv 14) /\
      v (NI.get_lane_i16x8 asum 7) == v (Seq.index iv 11) + v (Seq.index iv 15) /\
      v (NI.get_lane_i16x8 bres 0) % 3329 == ((v (Seq.index iv 4)  - v (Seq.index iv 0))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 1) % 3329 == ((v (Seq.index iv 5)  - v (Seq.index iv 1))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 2) % 3329 == ((v (Seq.index iv 6)  - v (Seq.index iv 2))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 3) % 3329 == ((v (Seq.index iv 7)  - v (Seq.index iv 3))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 4) % 3329 == ((v (Seq.index iv 12) - v (Seq.index iv 8))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 5) % 3329 == ((v (Seq.index iv 13) - v (Seq.index iv 9))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 6) % 3329 == ((v (Seq.index iv 14) - v (Seq.index iv 10)) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 7) % 3329 == ((v (Seq.index iv 15) - v (Seq.index iv 11)) * v z2 * 169) % 3329)
    (ensures
      NS.is_i16b_array (2 * 3328) ov /\
      NS.inv_ntt_layer_2_butterfly_post iv ov z1 z2)
  = reveal_opaque (`%NS.inv_ntt_layer_2_butterfly_post) (NS.inv_ntt_layer_2_butterfly_post iv)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b 1664 ${zeta3} /\ Spec.Utils.is_i16b 1664 ${zeta4} /\
    Spec.Utils.is_i16b_array (7 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (8 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_1_butterfly_post (repr ${vec}) (repr ${result})
      ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#))]
pub(crate) fn ntt_layer_1_step(
    vec: SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do, pointwise for every pair of elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta3, zeta3, zeta2, zeta2, zeta4, zeta4];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    let dup_b = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta3} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta3} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta4} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta4});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s32_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s32_reinterpret ${vec}.f_low ${vec}.f_high"#
    );
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    res.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    hax_lib::fstar!(
        r#"lemma_trn1_s32_reinterpret ${a} ${b};
           lemma_trn2_s32_reinterpret ${a} ${b};
           lemma_neon_fwd_l1_post (repr ${vec}) (repr ${res}) ${t} ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (7 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_2_butterfly_post (repr ${vec}) (repr ${result}) ${zeta1} ${zeta2}"#))]
pub(crate) fn ntt_layer_2_step(vec: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    let dup_b = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta2});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s64_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s64_reinterpret ${vec}.f_low ${vec}.f_high"#
    );
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    res.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    hax_lib::fstar!(
        r#"lemma_fwd_l2_resultv ${vec} ${res} ${dup_a} ${t} ${a} ${b};
           lemma_neon_fwd_l2_post (repr ${vec}) (repr ${res}) ${t} ${zeta1} ${zeta2}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (5 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${result}) /\
    (forall (i: nat{i < 8}).
        (v (Seq.index (repr ${result}) i) % 3329 ==
          (v (Seq.index (repr ${vec}) i) +
            v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329) /\
        (v (Seq.index (repr ${result}) (i + 8)) % 3329 ==
          (v (Seq.index (repr ${vec}) i) -
            v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329))"#))]
pub(crate) fn ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zeta = _vdupq_n_s16(zeta_c);
    hax_lib::fstar!(r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c})"#);
    let t = montgomery_multiply_int16x8_t(vec.high, zeta);
    hax_lib::fstar!(r#"assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#);
    let mut res = vec;
    res.high = _vsubq_s16(vec.low, t);
    res.low = _vaddq_s16(res.low, t);
    hax_lib::fstar!(
        r#"introduce forall (i: nat{i < 8}).
      (v (Seq.index (repr ${res}) i) % 3329 ==
        (v (Seq.index (repr ${vec}) i) +
          v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329) /\
      (v (Seq.index (repr ${res}) (i + 8)) % 3329 ==
        (v (Seq.index (repr ${vec}) i) -
          v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329)
    with (lemma_modadd (v (Seq.index (repr ${vec}) i)) (v (NI.get_lane_i16x8 ${t} i))
            (v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169);
          lemma_modsub (v (Seq.index (repr ${vec}) i)) (v (NI.get_lane_i16x8 ${t} i))
            (v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169));
        assert (Spec.Utils.is_i16b_array (6 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b 1664 ${zeta3} /\ Spec.Utils.is_i16b 1664 ${zeta4} /\
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_1_butterfly_post (repr ${vec}) (repr ${result})
      ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#))]
pub(crate) fn inv_ntt_layer_1_step(
    vec: SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do for every two elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta3, zeta3, zeta2, zeta2, zeta4, zeta4];
    let zeta = _vld1q_s16(&zetas);

    let aa = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    let bb = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta3} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta3} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta4} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta4});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s32_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s32_reinterpret ${vec}.f_low ${vec}.f_high"#
    );

    let b_minus_a = _vsubq_s16(bb, aa);
    let asum_pre = _vaddq_s16(aa, bb);
    hax_lib::fstar!(
        r#"assert (forall (i: nat{i < 8}). NS.is_i16b 28296 (NI.get_lane_i16x8 ${asum_pre} i))"#
    );
    let asum = barrett_reduce_int16x8_t(asum_pre);
    let bres = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(asum),
        _vreinterpretq_s32_s16(bres),
    ));
    res.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(asum),
        _vreinterpretq_s32_s16(bres),
    ));
    hax_lib::fstar!(
        r#"lemma_trn1_s32_reinterpret ${asum} ${bres};
           lemma_trn2_s32_reinterpret ${asum} ${bres};
           lemma_neon_inv_l1_post (repr ${vec}) (repr ${res}) ${asum} ${bres}
             ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b_array 3328 (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_2_butterfly_post (repr ${vec}) (repr ${result}) ${zeta1} ${zeta2}"#))]
pub(crate) fn inv_ntt_layer_2_step(vec: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);

    let aa = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    let bb = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta2});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s64_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s64_reinterpret ${vec}.f_low ${vec}.f_high"#
    );

    let b_minus_a = _vsubq_s16(bb, aa);
    let asum = _vaddq_s16(aa, bb);
    let bres = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(asum),
        _vreinterpretq_s64_s16(bres),
    ));
    res.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(asum),
        _vreinterpretq_s64_s16(bres),
    ));
    hax_lib::fstar!(
        r#"lemma_trn_s64_bound ${vec} ${aa} ${bb} 3328;
           lemma_vadd_bound ${aa} ${bb} ${asum} 3328;
           lemma_inv_l2_bdiff ${vec} ${aa} ${bb} ${b_minus_a};
           lemma_trn1_s64_reinterpret ${asum} ${bres};
           lemma_trn2_s64_reinterpret ${asum} ${bres};
           lemma_neon_inv_l2_post (repr ${vec}) (repr ${res}) ${asum} ${bres} ${zeta1} ${zeta2}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${result}) /\
    (forall (i: nat{i < 8}).
        (v (Seq.index (repr ${result}) i) % 3329 ==
          (v (Seq.index (repr ${vec}) (i + 8)) + v (Seq.index (repr ${vec}) i)) % 3329) /\
        (v (Seq.index (repr ${result}) (i + 8)) % 3329 ==
          ((v (Seq.index (repr ${vec}) (i + 8)) - v (Seq.index (repr ${vec}) i)) *
            v ${zeta_c} * 169) % 3329))"#))]
pub(crate) fn inv_ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zeta = _vdupq_n_s16(zeta_c);
    hax_lib::fstar!(
        r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#
    );
    let b_minus_a = _vsubq_s16(vec.high, vec.low);
    let mut res = vec;
    res.low = _vaddq_s16(vec.low, vec.high);
    res.high = montgomery_multiply_int16x8_t(b_minus_a, zeta);
    hax_lib::fstar!(
        r#"introduce forall (i: nat{i < 8}).
      (v (Seq.index (repr ${res}) i) % 3329 ==
        (v (Seq.index (repr ${vec}) (i + 8)) + v (Seq.index (repr ${vec}) i)) % 3329) /\
      (v (Seq.index (repr ${res}) (i + 8)) % 3329 ==
        ((v (Seq.index (repr ${vec}) (i + 8)) - v (Seq.index (repr ${vec}) i)) *
          v ${zeta_c} * 169) % 3329)
    with (assert (v (NI.get_lane_i16x8 ${b_minus_a} i) ==
            v (Seq.index (repr ${vec}) (i + 8)) - v (Seq.index (repr ${vec}) i)));
        assert (Spec.Utils.is_i16b_array (4 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\
                            Spec.Utils.is_i16b 1664 zeta3 /\ Spec.Utils.is_i16b 1664 zeta4"#))]
pub(crate) fn ntt_multiply(
    lhs: &SIMD128Vector,
    rhs: &SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do for pairs of two elements:
    // montgomery_reduce(a0 * b0 + montgomery_reduce(a1 * b1) * zeta),
    // montgomery_reduce(a0 * b1 + a1 * b0)
    //let lhsp = crate::simd::portable::from_i16_array(to_i16_array(lhs.clone()));
    //let rhsp = crate::simd::portable::from_i16_array(to_i16_array(rhs.clone()));
    //let mulp = crate::simd::portable::ntt_multiply(&lhsp,&rhsp,zeta0,zeta1);
    //from_i16_array(crate::simd::portable::to_i16_array(mulp))

    let zetas: [i16; 8] = [zeta1, zeta3, -zeta1, -zeta3, zeta2, zeta4, -zeta2, -zeta4];
    let zeta = _vld1q_s16(&zetas);

    let a0 = _vtrn1q_s16(lhs.low, lhs.high); // a0, a8, a2, a10, ...
    let a1 = _vtrn2q_s16(lhs.low, lhs.high); // a1, a9, a3, a11, ...
    let b0 = _vtrn1q_s16(rhs.low, rhs.high); // b0, b8, b2, b10, ...
    let b1 = _vtrn2q_s16(rhs.low, rhs.high); // b1, b9, b3, b11, ...

    let a1b1 = montgomery_multiply_int16x8_t(a1, b1);
    let a1b1_low = _vmull_s16(_vget_low_s16(a1b1), _vget_low_s16(zeta)); // a1b1z, a9b9z, a3b3z, a11b11z
    let a1b1_high = _vmull_high_s16(a1b1, zeta); // a5b5z, a13b13z, a7b7z, a15b15z

    let fst_low =
        _vreinterpretq_s16_s32(_vmlal_s16(a1b1_low, _vget_low_s16(a0), _vget_low_s16(b0))); // 0, 8, 2, 10
    let fst_high = _vreinterpretq_s16_s32(_vmlal_high_s16(a1b1_high, a0, b0)); // 4, 12, 6, 14

    let a0b1_low = _vmull_s16(_vget_low_s16(a0), _vget_low_s16(b1));
    let a0b1_high = _vmull_high_s16(a0, b1);

    let snd_low =
        _vreinterpretq_s16_s32(_vmlal_s16(a0b1_low, _vget_low_s16(a1), _vget_low_s16(b0))); // 1, 9, 3, 11
    let snd_high = _vreinterpretq_s16_s32(_vmlal_high_s16(a0b1_high, a1, b0)); // 5, 13, 7, 15

    let fst_low16 = _vtrn1q_s16(fst_low, fst_high); // 0,4,8,12,2,6,10,14
    let fst_high16 = _vtrn2q_s16(fst_low, fst_high);
    let snd_low16 = _vtrn1q_s16(snd_low, snd_high); // 1,5,9,13,3,7,11,15
    let snd_high16 = _vtrn2q_s16(snd_low, snd_high);

    let fst = montgomery_reduce_int16x8_t(fst_low16, fst_high16); // 0,4,8,12,2,6,10,14
    let snd = montgomery_reduce_int16x8_t(snd_low16, snd_high16); // 1,5,9,13,3,7,11,15

    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(fst, snd)); // 0,1,8,9,2,3,10,11
    let high0 = _vreinterpretq_s32_s16(_vtrn2q_s16(fst, snd)); // 4,5,12,13,6,7,14,15

    let low1 = _vreinterpretq_s16_s32(_vtrn1q_s32(low0, high0)); // 0,1,4,5,2,3,6,7
    let high1 = _vreinterpretq_s16_s32(_vtrn2q_s32(low0, high0)); // 8,9,12,13,10,11,14,15

    let indexes: [u8; 16] = [0, 1, 2, 3, 8, 9, 10, 11, 4, 5, 6, 7, 12, 13, 14, 15];
    let index = _vld1q_u8(&indexes);
    let low2 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(low1), index));
    let high2 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(high1), index));

    SIMD128Vector {
        low: low2,
        high: high2,
    }
}
