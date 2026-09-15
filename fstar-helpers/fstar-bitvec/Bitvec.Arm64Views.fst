(*
 * Bitvec.Arm64Views — u64x2 lane view + per-lane op-facts for the BASIC
 * Arm64 NEON ops used by the SHA3 software-fallback proofs, phrased over the
 * REAL core-models-backed `Libcrux_intrinsics.Arm64` (NOT the retired
 * `Arm64_extract` stub).
 *
 * This is the fstar-bitvec-local analogue of the sha3 crate's
 * `Libcrux_intrinsics.Arm64_sha3_views` companion, restricted to exactly the
 * four basic building blocks the fallback compositions decompose into:
 *   e_veorq_u64  (bitwise XOR)      e_vbicq_u64  (bitwise AND-NOT)
 *   e_vshlq_n_u64 (per-lane <<)     e_vshrq_n_u64 (per-lane >>)
 *
 * TRUST.  Zero assumptions.  The u64x2 lane view is a per-index read of the
 * canonical core-models codec (`NV.to_u64x2`), and every op-fact is PROVEN
 * from the canonical NEON op-lemma set in `Libcrux_core_models.Neon_views`
 * (which rests only on the differentially-tested `Arm.Interpretations.Int_vec`
 * lifts + the proven codec round-trip).  Under the retired pcm `Arm64_extract`
 * stub these per-lane facts were trusted op `ensures` (`.fsti` axioms); here
 * the trust surface has SHRUNK to zero.
 *
 * The veorq machinery (`lemma_bv_index128` .. `lemma_e_veorq_u64`) is copied
 * verbatim from the proven `Arm64_sha3_views` companion; `lemma_e_vbicq_u64`
 * mirrors it with the AND-NOT bit semantics (template:
 * `Intrinsics_views.lemma_andnot_u64x4_iv`); the two shift facts are the simple
 * per-lane FunArray form (template: the companion's `lemma_e_vdupq_n_u64`).
 *)
module Bitvec.Arm64Views

open FStar.Mul
open Core_models

include Libcrux_intrinsics.Arm64

module Funarr = Libcrux_core_models.Abstractions.Funarr
module BV     = Libcrux_core_models.Abstractions.Bitvec
module Bit    = Libcrux_core_models.Abstractions.Bit
module Canon  = Libcrux_core_models.Intrinsics_views
module NV     = Libcrux_core_models.Neon_views
module ArmIV  = Libcrux_core_models.Core_arch.Arm.Interpretations.Int_vec
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Num    = Core_models.Num
module Int    = Rust_primitives.Integers

(* ── lane-view type (mirrors the retired pcm `t_e_uint64x2_t`) ─────────────── *)
unfold type t_e_uint64x2_t = BV.t_BitVec (mk_u64 128)

(* ── u64x2 lane view (A-on-B adapter over canonical NV.to_u64x2).  OPAQUE:
      keeps pcm's abstraction; the only route to the codec is the index lemma
      below. ──────────────────────────────────────────────────────────────── *)
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
   width-128 bv-index helpers (copied from the Arm64_sha3_views companion). *)
let lemma_bv_index128 (bv: BV.t_BitVec (mk_u64 128)) (k: u64{v k < 128})
  : Lemma ((bv.[ k ] <: Bit.t_Bit) == Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit bv._0 k) = ()

let lemma_impl9_index128 (f: (i: u64{v i < 128}) -> Bit.t_Bit) (k: u64{v k < 128})
  : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit
             (BV.impl_9__from_fn (mk_u64 128) #(u64 -> Bit.t_Bit) f)._0 k == f k) = ()

(* ============================================================================
   e_veorq_u64 : bitwise XOR at the u64-lane view.  (Copied verbatim from
   Arm64_sha3_views; `ArmIV.veorq_u64 = veorq_s16` is a bit-level from_fn XOR,
   so the u64-lane fact needs the codec to commute with bitwise XOR.)
   ========================================================================== *)

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

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_veorq_raw (a b: t_e_uint64x2_t) (ii: u64{v ii < 2}) (bb: nat{bb < 64})
    : Lemma (IVi.bval (IVi.lane_reader (mk_u64 128) 64 (ArmIV.veorq_u64 a b) ii bb) ==
             Int.bit_xor (IVi.bval (IVi.lane_reader (mk_u64 128) 64 a ii bb))
                         (IVi.bval (IVi.lane_reader (mk_u64 128) 64 b ii bb))) =
  assert (64 * v ii + bb < 128);
  lemma_veorq_funarr a b (mk_u64 (64 * v ii + bb))
#pop-options

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
   e_vbicq_u64 : bitwise AND-NOT at the u64-lane view.  `ArmIV.vbicq_u64` is a
   bit-level from_fn `a AND NOT b`; mirrors the veorq codec-commute with the
   AND-NOT bit algebra (template: Intrinsics_views.lemma_andnot_u64x4_iv).
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_vbicq_funarr (a b: t_e_uint64x2_t) (k: u64{v k < 128})
    : Lemma (Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit (ArmIV.vbicq_u64 a b)._0 k ==
             (match Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit a._0 k,
                    Funarr.impl_5__get (mk_u64 128) #Bit.t_Bit b._0 k
              with
              | Bit.Bit_One, Bit.Bit_Zero -> Bit.Bit_One
              | _ -> Bit.Bit_Zero)) =
  let f : (i: u64{v i < 128}) -> Bit.t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: Bit.t_Bit), (b.[ i ] <: Bit.t_Bit) with
              | Bit.Bit_One, Bit.Bit_Zero -> Bit.Bit_One
              | _ -> Bit.Bit_Zero) in
  assert (ArmIV.vbicq_u64 a b ==
          BV.impl_9__from_fn (mk_u64 128) #(u64 -> Bit.t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%ArmIV.vbicq_u64];
                            iota; zeta; primops];
        FStar.Tactics.trefl ());
  lemma_impl9_index128 f k;
  lemma_bv_index128 a k;
  lemma_bv_index128 b k
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bic_u64x2_iv (a b: t_e_uint64x2_t) (i: nat{i < 2})
    : Lemma (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 (ArmIV.vbicq_u64 a b)) (mk_u64 i) ==
             (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 a) (mk_u64 i) &.
              (~. (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 b) (mk_u64 i))))) =
  let aBICb = ArmIV.vbicq_u64 a b in
  let ya : u64 = Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 a) (mk_u64 i) in
  let yb : u64 = Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 b) (mk_u64 i) in
  let yr : u64 = Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 aBICb) (mk_u64 i) in
  let aux (bb: usize{v bb < 64})
      : Lemma (Int.get_bit #Int.U64 yr bb == Int.get_bit #Int.U64 (ya &. (~. yb)) bb) =
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) aBICb (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) a (mk_u64 i) (v bb);
    Canon.lemma_readback Int.U64 (mk_u64 128) (mk_u64 2) b (mk_u64 i) (v bb);
    lemma_vbicq_funarr a b (mk_u64 (64 * i + v bb));
    Int.get_bit_and #Int.U64 ya (~. yb) bb;
    Int.get_bit_lognot #Int.U64 yb bb
  in
  Classical.forall_intro aux;
  Int.lemma_int_t_eq_via_bits #Int.U64 yr (ya &. (~. yb))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_e_vbicq_u64 (a b: t_e_uint64x2_t)
  : Lemma (vec128_as_u64x2 (e_vbicq_u64 a b)
           == Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i
                                   &. (~. (Seq.index (vec128_as_u64x2 b) i))))
          [SMTPat (vec128_as_u64x2 (e_vbicq_u64 a b))] =
  NV.lemma_vbicq_u64 a b;
  let aux (i: nat{i < 2})
    : Lemma (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 (ArmIV.vbicq_u64 a b)) (mk_u64 i)
             == (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 a) (mk_u64 i)
                 &. (~. (Funarr.impl_5__get (mk_u64 2) #u64 (NV.to_u64x2 b) (mk_u64 i))))) =
    lemma_bic_u64x2_iv a b i
  in
  FStar.Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vbicq_u64 a b))
                     (Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i
                                           &. (~. (Seq.index (vec128_as_u64x2 b) i))))
#pop-options

(* ============================================================================
   Per-lane shifts.  `ArmIV.{vshlq,vshrq}_n_u64` are per-lane FunArray ops, so
   `NV.lemma_v{shl,shr}q_n_u64` + `Seq.lemma_eq_intro` closes (template: the
   companion's `lemma_e_vdupq_n_u64`).  Stated with the `cast v_N <: u32` shift
   the retired stub's ensures used (and the `U64Rotate` bridge expects).
   ========================================================================== *)

#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vshlq_n_u64 (v_N: i32) (a: t_e_uint64x2_t)
  : Lemma (requires v (mk_i32 0) <= v v_N /\ v v_N < 64)
          (ensures
            vec128_as_u64x2 (e_vshlq_n_u64 v_N a)
            == Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i <<! (cast v_N <: u32)))
          [SMTPat (vec128_as_u64x2 (e_vshlq_n_u64 v_N a))] =
  NV.lemma_vshlq_n_u64 v_N a;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vshlq_n_u64 v_N a))
                     (Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i <<! (cast v_N <: u32)))
#pop-options

#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lemma_e_vshrq_n_u64 (v_N: i32) (a: t_e_uint64x2_t)
  : Lemma (requires v (mk_i32 0) < v v_N /\ v v_N < 64)
          (ensures
            vec128_as_u64x2 (e_vshrq_n_u64 v_N a)
            == Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i >>! (cast v_N <: u32)))
          [SMTPat (vec128_as_u64x2 (e_vshrq_n_u64 v_N a))] =
  NV.lemma_vshrq_n_u64 v_N a;
  Seq.lemma_eq_intro (vec128_as_u64x2 (e_vshrq_n_u64 v_N a))
                     (Seq.init 2 (fun i -> Seq.index (vec128_as_u64x2 a) i >>! (cast v_N <: u32)))
#pop-options

(* ============================================================================
   Per-lane `get_lane_u64x2`-form corollaries (SMTPat on the lane read that the
   fallback-proof goals actually mention).  Each unpacks the `Seq.init` op-fact
   above via `Seq.init_index`, so a consumer's `get_lane_u64x2 (e_op ...) i`
   rewrites directly to the per-lane RHS — no `Seq.index (Seq.init ...)` burden
   in the consumer's VC (keeps the fallback bodies fast-stable).
   ========================================================================== *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_e_veorq_u64_lane (a b: t_e_uint64x2_t) (i: nat{i < 2})
  : Lemma (get_lane_u64x2 (e_veorq_u64 a b) i
           == (get_lane_u64x2 a i ^. get_lane_u64x2 b i))
          [SMTPat (get_lane_u64x2 (e_veorq_u64 a b) i)] =
  lemma_e_veorq_u64 a b

let lemma_e_vbicq_u64_lane (a b: t_e_uint64x2_t) (i: nat{i < 2})
  : Lemma (get_lane_u64x2 (e_vbicq_u64 a b) i
           == (get_lane_u64x2 a i &. (~. (get_lane_u64x2 b i))))
          [SMTPat (get_lane_u64x2 (e_vbicq_u64 a b) i)] =
  lemma_e_vbicq_u64 a b

let lemma_e_vshlq_n_u64_lane (v_N: i32) (a: t_e_uint64x2_t) (i: nat{i < 2})
  : Lemma (requires v (mk_i32 0) <= v v_N /\ v v_N < 64)
          (ensures get_lane_u64x2 (e_vshlq_n_u64 v_N a) i
                   == (get_lane_u64x2 a i <<! (cast v_N <: u32)))
          [SMTPat (get_lane_u64x2 (e_vshlq_n_u64 v_N a) i)] =
  lemma_e_vshlq_n_u64 v_N a

let lemma_e_vshrq_n_u64_lane (v_N: i32) (a: t_e_uint64x2_t) (i: nat{i < 2})
  : Lemma (requires v (mk_i32 0) < v v_N /\ v v_N < 64)
          (ensures get_lane_u64x2 (e_vshrq_n_u64 v_N a) i
                   == (get_lane_u64x2 a i >>! (cast v_N <: u32)))
          [SMTPat (get_lane_u64x2 (e_vshrq_n_u64 v_N a) i)] =
  lemma_e_vshrq_n_u64 v_N a
#pop-options
