module Libcrux_sha3.Simd.Avx2.Store
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Avx2.Wrappers in
  let open Libcrux_sha3.Traits in
  ()

#push-options "--z3rlimit 400 --split_queries always"

/// Per-iteration store wrapper for `store_block_full_avx2`. Given the
/// four state vectors `s0..s3` (= `s[4*i + 0..4*i + 3]` after the
/// composer\'s linearisation), the four permute2x128 + two
/// unpacklo/unpackhi pass deinterleaves them into four output streams
/// `v_m`, each whose lane `k` corresponds to lane `m` of `s_k`. Four
/// `mm256_storeu_si256_u8` stores then write a 32-byte window per
/// buffer.
/// Factored out of `store_block_full_avx2` so the strong per-byte
/// ensures isolates the `update_at_range`/permute/unpack reasoning from
/// the outer loop\'s heavy invariant. Mirrors `store_u64x2x2` on the
/// AVX2 side (4 lanes instead of 2).
let store_u64x4x4
      (out0 out1 out2 out3: t_Slice u8)
      (s0 s1 s2 s3: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (start i: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize) &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          ((Rust_primitives.Hax.Int.from_machine (mk_i32 32) <: Hax_lib.Int.t_Int) *
            ((Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int) +
              (Rust_primitives.Hax.Int.from_machine (mk_i32 1) <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          b2t
          ((Core_models.Slice.impl__len #u8 out0_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out0 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out1_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out1 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out2_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out2 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out3_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out3 <: usize)
            <:
            bool) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if j <. (start +! (mk_usize 32 *! i <: usize) <: usize) <: bool
                  then
                    ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                    ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool) &&
                    ((out2.[ j ] <: u8) =. (out2_future.[ j ] <: u8) <: bool) &&
                    ((out3.[ j ] <: u8) =. (out3_future.[ j ] <: u8) <: bool)
                  else
                    if
                      j <. (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize)
                      <:
                      bool
                    then
                      if
                        ((j -! start <: usize) /! mk_usize 8 <: usize) =. (mk_usize 4 *! i <: usize)
                        <:
                        bool
                      then
                        ((out0_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                    s0
                                    (mk_usize 0)
                                  <:
                                  u64)
                              <:
                              t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                              <:
                              usize ]
                            <:
                            u8)
                          <:
                          bool) &&
                        ((out1_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                    s0
                                    (mk_usize 1)
                                  <:
                                  u64)
                              <:
                              t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                              <:
                              usize ]
                            <:
                            u8)
                          <:
                          bool) &&
                        ((out2_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                    s0
                                    (mk_usize 2)
                                  <:
                                  u64)
                              <:
                              t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                              <:
                              usize ]
                            <:
                            u8)
                          <:
                          bool) &&
                        ((out3_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                    s0
                                    (mk_usize 3)
                                  <:
                                  u64)
                              <:
                              t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                              <:
                              usize ]
                            <:
                            u8)
                          <:
                          bool)
                      else
                        if
                          ((j -! start <: usize) /! mk_usize 8 <: usize) =.
                          ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
                          <:
                          bool
                        then
                          ((out0_future.[ j ] <: u8) =.
                            ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                      s1
                                      (mk_usize 0)
                                    <:
                                    u64)
                                <:
                                t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                <:
                                usize ]
                              <:
                              u8)
                            <:
                            bool) &&
                          ((out1_future.[ j ] <: u8) =.
                            ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                      s1
                                      (mk_usize 1)
                                    <:
                                    u64)
                                <:
                                t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                <:
                                usize ]
                              <:
                              u8)
                            <:
                            bool) &&
                          ((out2_future.[ j ] <: u8) =.
                            ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                      s1
                                      (mk_usize 2)
                                    <:
                                    u64)
                                <:
                                t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                <:
                                usize ]
                              <:
                              u8)
                            <:
                            bool) &&
                          ((out3_future.[ j ] <: u8) =.
                            ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                      s1
                                      (mk_usize 3)
                                    <:
                                    u64)
                                <:
                                t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                <:
                                usize ]
                              <:
                              u8)
                            <:
                            bool)
                        else
                          if
                            ((j -! start <: usize) /! mk_usize 8 <: usize) =.
                            ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
                            <:
                            bool
                          then
                            ((out0_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s2
                                        (mk_usize 0)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool) &&
                            ((out1_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s2
                                        (mk_usize 1)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool) &&
                            ((out2_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s2
                                        (mk_usize 2)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool) &&
                            ((out3_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s2
                                        (mk_usize 3)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool)
                          else
                            ((out0_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s3
                                        (mk_usize 0)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool) &&
                            ((out1_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s3
                                        (mk_usize 1)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool) &&
                            ((out2_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s3
                                        (mk_usize 2)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool) &&
                            ((out3_future.[ j ] <: u8) =.
                              ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                        s3
                                        (mk_usize 3)
                                      <:
                                      u64)
                                  <:
                                  t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8
                                  <:
                                  usize ]
                                <:
                                u8)
                              <:
                              bool)
                    else
                      ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                      ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool) &&
                      ((out2.[ j ] <: u8) =. (out2_future.[ j ] <: u8) <: bool) &&
                      ((out3.[ j ] <: u8) =. (out3_future.[ j ] <: u8) <: bool)
                else true))) =
  let v0l:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 32) s0 s2
  in
  let v1h:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 32) s1 s3
  in
  let v2l:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 49) s0 s2
  in
  let v3h:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 49) s1 s3
  in
  let v0:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi64 v0l v1h
  in
  let v1:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 v0l v1h
  in
  let v2:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi64 v2l v3h
  in
  let v3:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 v2l v3h
  in
  let old_out0:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let old_out1:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let old_out2:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let old_out3:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out0.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          v0
        <:
        t_Slice u8)
  in
  let out1:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out1
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out1.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          v1
        <:
        t_Slice u8)
  in
  let out2:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out2
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out2.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          v2
        <:
        t_Slice u8)
  in
  let out3:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out3
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out3.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          v3
        <:
        t_Slice u8)
  in
  let _:Prims.unit =
    let a_pos:nat = v start + 32 * v i in
    assert (a_pos + 32 <= Seq.length old_out0);
    assert (a_pos + 32 <= Seq.length old_out1);
    assert (a_pos + 32 <= Seq.length old_out2);
    assert (a_pos + 32 <= Seq.length old_out3);
    let bridge_out0 (j_n: nat{j_n < Seq.length old_out0})
        : Lemma
        (if j_n < a_pos
          then Seq.index out0 j_n == Seq.index old_out0 j_n
          else
            if j_n < a_pos + 32
            then
              Seq.index out0 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        v0
                        (mk_usize ((j_n - a_pos) / 8))))
                ((j_n - a_pos) % 8)
            else Seq.index out0 j_n == Seq.index old_out0 j_n) =
      Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call old_out0
        out0
        v0
        a_pos
        j_n
    in
    let bridge_out1 (j_n: nat{j_n < Seq.length old_out1})
        : Lemma
        (if j_n < a_pos
          then Seq.index out1 j_n == Seq.index old_out1 j_n
          else
            if j_n < a_pos + 32
            then
              Seq.index out1 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        v1
                        (mk_usize ((j_n - a_pos) / 8))))
                ((j_n - a_pos) % 8)
            else Seq.index out1 j_n == Seq.index old_out1 j_n) =
      Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call old_out1
        out1
        v1
        a_pos
        j_n
    in
    let bridge_out2 (j_n: nat{j_n < Seq.length old_out2})
        : Lemma
        (if j_n < a_pos
          then Seq.index out2 j_n == Seq.index old_out2 j_n
          else
            if j_n < a_pos + 32
            then
              Seq.index out2 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        v2
                        (mk_usize ((j_n - a_pos) / 8))))
                ((j_n - a_pos) % 8)
            else Seq.index out2 j_n == Seq.index old_out2 j_n) =
      Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call old_out2
        out2
        v2
        a_pos
        j_n
    in
    let bridge_out3 (j_n: nat{j_n < Seq.length old_out3})
        : Lemma
        (if j_n < a_pos
          then Seq.index out3 j_n == Seq.index old_out3 j_n
          else
            if j_n < a_pos + 32
            then
              Seq.index out3 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        v3
                        (mk_usize ((j_n - a_pos) / 8))))
                ((j_n - a_pos) % 8)
            else Seq.index out3 j_n == Seq.index old_out3 j_n) =
      Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call old_out3
        out3
        v3
        a_pos
        j_n
    in
    Classical.forall_intro bridge_out0;
    Classical.forall_intro bridge_out1;
    Classical.forall_intro bridge_out2;
    Classical.forall_intro bridge_out3
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 400 --split_queries always"

/// Per-iteration store wrapper for the inner `for k in 0..chunks8`
/// loop of `store_block_tail_avx2`. Materializes a 32-byte scratch
/// from a single state vector via `mm256_storeu_si256_u8`, then
/// copies 4 disjoint 8-byte windows into `out0..out3`. Lane `m` of
/// `vec` writes to `out_m[start+8k..start+8(k+1))`.
/// Factored out so its per-byte ensures isolates the local
/// `update_at_range`/`copy_from_slice` reasoning from the surrounding
/// loop\'s heavy invariant. Mirrors `store_u64x4x4` but for the
/// inner 8-byte path.
let store_chunk8x4
      (out0 out1 out2 out3: t_Slice u8)
      (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (start k: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize) &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          ((Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int) *
            ((Rust_primitives.Hax.Int.from_machine k <: Hax_lib.Int.t_Int) +
              (Rust_primitives.Hax.Int.from_machine (mk_i32 1) <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          b2t
          ((Core_models.Slice.impl__len #u8 out0_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out0 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out1_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out1 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out2_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out2 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out3_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out3 <: usize)
            <:
            bool) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if j <. (start +! (mk_usize 8 *! k <: usize) <: usize) <: bool
                  then
                    ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                    ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool) &&
                    ((out2.[ j ] <: u8) =. (out2_future.[ j ] <: u8) <: bool) &&
                    ((out3.[ j ] <: u8) =. (out3_future.[ j ] <: u8) <: bool)
                  else
                    if
                      j <. (start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize)
                      <:
                      bool
                    then
                      ((out0_future.[ j ] <: u8) =.
                        ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                  vec
                                  (mk_usize 0)
                                <:
                                u64)
                            <:
                            t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize
                          ]
                          <:
                          u8)
                        <:
                        bool) &&
                      ((out1_future.[ j ] <: u8) =.
                        ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                  vec
                                  (mk_usize 1)
                                <:
                                u64)
                            <:
                            t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize
                          ]
                          <:
                          u8)
                        <:
                        bool) &&
                      ((out2_future.[ j ] <: u8) =.
                        ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                  vec
                                  (mk_usize 2)
                                <:
                                u64)
                            <:
                            t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize
                          ]
                          <:
                          u8)
                        <:
                        bool) &&
                      ((out3_future.[ j ] <: u8) =.
                        ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                  vec
                                  (mk_usize 3)
                                <:
                                u64)
                            <:
                            t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize
                          ]
                          <:
                          u8)
                        <:
                        bool)
                    else
                      ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                      ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool) &&
                      ((out2.[ j ] <: u8) =. (out2_future.[ j ] <: u8) <: bool) &&
                      ((out3.[ j ] <: u8) =. (out3_future.[ j ] <: u8) <: bool)
                else true))) =
  let u8s:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
  let old_out0:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let old_out1:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let old_out2:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let old_out3:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let u8s:t_Array u8 (mk_usize 32) =
    Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 u8s vec
  in
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out0.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (u8s.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = mk_usize 8
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let out1:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out1
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out1.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (u8s.[ {
                Core_models.Ops.Range.f_start = mk_usize 8;
                Core_models.Ops.Range.f_end = mk_usize 16
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let out2:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out2
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out2.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (u8s.[ {
                Core_models.Ops.Range.f_start = mk_usize 16;
                Core_models.Ops.Range.f_end = mk_usize 24
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let out3:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out3
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out3.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (u8s.[ {
                Core_models.Ops.Range.f_start = mk_usize 24;
                Core_models.Ops.Range.f_end = mk_usize 32
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let _:Prims.unit =
    let a_pos:nat = v start + 8 * v k in
    assert (a_pos + 8 <= Seq.length old_out0);
    assert (a_pos + 8 <= Seq.length old_out1);
    assert (a_pos + 8 <= Seq.length old_out2);
    assert (a_pos + 8 <= Seq.length old_out3);
    Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.mm256_storeu_si256_u8_byte_window (Rust_primitives.Hax.repeat
          (mk_u8 0)
          (mk_usize 32))
      vec;
    let bridge_out_m
          (m_lane: nat{m_lane < 4})
          (out_old out_new: Seq.seq u8)
          (j_n: nat{j_n < Seq.length out_old})
        : Lemma
          (requires
            a_pos + 8 <= Seq.length out_old /\ Seq.length out_new == Seq.length out_old /\
            Seq.slice out_new 0 a_pos == Seq.slice out_old 0 a_pos /\
            Seq.slice out_new a_pos (a_pos + 8) == Seq.slice u8s (m_lane * 8) (m_lane * 8 + 8) /\
            Seq.slice out_new (a_pos + 8) (Seq.length out_new) ==
            Seq.slice out_old (a_pos + 8) (Seq.length out_old))
          (ensures
            (if j_n < a_pos
              then Seq.index out_new j_n == Seq.index out_old j_n
              else
                if j_n < a_pos + 8
                then
                  Seq.index out_new j_n ==
                  Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                            vec
                            (mk_usize m_lane)))
                    (j_n - a_pos)
                else Seq.index out_new j_n == Seq.index out_old j_n)) =
      if j_n < a_pos
      then
        (assert (Seq.index (Seq.slice out_new 0 a_pos) j_n == Seq.index out_new j_n);
          assert (Seq.index (Seq.slice out_old 0 a_pos) j_n == Seq.index out_old j_n))
      else
        if j_n < a_pos + 8
        then
          let t:nat = j_n - a_pos in
          assert (Seq.index (Seq.slice out_new a_pos (a_pos + 8)) t == Seq.index out_new j_n);
          assert (Seq.index (Seq.slice u8s (m_lane * 8) (m_lane * 8 + 8)) t ==
              Seq.index u8s (m_lane * 8 + t));
          assert ((m_lane * 8 + t) / 8 == m_lane);
          assert ((m_lane * 8 + t) % 8 == t)
        else
          let t:nat = j_n - (a_pos + 8) in
          assert (Seq.index (Seq.slice out_new (a_pos + 8) (Seq.length out_new)) t ==
              Seq.index out_new j_n);
          assert (Seq.index (Seq.slice out_old (a_pos + 8) (Seq.length out_old)) t ==
              Seq.index out_old j_n)
    in
    let bridge_call_out0 (j_n: nat{j_n < Seq.length old_out0})
        : Lemma
        (if j_n < a_pos
          then Seq.index out0 j_n == Seq.index old_out0 j_n
          else
            if j_n < a_pos + 8
            then
              Seq.index out0 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 0)))
                (j_n - a_pos)
            else Seq.index out0 j_n == Seq.index old_out0 j_n) =
      bridge_out_m 0 old_out0 out0 j_n
    in
    let bridge_call_out1 (j_n: nat{j_n < Seq.length old_out1})
        : Lemma
        (if j_n < a_pos
          then Seq.index out1 j_n == Seq.index old_out1 j_n
          else
            if j_n < a_pos + 8
            then
              Seq.index out1 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 1)))
                (j_n - a_pos)
            else Seq.index out1 j_n == Seq.index old_out1 j_n) =
      bridge_out_m 1 old_out1 out1 j_n
    in
    let bridge_call_out2 (j_n: nat{j_n < Seq.length old_out2})
        : Lemma
        (if j_n < a_pos
          then Seq.index out2 j_n == Seq.index old_out2 j_n
          else
            if j_n < a_pos + 8
            then
              Seq.index out2 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 2)))
                (j_n - a_pos)
            else Seq.index out2 j_n == Seq.index old_out2 j_n) =
      bridge_out_m 2 old_out2 out2 j_n
    in
    let bridge_call_out3 (j_n: nat{j_n < Seq.length old_out3})
        : Lemma
        (if j_n < a_pos
          then Seq.index out3 j_n == Seq.index old_out3 j_n
          else
            if j_n < a_pos + 8
            then
              Seq.index out3 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 3)))
                (j_n - a_pos)
            else Seq.index out3 j_n == Seq.index old_out3 j_n) =
      bridge_out_m 3 old_out3 out3 j_n
    in
    Classical.forall_intro bridge_call_out0;
    Classical.forall_intro bridge_call_out1;
    Classical.forall_intro bridge_call_out2;
    Classical.forall_intro bridge_call_out3
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 300"

let store_block
      (v_RATE: usize)
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out0 out1 out2 out3: t_Slice u8)
      (start len: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          b2t
          ((Core_models.Slice.impl__len #u8 out0_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out0 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out1_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out1 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out2_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out2 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out3_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out3 <: usize)
            <:
            bool) /\
          (forall (i: usize).
              b2t
              (if i <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if i <. start <: bool
                  then (out0.[ i ] <: u8) =. (out0_future.[ i ] <: u8) <: bool
                  else
                    if i <. (start +! len <: usize) <: bool
                    then
                      (out0_future.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                (s.[ (i -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Avx2_extract.t_Vec256)
                                (mk_usize 0)
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out0.[ i ] <: u8) =. (out0_future.[ i ] <: u8) <: bool
                else true)) /\
          (forall (i: usize).
              b2t
              (if i <. (Core_models.Slice.impl__len #u8 out1 <: usize) <: bool
                then
                  if i <. start <: bool
                  then (out1.[ i ] <: u8) =. (out1_future.[ i ] <: u8) <: bool
                  else
                    if i <. (start +! len <: usize) <: bool
                    then
                      (out1_future.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                (s.[ (i -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Avx2_extract.t_Vec256)
                                (mk_usize 1)
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out1.[ i ] <: u8) =. (out1_future.[ i ] <: u8) <: bool
                else true)) /\
          (forall (i: usize).
              b2t
              (if i <. (Core_models.Slice.impl__len #u8 out2 <: usize) <: bool
                then
                  if i <. start <: bool
                  then (out2.[ i ] <: u8) =. (out2_future.[ i ] <: u8) <: bool
                  else
                    if i <. (start +! len <: usize) <: bool
                    then
                      (out2_future.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                (s.[ (i -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Avx2_extract.t_Vec256)
                                (mk_usize 2)
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out2.[ i ] <: u8) =. (out2_future.[ i ] <: u8) <: bool
                else true)) /\
          (forall (i: usize).
              b2t
              (if i <. (Core_models.Slice.impl__len #u8 out3 <: usize) <: bool
                then
                  if i <. start <: bool
                  then (out3.[ i ] <: u8) =. (out3_future.[ i ] <: u8) <: bool
                  else
                    if i <. (start +! len <: usize) <: bool
                    then
                      (out3_future.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                                (s.[ (i -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Avx2_extract.t_Vec256)
                                (mk_usize 3)
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out3.[ i ] <: u8) =. (out3_future.[ i ] <: u8) <: bool
                else true))) =
  let _:Prims.unit = admit () in
  let chunks:usize = len /! mk_usize 32 in
  let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      chunks
      (fun temp_0_ temp_1_ ->
          let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
            temp_0_
          in
          let _:usize = temp_1_ in
          true)
      (out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
      (fun temp_0_ i ->
          let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
            temp_0_
          in
          let i:usize = i in
          let i0:usize = (mk_usize 4 *! i <: usize) /! mk_usize 5 in
          let j0:usize = (mk_usize 4 *! i <: usize) %! mk_usize 5 in
          let i1:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize) /! mk_usize 5 in
          let j1:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize) %! mk_usize 5 in
          let i2:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize) /! mk_usize 5 in
          let j2:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize) %! mk_usize 5 in
          let i3:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize) /! mk_usize 5 in
          let j3:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize) %! mk_usize 5 in
          let v0l:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 32)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i0
                  j0
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i2
                  j2
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
          in
          let v1h:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 32)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i1
                  j1
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i3
                  j3
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
          in
          let v2l:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 49)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i0
                  j0
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i2
                  j2
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
          in
          let v3h:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 49)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i1
                  j1
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  i3
                  j3
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
          in
          let v0:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi64 v0l v1h
          in
          let v1:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 v0l v1h
          in
          let v2:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi64 v2l v3h
          in
          let v3:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 v2l v3h
          in
          let out0:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
              ({
                  Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out0.[ {
                        Core_models.Ops.Range.f_start
                        =
                        start +! (mk_usize 32 *! i <: usize) <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                  v0
                <:
                t_Slice u8)
          in
          let out1:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out1
              ({
                  Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out1.[ {
                        Core_models.Ops.Range.f_start
                        =
                        start +! (mk_usize 32 *! i <: usize) <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                  v1
                <:
                t_Slice u8)
          in
          let out2:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out2
              ({
                  Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out2.[ {
                        Core_models.Ops.Range.f_start
                        =
                        start +! (mk_usize 32 *! i <: usize) <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                  v2
                <:
                t_Slice u8)
          in
          let out3:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out3
              ({
                  Core_models.Ops.Range.f_start = start +! (mk_usize 32 *! i <: usize) <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 (out3.[ {
                        Core_models.Ops.Range.f_start
                        =
                        start +! (mk_usize 32 *! i <: usize) <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                  v3
                <:
                t_Slice u8)
          in
          out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
  in
  let rem:usize = len %! mk_usize 32 in
  let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
    if rem >. mk_usize 0
    then
      let start:usize = start +! (mk_usize 32 *! chunks <: usize) in
      let u8s:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
      let chunks8:usize = rem /! mk_usize 8 in
      let
      (out0: t_Slice u8),
      (out1: t_Slice u8),
      (out2: t_Slice u8),
      (out3: t_Slice u8),
      (u8s: t_Array u8 (mk_usize 32)) =
        Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
          chunks8
          (fun temp_0_ temp_1_ ->
              let
              (out0: t_Slice u8),
              (out1: t_Slice u8),
              (out2: t_Slice u8),
              (out3: t_Slice u8),
              (u8s: t_Array u8 (mk_usize 32)) =
                temp_0_
              in
              let _:usize = temp_1_ in
              true)
          (out0, out1, out2, out3, u8s
            <:
            (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Array u8 (mk_usize 32)))
          (fun temp_0_ k ->
              let
              (out0: t_Slice u8),
              (out1: t_Slice u8),
              (out2: t_Slice u8),
              (out3: t_Slice u8),
              (u8s: t_Array u8 (mk_usize 32)) =
                temp_0_
              in
              let k:usize = k in
              let i:usize = ((mk_usize 4 *! chunks <: usize) +! k <: usize) /! mk_usize 5 in
              let j:usize = ((mk_usize 4 *! chunks <: usize) +! k <: usize) %! mk_usize 5 in
              let u8s:t_Array u8 (mk_usize 32) =
                Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 u8s
                  (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                      #Libcrux_intrinsics.Avx2_extract.t_Vec256
                      s
                      i
                      j
                    <:
                    Libcrux_intrinsics.Avx2_extract.t_Vec256)
              in
              let out0:t_Slice u8 =
                Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
                  ({
                      Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                      Core_models.Ops.Range.f_end
                      =
                      start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize)
                  (Core_models.Slice.impl__copy_from_slice #u8
                      (out0.[ {
                            Core_models.Ops.Range.f_start
                            =
                            start +! (mk_usize 8 *! k <: usize) <: usize;
                            Core_models.Ops.Range.f_end
                            =
                            start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                      (u8s.[ {
                            Core_models.Ops.Range.f_start = mk_usize 0;
                            Core_models.Ops.Range.f_end = mk_usize 8
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                    <:
                    t_Slice u8)
              in
              let out1:t_Slice u8 =
                Rust_primitives.Hax.Monomorphized_update_at.update_at_range out1
                  ({
                      Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                      Core_models.Ops.Range.f_end
                      =
                      start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize)
                  (Core_models.Slice.impl__copy_from_slice #u8
                      (out1.[ {
                            Core_models.Ops.Range.f_start
                            =
                            start +! (mk_usize 8 *! k <: usize) <: usize;
                            Core_models.Ops.Range.f_end
                            =
                            start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                      (u8s.[ {
                            Core_models.Ops.Range.f_start = mk_usize 8;
                            Core_models.Ops.Range.f_end = mk_usize 16
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                    <:
                    t_Slice u8)
              in
              let out2:t_Slice u8 =
                Rust_primitives.Hax.Monomorphized_update_at.update_at_range out2
                  ({
                      Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                      Core_models.Ops.Range.f_end
                      =
                      start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize)
                  (Core_models.Slice.impl__copy_from_slice #u8
                      (out2.[ {
                            Core_models.Ops.Range.f_start
                            =
                            start +! (mk_usize 8 *! k <: usize) <: usize;
                            Core_models.Ops.Range.f_end
                            =
                            start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                      (u8s.[ {
                            Core_models.Ops.Range.f_start = mk_usize 16;
                            Core_models.Ops.Range.f_end = mk_usize 24
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                    <:
                    t_Slice u8)
              in
              let out3:t_Slice u8 =
                Rust_primitives.Hax.Monomorphized_update_at.update_at_range out3
                  ({
                      Core_models.Ops.Range.f_start = start +! (mk_usize 8 *! k <: usize) <: usize;
                      Core_models.Ops.Range.f_end
                      =
                      start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize)
                  (Core_models.Slice.impl__copy_from_slice #u8
                      (out3.[ {
                            Core_models.Ops.Range.f_start
                            =
                            start +! (mk_usize 8 *! k <: usize) <: usize;
                            Core_models.Ops.Range.f_end
                            =
                            start +! (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize) <: usize
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                      (u8s.[ {
                            Core_models.Ops.Range.f_start = mk_usize 24;
                            Core_models.Ops.Range.f_end = mk_usize 32
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                    <:
                    t_Slice u8)
              in
              out0, out1, out2, out3, u8s
              <:
              (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Array u8 (mk_usize 32)))
      in
      let rem8:usize = rem %! mk_usize 8 in
      if rem8 >. mk_usize 0
      then
        let i:usize = ((mk_usize 4 *! chunks <: usize) +! chunks8 <: usize) /! mk_usize 5 in
        let j:usize = ((mk_usize 4 *! chunks <: usize) +! chunks8 <: usize) %! mk_usize 5 in
        let u8s:t_Array u8 (mk_usize 32) =
          Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_u8 u8s
            (Libcrux_sha3.Traits.get_ij (mk_usize 4) #Libcrux_intrinsics.Avx2_extract.t_Vec256 s i j
              <:
              Libcrux_intrinsics.Avx2_extract.t_Vec256)
        in
        let out0:t_Slice u8 =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
            ({
                Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                Core_models.Ops.Range.f_end = start +! rem <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize)
            (Core_models.Slice.impl__copy_from_slice #u8
                (out0.[ {
                      Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                      Core_models.Ops.Range.f_end = start +! rem <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
                (u8s.[ {
                      Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = rem8
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
              <:
              t_Slice u8)
        in
        let out1:t_Slice u8 =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_range out1
            ({
                Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                Core_models.Ops.Range.f_end = start +! rem <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize)
            (Core_models.Slice.impl__copy_from_slice #u8
                (out1.[ {
                      Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                      Core_models.Ops.Range.f_end = start +! rem <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
                (u8s.[ {
                      Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 8 +! rem8 <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
              <:
              t_Slice u8)
        in
        let out2:t_Slice u8 =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_range out2
            ({
                Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                Core_models.Ops.Range.f_end = start +! rem <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize)
            (Core_models.Slice.impl__copy_from_slice #u8
                (out2.[ {
                      Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                      Core_models.Ops.Range.f_end = start +! rem <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
                (u8s.[ {
                      Core_models.Ops.Range.f_start = mk_usize 16;
                      Core_models.Ops.Range.f_end = mk_usize 16 +! rem8 <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
              <:
              t_Slice u8)
        in
        let out3:t_Slice u8 =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_range out3
            ({
                Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                Core_models.Ops.Range.f_end = start +! rem <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize)
            (Core_models.Slice.impl__copy_from_slice #u8
                (out3.[ {
                      Core_models.Ops.Range.f_start = (start +! rem <: usize) -! rem8 <: usize;
                      Core_models.Ops.Range.f_end = start +! rem <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
                (u8s.[ {
                      Core_models.Ops.Range.f_start = mk_usize 24;
                      Core_models.Ops.Range.f_end = mk_usize 24 +! rem8 <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
              <:
              t_Slice u8)
        in
        out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      else out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
    else out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_Squeeze4
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256)
  Libcrux_intrinsics.Avx2_extract.t_Vec256 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_squeeze4_pre
    =
    (fun
        (v_RATE: usize)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out0: t_Slice u8)
        (out1: t_Slice u8)
        (out2: t_Slice u8)
        (out3: t_Slice u8)
        (start: usize)
        (len: usize)
        ->
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize));
    f_squeeze4_post
    =
    (fun
        (v_RATE: usize)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (out0: t_Slice u8)
        (out1: t_Slice u8)
        (out2: t_Slice u8)
        (out3: t_Slice u8)
        (start: usize)
        (len: usize)
        (out0_future, out1_future, out2_future, out3_future:
          (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
        ->
        (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize));
    f_squeeze4
    =
    fun
      (v_RATE: usize)
      (self:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0: t_Slice u8)
      (out1: t_Slice u8)
      (out2: t_Slice u8)
      (out3: t_Slice u8)
      (start: usize)
      (len: usize)
      ->
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
        store_block v_RATE self.Libcrux_sha3.Generic_keccak.f_st out0 out1 out2 out3 start len
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let out2:t_Slice u8 = tmp2 in
      let out3:t_Slice u8 = tmp3 in
      let _:Prims.unit = () in
      out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
  }
