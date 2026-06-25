module Libcrux_sha3.Simd.Portable
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Traits in
  ()

let rotate_left (v_LEFT v_RIGHT: i32) (x: u64)
    : Prims.Pure u64
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) &&
        v_RIGHT >. mk_i32 0)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit = Hax_lib.v_assert ((v_LEFT +! v_RIGHT <: i32) =. mk_i32 64 <: bool) in
      ()
  in
  Core_models.Num.impl_u64__rotate_left x (cast (v_LEFT <: i32) <: u32)

let e_veor5q_u64 (a b c d e: u64) : u64 = (((a ^. b <: u64) ^. c <: u64) ^. d <: u64) ^. e

let e_vrax1q_u64 (a b: u64) : u64 = a ^. (rotate_left (mk_i32 1) (mk_i32 63) b <: u64)

let e_vxarq_u64 (v_LEFT v_RIGHT: i32) (a b: u64)
    : Prims.Pure u64
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) &&
        v_RIGHT >. mk_i32 0)
      (fun _ -> Prims.l_True) = rotate_left v_LEFT v_RIGHT (a ^. b <: u64)

let e_vbcaxq_u64 (a b c: u64) : u64 = a ^. (b &. (~.c <: u64) <: u64)

let e_veorq_n_u64 (a c: u64) : u64 = a ^. c

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_KeccakItem u64 (mk_usize 1) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_zero_pre = (fun (_: Prims.unit) -> true);
    f_zero_post = (fun (_: Prims.unit) (out: u64) -> true);
    f_zero = (fun (_: Prims.unit) -> mk_u64 0);
    f_xor5_pre = (fun (a: u64) (b: u64) (c: u64) (d: u64) (e: u64) -> true);
    f_xor5_post = (fun (a: u64) (b: u64) (c: u64) (d: u64) (e: u64) (out: u64) -> true);
    f_xor5 = (fun (a: u64) (b: u64) (c: u64) (d: u64) (e: u64) -> e_veor5q_u64 a b c d e);
    f_rotate_left1_and_xor_pre = (fun (a: u64) (b: u64) -> true);
    f_rotate_left1_and_xor_post = (fun (a: u64) (b: u64) (out: u64) -> true);
    f_rotate_left1_and_xor = (fun (a: u64) (b: u64) -> e_vrax1q_u64 a b);
    f_xor_and_rotate_pre
    =
    (fun (v_LEFT: i32) (v_RIGHT: i32) (a: u64) (b: u64) ->
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) &&
        v_RIGHT >. mk_i32 0);
    f_xor_and_rotate_post = (fun (v_LEFT: i32) (v_RIGHT: i32) (a: u64) (b: u64) (out: u64) -> true);
    f_xor_and_rotate
    =
    (fun (v_LEFT: i32) (v_RIGHT: i32) (a: u64) (b: u64) -> e_vxarq_u64 v_LEFT v_RIGHT a b);
    f_and_not_xor_pre = (fun (a: u64) (b: u64) (c: u64) -> true);
    f_and_not_xor_post = (fun (a: u64) (b: u64) (c: u64) (out: u64) -> true);
    f_and_not_xor = (fun (a: u64) (b: u64) (c: u64) -> e_vbcaxq_u64 a b c);
    f_xor_constant_pre = (fun (a: u64) (c: u64) -> true);
    f_xor_constant_post = (fun (a: u64) (c: u64) (out: u64) -> true);
    f_xor_constant = (fun (a: u64) (c: u64) -> e_veorq_n_u64 a c);
    f_xor_pre = (fun (a: u64) (b: u64) -> true);
    f_xor_post = (fun (a: u64) (b: u64) (out: u64) -> true);
    f_xor = fun (a: u64) (b: u64) -> a ^. b
  }

#push-options "--z3rlimit 300"

let load_block
      (v_RATE: usize)
      (state: t_Array u64 (mk_usize 25))
      (blocks: t_Slice u8)
      (start: usize)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 blocks <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun state_future ->
          let state_future:t_Array u64 (mk_usize 25) = state_future in
          forall (i: usize).
            b2t
            (if i <. mk_usize 25 <: bool
              then
                if i <. (v_RATE /! mk_usize 8 <: usize) <: bool
                then
                  (state_future.[ i ] <: u64) =.
                  ((state.[ i ] <: u64) ^.
                    (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array
                                u8 (mk_usize 8))
                            #Core_models.Array.t_TryFromSliceError
                            (Core_models.Convert.f_try_into #(t_Slice u8)
                                #(t_Array u8 (mk_usize 8))
                                #FStar.Tactics.Typeclasses.solve
                                (blocks.[ {
                                      Core_models.Ops.Range.f_start
                                      =
                                      start +! (mk_usize 8 *! i <: usize) <: usize;
                                      Core_models.Ops.Range.f_end
                                      =
                                      (start +! (mk_usize 8 *! i <: usize) <: usize) +! mk_usize 8
                                      <:
                                      usize
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
                    <:
                    u64)
                  <:
                  bool
                else (state_future.[ i ] <: u64) =. (state.[ i ] <: u64) <: bool
              else true)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((start +! v_RATE <: usize) <=.
            (Core_models.Slice.impl__len #u8 blocks <: usize)
            <:
            bool)
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit = Hax_lib.v_assert ((v_RATE %! mk_usize 8 <: usize) =. mk_usize 0 <: bool) in
      ()
  in
  let _:Prims.unit = () in
  let state_flat:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
  let state_flat:t_Array u64 (mk_usize 25) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (v_RATE /! mk_usize 8 <: usize)
      (fun state_flat i ->
          let state_flat:t_Array u64 (mk_usize 25) = state_flat in
          let i:usize = i in
          forall (j: usize).
            b2t
            (if j <. (v_RATE /! mk_usize 8 <: usize) <: bool
              then
                if j <. i <: bool
                then
                  (state_flat.[ j ] <: u64) =.
                  (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array
                              u8 (mk_usize 8))
                          #Core_models.Array.t_TryFromSliceError
                          (Core_models.Convert.f_try_into #(t_Slice u8)
                              #(t_Array u8 (mk_usize 8))
                              #FStar.Tactics.Typeclasses.solve
                              (blocks.[ {
                                    Core_models.Ops.Range.f_start
                                    =
                                    start +! (mk_usize 8 *! j <: usize) <: usize;
                                    Core_models.Ops.Range.f_end
                                    =
                                    (start +! (mk_usize 8 *! j <: usize) <: usize) +! mk_usize 8
                                    <:
                                    usize
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
                  <:
                  bool
                else (state_flat.[ j ] <: u64) =. mk_u64 0 <: bool
              else true))
      state_flat
      (fun state_flat i ->
          let state_flat:t_Array u64 (mk_usize 25) = state_flat in
          let i:usize = i in
          let offset:usize = start +! (mk_usize 8 *! i <: usize) in
          let state_flat:t_Array u64 (mk_usize 25) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize state_flat
              i
              (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
                          (mk_usize 8))
                      #Core_models.Array.t_TryFromSliceError
                      (Core_models.Convert.f_try_into #(t_Slice u8)
                          #(t_Array u8 (mk_usize 8))
                          #FStar.Tactics.Typeclasses.solve
                          (blocks.[ {
                                Core_models.Ops.Range.f_start = offset;
                                Core_models.Ops.Range.f_end = offset +! mk_usize 8 <: usize
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
          state_flat)
  in
  let e_old_state:t_Array u64 (mk_usize 25) = state in
  let state:t_Array u64 (mk_usize 25) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (v_RATE /! mk_usize 8 <: usize)
      (fun state i ->
          let state:t_Array u64 (mk_usize 25) = state in
          let i:usize = i in
          forall (j: usize).
            b2t
            (if j <. mk_usize 25 <: bool
              then
                if j <. i <: bool
                then
                  (state.[ j ] <: u64) =.
                  ((e_old_state.[ j ] <: u64) ^. (state_flat.[ j ] <: u64) <: u64)
                  <:
                  bool
                else (state.[ j ] <: u64) =. (e_old_state.[ j ] <: u64) <: bool
              else true))
      state
      (fun state i ->
          let state:t_Array u64 (mk_usize 25) = state in
          let i:usize = i in
          let state:t_Array u64 (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 1)
              #u64
              state
              (i /! mk_usize 5 <: usize)
              (i %! mk_usize 5 <: usize)
              ((Libcrux_sha3.Traits.get_ij (mk_usize 1)
                    #u64
                    state
                    (i /! mk_usize 5 <: usize)
                    (i %! mk_usize 5 <: usize)
                  <:
                  u64) ^.
                (state_flat.[ i ] <: u64)
                <:
                u64)
          in
          state)
  in
  state

#pop-options

let load_last
      (v_RATE: usize)
      (v_DELIMITER: u8)
      (state: t_Array u64 (mk_usize 25))
      (blocks: t_Slice u8)
      (start len: usize)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 blocks <: usize)
          <:
          Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((start +! len <: usize) <=.
            (Core_models.Slice.impl__len #u8 blocks <: usize)
            <:
            bool)
      in
      ()
  in
  let buffer:t_Array u8 v_RATE = Rust_primitives.Hax.repeat (mk_u8 0) v_RATE in
  let buffer:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range buffer
      ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = len }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (buffer.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = len
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (blocks.[ {
                Core_models.Ops.Range.f_start = start;
                Core_models.Ops.Range.f_end = start +! len <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let buffer:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer len v_DELIMITER
  in
  let buffer:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer
      (v_RATE -! mk_usize 1 <: usize)
      ((buffer.[ v_RATE -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  let state:t_Array u64 (mk_usize 25) =
    load_block v_RATE state (buffer <: t_Slice u8) (mk_usize 0)
  in
  state

#push-options "--z3rlimit 300"

let store_block (v_RATE: usize) (s: t_Array u64 (mk_usize 25)) (out: t_Slice u8) (start len: usize)
    : Prims.Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          b2t
          ((Core_models.Slice.impl__len #u8 out_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out <: usize)
            <:
            bool) /\
          (forall (i: usize).
              b2t
              (if i <. (Core_models.Slice.impl__len #u8 out <: usize) <: bool
                then
                  if i <. start <: bool
                  then (out.[ i ] <: u8) =. (out_future.[ i ] <: u8) <: bool
                  else
                    if i <. (start +! len <: usize) <: bool
                    then
                      (out_future.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (s.[ (i -! start <: usize) /!
                                mk_usize 8
                                <:
                                usize ]
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out.[ i ] <: u8) =. (out_future.[ i ] <: u8) <: bool
                else true))) =
  let octets:usize = len /! mk_usize 8 in
  let old_out:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let out:t_Slice u8 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      octets
      (fun out i ->
          let out:t_Slice u8 = out in
          let i:usize = i in
          b2t
          ((Core_models.Slice.impl__len #u8 out <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out <: usize)
            <:
            bool) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out <: usize) <: bool
                then
                  if j <. start <: bool
                  then (out.[ j ] <: u8) =. (old_out.[ j ] <: u8) <: bool
                  else
                    if j <. (start +! (i *! mk_usize 8 <: usize) <: usize) <: bool
                    then
                      (out.[ j ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (s.[ (j -! start <: usize) /!
                                mk_usize 8
                                <:
                                usize ]
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (j -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (out.[ j ] <: u8) =. (old_out.[ j ] <: u8) <: bool
                else true)))
      out
      (fun out i ->
          let out:t_Slice u8 = out in
          let i:usize = i in
          let bytes:t_Array u8 (mk_usize 8) =
            Core_models.Num.impl_u64__to_le_bytes (Libcrux_sha3.Traits.get_ij (mk_usize 1)
                  #u64
                  s
                  (i /! mk_usize 5 <: usize)
                  (i %! mk_usize 5 <: usize)
                <:
                u64)
          in
          let out_pos:usize = start +! (mk_usize 8 *! i <: usize) in
          let _:Prims.unit =
            Proof_Utils.Lemmas.lemma_index_update_at_range out
              ({
                  Core_models.Ops.Range.f_start = out_pos;
                  Core_models.Ops.Range.f_end = out_pos +! mk_usize 8
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              bytes
          in
          let out:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out
              ({
                  Core_models.Ops.Range.f_start = out_pos;
                  Core_models.Ops.Range.f_end = out_pos +! mk_usize 8 <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (Core_models.Slice.impl__copy_from_slice #u8
                  (out.[ {
                        Core_models.Ops.Range.f_start = out_pos;
                        Core_models.Ops.Range.f_end = out_pos +! mk_usize 8 <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                  (bytes <: t_Slice u8)
                <:
                t_Slice u8)
          in
          out)
  in
  let remaining:usize = len %! mk_usize 8 in
  let out:t_Slice u8 =
    if remaining >. mk_usize 0
    then
      let bytes:t_Array u8 (mk_usize 8) =
        Core_models.Num.impl_u64__to_le_bytes (Libcrux_sha3.Traits.get_ij (mk_usize 1)
              #u64
              s
              (octets /! mk_usize 5 <: usize)
              (octets %! mk_usize 5 <: usize)
            <:
            u64)
      in
      let out_pos:usize = (start +! len <: usize) -! remaining in
      let _:Prims.unit =
        Proof_Utils.Lemmas.lemma_index_update_at_range out
          ({
              Core_models.Ops.Range.f_start = out_pos;
              Core_models.Ops.Range.f_end = out_pos +! remaining
            }
            <:
            Core_models.Ops.Range.t_Range usize)
          (Seq.slice bytes 0 (v remaining))
      in
      let out:t_Slice u8 =
        Rust_primitives.Hax.Monomorphized_update_at.update_at_range out
          ({
              Core_models.Ops.Range.f_start = out_pos;
              Core_models.Ops.Range.f_end = out_pos +! remaining <: usize
            }
            <:
            Core_models.Ops.Range.t_Range usize)
          (Core_models.Slice.impl__copy_from_slice #u8
              (out.[ {
                    Core_models.Ops.Range.f_start = out_pos;
                    Core_models.Ops.Range.f_end = out_pos +! remaining <: usize
                  }
                  <:
                  Core_models.Ops.Range.t_Range usize ]
                <:
                t_Slice u8)
              (bytes.[ { Core_models.Ops.Range.f_end = remaining }
                  <:
                  Core_models.Ops.Range.t_RangeTo usize ]
                <:
                t_Slice u8)
            <:
            t_Slice u8)
      in
      out
    else out
  in
  out

#pop-options

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: Libcrux_sha3.Traits.t_Absorb
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64) (mk_usize 1) =
  {
    f_load_block_pre
    =
    (fun
        (v_RATE: usize)
        (self_: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (input: t_Array (t_Slice u8) (mk_usize 1))
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
          Hax_lib.Int.t_Int));
    f_load_block_post
    =
    (fun
        (v_RATE: usize)
        (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (input: t_Array (t_Slice u8) (mk_usize 1))
        (start: usize)
        (out: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        ->
        true);
    f_load_block
    =
    (fun
        (v_RATE: usize)
        (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (input: t_Array (t_Slice u8) (mk_usize 1))
        (start: usize)
        ->
        let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
          {
            self with
            Libcrux_sha3.Generic_keccak.f_st
            =
            load_block v_RATE
              self.Libcrux_sha3.Generic_keccak.f_st
              (input.[ mk_usize 0 ] <: t_Slice u8)
              start
          }
          <:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64
        in
        self);
    f_load_last_pre
    =
    (fun
        (v_RATE: usize)
        (v_DELIMITER: u8)
        (self_: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (input: t_Array (t_Slice u8) (mk_usize 1))
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
          Hax_lib.Int.t_Int));
    f_load_last_post
    =
    (fun
        (v_RATE: usize)
        (v_DELIMITER: u8)
        (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (input: t_Array (t_Slice u8) (mk_usize 1))
        (start: usize)
        (len: usize)
        (out: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        ->
        true);
    f_load_last
    =
    fun
      (v_RATE: usize)
      (v_DELIMITER: u8)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (input: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
      (len: usize)
      ->
      let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
        {
          self with
          Libcrux_sha3.Generic_keccak.f_st
          =
          load_last v_RATE
            v_DELIMITER
            self.Libcrux_sha3.Generic_keccak.f_st
            (input.[ mk_usize 0 ] <: t_Slice u8)
            start
            len
        }
        <:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64
      in
      self
  }

#push-options "--split_queries always --z3rlimit 300"

let e_squeeze_impl_opts (_: Prims.unit) : Prims.unit = ()

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2: Libcrux_sha3.Traits.t_Squeeze
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64) u64 =
  {
    f_squeeze_pre
    =
    (fun
        (v_RATE: usize)
        (self_: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (out: t_Slice u8)
        (start: usize)
        (len: usize)
        ->
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out <: usize)
          <:
          Hax_lib.Int.t_Int));
    f_squeeze_post
    =
    (fun
        (v_RATE: usize)
        (self_: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        (out: t_Slice u8)
        (start: usize)
        (len: usize)
        (out_future: t_Slice u8)
        ->
        b2t
        ((Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)
          <:
          bool) /\
        (forall (i: usize).
            b2t
            (if i <. (Core_models.Slice.impl__len #u8 out <: usize) <: bool
              then
                if i <. start <: bool
                then (out.[ i ] <: u8) =. (out_future.[ i ] <: u8) <: bool
                else
                  if
                    (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int) <
                    ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
                      (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
                      <:
                      Hax_lib.Int.t_Int)
                    <:
                    bool
                  then
                    if ((i -! start <: usize) /! mk_usize 8 <: usize) <. mk_usize 25 <: bool
                    then
                      (out_future.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (self_
                                .Libcrux_sha3.Generic_keccak.f_st.[ (i -! start <: usize) /!
                                mk_usize 8
                                <:
                                usize ]
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! start <: usize) %! mk_usize 8 <: usize ]
                        <:
                        u8)
                      <:
                      bool
                    else true
                  else (out.[ i ] <: u8) =. (out_future.[ i ] <: u8) <: bool
              else true)));
    f_squeeze
    =
    fun
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
      (start: usize)
      (len: usize)
      ->
      let out:t_Slice u8 = store_block v_RATE self.Libcrux_sha3.Generic_keccak.f_st out start len in
      out
  }
