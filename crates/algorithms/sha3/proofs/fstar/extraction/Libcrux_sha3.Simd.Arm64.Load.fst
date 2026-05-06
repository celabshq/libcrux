module Libcrux_sha3.Simd.Arm64.Load
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Arm64.Wrappers in
  let open Libcrux_sha3.Traits in
  ()

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

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_Absorb
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
