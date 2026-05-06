module Libcrux_sha3.Simd.Arm64.Wrappers
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_intrinsics.Arm64_extract in
  ()

let e_veor5q_u64 (a b c d e: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    : Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
  Libcrux_intrinsics.Arm64_extract.e_veor3q_u64 (Libcrux_intrinsics.Arm64_extract.e_veor3q_u64 a b c
      <:
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    d
    e

let e_vrax1q_u64 (a b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    : Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
  Libcrux_intrinsics.Arm64_extract.e_vrax1q_u64 a b

let e_vxarq_u64 (v_LEFT v_RIGHT: i32) (a b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    : Prims.Pure Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      (requires
        mk_i32 0 <. v_LEFT && v_LEFT <. mk_i32 64 && mk_i32 0 <. v_RIGHT && v_RIGHT <. mk_i32 64 &&
        (v_LEFT +! v_RIGHT <: i32) =. mk_i32 64)
      (fun _ -> Prims.l_True) = Libcrux_intrinsics.Arm64_extract.e_vxarq_u64 v_LEFT v_RIGHT a b

let e_vbcaxq_u64 (a b c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    : Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
  Libcrux_intrinsics.Arm64_extract.e_vbcaxq_u64 a b c

let e_veorq_n_u64 (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) (c: u64)
    : Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
  let c:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vdupq_n_u64 c
  in
  Libcrux_intrinsics.Arm64_extract.e_veorq_u64 a c

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_KeccakItem Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
  (mk_usize 2) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_zero_pre = (fun (_: Prims.unit) -> true);
    f_zero_post
    =
    (fun (_: Prims.unit) (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) -> true);
    f_zero = (fun (_: Prims.unit) -> Libcrux_intrinsics.Arm64_extract.e_vdupq_n_u64 (mk_u64 0));
    f_xor5_pre
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (d: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (e: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_xor5_post
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (d: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (e: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_xor5
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (d: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (e: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        e_veor5q_u64 a b c d e);
    f_rotate_left1_and_xor_pre
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_rotate_left1_and_xor_post
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_rotate_left1_and_xor
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        e_vrax1q_u64 a b);
    f_xor_and_rotate_pre
    =
    (fun
        (v_LEFT: i32)
        (v_RIGHT: i32)
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        mk_i32 0 <. v_LEFT && v_LEFT <. mk_i32 64 && mk_i32 0 <. v_RIGHT && v_RIGHT <. mk_i32 64 &&
        (v_LEFT +! v_RIGHT <: i32) =. mk_i32 64);
    f_xor_and_rotate_post
    =
    (fun
        (v_LEFT: i32)
        (v_RIGHT: i32)
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_xor_and_rotate
    =
    (fun
        (v_LEFT: i32)
        (v_RIGHT: i32)
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        e_vxarq_u64 v_LEFT v_RIGHT a b);
    f_and_not_xor_pre
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_and_not_xor_post
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_and_not_xor
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        e_vbcaxq_u64 a b c);
    f_xor_constant_pre = (fun (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) (c: u64) -> true);
    f_xor_constant_post
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (c: u64)
        (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_xor_constant
    =
    (fun (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) (c: u64) -> e_veorq_n_u64 a c);
    f_xor_pre
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_xor_post
    =
    (fun
        (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_xor
    =
    fun
      (a: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (b: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      ->
      Libcrux_intrinsics.Arm64_extract.e_veorq_u64 a b
  }
