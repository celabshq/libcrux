module Libcrux_core_models.Intrinsics_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 30"
open FStar.Mul
open Core_models

(* ============================================================================
   CANONICAL intrinsics lane-view + op-lemma companion.

   Single source of truth for the SIMD lane view + the proven op-lemma set that
   ml-kem / ml-dsa / sha3 will (eventually) all `open` — replacing ml-dsa's
   `Spec.Intrinsics`, ml-kem's `Libcrux_intrinsics.Avx2_ml_kem_views`, and sha3's
   companion.  It rests entirely on the differentially-tested `core-models` model.

   The lane VIEW is the CONCRETE core-models conversion `to_iWxL`
   (`Int_vec_interp.e_ee_N__impl__to_*`, a `FunArray`) — re-exported here, NO new
   axiom.  Each op-lemma
     `to_OUT (Avx2.e_mm256_OP a b) == Int_vec.e_mm256_OP (to_IN a) (to_IN b)`
   is PROVEN by a uniform, op-agnostic reduction:
       lift lemma  (Int_vec.Lemmas, differentially-tested `assume val`, [@@ v_LIFT_LEMMA])
     + round-trip  (Int_vec_interp.lemma_conv_rt, PROVEN)
     + Int_vec def (definitional).
   `op_lemma_from_lift_{bin,un}` capture the reduction; each op-lemma is one
   instance (const-generic ops partially-apply the immediate; ternary / two-view /
   scalar-result / raw-passthrough / set ops inline the same two-step body).

   NOTE (SMTPat): op-lemmas are exposed WITHOUT SMTPat — this foundational module
   stays cascade-free; consumers call them explicitly (or add scoped SMTPats
   themselves).  Ground-term ops (setzero/…) would otherwise mint variable-
   free triggers (Error 276), see ml-kem `Avx2_ml_kem_views` note.
   ============================================================================ *)

module BV     = Libcrux_core_models.Abstractions.Bitvec
module Funarr = Libcrux_core_models.Abstractions.Funarr
module Bit    = Libcrux_core_models.Abstractions.Bit
module Int    = Rust_primitives.Integers
module IV     = Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
module IVL    = Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.Lemmas
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Avx    = Libcrux_core_models.Core_arch.X86.Avx
module Avx2   = Libcrux_core_models.Core_arch.X86.Avx2
module Sse2   = Libcrux_core_models.Core_arch.X86.Sse2
module Ssse3  = Libcrux_core_models.Core_arch.X86.Ssse3

(* ── THE AXIOMS THIS MODULE RESTS ON ────────────────────────────────────────
   Every hand-written intrinsics AXIOM lives in `Libcrux_core_models.Trusted.
   Intrinsics` (with its differential-test justification) and is re-exported
   here, so THIS module contains only PROVEN lemmas.  `bv256` / `bv128` come
   from there too.  Do not add an `assume` below — put it in the Trusted module.
   ------------------------------------------------------------------------- *)
include Libcrux_core_models.Trusted.Intrinsics

(* ── the canonical lane views = core-models `to_iWxL` (re-export, no new axiom) ─ *)
let to_i16x16  = IVi.e_ee_2__impl__to_i16x16
let from_i16x16 = IVi.e_ee_2__impl__from_i16x16
let to_i32x8   = IVi.e___impl__to_i32x8
let from_i32x8  = IVi.e___impl__from_i32x8
let to_i64x4   = IVi.e_ee_1__impl__to_i64x4
let from_i64x4  = IVi.e_ee_1__impl__from_i64x4
let to_i8x32   = IVi.e_ee_4__impl__to_i8x32
let from_i8x32  = IVi.e_ee_4__impl__from_i8x32
let to_u32x8   = IVi.e_ee_5__impl__to_u32x8
let to_u64x4   = IVi.e_ee_6__impl__to_u64x4
let from_u64x4  = IVi.e_ee_6__impl__from_u64x4
let to_i128x2  = IVi.e_ee_3__impl__to_i128x2
let from_i128x2 = IVi.e_ee_3__impl__from_i128x2
let to_i128x1  = IVi.e_ee_12__impl__to_i128x1
let to_i32x4   = IVi.e_ee_9__impl__to_i32x4
let from_i32x4  = IVi.e_ee_9__impl__from_i32x4
let to_i64x2   = IVi.e_ee_10__impl__to_i64x2
let from_i64x2  = IVi.e_ee_10__impl__from_i64x2
let to_i16x8   = IVi.e_ee_11__impl__to_i16x8
let from_i16x8  = IVi.e_ee_11__impl__from_i16x8
let to_i8x16   = IVi.e_ee_13__impl__to_i8x16
let from_i8x16  = IVi.e_ee_13__impl__from_i8x16

(* ── the op-agnostic reduction combinators ─────────────────────────────────── *)

(* Every view-output op-lemma reduces to lift + round-trip; general binary form
   (input and output views may differ, e.g. mul_epi32: i32x8 -> i64x4). *)
let op_lemma_from_lift_bin
    (#in_w #out_w: u64)
    (#in_fa #out_fa: Type0)
    (to_in:    BV.t_BitVec in_w  -> in_fa)
    (from_out: out_fa -> BV.t_BitVec out_w)
    (to_out:   BV.t_BitVec out_w -> out_fa)
    (op_bit: BV.t_BitVec in_w -> BV.t_BitVec in_w -> BV.t_BitVec out_w)
    (op_int: in_fa -> in_fa -> out_fa)
    (lift: (a: BV.t_BitVec in_w) -> (b: BV.t_BitVec in_w)
           -> Lemma (op_bit a b == from_out (op_int (to_in a) (to_in b))))
    (rt: (y: out_fa) -> Lemma (to_out (from_out y) == y))
    (a b: BV.t_BitVec in_w)
  : Lemma (to_out (op_bit a b) == op_int (to_in a) (to_in b)) =
  lift a b;
  rt (op_int (to_in a) (to_in b))

(* Unary form (covers `castsi`/`cvtepi`/`abs` and every const-generic op once its
   immediate is partially applied). *)
let op_lemma_from_lift_un
    (#in_w #out_w: u64)
    (#in_fa #out_fa: Type0)
    (to_in:    BV.t_BitVec in_w  -> in_fa)
    (from_out: out_fa -> BV.t_BitVec out_w)
    (to_out:   BV.t_BitVec out_w -> out_fa)
    (op_bit: BV.t_BitVec in_w -> BV.t_BitVec out_w)
    (op_int: in_fa -> out_fa)
    (lift: (x: BV.t_BitVec in_w)
           -> Lemma (op_bit x == from_out (op_int (to_in x))))
    (rt: (y: out_fa) -> Lemma (to_out (from_out y) == y))
    (x: BV.t_BitVec in_w)
  : Lemma (to_out (op_bit x) == op_int (to_in x)) =
  lift x;
  rt (op_int (to_in x))

(* ── per-width round-trips (all instances of the PROVEN generic `lemma_conv_rt`) ─ *)
let rt_i16x16 (y: Funarr.t_FunArray (mk_u64 16) i16) : Lemma (to_i16x16 (from_i16x16 y) == y) =
  IVi.lemma_conv_rt Int.I16 (mk_u64 256) (mk_u64 16) y
let rt_i32x8 (y: Funarr.t_FunArray (mk_u64 8) i32) : Lemma (to_i32x8 (from_i32x8 y) == y) =
  IVi.lemma_conv_rt Int.I32 (mk_u64 256) (mk_u64 8) y
let rt_i64x4 (y: Funarr.t_FunArray (mk_u64 4) i64) : Lemma (to_i64x4 (from_i64x4 y) == y) =
  IVi.lemma_conv_rt Int.I64 (mk_u64 256) (mk_u64 4) y
let rt_u64x4 (y: Funarr.t_FunArray (mk_u64 4) u64) : Lemma (to_u64x4 (from_u64x4 y) == y) =
  IVi.lemma_conv_rt Int.U64 (mk_u64 256) (mk_u64 4) y
let rt_i8x32 (y: Funarr.t_FunArray (mk_u64 32) i8) : Lemma (to_i8x32 (from_i8x32 y) == y) =
  IVi.lemma_conv_rt Int.I8 (mk_u64 256) (mk_u64 32) y
let rt_i128x2 (y: Funarr.t_FunArray (mk_u64 2) i128) : Lemma (to_i128x2 (from_i128x2 y) == y) =
  IVi.lemma_conv_rt Int.I128 (mk_u64 256) (mk_u64 2) y
let rt_i32x4 (y: Funarr.t_FunArray (mk_u64 4) i32) : Lemma (to_i32x4 (from_i32x4 y) == y) =
  IVi.lemma_conv_rt Int.I32 (mk_u64 128) (mk_u64 4) y
let rt_i64x2 (y: Funarr.t_FunArray (mk_u64 2) i64) : Lemma (to_i64x2 (from_i64x2 y) == y) =
  IVi.lemma_conv_rt Int.I64 (mk_u64 128) (mk_u64 2) y
let rt_i16x8 (y: Funarr.t_FunArray (mk_u64 8) i16) : Lemma (to_i16x8 (from_i16x8 y) == y) =
  IVi.lemma_conv_rt Int.I16 (mk_u64 128) (mk_u64 8) y
let rt_i8x16 (y: Funarr.t_FunArray (mk_u64 16) i8) : Lemma (to_i8x16 (from_i8x16 y) == y) =
  IVi.lemma_conv_rt Int.I8 (mk_u64 128) (mk_u64 16) y

(* ============================================================================
   PROVEN op-lemmas — full op set (each an instance of the reduction above).
   ============================================================================ *)

let lemma_mm256_set1_epi32 (x: i32)
  : Lemma (to_i32x8 (Avx.e_mm256_set1_epi32 x) == IV.e_mm256_set1_epi32 x) =
  IVL.e_mm256_set1_epi32' x;
  rt_i32x8 (IV.e_mm256_set1_epi32 x)

let lemma_mm256_mul_epi32 (x: bv256) (y: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_mul_epi32 x y) == IV.e_mm256_mul_epi32 (to_i32x8 x) (to_i32x8 y)) =
  op_lemma_from_lift_bin to_i32x8 from_i64x4 to_i64x4
    Avx2.e_mm256_mul_epi32 IV.e_mm256_mul_epi32 IVL.e_mm256_mul_epi32' rt_i64x4 x y

let lemma_mm256_sub_epi32 (x: bv256) (y: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_sub_epi32 x y) == IV.e_mm256_sub_epi32 (to_i32x8 x) (to_i32x8 y)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_sub_epi32 IV.e_mm256_sub_epi32 IVL.e_mm256_sub_epi32' rt_i32x8 x y

let lemma_mm256_shuffle_epi32 (v_CONTROL: i32) (x: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_shuffle_epi32 v_CONTROL x) == IV.e_mm256_shuffle_epi32 v_CONTROL (to_i32x8 x)) =
  op_lemma_from_lift_un to_i32x8 from_i32x8 to_i32x8
    (Avx2.e_mm256_shuffle_epi32 v_CONTROL) (IV.e_mm256_shuffle_epi32 v_CONTROL) (IVL.e_mm256_shuffle_epi32' v_CONTROL) rt_i32x8 x

let lemma_mm256_blend_epi32 (v_CONTROL: i32) (x: bv256) (y: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_blend_epi32 v_CONTROL x y) == IV.e_mm256_blend_epi32 v_CONTROL (to_i32x8 x) (to_i32x8 y)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    (Avx2.e_mm256_blend_epi32 v_CONTROL) (IV.e_mm256_blend_epi32 v_CONTROL) (IVL.e_mm256_blend_epi32' v_CONTROL) rt_i32x8 x y

let lemma_mm256_set1_epi16 (x: i16)
  : Lemma (to_i16x16 (Avx.e_mm256_set1_epi16 x) == IV.e_mm256_set1_epi16 x) =
  IVL.e_mm256_set1_epi16' x;
  rt_i16x16 (IV.e_mm256_set1_epi16 x)

let lemma_mm_set1_epi16 (x: i16)
  : Lemma (to_i16x8 (Sse2.e_mm_set1_epi16 x) == IV.e_mm_set1_epi16 x) =
  IVL.e_mm_set1_epi16' x;
  rt_i16x8 (IV.e_mm_set1_epi16 x)

let lemma_mm_set_epi32 (e3: i32) (e2: i32) (e1: i32) (e0: i32)
  : Lemma (to_i32x4 (Sse2.e_mm_set_epi32 e3 e2 e1 e0) == IV.e_mm_set_epi32 e3 e2 e1 e0) =
  IVL.e_mm_set_epi32' e3 e2 e1 e0;
  rt_i32x4 (IV.e_mm_set_epi32 e3 e2 e1 e0)

let lemma_mm_add_epi16 (a: bv128) (b: bv128)
  : Lemma (to_i16x8 (Sse2.e_mm_add_epi16 a b) == IV.e_mm_add_epi16 (to_i16x8 a) (to_i16x8 b)) =
  op_lemma_from_lift_bin to_i16x8 from_i16x8 to_i16x8
    Sse2.e_mm_add_epi16 IV.e_mm_add_epi16 IVL.e_mm_add_epi16' rt_i16x8 a b

let lemma_mm256_add_epi16 (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_add_epi16 a b) == IV.e_mm256_add_epi16 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i16x16 to_i16x16
    Avx2.e_mm256_add_epi16 IV.e_mm256_add_epi16 IVL.e_mm256_add_epi16' rt_i16x16 a b

let lemma_mm256_add_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_add_epi32 a b) == IV.e_mm256_add_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_add_epi32 IV.e_mm256_add_epi32 IVL.e_mm256_add_epi32' rt_i32x8 a b

let lemma_mm256_add_epi64 (a: bv256) (b: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_add_epi64 a b) == IV.e_mm256_add_epi64 (to_i64x4 a) (to_i64x4 b)) =
  op_lemma_from_lift_bin to_i64x4 from_i64x4 to_i64x4
    Avx2.e_mm256_add_epi64 IV.e_mm256_add_epi64 IVL.e_mm256_add_epi64' rt_i64x4 a b

let lemma_mm256_abs_epi32 (a: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_abs_epi32 a) == IV.e_mm256_abs_epi32 (to_i32x8 a)) =
  op_lemma_from_lift_un to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_abs_epi32 IV.e_mm256_abs_epi32 IVL.e_mm256_abs_epi32' rt_i32x8 a

let lemma_mm256_sub_epi16 (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_sub_epi16 a b) == IV.e_mm256_sub_epi16 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i16x16 to_i16x16
    Avx2.e_mm256_sub_epi16 IV.e_mm256_sub_epi16 IVL.e_mm256_sub_epi16' rt_i16x16 a b

let lemma_mm_mullo_epi16 (a: bv128) (b: bv128)
  : Lemma (to_i16x8 (Sse2.e_mm_mullo_epi16 a b) == IV.e_mm_mullo_epi16 (to_i16x8 a) (to_i16x8 b)) =
  op_lemma_from_lift_bin to_i16x8 from_i16x8 to_i16x8
    Sse2.e_mm_mullo_epi16 IV.e_mm_mullo_epi16 IVL.e_mm_mullo_epi16' rt_i16x8 a b

let lemma_mm256_cmpgt_epi16 (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_cmpgt_epi16 a b) == IV.e_mm256_cmpgt_epi16 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i16x16 to_i16x16
    Avx2.e_mm256_cmpgt_epi16 IV.e_mm256_cmpgt_epi16 IVL.e_mm256_cmpgt_epi16' rt_i16x16 a b

let lemma_mm256_cmpgt_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_cmpgt_epi32 a b) == IV.e_mm256_cmpgt_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_cmpgt_epi32 IV.e_mm256_cmpgt_epi32 IVL.e_mm256_cmpgt_epi32' rt_i32x8 a b

let lemma_mm256_sign_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_sign_epi32 a b) == IV.e_mm256_sign_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_sign_epi32 IV.e_mm256_sign_epi32 IVL.e_mm256_sign_epi32' rt_i32x8 a b

let lemma_mm256_movemask_ps (a: bv256)
  : Lemma (Avx.e_mm256_movemask_ps a == IV.e_mm256_movemask_ps (to_i32x8 a)) =
  IVL.e_mm256_movemask_ps' a

let lemma_mm_mulhi_epi16 (a: bv128) (b: bv128)
  : Lemma (to_i16x8 (Sse2.e_mm_mulhi_epi16 a b) == IV.e_mm_mulhi_epi16 (to_i16x8 a) (to_i16x8 b)) =
  op_lemma_from_lift_bin to_i16x8 from_i16x8 to_i16x8
    Sse2.e_mm_mulhi_epi16 IV.e_mm_mulhi_epi16 IVL.e_mm_mulhi_epi16' rt_i16x8 a b

let lemma_mm256_mullo_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_mullo_epi32 a b) == IV.e_mm256_mullo_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_mullo_epi32 IV.e_mm256_mullo_epi32 IVL.e_mm256_mullo_epi32' rt_i32x8 a b

let lemma_mm256_mulhi_epi16 (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_mulhi_epi16 a b) == IV.e_mm256_mulhi_epi16 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i16x16 to_i16x16
    Avx2.e_mm256_mulhi_epi16 IV.e_mm256_mulhi_epi16 IVL.e_mm256_mulhi_epi16' rt_i16x16 a b

let lemma_mm256_mul_epu32 (a: bv256) (b: bv256)
  : Lemma (to_u64x4 (Avx2.e_mm256_mul_epu32 a b) == IV.e_mm256_mul_epu32 (to_u32x8 a) (to_u32x8 b)) =
  op_lemma_from_lift_bin to_u32x8 from_u64x4 to_u64x4
    Avx2.e_mm256_mul_epu32 IV.e_mm256_mul_epu32 IVL.e_mm256_mul_epu32' rt_u64x4 a b

let lemma_mm256_srai_epi16 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_srai_epi16 v_IMM8 a) == IV.e_mm256_srai_epi16 v_IMM8 (to_i16x16 a)) =
  op_lemma_from_lift_un to_i16x16 from_i16x16 to_i16x16
    (Avx2.e_mm256_srai_epi16 v_IMM8) (IV.e_mm256_srai_epi16 v_IMM8) (IVL.e_mm256_srai_epi16' v_IMM8) rt_i16x16 a

let lemma_mm256_srai_epi32 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_srai_epi32 v_IMM8 a) == IV.e_mm256_srai_epi32 v_IMM8 (to_i32x8 a)) =
  op_lemma_from_lift_un to_i32x8 from_i32x8 to_i32x8
    (Avx2.e_mm256_srai_epi32 v_IMM8) (IV.e_mm256_srai_epi32 v_IMM8) (IVL.e_mm256_srai_epi32' v_IMM8) rt_i32x8 a

let lemma_mm256_srli_epi16 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_srli_epi16 v_IMM8 a) == IV.e_mm256_srli_epi16 v_IMM8 (to_i16x16 a)) =
  op_lemma_from_lift_un to_i16x16 from_i16x16 to_i16x16
    (Avx2.e_mm256_srli_epi16 v_IMM8) (IV.e_mm256_srli_epi16 v_IMM8) (IVL.e_mm256_srli_epi16' v_IMM8) rt_i16x16 a

let lemma_mm256_srli_epi32 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_srli_epi32 v_IMM8 a) == IV.e_mm256_srli_epi32 v_IMM8 (to_i32x8 a)) =
  op_lemma_from_lift_un to_i32x8 from_i32x8 to_i32x8
    (Avx2.e_mm256_srli_epi32 v_IMM8) (IV.e_mm256_srli_epi32 v_IMM8) (IVL.e_mm256_srli_epi32' v_IMM8) rt_i32x8 a

let lemma_mm_srli_epi64 (v_IMM8: i32) (a: bv128)
  : Lemma (to_i64x2 (Sse2.e_mm_srli_epi64 v_IMM8 a) == IV.e_mm_srli_epi64 v_IMM8 (to_i64x2 a)) =
  op_lemma_from_lift_un to_i64x2 from_i64x2 to_i64x2
    (Sse2.e_mm_srli_epi64 v_IMM8) (IV.e_mm_srli_epi64 v_IMM8) (IVL.e_mm_srli_epi64' v_IMM8) rt_i64x2 a

let lemma_mm256_slli_epi32 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_slli_epi32 v_IMM8 a) == IV.e_mm256_slli_epi32 v_IMM8 (to_i32x8 a)) =
  op_lemma_from_lift_un to_i32x8 from_i32x8 to_i32x8
    (Avx2.e_mm256_slli_epi32 v_IMM8) (IV.e_mm256_slli_epi32 v_IMM8) (IVL.e_mm256_slli_epi32' v_IMM8) rt_i32x8 a

let lemma_mm256_permute4x64_epi64 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_permute4x64_epi64 v_IMM8 a) == IV.e_mm256_permute4x64_epi64 v_IMM8 (to_i64x4 a)) =
  op_lemma_from_lift_un to_i64x4 from_i64x4 to_i64x4
    (Avx2.e_mm256_permute4x64_epi64 v_IMM8) (IV.e_mm256_permute4x64_epi64 v_IMM8) (IVL.e_mm256_permute4x64_epi64' v_IMM8) rt_i64x4 a

let lemma_mm256_unpackhi_epi64 (a: bv256) (b: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_unpackhi_epi64 a b) == IV.e_mm256_unpackhi_epi64 (to_i64x4 a) (to_i64x4 b)) =
  op_lemma_from_lift_bin to_i64x4 from_i64x4 to_i64x4
    Avx2.e_mm256_unpackhi_epi64 IV.e_mm256_unpackhi_epi64 IVL.e_mm256_unpackhi_epi64' rt_i64x4 a b

let lemma_mm256_unpacklo_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_unpacklo_epi32 a b) == IV.e_mm256_unpacklo_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_unpacklo_epi32 IV.e_mm256_unpacklo_epi32 IVL.e_mm256_unpacklo_epi32' rt_i32x8 a b

let lemma_mm256_unpackhi_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_unpackhi_epi32 a b) == IV.e_mm256_unpackhi_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_unpackhi_epi32 IV.e_mm256_unpackhi_epi32 IVL.e_mm256_unpackhi_epi32' rt_i32x8 a b

let lemma_mm256_cvtepi16_epi32 (a: bv128)
  : Lemma (to_i32x8 (Avx2.e_mm256_cvtepi16_epi32 a) == IV.e_mm256_cvtepi16_epi32 (to_i16x8 a)) =
  op_lemma_from_lift_un to_i16x8 from_i32x8 to_i32x8
    Avx2.e_mm256_cvtepi16_epi32 IV.e_mm256_cvtepi16_epi32 IVL.e_mm256_cvtepi16_epi32' rt_i32x8 a

let lemma_mm_packs_epi16 (a: bv128) (b: bv128)
  : Lemma (to_i8x16 (Sse2.e_mm_packs_epi16 a b) == IV.e_mm_packs_epi16 (to_i16x8 a) (to_i16x8 b)) =
  op_lemma_from_lift_bin to_i16x8 from_i8x16 to_i8x16
    Sse2.e_mm_packs_epi16 IV.e_mm_packs_epi16 IVL.e_mm_packs_epi16' rt_i8x16 a b

let lemma_mm256_packs_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_packs_epi32 a b) == IV.e_mm256_packs_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i16x16 to_i16x16
    Avx2.e_mm256_packs_epi32 IV.e_mm256_packs_epi32 IVL.e_mm256_packs_epi32' rt_i16x16 a b

let lemma_mm256_inserti128_si256 (v_IMM8: i32) (a: bv256) (b: bv128)
  : Lemma (to_i128x2 (Avx2.e_mm256_inserti128_si256 v_IMM8 a b) == IV.e_mm256_inserti128_si256 v_IMM8 (to_i128x2 a) (to_i128x1 b)) =
  IVL.e_mm256_inserti128_si256' v_IMM8 a b;
  rt_i128x2 (IV.e_mm256_inserti128_si256 v_IMM8 (to_i128x2 a) (to_i128x1 b))

let lemma_mm256_blend_epi16 (v_IMM8: i32) (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_blend_epi16 v_IMM8 a b) == IV.e_mm256_blend_epi16 v_IMM8 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i16x16 to_i16x16
    (Avx2.e_mm256_blend_epi16 v_IMM8) (IV.e_mm256_blend_epi16 v_IMM8) (IVL.e_mm256_blend_epi16' v_IMM8) rt_i16x16 a b

let lemma_mm256_blendv_ps (a: bv256) (b: bv256) (c: bv256)
  : Lemma (to_i32x8 (Avx.e_mm256_blendv_ps a b c) == IV.e_mm256_blendv_ps (to_i32x8 a) (to_i32x8 b) (to_i32x8 c)) =
  IVL.e_mm256_blendv_ps' a b c;
  rt_i32x8 (IV.e_mm256_blendv_ps (to_i32x8 a) (to_i32x8 b) (to_i32x8 c))

let lemma_mm_movemask_epi8 (a: bv128)
  : Lemma (Sse2.e_mm_movemask_epi8 a == IV.e_mm_movemask_epi8 (to_i8x16 a)) =
  IVL.e_mm_movemask_epi8' a

let lemma_mm256_srlv_epi64 (a: bv256) (b: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_srlv_epi64 a b) == IV.e_mm256_srlv_epi64 (to_i64x4 a) (to_i64x4 b)) =
  op_lemma_from_lift_bin to_i64x4 from_i64x4 to_i64x4
    Avx2.e_mm256_srlv_epi64 IV.e_mm256_srlv_epi64 IVL.e_mm256_srlv_epi64' rt_i64x4 a b

let lemma_mm_sllv_epi32 (a: bv128) (b: bv128)
  : Lemma (to_i32x4 (Avx2.e_mm_sllv_epi32 a b) == IV.e_mm_sllv_epi32 (to_i32x4 a) (to_i32x4 b)) =
  op_lemma_from_lift_bin to_i32x4 from_i32x4 to_i32x4
    Avx2.e_mm_sllv_epi32 IV.e_mm_sllv_epi32 IVL.e_mm_sllv_epi32' rt_i32x4 a b

let lemma_mm256_slli_epi64 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_slli_epi64 v_IMM8 a) == IV.e_mm256_slli_epi64 v_IMM8 (to_i64x4 a)) =
  op_lemma_from_lift_un to_i64x4 from_i64x4 to_i64x4
    (Avx2.e_mm256_slli_epi64 v_IMM8) (IV.e_mm256_slli_epi64 v_IMM8) (IVL.e_mm256_slli_epi64' v_IMM8) rt_i64x4 a

let lemma_mm256_bsrli_epi128 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i128x2 (Avx2.e_mm256_bsrli_epi128 v_IMM8 a) == IV.e_mm256_bsrli_epi128 v_IMM8 (to_i128x2 a)) =
  op_lemma_from_lift_un to_i128x2 from_i128x2 to_i128x2
    (Avx2.e_mm256_bsrli_epi128 v_IMM8) (IV.e_mm256_bsrli_epi128 v_IMM8) (IVL.e_mm256_bsrli_epi128' v_IMM8) rt_i128x2 a

let lemma_mm256_set1_epi64x (a: i64)
  : Lemma (to_i64x4 (Avx.e_mm256_set1_epi64x a) == IV.e_mm256_set1_epi64x a) =
  IVL.e_mm256_set1_epi64x' a;
  rt_i64x4 (IV.e_mm256_set1_epi64x a)

let lemma_mm256_set_epi64x (e3: i64) (e2: i64) (e1: i64) (e0: i64)
  : Lemma (to_i64x4 (Avx.e_mm256_set_epi64x e3 e2 e1 e0) == IV.e_mm256_set_epi64x e3 e2 e1 e0) =
  IVL.e_mm256_set_epi64x' e3 e2 e1 e0;
  rt_i64x4 (IV.e_mm256_set_epi64x e3 e2 e1 e0)

let lemma_mm256_unpacklo_epi64 (a: bv256) (b: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_unpacklo_epi64 a b) == IV.e_mm256_unpacklo_epi64 (to_i64x4 a) (to_i64x4 b)) =
  op_lemma_from_lift_bin to_i64x4 from_i64x4 to_i64x4
    Avx2.e_mm256_unpacklo_epi64 IV.e_mm256_unpacklo_epi64 IVL.e_mm256_unpacklo_epi64' rt_i64x4 a b

let lemma_mm256_permute2x128_si256 (v_IMM8: i32) (a: bv256) (b: bv256)
  : Lemma (to_i128x2 (Avx2.e_mm256_permute2x128_si256 v_IMM8 a b) == IV.e_mm256_permute2x128_si256 v_IMM8 (to_i128x2 a) (to_i128x2 b)) =
  op_lemma_from_lift_bin to_i128x2 from_i128x2 to_i128x2
    (Avx2.e_mm256_permute2x128_si256 v_IMM8) (IV.e_mm256_permute2x128_si256 v_IMM8) (IVL.e_mm256_permute2x128_si256' v_IMM8) rt_i128x2 a b

let lemma_mm_sub_epi16 (a: bv128) (b: bv128)
  : Lemma (to_i16x8 (Sse2.e_mm_sub_epi16 a b) == IV.e_mm_sub_epi16 (to_i16x8 a) (to_i16x8 b)) =
  op_lemma_from_lift_bin to_i16x8 from_i16x8 to_i16x8
    Sse2.e_mm_sub_epi16 IV.e_mm_sub_epi16 IVL.e_mm_sub_epi16' rt_i16x8 a b

let lemma_mm256_cmpeq_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_cmpeq_epi32 a b) == IV.e_mm256_cmpeq_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_cmpeq_epi32 IV.e_mm256_cmpeq_epi32 IVL.e_mm256_cmpeq_epi32' rt_i32x8 a b

let lemma_mm256_castsi256_si128 (a: bv256)
  : Lemma (Avx.e_mm256_castsi256_si128 a == IV.e_mm256_castsi256_si128 a) =
  IVL.e_mm256_castsi256_si128' a

let lemma_mm256_extracti128_si256 (v_IMM8: i32) (a: bv256)
  : Lemma (Avx2.e_mm256_extracti128_si256 v_IMM8 a == IV.e_mm256_extracti128_si256 v_IMM8 a) =
  IVL.e_mm256_extracti128_si256' v_IMM8 a

let lemma_mm256_slli_epi16 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_slli_epi16 v_IMM8 a) == IV.e_mm256_slli_epi16 v_IMM8 (to_i16x16 a)) =
  op_lemma_from_lift_un to_i16x16 from_i16x16 to_i16x16
    (Avx2.e_mm256_slli_epi16 v_IMM8) (IV.e_mm256_slli_epi16 v_IMM8) (IVL.e_mm256_slli_epi16' v_IMM8) rt_i16x16 a

let lemma_mm256_srli_epi64 (v_IMM8: i32) (a: bv256)
  : Lemma (to_i64x4 (Avx2.e_mm256_srli_epi64 v_IMM8 a) == IV.e_mm256_srli_epi64 v_IMM8 (to_i64x4 a)) =
  op_lemma_from_lift_un to_i64x4 from_i64x4 to_i64x4
    (Avx2.e_mm256_srli_epi64 v_IMM8) (IV.e_mm256_srli_epi64 v_IMM8) (IVL.e_mm256_srli_epi64' v_IMM8) rt_i64x4 a

let lemma_mm256_sllv_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_sllv_epi32 a b) == IV.e_mm256_sllv_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_sllv_epi32 IV.e_mm256_sllv_epi32 IVL.e_mm256_sllv_epi32' rt_i32x8 a b

let lemma_mm256_srlv_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_srlv_epi32 a b) == IV.e_mm256_srlv_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_srlv_epi32 IV.e_mm256_srlv_epi32 IVL.e_mm256_srlv_epi32' rt_i32x8 a b

let lemma_mm256_permutevar8x32_epi32 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_permutevar8x32_epi32 a b) == IV.e_mm256_permutevar8x32_epi32 (to_i32x8 a) (to_i32x8 b)) =
  op_lemma_from_lift_bin to_i32x8 from_i32x8 to_i32x8
    Avx2.e_mm256_permutevar8x32_epi32 IV.e_mm256_permutevar8x32_epi32 IVL.e_mm256_permutevar8x32_epi32' rt_i32x8 a b

let lemma_mm256_shuffle_epi8 (a: bv256) (b: bv256)
  : Lemma (to_i8x32 (Avx2.e_mm256_shuffle_epi8 a b) == IV.e_mm256_shuffle_epi8 (to_i8x32 a) (to_i8x32 b)) =
  op_lemma_from_lift_bin to_i8x32 from_i8x32 to_i8x32
    Avx2.e_mm256_shuffle_epi8 IV.e_mm256_shuffle_epi8 IVL.e_mm256_shuffle_epi8' rt_i8x32 a b

let lemma_mm_set_epi8 (e15: i8) (e14: i8) (e13: i8) (e12: i8) (e11: i8) (e10: i8) (e9: i8) (e8: i8) (e7: i8) (e6: i8) (e5: i8) (e4: i8) (e3: i8) (e2: i8) (e1: i8) (e0: i8)
  : Lemma (to_i8x16 (Sse2.e_mm_set_epi8 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) == IV.e_mm_set_epi8 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) =
  IVL.e_mm_set_epi8' e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0;
  rt_i8x16 (IV.e_mm_set_epi8 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0)

let lemma_mm256_set_epi8 (e31: i8) (e30: i8) (e29: i8) (e28: i8) (e27: i8) (e26: i8) (e25: i8) (e24: i8) (e23: i8) (e22: i8) (e21: i8) (e20: i8) (e19: i8) (e18: i8) (e17: i8) (e16: i8) (e15: i8) (e14: i8) (e13: i8) (e12: i8) (e11: i8) (e10: i8) (e9: i8) (e8: i8) (e7: i8) (e6: i8) (e5: i8) (e4: i8) (e3: i8) (e2: i8) (e1: i8) (e0: i8)
  : Lemma (to_i8x32 (Avx.e_mm256_set_epi8 e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) == IV.e_mm256_set_epi8 e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) =
  IVL.e_mm256_set_epi8' e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0;
  rt_i8x32 (IV.e_mm256_set_epi8 e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0)

let lemma_mm256_set_epi16 (e15: i16) (e14: i16) (e13: i16) (e12: i16) (e11: i16) (e10: i16) (e9: i16) (e8: i16) (e7: i16) (e6: i16) (e5: i16) (e4: i16) (e3: i16) (e2: i16) (e1: i16) (e0: i16)
  : Lemma (to_i16x16 (Avx.e_mm256_set_epi16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) == IV.e_mm256_set_epi16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) =
  IVL.e_mm256_set_epi16' e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0;
  rt_i16x16 (IV.e_mm256_set_epi16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0)

let lemma_mm256_set_epi32 (e7: i32) (e6: i32) (e5: i32) (e4: i32) (e3: i32) (e2: i32) (e1: i32) (e0: i32)
  : Lemma (to_i32x8 (Avx.e_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0) == IV.e_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0) =
  IVL.e_mm256_set_epi32' e7 e6 e5 e4 e3 e2 e1 e0;
  rt_i32x8 (IV.e_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0)

let lemma_mm_setzero_si128 (u: Prims.unit)
  : Lemma (Sse2.e_mm_setzero_si128 () == IV.e_mm_setzero_si128 ()) =
  IVL.e_mm_setzero_si128' ()

let lemma_mm_xor_si128 (a: bv128) (b: bv128)
  : Lemma (Sse2.e_mm_xor_si128 a b == IV.e_mm_xor_si128 a b) =
  IVL.e_mm_xor_si128' a b

let lemma_mm_shuffle_epi32 (v_IMM8: i32) (a: bv128)
  : Lemma (to_i32x4 (Sse2.e_mm_shuffle_epi32 v_IMM8 a) == IV.e_mm_shuffle_epi32 v_IMM8 (to_i32x4 a)) =
  op_lemma_from_lift_un to_i32x4 from_i32x4 to_i32x4
    (Sse2.e_mm_shuffle_epi32 v_IMM8) (IV.e_mm_shuffle_epi32 v_IMM8) (IVL.e_mm_shuffle_epi32' v_IMM8) rt_i32x4 a

let lemma_mm_unpackhi_epi64 (a: bv128) (b: bv128)
  : Lemma (to_i64x2 (Sse2.e_mm_unpackhi_epi64 a b) == IV.e_mm_unpackhi_epi64 (to_i64x2 a) (to_i64x2 b)) =
  op_lemma_from_lift_bin to_i64x2 from_i64x2 to_i64x2
    Sse2.e_mm_unpackhi_epi64 IV.e_mm_unpackhi_epi64 IVL.e_mm_unpackhi_epi64' rt_i64x2 a b

let lemma_mm_unpacklo_epi64 (a: bv128) (b: bv128)
  : Lemma (to_i64x2 (Sse2.e_mm_unpacklo_epi64 a b) == IV.e_mm_unpacklo_epi64 (to_i64x2 a) (to_i64x2 b)) =
  op_lemma_from_lift_bin to_i64x2 from_i64x2 to_i64x2
    Sse2.e_mm_unpacklo_epi64 IV.e_mm_unpacklo_epi64 IVL.e_mm_unpacklo_epi64' rt_i64x2 a b

let lemma_mm_slli_si128 (v_IMM8: i32) (a: bv128)
  : Lemma (to_i8x16 (Sse2.e_mm_slli_si128 v_IMM8 a) == IV.e_mm_slli_si128 v_IMM8 (to_i8x16 a)) =
  op_lemma_from_lift_un to_i8x16 from_i8x16 to_i8x16
    (Sse2.e_mm_slli_si128 v_IMM8) (IV.e_mm_slli_si128 v_IMM8) (IVL.e_mm_slli_si128' v_IMM8) rt_i8x16 a

let lemma_mm_srli_si128 (v_IMM8: i32) (a: bv128)
  : Lemma (to_i8x16 (Sse2.e_mm_srli_si128 v_IMM8 a) == IV.e_mm_srli_si128 v_IMM8 (to_i8x16 a)) =
  op_lemma_from_lift_un to_i8x16 from_i8x16 to_i8x16
    (Sse2.e_mm_srli_si128 v_IMM8) (IV.e_mm_srli_si128 v_IMM8) (IVL.e_mm_srli_si128' v_IMM8) rt_i8x16 a

let lemma_mm256_mullo_epi16 (a: bv256) (b: bv256)
  : Lemma (to_i16x16 (Avx2.e_mm256_mullo_epi16 a b) == IV.e_mm256_mullo_epi16 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i16x16 to_i16x16
    Avx2.e_mm256_mullo_epi16 IV.e_mm256_mullo_epi16 IVL.e_mm256_mullo_epi16' rt_i16x16 a b

let lemma_mm256_madd_epi16 (a: bv256) (b: bv256)
  : Lemma (to_i32x8 (Avx2.e_mm256_madd_epi16 a b) == IV.e_mm256_madd_epi16 (to_i16x16 a) (to_i16x16 b)) =
  op_lemma_from_lift_bin to_i16x16 from_i32x8 to_i32x8
    Avx2.e_mm256_madd_epi16 IV.e_mm256_madd_epi16 IVL.e_mm256_madd_epi16' rt_i32x8 a b

let lemma_mm_shuffle_epi8 (a: bv128) (b: bv128)
  : Lemma (to_i8x16 (Ssse3.e_mm_shuffle_epi8 a b) == IV.e_mm_shuffle_epi8 (to_i8x16 a) (to_i8x16 b)) =
  op_lemma_from_lift_bin to_i8x16 from_i8x16 to_i8x16
    Ssse3.e_mm_shuffle_epi8 IV.e_mm_shuffle_epi8 IVL.e_mm_shuffle_epi8' rt_i8x16 a b

(* ============================================================================
   movemask bit companion — bit i of the movemask == sign bit of lane i.
   Discharged over the core-models movemask FOLD model (`e_movemask_bit_sum_*`,
   Int_vec.fst, a base-2 LSB-first accumulation of lane sign bits) by digit
   extraction.  This is the bit-level shape the serialize proofs (serialize_1 /
   rejection_sample) need.
   ============================================================================ *)
module MLem = FStar.Math.Lemmas

(* per-lane sign-bit readers over the canonical i8/i32 lane views *)
let sign_bit8 (a: Funarr.t_FunArray (mk_u64 16) i8) (i: nat{i < 16}) : n: nat{n < 2} =
  if (a.[ mk_u64 i ] <: i8) <. mk_i8 0 then 1 else 0
let sign_bit32 (a: Funarr.t_FunArray (mk_u64 8) i32) (i: nat{i < 8}) : n: nat{n < 2} =
  if (a.[ mk_u64 i ] <: i32) <. mk_i32 0 then 1 else 0

(* digit extraction: bit (i-off) of the base-2 fold == lane i's sign bit *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let rec digit_i8 (a: Funarr.t_FunArray (mk_u64 16) i8) (off n: nat)
    (i: nat{off <= i /\ i < off + n /\ off + n <= 16})
    : Lemma (ensures (IV.e_movemask_bit_sum_i8 a off n / pow2 (i - off)) % 2 == sign_bit8 a i)
            (decreases n) =
  let b0 = (if (a.[ mk_u64 off ] <: i8) <. mk_i8 0 then 1 else 0) in
  let rest = IV.e_movemask_bit_sum_i8 a (off + 1) (n - 1) in
  if i = off then (assert_norm (pow2 0 == 1); MLem.lemma_mod_plus b0 rest 2; MLem.small_mod b0 2)
  else (let k = i - off in
        digit_i8 a (off + 1) (n - 1) i;
        MLem.lemma_div_plus b0 rest 2; MLem.small_div b0 2;
        MLem.pow2_plus 1 (k - 1);
        MLem.division_multiplication_lemma (IV.e_movemask_bit_sum_i8 a off n) 2 (pow2 (k - 1)))

let rec digit_i32 (a: Funarr.t_FunArray (mk_u64 8) i32) (off n: nat)
    (i: nat{off <= i /\ i < off + n /\ off + n <= 8})
    : Lemma (ensures (IV.e_movemask_bit_sum_i32 a off n / pow2 (i - off)) % 2 == sign_bit32 a i)
            (decreases n) =
  let b0 = (if (a.[ mk_u64 off ] <: i32) <. mk_i32 0 then 1 else 0) in
  let rest = IV.e_movemask_bit_sum_i32 a (off + 1) (n - 1) in
  if i = off then (assert_norm (pow2 0 == 1); MLem.lemma_mod_plus b0 rest 2; MLem.small_mod b0 2)
  else (let k = i - off in
        digit_i32 a (off + 1) (n - 1) i;
        MLem.lemma_div_plus b0 rest 2; MLem.small_div b0 2;
        MLem.pow2_plus 1 (k - 1);
        MLem.division_multiplication_lemma (IV.e_movemask_bit_sum_i32 a off n) 2 (pow2 (k - 1)))
#pop-options

(* bit i of the movemask MODEL value == sign bit of lane i of the FunArray view *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let movemask_epi8_model_bit (a: Funarr.t_FunArray (mk_u64 16) i8) (i: nat{i < 16})
    : Lemma ((v (IV.e_mm_movemask_epi8 a) / pow2 i) % 2 == sign_bit8 a i) =
  IV.e_movemask_bit_sum_i8_bound a 0 16; assert_norm (pow2 16 == 65536);
  digit_i8 a 0 16 i

let movemask_ps_model_bit (a: Funarr.t_FunArray (mk_u64 8) i32) (i: nat{i < 8})
    : Lemma ((v (IV.e_mm256_movemask_ps a) / pow2 i) % 2 == sign_bit32 a i) =
  IV.e_movemask_bit_sum_i32_bound a 0 8; assert_norm (pow2 8 == 256);
  digit_i32 a 0 8 i
#pop-options

(* DELIVERABLE bit companions over the TESTED Sse2/Avx movemask ops: bit i of the
   movemask == sign bit of lane i of the canonical `to_i8x16` / `to_i32x8` view. *)
let movemask_epi8_bit (a: bv128) (i: nat{i < 16})
    : Lemma ((v (Sse2.e_mm_movemask_epi8 a) / pow2 i) % 2 == sign_bit8 (to_i8x16 a) i) =
  lemma_mm_movemask_epi8 a;
  movemask_epi8_model_bit (to_i8x16 a) i

let movemask_ps_bit (a: bv256) (i: nat{i < 8})
    : Lemma ((v (Avx.e_mm256_movemask_ps a) / pow2 i) % 2 == sign_bit32 (to_i32x8 a) i) =
  lemma_mm256_movemask_ps a;
  movemask_ps_model_bit (to_i32x8 a) i

(* ============================================================================
   CROSS-WIDTH + BITWISE codec bridges (Phase-3 gap fills; crate-independent).

   Two facts the op-lemma set above did not cover, proven ONCE here from the
   concrete `Int_vec_interp` codec (`to_iv`/`decode_lane`/`dsum2`/`tc_of_u`):
     (Gap 1) bitwise `and` at the i16 lane view — core-models models
             `e_mm256_and_si256` at the raw `t_BitVec` level, so no i16-view
             op-lemma existed;
     (Gap 2) the i16-pair <-> native-i32 lane bridge — ml-kem's 32-bit
             intrinsics decode an i16 PAIR into an i32, which must agree with
             the canonical `to_i32x8` decode.
   ============================================================================ *)

(* ── generic dsum2 helpers (base-2 codec algebra) ─────────────────────────── *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
(* digit extraction: bit `i-off` of the base-2 fold `dsum2 f off n` is bval (f i). *)
let rec dsum2_digit (f: nat -> Bit.t_Bit) (off n: nat) (i: nat{off <= i /\ i < off + n})
    : Lemma (ensures (IVi.dsum2 f off n / pow2 (i - off)) % 2 == IVi.bval (f i)) (decreases n) =
  let b0 = IVi.bval (f off) in
  let rest = IVi.dsum2 f (off + 1) (n - 1) in
  if i = off then (assert_norm (pow2 0 == 1); MLem.lemma_mod_plus b0 rest 2; MLem.small_mod b0 2)
  else (let k = i - off in
        dsum2_digit f (off + 1) (n - 1) i;
        MLem.lemma_div_plus b0 rest 2; MLem.small_div b0 2;
        MLem.pow2_plus 1 (k - 1);
        MLem.division_multiplication_lemma (IVi.dsum2 f off n) 2 (pow2 (k - 1)))

(* split a base-2 fold at width n1. *)
let rec dsum2_split (f: nat -> Bit.t_Bit) (off n1 n2: nat)
    : Lemma (ensures IVi.dsum2 f off (n1 + n2) ==
                     IVi.dsum2 f off n1 + pow2 n1 * IVi.dsum2 f (off + n1) n2)
            (decreases n1) =
  if n1 = 0 then assert_norm (pow2 0 == 1)
  else (dsum2_split f (off + 1) (n1 - 1) n2; MLem.pow2_plus 1 (n1 - 1))

(* two folds that agree pointwise (up to an offset shift) are equal. *)
let rec dsum2_shift (f g: nat -> Bit.t_Bit) (o1 o2 n: nat)
    (h: (k: nat{k < n}) -> Lemma (IVi.bval (f (o1 + k)) == IVi.bval (g (o2 + k))))
    : Lemma (ensures IVi.dsum2 f o1 n == IVi.dsum2 g o2 n) (decreases n) =
  if n = 0 then ()
  else (h 0; dsum2_shift f g (o1 + 1) (o2 + 1) (n - 1) (fun k -> h (k + 1)))
#pop-options

(* ── decode-lane readback: get_bit b of the decoded lane == raw bit `w*i+b` ─── *)

(* get_bit b of a two's-complement-decoded unsigned `u` is digit b of `u`. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 100"
let lemma_get_bit_tc (t: Int.inttype) (u: nat) (b: nat{b < Int.bits t})
    : Lemma (requires u < pow2 (Int.bits t))
            (ensures Int.get_bit #t (mk_int #t (IVi.tc_of_u t u)) (mk_usize b) == (u / pow2 b) % 2) =
  IVi.lemma_tc_range t u;
  reveal_opaque (`%Rust_primitives.Integers.get_bit) (Rust_primitives.Integers.get_bit #t)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_readback (t: Int.inttype) (n m: u64) (bv: BV.t_BitVec n) (i: u64{v i < v m})
      (b: nat{b < Int.bits t})
    : Lemma (requires v n == v m * Int.bits t)
            (ensures IVi.bval (IVi.lane_reader n (Int.bits t) bv i b) ==
                     Int.get_bit #t (Funarr.impl_5__get m #(Int.int_t t) (IVi.to_iv t n m bv) i)
                                    (mk_usize b)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader n (Int.bits t) bv i in
  IVi.dsum2_bound reader 0 (Int.bits t);
  let u = IVi.dsum2 reader 0 (Int.bits t) in
  lemma_get_bit_tc t u b;
  dsum2_digit reader 0 (Int.bits t) b
#pop-options

(* ── Gap 1: bitwise `and` at the i16 lane view ────────────────────────────── *)

(* `.[]` on a t_BitVec is the underlying-FunArray get (SMT-provable). *)
let lemma_bv_index (bv: bv256) (k: u64{v k < 256})
    : Lemma ((bv.[ k ] <: Bit.t_Bit) == Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit bv._0 k) = ()

(* index of a t_BitVec built by `impl_9__from_fn` (SMT reduces the `on_domain`). *)
let lemma_impl9_index (f: (i: u64{v i < 256}) -> Bit.t_Bit) (k: u64{v k < 256})
    : Lemma (Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit
               (Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256)
                  #(u64 -> Bit.t_Bit) f)._0 k == f k) = ()

(* the underlying-FunArray value at `k` of the INTERPRETED `IV.e_mm256_and_si256`
   (a concrete `impl_9__from_fn`) is the bit-and of the operands' values at `k`. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_and_funarr (a b: bv256) (k: u64{v k < 256})
    : Lemma (Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit (IV.e_mm256_and_si256 a b)._0 k ==
             (match Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit a._0 k,
                    Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit b._0 k
              with
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_One
              | _ -> Bit.Bit_Zero)) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_One
              | _ -> Bit.Bit_Zero) in
  assert (IV.e_mm256_and_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_and_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  lemma_impl9_index f k;
  lemma_bv_index a k;
  lemma_bv_index b k
#pop-options

(* raw-bit semantics of `IV.e_mm256_and_si256`: bit `k` is bit-and of the operands. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_and_raw (a b: bv256) (ii: u64{v ii < 16}) (bb: nat{bb < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 16 (IV.e_mm256_and_si256 a b) ii bb) ==
             Int.bit_and (IVi.bval (IVi.lane_reader (mk_u64 256) 16 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 256) 16 b ii bb))) =
  assert (16 * v ii + bb < 256);
  lemma_and_funarr a b (mk_u64 (16 * v ii + bb))
#pop-options

(* i16-lane commutation for the INTERPRETED and: decode ∘ bitwise-and == `&.`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_and_i16x16_iv (a b: bv256) (i: nat{i < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 (IV.e_mm256_and_si256 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 i)) &.
              (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 b) (mk_u64 i)))) =
  let aANDb = IV.e_mm256_and_si256 a b in
  let ya : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 i) in
  let yb : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 b) (mk_u64 i) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 aANDb) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.I16 yr bb == Int.get_bit #Int.I16 (ya &. yb) bb) =
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) aANDb (mk_u64 i) (v bb);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) a (mk_u64 i) (v bb);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) b (mk_u64 i) (v bb);
    lemma_and_raw a b (mk_u64 i) (v bb);
    Int.get_bit_and #Int.I16 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 yr (ya &. yb)
#pop-options

(* LIFT: the hardware model `Avx2.e_mm256_and_si256` (an opaque, differentially-
   tested `assume val`) agrees with its bit-level interpretation `IV.e_mm256_and_si256`.
   This is a raw-`t_BitVec` lift AXIOM in the exact style of the ~149
   `Int_vec.Lemmas.e_mm256_OP'` lifts the rest of this module already rests on
   (cf. `e_mm_xor_si128'`); it is validated by the core-models differential tests.
   The i16-VIEW commutation on top of it (`lemma_and_i16x16_iv`) is PROVEN, so this
   is a strict trust REDUCTION vs. the previous whole-op `admit ()`. *)
(* AXIOM `lemma_and_si256_lift` MOVED to Libcrux_core_models.Trusted.Intrinsics. *)

(* Gap-1 deliverable: i16-lane view of the (hardware) `and`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_and_i16x16 (a b: bv256) (i: nat{i < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 (Avx2.e_mm256_and_si256 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 i)) &.
              (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 b) (mk_u64 i)))) =
  lemma_and_si256_lift a b;
  lemma_and_i16x16_iv a b i
#pop-options

(* ── Gap 2: i16-pair <-> native-i32 lane bridge ───────────────────────────── *)

(* lane value of the i16 / i32 codec views, as a two's-complement decode. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_to_i16_val (vec: bv256) (i: nat{i < 16})
    : Lemma (v (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 vec) (mk_u64 i)) ==
             IVi.tc_of_u Int.I16 (IVi.dsum2 (IVi.lane_reader (mk_u64 256) 16 vec (mk_u64 i)) 0 16)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 256) 16 vec (mk_u64 i) in
  IVi.dsum2_bound reader 0 16;
  IVi.lemma_tc_range Int.I16 (IVi.dsum2 reader 0 16)

let lemma_to_i32_val (vec: bv256) (j: nat{j < 8})
    : Lemma (v (Funarr.impl_5__get (mk_u64 8) #i32 (to_i32x8 vec) (mk_u64 j)) ==
             IVi.tc_of_u Int.I32 (IVi.dsum2 (IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j)) 0 32)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j) in
  IVi.dsum2_bound reader 0 32;
  IVi.lemma_tc_range Int.I32 (IVi.dsum2 reader 0 32)
#pop-options

(* the i32-lane readers and their two i16 half-lane readers agree bit-for-bit. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_reader_lo (vec: bv256) (j: nat{j < 8}) (k: nat{k < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j) k) ==
             IVi.bval (IVi.lane_reader (mk_u64 256) 16 vec (mk_u64 (2 * j)) k)) =
  assert (32 * j + k < 256)

let lemma_reader_hi (vec: bv256) (j: nat{j < 8}) (k: nat{k < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j) (16 + k)) ==
             IVi.bval (IVi.lane_reader (mk_u64 256) 16 vec (mk_u64 (2 * j + 1)) k)) =
  assert (32 * j + 16 + k < 256)
#pop-options

(* tc-of-u round-trips modulo its width. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 100"
let lemma_tc_mod (t: Int.inttype) (u: nat)
    : Lemma (requires u < pow2 (Int.bits t)) (ensures IVi.tc_of_u t u % pow2 (Int.bits t) == u) =
  if Int.signed t && u >= pow2 (Int.bits t - 1)
  then (MLem.lemma_mod_plus u (-1) (pow2 (Int.bits t)); MLem.small_mod u (pow2 (Int.bits t)))
  else MLem.small_mod u (pow2 (Int.bits t))
#pop-options

(* the arithmetic heart of the bridge: an i32 two's-complement value equals its
   low-16-unsigned + 2^16 * high-16-signed decomposition. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_tc_pair (u_lo u_hi: nat)
    : Lemma (requires u_lo < pow2 16 /\ u_hi < pow2 16)
            (ensures u_lo + pow2 16 * IVi.tc_of_u Int.I16 u_hi ==
                     IVi.tc_of_u Int.I32 (u_lo + pow2 16 * u_hi)) =
  assert_norm (Int.bits Int.I16 == 16);
  assert_norm (Int.bits Int.I32 == 32);
  assert_norm (Int.signed Int.I16);
  assert_norm (Int.signed Int.I32);
  assert_norm (pow2 32 == pow2 16 * pow2 16);
  assert_norm (pow2 31 == pow2 16 * pow2 15);
  assert_norm (pow2 15 < pow2 16);
  let u32 = u_lo + pow2 16 * u_hi in
  if u_hi >= pow2 15 then () else ()
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_lane32_bridge (vec: bv256) (j: nat{j < 8})
    : Lemma ((v (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 vec) (mk_u64 (2 * j))) % pow2 16) +
             pow2 16 * v (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 vec) (mk_u64 (2 * j + 1))) ==
             v (Funarr.impl_5__get (mk_u64 8) #i32 (to_i32x8 vec) (mk_u64 j))) =
  let reader16lo = IVi.lane_reader (mk_u64 256) 16 vec (mk_u64 (2 * j)) in
  let reader16hi = IVi.lane_reader (mk_u64 256) 16 vec (mk_u64 (2 * j + 1)) in
  let reader32 = IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j) in
  IVi.dsum2_bound reader16lo 0 16;
  IVi.dsum2_bound reader16hi 0 16;
  let u_lo = IVi.dsum2 reader16lo 0 16 in
  let u_hi = IVi.dsum2 reader16hi 0 16 in
  lemma_to_i16_val vec (2 * j);
  lemma_to_i16_val vec (2 * j + 1);
  lemma_to_i32_val vec j;
  dsum2_split reader32 0 16 16;
  dsum2_shift reader32 reader16lo 0 0 16 (fun k -> lemma_reader_lo vec j k);
  dsum2_shift reader32 reader16hi 16 0 16 (fun k -> lemma_reader_hi vec j k);
  lemma_tc_mod Int.I16 u_lo;
  lemma_tc_pair u_lo u_hi
#pop-options

(* ── lane values of the INTERPRETED i32-lane ops (for the ml-kem `*_epi32`) ─── *)

(* `rem_euclid` by 256 is the identity for a small non-negative shift. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_rem_euclid256 (imm: i32)
    : Lemma (requires v imm >= 0 /\ v imm < 256)
            (ensures Core_models.Num.impl_i32__rem_euclid imm (mk_i32 256) == imm) =
  assert_norm (v (mk_i32 256) == 256);
  FStar.Math.Lemmas.small_mod (v imm) 256
#pop-options

(* per-lane value of each i32-lane int op (unfold the intrinsic to `impl_5__from_fn`
   with `delta_only`; SMT does the `on_domain`/index round-trip + the `if` guard
   via the `rem_euclid` fact). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_iv_set1_epi32 (x: i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_set1_epi32 x) (mk_u64 j) == x) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_set1_epi32 x) (mk_u64 j) == x)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_set1_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_srai32 (imm: i32) (arr: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (requires v imm >= 0 /\ v imm < 32)
            (ensures Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_srai_epi32 imm arr) (mk_u64 j) ==
                     ((Funarr.impl_5__get (mk_u64 8) #i32 arr (mk_u64 j)) >>! imm)) =
  lemma_rem_euclid256 imm;
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_srai_epi32 imm arr) (mk_u64 j) ==
          ((Funarr.impl_5__get (mk_u64 8) #i32 arr (mk_u64 j)) >>! imm))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_srai_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_srli32 (imm: i32) (arr: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (requires v imm >= 0 /\ v imm < 32)
            (ensures Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_srli_epi32 imm arr) (mk_u64 j) ==
                     (cast ((cast (Funarr.impl_5__get (mk_u64 8) #i32 arr (mk_u64 j)) <: u32) >>! imm <: u32)
                      <: i32)) =
  lemma_rem_euclid256 imm;
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_srli_epi32 imm arr) (mk_u64 j) ==
          (cast ((cast (Funarr.impl_5__get (mk_u64 8) #i32 arr (mk_u64 j)) <: u32) >>! imm <: u32) <: i32))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_srli_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_slli32 (imm: i32) (arr: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (requires v imm >= 0 /\ v imm < 32)
            (ensures Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_slli_epi32 imm arr) (mk_u64 j) ==
                     (cast ((cast (Funarr.impl_5__get (mk_u64 8) #i32 arr (mk_u64 j)) <: u32) <<! imm <: u32)
                      <: i32)) =
  lemma_rem_euclid256 imm;
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_slli_epi32 imm arr) (mk_u64 j) ==
          (cast ((cast (Funarr.impl_5__get (mk_u64 8) #i32 arr (mk_u64 j)) <: u32) <<! imm <: u32) <: i32))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_slli_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ============================================================================
   Gap 3 — RAW-`t_BitVec` ops: bitwise xor, ground zero, 128->256 cast.

   These ops are *view-agnostic* (`t_BitVec -> t_BitVec`), so core-models'
   `mk_lift_lemma!` macro — keyed on a lane-view round-trip
   `from_X (op_int (to_X a))` — emits no lift for them, exactly as for
   `and_si256` (Gap 1 above).  Their lift is therefore stated here as an
   IDENTITY axiom in the same style and trust class: the hardware model
   (`Avx*.e_*`, an opaque `assume val`) agrees with the concrete bit-level
   interpretation (`IV.e_*`).  Each is validated by the core-models
   differential-test harness, which compares `int_vec::_mm256_OP` against the
   REAL hardware intrinsic over 1000 random inputs — see
   `crates/utils/core-models/src/core_arch/x86/interpretations.rs`:
     `mk!(_mm256_setzero_si256())`,
     `mk!(_mm256_xor_si256(a: BitVec, b: BitVec))`,
     `mk!(_mm256_castsi128_si256(a: BitVec))`.
   The i16-lane VIEW commutation on top of each is PROVEN below, so the trusted
   surface is exactly the (tested) raw-op identity — a strict trust REDUCTION
   vs. the whole-op view `admit ()`s these replace.
   ============================================================================ *)

(* AXIOMS `lemma_xor_si256_lift` / `lemma_setzero_si256_lift` /
   `lemma_castsi128_si256_lift` MOVED to Libcrux_core_models.Trusted.Intrinsics. *)

(* bit `w * ii + bb` of a 256-bit vector built by `impl_9__from_fn f` is `f` at
   that flat index (the `on_domain` round-trip; SMT-reducible). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_from_fn_lane_reader (f: (i: u64{v i < 256}) -> Bit.t_Bit)
      (w: nat{w > 0}) (ii: u64) (bb: nat{bb < w /\ w * v ii + bb < 256})
    : Lemma (IVi.lane_reader (mk_u64 256) w
               (Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256)
                  #(u64 -> Bit.t_Bit) f) ii bb
             == f (mk_u64 (w * v ii + bb))) =
  lemma_impl9_index f (mk_u64 (w * v ii + bb))
#pop-options

(* ── xor: raw-bit semantics, then the i16-lane commutation ─────────────────── *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_xor_raw (a b: bv256) (ii: u64{v ii < 16}) (bb: nat{bb < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 16 (IV.e_mm256_xor_si256 a b) ii bb) ==
             Int.bit_xor (IVi.bval (IVi.lane_reader (mk_u64 256) 16 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 256) 16 b ii bb))) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_Zero
              | _ -> Bit.Bit_One) in
  assert (IV.e_mm256_xor_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256)
            #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_xor_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (16 * v ii + bb < 256);
  lemma_from_fn_lane_reader f 16 ii bb;
  lemma_bv_index a (mk_u64 (16 * v ii + bb));
  lemma_bv_index b (mk_u64 (16 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_xor_i16x16_iv (a b: bv256) (i: nat{i < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 (IV.e_mm256_xor_si256 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 i)) ^.
              (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 b) (mk_u64 i)))) =
  let aXORb = IV.e_mm256_xor_si256 a b in
  let ya : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 i) in
  let yb : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 b) (mk_u64 i) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 aXORb) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.I16 yr bb == Int.get_bit #Int.I16 (ya ^. yb) bb) =
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) aXORb (mk_u64 i) (v bb);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) a (mk_u64 i) (v bb);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) b (mk_u64 i) (v bb);
    lemma_xor_raw a b (mk_u64 i) (v bb);
    Int.get_bit_xor #Int.I16 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 yr (ya ^. yb)
#pop-options

(* DELIVERABLE: i16-lane view of the (hardware) xor. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_xor_i16x16 (a b: bv256) (i: nat{i < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 (Avx2.e_mm256_xor_si256 a b)) (mk_u64 i) ==
             ((Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 i)) ^.
              (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 b) (mk_u64 i)))) =
  lemma_xor_si256_lift a b;
  lemma_xor_i16x16_iv a b i
#pop-options

(* ── setzero: every raw bit is 0, hence every i16 lane is 0 ────────────────── *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_setzero_raw (ii: u64{v ii < 16}) (bb: nat{bb < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 16 (IV.e_mm256_setzero_si256 ()) ii bb) == 0) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit = fun temp_0_ -> (let _:u64 = temp_0_ in Bit.Bit_Zero) in
  assert (IV.e_mm256_setzero_si256 () ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256)
            #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_setzero_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (16 * v ii + bb < 256);
  lemma_from_fn_lane_reader f 16 ii bb
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_setzero_i16x16_iv (i: nat{i < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 (IV.e_mm256_setzero_si256 ())) (mk_u64 i)
             == mk_i16 0) =
  let z = IV.e_mm256_setzero_si256 () in
  let yr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 z) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.I16 yr bb == Int.get_bit #Int.I16 (mk_i16 0) bb) =
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) z (mk_u64 i) (v bb);
    lemma_setzero_raw (mk_u64 i) (v bb);
    reveal_opaque (`%Rust_primitives.Integers.get_bit) (Rust_primitives.Integers.get_bit #Int.I16)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 yr (mk_i16 0)
#pop-options

(* DELIVERABLE: i16-lane view of the (hardware) setzero. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_setzero_i16x16 (i: nat{i < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 (Avx.e_mm256_setzero_si256 ())) (mk_u64 i)
             == mk_i16 0) =
  lemma_setzero_si256_lift ();
  lemma_setzero_i16x16_iv i
#pop-options

(* ── cast 128->256: the low 128 raw bits are preserved ─────────────────────── *)

(* `.[]` on a t_BitVec of ANY width is the underlying-FunArray get. *)
let lemma_bv_index_n (#n: u64) (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
      (k: u64{v k < v n})
    : Lemma ((bv.[ k ] <: Bit.t_Bit) == Funarr.impl_5__get n #Bit.t_Bit bv._0 k) = ()

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_castsi128_raw (a: bv128) (ii: u64{v ii < 8}) (bb: nat{bb < 16})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 16 (IV.e_mm256_castsi128_si256 a) ii bb) ==
             IVi.bval (IVi.lane_reader (mk_u64 128) 16 a ii bb)) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              if i <. mk_u64 128 then (a.[ i ] <: Bit.t_Bit) else Bit.Bit_Zero) in
  assert (IV.e_mm256_castsi128_si256 a ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256)
            #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_castsi128_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (16 * v ii + bb < 128);
  lemma_from_fn_lane_reader f 16 ii bb;
  lemma_bv_index_n #(mk_u64 128) a (mk_u64 (16 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_castsi128_i16_iv (a: bv128) (i: nat{i < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16
               (to_i16x16 (IV.e_mm256_castsi128_si256 a)) (mk_u64 i) ==
             Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 a) (mk_u64 i)) =
  let c = IV.e_mm256_castsi128_si256 a in
  let xr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 c) (mk_u64 i) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 a) (mk_u64 i) in
  let aux (bb: usize{v bb < 16})
      : Lemma (Int.get_bit #Int.I16 xr bb == Int.get_bit #Int.I16 yr bb) =
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) c (mk_u64 i) (v bb);
    lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) a (mk_u64 i) (v bb);
    lemma_castsi128_raw a (mk_u64 i) (v bb)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 xr yr
#pop-options

(* DELIVERABLE: i16-lane view of the (hardware) 128->256 cast. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_castsi128_i16x16 (a: bv128) (i: nat{i < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16
               (to_i16x16 (Avx.e_mm256_castsi128_si256 a)) (mk_u64 i) ==
             Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 a) (mk_u64 i)) =
  lemma_castsi128_si256_lift a;
  lemma_castsi128_i16_iv a i
#pop-options

(* ============================================================================
   Gap 4 — GENERIC SUB-LANE REFINEMENT: a wider-view op-lemma ⇒ i16-lane facts.

   A `w*r`-bit lane `q` of a bit-vector IS the concatenation of its `r` `w`-bit
   sub-lanes `r*q .. r*q+r-1`; the two lane readers coincide by pure index
   arithmetic (`lemma_reader_refine`, definitional).  Consequently, whenever a
   canonical op-lemma states a per-lane conclusion at a WIDER view
   (`to_i32x8` / `to_i64x4` / `to_i128x2`) and that conclusion is a
   PERMUTATION/SELECTION of source lanes, the corresponding i16-lane fact is
   PROVEN by `lemma_sublane_transfer_i16` — no per-op arithmetic, no new axiom.

   This is the single bridge that makes ml-kem's `get_lane`-permutation facts
   (shuffle_epi32 / permute4x64_epi64 / unpacklo/hi_* / inserti128 / blend)
   derivable from the canonical op-lemma set.  It is stated across two possibly
   DIFFERENT vector widths (`nx` / `ny`) so it also covers the 128<->256 ops.
   ============================================================================ *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_reader_refine (n: u64) (w: pos) (r: pos{r <= 32})
      (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
      (q: nat{q < 32}) (i: nat{i < r}) (b: nat{b < w})
    : Lemma (requires (w * r) * q + (w * i + b) < v n)
            (ensures IVi.lane_reader n (w * r) bv (mk_u64 q) (w * i + b) ==
                     IVi.lane_reader n w bv (mk_u64 (q * r + i)) b) =
  FStar.Math.Lemmas.paren_mul_right w r q;
  assert (w * i + b < w * r);
  assert ((w * r) * q + (w * i + b) == w * (q * r + i) + b)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_sublane_transfer_i16
      (t: Int.inttype) (r: pos{r <= 32})
      (nx ny mx my mx16 my16: u64)
      (x: Libcrux_core_models.Abstractions.Bitvec.t_BitVec nx)
      (y: Libcrux_core_models.Abstractions.Bitvec.t_BitVec ny)
      (q: nat{q < 32}) (q': nat{q' < 32}) (i: nat{i < r})
    : Lemma
      (requires
        Int.bits t == 16 * r /\
        v nx == v mx * Int.bits t /\ v ny == v my * Int.bits t /\
        v mx16 == r * v mx /\ v my16 == r * v my /\
        v nx == v mx16 * 16 /\ v ny == v my16 * 16 /\
        q < v mx /\ q' < v my /\
        Funarr.impl_5__get mx #(Int.int_t t) (IVi.to_iv t nx mx x) (mk_u64 q) ==
        Funarr.impl_5__get my #(Int.int_t t) (IVi.to_iv t ny my y) (mk_u64 q'))
      (ensures
        (r * q + i < v mx16 /\ r * q' + i < v my16) /\
        Funarr.impl_5__get mx16 #i16 (IVi.to_iv Int.I16 nx mx16 x) (mk_u64 (r * q + i)) ==
        Funarr.impl_5__get my16 #i16 (IVi.to_iv Int.I16 ny my16 y) (mk_u64 (r * q' + i))) =
  (* r*q + i < r*mx = mx16, and likewise for y *)
  FStar.Math.Lemmas.lemma_mult_le_left r (q + 1) (v mx);
  FStar.Math.Lemmas.lemma_mult_le_left r (q' + 1) (v my);
  assert (r * q + i < v mx16);
  assert (r * q' + i < v my16);
  (* each i16 lane's bits are the corresponding bits of the wide lane *)
  FStar.Math.Lemmas.lemma_mult_le_left (16 * r) (q + 1) (v mx);
  FStar.Math.Lemmas.lemma_mult_le_left (16 * r) (q' + 1) (v my);
  let xr : i16 = Funarr.impl_5__get mx16 #i16 (IVi.to_iv Int.I16 nx mx16 x) (mk_u64 (r * q + i)) in
  let yr : i16 = Funarr.impl_5__get my16 #i16 (IVi.to_iv Int.I16 ny my16 y) (mk_u64 (r * q' + i)) in
  let aux (b: usize{v b < 16}) : Lemma (Int.get_bit #Int.I16 xr b == Int.get_bit #Int.I16 yr b) =
    assert_norm (Int.bits Int.I16 == 16);
    assert ((16 * r) * q + (16 * i + v b) < v nx);
    assert ((16 * r) * q' + (16 * i + v b) < v ny);
    lemma_readback Int.I16 nx mx16 x (mk_u64 (r * q + i)) (v b);
    lemma_readback Int.I16 ny my16 y (mk_u64 (r * q' + i)) (v b);
    lemma_readback t nx mx x (mk_u64 q) (16 * i + v b);
    lemma_readback t ny my y (mk_u64 q') (16 * i + v b);
    lemma_reader_refine nx 16 r x q i (v b);
    lemma_reader_refine ny 16 r y q' i (v b)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 xr yr
#pop-options

(* ── per-lane values of the remaining INTERPRETED i16-lane ops ─────────────── *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_iv_srli16 (imm: i32) (arr: Funarr.t_FunArray (mk_u64 16) i16) (j: nat{j < 16})
    : Lemma (requires v imm >= 0 /\ v imm < 16)
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_srli_epi16 imm arr) (mk_u64 j) ==
                     (cast ((cast (Funarr.impl_5__get (mk_u64 16) #i16 arr (mk_u64 j)) <: u16)
                            >>! imm <: u16) <: i16)) =
  lemma_rem_euclid256 imm;
  assert (Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_srli_epi16 imm arr) (mk_u64 j) ==
          (cast ((cast (Funarr.impl_5__get (mk_u64 16) #i16 arr (mk_u64 j)) <: u16) >>! imm <: u16)
           <: i16))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_srli_epi16];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_set_epi16 (e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0: i16) (j: nat{j < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i16
               (IV.e_mm256_set_epi16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0)
               (mk_u64 j) ==
             (match j with
              | 0 -> e0 | 1 -> e1 | 2 -> e2 | 3 -> e3 | 4 -> e4 | 5 -> e5 | 6 -> e6 | 7 -> e7
              | 8 -> e8 | 9 -> e9 | 10 -> e10 | 11 -> e11 | 12 -> e12 | 13 -> e13 | 14 -> e14
              | _ -> e15)) =
  assert (Funarr.impl_5__get (mk_u64 16) #i16
            (IV.e_mm256_set_epi16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0)
            (mk_u64 j) ==
          (match j with
           | 0 -> e0 | 1 -> e1 | 2 -> e2 | 3 -> e3 | 4 -> e4 | 5 -> e5 | 6 -> e6 | 7 -> e7
           | 8 -> e8 | 9 -> e9 | 10 -> e10 | 11 -> e11 | 12 -> e12 | 13 -> e13 | 14 -> e14
           | _ -> e15))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_set_epi16];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ============================================================================
   Gap 5 — per-lane values of the remaining INTERPRETED ops.

   Same recipe as `lemma_iv_srai32` above: unfold the interpretation with
   `norm [delta_only …]` and let SMT do the `impl_5__from_fn` / `on_domain`
   index round-trip.  Consumers (the ml-kem / ml-dsa / sha3 lane companions)
   compose these with the canonical op-lemma for the same op to get a per-lane
   fact about the HARDWARE op.

   NOTE on the immediate: the interpretations read an 8-bit immediate with
   `(IMM8 >> 2m) % 4` (resp. `(IMM8 >> m) % 2`), which is only the intended
   selector when the immediate is in [0, 256) — outside that range the model's
   own index would leave the lane domain.  The const-generic immediates are
   always literals in that range, so these lemmas take it as a `requires`
   rather than claiming something unprovable (and possibly false) beyond it.
   ============================================================================ *)

(* the 2-bit control digit `m` of an 8-bit immediate. *)
let ctl2 (c: i32) (m: nat) : nat = (v c / pow2 (2 * m)) % 4

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_iv_add_epi32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_add_epi32 a b) (mk_u64 j) ==
             Core_models.Num.impl_i32__wrapping_add
               (Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 j))
               (Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j))) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_add_epi32 a b) (mk_u64 j) ==
          Core_models.Num.impl_i32__wrapping_add
            (Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 j))
            (Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_add_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_mullo_epi32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_mullo_epi32 a b) (mk_u64 j) ==
             (Core_models.Num.impl_i32__overflowing_mul
                (Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 j))
                (Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j)))._1) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_mullo_epi32 a b) (mk_u64 j) ==
          (Core_models.Num.impl_i32__overflowing_mul
             (Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 j))
             (Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j)))._1)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_mullo_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_cvtepi16_epi32 (a: Funarr.t_FunArray (mk_u64 8) i16) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_cvtepi16_epi32 a) (mk_u64 j) ==
             (cast (Funarr.impl_5__get (mk_u64 8) #i16 a (mk_u64 j)) <: i32)) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_cvtepi16_epi32 a) (mk_u64 j) ==
          (cast (Funarr.impl_5__get (mk_u64 8) #i16 a (mk_u64 j)) <: i32))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_cvtepi16_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_unpacklo_epi32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_unpacklo_epi32 a b) (mk_u64 j) ==
             (match j with
              | 0 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 0)
              | 1 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 0)
              | 2 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 1)
              | 3 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 1)
              | 4 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 4)
              | 5 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 4)
              | 6 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 5)
              | _ -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 5))) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_unpacklo_epi32 a b) (mk_u64 j) ==
          (match j with
           | 0 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 0)
           | 1 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 0)
           | 2 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 1)
           | 3 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 1)
           | 4 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 4)
           | 5 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 4)
           | 6 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 5)
           | _ -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 5)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_unpacklo_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_unpackhi_epi32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_unpackhi_epi32 a b) (mk_u64 j) ==
             (match j with
              | 0 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 2)
              | 1 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 2)
              | 2 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 3)
              | 3 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 3)
              | 4 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 6)
              | 5 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 6)
              | 6 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 7)
              | _ -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 7))) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_unpackhi_epi32 a b) (mk_u64 j) ==
          (match j with
           | 0 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 2)
           | 1 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 2)
           | 2 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 3)
           | 3 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 3)
           | 4 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 6)
           | 5 -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 6)
           | 6 -> Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 7)
           | _ -> Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 7)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_unpackhi_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_unpackhi_epi64 (a b: Funarr.t_FunArray (mk_u64 4) i64) (j: nat{j < 4})
    : Lemma (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_unpackhi_epi64 a b) (mk_u64 j) ==
             (match j with
              | 0 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 1)
              | 1 -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 1)
              | 2 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 3)
              | _ -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 3))) =
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_unpackhi_epi64 a b) (mk_u64 j) ==
          (match j with
           | 0 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 1)
           | 1 -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 1)
           | 2 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 3)
           | _ -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 3)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_unpackhi_epi64];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_shuffle_epi32 (c: i32) (x: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (requires v c >= 0 /\ v c < 256)
            (ensures 4 * (j / 4) + ctl2 c (j % 4) < 8 /\
                     Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_shuffle_epi32 c x) (mk_u64 j) ==
                     Funarr.impl_5__get (mk_u64 8) #i32 x (mk_u64 (4 * (j / 4) + ctl2 c (j % 4)))) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 2 == 4);
  assert_norm (pow2 4 == 16); assert_norm (pow2 6 == 64);
  assert (4 * (j / 4) + ctl2 c (j % 4) < 8);
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_shuffle_epi32 c x) (mk_u64 j) ==
          Funarr.impl_5__get (mk_u64 8) #i32 x (mk_u64 (4 * (j / 4) + ctl2 c (j % 4))))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_shuffle_epi32;
                                        `%ctl2];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_permute4x64_epi64 (c: i32) (a: Funarr.t_FunArray (mk_u64 4) i64) (j: nat{j < 4})
    : Lemma (requires v c >= 0 /\ v c < 256)
            (ensures ctl2 c j < 4 /\
                     Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_permute4x64_epi64 c a) (mk_u64 j) ==
                     Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 (ctl2 c j))) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 2 == 4);
  assert_norm (pow2 4 == 16); assert_norm (pow2 6 == 64);
  assert (ctl2 c j < 4);
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_permute4x64_epi64 c a) (mk_u64 j) ==
          Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 (ctl2 c j)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_permute4x64_epi64;
                                        `%ctl2];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_blend_epi16 (c: i32) (a b: Funarr.t_FunArray (mk_u64 16) i16) (j: nat{j < 16})
    : Lemma (requires v c >= 0 /\ v c < 256)
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_blend_epi16 c a b) (mk_u64 j) ==
                     (if ((v c / pow2 (j % 8)) % 2) = 0
                      then Funarr.impl_5__get (mk_u64 16) #i16 a (mk_u64 j)
                      else Funarr.impl_5__get (mk_u64 16) #i16 b (mk_u64 j))) =
  assert (Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_blend_epi16 c a b) (mk_u64 j) ==
          (if ((v c / pow2 (j % 8)) % 2) = 0
           then Funarr.impl_5__get (mk_u64 16) #i16 a (mk_u64 j)
           else Funarr.impl_5__get (mk_u64 16) #i16 b (mk_u64 j)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_blend_epi16];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_inserti128_si256 (c: i32) (a: Funarr.t_FunArray (mk_u64 2) i128)
      (b: Funarr.t_FunArray (mk_u64 1) i128) (j: nat{j < 2})
    : Lemma (requires v c >= 0 /\ v c < 256)
            (ensures Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_inserti128_si256 c a b) (mk_u64 j) ==
                     (if (v c) % 2 = 0
                      then (if j = 0 then Funarr.impl_5__get (mk_u64 1) #i128 b (mk_u64 0)
                                     else Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 1))
                      else (if j = 0 then Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 0)
                                     else Funarr.impl_5__get (mk_u64 1) #i128 b (mk_u64 0)))) =
  assert (Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_inserti128_si256 c a b) (mk_u64 j) ==
          (if (v c) % 2 = 0
           then (if j = 0 then Funarr.impl_5__get (mk_u64 1) #i128 b (mk_u64 0)
                          else Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 1))
           else (if j = 0 then Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 0)
                          else Funarr.impl_5__get (mk_u64 1) #i128 b (mk_u64 0))))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_inserti128_si256];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_packs_epi32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (k: nat{k < 16})
    : Lemma (ensures
               (let src : i32 =
                  (if k < 4 then Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 k)
                   else if k < 8 then Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 (k - 4))
                   else if k < 12 then Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 (k - 4))
                   else Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 (k - 8))) in
                Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_packs_epi32 a b) (mk_u64 k) ==
                (if v src > 32767 then mk_i16 32767
                 else if v src < (-32768) then mk_i16 (-32768)
                 else mk_i16 (v src)))) =
  let src : i32 =
    (if k < 4 then Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 k)
     else if k < 8 then Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 (k - 4))
     else if k < 12 then Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 (k - 4))
     else Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 (k - 8))) in
  assert (Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_packs_epi32 a b) (mk_u64 k) ==
          (if v src > 32767 then mk_i16 32767
           else if v src < (-32768) then mk_i16 (-32768)
           else mk_i16 (v src)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_packs_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── convenience instances of the sub-lane transfer at the widths ml-kem /
      ml-dsa / sha3 actually use (all conversions are `to_iv` one-liners). ──── *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
(* i32 lane q of `x` == i32 lane q' of `y`  ⟹  their 2 i16 sub-lanes agree. *)
let lemma_sub_i32_i16 (x y: bv256) (q: nat{q < 8}) (q': nat{q' < 8}) (i: nat{i < 2})
    : Lemma (requires Funarr.impl_5__get (mk_u64 8) #i32 (to_i32x8 x) (mk_u64 q) ==
                      Funarr.impl_5__get (mk_u64 8) #i32 (to_i32x8 y) (mk_u64 q'))
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 x) (mk_u64 (2 * q + i)) ==
                     Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 y) (mk_u64 (2 * q' + i))) =
  assert_norm (Int.bits Int.I32 == 32);
  lemma_sublane_transfer_i16 Int.I32 2 (mk_u64 256) (mk_u64 256) (mk_u64 8) (mk_u64 8)
    (mk_u64 16) (mk_u64 16) x y q q' i

(* i64 lane q of `x` == i64 lane q' of `y`  ⟹  their 4 i16 sub-lanes agree. *)
let lemma_sub_i64_i16 (x y: bv256) (q: nat{q < 4}) (q': nat{q' < 4}) (i: nat{i < 4})
    : Lemma (requires Funarr.impl_5__get (mk_u64 4) #i64 (to_i64x4 x) (mk_u64 q) ==
                      Funarr.impl_5__get (mk_u64 4) #i64 (to_i64x4 y) (mk_u64 q'))
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 x) (mk_u64 (4 * q + i)) ==
                     Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 y) (mk_u64 (4 * q' + i))) =
  assert_norm (Int.bits Int.I64 == 64);
  lemma_sublane_transfer_i16 Int.I64 4 (mk_u64 256) (mk_u64 256) (mk_u64 4) (mk_u64 4)
    (mk_u64 16) (mk_u64 16) x y q q' i

(* i128 lane q of a 256-bit `x` == i128 lane q' of a 256-bit `y`  ⟹  8 i16 sub-lanes. *)
let lemma_sub_i128_i16 (x y: bv256) (q: nat{q < 2}) (q': nat{q' < 2}) (i: nat{i < 8})
    : Lemma (requires Funarr.impl_5__get (mk_u64 2) #i128 (to_i128x2 x) (mk_u64 q) ==
                      Funarr.impl_5__get (mk_u64 2) #i128 (to_i128x2 y) (mk_u64 q'))
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 x) (mk_u64 (8 * q + i)) ==
                     Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 y) (mk_u64 (8 * q' + i))) =
  assert_norm (Int.bits Int.I128 == 128);
  lemma_sublane_transfer_i16 Int.I128 8 (mk_u64 256) (mk_u64 256) (mk_u64 2) (mk_u64 2)
    (mk_u64 16) (mk_u64 16) x y q q' i

(* i128 lane q of a 256-bit `x` == THE i128 lane of a 128-bit `y`  ⟹  8 i16 sub-lanes
   (cross vector width: the `inserti128` / `castsi128` shape). *)
let lemma_sub_i128_i16_128 (x: bv256) (y: bv128) (q: nat{q < 2}) (i: nat{i < 8})
    : Lemma (requires Funarr.impl_5__get (mk_u64 2) #i128 (to_i128x2 x) (mk_u64 q) ==
                      Funarr.impl_5__get (mk_u64 1) #i128 (to_i128x1 y) (mk_u64 0))
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 x) (mk_u64 (8 * q + i)) ==
                     Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 y) (mk_u64 i)) =
  assert_norm (Int.bits Int.I128 == 128);
  lemma_sublane_transfer_i16 Int.I128 8 (mk_u64 256) (mk_u64 128) (mk_u64 2) (mk_u64 1)
    (mk_u64 16) (mk_u64 8) x y q 0 i
#pop-options

(* per-lane VALUE of the interpreted madd (the i16-pair dot product, wrapped). *)
(* #restart-solver: clears accumulated Z3 state from earlier module queries — this
   nonlinear madd goal proves at rlimit ~28 in isolation but saturates at 300 in-module
   without a fresh solver (admit_except-passes/full-build-fails; solver-state pollution). *)
#restart-solver
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_madd_epi16 (a b: Funarr.t_FunArray (mk_u64 16) i16) (j: nat{j < 8})
    : Lemma (v (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_madd_epi16 a b) (mk_u64 j)) ==
             ((v (Funarr.impl_5__get (mk_u64 16) #i16 a (mk_u64 (2 * j))) *
               v (Funarr.impl_5__get (mk_u64 16) #i16 b (mk_u64 (2 * j))) +
               v (Funarr.impl_5__get (mk_u64 16) #i16 a (mk_u64 (2 * j + 1))) *
               v (Funarr.impl_5__get (mk_u64 16) #i16 b (mk_u64 (2 * j + 1)))) @% pow2 32)) =
  assert_norm (pow2 32 == 4294967296);
  assert (v (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_madd_epi16 a b) (mk_u64 j)) ==
          ((v (Funarr.impl_5__get (mk_u64 16) #i16 a (mk_u64 (2 * j))) *
            v (Funarr.impl_5__get (mk_u64 16) #i16 b (mk_u64 (2 * j))) +
            v (Funarr.impl_5__get (mk_u64 16) #i16 a (mk_u64 (2 * j + 1))) *
            v (Funarr.impl_5__get (mk_u64 16) #i16 b (mk_u64 (2 * j + 1)))) @% pow2 32))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_madd_epi16;
                                        `%Core_models.Num.impl_i32__wrapping_add;
                                        (* impl_i32__wrapping_add is re-exported as
                                           Bundle.impl_14__wrapping_add — norm needs this extra hop *)
                                        `%Core_models.Bundle.impl_14__wrapping_add;
                                        `%Rust_primitives.Arithmetic.wrapping_add_i32;
                                        `%Rust_primitives.Integers.add_mod];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── byte-granular ops (`shuffle_epi8`) ─────────────────────────────────────── *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_set_epi8 (e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0: i8) (j: nat{j < 32})
    : Lemma (Funarr.impl_5__get (mk_u64 32) #i8
               (IV.e_mm256_set_epi8 e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) (mk_u64 j) ==
             (match j with
              | 0 -> e0
              | 1 -> e1
              | 2 -> e2
              | 3 -> e3
              | 4 -> e4
              | 5 -> e5
              | 6 -> e6
              | 7 -> e7
              | 8 -> e8
              | 9 -> e9
              | 10 -> e10
              | 11 -> e11
              | 12 -> e12
              | 13 -> e13
              | 14 -> e14
              | 15 -> e15
              | 16 -> e16
              | 17 -> e17
              | 18 -> e18
              | 19 -> e19
              | 20 -> e20
              | 21 -> e21
              | 22 -> e22
              | 23 -> e23
              | 24 -> e24
              | 25 -> e25
              | 26 -> e26
              | 27 -> e27
              | 28 -> e28
              | 29 -> e29
              | 30 -> e30
              | _ -> e31)) =
  assert (Funarr.impl_5__get (mk_u64 32) #i8
            (IV.e_mm256_set_epi8 e31 e30 e29 e28 e27 e26 e25 e24 e23 e22 e21 e20 e19 e18 e17 e16 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) (mk_u64 j) ==
          (match j with
              | 0 -> e0
              | 1 -> e1
              | 2 -> e2
              | 3 -> e3
              | 4 -> e4
              | 5 -> e5
              | 6 -> e6
              | 7 -> e7
              | 8 -> e8
              | 9 -> e9
              | 10 -> e10
              | 11 -> e11
              | 12 -> e12
              | 13 -> e13
              | 14 -> e14
              | 15 -> e15
              | 16 -> e16
              | 17 -> e17
              | 18 -> e18
              | 19 -> e19
              | 20 -> e20
              | 21 -> e21
              | 22 -> e22
              | 23 -> e23
              | 24 -> e24
              | 25 -> e25
              | 26 -> e26
              | 27 -> e27
              | 28 -> e28
              | 29 -> e29
              | 30 -> e30
              | _ -> e31))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_set_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

(* the high bit of an in-range (0..127) byte is clear, so PSHUFB takes the SELECT
   branch rather than the zeroing branch. *)
let lemma_u8_high_bit_clear (x: u8)
    : Lemma (requires v x < 128) (ensures (x &. mk_u8 128) == mk_u8 0) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 1 == 2); assert_norm (pow2 2 == 4);
  assert_norm (pow2 3 == 8); assert_norm (pow2 4 == 16); assert_norm (pow2 5 == 32);
  assert_norm (pow2 6 == 64); assert_norm (pow2 7 == 128);
  let aux (b: usize{v b < 8})
      : Lemma (Int.get_bit #Int.U8 (x &. mk_u8 128) b == Int.get_bit #Int.U8 (mk_u8 0) b) =
    Int.get_bit_and #Int.U8 x (mk_u8 128) b;
    reveal_opaque (`%Rust_primitives.Integers.get_bit) (Rust_primitives.Integers.get_bit #Int.U8)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U8 (x &. mk_u8 128) (mk_u8 0)

(* PSHUFB, select branch: byte `i` of the result is byte `16*(i/16) + idx%16` of the
   source, where `idx` is the (non-negative) index byte. *)
let lemma_iv_shuffle_epi8_sel (a b: Funarr.t_FunArray (mk_u64 32) i8) (i: nat{i < 32})
    : Lemma (requires v (Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i)) >= 0)
            (ensures
              (let idx : nat = v (Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i)) in
               16 * (i / 16) + idx % 16 < 32 /\
               Funarr.impl_5__get (mk_u64 32) #i8 (IV.e_mm256_shuffle_epi8 a b) (mk_u64 i) ==
               Funarr.impl_5__get (mk_u64 32) #i8 a (mk_u64 (16 * (i / 16) + idx % 16)))) =
  let bi = Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i) in
  let idx : nat = v bi in
  assert (v (cast bi <: u8) == idx);
  lemma_u8_high_bit_clear (cast bi <: u8);
  assert (16 * (i / 16) + idx % 16 < 32);
  assert (Funarr.impl_5__get (mk_u64 32) #i8 (IV.e_mm256_shuffle_epi8 a b) (mk_u64 i) ==
          Funarr.impl_5__get (mk_u64 32) #i8 a (mk_u64 (16 * (i / 16) + idx % 16)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_shuffle_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── the UPWARD direction of the sub-lane bridge (i8 pair -> i16 lane) ────────
   `lemma_sublane_transfer_i16` goes wide -> i16 (a permutation stated at
   i32/i64/i128 granularity yields i16 lane facts).  The BYTE-granular ops
   (`shuffle_epi8`) need the converse: if the two i8 sub-lanes of an i16 lane
   agree, the i16 lane agrees.  Same three ingredients (reader refinement,
   `lemma_readback`, bit extensionality), read in the other direction. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_i16_from_i8_pair (x y: bv256) (k: nat{k < 16}) (k': nat{k' < 16})
    : Lemma (requires
               Funarr.impl_5__get (mk_u64 32) #i8 (to_i8x32 x) (mk_u64 (2 * k)) ==
               Funarr.impl_5__get (mk_u64 32) #i8 (to_i8x32 y) (mk_u64 (2 * k')) /\
               Funarr.impl_5__get (mk_u64 32) #i8 (to_i8x32 x) (mk_u64 (2 * k + 1)) ==
               Funarr.impl_5__get (mk_u64 32) #i8 (to_i8x32 y) (mk_u64 (2 * k' + 1)))
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 x) (mk_u64 k) ==
                     Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 y) (mk_u64 k')) =
  assert_norm (Int.bits Int.I8 == 8);
  assert_norm (Int.bits Int.I16 == 16);
  let xr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 x) (mk_u64 k) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 y) (mk_u64 k') in
  let aux (b: usize{v b < 16}) : Lemma (Int.get_bit #Int.I16 xr b == Int.get_bit #Int.I16 yr b) =
    let h : nat = (v b) / 8 in
    let l : nat = (v b) % 8 in
    assert (v b == 8 * h + l);
    (* the i16 lane's bit `8h+l` IS bit `l` of its i8 sub-lane `2k+h` *)
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) x (mk_u64 k) (v b);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) y (mk_u64 k') (v b);
    lemma_readback Int.I8 (mk_u64 256) (mk_u64 32) x (mk_u64 (2 * k + h)) l;
    lemma_readback Int.I8 (mk_u64 256) (mk_u64 32) y (mk_u64 (2 * k' + h)) l;
    lemma_reader_refine (mk_u64 256) 8 2 x k h l;
    lemma_reader_refine (mk_u64 256) 8 2 y k' h l
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 xr yr
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_set_epi32 (e7 e6 e5 e4 e3 e2 e1 e0: i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0)
               (mk_u64 j) ==
             (match j with
              | 0 -> e0 | 1 -> e1 | 2 -> e2 | 3 -> e3
              | 4 -> e4 | 5 -> e5 | 6 -> e6 | _ -> e7)) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0)
            (mk_u64 j) ==
          (match j with
           | 0 -> e0 | 1 -> e1 | 2 -> e2 | 3 -> e3
           | 4 -> e4 | 5 -> e5 | 6 -> e6 | _ -> e7))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_set_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ============================================================================
   Unsigned 32/64 lane bridges — the `mul_epu32` shape.

   `lemma_mm256_mul_epu32` above is stated at the `to_u64x4` view over
   `to_u32x8` operands, so it is the ONE ml-kem lane fact that crosses BOTH a
   signedness change and a width change.  Two small codec facts close the gap
   to the i16/i32 views the ml-kem companion uses:

     (a) the u32 lane view is the i32 lane view reduced mod 2^32 — the SAME
         `lane_reader`, only `tc_of_u` differs (`lemma_tc_mod`);
     (b) a u64 lane is the base-2^32 concatenation of its two u32 sub-lanes —
         the unsigned analogue of `lemma_lane32_bridge` (no `tc_pair` step is
         needed, since `tc_of_u` is the identity on unsigned widths).
   ============================================================================ *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_to_u32_val (vec: bv256) (j: nat{j < 8})
    : Lemma (v (Funarr.impl_5__get (mk_u64 8) #u32 (to_u32x8 vec) (mk_u64 j)) ==
             IVi.tc_of_u Int.U32 (IVi.dsum2 (IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j)) 0 32)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j) in
  IVi.dsum2_bound reader 0 32;
  IVi.lemma_tc_range Int.U32 (IVi.dsum2 reader 0 32)

let lemma_to_u64_val (vec: bv256) (i: nat{i < 4})
    : Lemma (v (Funarr.impl_5__get (mk_u64 4) #u64 (to_u64x4 vec) (mk_u64 i)) ==
             IVi.tc_of_u Int.U64 (IVi.dsum2 (IVi.lane_reader (mk_u64 256) 64 vec (mk_u64 i)) 0 64)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 256) 64 vec (mk_u64 i) in
  IVi.dsum2_bound reader 0 64;
  IVi.lemma_tc_range Int.U64 (IVi.dsum2 reader 0 64)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_u32_of_i32 (vec: bv256) (j: nat{j < 8})
    : Lemma (v (Funarr.impl_5__get (mk_u64 8) #u32 (to_u32x8 vec) (mk_u64 j)) ==
             (v (Funarr.impl_5__get (mk_u64 8) #i32 (to_i32x8 vec) (mk_u64 j))) % pow2 32) =
  assert_norm (Int.bits Int.U32 == 32);
  assert_norm (Int.bits Int.I32 == 32);
  assert_norm (~(Int.signed Int.U32));
  let reader = IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 j) in
  IVi.dsum2_bound reader 0 32;
  let u = IVi.dsum2 reader 0 32 in
  lemma_to_u32_val vec j;
  lemma_to_i32_val vec j;
  lemma_tc_mod Int.U32 u;
  lemma_tc_mod Int.I32 u
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_u64_concat32 (vec: bv256) (i: nat{i < 4})
    : Lemma (v (Funarr.impl_5__get (mk_u64 4) #u64 (to_u64x4 vec) (mk_u64 i)) ==
             v (Funarr.impl_5__get (mk_u64 8) #u32 (to_u32x8 vec) (mk_u64 (2 * i))) +
             pow2 32 * v (Funarr.impl_5__get (mk_u64 8) #u32 (to_u32x8 vec) (mk_u64 (2 * i + 1)))) =
  assert_norm (Int.bits Int.U32 == 32);
  assert_norm (Int.bits Int.U64 == 64);
  let reader64 = IVi.lane_reader (mk_u64 256) 64 vec (mk_u64 i) in
  let readerLo = IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 (2 * i)) in
  let readerHi = IVi.lane_reader (mk_u64 256) 32 vec (mk_u64 (2 * i + 1)) in
  IVi.dsum2_bound reader64 0 64;
  IVi.dsum2_bound readerLo 0 32;
  IVi.dsum2_bound readerHi 0 32;
  lemma_to_u64_val vec i;
  lemma_to_u32_val vec (2 * i);
  lemma_to_u32_val vec (2 * i + 1);
  lemma_tc_mod Int.U64 (IVi.dsum2 reader64 0 64);
  lemma_tc_mod Int.U32 (IVi.dsum2 readerLo 0 32);
  lemma_tc_mod Int.U32 (IVi.dsum2 readerHi 0 32);
  dsum2_split reader64 0 32 32;
  dsum2_shift reader64 readerLo 0 0 32
    (fun k -> lemma_reader_refine (mk_u64 256) 32 2 vec i 0 k);
  dsum2_shift reader64 readerHi 32 0 32
    (fun k -> lemma_reader_refine (mk_u64 256) 32 2 vec i 1 k)
#pop-options

(* per-lane value of the interpreted unsigned 32x32->64 multiply. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_iv_mul_epu32 (a b: Funarr.t_FunArray (mk_u64 8) u32) (i: nat{i < 4})
    : Lemma (v (Funarr.impl_5__get (mk_u64 4) #u64 (IV.e_mm256_mul_epu32 a b) (mk_u64 i)) ==
             v (Funarr.impl_5__get (mk_u64 8) #u32 a (mk_u64 (2 * i))) *
             v (Funarr.impl_5__get (mk_u64 8) #u32 b (mk_u64 (2 * i)))) =
  assert (v (Funarr.impl_5__get (mk_u64 4) #u64 (IV.e_mm256_mul_epu32 a b) (mk_u64 i)) ==
          v (Funarr.impl_5__get (mk_u64 8) #u32 a (mk_u64 (2 * i))) *
          v (Funarr.impl_5__get (mk_u64 8) #u32 b (mk_u64 (2 * i))))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_mul_epu32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── 128-bit twins of the two byte-granular PSHUFB ingredients ────────────────
   The 256-bit versions above (`lemma_iv_shuffle_epi8_sel`, `lemma_i16_from_i8_pair`)
   retired the hand-written 256-bit PSHUFB semantics axiom.  ml-kem's rejection
   sampling needs the same pair at 128 bits.  Note the 128-bit model has NO
   half-lane base: `e_mm_shuffle_epi8` selects `vector.[idx % 16]` outright, where
   the 256-bit one selects `16*(i/16) + idx%16`.  So the `sel` statement is
   strictly simpler than its 256-bit counterpart, not merely narrower. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_mm_shuffle_epi8_sel (a b: Funarr.t_FunArray (mk_u64 16) i8) (i: nat{i < 16})
    : Lemma (requires v (Funarr.impl_5__get (mk_u64 16) #i8 b (mk_u64 i)) >= 0)
            (ensures
              (let idx : nat = v (Funarr.impl_5__get (mk_u64 16) #i8 b (mk_u64 i)) in
               idx % 16 < 16 /\
               Funarr.impl_5__get (mk_u64 16) #i8 (IV.e_mm_shuffle_epi8 a b) (mk_u64 i) ==
               Funarr.impl_5__get (mk_u64 16) #i8 a (mk_u64 (idx % 16)))) =
  let bi = Funarr.impl_5__get (mk_u64 16) #i8 b (mk_u64 i) in
  let idx : nat = v bi in
  assert (v (cast bi <: u8) == idx);
  lemma_u8_high_bit_clear (cast bi <: u8);
  assert (idx % 16 < 16);
  assert (Funarr.impl_5__get (mk_u64 16) #i8 (IV.e_mm_shuffle_epi8 a b) (mk_u64 i) ==
          Funarr.impl_5__get (mk_u64 16) #i8 a (mk_u64 (idx % 16)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm_shuffle_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_i16x8_from_i8_pair (x y: bv128) (k: nat{k < 8}) (k': nat{k' < 8})
    : Lemma (requires
               Funarr.impl_5__get (mk_u64 16) #i8 (to_i8x16 x) (mk_u64 (2 * k)) ==
               Funarr.impl_5__get (mk_u64 16) #i8 (to_i8x16 y) (mk_u64 (2 * k')) /\
               Funarr.impl_5__get (mk_u64 16) #i8 (to_i8x16 x) (mk_u64 (2 * k + 1)) ==
               Funarr.impl_5__get (mk_u64 16) #i8 (to_i8x16 y) (mk_u64 (2 * k' + 1)))
            (ensures Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 x) (mk_u64 k) ==
                     Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 y) (mk_u64 k')) =
  assert_norm (Int.bits Int.I8 == 8);
  assert_norm (Int.bits Int.I16 == 16);
  let xr : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 x) (mk_u64 k) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 y) (mk_u64 k') in
  let aux (b: usize{v b < 16}) : Lemma (Int.get_bit #Int.I16 xr b == Int.get_bit #Int.I16 yr b) =
    let h : nat = (v b) / 8 in
    let l : nat = (v b) % 8 in
    assert (v b == 8 * h + l);
    lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) x (mk_u64 k) (v b);
    lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) y (mk_u64 k') (v b);
    lemma_readback Int.I8 (mk_u64 128) (mk_u64 16) x (mk_u64 (2 * k + h)) l;
    lemma_readback Int.I8 (mk_u64 128) (mk_u64 16) y (mk_u64 (2 * k' + h)) l;
    lemma_reader_refine (mk_u64 128) 8 2 x k h l;
    lemma_reader_refine (mk_u64 128) 8 2 y k' h l
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 xr yr
#pop-options

(* ============================================================================
   Per-lane facts for the remaining AVX2 serialize/deserialize register ops —
   the variable 32-bit left shift, the 64-bit immediate right shift, the 8x32
   lane permute, and the 128-bit `set_epi8` twin.  Same recipes as their
   siblings above (`delta_only` norm + smt for the from_fn index round-trip;
   the set_epi8 twin mirrors `lemma_iv_set_epi8` at half width).  All PROVEN —
   no new trust.
   ============================================================================ *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_iv_sllv_epi32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_sllv_epi32 a b) (mk_u64 j) ==
             (let bj = Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j) in
              if (bj >. mk_i32 31) || (bj <. mk_i32 0)
              then mk_i32 0
              else cast ((cast (Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 j)) <: u32) <<! bj <: u32)
                   <: i32)) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_sllv_epi32 a b) (mk_u64 j) ==
          (let bj = Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j) in
           if (bj >. mk_i32 31) || (bj <. mk_i32 0)
           then mk_i32 0
           else cast ((cast (Funarr.impl_5__get (mk_u64 8) #i32 a (mk_u64 j)) <: u32) <<! bj <: u32)
                <: i32))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_sllv_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_srli64 (imm: i32) (arr: Funarr.t_FunArray (mk_u64 4) i64) (j: nat{j < 4})
    : Lemma (requires v imm >= 0 /\ v imm < 64)
            (ensures Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_srli_epi64 imm arr) (mk_u64 j) ==
                     (cast ((cast (Funarr.impl_5__get (mk_u64 4) #i64 arr (mk_u64 j)) <: u64) >>! imm <: u64)
                      <: i64)) =
  lemma_rem_euclid256 imm;
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_srli_epi64 imm arr) (mk_u64 j) ==
          (cast ((cast (Funarr.impl_5__get (mk_u64 4) #i64 arr (mk_u64 j)) <: u64) >>! imm <: u64) <: i64))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_srli_epi64];
                            iota; zeta; primops];
        FStar.Tactics.smt ())

let lemma_iv_permutevar8x32 (a b: Funarr.t_FunArray (mk_u64 8) i32) (j: nat{j < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_permutevar8x32_epi32 a b) (mk_u64 j) ==
             (Funarr.impl_5__get (mk_u64 8) #i32 a
                ((cast (Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j)) <: u64) %! mk_u64 8))) =
  assert (Funarr.impl_5__get (mk_u64 8) #i32 (IV.e_mm256_permutevar8x32_epi32 a b) (mk_u64 j) ==
          (Funarr.impl_5__get (mk_u64 8) #i32 a
             ((cast (Funarr.impl_5__get (mk_u64 8) #i32 b (mk_u64 j)) <: u64) %! mk_u64 8)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_permutevar8x32_epi32];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_mm_set_epi8 (e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0: i8) (j: nat{j < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i8
               (IV.e_mm_set_epi8 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) (mk_u64 j) ==
             (match j with
              | 0 -> e0
              | 1 -> e1
              | 2 -> e2
              | 3 -> e3
              | 4 -> e4
              | 5 -> e5
              | 6 -> e6
              | 7 -> e7
              | 8 -> e8
              | 9 -> e9
              | 10 -> e10
              | 11 -> e11
              | 12 -> e12
              | 13 -> e13
              | 14 -> e14
              | 15 -> e15)) =
  assert (Funarr.impl_5__get (mk_u64 16) #i8
            (IV.e_mm_set_epi8 e15 e14 e13 e12 e11 e10 e9 e8 e7 e6 e5 e4 e3 e2 e1 e0) (mk_u64 j) ==
          (match j with
           | 0 -> e0
           | 1 -> e1
           | 2 -> e2
           | 3 -> e3
           | 4 -> e4
           | 5 -> e5
           | 6 -> e6
           | 7 -> e7
           | 8 -> e8
           | 9 -> e9
           | 10 -> e10
           | 11 -> e11
           | 12 -> e12
           | 13 -> e13
           | 14 -> e14
           | 15 -> e15))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm_set_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── mm_packs_epi16 per-lane fact, the 128-half i16-lane transfers
   (castsi256_si128 / extracti128_si256 1), and the I8 lane-value decode (the
   `lemma_to_i16_val` mirror at 128/I8).  All PROVEN. *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_mm_packs_epi16 (a b: Funarr.t_FunArray (mk_u64 8) i16) (k: nat{k < 16})
    : Lemma (Funarr.impl_5__get (mk_u64 16) #i8 (IV.e_mm_packs_epi16 a b) (mk_u64 k) ==
             (let x = if k < 8
                      then Funarr.impl_5__get (mk_u64 8) #i16 a (mk_u64 k)
                      else Funarr.impl_5__get (mk_u64 8) #i16 b (mk_u64 (k - 8)) in
              if x >. mk_i16 127 then mk_i8 127
              else if x <. mk_i16 (-128) then mk_i8 (-128)
              else cast x <: i8)) =
  assert (Funarr.impl_5__get (mk_u64 16) #i8 (IV.e_mm_packs_epi16 a b) (mk_u64 k) ==
          (let x = if k < 8
                   then Funarr.impl_5__get (mk_u64 8) #i16 a (mk_u64 k)
                   else Funarr.impl_5__get (mk_u64 8) #i16 b (mk_u64 (k - 8)) in
           if x >. mk_i16 127 then mk_i8 127
           else if x <. mk_i16 (-128) then mk_i8 (-128)
           else cast x <: i8))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm_packs_epi16;
                                        `%Core_models.Num.impl_i8__MAX;
                                        `%Core_models.Num.impl_i8__MIN];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_cast256_si128_lane_i16 (a: bv256) (l: nat{l < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 (IV.e_mm256_castsi256_si128 a)) (mk_u64 l) ==
             Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 l)) =
  assert_norm (Int.bits Int.I16 == 16);
  let xr : i16 = Funarr.impl_5__get (mk_u64 8) #i16 (to_i16x8 (IV.e_mm256_castsi256_si128 a)) (mk_u64 l) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 l) in
  let aux (b: usize{v b < 16}) : Lemma (Int.get_bit #Int.I16 xr b == Int.get_bit #Int.I16 yr b) =
    lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) (IV.e_mm256_castsi256_si128 a) (mk_u64 l) (v b);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) a (mk_u64 l) (v b)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 xr yr

let lemma_extracti128_1_lane_i16 (a: bv256) (l: nat{l < 8})
    : Lemma (Funarr.impl_5__get (mk_u64 8) #i16
               (to_i16x8 (IV.e_mm256_extracti128_si256 (mk_i32 1) a)) (mk_u64 l) ==
             Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 (8 + l))) =
  assert_norm (Int.bits Int.I16 == 16);
  let xr : i16 = Funarr.impl_5__get (mk_u64 8) #i16
                   (to_i16x8 (IV.e_mm256_extracti128_si256 (mk_i32 1) a)) (mk_u64 l) in
  let yr : i16 = Funarr.impl_5__get (mk_u64 16) #i16 (to_i16x16 a) (mk_u64 (8 + l)) in
  let aux (b: usize{v b < 16}) : Lemma (Int.get_bit #Int.I16 xr b == Int.get_bit #Int.I16 yr b) =
    lemma_readback Int.I16 (mk_u64 128) (mk_u64 8) (IV.e_mm256_extracti128_si256 (mk_i32 1) a) (mk_u64 l) (v b);
    lemma_readback Int.I16 (mk_u64 256) (mk_u64 16) a (mk_u64 (8 + l)) (v b)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.I16 xr yr

let lemma_to_i8_val_128 (vec: bv128) (n: nat{n < 16})
    : Lemma (v (Funarr.impl_5__get (mk_u64 16) #i8 (to_i8x16 vec) (mk_u64 n)) ==
             IVi.tc_of_u Int.I8 (IVi.dsum2 (IVi.lane_reader (mk_u64 128) 8 vec (mk_u64 n)) 0 8)) =
  reveal_opaque (`%IVi.to_iv) (IVi.to_iv);
  let reader = IVi.lane_reader (mk_u64 128) 8 vec (mk_u64 n) in
  IVi.dsum2_bound reader 0 8;
  IVi.lemma_tc_range Int.I8 (IVi.dsum2 reader 0 8)
#pop-options

(* ── Per-lane slli_epi16, and the ZEROING branch of the 128-bit PSHUFB
   (negative index byte -> 0), the complement of `lemma_iv_mm_shuffle_epi8_sel`
   above.  All PROVEN. *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_iv_slli16 (imm: i32) (arr: Funarr.t_FunArray (mk_u64 16) i16) (j: nat{j < 16})
    : Lemma (requires v imm >= 0 /\ v imm < 16)
            (ensures Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_slli_epi16 imm arr) (mk_u64 j) ==
                     (cast ((cast (Funarr.impl_5__get (mk_u64 16) #i16 arr (mk_u64 j)) <: u16)
                            <<! imm <: u16) <: i16)) =
  lemma_rem_euclid256 imm;
  assert (Funarr.impl_5__get (mk_u64 16) #i16 (IV.e_mm256_slli_epi16 imm arr) (mk_u64 j) ==
          (cast ((cast (Funarr.impl_5__get (mk_u64 16) #i16 arr (mk_u64 j)) <: u16) <<! imm <: u16)
           <: i16))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_slli_epi16];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* the high bit of a wrapped negative index byte is SET, so PSHUFB takes the
   zeroing branch (converse of `lemma_u8_high_bit_clear`). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_u8_high_bit_set (x: u8)
    : Lemma (requires v x >= 128) (ensures ~((x &. mk_u8 128) == mk_u8 0)) =
  assert_norm (pow2 7 == 128);
  Int.get_bit_and #Int.U8 x (mk_u8 128) (sz 7);
  reveal_opaque (`%Rust_primitives.Integers.get_bit) (Rust_primitives.Integers.get_bit #Int.U8)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_mm_shuffle_epi8_neg (a b: Funarr.t_FunArray (mk_u64 16) i8) (i: nat{i < 16})
    : Lemma (requires v (Funarr.impl_5__get (mk_u64 16) #i8 b (mk_u64 i)) < 0)
            (ensures Funarr.impl_5__get (mk_u64 16) #i8 (IV.e_mm_shuffle_epi8 a b) (mk_u64 i) ==
                     mk_i8 0) =
  let bi = Funarr.impl_5__get (mk_u64 16) #i8 b (mk_u64 i) in
  let idx : u8 = cast bi <: u8 in
  assert (v idx >= 128);
  lemma_u8_high_bit_set idx;
  assert (Funarr.impl_5__get (mk_u64 16) #i8 (IV.e_mm_shuffle_epi8 a b) (mk_u64 i) == mk_i8 0)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm_shuffle_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* PSHUFB, zeroing branch (256-bit): a negative index byte zeroes the output byte.
   256-bit analogue of `lemma_iv_mm_shuffle_epi8_neg`, complement of the 256-bit
   `lemma_iv_shuffle_epi8_sel`. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_shuffle_epi8_neg (a b: Funarr.t_FunArray (mk_u64 32) i8) (i: nat{i < 32})
    : Lemma (requires v (Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i)) < 0)
            (ensures Funarr.impl_5__get (mk_u64 32) #i8 (IV.e_mm256_shuffle_epi8 a b) (mk_u64 i) ==
                     mk_i8 0) =
  let bi = Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i) in
  let idx : u8 = cast bi <: u8 in
  assert (v idx >= 128);
  lemma_u8_high_bit_set idx;
  assert (Funarr.impl_5__get (mk_u64 32) #i8 (IV.e_mm256_shuffle_epi8 a b) (mk_u64 i) == mk_i8 0)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_shuffle_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── Gap: `testz_si256` fold-all-zero characterization ──────────────────────
   `IV.e_mm256_testz_si256 a b` folds `impl_10__fold` (= `impl_5__fold`) over the
   256 AND-conjunct bits, returning 1 iff every conjunct bit is `Bit_Zero`.  The
   two lemmas below give (1) a PROVEN characterization of `impl_5__fold` with the
   AND-zero step function, and (2) the resulting testz spec that ml-dsa's
   `mm256_testz_si256_lemma` consumes.  No new axiom. *)

(* one-step unfold of the recursive `impl_5__fold` (delta/iota reduction). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let fold_unfold_eq (n: u64) (#t #a: Type0) (arr: Funarr.t_FunArray n t) (init: a) (f: (a -> t -> a))
  : Lemma (Funarr.impl_5__fold n #t #a arr init f ==
           (match n with
            | MkInt 0 -> init
            | MkInt m ->
              let acc: a = f init (arr._0 (mk_u64 0)) in
              let m1 = MkInt (m - 1) in
              Funarr.impl_5__fold m1 #t #a
                (Funarr.impl_5__from_fn m1 #t #(u64 -> t) (fun i -> arr._0 (i +. mk_u64 1)))
                acc f))
  = assert (Funarr.impl_5__fold n #t #a arr init f ==
           (match n with
            | MkInt 0 -> init
            | MkInt m ->
              let acc: a = f init (arr._0 (mk_u64 0)) in
              let m1 = MkInt (m - 1) in
              Funarr.impl_5__fold m1 #t #a
                (Funarr.impl_5__from_fn m1 #t #(u64 -> t) (fun i -> arr._0 (i +. mk_u64 1)))
                acc f))
    by (FStar.Tactics.trefl ())
#pop-options

(* `impl_5__fold` with the AND-zero step `f acc b = acc && (b = Bit_Zero)` is
   `true` iff `init` is `true` AND every array element is `Bit_Zero`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let rec lemma_fold_and_zero (nn: nat{nn < pow2 64})
  : Lemma
    (ensures forall (f: (bool -> Bit.t_Bit -> bool))
               (arr: Funarr.t_FunArray (mk_u64 nn) Bit.t_Bit)
               (init: bool).
       (forall (acc: bool) (b: Bit.t_Bit). f acc b == (acc && (b = Bit.Bit_Zero))) ==>
       (Funarr.impl_5__fold (mk_u64 nn) #Bit.t_Bit #bool arr init f == true <==>
        (init == true /\ (forall (i: u64). v i < nn ==> arr._0 i == Bit.Bit_Zero))))
    (decreases nn)
  = let aux (f: (bool -> Bit.t_Bit -> bool))
            (arr: Funarr.t_FunArray (mk_u64 nn) Bit.t_Bit)
            (init: bool)
      : Lemma ((forall (acc: bool) (b: Bit.t_Bit). f acc b == (acc && (b = Bit.Bit_Zero))) ==>
               (Funarr.impl_5__fold (mk_u64 nn) #Bit.t_Bit #bool arr init f == true <==>
                (init == true /\ (forall (i: u64). v i < nn ==> arr._0 i == Bit.Bit_Zero))))
      = introduce (forall (acc: bool) (b: Bit.t_Bit). f acc b == (acc && (b = Bit.Bit_Zero))) ==>
                  (Funarr.impl_5__fold (mk_u64 nn) #Bit.t_Bit #bool arr init f == true <==>
                   (init == true /\ (forall (i: u64). v i < nn ==> arr._0 i == Bit.Bit_Zero)))
        with _. begin
          fold_unfold_eq (mk_u64 nn) #Bit.t_Bit #bool arr init f;
          if nn = 0 then ()
          else begin
            lemma_fold_and_zero (nn - 1);
            assert (v (mk_u64 nn) == nn);
            assert (mk_u64 (nn - 1) == MkInt #Rust_primitives.Integers.U64 (nn - 1));
            introduce (Funarr.impl_5__fold (mk_u64 nn) #Bit.t_Bit #bool arr init f == true) ==>
                      (init == true /\ (forall (i: u64). v i < nn ==> arr._0 i == Bit.Bit_Zero))
            with _. begin
              introduce forall (i: u64). v i < nn ==> arr._0 i == Bit.Bit_Zero
              with introduce v i < nn ==> arr._0 i == Bit.Bit_Zero
              with _. begin
                if v i = 0 then assert (mk_u64 (v i) == i)
                else begin
                  let j : u64 = mk_u64 (v i - 1) in
                  assert (j +. mk_u64 1 == i)
                end
              end
            end;
            introduce (init == true /\ (forall (i: u64). v i < nn ==> arr._0 i == Bit.Bit_Zero)) ==>
                      (Funarr.impl_5__fold (mk_u64 nn) #Bit.t_Bit #bool arr init f == true)
            with _. begin
              assert (arr._0 (mk_u64 0) == Bit.Bit_Zero);
              assert (f init (arr._0 (mk_u64 0)) == true)
            end
          end
        end
    in
    Classical.forall_intro_3 aux
#pop-options

(* testz: the model returns 1 iff every bit of (a AND b) is zero. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_testz_funarr (a b: bv256)
  : Lemma (let r = IV.e_mm256_testz_si256 a b in
           (r == mk_i32 0 \/ r == mk_i32 1) /\
           (r == mk_i32 1 <==>
            (forall (k: u64). v k < 256 ==>
               Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit (IV.e_mm256_and_si256 a b)._0 k ==
               Bit.Bit_Zero)))
  = let f_c : (i: u64{v i < 256}) -> Bit.t_Bit =
      fun i -> (let i:u64 = i in
                match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
                | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_One
                | _ -> Bit.Bit_Zero) in
    let f_z : bool -> Bit.t_Bit -> bool =
      fun acc bit -> (let acc:bool = acc in
                      let bit:Bit.t_Bit = bit in
                      acc && (bit =. Bit.Bit_Zero)) in
    assert (IV.e_mm256_testz_si256 a b ==
            (let c = BV.impl_9__from_fn (mk_u64 256) #(u64 -> Bit.t_Bit) f_c in
             let all_zero = BV.impl_10__fold (mk_u64 256) #bool c true f_z in
             if all_zero then mk_i32 1 else mk_i32 0))
      by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_testz_si256];
                              iota; zeta; primops];
          FStar.Tactics.trefl ());
    let c = BV.impl_9__from_fn (mk_u64 256) #(u64 -> Bit.t_Bit) f_c in
    assert (BV.impl_10__fold (mk_u64 256) #bool c true f_z ==
            Funarr.impl_5__fold (mk_u64 256) #Bit.t_Bit #bool c._0 true f_z);
    lemma_fold_and_zero 256;
    introduce forall (k: u64). v k < 256 ==> c._0._0 k == f_c k
    with introduce v k < 256 ==> c._0._0 k == f_c k
    with _. lemma_impl9_index f_c k;
    introduce forall (k: u64). v k < 256 ==>
        Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit (IV.e_mm256_and_si256 a b)._0 k == f_c k
    with introduce v k < 256 ==>
        Funarr.impl_5__get (mk_u64 256) #Bit.t_Bit (IV.e_mm256_and_si256 a b)._0 k == f_c k
    with _. begin
      lemma_and_funarr a b k;
      lemma_bv_index a k;
      lemma_bv_index b k
    end
#pop-options

(* ============================================================================
   u64x4 lane-view op-lemmas — the sha3 AVX2 Keccak op set (xor / or / andnot /
   slli_epi64 / srli_epi64 / unpackhi_epi64 / unpacklo_epi64 / set1_epi64x /
   set_epi64x / permute2x128_si256).  Twin of the i16x16 family above at lane
   width 64.  PROVEN from the differentially-tested lifts + codec round-trip;
   ZERO new assumptions (the andnot lift completes the and/or/xor family in
   Trusted.Intrinsics).  Consumed (via `get_lane_u64x4`) by the sha3 companion
   `Libcrux_intrinsics.Avx2_sha3_views`.
   ============================================================================ *)

let get64 (vv: bv256) (i: nat{i < 4}) : u64 =
  Funarr.impl_5__get (mk_u64 4) #u64 (to_u64x4 vv) (mk_u64 i)
let get64i (vv: bv256) (i: nat{i < 4}) : i64 =
  Funarr.impl_5__get (mk_u64 4) #i64 (to_i64x4 vv) (mk_u64 i)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_xor_raw64 (a b: bv256) (ii: u64{v ii < 4}) (bb: nat{bb < 64})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 64 (IV.e_mm256_xor_si256 a b) ii bb) ==
             Int.bit_xor (IVi.bval (IVi.lane_reader (mk_u64 256) 64 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 256) 64 b ii bb))) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | Bit.Bit_One, Bit.Bit_One -> Bit.Bit_Zero
              | _ -> Bit.Bit_One) in
  assert (IV.e_mm256_xor_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_xor_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (64 * v ii + bb < 256);
  lemma_from_fn_lane_reader f 64 ii bb;
  lemma_bv_index a (mk_u64 (64 * v ii + bb));
  lemma_bv_index b (mk_u64 (64 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_or_raw64 (a b: bv256) (ii: u64{v ii < 4}) (bb: nat{bb < 64})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 64 (IV.e_mm256_or_si256 a b) ii bb) ==
             Int.bit_or (IVi.bval (IVi.lane_reader (mk_u64 256) 64 a ii bb))
                        (IVi.bval (IVi.lane_reader (mk_u64 256) 64 b ii bb))) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | _ -> Bit.Bit_One) in
  assert (IV.e_mm256_or_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_or_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (64 * v ii + bb < 256);
  lemma_from_fn_lane_reader f 64 ii bb;
  lemma_bv_index a (mk_u64 (64 * v ii + bb));
  lemma_bv_index b (mk_u64 (64 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_xor_u64x4_iv (a b: bv256) (i: nat{i < 4})
    : Lemma (get64 (IV.e_mm256_xor_si256 a b) i == (get64 a i ^. get64 b i)) =
  let aXORb = IV.e_mm256_xor_si256 a b in
  let ya = get64 a i in let yb = get64 b i in let yr = get64 aXORb i in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yr bb == Int.get_bit #Int.U64 (ya ^. yb) bb) =
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) aXORb (mk_u64 i) (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) a (mk_u64 i) (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) b (mk_u64 i) (v bb);
    lemma_xor_raw64 a b (mk_u64 i) (v bb);
    Int.get_bit_xor #Int.U64 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yr (ya ^. yb)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_or_u64x4_iv (a b: bv256) (i: nat{i < 4})
    : Lemma (get64 (IV.e_mm256_or_si256 a b) i == (get64 a i |. get64 b i)) =
  let aORb = IV.e_mm256_or_si256 a b in
  let ya = get64 a i in let yb = get64 b i in let yr = get64 aORb i in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yr bb == Int.get_bit #Int.U64 (ya |. yb) bb) =
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) aORb (mk_u64 i) (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) a (mk_u64 i) (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) b (mk_u64 i) (v bb);
    lemma_or_raw64 a b (mk_u64 i) (v bb);
    Int.get_bit_or #Int.U64 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yr (ya |. yb)
#pop-options

let lemma_xor_u64x4 (a b: bv256) (i: nat{i < 4})
    : Lemma (get64 (Avx2.e_mm256_xor_si256 a b) i == (get64 a i ^. get64 b i)) =
  lemma_xor_si256_lift a b; lemma_xor_u64x4_iv a b i

let lemma_or_u64x4 (a b: bv256) (i: nat{i < 4})
    : Lemma (get64 (Avx2.e_mm256_or_si256 a b) i == (get64 a i |. get64 b i)) =
  lemma_or_si256_lift a b; lemma_or_u64x4_iv a b i

(* u64-lane == cast of the i64-lane of the same vector (same 64 bits). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_u64x4_from_i64x4 (vec: bv256) (i: nat{i < 4})
    : Lemma (get64 vec i == Int.cast_mod #Int.I64 #Int.U64 (get64i vec i)) =
  let yu = get64 vec i in
  let yi = get64i vec i in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yu bb == Int.get_bit #Int.U64 (Int.cast_mod #Int.I64 #Int.U64 yi) bb) =
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) vec (mk_u64 i) (v bb);
    lemma_readback Int.I64 (mk_u64 256) (mk_u64 4) vec (mk_u64 i) (v bb);
    Int.get_bit_cast #Int.I64 #Int.U64 yi bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yu (Int.cast_mod #Int.I64 #Int.U64 yi)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_iv_unpacklo_epi64 (a b: Funarr.t_FunArray (mk_u64 4) i64) (j: nat{j < 4})
    : Lemma (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_unpacklo_epi64 a b) (mk_u64 j) ==
             (match j with
              | 0 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 0)
              | 1 -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 0)
              | 2 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 2)
              | _ -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 2))) =
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_unpacklo_epi64 a b) (mk_u64 j) ==
          (match j with
           | 0 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 0)
           | 1 -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 0)
           | 2 -> Funarr.impl_5__get (mk_u64 4) #i64 a (mk_u64 2)
           | _ -> Funarr.impl_5__get (mk_u64 4) #i64 b (mk_u64 2)))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_unpacklo_epi64];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_set1_epi64x_u64x4 (a: i64) (i: nat{i < 4})
    : Lemma (get64 (Avx.e_mm256_set1_epi64x a) i == Int.cast_mod #Int.I64 #Int.U64 a) =
  lemma_mm256_set1_epi64x a;
  lemma_u64x4_from_i64x4 (Avx.e_mm256_set1_epi64x a) i;
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_set1_epi64x a) (mk_u64 i) == a)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_set1_epi64x];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_set_epi64x_u64x4 (e3 e2 e1 e0: i64) (i: nat{i < 4})
    : Lemma (get64 (Avx.e_mm256_set_epi64x e3 e2 e1 e0) i ==
             Int.cast_mod #Int.I64 #Int.U64 (match i with | 0 -> e0 | 1 -> e1 | 2 -> e2 | _ -> e3)) =
  lemma_mm256_set_epi64x e3 e2 e1 e0;
  lemma_u64x4_from_i64x4 (Avx.e_mm256_set_epi64x e3 e2 e1 e0) i;
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_set_epi64x e3 e2 e1 e0) (mk_u64 i) ==
          (match i with | 0 -> e0 | 1 -> e1 | 2 -> e2 | _ -> e3))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_set_epi64x];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_unpacklo_epi64_u64x4 (a b: bv256) (j: nat{j < 4})
    : Lemma (get64 (Avx2.e_mm256_unpacklo_epi64 a b) j ==
             (match j with | 0 -> get64 a 0 | 1 -> get64 b 0 | 2 -> get64 a 2 | _ -> get64 b 2)) =
  lemma_mm256_unpacklo_epi64 a b;
  lemma_iv_unpacklo_epi64 (to_i64x4 a) (to_i64x4 b) j;
  lemma_u64x4_from_i64x4 (Avx2.e_mm256_unpacklo_epi64 a b) j;
  lemma_u64x4_from_i64x4 a 0;
  lemma_u64x4_from_i64x4 b 0;
  lemma_u64x4_from_i64x4 a 2;
  lemma_u64x4_from_i64x4 b 2
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_unpackhi_epi64_u64x4 (a b: bv256) (j: nat{j < 4})
    : Lemma (get64 (Avx2.e_mm256_unpackhi_epi64 a b) j ==
             (match j with | 0 -> get64 a 1 | 1 -> get64 b 1 | 2 -> get64 a 3 | _ -> get64 b 3)) =
  lemma_mm256_unpackhi_epi64 a b;
  lemma_iv_unpackhi_epi64 (to_i64x4 a) (to_i64x4 b) j;
  lemma_u64x4_from_i64x4 (Avx2.e_mm256_unpackhi_epi64 a b) j;
  lemma_u64x4_from_i64x4 a 1;
  lemma_u64x4_from_i64x4 b 1;
  lemma_u64x4_from_i64x4 a 3;
  lemma_u64x4_from_i64x4 b 3
#pop-options

(* u64 -> i64 -> u64 round trip (get_bit_cast SMTPats close it). *)
let lemma_cast_rt_u64 (z: u64)
    : Lemma (Int.cast_mod #Int.I64 #Int.U64 (Int.cast_mod #Int.U64 #Int.I64 z) == z) =
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 (Int.cast_mod #Int.I64 #Int.U64 (Int.cast_mod #Int.U64 #Int.I64 z)) bb
               == Int.get_bit #Int.U64 z bb) = () in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64
    (Int.cast_mod #Int.I64 #Int.U64 (Int.cast_mod #Int.U64 #Int.I64 z)) z

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_slli_epi64_u64x4 (n: i32) (a: bv256) (i: nat{i < 4})
    : Lemma (requires v n >= 0 /\ v n < 64)
            (ensures get64 (Avx2.e_mm256_slli_epi64 n a) i == (get64 a i <<! n)) =
  lemma_mm256_slli_epi64 n a;
  lemma_u64x4_from_i64x4 (Avx2.e_mm256_slli_epi64 n a) i;
  lemma_u64x4_from_i64x4 a i;
  let arr = to_i64x4 a in
  let uval : u64 = Int.cast_mod #Int.I64 #Int.U64 (Funarr.impl_5__get (mk_u64 4) #i64 arr (mk_u64 i)) in
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_slli_epi64 n arr) (mk_u64 i) ==
          Int.cast_mod #Int.U64 #Int.I64 (uval <<! n))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_slli_epi64;
                                        `%Core_models.Num.impl_i32__rem_euclid];
                            iota; zeta; primops];
        FStar.Tactics.smt ());
  lemma_cast_rt_u64 (uval <<! n)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_srli_epi64_u64x4 (n: i32) (a: bv256) (i: nat{i < 4})
    : Lemma (requires v n >= 0 /\ v n < 64)
            (ensures get64 (Avx2.e_mm256_srli_epi64 n a) i == (get64 a i >>! n)) =
  lemma_mm256_srli_epi64 n a;
  lemma_u64x4_from_i64x4 (Avx2.e_mm256_srli_epi64 n a) i;
  lemma_u64x4_from_i64x4 a i;
  let arr = to_i64x4 a in
  let uval : u64 = Int.cast_mod #Int.I64 #Int.U64 (Funarr.impl_5__get (mk_u64 4) #i64 arr (mk_u64 i)) in
  assert (Funarr.impl_5__get (mk_u64 4) #i64 (IV.e_mm256_srli_epi64 n arr) (mk_u64 i) ==
          Int.cast_mod #Int.U64 #Int.I64 (uval >>! n))
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_srli_epi64;
                                        `%Core_models.Num.impl_i32__rem_euclid];
                            iota; zeta; primops];
        FStar.Tactics.smt ());
  lemma_cast_rt_u64 (uval >>! n)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_andnot_raw64 (a b: bv256) (ii: u64{v ii < 4}) (bb: nat{bb < 64})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) 64 (IV.e_mm256_andnot_si256 a b) ii bb) ==
             (if IVi.bval (IVi.lane_reader (mk_u64 256) 64 a ii bb) = 0 &&
                 IVi.bval (IVi.lane_reader (mk_u64 256) 64 b ii bb) = 1
              then 1 else 0)) =
  let f : (i: u64{v i < 256}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_Zero, Bit.Bit_One -> Bit.Bit_One
              | _ -> Bit.Bit_Zero) in
  assert (IV.e_mm256_andnot_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_andnot_si256];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (64 * v ii + bb < 256);
  lemma_from_fn_lane_reader f 64 ii bb;
  lemma_bv_index a (mk_u64 (64 * v ii + bb));
  lemma_bv_index b (mk_u64 (64 * v ii + bb))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_andnot_u64x4_iv (a b: bv256) (i: nat{i < 4})
    : Lemma (get64 (IV.e_mm256_andnot_si256 a b) i == (get64 b i &. (~. (get64 a i)))) =
  let r = IV.e_mm256_andnot_si256 a b in
  let ya = get64 a i in let yb = get64 b i in let yr = get64 r i in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yr bb == Int.get_bit #Int.U64 (yb &. (~. ya)) bb) =
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) r (mk_u64 i) (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) a (mk_u64 i) (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) b (mk_u64 i) (v bb);
    lemma_andnot_raw64 a b (mk_u64 i) (v bb);
    Int.get_bit_and #Int.U64 yb (~. ya) bb;
    Int.get_bit_lognot #Int.U64 ya bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yr (yb &. (~. ya))
#pop-options

let lemma_andnot_u64x4 (a b: bv256) (i: nat{i < 4})
    : Lemma (get64 (Avx2.e_mm256_andnot_si256 a b) i == (get64 b i &. (~. (get64 a i)))) =
  lemma_andnot_si256_lift a b; lemma_andnot_u64x4_iv a b i

(* ── permute2x128_si256: 128-bit half selection.  Modeled over i128x2, so the
      u64x4 view needs an i128-lane <-> u64-lane bit bridge. ─────────────────── *)

(* raw bit `i` of a 256-bit vector as a Rust bit. *)
let bv_bit256 (bv: bv256) (i: nat{i < 256}) : Int.bit =
  match bv.[ mk_u64 i ] <: Bit.t_Bit with | Bit.Bit_One -> 1 | Bit.Bit_Zero -> 0

(* lane_reader bval == the absolute bit `w*l+b` (any lane width). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_reader_abs (w: pos) (bv: bv256) (l: nat) (b: nat{b < w /\ w * l + b < 256})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 256) w bv (mk_u64 l) b) == bv_bit256 bv (w * l + b)) =
  FStar.Math.Lemmas.lemma_mult_le_right l 1 w;
  assert (l <= w * l)
#pop-options

(* two u64 lanes read the same 64 bits iff their enclosing i128 lanes agree. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_u64_of_i128 (u w: bv256) (lu lw: nat)
    : Lemma (requires lu < 4 /\ lw < 4 /\ lu % 2 == lw % 2 /\
                      Funarr.impl_5__get (mk_u64 2) #i128 (to_i128x2 u) (mk_u64 (lu / 2)) ==
                      Funarr.impl_5__get (mk_u64 2) #i128 (to_i128x2 w) (mk_u64 (lw / 2)))
            (ensures get64 u lu == get64 w lw) =
  FStar.Math.Lemmas.euclidean_division_definition lu 2;
  FStar.Math.Lemmas.euclidean_division_definition lw 2;
  let yu = get64 u lu in let yw = get64 w lw in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yu bb == Int.get_bit #Int.U64 yw bb) =
    let o : nat = 64 * (lu % 2) + v bb in
    assert (o < 128);
    assert (64 * lu + v bb == 128 * (lu / 2) + o);
    assert (64 * lw + v bb == 128 * (lw / 2) + o);
    (* u64 side *)
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) u (mk_u64 lu) (v bb);
    lemma_reader_abs 64 u lu (v bb);
    lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) w (mk_u64 lw) (v bb);
    lemma_reader_abs 64 w lw (v bb);
    (* i128 side *)
    lemma_readback Int.I128 (mk_u64 256) (mk_u64 2) u (mk_u64 (lu / 2)) o;
    lemma_reader_abs 128 u (lu / 2) o;
    lemma_readback Int.I128 (mk_u64 256) (mk_u64 2) w (mk_u64 (lw / 2)) o;
    lemma_reader_abs 128 w (lw / 2) o
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yu yw
#pop-options

(* IV per-lane fact for the two IMM8 forms sha3 uses (0x20, 0x31). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv_permute2x128 (c: i32) (a b: Funarr.t_FunArray (mk_u64 2) i128) (j: nat{j < 2})
    : Lemma (requires v c == 32 \/ v c == 49)
            (ensures Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_permute2x128_si256 c a b) (mk_u64 j) ==
                     (if v c = 32
                      then (if j = 0 then Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 0)
                                     else Funarr.impl_5__get (mk_u64 2) #i128 b (mk_u64 0))
                      else (if j = 0 then Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 1)
                                     else Funarr.impl_5__get (mk_u64 2) #i128 b (mk_u64 1)))) =
  let cc : i32 = if v c = 32 then mk_i32 32 else mk_i32 49 in
  assert (c == cc);
  if v c = 32 then begin
    if j = 0 then
      assert (Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_permute2x128_si256 (mk_i32 32) a b) (mk_u64 0) ==
              Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 0))
        by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_permute2x128_si256];
                                iota; zeta; primops]; FStar.Tactics.smt ())
    else
      assert (Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_permute2x128_si256 (mk_i32 32) a b) (mk_u64 1) ==
              Funarr.impl_5__get (mk_u64 2) #i128 b (mk_u64 0))
        by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_permute2x128_si256];
                                iota; zeta; primops]; FStar.Tactics.smt ())
  end else begin
    if j = 0 then
      assert (Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_permute2x128_si256 (mk_i32 49) a b) (mk_u64 0) ==
              Funarr.impl_5__get (mk_u64 2) #i128 a (mk_u64 1))
        by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_permute2x128_si256];
                                iota; zeta; primops]; FStar.Tactics.smt ())
    else
      assert (Funarr.impl_5__get (mk_u64 2) #i128 (IV.e_mm256_permute2x128_si256 (mk_i32 49) a b) (mk_u64 1) ==
              Funarr.impl_5__get (mk_u64 2) #i128 b (mk_u64 1))
        by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_permute2x128_si256];
                                iota; zeta; primops]; FStar.Tactics.smt ())
  end
#pop-options

(* u64x4 view: 0x20 -> [a0,a1,b0,b1]; 0x31 -> [a2,a3,b2,b3]. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_permute2x128_si256_u64x4 (c: i32) (a b: bv256) (l: nat{l < 4})
    : Lemma (requires v c == 32 \/ v c == 49)
            (ensures get64 (Avx2.e_mm256_permute2x128_si256 c a b) l ==
                     (if v c = 32
                      then (match l with | 0 -> get64 a 0 | 1 -> get64 a 1 | 2 -> get64 b 0 | _ -> get64 b 1)
                      else (match l with | 0 -> get64 a 2 | 1 -> get64 a 3 | 2 -> get64 b 2 | _ -> get64 b 3))) =
  let r = Avx2.e_mm256_permute2x128_si256 c a b in
  lemma_mm256_permute2x128_si256 c a b;
  (* to_i128x2 r [j] == IV.perm c (to_i128x2 a) (to_i128x2 b) [j] *)
  lemma_iv_permute2x128 c (to_i128x2 a) (to_i128x2 b) 0;
  lemma_iv_permute2x128 c (to_i128x2 a) (to_i128x2 b) 1;
  (* per output u64 lane, bridge to the source lane via its i128 half *)
  (match l with
   | 0 -> if v c = 32 then lemma_u64_of_i128 r a 0 0 else lemma_u64_of_i128 r a 0 2
   | 1 -> if v c = 32 then lemma_u64_of_i128 r a 1 1 else lemma_u64_of_i128 r a 1 3
   | 2 -> if v c = 32 then lemma_u64_of_i128 r b 2 0 else lemma_u64_of_i128 r b 2 2
   | _ -> if v c = 32 then lemma_u64_of_i128 r b 3 1 else lemma_u64_of_i128 r b 3 3)
#pop-options
