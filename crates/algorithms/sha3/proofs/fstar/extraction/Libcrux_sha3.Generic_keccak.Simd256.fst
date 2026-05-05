module Libcrux_sha3.Generic_keccak.Simd256
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Avx2 in
  let open Libcrux_sha3.Traits in
  ()

#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always"

/// Absorb phase of `keccak4`: initialise a four-lane Keccak state,
/// absorb all full rate-byte blocks of `data[0..4]` in parallel,
/// then pad and absorb each lane\'s final partial block with
/// domain-separation byte `DELIM` and the pad10*1 terminator.
/// The ensures clause asserts per-lane equality with the scalar spec
/// function `Hacspec_sha3.Sponge.absorb`.  The loop invariant uses
/// `absorb_blocks` per lane, mirroring the Arm64 backend at N=2.
let absorb4 (v_RATE: usize) (v_DELIM: u8) (data: t_Array (t_Slice u8) (mk_usize 4))
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 3 ] <: t_Slice u8) <: usize))
      (ensures
        fun result ->
          let result:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            result
          in
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              result.Libcrux_sha3.Generic_keccak.f_st
              0) ==
          Hacspec_sha3.Sponge.absorb v_RATE
            v_DELIM
            (Core_models.Ops.Index.f_index data (mk_usize 0)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              result.Libcrux_sha3.Generic_keccak.f_st
              1) ==
          Hacspec_sha3.Sponge.absorb v_RATE
            v_DELIM
            (Core_models.Ops.Index.f_index data (mk_usize 1)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              result.Libcrux_sha3.Generic_keccak.f_st
              2) ==
          Hacspec_sha3.Sponge.absorb v_RATE
            v_DELIM
            (Core_models.Ops.Index.f_index data (mk_usize 2)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              result.Libcrux_sha3.Generic_keccak.f_st
              3) ==
          Hacspec_sha3.Sponge.absorb v_RATE
            v_DELIM
            (Core_models.Ops.Index.f_index data (mk_usize 3))) =
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__new (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      ()
  in
  let data_len:usize = Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) in
  let data_blocks:usize = data_len /! v_RATE in
  let rem:usize = data_len %! v_RATE in
  let _:Prims.unit =
    let zeros:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 0;
    EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 1;
    EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 2;
    EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 3;
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros
      v_RATE
      (mk_usize 0)
      (Core_models.Ops.Index.f_index data (mk_usize 0));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros
      v_RATE
      (mk_usize 0)
      (Core_models.Ops.Index.f_index data (mk_usize 1));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros
      v_RATE
      (mk_usize 0)
      (Core_models.Ops.Index.f_index data (mk_usize 2));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros
      v_RATE
      (mk_usize 0)
      (Core_models.Ops.Index.f_index data (mk_usize 3))
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      data_blocks
      (fun s i ->
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            s
          in
          let i:usize = i in
          let zeros:t_Array u64 (mk_usize 25) =
            Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25)
          in
          v i <= v data_blocks /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              s.Libcrux_sha3.Generic_keccak.f_st
              0) ==
          Hacspec_sha3.Sponge.absorb_blocks zeros
            v_RATE
            (mk_usize 0)
            i
            (Core_models.Ops.Index.f_index data (mk_usize 0)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              s.Libcrux_sha3.Generic_keccak.f_st
              1) ==
          Hacspec_sha3.Sponge.absorb_blocks zeros
            v_RATE
            (mk_usize 0)
            i
            (Core_models.Ops.Index.f_index data (mk_usize 1)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              s.Libcrux_sha3.Generic_keccak.f_st
              2) ==
          Hacspec_sha3.Sponge.absorb_blocks zeros
            v_RATE
            (mk_usize 0)
            i
            (Core_models.Ops.Index.f_index data (mk_usize 2)) /\
          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2
              s.Libcrux_sha3.Generic_keccak.f_st
              3) ==
          Hacspec_sha3.Sponge.absorb_blocks zeros
            v_RATE
            (mk_usize 0)
            i
            (Core_models.Ops.Index.f_index data (mk_usize 3)))
      s
      (fun s i ->
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256 =
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
            assert (Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) data);
            EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2 v_RATE s data (i *! v_RATE) 0;
            EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2 v_RATE s data (i *! v_RATE) 1;
            EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2 v_RATE s data (i *! v_RATE) 2;
            EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2 v_RATE s data (i *! v_RATE) 3;
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
              (Core_models.Ops.Index.f_index data (mk_usize 1));
            Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail zeros
              v_RATE
              (mk_usize 0)
              i
              (i +! mk_usize 1)
              (Core_models.Ops.Index.f_index data (mk_usize 2));
            Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail zeros
              v_RATE
              (mk_usize 0)
              i
              (i +! mk_usize 1)
              (Core_models.Ops.Index.f_index data (mk_usize 3))
          in
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256 =
            Libcrux_sha3.Generic_keccak.impl_2__absorb_block (mk_usize 4)
              #Libcrux_intrinsics.Avx2_extract.t_Vec256
              v_RATE
              s
              data
              (i *! v_RATE <: usize)
          in
          s)
  in
  let _:Prims.unit =
    let zeros:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    assert (Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) data);
    EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2 v_RATE
      v_DELIM
      s
      data
      (data_len -! rem)
      rem
      0;
    EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2 v_RATE
      v_DELIM
      s
      data
      (data_len -! rem)
      rem
      1;
    EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2 v_RATE
      v_DELIM
      s
      data
      (data_len -! rem)
      rem
      2;
    EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2 v_RATE
      v_DELIM
      s
      data
      (data_len -! rem)
      rem
      3;
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros
      v_RATE
      v_DELIM
      (Core_models.Ops.Index.f_index data (mk_usize 0));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros
      v_RATE
      v_DELIM
      (Core_models.Ops.Index.f_index data (mk_usize 1));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros
      v_RATE
      v_DELIM
      (Core_models.Ops.Index.f_index data (mk_usize 2));
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros
      v_RATE
      v_DELIM
      (Core_models.Ops.Index.f_index data (mk_usize 3))
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      v_RATE
      v_DELIM
      s
      data
      (data_len -! rem <: usize)
      rem
  in
  s

#pop-options

#push-options "--z3rlimit 600 --split_queries always"

/// Squeeze phase of `keccak4`: extract `out0.len()` bytes from each
/// lane of `s` into `out0..out3`, applying Keccak-f between each
/// full rate-byte block of output.
/// **Per-lane spec-equivalence ensures NOT YET proved.**  The
/// monolithic inline-ensures approach (mirror of Portable.squeeze at
/// N=4) hits the same Z3 BoxBool cascade documented in the
/// 2026-04-25 squeeze2 post-mortem; at N=4 it is even worse (8 forall
/// conjuncts in the loop invariant vs 4 at N=2).  See HANDOFF.md
/// \"2026-04-25 (later)\" section for the path forward via per-lane
/// `Sponge.Avx2.Steps` lemmas (Option B).  Until that lands,
/// `lemma_squeeze4_avx2` in `EquivImplSpec.Sponge.Avx2.API.fst`
/// remains an `assume val`.  The `squeeze_last4` helper above and the
/// `out0.len() < usize::MAX - 200` precondition propagation up to
/// `keccak4` and `avx2::shake256` are kept — they are infrastructure
/// for the eventual proof.
let squeeze4
      (v_RATE: usize)
      (s:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize) &&
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
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let out0_len:usize = Core_models.Slice.impl__len #u8 out0 in
  let out1_len:usize = Core_models.Slice.impl__len #u8 out1 in
  let out2_len:usize = Core_models.Slice.impl__len #u8 out2 in
  let out3_len:usize = Core_models.Slice.impl__len #u8 out3 in
  let outlen:usize = Core_models.Slice.impl__len #u8 out0 in
  let blocks:usize = outlen /! v_RATE in
  let last:usize = outlen -! (outlen %! v_RATE <: usize) in
  let
  (out0: t_Slice u8),
  (out1: t_Slice u8),
  (out2: t_Slice u8),
  (out3: t_Slice u8),
  (s:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256)
  =
    if blocks =. mk_usize 0
    then
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
        Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
          #FStar.Tactics.Typeclasses.solve v_RATE s out0 out1 out2 out3 (mk_usize 0) outlen
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let out2:t_Slice u8 = tmp2 in
      let out3:t_Slice u8 = tmp3 in
      let _:Prims.unit = () in
      out0, out1, out2, out3, s
      <:
      (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256)
    else
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
        Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
          #FStar.Tactics.Typeclasses.solve v_RATE s out0 out1 out2 out3 (mk_usize 0) v_RATE
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let out2:t_Slice u8 = tmp2 in
      let out3:t_Slice u8 = tmp3 in
      let _:Prims.unit = () in
      let
      (out0: t_Slice u8),
      (out1: t_Slice u8),
      (out2: t_Slice u8),
      (out3: t_Slice u8),
      (s:
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) =
        Rust_primitives.Hax.Folds.fold_range (mk_usize 1)
          blocks
          (fun temp_0_ temp_1_ ->
              let
              (out0: t_Slice u8),
              (out1: t_Slice u8),
              (out2: t_Slice u8),
              (out3: t_Slice u8),
              (s:
                Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                  Libcrux_intrinsics.Avx2_extract.t_Vec256) =
                temp_0_
              in
              let _:usize = temp_1_ in
              ((Core_models.Slice.impl__len #u8 out0 <: usize) =. out0_len <: bool) &&
              ((Core_models.Slice.impl__len #u8 out1 <: usize) =. out1_len <: bool) &&
              ((Core_models.Slice.impl__len #u8 out2 <: usize) =. out2_len <: bool) &&
              ((Core_models.Slice.impl__len #u8 out3 <: usize) =. out3_len <: bool))
          (out0, out1, out2, out3, s
            <:
            (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
              Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                Libcrux_intrinsics.Avx2_extract.t_Vec256))
          (fun temp_0_ i ->
              let
              (out0: t_Slice u8),
              (out1: t_Slice u8),
              (out2: t_Slice u8),
              (out3: t_Slice u8),
              (s:
                Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                  Libcrux_intrinsics.Avx2_extract.t_Vec256) =
                temp_0_
              in
              let i:usize = i in
              let _:Prims.unit =
                Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i blocks v_RATE
              in
              let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                Libcrux_intrinsics.Avx2_extract.t_Vec256 =
                Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256
                  s
              in
              let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
                Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState
                      (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256)
                  #Libcrux_intrinsics.Avx2_extract.t_Vec256 #FStar.Tactics.Typeclasses.solve v_RATE
                  s out0 out1 out2 out3 (i *! v_RATE <: usize) v_RATE
              in
              let out0:t_Slice u8 = tmp0 in
              let out1:t_Slice u8 = tmp1 in
              let out2:t_Slice u8 = tmp2 in
              let out3:t_Slice u8 = tmp3 in
              let _:Prims.unit = () in
              out0, out1, out2, out3, s
              <:
              (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
                Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                  Libcrux_intrinsics.Avx2_extract.t_Vec256))
      in
      if last <. outlen
      then
        let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
            #Libcrux_intrinsics.Avx2_extract.t_Vec256
            s
        in
        let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
          Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
            #FStar.Tactics.Typeclasses.solve v_RATE s out0 out1 out2 out3 last
            (outlen -! last <: usize)
        in
        let out0:t_Slice u8 = tmp0 in
        let out1:t_Slice u8 = tmp1 in
        let out2:t_Slice u8 = tmp2 in
        let out3:t_Slice u8 = tmp3 in
        let _:Prims.unit = () in
        out0, out1, out2, out3, s
        <:
        (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      else
        out0, out1, out2, out3, s
        <:
        (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
  in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#pop-options

let keccak4
      (v_RATE: usize)
      (v_DELIM: u8)
      (data: t_Array (t_Slice u8) (mk_usize 4))
      (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize) &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 1 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 2 ] <: t_Slice u8) <: usize) &&
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
        (Core_models.Slice.impl__len #u8 (data.[ mk_usize 3 ] <: t_Slice u8) <: usize))
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
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert (((Core_models.Slice.impl__len #u8 out0 <: usize) =.
              (Core_models.Slice.impl__len #u8 out1 <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 out0 <: usize) =.
              (Core_models.Slice.impl__len #u8 out2 <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 out0 <: usize) =.
              (Core_models.Slice.impl__len #u8 out3 <: usize)
              <:
              bool))
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert (((Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8)
                <:
                usize) =.
              (Core_models.Slice.impl__len #u8 (data.[ mk_usize 1 ] <: t_Slice u8) <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
              (Core_models.Slice.impl__len #u8 (data.[ mk_usize 2 ] <: t_Slice u8) <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 (data.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
              (Core_models.Slice.impl__len #u8 (data.[ mk_usize 3 ] <: t_Slice u8) <: usize)
              <:
              bool))
      in
      ()
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    absorb4 v_RATE v_DELIM data
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    squeeze4 v_RATE s out0 out1 out2 out3
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always"

let e_keccak_state_impl4_opts (_: Prims.unit) : Prims.unit = ()

/// Trailing partial-block squeeze for 4-lane state.  If
/// `output_rem != 0` apply one Keccak-f permutation and squeeze
/// the trailing `output_rem` bytes into each lane\'s output;
/// otherwise a no-op.  Mirrors the Portable
/// `KeccakState<1,u64>::squeeze_last` factor-out so the final
/// reconcile in `squeeze4` lands in a small VC.
let impl__squeeze_last4
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (output_rem: usize)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 &
        t_Slice u8 &
        t_Slice u8 &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize) &&
        output_rem <. v_RATE &&
        output_rem <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
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
              bool) &&
            ((Core_models.Slice.impl__len #u8 out2_future <: usize) =.
              (Core_models.Slice.impl__len #u8 out2 <: usize)
              <:
              bool) &&
            ((Core_models.Slice.impl__len #u8 out3_future <: usize) =.
              (Core_models.Slice.impl__len #u8 out3 <: usize)
              <:
              bool)) /\
          (let out_len = Core_models.Slice.impl__len #u8 out0 in
            let lane_st_pre (l: nat{l < 4}) =
              EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                EquivImplSpec.Keccakf.Avx2.lc_avx2
                self.Libcrux_sha3.Generic_keccak.f_st
                l
            in
            let lane_st_post (l: nat{l < 4}) =
              EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                EquivImplSpec.Keccakf.Avx2.lc_avx2
                self_e_future.Libcrux_sha3.Generic_keccak.f_st
                l
            in
            let st_spec_0, out_spec_0 =
              Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 0) out0 v_RATE output_rem
            in
            let st_spec_1, out_spec_1 =
              Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 1) out1 v_RATE output_rem
            in
            let st_spec_2, out_spec_2 =
              Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 2) out2 v_RATE output_rem
            in
            let st_spec_3, out_spec_3 =
              Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 3) out3 v_RATE output_rem
            in
            lane_st_post 0 == st_spec_0 /\ (out0_future <: Seq.seq u8) == (out_spec_0 <: Seq.seq u8) /\
            lane_st_post 1 == st_spec_1 /\ (out1_future <: Seq.seq u8) == (out_spec_1 <: Seq.seq u8) /\
            lane_st_post 2 == st_spec_2 /\ (out2_future <: Seq.seq u8) == (out_spec_2 <: Seq.seq u8) /\
            lane_st_post 3 == st_spec_3 /\ (out3_future <: Seq.seq u8) == (out_spec_3 <: Seq.seq u8)
          )) =
  let out0_original:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Slice.impl__to_vec #u8 out0 in
  let out1_original:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Slice.impl__to_vec #u8 out1 in
  let out2_original:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Slice.impl__to_vec #u8 out2 in
  let out3_original:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Slice.impl__to_vec #u8 out3 in
  let self_original_st:t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25) =
    self.Libcrux_sha3.Generic_keccak.f_st
  in
  let
  (out0: t_Slice u8),
  (out1: t_Slice u8),
  (out2: t_Slice u8),
  (out3: t_Slice u8),
  (self:
    Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256)
  =
    if output_rem <>. mk_usize 0
    then
      let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
        Libcrux_intrinsics.Avx2_extract.t_Vec256 =
        Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
          #Libcrux_intrinsics.Avx2_extract.t_Vec256
          self
      in
      let out_len:usize = Core_models.Slice.impl__len #u8 out0 in
      let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
        Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
          #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3
          (out_len -! output_rem <: usize) output_rem
      in
      let out0:t_Slice u8 = tmp0 in
      let out1:t_Slice u8 = tmp1 in
      let out2:t_Slice u8 = tmp2 in
      let out3:t_Slice u8 = tmp3 in
      let _:Prims.unit = () in
      let _:Prims.unit =
        let outputs:t_Array (t_Slice u8) (mk_usize 4) =
          let l:list (t_Slice u8) =
            [
              Alloc.Vec.impl_1__as_slice out0_original;
              Alloc.Vec.impl_1__as_slice out1_original;
              Alloc.Vec.impl_1__as_slice out2_original;
              Alloc.Vec.impl_1__as_slice out3_original
            ]
          in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length l) 4);
          Rust_primitives.Hax.array_of_list 4 l
        in
        let ks_pre:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 =
          { Libcrux_sha3.Generic_keccak.f_st = self_original_st }
        in
        let start:usize = (Core_models.Slice.impl__len #u8 out0) -! output_rem in
        EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2 v_RATE
          ks_pre
          outputs
          start
          output_rem
          0;
        EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2 v_RATE
          ks_pre
          outputs
          start
          output_rem
          1;
        EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2 v_RATE
          ks_pre
          outputs
          start
          output_rem
          2;
        EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2 v_RATE
          ks_pre
          outputs
          start
          output_rem
          3
      in
      out0, out1, out2, out3, self
      <:
      (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256)
    else
      out0, out1, out2, out3, self
      <:
      (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8 &
        Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256)
  in
  self, out0, out1, out2, out3
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8)

let impl__squeeze_next_block
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (start: usize)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 &
        t_Slice u8 &
        t_Slice u8 &
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
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256),
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
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3 start v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  self, out0, out1, out2, out3
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8)

/// Write out the first block of Keccak output.
/// This function MUST NOT be called after any of the other `squeeze_*`
/// functions have been called, since that would result in a duplicate output
/// block.
let impl__squeeze_first_block
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        v_RATE <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
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
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize)) =
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3 (mk_usize 0) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  out0, out1, out2, out3 <: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)

let impl__squeeze_first_three_blocks
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 &
        t_Slice u8 &
        t_Slice u8 &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (mk_usize 3 *! v_RATE <: usize) <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256),
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
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3 (mk_usize 0) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3 v_RATE v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3
      (mk_usize 2 *! v_RATE <: usize) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  self, out0, out1, out2, out3
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8)

let impl__squeeze_first_five_blocks
      (v_RATE: usize)
      (self:
          Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
            Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure
      (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256 &
        t_Slice u8 &
        t_Slice u8 &
        t_Slice u8 &
        t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (mk_usize 5 *! v_RATE <: usize) <=. (Core_models.Slice.impl__len #u8 out0 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future:
            Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
              Libcrux_intrinsics.Avx2_extract.t_Vec256),
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
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3 (mk_usize 0) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3 v_RATE v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3
      (mk_usize 2 *! v_RATE <: usize) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3
      (mk_usize 3 *! v_RATE <: usize) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
    Libcrux_intrinsics.Avx2_extract.t_Vec256 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
      #Libcrux_intrinsics.Avx2_extract.t_Vec256
      self
  in
  let (tmp0: t_Slice u8), (tmp1: t_Slice u8), (tmp2: t_Slice u8), (tmp3: t_Slice u8) =
    Libcrux_sha3.Traits.f_squeeze4 #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
          Libcrux_intrinsics.Avx2_extract.t_Vec256) #Libcrux_intrinsics.Avx2_extract.t_Vec256
      #FStar.Tactics.Typeclasses.solve v_RATE self out0 out1 out2 out3
      (mk_usize 4 *! v_RATE <: usize) v_RATE
  in
  let out0:t_Slice u8 = tmp0 in
  let out1:t_Slice u8 = tmp1 in
  let out2:t_Slice u8 = tmp2 in
  let out3:t_Slice u8 = tmp3 in
  let _:Prims.unit = () in
  self, out0, out1, out2, out3
  <:
  (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) Libcrux_intrinsics.Avx2_extract.t_Vec256 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8 &
    t_Slice u8)
