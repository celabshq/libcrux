module Libcrux_sha3.Avx2.X4
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// Perform 4 SHAKE256 operations in parallel
let shake256 (input0 input1 input2 input3 out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize) &&
        (Core_models.Slice.impl__len #u8 input0 <: usize) =.
        (Core_models.Slice.impl__len #u8 input1 <: usize) &&
        (Core_models.Slice.impl__len #u8 input0 <: usize) =.
        (Core_models.Slice.impl__len #u8 input2 <: usize) &&
        (Core_models.Slice.impl__len #u8 input0 <: usize) =.
        (Core_models.Slice.impl__len #u8 input3 <: usize))
      (ensures
        fun temp_0_ ->
          let
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
    Libcrux_sha3.Generic_keccak.Simd256.keccak4 (mk_usize 136)
      (mk_u8 31)
      (let list = [input0; input1; input2; input3] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
        Rust_primitives.Hax.array_of_list 4 list)
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
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
