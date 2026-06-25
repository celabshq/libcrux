module Libcrux_sha3.Simd.Avx2.Wrappers
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_intrinsics.Avx2_extract in
  ()

let rotate_left (v_LEFT v_RIGHT: i32) (x: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Prims.Pure Libcrux_intrinsics.Avx2_extract.t_Vec256
      (requires
        mk_i32 0 <=. v_LEFT && v_LEFT <=. mk_i32 64 && mk_i32 0 <=. v_RIGHT && v_RIGHT <=. mk_i32 64
      )
      (fun _ -> Prims.l_True) =
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 (Libcrux_intrinsics.Avx2_extract.mm256_slli_epi64 v_LEFT
        x
      <:
      Libcrux_intrinsics.Avx2_extract.t_Vec256)
    (Libcrux_intrinsics.Avx2_extract.mm256_srli_epi64 v_RIGHT x
      <:
      Libcrux_intrinsics.Avx2_extract.t_Vec256)

let e_veor5q_u64 (a b c d e: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Libcrux_intrinsics.Avx2_extract.t_Vec256 =
  let ab:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 a b
  in
  let abc:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 ab c
  in
  let abcd:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 abc d
  in
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 abcd e

let e_vrax1q_u64 (a b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Libcrux_intrinsics.Avx2_extract.t_Vec256 =
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 a
    (rotate_left (mk_i32 1) (mk_i32 63) b <: Libcrux_intrinsics.Avx2_extract.t_Vec256)

let e_vxarq_u64 (v_LEFT v_RIGHT: i32) (a b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Prims.Pure Libcrux_intrinsics.Avx2_extract.t_Vec256
      (requires
        mk_i32 0 <=. v_LEFT && v_LEFT <=. mk_i32 64 && mk_i32 0 <=. v_RIGHT && v_RIGHT <=. mk_i32 64
      )
      (fun _ -> Prims.l_True) =
  let ab:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 a b
  in
  rotate_left v_LEFT v_RIGHT ab

let e_vbcaxq_u64 (a b c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Libcrux_intrinsics.Avx2_extract.t_Vec256 =
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 a
    (Libcrux_intrinsics.Avx2_extract.mm256_andnot_si256 c b
      <:
      Libcrux_intrinsics.Avx2_extract.t_Vec256)

let e_veorq_n_u64 (a: Libcrux_intrinsics.Avx2_extract.t_Vec256) (c: u64)
    : Libcrux_intrinsics.Avx2_extract.t_Vec256 =
  let c:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_set1_epi64x (cast (c <: u64) <: i64)
  in
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 a c

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_KeccakItem Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 4) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_zero_pre = (fun (_: Prims.unit) -> true);
    f_zero_post = (fun (_: Prims.unit) (out: Libcrux_intrinsics.Avx2_extract.t_Vec256) -> true);
    f_zero = (fun (_: Prims.unit) -> Libcrux_intrinsics.Avx2_extract.mm256_set1_epi64x (mk_i64 0));
    f_xor5_pre
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (d: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (e: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_xor5_post
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (d: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (e: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_xor5
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (d: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (e: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        e_veor5q_u64 a b c d e);
    f_rotate_left1_and_xor_pre
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_rotate_left1_and_xor_post
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_rotate_left1_and_xor
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        e_vrax1q_u64 a b);
    f_xor_and_rotate_pre
    =
    (fun
        (v_LEFT: i32)
        (v_RIGHT: i32)
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) &&
        v_RIGHT >. mk_i32 0 &&
        v_RIGHT <. mk_i32 64);
    f_xor_and_rotate_post
    =
    (fun
        (v_LEFT: i32)
        (v_RIGHT: i32)
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_xor_and_rotate
    =
    (fun
        (v_LEFT: i32)
        (v_RIGHT: i32)
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        e_vxarq_u64 v_LEFT v_RIGHT a b);
    f_and_not_xor_pre
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_and_not_xor_post
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_and_not_xor
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        e_vbcaxq_u64 a b c);
    f_xor_constant_pre = (fun (a: Libcrux_intrinsics.Avx2_extract.t_Vec256) (c: u64) -> true);
    f_xor_constant_post
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (c: u64)
        (out: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_xor_constant
    =
    (fun (a: Libcrux_intrinsics.Avx2_extract.t_Vec256) (c: u64) -> e_veorq_n_u64 a c);
    f_xor_pre
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_xor_post
    =
    (fun
        (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out: Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_xor
    =
    fun
      (a: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (b: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      ->
      Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 a b
  }
