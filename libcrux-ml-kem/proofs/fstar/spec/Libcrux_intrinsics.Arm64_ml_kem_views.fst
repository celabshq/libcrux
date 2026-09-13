module Libcrux_intrinsics.Arm64_ml_kem_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Libcrux_intrinsics.Arm64

(* ============================================================================
   ml-kem NEON lane-view + per-op fact companion.

   The ARM/NEON analog of `Libcrux_intrinsics.Avx2_ml_kem_views`.  It exposes to
   ml-kem's NEON proofs the lane VIEWS (`vec128_as_i16x8` / `get_lane_i16x8` /
   ...) and per-op FACT lemmas phrased over the REAL `Libcrux_intrinsics.Arm64`
   ops (which delegate to the differentially-tested `libcrux-core-models` NEON
   model).

   TRUST.  The Seq lane view is a per-index read of the canonical core-models
   FunArray codec (`Canon.to_i16x8` = `Int_vec_interp` width 128), and every
   op-fact is PROVEN from the canonical NEON op-lemma set in
   `Libcrux_core_models.Neon_views` (which rests only on the differentially
   tested `Arm.Interpretations.Int_vec.Lemmas` lifts + the PROVEN codec
   round-trip).  NO fact in this module is assumed.

   CONTENTS.  This file carries the i16x8 "per-lane-codec backbone" (8 op-facts):
   the structural i16x8 lane view + arithmetic (`vadd/vsub/vmul/vmul_n_s16`),
   transpose (`vtrn1q/vtrn2q_s16`), broadcast (`vdupq_n_s16`) and right-shift
   (`vshrq_n_s16`), which prove directly from the `Neon_views` codec op-lemmas
   (`ArmIV.OP` is a per-lane FunArray op, so `Seq.init`/`map2 f (view a) (view b)`
   matches by `Seq.lemma_eq_intro`).

   NOT YET COVERED, by tier — each needs a companion op-fact and, where
   noted, a FOUNDATION lemma in `Neon_views` (width-128 analog of an existing
   `Intrinsics_views` width-256 lemma):
     * Logical (vand/veor/vbic/veor3/vbcax): `ArmIV.OP` is a BIT-LEVEL
       `impl_9__from_fn 128`, so needs the codec-commute lemma
       `to_i16x8 (bitop a b) [i] == (view a)[i] `op` (view b)[i]` — the width-128
       analog of `Intrinsics_views.lemma_{and,xor}_i16x16_iv` (which rests on
       `lemma_from_fn_lane_reader`/`lemma_bv_index`, both hardcoded to 256; port
       to 128 in `Neon_views`).
     * Shifts (vshrq_n / vshlq_n family): per-lane shift codec facts (ArmIV shift body).
     * Clamp (vqdmulhq_s16 / _n_s16 / _n_s32): saturating-clamp per-lane facts.
     * trn: s16 (vtrn1q/vtrn2q_s16) DONE; s32/s64/u64 remaining — pure mirror
       (ArmIV.trn on codec), like the arithmetic ops; need the i32x4/i64x2/u64x2
       views added here.
     * Reinterpret same-width (s16<->u16, s32<->u32): `result == a` (bit-identity
       `ArmIV.OP a == a`) + per-lane cast_mod codec-signedness fact.
     * Reinterpret cross-width (s32<->s16, s64<->s16, u32<->u8, u16<->u8): the
       HARD tier — `result == a` + a codec-repack lemma relating the two views of
       the same 128 bits (`i16x2_as_i32` / `i64_i16lane` / ...).  These are pure
       codec facts (independent of any op); prove via composed `Canon.lemma_readback`.
     * mull/mlal/get_low/high/addv (i16x4<->i32x4 halves): various.
     * load/store/dup (vld1q / vst1q / vdupq_n family): `Arm.Extra` slice-model facts.
     * The bit bridge `bit_vec_of_int_t_array_vec128_as_i16x8_lemma` (used by
       from_bytes/to_bytes): mirror `Avx2_ml_kem_views.bit_vec_of_int_t_array_
       vec128_as_i16x8_lemma` (one `Canon.lemma_readback` call).
     * Helper lets (`i16_bits_as_u32`, `u32_lo16_as_i16`, `i16x2_as_i32`,
       `i64_i16lane`, `arm_sshl_i16`, ...) needed by the reinterpret facts +
       consumer proofs.

   Lives in `proofs/fstar/spec/` (hand-maintained, NOT the hax-extraction dir),
   so `cargo hax into` never clobbers it; on ml-kem's include path only.  It is
   NOT a make ROOT — it verifies only as a dependency of ml-kem's NEON consumers.
   ========================================================================== *)

module Funarr = Libcrux_core_models.Abstractions.Funarr
module BV     = Libcrux_core_models.Abstractions.Bitvec
module Canon  = Libcrux_core_models.Intrinsics_views
module NV     = Libcrux_core_models.Neon_views
module ArmIV  = Libcrux_core_models.Core_arch.Arm.Interpretations.Int_vec
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Int    = Rust_primitives.Integers
module Bit    = Libcrux_core_models.Abstractions.Bit

(* ── Lane-view types (the NEON `t_e_*` vector types, all 128-bit) ──────────── *)
unfold type t_e_int16x8_t  = BV.t_BitVec (mk_u64 128)
unfold type t_e_int32x4_t  = BV.t_BitVec (mk_u64 128)
unfold type t_e_uint32x4_t = BV.t_BitVec (mk_u64 128)
unfold type t_e_uint16x8_t = BV.t_BitVec (mk_u64 128)
unfold type t_e_uint8x16_t = BV.t_BitVec (mk_u64 128)
unfold type t_e_int64x2_t  = BV.t_BitVec (mk_u64 128)
unfold type t_e_uint64x2_t = BV.t_BitVec (mk_u64 128)
unfold type t_e_int16x4_t  = BV.t_BitVec (mk_u64 64)
unfold type t_e_uint16x4_t = BV.t_BitVec (mk_u64 64)

(* ── i16x8 lane view (adapter over canonical to_i16x8).  OPAQUE for the
      same reasons as x86's `vec256_as_i16x16`: keeps the view abstract (still
      PROVEN, not assumed); the ONLY route to the codec is `vec128_index`. ──── *)
[@@ "opaque_to_smt"]
let vec128_as_i16x8 (x: t_e_int16x8_t) : t_Array i16 (sz 8) =
  Seq.init 8 (fun i -> Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 i))
let get_lane_i16x8 (v: t_e_int16x8_t) (i: nat{i < 8}) : i16 = Seq.index (vec128_as_i16x8 v) i

let vec128_index (x: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_i16x8 x) i
           == Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_i16x8 x) i)]
  = reveal_opaque (`%vec128_as_i16x8) vec128_as_i16x8

let vec128_as_i16x8_len (x: t_e_int16x8_t)
  : Lemma (Seq.length (vec128_as_i16x8 x) == 8)
          [SMTPat (Seq.length (vec128_as_i16x8 x))]
  = ()

let vec128_as_i16x8_slice_ok (x: t_e_int16x8_t)
  : Lemma (Seq.length (vec128_as_i16x8 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_i16x8 x)]
  = assert_norm (8 <= Rust_primitives.Integers.max_usize)

(* ── i16x8 arithmetic op-facts (VALIDATED backbone) ───────────────────────────
   Each mirrors `Avx2_ml_kem_views.lemma_mm256_add_epi16`: the real op delegates
   to `Neon.OP` (transparent), `NV.lemma_OP` gives `to_i16x8 (Neon.OP a b) ==
   ArmIV.OP (to_i16x8 a) (to_i16x8 b)`, `ArmIV.OP` is a per-lane FunArray op, so
   `Seq.lemma_eq_intro` closes against `Spec.Utils.map2`. ────────────────────── *)

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vaddq_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vaddq_s16 a b)
           == Spec.Utils.map2 ( +. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
          [SMTPat (vec128_as_i16x8 (e_vaddq_s16 a b))] =
  NV.lemma_vaddq_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vaddq_s16 a b))
                     (Spec.Utils.map2 ( +. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vsubq_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vsubq_s16 a b)
           == Spec.Utils.map2 ( -. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
          [SMTPat (vec128_as_i16x8 (e_vsubq_s16 a b))] =
  NV.lemma_vsubq_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vsubq_s16 a b))
                     (Spec.Utils.map2 ( -. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vmulq_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vmulq_s16 a b)
           == Spec.Utils.map2 mul_mod (vec128_as_i16x8 a) (vec128_as_i16x8 b))
          [SMTPat (vec128_as_i16x8 (e_vmulq_s16 a b))] =
  NV.lemma_vmulq_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vmulq_s16 a b))
                     (Spec.Utils.map2 mul_mod (vec128_as_i16x8 a) (vec128_as_i16x8 b))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vmulq_n_s16 (v: t_e_int16x8_t) (c: i16)
  : Lemma (vec128_as_i16x8 (e_vmulq_n_s16 v c)
           == Seq.init 8 (fun i -> Seq.index (vec128_as_i16x8 v) i *. c))
          [SMTPat (vec128_as_i16x8 (e_vmulq_n_s16 v c))] =
  NV.lemma_vmulq_n_s16 v c;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vmulq_n_s16 v c))
                     (Seq.init 8 (fun i -> Seq.index (vec128_as_i16x8 v) i *. c))
#pop-options

(* ── i16x8 transpose op-facts (mirror pattern; used by the NEON NTT) ────────── *)
(* ArmIV.vtrn1q_s16 = from_fn (if i even then a[i] else b[i-1]) (interleave lows);
   vtrn2q_s16 = from_fn (if i even then a[i+1] else b[i]) (interleave highs). *)

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn1q_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vtrn1q_s16 a b)
           == Seq.init 8 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i16x8 a) i
                                              else Seq.index (vec128_as_i16x8 b) (i - 1)))
          [SMTPat (vec128_as_i16x8 (e_vtrn1q_s16 a b))] =
  NV.lemma_vtrn1q_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vtrn1q_s16 a b))
                     (Seq.init 8 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i16x8 a) i
                                                    else Seq.index (vec128_as_i16x8 b) (i - 1)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn2q_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vtrn2q_s16 a b)
           == Seq.init 8 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i16x8 a) (i + 1)
                                              else Seq.index (vec128_as_i16x8 b) i))
          [SMTPat (vec128_as_i16x8 (e_vtrn2q_s16 a b))] =
  NV.lemma_vtrn2q_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vtrn2q_s16 a b))
                     (Seq.init 8 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i16x8 a) (i + 1)
                                                    else Seq.index (vec128_as_i16x8 b) i))
#pop-options

(* ── i16x8 broadcast + right-shift op-facts (mirror pattern) ────────────────── *)

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vdupq_n_s16 (c: i16)
  : Lemma (vec128_as_i16x8 (e_vdupq_n_s16 c) == Seq.create 8 c)
          [SMTPat (vec128_as_i16x8 (e_vdupq_n_s16 c))] =
  NV.lemma_vdupq_n_s16 c;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vdupq_n_s16 c)) (Seq.create 8 c)
#pop-options

(* Full ArmIV mirror (all shift ranges); consumers with 0<=SHIFT<16 reduce to
   the `>>! SHIFT` branch. *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vshrq_n_s16 (v_SHIFT_BY: i32) (v: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vshrq_n_s16 v_SHIFT_BY v)
           == Seq.init 8 (fun i ->
                let x = Seq.index (vec128_as_i16x8 v) i in
                if v_SHIFT_BY >=. mk_i32 16 then (if x <. mk_i16 0 then mk_i16 (-1) else mk_i16 0)
                else if v_SHIFT_BY <=. mk_i32 0 then x
                else x >>! v_SHIFT_BY))
          [SMTPat (vec128_as_i16x8 (e_vshrq_n_s16 v_SHIFT_BY v))] =
  NV.lemma_vshrq_n_s16 v_SHIFT_BY v;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vshrq_n_s16 v_SHIFT_BY v))
                     (Seq.init 8 (fun i ->
                        let x = Seq.index (vec128_as_i16x8 v) i in
                        if v_SHIFT_BY >=. mk_i32 16 then (if x <. mk_i16 0 then mk_i16 (-1) else mk_i16 0)
                        else if v_SHIFT_BY <=. mk_i32 0 then x
                        else x >>! v_SHIFT_BY))
#pop-options

(* ============================================================================
   TIER A — other-width lane views + Shape-A per-lane-codec op-facts.

   Same validated recipe as the i16x8 backbone: the real `Arm64.e_vOP`
   delegates to `Neon.OP` (transparent), the canonical `NV.lemma_vOP` gives the
   VIEW-level `to_WxL (Neon.OP ...) == ArmIV.OP (to_WxL a) ...`, `ArmIV.OP` is a
   per-lane FunArray op, so `Seq.lemma_eq_intro` closes against `Spec.Utils.map2`
   / `Seq.init` / `Seq.create`.  Signed views read `Canon.to_iWxL`; unsigned read
   `NV.to_uWxL` (the exact codec each `NV.lemma_*` is stated over).
   ========================================================================== *)

(* ── i32x4 lane view ──────────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec128_as_i32x4 (x: t_e_int32x4_t) : t_Array i32 (sz 4) =
  Seq.init 4 (fun i -> Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 i))
let get_lane_i32x4 (v: t_e_int32x4_t) (i: nat{i < 4}) : i32 = Seq.index (vec128_as_i32x4 v) i

let vec128_index_i32x4 (x: t_e_int32x4_t) (i: nat{i < 4})
  : Lemma (Seq.index (vec128_as_i32x4 x) i
           == Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_i32x4 x) i)]
  = reveal_opaque (`%vec128_as_i32x4) vec128_as_i32x4

let vec128_as_i32x4_len (x: t_e_int32x4_t)
  : Lemma (Seq.length (vec128_as_i32x4 x) == 4)
          [SMTPat (Seq.length (vec128_as_i32x4 x))]
  = ()

let vec128_as_i32x4_slice_ok (x: t_e_int32x4_t)
  : Lemma (Seq.length (vec128_as_i32x4 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_i32x4 x)]
  = assert_norm (4 <= Rust_primitives.Integers.max_usize)

(* ── i64x2 lane view ──────────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec128_as_i64x2 (x: t_e_int64x2_t) : t_Array i64 (sz 2) =
  Seq.init 2 (fun i -> Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 x) (mk_u64 i))
let get_lane_i64x2 (v: t_e_int64x2_t) (i: nat{i < 2}) : i64 = Seq.index (vec128_as_i64x2 v) i

let vec128_index_i64x2 (x: t_e_int64x2_t) (i: nat{i < 2})
  : Lemma (Seq.index (vec128_as_i64x2 x) i
           == Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_i64x2 x) i)]
  = reveal_opaque (`%vec128_as_i64x2) vec128_as_i64x2

let vec128_as_i64x2_len (x: t_e_int64x2_t)
  : Lemma (Seq.length (vec128_as_i64x2 x) == 2)
          [SMTPat (Seq.length (vec128_as_i64x2 x))]
  = ()

let vec128_as_i64x2_slice_ok (x: t_e_int64x2_t)
  : Lemma (Seq.length (vec128_as_i64x2 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_i64x2 x)]
  = assert_norm (2 <= Rust_primitives.Integers.max_usize)

(* ── u32x4 lane view ──────────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec128_as_u32x4 (x: t_e_uint32x4_t) : t_Array u32 (sz 4) =
  Seq.init 4 (fun i -> Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 x) (mk_u64 i))
let get_lane_u32x4 (v: t_e_uint32x4_t) (i: nat{i < 4}) : u32 = Seq.index (vec128_as_u32x4 v) i

let vec128_index_u32x4 (x: t_e_uint32x4_t) (i: nat{i < 4})
  : Lemma (Seq.index (vec128_as_u32x4 x) i
           == Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_u32x4 x) i)]
  = reveal_opaque (`%vec128_as_u32x4) vec128_as_u32x4

let vec128_as_u32x4_len (x: t_e_uint32x4_t)
  : Lemma (Seq.length (vec128_as_u32x4 x) == 4)
          [SMTPat (Seq.length (vec128_as_u32x4 x))]
  = ()

let vec128_as_u32x4_slice_ok (x: t_e_uint32x4_t)
  : Lemma (Seq.length (vec128_as_u32x4 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_u32x4 x)]
  = assert_norm (4 <= Rust_primitives.Integers.max_usize)

(* ── u16x8 lane view ──────────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec128_as_u16x8 (x: t_e_uint16x8_t) : t_Array u16 (sz 8) =
  Seq.init 8 (fun i -> Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 x) (mk_u64 i))
let get_lane_u16x8 (v: t_e_uint16x8_t) (i: nat{i < 8}) : u16 = Seq.index (vec128_as_u16x8 v) i

let vec128_index_u16x8 (x: t_e_uint16x8_t) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_u16x8 x) i
           == Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_u16x8 x) i)]
  = reveal_opaque (`%vec128_as_u16x8) vec128_as_u16x8

let vec128_as_u16x8_len (x: t_e_uint16x8_t)
  : Lemma (Seq.length (vec128_as_u16x8 x) == 8)
          [SMTPat (Seq.length (vec128_as_u16x8 x))]
  = ()

let vec128_as_u16x8_slice_ok (x: t_e_uint16x8_t)
  : Lemma (Seq.length (vec128_as_u16x8 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_u16x8 x)]
  = assert_norm (8 <= Rust_primitives.Integers.max_usize)

(* ── i32x4 transpose (used by the NEON NTT butterfly interleave) ───────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn1q_s32 (a b: t_e_int32x4_t)
  : Lemma (vec128_as_i32x4 (e_vtrn1q_s32 a b)
           == Seq.init 4 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i32x4 a) i
                                              else Seq.index (vec128_as_i32x4 b) (i - 1)))
          [SMTPat (vec128_as_i32x4 (e_vtrn1q_s32 a b))] =
  NV.lemma_vtrn1q_s32 a b;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vtrn1q_s32 a b))
                     (Seq.init 4 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i32x4 a) i
                                                    else Seq.index (vec128_as_i32x4 b) (i - 1)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn2q_s32 (a b: t_e_int32x4_t)
  : Lemma (vec128_as_i32x4 (e_vtrn2q_s32 a b)
           == Seq.init 4 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i32x4 a) (i + 1)
                                              else Seq.index (vec128_as_i32x4 b) i))
          [SMTPat (vec128_as_i32x4 (e_vtrn2q_s32 a b))] =
  NV.lemma_vtrn2q_s32 a b;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vtrn2q_s32 a b))
                     (Seq.init 4 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i32x4 a) (i + 1)
                                                    else Seq.index (vec128_as_i32x4 b) i))
#pop-options

(* ── i64x2 transpose ──────────────────────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn1q_s64 (a b: t_e_int64x2_t)
  : Lemma (vec128_as_i64x2 (e_vtrn1q_s64 a b)
           == Seq.init 2 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i64x2 a) i
                                              else Seq.index (vec128_as_i64x2 b) (i - 1)))
          [SMTPat (vec128_as_i64x2 (e_vtrn1q_s64 a b))] =
  NV.lemma_vtrn1q_s64 a b;
  Seq.lemma_eq_intro (vec128_as_i64x2 (e_vtrn1q_s64 a b))
                     (Seq.init 2 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i64x2 a) i
                                                    else Seq.index (vec128_as_i64x2 b) (i - 1)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn2q_s64 (a b: t_e_int64x2_t)
  : Lemma (vec128_as_i64x2 (e_vtrn2q_s64 a b)
           == Seq.init 2 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i64x2 a) (i + 1)
                                              else Seq.index (vec128_as_i64x2 b) i))
          [SMTPat (vec128_as_i64x2 (e_vtrn2q_s64 a b))] =
  NV.lemma_vtrn2q_s64 a b;
  Seq.lemma_eq_intro (vec128_as_i64x2 (e_vtrn2q_s64 a b))
                     (Seq.init 2 (fun i -> if i % 2 = 0 then Seq.index (vec128_as_i64x2 a) (i + 1)
                                                    else Seq.index (vec128_as_i64x2 b) i))
#pop-options

(* ── u32x4 arithmetic / shifts / broadcast ────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vaddq_u32 (a b: t_e_uint32x4_t)
  : Lemma (vec128_as_u32x4 (e_vaddq_u32 a b)
           == Spec.Utils.map2 ( +. ) (vec128_as_u32x4 a) (vec128_as_u32x4 b))
          [SMTPat (vec128_as_u32x4 (e_vaddq_u32 a b))] =
  NV.lemma_vaddq_u32 a b;
  Seq.lemma_eq_intro (vec128_as_u32x4 (e_vaddq_u32 a b))
                     (Spec.Utils.map2 ( +. ) (vec128_as_u32x4 a) (vec128_as_u32x4 b))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vmulq_n_u32 (a: t_e_uint32x4_t) (c: u32)
  : Lemma (vec128_as_u32x4 (e_vmulq_n_u32 a c)
           == Seq.init 4 (fun i -> Seq.index (vec128_as_u32x4 a) i *. c))
          [SMTPat (vec128_as_u32x4 (e_vmulq_n_u32 a c))] =
  NV.lemma_vmulq_n_u32 a c;
  Seq.lemma_eq_intro (vec128_as_u32x4 (e_vmulq_n_u32 a c))
                     (Seq.init 4 (fun i -> Seq.index (vec128_as_u32x4 a) i *. c))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vdupq_n_u32 (c: u32)
  : Lemma (vec128_as_u32x4 (e_vdupq_n_u32 c) == Seq.create 4 c)
          [SMTPat (vec128_as_u32x4 (e_vdupq_n_u32 c))] =
  NV.lemma_vdupq_n_u32 c;
  Seq.lemma_eq_intro (vec128_as_u32x4 (e_vdupq_n_u32 c)) (Seq.create 4 c)
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vshrq_n_u32 (v_N: i32) (a: t_e_uint32x4_t)
  : Lemma (vec128_as_u32x4 (e_vshrq_n_u32 v_N a)
           == Seq.init 4 (fun i ->
                let x = Seq.index (vec128_as_u32x4 a) i in
                if v_N >=. mk_i32 32 then mk_u32 0
                else if v_N <=. mk_i32 0 then x
                else x >>! (cast v_N <: u32)))
          [SMTPat (vec128_as_u32x4 (e_vshrq_n_u32 v_N a))] =
  NV.lemma_vshrq_n_u32 v_N a;
  Seq.lemma_eq_intro (vec128_as_u32x4 (e_vshrq_n_u32 v_N a))
                     (Seq.init 4 (fun i ->
                        let x = Seq.index (vec128_as_u32x4 a) i in
                        if v_N >=. mk_i32 32 then mk_u32 0
                        else if v_N <=. mk_i32 0 then x
                        else x >>! (cast v_N <: u32)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vshlq_n_u32 (v_SHIFT_BY: i32) (a: t_e_uint32x4_t)
  : Lemma (vec128_as_u32x4 (e_vshlq_n_u32 v_SHIFT_BY a)
           == Seq.init 4 (fun i ->
                let x = Seq.index (vec128_as_u32x4 a) i in
                if v_SHIFT_BY >=. mk_i32 32 || v_SHIFT_BY <. mk_i32 0 then mk_u32 0
                else x <<! (cast v_SHIFT_BY <: u32)))
          [SMTPat (vec128_as_u32x4 (e_vshlq_n_u32 v_SHIFT_BY a))] =
  NV.lemma_vshlq_n_u32 v_SHIFT_BY a;
  Seq.lemma_eq_intro (vec128_as_u32x4 (e_vshlq_n_u32 v_SHIFT_BY a))
                     (Seq.init 4 (fun i ->
                        let x = Seq.index (vec128_as_u32x4 a) i in
                        if v_SHIFT_BY >=. mk_i32 32 || v_SHIFT_BY <. mk_i32 0 then mk_u32 0
                        else x <<! (cast v_SHIFT_BY <: u32)))
#pop-options

(* ── u16x8 arithmetic / shift / broadcast ─────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vmulq_n_u16 (a: t_e_uint16x8_t) (c: u16)
  : Lemma (vec128_as_u16x8 (e_vmulq_n_u16 a c)
           == Seq.init 8 (fun i -> Seq.index (vec128_as_u16x8 a) i *. c))
          [SMTPat (vec128_as_u16x8 (e_vmulq_n_u16 a c))] =
  NV.lemma_vmulq_n_u16 a c;
  Seq.lemma_eq_intro (vec128_as_u16x8 (e_vmulq_n_u16 a c))
                     (Seq.init 8 (fun i -> Seq.index (vec128_as_u16x8 a) i *. c))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vdupq_n_u16 (c: u16)
  : Lemma (vec128_as_u16x8 (e_vdupq_n_u16 c) == Seq.create 8 c)
          [SMTPat (vec128_as_u16x8 (e_vdupq_n_u16 c))] =
  NV.lemma_vdupq_n_u16 c;
  Seq.lemma_eq_intro (vec128_as_u16x8 (e_vdupq_n_u16 c)) (Seq.create 8 c)
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vshrq_n_u16 (v_SHIFT_BY: i32) (v: t_e_uint16x8_t)
  : Lemma (vec128_as_u16x8 (e_vshrq_n_u16 v_SHIFT_BY v)
           == Seq.init 8 (fun i ->
                let x = Seq.index (vec128_as_u16x8 v) i in
                if v_SHIFT_BY >=. mk_i32 16 then mk_u16 0
                else if v_SHIFT_BY <=. mk_i32 0 then x
                else x >>! v_SHIFT_BY))
          [SMTPat (vec128_as_u16x8 (e_vshrq_n_u16 v_SHIFT_BY v))] =
  NV.lemma_vshrq_n_u16 v_SHIFT_BY v;
  Seq.lemma_eq_intro (vec128_as_u16x8 (e_vshrq_n_u16 v_SHIFT_BY v))
                     (Seq.init 8 (fun i ->
                        let x = Seq.index (vec128_as_u16x8 v) i in
                        if v_SHIFT_BY >=. mk_i32 16 then mk_u16 0
                        else if v_SHIFT_BY <=. mk_i32 0 then x
                        else x >>! v_SHIFT_BY))
#pop-options

(* ── ARM variable-shift helper lets (referenced by the vshlq_s16/u16 op-facts
      and by consumer proofs). ─────────────────────────────────────────────── *)
let arm_sshl_i16 (a b: i16) : i16 =
  let s = v (b %! mk_i16 256) in
  if s < 128 then (if s < 16 then a <<! mk_i32 s else mk_i16 0)
  else (let r = 256 - s in
        if r < 16 then a >>! mk_i32 r
        else (if a <. mk_i16 0 then mk_i16 (-1) else mk_i16 0))

let arm_ushl_u16 (a: u16) (b: i16) : u16 =
  let s = v (b %! mk_i16 256) in
  if s < 128 then (if s < 16 then a <<! mk_i32 s else mk_u16 0)
  else (let r = 256 - s in
        if r < 16 then a >>! mk_i32 r else mk_u16 0)

(* ── i16x8 saturating doubling multiply-high (vqdmulh) ────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vqdmulhq_n_s16 (k: t_e_int16x8_t) (b: i16)
  : Lemma (vec128_as_i16x8 (e_vqdmulhq_n_s16 k b)
           == Seq.init 8 (fun i ->
                let prod = ((cast (Seq.index (vec128_as_i16x8 k) i) <: i32) *. (cast b <: i32)) >>! (mk_i32 15) in
                if prod >. mk_i32 32767 then mk_i16 32767
                else if prod <. mk_i32 (- 32768) then mk_i16 (- 32768) else (cast prod <: i16)))
          [SMTPat (vec128_as_i16x8 (e_vqdmulhq_n_s16 k b))] =
  NV.lemma_vqdmulhq_n_s16 k b;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vqdmulhq_n_s16 k b))
                     (Seq.init 8 (fun i ->
                        let prod = ((cast (Seq.index (vec128_as_i16x8 k) i) <: i32) *. (cast b <: i32)) >>! (mk_i32 15) in
                        if prod >. mk_i32 32767 then mk_i16 32767
                        else if prod <. mk_i32 (- 32768) then mk_i16 (- 32768) else (cast prod <: i16)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vqdmulhq_s16 (a c: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vqdmulhq_s16 a c)
           == Seq.init 8 (fun i ->
                let prod = ((cast (Seq.index (vec128_as_i16x8 a) i) <: i32)
                            *. (cast (Seq.index (vec128_as_i16x8 c) i) <: i32)) >>! (mk_i32 15) in
                if prod >. mk_i32 32767 then mk_i16 32767
                else if prod <. mk_i32 (- 32768) then mk_i16 (- 32768) else (cast prod <: i16)))
          [SMTPat (vec128_as_i16x8 (e_vqdmulhq_s16 a c))] =
  NV.lemma_vqdmulhq_s16 a c;
  Seq.lemma_eq_intro (vec128_as_i16x8 (e_vqdmulhq_s16 a c))
                     (Seq.init 8 (fun i ->
                        let prod = ((cast (Seq.index (vec128_as_i16x8 a) i) <: i32)
                                    *. (cast (Seq.index (vec128_as_i16x8 c) i) <: i32)) >>! (mk_i32 15) in
                        if prod >. mk_i32 32767 then mk_i16 32767
                        else if prod <. mk_i32 (- 32768) then mk_i16 (- 32768) else (cast prod <: i16)))
#pop-options

(* ── i16x8 comparison -> u16x8 mask ───────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vcgeq_s16 (v c: t_e_int16x8_t)
  : Lemma (vec128_as_u16x8 (e_vcgeq_s16 v c)
           == Seq.init 8 (fun i ->
                if Seq.index (vec128_as_i16x8 v) i >=. Seq.index (vec128_as_i16x8 c) i
                then mk_u16 0xFFFF else mk_u16 0))
          [SMTPat (vec128_as_u16x8 (e_vcgeq_s16 v c))] =
  NV.lemma_vcgeq_s16 v c;
  Seq.lemma_eq_intro (vec128_as_u16x8 (e_vcgeq_s16 v c))
                     (Seq.init 8 (fun i ->
                        if Seq.index (vec128_as_i16x8 v) i >=. Seq.index (vec128_as_i16x8 c) i
                        then mk_u16 0xFFFF else mk_u16 0))
#pop-options

(* vshlq_s16 / vshlq_u16 op-facts deferred: `arm_sshl_i16` / `arm_ushl_u16`
   (the `v (b %! 256)` split-at-128 encoding) are only PROVABLY equal to the
   core-models `ArmIV.vshlq_s16` lane body (sign-extended low-byte + data-
   dependent shift) via a dedicated per-lane byte/shift bridge lemma; see the
   `lemma_arm_sshl_eq` work below. *)

(* ── 64-bit half-vector lane views (i16x4 / u16x4) ────────────────────────── *)
[@@ "opaque_to_smt"]
let vec64_as_i16x4 (x: t_e_int16x4_t) : t_Array i16 (sz 4) =
  Seq.init 4 (fun i -> Funarr.impl_5__get (mk_u64 4) #i16 (NV.to_i16x4 x) (mk_u64 i))
let get_lane_i16x4 (v: t_e_int16x4_t) (i: nat{i < 4}) : i16 = Seq.index (vec64_as_i16x4 v) i

let vec64_index_i16x4 (x: t_e_int16x4_t) (i: nat{i < 4})
  : Lemma (Seq.index (vec64_as_i16x4 x) i
           == Funarr.impl_5__get (mk_u64 4) #i16 (NV.to_i16x4 x) (mk_u64 i))
          [SMTPat (Seq.index (vec64_as_i16x4 x) i)]
  = reveal_opaque (`%vec64_as_i16x4) vec64_as_i16x4

let vec64_as_i16x4_len (x: t_e_int16x4_t)
  : Lemma (Seq.length (vec64_as_i16x4 x) == 4)
          [SMTPat (Seq.length (vec64_as_i16x4 x))]
  = ()

let vec64_as_i16x4_slice_ok (x: t_e_int16x4_t)
  : Lemma (Seq.length (vec64_as_i16x4 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec64_as_i16x4 x)]
  = assert_norm (4 <= Rust_primitives.Integers.max_usize)

[@@ "opaque_to_smt"]
let vec64_as_u16x4 (x: t_e_uint16x4_t) : t_Array u16 (sz 4) =
  Seq.init 4 (fun i -> Funarr.impl_5__get (mk_u64 4) #u16 (NV.to_u16x4 x) (mk_u64 i))
let get_lane_u16x4 (v: t_e_uint16x4_t) (i: nat{i < 4}) : u16 = Seq.index (vec64_as_u16x4 v) i

let vec64_index_u16x4 (x: t_e_uint16x4_t) (i: nat{i < 4})
  : Lemma (Seq.index (vec64_as_u16x4 x) i
           == Funarr.impl_5__get (mk_u64 4) #u16 (NV.to_u16x4 x) (mk_u64 i))
          [SMTPat (Seq.index (vec64_as_u16x4 x) i)]
  = reveal_opaque (`%vec64_as_u16x4) vec64_as_u16x4

let vec64_as_u16x4_len (x: t_e_uint16x4_t)
  : Lemma (Seq.length (vec64_as_u16x4 x) == 4)
          [SMTPat (Seq.length (vec64_as_u16x4 x))]
  = ()

let vec64_as_u16x4_slice_ok (x: t_e_uint16x4_t)
  : Lemma (Seq.length (vec64_as_u16x4 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec64_as_u16x4 x)]
  = assert_norm (4 <= Rust_primitives.Integers.max_usize)

(* ── get low / high half (128 -> 64) ──────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vget_low_s16 (a: t_e_int16x8_t)
  : Lemma (vec64_as_i16x4 (e_vget_low_s16 a)
           == Seq.init 4 (fun i -> Seq.index (vec128_as_i16x8 a) i))
          [SMTPat (vec64_as_i16x4 (e_vget_low_s16 a))] =
  NV.lemma_vget_low_s16 a;
  Seq.lemma_eq_intro (vec64_as_i16x4 (e_vget_low_s16 a))
                     (Seq.init 4 (fun i -> Seq.index (vec128_as_i16x8 a) i))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vget_low_u16 (a: t_e_uint16x8_t)
  : Lemma (vec64_as_u16x4 (e_vget_low_u16 a)
           == Seq.init 4 (fun i -> Seq.index (vec128_as_u16x8 a) i))
          [SMTPat (vec64_as_u16x4 (e_vget_low_u16 a))] =
  NV.lemma_vget_low_u16 a;
  Seq.lemma_eq_intro (vec64_as_u16x4 (e_vget_low_u16 a))
                     (Seq.init 4 (fun i -> Seq.index (vec128_as_u16x8 a) i))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vget_high_u16 (a: t_e_uint16x8_t)
  : Lemma (vec64_as_u16x4 (e_vget_high_u16 a)
           == Seq.init 4 (fun i -> Seq.index (vec128_as_u16x8 a) (i + 4)))
          [SMTPat (vec64_as_u16x4 (e_vget_high_u16 a))] =
  NV.lemma_vget_high_u16 a;
  Seq.lemma_eq_intro (vec64_as_u16x4 (e_vget_high_u16 a))
                     (Seq.init 4 (fun i -> Seq.index (vec128_as_u16x8 a) (i + 4)))
#pop-options

(* ── widening multiply (i16x4 / high halves -> i32x4) ─────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vmull_s16 (a b: t_e_int16x4_t)
  : Lemma (vec128_as_i32x4 (e_vmull_s16 a b)
           == Seq.init 4 (fun i -> (cast (Seq.index (vec64_as_i16x4 a) i) <: i32)
                                *. (cast (Seq.index (vec64_as_i16x4 b) i) <: i32)))
          [SMTPat (vec128_as_i32x4 (e_vmull_s16 a b))] =
  NV.lemma_vmull_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vmull_s16 a b))
                     (Seq.init 4 (fun i -> (cast (Seq.index (vec64_as_i16x4 a) i) <: i32)
                                        *. (cast (Seq.index (vec64_as_i16x4 b) i) <: i32)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vmull_high_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i32x4 (e_vmull_high_s16 a b)
           == Seq.init 4 (fun i -> (cast (Seq.index (vec128_as_i16x8 a) (i + 4)) <: i32)
                                *. (cast (Seq.index (vec128_as_i16x8 b) (i + 4)) <: i32)))
          [SMTPat (vec128_as_i32x4 (e_vmull_high_s16 a b))] =
  NV.lemma_vmull_high_s16 a b;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vmull_high_s16 a b))
                     (Seq.init 4 (fun i -> (cast (Seq.index (vec128_as_i16x8 a) (i + 4)) <: i32)
                                        *. (cast (Seq.index (vec128_as_i16x8 b) (i + 4)) <: i32)))
#pop-options

(* vaddvq_s16 / vaddv_u16 horizontal-reduction op-facts deferred: the core-models
   `ArmIV.vaddvq_s16` is a LEFT fold_range (wrapping_add accumulate), while the
   consumer form is the BALANCED sum tree `((a0+a1)+(a2+a3))+((a4+a5)+(a6+a7))`.
   Equal by i16 add_mod AC + fold unfolding, but a naive `fuel 9` unroll SATURATES
   (>75 s, killed).  Needs a dedicated reduction bridge lemma that unrolls the fold
   step-by-step and applies add_mod associativity as discrete rewrites. *)

(* ── u8x16 lane view ──────────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec128_as_u8x16 (x: t_e_uint8x16_t) : t_Array u8 (sz 16) =
  Seq.init 16 (fun i -> Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 i))
let get_lane_u8x16 (v: t_e_uint8x16_t) (i: nat{i < 16}) : u8 = Seq.index (vec128_as_u8x16 v) i

let vec128_index_u8x16 (x: t_e_uint8x16_t) (i: nat{i < 16})
  : Lemma (Seq.index (vec128_as_u8x16 x) i
           == Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_u8x16 x) i)]
  = reveal_opaque (`%vec128_as_u8x16) vec128_as_u8x16

let vec128_as_u8x16_len (x: t_e_uint8x16_t)
  : Lemma (Seq.length (vec128_as_u8x16 x) == 16)
          [SMTPat (Seq.length (vec128_as_u8x16 x))]
  = ()

let vec128_as_u8x16_slice_ok (x: t_e_uint8x16_t)
  : Lemma (Seq.length (vec128_as_u8x16 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_u8x16 x)]
  = assert_norm (16 <= Rust_primitives.Integers.max_usize)

(* ── table lookup (vqtbl1q_u8): data-dependent per-byte select ────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let lemma_e_vqtbl1q_u8 (t idx: t_e_uint8x16_t)
  : Lemma (vec128_as_u8x16 (e_vqtbl1q_u8 t idx)
           == Seq.init 16 (fun i ->
                let ix = v (Seq.index (vec128_as_u8x16 idx) i) in
                if ix < 16 then Seq.index (vec128_as_u8x16 t) ix else mk_u8 0))
          [SMTPat (vec128_as_u8x16 (e_vqtbl1q_u8 t idx))] =
  NV.lemma_vqtbl1q_u8 t idx;
  Seq.lemma_eq_intro (vec128_as_u8x16 (e_vqtbl1q_u8 t idx))
                     (Seq.init 16 (fun i ->
                        let ix = v (Seq.index (vec128_as_u8x16 idx) i) in
                        if ix < 16 then Seq.index (vec128_as_u8x16 t) ix else mk_u8 0))
#pop-options

(* ============================================================================
   TIER C/D (part 1) — reinterpret BIT-IDENTITY.  Every NEON reinterpret is a
   pure bit relabel: the real op delegates to `Neon.OP`, `NV.lemma_vreinterpretq_*`
   gives `Neon.OP a == ArmIV.OP a`, and `ArmIV.vreinterpretq_* a == a`
   (definitional `let OP a = a`).  So `result == a` for every reinterpret.  The
   per-lane cross-/same-width repack (i16x2_as_i32 / i32_lo16_as_i16 / cast_mod /
   ...) is a SEPARATE codec fact added on top (Tier C/D part 2) — this identity
   is what lets a consumer carry a value across a reinterpret unchanged.
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_e_vreinterpretq_s16_u16 (m0: t_e_uint16x8_t)
  : Lemma (e_vreinterpretq_s16_u16 m0 == m0) [SMTPat (e_vreinterpretq_s16_u16 m0)] =
  NV.lemma_vreinterpretq_s16_u16 m0
let lemma_e_vreinterpretq_u16_s16 (m0: t_e_int16x8_t)
  : Lemma (e_vreinterpretq_u16_s16 m0 == m0) [SMTPat (e_vreinterpretq_u16_s16 m0)] =
  NV.lemma_vreinterpretq_u16_s16 m0
let lemma_e_vreinterpretq_s32_u32 (a: t_e_uint32x4_t)
  : Lemma (e_vreinterpretq_s32_u32 a == a) [SMTPat (e_vreinterpretq_s32_u32 a)] =
  NV.lemma_vreinterpretq_s32_u32 a
let lemma_e_vreinterpretq_u32_s32 (a: t_e_int32x4_t)
  : Lemma (e_vreinterpretq_u32_s32 a == a) [SMTPat (e_vreinterpretq_u32_s32 a)] =
  NV.lemma_vreinterpretq_u32_s32 a
let lemma_e_vreinterpretq_s16_s32 (a: t_e_int32x4_t)
  : Lemma (e_vreinterpretq_s16_s32 a == a) [SMTPat (e_vreinterpretq_s16_s32 a)] =
  NV.lemma_vreinterpretq_s16_s32 a
let lemma_e_vreinterpretq_s32_s16 (a: t_e_int16x8_t)
  : Lemma (e_vreinterpretq_s32_s16 a == a) [SMTPat (e_vreinterpretq_s32_s16 a)] =
  NV.lemma_vreinterpretq_s32_s16 a
let lemma_e_vreinterpretq_s16_s64 (a: t_e_int64x2_t)
  : Lemma (e_vreinterpretq_s16_s64 a == a) [SMTPat (e_vreinterpretq_s16_s64 a)] =
  NV.lemma_vreinterpretq_s16_s64 a
let lemma_e_vreinterpretq_s64_s16 (a: t_e_int16x8_t)
  : Lemma (e_vreinterpretq_s64_s16 a == a) [SMTPat (e_vreinterpretq_s64_s16 a)] =
  NV.lemma_vreinterpretq_s64_s16 a
let lemma_e_vreinterpretq_s64_s32 (a: t_e_int32x4_t)
  : Lemma (e_vreinterpretq_s64_s32 a == a) [SMTPat (e_vreinterpretq_s64_s32 a)] =
  NV.lemma_vreinterpretq_s64_s32 a
let lemma_e_vreinterpretq_u32_s16 (a: t_e_int16x8_t)
  : Lemma (e_vreinterpretq_u32_s16 a == a) [SMTPat (e_vreinterpretq_u32_s16 a)] =
  NV.lemma_vreinterpretq_u32_s16 a
let lemma_e_vreinterpretq_s16_u32 (a: t_e_uint32x4_t)
  : Lemma (e_vreinterpretq_s16_u32 a == a) [SMTPat (e_vreinterpretq_s16_u32 a)] =
  NV.lemma_vreinterpretq_s16_u32 a
let lemma_e_vreinterpretq_u8_s16 (a: t_e_int16x8_t)
  : Lemma (e_vreinterpretq_u8_s16 a == a) [SMTPat (e_vreinterpretq_u8_s16 a)] =
  NV.lemma_vreinterpretq_u8_s16 a
let lemma_e_vreinterpretq_s16_u8 (a: t_e_uint8x16_t)
  : Lemma (e_vreinterpretq_s16_u8 a == a) [SMTPat (e_vreinterpretq_s16_u8 a)] =
  NV.lemma_vreinterpretq_s16_u8 a
let lemma_e_vreinterpretq_u16_u8 (a: t_e_uint8x16_t)
  : Lemma (e_vreinterpretq_u16_u8 a == a) [SMTPat (e_vreinterpretq_u16_u8 a)] =
  NV.lemma_vreinterpretq_u16_u8 a
let lemma_e_vreinterpretq_u8_s64 (a: t_e_int64x2_t)
  : Lemma (e_vreinterpretq_u8_s64 a == a) [SMTPat (e_vreinterpretq_u8_s64 a)] =
  NV.lemma_vreinterpretq_u8_s64 a
#pop-options

(* ============================================================================
   TIER C (part 2) — codec-value foundation + the i32<->u32 signedness bridge.

   Width-128 analogs of `Intrinsics_views.lemma_to_i32_val` (the shared codec
   `to_iv` decodes lane k of a 128-bit reg as the two's-complement of the raw
   32-bit slice value).  Both the i32 and u32 views read the SAME raw slice
   `dsum2 (lane_reader 128 32 x k) 0 32`; they differ only by the tc decode.
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_to_i32x4_val (x: t_e_int32x4_t) (k: nat{k < 4})
  : Lemma (v (Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 k)) ==
           IVi.tc_of_u Int.I32 (IVi.dsum2 (IVi.lane_reader (mk_u64 128) 32 x (mk_u64 k)) 0 32)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 128) 32 x (mk_u64 k) in
  IVi.dsum2_bound reader 0 32;
  IVi.lemma_tc_range Int.I32 (IVi.dsum2 reader 0 32)

let lemma_to_u32x4_val (x: t_e_uint32x4_t) (k: nat{k < 4})
  : Lemma (v (Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 x) (mk_u64 k)) ==
           IVi.dsum2 (IVi.lane_reader (mk_u64 128) 32 x (mk_u64 k)) 0 32) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 128) 32 x (mk_u64 k) in
  IVi.dsum2_bound reader 0 32;
  IVi.lemma_tc_range Int.U32 (IVi.dsum2 reader 0 32)
#pop-options

(* Per-lane signedness bridge between the i32x4 and u32x4 views of the same reg
   (mirrors `e_vreinterpret_i32_u32_lane_bridge`; consumed by compress). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let e_vreinterpret_i32_u32_lane_bridge (x: BV.t_BitVec (mk_u64 128)) (k: nat{k < 4})
  : Lemma
      (v (get_lane_i32x4 x k) ==
         (let u = v (get_lane_u32x4 x k) in if u < pow2 31 then u else u - pow2 32) /\
       v (get_lane_u32x4 x k) == (v (get_lane_i32x4 x k)) % pow2 32) =
  lemma_to_i32x4_val x k;
  lemma_to_u32x4_val x k;
  let raw = IVi.dsum2 (IVi.lane_reader (mk_u64 128) 32 x (mk_u64 k)) 0 32 in
  assert_norm (Int.bits Int.I32 == 32);
  Canon.lemma_tc_mod Int.I32 raw
#pop-options

(* ============================================================================
   TIER D foundation — width-128 i16<->i32 lane VALUE bridge (port of
   `Intrinsics_views.lemma_lane32_bridge` from 256->128).  A 32-bit i32 lane `j`
   is the little-endian concatenation of the two i16 lanes `2j` (low) / `2j+1`
   (high): its two's-complement value decomposes as
     (v(i16 2j) % 2^16) + 2^16 * v(i16 2j+1) == v(i32 j).
   This is the reusable arithmetic core the s32_s16 / s16_s32 repack op-facts
   rest on (the remaining step is the `i16x2_as_i32` cast-form pack lemma via
   `logor_disjoint`, mirroring Spec.MLDSA.Math).  All pieces (dsum2_split/shift,
   lemma_tc_mod, lemma_tc_pair) are PROVEN in `Intrinsics_views`.
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_to_i16x8_val (x: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (v (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 i)) ==
           IVi.tc_of_u Int.I16 (IVi.dsum2 (IVi.lane_reader (mk_u64 128) 16 x (mk_u64 i)) 0 16)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 128) 16 x (mk_u64 i) in
  IVi.dsum2_bound reader 0 16;
  IVi.lemma_tc_range Int.I16 (IVi.dsum2 reader 0 16)
#pop-options

(* the i32-lane reader and its two i16 half-lane readers agree bit-for-bit. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_reader_lo_128 (x: BV.t_BitVec (mk_u64 128)) (j: nat{j < 4}) (k: nat{k < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 32 x (mk_u64 j) k) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 16 x (mk_u64 (2 * j)) k)) =
  assert (32 * j + k < 128)

let lemma_reader_hi_128 (x: BV.t_BitVec (mk_u64 128)) (j: nat{j < 4}) (k: nat{k < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 32 x (mk_u64 j) (16 + k)) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 16 x (mk_u64 (2 * j + 1)) k)) =
  assert (32 * j + 16 + k < 128)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_lane_i16i32_128 (x: BV.t_BitVec (mk_u64 128)) (j: nat{j < 4})
    : Lemma ((v (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (2 * j))) % pow2 16) +
             pow2 16 * v (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (2 * j + 1))) ==
             v (Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 j))) =
  let reader16lo = IVi.lane_reader (mk_u64 128) 16 x (mk_u64 (2 * j)) in
  let reader16hi = IVi.lane_reader (mk_u64 128) 16 x (mk_u64 (2 * j + 1)) in
  let reader32 = IVi.lane_reader (mk_u64 128) 32 x (mk_u64 j) in
  IVi.dsum2_bound reader16lo 0 16;
  IVi.dsum2_bound reader16hi 0 16;
  let u_lo = IVi.dsum2 reader16lo 0 16 in
  let u_hi = IVi.dsum2 reader16hi 0 16 in
  lemma_to_i16x8_val x (2 * j);
  lemma_to_i16x8_val x (2 * j + 1);
  lemma_to_i32x4_val x j;
  Canon.dsum2_split reader32 0 16 16;
  Canon.dsum2_shift reader32 reader16lo 0 0 16 (fun k -> lemma_reader_lo_128 x j k);
  Canon.dsum2_shift reader32 reader16hi 16 0 16 (fun k -> lemma_reader_hi_128 x j k);
  Canon.lemma_tc_mod Int.I16 u_lo;
  Canon.lemma_tc_pair u_lo u_hi
#pop-options

(* ============================================================================
   TIER B — logical ops (vandq / veorq).  `ArmIV.vOPq_*` is a BIT-LEVEL
   `impl_9__from_fn 128`, so the codec-commute is proven bit-by-bit, mirroring
   `Intrinsics_views.lemma_and_i16x16_iv` at width 128:  per-bit readback (raw
   bit of the interpreted op == bit-op of operand bits) + `get_bit_OP` + bit-
   extensionality.  Needs the width-128 `impl_9__from_fn` index port (the width-
   256 `lemma_impl9_index` / `lemma_from_fn_lane_reader` don't apply at 128;
   `Canon.lemma_bv_index_n` is already width-generic).
   ========================================================================== *)

(* width-128 port of `Intrinsics_views.lemma_impl9_index` (the `on_domain` reduces). *)
let lemma_impl9_index_128 (f: (i: u64{v i < 128}) -> Bit.t_Bit) (k: u64{v k < 128})
    : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit
               (Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 128)
                  #(u64 -> Bit.t_Bit) f)._0 k == f k) = ()

(* raw underlying-FunArray bit `k` of the interpreted `vandq_s16` is the and of
   the operands' bits `k` (mirror `lemma_and_funarr`, width 128). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_vandq_funarr_128 (a b: BV.t_BitVec (mk_u64 128)) (k: u64{v k < 128})
    : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit (ArmIV.vandq_s16 a b)._0 k ==
             (match Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit a._0 k,
                    Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit b._0 k
              with
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_One
              | _ -> Bit.Bit_Zero)) =
  let f : (i: u64{v i < 128}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_One
              | _ -> Bit.Bit_Zero) in
  assert (ArmIV.vandq_s16 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 128) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%ArmIV.vandq_s16]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  lemma_impl9_index_128 f k;
  Canon.lemma_bv_index_n #(mk_u64 128) a k;
  Canon.lemma_bv_index_n #(mk_u64 128) b k
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_vandq_raw_128 (a b: BV.t_BitVec (mk_u64 128)) (ii: u64{v ii < 8}) (bb: nat{bb < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 16 (ArmIV.vandq_s16 a b) ii bb) ==
             Int.bit_and (IVi.bval (IVi.lane_reader (mk_u64 128) 16 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 128) 16 b ii bb))) =
  assert (16 * v ii + bb < 128);
  lemma_vandq_funarr_128 a b (mk_u64 (16 * v ii + bb))
#pop-options

(* i16-lane commutation for the interpreted vandq: decode ∘ bitwise-and == `&.`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_vandq_s16_i16x8_iv (a b: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 (ArmIV.vandq_s16 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 i)) &.
              (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 b) (mk_u64 i)))) =
  let aANDb = ArmIV.vandq_s16 a b in
  let ya : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 i) in
  let yb : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 b) (mk_u64 i) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 aANDb) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.I16 yr bb == Int.get_bit #Int.I16 (ya &. yb) bb) =
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) aANDb (mk_u64 i) (v bb);
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) a (mk_u64 i) (v bb);
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) b (mk_u64 i) (v bb);
    lemma_vandq_raw_128 a b (mk_u64 i) (v bb);
    Int.get_bit_and #Int.I16 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 yr (ya &. yb)
#pop-options

(* companion op-fact: i16x8 view of the (hardware) vandq_s16. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_e_vandq_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_vandq_s16 a b)
           == Spec.Utils.map2 ( &. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
          [SMTPat (vec128_as_i16x8 (e_vandq_s16 a b))] =
  let r = e_vandq_s16 a b in
  let aux (i: nat{i < 8})
      : Lemma (Seq.index (vec128_as_i16x8 r) i ==
               Seq.index (Spec.Utils.map2 ( &. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b)) i) =
    NV.lemma_vandq_s16 a b;
    lemma_vandq_s16_i16x8_iv a b i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec128_as_i16x8 r)
    (Spec.Utils.map2 ( &. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
#pop-options

(* vandq_u16 / vandq_u32 reuse the SAME bit-and funarr fact: ArmIV.vandq_u16 and
   ArmIV.vandq_u32 both delegate to ArmIV.vandq_s16.  Only the lane VIEW (u16x8 /
   u32x4) + lane width for the raw-bit fact change. *)

(* 32-bit-lane raw-bit variant (for the u32x4 view) reusing lemma_vandq_funarr_128. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_vandq_raw32_128 (a b: BV.t_BitVec (mk_u64 128)) (ii: u64{v ii < 4}) (bb: nat{bb < 32})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 32 (ArmIV.vandq_s16 a b) ii bb) ==
             Int.bit_and (IVi.bval (IVi.lane_reader (mk_u64 128) 32 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 128) 32 b ii bb))) =
  assert (32 * v ii + bb < 128);
  lemma_vandq_funarr_128 a b (mk_u64 (32 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_vandq_u16_u16x8_iv (a b: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 (ArmIV.vandq_u16 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 a) (mk_u64 i)) &.
              (Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 b) (mk_u64 i)))) =
  let r = ArmIV.vandq_u16 a b in
  let ya : u16 = Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 a) (mk_u64 i) in
  let yb : u16 = Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 b) (mk_u64 i) in
  let yr : u16 = Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 r) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.U16 yr bb == Int.get_bit #Int.U16 (ya &. yb) bb) =
    Canon.lemma_readback Int.U16 (mk_u64 128) (mk_u64 8) r (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U16 (mk_u64 128) (mk_u64 8) a (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U16 (mk_u64 128) (mk_u64 8) b (mk_u64 i) (v bb);
    lemma_vandq_raw_128 a b (mk_u64 i) (v bb);
    Int.get_bit_and #Int.U16 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U16 yr (ya &. yb)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_e_vandq_u16 (a b: t_e_uint16x8_t)
  : Lemma (vec128_as_u16x8 (e_vandq_u16 a b)
           == Spec.Utils.map2 ( &. ) (vec128_as_u16x8 a) (vec128_as_u16x8 b))
          [SMTPat (vec128_as_u16x8 (e_vandq_u16 a b))] =
  let r = e_vandq_u16 a b in
  let aux (i: nat{i < 8})
      : Lemma (Seq.index (vec128_as_u16x8 r) i ==
               Seq.index (Spec.Utils.map2 ( &. ) (vec128_as_u16x8 a) (vec128_as_u16x8 b)) i) =
    NV.lemma_vandq_u16 a b;
    lemma_vandq_u16_u16x8_iv a b i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec128_as_u16x8 r)
    (Spec.Utils.map2 ( &. ) (vec128_as_u16x8 a) (vec128_as_u16x8 b))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_vandq_u32_u32x4_iv (a b: BV.t_BitVec (mk_u64 128)) (i: nat{i < 4})
    : Lemma (Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 (ArmIV.vandq_u32 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 a) (mk_u64 i)) &.
              (Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 b) (mk_u64 i)))) =
  let r = ArmIV.vandq_u32 a b in
  let ya : u32 = Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 a) (mk_u64 i) in
  let yb : u32 = Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 b) (mk_u64 i) in
  let yr : u32 = Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 r) (mk_u64 i) in
  let aux (bb: usize{v bb < 32})
      : Lemma (Int.get_bit #Int.U32 yr bb == Int.get_bit #Int.U32 (ya &. yb) bb) =
    Canon.lemma_readback Int.U32 (mk_u64 128) (mk_u64 4) r (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U32 (mk_u64 128) (mk_u64 4) a (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U32 (mk_u64 128) (mk_u64 4) b (mk_u64 i) (v bb);
    lemma_vandq_raw32_128 a b (mk_u64 i) (v bb);
    Int.get_bit_and #Int.U32 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U32 yr (ya &. yb)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_e_vandq_u32 (a b: t_e_uint32x4_t)
  : Lemma (vec128_as_u32x4 (e_vandq_u32 a b)
           == Spec.Utils.map2 ( &. ) (vec128_as_u32x4 a) (vec128_as_u32x4 b))
          [SMTPat (vec128_as_u32x4 (e_vandq_u32 a b))] =
  let r = e_vandq_u32 a b in
  let aux (i: nat{i < 4})
      : Lemma (Seq.index (vec128_as_u32x4 r) i ==
               Seq.index (Spec.Utils.map2 ( &. ) (vec128_as_u32x4 a) (vec128_as_u32x4 b)) i) =
    NV.lemma_vandq_u32 a b;
    lemma_vandq_u32_u32x4_iv a b i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec128_as_u32x4 r)
    (Spec.Utils.map2 ( &. ) (vec128_as_u32x4 a) (vec128_as_u32x4 b))
#pop-options

(* ── veorq_s16 (XOR): own bit-level funarr/raw (Zero,Zero->Zero | One,One->Zero | _->One) *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_veorq_funarr_128 (a b: BV.t_BitVec (mk_u64 128)) (k: u64{v k < 128})
    : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit (ArmIV.veorq_s16 a b)._0 k ==
             (match Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit a._0 k,
                    Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit b._0 k
              with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_Zero
              | _ -> Bit.Bit_One)) =
  let f : (i: u64{v i < 128}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_Zero
              | _ -> Bit.Bit_One) in
  assert (ArmIV.veorq_s16 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 128) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%ArmIV.veorq_s16]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  lemma_impl9_index_128 f k;
  Canon.lemma_bv_index_n #(mk_u64 128) a k;
  Canon.lemma_bv_index_n #(mk_u64 128) b k
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_veorq_raw_128 (a b: BV.t_BitVec (mk_u64 128)) (ii: u64{v ii < 8}) (bb: nat{bb < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 16 (ArmIV.veorq_s16 a b) ii bb) ==
             Int.bit_xor (IVi.bval (IVi.lane_reader (mk_u64 128) 16 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 128) 16 b ii bb))) =
  assert (16 * v ii + bb < 128);
  lemma_veorq_funarr_128 a b (mk_u64 (16 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_veorq_s16_i16x8_iv (a b: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 (ArmIV.veorq_s16 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 i)) ^.
              (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 b) (mk_u64 i)))) =
  let aXORb = ArmIV.veorq_s16 a b in
  let ya : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 i) in
  let yb : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 b) (mk_u64 i) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 aXORb) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.I16 yr bb == Int.get_bit #Int.I16 (ya ^. yb) bb) =
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) aXORb (mk_u64 i) (v bb);
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) a (mk_u64 i) (v bb);
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) b (mk_u64 i) (v bb);
    lemma_veorq_raw_128 a b (mk_u64 i) (v bb);
    Int.get_bit_xor #Int.I16 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 yr (ya ^. yb)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_e_veorq_s16 (a b: t_e_int16x8_t)
  : Lemma (vec128_as_i16x8 (e_veorq_s16 a b)
           == Spec.Utils.map2 ( ^. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
          [SMTPat (vec128_as_i16x8 (e_veorq_s16 a b))] =
  let r = e_veorq_s16 a b in
  let aux (i: nat{i < 8})
      : Lemma (Seq.index (vec128_as_i16x8 r) i ==
               Seq.index (Spec.Utils.map2 ( ^. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b)) i) =
    NV.lemma_veorq_s16 a b;
    lemma_veorq_s16_i16x8_iv a b i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec128_as_i16x8 r)
    (Spec.Utils.map2 ( ^. ) (vec128_as_i16x8 a) (vec128_as_i16x8 b))
#pop-options

(* ============================================================================
   TIER D (part 2) — i16<->i32 cross-width reinterpret VALUE repacks.

   The real reinterpret is a pure bit relabel (`result == a`, TIER C part 1); the
   consumer post ALSO characterises each lane of the reinterpreted view as a
   little-endian repack of the source lanes:
     s32_s16 (i16x8->i32x4): get_lane_i32x4 r i == i16x2_as_i32 (i16 2i) (i16 2i+1)
     s16_s32 (i32x4->i16x8): get_lane_i16x8 r (2i)   == i32_lo16_as_i16 (i32 i)
                             get_lane_i16x8 r (2i+1) == i32_hi16_as_i16 (i32 i)

   Both close BIT-BY-BIT (no value arithmetic / no logor_disjoint / no
   v-injectivity), reusing the committed reader bridges:
     * `lemma_lane_i16i32_bit` (NEW): a BIT-level lane bridge — bit r of the i32
       lane j of x is bit r (resp. r-16) of the i16 lane 2j (resp. 2j+1) of x.
       Proven from `Canon.lemma_readback` (get_bit <-> lane_reader bval) +
       `lemma_reader_lo_128` / `lemma_reader_hi_128` (the same 32<->16 reader
       agreement that fed the width-128 VALUE bridge `lemma_lane_i16i32_128`).
     * The repack helper lets + their bit lemmas (`lemma_i16_bits_as_u32_bit`,
       `lemma_i16x2_as_i32_{lo,hi,bit}`).  Nothing here is assumed.
   ========================================================================== *)

(* ── repack helper lets (verbatim from Arm64_extract.fsti 561-592) ─────────── *)
let i16_bits_as_u32 (x: i16) : u32 =
  Rust_primitives.Integers.cast #Rust_primitives.Integers.u16_inttype #Rust_primitives.Integers.u32_inttype
    (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype #Rust_primitives.Integers.u16_inttype x)
let u32_lo16_as_i16 (x: u32) : i16 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype #Rust_primitives.Integers.i16_inttype
    (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype #Rust_primitives.Integers.u16_inttype x)
let u32_hi16_as_i16 (x: u32) : i16 = u32_lo16_as_i16 (x >>! mk_u32 16)
let i16x2_as_i32 (lo hi: i16) : i32 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype #Rust_primitives.Integers.i32_inttype
    (i16_bits_as_u32 lo |. (i16_bits_as_u32 hi <<! mk_u32 16))
let i32_lo16_as_i16 (x: i32) : i16 =
  u32_lo16_as_i16 (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i32_inttype #Rust_primitives.Integers.u32_inttype x)
let i32_hi16_as_i16 (x: i32) : i16 =
  u32_hi16_as_i16 (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i32_inttype #Rust_primitives.Integers.u32_inttype x)

(* ── repack bit lemmas (ported from Vector.Neon.Ntt_theory, now over the
      companion's own helper lets) ──────────────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_i16_bits_as_u32_bit (a: i16) (i: usize {v i < 32}) : Lemma
  (ensures Int.get_bit (i16_bits_as_u32 a) i ==
           (if v i < 16 then Int.get_bit a i else 0))
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
              #Rust_primitives.Integers.u16_inttype a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 32);
    assert (i16_bits_as_u32 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
              #Rust_primitives.Integers.u32_inttype w)

let lemma_i16x2_as_i32_lo (a b: i16) : Lemma
  (ensures i32_lo16_as_i16 (i16x2_as_i32 a b) == a)
  = let r = i32_lo16_as_i16 (i16x2_as_i32 a b) in
    let aux (i: usize {v i < 16}) : Lemma (Int.get_bit r i == Int.get_bit a i) =
      lemma_i16_bits_as_u32_bit a i in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r a

let lemma_i16x2_as_i32_hi (a b: i16) : Lemma
  (ensures i32_hi16_as_i16 (i16x2_as_i32 a b) == b)
  = let r = i32_hi16_as_i16 (i16x2_as_i32 a b) in
    let aux (i: usize {v i < 16}) : Lemma (Int.get_bit r i == Int.get_bit b i) =
      lemma_i16_bits_as_u32_bit a (sz (v i + 16));
      lemma_i16_bits_as_u32_bit b i in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r b

(* bit r (<32) of (i16x2_as_i32 lo hi): low half (r<16) is bit r of lo, high half
   is bit (r-16) of hi.  From get_bit_{cast,or,shl} SMTPats + the u32-pack bit. *)
let lemma_i16x2_as_i32_bit (lo hi: i16) (r: nat{r < 32})
    : Lemma (Int.get_bit (i16x2_as_i32 lo hi) (sz r) ==
             (if r < 16 then Int.get_bit lo (sz r) else Int.get_bit hi (sz (r - 16)))) =
  lemma_i16_bits_as_u32_bit lo (sz r);
  if r >= 16 then lemma_i16_bits_as_u32_bit hi (sz (r - 16))
#pop-options

(* ── BIT-level i16<->i32 lane bridge (bit analog of the VALUE bridge
      lemma_lane_i16i32_128): bit r of i32-lane j == bit (r|r-16) of i16-lane
      2j|2j+1 of the SAME 128-bit register.  readback x2 + reader_lo/hi. ─────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_lane_i16i32_bit (x: BV.t_BitVec (mk_u64 128)) (j: nat{j < 4}) (r: nat{r < 32})
    : Lemma (Int.get_bit #Int.I32 (Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 j)) (sz r) ==
             (if r < 16
              then Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (2 * j))) (sz r)
              else Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (2 * j + 1))) (sz (r - 16)))) =
  Canon.lemma_readback Int.I32 (mk_u64 128) (mk_u64 4) x (mk_u64 j) r;
  if r < 16 then begin
    lemma_reader_lo_128 x j r;
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 (2 * j)) r
  end
  else begin
    assert (16 + (r - 16) == r);
    lemma_reader_hi_128 x j (r - 16);
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 (2 * j + 1)) (r - 16)
  end
#pop-options

(* ── the two i16<->i32 cross-width reinterpret VALUE op-facts ──────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s32_s16_lane (a: t_e_int16x8_t) (i: nat{i < 4})
  : Lemma (get_lane_i32x4 (e_vreinterpretq_s32_s16 a) i ==
           i16x2_as_i32 (get_lane_i16x8 a (2 * i)) (get_lane_i16x8 a (2 * i + 1)))
          [SMTPat (get_lane_i32x4 (e_vreinterpretq_s32_s16 a) i)] =
  lemma_e_vreinterpretq_s32_s16 a;
  let w  = Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 a) (mk_u64 i) in
  let lo = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (2 * i)) in
  let hi = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (2 * i + 1)) in
  let rhs = i16x2_as_i32 lo hi in
  let aux (r: usize{v r < 32}) : Lemma (Int.get_bit w r == Int.get_bit rhs r) =
    lemma_lane_i16i32_bit a i (v r);
    lemma_i16x2_as_i32_bit lo hi (v r)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s16_s32_lane (a: t_e_int32x4_t) (i: nat{i < 4})
  : Lemma (get_lane_i16x8 (e_vreinterpretq_s16_s32 a) (2 * i) == i32_lo16_as_i16 (get_lane_i32x4 a i) /\
           get_lane_i16x8 (e_vreinterpretq_s16_s32 a) (2 * i + 1) == i32_hi16_as_i16 (get_lane_i32x4 a i))
          [SMTPat (get_lane_i16x8 (e_vreinterpretq_s16_s32 a) (2 * i))] =
  lemma_e_vreinterpretq_s16_s32 a;
  let lo = get_lane_i16x8 a (2 * i) in
  let hi = get_lane_i16x8 a (2 * i + 1) in
  lemma_e_vreinterpretq_s32_s16 a;
  lemma_e_vreinterpretq_s32_s16_lane a i;   (* get_lane_i32x4 a i == i16x2_as_i32 lo hi *)
  lemma_i16x2_as_i32_lo lo hi;
  lemma_i16x2_as_i32_hi lo hi
#pop-options

(* ============================================================================
   TIER D (part 3) — i16<->i64 cross-width reinterpret VALUE repacks.

   Same bit-by-bit recipe as part 2, at the 64-bit lane:
     s64_s16 (i16x8->i64x2): get_lane_i64x2 r i == i16x4_as_i64 (i16 4i..4i+3)
     s16_s64 (i64x2->i16x8): get_lane_i16x8 r k == i64_i16lane (i64 k/4) (k%4)
   `lemma_lane_i16i64_bit` (NEW) is the 64<->16 bit lane bridge (readback +
   `lemma_reader_i64_i16_128`, the 4-subLane analog of reader_lo/hi_128).  The
   pack/read helper lets + bit lemmas (`lemma_i16_bits_as_u64_bit`,
   `lemma_i16x4_as_i64_{bit,lane}`) are ported from Arm64_extract.fsti /
   Vector.Neon.Ntt_theory so repointed consumers see identical defs.
   ========================================================================== *)

(* ── i64 repack helper lets (verbatim from Arm64_extract.fsti 586-601) ─────── *)
let i16_bits_as_u64 (x: i16) : u64 =
  Rust_primitives.Integers.cast #Rust_primitives.Integers.u32_inttype #Rust_primitives.Integers.u64_inttype
    (i16_bits_as_u32 x)
let i16x4_as_i64 (a b c d: i16) : i64 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u64_inttype #Rust_primitives.Integers.i64_inttype
    (i16_bits_as_u64 a |. (i16_bits_as_u64 b <<! mk_u32 16) |.
     (i16_bits_as_u64 c <<! mk_u32 32) |. (i16_bits_as_u64 d <<! mk_u32 48))
let i64_i16lane (x: i64) (j: nat{j < 4}) : i16 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype #Rust_primitives.Integers.i16_inttype
    (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u64_inttype #Rust_primitives.Integers.u16_inttype
       ((Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i64_inttype #Rust_primitives.Integers.u64_inttype x)
        >>! mk_u32 (16 * j)))

(* ── i64 repack bit lemmas (ported from Vector.Neon.Ntt_theory) ────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_i16_bits_as_u64_bit (a: i16) (i: usize {v i < 64}) : Lemma
  (ensures Int.get_bit (i16_bits_as_u64 a) i ==
           (if v i < 16 then Int.get_bit a i else 0))
  = let w = i16_bits_as_u32 a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 64);
    assert (i16_bits_as_u64 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype
              #Rust_primitives.Integers.u64_inttype w);
    if v i < 32 then lemma_i16_bits_as_u32_bit a i

(* bit r (<64) of the packed i64 = bit (r mod 16) of the (r/16)-th of a,b,c,d *)
let lemma_i16x4_as_i64_bit (a b c d: i16) (r: nat{r < 64})
    : Lemma (Int.get_bit (i16x4_as_i64 a b c d) (sz r) ==
             (if r < 16 then Int.get_bit a (sz r)
              else if r < 32 then Int.get_bit b (sz (r - 16))
              else if r < 48 then Int.get_bit c (sz (r - 32))
              else Int.get_bit d (sz (r - 48)))) =
  lemma_i16_bits_as_u64_bit a (sz r);
  (if r >= 16 then lemma_i16_bits_as_u64_bit b (sz (r - 16)));
  (if r >= 32 then lemma_i16_bits_as_u64_bit c (sz (r - 32)));
  (if r >= 48 then lemma_i16_bits_as_u64_bit d (sz (r - 48)))

(* i64->i16 read-back: lane j of the packed i64 is the original 16-bit half. *)
let lemma_i16x4_as_i64_lane (a b c d: i16) (j: nat{j < 4}) : Lemma
  (ensures i64_i16lane (i16x4_as_i64 a b c d) j ==
           (match j with | 0 -> a | 1 -> b | 2 -> c | _ -> d))
  = let target : i16 = (match j with | 0 -> a | 1 -> b | 2 -> c | _ -> d) in
    let r = i64_i16lane (i16x4_as_i64 a b c d) j in
    let aux (i: usize {v i < 16}) : Lemma (Int.get_bit r i == Int.get_bit target i) =
      (match j with
       | 0 -> lemma_i16_bits_as_u64_bit a i
       | 1 -> lemma_i16_bits_as_u64_bit a (sz (v i + 16));
              lemma_i16_bits_as_u64_bit b i
       | 2 -> lemma_i16_bits_as_u64_bit a (sz (v i + 32));
              lemma_i16_bits_as_u64_bit b (sz (v i + 16));
              lemma_i16_bits_as_u64_bit c i
       | _ -> lemma_i16_bits_as_u64_bit a (sz (v i + 48));
              lemma_i16_bits_as_u64_bit b (sz (v i + 32));
              lemma_i16_bits_as_u64_bit c (sz (v i + 16));
              lemma_i16_bits_as_u64_bit d i) in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r target
#pop-options

(* ── 64<->16 reader agreement (4-subLane analog of reader_lo/hi_128) ───────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_reader_i64_i16_128 (x: BV.t_BitVec (mk_u64 128)) (i: nat{i < 2}) (s: nat{s < 4}) (k: nat{k < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 64 x (mk_u64 i) (16 * s + k)) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 16 x (mk_u64 (4 * i + s)) k)) =
  assert (64 * i + 16 * s + k < 128)
#pop-options

(* ── BIT-level i16<->i64 lane bridge: bit r of i64-lane i == bit (r%16) of
      i16-lane (4i + r/16) of the SAME register. ──────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_lane_i16i64_bit (x: BV.t_BitVec (mk_u64 128)) (i: nat{i < 2}) (r: nat{r < 64})
    : Lemma (Int.get_bit #Int.I64 (Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 x) (mk_u64 i)) (sz r) ==
             Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (4 * i + r / 16))) (sz (r % 16))) =
  Canon.lemma_readback Int.I64 (mk_u64 128) (mk_u64 2) x (mk_u64 i) r;
  let s = r / 16 in
  let k = r % 16 in
  assert (16 * s + k == r);
  lemma_reader_i64_i16_128 x i s k;
  Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 (4 * i + s)) k
#pop-options

(* ── the two i16<->i64 cross-width reinterpret VALUE op-facts ──────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s64_s16_lane (a: t_e_int16x8_t) (i: nat{i < 2})
  : Lemma (get_lane_i64x2 (e_vreinterpretq_s64_s16 a) i ==
           i16x4_as_i64 (get_lane_i16x8 a (4 * i)) (get_lane_i16x8 a (4 * i + 1))
                        (get_lane_i16x8 a (4 * i + 2)) (get_lane_i16x8 a (4 * i + 3)))
          [SMTPat (get_lane_i64x2 (e_vreinterpretq_s64_s16 a) i)] =
  lemma_e_vreinterpretq_s64_s16 a;
  let w  = Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 a) (mk_u64 i) in
  let l0 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (4 * i)) in
  let l1 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (4 * i + 1)) in
  let l2 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (4 * i + 2)) in
  let l3 = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (4 * i + 3)) in
  let rhs = i16x4_as_i64 l0 l1 l2 l3 in
  let aux (r: usize{v r < 64}) : Lemma (Int.get_bit w r == Int.get_bit rhs r) =
    lemma_lane_i16i64_bit a i (v r);
    lemma_i16x4_as_i64_bit l0 l1 l2 l3 (v r)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s16_s64_lane (a: t_e_int64x2_t) (k: nat{k < 8})
  : Lemma (get_lane_i16x8 (e_vreinterpretq_s16_s64 a) k ==
           i64_i16lane (get_lane_i64x2 a (k / 4)) (k % 4))
          [SMTPat (get_lane_i16x8 (e_vreinterpretq_s16_s64 a) k)] =
  lemma_e_vreinterpretq_s16_s64 a;
  let i = k / 4 in
  let s = k % 4 in
  assert (4 * i + s == k);
  let l0 = get_lane_i16x8 a (4 * i) in
  let l1 = get_lane_i16x8 a (4 * i + 1) in
  let l2 = get_lane_i16x8 a (4 * i + 2) in
  let l3 = get_lane_i16x8 a (4 * i + 3) in
  lemma_e_vreinterpretq_s64_s16 a;
  lemma_e_vreinterpretq_s64_s16_lane a i;   (* get_lane_i64x2 a i == i16x4_as_i64 l0 l1 l2 l3 *)
  lemma_i16x4_as_i64_lane l0 l1 l2 l3 s
#pop-options

(* ============================================================================
   TIER D (part 4) — u32<->s16 cross-width reinterpret VALUE repacks.

   The UNSIGNED-lane sibling of part 2: the wide view is `u32x4`, the pack is the
   raw u32 `i16_bits_as_u32 lo |. (i16_bits_as_u32 hi <<! 16)` (== the u32 that
   part 2's i16x2_as_i32 casts).  Bit-level via a U32 lane bridge + the same
   `i16_bits_as_u32` bit lemma.
     u32_s16 (i16x8->u32x4): get_lane_u32x4 r i == i16_bits_as_u32 (i16 2i) |.
                                                   (i16_bits_as_u32 (i16 2i+1) <<! 16)
     s16_u32 (u32x4->i16x8): get_lane_i16x8 r (2i)   == u32_lo16_as_i16 (u32 i)
                             get_lane_i16x8 r (2i+1) == u32_hi16_as_i16 (u32 i)
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
(* bit r of the raw u32 pack = bit r of lo (r<16) / bit r-16 of hi. *)
let lemma_i16x2_pack_u32_bit (lo hi: i16) (r: nat{r < 32})
    : Lemma (Int.get_bit (i16_bits_as_u32 lo |. (i16_bits_as_u32 hi <<! mk_u32 16)) (sz r) ==
             (if r < 16 then Int.get_bit lo (sz r) else Int.get_bit hi (sz (r - 16)))) =
  lemma_i16_bits_as_u32_bit lo (sz r);
  if r >= 16 then lemma_i16_bits_as_u32_bit hi (sz (r - 16))

(* u32->i16 read-back: the two 16-bit halves of the pack are the original i16s. *)
let lemma_pack_u32_lo (a b: i16) : Lemma
  (ensures u32_lo16_as_i16 (i16_bits_as_u32 a |. (i16_bits_as_u32 b <<! mk_u32 16)) == a)
  = let r = u32_lo16_as_i16 (i16_bits_as_u32 a |. (i16_bits_as_u32 b <<! mk_u32 16)) in
    let aux (i: usize {v i < 16}) : Lemma (Int.get_bit r i == Int.get_bit a i) =
      lemma_i16_bits_as_u32_bit a i in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r a

let lemma_pack_u32_hi (a b: i16) : Lemma
  (ensures u32_hi16_as_i16 (i16_bits_as_u32 a |. (i16_bits_as_u32 b <<! mk_u32 16)) == b)
  = let r = u32_hi16_as_i16 (i16_bits_as_u32 a |. (i16_bits_as_u32 b <<! mk_u32 16)) in
    let aux (i: usize {v i < 16}) : Lemma (Int.get_bit r i == Int.get_bit b i) =
      lemma_i16_bits_as_u32_bit a (sz (v i + 16));
      lemma_i16_bits_as_u32_bit b i in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r b
#pop-options

(* U32<->i16 bit lane bridge (part 2's lemma_lane_i16i32_bit with a U32 wide view). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_lane_u32i16_bit (x: BV.t_BitVec (mk_u64 128)) (j: nat{j < 4}) (r: nat{r < 32})
    : Lemma (Int.get_bit #Int.U32 (Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 x) (mk_u64 j)) (sz r) ==
             (if r < 16
              then Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (2 * j))) (sz r)
              else Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (2 * j + 1))) (sz (r - 16)))) =
  Canon.lemma_readback Int.U32 (mk_u64 128) (mk_u64 4) x (mk_u64 j) r;
  if r < 16 then begin
    lemma_reader_lo_128 x j r;
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 (2 * j)) r
  end
  else begin
    assert (16 + (r - 16) == r);
    lemma_reader_hi_128 x j (r - 16);
    Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 (2 * j + 1)) (r - 16)
  end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_u32_s16_lane (a: t_e_int16x8_t) (i: nat{i < 4})
  : Lemma (get_lane_u32x4 (e_vreinterpretq_u32_s16 a) i ==
           (i16_bits_as_u32 (get_lane_i16x8 a (2 * i)) |.
            (i16_bits_as_u32 (get_lane_i16x8 a (2 * i + 1)) <<! mk_u32 16)))
          [SMTPat (get_lane_u32x4 (e_vreinterpretq_u32_s16 a) i)] =
  lemma_e_vreinterpretq_u32_s16 a;
  let w  = Funarr.impl_5__get (mk_u64 4) #u32 (NV.to_u32x4 a) (mk_u64 i) in
  let lo = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (2 * i)) in
  let hi = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (2 * i + 1)) in
  let rhs = i16_bits_as_u32 lo |. (i16_bits_as_u32 hi <<! mk_u32 16) in
  let aux (r: usize{v r < 32}) : Lemma (Int.get_bit w r == Int.get_bit rhs r) =
    lemma_lane_u32i16_bit a i (v r);
    lemma_i16x2_pack_u32_bit lo hi (v r)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s16_u32_lane (a: t_e_uint32x4_t) (i: nat{i < 4})
  : Lemma (get_lane_i16x8 (e_vreinterpretq_s16_u32 a) (2 * i) == u32_lo16_as_i16 (get_lane_u32x4 a i) /\
           get_lane_i16x8 (e_vreinterpretq_s16_u32 a) (2 * i + 1) == u32_hi16_as_i16 (get_lane_u32x4 a i))
          [SMTPat (get_lane_i16x8 (e_vreinterpretq_s16_u32 a) (2 * i))] =
  lemma_e_vreinterpretq_s16_u32 a;
  let lo = get_lane_i16x8 a (2 * i) in
  let hi = get_lane_i16x8 a (2 * i + 1) in
  lemma_e_vreinterpretq_u32_s16 a;
  lemma_e_vreinterpretq_u32_s16_lane a i;   (* get_lane_u32x4 a i == pack lo hi *)
  lemma_pack_u32_lo lo hi;
  lemma_pack_u32_hi lo hi
#pop-options

(* ============================================================================
   TIER D (part 5) — s64_s32 cross-width reinterpret VALUE repack (i32<->i64).

   i32x2 little-endian pack into an i64 lane (only the s64_s32 direction is used).
     s64_s32 (i32x4->i64x2): get_lane_i64x2 r i == i32x2_as_i64 (i32 2i) (i32 2i+1)
   Bit-level: 64<->32 lane bridge + i32-pack bits.
   ========================================================================== *)

(* ── i32<->i64 repack helper lets (verbatim from Arm64_extract.fsti 568-577) ── *)
let i32_bits_as_u64 (x: i32) : u64 =
  Rust_primitives.Integers.cast #Rust_primitives.Integers.u32_inttype #Rust_primitives.Integers.u64_inttype
    (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i32_inttype #Rust_primitives.Integers.u32_inttype x)
let i32x2_as_i64 (lo hi: i32) : i64 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u64_inttype #Rust_primitives.Integers.i64_inttype
    (i32_bits_as_u64 lo |. (i32_bits_as_u64 hi <<! mk_u32 32))

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_i32_bits_as_u64_bit (a: i32) (i: usize {v i < 64}) : Lemma
  (ensures Int.get_bit (i32_bits_as_u64 a) i == (if v i < 32 then Int.get_bit a i else 0))
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i32_inttype
              #Rust_primitives.Integers.u32_inttype a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 64);
    assert (i32_bits_as_u64 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype
              #Rust_primitives.Integers.u64_inttype w)

let lemma_i32x2_as_i64_bit (lo hi: i32) (r: nat{r < 64})
    : Lemma (Int.get_bit (i32x2_as_i64 lo hi) (sz r) ==
             (if r < 32 then Int.get_bit lo (sz r) else Int.get_bit hi (sz (r - 32)))) =
  lemma_i32_bits_as_u64_bit lo (sz r);
  if r >= 32 then lemma_i32_bits_as_u64_bit hi (sz (r - 32))
#pop-options

(* 64<->32 reader agreement + bit lane bridge (2 sub-lanes per i64). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_reader_i64_i32_128 (x: BV.t_BitVec (mk_u64 128)) (i: nat{i < 2}) (s: nat{s < 2}) (k: nat{k < 32})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 64 x (mk_u64 i) (32 * s + k)) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 32 x (mk_u64 (2 * i + s)) k)) =
  assert (64 * i + 32 * s + k < 128)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_lane_i32i64_bit (x: BV.t_BitVec (mk_u64 128)) (i: nat{i < 2}) (r: nat{r < 64})
    : Lemma (Int.get_bit #Int.I64 (Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 x) (mk_u64 i)) (sz r) ==
             (if r < 32
              then Int.get_bit #Int.I32 (Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 (2 * i))) (sz r)
              else Int.get_bit #Int.I32 (Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 x) (mk_u64 (2 * i + 1))) (sz (r - 32)))) =
  Canon.lemma_readback Int.I64 (mk_u64 128) (mk_u64 2) x (mk_u64 i) r;
  if r < 32 then begin
    lemma_reader_i64_i32_128 x i 0 r;
    Canon.lemma_readback Int.I32 (mk_u64 128) (mk_u64 4) x (mk_u64 (2 * i)) r
  end
  else begin
    assert (32 + (r - 32) == r);
    lemma_reader_i64_i32_128 x i 1 (r - 32);
    Canon.lemma_readback Int.I32 (mk_u64 128) (mk_u64 4) x (mk_u64 (2 * i + 1)) (r - 32)
  end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s64_s32_lane (a: t_e_int32x4_t) (i: nat{i < 2})
  : Lemma (get_lane_i64x2 (e_vreinterpretq_s64_s32 a) i ==
           i32x2_as_i64 (get_lane_i32x4 a (2 * i)) (get_lane_i32x4 a (2 * i + 1)))
          [SMTPat (get_lane_i64x2 (e_vreinterpretq_s64_s32 a) i)] =
  lemma_e_vreinterpretq_s64_s32 a;
  let w  = Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 a) (mk_u64 i) in
  let lo = Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 a) (mk_u64 (2 * i)) in
  let hi = Funarr.impl_5__get (mk_u64 4) #i32 (Canon.to_i32x4 a) (mk_u64 (2 * i + 1)) in
  let rhs = i32x2_as_i64 lo hi in
  let aux (r: usize{v r < 64}) : Lemma (Int.get_bit w r == Int.get_bit rhs r) =
    lemma_lane_i32i64_bit a i (v r);
    lemma_i32x2_as_i64_bit lo hi (v r)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs
#pop-options

(* ============================================================================
   TIER D (part 6) — byte-level cross-width reinterpret VALUE repacks
   (u8<->s16, u16<->u8, u8<->s64).

     u8_s16 (i16x8->u8x16): get_lane_u8x16 r k == i16_byte (i16 k/2) (k%2)
     s16_u8 (u8x16->i16x8): get_lane_i16x8 r i == u8x2_as_i16 (u8 2i) (u8 2i+1)
     u16_u8 (u8x16->u16x8): get_lane_u16x8 r i == u8x2_as_u16 (u8 2i) (u8 2i+1)
     u8_s64 (i64x2->u8x16): get_lane_u8x16 r k == i64_byte (i64 k/8) (k%8)
   All bit-by-bit: the byte view is the width-8 lane, so the bridges relate
   `get_bit (u8-lane)` to `get_bit (wide-lane)` at the matching absolute bit,
   via `Canon.lemma_readback` + a reader-agreement one-liner.
   ========================================================================== *)

(* ── byte repack helper lets (verbatim from Arm64_extract.fsti 578-608) ─────── *)
let u8x2_as_u16 (lo hi: u8) : u16 =
  Rust_primitives.Integers.cast #Rust_primitives.Integers.u8_inttype #Rust_primitives.Integers.u16_inttype lo |.
  (Rust_primitives.Integers.cast #Rust_primitives.Integers.u8_inttype #Rust_primitives.Integers.u16_inttype hi <<! mk_u32 8)
let i64_byte (x: i64) (k: nat{k < 8}) : u8 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u64_inttype #Rust_primitives.Integers.u8_inttype
    ((Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i64_inttype #Rust_primitives.Integers.u64_inttype x)
     >>! mk_u32 (8 * k))
let i16_byte (x: i16) (j: nat{j < 2}) : u8 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype #Rust_primitives.Integers.u8_inttype
    ((Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype #Rust_primitives.Integers.u16_inttype x)
     >>! mk_u32 (8 * j))
let u8x2_as_i16 (lo hi: u8) : i16 =
  Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype #Rust_primitives.Integers.i16_inttype
    (u8x2_as_u16 lo hi)

(* ── byte-extraction + byte-pack bit lemmas ───────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_i16_byte_bit (x: i16) (j: nat{j < 2}) (b: nat{b < 8})
    : Lemma (Int.get_bit (i16_byte x j) (sz b) == Int.get_bit x (sz (8 * j + b))) = ()

let lemma_i64_byte_bit (x: i64) (k: nat{k < 8}) (b: nat{b < 8})
    : Lemma (Int.get_bit (i64_byte x k) (sz b) == Int.get_bit x (sz (8 * k + b))) = ()
#pop-options

(* u8x2_as_u16: ported from Vector.Neon.Serialize_theory (cast==cast_mod bridge). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_u8x2_as_u16_bit (lo hi: u8) (r: nat{r < 16})
    : Lemma (Int.get_bit (u8x2_as_u16 lo hi) (sz r) ==
             (if r < 8 then Int.get_bit lo (sz r) else Int.get_bit hi (sz (r - 8)))) =
  FStar.Math.Lemmas.small_mod (v lo) (pow2 16);
  FStar.Math.Lemmas.small_mod (v hi) (pow2 16);
  assert (Rust_primitives.Integers.cast #Rust_primitives.Integers.u8_inttype #Rust_primitives.Integers.u16_inttype lo ==
          Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u8_inttype #Rust_primitives.Integers.u16_inttype lo);
  assert (Rust_primitives.Integers.cast #Rust_primitives.Integers.u8_inttype #Rust_primitives.Integers.u16_inttype hi ==
          Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u8_inttype #Rust_primitives.Integers.u16_inttype hi)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_u8x2_as_i16_bit (lo hi: u8) (r: nat{r < 16})
    : Lemma (Int.get_bit (u8x2_as_i16 lo hi) (sz r) ==
             (if r < 8 then Int.get_bit lo (sz r) else Int.get_bit hi (sz (r - 8)))) =
  lemma_u8x2_as_u16_bit lo hi r
#pop-options

(* ── reader-agreement one-liners for the byte widths (16<->8 and 64<->8) ───── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_reader_i16_u8_128 (x: BV.t_BitVec (mk_u64 128)) (m: nat{m < 8}) (s: nat{s < 2}) (b: nat{b < 8})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 16 x (mk_u64 m) (8 * s + b)) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 8 x (mk_u64 (2 * m + s)) b)) =
  assert (16 * m + 8 * s + b < 128)

let lemma_reader_i64_u8_128 (x: BV.t_BitVec (mk_u64 128)) (m: nat{m < 2}) (s: nat{s < 8}) (b: nat{b < 8})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 64 x (mk_u64 m) (8 * s + b)) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 8 x (mk_u64 (8 * m + s)) b)) =
  assert (64 * m + 8 * s + b < 128)
#pop-options

(* ── byte bit lane bridges ────────────────────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
(* u8-lane k == byte (k%2) of i16-lane (k/2) *)
let lemma_lane_u8i16_bit (x: BV.t_BitVec (mk_u64 128)) (k: nat{k < 16}) (b: nat{b < 8})
    : Lemma (Int.get_bit #Int.U8 (Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 k)) (sz b) ==
             Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 (k / 2))) (sz (8 * (k % 2) + b))) =
  Canon.lemma_readback Int.U8 (mk_u64 128) (mk_u64 16) x (mk_u64 k) b;
  let m = k / 2 in let s = k % 2 in
  assert (2 * m + s == k);
  lemma_reader_i16_u8_128 x m s b;
  Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 m) (8 * s + b)

(* i16-lane i, bit r == u8-lane (2i + r/8), bit (r%8) *)
let lemma_lane_i16u8_bit (x: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8}) (r: nat{r < 16})
    : Lemma (Int.get_bit #Int.I16 (Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 i)) (sz r) ==
             Int.get_bit #Int.U8 (Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 (2 * i + r / 8))) (sz (r % 8))) =
  Canon.lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 i) r;
  let s = r / 8 in let b = r % 8 in
  assert (8 * s + b == r);
  lemma_reader_i16_u8_128 x i s b;
  Canon.lemma_readback Int.U8 (mk_u64 128) (mk_u64 16) x (mk_u64 (2 * i + s)) b

(* u16-lane i, bit r == u8-lane (2i + r/8), bit (r%8) *)
let lemma_lane_u16u8_bit (x: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8}) (r: nat{r < 16})
    : Lemma (Int.get_bit #Int.U16 (Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 x) (mk_u64 i)) (sz r) ==
             Int.get_bit #Int.U8 (Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 (2 * i + r / 8))) (sz (r % 8))) =
  Canon.lemma_readback Int.U16 (mk_u64 128) (mk_u64 8) x (mk_u64 i) r;
  let s = r / 8 in let b = r % 8 in
  assert (8 * s + b == r);
  lemma_reader_i16_u8_128 x i s b;
  Canon.lemma_readback Int.U8 (mk_u64 128) (mk_u64 16) x (mk_u64 (2 * i + s)) b

(* u8-lane k == byte (k%8) of i64-lane (k/8) *)
let lemma_lane_u8i64_bit (x: BV.t_BitVec (mk_u64 128)) (k: nat{k < 16}) (b: nat{b < 8})
    : Lemma (Int.get_bit #Int.U8 (Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 k)) (sz b) ==
             Int.get_bit #Int.I64 (Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 x) (mk_u64 (k / 8))) (sz (8 * (k % 8) + b))) =
  Canon.lemma_readback Int.U8 (mk_u64 128) (mk_u64 16) x (mk_u64 k) b;
  let m = k / 8 in let s = k % 8 in
  assert (8 * m + s == k);
  lemma_reader_i64_u8_128 x m s b;
  Canon.lemma_readback Int.I64 (mk_u64 128) (mk_u64 2) x (mk_u64 m) (8 * s + b)
#pop-options

(* ── the four byte reinterpret VALUE op-facts ─────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_u8_s16_lane (a: t_e_int16x8_t) (k: nat{k < 16})
  : Lemma (get_lane_u8x16 (e_vreinterpretq_u8_s16 a) k ==
           i16_byte (get_lane_i16x8 a (k / 2)) (k % 2))
          [SMTPat (get_lane_u8x16 (e_vreinterpretq_u8_s16 a) k)] =
  lemma_e_vreinterpretq_u8_s16 a;
  let w   = Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 a) (mk_u64 k) in
  let src = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 (k / 2)) in
  let rhs = i16_byte src (k % 2) in
  let aux (b: usize{v b < 8}) : Lemma (Int.get_bit w b == Int.get_bit rhs b) =
    lemma_lane_u8i16_bit a k (v b);
    lemma_i16_byte_bit src (k % 2) (v b)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs

let lemma_e_vreinterpretq_s16_u8_lane (a: t_e_uint8x16_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vreinterpretq_s16_u8 a) i ==
           u8x2_as_i16 (get_lane_u8x16 a (2 * i)) (get_lane_u8x16 a (2 * i + 1)))
          [SMTPat (get_lane_i16x8 (e_vreinterpretq_s16_u8 a) i)] =
  lemma_e_vreinterpretq_s16_u8 a;
  let w  = Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 a) (mk_u64 i) in
  let lo = Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 a) (mk_u64 (2 * i)) in
  let hi = Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 a) (mk_u64 (2 * i + 1)) in
  let rhs = u8x2_as_i16 lo hi in
  let aux (r: usize{v r < 16}) : Lemma (Int.get_bit w r == Int.get_bit rhs r) =
    lemma_lane_i16u8_bit a i (v r);
    lemma_u8x2_as_i16_bit lo hi (v r)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs

let lemma_e_vreinterpretq_u16_u8_lane (a: t_e_uint8x16_t) (i: nat{i < 8})
  : Lemma (get_lane_u16x8 (e_vreinterpretq_u16_u8 a) i ==
           u8x2_as_u16 (get_lane_u8x16 a (2 * i)) (get_lane_u8x16 a (2 * i + 1)))
          [SMTPat (get_lane_u16x8 (e_vreinterpretq_u16_u8 a) i)] =
  lemma_e_vreinterpretq_u16_u8 a;
  let w  = Funarr.impl_5__get (mk_u64 8) #u16 (NV.to_u16x8 a) (mk_u64 i) in
  let lo = Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 a) (mk_u64 (2 * i)) in
  let hi = Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 a) (mk_u64 (2 * i + 1)) in
  let rhs = u8x2_as_u16 lo hi in
  let aux (r: usize{v r < 16}) : Lemma (Int.get_bit w r == Int.get_bit rhs r) =
    lemma_lane_u16u8_bit a i (v r);
    lemma_u8x2_as_u16_bit lo hi (v r)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs

let lemma_e_vreinterpretq_u8_s64_lane (a: t_e_int64x2_t) (k: nat{k < 16})
  : Lemma (get_lane_u8x16 (e_vreinterpretq_u8_s64 a) k ==
           i64_byte (get_lane_i64x2 a (k / 8)) (k % 8))
          [SMTPat (get_lane_u8x16 (e_vreinterpretq_u8_s64 a) k)] =
  lemma_e_vreinterpretq_u8_s64 a;
  let w   = Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 a) (mk_u64 k) in
  let src = Funarr.impl_5__get (mk_u64 2) #i64 (Canon.to_i64x2 a) (mk_u64 (k / 8)) in
  let rhs = i64_byte src (k % 8) in
  let aux (b: usize{v b < 8}) : Lemma (Int.get_bit w b == Int.get_bit rhs b) =
    lemma_lane_u8i64_bit a k (v b);
    lemma_i64_byte_bit src (k % 8) (v b)
  in
  Classical.forall_intro aux;
  Rust_primitives.Integers.lemma_int_t_eq_via_bits w rhs
#pop-options

(* ============================================================================
   TIER F (load) — vld1q_s16: read 8 i16 lanes from a slice.

   `e_vld1q_s16 array` (transparent) == `Arm.Extra.vld1q_s16_model array`
   (opaque) == `from_i16x8 (from_fn 8 (fun j -> let j=j in if j<len then array[j]
   else 0))`.  Reveal the model, name the inner from_fn `y` (VERBATIM lambda so it
   equals the model's), round-trip `to_i16x8 (from_i16x8 y) == y` (`Canon.rt_i16x8`),
   read back lane i via `impl_5__from_fn`/`impl_5__get` reduction (`feq_on_domain`
   SMTPat, needs fuel 2), then the guard (i < 8 <= len) selects `array[i]`.
   Per-lane op-fact (i a param — no elaboration wall).
   ========================================================================== *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vld1q_s16_lane (array: t_Slice i16) (i: nat{i < 8})
  : Lemma (requires v (Core_models.Slice.impl__len #i16 array) >= 8)
          (ensures get_lane_i16x8 (e_vld1q_s16 array) i == Seq.index array i)
          [SMTPat (get_lane_i16x8 (e_vld1q_s16 array) i)] =
  let f : (j:u64{v j < 8}) -> i16 =
    (fun j -> let j:u64 = j in
              if (cast (j <: u64) <: usize) <. (Core_models.Slice.impl__len #i16 array <: usize) <: bool
              then array.[ cast (j <: u64) <: usize ] <: i16
              else mk_i16 0) in
  let y : Funarr.t_FunArray (mk_u64 8) i16 = Funarr.impl_5__from_fn (mk_u64 8) #i16 #(u64 -> i16) f in
  (* model unfold + closure equality by normalization (like lemma_veorq_funarr_128) *)
  assert (e_vld1q_s16 array == Canon.from_i16x8 y)
    by (FStar.Tactics.norm [delta_only [`%e_vld1q_s16;
                                        `%Libcrux_core_models.Core_arch.Arm.Extra.vld1q_s16_model];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  Canon.rt_i16x8 y;                                             (* to_i16x8 (from_i16x8 y) == y *)
  assert (Funarr.impl_5__get (mk_u64 8) #i16 y (mk_u64 i) == f (mk_u64 i));   (* feq_on_domain *)
  assert (f (mk_u64 i) == Seq.index array i)
#pop-options

(* vld1q_u8: read 16 u8 lanes.  e_vld1q_u8 delegates to Arm.Extra.vld1q_bytes_model
   (u8x16 codec); same recipe as vld1q_s16 with NV.from_u8x16 / NV.rt_u8x16. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vld1q_u8_lane (ptr: t_Slice u8) (i: nat{i < 16})
  : Lemma (requires v (Core_models.Slice.impl__len #u8 ptr) >= 16)
          (ensures get_lane_u8x16 (e_vld1q_u8 ptr) i == Seq.index ptr i)
          [SMTPat (get_lane_u8x16 (e_vld1q_u8 ptr) i)] =
  let f : (j:u64{v j < 16}) -> u8 =
    (fun j -> let j:u64 = j in
              if (cast (j <: u64) <: usize) <. (Core_models.Slice.impl__len #u8 ptr <: usize) <: bool
              then ptr.[ cast (j <: u64) <: usize ] <: u8
              else mk_u8 0) in
  let y : Funarr.t_FunArray (mk_u64 16) u8 = Funarr.impl_5__from_fn (mk_u64 16) #u8 #(u64 -> u8) f in
  assert (e_vld1q_u8 ptr == NV.from_u8x16 y)
    by (FStar.Tactics.norm [delta_only [`%e_vld1q_u8;
                                        `%Libcrux_core_models.Core_arch.Arm.Extra.vld1q_bytes_model];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  NV.rt_u8x16 y;
  assert (Funarr.impl_5__get (mk_u64 16) #u8 y (mk_u64 i) == f (mk_u64 i));
  assert (f (mk_u64 i) == Seq.index ptr i)
#pop-options

(* ============================================================================
   Tier F STORES.  The store ops write N lanes of the vector `vec` into the
   output slice via a straight chain of N `update_at_usize` (= `Seq.upd`) under
   `if len >= N`.  The compound post (length + per-lane write + frame) is
   split into per-index SMTPat op-facts to dodge the `.fst`-ensures elaboration
   wall (see [[feedback_fst_ensures_refinement_under_forall]]).

   Recipe: reveal `e_vstX` (opaque) -> `Arm.Extra.vstX_model` (opaque) -> the
   `Seq.upd` chain.  `update_at_usize s i x == Seq.upd s (v i) x` is TRANSPARENT
   and `Seq.upd` carries index/length SMTPats, so length + framing are automatic;
   the lane value `lanes.[mk_u64 k] == get_lane_i16x8 vec k` bridges through the
   `vec128_index` SMTPat (`lanes == to_i16x8 vec` definitionally). ────────────── *)

(* vst1q_s16: write 8 i16 lanes into `out` (needs len out >= 8). *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vst1q_s16_length (out: t_Slice i16) (vec: t_e_int16x8_t)
  : Lemma (ensures Seq.length (e_vst1q_s16 out vec) == Seq.length out)
          [SMTPat (Seq.length (e_vst1q_s16 out vec))] =
  reveal_opaque (`%e_vst1q_s16) e_vst1q_s16;
  reveal_opaque (`%Libcrux_core_models.Core_arch.Arm.Extra.vst1q_s16_model)
                Libcrux_core_models.Core_arch.Arm.Extra.vst1q_s16_model

let lemma_e_vst1q_s16_lane (out: t_Slice i16) (vec: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (requires v (Core_models.Slice.impl__len #i16 out) >= 8)
          (ensures Seq.index (e_vst1q_s16 out vec) i == get_lane_i16x8 vec i)
          [SMTPat (Seq.index (e_vst1q_s16 out vec) i)] =
  reveal_opaque (`%e_vst1q_s16) e_vst1q_s16;
  reveal_opaque (`%Libcrux_core_models.Core_arch.Arm.Extra.vst1q_s16_model)
                Libcrux_core_models.Core_arch.Arm.Extra.vst1q_s16_model

let lemma_e_vst1q_s16_frame (out: t_Slice i16) (vec: t_e_int16x8_t) (i: nat)
  : Lemma (requires v (Core_models.Slice.impl__len #i16 out) >= 8 /\ i >= 8 /\ i < Seq.length out)
          (ensures Seq.index (e_vst1q_s16 out vec) i == Seq.index out i)
          [SMTPat (Seq.index (e_vst1q_s16 out vec) i)] =
  reveal_opaque (`%e_vst1q_s16) e_vst1q_s16;
  reveal_opaque (`%Libcrux_core_models.Core_arch.Arm.Extra.vst1q_s16_model)
                Libcrux_core_models.Core_arch.Arm.Extra.vst1q_s16_model
#pop-options

(* vst1q_u8: write 16 u8 lanes into `out` (via vst1q_bytes_model; needs len out
   >= 16).  UNLIKE vst1q_s16 (8-deep), the 16-deep `Seq.upd` chain defeats Z3's
   monolithic peel for the per-lane WRITE: frame-only (all upd2) and a shallow
   lane (idx 15, upd1-first) prove, but the peel-to-written-lane at depth 16
   (15 upd2 + upd1 + FunArray bridge) bails "incomplete quantifiers" even at
   fuel/ifuel 6, rlimit 400.  This is the "long sequential update_at chain"
   pattern (§7): factor the chain into a recursive prefix fn whose index lemma
   proves ONE Seq.upd step per level — no monolithic deep peel. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vst1q_u8_length (out: t_Slice u8) (vec: t_e_uint8x16_t)
  : Lemma (ensures Seq.length (e_vst1q_u8 out vec) == Seq.length out)
          [SMTPat (Seq.length (e_vst1q_u8 out vec))] =
  reveal_opaque (`%e_vst1q_u8) e_vst1q_u8;
  reveal_opaque (`%Libcrux_core_models.Core_arch.Arm.Extra.vst1q_bytes_model)
                Libcrux_core_models.Core_arch.Arm.Extra.vst1q_bytes_model
#pop-options

(* `out` with lanes[0..n) written, updates applied ascending (matches the model
   chain: update 0 innermost, update 15 outermost).  fuel >= 1 for the one-level
   unfold in the index lemma (module default is fuel 0). *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 100"
let rec upd_prefix_u8 (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 16) u8)
                      (n: nat{n <= 16 /\ Seq.length out >= 16})
  : Tot (r: t_Slice u8 {Seq.length r == Seq.length out}) (decreases n) =
  if n = 0 then out
  else Rust_primitives.Hax.Monomorphized_update_at.update_at_usize
         (upd_prefix_u8 out lanes (n - 1)) (mk_usize (n - 1)) (lanes.[ mk_u64 (n - 1) ])

(* Index characterization: one Seq.upd step per recursion level. *)
let rec lemma_upd_prefix_u8_index (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 16) u8)
                                  (n: nat{n <= 16 /\ Seq.length out >= 16}) (k: nat{k < 16})
  : Lemma (ensures Seq.index (upd_prefix_u8 out lanes n) k
                   == (if k < n then lanes.[ mk_u64 k ] else Seq.index out k))
          (decreases n) =
  if n = 0 then () else lemma_upd_prefix_u8_index out lanes (n - 1) k

(* Frame (k >= 16): unwritten positions unchanged.  Separate from the index
   lemma so `lanes.[mk_u64 k]` never appears for k >= 16 (out-of-range). *)
let rec lemma_upd_prefix_u8_frame (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 16) u8)
                                  (n: nat{n <= 16 /\ Seq.length out >= 16})
                                  (k: nat{16 <= k /\ k < Seq.length out})
  : Lemma (ensures Seq.index (upd_prefix_u8 out lanes n) k == Seq.index out k)
          (decreases n) =
  if n = 0 then () else lemma_upd_prefix_u8_frame out lanes (n - 1) k
#pop-options

(* Connection: the revealed model chain (len >= 16 branch) IS upd_prefix_u8 16.
   High fuel to unfold the 16-level recursion into the model's explicit chain. *)
#push-options "--fuel 20 --ifuel 2 --z3rlimit 200"
let lemma_vst1q_u8_model_eq (out: t_Slice u8) (vec: t_e_uint8x16_t)
  : Lemma (requires Seq.length out >= 16)
          (ensures e_vst1q_u8 out vec == upd_prefix_u8 out (NV.to_u8x16 vec) 16) =
  reveal_opaque (`%e_vst1q_u8) e_vst1q_u8;
  reveal_opaque (`%Libcrux_core_models.Core_arch.Arm.Extra.vst1q_bytes_model)
                Libcrux_core_models.Core_arch.Arm.Extra.vst1q_bytes_model
#pop-options

#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vst1q_u8_lane (out: t_Slice u8) (vec: t_e_uint8x16_t) (i: nat{i < 16})
  : Lemma (requires v (Core_models.Slice.impl__len #u8 out) >= 16)
          (ensures Seq.index (e_vst1q_u8 out vec) i == get_lane_u8x16 vec i)
          [SMTPat (Seq.index (e_vst1q_u8 out vec) i)] =
  lemma_vst1q_u8_model_eq out vec;
  lemma_upd_prefix_u8_index out (NV.to_u8x16 vec) 16 i
  (* Seq.index result i == lanes.[mk_u64 i]; bridge to get_lane_u8x16 via
     vec128_index_u8x16 SMTPat (shallow, no upd chain). *)

let lemma_e_vst1q_u8_frame (out: t_Slice u8) (vec: t_e_uint8x16_t) (i: nat)
  : Lemma (requires v (Core_models.Slice.impl__len #u8 out) >= 16 /\ i >= 16 /\ i < Seq.length out)
          (ensures Seq.index (e_vst1q_u8 out vec) i == Seq.index out i)
          [SMTPat (Seq.index (e_vst1q_u8 out vec) i)] =
  lemma_vst1q_u8_model_eq out vec;
  lemma_upd_prefix_u8_frame out (NV.to_u8x16 vec) 16 i
#pop-options

(* ============================================================================
   Bit-level bridges for from_bytes / to_bytes (byte-serialization consumers).
   The lane VIEW (`t_BitVec 128`) exposes bits via `bv_bit`; `bit_vec_of_int_t_
   array` is the byte-array serialization.  The bridge relates the i16x8 view's
   d-bit serialization to raw bit `(i/d)*16 + i%d`.  All PROVEN from the codec
   read-back (`Canon.lemma_readback`) — no assumption.  Mirrors the x86
   `Avx2_ml_kem_views` bit bridges. ─────────────────────────────────────────── *)

(* bit `i` of a core-models `t_BitVec n`, as a Rust `bit`. *)
let bv_bit (#n: u64) (bv: BV.t_BitVec n) (i: nat{i < v n}) : Rust_primitives.Integers.bit =
  match bv.[ mk_u64 i ] <: Bit.t_Bit with
  | Bit.Bit_One  -> 1
  | Bit.Bit_Zero -> 0

(* bv_bit <-> canonical lane_reader collapse (both read `bv._0` at index w*l+b). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_bv_bit_reader (#n: u64) (w: pos) (bv: BV.t_BitVec n)
    (l: nat) (b: nat{b < w /\ w * l + b < v n})
  : Lemma (IVi.bval (IVi.lane_reader n w bv (mk_u64 l) b) == bv_bit bv (w * l + b)) =
  FStar.Math.Lemmas.lemma_mult_le_right l 1 w;
  assert (l <= w * l)
#pop-options

(* the i16x8 view's d-bit serialization at bit i == raw bit (i/d)*16 + i%d. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 250"
let bit_vec_of_int_t_array_vec128_as_i16x8_lemma
      (vec: t_e_int16x8_t) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 8 * d})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array (vec128_as_i16x8 vec) d i
             == bv_bit vec ((i / d) * 16 + i % d)) =
  FStar.Math.Lemmas.euclidean_division_definition i d;
  FStar.Math.Lemmas.cancel_mul_div 8 d;
  FStar.Math.Lemmas.lemma_div_le i (8 * d) d;
  assert (i / d <= 8);
  assert (i / d < 8);
  assert (i % d < 16);
  Canon.lemma_readback Rust_primitives.Integers.I16 (mk_u64 128) (mk_u64 8) vec
    (mk_u64 (i / d)) (i % d)
#pop-options

(* ── byte LOAD bit-readback: bit i of e_vld1q_bytes IS bit (i%8) of byte (i/8).
   e_vld1q_bytes and e_vld1q_u8 share vld1q_bytes_model, so the u8-lane fact
   supplies the byte value; Canon.lemma_readback + lemma_bv_bit_reader bridge the
   bit.  Mirrors Avx2_ml_kem_views.lemma_bv_bit_mm_loadu_si128. ──────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_e_vld1q_bytes (input: t_Slice u8) (i: nat{i < 128})
  : Lemma (requires Seq.length input == 16)
          (ensures bv_bit (e_vld1q_bytes input) i
                   == Rust_primitives.Integers.get_bit (Seq.index input (i / 8)) (sz (i % 8))) =
  reveal_opaque (`%e_vld1q_bytes) e_vld1q_bytes;
  reveal_opaque (`%e_vld1q_u8) e_vld1q_u8;
  let bv = e_vld1q_bytes input in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_bv_bit_reader #(mk_u64 128) 8 bv (i / 8) (i % 8);
  Canon.lemma_readback Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) bv
    (mk_u64 (i / 8)) (i % 8);
  lemma_e_vld1q_u8_lane input (i / 8);
  assert (NV.to_u8x16 bv == IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) bv)
#pop-options

(* ── byte STORE bit post: bit i of the stored byte array IS bit i of the vector.
   Reuses upd_prefix_u8 (e_vst1q_bytes shares vst1q_bytes_model with e_vst1q_u8).
   Mirrors Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128. ────────────────────── *)
#push-options "--fuel 20 --ifuel 2 --z3rlimit 200"
let lemma_vst1q_bytes_model_eq (out: t_Slice u8) (vec: t_e_int16x8_t)
  : Lemma (requires Seq.length out >= 16)
          (ensures e_vst1q_bytes out vec == upd_prefix_u8 out (NV.to_u8x16 vec) 16) =
  reveal_opaque (`%e_vst1q_bytes) e_vst1q_bytes;
  reveal_opaque (`%Libcrux_core_models.Core_arch.Arm.Extra.vst1q_bytes_model)
                Libcrux_core_models.Core_arch.Arm.Extra.vst1q_bytes_model
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_vst1q_bytes (out: t_Slice u8) (vec: t_e_int16x8_t)
  : Lemma (requires Seq.length out == 16)
          (ensures (let out' = e_vst1q_bytes out vec in
                    Seq.length out' == 16 /\
                    (forall (i: nat{i < 128}).
                       Rust_primitives.BitVectors.bit_vec_of_int_t_array
                         (out' <: t_Array u8 (sz 16)) 8 i == bv_bit vec i))) =
  lemma_vst1q_bytes_model_eq out vec;
  let out' = e_vst1q_bytes out vec in
  assert (Seq.length out' == 16);
  let aux (i: nat{i < 128})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
               (out' <: t_Array u8 (sz 16)) 8 i == bv_bit vec i) =
    FStar.Math.Lemmas.euclidean_division_definition i 8;
    lemma_upd_prefix_u8_index out (NV.to_u8x16 vec) 16 (i / 8);
    Canon.lemma_readback Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) vec
      (mk_u64 (i / 8)) (i % 8);
    lemma_bv_bit_reader #(mk_u64 128) 8 vec (i / 8) (i % 8);
    assert (NV.to_u8x16 vec == IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) vec);
    assert (Seq.index out' (i / 8) ==
            Funarr.impl_5__get (mk_u64 16) #u8
              (IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) vec) (mk_u64 (i / 8)))
  in
  FStar.Classical.forall_intro aux
#pop-options

(* ============================================================================
   Item-4 SHIFTS — vsliq_n_s32 / _s64 (shift-left-and-insert).  The op-fact
   ensures `(a[i] &. arm_low_mask (v v_N)) |. (b[i] <<! v_N)` fails to ELABORATE
   in a `.fst` Lemma (the requires' `v v_N < 32` isn't in scope under the
   refinement-carrying `arm_low_mask_i32 (v v_N)` / `<<! v_N`).  Dodge: an
   UNREFINED guarded helper `vsli_lane` (refinements internal), so the ensures
   carries no refinement obligation.  Consumers use concrete N (10/12/20/24) so
   the helper unfolds to the target form.  Model = ArmIV.vsliq (per-lane FunArray
   under NV foundation), else-branch = u32 mask + shift, bridged to the i32 form
   by two scalar lemmas. ─────────────────────────────────────────────────────── *)

(* low-N-bits mask 2^N-1. *)
let arm_low_mask_i32 (n: nat{n < 32}) : i32 =
  FStar.Math.Lemmas.pow2_le_compat 31 n;
  mk_i32 (pow2 n - 1)
let arm_low_mask_i64 (n: nat{n < 64}) : i64 =
  FStar.Math.Lemmas.pow2_le_compat 63 n;
  mk_i64 (pow2 n - 1)

(* unrefined guarded per-lane result (matches ArmIV.vsliq under 0<n<32). *)
let vsli_lane_i32 (a b: i32) (n: nat) : i32 =
  if 0 < n && n < 32 then (a &. arm_low_mask_i32 n) |. (b <<! mk_i32 n) else b

(* scalar bridge: the model's u32 shift-left-then-cast == the i32 shift. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_vsli_shift_i32 (bv v_N: i32) : Lemma
  (requires v v_N >= 0 /\ v v_N < 32)
  (ensures (cast ((cast (bv <: i32) <: u32) <<! (cast (v_N <: i32) <: u32) <: u32) <: i32)
           == (bv <<! v_N)) =
  let lhs : i32 = cast ((cast (bv <: i32) <: u32) <<! (cast (v_N <: i32) <: u32) <: u32) <: i32 in
  let rhs : i32 = bv <<! v_N in
  let aux (r: usize{v r < 32}) : Lemma (Int.get_bit lhs r == Int.get_bit rhs r) = () in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits lhs rhs
#pop-options

(* scalar bridge: the model's u32 mask == arm_low_mask_i32. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_vsli_mask_i32 (v_N: i32) : Lemma
  (requires v v_N > 0 /\ v v_N < 32)
  (ensures (cast (Core_models.Num.impl_u32__wrapping_sub (mk_u32 1 <<! v_N <: u32) (mk_u32 1) <: u32) <: i32)
           == arm_low_mask_i32 (v v_N)) =
  FStar.Math.Lemmas.pow2_lt_compat 32 (v v_N);
  assert (v (mk_u32 1 <<! v_N <: u32) == pow2 (v v_N))
#pop-options

(* op-fact: per-lane result of e_vsliq_n_s32 (transparent -> Neon.vsliq).  NV
   foundation gives the ArmIV per-lane FunArray; under 0<v_N<32 the model's
   else-branch (u32 mask + shift) bridges to vsli_lane_i32 by the two scalar
   lemmas.  Consumers use concrete N in (0,32) so vsli_lane_i32 unfolds to the
   target form (a[i] &. arm_low_mask (v v_N)) |. (b[i] <<! v_N). *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vsliq_n_s32_lane (v_N: i32) (a b: t_e_int32x4_t) (i: nat{i < 4})
  : Lemma (requires v v_N > 0 /\ v v_N < 32)
          (ensures get_lane_i32x4 (e_vsliq_n_s32 v_N a b) i
                   == vsli_lane_i32 (get_lane_i32x4 a i) (get_lane_i32x4 b i) (v v_N))
          [SMTPat (get_lane_i32x4 (e_vsliq_n_s32 v_N a b) i)] =
  NV.lemma_vsliq_n_s32 v_N a b;
  lemma_vsli_mask_i32 v_N;
  lemma_vsli_shift_i32 (get_lane_i32x4 b i) v_N
#pop-options

(* ── vsliq_n_s64: 2-lane i64 analog.  Model mask has an extra v_N=63 special
   case (i64_MAX); arm_low_mask_i64 63 == mk_i64 (2^63-1) == i64_MAX, so the
   bridge splits on 63. ──────────────────────────────────────────────────────── *)
let vsli_lane_i64 (a b: i64) (n: nat) : i64 =
  if 0 < n && n < 64 then (a &. arm_low_mask_i64 n) |. (b <<! mk_i32 n) else b

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_vsli_shift_i64 (bv: i64) (v_N: i32) : Lemma
  (requires v v_N >= 0 /\ v v_N < 64)
  (ensures (cast ((cast (bv <: i64) <: u64) <<! (cast (v_N <: i32) <: u32) <: u64) <: i64)
           == (bv <<! v_N)) =
  let lhs : i64 = cast ((cast (bv <: i64) <: u64) <<! (cast (v_N <: i32) <: u32) <: u64) <: i64 in
  let rhs : i64 = bv <<! v_N in
  let aux (r: usize{v r < 64}) : Lemma (Int.get_bit lhs r == Int.get_bit rhs r) = () in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits lhs rhs
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_vsli_mask_i64 (v_N: i32) : Lemma
  (requires v v_N > 0 /\ v v_N < 64)
  (ensures ((if v_N =. mk_i32 63
             then Core_models.Num.impl_i64__MAX
             else (cast (Core_models.Num.impl_u64__wrapping_sub (mk_u64 1 <<! v_N <: u64) (mk_u64 1) <: u64) <: i64))
            == arm_low_mask_i64 (v v_N))) =
  if v_N =. mk_i32 63
  then assert_norm (pow2 63 - 1 == 9223372036854775807)
  else (FStar.Math.Lemmas.pow2_lt_compat 64 (v v_N);
        assert (v (mk_u64 1 <<! v_N <: u64) == pow2 (v v_N)))
#pop-options

#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vsliq_n_s64_lane (v_N: i32) (a b: t_e_int64x2_t) (i: nat{i < 2})
  : Lemma (requires v v_N > 0 /\ v v_N < 64)
          (ensures get_lane_i64x2 (e_vsliq_n_s64 v_N a b) i
                   == vsli_lane_i64 (get_lane_i64x2 a i) (get_lane_i64x2 b i) (v v_N))
          [SMTPat (get_lane_i64x2 (e_vsliq_n_s64 v_N a b) i)] =
  NV.lemma_vsliq_n_s64 v_N a b;
  lemma_vsli_mask_i64 v_N;
  lemma_vsli_shift_i64 (get_lane_i64x2 b i) v_N
#pop-options

(* ── vshlq_s16 byte-sign crux: the model's shift count s = sign-extend(low byte
   of b) relates to arm_sshl's Euclidean byte v (b %! 256). ─────────────────── *)
let arm_shl_count_i16 (b: i16) : i32 = cast (cast (cast (cast (b <: i16) <: u16) <: u8) <: i8) <: i32

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_arm_shl_count_i16 (b: i16) : Lemma
  (ensures v (arm_shl_count_i16 b)
           == (let su = v (b %! mk_i16 256) in if su < 128 then su else su - 256)) =
  let su = v (b %! mk_i16 256) in
  let byte : u8 = cast (cast (b <: i16) <: u16) <: u8 in
  assert (v byte == su);
  ()
#pop-options

(* left-shift-via-u16 equivalence (0<=s<16), mirrors lemma_vsli_shift. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_arm_sshl_left (a: i16) (s: i32) : Lemma
  (requires v s >= 0 /\ v s < 16)
  (ensures (cast ((cast (a <: i16) <: u16) <<! (cast (s <: i32) <: u32) <: u16) <: i16)
           == (a <<! mk_i32 (v s))) =
  let lhs : i16 = cast ((cast (a <: i16) <: u16) <<! (cast (s <: i32) <: u32) <: u16) <: i16 in
  let rhs : i16 = a <<! mk_i32 (v s) in
  let aux (r: usize{v r < 16}) : Lemma (Int.get_bit lhs r == Int.get_bit rhs r) = () in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits lhs rhs
#pop-options

(* arm_sshl_i16 (Euclidean byte-count form) == the model's per-lane vshlq_s16
   (sign-extended low byte + 4 branches).  Case-split via the byte-sign crux. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_arm_sshl_eq (a b: i16) : Lemma
  (ensures arm_sshl_i16 a b ==
           (let s = arm_shl_count_i16 b in
            if s >=. mk_i32 16 then mk_i16 0
            else if s >=. mk_i32 0
                 then cast ((cast (a <: i16) <: u16) <<! (cast (s <: i32) <: u32) <: u16) <: i16
                 else if s <=. mk_i32 (-16)
                      then (if a <. mk_i16 0 then mk_i16 (-1) else mk_i16 0)
                      else a >>! (cast (Rust_primitives.Arithmetic.neg s <: i32) <: u32))) =
  lemma_arm_shl_count_i16 b;
  let s = arm_shl_count_i16 b in
  let su = v (b %! mk_i16 256) in
  if su < 16 then lemma_arm_sshl_left a s
  else if su < 128 then ()
  else ()
#pop-options

(* op-fact: per-lane e_vshlq_s16 == arm_sshl_i16 (NV foundation + arm_sshl_eq). *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vshlq_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (ensures get_lane_i16x8 (e_vshlq_s16 a b) i
                   == arm_sshl_i16 (get_lane_i16x8 a i) (get_lane_i16x8 b i))
          [SMTPat (get_lane_i16x8 (e_vshlq_s16 a b) i)] =
  NV.lemma_vshlq_s16 a b;
  lemma_arm_sshl_eq (get_lane_i16x8 a i) (get_lane_i16x8 b i)
#pop-options

(* unsigned analog: arm_ushl_u16 == model vshlq_u16 per-lane (sign-fill -> 0,
   logical right shift; no u16-cast-back on left since a is already u16). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_arm_ushl_eq (a: u16) (b: i16) : Lemma
  (ensures arm_ushl_u16 a b ==
           (let s = arm_shl_count_i16 b in
            if s >=. mk_i32 16 then mk_u16 0
            else if s >=. mk_i32 0
                 then (a <<! (cast (s <: i32) <: u32) <: u16)
                 else if s <=. mk_i32 (-16)
                      then mk_u16 0
                      else a >>! (cast (Rust_primitives.Arithmetic.neg s <: i32) <: u32))) =
  lemma_arm_shl_count_i16 b
#pop-options

#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vshlq_u16_lane (a: t_e_uint16x8_t) (b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (ensures get_lane_u16x8 (e_vshlq_u16 a b) i
                   == arm_ushl_u16 (get_lane_u16x8 a i) (get_lane_i16x8 b i))
          [SMTPat (get_lane_u16x8 (e_vshlq_u16 a b) i)] =
  NV.lemma_vshlq_u16 a b;
  lemma_arm_ushl_eq (get_lane_u16x8 a i) (get_lane_i16x8 b i)
#pop-options

(* ── vaddvq_s16 / vaddv_u16 (horizontal add).  Model is a
   `fold_range 0 N` LEFT fold of `impl_iN__wrapping_add` (== `+.` == add_mod) from
   0; consumer wants a balanced tree.  Three routes SATURATE (fuel-unroll, step
   lemma, unroll-lemma — all hit the fold_range CLOSURE-INEQUALITY / heavy-context
   wall).  ✅ CRACKED by the load-recipe `norm`/`trefl` closure-eq trick: reduce
   the fold DEFINITIONALLY to a ground left-fold (`lemma_arm_vaddv*_unroll`), then
   offload the add_mod AC into a CLEAN-context standalone lemma (`lemma_add{8,4}_ac`
   — the fold/tactic-laden op-fact context saturates on the AC; isolated it proves).
   NV foundation: `NV.lemma_vaddvq_s16` / `NV.lemma_vaddv_u16`. ────────────────── *)

(* ── vaddvq_s16 via norm/trefl (the load-recipe closure-eq trick): reduce the
   fold_range LEFT fold definitionally to a GROUND 8-term expression — sidesteps
   the closure inequality that saturates the fuel-unroll and unroll-lemma routes.
   Then add_mod AC on the ground expr (impl_i16__wrapping_add == (+.)). ────────── *)
unfold let arm_vaddvq_s16_laf (a: Funarr.t_FunArray (mk_u64 8) i16) : i16 =
  let wa = Core_models.Num.impl_i16__wrapping_add in
  wa (wa (wa (wa (wa (wa (wa (wa (mk_i16 0) (a.[ mk_u64 0 ] <: i16)) (a.[ mk_u64 1 ] <: i16))
    (a.[ mk_u64 2 ] <: i16)) (a.[ mk_u64 3 ] <: i16)) (a.[ mk_u64 4 ] <: i16))
    (a.[ mk_u64 5 ] <: i16)) (a.[ mk_u64 6 ] <: i16)) (a.[ mk_u64 7 ] <: i16)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 50"
let lemma_arm_vaddvq_s16_unroll (a: Funarr.t_FunArray (mk_u64 8) i16)
  : Lemma (ArmIV.vaddvq_s16 a == arm_vaddvq_s16_laf a) =
  assert (ArmIV.vaddvq_s16 a == arm_vaddvq_s16_laf a)
    by (FStar.Tactics.norm [delta_only [`%ArmIV.vaddvq_s16;
                                        `%Rust_primitives.Hax.Folds.fold_range];
                            iota; zeta; primops];
        FStar.Tactics.trefl ())
#pop-options

(* ── add_mod (i16 `+.`) associativity, DETERMINISTIC ────────────────────────
   A bare `= ()` proof of the 8-term `+.` AC below drives a nonlinear/modular
   SMT search whose recorded unsat-core does NOT replay (a slow/flaky query
   banks a lucky Z3 path).  We restructure to fast-stable: prove `@%`
   add-distributivity from FStar.Math.Lemmas once, lift to `add_mod` assoc /
   left-zero, then re-associate the left fold into the balanced tree by a 5-step
   `calc` — pure ground re-association, zero Z3 modular search. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 60"

(* `@%` (wrap into [-p/2,p/2[) depends only on the residue mod p *)
let lemma_atp_cong (a b: int) (p: pos{p % 2 = 0})
  : Lemma (requires a % p == b % p) (ensures a @% p == b @% p) = ()

(* `a @% p` is ≡ a  (mod p) *)
let lemma_atp_mod (a: int) (p: pos{p % 2 = 0})
  : Lemma ((a @% p) % p == a % p) =
  let m = a % p in
  FStar.Math.Lemmas.lemma_mod_twice a p;
  if m >= p / 2 then FStar.Math.Lemmas.lemma_mod_plus m (-1) p else ()

(* left / right `@%` add-distributivity *)
let lemma_atp_add_l (a b: int) (p: pos{p % 2 = 0})
  : Lemma (((a @% p) + b) @% p == (a + b) @% p) =
  lemma_atp_mod a p;
  FStar.Math.Lemmas.lemma_mod_plus_distr_l (a @% p) b p;
  FStar.Math.Lemmas.lemma_mod_plus_distr_l a b p;
  lemma_atp_cong ((a @% p) + b) (a + b) p

let lemma_atp_add_r (a b: int) (p: pos{p % 2 = 0})
  : Lemma ((a + (b @% p)) @% p == (a + b) @% p) =
  lemma_atp_mod b p;
  FStar.Math.Lemmas.lemma_mod_add_distr a (b @% p) p;
  FStar.Math.Lemmas.lemma_mod_add_distr a b p;
  lemma_atp_cong (a + (b @% p)) (a + b) p

(* bridge the i16 wrap `@%. I16` to `@% (pow2 16)` *)
let lemma_atpd_i16 (x: int) : Lemma ((x @%. I16) == x @% (pow2 16)) = ()

(* `@% (pow2 16)` is the identity on the i16 range *)
let lemma_atp_id_i16 (y: int{range y I16}) : Lemma ((y @% (pow2 16)) == y) =
  assert_norm (pow2 16 == 65536);
  assert_norm (pow2 15 == 32768);
  if y >= 0 then FStar.Math.Lemmas.small_mod y (pow2 16)
  else begin
    FStar.Math.Lemmas.lemma_mod_sub_1 (- y) (pow2 16);
    FStar.Math.Lemmas.small_mod (- y) (pow2 16)
  end

(* `add_mod` (i16 `+.`) left-zero and associativity *)
let lemma_add_mod_zero_l (x: i16) : Lemma (mk_i16 0 +. x == x) =
  let vx = v x in
  lemma_atpd_i16 (v (mk_i16 0) + vx);
  lemma_atpd_i16 vx;
  lemma_atp_id_i16 vx

let lemma_add_mod_assoc (a b c: i16) : Lemma ((a +. b) +. c == a +. (b +. c)) =
  let va = v a in let vb = v b in let vc = v c in
  lemma_atpd_i16 (va + vb);
  lemma_atpd_i16 (vb + vc);
  lemma_atpd_i16 (((va + vb) @%. I16) + vc);
  lemma_atpd_i16 (va + ((vb + vc) @%. I16));
  lemma_atp_add_l (va + vb) vc (pow2 16);
  lemma_atp_add_r va (vb + vc) (pow2 16)
#pop-options

(* 8-term add_mod AC (left-fold from 0 == balanced tree), in CLEAN context.
   Deterministic: drop the leading 0, then re-associate the left fold into the
   balanced tree via `lemma_add_mod_assoc` (each step a ground congruence). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 40"
let lemma_add8_ac (x0 x1 x2 x3 x4 x5 x6 x7: i16) : Lemma
  (ensures ((((((((mk_i16 0 +. x0) +. x1) +. x2) +. x3) +. x4) +. x5) +. x6) +. x7)
           == (((x0 +. x1) +. (x2 +. x3)) +. ((x4 +. x5) +. (x6 +. x7)))) =
  calc (==) {
    ((((((((mk_i16 0 +. x0) +. x1) +. x2) +. x3) +. x4) +. x5) +. x6) +. x7);
    == { lemma_add_mod_zero_l x0 }
    (((((((x0 +. x1) +. x2) +. x3) +. x4) +. x5) +. x6) +. x7);
    == { lemma_add_mod_assoc (x0 +. x1) x2 x3 }
    ((((((x0 +. x1) +. (x2 +. x3)) +. x4) +. x5) +. x6) +. x7);
    == { lemma_add_mod_assoc ((x0 +. x1) +. (x2 +. x3)) x4 x5 }
    (((((x0 +. x1) +. (x2 +. x3)) +. (x4 +. x5)) +. x6) +. x7);
    == { lemma_add_mod_assoc (((x0 +. x1) +. (x2 +. x3)) +. (x4 +. x5)) x6 x7 }
    ((((x0 +. x1) +. (x2 +. x3)) +. (x4 +. x5)) +. (x6 +. x7));
    == { lemma_add_mod_assoc ((x0 +. x1) +. (x2 +. x3)) (x4 +. x5) (x6 +. x7) }
    (((x0 +. x1) +. (x2 +. x3)) +. ((x4 +. x5) +. (x6 +. x7)));
  }
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_vaddvq_s16 (a: t_e_int16x8_t)
  : Lemma (ensures e_vaddvq_s16 a ==
             (((get_lane_i16x8 a 0 +. get_lane_i16x8 a 1) +. (get_lane_i16x8 a 2 +. get_lane_i16x8 a 3)) +.
              ((get_lane_i16x8 a 4 +. get_lane_i16x8 a 5) +. (get_lane_i16x8 a 6 +. get_lane_i16x8 a 7))))
          [SMTPat (e_vaddvq_s16 a)] =
  NV.lemma_vaddvq_s16 a;
  lemma_arm_vaddvq_s16_unroll (Canon.to_i16x8 a);
  lemma_add8_ac (get_lane_i16x8 a 0) (get_lane_i16x8 a 1) (get_lane_i16x8 a 2) (get_lane_i16x8 a 3)
                (get_lane_i16x8 a 4) (get_lane_i16x8 a 5) (get_lane_i16x8 a 6) (get_lane_i16x8 a 7)
#pop-options

(* vaddv_u16 (4-lane) — same norm/trefl closure-eq + clean AC recipe as vaddvq_s16. *)
unfold let arm_vaddv_u16_laf (a: Funarr.t_FunArray (mk_u64 4) u16) : u16 =
  let wa = Core_models.Num.impl_u16__wrapping_add in
  wa (wa (wa (wa (mk_u16 0) (a.[ mk_u64 0 ] <: u16)) (a.[ mk_u64 1 ] <: u16))
    (a.[ mk_u64 2 ] <: u16)) (a.[ mk_u64 3 ] <: u16)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 50"
let lemma_arm_vaddv_u16_unroll (a: Funarr.t_FunArray (mk_u64 4) u16)
  : Lemma (ArmIV.vaddv_u16 a == arm_vaddv_u16_laf a) =
  assert (ArmIV.vaddv_u16 a == arm_vaddv_u16_laf a)
    by (FStar.Tactics.norm [delta_only [`%ArmIV.vaddv_u16;
                                        `%Rust_primitives.Hax.Folds.fold_range];
                            iota; zeta; primops];
        FStar.Tactics.trefl ())
#pop-options

#push-options "--fuel 0 --ifuel 0 --z3rlimit 300"
let lemma_add4_ac (x0 x1 x2 x3: u16) : Lemma
  (ensures ((((mk_u16 0 +. x0) +. x1) +. x2) +. x3) == ((x0 +. x1) +. (x2 +. x3))) = ()
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_vaddv_u16 (a: t_e_uint16x4_t)
  : Lemma (ensures e_vaddv_u16 a ==
             ((get_lane_u16x4 a 0 +. get_lane_u16x4 a 1) +. (get_lane_u16x4 a 2 +. get_lane_u16x4 a 3)))
          [SMTPat (e_vaddv_u16 a)] =
  NV.lemma_vaddv_u16 a;
  lemma_arm_vaddv_u16_unroll (NV.to_u16x4 a);
  lemma_add4_ac (get_lane_u16x4 a 0) (get_lane_u16x4 a 1) (get_lane_u16x4 a 2) (get_lane_u16x4 a 3)
#pop-options

(* ── vmlal_s16 / _high_s16 (widening multiply-accumulate) op-facts.  Foundation
   NV.lemma_vmlal_s16 lives in core-models Neon_views (proven twin of
   NV.lemma_vmull_s16); the op-fact mirrors lemma_e_vmull_s16 exactly (direct
   Seq.lemma_eq_intro).  impl_i32__wrapping_add == (+.); cast b *! cast c ==
   cast b *. cast c (i16*i16 fits i32) — the same bridges vmull's op-fact does. *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_vmlal_s16 (a: t_e_int32x4_t) (b c: t_e_int16x4_t)
  : Lemma (vec128_as_i32x4 (e_vmlal_s16 a b c)
           == Seq.init 4 (fun i -> Seq.index (vec128_as_i32x4 a) i +.
                               ((cast (Seq.index (vec64_as_i16x4 b) i) <: i32)
                             *. (cast (Seq.index (vec64_as_i16x4 c) i) <: i32))))
          [SMTPat (vec128_as_i32x4 (e_vmlal_s16 a b c))] =
  NV.lemma_vmlal_s16 a b c;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vmlal_s16 a b c))
                     (Seq.init 4 (fun i -> Seq.index (vec128_as_i32x4 a) i +.
                                       ((cast (Seq.index (vec64_as_i16x4 b) i) <: i32)
                                     *. (cast (Seq.index (vec64_as_i16x4 c) i) <: i32))))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_vmlal_high_s16 (a: t_e_int32x4_t) (b c: t_e_int16x8_t)
  : Lemma (vec128_as_i32x4 (e_vmlal_high_s16 a b c)
           == Seq.init 4 (fun i -> Seq.index (vec128_as_i32x4 a) i +.
                               ((cast (Seq.index (vec128_as_i16x8 b) (i + 4)) <: i32)
                             *. (cast (Seq.index (vec128_as_i16x8 c) (i + 4)) <: i32))))
          [SMTPat (vec128_as_i32x4 (e_vmlal_high_s16 a b c))] =
  NV.lemma_vmlal_high_s16 a b c;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vmlal_high_s16 a b c))
                     (Seq.init 4 (fun i -> Seq.index (vec128_as_i32x4 a) i +.
                                       ((cast (Seq.index (vec128_as_i16x8 b) (i + 4)) <: i32)
                                     *. (cast (Seq.index (vec128_as_i16x8 c) (i + 4)) <: i32))))
#pop-options

(* ══ codec bridge: i16 <-> u16 same-width lane views (PROVEN, zero new axioms) ══
   The per-lane fact (`get_lane_i16x8 result i == cast_mod (get_lane_u16x8 m0 i)`),
   proven from the shared codec: the signed (i16) and unsigned (u16) 16-bit
   lane views of the SAME 128-bit vec share `lane_reader` (bits I16 = bits U16 =
   16); only `decode_lane`'s signedness differs, so each i16 lane is the
   two's-complement (`cast_mod`) reinterpret of the u16 lane.  Consumed by NEON
   proofs that build a mask via `reinterpret_s16_u16 (vcgeq_s16 ..)` — e.g.
   `Vector.Neon.Arithmetic.cond_subtract_3329_`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_decode_i16_u16 (f: (nat -> Bit.t_Bit))
  : Lemma (IVi.decode_lane Rust_primitives.Integers.I16 f
           == Rust_primitives.Integers.cast_mod
                #Rust_primitives.Integers.u16_inttype
                #Rust_primitives.Integers.i16_inttype
                (IVi.decode_lane Rust_primitives.Integers.U16 f)) =
  IVi.dsum2_bound f 0 16
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_i16_u16_lane_cast (bv: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_i16x8 bv) i
           == Rust_primitives.Integers.cast_mod
                #Rust_primitives.Integers.u16_inttype
                #Rust_primitives.Integers.i16_inttype
                (Seq.index (vec128_as_u16x8 bv) i)) =
  reveal_opaque (`%IVi.to_iv) IVi.to_iv;
  lemma_decode_i16_u16 (IVi.lane_reader (mk_u64 128) 16 bv (mk_u64 i))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_s16_u16_lane (m0: t_e_uint16x8_t) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_i16x8 (e_vreinterpretq_s16_u16 m0)) i
           == Rust_primitives.Integers.cast_mod
                #Rust_primitives.Integers.u16_inttype
                #Rust_primitives.Integers.i16_inttype
                (Seq.index (vec128_as_u16x8 m0) i))
          [SMTPat (Seq.index (vec128_as_i16x8 (e_vreinterpretq_s16_u16 m0)) i)] =
  lemma_e_vreinterpretq_s16_u16 m0;
  lemma_i16_u16_lane_cast m0 i
#pop-options

(* the inverse direction (u16 view == cast_mod of i16 view), for consumers that
   round-trip i16 -> u16 (`reinterpret_u16_s16`) — e.g. the montgomery-factor
   compute in `montgomery_reduce_int16x8_t` (`reinterpret_s16_u16 (vmulq_n_u16
   (reinterpret_u16_s16 low) c)`). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_decode_u16_i16 (f: (nat -> Bit.t_Bit))
  : Lemma (IVi.decode_lane Rust_primitives.Integers.U16 f
           == Rust_primitives.Integers.cast_mod
                #Rust_primitives.Integers.i16_inttype
                #Rust_primitives.Integers.u16_inttype
                (IVi.decode_lane Rust_primitives.Integers.I16 f)) =
  IVi.dsum2_bound f 0 16
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_u16_i16_lane_cast (bv: BV.t_BitVec (mk_u64 128)) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_u16x8 bv) i
           == Rust_primitives.Integers.cast_mod
                #Rust_primitives.Integers.i16_inttype
                #Rust_primitives.Integers.u16_inttype
                (Seq.index (vec128_as_i16x8 bv) i)) =
  reveal_opaque (`%IVi.to_iv) IVi.to_iv;
  lemma_decode_u16_i16 (IVi.lane_reader (mk_u64 128) 16 bv (mk_u64 i))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_e_vreinterpretq_u16_s16_lane (m0: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_u16x8 (e_vreinterpretq_u16_s16 m0)) i
           == Rust_primitives.Integers.cast_mod
                #Rust_primitives.Integers.i16_inttype
                #Rust_primitives.Integers.u16_inttype
                (Seq.index (vec128_as_i16x8 m0) i))
          [SMTPat (Seq.index (vec128_as_u16x8 (e_vreinterpretq_u16_s16 m0)) i)] =
  lemma_e_vreinterpretq_u16_s16 m0;
  lemma_u16_i16_lane_cast m0 i
#pop-options

(* ── per-lane get_lane facts for the whole-vector arith/logical/transpose ops ──
   The op-facts are WHOLE-vector (`vec128_as_* (op) == map2 OP ..`).  Per-lane consumers
   (the hand-written NEON theory companions) index those at a symbolic lane.  These
   lemmas expose the clean per-lane fact.  NO SMTPat — a global `get_lane (op) i`
   pattern compounds with the whole-vector SMTPat + the `map2`/`createi` index into
   an e-matching cascade (measured: 31-min grind on `Serialize_theory`).  Consumers
   call these EXPLICITLY, scoped to the lane they need. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_e_vaddq_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vaddq_s16 a b) i == (get_lane_i16x8 a i +. get_lane_i16x8 b i))
  = lemma_e_vaddq_s16 a b
let lemma_e_vsubq_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vsubq_s16 a b) i == (get_lane_i16x8 a i -. get_lane_i16x8 b i))
  = lemma_e_vsubq_s16 a b
let lemma_e_vmulq_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vmulq_s16 a b) i == mul_mod (get_lane_i16x8 a i) (get_lane_i16x8 b i))
  = lemma_e_vmulq_s16 a b
let lemma_e_vmulq_n_s16_lane (vv: t_e_int16x8_t) (c: i16) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vmulq_n_s16 vv c) i == (get_lane_i16x8 vv i *. c))
  = lemma_e_vmulq_n_s16 vv c
let lemma_e_vandq_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vandq_s16 a b) i == (get_lane_i16x8 a i &. get_lane_i16x8 b i))
  = lemma_e_vandq_s16 a b
let lemma_e_veorq_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_veorq_s16 a b) i == (get_lane_i16x8 a i ^. get_lane_i16x8 b i))
  = lemma_e_veorq_s16 a b
let lemma_e_vdupq_n_s16_lane (c: i16) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vdupq_n_s16 c) i == c)
  = lemma_e_vdupq_n_s16 c
let lemma_e_vtrn1q_s32_lane (a b: t_e_int32x4_t) (i: nat{i < 4})
  : Lemma (get_lane_i32x4 (e_vtrn1q_s32 a b) i
           == (if i % 2 = 0 then get_lane_i32x4 a i else get_lane_i32x4 b (i - 1)))
  = lemma_e_vtrn1q_s32 a b
let lemma_e_vtrn2q_s32_lane (a b: t_e_int32x4_t) (i: nat{i < 4})
  : Lemma (get_lane_i32x4 (e_vtrn2q_s32 a b) i
           == (if i % 2 = 0 then get_lane_i32x4 a (i + 1) else get_lane_i32x4 b i))
  = lemma_e_vtrn2q_s32 a b
let lemma_e_vtrn1q_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vtrn1q_s16 a b) i
           == (if i % 2 = 0 then get_lane_i16x8 a i else get_lane_i16x8 b (i - 1)))
  = lemma_e_vtrn1q_s16 a b
let lemma_e_vtrn2q_s16_lane (a b: t_e_int16x8_t) (i: nat{i < 8})
  : Lemma (get_lane_i16x8 (e_vtrn2q_s16 a b) i
           == (if i % 2 = 0 then get_lane_i16x8 a (i + 1) else get_lane_i16x8 b i))
  = lemma_e_vtrn2q_s16 a b
let lemma_e_vandq_u32_lane (a b: t_e_uint32x4_t) (i: nat{i < 4})
  : Lemma (get_lane_u32x4 (e_vandq_u32 a b) i == (get_lane_u32x4 a i &. get_lane_u32x4 b i))
  = lemma_e_vandq_u32 a b
let lemma_e_vaddq_u32_lane (a b: t_e_uint32x4_t) (i: nat{i < 4})
  : Lemma (get_lane_u32x4 (e_vaddq_u32 a b) i == (get_lane_u32x4 a i +. get_lane_u32x4 b i))
  = lemma_e_vaddq_u32 a b
let lemma_e_vmulq_n_u32_lane (a: t_e_uint32x4_t) (c: u32) (i: nat{i < 4})
  : Lemma (get_lane_u32x4 (e_vmulq_n_u32 a c) i == (get_lane_u32x4 a i *. c))
  = lemma_e_vmulq_n_u32 a c
let lemma_e_vdupq_n_u32_lane (c: u32) (i: nat{i < 4})
  : Lemma (get_lane_u32x4 (e_vdupq_n_u32 c) i == c)
  = lemma_e_vdupq_n_u32 c
#pop-options

(* ── TWO-WRITE byte-store framing glue (to_bytes) ────────────────────────────
   NEON analog of `Avx2.Byteperm_theory.lemma_store_glue_two_writes`, specialised
   to the two [0:16] / [16:32] writes of `to_bytes` and generalised to a `t_Slice
   u8` target (length >= 32, vs the AVX2 fixed length-32 array).  Threads the two
   `update_at_range` frames so the per-byte bit of `head = Seq.slice fin 0 32`
   comes from `o1` (i<128) or `o2` (i>=128); the caller discharges the two `o_k`
   bit-facts with `lemma_e_vst1q_bytes`.  Factoring the per-byte cross-product
   into this clean-context lemma keeps it OUT of the consumer's WP. *)
(* Per-byte bit fact for the two-write store, in CLEAN context — mirror of
   `Avx2.Byteperm_theory.lemma_store_glue_bits`.  The caller supplies the four
   frame foralls, so `update_at_range` and its unbounded forall never enter THIS
   query; the heavy `bit_vec_of_int_t_array` unfold for a symbolic `i` is proven
   here ONCE, keeping it OUT of the `forall_intro` spine below (which would
   otherwise swallow the whole per-byte cross-product and re-prove it, slow and
   with a hint that would not replay). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_store_glue_bits_neon
      (fin: t_Slice u8) (head: t_Array u8 (sz 32))
      (o1 o2: t_Slice u8)
      (lo hi: t_e_int16x8_t)
      (i: nat{i < 256})
  : Lemma
      (requires
        Seq.length fin >= 32 /\ Seq.length o1 == 16 /\ Seq.length o2 == 16 /\
        head == Seq.slice fin 0 32 /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array (o1 <: t_Array u8 (sz 16)) 8 j == bv_bit lo j) /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array (o2 <: t_Array u8 (sz 16)) 8 j == bv_bit hi j) /\
        (forall (k: nat). k < 16 ==> Seq.index fin k == Seq.index o1 k) /\
        (forall (k: nat). 16 <= k /\ k < 32 ==> Seq.index fin k == Seq.index o2 (k - 16)))
      (ensures
        Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i ==
        (if i < 128 then bv_bit lo i else bv_bit hi (i - 128))) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  Seq.lemma_index_slice fin 0 32 (i / 8);
  if i < 128 then begin
    FStar.Math.Lemmas.lemma_div_lt_nat i 128 8;
    assert (Seq.index head (i / 8) == Seq.index o1 (i / 8));
    assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array (o1 <: t_Array u8 (sz 16)) 8 i == bv_bit lo i)
  end
  else begin
    let j = i - 128 in
    FStar.Math.Lemmas.lemma_div_plus j 16 8;
    FStar.Math.Lemmas.lemma_mod_plus j 16 8;
    FStar.Math.Lemmas.lemma_div_lt_nat j 128 8;
    assert (Seq.index head (i / 8) == Seq.index o2 (j / 8));
    assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array (o2 <: t_Array u8 (sz 16)) 8 j == bv_bit hi j)
  end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_store_glue_two_writes_neon
      (ser0 ser1 fin: t_Slice u8)
      (o1 o2: t_Slice u8)
      (lo hi: t_e_int16x8_t)
      (r0 r1: Core_models.Ops.Range.t_Range usize)
  : Lemma
      (requires
        Seq.length ser0 >= 32 /\
        v r0.Core_models.Ops.Range.f_start == 0  /\ v r0.Core_models.Ops.Range.f_end == 16 /\
        v r1.Core_models.Ops.Range.f_start == 16 /\ v r1.Core_models.Ops.Range.f_end == 32 /\
        Seq.length o1 == 16 /\ Seq.length o2 == 16 /\
        ser1 == Rust_primitives.Hax.Monomorphized_update_at.update_at_range ser0 r0 o1 /\
        fin  == Rust_primitives.Hax.Monomorphized_update_at.update_at_range ser1 r1 o2 /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array (o1 <: t_Array u8 (sz 16)) 8 j == bv_bit lo j) /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array (o2 <: t_Array u8 (sz 16)) 8 j == bv_bit hi j))
      (ensures
        (let head : t_Array u8 (sz 32) = Seq.slice fin 0 32 in
         forall (i: nat{i < 256}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i ==
           (if i < 128 then bv_bit lo i else bv_bit hi (i - 128)))) =
  Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range ser0 r0 o1;
  Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range ser1 r1 o2;
  assert (Seq.length fin == Seq.length ser0);
  assert (forall (k: nat). k < 16 ==> Seq.index fin k == Seq.index o1 k);
  assert (forall (k: nat). 16 <= k /\ k < 32 ==> Seq.index fin k == Seq.index o2 (k - 16));
  let head : t_Array u8 (sz 32) = Seq.slice fin 0 32 in
  let aux (i: nat{i < 256})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i ==
             (if i < 128 then bv_bit lo i else bv_bit hi (i - 128))) =
    lemma_store_glue_bits_neon fin head o1 o2 lo hi i
  in
  FStar.Classical.forall_intro aux
#pop-options

(* ── i32x4 saturating doubling multiply-high (vqdmulh) — the s32 op-fact for
   `e_vqdmulhq_n_s32` (the s16 forms exist elsewhere).  Used by `Neon.Compress`
   `cmp_compress_u32_lane`.  Mirrors `lemma_e_vqdmulhq_n_s16` at the i32x4 width
   (i32 product widens to i64, shift 31, clamp to i32 MAX/MIN). ──────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let lemma_e_vqdmulhq_n_s32 (a: t_e_int32x4_t) (b: i32)
  : Lemma (vec128_as_i32x4 (e_vqdmulhq_n_s32 a b)
           == Seq.init 4 (fun i ->
                let prod = ((cast (Seq.index (vec128_as_i32x4 a) i) <: i64) *. (cast b <: i64)) >>! (mk_i32 31) in
                if prod >. mk_i64 2147483647 then mk_i32 2147483647
                else if prod <. mk_i64 (- 2147483648) then mk_i32 (- 2147483648) else (cast prod <: i32))) =
  NV.lemma_vqdmulhq_n_s32 a b;
  Seq.lemma_eq_intro (vec128_as_i32x4 (e_vqdmulhq_n_s32 a b))
                     (Seq.init 4 (fun i ->
                        let prod = ((cast (Seq.index (vec128_as_i32x4 a) i) <: i64) *. (cast b <: i64)) >>! (mk_i32 31) in
                        if prod >. mk_i64 2147483647 then mk_i32 2147483647
                        else if prod <. mk_i64 (- 2147483648) then mk_i32 (- 2147483648) else (cast prod <: i32)))
#pop-options

(* per-lane form (`get_lane_i32x4`); this is what
   `cmp_compress_u32_lane` consumes.  SMTPat is narrow — `e_vqdmulhq_n_s32` is
   used only in Compress. *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let lemma_e_vqdmulhq_n_s32_lane (a: t_e_int32x4_t) (b: i32) (i: nat{i < 4})
  : Lemma (get_lane_i32x4 (e_vqdmulhq_n_s32 a b) i ==
           (let prod = ((cast (get_lane_i32x4 a i) <: i64) *. (cast b <: i64)) >>! (mk_i32 31) in
            if prod >. mk_i64 2147483647 then mk_i32 2147483647
            else if prod <. mk_i64 (- 2147483648) then mk_i32 (- 2147483648) else (cast prod <: i32)))
          [SMTPat (get_lane_i32x4 (e_vqdmulhq_n_s32 a b) i)] =
  lemma_e_vqdmulhq_n_s32 a b
#pop-options
