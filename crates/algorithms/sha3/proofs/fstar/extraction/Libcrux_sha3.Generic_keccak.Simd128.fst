module Libcrux_sha3.Generic_keccak.Simd128
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Arm64.Load in
  let open Libcrux_sha3.Simd.Arm64.Store in
  let open Libcrux_sha3.Simd.Arm64.Wrappers in
  let open Libcrux_sha3.Traits in
  ()

#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always"

/// Absorb phase of `keccak2`: initialise a two-lane Keccak state,
/// absorb all full rate-byte blocks of `data[0]` and `data[1]` in
/// parallel, then pad and absorb each lane\'s final partial block
/// with domain-separation byte `DELIM` and the pad10*1 terminator.
/// The ensures clause asserts per-lane equality with the scalar spec
/// function `Hacspec_sha3.Sponge.absorb`.  The loop invariant uses
/// `absorb_blocks` per lane, mirroring the Portable backend.
let absorb2 (v_RATE: usize) (v_DELIM: u8) (data: t_Array (t_Slice u8) (mk_usize 2))
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 1 ] <: t_Slice u8) <: usize))
      (ensures
        fun result ->
          let result:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
            result
          in
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64
              result.Libcrux_sha3.Generic_keccak.f_st
              0) ==
          Hacspec_sha3.Sponge.absorb v_RATE
            v_DELIM
            (Core_models.Ops.Index.f_index data (mk_usize 0)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64
              result.Libcrux_sha3.Generic_keccak.f_st
              1) ==
          Hacspec_sha3.Sponge.absorb v_RATE
            v_DELIM
            (Core_models.Ops.Index.f_index data (mk_usize 1))) =
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__new (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      ()
  in
  let data_len:usize = Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) in
  let data_blocks:usize = data_len /! v_RATE in
  let rem:usize = data_len %! v_RATE in
  let _:Prims.unit =
    let zeros:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    EquivImplSpec.Keccakf.Arm64.lemma_extract_lane_zero_arm64 0;
    EquivImplSpec.Keccakf.Arm64.lemma_extract_lane_zero_arm64 1;
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros
      v_RATE
      (mk_usize 0)
      (Core_models.Ops.Index.f_index data (mk_usize 0));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros
      v_RATE
      (mk_usize 0)
      (Core_models.Ops.Index.f_index data (mk_usize 1))
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      data_blocks
      (fun s i ->
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
            s
          in
          let i:usize = i in
          let zeros:t_Array u64 (mk_usize 25) =
            Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25)
          in
          v i <= v data_blocks /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64
              s.Libcrux_sha3.Generic_keccak.f_st
              0) ==
          Hacspec_sha3.Sponge.absorb_blocks zeros
            v_RATE
            (mk_usize 0)
            i
            (Core_models.Ops.Index.f_index data (mk_usize 0)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64
              s.Libcrux_sha3.Generic_keccak.f_st
              1) ==
          Hacspec_sha3.Sponge.absorb_blocks zeros
            v_RATE
            (mk_usize 0)
            i
            (Core_models.Ops.Index.f_index data (mk_usize 1)))
      s
      (fun s i ->
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
            s
          in
          let i:usize = i in
          let _:Prims.unit =
            Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i data_blocks v_RATE
          in
          let _:Prims.unit =
            let zeros:t_Array u64 (mk_usize 25) =
              Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25)
            in
            EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_block_arm64 v_RATE s data (i *! v_RATE) 0;
            EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_block_arm64 v_RATE s data (i *! v_RATE) 1;
            Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail zeros
              v_RATE
              (mk_usize 0)
              i
              (i +! mk_usize 1)
              (Core_models.Ops.Index.f_index data (mk_usize 0));
            Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail zeros
              v_RATE
              (mk_usize 0)
              i
              (i +! mk_usize 1)
              (Core_models.Ops.Index.f_index data (mk_usize 1))
          in
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
            Libcrux_sha3.Generic_keccak.impl_2__absorb_block (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              v_RATE
              s
              data
              (i *! v_RATE <: usize)
          in
          s)
  in
  let _:Prims.unit =
    let zeros:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_last_arm64 v_RATE
      v_DELIM
      s
      data
      (data_len -! rem)
      rem
      0;
    EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_last_arm64 v_RATE
      v_DELIM
      s
      data
      (data_len -! rem)
      rem
      1;
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros
      v_RATE
      v_DELIM
      (Core_models.Ops.Index.f_index data (mk_usize 0));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros
      v_RATE
      v_DELIM
      (Core_models.Ops.Index.f_index data (mk_usize 1))
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      v_RATE
      v_DELIM
      s
      data
      (data_len -! rem <: usize)
      rem
  in
  s

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane'"

let squeeze2_blocks
      (v_RATE: usize)
      (s:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
      (blocks: usize)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        blocks >. mk_usize 0 &&
        blocks =. ((Core_models.Slice.impl__len #u8 out0 <: usize) /! v_RATE <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8) =
            temp_0_
          in
          b2t
          (((Core_models.Slice.impl__len #u8 out0_future <: usize) =.
              (Core_models.Slice.impl__len #u8 out0 <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 out1_future <: usize) =.
              (Core_models.Slice.impl__len #u8 out1 <: usize)
              <:
              bool)) /\
          (let outlen = Core_models.Slice.impl__len #u8 out0 in
            v outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
            ((EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64
                  s_future.Libcrux_sha3.Generic_keccak.f_st
                  0) ==
              Hacspec_sha3.Sponge.iterate_keccak_f (blocks -! mk_usize 1)
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s.Libcrux_sha3.Generic_keccak.f_st
                    0) /\
              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64
                  s_future.Libcrux_sha3.Generic_keccak.f_st
                  1) ==
              Hacspec_sha3.Sponge.iterate_keccak_f (blocks -! mk_usize 1)
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s.Libcrux_sha3.Generic_keccak.f_st
                    1) /\
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.squeezed_upto (out0_future <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze outlen
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s.Libcrux_sha3.Generic_keccak.f_st
                        0)
                    v_RATE
                  <:
                  Seq.seq u8)
                (v blocks * v v_RATE) /\
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.squeezed_upto (out1_future <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze outlen
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s.Libcrux_sha3.Generic_keccak.f_st
                        1)
                    v_RATE
                  <:
                  Seq.seq u8)
                (v blocks * v v_RATE)))) =
  let out0_len:usize = Core_models.Slice.impl__len #u8 out0 in
  let out1_len:usize = Core_models.Slice.impl__len #u8 out1 in
  let s_init_st:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
    s.Libcrux_sha3.Generic_keccak.f_st
  in
  let outlen:usize = Core_models.Slice.impl__len #u8 out0 in
  let _:Prims.unit =
    if v outlen < v Core_models.Num.impl_usize__MAX - 200
    then
      ((if v outlen < v v_RATE then FStar.Math.Lemmas.small_div (v outlen) (v v_RATE));
        assert (v v_RATE <= v outlen);
        EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_iterate_keccak_f_zero (EquivImplSpec.Keccakf.Generic.extract_lane
              (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64
              s_init_st
              0);
        EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_iterate_keccak_f_zero (EquivImplSpec.Keccakf.Generic.extract_lane
              (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64
              s_init_st
              1);
        EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_first_driver_arm64 v_RATE
          s
          out0
          out1
          v_RATE)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      s
      out0
      out1
      (mk_usize 0)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let
  (out0: t_Slice u8),
  (out1: t_Slice u8),
  (s:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 1)
      blocks
      (fun temp_0_ i ->
          let
          (out0: t_Slice u8),
          (out1: t_Slice u8),
          (s:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) =
            temp_0_
          in
          let i:usize = i in
          b2t
          (((Core_models.Slice.impl__len #u8 out0 <: usize) =. out0_len <: bool) &&
            ((Core_models.Slice.impl__len #u8 out1 <: usize) =. out1_len <: bool)) /\
          (v i >= 1) /\ (v i <= v blocks) /\
          (v outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
            (v i * v v_RATE <= v outlen /\
              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64
                  s.Libcrux_sha3.Generic_keccak.f_st
                  0) ==
              Hacspec_sha3.Sponge.iterate_keccak_f (i -! mk_usize 1)
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s_init_st
                    0) /\
              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64
                  s.Libcrux_sha3.Generic_keccak.f_st
                  1) ==
              Hacspec_sha3.Sponge.iterate_keccak_f (i -! mk_usize 1)
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s_init_st
                    1) /\
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.squeezed_upto (out0 <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze (Core_models.Slice.impl__len #u8 out0)
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s_init_st
                        0)
                    v_RATE
                  <:
                  Seq.seq u8)
                (v i * v v_RATE) /\
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.squeezed_upto (out1 <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze (Core_models.Slice.impl__len #u8 out1)
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s_init_st
                        1)
                    v_RATE
                  <:
                  Seq.seq u8)
                (v i * v v_RATE))))
      (out0, out1, s
        <:
        (t_Slice u8 & t_Slice u8 &
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t))
      (fun temp_0_ i ->
          let
          (out0: t_Slice u8),
          (out1: t_Slice u8),
          (s:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) =
            temp_0_
          in
          let i:usize = i in
          let _:Prims.unit = Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i blocks v_RATE in
          let _:Prims.unit =
            if v outlen < v Core_models.Num.impl_usize__MAX - 200
            then
              (Libcrux_sha3.Proof_utils.Lemmas.lemma_div_mul_mod outlen v_RATE;
                EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_mid_driver_arm64 v_RATE
                  s_init_st
                  s
                  out0
                  out1
                  i)
          in
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
            Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              s
          in
          let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
            Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
                  Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
              #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
              #FStar.Tactics.Typeclasses.solve
              v_RATE
              s
              out0
              out1
              (i *! v_RATE <: usize)
              v_RATE
          in
          let out0:t_Slice u8 = tmp0 in
          let out1:t_Slice u8 = tmp1 in
          let _:Prims.unit = () in
          out0, out1, s
          <:
          (t_Slice u8 & t_Slice u8 &
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t))
  in
  s, out0, out1
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
    t_Slice u8 &
    t_Slice u8)

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane'"

/// Squeeze phase of `keccak2`: extract `out0.len()` bytes from each
/// lane of `s` into `out0` and `out1`, applying Keccak-f between
/// each full rate-byte block of output.  Mirrors the Portable `squeeze`
/// shape: branch on `blocks==0`, delegate the full-blocks loop to
/// `squeeze2_blocks`, handle the trailing partial block, and discharge
/// the full `Seq`-equality functional spec via `lemma_squeezed_upto_full`
/// — closing within each branch so no VC carries both the loop and the
/// byteform `squeeze` equality.
let squeeze2
      (v_RATE: usize)
      (s:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize))
      (ensures
        fun temp_0_ ->
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
          b2t
          (((Core_models.Slice.impl__len #u8 out0_future <: usize) =.
              (Core_models.Slice.impl__len #u8 out0 <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 out1_future <: usize) =.
              (Core_models.Slice.impl__len #u8 out1 <: usize)
              <:
              bool)) /\
          (let outlen = Core_models.Slice.impl__len #u8 out0 in
            v outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
            (out0_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s.Libcrux_sha3.Generic_keccak.f_st
                    0)
                v_RATE
              <:
              t_Slice u8) /\
            (out1_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s.Libcrux_sha3.Generic_keccak.f_st
                    1)
                v_RATE
              <:
              t_Slice u8))) =
  let s_init_st:t_Array Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t (mk_usize 25) =
    s.Libcrux_sha3.Generic_keccak.f_st
  in
  let outlen:usize = Core_models.Slice.impl__len #u8 out0 in
  let blocks:usize = outlen /! v_RATE in
  let last:usize = outlen -! (outlen %! v_RATE <: usize) in
  let
  (out0: t_Slice u8),
  (out1: t_Slice u8),
  (s:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t) =
    if blocks =. mk_usize 0
    then
      let _:Prims.unit =
        if v outlen < v Core_models.Num.impl_usize__MAX - 200
        then
          EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_first_driver_arm64 v_RATE
            s
            out0
            out1
            outlen
      in
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
        Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
          #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
          #FStar.Tactics.Typeclasses.solve
          v_RATE
          s
          out0
          out1
          (mk_usize 0)
          outlen
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      let _:Prims.unit =
        if v outlen < v Core_models.Num.impl_usize__MAX - 200
        then
          (EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_length outlen
              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64
                  s_init_st
                  0)
              v_RATE;
            EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_length outlen
              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64
                  s_init_st
                  1)
              v_RATE;
            EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeezed_upto_full (out0 <: Seq.seq u8)
              (Hacspec_sha3.Sponge.squeeze outlen
                  (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                      EquivImplSpec.Keccakf.Arm64.lc_arm64
                      s_init_st
                      0)
                  v_RATE
                <:
                Seq.seq u8);
            EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeezed_upto_full (out1 <: Seq.seq u8)
              (Hacspec_sha3.Sponge.squeeze outlen
                  (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                      EquivImplSpec.Keccakf.Arm64.lc_arm64
                      s_init_st
                      1)
                  v_RATE
                <:
                Seq.seq u8))
      in
      out0, out1, s
      <:
      (t_Slice u8 & t_Slice u8 &
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
    else
      let
      (tmp0:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
      (tmp1: t_Slice u8),
      (tmp2: t_Slice u8) =
        squeeze2_blocks v_RATE s out0 out1 blocks
      in
      let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
        Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
        tmp0
      in
      let out0:t_Slice u8 = tmp1 in
      let out1:t_Slice u8 = tmp2 in
      let _:Prims.unit = () in
      if last <. outlen
      then
        let _:Prims.unit =
          if v outlen < v Core_models.Num.impl_usize__MAX - 200
          then
            (Math.Lemmas.lemma_div_mod (v outlen) (v v_RATE);
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_blocks_rate_split outlen v_RATE;
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_tail_driver_arm64 v_RATE
                s_init_st
                s
                out0
                out1
                blocks)
        in
        let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
            #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
            s
        in
        let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
          Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
                Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
            #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
            #FStar.Tactics.Typeclasses.solve
            v_RATE
            s
            out0
            out1
            last
            (outlen -! last <: usize)
        in
        let out0:t_Slice u8 = tmp0 in
        let out1:t_Slice u8 = tmp1 in
        let _:Prims.unit = () in
        let _:Prims.unit =
          if v outlen < v Core_models.Num.impl_usize__MAX - 200
          then
            (EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_length outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s_init_st
                    0)
                v_RATE;
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_length outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s_init_st
                    1)
                v_RATE;
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeezed_upto_full (out0 <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze outlen
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s_init_st
                        0)
                    v_RATE
                  <:
                  Seq.seq u8);
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeezed_upto_full (out1 <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze outlen
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s_init_st
                        1)
                    v_RATE
                  <:
                  Seq.seq u8))
        in
        out0, out1, s
        <:
        (t_Slice u8 & t_Slice u8 &
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      else
        let _:Prims.unit =
          if v outlen < v Core_models.Num.impl_usize__MAX - 200
          then
            (EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_exact_multiple outlen v_RATE;
              assert (v blocks * v v_RATE == v outlen);
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_length outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s_init_st
                    0)
                v_RATE;
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeeze_length outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                    EquivImplSpec.Keccakf.Arm64.lc_arm64
                    s_init_st
                    1)
                v_RATE;
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeezed_upto_full (out0 <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze outlen
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s_init_st
                        0)
                    v_RATE
                  <:
                  Seq.seq u8);
              EquivImplSpec.Sponge.Arm64.SqueezeDriver.lemma_squeezed_upto_full (out1 <: Seq.seq u8)
                (Hacspec_sha3.Sponge.squeeze outlen
                    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                        EquivImplSpec.Keccakf.Arm64.lc_arm64
                        s_init_st
                        1)
                    v_RATE
                  <:
                  Seq.seq u8))
        in
        out0, out1, s
        <:
        (t_Slice u8 & t_Slice u8 &
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
  in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

#pop-options

/// Two-lane Keccak driver.  The function-level ensures keep only the
/// bounds (length-preservation) here; the per-lane functional spec is
/// proved at the Neon-wrapper level (in `src/neon.rs`) using
/// `EquivImplSpec.Sponge.Arm64.API.lemma_keccak2_arm64`, which composes
/// `lemma_absorb2_arm64` + `lemma_squeeze2_arm64`.  Wiring the
/// per-lane functional spec here would create a circular dependency:
/// `EquivImplSpec.Sponge.Arm64.API` depends on this module\'s
/// `absorb2`, `squeeze2`, `keccak2`, so it cannot itself be cited
/// from this module\'s body.
let keccak2
      (v_RATE: usize)
      (v_DELIM: u8)
      (data: t_Array (t_Slice u8) (mk_usize 2))
      (out0 out1: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 1 ] <: t_Slice u8) <: usize))
      (ensures
        fun temp_0_ ->
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 out0 <: usize) =.
            (Core_models.Slice.impl__len #u8 out1 <: usize)
            <:
            bool)
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8)
              <:
              usize) =.
            (Core_models.Slice.impl__len #u8 (data.[ mk_usize 1 ] <: t_Slice u8) <: usize)
            <:
            bool)
      in
      ()
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    absorb2 v_RATE v_DELIM data
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) = squeeze2 v_RATE s out0 out1 in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

let impl__squeeze_next_block
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
      (start: usize)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize)) =
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      start
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  self, out0, out1
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
    t_Slice u8 &
    t_Slice u8)

/// Write out the first block of Keccak output.
/// This function MUST NOT be called after any of the other `squeeze_*`
/// functions have been called, since that would result in a duplicate output
/// block.
let impl__squeeze_first_block
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        v_RATE <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize))
      (ensures
        fun temp_0_ ->
          let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = temp_0_ in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize)) =
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 0)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  out0, out1 <: (t_Slice u8 & t_Slice u8)

let impl__squeeze_first_three_blocks
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (mk_usize 3 *! v_RATE <: usize) <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize)) =
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 0)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      v_RATE
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 2 *! v_RATE <: usize)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  self, out0, out1
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
    t_Slice u8 &
    t_Slice u8)

let impl__squeeze_first_five_blocks
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
            Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (mk_usize 5 *! v_RATE <: usize) <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
              Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize)) =
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 0)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      v_RATE
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 2 *! v_RATE <: usize)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 3 *! v_RATE <: usize)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
    Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze2 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
          Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      #Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out0
      out1
      (mk_usize 4 *! v_RATE <: usize)
      v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  self, out0, out1
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2)
      Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t &
    t_Slice u8 &
    t_Slice u8)
