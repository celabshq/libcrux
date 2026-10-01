module Libcrux_core_models.Neon_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 30"
open FStar.Mul
open Core_models

(* ============================================================================
   CANONICAL NEON lane-view op-lemma companion.

   The ARM/NEON analog of the x86 op-lemma set in
   `Libcrux_core_models.Intrinsics_views`.  It rests entirely on the
   differentially-tested `core-models` NEON model: each op-lemma

     `to_OUT (Neon.OP a b) == ArmIV.OP (to_IN a) (to_IN b)`

   is PROVEN by the same uniform reduction the x86 side uses:
       lift lemma  (Arm.Interpretations.Int_vec.Lemmas, differentially-tested
                    [@@ v_LIFT_LEMMA])
     + round-trip  (Int_vec_interp.lemma_conv_rt, PROVEN)
     + Int_vec def (definitional).

   The lane VIEW is the SAME shared core-models codec `Int_vec_interp.to_iWxL`
   (width 128 / 64) that `Intrinsics_views` already re-exports — NO new axiom;
   the missing unsigned / half-vector `rt_*` instances of the PROVEN
   `lemma_conv_rt` are added below.

   This module `open`s `Intrinsics_views` (as `Canon`) to reuse its codec views
   and round-trips.  It contains ONLY proven lemmas; every axiom it rests on
   lives in `Libcrux_core_models.Trusted.Intrinsics` /
   `Arm.Interpretations.Int_vec.Lemmas`.  Consumers (ml-kem / sha3 / aes NEON
   view companions) prove their per-lane `get_lane_*` facts on top of these,
   exactly as the x86 `Avx2_ml_kem_views` does over `Intrinsics_views`.

   NOTE (SMTPat): op-lemmas are exposed WITHOUT SMTPat — this foundational module
   stays cascade-free; consumers call them explicitly.
   ============================================================================ *)

module BV     = Libcrux_core_models.Abstractions.Bitvec
module Funarr = Libcrux_core_models.Abstractions.Funarr
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Int    = Rust_primitives.Integers
module Canon  = Libcrux_core_models.Intrinsics_views
module Neon   = Libcrux_core_models.Core_arch.Arm.Neon
module ArmHW  = Libcrux_core_models.Core_arch.Arm.Neon_handwritten
module ArmIV  = Libcrux_core_models.Core_arch.Arm.Interpretations.Int_vec
module ArmL   = Libcrux_core_models.Core_arch.Arm.Interpretations.Int_vec.Lemmas

unfold let bv128 = BV.t_BitVec (mk_u64 128)
unfold let bv64  = BV.t_BitVec (mk_u64 64)

(* ── codec views + round-trips NEON needs beyond `Intrinsics_views`
      (which already re-exports to_i16x8/to_i32x4/to_i64x2 + their rt).  All are
      the SAME shared `Int_vec_interp` codec; each `rt_*` is one instance of the
      PROVEN generic `IVi.lemma_conv_rt`. ─────────────────────────────────────── *)

let to_u32x4    = IVi.e_ee_14__impl__to_u32x4
let from_u32x4  = IVi.e_ee_14__impl__from_u32x4
let rt_u32x4 (y: Funarr.t_FunArray (mk_u64 4) u32) : Lemma (to_u32x4 (from_u32x4 y) == y) =
  IVi.lemma_conv_rt Int.U32 (mk_u64 128) (mk_u64 4) y

let to_u64x2    = IVi.e_ee_15__impl__to_u64x2
let from_u64x2  = IVi.e_ee_15__impl__from_u64x2
let rt_u64x2 (y: Funarr.t_FunArray (mk_u64 2) u64) : Lemma (to_u64x2 (from_u64x2 y) == y) =
  IVi.lemma_conv_rt Int.U64 (mk_u64 128) (mk_u64 2) y

let to_u16x8    = IVi.e_ee_16__impl__to_u16x8
let from_u16x8  = IVi.e_ee_16__impl__from_u16x8
let rt_u16x8 (y: Funarr.t_FunArray (mk_u64 8) u16) : Lemma (to_u16x8 (from_u16x8 y) == y) =
  IVi.lemma_conv_rt Int.U16 (mk_u64 128) (mk_u64 8) y

let to_u8x16    = IVi.e_ee_17__impl__to_u8x16
let from_u8x16  = IVi.e_ee_17__impl__from_u8x16
let rt_u8x16 (y: Funarr.t_FunArray (mk_u64 16) u8) : Lemma (to_u8x16 (from_u8x16 y) == y) =
  IVi.lemma_conv_rt Int.U8 (mk_u64 128) (mk_u64 16) y

let to_i16x4    = IVi.e_ee_19__impl__to_i16x4
let from_i16x4  = IVi.e_ee_19__impl__from_i16x4
let rt_i16x4 (y: Funarr.t_FunArray (mk_u64 4) i16) : Lemma (to_i16x4 (from_i16x4 y) == y) =
  IVi.lemma_conv_rt Int.I16 (mk_u64 64) (mk_u64 4) y

let to_u16x4    = IVi.e_ee_23__impl__to_u16x4
let from_u16x4  = IVi.e_ee_23__impl__from_u16x4
let rt_u16x4 (y: Funarr.t_FunArray (mk_u64 4) u16) : Lemma (to_u16x4 (from_u16x4 y) == y) =
  IVi.lemma_conv_rt Int.U16 (mk_u64 64) (mk_u64 4) y

(* shorthands for the shared codecs re-exported by Intrinsics_views *)
unfold let to_i16x8  = Canon.to_i16x8
unfold let to_i32x4  = Canon.to_i32x4
unfold let to_i64x2  = Canon.to_i64x2
unfold let rt_i16x8  = Canon.rt_i16x8
unfold let rt_i32x4  = Canon.rt_i32x4
unfold let rt_i64x2  = Canon.rt_i64x2

(* ============================================================================
   PROVEN op-lemmas.  Each = one lift + one round-trip (scalar-result / raw ops
   need only the lift).  The codec of each op is dictated by the corresponding
   `ArmL` lift lemma (mirror of `Intrinsics_views`).
   ============================================================================ *)

(* ── i16x8 arithmetic ─────────────────────────────────────────────────────── *)
let lemma_vaddq_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vaddq_s16 a b) == ArmIV.vaddq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vaddq_s16 a b; rt_i16x8 (ArmIV.vaddq_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vsubq_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vsubq_s16 a b) == ArmIV.vsubq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vsubq_s16 a b; rt_i16x8 (ArmIV.vsubq_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vmulq_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vmulq_s16 a b) == ArmIV.vmulq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vmulq_s16 a b; rt_i16x8 (ArmIV.vmulq_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vqdmulhq_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vqdmulhq_s16 a b) == ArmIV.vqdmulhq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vqdmulhq_s16 a b; rt_i16x8 (ArmIV.vqdmulhq_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vmulq_n_s16 (a: bv128) (b: i16)
  : Lemma (to_i16x8 (Neon.vmulq_n_s16 a b) == ArmIV.vmulq_n_s16 (to_i16x8 a) b) =
  ArmL.vmulq_n_s16 a b; rt_i16x8 (ArmIV.vmulq_n_s16 (to_i16x8 a) b)

let lemma_vqdmulhq_n_s16 (a: bv128) (b: i16)
  : Lemma (to_i16x8 (Neon.vqdmulhq_n_s16 a b) == ArmIV.vqdmulhq_n_s16 (to_i16x8 a) b) =
  ArmL.vqdmulhq_n_s16 a b; rt_i16x8 (ArmIV.vqdmulhq_n_s16 (to_i16x8 a) b)

let lemma_vmulq_n_u16 (a: bv128) (b: u16)
  : Lemma (to_u16x8 (Neon.vmulq_n_u16 a b) == ArmIV.vmulq_n_u16 (to_u16x8 a) b) =
  ArmL.vmulq_n_u16 a b; rt_u16x8 (ArmIV.vmulq_n_u16 (to_u16x8 a) b)

let lemma_vmulq_n_u32 (a: bv128) (b: u32)
  : Lemma (to_u32x4 (Neon.vmulq_n_u32 a b) == ArmIV.vmulq_n_u32 (to_u32x4 a) b) =
  ArmL.vmulq_n_u32 a b; rt_u32x4 (ArmIV.vmulq_n_u32 (to_u32x4 a) b)

let lemma_vqdmulhq_n_s32 (a: bv128) (b: i32)
  : Lemma (to_i32x4 (Neon.vqdmulhq_n_s32 a b) == ArmIV.vqdmulhq_n_s32 (to_i32x4 a) b) =
  ArmL.vqdmulhq_n_s32 a b; rt_i32x4 (ArmIV.vqdmulhq_n_s32 (to_i32x4 a) b)

let lemma_vaddq_u32 (a b: bv128)
  : Lemma (to_u32x4 (Neon.vaddq_u32 a b) == ArmIV.vaddq_u32 (to_u32x4 a) (to_u32x4 b)) =
  ArmL.vaddq_u32 a b; rt_u32x4 (ArmIV.vaddq_u32 (to_u32x4 a) (to_u32x4 b))

(* ── shifts ───────────────────────────────────────────────────────────────── *)
let lemma_vshlq_n_s16 (v_N: i32) (a: bv128)
  : Lemma (to_i16x8 (Neon.vshlq_n_s16 v_N a) == ArmIV.vshlq_n_s16 v_N (to_i16x8 a)) =
  ArmL.vshlq_n_s16 v_N a; rt_i16x8 (ArmIV.vshlq_n_s16 v_N (to_i16x8 a))

let lemma_vshrq_n_s16 (v_N: i32) (a: bv128)
  : Lemma (to_i16x8 (Neon.vshrq_n_s16 v_N a) == ArmIV.vshrq_n_s16 v_N (to_i16x8 a)) =
  ArmL.vshrq_n_s16 v_N a; rt_i16x8 (ArmIV.vshrq_n_s16 v_N (to_i16x8 a))

let lemma_vshrq_n_u16 (v_N: i32) (a: bv128)
  : Lemma (to_u16x8 (Neon.vshrq_n_u16 v_N a) == ArmIV.vshrq_n_u16 v_N (to_u16x8 a)) =
  ArmL.vshrq_n_u16 v_N a; rt_u16x8 (ArmIV.vshrq_n_u16 v_N (to_u16x8 a))

let lemma_vshlq_n_u32 (v_N: i32) (a: bv128)
  : Lemma (to_u32x4 (Neon.vshlq_n_u32 v_N a) == ArmIV.vshlq_n_u32 v_N (to_u32x4 a)) =
  ArmL.vshlq_n_u32 v_N a; rt_u32x4 (ArmIV.vshlq_n_u32 v_N (to_u32x4 a))

let lemma_vshrq_n_u32 (v_N: i32) (a: bv128)
  : Lemma (to_u32x4 (Neon.vshrq_n_u32 v_N a) == ArmIV.vshrq_n_u32 v_N (to_u32x4 a)) =
  ArmL.vshrq_n_u32 v_N a; rt_u32x4 (ArmIV.vshrq_n_u32 v_N (to_u32x4 a))

let lemma_vshlq_n_u64 (v_N: i32) (a: bv128)
  : Lemma (to_u64x2 (Neon.vshlq_n_u64 v_N a) == ArmIV.vshlq_n_u64 v_N (to_u64x2 a)) =
  ArmL.vshlq_n_u64 v_N a; rt_u64x2 (ArmIV.vshlq_n_u64 v_N (to_u64x2 a))

let lemma_vshrq_n_u64 (v_N: i32) (a: bv128)
  : Lemma (to_u64x2 (Neon.vshrq_n_u64 v_N a) == ArmIV.vshrq_n_u64 v_N (to_u64x2 a)) =
  ArmL.vshrq_n_u64 v_N a; rt_u64x2 (ArmIV.vshrq_n_u64 v_N (to_u64x2 a))

let lemma_vshlq_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vshlq_s16 a b) == ArmIV.vshlq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vshlq_s16 a b; rt_i16x8 (ArmIV.vshlq_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vshlq_u16 (a b: bv128)
  : Lemma (to_u16x8 (Neon.vshlq_u16 a b) == ArmIV.vshlq_u16 (to_u16x8 a) (to_i16x8 b)) =
  ArmL.vshlq_u16 a b; rt_u16x8 (ArmIV.vshlq_u16 (to_u16x8 a) (to_i16x8 b))

(* ── comparisons (i16x8 -> u16x8) ─────────────────────────────────────────── *)
let lemma_vcgeq_s16 (a b: bv128)
  : Lemma (to_u16x8 (Neon.vcgeq_s16 a b) == ArmIV.vcgeq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vcgeq_s16 a b; rt_u16x8 (ArmIV.vcgeq_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vcleq_s16 (a b: bv128)
  : Lemma (to_u16x8 (Neon.vcleq_s16 a b) == ArmIV.vcleq_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vcleq_s16 a b; rt_u16x8 (ArmIV.vcleq_s16 (to_i16x8 a) (to_i16x8 b))

(* ── duplicate scalar -> vector ───────────────────────────────────────────── *)
let lemma_vdupq_n_s16 (x: i16)
  : Lemma (to_i16x8 (Neon.vdupq_n_s16 x) == ArmIV.vdupq_n_s16 x) =
  ArmL.vdupq_n_s16 x; rt_i16x8 (ArmIV.vdupq_n_s16 x)

let lemma_vdupq_n_u16 (x: u16)
  : Lemma (to_u16x8 (Neon.vdupq_n_u16 x) == ArmIV.vdupq_n_u16 x) =
  ArmL.vdupq_n_u16 x; rt_u16x8 (ArmIV.vdupq_n_u16 x)

let lemma_vdupq_n_u32 (x: u32)
  : Lemma (to_u32x4 (Neon.vdupq_n_u32 x) == ArmIV.vdupq_n_u32 x) =
  ArmL.vdupq_n_u32 x; rt_u32x4 (ArmIV.vdupq_n_u32 x)

let lemma_vdupq_n_u64 (x: u64)
  : Lemma (to_u64x2 (Neon.vdupq_n_u64 x) == ArmIV.vdupq_n_u64 x) =
  ArmL.vdupq_n_u64 x; rt_u64x2 (ArmIV.vdupq_n_u64 x)

let lemma_vdupq_n_u8 (x: u8)
  : Lemma (to_u8x16 (Neon.vdupq_n_u8 x) == ArmIV.vdupq_n_u8 x) =
  ArmL.vdupq_n_u8 x; rt_u8x16 (ArmIV.vdupq_n_u8 x)

let lemma_vdupq_laneq_u32 (v_N: i32) (a: bv128)
  : Lemma (to_u32x4 (Neon.vdupq_laneq_u32 v_N a) == ArmIV.vdupq_laneq_u32 v_N (to_u32x4 a)) =
  ArmL.vdupq_laneq_u32 v_N a; rt_u32x4 (ArmIV.vdupq_laneq_u32 v_N (to_u32x4 a))

(* ── get low / high (128 -> 64) ───────────────────────────────────────────── *)
let lemma_vget_low_s16 (a: bv128)
  : Lemma (to_i16x4 (Neon.vget_low_s16 a) == ArmIV.vget_low_s16 (to_i16x8 a)) =
  ArmL.vget_low_s16 a; rt_i16x4 (ArmIV.vget_low_s16 (to_i16x8 a))

let lemma_vget_low_u16 (a: bv128)
  : Lemma (to_u16x4 (Neon.vget_low_u16 a) == ArmIV.vget_low_u16 (to_u16x8 a)) =
  ArmL.vget_low_u16 a; rt_u16x4 (ArmIV.vget_low_u16 (to_u16x8 a))

let lemma_vget_high_u16 (a: bv128)
  : Lemma (to_u16x4 (Neon.vget_high_u16 a) == ArmIV.vget_high_u16 (to_u16x8 a)) =
  ArmL.vget_high_u16 a; rt_u16x4 (ArmIV.vget_high_u16 (to_u16x8 a))

(* ── cross-width widening multiply ────────────────────────────────────────── *)
let lemma_vmull_s16 (a b: bv64)
  : Lemma (to_i32x4 (Neon.vmull_s16 a b) == ArmIV.vmull_s16 (to_i16x4 a) (to_i16x4 b)) =
  ArmL.vmull_s16 a b; rt_i32x4 (ArmIV.vmull_s16 (to_i16x4 a) (to_i16x4 b))

let lemma_vmull_high_s16 (a b: bv128)
  : Lemma (to_i32x4 (Neon.vmull_high_s16 a b) == ArmIV.vmull_high_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vmull_high_s16 a b; rt_i32x4 (ArmIV.vmull_high_s16 (to_i16x8 a) (to_i16x8 b))

(* ── widening multiply-ACCUMULATE (vmlal): a + b*c.  Same lift+round-trip recipe
      as vmull; the accumulator `a` is the i32x4 first operand. ─────────────────── *)
let lemma_vmlal_s16 (a: bv128) (b c: bv64)
  : Lemma (to_i32x4 (Neon.vmlal_s16 a b c)
           == ArmIV.vmlal_s16 (to_i32x4 a) (to_i16x4 b) (to_i16x4 c)) =
  ArmL.vmlal_s16 a b c; rt_i32x4 (ArmIV.vmlal_s16 (to_i32x4 a) (to_i16x4 b) (to_i16x4 c))

let lemma_vmlal_high_s16 (a b c: bv128)
  : Lemma (to_i32x4 (Neon.vmlal_high_s16 a b c)
           == ArmIV.vmlal_high_s16 (to_i32x4 a) (to_i16x8 b) (to_i16x8 c)) =
  ArmL.vmlal_high_s16 a b c; rt_i32x4 (ArmIV.vmlal_high_s16 (to_i32x4 a) (to_i16x8 b) (to_i16x8 c))

(* ── transpose (TRN1/TRN2) ────────────────────────────────────────────────── *)
let lemma_vtrn1q_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vtrn1q_s16 a b) == ArmIV.vtrn1q_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vtrn1q_s16 a b; rt_i16x8 (ArmIV.vtrn1q_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vtrn2q_s16 (a b: bv128)
  : Lemma (to_i16x8 (Neon.vtrn2q_s16 a b) == ArmIV.vtrn2q_s16 (to_i16x8 a) (to_i16x8 b)) =
  ArmL.vtrn2q_s16 a b; rt_i16x8 (ArmIV.vtrn2q_s16 (to_i16x8 a) (to_i16x8 b))

let lemma_vtrn1q_s32 (a b: bv128)
  : Lemma (to_i32x4 (Neon.vtrn1q_s32 a b) == ArmIV.vtrn1q_s32 (to_i32x4 a) (to_i32x4 b)) =
  ArmL.vtrn1q_s32 a b; rt_i32x4 (ArmIV.vtrn1q_s32 (to_i32x4 a) (to_i32x4 b))

let lemma_vtrn2q_s32 (a b: bv128)
  : Lemma (to_i32x4 (Neon.vtrn2q_s32 a b) == ArmIV.vtrn2q_s32 (to_i32x4 a) (to_i32x4 b)) =
  ArmL.vtrn2q_s32 a b; rt_i32x4 (ArmIV.vtrn2q_s32 (to_i32x4 a) (to_i32x4 b))

let lemma_vtrn1q_s64 (a b: bv128)
  : Lemma (to_i64x2 (Neon.vtrn1q_s64 a b) == ArmIV.vtrn1q_s64 (to_i64x2 a) (to_i64x2 b)) =
  ArmL.vtrn1q_s64 a b; rt_i64x2 (ArmIV.vtrn1q_s64 (to_i64x2 a) (to_i64x2 b))

let lemma_vtrn2q_s64 (a b: bv128)
  : Lemma (to_i64x2 (Neon.vtrn2q_s64 a b) == ArmIV.vtrn2q_s64 (to_i64x2 a) (to_i64x2 b)) =
  ArmL.vtrn2q_s64 a b; rt_i64x2 (ArmIV.vtrn2q_s64 (to_i64x2 a) (to_i64x2 b))

let lemma_vtrn1q_u64 (a b: bv128)
  : Lemma (to_u64x2 (Neon.vtrn1q_u64 a b) == ArmIV.vtrn1q_u64 (to_u64x2 a) (to_u64x2 b)) =
  ArmL.vtrn1q_u64 a b; rt_u64x2 (ArmIV.vtrn1q_u64 (to_u64x2 a) (to_u64x2 b))

let lemma_vtrn2q_u64 (a b: bv128)
  : Lemma (to_u64x2 (Neon.vtrn2q_u64 a b) == ArmIV.vtrn2q_u64 (to_u64x2 a) (to_u64x2 b)) =
  ArmL.vtrn2q_u64 a b; rt_u64x2 (ArmIV.vtrn2q_u64 (to_u64x2 a) (to_u64x2 b))

(* ── table lookup / extract / shift-insert ────────────────────────────────── *)
let lemma_vqtbl1q_u8 (t idx: bv128)
  : Lemma (to_u8x16 (Neon.vqtbl1q_u8 t idx) == ArmIV.vqtbl1q_u8 (to_u8x16 t) (to_u8x16 idx)) =
  ArmL.vqtbl1q_u8 t idx; rt_u8x16 (ArmIV.vqtbl1q_u8 (to_u8x16 t) (to_u8x16 idx))

let lemma_vextq_u32 (v_N: i32) (a b: bv128)
  : Lemma (to_u32x4 (Neon.vextq_u32 v_N a b) == ArmIV.vextq_u32 v_N (to_u32x4 a) (to_u32x4 b)) =
  ArmL.vextq_u32 v_N a b; rt_u32x4 (ArmIV.vextq_u32 v_N (to_u32x4 a) (to_u32x4 b))

let lemma_vsliq_n_s32 (v_N: i32) (a b: bv128)
  : Lemma (to_i32x4 (Neon.vsliq_n_s32 v_N a b) == ArmIV.vsliq_n_s32 v_N (to_i32x4 a) (to_i32x4 b)) =
  ArmL.vsliq_n_s32 v_N a b; rt_i32x4 (ArmIV.vsliq_n_s32 v_N (to_i32x4 a) (to_i32x4 b))

let lemma_vsliq_n_s64 (v_N: i32) (a b: bv128)
  : Lemma (to_i64x2 (Neon.vsliq_n_s64 v_N a b) == ArmIV.vsliq_n_s64 v_N (to_i64x2 a) (to_i64x2 b)) =
  ArmL.vsliq_n_s64 v_N a b; rt_i64x2 (ArmIV.vsliq_n_s64 v_N (to_i64x2 a) (to_i64x2 b))

(* ── scalar-result reductions (lift IS the op-lemma; no from/rt) ───────────── *)
let lemma_vaddvq_s16 (a: bv128)
  : Lemma (Neon.vaddvq_s16 a == ArmIV.vaddvq_s16 (to_i16x8 a)) = ArmL.vaddvq_s16 a

let lemma_vaddvq_u16 (a: bv128)
  : Lemma (Neon.vaddvq_u16 a == ArmIV.vaddvq_u16 (to_u16x8 a)) = ArmL.vaddvq_u16 a

let lemma_vaddv_u16 (a: bv64)
  : Lemma (Neon.vaddv_u16 a == ArmIV.vaddv_u16 (to_u16x4 a)) = ArmL.vaddv_u16 a

(* ── bitwise: raw t_BitVec passthrough (lift IS the op-lemma; no codec) ────── *)
let lemma_vandq_s16 (a b: bv128) : Lemma (Neon.vandq_s16 a b == ArmIV.vandq_s16 a b) = ArmL.vandq_s16 a b
let lemma_vandq_u16 (a b: bv128) : Lemma (Neon.vandq_u16 a b == ArmIV.vandq_u16 a b) = ArmL.vandq_u16 a b
let lemma_vandq_u32 (a b: bv128) : Lemma (Neon.vandq_u32 a b == ArmIV.vandq_u32 a b) = ArmL.vandq_u32 a b
let lemma_vbicq_u64 (a b: bv128) : Lemma (Neon.vbicq_u64 a b == ArmIV.vbicq_u64 a b) = ArmL.vbicq_u64 a b
let lemma_veorq_s16 (a b: bv128) : Lemma (Neon.veorq_s16 a b == ArmIV.veorq_s16 a b) = ArmL.veorq_s16 a b
let lemma_veorq_u32 (a b: bv128) : Lemma (Neon.veorq_u32 a b == ArmIV.veorq_u32 a b) = ArmL.veorq_u32 a b
let lemma_veorq_u64 (a b: bv128) : Lemma (Neon.veorq_u64 a b == ArmIV.veorq_u64 a b) = ArmL.veorq_u64 a b
let lemma_veorq_u8  (a b: bv128) : Lemma (Neon.veorq_u8  a b == ArmIV.veorq_u8  a b) = ArmL.veorq_u8  a b

(* ── reinterprets: raw t_BitVec passthrough ───────────────────────────────── *)
let lemma_vreinterpretq_s16_s32 (a: bv128) : Lemma (Neon.vreinterpretq_s16_s32 a == ArmIV.vreinterpretq_s16_s32 a) = ArmL.vreinterpretq_s16_s32 a
let lemma_vreinterpretq_s16_s64 (a: bv128) : Lemma (Neon.vreinterpretq_s16_s64 a == ArmIV.vreinterpretq_s16_s64 a) = ArmL.vreinterpretq_s16_s64 a
let lemma_vreinterpretq_s16_u16 (a: bv128) : Lemma (Neon.vreinterpretq_s16_u16 a == ArmIV.vreinterpretq_s16_u16 a) = ArmL.vreinterpretq_s16_u16 a
let lemma_vreinterpretq_s16_u32 (a: bv128) : Lemma (Neon.vreinterpretq_s16_u32 a == ArmIV.vreinterpretq_s16_u32 a) = ArmL.vreinterpretq_s16_u32 a
let lemma_vreinterpretq_s16_u8  (a: bv128) : Lemma (Neon.vreinterpretq_s16_u8  a == ArmIV.vreinterpretq_s16_u8  a) = ArmL.vreinterpretq_s16_u8  a
let lemma_vreinterpretq_s32_s16 (a: bv128) : Lemma (Neon.vreinterpretq_s32_s16 a == ArmIV.vreinterpretq_s32_s16 a) = ArmL.vreinterpretq_s32_s16 a
let lemma_vreinterpretq_s32_u32 (a: bv128) : Lemma (Neon.vreinterpretq_s32_u32 a == ArmIV.vreinterpretq_s32_u32 a) = ArmL.vreinterpretq_s32_u32 a
let lemma_vreinterpretq_s64_s16 (a: bv128) : Lemma (Neon.vreinterpretq_s64_s16 a == ArmIV.vreinterpretq_s64_s16 a) = ArmL.vreinterpretq_s64_s16 a
let lemma_vreinterpretq_s64_s32 (a: bv128) : Lemma (Neon.vreinterpretq_s64_s32 a == ArmIV.vreinterpretq_s64_s32 a) = ArmL.vreinterpretq_s64_s32 a
let lemma_vreinterpretq_u16_s16 (a: bv128) : Lemma (Neon.vreinterpretq_u16_s16 a == ArmIV.vreinterpretq_u16_s16 a) = ArmL.vreinterpretq_u16_s16 a
let lemma_vreinterpretq_u16_u8  (a: bv128) : Lemma (Neon.vreinterpretq_u16_u8  a == ArmIV.vreinterpretq_u16_u8  a) = ArmL.vreinterpretq_u16_u8  a
let lemma_vreinterpretq_u32_s16 (a: bv128) : Lemma (Neon.vreinterpretq_u32_s16 a == ArmIV.vreinterpretq_u32_s16 a) = ArmL.vreinterpretq_u32_s16 a
let lemma_vreinterpretq_u32_s32 (a: bv128) : Lemma (Neon.vreinterpretq_u32_s32 a == ArmIV.vreinterpretq_u32_s32 a) = ArmL.vreinterpretq_u32_s32 a
let lemma_vreinterpretq_u32_u8  (a: bv128) : Lemma (Neon.vreinterpretq_u32_u8  a == ArmIV.vreinterpretq_u32_u8  a) = ArmL.vreinterpretq_u32_u8  a
let lemma_vreinterpretq_u8_s16  (a: bv128) : Lemma (Neon.vreinterpretq_u8_s16  a == ArmIV.vreinterpretq_u8_s16  a) = ArmL.vreinterpretq_u8_s16  a
let lemma_vreinterpretq_u8_s64  (a: bv128) : Lemma (Neon.vreinterpretq_u8_s64  a == ArmIV.vreinterpretq_u8_s64  a) = ArmL.vreinterpretq_u8_s64  a
let lemma_vreinterpretq_u8_u32  (a: bv128) : Lemma (Neon.vreinterpretq_u8_u32  a == ArmIV.vreinterpretq_u8_u32  a) = ArmL.vreinterpretq_u8_u32  a

(* ── SHA3 / AES handwritten ops (Neon_handwritten; lifts in ArmL) ──────────── *)
let lemma_vrax1q_u64 (a b: bv128)
  : Lemma (to_u64x2 (ArmHW.vrax1q_u64 a b) == ArmIV.vrax1q_u64 (to_u64x2 a) (to_u64x2 b)) =
  ArmL.vrax1q_u64 a b; rt_u64x2 (ArmIV.vrax1q_u64 (to_u64x2 a) (to_u64x2 b))

let lemma_veor3q_u64 (a b c: bv128)
  : Lemma (to_u64x2 (ArmHW.veor3q_u64 a b c) == ArmIV.veor3q_u64 (to_u64x2 a) (to_u64x2 b) (to_u64x2 c)) =
  ArmL.veor3q_u64 a b c; rt_u64x2 (ArmIV.veor3q_u64 (to_u64x2 a) (to_u64x2 b) (to_u64x2 c))

let lemma_vbcaxq_u64 (a b c: bv128)
  : Lemma (to_u64x2 (ArmHW.vbcaxq_u64 a b c) == ArmIV.vbcaxq_u64 (to_u64x2 a) (to_u64x2 b) (to_u64x2 c)) =
  ArmL.vbcaxq_u64 a b c; rt_u64x2 (ArmIV.vbcaxq_u64 (to_u64x2 a) (to_u64x2 b) (to_u64x2 c))

let lemma_vxarq_u64 (v_N: i32) (a b: bv128)
  : Lemma (to_u64x2 (ArmHW.vxarq_u64 v_N a b) == ArmIV.vxarq_u64 v_N (to_u64x2 a) (to_u64x2 b)) =
  ArmL.vxarq_u64 v_N a b; rt_u64x2 (ArmIV.vxarq_u64 v_N (to_u64x2 a) (to_u64x2 b))

let lemma_vaeseq_u8 (data key: bv128)
  : Lemma (to_u8x16 (ArmHW.vaeseq_u8 data key) == ArmIV.vaeseq_u8 (to_u8x16 data) (to_u8x16 key)) =
  ArmL.vaeseq_u8 data key; rt_u8x16 (ArmIV.vaeseq_u8 (to_u8x16 data) (to_u8x16 key))

let lemma_vaesmcq_u8 (data: bv128)
  : Lemma (to_u8x16 (ArmHW.vaesmcq_u8 data) == ArmIV.vaesmcq_u8 (to_u8x16 data)) =
  ArmL.vaesmcq_u8 data; rt_u8x16 (ArmIV.vaesmcq_u8 (to_u8x16 data))

let lemma_vmull_p64 (a b: u64)
  : Lemma (ArmHW.vmull_p64 a b == ArmIV.vmull_p64 a b) = ArmL.vmull_p64 a b
