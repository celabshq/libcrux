module Libcrux_intrinsics.Arm64_sha3_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models

(* ============================================================================
   sha3 NEON (Arm64) lane-view + per-op fact companion.

   The u64 analog of `Libcrux_intrinsics.Arm64_ml_kem_views` (which carries the
   i16/i32 families for the ml-kem NTT).  It exposes to sha3's NEON proofs the
   u64x2 lane VIEW (`vec128_as_u64x2` / `get_lane_u64x2`) and the per-op FACT
   lemmas that the hand-written `Libcrux_intrinsics.Arm64_extract` interface
   carried as op `ensures`, now phrased over the REAL `Libcrux_intrinsics.Arm64`
   ops (which delegate to the differentially-tested `libcrux-core-models` NEON
   model + `Arm.Extra` slice-I/O models).

   TRUST.  The Seq lane view is a per-index read of the canonical core-models
   codec (`NV.to_u64x2` = `Int_vec_interp` width 128 / lane 64), and every
   op-fact is PROVEN from the canonical NEON op-lemma set in
   `Libcrux_core_models.Neon_views` (which rests only on the differentially
   tested `Arm.Interpretations.Int_vec.Lemmas` lifts + the PROVEN codec
   round-trip).  Every fact here is PROVEN, not an assumed op `ensures`.  NO
   fact in this module is assumed.

   It `include`s the real `Libcrux_intrinsics.Arm64` so consumers that alias
   this module as `I` resolve `I.e_vOP_u64` to the real op AND
   `I.vec128_as_u64x2` / `I.get_lane_u64x2` to the views below.  Lives in
   `proofs/fstar/spec/` (hand-maintained), on sha3's include path only; NOT a
   make ROOT (verifies as a dependency of the repointed NEON consumers).

   sha3 NEON uses ONLY the u64x2 family (Keccak lanes) — no i64x2 / vsli.
   Ops covered: dupq_n, veorq, veor3q, vbcaxq, vrax1q, vxarq, vtrn1q, vtrn2q,
   the u64 loads/stores (vld1q_u64 / vld1q_bytes_u64 / vst1q_u64 /
   vst1q_bytes_u64) and the get_lane_u64 bridge.
   ========================================================================== *)

include Libcrux_intrinsics.Arm64

module Funarr = Libcrux_core_models.Abstractions.Funarr
module BV     = Libcrux_core_models.Abstractions.Bitvec
module Bit    = Libcrux_core_models.Abstractions.Bit
module Canon  = Libcrux_core_models.Intrinsics_views
module NV     = Libcrux_core_models.Neon_views
module ArmIV  = Libcrux_core_models.Core_arch.Arm.Interpretations.Int_vec
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Extra  = Libcrux_core_models.Core_arch.Arm.Extra
module Num    = Core_models.Num
module Int    = Rust_primitives.Integers

(* ── lane-view type (mirrors the `Arm64_extract` `t_e_uint64x2_t`) ─────────── *)
unfold type t_e_uint64x2_t = BV.t_BitVec (mk_u64 128)

(* ── u64x2 lane view (A-on-B adapter over canonical NV.to_u64x2).  OPAQUE for
      the same reasons as ml-kem's `vec128_as_i16x8`: keeps the `Arm64_extract`
      abstraction (still PROVEN, not assumed); the only route to the codec is the index
      lemma below. ─────────────────────────────────────────────────────────── *)
[@@ "opaque_to_smt"]
let vec128_as_u64x2 (x: t_e_uint64x2_t) : t_Array u64 (sz 2) =
  Seq.init 2 (fun i -> Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 x) (mk_u64 i))
let get_lane_u64x2 (v: t_e_uint64x2_t) (i: nat{i < 2}) : u64 = Seq.index (vec128_as_u64x2 v) i

let vec128_index_u64x2 (x: t_e_uint64x2_t) (i: nat{i < 2})
  : Lemma (Seq.index (vec128_as_u64x2 x) i
           == Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_u64x2 x) i)]
  = reveal_opaque (`%vec128_as_u64x2) vec128_as_u64x2

let vec128_as_u64x2_len (x: t_e_uint64x2_t)
  : Lemma (Seq.length (vec128_as_u64x2 x) == 2)
          [SMTPat (Seq.length (vec128_as_u64x2 x))]
  = ()

let vec128_as_u64x2_slice_ok (x: t_e_uint64x2_t)
  : Lemma (Seq.length (vec128_as_u64x2 x) <= Int.max_usize)
          [SMTPat (vec128_as_u64x2 x)]
  = assert_norm (2 <= Int.max_usize)

(* ============================================================================
   FunArray op-facts (Shape-A per-lane-codec): the real `e_vOP_u64` delegates to
   `Neon.vOP_u64` / `ArmHW.vOP_u64` (transparent), `NV.lemma_vOP_u64` gives the
   VIEW-level `to_u64x2 (op) == ArmIV.vOP_u64 (to_u64x2 a) ...`, `ArmIV.vOP_u64`
   is a per-lane FunArray op, so `Seq.lemma_eq_intro` closes.  Mirrors the
   ml-kem `Arm64_ml_kem_views.lemma_e_vaddq_s16` recipe.
   ========================================================================== *)

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vdupq_n_u64 (c: u64)
  : Lemma (vec128_as_u64x2 (e_vdupq_n_u64 c) == Seq.create 2 c)
          [SMTPat (vec128_as_u64x2 (e_vdupq_n_u64 c))] =
  NV.lemma_vdupq_n_u64 c;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vdupq_n_u64 c)) (Seq.create 2 c)
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_veor3q_u64 (a b c: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_veor3q_u64 a b c)
           == Seq.init 2 (fun i -> (Seq.index (vec128_as_u64x2 a) i ^. Seq.index (vec128_as_u64x2 b) i)
                                    ^. Seq.index (vec128_as_u64x2 c) i))
          [SMTPat (vec128_as_u64x2 (e_veor3q_u64 a b c))] =
  NV.lemma_veor3q_u64 a b c;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_veor3q_u64 a b c))
                     (Seq.init 2 (fun i -> (Seq.index (vec128_as_u64x2 a) i ^. Seq.index (vec128_as_u64x2 b) i)
                                           ^. Seq.index (vec128_as_u64x2 c) i))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vbcaxq_u64 (a b c: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_vbcaxq_u64 a b c)
           == Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i
                                   ^. (Seq.index (vec128_as_u64x2 b) i
                                       &. (~. (Seq.index (vec128_as_u64x2 c) i)))))
          [SMTPat (vec128_as_u64x2 (e_vbcaxq_u64 a b c))] =
  NV.lemma_vbcaxq_u64 a b c;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vbcaxq_u64 a b c))
                     (Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i
                                           ^. (Seq.index (vec128_as_u64x2 b) i
                                               &. (~. (Seq.index (vec128_as_u64x2 c) i)))))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vrax1q_u64 (a b: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_vrax1q_u64 a b)
           == Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i
                                   ^. Num.impl_u64__rotate_left (Seq.index (vec128_as_u64x2 b) i) (mk_u32 1)))
          [SMTPat (vec128_as_u64x2 (e_vrax1q_u64 a b))] =
  NV.lemma_vrax1q_u64 a b;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vrax1q_u64 a b))
                     (Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i
                                           ^. Num.impl_u64__rotate_left (Seq.index (vec128_as_u64x2 b) i) (mk_u32 1)))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn1q_u64 (a b: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_vtrn1q_u64 a b)
           == Seq.init 2 (fun i -> if i = 0 then Seq.index (vec128_as_u64x2 a) 0
                                            else Seq.index (vec128_as_u64x2 b) 0))
          [SMTPat (vec128_as_u64x2 (e_vtrn1q_u64 a b))] =
  NV.lemma_vtrn1q_u64 a b;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vtrn1q_u64 a b))
                     (Seq.init 2 (fun i -> if i = 0 then Seq.index (vec128_as_u64x2 a) 0
                                                    else Seq.index (vec128_as_u64x2 b) 0))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vtrn2q_u64 (a b: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_vtrn2q_u64 a b)
           == Seq.init 2 (fun i -> if i = 0 then Seq.index (vec128_as_u64x2 a) 1
                                            else Seq.index (vec128_as_u64x2 b) 1))
          [SMTPat (vec128_as_u64x2 (e_vtrn2q_u64 a b))] =
  NV.lemma_vtrn2q_u64 a b;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vtrn2q_u64 a b))
                     (Seq.init 2 (fun i -> if i = 0 then Seq.index (vec128_as_u64x2 a) 1
                                                    else Seq.index (vec128_as_u64x2 b) 1))
#pop-options

(* ── vxarq: real op = ArmHW.vxarq_u64 v_RIGHT a b = per-lane rotate of (a^b).
      The core-models model `ArmIV.vxarq_u64` expresses the XAR right-rotate
      as the EQUIVALENT left rotation `rotate_LEFT (a^b) by ((64 - v_RIGHT%64)%64)`
      (`rotate_right x k == rotate_left x ((64-k)%64)` for a 64-bit word) — the
      form the Keccak-rho equivalence consumers need, keeping this companion
      axiom-free.  This companion fact just lifts that model per-lane; a consumer
      with `v_LEFT + v_RIGHT = 64 /\ 0 < v_RIGHT < 64` then rewrites the rotate
      amount `(64 - v_RIGHT%64)%64` to `cast v_LEFT` by u32 arithmetic. ──────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_e_vxarq_u64 (v_LEFT v_RIGHT: i32) (a b: t_e_uint64x2_t)
  : Lemma (requires Int.v v_LEFT + Int.v v_RIGHT == 64)
          (ensures
            vec128_as_u64x2 (e_vxarq_u64 v_LEFT v_RIGHT a b)
            == Seq.init 2 (fun i ->
                 Num.impl_u64__rotate_left
                   (Seq.index (vec128_as_u64x2 a) i ^. Seq.index (vec128_as_u64x2 b) i)
                   ((mk_u32 64 -! ((cast v_RIGHT <: u32) %! mk_u32 64) <: u32) %! mk_u32 64)))
          [SMTPat (vec128_as_u64x2 (e_vxarq_u64 v_LEFT v_RIGHT a b))] =
  NV.lemma_vxarq_u64 v_RIGHT a b;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vxarq_u64 v_LEFT v_RIGHT a b))
                     (Seq.init 2 (fun i ->
                        Num.impl_u64__rotate_left
                          (Seq.index (vec128_as_u64x2 a) i ^. Seq.index (vec128_as_u64x2 b) i)
                          ((mk_u32 64 -! ((cast v_RIGHT <: u32) %! mk_u32 64) <: u32) %! mk_u32 64)))
#pop-options

(* ============================================================================
   veorq_u64 codec-commute.  `ArmIV.veorq_u64 = veorq_s16` is a BIT-LEVEL
   from_fn XOR over 128 bits (not a per-lane FunArray op), so the u64-lane fact
   needs the codec to commute with bitwise XOR: reading a 64-bit lane of a
   bitwise-XOR equals the XOR of the two lanes.  Ported from the x86 i16x16
   template `Intrinsics_views.lemma_xor_i16x16_iv` (width 128 / lane 64).  For
   arm the `veorq` bit model is DEFINITIONAL (`veorq_u64 = veorq_s16`), so no
   lift axiom is needed — the raw-bit fact is closed by norm/trefl.
   ========================================================================== *)

(* width-128 bv-index helpers (analogs of Intrinsics_views' width-256 ones). *)
let lemma_bv_index128 (bv: BV.t_BitVec (mk_u64 128)) (k: u64{v k < 128})
  : Lemma ((bv.[ k ] <: Bit.t_Bit) == Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit bv._0 k) = ()

let lemma_impl9_index128 (f: (i: u64{v i < 128}) -> Bit.t_Bit) (k: u64{v k < 128})
  : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit
             (BV.impl_9__from_fn (mk_u64 128) #(u64 -> Bit.t_Bit) f)._0 k == f k) = ()

(* raw-bit semantics of ArmIV.veorq_u64 (= veorq_s16): bit k is XOR of the bits. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_veorq_funarr (a b: t_e_uint64x2_t) (k: u64{v k < 128})
    : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit (ArmIV.veorq_u64 a b)._0 k ==
             (match Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit a._0 k,
                    Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit b._0 k
              with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | Bit.Bit_One,  Bit.Bit_One  -> Bit.Bit_Zero
              | _ -> Bit.Bit_One)) =
  let f : (i: u64{v i < 128}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_Zero, Bit.Bit_Zero -> Bit.Bit_Zero
              | Bit.Bit_One,  Bit.Bit_One  -> Bit.Bit_Zero
              | _ -> Bit.Bit_One) in
  assert (ArmIV.veorq_u64 a b ==
          BV.impl_9__from_fn (mk_u64 128) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%ArmIV.veorq_u64; `%ArmIV.veorq_s16];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  lemma_impl9_index128 f k;
  lemma_bv_index128 a k;
  lemma_bv_index128 b k
#pop-options

(* raw XOR at the u64 lane_reader granularity. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_veorq_raw (a b: t_e_uint64x2_t) (ii: u64{v ii < 2}) (bb: nat{bb < 64})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 64 (ArmIV.veorq_u64 a b) ii bb) ==
             Int.bit_xor (IVi.bval (IVi.lane_reader (mk_u64 128) 64 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 128) 64 b ii bb))) =
  assert (64 * v ii + bb < 128);
  lemma_veorq_funarr a b (mk_u64 (64 * v ii + bb))
#pop-options

(* u64-lane commutation for veorq: decode ∘ bitwise-xor == `^.`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_xor_u64x2_iv (a b: t_e_uint64x2_t) (i: nat{i < 2})
    : Lemma (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 (ArmIV.veorq_u64 a b)) (mk_u64 i) ==
             (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 a) (mk_u64 i) ^.
              Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 b) (mk_u64 i))) =
  let aXORb = ArmIV.veorq_u64 a b in
  let ya : u64 = Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 a) (mk_u64 i) in
  let yb : u64 = Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 b) (mk_u64 i) in
  let yr : u64 = Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 aXORb) (mk_u64 i) in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yr bb == Int.get_bit #Int.U64 (ya ^. yb) bb) =
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) aXORb (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) a (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) b (mk_u64 i) (v bb);
    lemma_veorq_raw a b (mk_u64 i) (v bb);
    Int.get_bit_xor #Int.U64 ya yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yr (ya ^. yb)
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_veorq_u64 (a b: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_veorq_u64 a b)
           == Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i ^. Seq.index (vec128_as_u64x2 b) i))
          [SMTPat (vec128_as_u64x2 (e_veorq_u64 a b))] =
  NV.lemma_veorq_u64 a b;
  let aux (i: nat{i < 2})
    : Lemma (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 (ArmIV.veorq_u64 a b)) (mk_u64 i)
             == (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 a) (mk_u64 i)
                 ^. Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 b) (mk_u64 i))) =
    lemma_xor_u64x2_iv a b i
  in
  FStar.Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_veorq_u64 a b))
                     (Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i ^. Seq.index (vec128_as_u64x2 b) i))
#pop-options

(* ============================================================================
   get_lane_u64 bridge: the real op = `Extra.get_lane_u64_model vec lane`
   = (for lane<2) `(to_u64x2 vec).[lane]` = `get_lane_u64x2 vec (v lane)`.
   ========================================================================== *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_get_lane_u64 (vec: t_e_uint64x2_t) (lane: usize)
  : Lemma (requires Int.v lane < 2)
          (ensures get_lane_u64 vec lane == get_lane_u64x2 vec (Int.v lane))
          [SMTPat (get_lane_u64 vec lane)] =
  reveal_opaque (`%get_lane_u64) get_lane_u64;
  reveal_opaque (`%Extra.get_lane_u64_model) Extra.get_lane_u64_model;
  reveal_opaque (`%vec128_as_u64x2) vec128_as_u64x2
#pop-options

(* ============================================================================
   u64 loads/stores (non-byte): read/write whole u64 lanes.
   ========================================================================== *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_e_vld1q_u64 (array: t_Slice u64)
  : Lemma (requires Seq.length array >= 2)
          (ensures (forall (i: nat{i < 2}).
                      get_lane_u64x2 (e_vld1q_u64 array) i == Seq.index array i)) =
  reveal_opaque (`%e_vld1q_u64) e_vld1q_u64;
  reveal_opaque (`%Extra.vld1q_u64_model) Extra.vld1q_u64_model;
  reveal_opaque (`%vec128_as_u64x2) vec128_as_u64x2;
  let fa = Funarr.impl_5__from_fn (mk_u64 2) #u64 #(u64 -> u64)
             (fun j -> let j:u64 = j in
                       if (cast j <: usize) <. (Core_models.Slice.impl__len #u64 array <: usize)
                       then array.[ cast j <: usize ] else mk_u64 0) in
  NV.rt_u64x2 fa
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_e_vst1q_u64 (out: t_Slice u64) (v: t_e_uint64x2_t)
  : Lemma (requires Seq.length out >= 2)
          (ensures (let out' = e_vst1q_u64 out v in
                    Seq.length out' == Seq.length out /\
                    (forall (i: nat{i < 2}). Seq.index out' i == get_lane_u64x2 v i))) =
  reveal_opaque (`%e_vst1q_u64) e_vst1q_u64;
  reveal_opaque (`%Extra.vst1q_u64_model) Extra.vst1q_u64_model;
  reveal_opaque (`%vec128_as_u64x2) vec128_as_u64x2
#pop-options

(* ============================================================================
   Byte load/store bridges — CODEC form (over the `to_u8x16` view), ZERO new
   trust (codec-form, not a le_bytes axiom).

   The `Arm64_extract` interface stated the byte fact in `to_le_bytes`/`from_le_bytes`
   form, but `Core_models.Num.impl_u64__{to,from}_le_bytes` are `assume val`
   (no semantics) — so that spelling is unprovable without a le_bytes↔bits axiom.
   The real ops share `Arm.Extra`'s `{vst1q,vld1q}_bytes_model`, which are
   byte-granular over the `to_u8x16` CODEC view; we bridge to that.  Consumers
   (Simd.Arm64.{Store,Load,StoreBlockHelpers}) use the codec form to match.  The
   u8x16 <-> u64x2 REPACK (`get_lane_u8x16 v (8i+b)` == byte b of
   `get_lane_u64x2 v i`, via `Canon.lemma_readback` on both views) is a pure
   codec fact — added here once a consumer pins its exact needed form.
   ========================================================================== *)

(* ── u8x16 lane view (mirror the u64x2 view) ──────────────────────────────── *)
unfold type t_e_uint8x16_t = BV.t_BitVec (mk_u64 128)

[@@ "opaque_to_smt"]
let vec128_as_u8x16 (x: t_e_uint8x16_t) : t_Array u8 (sz 16) =
  Seq.init 16 (fun i -> Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 i))
let get_lane_u8x16 (v: t_e_uint8x16_t) (i: nat{i < 16}) : u8 = Seq.index (vec128_as_u8x16 v) i

let vec128_index_u8x16 (x: t_e_uint8x16_t) (i: nat{i < 16})
  : Lemma (Seq.index (vec128_as_u8x16 x) i
           == Funarr.impl_5__get (mk_u64 16) #u8 (NV.to_u8x16 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_u8x16 x) i)]
  = reveal_opaque (`%vec128_as_u8x16) vec128_as_u8x16

(* ── recursive prefix-store abstraction (mirror ml-kem `upd_prefix_u8`): the
      16-deep model update chain defeats a monolithic peel, so characterize it by
      a recursion proving one `Seq.upd` step per level. ───────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 100"
let rec upd_prefix_u8 (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 16) u8)
                      (n: nat{n <= 16 /\ Seq.length out >= 16})
  : Tot (r: t_Slice u8 {Seq.length r == Seq.length out}) (decreases n) =
  if n = 0 then out
  else Rust_primitives.Hax.Monomorphized_update_at.update_at_usize
         (upd_prefix_u8 out lanes (n - 1)) (mk_usize (n - 1)) (lanes.[ mk_u64 (n - 1) ])

let rec lemma_upd_prefix_u8_index (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 16) u8)
                                  (n: nat{n <= 16 /\ Seq.length out >= 16}) (k: nat{k < 16})
  : Lemma (ensures Seq.index (upd_prefix_u8 out lanes n) k
                   == (if k < n then lanes.[ mk_u64 k ] else Seq.index out k))
          (decreases n) =
  if n = 0 then () else lemma_upd_prefix_u8_index out lanes (n - 1) k

let rec lemma_upd_prefix_u8_frame (out: t_Slice u8) (lanes: Funarr.t_FunArray (mk_u64 16) u8)
                                  (n: nat{n <= 16 /\ Seq.length out >= 16})
                                  (k: nat{16 <= k /\ k < Seq.length out})
  : Lemma (ensures Seq.index (upd_prefix_u8 out lanes n) k == Seq.index out k)
          (decreases n) =
  if n = 0 then () else lemma_upd_prefix_u8_frame out lanes (n - 1) k
#pop-options

(* the revealed model chain (len >= 16 branch) IS upd_prefix_u8 16. *)
#push-options "--fuel 20 --ifuel 2 --z3rlimit 200"
let lemma_vst1q_bytes_u64_model_eq (out: t_Slice u8) (v: t_e_uint8x16_t)
  : Lemma (requires Seq.length out >= 16)
          (ensures e_vst1q_bytes_u64 out v == upd_prefix_u8 out (NV.to_u8x16 v) 16) =
  reveal_opaque (`%e_vst1q_bytes_u64) e_vst1q_bytes_u64;
  reveal_opaque (`%Extra.vst1q_bytes_model) Extra.vst1q_bytes_model
#pop-options

(* ── byte STORE bridge (codec): out'.[i] == get_lane_u8x16 v i for i<16, suffix
      preserved, length preserved. ─────────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vst1q_bytes_u64_lane (out: t_Slice u8) (v: t_e_uint8x16_t) (i: nat{i < 16})
  : Lemma (requires Seq.length out >= 16)
          (ensures Seq.index (e_vst1q_bytes_u64 out v) i == get_lane_u8x16 v i) =
  lemma_vst1q_bytes_u64_model_eq out v;
  lemma_upd_prefix_u8_index out (NV.to_u8x16 v) 16 i

let lemma_e_vst1q_bytes_u64_frame (out: t_Slice u8) (v: t_e_uint8x16_t) (i: nat)
  : Lemma (requires Seq.length out >= 16 /\ i >= 16 /\ i < Seq.length out)
          (ensures Seq.index (e_vst1q_bytes_u64 out v) i == Seq.index out i) =
  lemma_vst1q_bytes_u64_model_eq out v;
  lemma_upd_prefix_u8_frame out (NV.to_u8x16 v) 16 i

let lemma_e_vst1q_bytes_u64_length (out: t_Slice u8) (v: t_e_uint8x16_t)
  : Lemma (ensures Seq.length (e_vst1q_bytes_u64 out v) == Seq.length out)
          [SMTPat (Seq.length (e_vst1q_bytes_u64 out v))] =
  reveal_opaque (`%e_vst1q_bytes_u64) e_vst1q_bytes_u64;
  reveal_opaque (`%Extra.vst1q_bytes_model) Extra.vst1q_bytes_model
#pop-options

(* ── byte LOAD bridge (codec): get_lane_u8x16 (result) i == array.[i] for i<16. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_e_vld1q_bytes_u64_lane (array: t_Slice u8) (i: nat{i < 16})
  : Lemma (requires Seq.length array >= 16)
          (ensures get_lane_u8x16 (e_vld1q_bytes_u64 array) i == Seq.index array i) =
  reveal_opaque (`%e_vld1q_bytes_u64) e_vld1q_bytes_u64;
  reveal_opaque (`%Extra.vld1q_bytes_model) Extra.vld1q_bytes_model;
  reveal_opaque (`%vec128_as_u8x16) vec128_as_u8x16;
  let fa = Funarr.impl_5__from_fn (mk_u64 16) #u8 #(u64 -> u8)
             (fun j -> let j:u64 = j in
                       if (cast j <: usize) <. (Core_models.Slice.impl__len #u8 array <: usize)
                       then array.[ cast j <: usize ] else mk_u8 0) in
  NV.rt_u8x16 fa
#pop-options

(* ============================================================================
   u8x16 <-> u64x2 REPACK (pure CODEC, ZERO trust): byte (8*i+b) of the u8 view
   equals byte b of u64-lane i, expressed as `cast (lane >>! 8b) <: u8` (NOT
   to_le_bytes).  Both views read the SAME 128-bit BitVec; readback (U8 128 16
   and U64 128 2) collapses each byte-bit to the same absolute bit `64i+8b+c`.
   Ports `bv_bit` / `lemma_bv_bit_reader` from ml-kem `Arm64_ml_kem_views`.
   The le_bytes SPELLING is supplied ON TOP by the core-models Trusted axiom
   `lemma_u64_to_le_bytes_index` — see `lemma_get_lane_u8x16_eq_to_le_bytes`.
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

(* the repack: byte (8i+b) of the u8x16 view == byte b of u64-lane i (shift form). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_u8x16_u64x2_repack (vv: t_e_uint8x16_t) (i: nat{i < 2}) (b: nat{b < 8})
  : Lemma (get_lane_u8x16 vv (8 * i + b)
           == (cast (get_lane_u64x2 vv i >>! mk_u32 (8 * b)) <: u8)) =
  reveal_opaque (`%vec128_as_u8x16) vec128_as_u8x16;
  reveal_opaque (`%vec128_as_u64x2) vec128_as_u64x2;
  let ybyte : u8 = get_lane_u8x16 vv (8 * i + b) in
  let ylane : u64 = get_lane_u64x2 vv i in
  let target : u8 = cast (ylane >>! mk_u32 (8 * b)) <: u8 in
  let aux (c: usize{v c < 8})
    : Lemma (Int.get_bit #Int.U8 ybyte c == Int.get_bit #Int.U8 target c) =
    Canon.lemma_readback Int.U8 (mk_u64 128) (mk_u64 16) vv (mk_u64 (8 * i + b)) (v c);
    lemma_bv_bit_reader #(mk_u64 128) 8 vv (8 * i + b) (v c);
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) vv (mk_u64 i) (8 * b + v c);
    lemma_bv_bit_reader #(mk_u64 128) 64 vv i (8 * b + v c);
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
   spec `Hacspec_sha3.Sponge.{squeeze_state,xor_block_into_state}` speak in
   to_le_bytes / from_le_bytes form — reconnect.  These REPLACE the
   `Arm64_extract` byte op-ensures (which asserted the same le_bytes facts as
   TRUSTED); net trust drops to the two core-models axioms.  See
   [[project_sha3_lebytes_semantics_decision]].
   ========================================================================== *)

(* byte-form of the repack: byte k of the u8 view == byte (k%8) of to_le_bytes of
   u64-lane (k/8).  repack (codec) + to_le_bytes axiom (codec == to_le_bytes). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_get_lane_u8x16_eq_to_le_bytes (vv: t_e_uint8x16_t) (k: nat{k < 16})
  : Lemma (get_lane_u8x16 vv k
           == (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x2 vv (k / 8))
               <: t_Array u8 (mk_usize 8)).[ mk_usize (k % 8) ]) =
  FStar.Math.Lemmas.euclidean_division_definition k 8;
  lemma_u8x16_u64x2_repack vv (k / 8) (k % 8);
  Libcrux_core_models.Trusted.Intrinsics.lemma_u64_to_le_bytes_index
    (get_lane_u64x2 vv (k / 8)) (k % 8)
#pop-options

(* the `Arm64_extract` `e_vst1q_bytes_u64` op-ensures replacement, in to_le_bytes form:
   the stored byte i is byte (i%8) of to_le_bytes(lane i/8).  Consumers
   (Store.store_block chain, StoreBlockHelpers) establish their window forall
   by calling this per byte. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_e_vst1q_bytes_u64_le (out: t_Slice u8) (v: t_e_uint8x16_t) (i: nat{i < 16})
  : Lemma (requires Seq.length out >= 16)
          (ensures Seq.index (e_vst1q_bytes_u64 out v) i
                   == (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x2 v (i / 8))
                       <: t_Array u8 (mk_usize 8)).[ mk_usize (i % 8) ])
          [SMTPat (Seq.index (e_vst1q_bytes_u64 out v) i)] =
  lemma_e_vst1q_bytes_u64_lane out v i;
  lemma_get_lane_u8x16_eq_to_le_bytes v i
#pop-options

(* LOAD dual: u64-lane `lane` of a byte-loaded vector == from_le_bytes of the 8
   little-endian bytes at [8*lane, 8*lane+8) of the window.  byte-load fact
   (codec) + from_le_bytes bit axiom, bit-by-bit.  Consumers (Load.load_u64x2x2)
   use this to reconnect `get_lane_u64x2 (vld1q_bytes window) lane` to the
   `from_le_bytes` term in `load_lane_u64`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_get_lane_u64x2_vld1q_bytes_le (window: t_Slice u8) (lane: nat{lane < 2})
  : Lemma (requires Seq.length window >= 16)
          (ensures get_lane_u64x2 (e_vld1q_bytes_u64 window) lane
                   == Core_models.Num.impl_u64__from_le_bytes
                        (Seq.slice window (8 * lane) (8 * lane + 8) <: t_Array u8 (mk_usize 8)))
          [SMTPat (get_lane_u64x2 (e_vld1q_bytes_u64 window) lane)] =
  reveal_opaque (`%vec128_as_u8x16) vec128_as_u8x16;
  reveal_opaque (`%vec128_as_u64x2) vec128_as_u64x2;
  let vv = e_vld1q_bytes_u64 window in
  let y : u64 = get_lane_u64x2 vv lane in
  let bs : t_Array u8 (mk_usize 8) = Seq.slice window (8 * lane) (8 * lane + 8) in
  let fromle : u64 = Core_models.Num.impl_u64__from_le_bytes bs in
  let aux (k: usize{v k < 64})
    : Lemma (Int.get_bit #Int.U64 y k == Int.get_bit #Int.U64 fromle k) =
    FStar.Math.Lemmas.euclidean_division_definition (v k) 8;
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) vv (mk_u64 lane) (v k);
    lemma_bv_bit_reader #(mk_u64 128) 64 vv lane (v k);
    Canon.lemma_readback Int.U8 (mk_u64 128) (mk_u64 16) vv (mk_u64 (8 * lane + (v k) / 8)) ((v k) % 8);
    lemma_bv_bit_reader #(mk_u64 128) 8 vv (8 * lane + (v k) / 8) ((v k) % 8);
    lemma_e_vld1q_bytes_u64_lane window (8 * lane + (v k) / 8);
    Libcrux_core_models.Trusted.Intrinsics.lemma_u64_from_le_bytes_bit bs (v k);
    assert (Seq.index bs ((v k) / 8) == Seq.index window (8 * lane + (v k) / 8));
    assert (8 * (8 * lane + (v k) / 8) + (v k) % 8 == 64 * lane + v k)
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 y fromle
#pop-options

(* ============================================================================
   `try_into`-array <-> Seq.slice bridge (pure hax proof-lib plumbing, no trust).
   `load_lane_u64` spells its 8 input bytes as
   `impl__unwrap (f_try_into (slice_slice blocks[L] lo (lo+8)))` (Rust
   `.try_into().unwrap()`), whereas `lemma_get_lane_u64x2_vld1q_bytes_le` produces
   the from_le_bytes arg as `Seq.slice window …`.  Both are the same 8 bytes; this
   lemma reduces the try_into-array to the Seq.slice so the two from_le_bytes args
   match.  `Core_models.Convert.f_try_into` on a len-8 slice = Ok(array_from_fn 8
   (slice_index sub)); impl__unwrap picks the array; array_from_fn's ensures gives
   per-index equality with `Seq.slice s lo (lo+8)`. ─────────────────────────── *)
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
  (* f_try_into on a len-8 slice reduces (via the TryFrom<[T;N]> for &[T] model,
     Core_models.Convert.impl_2/impl_3) to Ok (array_from_fn 8 (slice_index sub));
     impl__unwrap picks out that array.  Spell the reduction out so the ensures'
     `impl__unwrap (f_try_into sub)` connects to `arr` cold (no hint). *)
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

