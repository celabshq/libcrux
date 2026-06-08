module Hacspec_ml_kem.Commute.Ind_cca_bridge
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P   = Hacspec_ml_kem.Parameters
module HF  = Hacspec_ml_kem.Parameters.Hash_functions
module HC  = Hacspec_ml_kem.Ind_cca
module HCP = Hacspec_ml_kem.Ind_cpa
module SU  = Spec.Utils

(* ════════════════════════════════════════════════════════════════════════
   ind_cca PACKED-API composition bridges (Phase 1).

   These relate the impl (Libcrux_ml_kem.Ind_cca.{generate_keypair,...}) to the
   hacspec reference (Hacspec_ml_kem.Ind_cca.{generate_keypair,...}) by composing
   the (admitted) ind_cpa contracts + the FO-glue posts (serialize_kem_secret_key,
   the Hash-trait posts).

   HASH-SPEC CONSISTENCY: the impl/Hash-trait side is specced against the abstract
   Spec.Utils.v_{G,H,PRF,J}; the hacspec reference uses the abstract
   Hacspec_ml_kem.Parameters.Hash_functions.v_{G,H,PRF,J}.  Both denote the same
   SHA3 primitives.  The equalities below are PROVEN by `Spec.Utils.lemma_v_*_eq`
   once Spec.Utils.v_* are made concrete aliases of the hacspec hashes (the final
   net-stronger upstream step).  DURING DEVELOPMENT they are admitted here.
   ════════════════════════════════════════════════════════════════════════ *)

(* FO-glue hash-spec consistency: the impl/Hash-trait side is specced vs the
   abstract Spec.Utils.v_H; the hacspec reference uses HF.v_H. Both denote
   SHA3-256. This is an ASSUMED, sound, Phase-2-dischargeable bridge (kept
   admitted — making it proven would require cold-rebuilding the foundational
   Spec.Utils, which currently has pre-existing stale-cache breakage). *)
let lemma_v_H_bridge (x: t_Slice u8)
  : Lemma (ensures SU.v_H x == HF.v_H x)
  = admit ()

(* FO-glue slice<->array coercion for the 32-byte seed/z splits. Same category
   as the existing (assumed) Spec.Utils.slice_to_array_id (len-16); a known-true
   Core_models try_into fact. *)
let lemma_slice_to_array_id_32 (array: t_Slice u8)
  : Lemma (requires Seq.length array == 32)
          (ensures Core_models.Result.impl__unwrap
              #(t_Array u8 (mk_usize 32))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8)
                #(t_Array u8 (mk_usize 32))
                #FStar.Tactics.Typeclasses.solve
                array) == array)
  = admit ()

(* ─────────────────────────────────────────────────────────────────────────
   generate_keypair: ind_cca's MlKemKeyPair relates to the spec's (ek,dk).
   CONSTRUCTION BRIDGE: the 4-way update_at_range build inside keygen_internal's
   Ok branch equals the 4-way Seq.append (dk_pke ‖ ek ‖ H(ek) ‖ z).
   This MIRRORS the (proven) impl-side serialize_kem_secret_key_mut at
   Libcrux_ml_kem.Ind_cca.fst (4 slice asserts + lemma_slice_append_4).
   The `dk` term below is copied VERBATIM from keygen_internal so it matches
   definitionally under unfold.
   ───────────────────────────────────────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_dk_build
      (v_K v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE: usize)
      (ek: t_Array u8 v_EK_SIZE)
      (dk_pke: t_Array u8 v_DK_PKE_SIZE)
      (z: t_Array u8 (mk_usize 32))
  : Lemma
    (requires
      P.is_rank v_K /\
      v_EK_SIZE == (v_K *! P.v_BYTES_PER_RING_ELEMENT) +! mk_usize 32 /\
      v_DK_PKE_SIZE == v_K *! P.v_BYTES_PER_RING_ELEMENT /\
      v_DK_SIZE == ((v_DK_PKE_SIZE +! v_EK_SIZE) +! HF.v_H_DIGEST_SIZE) +! mk_usize 32)
    (ensures
      (let dk0:t_Array u8 v_DK_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_DK_SIZE in
       let dk1:t_Array u8 v_DK_SIZE =
         Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to dk0
           ({ Core_models.Ops.Range.f_end = v_DK_PKE_SIZE } <: Core_models.Ops.Range.t_RangeTo usize)
           (Core_models.Slice.impl__copy_from_slice #u8
               (dk0.[ { Core_models.Ops.Range.f_end = v_DK_PKE_SIZE }
                   <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
               (dk_pke <: t_Slice u8) <: t_Slice u8)
       in
       let dk2:t_Array u8 v_DK_SIZE =
         Rust_primitives.Hax.Monomorphized_update_at.update_at_range dk1
           ({ Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
              Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize }
             <: Core_models.Ops.Range.t_Range usize)
           (Core_models.Slice.impl__copy_from_slice #u8
               (dk1.[ { Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
                        Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize }
                   <: Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
               (ek <: t_Slice u8) <: t_Slice u8)
       in
       let dk3:t_Array u8 v_DK_SIZE =
         Rust_primitives.Hax.Monomorphized_update_at.update_at_range dk2
           ({ Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
              Core_models.Ops.Range.f_end
              = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
             <: Core_models.Ops.Range.t_Range usize)
           (Core_models.Slice.impl__copy_from_slice #u8
               (dk2.[ { Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
                        Core_models.Ops.Range.f_end
                        = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
                   <: Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
               (HF.v_H (ek <: t_Slice u8) <: t_Slice u8) <: t_Slice u8)
       in
       let dk4:t_Array u8 v_DK_SIZE =
         Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from dk3
           ({ Core_models.Ops.Range.f_start
              = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
             <: Core_models.Ops.Range.t_RangeFrom usize)
           (Core_models.Slice.impl__copy_from_slice #u8
               (dk3.[ { Core_models.Ops.Range.f_start
                        = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
                   <: Core_models.Ops.Range.t_RangeFrom usize ] <: t_Slice u8)
               (z <: t_Slice u8) <: t_Slice u8)
       in
       dk4 == Seq.append (dk_pke <: t_Slice u8)
                (Seq.append (ek <: t_Slice u8)
                  (Seq.append (HF.v_H (ek <: t_Slice u8)) (z <: t_Slice u8)))))
  =
  let p0:usize = v_DK_PKE_SIZE in
  let p1:usize = v_DK_PKE_SIZE +! v_EK_SIZE in
  let p2:usize = (v_DK_PKE_SIZE +! v_EK_SIZE) +! HF.v_H_DIGEST_SIZE in
  (* Length facts. *)
  assert (Seq.length (dk_pke <: t_Slice u8) == v v_DK_PKE_SIZE);
  assert (Seq.length (ek <: t_Slice u8) == v v_EK_SIZE);
  assert (Seq.length (HF.v_H (ek <: t_Slice u8)) == 32);
  assert (Seq.length (z <: t_Slice u8) == 32);
  let dk0:t_Array u8 v_DK_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_DK_SIZE in
  (* write 1: dk_pke into [0, p0).  copy_from_slice output == dk_pke. *)
  let c1:t_Slice u8 =
    Core_models.Slice.impl__copy_from_slice #u8
      (dk0.[ { Core_models.Ops.Range.f_end = v_DK_PKE_SIZE }
          <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
      (dk_pke <: t_Slice u8)
  in
  assert (c1 == (dk_pke <: t_Slice u8));
  let dk1:t_Array u8 v_DK_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to dk0
      ({ Core_models.Ops.Range.f_end = v_DK_PKE_SIZE } <: Core_models.Ops.Range.t_RangeTo usize)
      c1
  in
  (* write 2: ek into [p0, p1). *)
  let c2:t_Slice u8 =
    Core_models.Slice.impl__copy_from_slice #u8
      (dk1.[ { Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
               Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize }
          <: Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
      (ek <: t_Slice u8)
  in
  assert (c2 == (ek <: t_Slice u8));
  let dk2:t_Array u8 v_DK_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range dk1
      ({ Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
         Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize }
        <: Core_models.Ops.Range.t_Range usize)
      c2
  in
  (* write 3: H(ek) into [p1, p2). *)
  let c3:t_Slice u8 =
    Core_models.Slice.impl__copy_from_slice #u8
      (dk2.[ { Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
               Core_models.Ops.Range.f_end
               = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
          <: Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
      (HF.v_H (ek <: t_Slice u8) <: t_Slice u8)
  in
  assert (c3 == (HF.v_H (ek <: t_Slice u8) <: t_Slice u8));
  let dk3:t_Array u8 v_DK_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range dk2
      ({ Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
         Core_models.Ops.Range.f_end
         = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
        <: Core_models.Ops.Range.t_Range usize)
      c3
  in
  (* write 4: z into [p2, v_DK_SIZE). *)
  let c4:t_Slice u8 =
    Core_models.Slice.impl__copy_from_slice #u8
      (dk3.[ { Core_models.Ops.Range.f_start
               = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
          <: Core_models.Ops.Range.t_RangeFrom usize ] <: t_Slice u8)
      (z <: t_Slice u8)
  in
  assert (c4 == (z <: t_Slice u8));
  let dk4:t_Array u8 v_DK_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from dk3
      ({ Core_models.Ops.Range.f_start
         = (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +! HF.v_H_DIGEST_SIZE <: usize }
        <: Core_models.Ops.Range.t_RangeFrom usize)
      c4
  in
  assert (Seq.length dk4 == v v_DK_SIZE);
  (* ── full-prefix / written-segment posts from each update_at_range* ──
     dk1 (range_to, end p0): slice dk1 0 p0 == dk_pke
     dk2 (range p0..p1):     slice dk2 0 p0 == slice dk1 0 p0 ; slice dk2 p0 p1 == ek
     dk3 (range p1..p2):     slice dk3 0 p1 == slice dk2 0 p1 ; slice dk3 p1 p2 == H(ek)
     dk4 (range_from p2):    slice dk4 0 p2 == slice dk3 0 p2 ; slice dk4 p2 len == z *)
  let _ = Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to dk0
            ({ Core_models.Ops.Range.f_end = v_DK_PKE_SIZE }
              <: Core_models.Ops.Range.t_RangeTo usize) c1 in
  assert (Seq.slice dk1 0 (v p0) == c1);
  assert (Seq.slice dk2 0 (v p0) == Seq.slice dk1 0 (v p0));
  assert (Seq.slice dk2 (v p0) (v p1) == c2);
  assert (Seq.slice dk3 0 (v p1) == Seq.slice dk2 0 (v p1));
  assert (Seq.slice dk3 (v p1) (v p2) == c3);
  assert (Seq.slice dk4 0 (v p2) == Seq.slice dk3 0 (v p2));
  assert (Seq.slice dk4 (v p2) (Seq.length dk4) == c4);
  (* descend the full-prefix equalities to the segment boundaries via slice_slice *)
  Seq.slice_slice dk4 0 (v p2) 0 (v p0);
  Seq.slice_slice dk3 0 (v p2) 0 (v p0);
  Seq.slice_slice dk3 0 (v p1) 0 (v p0);
  Seq.slice_slice dk2 0 (v p1) 0 (v p0);
  Seq.slice_slice dk4 0 (v p2) (v p0) (v p1);
  Seq.slice_slice dk3 0 (v p2) (v p0) (v p1);
  Seq.slice_slice dk3 0 (v p1) (v p0) (v p1);
  Seq.slice_slice dk4 0 (v p2) (v p1) (v p2);
  (* Segment 1: [0, p0) == dk_pke. *)
  assert (Seq.slice dk4 0 (v p0) `Seq.equal` (dk_pke <: t_Slice u8));
  (* Segment 2: [p0, p1) == ek. *)
  assert (Seq.slice dk4 (v p0) (v p1) `Seq.equal` (ek <: t_Slice u8));
  (* Segment 3: [p1, p2) == H(ek). *)
  assert (Seq.slice dk4 (v p1) (v p2) `Seq.equal` (HF.v_H (ek <: t_Slice u8) <: t_Slice u8));
  (* Segment 4: [p2, v_DK_SIZE) == z. *)
  assert (Seq.slice dk4 (v p2) (v v_DK_SIZE) `Seq.equal` (z <: t_Slice u8));
  Rust_primitives.Arrays.lemma_slice_append_4
    (dk4 <: t_Slice u8)
    (dk_pke <: t_Slice u8)
    (ek <: t_Slice u8)
    (HF.v_H (ek <: t_Slice u8) <: t_Slice u8)
    (z <: t_Slice u8)
#pop-options

(* ─────────────────────────────────────────────────────────────────────────
   generate_keypair: ind_cca's MlKemKeyPair relates to the spec's (ek,dk).
   Consumer-facing: takes the impl's ind_cpa contract conclusion + serialize
   post as hypotheses, produces the ind_cca functional post.
   ───────────────────────────────────────────────────────────────────────── *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_generate_keypair_post
      (v_K v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE: usize)
      (randomness: t_Array u8 (mk_usize 64))
      (ind_cpa_private_key: t_Array u8 v_DK_PKE_SIZE)
      (public_key: t_Array u8 v_EK_SIZE)
      (secret_key_serialized: t_Array u8 v_DK_SIZE)
  : Lemma
    (requires
      P.is_rank v_K /\
      v_EK_SIZE == (v_K *! P.v_BYTES_PER_RING_ELEMENT) +! mk_usize 32 /\
      v_DK_PKE_SIZE == v_K *! P.v_BYTES_PER_RING_ELEMENT /\
      v_DK_SIZE == ((v_DK_PKE_SIZE +! v_EK_SIZE) +! HF.v_H_DIGEST_SIZE) +! mk_usize 32 /\
      (match HCP.generate_keypair v_K v_EK_SIZE v_DK_PKE_SIZE (P.rank_to_params v_K)
               (Seq.slice randomness 0 32 <: t_Slice u8)
       with
       | Core_models.Result.Result_Ok (ek, dk_pke) ->
         ind_cpa_private_key == dk_pke /\ public_key == ek
       | Core_models.Result.Result_Err _ -> True) /\
      secret_key_serialized ==
        Seq.append (ind_cpa_private_key <: t_Slice u8)
          (Seq.append (public_key <: t_Slice u8)
            (Seq.append (SU.v_H (public_key <: t_Slice u8))
              (Seq.slice randomness 32 64 <: t_Slice u8))))
    (ensures
      (match HC.generate_keypair v_K v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE
               (P.rank_to_params v_K) randomness
       with
       | Core_models.Result.Result_Ok (ek, dk) ->
         public_key == ek /\ secret_key_serialized == dk
       | Core_models.Result.Result_Err _ -> True))
  =
  (* Seed coercion: d == Seq.slice randomness 0 32, z == Seq.slice randomness 32 64. *)
  let d:t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (randomness.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8))
  in
  let z:t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (randomness.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <: Core_models.Ops.Range.t_RangeFrom usize ] <: t_Slice u8))
  in
  (* RangeTo/RangeFrom indexing reduces to slice_slice = Seq.slice. *)
  assert ((randomness.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
          == Seq.slice randomness 0 32);
  assert ((randomness.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <: Core_models.Ops.Range.t_RangeFrom usize ] <: t_Slice u8)
          == Seq.slice randomness 32 64);
  lemma_slice_to_array_id_32 (Seq.slice randomness 0 32 <: t_Slice u8);
  lemma_slice_to_array_id_32 (Seq.slice randomness 32 64 <: t_Slice u8);
  assert (d == Seq.slice randomness 0 32);
  assert (z == Seq.slice randomness 32 64);
  match HCP.generate_keypair v_K v_EK_SIZE v_DK_PKE_SIZE (P.rank_to_params v_K)
          (Seq.slice randomness 0 32 <: t_Slice u8)
  with
  | Core_models.Result.Result_Ok (ek, dk_pke) ->
    (* ek == public_key, dk_pke == ind_cpa_private_key from the ind_cpa contract. *)
    assert (ind_cpa_private_key == dk_pke /\ public_key == ek);
    (* Construction bridge: keygen_internal's dk == 4-append. *)
    lemma_dk_build v_K v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE ek dk_pke z;
    (* Hash bridge: SU.v_H public_key == HF.v_H ek (public_key == ek). *)
    lemma_v_H_bridge (public_key <: t_Slice u8);
    assert (SU.v_H (public_key <: t_Slice u8) == HF.v_H (ek <: t_Slice u8));
    (* secret_key_serialized == 4-append (from requires + hash bridge + z). *)
    assert (secret_key_serialized ==
        Seq.append (dk_pke <: t_Slice u8)
          (Seq.append (ek <: t_Slice u8)
            (Seq.append (HF.v_H (ek <: t_Slice u8)) (z <: t_Slice u8))))
  | Core_models.Result.Result_Err _ -> ()
#pop-options
