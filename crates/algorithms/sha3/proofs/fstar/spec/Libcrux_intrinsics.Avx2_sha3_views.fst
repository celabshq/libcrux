module Libcrux_intrinsics.Avx2_sha3_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models

(* ============================================================================
   sha3 AVX2 (X4) lane-view + per-op fact companion.

   The u64x4 / 256-bit analog of `Libcrux_intrinsics.Arm64_sha3_views` (the NEON
   u64x2 companion).  It exposes to sha3's AVX2 proofs the u64x4 lane VIEW
   (`vec256_as_u64x4` / `get_lane_u64x4`) and the per-op FACT lemmas that the
   hand-written `Libcrux_intrinsics.Avx2_extract` interface carried as op
   `ensures` (or `fstar::replace(interface,...)` lemma discharges), now phrased
   over the REAL `Libcrux_intrinsics.Avx2` ops (which delegate to the
   differentially-tested `libcrux-core-models` x86 model + `X86.Extra` slice-I/O
   models: raw bit-ops -> `Core_arch.X86.{Avx,Avx2}.e_mm256_*`; get_lane_u64 ->
   `Extra.get_lane_u64_model`; byte I/O -> `Extra.mm256_{storeu,loadu}_si256_u8_model`).

   TRUST.  The Seq lane view is a per-index read of the canonical core-models
   codec (`Canon.to_u64x4` = `Int_vec_interp` width 256 / lane 64), and every
   op-fact is PROVEN from the canonical u64x4 op-lemma set in
   `Libcrux_core_models.Intrinsics_views` (which rests only on the differentially
   tested `Trusted.Intrinsics` lifts + the PROVEN codec round-trip).  Every fact
   here is PROVEN, not an assumed op `ensures` or `admit`.  NO fact in this
   module is assumed.

   It `include`s the real `Libcrux_intrinsics.Avx2` so consumers that alias this
   module resolve both `mm256_OP` to the real op AND `vec256_as_u64x4` /
   `get_lane_u64x4` / the byte-fact lemmas to the views below.  Lives in
   `proofs/fstar/spec/` (hand-maintained), on sha3's include path only; NOT a
   make ROOT (verifies as a dependency of the repointed AVX2 consumers).

   Ops covered (the Keccak u64x4 set): mm256_{xor,or,andnot}_si256,
   mm256_{slli,srli}_epi64, mm256_{unpacklo,unpackhi}_epi64,
   mm256_{set1,set}_epi64x, mm256_permute2x128_si256, the u64 lane read
   (get_lane_u64) and the byte load/store bridges
   (mm256_{loadu,storeu}_si256_u8).
   ========================================================================== *)

include Libcrux_intrinsics.Avx2

module Funarr = Libcrux_core_models.Abstractions.Funarr
module BV     = Libcrux_core_models.Abstractions.Bitvec
module Bit    = Libcrux_core_models.Abstractions.Bit
module Canon  = Libcrux_core_models.Intrinsics_views
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Extra  = Libcrux_core_models.Core_arch.X86.Extra
module Avx    = Libcrux_core_models.Core_arch.X86.Avx
module Avx2m  = Libcrux_core_models.Core_arch.X86.Avx2
module Num    = Core_models.Num
module Int    = Rust_primitives.Integers

(* ── lane-view type (mirrors the `Avx2_extract` `t_Vec256`; the REAL `Avx2` wrappers take
      `BV.t_BitVec (mk_u64 256)` inline and define no alias, so — exactly as the
      NEON companion supplies `t_e_uint64x2_t` — this module supplies `t_Vec256`
      for the repointed consumers (`I.t_Vec256`)). ───────────────────────────── *)
unfold type t_Vec256 = BV.t_BitVec (mk_u64 256)

(* ── u64x4 lane view (A-on-B adapter over canonical Canon.to_u64x4).  OPAQUE
      for the same reasons as ml-kem's `vec256_as_i16x16`: keeps the `Avx2_extract`
      abstraction (still PROVEN, not assumed); the only route to the codec is
      the index lemma below. ─────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec256_as_u64x4 (x: t_Vec256) : t_Array u64 (sz 4) =
  Seq.init 4 (fun i -> Funarr.impl_5__get (mk_u64 4) #u64 (Canon.to_u64x4 x) (mk_u64 i))
let get_lane_u64x4 (v: t_Vec256) (i: nat{i < 4}) : u64 = Seq.index (vec256_as_u64x4 v) i

let vec256_index_u64x4 (x: t_Vec256) (i: nat{i < 4})
  : Lemma (Seq.index (vec256_as_u64x4 x) i
           == Funarr.impl_5__get (mk_u64 4) #u64 (Canon.to_u64x4 x) (mk_u64 i))
          [SMTPat (Seq.index (vec256_as_u64x4 x) i)]
  = reveal_opaque (`%vec256_as_u64x4) vec256_as_u64x4

let vec256_as_u64x4_len (x: t_Vec256)
  : Lemma (Seq.length (vec256_as_u64x4 x) == 4)
          [SMTPat (Seq.length (vec256_as_u64x4 x))]
  = ()

let vec256_as_u64x4_slice_ok (x: t_Vec256)
  : Lemma (Seq.length (vec256_as_u64x4 x) <= Int.max_usize)
          [SMTPat (vec256_as_u64x4 x)]
  = assert_norm (4 <= Int.max_usize)

(* the view lane == the canonical `Canon.get64` (same Funarr read). *)
let lemma_glx4_eq_get64 (v: t_Vec256) (i: nat{i < 4})
  : Lemma (get_lane_u64x4 v i == Canon.get64 v i)
  = vec256_index_u64x4 v i

(* ============================================================================
   u64x4 op-facts.  Each real `mm256_OP` is `[@@ opaque_to_smt]` and delegates
   (definitionally) to `Avx2m.e_mm256_OP` / `Avx.e_mm256_OP`; revealing it +
   the PROVEN `Canon.lemma_OP_u64x4` (over the same core-models op) + the view
   bridge closes the per-lane fact.  Statements mirror
   `Avx2_extract.lemma_mm256_*_u64x4` (SMTPat on the op application, `forall`
   ensures over `get_lane_u64x4`).
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_xor_si256_u64x4 (lhs rhs: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_xor_si256 lhs rhs) i ==
             (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i))
          [SMTPat (mm256_xor_si256 lhs rhs)] =
  reveal_opaque (`%mm256_xor_si256) mm256_xor_si256;
  let aux (i: nat{i < 4})
    : Lemma (get_lane_u64x4 (mm256_xor_si256 lhs rhs) i ==
             (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i)) =
    reveal_opaque (`%mm256_xor_si256) mm256_xor_si256;
    lemma_glx4_eq_get64 (mm256_xor_si256 lhs rhs) i;
    lemma_glx4_eq_get64 lhs i;
    lemma_glx4_eq_get64 rhs i;
    Canon.lemma_xor_u64x4 lhs rhs i
  in Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_or_si256_u64x4 (a b: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_or_si256 a b) i ==
             (get_lane_u64x4 a i |. get_lane_u64x4 b i))
          [SMTPat (mm256_or_si256 a b)] =
  reveal_opaque (`%mm256_or_si256) mm256_or_si256;
  let aux (i: nat{i < 4})
    : Lemma (get_lane_u64x4 (mm256_or_si256 a b) i ==
             (get_lane_u64x4 a i |. get_lane_u64x4 b i)) =
    reveal_opaque (`%mm256_or_si256) mm256_or_si256;
    lemma_glx4_eq_get64 (mm256_or_si256 a b) i;
    lemma_glx4_eq_get64 a i;
    lemma_glx4_eq_get64 b i;
    Canon.lemma_or_u64x4 a b i
  in Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_andnot_si256_u64x4 (a b: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_andnot_si256 a b) i ==
             (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i))))
          [SMTPat (mm256_andnot_si256 a b)] =
  reveal_opaque (`%mm256_andnot_si256) mm256_andnot_si256;
  let aux (i: nat{i < 4})
    : Lemma (get_lane_u64x4 (mm256_andnot_si256 a b) i ==
             (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i)))) =
    reveal_opaque (`%mm256_andnot_si256) mm256_andnot_si256;
    lemma_glx4_eq_get64 (mm256_andnot_si256 a b) i;
    lemma_glx4_eq_get64 a i;
    lemma_glx4_eq_get64 b i;
    Canon.lemma_andnot_u64x4 a b i
  in Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_slli_epi64_u64x4 (v_LEFT: i32) (x: t_Vec256)
  : Lemma
      (requires Int.v v_LEFT >= 0 /\ Int.v v_LEFT < 64)
      (ensures
        forall (i: nat{i < 4}).
          get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i ==
          (get_lane_u64x4 x i <<! v_LEFT))
      [SMTPat (mm256_slli_epi64 v_LEFT x)] =
  reveal_opaque (`%mm256_slli_epi64) mm256_slli_epi64;
  let aux (i: nat{i < 4})
    : Lemma (get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i ==
             (get_lane_u64x4 x i <<! v_LEFT)) =
    reveal_opaque (`%mm256_slli_epi64) mm256_slli_epi64;
    lemma_glx4_eq_get64 (mm256_slli_epi64 v_LEFT x) i;
    lemma_glx4_eq_get64 x i;
    Canon.lemma_slli_epi64_u64x4 v_LEFT x i
  in Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_srli_epi64_u64x4 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma
      (requires Int.v v_SHIFT_BY > 0 /\ Int.v v_SHIFT_BY < 64)  (* real srli wrapper requires > 0 *)
      (ensures
        forall (i: nat{i < 4}).
          get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i ==
          (get_lane_u64x4 vector i >>! v_SHIFT_BY))
      [SMTPat (mm256_srli_epi64 v_SHIFT_BY vector)] =
  reveal_opaque (`%mm256_srli_epi64) mm256_srli_epi64;
  let aux (i: nat{i < 4})
    : Lemma (get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i ==
             (get_lane_u64x4 vector i >>! v_SHIFT_BY)) =
    reveal_opaque (`%mm256_srli_epi64) mm256_srli_epi64;
    lemma_glx4_eq_get64 (mm256_srli_epi64 v_SHIFT_BY vector) i;
    lemma_glx4_eq_get64 vector i;
    Canon.lemma_srli_epi64_u64x4 v_SHIFT_BY vector i
  in Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_unpacklo_epi64_u64x4 (a b: t_Vec256)
  : Lemma (
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 0 == get_lane_u64x4 a 0 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 1 == get_lane_u64x4 b 0 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 2 == get_lane_u64x4 a 2 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 3 == get_lane_u64x4 b 2)
    [SMTPat (mm256_unpacklo_epi64 a b)] =
  reveal_opaque (`%mm256_unpacklo_epi64) mm256_unpacklo_epi64;
  let r = mm256_unpacklo_epi64 a b in
  lemma_glx4_eq_get64 r 0; lemma_glx4_eq_get64 r 1;
  lemma_glx4_eq_get64 r 2; lemma_glx4_eq_get64 r 3;
  lemma_glx4_eq_get64 a 0; lemma_glx4_eq_get64 b 0;
  lemma_glx4_eq_get64 a 2; lemma_glx4_eq_get64 b 2;
  Canon.lemma_unpacklo_epi64_u64x4 a b 0;
  Canon.lemma_unpacklo_epi64_u64x4 a b 1;
  Canon.lemma_unpacklo_epi64_u64x4 a b 2;
  Canon.lemma_unpacklo_epi64_u64x4 a b 3
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_unpackhi_epi64_u64x4 (lhs rhs: t_Vec256)
  : Lemma (
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 0 == get_lane_u64x4 lhs 1 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 1 == get_lane_u64x4 rhs 1 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 2 == get_lane_u64x4 lhs 3 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 3 == get_lane_u64x4 rhs 3)
    [SMTPat (mm256_unpackhi_epi64 lhs rhs)] =
  reveal_opaque (`%mm256_unpackhi_epi64) mm256_unpackhi_epi64;
  let r = mm256_unpackhi_epi64 lhs rhs in
  lemma_glx4_eq_get64 r 0; lemma_glx4_eq_get64 r 1;
  lemma_glx4_eq_get64 r 2; lemma_glx4_eq_get64 r 3;
  lemma_glx4_eq_get64 lhs 1; lemma_glx4_eq_get64 rhs 1;
  lemma_glx4_eq_get64 lhs 3; lemma_glx4_eq_get64 rhs 3;
  Canon.lemma_unpackhi_epi64_u64x4 lhs rhs 0;
  Canon.lemma_unpackhi_epi64_u64x4 lhs rhs 1;
  Canon.lemma_unpackhi_epi64_u64x4 lhs rhs 2;
  Canon.lemma_unpackhi_epi64_u64x4 lhs rhs 3
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_set1_epi64x_u64x4 (a: i64)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_set1_epi64x a) i == (Int.cast_mod #Int.I64 #Int.U64 a))
          [SMTPat (mm256_set1_epi64x a)] =
  reveal_opaque (`%mm256_set1_epi64x) mm256_set1_epi64x;
  let aux (i: nat{i < 4})
    : Lemma (get_lane_u64x4 (mm256_set1_epi64x a) i == (Int.cast_mod #Int.I64 #Int.U64 a)) =
    reveal_opaque (`%mm256_set1_epi64x) mm256_set1_epi64x;
    lemma_glx4_eq_get64 (mm256_set1_epi64x a) i;
    Canon.lemma_set1_epi64x_u64x4 a i
  in Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_set_epi64x_u64x4 (input3 input2 input1 input0: i64)
  : Lemma (
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 0 == Int.cast_mod #Int.I64 #Int.U64 input0 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 1 == Int.cast_mod #Int.I64 #Int.U64 input1 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 2 == Int.cast_mod #Int.I64 #Int.U64 input2 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 3 == Int.cast_mod #Int.I64 #Int.U64 input3)
    [SMTPat (mm256_set_epi64x input3 input2 input1 input0)] =
  reveal_opaque (`%mm256_set_epi64x) mm256_set_epi64x;
  let r = mm256_set_epi64x input3 input2 input1 input0 in
  lemma_glx4_eq_get64 r 0; lemma_glx4_eq_get64 r 1;
  lemma_glx4_eq_get64 r 2; lemma_glx4_eq_get64 r 3;
  Canon.lemma_set_epi64x_u64x4 input3 input2 input1 input0 0;
  Canon.lemma_set_epi64x_u64x4 input3 input2 input1 input0 1;
  Canon.lemma_set_epi64x_u64x4 input3 input2 input1 input0 2;
  Canon.lemma_set_epi64x_u64x4 input3 input2 input1 input0 3
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_mm256_permute2x128_si256_u64x4 (v_IMM8: i32) (a b: t_Vec256)
  : Lemma
      (requires Int.v v_IMM8 == 0x20 \/ Int.v v_IMM8 == 0x31)
      (ensures
        (Int.v v_IMM8 == 0x20 ==>
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 0 == get_lane_u64x4 a 0 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 1 == get_lane_u64x4 a 1 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 2 == get_lane_u64x4 b 0 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 3 == get_lane_u64x4 b 1) /\
        (Int.v v_IMM8 == 0x31 ==>
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 0 == get_lane_u64x4 a 2 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 1 == get_lane_u64x4 a 3 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 2 == get_lane_u64x4 b 2 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 3 == get_lane_u64x4 b 3))
      [SMTPat (mm256_permute2x128_si256 v_IMM8 a b)] =
  reveal_opaque (`%mm256_permute2x128_si256) mm256_permute2x128_si256;
  let r = mm256_permute2x128_si256 v_IMM8 a b in
  lemma_glx4_eq_get64 r 0; lemma_glx4_eq_get64 r 1;
  lemma_glx4_eq_get64 r 2; lemma_glx4_eq_get64 r 3;
  lemma_glx4_eq_get64 a 0; lemma_glx4_eq_get64 a 1;
  lemma_glx4_eq_get64 a 2; lemma_glx4_eq_get64 a 3;
  lemma_glx4_eq_get64 b 0; lemma_glx4_eq_get64 b 1;
  lemma_glx4_eq_get64 b 2; lemma_glx4_eq_get64 b 3;
  Canon.lemma_permute2x128_si256_u64x4 v_IMM8 a b 0;
  Canon.lemma_permute2x128_si256_u64x4 v_IMM8 a b 1;
  Canon.lemma_permute2x128_si256_u64x4 v_IMM8 a b 2;
  Canon.lemma_permute2x128_si256_u64x4 v_IMM8 a b 3
#pop-options

(* ============================================================================
   get_lane_u64 bridge: the real op = `Extra.get_lane_u64_model vec lane`
   = (for v lane < 4) `(Canon.to_u64x4 vec).[cast lane]` = `get_lane_u64x4 vec
   (v lane)`.  Matches `Avx2_extract`'s `get_lane_u64_post` SMTPat.
   ========================================================================== *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let get_lane_u64_post (vec: t_Vec256) (lane: usize)
  : Lemma (requires Int.v lane < 4)
          (ensures get_lane_u64 vec lane == get_lane_u64x4 vec (Int.v lane))
          [SMTPat (get_lane_u64 vec lane)] =
  reveal_opaque (`%get_lane_u64) get_lane_u64;
  reveal_opaque (`%Extra.get_lane_u64_model) Extra.get_lane_u64_model;
  reveal_opaque (`%vec256_as_u64x4) vec256_as_u64x4
#pop-options

(* ============================================================================
   Byte STORE bridge (codec) for `mm256_storeu_si256_u8`
   (= `Extra.mm256_storeu_si256_u8_model`, a 32-deep update chain over the
   `to_u8x32` codec view).  Mirrors NEON `Arm64_sha3_views` at 32 bytes.
   ========================================================================== *)

(* u8x32 codec (Int_vec_interp width 256 / lane 8) + its round-trip. *)
let to_u8x32   = IVi.e_ee_8__impl__to_u8x32
let from_u8x32 = IVi.e_ee_8__impl__from_u8x32
let rt_u8x32 (y: Funarr.t_FunArray (mk_u64 32) u8)
  : Lemma (to_u8x32 (from_u8x32 y) == y)
  = IVi.lemma_conv_rt Int.U8 (mk_u64 256) (mk_u64 32) y

(* ── u8x32 lane view (mirror the u64x4 view) ──────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec256_as_u8x32 (x: t_Vec256) : t_Array u8 (sz 32) =
  Seq.init 32 (fun i -> Funarr.impl_5__get (mk_u64 32) #u8 (to_u8x32 x) (mk_u64 i))
let get_lane_u8x32 (v: t_Vec256) (i: nat{i < 32}) : u8 = Seq.index (vec256_as_u8x32 v) i

let vec256_index_u8x32 (x: t_Vec256) (i: nat{i < 32})
  : Lemma (Seq.index (vec256_as_u8x32 x) i
           == Funarr.impl_5__get (mk_u64 32) #u8 (to_u8x32 x) (mk_u64 i))
          [SMTPat (Seq.index (vec256_as_u8x32 x) i)]
  = reveal_opaque (`%vec256_as_u8x32) vec256_as_u8x32

(* ── recursive prefix-store abstraction (mirror NEON `upd_prefix_u8`): the
      32-deep model update chain defeats a monolithic peel, so characterize it
      by a recursion proving one `Seq.upd` step per level. ─────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 100"
let rec upd_prefix_u8 (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 32) u8)
                      (n: nat{n <= 32 /\ Seq.length out >= 32})
  : Tot (r: t_Slice u8 {Seq.length r == Seq.length out}) (decreases n) =
  if n = 0 then out
  else Rust_primitives.Hax.Monomorphized_update_at.update_at_usize
         (upd_prefix_u8 out lanes (n - 1)) (mk_usize (n - 1)) (lanes.[ mk_u64 (n - 1) ])

let rec lemma_upd_prefix_u8_index (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 32) u8)
                                  (n: nat{n <= 32 /\ Seq.length out >= 32}) (k: nat{k < 32})
  : Lemma (ensures Seq.index (upd_prefix_u8 out lanes n) k
                   == (if k < n then lanes.[ mk_u64 k ] else Seq.index out k))
          (decreases n) =
  if n = 0 then () else lemma_upd_prefix_u8_index out lanes (n - 1) k

let rec lemma_upd_prefix_u8_frame (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 32) u8)
                                  (n: nat{n <= 32 /\ Seq.length out >= 32})
                                  (k: nat{32 <= k /\ k < Seq.length out})
  : Lemma (ensures Seq.index (upd_prefix_u8 out lanes n) k == Seq.index out k)
          (decreases n) =
  if n = 0 then () else lemma_upd_prefix_u8_frame out lanes (n - 1) k
#pop-options

(* the revealed model chain (len >= 32 branch) IS upd_prefix_u8 32. *)
#push-options "--fuel 34 --ifuel 2 --z3rlimit 400"
let lemma_storeu_u8_model_eq (out: t_Slice u8) (v: t_Vec256)
  : Lemma (requires Seq.length out >= 32)
          (ensures mm256_storeu_si256_u8 out v == upd_prefix_u8 out (to_u8x32 v) 32) =
  reveal_opaque (`%mm256_storeu_si256_u8) mm256_storeu_si256_u8;
  reveal_opaque (`%Extra.mm256_storeu_si256_u8_model) Extra.mm256_storeu_si256_u8_model
#pop-options

(* ── byte STORE bridge (codec): out'.[i] == get_lane_u8x32 v i for i<32, suffix
      preserved, length preserved. ─────────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_storeu_u8_lane (out: t_Slice u8) (v: t_Vec256) (i: nat{i < 32})
  : Lemma (requires Seq.length out >= 32)
          (ensures Seq.index (mm256_storeu_si256_u8 out v) i == get_lane_u8x32 v i) =
  lemma_storeu_u8_model_eq out v;
  lemma_upd_prefix_u8_index out (to_u8x32 v) 32 i

let lemma_storeu_u8_frame (out: t_Slice u8) (v: t_Vec256) (i: nat)
  : Lemma (requires Seq.length out >= 32 /\ i >= 32 /\ i < Seq.length out)
          (ensures Seq.index (mm256_storeu_si256_u8 out v) i == Seq.index out i) =
  lemma_storeu_u8_model_eq out v;
  lemma_upd_prefix_u8_frame out (to_u8x32 v) 32 i

let lemma_storeu_u8_length (out: t_Slice u8) (v: t_Vec256)
  : Lemma (ensures Seq.length (mm256_storeu_si256_u8 out v) == Seq.length out)
          [SMTPat (Seq.length (mm256_storeu_si256_u8 out v))] =
  reveal_opaque (`%mm256_storeu_si256_u8) mm256_storeu_si256_u8;
  reveal_opaque (`%Extra.mm256_storeu_si256_u8_model) Extra.mm256_storeu_si256_u8_model
#pop-options

(* ── byte LOAD bridge (codec): get_lane_u8x32 (result) i == input.[i] for i<32. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_loadu_u8_lane (input: t_Slice u8) (i: nat{i < 32})
  : Lemma (requires Seq.length input >= 32)
          (ensures get_lane_u8x32 (mm256_loadu_si256_u8 input) i == Seq.index input i) =
  reveal_opaque (`%mm256_loadu_si256_u8) mm256_loadu_si256_u8;
  reveal_opaque (`%Extra.mm256_loadu_si256_u8_model) Extra.mm256_loadu_si256_u8_model;
  reveal_opaque (`%vec256_as_u8x32) vec256_as_u8x32;
  let fa = Funarr.impl_5__from_fn (mk_u64 32) #u8 #(u64 -> u8)
             (fun j -> let j:u64 = j in
                       if (cast j <: usize) <. (Core_models.Slice.impl__len #u8 input <: usize)
                       then input.[ cast j <: usize ] else mk_u8 0) in
  rt_u8x32 fa
#pop-options

(* ============================================================================
   u8x32 <-> u64x4 REPACK (pure CODEC, ZERO trust): byte (8*i+b) of the u8 view
   equals byte b of u64-lane i, expressed as `cast (lane >>! 8b) <: u8`.  Both
   views read the SAME 256-bit BitVec; readback (U8 256 32 and U64 256 4)
   collapses each byte-bit to the same absolute bit `64i+8b+c`.  Mirror of NEON
   `lemma_u8x16_u64x2_repack`.
   ========================================================================== *)

(* bit `i` of a core-models `t_BitVec n`, as a Rust bit. *)
let bv_bit (#n: u64) (bv: BV.t_BitVec n) (i: nat{i < v n}) : Int.bit =
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

(* the repack: byte (8i+b) of the u8x32 view == byte b of u64-lane i (shift form). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_u8x32_u64x4_repack (vv: t_Vec256) (i: nat{i < 4}) (b: nat{b < 8})
  : Lemma (get_lane_u8x32 vv (8 * i + b)
           == (cast (get_lane_u64x4 vv i >>! mk_u32 (8 * b)) <: u8)) =
  reveal_opaque (`%vec256_as_u8x32) vec256_as_u8x32;
  reveal_opaque (`%vec256_as_u64x4) vec256_as_u64x4;
  let ybyte : u8 = get_lane_u8x32 vv (8 * i + b) in
  let ylane : u64 = get_lane_u64x4 vv i in
  let target : u8 = cast (ylane >>! mk_u32 (8 * b)) <: u8 in
  let aux (c: usize{v c < 8})
    : Lemma (Int.get_bit #Int.U8 ybyte c == Int.get_bit #Int.U8 target c) =
    Canon.lemma_readback Int.U8 (mk_u64 256) (mk_u64 32) vv (mk_u64 (8 * i + b)) (v c);
    lemma_bv_bit_reader #(mk_u64 256) 8 vv (8 * i + b) (v c);
    Canon.lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) vv (mk_u64 i) (8 * b + v c);
    lemma_bv_bit_reader #(mk_u64 256) 64 vv i (8 * b + v c);
    assert (8 * (8 * i + b) + v c == 64 * i + 8 * b + v c)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U8 ybyte target
#pop-options

(* ============================================================================
   le_bytes-SPELLING byte bridges: compose the axiom-free codec facts (repack /
   byte load) with the core-models Trusted le_bytes semantics axioms
   (`Trusted.Intrinsics.lemma_u64_{to,from}_le_bytes_*`) so the SHA3 store/load
   consumers — whose `stored` predicate and the to_le_bytes-defined reference
   spec speak in to_le_bytes / from_le_bytes form — reconnect.  These REPLACE
   the `Avx2_extract` byte op-ensures (which asserted the same le_bytes
   facts as TRUSTED); net trust drops to the two core-models axioms.
   ========================================================================== *)

(* byte-form of the repack: byte k of the u8 view == byte (k%8) of to_le_bytes of
   u64-lane (k/8).  repack (codec) + to_le_bytes axiom (codec == to_le_bytes). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_get_lane_u8x32_eq_to_le_bytes (vv: t_Vec256) (k: nat{k < 32})
  : Lemma (get_lane_u8x32 vv k
           == (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x4 vv (k / 8))
               <: t_Array u8 (mk_usize 8)).[ mk_usize (k % 8) ]) =
  FStar.Math.Lemmas.euclidean_division_definition k 8;
  lemma_u8x32_u64x4_repack vv (k / 8) (k % 8);
  Libcrux_core_models.Trusted.Intrinsics.lemma_u64_to_le_bytes_index
    (get_lane_u64x4 vv (k / 8)) (k % 8)
#pop-options

(* the `Avx2_extract` `lemma_mm256_storeu_si256_u8_byte` replacement, in to_le_bytes form
   over the REAL `get_lane_u64` (same name/statement as the `Avx2_extract` interface's, so
   StoreBlockHelpers.Avx2's by-name call repoints verbatim): the stored byte k
   is byte (k%8) of to_le_bytes(get_lane_u64 vector (k/8)). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_storeu_si256_u8_byte (output: t_Slice u8) (vector: t_Vec256) (k: nat)
  : Lemma
      (requires Seq.length output == 32 /\ k < 32)
      (ensures
        Seq.index (mm256_storeu_si256_u8 output vector <: t_Slice u8) k ==
        Seq.index
          (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vector (mk_usize (k / 8))))
          (k % 8)) =
  lemma_storeu_u8_lane output vector k;
  lemma_get_lane_u8x32_eq_to_le_bytes vector k;
  get_lane_u64_post vector (mk_usize (k / 8))
#pop-options

(* LOAD dual: u64-lane `lane` of a byte-loaded vector == from_le_bytes of the 8
   little-endian bytes at [8*lane, 8*lane+8) of the window.  byte-load fact
   (codec) + from_le_bytes bit axiom, bit-by-bit.  Consumers (load_u64x4x4) use
   this to reconnect `get_lane_u64x4 (mm256_loadu_si256_u8 window) lane` to the
   `from_le_bytes` term in `load_lane_u64`.  Mirror NEON
   `lemma_get_lane_u64x2_vld1q_bytes_le`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_get_lane_u64x4_loadu_le (window: t_Slice u8) (lane: nat{lane < 4})
  : Lemma (requires Seq.length window >= 32)
          (ensures get_lane_u64x4 (mm256_loadu_si256_u8 window) lane
                   == Core_models.Num.impl_u64__from_le_bytes
                        (Seq.slice window (8 * lane) (8 * lane + 8) <: t_Array u8 (mk_usize 8)))
          [SMTPat (get_lane_u64x4 (mm256_loadu_si256_u8 window) lane)] =
  reveal_opaque (`%vec256_as_u8x32) vec256_as_u8x32;
  reveal_opaque (`%vec256_as_u64x4) vec256_as_u64x4;
  let vv = mm256_loadu_si256_u8 window in
  let y : u64 = get_lane_u64x4 vv lane in
  let bs : t_Array u8 (mk_usize 8) = Seq.slice window (8 * lane) (8 * lane + 8) in
  let fromle : u64 = Core_models.Num.impl_u64__from_le_bytes bs in
  let aux (k: usize{v k < 64})
    : Lemma (Int.get_bit #Int.U64 y k == Int.get_bit #Int.U64 fromle k) =
    FStar.Math.Lemmas.euclidean_division_definition (v k) 8;
    Canon.lemma_readback Int.U64 (mk_u64 256) (mk_u64 4) vv (mk_u64 lane) (v k);
    lemma_bv_bit_reader #(mk_u64 256) 64 vv lane (v k);
    Canon.lemma_readback Int.U8 (mk_u64 256) (mk_u64 32) vv (mk_u64 (8 * lane + (v k) / 8)) ((v k) % 8);
    lemma_bv_bit_reader #(mk_u64 256) 8 vv (8 * lane + (v k) / 8) ((v k) % 8);
    lemma_loadu_u8_lane window (8 * lane + (v k) / 8);
    Libcrux_core_models.Trusted.Intrinsics.lemma_u64_from_le_bytes_bit bs (v k);
    assert (Seq.index bs ((v k) / 8) == Seq.index window (8 * lane + (v k) / 8));
    assert (8 * (8 * lane + (v k) / 8) + (v k) % 8 == 64 * lane + v k)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 y fromle
#pop-options

(* ── loadu window-lane bridge: lane `lane` of a vector loaded from the 32-byte
      window `block[start, start+32)` == from_le_bytes of the 8 bytes at
      `block[start+8*lane, start+8*lane+8)`.  Reduces the range-index to Seq.slice,
      applies the codec loadu fact, and `slice_slice`-collapses the nested slice —
      so the AVX2 `load_u64x4x4` leaf never spells out the window↔block split.
      Consumes the exact hax Range-record indexing form the extraction produces. ─ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_loadu_window_lane (block: t_Slice u8) (start: usize) (lane: nat{lane < 4})
  : Lemma (requires v start + 32 <= Seq.length block)
          (ensures
            get_lane_u64x4
              (mm256_loadu_si256_u8
                 (block.[ ({ Core_models.Ops.Range.f_start = start;
                             Core_models.Ops.Range.f_end = start +! mk_usize 32 <: usize }
                           <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8))
              lane
            == Core_models.Num.impl_u64__from_le_bytes
                 (Seq.slice block (v start + 8 * lane) (v start + 8 * lane + 8)
                  <: t_Array u8 (mk_usize 8))) =
  let window : t_Slice u8 =
    block.[ ({ Core_models.Ops.Range.f_start = start;
               Core_models.Ops.Range.f_end = start +! mk_usize 32 <: usize }
             <: Core_models.Ops.Range.t_Range usize) ] in
  assert (window == Seq.slice block (v start) (v start + 32));
  lemma_get_lane_u64x4_loadu_le window lane;
  FStar.Seq.Properties.slice_slice block (v start) (v start + 32) (8 * lane) (8 * lane + 8)
#pop-options

(* ============================================================================
   `try_into`-array <-> Seq.slice bridge (pure hax proof-lib plumbing, no trust).
   `load_lane_u64` spells its 8 input bytes as `impl__unwrap (f_try_into
   (slice_slice blocks[L] lo (lo+8)))` (Rust `.try_into().unwrap()`), whereas
   `lemma_get_lane_u64x4_loadu_le` produces the from_le_bytes arg as `Seq.slice
   window …`.  Same 8 bytes; this reduces the try_into-array to the Seq.slice so
   the two from_le_bytes args match.  Identical to the NEON companion's lemma.
   ========================================================================== *)
#push-options "--fuel 4 --ifuel 2 --z3rlimit 300"
let lemma_slice8_as_array (s: t_Slice u8) (lo: usize)
  : Lemma (requires v lo + 8 <= Seq.length s)
          (ensures
            (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 8))
               #Core_models.Array.t_TryFromSliceError
               (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 (mk_usize 8))
                  #FStar.Tactics.Typeclasses.solve
                  (Rust_primitives.Slice.slice_slice s lo (lo +! mk_usize 8) <: t_Slice u8))
             <: t_Array u8 (mk_usize 8))
            == Seq.slice s (v lo) (v lo + 8)) =
  let sub : t_Slice u8 = Rust_primitives.Slice.slice_slice s lo (lo +! mk_usize 8) in
  assert (Seq.length sub == 8);
  assert (Core_models.Slice.impl__len #u8 sub == mk_usize 8);
  let arr : t_Array u8 (mk_usize 8) =
    Rust_primitives.Slice.array_from_fn #u8 (mk_usize 8) #(usize -> u8)
      (fun i -> let i:usize = i in Rust_primitives.Slice.slice_index #u8 sub i) in
  assert (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 (mk_usize 8))
            #FStar.Tactics.Typeclasses.solve sub
          == (Core_models.Result.Result_Ok arr
              <: Core_models.Result.t_Result (t_Array u8 (mk_usize 8))
                   Core_models.Array.t_TryFromSliceError))
    by (FStar.Tactics.norm [delta_only [`%Core_models.Convert.f_try_into;
                                        `%Core_models.Convert.f_try_from;
                                        `%Core_models.Convert.impl_2;
                                        `%Core_models.Convert.impl_3];
                            iota; zeta];
        FStar.Tactics.smt ());
  Seq.lemma_eq_intro arr (Seq.slice s (v lo) (v lo + 8))
#pop-options
