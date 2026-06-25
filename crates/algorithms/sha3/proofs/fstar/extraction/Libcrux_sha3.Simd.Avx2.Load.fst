module Libcrux_sha3.Simd.Avx2.Load
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

/// Spec function (mirrors arm64::load_lane_u64 at N=4): per-lane
/// semantics of \"XOR state element with 8 bytes from input block\".
let load_lane_u64
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (statei: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (lane: usize)
    : Prims.Pure u64
      (requires
        i <. mk_usize 25 && lane <. mk_usize 4 &&
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
  (Libcrux_intrinsics.Avx2_extract.get_lane_u64 statei lane <: u64) ^.
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

val load_lane_u64_lane_extensionality
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (s1 s2: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (lane: usize)
  : Lemma
    (requires
      (i <. mk_usize 25 && lane <. mk_usize 4 &&
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
         Hax_lib.Int.t_Int)) /\
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s1 lane ==
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 lane)
    (ensures
      load_lane_u64 blocks offset i s1 lane ==
      load_lane_u64 blocks offset i s2 lane)
    [SMTPat (load_lane_u64 blocks offset i s1 lane);
     SMTPat (load_lane_u64 blocks offset i s2 lane)]

let load_lane_u64_lane_extensionality
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (s1 s2: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (lane: usize)
  : Lemma
    (requires
      (i <. mk_usize 25 && lane <. mk_usize 4 &&
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
         Hax_lib.Int.t_Int)) /\
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s1 lane ==
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 lane)
    (ensures
      load_lane_u64 blocks offset i s1 lane ==
      load_lane_u64 blocks offset i s2 lane)
    [SMTPat (load_lane_u64 blocks offset i s1 lane);
     SMTPat (load_lane_u64 blocks offset i s2 lane)]
  = reveal_opaque (`%load_lane_u64) load_lane_u64

#push-options "--z3rlimit 400 --split_queries always"

[@@ "opaque_to_smt"]

/// Bulk-block load helper (mirrors arm64::load_u64x2x2 at N=4).
/// Loads 32 bytes from each of the 4 blocks at `offset + 32*i`,
/// gathers them via unpack/permute into 4 Vec256s, each holding the
/// `(4*i + idx)`th u64 from each block in lane `lane`, then XORs
/// with the corresponding state inputs `inK`.
let load_u64x4x4
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (in0 in1 in2 in3: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Prims.Pure
      (Libcrux_intrinsics.Avx2_extract.t_Vec256 & Libcrux_intrinsics.Avx2_extract.t_Vec256 &
        Libcrux_intrinsics.Avx2_extract.t_Vec256 &
        Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (requires
        i <. mk_usize 6 &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 3 ] <: t_Slice u8) <: usize) &&
        (((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
            ((Rust_primitives.Hax.Int.from_machine (mk_i32 32) <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine (mk_i32 32) <: Hax_lib.Int.t_Int)
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
          (r0: Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (r1: Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (r2: Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (r3: Libcrux_intrinsics.Avx2_extract.t_Vec256) =
            temp_0_
          in
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r0 (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks offset (mk_usize 4 *! i <: usize) in0 (mk_usize 0) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r0 (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks offset (mk_usize 4 *! i <: usize) in0 (mk_usize 1) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r0 (mk_usize 2) <: u64) =.
          (load_lane_u64 blocks offset (mk_usize 4 *! i <: usize) in0 (mk_usize 2) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r0 (mk_usize 3) <: u64) =.
          (load_lane_u64 blocks offset (mk_usize 4 *! i <: usize) in0 (mk_usize 3) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r1 (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
              in1
              (mk_usize 0)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r1 (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
              in1
              (mk_usize 1)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r1 (mk_usize 2) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
              in1
              (mk_usize 2)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r1 (mk_usize 3) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
              in1
              (mk_usize 3)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r2 (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
              in2
              (mk_usize 0)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r2 (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
              in2
              (mk_usize 1)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r2 (mk_usize 2) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
              in2
              (mk_usize 2)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r2 (mk_usize 3) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
              in2
              (mk_usize 3)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r3 (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
              in3
              (mk_usize 0)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r3 (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
              in3
              (mk_usize 1)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r3 (mk_usize 2) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
              in3
              (mk_usize 2)
            <:
            u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 r3 (mk_usize 3) <: u64) =.
          (load_lane_u64 blocks
              offset
              ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
              in3
              (mk_usize 3)
            <:
            u64)) =
  let _:Prims.unit = reveal_opaque (`%load_lane_u64) load_lane_u64 in
  let start:usize = offset +! (mk_usize 32 *! i <: usize) in
  let v0:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_loadu_si256_u8 ((blocks.[ mk_usize 0 ] <: t_Slice u8).[ {
            Core_models.Ops.Range.f_start = start;
            Core_models.Ops.Range.f_end = start +! mk_usize 32 <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
  in
  let v1:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_loadu_si256_u8 ((blocks.[ mk_usize 1 ] <: t_Slice u8).[ {
            Core_models.Ops.Range.f_start = start;
            Core_models.Ops.Range.f_end = start +! mk_usize 32 <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
  in
  let v2:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_loadu_si256_u8 ((blocks.[ mk_usize 2 ] <: t_Slice u8).[ {
            Core_models.Ops.Range.f_start = start;
            Core_models.Ops.Range.f_end = start +! mk_usize 32 <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
  in
  let v3:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_loadu_si256_u8 ((blocks.[ mk_usize 3 ] <: t_Slice u8).[ {
            Core_models.Ops.Range.f_start = start;
            Core_models.Ops.Range.f_end = start +! mk_usize 32 <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
  in
  let v0l:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi64 v0 v1
  in
  let v1h:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 v0 v1
  in
  let v2l:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpacklo_epi64 v2 v3
  in
  let v3h:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_unpackhi_epi64 v2 v3
  in
  let g0:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 32) v0l v2l
  in
  let g1:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 32) v1h v3h
  in
  let g2:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 49) v0l v2l
  in
  let g3:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_permute2x128_si256 (mk_i32 49) v1h v3h
  in
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 in0 g0,
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 in1 g1,
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 in2 g2,
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 in3 g3
  <:
  (Libcrux_intrinsics.Avx2_extract.t_Vec256 & Libcrux_intrinsics.Avx2_extract.t_Vec256 &
    Libcrux_intrinsics.Avx2_extract.t_Vec256 &
    Libcrux_intrinsics.Avx2_extract.t_Vec256)

#pop-options

[@@ "opaque_to_smt"]

/// Partial-block load helper (mirrors arm64::load_u64x2 at N=4).
/// Loads 8 bytes from each of the 4 blocks at `offset + 8*i`,
/// gathers them into a Vec256, and XORs with `statei`.
let load_u64x4
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (statei: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Prims.Pure Libcrux_intrinsics.Avx2_extract.t_Vec256
      (requires
        i <. mk_usize 25 &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 3 ] <: t_Slice u8) <: usize) &&
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
          let result:Libcrux_intrinsics.Avx2_extract.t_Vec256 = result in
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 result (mk_usize 0) <: u64) =.
          (load_lane_u64 blocks offset i statei (mk_usize 0) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 result (mk_usize 1) <: u64) =.
          (load_lane_u64 blocks offset i statei (mk_usize 1) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 result (mk_usize 2) <: u64) =.
          (load_lane_u64 blocks offset i statei (mk_usize 2) <: u64) &&
          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 result (mk_usize 3) <: u64) =.
          (load_lane_u64 blocks offset i statei (mk_usize 3) <: u64)) =
  let _:Prims.unit = reveal_opaque (`%load_lane_u64) load_lane_u64 in
  let v0:i64 =
    cast (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
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
    <:
    i64
  in
  let v1:i64 =
    cast (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
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
    <:
    i64
  in
  let v2:i64 =
    cast (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
                  (mk_usize 8))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8)
                  #(t_Array u8 (mk_usize 8))
                  #FStar.Tactics.Typeclasses.solve
                  ((blocks.[ mk_usize 2 ] <: t_Slice u8).[ {
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
    <:
    i64
  in
  let v3:i64 =
    cast (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
                  (mk_usize 8))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8)
                  #(t_Array u8 (mk_usize 8))
                  #FStar.Tactics.Typeclasses.solve
                  ((blocks.[ mk_usize 3 ] <: t_Slice u8).[ {
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
    <:
    i64
  in
  let u:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_intrinsics.Avx2_extract.mm256_set_epi64x v3 v2 v1 v0
  in
  Libcrux_intrinsics.Avx2_extract.mm256_xor_si256 statei u

let lemma_rate_mod (rate: usize)
    : Prims.Pure Prims.unit
      (requires Libcrux_sha3.Proof_utils.valid_rate rate)
      (ensures
        fun temp_0_ ->
          let _:Prims.unit = temp_0_ in
          ((rate %! mk_usize 32 <: usize) =. mk_usize 8 ||
          (rate %! mk_usize 32 <: usize) =. mk_usize 16) &&
          (if (rate %! mk_usize 32 <: usize) =. mk_usize 16
            then
              (rate /! mk_usize 8 <: usize) =.
              ((mk_usize 4 *! (rate /! mk_usize 32 <: usize) <: usize) +! mk_usize 2 <: usize)
            else
              (rate /! mk_usize 8 <: usize) =.
              ((mk_usize 4 *! (rate /! mk_usize 32 <: usize) <: usize) +! mk_usize 1 <: usize))) =
  ()

#push-options "--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'"

let load_block
      (v_RATE: usize)
      (state: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset: usize)
    : Prims.Pure (t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 3 ] <: t_Slice u8) <: usize) &&
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
          let state_future:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
            state_future
          in
          forall (i: usize).
            b2t
            (if i <. mk_usize 25 <: bool
              then
                if i <. (v_RATE /! mk_usize 8 <: usize) <: bool
                then
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        i
                        (state.[ i ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        i
                        (state.[ i ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        i
                        (state.[ i ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        i
                        (state.[ i ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64)
                    <:
                    bool)
                else
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state_future.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ i ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64)
                    <:
                    bool)
              else true)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((v_RATE <=.
              (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize)
              <:
              bool) &&
            ((v_RATE /! mk_usize 32 <: usize) <=. mk_usize 6 <: bool) &&
            (((mk_usize 32 *! ((v_RATE /! mk_usize 32 <: usize) -! mk_usize 1 <: usize) <: usize) +!
                mk_usize 32
                <:
                usize) <=.
              v_RATE
              <:
              bool) &&
            ((v_RATE %! mk_usize 8 <: usize) =. mk_usize 0 <: bool) &&
            (((v_RATE %! mk_usize 32 <: usize) =. mk_usize 8 <: bool) ||
            ((v_RATE %! mk_usize 32 <: usize) =. mk_usize 16 <: bool)))
      in
      ()
  in
  let old_state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) = state in
  let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (v_RATE /! mk_usize 32 <: usize)
      (fun state i ->
          let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) = state in
          let i:usize = i in
          forall (j: usize).
            b2t
            (if j <. mk_usize 25 <: bool
              then
                if j <. (mk_usize 4 *! i <: usize) <: bool
                then
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        j
                        (old_state.[ j ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        j
                        (old_state.[ j ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        j
                        (old_state.[ j ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64) =.
                    (load_lane_u64 blocks
                        offset
                        j
                        (old_state.[ j ] <: Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64)
                    <:
                    bool)
                else
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (old_state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 0)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (old_state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 1)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (old_state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 2)
                      <:
                      u64)
                    <:
                    bool) &&
                  ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64) =.
                    (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (old_state.[ j ]
                          <:
                          Libcrux_intrinsics.Avx2_extract.t_Vec256)
                        (mk_usize 3)
                      <:
                      u64)
                    <:
                    bool)
              else true))
      state
      (fun state i ->
          let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) = state in
          let i:usize = i in
          let i0:usize = (mk_usize 4 *! i <: usize) /! mk_usize 5 in
          let j0:usize = (mk_usize 4 *! i <: usize) %! mk_usize 5 in
          let i1:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize) /! mk_usize 5 in
          let j1:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize) %! mk_usize 5 in
          let i2:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize) /! mk_usize 5 in
          let j2:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize) %! mk_usize 5 in
          let i3:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize) /! mk_usize 5 in
          let j3:usize = ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize) %! mk_usize 5 in
          let _:Prims.unit =
            assert (v v_RATE / 32 > 0);
            assert (v i <= v v_RATE / 32 - 1);
            assert (v i < 6);
            assert (v i + 1 <= v v_RATE / 32);
            assert ((v v_RATE / 32) * 32 <= v v_RATE);
            assert (32 * (v i + 1) <= v v_RATE);
            assert (32 * v i + 32 <= v v_RATE);
            assert (sz 32 *! i +! sz 32 <=. v_RATE)
          in
          let
          (g0: Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (g1: Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (g2: Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (g3: Libcrux_intrinsics.Avx2_extract.t_Vec256) =
            load_u64x4x4 blocks
              offset
              i
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  state
                  i0
                  j0
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  state
                  i1
                  j1
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  state
                  i2
                  j2
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
              (Libcrux_sha3.Traits.get_ij (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  state
                  i3
                  j3
                <:
                Libcrux_intrinsics.Avx2_extract.t_Vec256)
          in
          let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              state
              i0
              j0
              g0
          in
          let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              state
              i1
              j1
              g1
          in
          let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              state
              i2
              j2
              g2
          in
          let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
            Libcrux_sha3.Traits.set_ij (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              state
              i3
              j3
              g3
          in
          let _:Prims.unit =
            Hax_lib.v_assert (((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ mk_usize 4 *!
                          i
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      (mk_usize 4 *! i <: usize)
                      (old_state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      (mk_usize 4 *! i <: usize)
                      (old_state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      (mk_usize 4 *! i <: usize)
                      (old_state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      (mk_usize 4 *! i <: usize)
                      (old_state.[ mk_usize 4 *! i <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64)
                  <:
                  bool))
          in
          let _:Prims.unit =
            Hax_lib.v_assert (((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *!
                            i
                            <:
                            usize) +!
                          mk_usize 1
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 1
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 1
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 1
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 1 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64)
                  <:
                  bool))
          in
          let _:Prims.unit =
            Hax_lib.v_assert (((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *!
                            i
                            <:
                            usize) +!
                          mk_usize 2
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 2
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 2
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 2
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 2 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64)
                  <:
                  bool))
          in
          let _:Prims.unit =
            Hax_lib.v_assert (((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *!
                            i
                            <:
                            usize) +!
                          mk_usize 3
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 0)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 3
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 1)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 3
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 2)
                    <:
                    u64)
                  <:
                  bool) &&
                ((Libcrux_intrinsics.Avx2_extract.get_lane_u64 (state.[ (mk_usize 4 *! i <: usize) +!
                          mk_usize 3
                          <:
                          usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64) =.
                  (load_lane_u64 blocks
                      offset
                      ((mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize)
                      (old_state.[ (mk_usize 4 *! i <: usize) +! mk_usize 3 <: usize ]
                        <:
                        Libcrux_intrinsics.Avx2_extract.t_Vec256)
                      (mk_usize 3)
                    <:
                    u64)
                  <:
                  bool))
          in
          state)
  in
  let _:Prims.unit = lemma_rate_mod v_RATE in
  let rem:usize = v_RATE %! mk_usize 32 in
  let i:usize = mk_usize 4 *! (v_RATE /! mk_usize 32 <: usize) in
  let result:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    load_u64x4 blocks
      offset
      i
      (Libcrux_sha3.Traits.get_ij (mk_usize 4)
          #Libcrux_intrinsics.Avx2_extract.t_Vec256
          state
          (i /! mk_usize 5 <: usize)
          (i %! mk_usize 5 <: usize)
        <:
        Libcrux_intrinsics.Avx2_extract.t_Vec256)
  in
  let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
    Libcrux_sha3.Traits.set_ij (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      state
      (i /! mk_usize 5 <: usize)
      (i %! mk_usize 5 <: usize)
      result
  in
  let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
    if rem =. mk_usize 16
    then
      let i:usize = (mk_usize 4 *! (v_RATE /! mk_usize 32 <: usize) <: usize) +! mk_usize 1 in
      let result:Libcrux_intrinsics.Avx2_extract.t_Vec256 =
        load_u64x4 blocks
          offset
          i
          (Libcrux_sha3.Traits.get_ij (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              state
              (i /! mk_usize 5 <: usize)
              (i %! mk_usize 5 <: usize)
            <:
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      in
      let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
        Libcrux_sha3.Traits.set_ij (mk_usize 4)
          #Libcrux_intrinsics.Avx2_extract.t_Vec256
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
      (state: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (start len: usize)
    : Prims.Pure (t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <. v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
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
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (blocks.[ mk_usize 3 ] <: t_Slice u8) <: usize))
      (fun _ -> Prims.l_True) =
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
  let buffer1:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer1 len v_DELIMITER
  in
  let buffer1:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer1
      (v_RATE -! mk_usize 1 <: usize)
      ((buffer1.[ v_RATE -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  let buffer2:t_Array u8 v_RATE = Rust_primitives.Hax.repeat (mk_u8 0) v_RATE in
  let buffer2:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range buffer2
      ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = len }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (buffer2.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = len
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          ((blocks.[ mk_usize 2 ] <: t_Slice u8).[ {
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
  let buffer2:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer2 len v_DELIMITER
  in
  let buffer2:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer2
      (v_RATE -! mk_usize 1 <: usize)
      ((buffer2.[ v_RATE -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  let buffer3:t_Array u8 v_RATE = Rust_primitives.Hax.repeat (mk_u8 0) v_RATE in
  let buffer3:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range buffer3
      ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = len }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (buffer3.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = len
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          ((blocks.[ mk_usize 3 ] <: t_Slice u8).[ {
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
  let buffer3:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer3 len v_DELIMITER
  in
  let buffer3:t_Array u8 v_RATE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer3
      (v_RATE -! mk_usize 1 <: usize)
      ((buffer3.[ v_RATE -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  let state:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
    load_block v_RATE
      state
      (let list =
          [
            buffer0 <: t_Slice u8;
            buffer1 <: t_Slice u8;
            buffer2 <: t_Slice u8;
            buffer3 <: t_Slice u8
          ]
        in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
        Rust_primitives.Hax.array_of_list 4 list)
      (mk_usize 0)
  in
  state

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_sha3.Traits.t_Absorb
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256)
  (mk_usize 4) =
  {
    f_load_block_pre
    =
    (fun
        (v_RATE: usize)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (input: t_Array (t_Slice u8) (mk_usize 4))
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
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 3 ] <: t_Slice u8) <: usize));
    f_load_block_post
    =
    (fun
        (v_RATE: usize)
        (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (input: t_Array (t_Slice u8) (mk_usize 4))
        (start: usize)
        (out:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_load_block
    =
    (fun
        (v_RATE: usize)
        (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (input: t_Array (t_Slice u8) (mk_usize 4))
        (start: usize)
        ->
        let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 =
          {
            self with
            Libcrux_sha3.Generic_keccak.f_st
            =
            load_block v_RATE self.Libcrux_sha3.Generic_keccak.f_st input start
          }
          <:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256
        in
        self);
    f_load_last_pre
    =
    (fun
        (v_RATE: usize)
        (v_DELIMITER: u8)
        (self_:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (input: t_Array (t_Slice u8) (mk_usize 4))
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
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (input.[ mk_usize 3 ] <: t_Slice u8) <: usize));
    f_load_last_post
    =
    (fun
        (v_RATE: usize)
        (v_DELIMITER: u8)
        (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        (input: t_Array (t_Slice u8) (mk_usize 4))
        (start: usize)
        (len: usize)
        (out:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
        ->
        true);
    f_load_last
    =
    fun
      (v_RATE: usize)
      (v_DELIMITER: u8)
      (self:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (input: t_Array (t_Slice u8) (mk_usize 4))
      (start: usize)
      (len: usize)
      ->
      let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
        Libcrux_intrinsics.Avx2_extract.t_Vec256 =
        {
          self with
          Libcrux_sha3.Generic_keccak.f_st
          =
          load_last v_RATE v_DELIMITER self.Libcrux_sha3.Generic_keccak.f_st input start len
        }
        <:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256
      in
      self
  }
