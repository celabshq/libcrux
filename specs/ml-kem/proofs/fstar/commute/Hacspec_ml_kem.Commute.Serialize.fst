module Hacspec_ml_kem.Commute.Serialize
/// Bridge lemmas connecting the Hacspec serializers to their createi structure.
///
/// serialize_secret_key: chunk equality (per polynomial) and all-chunks forall.
/// serialize_public_key: vector part == serialize_secret_key; seed part == seed[0..32].
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
#push-options "--z3rlimit 80"
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
#pop-options

(* ------------------------------------------------------------------ *)
(* serialize_public_key bridge                                          *)
(* ------------------------------------------------------------------ *)

/// The first RANK*384 bytes of serialize_public_key equal serialize_secret_key.
/// Both are createi-based; the SMTPat fires for each side and Z3 closes by
/// the if-branch condition (m < RANK*384) and matching arithmetic.
#push-options "--z3rlimit 300"
let serialize_public_key_vector_eq
      (v_K: usize)
      (tt_as_ntt: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (seed: t_Slice u8)
    : Lemma
      (requires
        v v_K <= 4 /\
        (Core_models.Slice.impl__len #u8 seed <: usize) >=. mk_usize 32)
      (ensures
        (let v_EK_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT +! mk_usize 32 in
         Seq.slice
           (S.serialize_public_key v_K v_EK_SIZE tt_as_ntt seed)
           0
           (v v_K * v P.v_BYTES_PER_RING_ELEMENT)
         == S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT) tt_as_ntt))
    =
  let v_EK_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT +! mk_usize 32 in
  let pk = S.serialize_public_key v_K v_EK_SIZE tt_as_ntt seed in
  let sk = S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT) tt_as_ntt in
  let n = v v_K * v P.v_BYTES_PER_RING_ELEMENT in
  let aux (m: nat{m < n})
    : Lemma (Seq.index (Seq.slice pk 0 n) m == Seq.index sk m) =
    // Bridge slice to full-array index
    FStar.Seq.Base.lemma_index_slice pk 0 n m;
    // nat→usize so createi_lemma SMTPat fires on both pk[m] and sk[m]
    let km = mk_usize m in
    assert (m == v km);
    // m < n = RANK*384: the if-branch fires in f_pk(km), giving byte_encode(...)[km%384]
    // sk[m] = f_sk(km) = byte_encode(...)[km%384] — same result
    ()
  in
  let aux' (m: nat)
    : Lemma (m < n ==> Seq.index (Seq.slice pk 0 n) m == Seq.index sk m) =
    if m < n then aux m
  in
  FStar.Classical.forall_intro aux';
  FStar.Seq.lemma_eq_intro (Seq.slice pk 0 n) sk
#pop-options

/// The trailing 32 bytes of serialize_public_key equal Seq.slice seed 0 32.
/// The else-branch in f_pk fires for k >= RANK*384, giving seed[k - RANK*384].
#push-options "--z3rlimit 300"
let serialize_public_key_seed_eq
      (v_K: usize)
      (tt_as_ntt: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (seed: t_Slice u8)
    : Lemma
      (requires
        v v_K <= 4 /\
        (Core_models.Slice.impl__len #u8 seed <: usize) >=. mk_usize 32)
      (ensures
        (let v_EK_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT +! mk_usize 32 in
         let b = v v_K * v P.v_BYTES_PER_RING_ELEMENT in
         Seq.slice
           (S.serialize_public_key v_K v_EK_SIZE tt_as_ntt seed)
           b
           (b + 32)
         == Seq.slice seed 0 32))
    =
  let v_EK_SIZE = v_K *! P.v_BYTES_PER_RING_ELEMENT +! mk_usize 32 in
  let pk = S.serialize_public_key v_K v_EK_SIZE tt_as_ntt seed in
  let b = v v_K * v P.v_BYTES_PER_RING_ELEMENT in
  let aux (m: nat{m < 32})
    : Lemma (Seq.index (Seq.slice pk b (b + 32)) m == Seq.index (Seq.slice seed 0 32) m) =
    FStar.Seq.Base.lemma_index_slice pk b (b + 32) m;
    FStar.Seq.Base.lemma_index_slice seed 0 32 m;
    // nat→usize hint: pk[b+m] = f_pk(mk_usize(b+m))
    let km = mk_usize (b + m) in
    assert (b + m == v km);
    // else-branch: km >=. (v_K *! v_BYTES_PER_RING_ELEMENT) so seed[km - b_usize] fires
    // seed[(b+m) - b] = seed[m] = (Seq.slice seed 0 32)[m]
    ()
  in
  let aux' (m: nat)
    : Lemma (m < 32 ==> Seq.index (Seq.slice pk b (b + 32)) m == Seq.index (Seq.slice seed 0 32) m) =
    if m < 32 then aux m
  in
  FStar.Classical.forall_intro aux';
  FStar.Seq.lemma_eq_intro (Seq.slice pk b (b + 32)) (Seq.slice seed 0 32)
#pop-options
