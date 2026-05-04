module Hacspec_ml_kem.Commute.Serialize
/// Phase C bridge lemmas connecting `Hacspec_ml_kem.Serialize.serialize_secret_key`
/// to per-element `byte_encode` calls.  These unblock `serialize_vector` and
/// `compress_then_serialize_u` in `Libcrux_ml_kem.Ind_cpa`.
///
/// Strategy: `serialize_secret_key` is defined as a direct `createi` call in the
/// extraction (not a fold_range loop).  Z3 unfolds the transparent definition,
/// `createi_lemma`'s SMTPat fires on `Seq.index result (v k)`, then Z3 arithmetic
/// (b = 384, (j*b+m)/b = j, (j*b+m)%b = m) closes each byte position.
///
/// Key trick: `Seq.index result (lo+m)` where `lo+m : nat` does NOT trigger the
/// SMTPat.  We assert `lo+m == v (mk_usize (lo+m))` so Z3 can rewrite to the
/// `v k` form the pattern expects.

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

module S   = Hacspec_ml_kem.Serialize
module P   = Hacspec_ml_kem.Parameters

(* ------------------------------------------------------------------ *)
(* Main lemmas                                                           *)
(* ------------------------------------------------------------------ *)

/// For each `j < v_K`, the `j`-th chunk of `serialize_secret_key` equals
/// the `byte_encode` of the `j`-th polynomial.
#push-options "--z3rlimit 300"
let serialize_secret_key_chunk_eq
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
    =
  let v_T_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT in
  let result = S.serialize_secret_key v_K v_T_SIZE spec_vec in
  let target = S.byte_encode (mk_usize 384) (mk_usize 3072)
    (Seq.index spec_vec (v j)) (mk_usize 12) in
  let b = v P.v_BYTES_PER_RING_ELEMENT in
  let lo = v j * b in
  let hi = (v j + 1) * b in
  // Prove pointwise: for each position m in [0, b), result[lo+m] == target[m]
  let aux (m: nat{m < b})
    : Lemma (Seq.index (Seq.slice result lo hi) m == Seq.index target m) =
    // Step 1: bridge slice-index to full-sequence index
    FStar.Seq.Base.lemma_index_slice result lo hi m;
    // Step 2: coerce lo+m from nat to v (usize) so createi_lemma SMTPat fires.
    // Since serialize_secret_key is transparent (not assume val), Z3 unfolds it to
    // createi v_T_SIZE f, and the SMTPat on Seq.index (createi f) (v k) triggers.
    let k_lom = mk_usize (lo + m) in
    assert (lo + m == v k_lom);
    // Step 3: Z3 arithmetic: (j*b+m)/b = j (since m < b), (j*b+m)%b = m.
    // This establishes vector[k_lom /! b] = spec_vec.[j] and k_lom %! b = m.
    // Combined with createi_lemma, the goal reduces to:
    //   byte_encode 384 3072 (spec_vec.[j]) 12).[mk_usize m] == Seq.index target m
    // which holds by definition of target.
    ()
  in
  // Lift pointwise equality to sequence equality
  let aux' (m: nat)
    : Lemma (m < b ==>
      Seq.index (Seq.slice result lo hi) m == Seq.index target m) =
    if m < b then aux m
  in
  FStar.Classical.forall_intro aux';
  FStar.Seq.lemma_eq_intro (Seq.slice result lo hi) target
#pop-options

/// Forall-quantified version for use in loop-invariant closures.
let serialize_secret_key_all_chunks_eq
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
    =
  let aux (j: nat)
    : Lemma (j < v v_K ==>
        Seq.slice
          (S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT) spec_vec)
          (j * v P.v_BYTES_PER_RING_ELEMENT)
          ((j + 1) * v P.v_BYTES_PER_RING_ELEMENT)
        == (S.byte_encode (mk_usize 384) (mk_usize 3072)
              (Seq.index spec_vec j)
              (mk_usize 12))) =
    if j < v v_K
    then serialize_secret_key_chunk_eq v_K spec_vec (mk_usize j)
  in
  FStar.Classical.forall_intro aux
