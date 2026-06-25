module Hacspec_ml_dsa.Commute.Chunk
(* ABSTRACT interface for the per-element / per-layer commute lemma library.
   Hand-written companion to Hacspec_ml_dsa.Commute.Chunk.fst.  Exposes ONLY the
   declarations used by external consumers (the AVX2/Portable NTT+Invntt modules,
   Avx2NttTheory, Avx2NttCompose): the transparent per-lane spec reducers
   (`*_lane`, `simd_units_to_array`) consumers compute against are kept here as
   `let`; every externally-called lemma is exposed as a `val` (its proof BODY stays
   in the .fst).  The many internal-only helper lemmas (chunk_pair / cross_pair /
   *_chunk_to_hacspec / clamp / bittrick / int_bridge) are NOT exposed, so closing
   an admit by editing a .fst proof body leaves this interface untouched and the
   heavy NTT consumers' .checked stay valid (no cold reprove). *)
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Libcrux_ml_dsa.Simd.Traits.Specs

module P = Hacspec_ml_dsa.Parameters
module A = Hacspec_ml_dsa.Arithmetic
module L = FStar.Math.Lemmas
module TS = Libcrux_ml_dsa.Simd.Traits.Specs

val lemma_reduce_lane_commute (input result: i32)
    : Lemma
        (requires
          Spec.Utils.is_i32b 8380416 result /\
          (v input) % 8380417 == (v result) % 8380417)
        (ensures TS.reduce_lane_post input result)

val lemma_barrett_red_bound_and_mod_q (x: i32)
    : Lemma
        (requires Spec.Utils.is_i32b 2143289343 x)
        (ensures
          Spec.Utils.is_i32b 8380416 (Spec.MLDSA.Math.barrett_red x) /\
          (v (Spec.MLDSA.Math.barrett_red x)) % 8380417 == (v x) % 8380417)

val lemma_add_lane_commute (lhs rhs lhs_future: i32)
    : Lemma
        (requires
          Libcrux_ml_dsa.Simd.Traits.Specs.int_is_i32 (v lhs + v rhs) /\
          lhs_future == Spec.Intrinsics.add_mod_opaque lhs rhs)
        (ensures v lhs_future == v lhs + v rhs)

val lemma_sub_lane_commute (lhs rhs lhs_future: i32)
    : Lemma
        (requires
          Libcrux_ml_dsa.Simd.Traits.Specs.int_is_i32 (v lhs - v rhs) /\
          lhs_future == Spec.Intrinsics.sub_mod_opaque lhs rhs)
        (ensures v lhs_future == v lhs - v rhs)

val lemma_power2round_lane_commute (input future_t1 future_t0: i32)
    : Lemma
        (requires
          (let pair = Spec.MLDSA.Math.power2round (v input) in
           v future_t0 == fst pair /\ v future_t1 == snd pair))
        (ensures TS.power2round_lane_post input future_t1 future_t0)

val lemma_power2round_t1_bound (input: i32)
    : Lemma
        (let (_, t1_s) = Spec.MLDSA.Math.power2round (v input) in
         0 <= t1_s /\ t1_s < pow2 10)

val lemma_power2round_t0_strict_lower_bound (input: i32)
    : Lemma
        (let (t0_s, _) = Spec.MLDSA.Math.power2round (v input) in
         -pow2 12 < t0_s /\ t0_s <= pow2 12)

val lemma_shift_left_then_reduce_lane_commute (input future: i32)
    : Lemma
        (requires
          v input >= 0 /\ v input <= 261631 /\
          future == Spec.MLDSA.Math.barrett_red
                      (Spec.Intrinsics.shift_left_opaque input (mk_i32 13)))
        (ensures TS.shift_left_then_reduce_lane_post input future)

val lemma_shift_left_then_reduce_lane_commute_mod_q
    (input future: i32)
    : Lemma
        (requires
          v input >= 0 /\ v input <= 261631 /\
          Spec.Utils.is_i32b 8380416 future /\
          Spec.MLDSA.Math.mod_q (v future) ==
            Spec.MLDSA.Math.mod_q (v (input <<! mk_i32 13 <: i32)))
        (ensures TS.shift_left_then_reduce_lane_post input future)

val lemma_use_one_hint_bound (g r hint: i32)
    : Lemma
        (requires
          (v g == 95232 \/ v g == 261888) /\
          (v hint == 0 \/ v hint == 1))
        (ensures
          (let res = Spec.MLDSA.Math.use_one_hint (v g) (v r) (v hint) in
           (v g == 95232 ==> 0 <= res /\ res < 44) /\
           (v g == 261888 ==> 0 <= res /\ res < 16)))

val lemma_use_hint_lane_commute_conditional
    (gamma2 input hint future_hint: i32)
    : Lemma
        (requires
          (v gamma2 == 95232 \/ v gamma2 == 261888) /\
          (v hint == 0 \/ v hint == 1) /\
          v future_hint == Spec.MLDSA.Math.use_one_hint (v gamma2) (v input) (v hint))
        (ensures TS.use_hint_lane_post gamma2 input hint future_hint)

val lemma_decompose_bound (gamma2 r: i32)
    : Lemma
        (requires (v gamma2 == 95232 \/ v gamma2 == 261888))
        (ensures
          (let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v gamma2) (v r) in
           - (v gamma2) <= r0_s /\ r0_s <= v gamma2 /\
           (v gamma2 == 95232 ==> 0 <= r1_s /\ r1_s < 44) /\
           (v gamma2 == 261888 ==> 0 <= r1_s /\ r1_s < 16)))

val lemma_decompose_lane_commute_conditional
    (gamma2 input low_future high_future: i32)
    : Lemma
        (requires
          (v gamma2 == 95232 \/ v gamma2 == 261888) /\
          (let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v gamma2) (v input) in
           v low_future == r0_s /\ v high_future == r1_s))
        (ensures TS.decompose_lane_post gamma2 input low_future high_future)

val lemma_compute_one_hint_bound (low high gamma2: i32)
    : Lemma
        (let res = Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2) in
         res == 0 \/ res == 1)

val lemma_compute_hint_bound (hint: t_Array i32 (sz 8))
    : Lemma
        (requires
          (forall (i: nat). i < 8 ==>
            (v (Seq.index hint i) == 0 \/ v (Seq.index hint i) == 1)))
        (ensures Spec.MLDSA.Math.compute_hint hint <= 8)

val lemma_compute_hint_lane_commute_conditional
    (gamma2 low high hint_future: i32)
    : Lemma
        (requires
          (v gamma2 == 95232 \/ v gamma2 == 261888) /\
          v hint_future == Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2))
        (ensures TS.compute_hint_lane_post gamma2 low high hint_future)

val lemma_mont_mul_bound_and_mod_q (x y: i32)
    : Lemma
        (requires Spec.Utils.is_i32b 8380416 y)
        (ensures
          Spec.Utils.is_i32b 8380416 (Spec.MLDSA.Math.mont_mul x y) /\
          (v (Spec.MLDSA.Math.mont_mul x y)) % 8380417 ==
          (v x * v y * 8265825) % 8380417)

let simd_units_to_array
      (chunks: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : t_Array i32 (mk_usize 256)
  = Hacspec_ml_dsa.createi #i32 (mk_usize 256)
      #(usize -> i32)
      (fun (i: usize{i <. mk_usize 256}) ->
         Seq.index (Seq.index chunks (v i / 8)) (v i % 8))

val lemma_simd_units_to_array_reveal
      (chunks: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
      (b: nat{b < 32}) (l: nat{l < 8})
    : Lemma
        (Seq.index (simd_units_to_array chunks) (8*b + l) ==
         Seq.index (Seq.index chunks b) l)

let layer_0_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 2 in
  let idx:usize = i %! mk_usize 2 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 128 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 1
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 1 <: usize ] <: i32) <: i64)
          <:
          i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (t <: i32) <: i64)
        <:
        i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 1 <: usize ] <: i32) <: i64) -!
        (cast (t <: i32) <: i64)
        <:
        i64)

val lemma_ntt_layer_0_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 0)) (v i) == layer_0_lane p i)

val lemma_mod_q_v (a: i64)
    : Lemma (v (Hacspec_ml_dsa.Arithmetic.mod_q a) == (v a) % 8380417)

val lemma_ntt_layer_0_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t zm: (b: nat{b < 32} -> p: nat{p < 4} -> i32))
    : Lemma
        (requires
          (forall (b: nat{b < 32}) (p: nat{p < 4}).
           (let ci = Seq.index input b in
            let co = Seq.index transformed b in
            let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] in
            v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (t b p) /\
            v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v (t b p) /\
            (v (t b p)) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zm b p) * 8265825) % 8380417 /\
            (v (zm b p)) % 8380417 == (v z * pow2 32) % 8380417)))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 ==
             (v (Seq.index spec i)) % 8380417))

let layer_1_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 4 in
  let idx:usize = i %! mk_usize 4 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 64 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 2
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 2 <: usize ] <: i32) <: i64)
          <:
          i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (t <: i32) <: i64)
        <:
        i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 2 <: usize ] <: i32) <: i64) -!
        (cast (t <: i32) <: i64)
        <:
        i64)

val lemma_ntt_layer_1_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 1)) (v i) == layer_1_lane p i)

val lemma_ntt_layer_1_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (b: nat{b < 32} -> h: nat{h < 2} -> j: nat{j < 2} -> i32))
    (zm: (b: nat{b < 32} -> h: nat{h < 2} -> i32))
    : Lemma
        (requires
          (* Butterfly relations quantified per (b, h, j): the natural trigger
             `t b h j` covers all three binders.  The zeta congruence lives in
             a SEPARATE forall per (b, h): inside the (b,h,j) forall the only
             all-binder trigger is `t b h j`, which the zeta sub-goals never
             mention, so under --split_queries Z3 cannot instantiate there
             (observed: "incomplete quantifiers" on exactly the two zeta
             conjuncts of the chunk-lemma call). *)
          (forall (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2}).
           (let ci = Seq.index input b in
            let co = Seq.index transformed b in
            v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (t b h j) /\
            v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v (t b h j) /\
            (v (t b h j)) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zm b h) * 8265825) % 8380417)) /\
          (forall (b: nat{b < 32}) (h: nat{h < 2}).
           (v (zm b h)) % 8380417 ==
           (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + h + 64) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 ==
             (v (Seq.index spec i)) % 8380417))

let layer_2_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 8 in
  let idx:usize = i %! mk_usize 8 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 32 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 4
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 4 <: usize ] <: i32) <: i64)
          <:
          i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (t <: i32) <: i64)
        <:
        i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 4 <: usize ] <: i32) <: i64) -!
        (cast (t <: i32) <: i64)
        <:
        i64)

val lemma_ntt_layer_2_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 2)) (v i) == layer_2_lane p i)

val lemma_ntt_layer_2_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (b: nat{b < 32} -> p: nat{p < 4} -> i32))
    (zm: (b: nat{b < 32} -> i32))
    : Lemma
        (requires
          (* Same trigger-alignment split as layer 1: the zeta congruence
             only mentions `zm b` (no p), so it gets its own per-b forall. *)
          (forall (b: nat{b < 32}) (p: nat{p < 4}).
           (let ci = Seq.index input b in
            let co = Seq.index transformed b in
            v (Seq.index co p)     == v (Seq.index ci p) + v (t b p) /\
            v (Seq.index co (p+4)) == v (Seq.index ci p) - v (t b p) /\
            (v (t b p)) % 8380417 == (v (Seq.index ci (p+4)) * v (zm b) * 8265825) % 8380417)) /\
          (forall (b: nat{b < 32}).
           (v (zm b)) % 8380417 ==
           (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 ==
             (v (Seq.index spec i)) % 8380417))

val lemma_cross_idx (s:pos{16 % s == 0 /\ s <= 16}) (ulo:nat{ulo < 32 /\ ulo % (2*s) < s}) (l:nat{l<8})
  : Lemma (ensures
      ulo + s < 32 /\
      (8*ulo+l) / (16*s) == ulo/(2*s) /\
      (8*ulo+l) % (16*s) == 8*(ulo%(2*s)) + l /\
      8*(ulo%(2*s)) + l < 8*s /\
      (8*ulo + 8*s + l) / (16*s) == ulo/(2*s) /\
      (8*ulo + 8*s + l) % (16*s) == 8*s + 8*(ulo%(2*s)) + l /\
      8*s + 8*(ulo%(2*s)) + l >= 8*s /\
      8*s + 8*(ulo%(2*s)) + l < 16*s)

let layer_3_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 16 in
  let idx:usize = i %! mk_usize 16 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 16 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 8
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 8 <: usize ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +! (cast (t <: i32) <: i64) <: i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 8 <: usize ] <: i32) <: i64) -! (cast (t <: i32) <: i64) <: i64)

val lemma_ntt_layer_3_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 3)) (v i) == layer_3_lane p i)

val lemma_ntt_layer_3_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 2 == 0 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+1) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+1) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t ulo l) /\
            v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t ulo l) /\
            (v (t ulo l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 2 == 0 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/2 + 16) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let layer_4_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 32 in
  let idx:usize = i %! mk_usize 32 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 8 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 16
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 16 <: usize ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +! (cast (t <: i32) <: i64) <: i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 16 <: usize ] <: i32) <: i64) -! (cast (t <: i32) <: i64) <: i64)

val lemma_ntt_layer_4_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 4)) (v i) == layer_4_lane p i)

val lemma_ntt_layer_4_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 4 < 2 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+2) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+2) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t ulo l) /\
            v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t ulo l) /\
            (v (t ulo l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 4 < 2 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/4 + 8) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let layer_5_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 64 in
  let idx:usize = i %! mk_usize 64 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 4 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 32
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 32 <: usize ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +! (cast (t <: i32) <: i64) <: i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 32 <: usize ] <: i32) <: i64) -! (cast (t <: i32) <: i64) <: i64)

val lemma_ntt_layer_5_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 5)) (v i) == layer_5_lane p i)

val lemma_ntt_layer_5_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 8 < 4 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+4) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+4) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t ulo l) /\
            v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t ulo l) /\
            (v (t ulo l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 8 < 4 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/8 + 4) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let layer_6_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 128 in
  let idx:usize = i %! mk_usize 128 in
  let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 2 <: usize ] <: i32) <: i64 in
  if idx <. mk_usize 64
  then
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i +! mk_usize 64 <: usize ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +! (cast (t <: i32) <: i64) <: i64)
  else
    let t:i32 =
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast (p.[ i ] <: i32) <: i64) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i -! mk_usize 64 <: usize ] <: i32) <: i64) -! (cast (t <: i32) <: i64) <: i64)

val lemma_ntt_layer_6_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer p (mk_usize 6)) (v i) == layer_6_lane p i)

val lemma_ntt_layer_6_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 16 < 8 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+8) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+8) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t ulo l) /\
            v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t ulo l) /\
            (v (t ulo l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 16 < 8 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/16 + 2) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

val lemma_ntt_layer_7_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (t: (ulo: nat{ulo < 32} -> l: nat{l < 8} -> i32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 32 < 16 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+16) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+16) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t ulo l) /\
            v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t ulo l) /\
            (v (t ulo l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 32 < 16 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (ulo/32 + 1) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

val lemma_v_zetas_eq_zeta (i: nat{1 <= i /\ i < 256})
    : Lemma (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize i ] <: i32) == Spec.MLDSA.NttConstants.zeta i)

let intt_layer_0_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 2 in
  let idx:usize = i %! mk_usize 2 in
  if idx <. mk_usize 1
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 1 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 255 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 1 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_0_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 0)) (v i) == intt_layer_0_lane p i)

val lemma_intt_layer_0_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (b: nat{b < 32} -> p: nat{p < 4} -> i32))
    : Lemma
        (requires
          (forall (b: nat{b < 32}) (p: nat{p < 4}). {:pattern (zm b p)}
           (let ci = Seq.index input b in
            let co = Seq.index transformed b in
            let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] in
            v (Seq.index co (2*p)) == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
            (v (Seq.index co (2*p+1))) % 8380417 ==
              ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zm b p) * 8265825) % 8380417 /\
            (v (zm b p)) % 8380417 == (v z * pow2 32) % 8380417)))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 ==
             (v (Seq.index spec i)) % 8380417))

let intt_layer_1_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 4 in
  let idx:usize = i %! mk_usize 4 in
  if idx <. mk_usize 2
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 2 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 127 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 2 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_1_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 1)) (v i) == intt_layer_1_lane p i)

val lemma_intt_layer_1_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (b: nat{b < 32} -> h: nat{h < 2} -> i32))
    : Lemma
        (requires
          (forall (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2}).
           (let ci = Seq.index input b in
            let co = Seq.index transformed b in
            v (Seq.index co (4*h+j)) == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
            (v (Seq.index co (4*h+j+2))) % 8380417 ==
              ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zm b h) * 8265825) % 8380417)) /\
          (forall (b: nat{b < 32}) (h: nat{h < 2}).
           (v (zm b h)) % 8380417 ==
           (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 ==
             (v (Seq.index spec i)) % 8380417))

let intt_layer_2_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 8 in
  let idx:usize = i %! mk_usize 8 in
  if idx <. mk_usize 4
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 4 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 63 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 4 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_2_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 2)) (v i) == intt_layer_2_lane p i)

val lemma_intt_layer_2_step_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (b: nat{b < 32} -> i32))
    : Lemma
        (requires
          (forall (b: nat{b < 32}) (p: nat{p < 4}).
           (let ci = Seq.index input b in
            let co = Seq.index transformed b in
            v (Seq.index co p) == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
            (v (Seq.index co (p+4))) % 8380417 ==
              ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v (zm b) * 8265825) % 8380417)) /\
          (forall (b: nat{b < 32}).
           (v (zm b)) % 8380417 ==
           (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 ==
             (v (Seq.index spec i)) % 8380417))

let intt_layer_3_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 16 in
  let idx:usize = i %! mk_usize 16 in
  if idx <. mk_usize 8
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 8 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 31 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 8 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_3_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 3)) (v i) == intt_layer_3_lane p i)

val lemma_intt_layer_3_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 2 == 0 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+1) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+1) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
            (v (Seq.index co_hi l)) % 8380417 ==
              ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 2 == 0 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (31 - ulo/2) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let intt_layer_4_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 32 in
  let idx:usize = i %! mk_usize 32 in
  if idx <. mk_usize 16
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 16 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 15 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 16 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_4_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 4)) (v i) == intt_layer_4_lane p i)

val lemma_intt_layer_4_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 4 < 2 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+2) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+2) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
            (v (Seq.index co_hi l)) % 8380417 ==
              ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 4 < 2 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (15 - ulo/4) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let intt_layer_5_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 64 in
  let idx:usize = i %! mk_usize 64 in
  if idx <. mk_usize 32
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 32 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 7 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 32 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_5_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 5)) (v i) == intt_layer_5_lane p i)

val lemma_intt_layer_5_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 8 < 4 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+4) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+4) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
            (v (Seq.index co_hi l)) % 8380417 ==
              ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 8 < 4 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (7 - ulo/8) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let intt_layer_6_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 128 in
  let idx:usize = i %! mk_usize 128 in
  if idx <. mk_usize 64
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 64 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 3 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 64 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_6_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 6)) (v i) == intt_layer_6_lane p i)

val lemma_intt_layer_6_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo % 16 < 8 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+8) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+8) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
            (v (Seq.index co_hi l)) % 8380417 ==
              ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo % 16 < 8 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (3 - ulo/16) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

let intt_layer_7_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256}) : i32 =
  let round:usize = i /! mk_usize 256 in
  let idx:usize = i %! mk_usize 256 in
  if idx <. mk_usize 128
  then
    Hacspec_ml_dsa.Arithmetic.mod_q ((cast (p.[ i ] <: i32) <: i64) +!
        (cast (p.[ i +! mk_usize 128 <: usize ] <: i32) <: i64)
        <:
        i64)
  else
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 1 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64)
    in
    Hacspec_ml_dsa.Arithmetic.mod_q (z *!
        ((cast (p.[ i -! mk_usize 128 <: usize ] <: i32) <: i64) -! (cast (p.[ i ] <: i32) <: i64)
          <:
          i64)
        <:
        i64)

val lemma_intt_layer_7_lane (p: t_Array i32 (mk_usize 256)) (i: usize{i <. mk_usize 256})
    : Lemma (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 7)) (v i) == intt_layer_7_lane p i)

val lemma_intt_layer_7_cross_to_hacspec_poly
    (input transformed: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    (zm: (ulo: nat{ulo < 32} -> i32))
    : Lemma
        (requires
          (forall (ulo: nat{ulo < 32}) (l: nat{l < 8}). ulo < 16 ==>
           (let ci_lo = Seq.index input ulo in let ci_hi = Seq.index input (ulo+16) in
            let co_lo = Seq.index transformed ulo in let co_hi = Seq.index transformed (ulo+16) in
            v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
            (v (Seq.index co_hi l)) % 8380417 ==
              ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm ulo) * 8265825) % 8380417)) /\
          (forall (ulo: nat{ulo < 32}). ulo < 16 ==>
            (v (zm ulo)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (1 - ulo/32) ] <: i32) * pow2 32) % 8380417))
        (ensures
          (let in_flat = simd_units_to_array input in
           let out_flat = simd_units_to_array transformed in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

val lemma_decompose_spec_eq_decompose (gamma2 r: i32)
    : Lemma
        (requires
          (v gamma2 == 95232 \/ v gamma2 == 261888) /\
          Spec.Utils.is_i32b 8380416 r)
        (ensures
          (let (r0_s_avx, r1_s_avx) = Spec.MLDSA.Math.decompose_spec gamma2 r in
           let (r0_int, r1_int, _) = Spec.MLDSA.Math.decompose (v gamma2) (v r) in
           v r0_s_avx == r0_int /\ v r1_s_avx == r1_int))

