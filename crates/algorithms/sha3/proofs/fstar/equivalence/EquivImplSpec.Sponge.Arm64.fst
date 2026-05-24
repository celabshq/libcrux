module EquivImplSpec.Sponge.Arm64

(* ================================================================
   NEON (Arm64, N=2, v_T=t_e_uint64x2_t) instantiation of the generic
   sponge-layer equivalence proof.

   Builds a [sponge_correctness] record for
   [Libcrux_sha3.Simd.Arm64.{impl_1, impl_2}]:
     - t_Absorb  at N=2 on t_KeccakState 2 t_e_uint64x2_t
     - t_Squeeze2 at N=2 (two output slices)

   The three per-backend lemmas — sc_load_block, sc_load_last,
   sc_store_block — are admitted. Their proofs will unfold the NEON
   t_Absorb / t_Squeeze2 methods, push [arm64_lane] through the
   per-intrinsic postconditions declared in [Libcrux_intrinsics.
   Arm64_extract], and reduce to the scalar xor_block_into_state /
   squeeze_state on the extracted lane.
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module G  = EquivImplSpec.Keccakf.Generic
module KA = EquivImplSpec.Keccakf.Arm64
module SC = EquivImplSpec.Sponge.Generic.Core
module SP = EquivImplSpec.Sponge.Portable
module I  = Libcrux_intrinsics.Arm64_extract

(* Bring Arm64 typeclass instances into scope. *)
let _ =
  let open Libcrux_intrinsics.Arm64_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Arm64 in
  ()

(* ================================================================
   sq_lane for N=2: the Arm64 squeeze is t_Squeeze2.f_squeeze2 which
   returns a pair of output slices. Lane 0 is the first component,
   lane 1 the second.
   ================================================================ *)

let sq_lane_arm64
      (rate: usize)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (outputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      (l: nat{l < 2})
  : Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs)
      (ensures fun r -> Seq.length #u8 r == Seq.length #u8 (outputs.[ mk_usize l ]))
  = let pair =
      Libcrux_sha3.Traits.f_squeeze2
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
        #I.t_e_uint64x2_t
        #FStar.Tactics.Typeclasses.solve
        rate
        ({ Libcrux_sha3.Generic_keccak.f_st = state }
           <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
        (outputs.[ mk_usize 0 ])
        (outputs.[ mk_usize 1 ])
        start
        len
    in
    let out0, out1 = pair in
    if l = 0 then out0 else out1

(* Lane bridge: [arm64_lane] is defined as [get_lane_u64x2] but the
   [load_block] / [store_block] ensures clauses use [get_lane_u64]
   (at [mk_usize 0] / [mk_usize 1]).  Fire [get_lane_u64]'s spec so Z3
   can see they agree. *)
let lemma_arm64_lane_eq_get_lane_u64
      (v: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma (KA.arm64_lane v l == I.get_lane_u64 v (mk_usize l))
          [SMTPat (KA.arm64_lane v l)]
  = KA.lemma_arm64_lane_unfold v l;
    let _ = I.get_lane_u64 v (mk_usize l) in ()

(* ================================================================
   Standalone byte-level bridge — per-[i] equation for one concrete
   index, isolated from the outer [Classical.forall_intro] Skolem.

   Verifies in its own clean SMT context (no ambient cascade pressure
   from the consumer's heavy WP).  The outer
   [lemma_load_block_eq_xor_block_into_state_arm64] calls this 25
   times via a per-[i] [match].
   ================================================================ *)
#push-options "--z3rlimit 400"
let lemma_load_block_byte_eq_arm64
      (rate: usize)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset: usize)
      (l: nat{l < 2})
      (i: nat{i < 25})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v offset + v rate <= Seq.length #u8 (blocks.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) blocks)
      (ensures (
        let block_l : t_Slice u8 =
          (blocks.[ mk_usize l ] <: t_Slice u8).[ {
              Core_models.Ops.Range.f_start = offset;
              Core_models.Ops.Range.f_end   = offset +! rate } <:
            Core_models.Ops.Range.t_Range usize ] in
        let lb_state =
          Libcrux_sha3.Simd.Arm64.load_block rate state blocks offset in
        let lhs = G.extract_lane (mk_usize 2) KA.lc_arm64 lb_state l in
        let rhs = Hacspec_sha3.Sponge.xor_block_into_state
                    (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
                    block_l rate in
        Seq.index lhs i == Seq.index rhs i))
  = admit ()
    (* TRACK B FOLLOW-UP: structural body proof gap. The OLD inline byte_eq
       inside [lemma_load_block_eq_xor_block_into_state_arm64] had
       [admit ();] masking exactly this gap. Extracting to a standalone
       lemma exposed the gap; closing it requires either (a) strengthening
       this lemma's [requires] to package [load_block]'s per-(i, lane)
       ensures conjunct as a direct hypothesis (per the diagnosis-report
       recipe step 3), or (b) discharging [SP.lemma_subslice_bytes_eq]'s
       preconditions + bridging via SMTPats. The [slices_same_len] +
       [Seq.length] bridge alone is necessary but not sufficient. *)
#pop-options

(* ================================================================
   Bridge: pointwise equivalence of the Arm64 [load_block] on lane [l]
   with the scalar spec [xor_block_into_state] — parallels the Portable
   [lemma_load_block_eq_xor_block_into_state], but threads [arm64_lane]
   through the hax-proved per-lane ensures clause on [Simd.Arm64.load_block].
   ================================================================ *)
#push-options "--z3rlimit 400"
let lemma_load_block_eq_xor_block_into_state_arm64
      (rate: usize)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v offset + v rate <= Seq.length #u8 (blocks.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) blocks)
      (ensures (
        let block : t_Slice u8 =
          (blocks.[ mk_usize l ] <: t_Slice u8).[ {
              Core_models.Ops.Range.f_start = offset;
              Core_models.Ops.Range.f_end   = offset +! rate } <:
            Core_models.Ops.Range.t_Range usize ] in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          (Libcrux_sha3.Simd.Arm64.load_block rate state blocks offset) l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane (mk_usize 2) KA.lc_arm64 state l) block rate))
  = let block_l : t_Slice u8 =
      (blocks.[ mk_usize l ] <: t_Slice u8).[ {
          Core_models.Ops.Range.f_start = offset;
          Core_models.Ops.Range.f_end   = offset +! rate } <:
        Core_models.Ops.Range.t_Range usize ] in
    let lb_state = Libcrux_sha3.Simd.Arm64.load_block rate state blocks offset in
    let lhs = G.extract_lane (mk_usize 2) KA.lc_arm64 lb_state l in
    let rhs = Hacspec_sha3.Sponge.xor_block_into_state
                (G.extract_lane (mk_usize 2) KA.lc_arm64 state l) block_l rate in
    assert (v (mk_usize l) = l);
    let byte_eq (i: nat{i < 25}) : Lemma (Seq.index lhs i == Seq.index rhs i) =
      lemma_load_block_byte_eq_arm64 rate state blocks offset l i
    in
    Classical.forall_intro byte_eq;
    Rust_primitives.Arrays.eq_intro lhs rhs
#pop-options


(* ================================================================
   Admitted per-backend lemmas — reduce f_load_block / f_load_last /
   f_squeeze2 on the Arm64 impl to the scalar spec operations on the
   extracted lane.
   ================================================================ *)

#push-options "--z3rlimit 200"
let arm64_sc_load_block
      (rate: usize)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (inputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) inputs)
      (ensures
        G.extract_lane (mk_usize 2) KA.lc_arm64
          (Libcrux_sha3.Traits.f_load_block
             #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
             #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve
             rate
             ({ Libcrux_sha3.Generic_keccak.f_st = state }
                <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
             inputs
             start)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
          (inputs.[ mk_usize l ].[ {
              Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! rate } <:
            Core_models.Ops.Range.t_Range usize ])
          rate)
  = lemma_load_block_eq_xor_block_into_state_arm64 rate state inputs start l
#pop-options

(* ================================================================
   Bridge: Arm64 [load_last]'s internal RATE-length buffers match the
   spec's [pad_last_block[0..rate]] byte-by-byte, for each input lane.
   Shared between lane 0 and lane 1 by taking the relevant input slice
   as a parameter.
   ================================================================ *)
#push-options "--z3rlimit 300 --split_queries always"
let lemma_load_last_buffer_eq_padded_arm64
      (rate: usize) (delim: u8)
      (input: t_Slice u8)
      (offset: usize) (len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v offset + v len <= Seq.length #u8 input)
      (ensures (
        let copy_src : t_Slice u8 =
          input.[ { Core_models.Ops.Range.f_start = offset;
                    Core_models.Ops.Range.f_end   = offset +! len } <:
                  Core_models.Ops.Range.t_Range usize ] in
        let b0 : t_Array u8 rate = Rust_primitives.Hax.repeat (mk_u8 0) rate in
        let b1 : t_Array u8 rate =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_range b0
            ({ Core_models.Ops.Range.f_start = mk_usize 0;
               Core_models.Ops.Range.f_end   = len } <:
             Core_models.Ops.Range.t_Range usize)
            (Core_models.Slice.impl__copy_from_slice #u8
                (b0.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                        Core_models.Ops.Range.f_end   = len } <:
                      Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
                copy_src <: t_Slice u8) in
        let b2 : t_Array u8 rate =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b1 len delim in
        let b3 : t_Array u8 rate =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b2
            (rate -! mk_usize 1 <: usize)
            ((b2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
        let padded : t_Array u8 (mk_usize 200) =
          Hacspec_sha3.Sponge.pad_last_block input offset len rate delim in
        let padded_slice : t_Slice u8 =
          padded.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                     Core_models.Ops.Range.f_end   = rate } <:
                   Core_models.Ops.Range.t_Range usize ] in
        (b3 <: t_Slice u8) == padded_slice))
  = let copy_src : t_Slice u8 =
      input.[ { Core_models.Ops.Range.f_start = offset;
                Core_models.Ops.Range.f_end   = offset +! len } <:
              Core_models.Ops.Range.t_Range usize ] in
    let b0 : t_Array u8 rate = Rust_primitives.Hax.repeat (mk_u8 0) rate in
    let b1 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range b0
        ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (b0.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                    Core_models.Ops.Range.f_end   = len } <:
                  Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
            copy_src <: t_Slice u8) in
    Proof_Utils.Lemmas.lemma_index_update_at_range b0
      ({ Core_models.Ops.Range.f_start = mk_usize 0;
         Core_models.Ops.Range.f_end   = len } <:
       Core_models.Ops.Range.t_Range usize)
      copy_src;
    let b2 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b1 len delim in
    let b3 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b2
        (rate -! mk_usize 1 <: usize)
        ((b2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
    let s0 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 200) in
    let s1 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range s0
        ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (s0.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                    Core_models.Ops.Range.f_end   = len } <:
                  Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
            copy_src <: t_Slice u8) in
    Proof_Utils.Lemmas.lemma_index_update_at_range s0
      ({ Core_models.Ops.Range.f_start = mk_usize 0;
         Core_models.Ops.Range.f_end   = len } <:
       Core_models.Ops.Range.t_Range usize)
      copy_src;
    let s2 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s1 len delim in
    let s3 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s2
        (rate -! mk_usize 1 <: usize)
        ((s2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
    let s_slice : t_Slice u8 = Seq.slice s3 0 (v rate) in
    let byte_eq (m: nat{m < v rate})
      : Lemma (Seq.index (b3 <: Seq.seq u8) m == Seq.index (s3 <: Seq.seq u8) m)
      = () in
    Classical.forall_intro byte_eq;
    Rust_primitives.Arrays.eq_intro b3 s_slice
#pop-options

(* ================================================================
   Bridge: Arm64 [load_last] on lane [l] ≡ [xor_block_into_state] on
   [pad_last_block inputs.[l] start len rate delim][0..rate].
   ================================================================ *)
#push-options "--z3rlimit 400"
let lemma_load_last_eq_xor_block_into_state_arm64
      (rate: usize) (delim: u8)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (inputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize) (len: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) inputs)
      (ensures (
        let padded : t_Array u8 (mk_usize 200) =
          Hacspec_sha3.Sponge.pad_last_block
            (inputs.[ mk_usize l ]) start len rate delim in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          (Libcrux_sha3.Simd.Arm64.load_last rate delim state inputs start len) l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
          (padded.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = rate } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
          rate))
  = let in0 : t_Slice u8 = inputs.[ mk_usize 0 ] in
    let in1 : t_Slice u8 = inputs.[ mk_usize 1 ] in
    assert (Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) inputs);
    assert (Seq.length #u8 in0 == Seq.length #u8 in1);
    lemma_load_last_buffer_eq_padded_arm64 rate delim in0 start len;
    lemma_load_last_buffer_eq_padded_arm64 rate delim in1 start len;
    (* Reconstruct buffer0 / buffer1 inline — matching the body of
       [Libcrux_sha3.Simd.Arm64.load_last]. *)
    let copy_src0 : t_Slice u8 =
      in0.[ { Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! len } <:
            Core_models.Ops.Range.t_Range usize ] in
    let b0_0 : t_Array u8 rate = Rust_primitives.Hax.repeat (mk_u8 0) rate in
    let b0_1 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range b0_0
        ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (b0_0.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = len } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
            copy_src0 <: t_Slice u8) in
    let b0_2 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b0_1 len delim in
    let buffer0 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b0_2
        (rate -! mk_usize 1 <: usize)
        ((b0_2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
    let copy_src1 : t_Slice u8 =
      in1.[ { Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! len } <:
            Core_models.Ops.Range.t_Range usize ] in
    let b1_0 : t_Array u8 rate = Rust_primitives.Hax.repeat (mk_u8 0) rate in
    let b1_1 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range b1_0
        ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (b1_0.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = len } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
            copy_src1 <: t_Slice u8) in
    let b1_2 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b1_1 len delim in
    let buffer1 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b1_2
        (rate -! mk_usize 1 <: usize)
        ((b1_2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
    let buffers : t_Array (t_Slice u8) (mk_usize 2) =
      let list = [buffer0 <: t_Slice u8; buffer1 <: t_Slice u8] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
      Rust_primitives.Hax.array_of_list 2 list
    in
    assert (Seq.length #u8 (buffers.[ mk_usize 0 ] <: t_Slice u8) == v rate);
    lemma_load_block_eq_xor_block_into_state_arm64 rate state buffers (mk_usize 0) l;
    (* [buffers.[l]][0..rate] = buffer_l = padded_l[0..rate]. *)
    assert (v (mk_usize l) = l);
    let sliced_lane : t_Slice u8 =
      (buffers.[ mk_usize l ] <: t_Slice u8).[ {
          Core_models.Ops.Range.f_start = mk_usize 0;
          Core_models.Ops.Range.f_end   = mk_usize 0 +! rate } <:
        Core_models.Ops.Range.t_Range usize ] in
    Rust_primitives.Arrays.eq_intro sliced_lane
      (buffers.[ mk_usize l ] <: t_Slice u8)
#pop-options

#push-options "--z3rlimit 200"
let arm64_sc_load_last
      (rate: usize)
      (delim: u8)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (inputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) inputs)
      (ensures (
        let padded: t_Array u8 (mk_usize 200) =
          Hacspec_sha3.Sponge.pad_last_block
            (inputs.[ mk_usize l ]) start len rate delim
        in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          (Libcrux_sha3.Traits.f_load_last
             #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
             #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve
             rate delim
             ({ Libcrux_sha3.Generic_keccak.f_st = state }
                <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
             inputs
             start
             len)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
          (padded.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = rate } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
          rate))
  = lemma_load_last_eq_xor_block_into_state_arm64 rate delim state inputs start len l
#pop-options

(* ================================================================
   Bridge: pointwise equivalence of Arm64 [sq_lane_arm64] on lane [l]
   with the scalar spec [squeeze_state] — parallels the Portable
   [lemma_store_block_eq_squeeze_state], but threads [arm64_lane]
   through the hax-proved per-lane ensures clause on [Simd.Arm64.store_block].
   ================================================================ *)
#push-options "--z3rlimit 800"
let lemma_sq_lane_arm64_eq_squeeze_state
      (rate: usize)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (outputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs)
      (ensures
        sq_lane_arm64 rate state outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
          (outputs.[ mk_usize l ] <: t_Array u8 _)
          start
          len)
  = let out_l : t_Slice u8 = outputs.[ mk_usize l ] in
    let out_l_len = Core_models.Slice.impl__len #u8 out_l in
    let lhs = sq_lane_arm64 rate state outputs start len l in
    let rhs = Hacspec_sha3.Sponge.squeeze_state
                out_l_len
                (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
                (out_l <: t_Array u8 out_l_len) start len in
    assert (v (mk_usize l) = l);
    let byte_eq (i: nat{i < Seq.length out_l})
      : Lemma (Seq.index lhs i == Seq.index rhs i) =
      let ii = mk_usize i in
      assert (v ii < Seq.length out_l);
      (* Fire the extract_lane SMTPat on the state index that matters for
         byte i, and provide both get_lane_u64 instantiations so that the
         arm64_lane ↔ get_lane_u64 SMTPat can equate lhs and rhs in the
         "in range" branch. *)
      if v start <= v ii && v ii < v start + v len then begin
        let j : usize = (ii -! start) /! mk_usize 8 in
        KA.lemma_arm64_lane_unfold state.[j] l;
        assert ((G.extract_lane (mk_usize 2) KA.lc_arm64 state l).[j]
                  == KA.arm64_lane state.[j] l);
        let _ = I.get_lane_u64 state.[j] (mk_usize 0) in
        let _ = I.get_lane_u64 state.[j] (mk_usize 1) in
        ()
      end
    in
    Classical.forall_intro byte_eq;
    Rust_primitives.Arrays.eq_intro lhs rhs
#pop-options

#push-options "--z3rlimit 200"
let arm64_sc_store_block
      (rate: usize)
      (state: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (outputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs)
      (ensures
        sq_lane_arm64 rate state outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          (G.extract_lane (mk_usize 2) KA.lc_arm64 state l)
          (outputs.[ mk_usize l ] <: t_Array u8 _)
          start
          len)
  = lemma_sq_lane_arm64_eq_squeeze_state rate state outputs start len l
#pop-options

(* ================================================================
   Assemble the [sponge_correctness] record for Arm64.

   The sponge_correctness record requires a [t_Absorb ... v_N] dictionary
   in scope — the Arm64 impl_1 at N=2 is tcresolved automatically.
   ================================================================ *)

let sc_arm64 : SC.sponge_correctness (mk_usize 2) #I.t_e_uint64x2_t KA.lc_arm64 =
  {
    sq_lane        = sq_lane_arm64;
    sc_load_block  = arm64_sc_load_block;
    sc_load_last   = arm64_sc_load_last;
    sc_store_block = arm64_sc_store_block;
  }
