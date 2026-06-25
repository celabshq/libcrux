module Spec.MLDSA.Math
(*
  Tier-1 shared integer spec for SIMD arithmetic operations.

  This module holds the *integer-form* specs that both the Portable
  and AVX2 `Operations` impls cite as their underlying free-fn post.
  Everything here is stated in plain integer arithmetic plus the
  opaque ops from `Spec.Intrinsics` — no `mod_q`, no field-modulus
  opacity in the spec body.

  The companion Tier-2 layer
  (`specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`)
  proves the hacspec-lift property for each spec here — i.e., what
  the integer-level definition means after lifting modulo q.

  Eventual relocation into `specs/ml-dsa/proofs/fstar/commute/` (or
  a sibling) is fine for cleanliness but is *not* a deletion.  This
  is the shared-spec layer of the proof.
*)
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

include Spec.Utils
open Spec.Intrinsics
  



(* Single-arg Montgomery reduction: mirrors the body of
   `simd/portable/arithmetic.rs::montgomery_reduce_element`.  Opaque
   to SMT — callers reveal it where needed, or invoke one of the
   bound/correctness lemmas defined below to extract specific facts
   without dragging the full hi/low/k/c arithmetic into the proof
   context. *)

(* Internal parametric bound lemma for `mont_red`.

   For input `value: i64` with `|value| <= n`, the result satisfies
   `|mont_red value| <= q/2 + ceil(n / 2^32)`, where q = 8380417.
   Equivalently: `is_i32b (4190208 + (n + pow2 32 - 1) / pow2 32) result`
   (or, more loosely but easier to discharge,
    `is_i32b (4190209 + n / pow2 32) result`).

   This is the SINGLE source of truth for Montgomery-reduce bounds.
   All specialized lookup lemmas (q^2, FIELD_MAX*pow2 31,
   FIELD_MAX*41978, 256*FIELD_MAX*41978, ...) derive from this
   parametric form via concrete arithmetic discharge.

   Mirrors the doc-comment formula in ML-KEM's
   `montgomery_reduce_element` (see
   libcrux-ml-kem/src/vector/portable/arithmetic.rs:343-348). *)
#push-options "--z3rlimit 600 --fuel 0 --ifuel 1"
let lemma_mont_red_bound_internal (n: nat) (value: i64)
    : Lemma
        (requires Spec.Utils.is_i64b n value /\
                  n <= 8380416 * pow2 32)  // safe upper bound; covers all our uses
        (ensures
          Spec.Utils.is_i32b (4190209 + n / pow2 32) (mont_red value))
  = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i64_inttype;
    Spec.Intrinsics.reveal_opaque_cast_ops #i32_inttype #i64_inttype;
    reveal_opaque (`%mont_red) (mont_red value);
    // Step 1: hi = cast_mod (value >> 32) <: i32 = value / 2^32 (as int).
    let val_int : int = v value in
    let val_shifted : i64 = value >>! mk_i32 32 in
    assert (v val_shifted == val_int / pow2 32);
    // The bound on val_int / pow2 32: |val_int / pow2 32| <= n / pow2 32 + 1
    // (the +1 is for the negative-floor case; we'll absorb it via the formula).
    Spec.Utils.lemma_range_at_percent (val_int / pow2 32) (pow2 32);
    let hi : i32 = cast val_shifted <: i32 in
    assert (v hi == (val_int / pow2 32) @% pow2 32);
    // Step 2: low = cast_mod value <: i32 = value @% 2^32.
    let low : i32 = cast value <: i32 in
    assert (v low == val_int @% pow2 32);
    // Step 3: k = cast_mod (low * Q' as i64) <: i32 = (low * Q') @% 2^32.
    let q'_i32 = mk_i32 58728449 in
    Spec.Utils.lemma_range_at_percent (v low) (pow2 64);
    Spec.Utils.lemma_range_at_percent 58728449 (pow2 64);
    let lq_product : i64 = i32_mul low q'_i32 in
    assert_norm (pow2 31 * 58728449 < pow2 63);
    Spec.Utils.lemma_range_at_percent (v low * 58728449) (pow2 64);
    assert (v lq_product == v low * 58728449);
    let k : i32 = cast lq_product <: i32 in
    assert (v k == (v low * 58728449) @% pow2 32);
    // Step 4: c = cast_mod ((k * Q as i64) >> 32) = (k*q)/2^32.
    let q_i32 = mk_i32 8380417 in
    Spec.Utils.lemma_range_at_percent (v k) (pow2 64);
    Spec.Utils.lemma_range_at_percent 8380417 (pow2 64);
    let kq_product : i64 = i32_mul k q_i32 in
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
    // |c| <= q/2 = 4190208 (since |k| < 2^31 so |k*q| < 2^31 * q,
    //                       /2^32 gives |c| <= q/2).
    assert (v c >= -4190209 /\ v c <= 4190209);
    // |hi| <= ceil(n / 2^32)  (from |value| <= n and signed shift).
    assert (v hi >= -(n / pow2 32) - 1 /\ v hi <= n / pow2 32 + 1);
    // Result = hi - c.  Bound: |hi - c| <= |hi| + |c| <= n/2^32 + 1 + 4190209
    //                                     = 4190209 + n/2^32 + 1
    //                                     <= 4190209 + n/2^32  (loose).
    assert_norm (4190209 + (8380416 * pow2 32) / pow2 32 < pow2 31);
    let result : i32 = hi -! c in
    assert (v result == v hi - v c);
    ()
#pop-options

(* === Specialized lookup lemmas: bound for `mont_red` at common
       input magnitudes that occur in the keygen / sign / verify
       chain.  Each is a one-line derivation from
       `lemma_mont_red_bound_internal`; callers invoke these by name
       and don't need to reason about the closed-form
       `(n + pow2 32 - 1) / pow2 32` arithmetic. *)

(* Input bound: FIELD_MAX · 2³² (the always-clause range — full i64
   product of two FIELD_MAX-bounded values).  Output bound 12570625
   matches the original `montgomery_reduce_element` always-clause
   (8380416 + 4190209). *)
let lemma_mont_red_bound_field_max_times_pow2_32 (value: i64)
    = lemma_mont_red_bound_internal (8380416 * pow2 32) value;
    assert_norm (4190209 + (8380416 * pow2 32) / pow2 32 == 12570625)

(* Input bound: q² = FIELD_MAX² = 8380416² ≈ 7·10¹³.  Common when a
   product `i32_mul x y` is formed from two FIELD_MAX-bounded i32s. *)
let lemma_mont_red_bound_q_squared (value: i64)
    : Lemma
        (requires Spec.Utils.is_i64b (8380416 * 8380416) value)
        (ensures Spec.Utils.is_i32b 4206561 (mont_red value))
  = assert_norm (8380416 * 8380416 <= 8380416 * pow2 32);
    lemma_mont_red_bound_internal (8380416 * 8380416) value;
    assert_norm (4190209 + (8380416 * 8380416) / pow2 32 == 4206561)

(* Input bound: FIELD_MAX · 2³¹.  Matches the existing
   `montgomery_reduce_element` tight-branch shape (input bounded by
   one i32-arg times an arbitrary i32, so |product| ≤ FIELD_MAX · 2³¹). *)
let lemma_mont_red_bound_field_max_times_pow2_31 (value: i64)
    = assert_norm (8380416 * pow2 31 <= 8380416 * pow2 32);
    lemma_mont_red_bound_internal (8380416 * pow2 31) value;
    assert_norm (4190209 + (8380416 * pow2 31) / pow2 32 == 8380417)

(* Input bound: FIELD_MAX · 41978.  Common from per-element
   `montgomery_multiply_by_constant(_, 41_978)` calls (where one
   factor is the inverse-2N constant and the other is FIELD_MAX-bounded
   from a Barrett-reduced NTT input). *)
let lemma_mont_red_bound_field_max_times_41978 (value: i64)
    : Lemma
        (requires Spec.Utils.is_i64b (8380416 * 41978) value)
        (ensures Spec.Utils.is_i32b 4190290 (mont_red value))
  = assert_norm (8380416 * 41978 <= 8380416 * pow2 32);
    lemma_mont_red_bound_internal (8380416 * 41978) value;
    assert_norm (4190209 + (8380416 * 41978) / pow2 32 == 4190290)

(* Input bound: 256 · FIELD_MAX · 41978.  Specifically the input to
   the FINAL `montgomery_multiply_by_constant(_, 41_978)` call inside
   `invert_ntt_montgomery` (where the layered NTT has scaled up to
   256 · FIELD_MAX before the final scale-back).  This is the bound
   that propagates out of `invert_ntt_montgomery` and unblocks the
   `compute_as1_plus_s2` body proof in Sprint 2. *)
let lemma_mont_red_bound_256_field_max_times_41978 (value: i64)
    = assert_norm (256 * 8380416 * 41978 <= 8380416 * pow2 32);
    lemma_mont_red_bound_internal (256 * 8380416 * 41978) value;
    assert_norm (4190209 + (256 * 8380416 * 41978) / pow2 32 == 4211177)

(* Mod-q correctness for `mont_red`: the result is congruent to
   `value * R^(-1) mod q` where R = 2^32 and R^(-1) mod q = 8265825.

   USER-FOLLOWUP: admitted for now.  Lifting a proper proof here from
   the original inline calc-chain in
   `simd/portable/arithmetic.rs::montgomery_reduce_element` is
   tractable but I (the agent) couldn't close it within budget.

   The original calc-chain (preserved verbatim from the impl body
   prior to the Stage 2 refactor — at git tip e945e1954, lines
   153-192 of `src/simd/portable/arithmetic.rs`) is reproduced below
   for reference.  Local variables are named after the impl body's
   bindings (`value`, `value_high`, `c`, `k`, `k_times_modulus`, `res`)
   which correspond to mont_red's `(value, hi, c, k, kq_product, result)`
   after revealing `mont_red value`.

       calc ( == ) {
           v $k_times_modulus % pow2 32;
           ( == ) { assert (v $k_times_modulus == v $k * 8380417) }
           (v $k * 8380417) % pow2 32;
           ( == ) { assert (v $k = ((v $value % pow2 32) * 58728449) @% pow2 32) }
           ((((v $value % pow2 32) * 58728449) @% pow2 32) * 8380417) % pow2 32;
           ( == ) { Math.Lemmas.lemma_mod_sub ((((v $value % pow2 32) * 58728449) % pow2 32) * 8380417) (pow2 32) 8380417 }
           ((((v $value % pow2 32) * 58728449) % pow2 32) * 8380417) % pow2 32;
           ( == ) { Math.Lemmas.lemma_mod_mul_distr_l ((v $value % pow2 32) * 58728449) 8380417 (pow2 32) }
           ((((v $value % pow2 32) * 58728449) * 8380417) % pow2 32);
           ( == ) { Math.Lemmas.lemma_mod_mul_distr_r (v $value % pow2 32) (58728449 * 8380417) (pow2 32) }
           ((v $value % pow2 32) % pow2 32);
           ( == ) { Math.Lemmas.lemma_mod_sub (v $value) (pow2 32) 1 }
           (v $value) % pow2 32;
       };
       Math.Lemmas.modulo_add (pow2 32) (- (v $k_times_modulus)) (v $value) (v $k_times_modulus);
       assert ((v $value - v $k_times_modulus) % pow2 32 == 0)

   Then a second calc-chain establishes the mod-q goal:

       calc ( == ) {
           v $res % 8380417;
           ( == ) { assert (v $res == v $value_high - v $c) }
           (v $value / pow2 32 - v $k_times_modulus / pow2 32) % 8380417;
           ( == ) { Math.Lemmas.lemma_div_exact (v $value - v $k_times_modulus) (pow2 32) }
           ((v $value - v $k_times_modulus) / pow2 32) % 8380417;
           ( == ) { assert ((pow2 32 * 8265825) % 8380417 == 1) }
           (((v $value - v $k_times_modulus) / pow2 32) * ((pow2 32 * 8265825) % 8380417)) % 8380417;
           ( == ) { Math.Lemmas.lemma_mod_mul_distr_r ((v $value - v $k_times_modulus) / pow2 32) (pow2 32 * 8265825) 8380417 }
           (((v $value - v $k_times_modulus) / pow2 32) * pow2 32 * 8265825) % 8380417;
           ( == ) { Math.Lemmas.lemma_div_exact (v $value - v $k_times_modulus) (pow2 32) }
           ((v $value - v $k_times_modulus) * 8265825) % 8380417;
           ( == ) { assert (v $k_times_modulus == (v $k @% pow2 32) * 8380417) }
           ((v $value * 8265825) - ((v $k @% pow2 32) * 8380417 * 8265825)) % 8380417;
           ( == ) { Math.Lemmas.lemma_mod_sub (v $value * 8265825) 8380417 ((v $k @% pow2 32) * 8265825) }
           (v $value * 8265825) % 8380417;
       }

   To lift to this lemma: bind the mont_red body's variables
   (`hi`, `low`, `k`, `c`) via `reveal_opaque mont_red value`, then
   run the two calc-chains above with those bindings.  See also
   `lemma_mont_mul_bound_and_mod_q` in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst:656`
   which proves the same property for `mont_mul` (= `mont_red ∘ i32_mul`)
   with a fully-spelled-out proof.  *)
#push-options "--z3rlimit 600 --fuel 0 --ifuel 1"
let lemma_mont_red_mod_q (value: i64)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i64_inttype;
    Spec.Intrinsics.reveal_opaque_cast_ops #i32_inttype #i64_inttype;
    reveal_opaque (`%mont_red) (mont_red value);
    reveal_opaque (`%i32_mul) (i32_mul);
    let val_int : int = v value in
    // Step 1: hi = cast_mod (value >> 32) <: i32 = value / 2^32.
    let val_shifted : i64 = value >>! mk_i32 32 in
    assert (v val_shifted == val_int / pow2 32);
    // Under |value| <= 8380416 * pow2 32, |val_int / pow2 32| <= 8380416 < pow2 31,
    // so the cast_mod (@% pow2 32) is the identity.
    assert_norm (8380416 < pow2 31);
    Spec.Utils.lemma_range_at_percent (val_int / pow2 32) (pow2 32);
    let hi : i32 = cast val_shifted <: i32 in
    assert (v hi == val_int / pow2 32);
    // Step 2: low = cast_mod value <: i32 = value @% 2^32.
    let low : i32 = cast value <: i32 in
    assert (v low == val_int @% pow2 32);
    // Step 3: k = cast_mod (low * Q' as i64) <: i32 = (low * Q') @% 2^32.
    let q'_i32 = mk_i32 58728449 in
    Spec.Utils.lemma_range_at_percent (v low) (pow2 64);
    Spec.Utils.lemma_range_at_percent 58728449 (pow2 64);
    let lq_product : i64 = i32_mul low q'_i32 in
    assert_norm (pow2 31 * 58728449 < pow2 63);
    Spec.Utils.lemma_range_at_percent (v low * 58728449) (pow2 64);
    assert (v lq_product == v low * 58728449);
    let k : i32 = cast lq_product <: i32 in
    assert (v k == (v low * 58728449) @% pow2 32);
    // Step 4: c = cast_mod ((k * Q as i64) >> 32) = (k*q)/2^32.
    let q_i32 = mk_i32 8380417 in
    Spec.Utils.lemma_range_at_percent (v k) (pow2 64);
    Spec.Utils.lemma_range_at_percent 8380417 (pow2 64);
    let kq_product : i64 = i32_mul k q_i32 in
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
    // Step 5: result = sub_mod hi c.
    assert_norm (8380416 + (pow2 31 * 8380417 / pow2 32) < pow2 31);
    let result : i32 = hi -! c in
    assert (v result == v hi - v c);
    // === MOD-q PROOF (mirroring the sibling lemma_mont_mul_bound_and_mod_q
    //     calc chain; prod -> v value since `value` is the direct mont_red input) ===
    // Show: (k * q) % 2^32 == value % 2^32  (so value - k*q is divisible by 2^32).
    assert_norm ((58728449 * 8380417) % pow2 32 == 1);
    Spec.Utils.lemma_at_percent_mod (v low * 58728449) (pow2 32);
    FStar.Math.Lemmas.lemma_mod_mul_distr_l (v low * 58728449) 8380417 (pow2 32);
    FStar.Math.Lemmas.lemma_mod_mul_distr_l ((v low * 58728449) @% pow2 32) 8380417 (pow2 32);
    Spec.Utils.lemma_at_percent_mod (v low * 58728449) (pow2 32);
    // (v k * 8380417) % 2^32 == (v low * 58728449 * 8380417) % 2^32
    //                        == (v low * 1) % 2^32  (using q'*q ≡ 1 mod 2^32)
    //                        == v low % 2^32 == v value % 2^32
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v low) (58728449 * 8380417) (pow2 32);
    Spec.Utils.lemma_at_percent_mod val_int (pow2 32);
    assert ((v k * 8380417) % pow2 32 == val_int % pow2 32);
    // (value - k*q) % 2^32 == 0:
    FStar.Math.Lemmas.lemma_mod_sub_distr val_int (v k * 8380417) (pow2 32);
    assert ((val_int - v k * 8380417) % pow2 32 == 0);
    FStar.Math.Lemmas.lemma_div_exact (val_int - v k * 8380417) (pow2 32);
    // hi - c = value/2^32 - (k*q)/2^32 = (value - k*q)/2^32.
    assert (v result == (val_int - v k * 8380417) / pow2 32);
    // Final step: ((value - k*q)/2^32) % q == (value * 8265825) % q.
    assert_norm ((pow2 32 * 8265825) % 8380417 == 1);
    FStar.Math.Lemmas.lemma_mod_mul_distr_r ((val_int - v k * 8380417) / pow2 32) (pow2 32 * 8265825) 8380417;
    FStar.Math.Lemmas.lemma_div_exact (val_int - v k * 8380417) (pow2 32);
    FStar.Math.Lemmas.lemma_mod_sub (val_int * 8265825) 8380417 (v k * 8265825)
#pop-options

(* Two-arg Montgomery multiplication: thin non-opaque wrapper over
   `mont_red`.  Callers who need to inspect the arithmetic only need
   to reveal `mont_red`, not `mont_mul` — this keeps the opacity
   layer at a single level. *)

(* Bound + mod-q correctness for `mont_mul` when BOTH operands are bounded
   by NTT_OUTPUT_BOUND = 9*FIELD_MAX (the lazily-accumulated forward-NTT
   output bound).  The product `i32_mul x y` is then bounded by
   (9*FIELD_MAX)^2 = 81*FIELD_MAX^2, which is < FIELD_MAX*pow2 32 (indeed
   81*FIELD_MAX < pow2 31), so `mont_red` reduces it back to a value bounded
   by 4190209 + (81*FIELD_MAX^2)/pow2 32 = 5514722 <= FIELD_MAX = 8380416.

   This is the below-trait fact that lets `montgomery_multiply`'s output post
   stay FIELD_MAX even though both operands may be NTT-domain values up to
   9*FIELD_MAX (the forward NTT is deliberately not reduced; the multiply
   absorbs the lazy bound).  Mirrors `lemma_mont_mul_bound_and_mod_q` (which
   requires `is_i32b 8380416 y`, one operand bounded by FIELD_MAX) but bounds
   BOTH operands by 9*FIELD_MAX. *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_mont_mul_bound_and_mod_q_ntt_output (x y: i32)
    = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i64_inttype;
    Spec.Intrinsics.reveal_opaque_cast_ops #i32_inttype #i64_inttype;
    reveal_opaque (`%i32_mul) (i32_mul);
    let prod : int = v x * v y in
    // product = i32_mul x y (i64 with v == prod, since |prod| < pow2 63).
    assert_norm ((9 * 8380416) * (9 * 8380416) < pow2 63);
    Spec.Utils.lemma_range_at_percent (v x) (pow2 64);
    Spec.Utils.lemma_range_at_percent (v y) (pow2 64);
    let cast_x : i64 = cast x <: i64 in
    let cast_y : i64 = cast y <: i64 in
    assert (v cast_x == v x /\ v cast_y == v y);
    let value : i64 = i32_mul x y in
    Spec.Utils.lemma_range_at_percent prod (pow2 64);
    assert (v value == prod);
    // |value| <= 81*FIELD_MAX^2 <= 8380416*pow2 32, so the parametric mont_red
    // bound applies and yields an output bounded by 5514722 <= 8380416.
    assert_norm ((9 * 8380416) * (9 * 8380416) <= 8380416 * pow2 32);
    assert (Spec.Utils.is_i64b ((9 * 8380416) * (9 * 8380416)) value);
    lemma_mont_red_bound_internal ((9 * 8380416) * (9 * 8380416)) value;
    assert_norm (4190209 + ((9 * 8380416) * (9 * 8380416)) / pow2 32 <= 8380416);
    // mod-q correctness: mont_red value ≡ value * R^{-1} (mod q).
    lemma_mont_red_mod_q value
#pop-options


     
    











let rec hint_counter_loop hint_1 hint_2 n =
  if n = 0 then begin
    eq_repeati0 (sz n) (hint_counter hint_1) 0;
    eq_repeati0 (sz n) (hint_counter hint_2) 0;
    () end
  else begin
    hint_counter_loop hint_1 hint_2 (n - 1);
    unfold_repeati (sz n) (hint_counter hint_1) 0 (sz (n - 1));
    unfold_repeati (sz n) (hint_counter hint_2) 0 (sz (n - 1));
    () end









#push-options "--z3rlimit 1500 --fuel 3 --ifuel 3 --ext context_pruning --z3refresh"

let rejection_sample_coefficient_lemma (randomness:Seq.seq u8) (i:usize{v i < (Seq.length randomness) / 3}) =
  let b0 = cast (Seq.index randomness (v i * 3)) <: i32 in
  let b1 = cast (Seq.index randomness (v i * 3 + 1)) <: i32 in
  let b2 = cast (Seq.index randomness (v i * 3 + 2)) <: i32 in
  let b2' = if b2 >. mk_i32 127 then b2 -. mk_i32 128  else b2 in
  assert_norm (pow2 23 == 8388608);
  assert (b2' == (b2 %! mk_i32 128));
  assert (((mk_i32 (pow2 16) *. b2) %! mk_i32 (pow2 23)) == (mk_i32 (pow2 16) *. (b2 %! mk_i32 128)));
  logor_disjoint (b2 <<! mk_i32 16) (b1 <<! mk_i32 8) 16;
  assert (((b2 <<! mk_i32 16) |. (b1 <<! mk_i32 8)) == ((b2 <<! mk_i32 16) +. (b1 <<! mk_i32 8)));
  logor_disjoint ((b2 <<! mk_i32 16) |. (b1 <<! mk_i32 8)) b0 8;
  assert ((((b2 <<! mk_i32 16) |. (b1 <<! mk_i32 8)) |. b0) ==
    (((b2 <<! mk_i32 16) +. (b1 <<! mk_i32 8)) +. b0));
  assert ((b2 <<! mk_i32 16) == (mk_i32 (pow2 16) *. b2));
  assert ((b1 <<! mk_i32 8) == (mk_i32 (pow2 8) *. b1));
  logand_mask_lemma (((mk_i32 (pow2 16) *. b2) +. (mk_i32 (pow2 8) *. b1)) +. b0) 23;
  assert (((((mk_i32 (pow2 16) *. b2) +. ((mk_i32 (pow2 8) *. b1)) +. b0)) %! mk_i32 (pow2 23)) ==
    ((((mk_i32 (pow2 16) *. b2) %! mk_i32 (pow2 23)) +. ((mk_i32 (pow2 8) *. b1)) +. b0)));
  assert (((((mk_i32 (pow2 16) *. b2) +. (mk_i32 (pow2 8) *. b1)) +. b0) %! mk_i32 (pow2 23)) ==
    (((mk_i32 (pow2 16) *. (b2 %! mk_i32 128)) +. (mk_i32 (pow2 8) *. b1)) +. b0));
  assert (((((mk_i32 (pow2 16) *. b2) +. (mk_i32 (pow2 8) *. b1)) +. b0) %! mk_i32 (pow2 23)) ==
    (((mk_i32 (pow2 16) *. b2') +. (mk_i32 (pow2 8) *. b1)) +. b0))

#pop-options

(* Montgomery-domain lift of the inverse NTT output: to_mont x = mod_q(R*x),
   R = 2^32 mod q = 4193792.  The inverse-NTT impl stays in the Montgomery
   domain (mont_mul by 41978 = R*256^{-1}), so its output is the clean intt
   times R.  Relocated here (a base spec module, below the SIMD trait) so the
   `Operations::invert_ntt_montgomery` trait post can state functional
   correctness `out ≡ to_mont (intt in) (mod q)`.  Byte-identical to the
   def in `Libcrux_ml_dsa.Simd.Portable.Invntt` (so the two are defeq and the
   free-fn post bridges to the trait post with no rewrite). *)

