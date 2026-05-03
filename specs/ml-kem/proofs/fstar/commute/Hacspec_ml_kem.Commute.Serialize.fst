module Hacspec_ml_kem.Commute.Serialize
/// Phase C bridge lemmas connecting `Hacspec_ml_kem.Serialize.serialize_secret_key`
/// to per-element `byte_encode` calls.  These unblock `serialize_vector` and
/// `compress_then_serialize_u` in `Libcrux_ml_kem.Ind_cpa`.
///
/// Status (2026-05-03): `serialize_secret_key_chunk_eq` body is admitted.
/// The proof follows from unfolding `serialize_secret_key_into`'s fold_range
/// and showing that chunk `j` is exactly the `byte_encode` call at iteration `j`.
/// U-task 6 will close the admit.

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

module S   = Hacspec_ml_kem.Serialize
module P   = Hacspec_ml_kem.Parameters

/// For each `j < v_K`, the `j`-th chunk of `serialize_secret_key` equals
/// the `byte_encode` of the `j`-th polynomial.
///
/// This is the key slice-equality connecting the spec serializer to per-element
/// encoding.  Used by `serialize_vector` (Family A) via `eq_intro`.
val serialize_secret_key_chunk_eq
      (v_K: usize)
      (spec_vec: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (j: usize)
    : Lemma
      (requires
        v v_K <= 4 /\
        v j < v v_K)
      (ensures
        (let v_T_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT in
         Seq.slice
           (S.serialize_secret_key v_K v_T_SIZE spec_vec)
           (v j * v P.v_BYTES_PER_RING_ELEMENT)
           ((v j + 1) * v P.v_BYTES_PER_RING_ELEMENT)
         == (S.byte_encode (mk_usize 384) (mk_usize 3072)
               (Seq.index spec_vec (v j))
               (mk_usize 12))))

let serialize_secret_key_chunk_eq v_K spec_vec j = admit ()

/// Forall-quantified version for use in loop-invariant closures.
val serialize_secret_key_all_chunks_eq
      (v_K: usize)
      (spec_vec: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    : Lemma
      (requires v v_K <= 4)
      (ensures
        (let v_T_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT in
         forall (j: nat). j < v v_K ==>
           Seq.slice
             (S.serialize_secret_key v_K v_T_SIZE spec_vec)
             (j * v P.v_BYTES_PER_RING_ELEMENT)
             ((j + 1) * v P.v_BYTES_PER_RING_ELEMENT)
           == (S.byte_encode (mk_usize 384) (mk_usize 3072)
                 (Seq.index spec_vec j)
                 (mk_usize 12))))

let serialize_secret_key_all_chunks_eq v_K spec_vec = admit ()
