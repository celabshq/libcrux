module Libcrux_sha3.Avx2.X4.Incremental
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Avx2 in
  let open Libcrux_sha3.Traits in
  ()

/// The Keccak state for the incremental API.
noeq type t_KeccakState = {
  f_state:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256
}

/// Initialise the [`KeccakState`].
let init (_: Prims.unit) : t_KeccakState =
  {
    f_state
    =
    Libcrux_sha3.Generic_keccak.impl_2__new (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      ()
  }
  <:
  t_KeccakState

/// Absorb
let shake128_absorb_final (s: t_KeccakState) (data0 data1 data2 data3: t_Slice u8)
    : Prims.Pure t_KeccakState
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data0 <: usize)
          <:
          Hax_lib.Int.t_Int) <
        (168 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data1 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data2 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data3 <: usize))
      (fun _ -> Prims.l_True) =
  let s:t_KeccakState =
    {
      s with
      f_state
      =
      Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 4)
        #Libcrux_intrinsics.Avx2_extract.t_Vec256
        (mk_usize 168)
        (mk_u8 31)
        s.f_state
        (let list = [data0; data1; data2; data3] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
          Rust_primitives.Hax.array_of_list 4 list)
        (mk_usize 0)
        (Core_models.Slice.impl__len #u8 data0 <: usize)
    }
    <:
    t_KeccakState
  in
  s

/// Absorb
let shake256_absorb_final (s: t_KeccakState) (data0 data1 data2 data3: t_Slice u8)
    : Prims.Pure t_KeccakState
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data0 <: usize)
          <:
          Hax_lib.Int.t_Int) <
        (136 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data1 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data2 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data3 <: usize))
      (fun _ -> Prims.l_True) =
  let s:t_KeccakState =
    {
      s with
      f_state
      =
      Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 4)
        #Libcrux_intrinsics.Avx2_extract.t_Vec256
        (mk_usize 136)
        (mk_u8 31)
        s.f_state
        (let list = [data0; data1; data2; data3] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
          Rust_primitives.Hax.array_of_list 4 list)
        (mk_usize 0)
        (Core_models.Slice.impl__len #u8 data0 <: usize)
    }
    <:
    t_KeccakState
  in
  s

/// Squeeze block
let shake256_squeeze_first_block (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (136 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Simd256.impl__squeeze_first_block (mk_usize 136)
      s.f_state
      out0
      out1
      out2
      out3
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  s, out0, out1, out2, out3 <: (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

/// Squeeze next block
let shake256_squeeze_next_block (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (136 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let
  (tmp0:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256),
  (tmp1: t_Slice u8),
  (tmp2: t_Slice u8),
  (tmp3: t_Slice u8),
  (tmp4: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Simd256.impl__squeeze_next_block (mk_usize 136)
      s.f_state
      out0
      out1
      out2
      out3
      (mk_usize 0)
  in
  let s:t_KeccakState = { s with f_state = tmp0 } <: t_KeccakState in
  let out0:t_Slice u8 = tmp1 in
  let out1:t_Slice u8 = tmp2 in
  let out2:t_Slice u8 = tmp3 in
  let out3:t_Slice u8 = tmp4 in
  let _:Prims.unit = () in
  s, out0, out1, out2, out3 <: (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

/// Squeeze three blocks
let shake128_squeeze_first_three_blocks (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (504 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let
  (tmp0:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256),
  (tmp1: t_Slice u8),
  (tmp2: t_Slice u8),
  (tmp3: t_Slice u8),
  (tmp4: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Simd256.impl__squeeze_first_three_blocks (mk_usize 168)
      s.f_state
      out0
      out1
      out2
      out3
  in
  let s:t_KeccakState = { s with f_state = tmp0 } <: t_KeccakState in
  let out0:t_Slice u8 = tmp1 in
  let out1:t_Slice u8 = tmp2 in
  let out2:t_Slice u8 = tmp3 in
  let out3:t_Slice u8 = tmp4 in
  let _:Prims.unit = () in
  s, out0, out1, out2, out3 <: (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

/// Squeeze five blocks
let shake128_squeeze_first_five_blocks (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (840 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let
  (tmp0:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256),
  (tmp1: t_Slice u8),
  (tmp2: t_Slice u8),
  (tmp3: t_Slice u8),
  (tmp4: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Simd256.impl__squeeze_first_five_blocks (mk_usize 168)
      s.f_state
      out0
      out1
      out2
      out3
  in
  let s:t_KeccakState = { s with f_state = tmp0 } <: t_KeccakState in
  let out0:t_Slice u8 = tmp1 in
  let out1:t_Slice u8 = tmp2 in
  let out2:t_Slice u8 = tmp3 in
  let out3:t_Slice u8 = tmp4 in
  let _:Prims.unit = () in
  s, out0, out1, out2, out3 <: (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

/// Squeeze another block
let shake128_squeeze_next_block (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (168 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let
  (tmp0:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256),
  (tmp1: t_Slice u8),
  (tmp2: t_Slice u8),
  (tmp3: t_Slice u8),
  (tmp4: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Simd256.impl__squeeze_next_block (mk_usize 168)
      s.f_state
      out0
      out1
      out2
      out3
      (mk_usize 0)
  in
  let s:t_KeccakState = { s with f_state = tmp0 } <: t_KeccakState in
  let out0:t_Slice u8 = tmp1 in
  let out1:t_Slice u8 = tmp2 in
  let out2:t_Slice u8 = tmp3 in
  let out3:t_Slice u8 = tmp4 in
  let _:Prims.unit = () in
  s, out0, out1, out2, out3 <: (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
