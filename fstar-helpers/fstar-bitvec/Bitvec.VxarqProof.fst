(*
 * Bitvec.VxarqProof — discharge proof for the manual `_vxarq_u64` fallback.
 *
 * `Libcrux_intrinsics.Arm64.e_vxarq_u64`'s post-condition is
 *
 *   forall (i: nat{i < 2}).
 *     get_lane_u64x2 result i ==
 *       Core_models.Num.impl_u64__rotate_left
 *         (get_lane_u64x2 a i ^. get_lane_u64x2 b i) (cast LEFT)
 *
 * and the manual fallback body extracts to a composition of `_veorq_u64`,
 * `_vshlq_n_u64`, and `_vshrq_n_u64`, each of which has a per-lane
 * spec.  This file mirrors the body verbatim and proves the spec match.
 *
 * The proof has one non-trivial step — bridging
 *   `(x <<! sh) ^. (x >>! (64-sh))`  ==  `rotate_left x sh`
 * — which is delegated to `Bitvec.U64Rotate.lemma_u64_rotate_left_decomp`.
 *)
module Bitvec.VxarqProof

open Core_models
open Bitvec.U64Rotate
open Bitvec.Arm64Views

(* Mirror of the manual fallback body, expressed at the bit-vector level
   the F*-extracted intrinsics use.

   Proof: the `Bitvec.Arm64Views` per-lane op-facts (SMTPat) rewrite each
   `get_lane_u64x2 (e_v* ...) i` to its per-lane form, giving
   `(x <<! cast left) ^. (x >>! cast right)` with `x = a_i ^. b_i`; the
   `U64Rotate` decomposition (SMTPat) rewrites the goal's `rotate_left x (cast
   left)` to `(x <<! cast left) ^. (x >>! (64 -! cast left))`; the two meet once
   we bridge `cast right == 64 -! cast left` (from `v left + v right == 64`). *)
#push-options "--z3rlimit 50 --fuel 2 --ifuel 2"
let vxarq_u64_fallback_body
  (left right: i32)
  (a b: t_e_uint64x2_t)
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
  let a_xor_b: t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64.e_veorq_u64 a b
  in
  let result: t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64.e_veorq_u64
      (Libcrux_intrinsics.Arm64.e_vshlq_n_u64 left  a_xor_b)
      (Libcrux_intrinsics.Arm64.e_vshrq_n_u64 right a_xor_b)
  in
  (* i32 -> u32 cast preserves value in-range, and 64 - v left == v right, so the
     shift-right amount matches the rotate decomposition's `64 -! cast left`. *)
  assert (Rust_primitives.Integers.v (cast left  <: u32) == Rust_primitives.Integers.v left);
  assert (Rust_primitives.Integers.v (cast right <: u32) == Rust_primitives.Integers.v right);
  assert ((cast right <: u32) == (mk_u32 64 -! (cast left <: u32)));
  assert (in_range64 (cast left <: u32));
  result
#pop-options
