module Libcrux_sha3.Generic_keccak.Portable
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Portable in
  let open Libcrux_sha3.Traits in
  ()

#push-options "--z3rlimit 800"

let e_keccak_state_impl_opts (_: Prims.unit) : Prims.unit = ()

let impl__squeeze_next_block
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
      (start: usize)
    : Prims.Pure (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out <: usize)
          <:
          Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64),
          (out_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      start
      v_RATE
  in
  self, out <: (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)

let impl__squeeze_first_block
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        v_RATE <=. (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 0)
      v_RATE
  in
  out

let impl__squeeze_first_three_blocks
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (mk_usize 3 *! v_RATE <: usize) <=. (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64),
          (out_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 0)
      v_RATE
  in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      v_RATE
      v_RATE
  in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 2 *! v_RATE <: usize)
      v_RATE
  in
  self, out <: (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)

/// Final partial-block step of the squeeze phase: if `output_rem != 0`,
/// apply one Keccak-f permutation and then extract the trailing
/// `output_rem` bytes of output into the tail of `out`; otherwise
/// a no-op.  Factored out of `squeeze` so the final post-condition
/// reconciling impl vs spec is proved within a small dedicated VC.
let impl__squeeze_last
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
      (output_rem: usize)
    : Prims.Pure (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 out <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize) &&
        output_rem <. v_RATE &&
        output_rem <=. (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64),
          (out_future: t_Slice u8) =
            temp_0_
          in
          b2t
          ((Core_models.Slice.impl__len #u8 out_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out <: usize)
            <:
            bool) /\
          (let out_len = Core_models.Slice.impl__len #u8 out in
            let prefix_len:usize = out_len -! output_rem in
            ((output_rem =. mk_usize 0) ==>
              self_e_future.Libcrux_sha3.Generic_keccak.f_st ==
              self.Libcrux_sha3.Generic_keccak.f_st) /\
            ((output_rem <>. mk_usize 0) ==>
              self_e_future.Libcrux_sha3.Generic_keccak.f_st ==
              Hacspec_sha3.Keccak_f.keccak_f self.Libcrux_sha3.Generic_keccak.f_st) /\
            (forall (k: nat).
                k < v prefix_len ==>
                Seq.index (out_future <: Seq.seq u8) k == Seq.index (out <: Seq.seq u8) k) /\
            (forall (k: nat).
                v prefix_len <= k /\ k < v out_len ==>
                Seq.index (out_future <: Seq.seq u8) k ==
                ((Core_models.Num.impl_u64__to_le_bytes (self_e_future
                          .Libcrux_sha3.Generic_keccak.f_st.[ (mk_usize (k - v prefix_len) /!
                            mk_usize 8)
                          <:
                          usize ]
                        <:
                        u64)
                    <:
                    t_Array u8 (mk_usize 8)).[ (mk_usize (k - v prefix_len) %! mk_usize 8) <: usize
                  ]
                  <:
                  u8)))) =
  let out_original:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Slice.impl__to_vec #u8 out in
  let self_original_st:t_Array u64 (mk_usize 25) = self.Libcrux_sha3.Generic_keccak.f_st in
  let (out: t_Slice u8), (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64) =
    if output_rem <>. mk_usize 0
    then
      let _:Prims.unit = EquivImplSpec.Keccakf.Portable.lemma_keccakf1600_portable self in
      let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
        Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
      in
      let out:t_Slice u8 =
        Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
          #u64
          #FStar.Tactics.Typeclasses.solve
          v_RATE
          self
          out
          ((Core_models.Slice.impl__len #u8 out <: usize) -! output_rem <: usize)
          output_rem
      in
      let _:Prims.unit =
        let out_len = Core_models.Slice.impl__len #u8 out in
        let offset = out_len -! output_rem in
        let out_orig_slice:t_Slice u8 = Alloc.Vec.impl_1__as_slice out_original in
        let new_state = Hacspec_sha3.Keccak_f.keccak_f self_original_st in
        assert (v v_RATE <= 200);
        assert (self.Libcrux_sha3.Generic_keccak.f_st == new_state);
        let aux_prefix (k: nat{k < v offset})
            : Lemma (Seq.index (out <: Seq.seq u8) k == Seq.index (out_orig_slice <: Seq.seq u8) k)
        =
          let i:usize = mk_usize k in
          assert (v i < v offset)
        in
        FStar.Classical.forall_intro aux_prefix;
        let aux_tail (k: nat{v offset <= k /\ k < v out_len})
            : Lemma
            (let j:usize = mk_usize (k - v offset) in
              Seq.index (out <: Seq.seq u8) k ==
              ((Core_models.Num.impl_u64__to_le_bytes (new_state.[ j /! mk_usize 8 <: usize ] <: u64
                    )
                  <:
                  t_Array u8 (mk_usize 8)).[ j %! mk_usize 8 <: usize ]
                <:
                u8)) =
          let i:usize = mk_usize k in
          let j:usize = mk_usize (k - v offset) in
          assert (v i - v offset < v output_rem);
          assert ((v i - v offset) / 8 < 25);
          assert (v j == v i - v offset)
        in
        FStar.Classical.forall_intro aux_tail
      in
      out, self <: (t_Slice u8 & Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
    else out, self <: (t_Slice u8 & Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
  in
  self, out <: (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)

let impl__squeeze_first_five_blocks
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (mk_usize 5 *! v_RATE <: usize) <=. (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun temp_0_ ->
          let
          (self_e_future: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64),
          (out_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 0)
      v_RATE
  in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      v_RATE
      v_RATE
  in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 2 *! v_RATE <: usize)
      v_RATE
  in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 3 *! v_RATE <: usize)
      v_RATE
  in
  let self:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 self
  in
  let out:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      out
      (mk_usize 4 *! v_RATE <: usize)
      v_RATE
  in
  self, out <: (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always"

/// Absorb phase of `keccak1`: initialise a Keccak state, absorb all full
/// rate-byte blocks of `input`, then pad and absorb the final partial block
/// with domain-separation byte `DELIM` and the pad10*1 terminator.
/// The ensures clause asserts direct equality with the spec function
/// `Hacspec_sha3.Sponge.absorb`. The loop invariant uses the spec helper
/// `absorb_blocks` (block-indexed analogue of `absorb_rec`, avoiding the
/// slice-of-slice reasoning that triggers a Z3 4.13.3 LP-solver bug in
/// older proofs based on `absorb_rec` recursion).
let absorb (v_RATE: usize) (v_DELIM: u8) (input: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (requires Libcrux_sha3.Proof_utils.valid_rate v_RATE)
      (ensures
        fun result ->
          let result:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 = result in
          result.Libcrux_sha3.Generic_keccak.f_st == Hacspec_sha3.Sponge.absorb v_RATE v_DELIM input
      ) =
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__new (mk_usize 1) #u64 ()
  in
  let input_len:usize = Core_models.Slice.impl__len #u8 input in
  let input_blocks:usize = input_len /! v_RATE in
  let input_rem:usize = input_len %! v_RATE in
  let _:Prims.unit =
    let zeros:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base zeros v_RATE (mk_usize 0) input
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      input_blocks
      (fun s i ->
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 = s in
          let i:usize = i in
          let zeros:t_Array u64 (mk_usize 25) =
            Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25)
          in
          v i <= v input_blocks /\
          s.Libcrux_sha3.Generic_keccak.f_st ==
          Hacspec_sha3.Sponge.absorb_blocks zeros v_RATE (mk_usize 0) i input)
      s
      (fun s i ->
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 = s in
          let i:usize = i in
          let _:Prims.unit =
            Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i input_blocks v_RATE
          in
          let _:Prims.unit =
            let zeros:t_Array u64 (mk_usize 25) =
              Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25)
            in
            let inputs:t_Array (t_Slice u8) (mk_usize 1) =
              let list = [input] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list
            in
            assert (inputs.[ mk_usize 0 ] == input);
            EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable v_RATE
              s
              inputs
              (i *! v_RATE);
            Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail zeros
              v_RATE
              (mk_usize 0)
              i
              (i +! mk_usize 1)
              input
          in
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
            Libcrux_sha3.Generic_keccak.impl_2__absorb_block (mk_usize 1)
              #u64
              v_RATE
              s
              (let list = [input] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
              (i *! v_RATE <: usize)
          in
          s)
  in
  let _:Prims.unit =
    let zeros:t_Array u64 (mk_usize 25) = Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    let inputs:t_Array (t_Slice u8) (mk_usize 1) =
      let list = [input] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
      Rust_primitives.Hax.array_of_list 1 list
    in
    assert (inputs.[ mk_usize 0 ] == input);
    EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_last_portable v_RATE
      v_DELIM
      s
      inputs
      (input_len -! input_rem)
      input_rem;
    Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks zeros v_RATE v_DELIM input
  in
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
    Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 1)
      #u64
      v_RATE
      v_DELIM
      s
      (let list = [input] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
        Rust_primitives.Hax.array_of_list 1 list)
      (input_len -! input_rem <: usize)
      input_rem
  in
  s

#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"

let squeeze_blocks
      (v_RATE: usize)
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (output: t_Slice u8)
      (output_blocks: usize)
    : Prims.Pure (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE && output_blocks >. mk_usize 0 &&
        output_blocks =. ((Core_models.Slice.impl__len #u8 output <: usize) /! v_RATE <: usize) &&
        (Core_models.Slice.impl__len #u8 output <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64),
          (output_future: t_Slice u8) =
            temp_0_
          in
          b2t
          ((Core_models.Slice.impl__len #u8 output_future <: usize) =.
            (Core_models.Slice.impl__len #u8 output <: usize)
            <:
            bool) /\
          (let spec_out:t_Slice u8 =
              Hacspec_sha3.Sponge.squeeze (Core_models.Slice.impl__len #u8 output)
                s.Libcrux_sha3.Generic_keccak.f_st
                v_RATE
            in
            s_future.f_st == Hacspec_sha3.Sponge.iterate_keccak_f (output_blocks -! sz 1) s.f_st /\
            (forall (k: nat).
                (k < v output_blocks * v v_RATE) ==>
                Seq.index output_future k == Seq.index spec_out k))) =
  let output_len:usize = Core_models.Slice.impl__len #u8 output in
  let s_init_st:t_Array u64 (mk_usize 25) = s.Libcrux_sha3.Generic_keccak.f_st in
  let output_initial:t_Slice u8 =
    Alloc.Vec.impl_1__as_slice #u8
      #Alloc.Alloc.t_Global
      (Alloc.Slice.impl__to_vec #u8 output <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let output:t_Slice u8 =
    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      s
      output
      (mk_usize 0)
      v_RATE
  in
  let _:Prims.unit =
    let spec_out:t_Slice u8 = Hacspec_sha3.Sponge.squeeze output_len s_init_st v_RATE in
    assert (v output_blocks >= 1);
    assert (v v_RATE <= 200);
    assert (s_init_st == Hacspec_sha3.Sponge.iterate_keccak_f (mk_usize 0) s_init_st);
    let aux (k: nat{k < v v_RATE})
        : Lemma (Seq.index (output <: Seq.seq u8) k == Seq.index (spec_out <: Seq.seq u8) k) =
      let i:usize = mk_usize k in
      assert (v i == k);
      assert (v i / 8 < 25);
      FStar.Math.Lemmas.small_div k (v v_RATE);
      assert (v i / v v_RATE = 0)
    in
    FStar.Classical.forall_intro aux
  in
  let (output: t_Slice u8), (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 1)
      output_blocks
      (fun temp_0_ i ->
          let (output: t_Slice u8), (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
          =
            temp_0_
          in
          let i:usize = i in
          b2t ((Core_models.Slice.impl__len #u8 output <: usize) =. output_len <: bool) /\
          (let spec_out:t_Slice u8 = Hacspec_sha3.Sponge.squeeze output_len s_init_st v_RATE in
            v i >= 1 /\ v i <= v output_blocks /\ v i * v v_RATE <= v output_len /\
            s.Libcrux_sha3.Generic_keccak.f_st ==
            Hacspec_sha3.Sponge.iterate_keccak_f (i -! mk_usize 1) s_init_st /\
            (forall (k: nat).
                (k < v i * v v_RATE) ==>
                Seq.index (output <: Seq.seq u8) k == Seq.index (spec_out <: Seq.seq u8) k)))
      (output, s <: (t_Slice u8 & Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64))
      (fun temp_0_ i ->
          let (output: t_Slice u8), (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
          =
            temp_0_
          in
          let i:usize = i in
          let _:Prims.unit =
            Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i output_blocks v_RATE
          in
          let _:Prims.unit =
            Libcrux_sha3.Proof_utils.Lemmas.lemma_div_mul_mod output_len v_RATE;
            assert (v i * v v_RATE + v v_RATE <= v output_len);
            assert (v output_blocks == v output_len / v v_RATE);
            assert (v v_RATE >= 1);
            assert (v i * v v_RATE <= v output_len);
            Math.Lemmas.nat_times_nat_is_nat (v i) (v v_RATE);
            assert (v output_len < v Core_models.Num.impl_usize__MAX);
            assert (v i * v v_RATE < v Core_models.Num.impl_usize__MAX);
            EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_one_step_portable v_RATE
              s_init_st
              s
              output
              output_initial
              i
          in
          let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
            Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 s
          in
          let _:Prims.unit = assert (s.f_st == Hacspec_sha3.Sponge.iterate_keccak_f i s_init_st) in
          let output:t_Slice u8 =
            Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1)
                  u64)
              #u64
              #FStar.Tactics.Typeclasses.solve
              v_RATE
              s
              output
              (i *! v_RATE <: usize)
              v_RATE
          in
          let _:Prims.unit =
            assert (v i * v v_RATE + v v_RATE <= v output_len);
            Math.Lemmas.distributivity_add_left (v i) 1 (v v_RATE)
          in
          output, s <: (t_Slice u8 & Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64))
  in
  s, output <: (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 & t_Slice u8)

#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"

/// Squeeze phase of `keccak1`: extract `output.len()` bytes from `s`,
/// applying Keccak-f between each full rate-byte block of output.
/// The ensures clause pins the result to the byteform spec
/// `Hacspec_sha3.Sponge.squeeze` (per-byte: byte k uses
/// `iterate_keccak_f(k/RATE, s_init)`\'s lane).  Body proof:
///   - establish the byteform invariant after the first block
///   - cite `lemma_squeeze_one_step_portable` per loop iteration to
///     advance the invariant from i to i+1
///   - reconcile the trailing partial block (via `squeeze_last`) with
///     the byteform\'s last block.
///
let squeeze
      (v_RATE: usize)
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (output: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 output <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun output_future ->
          let output_future:t_Slice u8 = output_future in
          b2t
          ((Core_models.Slice.impl__len #u8 output_future <: usize) =.
            (Core_models.Slice.impl__len #u8 output <: usize)
            <:
            bool) /\
          (output_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.squeeze (Core_models.Slice.impl__len #u8 output)
              s.Libcrux_sha3.Generic_keccak.f_st
              v_RATE
            <:
            t_Slice u8)) =
  let output_len:usize = Core_models.Slice.impl__len #u8 output in
  let output_blocks:usize = output_len /! v_RATE in
  let output_rem:usize = output_len %! v_RATE in
  let s_init_st:t_Array u64 (mk_usize 25) = s.Libcrux_sha3.Generic_keccak.f_st in
  let (output: t_Slice u8), (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64) =
    if output_blocks =. mk_usize 0
    then
      let output:t_Slice u8 =
        Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
          #u64
          #FStar.Tactics.Typeclasses.solve
          v_RATE
          s
          output
          (mk_usize 0)
          output_len
      in
      let _:Prims.unit =
        let spec_out:t_Slice u8 = Hacspec_sha3.Sponge.squeeze output_len s_init_st v_RATE in
        assert (v output_len < v v_RATE);
        assert (v v_RATE <= 200);
        let aux (k: nat{k < v output_len})
            : Lemma (Seq.index (output <: Seq.seq u8) k == Seq.index (spec_out <: Seq.seq u8) k) =
          let i:usize = mk_usize k in
          assert (v i == k);
          assert (v i / 8 < 25);
          FStar.Math.Lemmas.small_div k (v v_RATE);
          assert (v i / v v_RATE = 0)
        in
        FStar.Classical.forall_intro aux;
        Seq.lemma_eq_intro (output <: Seq.seq u8) (spec_out <: Seq.seq u8)
      in
      output, s <: (t_Slice u8 & Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
    else
      let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64), (tmp1: t_Slice u8) =
        squeeze_blocks v_RATE s output output_blocks
      in
      let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 = tmp0 in
      let output:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      let output_after_blocks:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
        Alloc.Slice.impl__to_vec #u8 output
      in
      let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64), (tmp1: t_Slice u8) =
        impl__squeeze_last v_RATE s output output_rem
      in
      let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 = tmp0 in
      let output:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      let _:Prims.unit =
        Math.Lemmas.lemma_div_mod (v output_len) (v v_RATE);
        let spec_out:t_Slice u8 = Hacspec_sha3.Sponge.squeeze output_len s_init_st v_RATE in
        assert (v output_blocks >= 1);
        assert (Hacspec_sha3.Sponge.iterate_keccak_f output_blocks s_init_st ==
            Hacspec_sha3.Keccak_f.keccak_f (Hacspec_sha3.Sponge.iterate_keccak_f (output_blocks -!
                    mk_usize 1)
                  s_init_st));
        let output_after_blocks_slice:t_Slice u8 = Alloc.Vec.impl_1__as_slice output_after_blocks in
        let aux (k: nat{k < v output_len})
            : Lemma (Seq.index (output <: Seq.seq u8) k == Seq.index (spec_out <: Seq.seq u8) k) =
          if k < v output_len - v output_rem
          then
            EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_prefix_preserved_portable v_RATE
              s_init_st
              output_after_blocks_slice
              output
              output_blocks
              output_rem
              k
          else
            (assert (v output_rem > 0);
              assert (s.Libcrux_sha3.Generic_keccak.f_st ==
                  Hacspec_sha3.Sponge.iterate_keccak_f output_blocks s_init_st);
              EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_trailing_byteform_portable v_RATE
                s_init_st
                s.Libcrux_sha3.Generic_keccak.f_st
                output
                output_blocks
                output_rem
                k)
        in
        FStar.Classical.forall_intro aux;
        Seq.lemma_eq_intro (output <: Seq.seq u8) (spec_out <: Seq.seq u8)
      in
      output, s <: (t_Slice u8 & Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
  in
  output

#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"

let keccak1 (v_RATE: usize) (v_DELIM: u8) (input output: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE &&
        (Core_models.Slice.impl__len #u8 output <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun output_future ->
          let output_future:t_Slice u8 = output_future in
          b2t
          ((Core_models.Slice.impl__len #u8 output_future <: usize) =.
            (Core_models.Slice.impl__len #u8 output <: usize)
            <:
            bool) /\
          (output_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 output) v_RATE v_DELIM input
            <:
            t_Slice u8)) =
  let s:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 = absorb v_RATE v_DELIM input in
  let output:t_Slice u8 = squeeze v_RATE s output in
  output

#pop-options
