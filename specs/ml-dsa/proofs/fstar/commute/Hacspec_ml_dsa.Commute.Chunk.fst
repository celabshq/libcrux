module Hacspec_ml_dsa.Commute.Chunk
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Libcrux_ml_dsa.Simd.Traits.Specs

(* Per-element commute lemmas bridging the impl-side `arithmetic::*` free-fn
   posts (in their existing spec form) to the trait-side per-lane post
   predicates in `Libcrux_ml_dsa.Simd.Traits.Specs`.  Each lemma converts
   one shape to the other for one i32 lane; the per-array forall is closed
   at the call site via `Classical.forall_intro`. *)

module P = Hacspec_ml_dsa.Parameters
module A = Hacspec_ml_dsa.Arithmetic
module L = FStar.Math.Lemmas
module TS = Libcrux_ml_dsa.Simd.Traits.Specs

(* Bridge: given the centered Barrett bound on `result` and the raw mod-q
   congruence between `input` and `result`, conclude `reduce_lane_post`.
   The free-fn `arithmetic::reduce` proves both conjuncts; the impl method
   reveals `Spec.MLDSA.Math.mod_q` opacity at the call site to convert
   the existing free-fn post into the raw-mod shape this lemma consumes. *)
let lemma_reduce_lane_commute (input result: i32)
    = reveal_opaque (`%TS.reduce_lane_post) (TS.reduce_lane_post input result)

(* Bridge: the AVX2 free fn `arithmetic::reduce` advertises its post in the
   raw `Spec.MLDSA.Math.barrett_red` shape.  This lemma converts that shape
   into the (centered Barrett bound) + (raw mod-q congruence) shape that
   `lemma_reduce_lane_commute` consumes.

   Spec.MLDSA.Math.barrett_red x = x - q * 8380417  where
     q = (x + 2^22) >> 23
   (centered Barrett reduction by 2^23).  For |x| < 2^31 - 2^22, the
   output fits in i32 with |r| < 8380417, and r ≡ x (mod 8380417). *)
#push-options "--z3rlimit 200"
let lemma_barrett_red_bound_and_mod_q (x: i32)
    = reveal_opaque (`%Spec.MLDSA.Math.barrett_red) (Spec.MLDSA.Math.barrett_red x);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    let two_22 = shift_left (mk_i32 1) (mk_i32 22) in
    assert (v two_22 == pow2 22);
    let sum = add_mod x two_22 in
    assert (v sum == v x + pow2 22);
    let q = shift_right sum (mk_i32 23) in
    assert (v q == (v x + pow2 22) / pow2 23);
    let prod = mul_mod q Spec.MLDSA.Math.v_FIELD_MODULUS in
    assert (v prod == v q * 8380417);
    let r = sub_mod x prod in
    assert (v r == v x - v q * 8380417);
    L.lemma_mod_sub (v x) 8380417 (v q)
#pop-options

(* Bridge: convert the AVX2 `arithmetic::add` post (per-lane
   `to_i32x8 lhs_future i == add_mod_opaque lhs[i] rhs[i]`) into the
   integer-equality shape consumed by `Libcrux_ml_dsa.Simd.Traits.Specs.add_post`.
   The trait pre `add_pre` (per-lane sum is i32) makes `add_mod` non-wrapping.
   `int_is_i32` from `Libcrux_ml_dsa.Simd.Traits.Specs` reduces (since
   `Hax_lib.Int.t_Int` is `int` unfolded) to the bound `-2^31 <= x <= 2^31 - 1`
   which is exactly `range x i32_inttype`. *)
let lemma_add_lane_commute (lhs rhs lhs_future: i32)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype

(* Bridge: same, for `arithmetic::subtract` / `sub_mod_opaque`. *)
let lemma_sub_lane_commute (lhs rhs lhs_future: i32)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype

(* Bridge: convert the Tier-1 `Spec.MLDSA.Math.power2round` int-pair shape
   into `power2round_lane_post`.  Both impls' `arithmetic::power2round`
   posts state, per-lane:
     let (t0_s, t1_s) = Spec.MLDSA.Math.power2round (v input)
     v future_t0 == t0_s /\ v future_t1 == t1_s
   The trait-side post `power2round_lane_post` cites
   `Hacspec_ml_dsa.Arithmetic.power2round` (returning (r1, r0) i32 pair).
   For input in [0, q), the two specs compute the same i32 values; this
   lemma just unfolds both and lets Z3 match them. *)
#push-options "--z3rlimit 200"
let lemma_power2round_lane_commute (input future_t1 future_t0: i32)
    = reveal_opaque (`%TS.power2round_lane_post)
                  (TS.power2round_lane_post input future_t1 future_t0)
#pop-options

(* Math lemma: for any i32 input, the t1 component of
   `Spec.MLDSA.Math.power2round` lies in `[0, pow2 10)`.  The trait
   post advertises this as an unconditional per-lane bound on
   `t1_future` (cherry-pick a331580ec); the underlying arithmetic
   free-fn only states `v t1 == snd (...power2round (v input))`, so
   the bound has to come from the math spec.

   Reasoning:
     representative = (v input) % q  ∈ [0, q-1] = [0, 8380416]
     m              = representative % (pow2 13)  ∈ [0, pow2 13)
     t0             = if m > pow2 12 then m - pow2 13 else m
                       ∈ (-pow2 12, pow2 12]
     t1             = (representative - t0) / pow2 13

   - If m ≤ pow2 12: t0 = m, so representative - t0 = (representative
     / pow2 13) * pow2 13 (since m = representative % pow2 13).  Hence
     t1 = representative / pow2 13.  Since representative < q,
     t1 < q / pow2 13 = 1023 + 1/pow2 13, and as int floor:
     t1 ≤ (q-1) / pow2 13 = 1023.  So t1 ∈ [0, 1024) = [0, pow2 10).
   - If m > pow2 12: t0 = m - pow2 13, so representative - t0 =
     representative - m + pow2 13.  Since representative - m is a
     non-negative multiple of pow2 13, t1 = (representative / pow2 13) + 1.
     Worst case t1 = 1024 would require representative / pow2 13 = 1023
     and m > pow2 12.  But (q-1) / pow2 13 = 1023 only when
     representative ≥ 1023 * pow2 13 = 8380416 = q-1, and at exactly
     that value m = 0 (since (q-1) is divisible by pow2 13), which
     contradicts m > pow2 12.  So t1 ≤ 1023 in this branch too. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 300"
let lemma_power2round_t1_bound (input: i32)
    = let q : pos = 8380417 in
    let p : pos = pow2 13 in
    assert (p == 8192);
    assert (pow2 12 == 4096);
    assert (pow2 10 == 1024);
    let representative = (v input) % q in
    L.lemma_mod_lt (v input) q;
    L.lemma_mod_plus_distr_l (v input) 0 q;
    assert (0 <= representative /\ representative <= q - 1);
    let m = representative % p in
    L.lemma_mod_lt representative p;
    L.lemma_mod_plus_distr_l representative 0 p;
    assert (0 <= m /\ m < p);
    let t0 = if m > pow2 12 then m - p else m in
    let t1 = (representative - t0) / p in
    if m > pow2 12 then begin
      assert (t0 == m - p);
      assert (representative - t0 == representative - m + p);
      L.lemma_div_mod representative p;
      assert (representative - m == (representative / p) * p);
      assert (representative - t0 == (representative / p + 1) * p);
      L.cancel_mul_div (representative / p + 1) p;
      assert (t1 == representative / p + 1);
      // representative / p ≤ (q-1) / p = 1023, but if it equals 1023
      // then representative ≥ 1023 * p = q - 1 and (q-1) % p = 0,
      // contradicting m > pow2 12.
      L.lemma_div_le representative (q - 1) p;
      assert (representative / p <= (q - 1) / p);
      assert ((q - 1) / p == 1023);
      assert (representative / p <= 1023);
      // Exclude representative/p = 1023 case under m > pow2 12.
      if representative / p = 1023 then begin
        L.lemma_div_mod representative p;
        assert (representative == 1023 * p + m);
        assert (representative <= q - 1);
        assert (1023 * p + m <= q - 1);
        // q - 1 = 1023 * p, so 1023 * p + m <= 1023 * p, thus m <= 0,
        // contradicting m > pow2 12 > 0.
        assert (1023 * p == q - 1);
        assert (m <= 0);
        ()
      end
      else assert (representative / p <= 1022);
      assert (t1 <= 1023)
    end
    else begin
      assert (t0 == m);
      assert (representative - t0 == representative - m);
      L.lemma_div_mod representative p;
      assert (representative - m == (representative / p) * p);
      L.cancel_mul_div (representative / p) p;
      assert (t1 == representative / p);
      L.lemma_div_le representative (q - 1) p;
      assert ((q - 1) / p == 1023);
      assert (t1 <= 1023)
    end
#pop-options

(* Math lemma: for any i32 input, the t0 component of
   `Spec.MLDSA.Math.power2round` lies in `(-pow2 12, pow2 12]` (half-open).
   Used by `power2round_with_proof` to discharge the trait post's
   `is_i32b_strict_lower_array_opaque (pow2 12) t0_future` conjunct.

   Reasoning: representative = (v input) % q ∈ [0, q-1].
     t0 = mod_p representative (pow2 13).
     mod_p sets m = representative % (pow2 13) ∈ [0, pow2 13);
     if m > pow2 12 then t0 = m - pow2 13 ∈ (-pow2 12, 0)
     else (m <= pow2 12) t0 = m ∈ [0, pow2 12].
   Combined: t0 ∈ (-pow2 12, pow2 12].  Unlike `decompose`, there is
   no special-case adjustment, so the half-open bound holds (cf. F-13). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_power2round_t0_strict_lower_bound (input: i32)
    = let q : pos = 8380417 in
    let p : pos = pow2 13 in
    assert (p == 8192);
    assert (pow2 12 == 4096);
    let representative = (v input) % q in
    L.lemma_mod_lt (v input) q;
    let m = representative % p in
    L.lemma_mod_lt representative p;
    assert (0 <= m /\ m < p)
#pop-options

(* Bridge: convert the AVX2 free fn post `barrett_red(shift_left_opaque
   input 13)` into the relaxed
   `shift_left_then_reduce_lane_post` (centered-Barrett bound + mod-q
   congruence with input * 8192).  The hypothesis `0 <= v input <= 261631`
   bounds `input <<! 13 < 2^31 - 2^22` so `lemma_barrett_red_bound_and_mod_q`
   applies and produces both halves of the post. *)
#push-options "--z3rlimit 200"
let lemma_shift_left_then_reduce_lane_commute (input future: i32)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    let shifted = Spec.Intrinsics.shift_left_opaque input (mk_i32 13) in
    assert (v shifted == v input * 8192);
    lemma_barrett_red_bound_and_mod_q shifted;
    reveal_opaque (`%TS.shift_left_then_reduce_lane_post)
                  (TS.shift_left_then_reduce_lane_post input future)
#pop-options

(* Bridge for the Portable side: the Portable `arithmetic::shift_left_then_reduce`
   post advertises mod_q congruence with `input <<! 13` plus the
   centered-Barrett bound.  This lemma converts that shape into the
   relaxed lane post. *)
let lemma_shift_left_then_reduce_lane_commute_mod_q
    (input future: i32)
    = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
    assert (v (input <<! mk_i32 13 <: i32) == v input * 8192);
    reveal_opaque (`%TS.shift_left_then_reduce_lane_post)
                  (TS.shift_left_then_reduce_lane_post input future)

(* === F-1 restructuring (above-trait verdict 7a4dc28df, option d) ===
   The trait pre `is_i32b_array_opaque FIELD_MAX` for use_hint /
   decompose / compute_hint is intentionally weaker than the lane
   posts' `[0, q)`-conditional `==>` shape.  Each impl-side commute
   is split into two lemmas:
   (1) Unconditional bound — discharges the new cherry-picked
       array-level bound conjuncts (44 / 16 for use_hint, 95232 /
       44 / 261888 / 16 for decompose, 44 / 16 for compute_hint
       hint_future).  Proved over any input by inspection of the
       impl's internal normalize / `% m` step.
   (2) Conditional equation — uses `introduce ... with hyp.` to
       produce the lane post's `==>` shape, discharging the
       Spec-vs-Hacspec equivalence under `v input ∈ [0, q)`. *)

(* Math lemma: for `g ∈ {95232, 261888}` and any int `r`, the `r1`
   component of `Spec.MLDSA.Math.decompose g r` lies in `[0, m)` where
   m = 4190208 / g (= 44 for γ2=95232, = 16 for γ2=261888).

   Reasoning (mirrors lemma_power2round_t1_bound):
     r_q = r % q  ∈ [0, q-1] = [0, m*twog]   where twog = 2g, q-1 = m*twog
     r_g = mod_p r_q twog  ∈ (-g, g]
     - Special case r_q - r_g = q-1: spec returns r1 = 0 ∈ [0, m).
     - Otherwise: r1 = (r_q - r_g) / twog.  Two sub-cases on r_g_raw =
       r_q % twog ∈ [0, twog):
       (A) r_g_raw ≤ g: r_g = r_g_raw, r_q - r_g = (r_q / twog) * twog,
           r1 = r_q / twog ∈ [0, m].  r1 = m iff r_q = m*twog = q-1
           and r_g_raw = 0, which gives r_q - r_g = q-1 → special case
           (excluded).  So r1 ≤ m-1 in non-special.
       (B) r_g_raw > g: r_g = r_g_raw - twog, r_q - r_g = (r_q/twog + 1)*twog,
           r1 = r_q/twog + 1 ∈ [1, m+1].  r1 = m+1 would need r_q ≥
           m*twog = q-1, but then r_g_raw = (q-1) % twog = 0 ≤ g
           (case A), contradiction.  r1 = m iff r_q/twog = m-1, giving
           r_q - r_g = m*twog = q-1 → special (excluded).  So r1 ≤ m-1
           in non-special. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400"
let lemma_spec_decompose_r1_bound (g: int) (r: int)
    : Lemma
        (requires (g == 95232 \/ g == 261888))
        (ensures
          (let r_q = r % 8380417 in
           let r_g = Spec.Utils.mod_p r_q (g * 2) in
           let m = 4190208 / g in
           let r1 = if r_q - r_g = 8380416 then 0 else (r_q - r_g) / (g * 2) in
           0 <= r1 /\ r1 < m))
  = let q : pos = 8380417 in
    let twog : pos = g * 2 in
    let m : pos = 4190208 / g in
    assert (m * twog == 8380416);
    let r_q = r % q in
    L.lemma_mod_lt r q;
    assert (0 <= r_q /\ r_q <= q - 1);
    let r_g_raw = r_q % twog in
    L.lemma_mod_lt r_q twog;
    assert (0 <= r_g_raw /\ r_g_raw < twog);
    assert (twog / 2 == g);
    let r_g = Spec.Utils.mod_p r_q twog in
    assert (r_g == (if r_g_raw > g then r_g_raw - twog else r_g_raw));
    L.lemma_div_mod r_q twog;
    assert (r_q == (r_q / twog) * twog + r_g_raw);
    if r_q - r_g = q - 1 then ()
    else if r_g_raw > g then begin
      assert (r_g == r_g_raw - twog);
      assert (r_q - r_g == (r_q / twog) * twog + twog);
      assert (r_q - r_g == (r_q / twog + 1) * twog);
      L.cancel_mul_div (r_q / twog + 1) twog;
      assert ((r_q - r_g) / twog == r_q / twog + 1);
      // In non-special: r_q - r_g != q - 1 = m*twog, so r_q/twog + 1 != m.
      // Upper bound: r_q ≤ q-1 = m*twog, but if r_q = q-1 then r_g_raw = 0,
      // contradicting r_g_raw > g.  So r_q ≤ q-2, hence r_q/twog ≤ (q-2)/twog.
      assert ((q - 2) / twog == m - 1);
      L.lemma_div_le r_q (q - 2) twog;
      // r_q/twog ≤ m-1, so r_q/twog + 1 ≤ m.  And ≠ m (non-special), so ≤ m-1.
      ()
    end
    else begin
      assert (r_g == r_g_raw);
      assert (r_q - r_g == (r_q / twog) * twog);
      L.cancel_mul_div (r_q / twog) twog;
      assert ((r_q - r_g) / twog == r_q / twog);
      // r_q ≤ q-1 = m*twog, so r_q/twog ≤ m.  If r_q/twog = m then
      // r_q ≥ m*twog = q-1, hence r_q = q-1 and r_g_raw = (q-1) % twog = 0,
      // r_g = 0, r_q - r_g = q - 1 → special case (excluded).  So in
      // non-special r_q/twog ≤ m-1.
      L.lemma_div_le r_q (q - 1) twog;
      assert ((q - 1) / twog == m);
      ()
    end
#pop-options

(* Unconditional bound for `Spec.MLDSA.Math.use_one_hint`.  The Spec
   computes either `r1` (hint=0) or `(r1 ± 1) % (4190208 / g)`
   (hint=1).  In both cases the result lies in `[0, m)` where
   m = 4190208 / g.  hint=1 follows from `lemma_mod_lt`; hint=0
   reduces to `lemma_spec_decompose_r1_bound`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_use_one_hint_bound (g r hint: i32)
    = let m : int = 4190208 / v g in
    assert (m == 44 \/ m == 16);
    let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v g) (v r) in
    lemma_spec_decompose_r1_bound (v g) (v r);
    if v hint = 0 then ()
    else if r0_s > 0 then L.lemma_mod_lt (r1_s + 1) m
    else L.lemma_mod_lt (r1_s - 1) m
#pop-options

(* Sub-lemma: `Hacspec_ml_dsa.Arithmetic.mod_pm` matches `Spec.Utils.mod_p`
   in v-image, for non-negative `a` and positive even `m` fitting in i32.
   The Hacspec version computes `((a%m)+m)%m` in i64 then folds the
   half-shift; both produce the centered representative in `(-m/2, m/2]`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mod_pm_eq_mod_p (a m: i32)
    : Lemma
        (requires v a >= 0 /\ v a < 8380417 /\
                  v m > 0 /\ v m % 2 == 0 /\ v m <= 1000000)
        (ensures
          v (Hacspec_ml_dsa.Arithmetic.mod_pm a m) == Spec.Utils.mod_p (v a) (v m))
  = let a64 : i64 = cast a <: i64 in
    let m64 : i64 = cast m <: i64 in
    assert (v a64 == v a /\ v m64 == v m);
    let r1 = a64 %! m64 in
    L.lemma_mod_lt (v a) (v m);
    assert (v r1 == v a % v m);
    let r2 = r1 +! m64 in
    assert (v r2 == v a % v m + v m);
    let r3 = r2 %! m64 in
    L.lemma_mod_plus (v a % v m) 1 (v m);
    L.modulo_lemma (v a % v m) (v m);
    assert (v r3 == v a % v m);
    let r32 : i32 = cast r3 <: i32 in
    assert (v r32 == v a % v m);
    let half = m /! mk_i32 2 in
    assert (v half == v m / 2)
#pop-options

(* Bridge: under `v input ∈ [0, q)` and `v gamma2 ∈ {95232, 261888}`,
   the i32 `Hacspec.decompose` agrees in v-image with the int-level
   `Spec.MLDSA.Math.decompose` (note layouts differ: Spec returns
   `(r0, r1, bool)`, Hacspec returns `(r1, r0)` i32 pair). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 600"
let lemma_decompose_bridge (input gamma2: i32)
    : Lemma
        (requires
          (v gamma2 == 95232 \/ v gamma2 == 261888) /\
          v input >= 0 /\ v input < 8380417)
        (ensures
          (let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v gamma2) (v input) in
           let (r1_h, r0_h) = Hacspec_ml_dsa.Arithmetic.decompose input gamma2 in
           v r1_h == r1_s /\ v r0_h == r0_s))
  = let q = 8380417 in
    let twog = v gamma2 * 2 in
    // Hacspec body: r_plus = input %! Q, fixup, alpha = 2*gamma2, r0 = mod_pm,
    //               then if/else.
    let r_plus0 = input %! Hacspec_ml_dsa.Parameters.v_Q in
    L.small_mod (v input) q;
    assert (v r_plus0 == v input);
    // r_plus0 >= 0, so the fixup branch is unchanged.
    let alpha = mk_i32 2 *! gamma2 in
    assert (v alpha == twog);
    let r0_h = Hacspec_ml_dsa.Arithmetic.mod_pm r_plus0 alpha in
    lemma_mod_pm_eq_mod_p r_plus0 alpha;
    assert (v r0_h == Spec.Utils.mod_p (v input) twog);
    // Spec body: r_q = input % q, r_g = mod_p r_q twog, etc.
    L.small_mod (v input) q;
    assert ((v input) % q == v input);
    // The branch comparisons match because v r_plus0 - v r0_h fits in i32.
    let diff = r_plus0 -! r0_h in
    assert (v r0_h > -(v gamma2) /\ v r0_h <= v gamma2);
    assert (v diff == v input - v r0_h);
    ()
#pop-options

(* Conditional equation: under `v input ∈ [0, q)`, the Spec.MLDSA.Math
   and Hacspec computations of use_one_hint agree.  The lane post's
   `==>` shape is discharged via `introduce ... with hyp`.  Outside
   `[0, q)`, the lane post is vacuously true (the `==>` premise
   fails). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 600"
let lemma_use_hint_lane_commute_conditional
    (gamma2 input hint future_hint: i32)
    = reveal_opaque (`%TS.use_hint_lane_post)
                  (TS.use_hint_lane_post gamma2 input hint future_hint);
    introduce
        v input >= 0 /\ v input < 8380417 /\ (v hint == 0 \/ v hint == 1) ==>
        v future_hint == v (Hacspec_ml_dsa.Arithmetic.uuse_hint (v hint = 1) input gamma2)
    with hyp.
      let m_int = 4190208 / v gamma2 in
      assert (m_int == 44 \/ m_int == 16);
      let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v gamma2) (v input) in
      let (r1_h, r0_h) = Hacspec_ml_dsa.Arithmetic.decompose input gamma2 in
      lemma_decompose_bridge input gamma2;
      lemma_spec_decompose_r1_bound (v gamma2) (v input);
      assert (v r1_h == r1_s /\ v r0_h == r0_s);
      assert (0 <= r1_s /\ r1_s < m_int);
      // Hacspec uses `m_h = (Q-1) /! (2 *! gamma2)` which equals m_int.
      let m_h : i32 = (Hacspec_ml_dsa.Parameters.v_Q -! mk_i32 1) /! (mk_i32 2 *! gamma2) in
      assert (v m_h == m_int);
      // Note: in F*'s hax-lib, `%!` on machine ints is Euclidean (returns
      // non-negative values strictly less than the modulus), the same as
      // F*'s int `%`.  So the i32 expressions match the int expressions
      // directly under v-image, modulo i32-range checks.
      if v hint = 0 then ()
      else if r0_s > 0 then begin
        // Spec: (r1_s + 1) % m_int.  Hacspec: (r1_h +! 1) %! m_h.
        let one_plus = r1_h +! mk_i32 1 in
        assert (v one_plus == r1_s + 1)
      end
      else begin
        // Spec: (r1_s - 1) % m_int.  Hacspec: (((r1_h -! 1) %! m_h) +! m_h) %! m_h.
        let m1 = r1_h -! mk_i32 1 in
        assert (v m1 == r1_s - 1);
        let s1 = m1 %! m_h in
        L.lemma_mod_lt (v m1) m_int;
        assert (v s1 == (r1_s - 1) % m_int /\ 0 <= v s1 /\ v s1 < m_int);
        let s2 = s1 +! m_h in
        assert (v s2 == (r1_s - 1) % m_int + m_int);
        L.lemma_mod_plus ((r1_s - 1) % m_int) 1 m_int;
        L.small_mod ((r1_s - 1) % m_int) m_int;
        assert (v (s2 %! m_h) == (r1_s - 1) % m_int)
      end
#pop-options

(* === Track 2: paired-lemma template for decompose, compute_hint === *)

(* Bound lemma for decompose (paired-lemma template).  Used by the
   Portable (and AVX2) impls to discharge the array-level bound
   conjuncts on `low_future` (= Spec.r0; `is_i32b_array g`) and
   `high_future` (= Spec.r1; `is_i32b_array (4190208/g)`).

   Spec returns r0 = either `r_g` (non-special) or `r_g - 1` (special,
   when r_q - r_g = q-1, which forces r_g = 0 and r0 = -1).
   Both cases yield r0 ∈ [-g, g]. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_decompose_bound (gamma2 r: i32)
    = let g = v gamma2 in
    let q = 8380417 in
    let twog = g * 2 in
    let r_q = (v r) % q in
    L.lemma_mod_lt (v r) q;
    let r_g = Spec.Utils.mod_p r_q twog in
    L.lemma_mod_lt r_q twog;
    assert (-g < r_g /\ r_g <= g);
    lemma_spec_decompose_r1_bound g (v r)
#pop-options

(* Conditional equation for decompose (paired-lemma template).
   Under `v input ∈ [0, q)`, `Hacspec.decompose` agrees with
   `Spec.MLDSA.Math.decompose` (output layouts differ; v-image agrees).
   Discharges the trait-side `decompose_lane_post`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_decompose_lane_commute_conditional
    (gamma2 input low_future high_future: i32)
    = reveal_opaque (`%TS.decompose_lane_post)
                  (TS.decompose_lane_post gamma2 input low_future high_future);
    introduce
        v input >= 0 /\ v input < 8380417 ==>
        (let pair = Hacspec_ml_dsa.Arithmetic.decompose input gamma2 in
         v low_future == v (snd pair) /\ v high_future == v (fst pair))
    with hyp.
      lemma_decompose_bridge input gamma2
#pop-options

(* Bound lemma for compute_hint (paired-lemma template).  Trivial:
   `Spec.MLDSA.Math.compute_one_hint` returns 0 or 1 by definition.
   Used to discharge `is_binary_array_8_opaque hint_future`. *)
let lemma_compute_one_hint_bound (low high gamma2: i32)
    = ()

(* Bound lemma for the popcount (`Spec.MLDSA.Math.compute_hint`)
   under the binary-hint hypothesis: each of the 8 lanes is 0 or 1,
   so the sum is in [0, 8].  Discharges the trait-side
   `v result <= 8` conjunct on `Operations::compute_hint`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let rec lemma_compute_hint_bound_aux (hint: t_Array i32 (sz 8)) (n: nat{n <= 8})
    : Lemma
        (requires
          (forall (i: nat). i < 8 ==>
            (v (Seq.index hint i) == 0 \/ v (Seq.index hint i) == 1)))
        (ensures
          Spec.Utils.repeati (sz n) (Spec.MLDSA.Math.hint_counter hint) 0 <= n)
        (decreases n)
  = if n = 0 then
      Spec.Utils.eq_repeati0 (sz 0) (Spec.MLDSA.Math.hint_counter hint) 0
    else begin
      lemma_compute_hint_bound_aux hint (n - 1);
      Spec.Utils.unfold_repeati (sz n) (Spec.MLDSA.Math.hint_counter hint) 0 (sz (n - 1));
      // step adds v (cast hint[n-1] <: usize) which is 0 or 1 under hyp.
      assert (v (cast (Seq.index hint (n - 1)) <: usize) == 0 \/
              v (cast (Seq.index hint (n - 1)) <: usize) == 1)
    end
#pop-options

let lemma_compute_hint_bound (hint: t_Array i32 (sz 8))
    = lemma_compute_hint_bound_aux hint 8

(* Conditional equation for compute_hint (paired-lemma template).
   Trivial under F-4 (cdb6e946e): `compute_hint_lane_post` now cites
   `Spec.MLDSA.Math.compute_one_hint` directly, matching the lemma's
   `requires` exactly.  The `make_hint` cross-spec link was dropped
   on the above-trait side because it is unprovable at the boundary
   `low = -gamma2, high != 0` (Spec returns 1, Hacspec returns 0). *)
let lemma_compute_hint_lane_commute_conditional
    (gamma2 low high hint_future: i32)
    = reveal_opaque (`%TS.compute_hint_lane_post)
                  (TS.compute_hint_lane_post gamma2 low high hint_future)

(* === Track 4 (Step 9.6 AVX2 montgomery_multiply) === *)

(* Bound + mod-q congruence for `Spec.MLDSA.Math.mont_mul`.

   Mirror of the ML-KEM template `lemma_mont_mul_red_i16_int`
   (`libcrux-ml-kem/proofs/fstar/spec/Spec.Utils.fst:505`) with
   i16→i32, i32→i64, shift 16→32, q=3329→8380417,
   q'=−3327→58728449 (note: ML-DSA stores q' as positive,
   58728449 = q^-1 mod 2^32; ML-KEM stored as -3327 ≡ q^-1 mod 2^16
   due to negation convention), R^-1=169→8265825.

   The Montgomery property `q' * q ≡ 1 (mod R)`:
     58728449 * 8380417 = 4294967296 * 114592 + 1, so the product
     mod 2^32 = 1.  Verified via assert_norm. *)
#push-options "--z3rlimit 600 --fuel 0 --ifuel 1"
let lemma_mont_mul_bound_and_mod_q (x y: i32)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i64_inttype;
    Spec.Intrinsics.reveal_opaque_cast_ops #i32_inttype #i64_inttype;
    (* Post mont_mul refactor: mont_mul x y == mont_red (i32_mul x y).
       mont_mul is now non-opaque (unfolds automatically); we reveal
       mont_red instead to expose hi/low/k/c. *)
    reveal_opaque (`%Spec.MLDSA.Math.mont_red) (Spec.MLDSA.Math.mont_red);
    reveal_opaque (`%Spec.MLDSA.Math.i32_mul) (Spec.MLDSA.Math.i32_mul);
    let prod : int = v x * v y in
    // Step 1: product = i32_mul x y (i64 with v == prod, since |prod| < pow2 63).
    assert_norm (pow2 31 * 8380416 < pow2 63);
    Spec.Utils.lemma_range_at_percent prod (pow2 64);
    let cast_x : i64 = cast x <: i64 in
    let cast_y : i64 = cast y <: i64 in
    Spec.Utils.lemma_range_at_percent (v x) (pow2 64);
    Spec.Utils.lemma_range_at_percent (v y) (pow2 64);
    assert (v cast_x == v x /\ v cast_y == v y);
    let product : i64 = Spec.MLDSA.Math.i32_mul x y in
    assert (v product == prod);
    // Step 2: hi = cast_mod (product >> 32) <: i32 = prod / 2^32.
    let prod_shifted : i64 = product >>! mk_i32 32 in
    assert (v prod_shifted == prod / pow2 32);
    assert_norm (pow2 31 * 8380416 / pow2 32 < pow2 31);
    assert_norm (- (pow2 31 * 8380416 / pow2 32) > - pow2 31);
    Spec.Utils.lemma_range_at_percent (prod / pow2 32) (pow2 32);
    let hi : i32 = cast prod_shifted <: i32 in
    assert (v hi == prod / pow2 32);
    // Step 3: low = cast_mod product <: i32 = prod @% 2^32.
    let low : i32 = cast product <: i32 in
    assert (v low == prod @% pow2 32);
    // Step 4: k = cast_mod (low *! Q' as i64) <: i32 = (low * Q') @% 2^32.
    let q'_i32 = mk_i32 58728449 in
    let cast_low : i64 = cast low <: i64 in
    let cast_qp : i64 = cast q'_i32 <: i64 in
    Spec.Utils.lemma_range_at_percent (v low) (pow2 64);
    Spec.Utils.lemma_range_at_percent 58728449 (pow2 64);
    assert (v cast_low == v low /\ v cast_qp == 58728449);
    let lq_product : i64 = Spec.MLDSA.Math.i32_mul low q'_i32 in
    assert_norm (pow2 31 * 58728449 < pow2 63);
    Spec.Utils.lemma_range_at_percent (v low * 58728449) (pow2 64);
    assert (v lq_product == v low * 58728449);
    let k : i32 = cast lq_product <: i32 in
    assert (v k == (v low * 58728449) @% pow2 32);
    // Step 5: c = cast_mod ((k * Q as i64) >> 32) <: i32 = (k*q)/2^32 (under bound).
    let q_i32 = mk_i32 8380417 in
    let cast_k : i64 = cast k <: i64 in
    let cast_q : i64 = cast q_i32 <: i64 in
    Spec.Utils.lemma_range_at_percent (v k) (pow2 64);
    Spec.Utils.lemma_range_at_percent 8380417 (pow2 64);
    assert (v cast_k == v k /\ v cast_q == 8380417);
    let kq_product : i64 = Spec.MLDSA.Math.i32_mul k q_i32 in
    assert_norm (pow2 31 * 8380417 < pow2 63);
    Spec.Utils.lemma_range_at_percent (v k * 8380417) (pow2 64);
    assert (v kq_product == v k * 8380417);
    let kq_shifted : i64 = kq_product >>! mk_i32 32 in
    assert (v kq_shifted == (v k * 8380417) / pow2 32);
    assert_norm (pow2 31 * 8380417 / pow2 32 < pow2 31);
    assert_norm (- (pow2 31 * 8380417 / pow2 32) > - pow2 31);
    Spec.Utils.lemma_range_at_percent ((v k * 8380417) / pow2 32) (pow2 32);
    let c : i32 = cast kq_shifted <: i32 in
    assert (v c == (v k * 8380417) / pow2 32);
    // Step 6: result = sub_mod hi c.  Bound preservation needs |hi - c| < 2^31.
    assert_norm (pow2 22 + (pow2 31 * 8380417 / pow2 32) < pow2 31);
    let result : i32 = hi -! c in
    assert (v result == v hi - v c);
    // === MOD-q PROOF (mirroring ML-KEM's calc chain) ===
    // Show: (k * q) % 2^32 == prod % 2^32  (so prod - k*q is divisible by 2^32).
    assert_norm ((58728449 * 8380417) % pow2 32 == 1);
    Spec.Utils.lemma_at_percent_mod (v low * 58728449) (pow2 32);
    // (v k) * 8380417 ≡ ((v low * 58728449) @% 2^32) * 8380417 (mod 2^32)
    // Apply lemma_mod_mul_distr_l to push @% inside:
    L.lemma_mod_mul_distr_l (v low * 58728449) 8380417 (pow2 32);
    L.lemma_mod_mul_distr_l ((v low * 58728449) @% pow2 32) 8380417 (pow2 32);
    Spec.Utils.lemma_at_percent_mod (v low * 58728449) (pow2 32);
    // Now: (v k * 8380417) % 2^32 == (v low * 58728449 * 8380417) % 2^32
    //                            == (v low * 1) % 2^32  (using q'*q ≡ 1 mod 2^32)
    //                            == v low % 2^32
    //                            == prod % 2^32
    L.lemma_mod_mul_distr_r (v low) (58728449 * 8380417) (pow2 32);
    Spec.Utils.lemma_at_percent_mod prod (pow2 32);
    assert ((v k * 8380417) % pow2 32 == prod % pow2 32);
    // (prod - k*q) % 2^32 == 0:
    L.lemma_mod_sub_distr prod (v k * 8380417) (pow2 32);
    assert ((prod - v k * 8380417) % pow2 32 == 0);
    L.lemma_div_exact (prod - v k * 8380417) (pow2 32);
    // hi - c = prod/2^32 - (k*q)/2^32 = (prod - k*q)/2^32 (using lemma_div_exact).
    assert (v result == (prod - v k * 8380417) / pow2 32);
    // Final step: ((prod - k*q)/2^32) % q == (prod * 8265825) % q.
    assert_norm ((pow2 32 * 8265825) % 8380417 == 1);
    L.lemma_mod_mul_distr_r ((prod - v k * 8380417) / pow2 32) (pow2 32 * 8265825) 8380417;
    L.lemma_div_exact (prod - v k * 8380417) (pow2 32);
    L.lemma_mod_sub (prod * 8265825) 8380417 (v k * 8265825);
    // === BOUND PROOF: |v result| ≤ q-1 = 8380416 ===
    // |hi| ≤ pow2 22 (since |prod| ≤ pow2 31 * 8380416 < pow2 54, /pow2 32 < pow2 22).
    // |c| ≤ pow2 31 * 8380417 / pow2 32 ≈ 2^22.  Combined |hi - c| < q from Montgomery.
    // The tight bound: |v result * pow2 32| = |prod - k*q|.
    //   |prod| < pow2 31 * 8380416 < (pow2 31) * q
    //   |v k| < pow2 31 (i32 range), |v k * q| < pow2 31 * q
    //   |prod - k*q| < 2 * pow2 31 * q = pow2 32 * q
    //   |v result| < pow2 32 * q / pow2 32 = q, i.e., |v result| ≤ q-1.
    assert (v product == prod);  // anchor
    assert_norm (pow2 31 * 8380417 + pow2 31 * 8380416 < pow2 32 * 8380417)
#pop-options

(* === Chunking lemma — bridges 32 chunks of 8-lane i32 arrays to flat 256.
   Used by per-layer NTT/Invntt commute lemmas to relate the
   PolynomialRingElement-level repr (32 SIMD units of 8 lanes each) to
   `Hacspec_ml_dsa.Ntt.{ntt, intt, ntt_layer, intt_layer}` which all
   operate on `t_Array i32 256`.  Backend-agnostic: works on the abstract
   i32-array view, not the per-backend SIMD unit struct.  Consumers
   project SIMDUnit -> t_Array i32 8 via the trait method `f_repr`
   before invoking. *)


(* Index reveal — `simd_units_to_array` at flat index `8b + l` is
   `chunks.[b].[l]`.  Discharges via the SMTPat on `createi_lemma`. *)
let lemma_simd_units_to_array_reveal
      (chunks: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
      (b: nat{b < 32}) (l: nat{l < 8})
    = let i: usize = mk_usize (8*b + l) in
    assert (v i = 8*b + l);
    assert (v i / 8 = b);
    assert (v i % 8 = l)

(* Frame property — if `chunks_future` agrees with `chunks` on every
   chunk index `j <> b`, then `simd_units_to_array` agrees on every
   flat index `i` whose chunk `i/8 <> b`.  At every per-layer NTT
   bridge, exactly one chunk is being updated; this lemma carries the
   "other 31 chunks unchanged" invariant through the chunk-to-flat
   transformation. *)
let lemma_simd_units_to_array_other_chunk_unchanged
      (chunks chunks_future: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
      (b: nat{b < 32})
    : Lemma
        (requires forall (j: nat). j < 32 /\ j <> b ==>
                  Seq.index chunks_future j == Seq.index chunks j)
        (ensures forall (i: nat). i < 256 /\ (i / 8) <> b ==>
                  Seq.index (simd_units_to_array chunks_future) i ==
                  Seq.index (simd_units_to_array chunks) i)
  = let aux (i: nat{i < 256 /\ (i / 8) <> b})
          : Lemma (Seq.index (simd_units_to_array chunks_future) i ==
                   Seq.index (simd_units_to_array chunks) i)
      = let b' : nat = i / 8 in
        let l' : nat = i % 8 in
        assert (b' < 32);
        assert (l' < 8);
        assert (8 * b' + l' == i);
        lemma_simd_units_to_array_reveal chunks_future b' l';
        lemma_simd_units_to_array_reveal chunks b' l'
    in
    Classical.forall_intro aux

(* ===== NTT layer 0 (forward, within-chunk, len=1) =====
   Bridges from the impl's `simd_unit_ntt_at_layer_0_` (4 butterfly
   steps over disjoint lane pairs) to `Hacspec_ml_dsa.Ntt.ntt_layer
   flat 0` indexed on the 8 lanes of one chunk.

   The impl uses Montgomery-form zetas hardcoded inline; the spec uses
   standard-form zetas from `Hacspec_ml_dsa.Ntt.v_ZETAS`.  The bridge
   carries the per-zeta congruence
       (v zeta_mont) % q == (zeta_std * pow2 32) % q
   as an explicit precondition; consumers discharge it with
   `Spec.MLDSA.NttConstants.zeta_r` (already proven for all 256 indices).

   Key Mont identity: `pow2 32 * 8265825 ≡ 1 (mod 8380417)` —
   `assert_norm`'d once in `lemma_butterfly_step_fe` below. *)

(* Per-butterfly-step FE bridge.  Pure algebraic — no impl reveal.
   Hypothesis shape mirrors the FE-form post we will expose on
   `simd_unit_ntt_step` after re-extraction (Step 5).  The impl's
   `simd_unit_ntt_step simd_unit zeta_mont index step` leaves all but
   the two pair lanes unchanged; this lemma talks about the two-lane
   slice (lo_old / hi_old / t / lo_new / hi_new). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_butterfly_step_fe
    (lo_old hi_old t lo_new hi_new zeta_mont: i32)
    (zeta_std: int)
    : Lemma
        (requires
          v lo_new == v lo_old + v t /\
          v hi_new == v lo_old - v t /\
          (v t) % 8380417 == (v hi_old * v zeta_mont * 8265825) % 8380417 /\
          (v zeta_mont) % 8380417 == (zeta_std * pow2 32) % 8380417)
        (ensures
          (v lo_new) % 8380417 == (v lo_old + v hi_old * zeta_std) % 8380417 /\
          (v hi_new) % 8380417 == (v lo_old - v hi_old * zeta_std) % 8380417)
  = let q : pos = 8380417 in
    assert_norm ((pow2 32 * 8265825) % q == 1);
    // Step 1: v zeta_mont * 8265825 ≡ zeta_std (mod q)
    L.lemma_mod_mul_distr_l (v zeta_mont) 8265825 q;
    L.lemma_mod_mul_distr_l (zeta_std * pow2 32) 8265825 q;
    L.lemma_mod_mul_distr_r zeta_std (pow2 32 * 8265825) q;
    assert ((v zeta_mont * 8265825) % q == zeta_std % q);
    // Step 2: v t ≡ v hi_old * zeta_std (mod q)
    L.lemma_mod_mul_distr_r (v hi_old) (v zeta_mont * 8265825) q;
    L.lemma_mod_mul_distr_r (v hi_old) zeta_std q;
    assert ((v hi_old * v zeta_mont * 8265825) % q == (v hi_old * zeta_std) % q);
    assert ((v t) % q == (v hi_old * zeta_std) % q);
    // Step 3: lo_new ≡ lo_old + hi_old * zeta_std (mod q)
    L.lemma_mod_plus_distr_r (v lo_old) (v t) q;
    L.lemma_mod_plus_distr_r (v lo_old) (v hi_old * zeta_std) q;
    // Step 4: hi_new ≡ lo_old - hi_old * zeta_std (mod q)
    L.lemma_mod_sub_distr (v lo_old) (v t) q;
    L.lemma_mod_sub_distr (v lo_old) (v hi_old * zeta_std) q
#pop-options

(* === NTT layer-0 reducer ===
   `layer_0_lane p i` is exactly `Hacspec_ml_dsa.Ntt.ntt_layer p 0` at flat
   index `i`, factored into a top-level definition so the per-lane unfold of
   the `createi`-of-`if` is a one-liner.  Layer 0 has len=1, k=128:
     round = i/2,  idx = i%2,  z = v_ZETAS.[i/2 + 128].
     even (idx=0): mod_q(p.[i] + mod_q(z*p.[i+1])).
     odd  (idx=1): mod_q(p.[i-1] - mod_q(z*p.[i])). *)

(* Reduction lemma: `ntt_layer p 0` at flat index `i` equals `layer_0_lane p i`.
   Discharges via the `createi_lemma` SMTPat + the fact that at layer 0
   `len = 1 << 0 = 1`, `k = 128 / 1 = 128`, `2*len = 2`.  The `ntt_layer`
   createi body is heavy; `--split_queries always` keeps the discharge
   deterministic (otherwise a cold/stale-hint run cancels the monolithic VC
   before splitting). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_0_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

(* v-image of `Hacspec_ml_dsa.Arithmetic.mod_q`.  `mod_q a = cast (a %! q)`
   with the (dead) negative fixup; F* machine `%!` is Euclidean so
   `v (a %! q) = (v a) % q ∈ [0, q-1]`, the fixup branch never fires, and
   the cast i64→i32 is exact.  Hence `v (mod_q a) == (v a) % q`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_mod_q_v (a: i64)
    = let q : i32 = Hacspec_ml_dsa.Parameters.v_Q in
    let cq : i64 = cast q <: i64 in
    assert (v cq == 8380417);
    let r0 : i64 = a %! cq in
    assert (v r0 == (v a) % 8380417);
    L.lemma_mod_lt (v a) 8380417;
    assert (0 <= v r0 /\ v r0 <= 8380416);
    let r : i32 = cast r0 <: i32 in
    assert (v r == (v a) % 8380417)
#pop-options

(* Per-pair butterfly -> spec-lane bridge.  Combines `lemma_butterfly_step_fe`
   (Mont -> mod-q congruence on lo_new/hi_new) with `lemma_mod_q_v` to relate
   the impl's two new lanes to the spec's two `mod_q`-reduced lanes.
   `z` is the standard-form table zeta `v_ZETAS.[4b+p+128]`; `zeta_std = v z`.
   The two `ensures` are exactly the bodies of `layer_0_lane p (8b+2p)`
   (even) and `layer_0_lane p (8b+2p+1)` (odd), with `lo_old = p.[8b+2p]`,
   `hi_old = p.[8b+2p+1]`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_layer_0_pair_spec
    (lo_old hi_old t lo_new hi_new zeta_mont z: i32)
    : Lemma
        (requires
          v lo_new == v lo_old + v t /\
          v hi_new == v lo_old - v t /\
          (v t) % 8380417 == (v hi_old * v zeta_mont * 8265825) % 8380417 /\
          (v zeta_mont) % 8380417 == (v z * pow2 32) % 8380417)
        (ensures
          (let zi : i64 = cast z <: i64 in
           let lo_i : i64 = cast lo_old <: i64 in
           let hi_i : i64 = cast hi_old <: i64 in
           let tt : i32 = Hacspec_ml_dsa.Arithmetic.mod_q (zi *! hi_i <: i64) in
           let even_spec : i32 =
             Hacspec_ml_dsa.Arithmetic.mod_q (lo_i +! (cast tt <: i64) <: i64) in
           let odd_spec : i32 =
             Hacspec_ml_dsa.Arithmetic.mod_q (lo_i -! (cast tt <: i64) <: i64) in
           (v lo_new) % 8380417 == (v even_spec) % 8380417 /\
           (v hi_new) % 8380417 == (v odd_spec) % 8380417))
  = let q : pos = 8380417 in
    lemma_butterfly_step_fe lo_old hi_old t lo_new hi_new zeta_mont (v z);
    // butterfly gives: lo_new ≡ lo_old + hi_old*z, hi_new ≡ lo_old - hi_old*z (mod q)
    assert ((v lo_new) % q == (v lo_old + v hi_old * v z) % q);
    assert ((v hi_new) % q == (v lo_old - v hi_old * v z) % q);
    let zi : i64 = cast z <: i64 in
    let lo_i : i64 = cast lo_old <: i64 in
    let hi_i : i64 = cast hi_old <: i64 in
    assert (v zi == v z /\ v lo_i == v lo_old /\ v hi_i == v hi_old);
    let prod : i64 = zi *! hi_i in
    assert (v prod == v z * v hi_old);
    let tt : i32 = Hacspec_ml_dsa.Arithmetic.mod_q prod in
    lemma_mod_q_v prod;
    assert (v tt == (v z * v hi_old) % q);
    let tt_i : i64 = cast tt <: i64 in
    assert (v tt_i == v tt);
    // even lane
    let even_sum : i64 = lo_i +! tt_i in
    assert (v even_sum == v lo_old + (v z * v hi_old) % q);
    let even_spec : i32 = Hacspec_ml_dsa.Arithmetic.mod_q even_sum in
    lemma_mod_q_v even_sum;
    assert (v even_spec == (v lo_old + (v z * v hi_old) % q) % q);
    L.lemma_mod_plus_distr_r (v lo_old) (v z * v hi_old) q;
    assert ((v lo_old + (v z * v hi_old) % q) % q == (v lo_old + v z * v hi_old) % q);
    // odd lane
    let odd_sub : i64 = lo_i -! tt_i in
    assert (v odd_sub == v lo_old - (v z * v hi_old) % q);
    let odd_spec : i32 = Hacspec_ml_dsa.Arithmetic.mod_q odd_sub in
    lemma_mod_q_v odd_sub;
    assert (v odd_spec == (v lo_old - (v z * v hi_old) % q) % q);
    L.lemma_mod_sub_distr (v lo_old) (v z * v hi_old) q;
    assert ((v lo_old - (v z * v hi_old) % q) % q == (v lo_old - v z * v hi_old) % q);
    // tie together: hi_old*z == z*hi_old
    assert (v hi_old * v z == v z * v hi_old)
#pop-options

(* === Per-chunk lane bridge: lemma_ntt_layer_0_chunk_to_hacspec ===

   Relates the impl's within-chunk layer-0 transform of ONE chunk `b` to
   `Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array input) 0` on chunk
   `b`'s 8 flat lanes.  The impl applies 4 within-pair butterflies on lane
   pairs (0,1),(2,3),(4,5),(6,7) of chunk `b`; the spec, restricted to those
   8 indices, is exactly that (len=1, k=128, pair index 4b+p).

   `input` / `transformed` are the flat-array views BEFORE/AFTER the chunk-b
   transform.  Per pair p∈{0..3}, the consumer supplies the witness `t_p`
   (the Montgomery butterfly product) and `zeta_mont_p` (the impl's
   hardcoded Mont-form zeta), together with the four butterfly relations
   that the impl's `simd_unit_ntt_step` FE-post provides, and the per-zeta
   congruence `(v zeta_mont_p) % q == (v v_ZETAS.[4b+p+128] * pow2 32) % q`
   (consumer discharges via `Spec.MLDSA.NttConstants.zeta_r`).

   Conclusion: per-lane mod-q congruence on chunk b's 8 lanes (the impl is
   in bounded Montgomery form, NOT reduced to [0,q), so equality is only
   modulo q — exactly as `lemma_butterfly_step_fe` states).  The hypotheses
   tie `transformed`/`input` to a single chunk via `simd_units_to_array`'s
   index reveal; chunk-`b` lanes are read on both sides. *)
(* Per-pair spec-lane bridge in CLEAN context (one VC per pair).  For chunk
   `b` pair `p`, given the four butterfly relations + zeta congruence on that
   pair's lanes, proves the two spec-lane congruences (even = 8b+2p,
   odd = 8b+2p+1).  Reduces the spec via `lemma_ntt_layer_0_lane` and bridges
   via `lemma_layer_0_pair_spec`.  Factored top-level so the createi unfold
   runs in a minimal context (the in-monolithic-chunk version saturated). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_0_chunk_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32}) (p: nat{p < 4})
    (tp zmp: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] in
           v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v tp /\
           v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci (2*p+1)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co = Seq.index transformed b in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
           (v (Seq.index co (2*p)))   % 8380417 == (v (Seq.index spec (8*b + 2*p)))   % 8380417 /\
           (v (Seq.index co (2*p+1))) % 8380417 == (v (Seq.index spec (8*b + 2*p+1))) % 8380417))
  = let q : pos = 8380417 in
    let ci = Seq.index input b in
    let co = Seq.index transformed b in
    let in_flat = simd_units_to_array input in
    let lo_old = Seq.index ci (2*p) in
    let hi_old = Seq.index ci (2*p+1) in
    let lo_new = Seq.index co (2*p) in
    let hi_new = Seq.index co (2*p+1) in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] in
    let i_even : usize = mk_usize (8*b + 2*p) in
    let i_odd  : usize = mk_usize (8*b + 2*p + 1) in
    // in_flat lanes at i_even / i_odd are ci.[2p] / ci.[2p+1].
    lemma_simd_units_to_array_reveal input b (2*p);
    lemma_simd_units_to_array_reveal input b (2*p+1);
    assert (Seq.index in_flat (8*b + 2*p) == lo_old);
    assert (Seq.index in_flat (8*b + 2*p + 1) == hi_old);
    // index identities so layer_0_lane unfolds to the matching pair / zeta.
    assert (v i_even == 8*b + 2*p);
    assert (v i_odd == 8*b + 2*p + 1);
    assert ((8*b + 2*p) / 2 == 4*b + p);
    assert ((8*b + 2*p + 1) / 2 == 4*b + p);
    assert ((8*b + 2*p) % 2 == 0);
    assert ((8*b + 2*p + 1) % 2 == 1);
    // reduce the two spec lanes to layer_0_lane (createi unfold, minimal context)
    lemma_ntt_layer_0_lane in_flat i_even;
    lemma_ntt_layer_0_lane in_flat i_odd;
    // bridge the impl pair to the two spec mod_q lanes
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

(* === Per-chunk lane bridge: lemma_ntt_layer_0_chunk_to_hacspec ===

   Relates the impl's within-chunk layer-0 transform of ONE chunk `b` to
   `Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array input) 0` on chunk
   `b`'s 8 flat lanes.  The impl applies 4 within-pair butterflies on lane
   pairs (0,1),(2,3),(4,5),(6,7) of chunk `b`; the spec, restricted to those
   8 indices, is exactly that (len=1, k=128, pair index 4b+p).

   `input` / `transformed` are the flat-array views BEFORE/AFTER the chunk-b
   transform.  Per pair p∈{0..3}, the consumer supplies the witness `t_p`
   (the Montgomery butterfly product) and `zeta_mont_p` (the impl's
   hardcoded Mont-form zeta), together with the four butterfly relations
   that the impl's `simd_unit_ntt_step` FE-post provides, and the per-zeta
   congruence `(v zeta_mont_p) % q == (v v_ZETAS.[4b+p+128] * pow2 32) % q`
   (consumer discharges via `Spec.MLDSA.NttConstants.zeta_r`).

   Conclusion: per-lane mod-q congruence on chunk b's 8 lanes (the impl is
   in bounded Montgomery form, NOT reduced to [0,q), so equality is only
   modulo q — exactly as `lemma_butterfly_step_fe` states).  Thin dispatcher
   over the 4 clean per-pair lemmas + a forall over the 8 lanes. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_0_chunk_to_hacspec
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32})
    (t0 t1 t2 t3 zm0 zm1 zm2 zm3: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z (p:nat{p<4}) : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] in
           (* pair 0: lanes 0,1 *)
           v (Seq.index co 0) == v (Seq.index ci 0) + v t0 /\
           v (Seq.index co 1) == v (Seq.index ci 0) - v t0 /\
           (v t0) % 8380417 == (v (Seq.index ci 1) * v zm0 * 8265825) % 8380417 /\
           (v zm0) % 8380417 == (v (z 0) * pow2 32) % 8380417 /\
           (* pair 1: lanes 2,3 *)
           v (Seq.index co 2) == v (Seq.index ci 2) + v t1 /\
           v (Seq.index co 3) == v (Seq.index ci 2) - v t1 /\
           (v t1) % 8380417 == (v (Seq.index ci 3) * v zm1 * 8265825) % 8380417 /\
           (v zm1) % 8380417 == (v (z 1) * pow2 32) % 8380417 /\
           (* pair 2: lanes 4,5 *)
           v (Seq.index co 4) == v (Seq.index ci 4) + v t2 /\
           v (Seq.index co 5) == v (Seq.index ci 4) - v t2 /\
           (v t2) % 8380417 == (v (Seq.index ci 5) * v zm2 * 8265825) % 8380417 /\
           (v zm2) % 8380417 == (v (z 2) * pow2 32) % 8380417 /\
           (* pair 3: lanes 6,7 *)
           v (Seq.index co 6) == v (Seq.index ci 6) + v t3 /\
           v (Seq.index co 7) == v (Seq.index ci 6) - v t3 /\
           (v t3) % 8380417 == (v (Seq.index ci 7) * v zm3 * 8265825) % 8380417 /\
           (v zm3) % 8380417 == (v (z 3) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
           (forall (l: nat). l < 8 ==>
             (v (Seq.index out_flat (8*b + l))) % 8380417 ==
             (v (Seq.index spec (8*b + l))) % 8380417)))
  = let q : pos = 8380417 in
    let co = Seq.index transformed b in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array input) (mk_usize 0) in
    // discharge each pair in its own clean lemma
    lemma_layer_0_chunk_pair input transformed b 0 t0 zm0;
    lemma_layer_0_chunk_pair input transformed b 1 t1 zm1;
    lemma_layer_0_chunk_pair input transformed b 2 t2 zm2;
    lemma_layer_0_chunk_pair input transformed b 3 t3 zm3;
    // per-lane: pick pair p = l/2, reveal out_flat lane, case even/odd.
    let aux (l: nat{l < 8}) : Lemma
        ((v (Seq.index out_flat (8*b + l))) % q == (v (Seq.index spec (8*b + l))) % q)
      = let p : nat = l / 2 in
        assert (p < 4);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index co l);
        if l % 2 = 0 then begin
          assert (l == 2*p);
          assert (Seq.index co l == Seq.index co (2*p))
        end else begin
          assert (l == 2*p + 1);
          assert (Seq.index co l == Seq.index co (2*p + 1))
        end
    in
    Classical.forall_intro aux
#pop-options

(* === All-32-chunk composition: lemma_ntt_layer_0_step_to_hacspec_poly ===

   Composes `lemma_ntt_layer_0_chunk_to_hacspec` over all 32 chunks into the
   poly-level statement: every flat lane of `transformed` is mod-q congruent
   to `Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array input) 0`.

   The per-chunk butterfly witnesses — the Montgomery product `t b p` and the
   impl's hardcoded Mont-form zeta `zm b p`, for pair p of chunk b — are
   supplied as total witness FUNCTIONS over (chunk, pair); the consumer
   (the impl's `ntt_at_layer_0` post) instantiates them from the per-round
   `simd_unit_ntt_step` FE-posts and discharges the per-zeta congruence via
   `Spec.MLDSA.NttConstants.zeta_r`.  The hypothesis is the same 4-relation butterfly
   shape as `lemma_ntt_layer_0_chunk_to_hacspec`'s requires, universally
   quantified over (b, p). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_0_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t zm: (b: nat{b < 32} -> p: nat{p < 4} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
    let chunk (b: nat{b < 32}) : Lemma
        (forall (l: nat). l < 8 ==>
           (v (Seq.index out_flat (8*b + l))) % q ==
           (v (Seq.index spec (8*b + l))) % q)
      = lemma_ntt_layer_0_chunk_to_hacspec input transformed b
          (t b 0) (t b 1) (t b 2) (t b 3)
          (zm b 0) (zm b 1) (zm b 2) (zm b 3)
    in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let b : nat = i / 8 in
        let l : nat = i % 8 in
        assert (b < 32 /\ l < 8);
        assert (8*b + l == i);
        chunk b;
        assert ((v (Seq.index out_flat (8*b + l))) % q ==
                (v (Seq.index spec (8*b + l))) % q)
    in
    Classical.forall_intro aux
#pop-options

(* ===== NTT layer 1 (forward, within-chunk, len=2) =====
   Same shape as layer 0 with stride-2 pairs: within one chunk the 4
   butterflies act on lane pairs (0,2),(1,3),(4,6),(5,7).  Pairs are
   indexed by (half h, sub-index j): lanes (4h+j, 4h+j+2).  The spec
   zeta for half h of chunk b is `v_ZETAS.[2b + h + 64]` (round = i/4,
   k = 128/len = 64) — ONE zeta per half, shared by its two pairs,
   matching the impl's `simd_unit_ntt_at_layer_1 (zeta1, zeta2)`.
   The per-pair algebra is layer-agnostic: `lemma_layer_0_pair_spec`
   is reused as-is. *)

(* layer-1 reducer: `ntt_layer p 1` at flat index `i`.  len=2, k=64:
     round = i/4,  idx = i%4,  z = v_ZETAS.[i/4 + 64].
     lo  (idx<2):  mod_q(p.[i] + mod_q(z*p.[i+2])).
     hi  (idx>=2): mod_q(p.[i-2] - mod_q(z*p.[i])). *)

(* Reduction lemma: `ntt_layer p 1` at flat index `i` equals `layer_1_lane p i`.
   Mirrors `lemma_ntt_layer_0_lane` (len = 1 << 1 = 2, k = 128/2 = 64,
   2*len = 4). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_1_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

(* Per-pair clean-context spec-lane bridge for layer 1.  Pair (h, j) of
   chunk `b` acts on lanes (4h+j, 4h+j+2); the spec zeta is
   `v_ZETAS.[2b + h + 64]`.  Same structure as `lemma_layer_0_chunk_pair`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_1_chunk_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2})
    (tp zmp: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + h + 64) ] in
           v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v tp /\
           v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co = Seq.index transformed b in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
           (v (Seq.index co (4*h+j)))   % 8380417 == (v (Seq.index spec (8*b + 4*h+j)))   % 8380417 /\
           (v (Seq.index co (4*h+j+2))) % 8380417 == (v (Seq.index spec (8*b + 4*h+j+2))) % 8380417))
  = let q : pos = 8380417 in
    let ci = Seq.index input b in
    let co = Seq.index transformed b in
    let in_flat = simd_units_to_array input in
    let lo_old = Seq.index ci (4*h+j) in
    let hi_old = Seq.index ci (4*h+j+2) in
    let lo_new = Seq.index co (4*h+j) in
    let hi_new = Seq.index co (4*h+j+2) in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + h + 64) ] in
    let i_lo : usize = mk_usize (8*b + 4*h + j) in
    let i_hi : usize = mk_usize (8*b + 4*h + j + 2) in
    // in_flat lanes at i_lo / i_hi are ci.[4h+j] / ci.[4h+j+2].
    lemma_simd_units_to_array_reveal input b (4*h+j);
    lemma_simd_units_to_array_reveal input b (4*h+j+2);
    assert (Seq.index in_flat (8*b + 4*h+j) == lo_old);
    assert (Seq.index in_flat (8*b + 4*h+j+2) == hi_old);
    // index identities so layer_1_lane unfolds to the matching pair / zeta.
    assert (v i_lo == 8*b + 4*h + j);
    assert (v i_hi == 8*b + 4*h + j + 2);
    assert ((8*b + 4*h + j) / 4 == 2*b + h);
    assert ((8*b + 4*h + j + 2) / 4 == 2*b + h);
    assert ((8*b + 4*h + j) % 4 == j);
    assert ((8*b + 4*h + j + 2) % 4 == j + 2);
    // reduce the two spec lanes to layer_1_lane (createi unfold, minimal context)
    lemma_ntt_layer_1_lane in_flat i_lo;
    lemma_ntt_layer_1_lane in_flat i_hi;
    // bridge the impl pair to the two spec mod_q lanes (layer-agnostic algebra)
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

(* Per-chunk 8-lane bridge for layer 1.  Thin dispatcher over the 4
   (h, j) pair lemmas + a forall over the 8 lanes.  Witnesses: per-pair
   Montgomery products t_hj, ONE Mont-form zeta per half (zm0, zm1) —
   matching the impl's `simd_unit_ntt_at_layer_1 (zeta1, zeta2)`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_1_chunk_to_hacspec
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32})
    (t00 t01 t10 t11 zm0 zm1: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z (h:nat{h<2}) : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + h + 64) ] in
           (* half 0, pair j=0: lanes 0,2 *)
           v (Seq.index co 0) == v (Seq.index ci 0) + v t00 /\
           v (Seq.index co 2) == v (Seq.index ci 0) - v t00 /\
           (v t00) % 8380417 == (v (Seq.index ci 2) * v zm0 * 8265825) % 8380417 /\
           (* half 0, pair j=1: lanes 1,3 *)
           v (Seq.index co 1) == v (Seq.index ci 1) + v t01 /\
           v (Seq.index co 3) == v (Seq.index ci 1) - v t01 /\
           (v t01) % 8380417 == (v (Seq.index ci 3) * v zm0 * 8265825) % 8380417 /\
           (* half 1, pair j=0: lanes 4,6 *)
           v (Seq.index co 4) == v (Seq.index ci 4) + v t10 /\
           v (Seq.index co 6) == v (Seq.index ci 4) - v t10 /\
           (v t10) % 8380417 == (v (Seq.index ci 6) * v zm1 * 8265825) % 8380417 /\
           (* half 1, pair j=1: lanes 5,7 *)
           v (Seq.index co 5) == v (Seq.index ci 5) + v t11 /\
           v (Seq.index co 7) == v (Seq.index ci 5) - v t11 /\
           (v t11) % 8380417 == (v (Seq.index ci 7) * v zm1 * 8265825) % 8380417 /\
           (* zeta congruences: one per half *)
           (v zm0) % 8380417 == (v (z 0) * pow2 32) % 8380417 /\
           (v zm1) % 8380417 == (v (z 1) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
           (forall (l: nat). l < 8 ==>
             (v (Seq.index out_flat (8*b + l))) % 8380417 ==
             (v (Seq.index spec (8*b + l))) % 8380417)))
  = let q : pos = 8380417 in
    let co = Seq.index transformed b in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array input) (mk_usize 1) in
    // discharge each (h, j) pair in its own clean lemma
    lemma_layer_1_chunk_pair input transformed b 0 0 t00 zm0;
    lemma_layer_1_chunk_pair input transformed b 0 1 t01 zm0;
    lemma_layer_1_chunk_pair input transformed b 1 0 t10 zm1;
    lemma_layer_1_chunk_pair input transformed b 1 1 t11 zm1;
    // per-lane: half h = l/4, sub-index j = l%2; lo lane iff l%4 < 2.
    let aux (l: nat{l < 8}) : Lemma
        ((v (Seq.index out_flat (8*b + l))) % q == (v (Seq.index spec (8*b + l))) % q)
      = let h : nat = l / 4 in
        let j : nat = l % 2 in
        assert (h < 2 /\ j < 2);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index co l);
        if l % 4 < 2 then begin
          assert (l == 4*h + j);
          assert (Seq.index co l == Seq.index co (4*h + j))
        end else begin
          assert (l == 4*h + j + 2);
          assert (Seq.index co l == Seq.index co (4*h + j + 2))
        end
    in
    Classical.forall_intro aux
#pop-options

(* All-32-chunk composition for layer 1.  Witness functions: `t b h j`
   (Montgomery butterfly product for pair (h,j) of chunk b) and `zm b h`
   (the impl's Mont-form zeta for half h of chunk b). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_1_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (b: nat{b < 32} -> h: nat{h < 2} -> j: nat{j < 2} -> i32))
    (zm: (b: nat{b < 32} -> h: nat{h < 2} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
    let chunk (b: nat{b < 32}) : Lemma
        (forall (l: nat). l < 8 ==>
           (v (Seq.index out_flat (8*b + l))) % q ==
           (v (Seq.index spec (8*b + l))) % q)
      = lemma_ntt_layer_1_chunk_to_hacspec input transformed b
          (t b 0 0) (t b 0 1) (t b 1 0) (t b 1 1)
          (zm b 0) (zm b 1)
    in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let b : nat = i / 8 in
        let l : nat = i % 8 in
        assert (b < 32 /\ l < 8);
        assert (8*b + l == i);
        chunk b;
        assert ((v (Seq.index out_flat (8*b + l))) % q ==
                (v (Seq.index spec (8*b + l))) % q)
    in
    Classical.forall_intro aux
#pop-options

(* ===== NTT layer 2 (forward, within-chunk, len=4) =====
   Stride-4 pairs: within one chunk the 4 butterflies act on lane pairs
   (p, p+4) for p in 0..3, ALL sharing the single spec zeta
   `v_ZETAS.[b + 32]` (round = i/8, k = 128/len = 32) — matching the
   impl's `simd_unit_ntt_at_layer_2 (zeta)` (one zeta per chunk).
   The per-pair algebra is again `lemma_layer_0_pair_spec`. *)

(* layer-2 reducer: `ntt_layer p 2` at flat index `i`.  len=4, k=32:
     round = i/8,  idx = i%8,  z = v_ZETAS.[i/8 + 32].
     lo  (idx<4):  mod_q(p.[i] + mod_q(z*p.[i+4])).
     hi  (idx>=4): mod_q(p.[i-4] - mod_q(z*p.[i])). *)

(* Reduction lemma: `ntt_layer p 2` at flat index `i` equals `layer_2_lane p i`.
   Mirrors `lemma_ntt_layer_0_lane` (len = 1 << 2 = 4, k = 128/4 = 32,
   2*len = 8). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_2_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

(* Per-pair clean-context spec-lane bridge for layer 2.  Pair `p` of
   chunk `b` acts on lanes (p, p+4); the spec zeta is `v_ZETAS.[b + 32]`
   (shared by all 4 pairs).  Same structure as `lemma_layer_0_chunk_pair`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_2_chunk_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32}) (p: nat{p < 4})
    (tp zmp: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] in
           v (Seq.index co p)     == v (Seq.index ci p) + v tp /\
           v (Seq.index co (p+4)) == v (Seq.index ci p) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci (p+4)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co = Seq.index transformed b in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
           (v (Seq.index co p))     % 8380417 == (v (Seq.index spec (8*b + p)))   % 8380417 /\
           (v (Seq.index co (p+4))) % 8380417 == (v (Seq.index spec (8*b + p+4))) % 8380417))
  = let q : pos = 8380417 in
    let ci = Seq.index input b in
    let co = Seq.index transformed b in
    let in_flat = simd_units_to_array input in
    let lo_old = Seq.index ci p in
    let hi_old = Seq.index ci (p+4) in
    let lo_new = Seq.index co p in
    let hi_new = Seq.index co (p+4) in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] in
    let i_lo : usize = mk_usize (8*b + p) in
    let i_hi : usize = mk_usize (8*b + p + 4) in
    // in_flat lanes at i_lo / i_hi are ci.[p] / ci.[p+4].
    lemma_simd_units_to_array_reveal input b p;
    lemma_simd_units_to_array_reveal input b (p+4);
    assert (Seq.index in_flat (8*b + p) == lo_old);
    assert (Seq.index in_flat (8*b + p+4) == hi_old);
    // index identities so layer_2_lane unfolds to the matching pair / zeta.
    assert (v i_lo == 8*b + p);
    assert (v i_hi == 8*b + p + 4);
    assert ((8*b + p) / 8 == b);
    assert ((8*b + p + 4) / 8 == b);
    assert ((8*b + p) % 8 == p);
    assert ((8*b + p + 4) % 8 == p + 4);
    // reduce the two spec lanes to layer_2_lane (createi unfold, minimal context)
    lemma_ntt_layer_2_lane in_flat i_lo;
    lemma_ntt_layer_2_lane in_flat i_hi;
    // bridge the impl pair to the two spec mod_q lanes (layer-agnostic algebra)
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

(* Per-chunk 8-lane bridge for layer 2.  Thin dispatcher over the 4 pair
   lemmas + a forall over the 8 lanes.  Witnesses: per-pair Montgomery
   products t0..t3, ONE Mont-form zeta `zm` for the whole chunk —
   matching the impl's `simd_unit_ntt_at_layer_2 (zeta)`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_2_chunk_to_hacspec
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32})
    (t0 t1 t2 t3 zm: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] in
           (* pair 0: lanes 0,4 *)
           v (Seq.index co 0) == v (Seq.index ci 0) + v t0 /\
           v (Seq.index co 4) == v (Seq.index ci 0) - v t0 /\
           (v t0) % 8380417 == (v (Seq.index ci 4) * v zm * 8265825) % 8380417 /\
           (* pair 1: lanes 1,5 *)
           v (Seq.index co 1) == v (Seq.index ci 1) + v t1 /\
           v (Seq.index co 5) == v (Seq.index ci 1) - v t1 /\
           (v t1) % 8380417 == (v (Seq.index ci 5) * v zm * 8265825) % 8380417 /\
           (* pair 2: lanes 2,6 *)
           v (Seq.index co 2) == v (Seq.index ci 2) + v t2 /\
           v (Seq.index co 6) == v (Seq.index ci 2) - v t2 /\
           (v t2) % 8380417 == (v (Seq.index ci 6) * v zm * 8265825) % 8380417 /\
           (* pair 3: lanes 3,7 *)
           v (Seq.index co 3) == v (Seq.index ci 3) + v t3 /\
           v (Seq.index co 7) == v (Seq.index ci 3) - v t3 /\
           (v t3) % 8380417 == (v (Seq.index ci 7) * v zm * 8265825) % 8380417 /\
           (* zeta congruence: one per chunk *)
           (v zm) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
           (forall (l: nat). l < 8 ==>
             (v (Seq.index out_flat (8*b + l))) % 8380417 ==
             (v (Seq.index spec (8*b + l))) % 8380417)))
  = let q : pos = 8380417 in
    let co = Seq.index transformed b in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array input) (mk_usize 2) in
    // discharge each pair in its own clean lemma
    lemma_layer_2_chunk_pair input transformed b 0 t0 zm;
    lemma_layer_2_chunk_pair input transformed b 1 t1 zm;
    lemma_layer_2_chunk_pair input transformed b 2 t2 zm;
    lemma_layer_2_chunk_pair input transformed b 3 t3 zm;
    // per-lane: pair p = l%4; lo lane iff l < 4.
    let aux (l: nat{l < 8}) : Lemma
        ((v (Seq.index out_flat (8*b + l))) % q == (v (Seq.index spec (8*b + l))) % q)
      = let p : nat = l % 4 in
        assert (p < 4);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index co l);
        if l < 4 then begin
          assert (l == p);
          assert (Seq.index co l == Seq.index co p)
        end else begin
          assert (l == p + 4);
          assert (Seq.index co l == Seq.index co (p + 4))
        end
    in
    Classical.forall_intro aux
#pop-options

(* All-32-chunk composition for layer 2.  Witness functions: `t b p`
   (Montgomery butterfly product for pair p of chunk b) and `zm b`
   (the impl's Mont-form zeta for chunk b). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_2_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (b: nat{b < 32} -> p: nat{p < 4} -> i32))
    (zm: (b: nat{b < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
    let chunk (b: nat{b < 32}) : Lemma
        (forall (l: nat). l < 8 ==>
           (v (Seq.index out_flat (8*b + l))) % q ==
           (v (Seq.index spec (8*b + l))) % q)
      = lemma_ntt_layer_2_chunk_to_hacspec input transformed b
          (t b 0) (t b 1) (t b 2) (t b 3)
          (zm b)
    in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let b : nat = i / 8 in
        let l : nat = i % 8 in
        assert (b < 32 /\ l < 8);
        assert (8*b + l == i);
        chunk b;
        assert ((v (Seq.index out_flat (8*b + l))) % q ==
                (v (Seq.index spec (8*b + l))) % q)
    in
    Classical.forall_intro aux
#pop-options

(* ===== Cross-chunk forward-NTT layers 3-7 ====================================
   The butterfly pairs span TWO simd units (unit u lane l <-> unit u+step_by
   lane l) sharing ONE zeta across all 8 lanes.  Mirrors the within-chunk
   layer-0/1/2 bridge (layer_2_lane / lemma_ntt_layer_2_lane /
   lemma_layer_2_chunk_pair / lemma_ntt_layer_2_step_to_hacspec_poly) but with
   the cross-unit geometry.  Reuses the layer-agnostic butterfly algebra
   lemma_layer_0_pair_spec and one generic index lemma below.

   Layer L: len = 1<<L = 8*step_by coefficients, step_by = len/8 units,
   k = 128/len.  step_by/len/k per layer:
     L=3: step_by=1,  len=8,   k=16
     L=4: step_by=2,  len=16,  k=8
     L=5: step_by=4,  len=32,  k=4
     L=6: step_by=8,  len=64,  k=2
     L=7: step_by=16, len=128, k=1
   A lo-unit ulo satisfies ulo % (2*step_by) < step_by and pairs with
   ulo+step_by; block c = ulo/(2*step_by); zeta index = c + k. *)

(* Generic flat-index arithmetic for one lo-unit pair.  Discharges, for
   step_by = s (any power-of-two divisor of 16), the round/idx facts that
   layer_L_lane needs at the two flat indices 8*ulo+l (lo) and 8*ulo+8s+l
   (hi), plus the hi-partner unit bound ulo+s < 32. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_cross_idx (s:pos{16 % s == 0 /\ s <= 16}) (ulo:nat{ulo < 32 /\ ulo % (2*s) < s}) (l:nat{l<8})
  = let c = ulo / (2*s) in
    let r = ulo % (2*s) in
    L.lemma_div_mod ulo (2*s);
    assert (ulo == (2*s)*c + r);
    assert (r < s);
    L.lemma_div_mod 32 (2*s);
    assert (32 == (2*s)*(32/(2*s)));
    assert ((2*s)*c < (2*s)*(32/(2*s)));
    L.lemma_mult_lt_left (2*s) c (32/(2*s));
    assert (c < 32/(2*s));
    L.lemma_mult_le_left (2*s) (c+1) (32/(2*s));
    assert ((2*s)*(c+1) <= 32);
    assert (ulo + s < (2*s)*(c+1));
    assert (8*ulo == 16*s*c + 8*r);
    assert (8*ulo + l == (8*r+l) + c*(16*s));
    L.lemma_div_plus (8*r+l) c (16*s);
    L.lemma_mod_plus (8*r+l) c (16*s);
    L.small_div (8*r+l) (16*s);
    L.small_mod (8*r+l) (16*s);
    assert (8*ulo + 8*s + l == (8*s+8*r+l) + c*(16*s));
    L.lemma_div_plus (8*s+8*r+l) c (16*s);
    L.lemma_mod_plus (8*s+8*r+l) c (16*s);
    L.small_div (8*s+8*r+l) (16*s);
    L.small_mod (8*s+8*r+l) (16*s)
#pop-options

///////////////////////////////////////////////////////////////////////////////
// LAYER 3 (step_by=1, len=8, k=16)
///////////////////////////////////////////////////////////////////////////////

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_3_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_3_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 2 == 0}) (l: nat{l < 8}) (tp zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+1) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+1) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/2 + 16) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v tp /\
           v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci_hi l) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+1) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))     % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 8 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+1)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+1)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/2 + 16) ] in
    lemma_cross_idx 1 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+1) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+1) + l == 8*ulo + 8 + l);
    assert (Seq.index in_flat (8*ulo + 8 + l) == hi_old);
    lemma_ntt_layer_3_lane in_flat (mk_usize (8*ulo + l));
    lemma_ntt_layer_3_lane in_flat (mk_usize (8*ulo + 8 + l));
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_3_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 2 = 0 then begin
          lemma_layer_3_cross_pair input transformed u l (t u l) (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 1 in
          L.lemma_div_mod u 2;
          assert (ulo % 2 == 0 /\ ulo + 1 == u /\ 8*ulo + 8 + l == 8*u + l);
          lemma_layer_3_cross_pair input transformed ulo l (t ulo l) (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

///////////////////////////////////////////////////////////////////////////////
// LAYER 4 (step_by=2, len=16, k=8)
///////////////////////////////////////////////////////////////////////////////

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_4_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_4_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 4 < 2}) (l: nat{l < 8}) (tp zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+2) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+2) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/4 + 8) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v tp /\
           v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci_hi l) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+2) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))      % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 16 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+2)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+2)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/4 + 8) ] in
    lemma_cross_idx 2 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+2) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+2) + l == 8*ulo + 16 + l);
    assert (Seq.index in_flat (8*ulo + 16 + l) == hi_old);
    lemma_ntt_layer_4_lane in_flat (mk_usize (8*ulo + l));
    lemma_ntt_layer_4_lane in_flat (mk_usize (8*ulo + 16 + l));
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_4_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 4 < 2 then begin
          lemma_layer_4_cross_pair input transformed u l (t u l) (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 2 in
          L.lemma_div_mod u 4;
          assert (u % 4 >= 2 /\ u >= 2);
          assert (ulo % 4 < 2 /\ ulo + 2 == u /\ 8*ulo + 16 + l == 8*u + l);
          lemma_layer_4_cross_pair input transformed ulo l (t ulo l) (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

///////////////////////////////////////////////////////////////////////////////
// LAYER 5 (step_by=4, len=32, k=4)
///////////////////////////////////////////////////////////////////////////////

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_5_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_5_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 8 < 4}) (l: nat{l < 8}) (tp zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+4) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+4) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/8 + 4) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v tp /\
           v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci_hi l) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+4) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))      % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 32 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+4)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+4)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/8 + 4) ] in
    lemma_cross_idx 4 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+4) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+4) + l == 8*ulo + 32 + l);
    assert (Seq.index in_flat (8*ulo + 32 + l) == hi_old);
    lemma_ntt_layer_5_lane in_flat (mk_usize (8*ulo + l));
    lemma_ntt_layer_5_lane in_flat (mk_usize (8*ulo + 32 + l));
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_5_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 8 < 4 then begin
          lemma_layer_5_cross_pair input transformed u l (t u l) (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 4 in
          L.lemma_div_mod u 8;
          assert (u % 8 >= 4 /\ u >= 4);
          assert (ulo % 8 < 4 /\ ulo + 4 == u /\ 8*ulo + 32 + l == 8*u + l);
          lemma_layer_5_cross_pair input transformed ulo l (t ulo l) (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

///////////////////////////////////////////////////////////////////////////////
// LAYER 6 (step_by=8, len=64, k=2)
///////////////////////////////////////////////////////////////////////////////

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_6_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_6_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 16 < 8}) (l: nat{l < 8}) (tp zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+8) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+8) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/16 + 2) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v tp /\
           v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci_hi l) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+8) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 6) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))      % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 64 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+8)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+8)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/16 + 2) ] in
    lemma_cross_idx 8 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+8) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+8) + l == 8*ulo + 64 + l);
    assert (Seq.index in_flat (8*ulo + 64 + l) == hi_old);
    lemma_ntt_layer_6_lane in_flat (mk_usize (8*ulo + l));
    lemma_ntt_layer_6_lane in_flat (mk_usize (8*ulo + 64 + l));
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_6_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 6) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 16 < 8 then begin
          lemma_layer_6_cross_pair input transformed u l (t u l) (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 8 in
          L.lemma_div_mod u 16;
          assert (u % 16 >= 8 /\ u >= 8);
          assert (ulo % 16 < 8 /\ ulo + 8 == u /\ 8*ulo + 64 + l == 8*u + l);
          lemma_layer_6_cross_pair input transformed ulo l (t ulo l) (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

///////////////////////////////////////////////////////////////////////////////
// LAYER 7 (step_by=16, len=128, k=1)
///////////////////////////////////////////////////////////////////////////////

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let layer_7_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 256 in
  let idx:usize = i %! mk_usize 256 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 1 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 128
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 128 <: usize ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +! (cast (t <: i32) <: i64) <: i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 128 <: usize ] <: i32) <: i64) -! (cast (t <: i32) <: i64) <: i64)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_ntt_layer_7_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 7)) (v i) == layer_7_lane p i) = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer_7_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 32 < 16}) (l: nat{l < 8}) (tp zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+16) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+16) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/32 + 1) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v tp /\
           v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v tp /\
           (v tp) % 8380417 == (v (Seq.index ci_hi l) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+16) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))       % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 128 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+16)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+16)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/32 + 1) ] in
    lemma_cross_idx 16 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+16) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+16) + l == 8*ulo + 128 + l);
    assert (Seq.index in_flat (8*ulo + 128 + l) == hi_old);
    lemma_ntt_layer_7_lane in_flat (mk_usize (8*ulo + l));
    lemma_ntt_layer_7_lane in_flat (mk_usize (8*ulo + 128 + l));
    lemma_layer_0_pair_spec lo_old hi_old tp lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_7_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 32 < 16 then begin
          lemma_layer_7_cross_pair input transformed u l (t u l) (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 16 in
          L.lemma_div_mod u 32;
          assert (u % 32 >= 16 /\ u >= 16);
          assert (ulo % 32 < 16 /\ ulo + 16 == u /\ 8*ulo + 128 + l == 8*u + l);
          lemma_layer_7_cross_pair input transformed ulo l (t ulo l) (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options


(* === Reusable spec zeta-table bridge: v_ZETAS[i] == Spec.MLDSA.NttConstants.zeta i ===
   Ties the extracted table `v_ZETAS` (= array_of_list 256 <list> =
   Seq.seq_of_list <list>), read by the canonical `Hacspec_ml_dsa.Ntt.ntt_layer`,
   to `Spec.MLDSA.NttConstants.zeta` (a `match`), which `zeta_r` relates to the impl's
   Mont zetas.  Lets a driver discharge the per-pair zeta congruence via
   `zeta_r` (symbolic) rather than per-index.  v_ZETAS and `zeta` agree for
   i>=1 (v_ZETAS.[0]=1 vs zeta 0 = 0; unused at any butterfly).

   RECIPE (ntt-functional-layers012 doc): F* won't auto-unfold the plain `let`
   v_ZETAS to seq_of_list (SMTPat won't fire; assert_norm through Seq.index
   hangs).  Fix: verbatim list copy + tactic-forced trefl unfold (pure defeq,
   no Seq normalization) + explicit lemma_seq_of_list_index + per-index
   assert_norm over the reducible List.Tot.index. *)
let zetas_list_dsa : list i32 = [mk_i32 1; mk_i32 4808194; mk_i32 3765607; mk_i32 3761513; mk_i32 5178923; mk_i32 5496691; mk_i32 5234739; mk_i32 5178987; mk_i32 7778734; mk_i32 3542485; mk_i32 2682288; mk_i32 2129892; mk_i32 3764867; mk_i32 7375178; mk_i32 557458; mk_i32 7159240; mk_i32 5010068; mk_i32 4317364; mk_i32 2663378; mk_i32 6705802; mk_i32 4855975; mk_i32 7946292; mk_i32 676590; mk_i32 7044481; mk_i32 5152541; mk_i32 1714295; mk_i32 2453983; mk_i32 1460718; mk_i32 7737789; mk_i32 4795319; mk_i32 2815639; mk_i32 2283733; mk_i32 3602218; mk_i32 3182878; mk_i32 2740543; mk_i32 4793971; mk_i32 5269599; mk_i32 2101410; mk_i32 3704823; mk_i32 1159875; mk_i32 394148; mk_i32 928749; mk_i32 1095468; mk_i32 4874037; mk_i32 2071829; mk_i32 4361428; mk_i32 3241972; mk_i32 2156050; mk_i32 3415069; mk_i32 1759347; mk_i32 7562881; mk_i32 4805951; mk_i32 3756790; mk_i32 6444618; mk_i32 6663429; mk_i32 4430364; mk_i32 5483103; mk_i32 3192354; mk_i32 556856; mk_i32 3870317; mk_i32 2917338; mk_i32 1853806; mk_i32 3345963; mk_i32 1858416; mk_i32 3073009; mk_i32 1277625; mk_i32 5744944; mk_i32 3852015; mk_i32 4183372; mk_i32 5157610; mk_i32 5258977; mk_i32 8106357; mk_i32 2508980; mk_i32 2028118; mk_i32 1937570; mk_i32 4564692; mk_i32 2811291; mk_i32 5396636; mk_i32 7270901; mk_i32 4158088; mk_i32 1528066; mk_i32 482649; mk_i32 1148858; mk_i32 5418153; mk_i32 7814814; mk_i32 169688; mk_i32 2462444; mk_i32 5046034; mk_i32 4213992; mk_i32 4892034; mk_i32 1987814; mk_i32 5183169; mk_i32 1736313; mk_i32 235407; mk_i32 5130263; mk_i32 3258457; mk_i32 5801164; mk_i32 1787943; mk_i32 5989328; mk_i32 6125690; mk_i32 3482206; mk_i32 4197502; mk_i32 7080401; mk_i32 6018354; mk_i32 7062739; mk_i32 2461387; mk_i32 3035980; mk_i32 621164; mk_i32 3901472; mk_i32 7153756; mk_i32 2925816; mk_i32 3374250; mk_i32 1356448; mk_i32 5604662; mk_i32 2683270; mk_i32 5601629; mk_i32 4912752; mk_i32 2312838; mk_i32 7727142; mk_i32 7921254; mk_i32 348812; mk_i32 8052569; mk_i32 1011223; mk_i32 6026202; mk_i32 4561790; mk_i32 6458164; mk_i32 6143691; mk_i32 1744507; mk_i32 1753; mk_i32 6444997; mk_i32 5720892; mk_i32 6924527; mk_i32 2660408; mk_i32 6600190; mk_i32 8321269; mk_i32 2772600; mk_i32 1182243; mk_i32 87208; mk_i32 636927; mk_i32 4415111; mk_i32 4423672; mk_i32 6084020; mk_i32 5095502; mk_i32 4663471; mk_i32 8352605; mk_i32 822541; mk_i32 1009365; mk_i32 5926272; mk_i32 6400920; mk_i32 1596822; mk_i32 4423473; mk_i32 4620952; mk_i32 6695264; mk_i32 4969849; mk_i32 2678278; mk_i32 4611469; mk_i32 4829411; mk_i32 635956; mk_i32 8129971; mk_i32 5925040; mk_i32 4234153; mk_i32 6607829; mk_i32 2192938; mk_i32 6653329; mk_i32 2387513; mk_i32 4768667; mk_i32 8111961; mk_i32 5199961; mk_i32 3747250; mk_i32 2296099; mk_i32 1239911; mk_i32 4541938; mk_i32 3195676; mk_i32 2642980; mk_i32 1254190; mk_i32 8368000; mk_i32 2998219; mk_i32 141835; mk_i32 8291116; mk_i32 2513018; mk_i32 7025525; mk_i32 613238; mk_i32 7070156; mk_i32 6161950; mk_i32 7921677; mk_i32 6458423; mk_i32 4040196; mk_i32 4908348; mk_i32 2039144; mk_i32 6500539; mk_i32 7561656; mk_i32 6201452; mk_i32 6757063; mk_i32 2105286; mk_i32 6006015; mk_i32 6346610; mk_i32 586241; mk_i32 7200804; mk_i32 527981; mk_i32 5637006; mk_i32 6903432; mk_i32 1994046; mk_i32 2491325; mk_i32 6987258; mk_i32 507927; mk_i32 7192532; mk_i32 7655613; mk_i32 6545891; mk_i32 5346675; mk_i32 8041997; mk_i32 2647994; mk_i32 3009748; mk_i32 5767564; mk_i32 4148469; mk_i32 749577; mk_i32 4357667; mk_i32 3980599; mk_i32 2569011; mk_i32 6764887; mk_i32 1723229; mk_i32 1665318; mk_i32 2028038; mk_i32 1163598; mk_i32 5011144; mk_i32 3994671; mk_i32 8368538; mk_i32 7009900; mk_i32 3020393; mk_i32 3363542; mk_i32 214880; mk_i32 545376; mk_i32 7609976; mk_i32 3105558; mk_i32 7277073; mk_i32 508145; mk_i32 7826699; mk_i32 860144; mk_i32 3430436; mk_i32 140244; mk_i32 6866265; mk_i32 6195333; mk_i32 3123762; mk_i32 2358373; mk_i32 6187330; mk_i32 5365997; mk_i32 6663603; mk_i32 2926054; mk_i32 7987710; mk_i32 8077412; mk_i32 3531229; mk_i32 4405932; mk_i32 4606686; mk_i32 1900052; mk_i32 7598542; mk_i32 1054478; mk_i32 7648983]

let lemma_vzetas_unfold () : Lemma (Hacspec_ml_dsa.Ntt.v_ZETAS == Seq.seq_of_list zetas_list_dsa) =
  assert (Hacspec_ml_dsa.Ntt.v_ZETAS == Seq.seq_of_list zetas_list_dsa)
    by (FStar.Tactics.norm [delta_only [`%Hacspec_ml_dsa.Ntt.v_ZETAS;
                                        `%Rust_primitives.Hax.array_of_list]];
        FStar.Tactics.trefl ())

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50 --split_queries always"
let lemma_v_zetas_eq_zeta (i: nat{1 <= i /\ i < 256})
    = lemma_vzetas_unfold ();
    FStar.Seq.Properties.lemma_seq_of_list_index zetas_list_dsa i;
    (match i with
     | 1 -> assert_norm (v (List.Tot.index zetas_list_dsa 1) == Spec.MLDSA.NttConstants.zeta 1)
     | 2 -> assert_norm (v (List.Tot.index zetas_list_dsa 2) == Spec.MLDSA.NttConstants.zeta 2)
     | 3 -> assert_norm (v (List.Tot.index zetas_list_dsa 3) == Spec.MLDSA.NttConstants.zeta 3)
     | 4 -> assert_norm (v (List.Tot.index zetas_list_dsa 4) == Spec.MLDSA.NttConstants.zeta 4)
     | 5 -> assert_norm (v (List.Tot.index zetas_list_dsa 5) == Spec.MLDSA.NttConstants.zeta 5)
     | 6 -> assert_norm (v (List.Tot.index zetas_list_dsa 6) == Spec.MLDSA.NttConstants.zeta 6)
     | 7 -> assert_norm (v (List.Tot.index zetas_list_dsa 7) == Spec.MLDSA.NttConstants.zeta 7)
     | 8 -> assert_norm (v (List.Tot.index zetas_list_dsa 8) == Spec.MLDSA.NttConstants.zeta 8)
     | 9 -> assert_norm (v (List.Tot.index zetas_list_dsa 9) == Spec.MLDSA.NttConstants.zeta 9)
     | 10 -> assert_norm (v (List.Tot.index zetas_list_dsa 10) == Spec.MLDSA.NttConstants.zeta 10)
     | 11 -> assert_norm (v (List.Tot.index zetas_list_dsa 11) == Spec.MLDSA.NttConstants.zeta 11)
     | 12 -> assert_norm (v (List.Tot.index zetas_list_dsa 12) == Spec.MLDSA.NttConstants.zeta 12)
     | 13 -> assert_norm (v (List.Tot.index zetas_list_dsa 13) == Spec.MLDSA.NttConstants.zeta 13)
     | 14 -> assert_norm (v (List.Tot.index zetas_list_dsa 14) == Spec.MLDSA.NttConstants.zeta 14)
     | 15 -> assert_norm (v (List.Tot.index zetas_list_dsa 15) == Spec.MLDSA.NttConstants.zeta 15)
     | 16 -> assert_norm (v (List.Tot.index zetas_list_dsa 16) == Spec.MLDSA.NttConstants.zeta 16)
     | 17 -> assert_norm (v (List.Tot.index zetas_list_dsa 17) == Spec.MLDSA.NttConstants.zeta 17)
     | 18 -> assert_norm (v (List.Tot.index zetas_list_dsa 18) == Spec.MLDSA.NttConstants.zeta 18)
     | 19 -> assert_norm (v (List.Tot.index zetas_list_dsa 19) == Spec.MLDSA.NttConstants.zeta 19)
     | 20 -> assert_norm (v (List.Tot.index zetas_list_dsa 20) == Spec.MLDSA.NttConstants.zeta 20)
     | 21 -> assert_norm (v (List.Tot.index zetas_list_dsa 21) == Spec.MLDSA.NttConstants.zeta 21)
     | 22 -> assert_norm (v (List.Tot.index zetas_list_dsa 22) == Spec.MLDSA.NttConstants.zeta 22)
     | 23 -> assert_norm (v (List.Tot.index zetas_list_dsa 23) == Spec.MLDSA.NttConstants.zeta 23)
     | 24 -> assert_norm (v (List.Tot.index zetas_list_dsa 24) == Spec.MLDSA.NttConstants.zeta 24)
     | 25 -> assert_norm (v (List.Tot.index zetas_list_dsa 25) == Spec.MLDSA.NttConstants.zeta 25)
     | 26 -> assert_norm (v (List.Tot.index zetas_list_dsa 26) == Spec.MLDSA.NttConstants.zeta 26)
     | 27 -> assert_norm (v (List.Tot.index zetas_list_dsa 27) == Spec.MLDSA.NttConstants.zeta 27)
     | 28 -> assert_norm (v (List.Tot.index zetas_list_dsa 28) == Spec.MLDSA.NttConstants.zeta 28)
     | 29 -> assert_norm (v (List.Tot.index zetas_list_dsa 29) == Spec.MLDSA.NttConstants.zeta 29)
     | 30 -> assert_norm (v (List.Tot.index zetas_list_dsa 30) == Spec.MLDSA.NttConstants.zeta 30)
     | 31 -> assert_norm (v (List.Tot.index zetas_list_dsa 31) == Spec.MLDSA.NttConstants.zeta 31)
     | 32 -> assert_norm (v (List.Tot.index zetas_list_dsa 32) == Spec.MLDSA.NttConstants.zeta 32)
     | 33 -> assert_norm (v (List.Tot.index zetas_list_dsa 33) == Spec.MLDSA.NttConstants.zeta 33)
     | 34 -> assert_norm (v (List.Tot.index zetas_list_dsa 34) == Spec.MLDSA.NttConstants.zeta 34)
     | 35 -> assert_norm (v (List.Tot.index zetas_list_dsa 35) == Spec.MLDSA.NttConstants.zeta 35)
     | 36 -> assert_norm (v (List.Tot.index zetas_list_dsa 36) == Spec.MLDSA.NttConstants.zeta 36)
     | 37 -> assert_norm (v (List.Tot.index zetas_list_dsa 37) == Spec.MLDSA.NttConstants.zeta 37)
     | 38 -> assert_norm (v (List.Tot.index zetas_list_dsa 38) == Spec.MLDSA.NttConstants.zeta 38)
     | 39 -> assert_norm (v (List.Tot.index zetas_list_dsa 39) == Spec.MLDSA.NttConstants.zeta 39)
     | 40 -> assert_norm (v (List.Tot.index zetas_list_dsa 40) == Spec.MLDSA.NttConstants.zeta 40)
     | 41 -> assert_norm (v (List.Tot.index zetas_list_dsa 41) == Spec.MLDSA.NttConstants.zeta 41)
     | 42 -> assert_norm (v (List.Tot.index zetas_list_dsa 42) == Spec.MLDSA.NttConstants.zeta 42)
     | 43 -> assert_norm (v (List.Tot.index zetas_list_dsa 43) == Spec.MLDSA.NttConstants.zeta 43)
     | 44 -> assert_norm (v (List.Tot.index zetas_list_dsa 44) == Spec.MLDSA.NttConstants.zeta 44)
     | 45 -> assert_norm (v (List.Tot.index zetas_list_dsa 45) == Spec.MLDSA.NttConstants.zeta 45)
     | 46 -> assert_norm (v (List.Tot.index zetas_list_dsa 46) == Spec.MLDSA.NttConstants.zeta 46)
     | 47 -> assert_norm (v (List.Tot.index zetas_list_dsa 47) == Spec.MLDSA.NttConstants.zeta 47)
     | 48 -> assert_norm (v (List.Tot.index zetas_list_dsa 48) == Spec.MLDSA.NttConstants.zeta 48)
     | 49 -> assert_norm (v (List.Tot.index zetas_list_dsa 49) == Spec.MLDSA.NttConstants.zeta 49)
     | 50 -> assert_norm (v (List.Tot.index zetas_list_dsa 50) == Spec.MLDSA.NttConstants.zeta 50)
     | 51 -> assert_norm (v (List.Tot.index zetas_list_dsa 51) == Spec.MLDSA.NttConstants.zeta 51)
     | 52 -> assert_norm (v (List.Tot.index zetas_list_dsa 52) == Spec.MLDSA.NttConstants.zeta 52)
     | 53 -> assert_norm (v (List.Tot.index zetas_list_dsa 53) == Spec.MLDSA.NttConstants.zeta 53)
     | 54 -> assert_norm (v (List.Tot.index zetas_list_dsa 54) == Spec.MLDSA.NttConstants.zeta 54)
     | 55 -> assert_norm (v (List.Tot.index zetas_list_dsa 55) == Spec.MLDSA.NttConstants.zeta 55)
     | 56 -> assert_norm (v (List.Tot.index zetas_list_dsa 56) == Spec.MLDSA.NttConstants.zeta 56)
     | 57 -> assert_norm (v (List.Tot.index zetas_list_dsa 57) == Spec.MLDSA.NttConstants.zeta 57)
     | 58 -> assert_norm (v (List.Tot.index zetas_list_dsa 58) == Spec.MLDSA.NttConstants.zeta 58)
     | 59 -> assert_norm (v (List.Tot.index zetas_list_dsa 59) == Spec.MLDSA.NttConstants.zeta 59)
     | 60 -> assert_norm (v (List.Tot.index zetas_list_dsa 60) == Spec.MLDSA.NttConstants.zeta 60)
     | 61 -> assert_norm (v (List.Tot.index zetas_list_dsa 61) == Spec.MLDSA.NttConstants.zeta 61)
     | 62 -> assert_norm (v (List.Tot.index zetas_list_dsa 62) == Spec.MLDSA.NttConstants.zeta 62)
     | 63 -> assert_norm (v (List.Tot.index zetas_list_dsa 63) == Spec.MLDSA.NttConstants.zeta 63)
     | 64 -> assert_norm (v (List.Tot.index zetas_list_dsa 64) == Spec.MLDSA.NttConstants.zeta 64)
     | 65 -> assert_norm (v (List.Tot.index zetas_list_dsa 65) == Spec.MLDSA.NttConstants.zeta 65)
     | 66 -> assert_norm (v (List.Tot.index zetas_list_dsa 66) == Spec.MLDSA.NttConstants.zeta 66)
     | 67 -> assert_norm (v (List.Tot.index zetas_list_dsa 67) == Spec.MLDSA.NttConstants.zeta 67)
     | 68 -> assert_norm (v (List.Tot.index zetas_list_dsa 68) == Spec.MLDSA.NttConstants.zeta 68)
     | 69 -> assert_norm (v (List.Tot.index zetas_list_dsa 69) == Spec.MLDSA.NttConstants.zeta 69)
     | 70 -> assert_norm (v (List.Tot.index zetas_list_dsa 70) == Spec.MLDSA.NttConstants.zeta 70)
     | 71 -> assert_norm (v (List.Tot.index zetas_list_dsa 71) == Spec.MLDSA.NttConstants.zeta 71)
     | 72 -> assert_norm (v (List.Tot.index zetas_list_dsa 72) == Spec.MLDSA.NttConstants.zeta 72)
     | 73 -> assert_norm (v (List.Tot.index zetas_list_dsa 73) == Spec.MLDSA.NttConstants.zeta 73)
     | 74 -> assert_norm (v (List.Tot.index zetas_list_dsa 74) == Spec.MLDSA.NttConstants.zeta 74)
     | 75 -> assert_norm (v (List.Tot.index zetas_list_dsa 75) == Spec.MLDSA.NttConstants.zeta 75)
     | 76 -> assert_norm (v (List.Tot.index zetas_list_dsa 76) == Spec.MLDSA.NttConstants.zeta 76)
     | 77 -> assert_norm (v (List.Tot.index zetas_list_dsa 77) == Spec.MLDSA.NttConstants.zeta 77)
     | 78 -> assert_norm (v (List.Tot.index zetas_list_dsa 78) == Spec.MLDSA.NttConstants.zeta 78)
     | 79 -> assert_norm (v (List.Tot.index zetas_list_dsa 79) == Spec.MLDSA.NttConstants.zeta 79)
     | 80 -> assert_norm (v (List.Tot.index zetas_list_dsa 80) == Spec.MLDSA.NttConstants.zeta 80)
     | 81 -> assert_norm (v (List.Tot.index zetas_list_dsa 81) == Spec.MLDSA.NttConstants.zeta 81)
     | 82 -> assert_norm (v (List.Tot.index zetas_list_dsa 82) == Spec.MLDSA.NttConstants.zeta 82)
     | 83 -> assert_norm (v (List.Tot.index zetas_list_dsa 83) == Spec.MLDSA.NttConstants.zeta 83)
     | 84 -> assert_norm (v (List.Tot.index zetas_list_dsa 84) == Spec.MLDSA.NttConstants.zeta 84)
     | 85 -> assert_norm (v (List.Tot.index zetas_list_dsa 85) == Spec.MLDSA.NttConstants.zeta 85)
     | 86 -> assert_norm (v (List.Tot.index zetas_list_dsa 86) == Spec.MLDSA.NttConstants.zeta 86)
     | 87 -> assert_norm (v (List.Tot.index zetas_list_dsa 87) == Spec.MLDSA.NttConstants.zeta 87)
     | 88 -> assert_norm (v (List.Tot.index zetas_list_dsa 88) == Spec.MLDSA.NttConstants.zeta 88)
     | 89 -> assert_norm (v (List.Tot.index zetas_list_dsa 89) == Spec.MLDSA.NttConstants.zeta 89)
     | 90 -> assert_norm (v (List.Tot.index zetas_list_dsa 90) == Spec.MLDSA.NttConstants.zeta 90)
     | 91 -> assert_norm (v (List.Tot.index zetas_list_dsa 91) == Spec.MLDSA.NttConstants.zeta 91)
     | 92 -> assert_norm (v (List.Tot.index zetas_list_dsa 92) == Spec.MLDSA.NttConstants.zeta 92)
     | 93 -> assert_norm (v (List.Tot.index zetas_list_dsa 93) == Spec.MLDSA.NttConstants.zeta 93)
     | 94 -> assert_norm (v (List.Tot.index zetas_list_dsa 94) == Spec.MLDSA.NttConstants.zeta 94)
     | 95 -> assert_norm (v (List.Tot.index zetas_list_dsa 95) == Spec.MLDSA.NttConstants.zeta 95)
     | 96 -> assert_norm (v (List.Tot.index zetas_list_dsa 96) == Spec.MLDSA.NttConstants.zeta 96)
     | 97 -> assert_norm (v (List.Tot.index zetas_list_dsa 97) == Spec.MLDSA.NttConstants.zeta 97)
     | 98 -> assert_norm (v (List.Tot.index zetas_list_dsa 98) == Spec.MLDSA.NttConstants.zeta 98)
     | 99 -> assert_norm (v (List.Tot.index zetas_list_dsa 99) == Spec.MLDSA.NttConstants.zeta 99)
     | 100 -> assert_norm (v (List.Tot.index zetas_list_dsa 100) == Spec.MLDSA.NttConstants.zeta 100)
     | 101 -> assert_norm (v (List.Tot.index zetas_list_dsa 101) == Spec.MLDSA.NttConstants.zeta 101)
     | 102 -> assert_norm (v (List.Tot.index zetas_list_dsa 102) == Spec.MLDSA.NttConstants.zeta 102)
     | 103 -> assert_norm (v (List.Tot.index zetas_list_dsa 103) == Spec.MLDSA.NttConstants.zeta 103)
     | 104 -> assert_norm (v (List.Tot.index zetas_list_dsa 104) == Spec.MLDSA.NttConstants.zeta 104)
     | 105 -> assert_norm (v (List.Tot.index zetas_list_dsa 105) == Spec.MLDSA.NttConstants.zeta 105)
     | 106 -> assert_norm (v (List.Tot.index zetas_list_dsa 106) == Spec.MLDSA.NttConstants.zeta 106)
     | 107 -> assert_norm (v (List.Tot.index zetas_list_dsa 107) == Spec.MLDSA.NttConstants.zeta 107)
     | 108 -> assert_norm (v (List.Tot.index zetas_list_dsa 108) == Spec.MLDSA.NttConstants.zeta 108)
     | 109 -> assert_norm (v (List.Tot.index zetas_list_dsa 109) == Spec.MLDSA.NttConstants.zeta 109)
     | 110 -> assert_norm (v (List.Tot.index zetas_list_dsa 110) == Spec.MLDSA.NttConstants.zeta 110)
     | 111 -> assert_norm (v (List.Tot.index zetas_list_dsa 111) == Spec.MLDSA.NttConstants.zeta 111)
     | 112 -> assert_norm (v (List.Tot.index zetas_list_dsa 112) == Spec.MLDSA.NttConstants.zeta 112)
     | 113 -> assert_norm (v (List.Tot.index zetas_list_dsa 113) == Spec.MLDSA.NttConstants.zeta 113)
     | 114 -> assert_norm (v (List.Tot.index zetas_list_dsa 114) == Spec.MLDSA.NttConstants.zeta 114)
     | 115 -> assert_norm (v (List.Tot.index zetas_list_dsa 115) == Spec.MLDSA.NttConstants.zeta 115)
     | 116 -> assert_norm (v (List.Tot.index zetas_list_dsa 116) == Spec.MLDSA.NttConstants.zeta 116)
     | 117 -> assert_norm (v (List.Tot.index zetas_list_dsa 117) == Spec.MLDSA.NttConstants.zeta 117)
     | 118 -> assert_norm (v (List.Tot.index zetas_list_dsa 118) == Spec.MLDSA.NttConstants.zeta 118)
     | 119 -> assert_norm (v (List.Tot.index zetas_list_dsa 119) == Spec.MLDSA.NttConstants.zeta 119)
     | 120 -> assert_norm (v (List.Tot.index zetas_list_dsa 120) == Spec.MLDSA.NttConstants.zeta 120)
     | 121 -> assert_norm (v (List.Tot.index zetas_list_dsa 121) == Spec.MLDSA.NttConstants.zeta 121)
     | 122 -> assert_norm (v (List.Tot.index zetas_list_dsa 122) == Spec.MLDSA.NttConstants.zeta 122)
     | 123 -> assert_norm (v (List.Tot.index zetas_list_dsa 123) == Spec.MLDSA.NttConstants.zeta 123)
     | 124 -> assert_norm (v (List.Tot.index zetas_list_dsa 124) == Spec.MLDSA.NttConstants.zeta 124)
     | 125 -> assert_norm (v (List.Tot.index zetas_list_dsa 125) == Spec.MLDSA.NttConstants.zeta 125)
     | 126 -> assert_norm (v (List.Tot.index zetas_list_dsa 126) == Spec.MLDSA.NttConstants.zeta 126)
     | 127 -> assert_norm (v (List.Tot.index zetas_list_dsa 127) == Spec.MLDSA.NttConstants.zeta 127)
     | 128 -> assert_norm (v (List.Tot.index zetas_list_dsa 128) == Spec.MLDSA.NttConstants.zeta 128)
     | 129 -> assert_norm (v (List.Tot.index zetas_list_dsa 129) == Spec.MLDSA.NttConstants.zeta 129)
     | 130 -> assert_norm (v (List.Tot.index zetas_list_dsa 130) == Spec.MLDSA.NttConstants.zeta 130)
     | 131 -> assert_norm (v (List.Tot.index zetas_list_dsa 131) == Spec.MLDSA.NttConstants.zeta 131)
     | 132 -> assert_norm (v (List.Tot.index zetas_list_dsa 132) == Spec.MLDSA.NttConstants.zeta 132)
     | 133 -> assert_norm (v (List.Tot.index zetas_list_dsa 133) == Spec.MLDSA.NttConstants.zeta 133)
     | 134 -> assert_norm (v (List.Tot.index zetas_list_dsa 134) == Spec.MLDSA.NttConstants.zeta 134)
     | 135 -> assert_norm (v (List.Tot.index zetas_list_dsa 135) == Spec.MLDSA.NttConstants.zeta 135)
     | 136 -> assert_norm (v (List.Tot.index zetas_list_dsa 136) == Spec.MLDSA.NttConstants.zeta 136)
     | 137 -> assert_norm (v (List.Tot.index zetas_list_dsa 137) == Spec.MLDSA.NttConstants.zeta 137)
     | 138 -> assert_norm (v (List.Tot.index zetas_list_dsa 138) == Spec.MLDSA.NttConstants.zeta 138)
     | 139 -> assert_norm (v (List.Tot.index zetas_list_dsa 139) == Spec.MLDSA.NttConstants.zeta 139)
     | 140 -> assert_norm (v (List.Tot.index zetas_list_dsa 140) == Spec.MLDSA.NttConstants.zeta 140)
     | 141 -> assert_norm (v (List.Tot.index zetas_list_dsa 141) == Spec.MLDSA.NttConstants.zeta 141)
     | 142 -> assert_norm (v (List.Tot.index zetas_list_dsa 142) == Spec.MLDSA.NttConstants.zeta 142)
     | 143 -> assert_norm (v (List.Tot.index zetas_list_dsa 143) == Spec.MLDSA.NttConstants.zeta 143)
     | 144 -> assert_norm (v (List.Tot.index zetas_list_dsa 144) == Spec.MLDSA.NttConstants.zeta 144)
     | 145 -> assert_norm (v (List.Tot.index zetas_list_dsa 145) == Spec.MLDSA.NttConstants.zeta 145)
     | 146 -> assert_norm (v (List.Tot.index zetas_list_dsa 146) == Spec.MLDSA.NttConstants.zeta 146)
     | 147 -> assert_norm (v (List.Tot.index zetas_list_dsa 147) == Spec.MLDSA.NttConstants.zeta 147)
     | 148 -> assert_norm (v (List.Tot.index zetas_list_dsa 148) == Spec.MLDSA.NttConstants.zeta 148)
     | 149 -> assert_norm (v (List.Tot.index zetas_list_dsa 149) == Spec.MLDSA.NttConstants.zeta 149)
     | 150 -> assert_norm (v (List.Tot.index zetas_list_dsa 150) == Spec.MLDSA.NttConstants.zeta 150)
     | 151 -> assert_norm (v (List.Tot.index zetas_list_dsa 151) == Spec.MLDSA.NttConstants.zeta 151)
     | 152 -> assert_norm (v (List.Tot.index zetas_list_dsa 152) == Spec.MLDSA.NttConstants.zeta 152)
     | 153 -> assert_norm (v (List.Tot.index zetas_list_dsa 153) == Spec.MLDSA.NttConstants.zeta 153)
     | 154 -> assert_norm (v (List.Tot.index zetas_list_dsa 154) == Spec.MLDSA.NttConstants.zeta 154)
     | 155 -> assert_norm (v (List.Tot.index zetas_list_dsa 155) == Spec.MLDSA.NttConstants.zeta 155)
     | 156 -> assert_norm (v (List.Tot.index zetas_list_dsa 156) == Spec.MLDSA.NttConstants.zeta 156)
     | 157 -> assert_norm (v (List.Tot.index zetas_list_dsa 157) == Spec.MLDSA.NttConstants.zeta 157)
     | 158 -> assert_norm (v (List.Tot.index zetas_list_dsa 158) == Spec.MLDSA.NttConstants.zeta 158)
     | 159 -> assert_norm (v (List.Tot.index zetas_list_dsa 159) == Spec.MLDSA.NttConstants.zeta 159)
     | 160 -> assert_norm (v (List.Tot.index zetas_list_dsa 160) == Spec.MLDSA.NttConstants.zeta 160)
     | 161 -> assert_norm (v (List.Tot.index zetas_list_dsa 161) == Spec.MLDSA.NttConstants.zeta 161)
     | 162 -> assert_norm (v (List.Tot.index zetas_list_dsa 162) == Spec.MLDSA.NttConstants.zeta 162)
     | 163 -> assert_norm (v (List.Tot.index zetas_list_dsa 163) == Spec.MLDSA.NttConstants.zeta 163)
     | 164 -> assert_norm (v (List.Tot.index zetas_list_dsa 164) == Spec.MLDSA.NttConstants.zeta 164)
     | 165 -> assert_norm (v (List.Tot.index zetas_list_dsa 165) == Spec.MLDSA.NttConstants.zeta 165)
     | 166 -> assert_norm (v (List.Tot.index zetas_list_dsa 166) == Spec.MLDSA.NttConstants.zeta 166)
     | 167 -> assert_norm (v (List.Tot.index zetas_list_dsa 167) == Spec.MLDSA.NttConstants.zeta 167)
     | 168 -> assert_norm (v (List.Tot.index zetas_list_dsa 168) == Spec.MLDSA.NttConstants.zeta 168)
     | 169 -> assert_norm (v (List.Tot.index zetas_list_dsa 169) == Spec.MLDSA.NttConstants.zeta 169)
     | 170 -> assert_norm (v (List.Tot.index zetas_list_dsa 170) == Spec.MLDSA.NttConstants.zeta 170)
     | 171 -> assert_norm (v (List.Tot.index zetas_list_dsa 171) == Spec.MLDSA.NttConstants.zeta 171)
     | 172 -> assert_norm (v (List.Tot.index zetas_list_dsa 172) == Spec.MLDSA.NttConstants.zeta 172)
     | 173 -> assert_norm (v (List.Tot.index zetas_list_dsa 173) == Spec.MLDSA.NttConstants.zeta 173)
     | 174 -> assert_norm (v (List.Tot.index zetas_list_dsa 174) == Spec.MLDSA.NttConstants.zeta 174)
     | 175 -> assert_norm (v (List.Tot.index zetas_list_dsa 175) == Spec.MLDSA.NttConstants.zeta 175)
     | 176 -> assert_norm (v (List.Tot.index zetas_list_dsa 176) == Spec.MLDSA.NttConstants.zeta 176)
     | 177 -> assert_norm (v (List.Tot.index zetas_list_dsa 177) == Spec.MLDSA.NttConstants.zeta 177)
     | 178 -> assert_norm (v (List.Tot.index zetas_list_dsa 178) == Spec.MLDSA.NttConstants.zeta 178)
     | 179 -> assert_norm (v (List.Tot.index zetas_list_dsa 179) == Spec.MLDSA.NttConstants.zeta 179)
     | 180 -> assert_norm (v (List.Tot.index zetas_list_dsa 180) == Spec.MLDSA.NttConstants.zeta 180)
     | 181 -> assert_norm (v (List.Tot.index zetas_list_dsa 181) == Spec.MLDSA.NttConstants.zeta 181)
     | 182 -> assert_norm (v (List.Tot.index zetas_list_dsa 182) == Spec.MLDSA.NttConstants.zeta 182)
     | 183 -> assert_norm (v (List.Tot.index zetas_list_dsa 183) == Spec.MLDSA.NttConstants.zeta 183)
     | 184 -> assert_norm (v (List.Tot.index zetas_list_dsa 184) == Spec.MLDSA.NttConstants.zeta 184)
     | 185 -> assert_norm (v (List.Tot.index zetas_list_dsa 185) == Spec.MLDSA.NttConstants.zeta 185)
     | 186 -> assert_norm (v (List.Tot.index zetas_list_dsa 186) == Spec.MLDSA.NttConstants.zeta 186)
     | 187 -> assert_norm (v (List.Tot.index zetas_list_dsa 187) == Spec.MLDSA.NttConstants.zeta 187)
     | 188 -> assert_norm (v (List.Tot.index zetas_list_dsa 188) == Spec.MLDSA.NttConstants.zeta 188)
     | 189 -> assert_norm (v (List.Tot.index zetas_list_dsa 189) == Spec.MLDSA.NttConstants.zeta 189)
     | 190 -> assert_norm (v (List.Tot.index zetas_list_dsa 190) == Spec.MLDSA.NttConstants.zeta 190)
     | 191 -> assert_norm (v (List.Tot.index zetas_list_dsa 191) == Spec.MLDSA.NttConstants.zeta 191)
     | 192 -> assert_norm (v (List.Tot.index zetas_list_dsa 192) == Spec.MLDSA.NttConstants.zeta 192)
     | 193 -> assert_norm (v (List.Tot.index zetas_list_dsa 193) == Spec.MLDSA.NttConstants.zeta 193)
     | 194 -> assert_norm (v (List.Tot.index zetas_list_dsa 194) == Spec.MLDSA.NttConstants.zeta 194)
     | 195 -> assert_norm (v (List.Tot.index zetas_list_dsa 195) == Spec.MLDSA.NttConstants.zeta 195)
     | 196 -> assert_norm (v (List.Tot.index zetas_list_dsa 196) == Spec.MLDSA.NttConstants.zeta 196)
     | 197 -> assert_norm (v (List.Tot.index zetas_list_dsa 197) == Spec.MLDSA.NttConstants.zeta 197)
     | 198 -> assert_norm (v (List.Tot.index zetas_list_dsa 198) == Spec.MLDSA.NttConstants.zeta 198)
     | 199 -> assert_norm (v (List.Tot.index zetas_list_dsa 199) == Spec.MLDSA.NttConstants.zeta 199)
     | 200 -> assert_norm (v (List.Tot.index zetas_list_dsa 200) == Spec.MLDSA.NttConstants.zeta 200)
     | 201 -> assert_norm (v (List.Tot.index zetas_list_dsa 201) == Spec.MLDSA.NttConstants.zeta 201)
     | 202 -> assert_norm (v (List.Tot.index zetas_list_dsa 202) == Spec.MLDSA.NttConstants.zeta 202)
     | 203 -> assert_norm (v (List.Tot.index zetas_list_dsa 203) == Spec.MLDSA.NttConstants.zeta 203)
     | 204 -> assert_norm (v (List.Tot.index zetas_list_dsa 204) == Spec.MLDSA.NttConstants.zeta 204)
     | 205 -> assert_norm (v (List.Tot.index zetas_list_dsa 205) == Spec.MLDSA.NttConstants.zeta 205)
     | 206 -> assert_norm (v (List.Tot.index zetas_list_dsa 206) == Spec.MLDSA.NttConstants.zeta 206)
     | 207 -> assert_norm (v (List.Tot.index zetas_list_dsa 207) == Spec.MLDSA.NttConstants.zeta 207)
     | 208 -> assert_norm (v (List.Tot.index zetas_list_dsa 208) == Spec.MLDSA.NttConstants.zeta 208)
     | 209 -> assert_norm (v (List.Tot.index zetas_list_dsa 209) == Spec.MLDSA.NttConstants.zeta 209)
     | 210 -> assert_norm (v (List.Tot.index zetas_list_dsa 210) == Spec.MLDSA.NttConstants.zeta 210)
     | 211 -> assert_norm (v (List.Tot.index zetas_list_dsa 211) == Spec.MLDSA.NttConstants.zeta 211)
     | 212 -> assert_norm (v (List.Tot.index zetas_list_dsa 212) == Spec.MLDSA.NttConstants.zeta 212)
     | 213 -> assert_norm (v (List.Tot.index zetas_list_dsa 213) == Spec.MLDSA.NttConstants.zeta 213)
     | 214 -> assert_norm (v (List.Tot.index zetas_list_dsa 214) == Spec.MLDSA.NttConstants.zeta 214)
     | 215 -> assert_norm (v (List.Tot.index zetas_list_dsa 215) == Spec.MLDSA.NttConstants.zeta 215)
     | 216 -> assert_norm (v (List.Tot.index zetas_list_dsa 216) == Spec.MLDSA.NttConstants.zeta 216)
     | 217 -> assert_norm (v (List.Tot.index zetas_list_dsa 217) == Spec.MLDSA.NttConstants.zeta 217)
     | 218 -> assert_norm (v (List.Tot.index zetas_list_dsa 218) == Spec.MLDSA.NttConstants.zeta 218)
     | 219 -> assert_norm (v (List.Tot.index zetas_list_dsa 219) == Spec.MLDSA.NttConstants.zeta 219)
     | 220 -> assert_norm (v (List.Tot.index zetas_list_dsa 220) == Spec.MLDSA.NttConstants.zeta 220)
     | 221 -> assert_norm (v (List.Tot.index zetas_list_dsa 221) == Spec.MLDSA.NttConstants.zeta 221)
     | 222 -> assert_norm (v (List.Tot.index zetas_list_dsa 222) == Spec.MLDSA.NttConstants.zeta 222)
     | 223 -> assert_norm (v (List.Tot.index zetas_list_dsa 223) == Spec.MLDSA.NttConstants.zeta 223)
     | 224 -> assert_norm (v (List.Tot.index zetas_list_dsa 224) == Spec.MLDSA.NttConstants.zeta 224)
     | 225 -> assert_norm (v (List.Tot.index zetas_list_dsa 225) == Spec.MLDSA.NttConstants.zeta 225)
     | 226 -> assert_norm (v (List.Tot.index zetas_list_dsa 226) == Spec.MLDSA.NttConstants.zeta 226)
     | 227 -> assert_norm (v (List.Tot.index zetas_list_dsa 227) == Spec.MLDSA.NttConstants.zeta 227)
     | 228 -> assert_norm (v (List.Tot.index zetas_list_dsa 228) == Spec.MLDSA.NttConstants.zeta 228)
     | 229 -> assert_norm (v (List.Tot.index zetas_list_dsa 229) == Spec.MLDSA.NttConstants.zeta 229)
     | 230 -> assert_norm (v (List.Tot.index zetas_list_dsa 230) == Spec.MLDSA.NttConstants.zeta 230)
     | 231 -> assert_norm (v (List.Tot.index zetas_list_dsa 231) == Spec.MLDSA.NttConstants.zeta 231)
     | 232 -> assert_norm (v (List.Tot.index zetas_list_dsa 232) == Spec.MLDSA.NttConstants.zeta 232)
     | 233 -> assert_norm (v (List.Tot.index zetas_list_dsa 233) == Spec.MLDSA.NttConstants.zeta 233)
     | 234 -> assert_norm (v (List.Tot.index zetas_list_dsa 234) == Spec.MLDSA.NttConstants.zeta 234)
     | 235 -> assert_norm (v (List.Tot.index zetas_list_dsa 235) == Spec.MLDSA.NttConstants.zeta 235)
     | 236 -> assert_norm (v (List.Tot.index zetas_list_dsa 236) == Spec.MLDSA.NttConstants.zeta 236)
     | 237 -> assert_norm (v (List.Tot.index zetas_list_dsa 237) == Spec.MLDSA.NttConstants.zeta 237)
     | 238 -> assert_norm (v (List.Tot.index zetas_list_dsa 238) == Spec.MLDSA.NttConstants.zeta 238)
     | 239 -> assert_norm (v (List.Tot.index zetas_list_dsa 239) == Spec.MLDSA.NttConstants.zeta 239)
     | 240 -> assert_norm (v (List.Tot.index zetas_list_dsa 240) == Spec.MLDSA.NttConstants.zeta 240)
     | 241 -> assert_norm (v (List.Tot.index zetas_list_dsa 241) == Spec.MLDSA.NttConstants.zeta 241)
     | 242 -> assert_norm (v (List.Tot.index zetas_list_dsa 242) == Spec.MLDSA.NttConstants.zeta 242)
     | 243 -> assert_norm (v (List.Tot.index zetas_list_dsa 243) == Spec.MLDSA.NttConstants.zeta 243)
     | 244 -> assert_norm (v (List.Tot.index zetas_list_dsa 244) == Spec.MLDSA.NttConstants.zeta 244)
     | 245 -> assert_norm (v (List.Tot.index zetas_list_dsa 245) == Spec.MLDSA.NttConstants.zeta 245)
     | 246 -> assert_norm (v (List.Tot.index zetas_list_dsa 246) == Spec.MLDSA.NttConstants.zeta 246)
     | 247 -> assert_norm (v (List.Tot.index zetas_list_dsa 247) == Spec.MLDSA.NttConstants.zeta 247)
     | 248 -> assert_norm (v (List.Tot.index zetas_list_dsa 248) == Spec.MLDSA.NttConstants.zeta 248)
     | 249 -> assert_norm (v (List.Tot.index zetas_list_dsa 249) == Spec.MLDSA.NttConstants.zeta 249)
     | 250 -> assert_norm (v (List.Tot.index zetas_list_dsa 250) == Spec.MLDSA.NttConstants.zeta 250)
     | 251 -> assert_norm (v (List.Tot.index zetas_list_dsa 251) == Spec.MLDSA.NttConstants.zeta 251)
     | 252 -> assert_norm (v (List.Tot.index zetas_list_dsa 252) == Spec.MLDSA.NttConstants.zeta 252)
     | 253 -> assert_norm (v (List.Tot.index zetas_list_dsa 253) == Spec.MLDSA.NttConstants.zeta 253)
     | 254 -> assert_norm (v (List.Tot.index zetas_list_dsa 254) == Spec.MLDSA.NttConstants.zeta 254)
     | 255 -> assert_norm (v (List.Tot.index zetas_list_dsa 255) == Spec.MLDSA.NttConstants.zeta 255))
#pop-options

///////////////////////////////////////////////////////////////////////////////
// ============ INVERSE NTT (Gentleman-Sande) bridge lemmas =================
///////////////////////////////////////////////////////////////////////////////

(* Per-butterfly-step FE bridge for the Gentleman-Sande (inverse) butterfly.
   Pure algebraic — no impl reveal.  Mirror of `lemma_butterfly_step_fe`
   (forward / Cooley-Tukey) but for the GS shape the impl's
   `simd_unit_inv_ntt_step` produces:
     lo_new = lo_old + hi_old                     (exact i32 add; even-lane result)
     hi_new = mont_mul (hi_old - lo_old) zeta_mont (odd-lane result)
   with `(v hi_new) % q == ((v hi_old - v lo_old) * v zeta_mont * 8265825) % q`.
   `zeta_mont` is the POSITIVE Mont-form table zeta (= zeta_r (k-round),
   CONFIRMED at L7 r0: 25847 == (v_ZETAS.[1] * pow2 32) mod q), so
   `(v zeta_mont) % q == (zeta_std * pow2 32) % q` with `zeta_std = v v_ZETAS.[k-round]`.

   Sign cancellation: hi_new ≡ (hi-lo)*zeta_std ≡ (lo-hi)*(-zeta_std) (mod q),
   exactly matching the spec odd lane `z*(lo-hi)` with z ≡ -zeta_std. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_inv_butterfly_step_fe
    (lo_old hi_old lo_new hi_new zeta_mont: i32)
    (zeta_std: int)
    : Lemma
        (requires
          v lo_new == v lo_old + v hi_old /\
          (v hi_new) % 8380417 == ((v hi_old - v lo_old) * v zeta_mont * 8265825) % 8380417 /\
          (v zeta_mont) % 8380417 == (zeta_std * pow2 32) % 8380417)
        (ensures
          (v lo_new) % 8380417 == (v lo_old + v hi_old) % 8380417 /\
          (v hi_new) % 8380417 == ((v lo_old - v hi_old) * (- zeta_std)) % 8380417)
  = let q : pos = 8380417 in
    assert_norm ((pow2 32 * 8265825) % q == 1);
    // Step 1: v zeta_mont * 8265825 ≡ zeta_std (mod q)
    L.lemma_mod_mul_distr_l (v zeta_mont) 8265825 q;
    L.lemma_mod_mul_distr_l (zeta_std * pow2 32) 8265825 q;
    L.lemma_mod_mul_distr_r zeta_std (pow2 32 * 8265825) q;
    assert ((v zeta_mont * 8265825) % q == zeta_std % q);
    // Step 2: hi_new ≡ (hi_old - lo_old) * zeta_std (mod q)
    L.lemma_mod_mul_distr_r (v hi_old - v lo_old) (v zeta_mont * 8265825) q;
    L.lemma_mod_mul_distr_r (v hi_old - v lo_old) zeta_std q;
    assert (((v hi_old - v lo_old) * v zeta_mont * 8265825) % q
            == ((v hi_old - v lo_old) * zeta_std) % q);
    assert ((v hi_new) % q == ((v hi_old - v lo_old) * zeta_std) % q);
    // Step 3: (hi_old - lo_old) * zeta_std == (lo_old - hi_old) * (- zeta_std)
    assert ((v hi_old - v lo_old) * zeta_std == (v lo_old - v hi_old) * (- zeta_std));
    // even lane is exact
    assert (v lo_new == v lo_old + v hi_old)
#pop-options

(* GS per-pair butterfly -> spec-lane bridge for the inverse NTT.  Combines
   `lemma_inv_butterfly_step_fe` (Mont -> mod-q congruence on lo_new/hi_new)
   with `lemma_mod_q_v` to relate the impl's two new lanes to the spec's two
   `intt_layer` lanes.  `z` is the standard-form POSITIVE table zeta
   `v_ZETAS.[k-round]`; `zeta_std = v z`.  The two `ensures` are exactly the
   bodies of `intt_layer`'s even lane (lo position: mod_q(p.[i] + p.[i+len]))
   and odd lane (hi position:
     let zspec = ((cast Q) -! (cast z)) %! (cast Q) in
     mod_q(zspec * ((cast p.[i-len]) - (cast p.[i])))),
   with `lo_old = p.[i]` (even idx) and `hi_old = p.[i+len]`, so for the odd
   lane `p.[i-len] = lo_old`, `p.[i] = hi_old`.  Reused by all 8 inverse layers
   (mirror of forward `lemma_layer_0_pair_spec`). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_inv_layer_pair_spec
    (lo_old hi_old lo_new hi_new zeta_mont z: i32)
    : Lemma
        (requires
          v lo_new == v lo_old + v hi_old /\
          (v hi_new) % 8380417 == ((v hi_old - v lo_old) * v zeta_mont * 8265825) % 8380417 /\
          (v zeta_mont) % 8380417 == (v z * pow2 32) % 8380417)
        (ensures
          (let zi : i64 = cast z <: i64 in
           let lo_i : i64 = cast lo_old <: i64 in
           let hi_i : i64 = cast hi_old <: i64 in
           let qi  : i64 = cast Hacspec_ml_dsa.Parameters.v_Q <: i64 in
           let zspec : i64 = (qi -! zi) %! qi in
           let even_spec : i32 =
             Hacspec_ml_dsa.Arithmetic.mod_q (lo_i +! hi_i <: i64) in
           let odd_spec : i32 =
             Hacspec_ml_dsa.Arithmetic.mod_q (zspec *! (lo_i -! hi_i <: i64) <: i64) in
           (v lo_new) % 8380417 == (v even_spec) % 8380417 /\
           (v hi_new) % 8380417 == (v odd_spec) % 8380417))
  = let q : pos = 8380417 in
    lemma_inv_butterfly_step_fe lo_old hi_old lo_new hi_new zeta_mont (v z);
    // butterfly gives: lo_new ≡ lo_old + hi_old, hi_new ≡ (lo_old - hi_old)*(-z) (mod q)
    assert ((v lo_new) % q == (v lo_old + v hi_old) % q);
    assert ((v hi_new) % q == ((v lo_old - v hi_old) * (- (v z))) % q);
    let zi : i64 = cast z <: i64 in
    let lo_i : i64 = cast lo_old <: i64 in
    let hi_i : i64 = cast hi_old <: i64 in
    let qi : i64 = cast Hacspec_ml_dsa.Parameters.v_Q <: i64 in
    assert (v zi == v z /\ v lo_i == v lo_old /\ v hi_i == v hi_old /\ v qi == q);
    // even lane: mod_q (lo_i + hi_i)
    let even_sum : i64 = lo_i +! hi_i in
    assert (v even_sum == v lo_old + v hi_old);
    let even_spec : i32 = Hacspec_ml_dsa.Arithmetic.mod_q even_sum in
    lemma_mod_q_v even_sum;
    assert (v even_spec == (v lo_old + v hi_old) % q);
    // odd lane: zspec = (q - z) %! q ; v zspec == (q - v z) % q  (Euclidean %!)
    let zspec : i64 = (qi -! zi) %! qi in
    assert (v (qi -! zi) == q - v z);
    assert (v zspec == (q - v z) % q);
    // (q - v z) % q ≡ - v z  (mod q)
    L.lemma_mod_sub_distr q (v z) q;
    assert ((q - v z) % q == (- (v z)) % q);
    let diff : i64 = lo_i -! hi_i in
    assert (v diff == v lo_old - v hi_old);
    let odd_prod : i64 = zspec *! diff in
    assert (v odd_prod == ((q - v z) % q) * (v lo_old - v hi_old));
    let odd_spec : i32 = Hacspec_ml_dsa.Arithmetic.mod_q odd_prod in
    lemma_mod_q_v odd_prod;
    assert (v odd_spec == (((q - v z) % q) * (v lo_old - v hi_old)) % q);
    // push the inner %q out: ((q - v z) % q) * d ≡ (q - v z) * d ≡ (- v z) * d (mod q)
    L.lemma_mod_mul_distr_l (q - v z) (v lo_old - v hi_old) q;
    L.lemma_mod_mul_distr_l (- (v z)) (v lo_old - v hi_old) q;
    assert ((((q - v z) % q) * (v lo_old - v hi_old)) % q
            == ((q - v z) * (v lo_old - v hi_old)) % q);
    // (q - v z) * d ≡ (- v z) * d  (mod q) since q*d ≡ 0
    L.lemma_mod_plus ((- (v z)) * (v lo_old - v hi_old)) (v lo_old - v hi_old) q;
    assert ((q - v z) * (v lo_old - v hi_old)
            == (- (v z)) * (v lo_old - v hi_old) + (v lo_old - v hi_old) * q);
    // tie: (lo - hi)*(-z) == (-z)*(lo - hi)
    assert ((v lo_old - v hi_old) * (- (v z)) == (- (v z)) * (v lo_old - v hi_old))
#pop-options

(* ===== INVERSE NTT layer 0 (within-chunk, len=1, k=255) =====
   intt_layer reducer: `intt_layer p 0` at flat index `i`.  len=1, k=255:
     round = i/2,  idx = i%2,  z = (Q - v_ZETAS.[255 - round]) %! Q.
     even (idx<1): mod_q(p.[i] + p.[i+1]).
     odd  (idx>=1): mod_q(z * (p.[i-1] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_0_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

(* GS per-pair clean-context spec-lane bridge for inverse layer 0.
   zeta index = 255 - (4*b + p).  Mirror of forward `lemma_layer_0_chunk_pair`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_0_chunk_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32}) (p: nat{p < 4})
    (zmp: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] in
           v (Seq.index co (2*p)) == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
           (v (Seq.index co (2*p+1))) % 8380417 ==
             ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co = Seq.index transformed b in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
           (v (Seq.index co (2*p))) % 8380417 == (v (Seq.index spec (8*b + 2*p))) % 8380417 /\
           (v (Seq.index co (2*p+1))) % 8380417 == (v (Seq.index spec (8*b + 2*p + 1))) % 8380417))
  = let q : pos = 8380417 in
    let ci = Seq.index input b in
    let co = Seq.index transformed b in
    let in_flat = simd_units_to_array input in
    let lo_old = Seq.index ci (2*p) in
    let hi_old = Seq.index ci (2*p+1) in
    let lo_new = Seq.index co (2*p) in
    let hi_new = Seq.index co (2*p+1) in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] in
    let i_even : usize = mk_usize (8*b + 2*p) in
    let i_odd  : usize = mk_usize (8*b + 2*p + 1) in
    lemma_simd_units_to_array_reveal input b ((2*p));
    lemma_simd_units_to_array_reveal input b ((2*p+1));
    assert (Seq.index in_flat (8*b + 2*p) == lo_old);
    assert (Seq.index in_flat (8*b + 2*p + 1) == hi_old);
    assert (v i_even == 8*b + 2*p);
    assert (v i_odd == 8*b + 2*p + 1);
    assert ((8*b + 2*p) / 2 == 4*b + p);
    assert ((8*b + 2*p + 1) / 2 == 4*b + p);
    assert ((8*b + 2*p) % 2 == 0);
    assert ((8*b + 2*p + 1) % 2 == 1);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 255 -! round_odd];
    // equate its index value to z's index (255 - (4*b + p)).  Required for the
    // subtraction-form inverse zeta (forward used addition, no bridge needed).
    assert (v (mk_usize 255 -! i_odd /! mk_usize 2) == (255 - (4*b + p)));
    lemma_intt_layer_0_lane in_flat i_even;
    lemma_intt_layer_0_lane in_flat i_odd;
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

(* Per-chunk 8-lane bridge for inverse layer 0.  Thin dispatcher over the 4
   pair lemmas + a forall over the 8 lanes.  Mirror of forward
   `lemma_ntt_layer_0_chunk_to_hacspec`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_0_chunk_to_hacspec
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32})
    (zm0 zm1 zm2 zm3: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z (p:nat{p<4}) : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] in
           (* pair 0: lanes 0,1 *)
           v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 1) /\
           (v (Seq.index co 1)) % 8380417 ==
             ((v (Seq.index ci 1) - v (Seq.index ci 0)) * v zm0 * 8265825) % 8380417 /\
           (v zm0) % 8380417 == (v (z 0) * pow2 32) % 8380417 /\
           (* pair 1: lanes 2,3 *)
           v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 3) /\
           (v (Seq.index co 3)) % 8380417 ==
             ((v (Seq.index ci 3) - v (Seq.index ci 2)) * v zm1 * 8265825) % 8380417 /\
           (v zm1) % 8380417 == (v (z 1) * pow2 32) % 8380417 /\
           (* pair 2: lanes 4,5 *)
           v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 5) /\
           (v (Seq.index co 5)) % 8380417 ==
             ((v (Seq.index ci 5) - v (Seq.index ci 4)) * v zm2 * 8265825) % 8380417 /\
           (v zm2) % 8380417 == (v (z 2) * pow2 32) % 8380417 /\
           (* pair 3: lanes 6,7 *)
           v (Seq.index co 6) == v (Seq.index ci 6) + v (Seq.index ci 7) /\
           (v (Seq.index co 7)) % 8380417 ==
             ((v (Seq.index ci 7) - v (Seq.index ci 6)) * v zm3 * 8265825) % 8380417 /\
           (v zm3) % 8380417 == (v (z 3) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
           (forall (l: nat). l < 8 ==>
             (v (Seq.index out_flat (8*b + l))) % 8380417 ==
             (v (Seq.index spec (8*b + l))) % 8380417)))
  = let q : pos = 8380417 in
    let co = Seq.index transformed b in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer (simd_units_to_array input) (mk_usize 0) in
    // Per-lane dispatch with the pair lemma called INSIDE aux (minimal context per
    // sub-goal): calling all 4 chunk_pair lemmas up front pollutes the trivial
    // index asserts and saturates (GS bridge context is heavier than forward).
    let zmf (p: nat{p < 4}) : i32 = if p = 0 then zm0 else if p = 1 then zm1 else if p = 2 then zm2 else zm3 in
    let aux (l: nat{l < 8}) : Lemma
        ((v (Seq.index out_flat (8*b + l))) % q == (v (Seq.index spec (8*b + l))) % q)
      = let p : nat = l / 2 in
        assert (p < 4);
        lemma_inv_layer_0_chunk_pair input transformed b p (zmf p);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index co l);
        if l % 2 = 0 then begin
          assert (l == 2*p);
          assert (Seq.index co l == Seq.index co (2*p))
        end else begin
          assert (l == 2*p + 1);
          assert (Seq.index co l == Seq.index co (2*p + 1))
        end
    in
    Classical.forall_intro aux
#pop-options

(* All-32-chunk composition for inverse layer 0.  Mirror of forward
   `lemma_ntt_layer_0_step_to_hacspec_poly`.  Inverse butterfly takes NO `t`
   witness (odd output IS co.[hi]); only a `zm` (mont-zeta) witness function.
   NOTE: --split_queries always prunes the requires foralls per sub-query and
   yields "incomplete quantifiers"; the monolithic VC keeps them, so this poly
   composition runs WITHOUT split_queries (unlike the forward, whose `t b p`
   witness gave a split-friendly trigger). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_intt_layer_0_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (b: nat{b < 32} -> p: nat{p < 4} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
    let co (b: nat{b < 32}) = Seq.index transformed b in
    // Self-contained per-lane composition (bypasses chunk_to_hacspec to avoid the
    // poly-forall -> chunk-requires instantiation problem; the GS poly has no
    // `t b p` witness so the chunk-requires conjunction won't e-match cleanly).
    // aux at flat i picks pair p = (i%8)/2 and calls the pair lemma at that (b,p),
    // discharging the pair requires from the requires forall instantiated at (b,p).
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let b : nat = i / 8 in
        let l : nat = i % 8 in
        let p : nat = l / 2 in
        assert (b < 32 /\ l < 8 /\ p < 4 /\ 8*b + l == i);
        let _ = zm b p in  // trigger the requires forall at (b, p)
        lemma_inv_layer_0_chunk_pair input transformed b p (zm b p);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index (co b) l);
        if l % 2 = 0 then assert (l == 2*p)
        else assert (l == 2*p + 1)
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 1 (within-chunk, len=2, k=127) =====
   intt_layer reducer: `intt_layer p 1` at flat index `i`.  len=2, k=127:
     round = i/4,  idx = i%4,  z = (Q - v_ZETAS.[127 - round]) %! Q.
     even (idx<2): mod_q(p.[i] + p.[i+2]).
     odd  (idx>=2): mod_q(z * (p.[i-2] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_1_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

(* GS per-pair clean-context spec-lane bridge for inverse layer 1.
   zeta index = 127 - (2*b + h).  Mirror of forward `lemma_layer_1_chunk_pair`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_1_chunk_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2})
    (zmp: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] in
           v (Seq.index co (4*h+j)) == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
           (v (Seq.index co (4*h+j+2))) % 8380417 ==
             ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co = Seq.index transformed b in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
           (v (Seq.index co (4*h+j))) % 8380417 == (v (Seq.index spec (8*b + 4*h+j))) % 8380417 /\
           (v (Seq.index co (4*h+j+2))) % 8380417 == (v (Seq.index spec (8*b + 4*h+j+2))) % 8380417))
  = let q : pos = 8380417 in
    let ci = Seq.index input b in
    let co = Seq.index transformed b in
    let in_flat = simd_units_to_array input in
    let lo_old = Seq.index ci (4*h+j) in
    let hi_old = Seq.index ci (4*h+j+2) in
    let lo_new = Seq.index co (4*h+j) in
    let hi_new = Seq.index co (4*h+j+2) in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] in
    let i_even : usize = mk_usize (8*b + 4*h + j) in
    let i_odd  : usize = mk_usize (8*b + 4*h + j + 2) in
    lemma_simd_units_to_array_reveal input b ((4*h+j));
    lemma_simd_units_to_array_reveal input b ((4*h+j+2));
    assert (Seq.index in_flat (8*b + 4*h+j) == lo_old);
    assert (Seq.index in_flat (8*b + 4*h+j+2) == hi_old);
    assert (v i_even == 8*b + 4*h+j);
    assert (v i_odd == 8*b + 4*h+j+2);
    assert ((8*b + 4*h + j) / 4 == 2*b + h);
    assert ((8*b + 4*h + j + 2) / 4 == 2*b + h);
    assert ((8*b + 4*h + j) % 4 == j);
    assert ((8*b + 4*h + j + 2) % 4 == j + 2);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 127 -! round_odd];
    // equate its index value to z's index (127 - (2*b + h)).  Required for the
    // subtraction-form inverse zeta (forward used addition, no bridge needed).
    assert (v (mk_usize 127 -! i_odd /! mk_usize 4) == (127 - (2*b + h)));
    lemma_intt_layer_1_lane in_flat i_even;
    lemma_intt_layer_1_lane in_flat i_odd;
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

(* Per-chunk 8-lane bridge for inverse layer 1.  Thin dispatcher over the 4
   pair lemmas + a forall over the 8 lanes.  Mirror of forward
   `lemma_ntt_layer_1_chunk_to_hacspec`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_1_chunk_to_hacspec
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32})
    (zm0 zm1: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z (h:nat{h<2}) : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] in
           (* half 0, j=0: lanes 0,2 *)
           v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 2) /\
           (v (Seq.index co 2)) % 8380417 ==
             ((v (Seq.index ci 2) - v (Seq.index ci 0)) * v zm0 * 8265825) % 8380417 /\
           (* half 0, j=1: lanes 1,3 *)
           v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 3) /\
           (v (Seq.index co 3)) % 8380417 ==
             ((v (Seq.index ci 3) - v (Seq.index ci 1)) * v zm0 * 8265825) % 8380417 /\
           (* half 1, j=0: lanes 4,6 *)
           v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 6) /\
           (v (Seq.index co 6)) % 8380417 ==
             ((v (Seq.index ci 6) - v (Seq.index ci 4)) * v zm1 * 8265825) % 8380417 /\
           (* half 1, j=1: lanes 5,7 *)
           v (Seq.index co 5) == v (Seq.index ci 5) + v (Seq.index ci 7) /\
           (v (Seq.index co 7)) % 8380417 ==
             ((v (Seq.index ci 7) - v (Seq.index ci 5)) * v zm1 * 8265825) % 8380417 /\
           (* zeta congruences: one per half *)
           (v zm0) % 8380417 == (v (z 0) * pow2 32) % 8380417 /\
           (v zm1) % 8380417 == (v (z 1) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
           (forall (l: nat). l < 8 ==>
             (v (Seq.index out_flat (8*b + l))) % 8380417 ==
             (v (Seq.index spec (8*b + l))) % 8380417)))
  = let q : pos = 8380417 in
    let co = Seq.index transformed b in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer (simd_units_to_array input) (mk_usize 1) in
    let zmf (h: nat{h < 2}) : i32 = if h = 0 then zm0 else zm1 in
    // pair lemma called INSIDE aux (minimal context per sub-goal) to avoid the
    // cascade pollution that saturates the trivial index asserts (see L0).
    let aux (l: nat{l < 8}) : Lemma
        ((v (Seq.index out_flat (8*b + l))) % q == (v (Seq.index spec (8*b + l))) % q)
      = let h : nat = l / 4 in
        let j : nat = l % 2 in
        assert (h < 2 /\ j < 2);
        lemma_inv_layer_1_chunk_pair input transformed b h j (zmf h);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index co l);
        if l % 4 < 2 then begin
          assert (l == 4*h + j);
          assert (Seq.index co l == Seq.index co (4*h + j))
        end else begin
          assert (l == 4*h + j + 2);
          assert (Seq.index co l == Seq.index co (4*h + j + 2))
        end
    in
    Classical.forall_intro aux
#pop-options

(* All-32-chunk composition for inverse layer 1.  Mirror of forward
   `lemma_ntt_layer_1_step_to_hacspec_poly`.  Inverse butterfly takes NO `t`
   witness (odd output IS co.[hi]); only a `zm` (mont-zeta) witness function. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_1_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (b: nat{b < 32} -> h: nat{h < 2} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
    let co (b: nat{b < 32}) = Seq.index transformed b in
    // Self-contained per-lane composition (mirror of the verified L0 poly): the GS
    // poly has no `t b h j` witness, so call the pair lemma directly per flat lane.
    // pair (h,j): h = l/4, j = l%2; lo lane iff l%4 < 2.
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let b : nat = i / 8 in
        let l : nat = i % 8 in
        let h : nat = l / 4 in
        let j : nat = l % 2 in
        assert (b < 32 /\ l < 8 /\ h < 2 /\ j < 2 /\ 8*b + l == i);
        let _ = zm b h in  // trigger the requires foralls at (b, h[, j])
        lemma_inv_layer_1_chunk_pair input transformed b h j (zm b h);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index (co b) l);
        if l % 4 < 2 then assert (l == 4*h + j)
        else assert (l == 4*h + j + 2)
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 2 (within-chunk, len=4, k=63) =====
   intt_layer reducer: `intt_layer p 2` at flat index `i`.  len=4, k=63:
     round = i/8,  idx = i%8,  z = (Q - v_ZETAS.[63 - round]) %! Q.
     even (idx<4): mod_q(p.[i] + p.[i+4]).
     odd  (idx>=4): mod_q(z * (p.[i-4] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_2_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

(* GS per-pair clean-context spec-lane bridge for inverse layer 2.
   zeta index = 63 - b.  Mirror of forward `lemma_layer_2_chunk_pair`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_2_chunk_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32}) (p: nat{p < 4})
    (zmp: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] in
           v (Seq.index co (p)) == v (Seq.index ci (p)) + v (Seq.index ci (p+4)) /\
           (v (Seq.index co (p+4))) % 8380417 ==
             ((v (Seq.index ci (p+4)) - v (Seq.index ci (p))) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co = Seq.index transformed b in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
           (v (Seq.index co (p))) % 8380417 == (v (Seq.index spec (8*b + p))) % 8380417 /\
           (v (Seq.index co (p+4))) % 8380417 == (v (Seq.index spec (8*b + p+4))) % 8380417))
  = let q : pos = 8380417 in
    let ci = Seq.index input b in
    let co = Seq.index transformed b in
    let in_flat = simd_units_to_array input in
    let lo_old = Seq.index ci (p) in
    let hi_old = Seq.index ci (p+4) in
    let lo_new = Seq.index co (p) in
    let hi_new = Seq.index co (p+4) in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] in
    let i_even : usize = mk_usize (8*b + p) in
    let i_odd  : usize = mk_usize (8*b + p + 4) in
    lemma_simd_units_to_array_reveal input b (p);
    lemma_simd_units_to_array_reveal input b ((p+4));
    assert (Seq.index in_flat (8*b + p) == lo_old);
    assert (Seq.index in_flat (8*b + p+4) == hi_old);
    assert (v i_even == 8*b + p);
    assert (v i_odd == 8*b + p+4);
    assert ((8*b + p) / 8 == b);
    assert ((8*b + p + 4) / 8 == b);
    assert ((8*b + p) % 8 == p);
    assert ((8*b + p + 4) % 8 == p + 4);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 63 -! round_odd];
    // equate its index value to z's index (63 - b).  Required for the
    // subtraction-form inverse zeta (forward used addition, no bridge needed).
    assert (v (mk_usize 63 -! i_odd /! mk_usize 8) == (63 - b));
    lemma_intt_layer_2_lane in_flat i_even;
    lemma_intt_layer_2_lane in_flat i_odd;
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

(* Per-chunk 8-lane bridge for inverse layer 2.  Thin dispatcher over the 4
   pair lemmas + a forall over the 8 lanes.  Mirror of forward
   `lemma_ntt_layer_2_chunk_to_hacspec`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_2_chunk_to_hacspec
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (b: nat{b < 32})
    (zm: i32)
    : Lemma
        (requires
          (let ci = Seq.index input b in
           let co = Seq.index transformed b in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] in
           (* pair 0: lanes 0,4 *)
           v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 4) /\
           (v (Seq.index co 4)) % 8380417 ==
             ((v (Seq.index ci 4) - v (Seq.index ci 0)) * v zm * 8265825) % 8380417 /\
           (* pair 1: lanes 1,5 *)
           v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 5) /\
           (v (Seq.index co 5)) % 8380417 ==
             ((v (Seq.index ci 5) - v (Seq.index ci 1)) * v zm * 8265825) % 8380417 /\
           (* pair 2: lanes 2,6 *)
           v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 6) /\
           (v (Seq.index co 6)) % 8380417 ==
             ((v (Seq.index ci 6) - v (Seq.index ci 2)) * v zm * 8265825) % 8380417 /\
           (* pair 3: lanes 3,7 *)
           v (Seq.index co 3) == v (Seq.index ci 3) + v (Seq.index ci 7) /\
           (v (Seq.index co 7)) % 8380417 ==
             ((v (Seq.index ci 7) - v (Seq.index ci 3)) * v zm * 8265825) % 8380417 /\
           (* zeta congruence: one per chunk *)
           (v zm) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
           (forall (l: nat). l < 8 ==>
             (v (Seq.index out_flat (8*b + l))) % 8380417 ==
             (v (Seq.index spec (8*b + l))) % 8380417)))
  = let q : pos = 8380417 in
    let co = Seq.index transformed b in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer (simd_units_to_array input) (mk_usize 2) in
    // pair lemma called INSIDE aux (minimal context per sub-goal) to avoid the
    // cascade pollution that saturates the trivial index asserts (see L0).
    let aux (l: nat{l < 8}) : Lemma
        ((v (Seq.index out_flat (8*b + l))) % q == (v (Seq.index spec (8*b + l))) % q)
      = let p : nat = l % 4 in
        assert (p < 4);
        lemma_inv_layer_2_chunk_pair input transformed b p zm;
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index co l);
        if l < 4 then begin
          assert (l == p);
          assert (Seq.index co l == Seq.index co p)
        end else begin
          assert (l == p + 4);
          assert (Seq.index co l == Seq.index co (p + 4))
        end
    in
    Classical.forall_intro aux
#pop-options

(* All-32-chunk composition for inverse layer 2.  Mirror of forward
   `lemma_ntt_layer_2_step_to_hacspec_poly`.  Inverse butterfly takes NO `t`
   witness (odd output IS co.[hi]); only a `zm` (mont-zeta) witness function. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_2_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (b: nat{b < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
    let co (b: nat{b < 32}) = Seq.index transformed b in
    // Self-contained per-lane composition (mirror of the verified L0 poly).
    // pair p = l%4; lanes (p, p+4); lo lane iff l < 4; single zeta `zm b`.
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let b : nat = i / 8 in
        let l : nat = i % 8 in
        let p : nat = l % 4 in
        assert (b < 32 /\ l < 8 /\ p < 4 /\ 8*b + l == i);
        let _ = zm b in  // trigger the requires foralls at b
        lemma_inv_layer_2_chunk_pair input transformed b p (zm b);
        lemma_simd_units_to_array_reveal transformed b l;
        assert (Seq.index out_flat (8*b + l) == Seq.index (co b) l);
        if l < 4 then assert (l == p)
        else assert (l == p + 4)
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 3 (cross-chunk, len=8, k=31) =====
   intt_layer reducer: `intt_layer p 3` at flat index `i`.  len=8, k=31:
     round = i/16,  idx = i%16,  z = (Q - v_ZETAS.[31 - round]) %! Q.
     even (idx<8): mod_q(p.[i] + p.[i+8]).
     odd  (idx>=8): mod_q(z * (p.[i-8] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_3_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_3_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 2 == 0}) (l: nat{l < 8}) (zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+1) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+1) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (31 - ulo/2) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
           (v (Seq.index co_hi l)) % 8380417 ==
             ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+1) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))       % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 8 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+1)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+1)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (31 - ulo/2) ] in
    lemma_cross_idx 1 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+1) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+1) + l == 8*ulo + 8 + l);
    assert (Seq.index in_flat (8*ulo + 8 + l) == hi_old);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 31 -! round_odd] with
    // round_odd = (8*ulo+8+l)/16 == ulo/2 (from lemma_cross_idx).
    assert ((8*ulo + 8 + l) / 16 == ulo / 2);
    assert (v (mk_usize 31 -! mk_usize (8*ulo + 8 + l) /! mk_usize 16) == (31 - ulo/2));
    lemma_intt_layer_3_lane in_flat (mk_usize (8*ulo + l));
    lemma_intt_layer_3_lane in_flat (mk_usize (8*ulo + 8 + l));
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_3_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 2 = 0 then begin
          lemma_inv_layer_3_cross_pair input transformed u l (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 1 in
          L.lemma_div_mod u 2;
          assert (u % 2 == 1);
          assert ((ulo % 2 == 0) /\ ulo + 1 == u /\ 8*ulo + 8 + l == 8*u + l);
          lemma_inv_layer_3_cross_pair input transformed ulo l (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 4 (cross-chunk, len=16, k=15) =====
   intt_layer reducer: `intt_layer p 4` at flat index `i`.  len=16, k=15:
     round = i/32,  idx = i%32,  z = (Q - v_ZETAS.[15 - round]) %! Q.
     even (idx<16): mod_q(p.[i] + p.[i+16]).
     odd  (idx>=16): mod_q(z * (p.[i-16] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_4_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_4_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 4 < 2}) (l: nat{l < 8}) (zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+2) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+2) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (15 - ulo/4) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
           (v (Seq.index co_hi l)) % 8380417 ==
             ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+2) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))       % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 16 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+2)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+2)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (15 - ulo/4) ] in
    lemma_cross_idx 2 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+2) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+2) + l == 8*ulo + 16 + l);
    assert (Seq.index in_flat (8*ulo + 16 + l) == hi_old);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 15 -! round_odd] with
    // round_odd = (8*ulo+16+l)/32 == ulo/4 (from lemma_cross_idx).
    assert ((8*ulo + 16 + l) / 32 == ulo / 4);
    assert (v (mk_usize 15 -! mk_usize (8*ulo + 16 + l) /! mk_usize 32) == (15 - ulo/4));
    lemma_intt_layer_4_lane in_flat (mk_usize (8*ulo + l));
    lemma_intt_layer_4_lane in_flat (mk_usize (8*ulo + 16 + l));
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_4_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 4 < 2 then begin
          lemma_inv_layer_4_cross_pair input transformed u l (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 2 in
          L.lemma_div_mod u 4;
          assert (u % 4 >= 2 /\ u >= 2);
          assert ((ulo % 4 < 2) /\ ulo + 2 == u /\ 8*ulo + 16 + l == 8*u + l);
          lemma_inv_layer_4_cross_pair input transformed ulo l (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 5 (cross-chunk, len=32, k=7) =====
   intt_layer reducer: `intt_layer p 5` at flat index `i`.  len=32, k=7:
     round = i/64,  idx = i%64,  z = (Q - v_ZETAS.[7 - round]) %! Q.
     even (idx<32): mod_q(p.[i] + p.[i+32]).
     odd  (idx>=32): mod_q(z * (p.[i-32] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_5_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_5_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 8 < 4}) (l: nat{l < 8}) (zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+4) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+4) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (7 - ulo/8) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
           (v (Seq.index co_hi l)) % 8380417 ==
             ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+4) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))       % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 32 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+4)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+4)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (7 - ulo/8) ] in
    lemma_cross_idx 4 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+4) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+4) + l == 8*ulo + 32 + l);
    assert (Seq.index in_flat (8*ulo + 32 + l) == hi_old);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 7 -! round_odd] with
    // round_odd = (8*ulo+32+l)/64 == ulo/8 (from lemma_cross_idx).
    assert ((8*ulo + 32 + l) / 64 == ulo / 8);
    assert (v (mk_usize 7 -! mk_usize (8*ulo + 32 + l) /! mk_usize 64) == (7 - ulo/8));
    lemma_intt_layer_5_lane in_flat (mk_usize (8*ulo + l));
    lemma_intt_layer_5_lane in_flat (mk_usize (8*ulo + 32 + l));
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_5_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 8 < 4 then begin
          lemma_inv_layer_5_cross_pair input transformed u l (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 4 in
          L.lemma_div_mod u 8;
          assert (u % 8 >= 4 /\ u >= 4);
          assert ((ulo % 8 < 4) /\ ulo + 4 == u /\ 8*ulo + 32 + l == 8*u + l);
          lemma_inv_layer_5_cross_pair input transformed ulo l (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 6 (cross-chunk, len=64, k=3) =====
   intt_layer reducer: `intt_layer p 6` at flat index `i`.  len=64, k=3:
     round = i/128,  idx = i%128,  z = (Q - v_ZETAS.[3 - round]) %! Q.
     even (idx<64): mod_q(p.[i] + p.[i+64]).
     odd  (idx>=64): mod_q(z * (p.[i-64] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_6_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_6_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo % 16 < 8}) (l: nat{l < 8}) (zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+8) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+8) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (3 - ulo/16) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
           (v (Seq.index co_hi l)) % 8380417 ==
             ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+8) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))       % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 64 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+8)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+8)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (3 - ulo/16) ] in
    lemma_cross_idx 8 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+8) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+8) + l == 8*ulo + 64 + l);
    assert (Seq.index in_flat (8*ulo + 64 + l) == hi_old);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 3 -! round_odd] with
    // round_odd = (8*ulo+64+l)/128 == ulo/16 (from lemma_cross_idx).
    assert ((8*ulo + 64 + l) / 128 == ulo / 16);
    assert (v (mk_usize 3 -! mk_usize (8*ulo + 64 + l) /! mk_usize 128) == (3 - ulo/16));
    lemma_intt_layer_6_lane in_flat (mk_usize (8*ulo + l));
    lemma_intt_layer_6_lane in_flat (mk_usize (8*ulo + 64 + l));
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_6_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u % 16 < 8 then begin
          lemma_inv_layer_6_cross_pair input transformed u l (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 8 in
          L.lemma_div_mod u 16;
          assert (u % 16 >= 8 /\ u >= 8);
          assert ((ulo % 16 < 8) /\ ulo + 8 == u /\ 8*ulo + 64 + l == 8*u + l);
          lemma_inv_layer_6_cross_pair input transformed ulo l (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options

(* ===== INVERSE NTT layer 7 (cross-chunk, len=128, k=1) =====
   intt_layer reducer: `intt_layer p 7` at flat index `i`.  len=128, k=1:
     round = i/256,  idx = i%256,  z = (Q - v_ZETAS.[1 - round]) %! Q.
     even (idx<128): mod_q(p.[i] + p.[i+128]).
     odd  (idx>=128): mod_q(z * (p.[i-128] - p.[i])). *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_intt_layer_7_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inv_layer_7_cross_pair
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (ulo: nat{ulo < 32 /\ ulo < 16}) (l: nat{l < 8}) (zmp: i32)
    : Lemma
        (requires
          (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+16) in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+16) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (1 - ulo/32) ] in
           v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
           (v (Seq.index co_hi l)) % 8380417 ==
             ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zmp * 8265825) % 8380417 /\
           (v zmp) % 8380417 == (v z * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+16) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
           (v (Seq.index co_lo l)) % 8380417 == (v (Seq.index spec (8*ulo + l)))       % 8380417 /\
           (v (Seq.index co_hi l)) % 8380417 == (v (Seq.index spec (8*ulo + 128 + l))) % 8380417))
  = let in_flat = simd_units_to_array input in
    let lo_old = Seq.index (Seq.index input ulo) l in
    let hi_old = Seq.index (Seq.index input (ulo+16)) l in
    let lo_new = Seq.index (Seq.index transformed ulo) l in
    let hi_new = Seq.index (Seq.index transformed (ulo+16)) l in
    let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (1 - ulo/32) ] in
    lemma_cross_idx 16 ulo l;
    lemma_simd_units_to_array_reveal input ulo l;
    lemma_simd_units_to_array_reveal input (ulo+16) l;
    assert (Seq.index in_flat (8*ulo + l) == lo_old);
    assert (8*(ulo+16) + l == 8*ulo + 128 + l);
    assert (Seq.index in_flat (8*ulo + 128 + l) == hi_old);
    // GS zeta-index bridge: spec lane reads v_ZETAS.[mk_usize 1 -! round_odd] with
    // round_odd = (8*ulo+128+l)/256 == ulo/32 (from lemma_cross_idx).
    assert ((8*ulo + 128 + l) / 256 == ulo / 32);
    assert (v (mk_usize 1 -! mk_usize (8*ulo + 128 + l) /! mk_usize 256) == (1 - ulo/32));
    lemma_intt_layer_7_lane in_flat (mk_usize (8*ulo + l));
    lemma_intt_layer_7_lane in_flat (mk_usize (8*ulo + 128 + l));
    lemma_inv_layer_pair_spec lo_old hi_old lo_new hi_new zmp z
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_7_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    = let q : pos = 8380417 in
    let in_flat = simd_units_to_array input in
    let out_flat = simd_units_to_array transformed in
    let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
    let aux (i: nat{i < 256}) : Lemma
        ((v (Seq.index out_flat i)) % q == (v (Seq.index spec i)) % q)
      = let u : nat = i / 8 in let l : nat = i % 8 in
        assert (u < 32 /\ l < 8 /\ 8*u + l == i);
        if u < 16 then begin
          lemma_inv_layer_7_cross_pair input transformed u l (zm u);
          lemma_simd_units_to_array_reveal transformed u l
        end else begin
          let ulo : nat = u - 16 in
          L.lemma_div_mod u 32;
          assert (u >= 16);
          assert ((ulo < 16) /\ ulo + 16 == u /\ 8*ulo + 128 + l == 8*u + l);
          lemma_inv_layer_7_cross_pair input transformed ulo l (zm ulo);
          lemma_simd_units_to_array_reveal transformed u l
        end
    in
    Classical.forall_intro aux
#pop-options


(* === Step 12 Track B: AVX2 decompose impl-side bridge === *)

(* --- Bit-trick correctness, integer level ---

   The AVX2 decompose computes the high part `r1` with a reciprocal
   multiply-shift.  For both gamma2 values the chain is:
     c      = (r' + 127) / 128            (= ceil(r'/128))
     result = (c * coef + addc) / 2^sh
   where (coef, sh, addc) = (11275, 24, 2^23) for gamma2 = 95232 and
   (1025, 22, 2^21) for gamma2 = 261888.  The CLAIM proven here is the
   exact floor identity
        result == (r' + g - 1) / (2*g)
   for r' ∈ [0, q-1], g = gamma2.

   Key algebra (validated against an exhaustive Python sim over the
   whole [0,q) range):  let alpha = 2*g, d = 128*2^sh - alpha*coef,
   rem = c*coef + addc - result*2^sh ∈ [0, 2^sh).  The constants satisfy
        alpha*coef = 128*2^sh - d      (d = 2048 / 512)
        alpha*addc = g*2^sh            (since addc = 2^(sh-1))
   from which, with E := 128*c + g - result*alpha,
        E * 2^sh == d*c + alpha*rem.
   Then:
     * E ≡ 0 (mod 128)   (128*c, g, alpha all ≡ 0 mod 128),
     * E*2^sh = d*c + alpha*rem > 0   ⇒ E ≥ 1   ⇒ (mod 128) E ≥ 128,
     * the floor lower bound result*2^sh ≥ c*coef+addc-2^sh+1 gives
       result*alpha > 128c+g-128-alpha ⇒ E < 128+alpha ⇒ (mod128) E ≤ alpha.
   So E ∈ [128, alpha], and with r' ∈ [128c-127, 128c] the residue
   F := r'+g-1-result*alpha = (r'-128c) + (E-1) ∈ [0, alpha-1], which is
   exactly the `division_definition` certificate for result = (r'+g-1)/alpha. *)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 400"
let lemma_decompose_bittrick_div
      (rp g coef sh addc : int)
    : Lemma
        (requires
          ((g == 95232 /\ coef == 11275 /\ sh == 24 /\ addc == 8388608) \/
           (g == 261888 /\ coef == 1025 /\ sh == 22 /\ addc == 2097152)) /\
          0 <= rp /\ rp <= 8380416)
        (ensures
          (let c = (rp + 127) / 128 in
           let result = (c * coef + addc) / pow2 sh in
           result == (rp + g - 1) / (2 * g)))
  = let alpha = 2 * g in
    let twosh = pow2 sh in
    assert_norm (pow2 24 == 16777216);
    assert_norm (pow2 22 == 4194304);
    // d = 128*2^sh - alpha*coef.
    let d = 128 * twosh - alpha * coef in
    assert (d == 2048 \/ d == 512);
    assert (alpha * coef == 128 * twosh - d);
    assert (alpha * addc == g * twosh);   // addc = 2^(sh-1)
    // c bounds: 128c ≤ rp+127 < 128c+128, i.e. rp-127 ≤ 128c ≤ rp ... centered as rp ∈ [128c-127,128c].
    let c = (rp + 127) / 128 in
    L.lemma_div_mod (rp + 127) 128;
    L.lemma_mod_lt (rp + 127) 128;
    assert (128 * c <= rp + 127 /\ rp + 127 < 128 * c + 128);
    assert (128 * c - 127 <= rp /\ rp <= 128 * c);
    // result floor: result*2^sh ≤ c*coef+addc < (result+1)*2^sh.
    let result = (c * coef + addc) / twosh in
    L.lemma_div_mod (c * coef + addc) twosh;
    L.lemma_mod_lt (c * coef + addc) twosh;
    let rem = (c * coef + addc) - result * twosh in
    assert (0 <= rem /\ rem < twosh);
    assert (result * twosh + rem == c * coef + addc);
    // E := 128c + g - result*alpha.  E*2^sh == d*c + alpha*rem.
    let e = 128 * c + g - result * alpha in
    // result*alpha*2^sh = alpha*(result*2^sh) = alpha*(c*coef+addc-rem)
    //   = (128*2^sh-d)*c + g*2^sh - alpha*rem
    // so e*2^sh = (128c+g)*2^sh - result*alpha*2^sh = d*c + alpha*rem.
    assert (result * alpha * twosh == alpha * (result * twosh));
    assert (alpha * (result * twosh) == alpha * (c * coef + addc - rem));
    assert (alpha * (c * coef + addc - rem)
            == (128 * twosh - d) * c + g * twosh - alpha * rem);
    assert (e * twosh == d * c + alpha * rem);
    // E ≥ 0 (E*2^sh ≥ 0, 2^sh>0) and E ≥ 1 (E*2^sh > 0).
    assert (d * c >= 0 /\ alpha * rem >= 0);
    assert (e * twosh >= 0);
    // strict: e*2^sh > 0.  If c = 0 then rem = addc mod 2^sh = addc (>0) so alpha*rem>0;
    //         if c > 0 then d*c > 0.
    assert (c == 0 ==> rem == addc /\ addc > 0);
    assert (c > 0 ==> d * c > 0);
    assert (e * twosh > 0);
    assert (e >= 1);
    // E ≡ 0 (mod 128): 128c ≡ 0, g ≡ 0, alpha ≡ 0 (mod 128).
    assert (g % 128 == 0 /\ alpha % 128 == 0);
    L.lemma_mod_mul_distr_l result alpha 128;   // (result*alpha) % 128
    assert ((result * alpha) % 128 == 0);
    assert ((128 * c) % 128 == 0);
    L.lemma_mod_plus_distr_l (128 * c + g) (- (result * alpha)) 128;
    assert (e % 128 == 0);
    // E ≥ 1 ∧ E ≡ 0 (mod 128) ⇒ E ≥ 128.
    assert (e >= 128);
    // floor LOWER: result*2^sh ≥ c*coef+addc-2^sh+1.
    assert (result * twosh >= c * coef + addc - twosh + 1);
    // ⇒ result*alpha*2^sh ≥ alpha*(c*coef+addc-2^sh+1) ⇒ e*2^sh < (128+alpha)*2^sh ⇒ e < 128+alpha.
    assert (alpha * (result * twosh) >= alpha * (c * coef + addc - twosh + 1));
    assert (e * twosh <= d * c + alpha * (twosh - 1));
    // e < 128 + alpha:  e*2^sh = d*c+alpha*rem, rem ≤ 2^sh-1, and the strict floor bound.
    // Use: e*2^sh ≤ d*c + alpha*(2^sh-1).  Combined with e ≡0 mod128 below pins e ≤ alpha.
    // Establish e ≤ 127 + alpha via the linear LO directly:
    //   result*alpha > 128c+g-128-alpha  ⟺  e < 128+alpha.
    assert (result * alpha * twosh >= (128 * twosh - d) * c + g * twosh - alpha * (twosh - 1));
    assert ((result * alpha) * twosh > (128 * c + g - 128 - alpha) * twosh);
    assert (result * alpha > 128 * c + g - 128 - alpha);
    assert (e < 128 + alpha);
    // E ≡ 0 (mod 128) ∧ E < 128+alpha ∧ alpha ≡ 0 (mod 128) ⇒ E ≤ alpha.
    assert (e <= alpha);
    // F := rp+g-1 - result*alpha = (rp-128c) + (e-1) ∈ [0, alpha-1].
    let f = rp + g - 1 - result * alpha in
    assert (f == (rp - 128 * c) + (e - 1));
    assert (- 127 <= rp - 128 * c /\ rp - 128 * c <= 0);
    assert (0 <= f /\ f < alpha);
    // division_definition: result == (rp+g-1) / alpha.
    assert (result * alpha <= rp + g - 1 /\ rp + g - 1 < result * alpha + alpha);
    L.division_definition (rp + g - 1) alpha result;
    assert (result == (rp + g - 1) / alpha)
#pop-options

(* Uniqueness of the centered residue `mod_p`: if y ≡ x (mod p) and
   y ∈ (-p/2, p/2], then mod_p x p == y.  (p even, p > 0.) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_mod_p_unique (x y p: int)
    : Lemma
        (requires p > 0 /\ p % 2 == 0 /\
                  (x - y) % p == 0 /\ - (p / 2) < y /\ y <= p / 2)
        (ensures Spec.Utils.mod_p x p == y)
  = let m = x % p in
    L.lemma_mod_lt x p;
    assert (0 <= m /\ m < p);
    // y ≡ x (mod p), so m == y % p.
    L.lemma_mod_sub x p ((x - y) / p);
    L.lemma_div_mod (x - y) p;
    assert (x - y == p * ((x - y) / p));
    assert (x == y + p * ((x - y) / p));
    L.lemma_mod_plus y ((x - y) / p) p;
    assert (m == y % p);
    if y >= 0 then begin
      L.modulo_lemma y p;
      assert (m == y)
    end else begin
      L.lemma_mod_plus y 1 p;
      L.modulo_lemma (y + p) p;
      assert (m == y + p)
    end
#pop-options

(* Clamp for gamma2 = 95232:  given `v result ∈ [0, 44]`, the masked
   chain `result &. ((result ^. ((43 - result) >> 31)))` yields
   `result` when result ≤ 43 and 0 when result = 44. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_clamp_95232 (result: i32)
    : Lemma
        (requires v result >= 0 /\ v result <= 44)
        (ensures
          (let mask = Spec.Intrinsics.shift_right_opaque
                        (Spec.Intrinsics.sub_mod_opaque (mk_i32 43) result) (mk_i32 31) in
           let not_result = result ^. mask in
           let r1 = result &. not_result in
           v r1 == (if v result <= 43 then v result else 0)))
  = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    let diff = sub_mod (mk_i32 43) result in
    assert (v diff == 43 - v result);
    let mask = shift_right diff (mk_i32 31) in
    assert (v mask == (43 - v result) / pow2 31);
    assert_norm (pow2 31 == 2147483648);
    if v result <= 43 then begin
      // diff ∈ [-1, 43], here ≥ 0 ⇒ mask = 0.
      L.small_div (43 - v result) (pow2 31);
      assert (v mask == 0);
      assert (mask == mk_i32 0);
      logxor_lemma result mask;
      assert ((result ^. mask) == result);            // a ^. zero == a
      logand_lemma result result;
      assert ((result &. result) == result)            // a == a ⇒ logand == a
    end else begin
      // result = 44 ⇒ diff = -1 ⇒ mask = -1 = ones.
      assert (v diff == -1);
      assert ((-1) / pow2 31 == -1);
      assert (v mask == -1);
      assert (mask == mk_i32 (-1));
      logxor_lemma result mask;                       // a ^. ones == lognot a
      assert ((result ^. mask) == lognot result);
      logand_lemma result (lognot result);            // b == lognot a ⇒ logand == zero
      assert ((result &. (lognot result)) == mk_i32 0)
    end
#pop-options

(* Clamp for gamma2 = 261888:  given `v result ∈ [0, 16]`, `result &. 15`
   yields `result` when result ≤ 15 and 0 when result = 16. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_clamp_261888 (result: i32)
    : Lemma
        (requires v result >= 0 /\ v result <= 16)
        (ensures v (result &. (mk_i32 15)) == (if v result <= 15 then v result else 0))
  = assert_norm (pow2 4 == 16);
    assert (mk_i32 15 == sub (mk_i32 (pow2 4)) (mk_i32 1));
    logand_mask_lemma result 4;
    assert (v (result &. (mk_i32 15)) == v result % pow2 4);
    if v result <= 15 then L.small_mod (v result) (pow2 4)
    else assert (v result % pow2 4 == 0)
#pop-options

(* Pure-integer bridge for decompose: given the bit-trick high part
   `result_int == (rp + g - 1) / (2g)` for normalized `rp ∈ [0, q-1]`,
   establishes ALL the integer-level facts the v-image proof needs —
   `result_int*(2g) == rp - r_g`, the clamp value `r1_int`, and the
   r0 value/mask conditions.  Proven in a CLEAN context (no i32/module
   ambient), so the heavy nonlinear division/mod_p reasoning does not
   combine with the host module's `t_Array` refinement cascade. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_decompose_int_bridge (rp g result_int: int)
    : Lemma
        (requires
          (g == 95232 \/ g == 261888) /\
          0 <= rp /\ rp <= 8380416 /\
          result_int == (rp + g - 1) / (2 * g))
        (ensures
          (let alpha = 2 * g in
           let r_g = Spec.Utils.mod_p rp alpha in
           let m = 8380416 / alpha in
           let special = (rp - r_g = 8380416) in
           let r1_int = if special then 0 else (rp - r_g) / alpha in
           // r1 facts
           result_int * alpha == rp - r_g /\
           0 <= result_int /\ result_int <= m /\
           (special <==> result_int == m) /\
           (if result_int < m then result_int else 0) == r1_int /\
           0 <= r1_int /\ r1_int <= 44 /\
           // r0 facts
           - g < r_g /\ r_g <= g /\
           (special ==> rp - r1_int * alpha == 8380416 + r_g) /\
           ((not special) ==> rp - r1_int * alpha == r_g)))
  = let q : pos = 8380417 in
    let alpha = 2 * g in
    let r_g = Spec.Utils.mod_p rp alpha in
    L.lemma_mod_lt rp alpha;
    assert (- g < r_g /\ r_g <= g);
    // result_int*alpha == rp - r_g via centered-division uniqueness.
    L.lemma_div_mod (rp + g - 1) alpha;
    L.lemma_mod_lt (rp + g - 1) alpha;
    let fres = (rp + g - 1) - result_int * alpha in
    assert (0 <= fres /\ fres < alpha);
    assert (rp - result_int * alpha == fres - g + 1);
    assert (- g < (rp - result_int * alpha) /\ (rp - result_int * alpha) <= g);
    L.lemma_mod_mul_distr_l result_int alpha alpha;
    assert ((rp - (rp - result_int * alpha) - result_int * alpha) == 0);
    lemma_mod_p_unique rp (rp - result_int * alpha) alpha;
    assert (r_g == rp - result_int * alpha);
    assert (result_int * alpha == rp - r_g);
    let m = 8380416 / alpha in
    assert (m == 44 \/ m == 16);
    lemma_spec_decompose_r1_bound g rp;
    // result_int ∈ [0, m]; special ⟺ result_int == m.
    assert (0 <= result_int /\ result_int <= m);
    let special = (rp - r_g = 8380416) in
    // special ⟺ rp-r_g == q-1 == m*alpha ⟺ result_int == m.
    assert (m * alpha == 8380416);
    assert (special <==> result_int == m)
#pop-options

(* Bridge: AVX2 SIMD-shape `Spec.MLDSA.Math.decompose_spec` agrees in
   v-image with the canonical `Spec.MLDSA.Math.decompose` for any
   `v r` in i32-bounded trait range and valid gamma2.  The
   decompose_spec body normalizes negatives via `if r < 0 then r + q`,
   so r' = if v r >= 0 then v r else v r + q ∈ [0, q-1].  This is the
   same value as `(v r) % q` (Euclidean), which is the input that
   `Spec.MLDSA.Math.decompose` consumes.  So the two agree
   unconditionally for v r ∈ [-(q-1), q-1].

   The bit-trick correctness `result == (r'+g-1)/(2g)` is
   `lemma_decompose_bittrick_div`; the integer-level reconciliation is
   `lemma_decompose_int_bridge` (proven in a clean context); this lemma
   just does the i32↔int bridging + clamp/mask bit-ops. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --using_facts_from 'Prims FStar Rust_primitives Core_models Spec.Utils Spec.Intrinsics Spec.MLDSA Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_bittrick_div Hacspec_ml_dsa.Commute.Chunk.lemma_mod_p_unique Hacspec_ml_dsa.Commute.Chunk.lemma_clamp_95232 Hacspec_ml_dsa.Commute.Chunk.lemma_clamp_261888 Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_int_bridge Hacspec_ml_dsa.Commute.Chunk.lemma_spec_decompose_r1_bound'"
let lemma_decompose_spec_eq_decompose (gamma2 r: i32)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    let q : pos = 8380417 in
    let g = v gamma2 in
    let alpha_int = 2 * g in
    assert_norm (pow2 31 == 2147483648);
    // --- r' = normalized r ∈ [0, q-1], equals (v r) % q ---
    let rp_i : i32 = if r <. mk_i32 0 then add_mod r Spec.MLDSA.Math.v_FIELD_MODULUS else r in
    let rp = v rp_i in
    if v r < 0 then begin
      assert (v rp_i == v r + q);
      L.lemma_mod_plus (v r) 1 q;
      L.modulo_lemma (v r + q) q
    end else L.modulo_lemma (v r) q;
    assert (rp == (v r) % q);
    assert (0 <= rp /\ rp <= q - 1);
    let r_q = (v r) % q in
    assert (r_q == rp);
    // decompose's r_g, r1_int, r0_int.
    let r_g = Spec.Utils.mod_p r_q alpha_int in
    let special = (r_q - r_g = q - 1) in
    let r1_int = if special then 0 else (r_q - r_g) / alpha_int in
    let r0_int = if special then r_g - 1 else r_g in
    let m = (q - 1) / alpha_int in
    // --- c = ceil(rp/128) ---
    let c_i : i32 = Spec.Intrinsics.shift_right_opaque
                      (Spec.Intrinsics.add_mod_opaque rp_i (mk_i32 127)) (mk_i32 7) in
    assert (v c_i == (rp + 127) / pow2 7);
    assert (pow2 7 == 128);
    let c = (rp + 127) / 128 in
    assert (v c_i == c);
    L.lemma_div_mod (rp + 127) 128;
    L.lemma_mod_lt (rp + 127) 128;
    assert (128 * c - 127 <= rp /\ rp <= 128 * c);
    // --- the bit-trick result value (per gamma2) ---
    let coef = if g = 95232 then 11275 else 1025 in
    let sh = if g = 95232 then 24 else 22 in
    let addc = if g = 95232 then 8388608 else 2097152 in
    let result_int = (c * coef + addc) / pow2 sh in
    lemma_decompose_bittrick_div rp g coef sh addc;
    assert (result_int == (rp + g - 1) / alpha_int);
    // ALL integer-level facts (nonlinear/mod_p) via the clean-context bridge.
    lemma_decompose_int_bridge rp g result_int;
    assert (result_int * alpha_int == r_q - r_g);
    assert (0 <= result_int /\ result_int <= m);
    assert (special <==> result_int == m);
    assert ((if result_int < m then result_int else 0) == r1_int);
    assert (0 <= r1_int /\ r1_int <= 44);
    assert (- g < r_g /\ r_g <= g);
    // --- compute r1 (the spec) per gamma2 (light: clamp bit-ops) ---
    let r1_i : i32 =
      if v gamma2 = 95232 then begin
        let result = Spec.Intrinsics.mul_mod_opaque c_i (mk_i32 11275) in
        assert (v result == c * 11275);
        let result = Spec.Intrinsics.add_mod_opaque result (mk_i32 1 <<! mk_i32 23 <: i32) in
        assert (v (mk_i32 1 <<! mk_i32 23 <: i32) == pow2 23);
        assert (v result == c * 11275 + 8388608);
        let result = Spec.Intrinsics.shift_right_opaque result (mk_i32 24) in
        assert (v result == (c * 11275 + 8388608) / pow2 24);
        assert (v result == result_int);
        assert (v result >= 0 /\ v result <= 44);
        lemma_clamp_95232 result;
        let mask = Spec.Intrinsics.shift_right_opaque
                     (Spec.Intrinsics.sub_mod_opaque (mk_i32 43) result) (mk_i32 31) in
        let not_result = result ^. mask in
        result &. not_result
      end else begin
        let result = Spec.Intrinsics.mul_mod_opaque c_i (mk_i32 1025) in
        assert (v result == c * 1025);
        let result = Spec.Intrinsics.add_mod_opaque result (mk_i32 1 <<! mk_i32 21 <: i32) in
        assert (v (mk_i32 1 <<! mk_i32 21 <: i32) == pow2 21);
        assert (v result == c * 1025 + 2097152);
        let result = Spec.Intrinsics.shift_right_opaque result (mk_i32 22) in
        assert (v result == (c * 1025 + 2097152) / pow2 22);
        assert (v result == result_int);
        assert (v result >= 0 /\ v result <= 16);
        lemma_clamp_261888 result;
        result &. (mk_i32 15)
      end
    in
    assert (v r1_i == (if result_int < m then result_int else 0));
    assert (v r1_i == r1_int);
    // --- r0 finalize (light: i32 arithmetic + mask, values from the bridge) ---
    let alpha_i : i32 = gamma2 *! (mk_i32 2) in
    assert (v alpha_i == alpha_int);
    assert (r1_int * alpha_int <= 8380416);
    let prod = Spec.Intrinsics.mul_mod_opaque r1_i alpha_i in
    assert (v prod == r1_int * alpha_int);
    let r0_tmp = Spec.Intrinsics.sub_mod_opaque rp_i prod in
    assert (v r0_tmp == rp - r1_int * alpha_int);
    // value of r0_tmp from the bridge:
    //   special ⇒ rp - r1_int*alpha == q-1+r_g; else == r_g.
    assert (special ==> v r0_tmp == q - 1 + r_g);
    assert ((not special) ==> v r0_tmp == r_g);
    let half : i32 = mk_i32 ((v Spec.MLDSA.Math.v_FIELD_MODULUS - 1) / 2) in
    assert (v half == (q - 1) / 2);
    let mask0 = Spec.Intrinsics.sub_mod_opaque half r0_tmp in
    assert (v mask0 == (q - 1) / 2 - v r0_tmp);
    let mask = Spec.Intrinsics.shift_right_opaque mask0 (mk_i32 31) in
    assert (v mask == ((q - 1) / 2 - v r0_tmp) / pow2 31);
    let fmm = mask &. Spec.MLDSA.Math.v_FIELD_MODULUS in
    let r0_i = Spec.Intrinsics.sub_mod_opaque r0_tmp fmm in
    if special then begin
      // r0_tmp = q-1+r_g, (q-1)/2 - r0_tmp = -(q-1)/2 - r_g < 0 ⇒ mask = -1, fmm = q.
      assert (v mask0 == (q - 1) / 2 - (q - 1 + r_g));
      assert (v mask0 < 0 /\ v mask0 >= - pow2 31);
      assert (v mask0 / pow2 31 == -1);
      assert (v mask == -1);
      assert (mask == ones);
      logand_lemma Spec.MLDSA.Math.v_FIELD_MODULUS mask;     // logand ones a == a
      assert (logand ones Spec.MLDSA.Math.v_FIELD_MODULUS == Spec.MLDSA.Math.v_FIELD_MODULUS);
      assert (fmm == Spec.MLDSA.Math.v_FIELD_MODULUS);
      assert (v fmm == q);
      assert (v r0_i == (q - 1 + r_g) - q);
      assert (v r0_i == r_g - 1);
      assert (r0_int == r_g - 1)
    end else begin
      // r0_tmp = r_g ∈ (-g, g], (q-1)/2 - r_g > 0 ⇒ mask = 0, fmm = 0.
      assert (v r0_tmp == r_g);
      assert (- g < r_g /\ r_g <= g);
      assert (v mask0 == (q - 1) / 2 - r_g);
      assert (v mask0 > 0 /\ v mask0 < pow2 31);
      L.small_div (v mask0) (pow2 31);
      assert (v mask == 0);
      assert (mask == zero);
      logand_lemma Spec.MLDSA.Math.v_FIELD_MODULUS mask;     // logand zero a == zero
      assert (logand zero Spec.MLDSA.Math.v_FIELD_MODULUS == zero);
      assert (fmm == zero);
      assert (v fmm == 0);
      assert (v r0_i == r_g);
      assert (r0_int == r_g)
    end;
    assert (v r0_i == r0_int);
    // tie the let-bound terms back to decompose_spec's outputs.
    assert (Spec.MLDSA.Math.decompose_spec gamma2 r == (r0_i, r1_i))
#pop-options
