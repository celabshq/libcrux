module Libcrux_ml_kem.Vector.Avx2.Concat_pairs_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views

(* ============================================================================
   concat-pairs (madd-by-[2^n;1]) keystone — `mm256_concat_pairs_n`.

   `mm256_concat_pairs_n n x = madd(x, set_epi16(2^n, 1, 2^n, 1, ...))`.  When
   every i16 lane of `x` has its bits >= n ZERO, the 32-bit lane value is the
   exact bit CONCATENATION `x_{2q} + 2^n * x_{2q+1}` (no carries), so bit b of
   the 32-lane is bit b of the even lane (b < n), bit b-n of the odd lane
   (n <= b < 2n), and 0 otherwise.  This is the shared P2 obligation behind
   serialize_4 / _5 / _10 / _12.

   WHY ITS OWN MODULE.  In `Libcrux_intrinsics.Avx2_ml_kem_views` (2100+ lines,
   ~50 SMTPat view/op facts) these two lemmas suffer solver-state pollution from
   the host module's earlier queries: the same ground per-arm assertions
   flip-flop between passing and canceled-400.000.  A dedicated module gives them
   a fresh solver and a small pruned context, and —
   per the SMTPat rule (`feedback_smtpat_only_for_user_consumed_lemmas`) — the
   proof below reaches every companion fact by an EXPLICIT CALL rather than by
   ambient pattern firing.  Digit-bridge helpers (`lemma_bv_bit_lane32_digit`,
   `lemma_bv_bit_lane16_digit`, `lemma_lane_high_zero_bound`,
   `lemma_concat_digit`) stay in the companion: they are width-generic and are
   reused by every deserialize proof, which must not depend on this module.
   ========================================================================== *)

(* `16 * l + c` lands in lane `l` at bit `c`.  The `% 16` is nonlinear at a
   symbolic `l`, so it is discharged here rather than inside the keystone. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_lane_window_index (l: nat{l < 16}) (c: nat{c < 16})
  : Lemma (16 * l + c < 256 /\ (16 * l + c) % 16 == c) =
  FStar.Math.Lemmas.lemma_mod_plus c l 16;
  FStar.Math.Lemmas.small_mod c 16
#pop-options

(* The whole arithmetic core of the keystone, over plain integers: with both
   halves in [0, 2^n) and n <= 12, the pair sum is a carry-free concatenation
   that fits in 31 bits, so the i32 wrap-around `@%` is the identity.

   Kept OUT of the keystone body on purpose: proved there, these `pow2` /
   `@%` steps cost 131 s across 3 sub-queries (272/181/155 of 400) because
   every one of them re-enters the lane-view context; here they are ground
   integer arithmetic. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 200"
let lemma_concat_value_ok (nn: nat{1 <= nn /\ nn <= 12}) (x0 x1 sh: nat)
  : Lemma (requires x0 < pow2 nn /\ x1 < pow2 nn /\ sh == pow2 nn)
          (ensures (let s = x0 + pow2 nn * x1 in
                    0 <= s /\ s < pow2 31 /\ x0 * 1 + x1 * sh == s /\
                    s @% 4294967296 == s)) =
  let s = x0 + pow2 nn * x1 in
  FStar.Math.Lemmas.pow2_plus nn nn;
  FStar.Math.Lemmas.lemma_mult_lt_left (pow2 nn) x1 (pow2 nn);
  FStar.Math.Lemmas.pow2_le_compat 24 (2 * nn);
  assert_norm (pow2 24 < pow2 31);
  assert (0 <= s /\ s < pow2 31);
  assert_norm (pow2 32 == 4294967296);
  assert_norm (pow2 31 + pow2 31 == pow2 32);
  FStar.Math.Lemmas.small_mod s (pow2 32)
#pop-options

(* The two multiplier lanes of the `set_epi16(2^n,1,…)` constant at pair `q`.

   Factored out on purpose (skill §7, "shared-context literal dispatch
   saturates").  The 8-way ground dispatch is unavoidable — instantiating the
   16 ground lane equalities at a SYMBOLIC `2 * q` never unifies — but done
   INSIDE the keystone it drags the keystone's whole context (the madd value,
   `x`'s high-zero forall, the pow2 bounds, the split ensures) into all 8
   branches: measured 480 s / 8 heavy sub-queries (up to 286/400) on the cold
   first landing of this module.  Here the branch context is just the 16 ground
   multiplier lanes, so every arm is trivial and the keystone gets the pair as
   ONE fact at symbolic `q`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
let lemma_mults_lane_pair (sh: i16) (q: nat{q < 8})
  : Lemma
      (ensures (let m = mm256_set_epi16 sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)
                          sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) in
                get_lane m (2 * q) == mk_i16 1 /\ get_lane m (2 * q + 1) == sh)) =
  lemma_mm256_set_epi16_lanes sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)
                              sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1);
  if q = 0 then () else if q = 1 then () else if q = 2 then () else if q = 3 then ()
  else if q = 4 then () else if q = 5 then () else if q = 6 then () else ()
#pop-options

(* The madd 32-lane VALUE: exact bit concatenation of the two i16 half lanes.

   `sh` is threaded as a FREE parameter (with `v sh == 2^(v n)` in the requires)
   so the call site links by congruence against its own `1 <<! n` binding rather
   than re-deriving the shift value here. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_concat_pairs_lane32 (n: u8) (sh: i16) (x: t_Vec256) (q: nat{q < 8})
  : Lemma
      (requires 1 <= v n /\ v n <= 12 /\ v sh == pow2 (v n) /\
                (forall (l: nat{l < 256}). l % 16 >= v n ==> bv_bit x l = 0))
      (ensures (let r = mm256_madd_epi16 x
                          (mm256_set_epi16 sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)
                             sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)) in
                v (get_lane x (2 * q)) >= 0 /\ v (get_lane x (2 * q)) < pow2 (v n) /\
                v (get_lane x (2 * q + 1)) >= 0 /\ v (get_lane x (2 * q + 1)) < pow2 (v n) /\
                lane32 r q == v (get_lane x (2 * q)) + pow2 (v n) * v (get_lane x (2 * q + 1)) /\
                0 <= lane32 r q /\ lane32 r q < pow2 31)) =
  let nn = v n in
  let m = mm256_set_epi16 sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)
            sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) in
  let r = mm256_madd_epi16 x m in
  (* EXPLICIT: the two multiplier lanes at pair q (even = 1, odd = sh) ... *)
  lemma_mults_lane_pair sh q;
  (* ... and the madd 32-lane pair sum. *)
  lemma_madd_epi16_lane32 x m;
  assert (v (get_lane m (2 * q)) == 1);
  assert (v (get_lane m (2 * q + 1)) == v sh);
  assert (lane32 r q == (v (get_lane x (2 * q)) * 1 + v (get_lane x (2 * q + 1)) * v sh) @% 4294967296);
  let hz (l: nat{l < 16}) : Lemma (forall (c: nat{c < 16}). c >= nn ==> bv_bit x (16 * l + c) = 0) =
    let aux (c: nat{c < 16}) : Lemma (c >= nn ==> bv_bit x (16 * l + c) = 0) =
      lemma_lane_window_index l c
    in
    Classical.forall_intro aux
  in
  hz (2 * q); hz (2 * q + 1);
  lemma_lane_high_zero_bound x (2 * q) nn;
  lemma_lane_high_zero_bound x (2 * q + 1) nn;
  let x0: nat = v (get_lane x (2 * q)) in
  let x1: nat = v (get_lane x (2 * q + 1)) in
  lemma_concat_value_ok nn x0 x1 (v sh)
#pop-options

(* THE concat-pairs bit obligation — pure composition of the lane32 keystone
   with the (companion) digit bridges. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_concat_pairs_bits (n: u8) (sh: i16) (x: t_Vec256) (i: nat{i < 256})
  : Lemma
      (requires 1 <= v n /\ v n <= 12 /\ v sh == pow2 (v n) /\
                (forall (l: nat{l < 256}). l % 16 >= v n ==> bv_bit x l = 0))
      (ensures (let r = mm256_madd_epi16 x
                          (mm256_set_epi16 sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)
                             sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)) in
                bv_bit r i ==
                (if i % 32 < v n then bv_bit x ((i / 32) * 32 + i % 32)
                 else if i % 32 < 2 * v n then bv_bit x ((i / 32) * 32 + 16 + (i % 32 - v n))
                 else 0))) =
  let nn = v n in
  let m = mm256_set_epi16 sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1)
            sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) sh (mk_i16 1) in
  let r = mm256_madd_epi16 x m in
  let q = i / 32 in
  let b = i % 32 in
  FStar.Math.Lemmas.euclidean_division_definition i 32;
  lemma_concat_pairs_lane32 n sh x q;
  lemma_bv_bit_lane32_digit r q b;
  let x0 = v (get_lane x (2 * q)) in
  let x1 = v (get_lane x (2 * q + 1)) in
  lemma_concat_digit x0 x1 nn b;
  (if b < nn then lemma_bv_bit_lane16_digit x (2 * q) b
   else if b < 2 * nn then lemma_bv_bit_lane16_digit x (2 * q + 1) (b - nn)
   else ());
  assert (32 * q == (i / 32) * 32);
  assert (16 * (2 * q) + b == (i / 32) * 32 + i % 32 \/ ~(b < nn));
  assert (16 * (2 * q + 1) + (b - nn) == (i / 32) * 32 + 16 + (i % 32 - nn) \/ ~(b >= nn /\ b < 2 * nn))
#pop-options
