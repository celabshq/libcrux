module Libcrux_ml_kem.Vector.Avx2.Arithmetic_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

(* Hand-written proof theory relocated from src/vector/avx2/arithmetic.rs
   `hax_lib::fstar::before` blocks (byte-exact raw-string contents).
   `mont_reduce_lane` stays in arithmetic.rs: its statement cites the module's
   interface-level `lane32`/`mont_red_i32_lane` (module-level dep cycle
   otherwise, F* Error 308). *)

open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views

let lemma_add_i (lhs rhs: t_Vec256) (i:nat): Lemma 
  (requires (i < 16 /\ Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane lhs i) + v (get_lane rhs i))))
  (ensures (v (add_mod (get_lane lhs i) (get_lane rhs i)) ==
            (v (get_lane lhs i) + v (get_lane rhs i))))
  [SMTPat (v (add_mod (get_lane lhs i) (get_lane rhs i)))] = ()

let lemma_sub_i (lhs rhs: t_Vec256) (i:nat):  Lemma 
  (requires (i < 16 /\ Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane lhs i) - v (get_lane rhs i))))
  (ensures (v (sub_mod (get_lane lhs i) (get_lane rhs i)) ==
            (v (get_lane lhs i) - v (get_lane rhs i))))
  [SMTPat (v (sub_mod (get_lane lhs i) (get_lane rhs i)))] = ()

let lemma_mul_i (lhs: t_Vec256) (i:nat) (c:i16):  Lemma 
  (requires (i < 16 /\ Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane lhs i) * v c)))
  (ensures (v (mul_mod (get_lane lhs i) c) ==
            (v (get_lane lhs i) * v c)))
  [SMTPat (v (mul_mod (get_lane lhs i) c))] = ()

(* ── A4 montgomery_reduce_i32s proof scaffolding ──────────────────────────── *)

(* lane32 (the intrinsic, transparent i32-lane view) decomposes into its two
   i16 sub-lanes: the @%-into-i16 is the low lane, the floor-/pow2 16 is the high
   lane.  Pure modular arithmetic over the lane32 definition. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_lane32_halves (w: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (ensures
      (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 w j) @% pow2 16 ==
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane w (2 * j)) /\
      (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 w j) / pow2 16 ==
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane w (2 * j + 1)))
  = let lo = v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane w (2 * j)) in
    let hi = v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane w (2 * j + 1)) in
    assert_norm (pow2 16 == 65536);
    FStar.Math.Lemmas.lemma_div_plus (lo % pow2 16) hi (pow2 16);
    FStar.Math.Lemmas.small_div (lo % pow2 16) (pow2 16);
    FStar.Math.Lemmas.modulo_addition_lemma (lo % pow2 16) (pow2 16) hi;
    FStar.Math.Lemmas.small_mod (lo % pow2 16) (pow2 16);
    Spec.Utils.lemma_range_at_percent lo (pow2 16)
#pop-options

(* The logical srli-by-16 reproduces, mod 2^16, the arithmetic floor /2^16 of the
   (signed) lane: this is what makes `srli_epi32 16` deliver the high i16 lane. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_srli_hi (vv: int)
  : Lemma (requires - (pow2 31) <= vv /\ vv < pow2 31)
          (ensures ((vv % pow2 32) / pow2 16) @% pow2 16 == vv / pow2 16)
  = assert_norm (pow2 32 == pow2 16 * pow2 16);
    assert_norm (pow2 31 == pow2 16 * pow2 15);
    FStar.Math.Lemmas.lemma_div_lt_nat (if vv >= 0 then vv else vv + pow2 32) 32 16;
    if vv >= 0 then begin
      FStar.Math.Lemmas.small_mod vv (pow2 32);
      FStar.Math.Lemmas.lemma_div_lt_nat vv 31 16;
      Spec.Utils.lemma_range_at_percent (vv / pow2 16) (pow2 16)
    end
    else begin
      FStar.Math.Lemmas.small_mod (vv + pow2 32) (pow2 32);
      FStar.Math.Lemmas.modulo_addition_lemma vv (pow2 32) 1;
      FStar.Math.Lemmas.lemma_div_plus vv (pow2 16) (pow2 16);
      FStar.Math.Lemmas.modulo_addition_lemma (vv / pow2 16) (pow2 16) (pow2 16);
      FStar.Math.Lemmas.small_mod (vv / pow2 16 + pow2 16) (pow2 16);
      Spec.Utils.lemma_range_at_percent (vv / pow2 16) (pow2 16)
    end
#pop-options

(* Ground per-op lane facts (clean single-op context, like Compress's
   slli_lane_nowrap / srli3_lane), so the consumer can cite them as posts
   instead of letting the slli/srai lane-foralls auto-fire and cascade.  The
   slli get-lane facts keep `lane32` atomic so the slli *general* (lane32 @%
   2^32) clause cannot pull in nonlinear work. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60 --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lane32'"
let lemma_slli16_even
      (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
    : Lemma
      (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane
          (Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) vv) (2 * j) == mk_i16 0)
  = let r = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) vv in
    ()
let lemma_slli16_odd
      (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
    : Lemma
      (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane
          (Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) vv) (2 * j + 1) ==
        Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (2 * j))
  = let r = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) vv in
    ()
#pop-options
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_srai16_lane
      (r2 r3: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
    : Lemma (requires r3 == Libcrux_intrinsics.Avx2.mm256_srai_epi32 (mk_i32 16) r2)
            (ensures
              Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r3 j ==
                (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r2 j) / pow2 16)
  = ()
#pop-options

(* slli 16 then arithmetic srai 16 sign-extends the even i16 sub-lane `t`
   (the Montgomery result, |t| <= 3328) back into the full 32-bit lane.  The raw
   slli/srai posts are excluded so they cannot auto-fire and cascade; the lane
   facts come from the ground lemmas above. *)
(* Migration (core-models): the slli/srai lane-foralls now arrive via the
   COMPANION SMTPat lemmas (Avx2_ml_kem_views.lemma_mm256_{slli,srai}_epi32), not
   the (now-opaque l_True) real ops.  Exclude THOSE companion lemmas so they
   cannot auto-fire on the `requires` terms and cascade; the lane facts are
   supplied explicitly by the ground lemmas (lemma_slli16_even/odd,
   lemma_srai16_lane) called in the body. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_slli_epi32 -Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_srai_epi32'"
let lemma_sign_extend
      (r1 r2 r3: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) (t: i16)
    : Lemma
      (requires
        Spec.Utils.is_i16b 3328 t /\
        Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r1 (2 * j) == t /\
        r2 == Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) r1 /\
        r3 == Libcrux_intrinsics.Avx2.mm256_srai_epi32 (mk_i32 16) r2)
      (ensures
        Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r3 (2 * j) == t /\
        Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r3 j == v t)
  = assert_norm (pow2 16 == 65536);
    lemma_slli16_even r1 j;
    lemma_slli16_odd r1 j;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r2 j == pow2 16 * v t);
    lemma_srai16_lane r2 r3 j;
    FStar.Math.Lemmas.cancel_mul_div (v t) (pow2 16);
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r3 j == v t);
    lemma_lane32_halves r3 j;
    Spec.Utils.lemma_range_at_percent (v t) (pow2 16)
#pop-options
