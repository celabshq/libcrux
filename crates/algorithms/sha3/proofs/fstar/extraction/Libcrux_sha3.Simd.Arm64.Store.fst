module Libcrux_sha3.Simd.Arm64.Store
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Arm64.Wrappers in
  let open Libcrux_sha3.Traits in
  ()

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

#push-options "--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'"

/// Loop-only half of `store_block`. Iterates over `i in 0..q`, calling
/// `store_u64x2x2` per iteration to fill `out0[start..start+16q]` and
/// `out1[start..start+16q]` from state slots `s[2*i]` and `s[2*i+1]`.
/// Frame: bytes outside `[start, start+16q)` are unchanged.
/// Factored out of `store_block` so its byte-level ensures composes
/// additively with the tail (`store_block_tail`) without forcing
/// in-body Euclidean div/mod bridging on the function ensures.
/// Verifies as a monolithic query (matching the prior verified shape
/// of `store_block` at commit `c14f94d2c`); split_queries with rlimit
/// 400 cliffs at the fold_range usize-range subtype check inside the
/// per-iteration body. `--z3refresh` keeps the SMT context fresh
/// across runs so that hint replay does not cause cliffs.
let store_block_full
      (s: t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (out0 out1: t_Slice u8)
      (start q: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        q <=. mk_usize 12 &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          ((Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int) *
            (Rust_primitives.Hax.Int.from_machine q <: Hax_lib.Int.t_Int)
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
                  if j <. start <: bool
                  then (out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool
                  else
                    if j <. (start +! (mk_usize 16 *! q <: usize) <: usize) <: bool
                    then
                      (out0_future.[ j ] <: u8) =.
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
                    else (out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool
                else true)) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out1 <: usize) <: bool
                then
                  if j <. start <: bool
                  then (out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool
                  else
                    if j <. (start +! (mk_usize 16 *! q <: usize) <: usize) <: bool
                    then
                      (out1_future.[ j ] <: u8) =.
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
                    else (out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool
                else true))) =
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
      q
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
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'"

/// Tail-only half of `store_block`. Dispatches to `store_tail_high`
/// (when `remaining > 8`) or `store_tail_low` (when
/// `0 < remaining <= 8`) to fill the partial window
/// `out0[start+16q..start+16q+remaining]` (likewise `out1`). When
/// `remaining == 0` the function is a no-op.
/// Frame: bytes outside `[start+16*q, start+16*q+remaining)` are
/// unchanged.
/// Verifies as a monolithic query (matching `store_block_full`).
/// `--z3refresh` keeps SMT context fresh across runs.
let store_block_tail
      (s: t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25))
      (out0 out1: t_Slice u8)
      (start q remaining: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        q <=. mk_usize 12 &&
        remaining <. mk_usize 16 &&
        (((Rust_primitives.Hax.Int.from_machine (mk_i32 16) <: Hax_lib.Int.t_Int) *
            (Rust_primitives.Hax.Int.from_machine q <: Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine remaining <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <
        (Rust_primitives.Hax.Int.from_machine (mk_i32 200) <: Hax_lib.Int.t_Int) &&
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
                  then (out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool
                  else
                    if
                      j <. ((start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize)
                      <:
                      bool
                    then
                      (out0_future.[ j ] <: u8) =.
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
                    else (out0.[ j ] <: u8) =. (out0_future.[ j ] <: u8) <: bool
                else true)) /\
          (forall (j: usize).
              b2t
              (if j <. (Core_models.Slice.impl__len #u8 out1 <: usize) <: bool
                then
                  if j <. (start +! (mk_usize 16 *! q <: usize) <: usize) <: bool
                  then (out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool
                  else
                    if
                      j <. ((start +! (mk_usize 16 *! q <: usize) <: usize) +! remaining <: usize)
                      <:
                      bool
                    then
                      (out1_future.[ j ] <: u8) =.
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
                    else (out1.[ j ] <: u8) =. (out1_future.[ j ] <: u8) <: bool
                else true))) =
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
    if remaining >. mk_usize 8
    then
      let i:usize = mk_usize 2 *! q in
      let i0:usize = i /! mk_usize 5 in
      let j0:usize = i %! mk_usize 5 in
      let i1:usize = (i +! mk_usize 1 <: usize) /! mk_usize 5 in
      let j1:usize = (i +! mk_usize 1 <: usize) %! mk_usize 5 in
      let s_2i:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
        Libcrux_sha3.Traits.get_ij (mk_usize 2)
          #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
          s
          i0
          j0
      in
      let s_succ:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
        Libcrux_sha3.Traits.get_ij (mk_usize 2)
          #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
          s
          i1
          j1
      in
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
        store_tail_high out0 out1 s_2i s_succ start q remaining
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      let _:Prims.unit =
        FStar.Math.Lemmas.lemma_div_mod (2 * v q) 5;
        FStar.Math.Lemmas.lemma_div_mod (2 * v q + 1) 5;
        assert (5 * ((2 * v q) / 5) + (2 * v q) % 5 == 2 * v q);
        assert (5 * ((2 * v q + 1) / 5) + (2 * v q + 1) % 5 == 2 * v q + 1);
        assert (Seq.index s (2 * v q) == s_2i);
        assert (Seq.index s (2 * v q + 1) == s_succ)
      in
      out0, out1 <: (t_Slice u8 & t_Slice u8)
    else
      if remaining >. mk_usize 0
      then
        let i:usize = mk_usize 2 *! q in
        let s_2q:Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
          Libcrux_sha3.Traits.get_ij (mk_usize 2)
            #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
            s
            (i /! mk_usize 5 <: usize)
            (i %! mk_usize 5 <: usize)
        in
        let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
          store_tail_low out0 out1 s_2q start q remaining
        in
        let out0:t_Slice u8 = tmp0 in
        let out1:t_Slice u8 = tmp1 in
        let _:Prims.unit = () in
        let _:Prims.unit =
          FStar.Math.Lemmas.lemma_div_mod (2 * v q) 5;
          assert (5 * ((2 * v q) / 5) + (2 * v q) % 5 == 2 * v q);
          assert (Seq.index s (2 * v q) == s_2q)
        in
        out0, out1 <: (t_Slice u8 & t_Slice u8)
      else out0, out1 <: (t_Slice u8 & t_Slice u8)
  in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

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
  let q:usize = len /! mk_usize 16 in
  let remaining:usize = len %! mk_usize 16 in
  let _:Prims.unit =
    assert (v len == 16 * v q + v remaining);
    assert (v remaining < 16);
    assert (v q <= 12);
    assert (16 * v q + v remaining == v len);
    assert (v len < 200);
    assert (v start + 16 * v q + v remaining == v start + v len);
    assert (v start + 16 * v q <= v start + v len);
    assert (v start + v len <= Seq.length out0);
    assert (v start + v len <= Seq.length out1)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) = store_block_full s out0 out1 start q in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) = store_block_tail s out0 out1 start q remaining in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_Squeeze2
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
