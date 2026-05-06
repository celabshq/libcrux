module Libcrux_sha3.Simd.Arm64
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_intrinsics.Arm64_extract in
  let open Libcrux_sha3.Traits in
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

let load_lane_u64
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset i: usize)
      (statei: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (lane: usize)
    : Prims.Pure u64
      (requires
        i <. mk_usize 25 && lane <. mk_usize 2 &&
        (((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (blocks.[ lane ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) =
  (Libcrux_intrinsics.Arm64_extract.get_lane_u64 statei lane <: u64) ^.
  (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
              (mk_usize 8))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 8))
              #FStar.Tactics.Typeclasses.solve
              ((blocks.[ lane ] <: t_Slice u8).[ {
                    Core_models.Ops.Range.f_start = offset +! (mk_usize 8 *! i <: usize) <: usize;
                    Core_models.Ops.Range.f_end
                    =
                    (offset +! (mk_usize 8 *! i <: usize) <: usize) +! mk_usize 8 <: usize
                  }
                  <:
                  Core_models.Ops.Range.t_Range usize ]
                <:
                t_Slice u8)
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 8))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 8))
    <:
    u64)

let lemma_rate_mod (rate: usize)
    : Prims.Pure Prims.unit
      (requires Libcrux_sha3.Proof_utils.valid_rate rate)
      (ensures
        fun temp_0_ ->
          let _:Prims.unit = temp_0_ in
          if (rate %! mk_usize 16 <: usize) >. mk_usize 0
          then
            (rate /! mk_usize 8 <: usize) =.
            ((mk_usize 2 *! (rate /! mk_usize 16 <: usize) <: usize) +! mk_usize 1 <: usize)
          else
            (rate /! mk_usize 8 <: usize) =. (mk_usize 2 *! (rate /! mk_usize 16 <: usize) <: usize)
      ) = ()

let load_u64x2
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset i: usize)
      (statei: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    : Prims.Pure Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      (requires
        i <. mk_usize 25 &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (blocks.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun result ->
          let result:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t = result in
          (Libcrux_intrinsics.Arm64_extract.get_lane_u64 result (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks offset i statei (mk_usize 0) <: u64) &&
          (Libcrux_intrinsics.Arm64_extract.get_lane_u64 result (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks offset i statei (mk_usize 1) <: u64)) =
  let u:t_Array u64 (mk_usize 2) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 2) in
  let u:t_Array u64 (mk_usize 2) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize u
      (mk_usize 0)
      (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
                  (mk_usize 8))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8)
                  #(t_Array u8 (mk_usize 8))
                  #FStar.Tactics.Typeclasses.solve
                  ((blocks.[ mk_usize 0 ] <: t_Slice u8).[ {
                        Core_models.Ops.Range.f_start
                        =
                        offset +! (mk_usize 8 *! i <: usize) <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        (offset +! (mk_usize 8 *! i <: usize) <: usize) +! mk_usize 8 <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                <:
                Core_models.Result.t_Result (t_Array u8 (mk_usize 8))
                  Core_models.Array.t_TryFromSliceError)
            <:
            t_Array u8 (mk_usize 8))
        <:
        u64)
  in
  let u:t_Array u64 (mk_usize 2) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize u
      (mk_usize 1)
      (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
                  (mk_usize 8))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8)
                  #(t_Array u8 (mk_usize 8))
                  #FStar.Tactics.Typeclasses.solve
                  ((blocks.[ mk_usize 1 ] <: t_Slice u8).[ {
                        Core_models.Ops.Range.f_start
                        =
                        offset +! (mk_usize 8 *! i <: usize) <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        (offset +! (mk_usize 8 *! i <: usize) <: usize) +! mk_usize 8 <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                <:
                Core_models.Result.t_Result (t_Array u8 (mk_usize 8))
                  Core_models.Array.t_TryFromSliceError)
            <:
            t_Array u8 (mk_usize 8))
        <:
        u64)
  in
  let uvec:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vld1q_u64 (u <: t_Slice u64)
  in
  Libcrux_intrinsics.Arm64_extract.e_veorq_u64 statei uvec

let load_u64x2x2
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset i: usize)
      (in0 in1: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    : Prims.Pure
      (Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
        Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (requires
        i <. mk_usize 12 &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (blocks.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let
          (r0: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
          (r1: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) =
            temp_0_
          in
          (Libcrux_intrinsics.Arm64_extract.get_lane_u64 r0 (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks offset (mk_usize 2 *! i <: usize) in0 (mk_usize 0) <: u64) &&
          (Libcrux_intrinsics.Arm64_extract.get_lane_u64 r0 (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks offset (mk_usize 2 *! i <: usize) in0 (mk_usize 1) <: u64) &&
          (Libcrux_intrinsics.Arm64_extract.get_lane_u64 r1 (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize)
              in1
              (mk_usize 0)
            <:
            u64) &&
          (Libcrux_intrinsics.Arm64_extract.get_lane_u64 r1 (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize)
              in1
              (mk_usize 1)
            <:
            u64)) =
  let v0:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vld1q_bytes_u64 ((blocks.[ mk_usize 0 ] <: t_Slice u8).[ {
            Core_models.Ops.Range.f_start = offset +! (mk_usize 16 *! i <: usize) <: usize;
            Core_models.Ops.Range.f_end
            =
            (offset +! (mk_usize 16 *! i <: usize) <: usize) +! mk_usize 16 <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
  in
  let v1:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vld1q_bytes_u64 ((blocks.[ mk_usize 1 ] <: t_Slice u8).[ {
            Core_models.Ops.Range.f_start = offset +! (mk_usize 16 *! i <: usize) <: usize;
            Core_models.Ops.Range.f_end
            =
            (offset +! (mk_usize 16 *! i <: usize) <: usize) +! mk_usize 16 <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
  in
  Libcrux_intrinsics.Arm64_extract.e_veorq_u64 in0
    (Libcrux_intrinsics.Arm64_extract.e_vtrn1q_u64 v0 v1
      <:
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
  Libcrux_intrinsics.Arm64_extract.e_veorq_u64 in1
    (Libcrux_intrinsics.Arm64_extract.e_vtrn2q_u64 v0 v1
      <:
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
  <:
  (Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t & Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
  )

#push-options "--z3rlimit 400 --split_queries always"

/// Per-iteration store wrapper for the `store_block` loop body.
/// Given the two state slots `s_2i` (state[2*i]) and `s_succ`
/// (state[2*i + 1]), and the output slices `out0`/`out1`, performs the
/// per-iteration `vtrn1q_u64`/`vtrn2q_u64` interleave + two
/// `_vst1q_bytes_u64` stores and returns updated slices that satisfy
/// the byte-level loop invariant for the freshly-stored 16-byte
/// window.
/// Factored out of `store_block` so its strong per-byte ensures
/// isolates the `update_at_range`/slice precondition cliff from the
/// outer loop\'s heavy invariant. Mirrors `load_u64x2x2` on the load
/// side.
let store_u64x2x2
      (out0 out1: t_Slice u8)
      (s_2i s_succ: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (start i: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          ((Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int) *
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
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
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
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if j <. (start +! (mk_usize 16 *! i <: usize) <: usize) <: bool
                  then
                    ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                    ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool)
                  else
                    if
                      j <. (start +! (mk_usize 16 *! (i +! mk_usize 1 <: usize) <: usize) <: usize)
                      <:
                      bool
                    then
                      if
                        ((j -! start <: usize) /! mk_usize 8 <: usize) =. (mk_usize 2 *! i <: usize)
                        <:
                        bool
                      then
                        ((out0_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_2i
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
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_2i
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
                          bool)
                      else
                        ((out0_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_succ
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
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_succ
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
                          bool)
                    else
                      ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                      ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool)
                else true))) =
  let v0:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vtrn1q_u64 s_2i s_succ
  in
  let v1:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vtrn2q_u64 s_2i s_succ
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
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! i <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 16 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 (out0.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! i <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 16 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
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
          Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! i <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          start +! (mk_usize 16 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 (out1.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! i <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                start +! (mk_usize 16 *! (i +! mk_usize 1 <: usize) <: usize) <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          v1
        <:
        t_Slice u8)
  in
  let _:Prims.unit =
    let a_pos:nat = v start + 16 * v i in
    assert (a_pos + 16 <= Seq.length old_out0);
    assert (a_pos + 16 <= Seq.length old_out1);
    let bridge_out0 (j_n: nat{j_n < Seq.length old_out0})
        : Lemma
        (if j_n < a_pos
          then Seq.index out0 j_n == Seq.index old_out0 j_n
          else
            if j_n < a_pos + 16
            then
              Seq.index out0 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2
                        v0
                        ((j_n - a_pos) / 8)))
                ((j_n - a_pos) % 8)
            else Seq.index out0 j_n == Seq.index old_out0 j_n) =
      Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.store_block_window_byte_of_vst old_out0
        out0
        (Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 (Seq.slice old_out0 a_pos (a_pos + 16))
            v0)
        v0
        a_pos
        j_n
    in
    let bridge_out1 (j_n: nat{j_n < Seq.length old_out1})
        : Lemma
        (if j_n < a_pos
          then Seq.index out1 j_n == Seq.index old_out1 j_n
          else
            if j_n < a_pos + 16
            then
              Seq.index out1 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2
                        v1
                        ((j_n - a_pos) / 8)))
                ((j_n - a_pos) % 8)
            else Seq.index out1 j_n == Seq.index old_out1 j_n) =
      Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.store_block_window_byte_of_vst old_out1
        out1
        (Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 (Seq.slice old_out1 a_pos (a_pos + 16))
            v1)
        v1
        a_pos
        j_n
    in
    Classical.forall_intro bridge_out0;
    Classical.forall_intro bridge_out1
  in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 400 --split_queries always"

/// Tail wrapper for the `remaining > 8` branch of `store_block`.
/// Stores the partial 16-byte window `out0[start+16*q .. start+16*q+remaining]`
/// (and the analogous out1 window) by:
/// (1) materializing both 16-byte tmp arrays via `_vst1q_bytes_u64`,
/// (2) `copy_from_slice`-ing the first `remaining` bytes into the
///     `out0`/`out1` windows.
/// `q = len/16` (the post-loop iteration count). The window covers the
/// last `remaining` bytes of `[start, start+len)` with `8 < remaining
/// < 16`. The window\'s first 8 bytes correspond to `s_2i`; bytes 8..remaining
/// correspond to `s_succ`. Lanes 0/1 of each go to out0/out1.
/// Mirrors `store_u64x2x2` on the partial-window side: the strong
/// per-byte ensures isolates the local update_at_range slice precond
/// + `_vst1q_bytes_u64`/`vtrn` reasoning so the calling `store_block`
/// body composes additively.
let store_tail_high
      (out0 out1: t_Slice u8)
      (s_2i s_succ: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (start q remaining: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        remaining >. mk_usize 8 &&
        remaining <. mk_usize 16 &&
        (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine q <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine remaining <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
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
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if j <. (start +! (mk_usize 16 *! q <: usize) <: usize) <: bool
                  then
                    ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                    ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool)
                  else
                    if
                      j <. ((start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize)
                      <:
                      bool
                    then
                      if
                        ((j -! start <: usize) /! mk_usize 8 <: usize) =. (mk_usize 2 *! q <: usize)
                        <:
                        bool
                      then
                        ((out0_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_2i
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
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_2i
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
                          bool)
                      else
                        ((out0_future.[ j ] <: u8) =.
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_succ
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
                          ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                    s_succ
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
                          bool)
                    else
                      ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                      ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool)
                else true))) =
  let v0:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vtrn1q_u64 s_2i s_succ
  in
  let v1:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_intrinsics.Arm64_extract.e_vtrn2q_u64 s_2i s_succ
  in
  let out0_tmp:t_Array u8 (mk_usize 16) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16) in
  let out1_tmp:t_Array u8 (mk_usize 16) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16) in
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
  let out0_tmp:t_Array u8 (mk_usize 16) =
    Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 out0_tmp v0
  in
  let out1_tmp:t_Array u8 (mk_usize 16) =
    Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 out1_tmp v1
  in
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out0.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (out0_tmp.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = remaining
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
          Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out1.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (out1_tmp.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = remaining
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let _:Prims.unit =
    let a_pos:nat = v start + 16 * v q in
    let r:nat = v remaining in
    assert (a_pos + r <= Seq.length old_out0);
    assert (a_pos + r <= Seq.length old_out1);
    let bridge_out0 (j_n: nat{j_n < Seq.length old_out0})
        : Lemma
        (if j_n < a_pos
          then Seq.index out0 j_n == Seq.index old_out0 j_n
          else
            if j_n < a_pos + r
            then
              (let k:nat = j_n - a_pos in
                Seq.index out0 j_n ==
                Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2
                          v0
                          (k / 8)))
                  (k % 8))
            else Seq.index out0 j_n == Seq.index old_out0 j_n) =
      if j_n < a_pos
      then
        (assert (Seq.index (Seq.slice out0 0 a_pos) j_n == Seq.index out0 j_n);
          assert (Seq.index (Seq.slice old_out0 0 a_pos) j_n == Seq.index old_out0 j_n))
      else
        if j_n < a_pos + r
        then
          let k:nat = j_n - a_pos in
          assert (k < r);
          assert (Seq.index (Seq.slice out0 a_pos (a_pos + r)) k == Seq.index out0 j_n);
          assert (Seq.index (Seq.slice out0_tmp 0 r) k == Seq.index out0_tmp k)
        else
          let k:nat = j_n - (a_pos + r) in
          assert (Seq.index (Seq.slice out0 (a_pos + r) (Seq.length out0)) k == Seq.index out0 j_n);
          assert (Seq.index (Seq.slice old_out0 (a_pos + r) (Seq.length old_out0)) k ==
              Seq.index old_out0 j_n)
    in
    let bridge_out1 (j_n: nat{j_n < Seq.length old_out1})
        : Lemma
        (if j_n < a_pos
          then Seq.index out1 j_n == Seq.index old_out1 j_n
          else
            if j_n < a_pos + r
            then
              (let k:nat = j_n - a_pos in
                Seq.index out1 j_n ==
                Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2
                          v1
                          (k / 8)))
                  (k % 8))
            else Seq.index out1 j_n == Seq.index old_out1 j_n) =
      if j_n < a_pos
      then
        (assert (Seq.index (Seq.slice out1 0 a_pos) j_n == Seq.index out1 j_n);
          assert (Seq.index (Seq.slice old_out1 0 a_pos) j_n == Seq.index old_out1 j_n))
      else
        if j_n < a_pos + r
        then
          let k:nat = j_n - a_pos in
          assert (k < r);
          assert (Seq.index (Seq.slice out1 a_pos (a_pos + r)) k == Seq.index out1 j_n);
          assert (Seq.index (Seq.slice out1_tmp 0 r) k == Seq.index out1_tmp k)
        else
          let k:nat = j_n - (a_pos + r) in
          assert (Seq.index (Seq.slice out1 (a_pos + r) (Seq.length out1)) k == Seq.index out1 j_n);
          assert (Seq.index (Seq.slice old_out1 (a_pos + r) (Seq.length old_out1)) k ==
              Seq.index old_out1 j_n)
    in
    Classical.forall_intro bridge_out0;
    Classical.forall_intro bridge_out1
  in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 400 --split_queries always"

/// Tail wrapper for the `remaining > 0 && remaining <= 8` branch of
/// `store_block`. A single 16-byte tmp materialized from one state
/// slot — its low half (`tmp[0..remaining]`) goes to `out0`, its high
/// half (`tmp[8..8+remaining]`) goes to `out1`.
/// `q = len/16`. Window: `[start+16*q, start+16*q+remaining)`, with
/// `0 < remaining <= 8`. Both `out0[j]` and `out1[j]` map to lanes 0
/// and 1 of the same state slot `s_2q`; the lo-half / hi-half split
/// is exactly `_vst1q_bytes_u64`\'s definition.
let store_tail_low
      (out0 out1: t_Slice u8)
      (s_2q: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (start q remaining: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        remaining >. mk_usize 0 &&
        remaining <=. mk_usize 8 &&
        (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine q <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine remaining <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
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
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if j <. (start +! (mk_usize 16 *! q <: usize) <: usize) <: bool
                  then
                    ((out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool) &&
                    ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool)
                  else
                    if
                      j <. ((start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize)
                      <:
                      bool
                    then
                      ((out0_future.[ j ] <: u8) =.
                        ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                  s_2q
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
                        ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                  s_2q
                                  (mk_usize 1)
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
                      ((out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool)
                else true))) =
  let out01:t_Array u8 (mk_usize 16) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16) in
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
  let out01:t_Array u8 (mk_usize 16) =
    Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64 out01 s_2q
  in
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({
          Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out0.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (out01.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = remaining
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
          Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
          Core_models.Ops.Range.f_end
          =
          (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out1.[ {
                Core_models.Ops.Range.f_start = start +! (mk_usize 16 *! q <: usize) <: usize;
                Core_models.Ops.Range.f_end
                =
                (start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (out01.[ {
                Core_models.Ops.Range.f_start = mk_usize 8;
                Core_models.Ops.Range.f_end = mk_usize 8 +! remaining <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let _:Prims.unit =
    let a_pos:nat = v start + 16 * v q in
    let r:nat = v remaining in
    assert (r <= 8);
    let bridge_out0 (j_n: nat{j_n < Seq.length old_out0})
        : Lemma
        (if j_n < a_pos
          then Seq.index out0 j_n == Seq.index old_out0 j_n
          else
            if j_n < a_pos + r
            then
              (let k:nat = j_n - a_pos in
                Seq.index out0 j_n ==
                Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2
                          s_2q
                          0))
                  k)
            else Seq.index out0 j_n == Seq.index old_out0 j_n) =
      if j_n < a_pos
      then
        (assert (Seq.index (Seq.slice out0 0 a_pos) j_n == Seq.index out0 j_n);
          assert (Seq.index (Seq.slice old_out0 0 a_pos) j_n == Seq.index old_out0 j_n))
      else
        if j_n < a_pos + r
        then
          let k:nat = j_n - a_pos in
          assert (k < r /\ k < 8);
          assert (Seq.index (Seq.slice out0 a_pos (a_pos + r)) k == Seq.index out0 j_n);
          assert (Seq.index (Seq.slice out01 0 r) k == Seq.index out01 k);
          assert (k / 8 == 0 /\ k % 8 == k)
        else
          let k:nat = j_n - (a_pos + r) in
          assert (Seq.index (Seq.slice out0 (a_pos + r) (Seq.length out0)) k == Seq.index out0 j_n);
          assert (Seq.index (Seq.slice old_out0 (a_pos + r) (Seq.length old_out0)) k ==
              Seq.index old_out0 j_n)
    in
    let bridge_out1 (j_n: nat{j_n < Seq.length old_out1})
        : Lemma
        (if j_n < a_pos
          then Seq.index out1 j_n == Seq.index old_out1 j_n
          else
            if j_n < a_pos + r
            then
              (let k:nat = j_n - a_pos in
                Seq.index out1 j_n ==
                Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2
                          s_2q
                          1))
                  k)
            else Seq.index out1 j_n == Seq.index old_out1 j_n) =
      if j_n < a_pos
      then
        (assert (Seq.index (Seq.slice out1 0 a_pos) j_n == Seq.index out1 j_n);
          assert (Seq.index (Seq.slice old_out1 0 a_pos) j_n == Seq.index old_out1 j_n))
      else
        if j_n < a_pos + r
        then
          let k:nat = j_n - a_pos in
          assert (k < r /\ k < 8);
          assert (Seq.index (Seq.slice out1 a_pos (a_pos + r)) k == Seq.index out1 j_n);
          assert (Seq.index (Seq.slice out01 8 (8 + r)) k == Seq.index out01 (8 + k));
          assert ((8 + k) / 8 == 1 /\ (8 + k) % 8 == k)
        else
          let k:nat = j_n - (a_pos + r) in
          assert (Seq.index (Seq.slice out1 (a_pos + r) (Seq.length out1)) k == Seq.index out1 j_n);
          assert (Seq.index (Seq.slice old_out1 (a_pos + r) (Seq.length old_out1)) k ==
              Seq.index old_out1 j_n)
    in
    Classical.forall_intro bridge_out0;
    Classical.forall_intro bridge_out1
  in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

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

#push-options "--z3rlimit 800 --split_queries always"

let load_block
      (v_RATE: usize)
      (state: t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset: usize)
    : Prims.Pure (t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        ((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (blocks.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun state_future ->
          let state_future:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
            state_future
          in
          forall (i: usize).
            b2t
            (if i <. mk_usize 25 <: bool
              then
                if i <. (v_RATE /! mk_usize 8 <: usize) <: bool
                then
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        i
                        (state.[ i ] <: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        i
                        (state.[ i ] <: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool)
                else
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state.[ i ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state.[ i ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool)
              else true)) =
  let old_state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) = state in
  let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (v_RATE /! mk_usize 16 <: usize)
      (fun state i ->
          let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) = state in
          let i:usize = i in
          forall (j: usize).
            b2t
            (if j <. mk_usize 25 <: bool
              then
                if j <. (mk_usize 2 *! i <: usize) <: bool
                then
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        j
                        (old_state.[ j ] <: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        j
                        (old_state.[ j ] <: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool)
                else
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Arm64_extract.get_lane_u64 (old_state.[ j ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Arm64_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Arm64_extract.get_lane_u64 (old_state.[ j ]
                          <:
                          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool)
              else true))
      state
      (fun state i ->
          let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) = state in
          let i:usize = i in
          let i0:usize = (mk_usize 2 *! i <: usize) /! mk_usize 5 in
          let j0:usize = (mk_usize 2 *! i <: usize) %! mk_usize 5 in
          let i1:usize = ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize) /! mk_usize 5 in
          let j1:usize = ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize) %! mk_usize 5 in
          let
          (v0: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
          (v1: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) =
            load_u64x2x2 blocks
              offset
              i
              (Libcrux_sha3.Traits.get_ij (mk_usize 2)
                  #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
                  state
                  i0
                  j0
                <:
                Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
              (Libcrux_sha3.Traits.get_ij (mk_usize 2)
                  #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
                  state
                  i1
                  j1
                <:
                Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
          in
          let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              state
              i0
              j0
              v0
          in
          let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              state
              i1
              j1
              v1
          in
          state)
  in
  let _:Prims.unit = lemma_rate_mod v_RATE in
  let remaining:usize = v_RATE %! mk_usize 16 in
  let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
    if remaining >. mk_usize 0
    then
      let i:usize = (v_RATE /! mk_usize 8 <: usize) -! mk_usize 1 in
      let result:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
        load_u64x2 blocks
          offset
          i
          (Libcrux_sha3.Traits.get_ij (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              state
              (i /! mk_usize 5 <: usize)
              (i %! mk_usize 5 <: usize)
            <:
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      in
      let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
        Libcrux_sha3.Traits.set_ij (mk_usize 2)
          #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
          state
          (i /! mk_usize 5 <: usize)
          (i %! mk_usize 5 <: usize)
          result
      in
      state
    else state
  in
  state

#pop-options

let load_last
      (v_RATE: usize)
      (v_DELIMITER: u8)
      (state: t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset len: usize)
    : Prims.Pure (t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (blocks.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert (((offset +! len <: usize) <=.
              (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
              (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize)
              <:
              bool))
      in
      ()
  in
  let buffer0:t_Array u8 v_RATE = Rust_primitives.Hax.repeat (mk_u8 0) v_RATE in
  let buffer0:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range buffer0
      ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = len }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (buffer0.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = len
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          ((blocks.[ mk_usize 0 ] <: t_Slice u8).[ {
                Core_models.Ops.Range.f_start = offset;
                Core_models.Ops.Range.f_end = offset +! len <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let buffer0:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer0 len v_DELIMITER
  in
  let buffer0:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer0
      (v_RATE -! mk_usize 1 <: usize)
      ((buffer0.[ v_RATE -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  let buffer1:t_Array u8 v_RATE = Rust_primitives.Hax.repeat (mk_u8 0) v_RATE in
  let buffer1:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range buffer1
      ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = len }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (buffer1.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = len
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          ((blocks.[ mk_usize 1 ] <: t_Slice u8).[ {
                Core_models.Ops.Range.f_start = offset;
                Core_models.Ops.Range.f_end = offset +! len <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let buffer1:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer1 len v_DELIMITER
  in
  let buffer1:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer1
      (v_RATE -! mk_usize 1 <: usize)
      ((buffer1.[ v_RATE -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  let state:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
    load_block v_RATE
      state
      (let list = [buffer0 <: t_Slice u8; buffer1 <: t_Slice u8] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      (mk_usize 0)
  in
  state

#push-options "--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'"

let store_block
      (v_RATE: usize)
      (s: t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (out0 out1: t_Slice u8)
      (start len: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
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
        (Core_models.Slice.impl__len #u8 out1 <: usize))
      (ensures
        fun temp_0_ ->
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
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
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                (s.[ (i -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
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
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                (s.[ (i -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
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
                else true))) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((len <=. v_RATE <: bool) &&
            ((start +! len <: usize) <=. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool) &&
            ((Core_models.Slice.impl__len #u8 out0 <: usize) =.
              (Core_models.Slice.impl__len #u8 out1 <: usize)
              <:
              bool))
      in
      ()
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
  let _:Prims.unit =
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
    assert (old_out0 == out0);
    assert (old_out1 == out1)
  in
  let (out0: t_Slice u8), (out1: t_Slice u8) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (len /! mk_usize 16 <: usize)
      (fun temp_0_ i ->
          let (out0: t_Slice u8), (out1: t_Slice u8) = temp_0_ in
          let i:usize = i in
          b2t
          ((Core_models.Slice.impl__len #u8 out0 <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out0 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out1 <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out1 <: usize)
            <:
            bool) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out0 <: usize) <: bool
                then
                  if j <. start <: bool
                  then (out0.[ j ] <: u8) =. (old_out0.[ j ] <: u8) <: bool
                  else
                    if j <. (start +! (i *! mk_usize 16 <: usize) <: usize) <: bool
                    then
                      (out0.[ j ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                (s.[ (j -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                                (mk_usize 0)
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out0.[ j ] <: u8) =. (old_out0.[ j ] <: u8) <: bool
                else true)) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out1 <: usize) <: bool
                then
                  if j <. start <: bool
                  then (out1.[ j ] <: u8) =. (old_out1.[ j ] <: u8) <: bool
                  else
                    if j <. (start +! (i *! mk_usize 16 <: usize) <: usize) <: bool
                    then
                      (out1.[ j ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Arm64_extract.get_lane_u64
                                (s.[ (j -! start <: usize) /! mk_usize 8 <: usize ]
                                  <:
                                  Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
                                (mk_usize 1)
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out1.[ j ] <: u8) =. (old_out1.[ j ] <: u8) <: bool
                else true)))
      (out0, out1 <: (t_Slice u8 & t_Slice u8))
      (fun temp_0_ i ->
          let (out0: t_Slice u8), (out1: t_Slice u8) = temp_0_ in
          let i:usize = i in
          let i0:usize = (mk_usize 2 *! i <: usize) /! mk_usize 5 in
          let j0:usize = (mk_usize 2 *! i <: usize) %! mk_usize 5 in
          let i1:usize = ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize) /! mk_usize 5 in
          let j1:usize = ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize) %! mk_usize 5 in
          let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
            store_u64x2x2 out0
              out1
              (Libcrux_sha3.Traits.get_ij (mk_usize 2)
                  #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
                  s
                  i0
                  j0
                <:
                Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
              (Libcrux_sha3.Traits.get_ij (mk_usize 2)
                  #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
                  s
                  i1
                  j1
                <:
                Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
              start
              i
          in
          let out0:t_Slice u8 = tmp0 in
          let out1:t_Slice u8 = tmp1 in
          let _:Prims.unit = () in
          out0, out1 <: (t_Slice u8 & t_Slice u8))
  in
  let q:usize = len /! mk_usize 16 in
  let remaining:usize = len %! mk_usize 16 in
  let _:Prims.unit =
    assert (v len == 16 * v q + v remaining);
    assert (v remaining < 16);
    assert (v start + 16 * v q + v remaining == v start + v len);
    assert (v start + v len <= Seq.length out0);
    assert (v start + v len <= Seq.length out1)
  in
  let (out0: t_Slice u8), (out1: t_Slice u8) =
    if remaining >. mk_usize 8
    then
      let i:usize = mk_usize 2 *! q in
      let i0:usize = i /! mk_usize 5 in
      let j0:usize = i %! mk_usize 5 in
      let i1:usize = (i +! mk_usize 1 <: usize) /! mk_usize 5 in
      let j1:usize = (i +! mk_usize 1 <: usize) %! mk_usize 5 in
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
        store_tail_high out0
          out1
          (Libcrux_sha3.Traits.get_ij (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              s
              i0
              j0
            <:
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
          (Libcrux_sha3.Traits.get_ij (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              s
              i1
              j1
            <:
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
          start
          q
          remaining
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      out0, out1 <: (t_Slice u8 & t_Slice u8)
    else
      if remaining >. mk_usize 0
      then
        let i:usize = mk_usize 2 *! q in
        let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
          store_tail_low out0
            out1
            (Libcrux_sha3.Traits.get_ij (mk_usize 2)
                #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
                s
                (i /! mk_usize 5 <: usize)
                (i %! mk_usize 5 <: usize)
              <:
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
            start
            q
            remaining
        in
        let out0:t_Slice u8 = tmp0 in
        let out1:t_Slice u8 = tmp1 in
        let _:Prims.unit = () in
        out0, out1 <: (t_Slice u8 & t_Slice u8)
      else out0, out1 <: (t_Slice u8 & t_Slice u8)
  in
  let _:Prims.unit = admit () in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: Libcrux_sha3.Traits.t_Absorb
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) (mk_usize 2) =
  {
    f_load_block_pre
    =
    (fun
        (v_RATE: usize)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (input: t_Array (t_Slice u8) (mk_usize 2))
        (start: usize)
        ->
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (input.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 1 ] <: t_Slice u8) <: usize));
    f_load_block_post
    =
    (fun
        (v_RATE: usize)
        (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (input: t_Array (t_Slice u8) (mk_usize 2))
        (start: usize)
        (out:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_load_block
    =
    (fun
        (v_RATE: usize)
        (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (input: t_Array (t_Slice u8) (mk_usize 2))
        (start: usize)
        ->
        let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
          {
            self with
            Libcrux_sha3.Generic_keccak.f_st
            =
            load_block v_RATE self.Libcrux_sha3.Generic_keccak.f_st input start
          }
          <:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
        in
        self);
    f_load_last_pre
    =
    (fun
        (v_RATE: usize)
        (v_DELIMITER: u8)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (input: t_Array (t_Slice u8) (mk_usize 2))
        (start: usize)
        (len: usize)
        ->
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                (input.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize)
          <:
          Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 1 ] <: t_Slice u8) <: usize));
    f_load_last_post
    =
    (fun
        (v_RATE: usize)
        (v_DELIMITER: u8)
        (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (input: t_Array (t_Slice u8) (mk_usize 2))
        (start: usize)
        (len: usize)
        (out:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        ->
        true);
    f_load_last
    =
    fun
      (v_RATE: usize)
      (v_DELIMITER: u8)
      (self:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (input: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      ->
      let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
        Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
        {
          self with
          Libcrux_sha3.Generic_keccak.f_st
          =
          load_last v_RATE v_DELIMITER self.Libcrux_sha3.Generic_keccak.f_st input start len
        }
        <:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      in
      self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2: Libcrux_sha3.Traits.t_Squeeze2
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
  Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_squeeze2_pre
    =
    (fun
        (v_RATE: usize)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out0: t_Slice u8)
        (out1: t_Slice u8)
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
        (Core_models.Slice.impl__len #u8 out1 <: usize));
    f_squeeze2_post
    =
    (fun
        (v_RATE: usize)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
        (out0: t_Slice u8)
        (out1: t_Slice u8)
        (start: usize)
        (len: usize)
        (out0_future, out1_future: (t_Slice u8 & t_Slice u8))
        ->
        (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize));
    f_squeeze2
    =
    fun
      (v_RATE: usize)
      (self:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0: t_Slice u8)
      (out1: t_Slice u8)
      (start: usize)
      (len: usize)
      ->
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
        store_block v_RATE self.Libcrux_sha3.Generic_keccak.f_st out0 out1 start len
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      out0, out1 <: (t_Slice u8 & t_Slice u8)
  }
