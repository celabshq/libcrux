(*
 * Bitvec.Sha3FallbackProof — discharge proofs for the four ARM64 NEON
 * SHA3-extension *fallback* implementations in
 * `crates/utils/intrinsics/src/arm64_extract.rs`.
 *
 * These four intrinsics have REAL Rust bodies (compositions of the basic
 * NEON ops `_veorq_u64` / `_vbicq_u64` / `_vshlq_n_u64` / `_vshrq_n_u64`),
 * not `unimplemented!()` model stubs.  Each carries an `#[hax_lib::ensures]`
 * functional spec.  But the only F* artifact CI builds for the intrinsics
 * crate is the `.fsti` *interface* — so the fallbacks' specs are currently
 * trusted as axioms; nobody proves the body discharges the spec.
 *
 * This module closes that gap.  For each fallback it states the body
 * (mirrored verbatim from the hax-extracted `Libcrux_intrinsics.Arm64_extract.fst`)
 * as a `Pure` function whose `ensures` is the fallback's spec, and lets F*
 * verify the body against it.  The proof rests ONLY on:
 *   - the basic NEON ops' per-lane specs (`e_veorq_u64`, `e_vbicq_u64`,
 *     `e_vshlq_n_u64`, `e_vshrq_n_u64` from `Arm64_extract.fsti`), and
 *   - the bit-width rotate identity `Bitvec.U64Rotate.lemma_u64_rotate_left_decomp`
 *     (one auditable assume; see that module's header) for the two
 *     rotate-bearing fallbacks.
 * It does NOT depend on any of the `unimplemented!()` model ops (e.g.
 * `e_vdupq_n_s16`) that prevent `Arm64_extract.fst` from verifying as a
 * whole — which is why these proofs are isolated here.
 *
 *   _veor3q_u64 a b c   == (a ^ b) ^ c                    (triple XOR)
 *   _vbcaxq_u64 a b c   == a ^ (b & ~c)                   (XOR-and-bit-clear)
 *   _vrax1q_u64 a b     == a ^ rotate_left(b, 1)          (XOR-and-rotate-left-1)
 *   _vxarq_u64<L,R> a b == rotate_left(a ^ b, L)          (XOR-and-rotate)
 *)
module Bitvec.Sha3FallbackProof

open Core_models
open Bitvec.U64Rotate
open Libcrux_intrinsics.Arm64_extract

(* These are leaf per-lane equalities over the basic NEON op specs; give a
   modest, deterministic rlimit so CI does not depend on Z3's split/retry
   heuristics catching the proof under the default rlimit 5. *)
#set-options "--z3rlimit 50 --fuel 2 --ifuel 2"

(* ----------------------------------------------------------------------- *)
(* _veor3q_u64 : triple XOR, `(a ^ b) ^ c`.
   Body mirrors `e_veor3q_u64 a b c = e_veorq_u64 (e_veorq_u64 a b) c`. *)
let veor3q_u64_fallback_body (a b c: t_e_uint64x2_t)
  : Pure t_e_uint64x2_t
    (requires True)
    (ensures fun result ->
      forall (i: nat{i < 2}).
        get_lane_u64x2 result i ==
        ((get_lane_u64x2 a i ^. get_lane_u64x2 b i) ^. get_lane_u64x2 c i))
=
  e_veorq_u64 (e_veorq_u64 a b) c

(* ----------------------------------------------------------------------- *)
(* _vbcaxq_u64 : XOR-and-bit-clear, `a ^ (b & ~c)`.
   Body mirrors `e_vbcaxq_u64 a b c = e_veorq_u64 a (e_vbicq_u64 b c)`. *)
let vbcaxq_u64_fallback_body (a b c: t_e_uint64x2_t)
  : Pure t_e_uint64x2_t
    (requires True)
    (ensures fun result ->
      forall (i: nat{i < 2}).
        get_lane_u64x2 result i ==
        (get_lane_u64x2 a i ^. (get_lane_u64x2 b i &. (~.(get_lane_u64x2 c i)))))
=
  e_veorq_u64 a (e_vbicq_u64 b c)

(* ----------------------------------------------------------------------- *)
(* _vrax1q_u64 : XOR-and-rotate-left-by-1, `a ^ rotate_left(b, 1)`.
   Body mirrors
     `e_vrax1q_u64 a b = e_veorq_u64 a (e_veorq_u64 (e_vshlq_n_u64 1 b)
                                                    (e_vshrq_n_u64 63 b))`.
   The rotate is bridged by `lemma_u64_rotate_left_decomp` (SMTPat):
     rotate_left x 1 == (x <<! 1) ^. (x >>! 63). *)
let vrax1q_u64_fallback_body (a b: t_e_uint64x2_t)
  : Pure t_e_uint64x2_t
    (requires True)
    (ensures fun result ->
      forall (i: nat{i < 2}).
        get_lane_u64x2 result i ==
        (get_lane_u64x2 a i ^.
          Core_models.Num.impl_u64__rotate_left (get_lane_u64x2 b i) (mk_u32 1)))
=
  e_veorq_u64 a
    (e_veorq_u64 (e_vshlq_n_u64 (mk_i32 1) b) (e_vshrq_n_u64 (mk_i32 63) b))

(* ----------------------------------------------------------------------- *)
(* _vxarq_u64<LEFT,RIGHT> : XOR-and-rotate, `rotate_left(a ^ b, LEFT)`
   when LEFT + RIGHT == 64.  Body mirrors
     `e_vxarq_u64 LEFT RIGHT a b =
        let x = e_veorq_u64 a b in
        e_veorq_u64 (e_vshlq_n_u64 LEFT x) (e_vshrq_n_u64 RIGHT x)`.
   (Re-proved here so all four fallbacks live in one auditable module;
   the standalone `Bitvec.VxarqProof` proves the same statement.) *)
let vxarq_u64_fallback_body (left right: i32) (a b: t_e_uint64x2_t)
  : Pure t_e_uint64x2_t
    (requires
      mk_i32 0 <. left  /\ left  <. mk_i32 64 /\
      mk_i32 0 <. right /\ right <. mk_i32 64 /\
      Rust_primitives.Integers.v left + Rust_primitives.Integers.v right == 64)
    (ensures fun result ->
      forall (i: nat{i < 2}).
        get_lane_u64x2 result i ==
        Core_models.Num.impl_u64__rotate_left
          (get_lane_u64x2 a i ^. get_lane_u64x2 b i)
          (cast left <: u32))
=
  let a_xor_b: t_e_uint64x2_t = e_veorq_u64 a b in
  e_veorq_u64 (e_vshlq_n_u64 left a_xor_b) (e_vshrq_n_u64 right a_xor_b)
