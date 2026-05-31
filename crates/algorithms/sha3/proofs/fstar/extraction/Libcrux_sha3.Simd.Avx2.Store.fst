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

[@@ "opaque_to_smt"]

/// `stored s out start lane lo hi`: every byte index `k` in the
/// half-open range `[lo, hi)` of `out` already holds the correct
/// squeezed output byte — namely byte `(k-start) % 8` of the
/// little-endian encoding of lane `lane` of state word
/// `s[(k-start)/8]`. The AVX2 analog of the user's
/// `store_block_output`, lifted to a range. Opaque so the outer-loop
/// and composer never unfold the per-byte content; only the producer
/// (`store_u64x4x4`) reveals it.
let stored
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out: t_Slice u8)
      (start lane lo hi: usize)
    : Hax_lib.Prop.t_Prop =
  forall (k: usize).
    b2t
    ((lane <. mk_usize 4 <: bool) && (start <=. k <: bool) && (lo <=. k <: bool) &&
      (k <. hi <: bool) &&
      (k <. (Core_models.Slice.impl__len #u8 out <: usize) <: bool) &&
      (((k -! start <: usize) /! mk_usize 8 <: usize) <. mk_usize 25 <: bool)) ==>
    b2t
    ((out.[ k ] <: u8) =.
      ((Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (s.[ (k -!
                      start
                      <:
                      usize) /!
                    mk_usize 8
                    <:
                    usize ]
                  <:
                  Libcrux_intrinsics.Avx2_extract.t_Vec256)
                lane
              <:
              u64)
          <:
          t_Array u8 (mk_usize 8)).[ (k -! start <: usize) %! mk_usize 8 <: usize ]
        <:
        u8)
      <:
      bool)

let lemma_stored_frame
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (a b: t_Slice u8)
      (start lane lo hi mlo mhi: usize)
  : Lemma
    (requires
      stored s a start lane lo hi /\
      Libcrux_sha3.Proof_utils.modifies_range a b mlo mhi /\
      v hi <= v mlo)
    (ensures stored s b start lane lo hi)
  = reveal_opaque (`%stored) stored;
    reveal_opaque (`%Libcrux_sha3.Proof_utils.modifies_range)
      Libcrux_sha3.Proof_utils.modifies_range

let lemma_stored_union
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out: t_Slice u8)
      (start lane lo mid hi: usize)
  : Lemma
    (requires
      stored s out start lane lo mid /\ stored s out start lane mid hi /\
      v lo <= v mid /\ v mid <= v hi)
    (ensures stored s out start lane lo hi)
  = reveal_opaque (`%stored) stored

let lemma_stored_empty
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out: t_Slice u8)
      (start lane lo: usize)
  : Lemma (ensures stored s out start lane lo lo)
  = reveal_opaque (`%stored) stored

#push-options "--z3rlimit 400"
let lemma_window_stored
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (s0 s1 s2 s3: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (start i: usize)
      (lane_m: nat{lane_m < 4})
      (out_m: t_Slice u8)
  : Lemma
    (requires
      v i < 6 /\
      v start + 32 * (v i + 1) <= Seq.length out_m /\
      s0 == Seq.index s (4 * v i + 0) /\
      s1 == Seq.index s (4 * v i + 1) /\
      s2 == Seq.index s (4 * v i + 2) /\
      s3 == Seq.index s (4 * v i + 3) /\
      (forall (j_n: nat).
        (v start + 32 * v i <= j_n /\ j_n < v start + 32 * (v i + 1) /\ j_n < Seq.length out_m) ==>
        (if (j_n - v start) / 8 = 4 * v i then
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s0 (mk_usize lane_m))) ((j_n - v start) % 8)
         else if (j_n - v start) / 8 = 4 * v i + 1 then
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s1 (mk_usize lane_m))) ((j_n - v start) % 8)
         else if (j_n - v start) / 8 = 4 * v i + 2 then
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 (mk_usize lane_m))) ((j_n - v start) % 8)
         else
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s3 (mk_usize lane_m))) ((j_n - v start) % 8))))
    (ensures
      stored s out_m start (mk_usize lane_m)
        (start +! (mk_usize 32 *! i)) (start +! (mk_usize 32 *! (i +! mk_usize 1))))
  = Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.lemma_lane_chain_to_s_all_j
      s s0 s1 s2 s3 (v start) (v i) lane_m out_m;
    reveal_opaque (`%stored) stored
#pop-options

let lemma_window_modifies (out_old out_new: t_Slice u8) (start i: usize)
  : Lemma
    (requires
      v start + 32 * (v i + 1) <= Seq.length out_old /\
      Seq.length out_new == Seq.length out_old /\
      (forall (j_n: nat).
        (j_n < Seq.length out_old /\
         (j_n < v start + 32 * v i \/ j_n >= v start + 32 * (v i + 1))) ==>
        Seq.index out_new j_n == Seq.index out_old j_n))
    (ensures
      Libcrux_sha3.Proof_utils.modifies_range out_old out_new
        (start +! (mk_usize 32 *! i)) (start +! (mk_usize 32 *! (i +! mk_usize 1))))
  = reveal_opaque (`%Libcrux_sha3.Proof_utils.modifies_range)
      Libcrux_sha3.Proof_utils.modifies_range

#push-options "--z3rlimit 400"
let lemma_window_stored_single
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out_m: t_Slice u8)
      (start: usize)
      (lane_m: nat{lane_m < 4})
      (off w: usize)
      (base: nat)
  : Lemma
    (requires
      base < 25 /\ vec == Seq.index s base /\
      v start <= v off /\ v w <= 8 /\ v off + v w <= Seq.length out_m /\
      (v off - v start) / 8 == base /\ (v off - v start) % 8 == 0 /\
      (forall (j_n: nat).
        (v off <= j_n /\ j_n < v off + v w /\ j_n < Seq.length out_m) ==>
        Seq.index out_m j_n == Seq.index
          (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize lane_m)))
          (j_n - v off)))
    (ensures stored s out_m start (mk_usize lane_m) off (off +! w))
  = reveal_opaque (`%stored) stored;
    let aux (j_n: nat{j_n < Seq.length out_m}) :
      Lemma ((v off <= j_n /\ j_n < v off + v w) ==>
        Seq.index out_m j_n == Seq.index
          (Core_models.Num.impl_u64__to_le_bytes
            (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (Seq.index s ((j_n - v start) / 8)) (mk_usize lane_m)))
          ((j_n - v start) % 8))
      = if v off <= j_n && j_n < v off + v w then begin
          // off - start = 8 * base; j_n - start = (j_n - off) + 8 * base,
          // with 0 <= j_n - off < 8, so /8 == base and %8 == j_n - off.
          FStar.Math.Lemmas.lemma_div_plus (j_n - v off) base 8;
          FStar.Math.Lemmas.lemma_mod_plus (j_n - v off) base 8
        end else ()
    in
    Classical.forall_intro aux
#pop-options

#push-options "--z3rlimit 800 --split_queries always --z3refresh"

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
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (s0 s1 s2 s3: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (start i: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Seq.length out0 == Seq.length out1 /\ Seq.length out0 == Seq.length out2 /\
        Seq.length out0 == Seq.length out3 /\ v i < 6 /\ v start + 32 * (v i + 1) <= Seq.length out0 /\
        s0 == Seq.index s (4 * v i + 0) /\ s1 == Seq.index s (4 * v i + 1) /\
        s2 == Seq.index s (4 * v i + 2) /\ s3 == Seq.index s (4 * v i + 3))
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
          Libcrux_sha3.Proof_utils.modifies_range out0
            out0_future
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out1
            out1_future
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out2
            out2_future
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out3
            out3_future
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          stored s
            out0_future
            start
            (mk_usize 0)
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          stored s
            out1_future
            start
            (mk_usize 1)
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          stored s
            out2_future
            start
            (mk_usize 2)
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize) /\
          stored s
            out3_future
            start
            (mk_usize 3)
            (start +! (mk_usize 32 *! i <: usize) <: usize)
            (start +! (mk_usize 32 *! (i +! mk_usize 1 <: usize) <: usize) <: usize)) =
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
    Classical.forall_intro bridge_out3;
    FStar.Math.Lemmas.lemma_div_mod (4 * v i) 5;
    FStar.Math.Lemmas.lemma_div_mod (4 * v i + 1) 5;
    FStar.Math.Lemmas.lemma_div_mod (4 * v i + 2) 5;
    FStar.Math.Lemmas.lemma_div_mod (4 * v i + 3) 5;
    assert (Seq.index s (4 * v i + 0) == s0);
    assert (Seq.index s (4 * v i + 1) == s1);
    assert (Seq.index s (4 * v i + 2) == s2);
    assert (Seq.index s (4 * v i + 3) == s3);
    lemma_window_stored s s0 s1 s2 s3 start i 0 out0;
    lemma_window_stored s s0 s1 s2 s3 start i 1 out1;
    lemma_window_stored s s0 s1 s2 s3 start i 2 out2;
    lemma_window_stored s s0 s1 s2 s3 start i 3 out3;
    lemma_window_modifies old_out0 out0 start i;
    lemma_window_modifies old_out1 out1 start i;
    lemma_window_modifies old_out2 out2 start i;
    lemma_window_modifies old_out3 out3 start i
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 400 --split_queries always --z3refresh"

/// Inner-loop leaf producer (8-byte chunk) for the tail. Writes the
/// 8-byte window `[off, off+8)` (`off = start+32*q+8*k`) of each output
/// from lane `m` of `vec` (= `s[4*q+k]`, supplied + linked by the
/// caller). The per-byte storeu/copy facts are bridged here, then
/// packaged into the opaque `stored` / `modifies_range` via the
/// confined-reveal lemmas.
let store_chunk8x4
      (out0 out1 out2 out3: t_Slice u8)
      (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (start q k: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Seq.length out0 == Seq.length out1 /\ Seq.length out0 == Seq.length out2 /\
        Seq.length out0 == Seq.length out3 /\ 4 * v q + v k < 25 /\
        v start + 32 * v q + 8 * (v k + 1) <= Seq.length out0 /\ vec == Seq.index s (4 * v q + v k))
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
          Libcrux_sha3.Proof_utils.modifies_range out0
            out0_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out1
            out1_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out2
            out2_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out3
            out3_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          stored s
            out0_future
            start
            (mk_usize 0)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          stored s
            out1_future
            start
            (mk_usize 1)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          stored s
            out2_future
            start
            (mk_usize 2)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize) /\
          stored s
            out3_future
            start
            (mk_usize 3)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +!
              (mk_usize 8 *! (k +! mk_usize 1 <: usize) <: usize)
              <:
              usize)) =
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
  let off:usize = (start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) in
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({
          Core_models.Ops.Range.f_start = off;
          Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out0.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
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
          Core_models.Ops.Range.f_start = off;
          Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out1.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
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
          Core_models.Ops.Range.f_start = off;
          Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out2.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
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
          Core_models.Ops.Range.f_start = off;
          Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out3.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! mk_usize 8 <: usize
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
    let a_pos:nat = v start + 32 * v q + 8 * v k in
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
    Classical.forall_intro bridge_call_out3;
    lemma_window_stored_single s vec out0 start 0 off (mk_usize 8) (4 * v q + v k);
    lemma_window_stored_single s vec out1 start 1 off (mk_usize 8) (4 * v q + v k);
    lemma_window_stored_single s vec out2 start 2 off (mk_usize 8) (4 * v q + v k);
    lemma_window_stored_single s vec out3 start 3 off (mk_usize 8) (4 * v q + v k);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out0 out0 off (off +! mk_usize 8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out1 out1 off (off +! mk_usize 8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out2 out2 off (off +! mk_usize 8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out3 out3 off (off +! mk_usize 8)
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 400 --split_queries always --z3refresh"

/// Ragged leaf producer for the tail\'s final `rem8 < 8` bytes. Writes
/// `[off, off+rem8)` (`off = start+32*q+8*chunks8`) of each output from
/// lane `m` of `vec` (= `s[4*q+chunks8]`, supplied + linked). Per-byte
/// bridge then packaged into opaque `stored` / `modifies_range`.
let store_tail_ragged_avx2
      (out0 out1 out2 out3: t_Slice u8)
      (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (start q chunks8 rem8: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Seq.length out0 == Seq.length out1 /\ Seq.length out0 == Seq.length out2 /\
        Seq.length out0 == Seq.length out3 /\ v rem8 > 0 /\ v rem8 < 8 /\ 4 * v q + v chunks8 < 25 /\
        v start + 32 * v q + 8 * v chunks8 + v rem8 <= Seq.length out0 /\
        vec == Seq.index s (4 * v q + v chunks8))
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
          Libcrux_sha3.Proof_utils.modifies_range out0
            out0_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out1
            out1_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out2
            out2_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out3
            out3_future
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          stored s
            out0_future
            start
            (mk_usize 0)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          stored s
            out1_future
            start
            (mk_usize 1)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          stored s
            out2_future
            start
            (mk_usize 2)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize) /\
          stored s
            out3_future
            start
            (mk_usize 3)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
              <:
              usize)
            (((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
                <:
                usize) +!
              rem8
              <:
              usize)) =
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
  let off:usize =
    (start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! chunks8 <: usize)
  in
  let out0:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out0
      ({ Core_models.Ops.Range.f_start = off; Core_models.Ops.Range.f_end = off +! rem8 <: usize }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out0.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! rem8 <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (u8s.[ { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = rem8 }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let out1:t_Slice u8 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range out1
      ({ Core_models.Ops.Range.f_start = off; Core_models.Ops.Range.f_end = off +! rem8 <: usize }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out1.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! rem8 <: usize
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
      ({ Core_models.Ops.Range.f_start = off; Core_models.Ops.Range.f_end = off +! rem8 <: usize }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out2.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! rem8 <: usize
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
      ({ Core_models.Ops.Range.f_start = off; Core_models.Ops.Range.f_end = off +! rem8 <: usize }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (out3.[ {
                Core_models.Ops.Range.f_start = off;
                Core_models.Ops.Range.f_end = off +! rem8 <: usize
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
  let _:Prims.unit =
    let a_pos:nat = v start + 32 * v q + 8 * v chunks8 in
    let r:nat = v rem8 in
    assert (r < 8);
    assert (a_pos + r <= Seq.length old_out0);
    assert (a_pos + r <= Seq.length old_out1);
    assert (a_pos + r <= Seq.length old_out2);
    assert (a_pos + r <= Seq.length old_out3);
    Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.mm256_storeu_si256_u8_byte_window (Rust_primitives.Hax.repeat
          (mk_u8 0)
          (mk_usize 32))
      vec;
    let bridge_partial_out_m
          (m_lane: nat{m_lane < 4})
          (out_old out_new: Seq.seq u8)
          (j_n: nat{j_n < Seq.length out_old})
        : Lemma
          (requires
            a_pos + r <= Seq.length out_old /\ Seq.length out_new == Seq.length out_old /\
            Seq.slice out_new 0 a_pos == Seq.slice out_old 0 a_pos /\
            Seq.slice out_new a_pos (a_pos + r) == Seq.slice u8s (m_lane * 8) (m_lane * 8 + r) /\
            Seq.slice out_new (a_pos + r) (Seq.length out_new) ==
            Seq.slice out_old (a_pos + r) (Seq.length out_old))
          (ensures
            (if j_n < a_pos
              then Seq.index out_new j_n == Seq.index out_old j_n
              else
                if j_n < a_pos + r
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
        if j_n < a_pos + r
        then
          let t:nat = j_n - a_pos in
          assert (t < r /\ t < 8);
          assert (Seq.index (Seq.slice out_new a_pos (a_pos + r)) t == Seq.index out_new j_n);
          assert (Seq.index (Seq.slice u8s (m_lane * 8) (m_lane * 8 + r)) t ==
              Seq.index u8s (m_lane * 8 + t));
          assert ((m_lane * 8 + t) / 8 == m_lane);
          assert ((m_lane * 8 + t) % 8 == t)
        else
          let t:nat = j_n - (a_pos + r) in
          assert (Seq.index (Seq.slice out_new (a_pos + r) (Seq.length out_new)) t ==
              Seq.index out_new j_n);
          assert (Seq.index (Seq.slice out_old (a_pos + r) (Seq.length out_old)) t ==
              Seq.index out_old j_n)
    in
    let bridge_call_out0 (j_n: nat{j_n < Seq.length old_out0})
        : Lemma
        (if j_n < a_pos
          then Seq.index out0 j_n == Seq.index old_out0 j_n
          else
            if j_n < a_pos + r
            then
              Seq.index out0 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 0)))
                (j_n - a_pos)
            else Seq.index out0 j_n == Seq.index old_out0 j_n) =
      bridge_partial_out_m 0 old_out0 out0 j_n
    in
    let bridge_call_out1 (j_n: nat{j_n < Seq.length old_out1})
        : Lemma
        (if j_n < a_pos
          then Seq.index out1 j_n == Seq.index old_out1 j_n
          else
            if j_n < a_pos + r
            then
              Seq.index out1 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 1)))
                (j_n - a_pos)
            else Seq.index out1 j_n == Seq.index old_out1 j_n) =
      bridge_partial_out_m 1 old_out1 out1 j_n
    in
    let bridge_call_out2 (j_n: nat{j_n < Seq.length old_out2})
        : Lemma
        (if j_n < a_pos
          then Seq.index out2 j_n == Seq.index old_out2 j_n
          else
            if j_n < a_pos + r
            then
              Seq.index out2 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 2)))
                (j_n - a_pos)
            else Seq.index out2 j_n == Seq.index old_out2 j_n) =
      bridge_partial_out_m 2 old_out2 out2 j_n
    in
    let bridge_call_out3 (j_n: nat{j_n < Seq.length old_out3})
        : Lemma
        (if j_n < a_pos
          then Seq.index out3 j_n == Seq.index old_out3 j_n
          else
            if j_n < a_pos + r
            then
              Seq.index out3 j_n ==
              Seq.index (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64
                        vec
                        (mk_usize 3)))
                (j_n - a_pos)
            else Seq.index out3 j_n == Seq.index old_out3 j_n) =
      bridge_partial_out_m 3 old_out3 out3 j_n
    in
    Classical.forall_intro bridge_call_out0;
    Classical.forall_intro bridge_call_out1;
    Classical.forall_intro bridge_call_out2;
    Classical.forall_intro bridge_call_out3;
    lemma_window_stored_single s vec out0 start 0 off rem8 (4 * v q + v chunks8);
    lemma_window_stored_single s vec out1 start 1 off rem8 (4 * v q + v chunks8);
    lemma_window_stored_single s vec out2 start 2 off rem8 (4 * v q + v chunks8);
    lemma_window_stored_single s vec out3 start 3 off rem8 (4 * v q + v chunks8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out0 out0 off (off +! rem8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out1 out1 off (off +! rem8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out2 out2 off (off +! rem8);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out3 out3 off (off +! rem8)
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'"

/// Outer-loop half of `store_block`: writes the full 32-byte windows
/// `[start, start+32*q)` by calling `store_u64x4x4` per iteration.
/// Verified via an opaque-`stored`/`modifies_range` loop invariant with
/// per-iteration frame/union carryover.
let store_block_full_avx2
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out0 out1 out2 out3: t_Slice u8)
      (start q: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize) &&
        q <=. mk_usize 6 &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          ((Rust_primitives.Hax.Int.from_machine (mk_i32 32) <: Hax_lib.Int.t_Int) *
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
          Libcrux_sha3.Proof_utils.modifies_range out0
            out0_future
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out1
            out1_future
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out2
            out2_future
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out3
            out3_future
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          stored s
            out0_future
            start
            (mk_usize 0)
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          stored s
            out1_future
            start
            (mk_usize 1)
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          stored s
            out2_future
            start
            (mk_usize 2)
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize) /\
          stored s
            out3_future
            start
            (mk_usize 3)
            start
            (start +! (mk_usize 32 *! q <: usize) <: usize)) =
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
  let _:Prims.unit =
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
    assert (old_out0 == out0);
    assert (old_out1 == out1);
    assert (old_out2 == out2);
    assert (old_out3 == out3);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out0 start start;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out1 start start;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out2 start start;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out3 start start;
    lemma_stored_empty s out0 start (mk_usize 0) start;
    lemma_stored_empty s out1 start (mk_usize 1) start;
    lemma_stored_empty s out2 start (mk_usize 2) start;
    lemma_stored_empty s out3 start (mk_usize 3) start
  in
  let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      q
      (fun temp_0_ i ->
          let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
            temp_0_
          in
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
          b2t
          ((Core_models.Slice.impl__len #u8 out2 <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out2 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out3 <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out3 <: usize)
            <:
            bool) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out0
            out0
            start
            (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out1
            out1
            start
            (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out2
            out2
            start
            (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out3
            out3
            start
            (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          stored s out0 start (mk_usize 0) start (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          stored s out1 start (mk_usize 1) start (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          stored s out2 start (mk_usize 2) start (start +! (mk_usize 32 *! i <: usize) <: usize) /\
          stored s out3 start (mk_usize 3) start (start +! (mk_usize 32 *! i <: usize) <: usize))
      (out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
      (fun temp_0_ i ->
          let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
            temp_0_
          in
          let i:usize = i in
          let p0:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let p1:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let p2:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let p3:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let _:Prims.unit =
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
            assert (p0 == out0);
            assert (p1 == out1);
            assert (p2 == out2);
            assert (p3 == out3);
            FStar.Math.Lemmas.lemma_div_mod (4 * v i) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 1) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 2) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 3) 5
          in
          let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
            store_u64x4x4 out0 out1 out2 out3 s
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  ((mk_usize 4 *! i <: usize) /! mk_usize 5 <: usize)
                  ((mk_usize 4 *! i <: usize) %! mk_usize 5 <: usize)
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  (((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize) /! mk_usize 5 <: usize)
                  (((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize) %! mk_usize 5 <: usize)
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  (((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize) /! mk_usize 5 <: usize)
                  (((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize) %! mk_usize 5 <: usize)
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  (((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize) /! mk_usize 5 <: usize)
                  (((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize) %! mk_usize 5 <: usize)
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256) start i
          in
          let out0:t_Slice u8 = tmp0 in
          let out1:t_Slice u8 = tmp1 in
          let out2:t_Slice u8 = tmp2 in
          let out3:t_Slice u8 = tmp3 in
          let _:Prims.unit = () in
          let _:Prims.unit =
            let lo:usize = start in
            let mid:usize = start +! (mk_usize 32 *! i) in
            let hi:usize = start +! (mk_usize 32 *! (i +! mk_usize 1)) in
            assert (v lo <= v mid /\ v mid <= v hi);
            lemma_stored_frame s p0 out0 start (mk_usize 0) lo mid mid hi;
            lemma_stored_frame s p1 out1 start (mk_usize 1) lo mid mid hi;
            lemma_stored_frame s p2 out2 start (mk_usize 2) lo mid mid hi;
            lemma_stored_frame s p3 out3 start (mk_usize 3) lo mid mid hi;
            lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
            lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
            lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
            lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out0 p0 out0 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out1 p1 out1 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out2 p2 out2 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out3 p3 out3 lo mid hi
          in
          out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'"

/// Tail half of `store_block`: writes the partial window
/// `[start+32*q, start+32*q+rem)` (`rem < 32`) via the inner 8-byte
/// loop (`store_chunk8x4`) and the ragged remainder
/// (`store_tail_ragged_avx2`), composing their opaque posts.
let store_block_tail_avx2
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out0 out1 out2 out3: t_Slice u8)
      (start q rem: usize)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize) &&
        q <=. mk_usize 6 &&
        rem <. mk_usize 32 &&
        ((mk_usize 4 *! q <: usize) +! (rem /! mk_usize 8 <: usize) <: usize) <. mk_usize 25 &&
        (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 32) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine q <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine rem <: Hax_lib.Int.t_Int)
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
          Libcrux_sha3.Proof_utils.modifies_range out0
            out0_future
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out1
            out1_future
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out2
            out2_future
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out3
            out3_future
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          stored s
            out0_future
            start
            (mk_usize 0)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          stored s
            out1_future
            start
            (mk_usize 1)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          stored s
            out2_future
            start
            (mk_usize 2)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize) /\
          stored s
            out3_future
            start
            (mk_usize 3)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! rem <: usize)) =
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
  let _:Prims.unit =
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
    assert (old_out0 == out0);
    assert (old_out1 == out1);
    assert (old_out2 == out2);
    assert (old_out3 == out3);
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out0
      (start +! (mk_usize 32 *! q))
      (start +! (mk_usize 32 *! q));
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out1
      (start +! (mk_usize 32 *! q))
      (start +! (mk_usize 32 *! q));
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out2
      (start +! (mk_usize 32 *! q))
      (start +! (mk_usize 32 *! q));
    Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out3
      (start +! (mk_usize 32 *! q))
      (start +! (mk_usize 32 *! q));
    lemma_stored_empty s out0 start (mk_usize 0) (start +! (mk_usize 32 *! q));
    lemma_stored_empty s out1 start (mk_usize 1) (start +! (mk_usize 32 *! q));
    lemma_stored_empty s out2 start (mk_usize 2) (start +! (mk_usize 32 *! q));
    lemma_stored_empty s out3 start (mk_usize 3) (start +! (mk_usize 32 *! q))
  in
  let chunks8:usize = rem /! mk_usize 8 in
  let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      chunks8
      (fun temp_0_ k ->
          let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
            temp_0_
          in
          let k:usize = k in
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
          b2t
          ((Core_models.Slice.impl__len #u8 out2 <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out2 <: usize)
            <:
            bool) /\
          b2t
          ((Core_models.Slice.impl__len #u8 out3 <: usize) =.
            (Core_models.Slice.impl__len #u8 old_out3 <: usize)
            <:
            bool) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out0
            out0
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out1
            out1
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out2
            out2
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range old_out3
            out3
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          stored s
            out0
            start
            (mk_usize 0)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          stored s
            out1
            start
            (mk_usize 1)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          stored s
            out2
            start
            (mk_usize 2)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize) /\
          stored s
            out3
            start
            (mk_usize 3)
            (start +! (mk_usize 32 *! q <: usize) <: usize)
            ((start +! (mk_usize 32 *! q <: usize) <: usize) +! (mk_usize 8 *! k <: usize) <: usize)
      )
      (out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
      (fun temp_0_ k ->
          let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
            temp_0_
          in
          let k:usize = k in
          let p0:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let p1:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let p2:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let p3:t_Slice u8 =
            Alloc.Vec.impl_1__as_slice #u8
              #Alloc.Alloc.t_Global
              (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
          in
          let _:Prims.unit =
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
            assert (p0 == out0);
            assert (p1 == out1);
            assert (p2 == out2);
            assert (p3 == out3);
            FStar.Math.Lemmas.lemma_div_mod (4 * v q + v k) 5
          in
          let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
            store_chunk8x4 out0
              out1
              out2
              out3
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
                  (((mk_usize 4 *! q <: usize) +! k <: usize) /! mk_usize 5 <: usize)
                  (((mk_usize 4 *! q <: usize) +! k <: usize) %! mk_usize 5 <: usize)
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              s
              start
              q
              k
          in
          let out0:t_Slice u8 = tmp0 in
          let out1:t_Slice u8 = tmp1 in
          let out2:t_Slice u8 = tmp2 in
          let out3:t_Slice u8 = tmp3 in
          let _:Prims.unit = () in
          let _:Prims.unit =
            let lo:usize = start +! (mk_usize 32 *! q) in
            let mid:usize = (start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! k) in
            let hi:usize = (start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! (k +! mk_usize 1)) in
            assert (v lo <= v mid /\ v mid <= v hi);
            lemma_stored_frame s p0 out0 start (mk_usize 0) lo mid mid hi;
            lemma_stored_frame s p1 out1 start (mk_usize 1) lo mid mid hi;
            lemma_stored_frame s p2 out2 start (mk_usize 2) lo mid mid hi;
            lemma_stored_frame s p3 out3 start (mk_usize 3) lo mid mid hi;
            lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
            lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
            lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
            lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out0 p0 out0 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out1 p1 out1 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out2 p2 out2 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out3 p3 out3 lo mid hi
          in
          out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8))
  in
  let rem8:usize = rem %! mk_usize 8 in
  let (out0: t_Slice u8), (out1: t_Slice u8), (out2: t_Slice u8), (out3: t_Slice u8) =
    if rem8 >. mk_usize 0
    then
      let r0:t_Slice u8 =
        Alloc.Vec.impl_1__as_slice #u8
          #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
      in
      let r1:t_Slice u8 =
        Alloc.Vec.impl_1__as_slice #u8
          #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
      in
      let r2:t_Slice u8 =
        Alloc.Vec.impl_1__as_slice #u8
          #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
      in
      let r3:t_Slice u8 =
        Alloc.Vec.impl_1__as_slice #u8
          #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
      in
      let _:Prims.unit =
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
        assert (r0 == out0);
        assert (r1 == out1);
        assert (r2 == out2);
        assert (r3 == out3);
        FStar.Math.Lemmas.lemma_div_mod (4 * v q + v chunks8) 5
      in
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
        store_tail_ragged_avx2 out0 out1 out2 out3
          (Libcrux_sha3.Traits.get_ij (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              s
              (((mk_usize 4 *! q <: usize) +! chunks8 <: usize) /! mk_usize 5 <: usize)
              (((mk_usize 4 *! q <: usize) +! chunks8 <: usize) %! mk_usize 5 <: usize)
            <:
            Libcrux_intrinsics.Avx2_extract.t_Vec256) s start q chunks8 rem8
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let out2:t_Slice u8 = tmp2 in
      let out3:t_Slice u8 = tmp3 in
      let _:Prims.unit = () in
      let _:Prims.unit =
        FStar.Math.Lemmas.lemma_div_mod (v rem) 8;
        let lo:usize = start +! (mk_usize 32 *! q) in
        let mid:usize = (start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! chunks8) in
        let hi:usize = (start +! (mk_usize 32 *! q)) +! rem in
        assert (v mid + v rem8 == v hi);
        assert (mid +! rem8 == hi);
        assert (v lo <= v mid /\ v mid <= v hi);
        lemma_stored_frame s r0 out0 start (mk_usize 0) lo mid mid hi;
        lemma_stored_frame s r1 out1 start (mk_usize 1) lo mid mid hi;
        lemma_stored_frame s r2 out2 start (mk_usize 2) lo mid mid hi;
        lemma_stored_frame s r3 out3 start (mk_usize 3) lo mid mid hi;
        lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
        lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
        lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
        lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out0 r0 out0 lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out1 r1 out1 lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out2 r2 out2 lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out3 r3 out3 lo mid hi
      in
      out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
    else
      let _:Prims.unit =
        FStar.Math.Lemmas.lemma_div_mod (v rem) 8;
        assert (v rem == 8 * v chunks8);
        assert ((start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! chunks8) ==
            (start +! (mk_usize 32 *! q)) +! rem)
      in
      out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

#push-options "--z3rlimit 400 --split_queries always"

/// Composer (top of the store_block proof). Splits `len = 32*chunks + rem`,
/// calls the full and tail halves, and composes their opaque
/// `stored` / `modifies_range` posts over the adjacent ranges
/// `[start, start+32*chunks)` and `[start+32*chunks, start+len)` into a
/// single `stored` / `modifies_range` over `[start, start+len)` via the
/// frame lemmas. No `reveal` here — the byte content stays opaque.
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
          Libcrux_sha3.Proof_utils.modifies_range out0 out0_future start (start +! len <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out1 out1_future start (start +! len <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out2 out2_future start (start +! len <: usize) /\
          Libcrux_sha3.Proof_utils.modifies_range out3 out3_future start (start +! len <: usize) /\
          stored s out0_future start (mk_usize 0) start (start +! len <: usize) /\
          stored s out1_future start (mk_usize 1) start (start +! len <: usize) /\
          stored s out2_future start (mk_usize 2) start (start +! len <: usize) /\
          stored s out3_future start (mk_usize 3) start (start +! len <: usize)) =
  let chunks:usize = len /! mk_usize 32 in
  let rem:usize = len %! mk_usize 32 in
  let e0:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let e1:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let e2:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let e3:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let _:Prims.unit =
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
    assert (e0 == out0);
    assert (e1 == out1);
    assert (e2 == out2);
    assert (e3 == out3);
    assert (v len == 32 * v chunks + v rem);
    assert (v chunks <= 6);
    assert (v rem < 32);
    assert (v start + 32 * v chunks + v rem == v start + v len);
    assert (v start + 32 * v chunks <= v start + v len);
    assert (v len == v rem + (4 * v chunks) * 8);
    FStar.Math.Lemmas.lemma_div_plus (v rem) (4 * v chunks) 8;
    assert (v len / 8 == v rem / 8 + 4 * v chunks);
    assert (v len < 200);
    assert (4 * v chunks + v rem / 8 < 25)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    store_block_full_avx2 s out0 out1 out2 out3 start chunks
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let mid0:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out0 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let mid1:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out1 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let mid2:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out2 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let mid3:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 out3 <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let _:Prims.unit =
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
    assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
    assert (mid0 == out0);
    assert (mid1 == out1);
    assert (mid2 == out2);
    assert (mid3 == out3)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    store_block_tail_avx2 s out0 out1 out2 out3 start chunks rem
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let _:Prims.unit =
    let lo:usize = start in
    let mid:usize = start +! (mk_usize 32 *! chunks) in
    let hi:usize = start +! len in
    assert (v mid + v rem == v hi);
    assert (mid +! rem == hi);
    assert (v lo <= v mid /\ v mid <= v hi);
    lemma_stored_frame s mid0 out0 start (mk_usize 0) lo mid mid hi;
    lemma_stored_frame s mid1 out1 start (mk_usize 1) lo mid mid hi;
    lemma_stored_frame s mid2 out2 start (mk_usize 2) lo mid mid hi;
    lemma_stored_frame s mid3 out3 start (mk_usize 3) lo mid mid hi;
    lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
    lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
    lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
    lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_union e0 mid0 out0 lo mid hi;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_union e1 mid1 out1 lo mid hi;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_union e2 mid2 out2 lo mid hi;
    Libcrux_sha3.Proof_utils.lemma_modifies_range_union e3 mid3 out3 lo mid hi
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
