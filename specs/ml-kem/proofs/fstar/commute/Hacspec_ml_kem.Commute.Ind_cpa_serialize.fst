module Hacspec_ml_kem.Commute.Ind_cpa_serialize

/// Finalize bridges for the ind_cpa serialize/deserialize helpers.
/// Each takes the loop-invariant conclusion (per-chunk byte_encode / byte_decode
/// equality) and lifts it to the whole-array Hacspec serializer via
/// chunk-extensionality, reusing the proven Hacspec_ml_kem.Commute.Serialize
/// per-chunk lemmas.

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

module S   = Hacspec_ml_kem.Serialize
module P   = Hacspec_ml_kem.Parameters
module VS  = Libcrux_ml_kem.Vector.Spec
module CS  = Hacspec_ml_kem.Commute.Serialize
module F   = Rust_primitives.Hax.Folds
module U   = Rust_primitives.Hax.Monomorphized_update_at

(* ------------------------------------------------------------------ *)
(* arithmetic helper: m < n*k ==> m/k < n                               *)
(* ------------------------------------------------------------------ *)
let lemma_div_lt_mul (m: nat) (k: pos) (n: pos)
    : Lemma (requires m < n * k) (ensures m / k < n)
  = FStar.Math.Lemmas.lemma_div_le m (n * k - 1) k;
    FStar.Math.Lemmas.small_division_lemma_1 (k - 1) k;
    FStar.Math.Lemmas.lemma_div_plus (k - 1) (n - 1) k

(* ------------------------------------------------------------------ *)
(* chunked-equality extensionality                                      *)
(*   two length-(N*b) seqs that agree on every b-chunk are equal        *)
(* ------------------------------------------------------------------ *)
#push-options "--z3rlimit 300"
let lemma_chunked_eq
      (a b_: Seq.seq u8)
      (v_N b: pos)
    : Lemma
      (requires
        Seq.length a == v_N * b /\
        Seq.length b_ == v_N * b /\
        (forall (j: nat). j < v_N ==>
          Seq.slice a (j * b) ((j + 1) * b) == Seq.slice b_ (j * b) ((j + 1) * b)))
      (ensures a == b_)
    =
  let aux (m: nat{m < v_N * b})
    : Lemma (Seq.index a m == Seq.index b_ m) =
    let j = m / b in
    let r = m % b in
    lemma_div_lt_mul m b v_N;              // j < v_N
    FStar.Math.Lemmas.lemma_div_mod m b;   // m = b*j + r, 0 <= r < b
    assert (j < v_N);
    assert (r < b);
    assert (m == j * b + r);
    FStar.Seq.Base.lemma_index_slice a  (j * b) ((j + 1) * b) r;
    FStar.Seq.Base.lemma_index_slice b_ (j * b) ((j + 1) * b) r
  in
  let aux' (m: nat) : Lemma (m < v_N * b ==> Seq.index a m == Seq.index b_ m) =
    if m < v_N * b then aux m
  in
  FStar.Classical.forall_intro aux';
  FStar.Seq.lemma_eq_intro a b_
#pop-options

(* ------------------------------------------------------------------ *)
(* serialize_vector finalize                                            *)
(* ------------------------------------------------------------------ *)

/// Given that every 384-byte chunk of `out` equals `byte_encode` of the
/// corresponding impl polynomial (the serialize_vector loop invariant at
/// i = K), conclude `out == serialize_secret_key (vector_to_spec key)`.
#push-options "--z3rlimit 300"
let lemma_serialize_vector_finalize
      (v_K: usize)
      (#v_V: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V |}
      (out: t_Slice u8)
      (key: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V) v_K)
    : Lemma
      (requires
        v v_K <= 4 /\ v v_K > 0 /\
        Seq.length out == v v_K * v P.v_BYTES_PER_RING_ELEMENT /\
        (forall (j: nat). j < v v_K ==>
          Seq.slice out
            (j * v P.v_BYTES_PER_RING_ELEMENT)
            ((j + 1) * v P.v_BYTES_PER_RING_ELEMENT)
          == S.byte_encode (mk_usize 384) (mk_usize 3072)
               (VS.poly_to_spec #v_V (Seq.index key j)) (mk_usize 12)))
      (ensures
        out ==
        S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT)
          (VS.vector_to_spec v_K #v_V key))
    =
  assert_norm (v P.v_BYTES_PER_RING_ELEMENT == 384);
  let b = v P.v_BYTES_PER_RING_ELEMENT in
  let spec_vec = VS.vector_to_spec v_K #v_V key in
  let sk = S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT) spec_vec in
  // sk's chunks == byte_encode (spec_vec[j])
  CS.serialize_secret_key_all_chunks_eq v_K spec_vec;
  // bridge spec_vec[j] == poly_to_spec key[j], for all j
  let bridge (j: nat) : Lemma (j < v v_K ==>
        Seq.index spec_vec j == VS.poly_to_spec #v_V (Seq.index key j)) =
    if j < v v_K then VS.vector_to_spec_index v_K #v_V key j
  in
  FStar.Classical.forall_intro bridge;
  // now: forall j<K. slice out (j*b) ((j+1)*b) == slice sk (j*b) ((j+1)*b)
  lemma_chunked_eq out sk (v v_K) b
#pop-options

(* ------------------------------------------------------------------ *)
(* serialize_public_key_mut finalize                                    *)
(* ------------------------------------------------------------------ *)

/// Given that the first K*384 bytes of `out` equal serialize_secret_key
/// (the post of the inner serialize_vector call) and the trailing 32 bytes
/// equal seed[0..32] (the copy_from_slice post), conclude that the whole
/// `out` equals serialize_public_key.
#push-options "--z3rlimit 300"
let lemma_serialize_public_key_mut_finalize
      (v_K v_PUBLIC_KEY_SIZE: usize)
      (out: t_Slice u8)
      (spec_vec: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (seed: t_Slice u8)
    : Lemma
      (requires
        v v_K <= 4 /\ v v_K > 0 /\
        (Core_models.Slice.impl__len #u8 seed <: usize) == mk_usize 32 /\
        v v_PUBLIC_KEY_SIZE == v v_K * v P.v_BYTES_PER_RING_ELEMENT + 32 /\
        Seq.length out == v v_PUBLIC_KEY_SIZE /\
        Seq.slice out 0 (v v_K * v P.v_BYTES_PER_RING_ELEMENT)
          == S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT) spec_vec /\
        Seq.slice out (v v_K * v P.v_BYTES_PER_RING_ELEMENT)
          (v v_K * v P.v_BYTES_PER_RING_ELEMENT + 32)
          == seed)
      (ensures
        out == S.serialize_public_key v_K v_PUBLIC_KEY_SIZE spec_vec seed)
    =
  assert_norm (v P.v_BYTES_PER_RING_ELEMENT == 384);
  let b = v v_K * v P.v_BYTES_PER_RING_ELEMENT in
  let pk = S.serialize_public_key v_K v_PUBLIC_KEY_SIZE spec_vec seed in
  let sk = S.serialize_secret_key v_K (v_K *! P.v_BYTES_PER_RING_ELEMENT) spec_vec in
  // chunk fact for the vector part: every 384-chunk of sk == byte_encode(spec_vec[j])
  CS.serialize_secret_key_all_chunks_eq v_K spec_vec;
  // pk's first b bytes == sk
  CS.serialize_public_key_vector_eq v_K spec_vec seed;
  // pk's trailing 32 bytes == seed[0..32] == seed (len 32)
  CS.serialize_public_key_seed_eq v_K spec_vec seed;
  FStar.Seq.lemma_eq_intro (Seq.slice seed 0 32) seed;
  // pointwise equality on out vs pk
  let aux (m: nat{m < v v_PUBLIC_KEY_SIZE})
    : Lemma (Seq.index out m == Seq.index pk m) =
    if m < b then begin
      FStar.Seq.Base.lemma_index_slice out 0 b m;
      FStar.Seq.Base.lemma_index_slice pk 0 b m;
      // Seq.slice out 0 b == sk, Seq.slice pk 0 b == sk
      ()
    end
    else begin
      FStar.Seq.Base.lemma_index_slice out b (b + 32) (m - b);
      FStar.Seq.Base.lemma_index_slice pk b (b + 32) (m - b);
      // Seq.slice out b (b+32) == seed, Seq.slice pk b (b+32) == seed
      ()
    end
  in
  let aux' (m: nat) : Lemma (m < v v_PUBLIC_KEY_SIZE ==> Seq.index out m == Seq.index pk m) =
    if m < v v_PUBLIC_KEY_SIZE then aux m
  in
  FStar.Classical.forall_intro aux';
  FStar.Seq.lemma_eq_intro out pk
#pop-options

(* ================================================================== *)
(* compress_then_serialize_u finalize                                  *)
(* ================================================================== *)

(* byte_encode_into for du in {10,11} ignores the `out` slice content and
   returns byte_encode (32*du) (256*du) p du. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 100"
let lemma_byte_encode_into_du
      (p: t_Array P.t_FieldElement (mk_usize 256))
      (du: usize { v du == 10 \/ v du == 11 })
      (out: t_Slice u8 { Seq.length out == 32 * v du })
    : Lemma
      (ensures
        S.byte_encode_into p du out
          == S.byte_encode (mk_usize 32 *! du) (mk_usize 256 *! du) p du)
    =
  if v du = 10
  then assert (S.byte_encode_into p du out
                 == S.byte_encode (mk_usize 320) (mk_usize 2560) p du)
  else assert (S.byte_encode_into p du out
                 == S.byte_encode (mk_usize 352) (mk_usize 2816) p du)
#pop-options

(* The spec fold, replicated so it delta/beta-reduces to the same term as
   `compress_then_serialize_u_into`.  `dp = 32*du` is the per-poly chunk size. *)
let cts_inv (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
    (out: t_Slice u8) (i: usize{F.fold_range_wf_index (mk_usize 0) v_K false (v i)}) : Type0 =
  (Core_models.Slice.impl__len #u8 out <: usize)
    =. (((v_K *! P.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du <: usize) /! mk_usize 8 <: usize)

let cts_step
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    : (acc: t_Slice u8 -> i:usize{ v i <= v v_K /\ F.fold_range_wf_index (mk_usize 0) v_K true (v i)
                                   /\ cts_inv v_K du acc i }
         -> acc': t_Slice u8 { cts_inv v_K du acc' (mk_int (v i + 1)) })
    =
  let dp = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8 in
  fun out i ->
    U.update_at_range out
      ({ Core_models.Ops.Range.f_start = i *! dp <: usize;
         Core_models.Ops.Range.f_end = (i +! mk_usize 1 <: usize) *! dp <: usize }
        <: Core_models.Ops.Range.t_Range usize)
      (S.byte_encode_into (Hacspec_ml_kem.Compress.compress (u.[ i ]) du) du
        (out.[ ({ Core_models.Ops.Range.f_start = i *! dp <: usize;
                  Core_models.Ops.Range.f_end = (i +! mk_usize 1 <: usize) *! dp <: usize }
                  <: Core_models.Ops.Range.t_Range usize) ]))

(* Length of the step output equals the length of its input (update_at_range
   preserves length). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_cts_step_len
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (out: t_Slice u8 { Seq.length out == v v_K * (32 * v du) })
      (i: usize { v i < v v_K })
    : Lemma
      (ensures
        Seq.length (cts_step v_K du u out i) == v v_K * (32 * v du))
    =
  let dp = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8 in
  assert (v dp == 32 * v du)
#pop-options

(* The step writes chunk i: chunk i of (cts_step out i) equals the value
   byte_encode_into (compress u[i] du) du _, hence (via lemma_byte_encode_into_du)
   the chunk-independent byte_encode. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_cts_step_chunk
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (out: t_Slice u8 { Seq.length out == v v_K * (32 * v du) })
      (i: usize { v i < v v_K })
    : Lemma
      (ensures
        (let dp = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8 in
         Seq.slice (cts_step v_K du u out i) (v i * (32 * v du)) ((v i + 1) * (32 * v du))
         == S.byte_encode (mk_usize 32 *! du) (mk_usize 256 *! du)
              (Hacspec_ml_kem.Compress.compress (Seq.index u (v i)) du) du))
    =
  let dp = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8 in
  assert (v dp == 32 * v du);
  let r = { Core_models.Ops.Range.f_start = i *! dp <: usize;
            Core_models.Ops.Range.f_end = (i +! mk_usize 1 <: usize) *! dp <: usize }
            <: Core_models.Ops.Range.t_Range usize in
  let written =
    S.byte_encode_into (Hacspec_ml_kem.Compress.compress (u.[ i ]) du) du (out.[ r ]) in
  // update_at_range post: chunk [start,end) == written
  let _ = U.update_at_range out r written in
  assert (Seq.slice (cts_step v_K du u out i) (v i * v dp) ((v i + 1) * v dp) == written);
  lemma_byte_encode_into_du (Hacspec_ml_kem.Compress.compress (u.[ i ]) du) du (out.[ r ])
#pop-options

(* Chunk j (j < i) is untouched by a step that writes chunk i (disjoint). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_cts_step_frame
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (out: t_Slice u8 { Seq.length out == v v_K * (32 * v du) })
      (i: usize { v i < v v_K })
      (j: nat { j < v i })
    : Lemma
      (ensures
        Seq.slice (cts_step v_K du u out i) (j * (32 * v du)) ((j + 1) * (32 * v du))
        == Seq.slice out (j * (32 * v du)) ((j + 1) * (32 * v du)))
    =
  let dp = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8 in
  assert (v dp == 32 * v du);
  let r = { Core_models.Ops.Range.f_start = i *! dp <: usize;
            Core_models.Ops.Range.f_end = (i +! mk_usize 1 <: usize) *! dp <: usize }
            <: Core_models.Ops.Range.t_Range usize in
  let written =
    S.byte_encode_into (Hacspec_ml_kem.Compress.compress (u.[ i ]) du) du (out.[ r ]) in
  let res = U.update_at_range out r written in
  // update_at_range post: prefix [0, i*dp) preserved.  chunk j (j<i) lives in [0, i*dp).
  assert (Seq.slice res 0 (v i * v dp) == Seq.slice out 0 (v i * v dp));
  Seq.slice_slice res 0 (v i * v dp) (j * v dp) ((j + 1) * v dp);
  Seq.slice_slice out 0 (v i * v dp) (j * v dp) ((j + 1) * v dp)
#pop-options

(* Recursive chunk-extraction over the cts fold.
   For s <= j < K, the j-th dp-chunk of the fold result equals
   byte_encode (32du) (256du) (compress u[j] du) du, independent of out0's
   content. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let rec lemma_cts_fold_chunk
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (s: usize { v s <= v v_K })
      (out0: t_Slice u8 { Seq.length out0 == v v_K * (32 * v du) })
      (j: nat { v s <= j /\ j < v v_K })
    : Lemma
      (ensures
        Seq.slice (F.fold_range s v_K (cts_inv v_K du) out0 (cts_step v_K du u))
          (j * (32 * v du)) ((j + 1) * (32 * v du))
        == S.byte_encode (mk_usize 32 *! du) (mk_usize 256 *! du)
             (Hacspec_ml_kem.Compress.compress (Seq.index u j) du) du)
      (decreases (v v_K - v s))
    =
  let step = cts_step v_K du u in
  let inv = cts_inv v_K du in
  let s1 = s +! mk_usize 1 in
  // s <= j < K so the fold is non-empty: peel one step.
  let out1 = step out0 s in
  lemma_cts_step_len v_K du u out0 s;
  // fold_range s K out0 == fold_range (s+1) K out1
  assert (F.fold_range s v_K inv out0 step == F.fold_range s1 v_K inv out1 step);
  let lo = j * (32 * v du) in
  let hi = (j + 1) * (32 * v du) in
  if j = v s then begin
    // chunk s was written by `step out0 s`; the remaining fold [s+1, K) is
    // either empty (s+1 == K) or preserves chunk s (disjoint frame).
    lemma_cts_step_chunk v_K du u out0 s;
    if v s1 < v v_K
    then lemma_cts_fold_frame v_K du u s1 out1 j
    else assert (F.fold_range s1 v_K inv out1 step == out1);
    assert (Seq.slice (F.fold_range s1 v_K inv out1 step) lo hi == Seq.slice out1 lo hi)
  end
  else
    lemma_cts_fold_chunk v_K du u s1 out1 j

(* Chunk j (j < s) is preserved by the whole fold over [s, K) (all iterations
   write chunks >= s, disjoint from j). *)
and lemma_cts_fold_frame
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (s: usize { v s <= v v_K })
      (out0: t_Slice u8 { Seq.length out0 == v v_K * (32 * v du) })
      (j: nat { j < v s })
    : Lemma
      (ensures
        Seq.slice (F.fold_range s v_K (cts_inv v_K du) out0 (cts_step v_K du u))
          (j * (32 * v du)) ((j + 1) * (32 * v du))
        == Seq.slice out0 (j * (32 * v du)) ((j + 1) * (32 * v du)))
      (decreases (v v_K - v s))
    =
  let step = cts_step v_K du u in
  let inv = cts_inv v_K du in
  if v s < v v_K then begin
    let out1 = step out0 s in
    lemma_cts_step_len v_K du u out0 s;
    assert (F.fold_range s v_K inv out0 step == F.fold_range (s +! mk_usize 1) v_K inv out1 step);
    // step at s writes chunk s; chunk j (j<s) preserved.
    lemma_cts_step_frame v_K du u out0 s j;
    // recurse: chunk j preserved over [s+1, K).
    lemma_cts_fold_frame v_K du u (s +! mk_usize 1) out1 j
  end
  else ()
#pop-options

(* Bridge: the spec `compress_then_serialize_u_into` equals my replicated fold.
   fold_range never inspects `inv` at runtime, and the spec's step lambda is
   pointwise-equal to `cts_step`; so the OUTPUTS are equal. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_cts_into_eq_fold
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (out0: t_Slice u8 { Seq.length out0 == v v_K * (32 * v du) })
    : Lemma
      (ensures
        S.compress_then_serialize_u_into v_K u du out0
        == F.fold_range (mk_usize 0) v_K (cts_inv v_K du) out0 (cts_step v_K du u))
    =
  assert (S.compress_then_serialize_u_into v_K u du out0
          == F.fold_range (mk_usize 0) v_K (cts_inv v_K du) out0 (cts_step v_K du u))
    by (FStar.Tactics.norm [delta_only [`%S.compress_then_serialize_u_into; `%cts_step; `%cts_inv];
                            zeta; iota; primops];
        FStar.Tactics.trefl ())
#pop-options

(* Per-chunk lemma for the spec fn, in the form the loop invariant produces:
   chunk j == byte_encode (32du) (256du) (compress u[j] du) du. *)
(* Phrased in terms of the FOLD (whose length is known), so the slice is
   well-formed.  Callers compose with lemma_cts_into_eq_fold to switch to the
   spec fn.  Kept separate from the eq to avoid the spec-fn unknown-length
   slice well-formedness cascade. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_cts_into_chunk
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (out0: t_Slice u8 { Seq.length out0 == v v_K * (32 * v du) })
      (j: nat { j < v v_K })
    : Lemma
      (ensures
        Seq.slice (F.fold_range (mk_usize 0) v_K (cts_inv v_K du) out0 (cts_step v_K du u))
          (j * (32 * v du)) ((j + 1) * (32 * v du))
        == S.byte_encode (mk_usize 32 *! du) (mk_usize 256 *! du)
             (Hacspec_ml_kem.Compress.compress (Seq.index u j) du) du)
    =
  lemma_cts_fold_chunk v_K du u (mk_usize 0) out0 j
#pop-options

(* Length of the fold output (== K*32du). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_cts_fold_len
      (v_K du: usize { (v du == 10 \/ v du == 11) /\ v v_K <= 4 })
      (u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (out0: t_Slice u8 { Seq.length out0 == v v_K * (32 * v du) })
    : Lemma
      (ensures
        Seq.length (F.fold_range (mk_usize 0) v_K (cts_inv v_K du) out0 (cts_step v_K du u))
          == v v_K * (32 * v du))
    =
  assert (v P.v_COEFFICIENTS_IN_RING_ELEMENT == 256);
  assert (v (((v_K *! P.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du <: usize) /! mk_usize 8 <: usize)
          == v v_K * (32 * v du));
  let fold_res = F.fold_range (mk_usize 0) v_K (cts_inv v_K du) out0 (cts_step v_K du u) in
  assert (cts_inv v_K du fold_res v_K)
#pop-options

(* NOTE (2026-06-11): the whole-array assembly (out == compress_then_serialize_u_into)
   remains CLIFFED at the LEAF fold->spec-fn chunk transport.  Progress this session:
   - lemma_compress_then_serialize_u_finalize VERIFIES (monolithic, rlimit ~15) when
     phrased over the spec-fn APPLICATION `compress_then_serialize_u_into` (opaque to
     Z3) and fed per-chunk facts about that application — an exact mirror of the proven
     lemma_serialize_vector_finalize.
   - The missing leaf is `lemma_cts_spec_chunk`: transporting one chunk fact from
     `F.fold_range (cts_inv) (cts_step)` (lemma_cts_into_chunk) onto the spec fn via the
     `compress_then_serialize_u_into == F.fold_range` equation (lemma_cts_into_eq_fold).
     Every attempt (fuel 0/1, calc, lemma_mult_le_right, fold_len) fails: referencing the
     raw `F.fold_range ... cts_step` term in consumer code reintroduces the cts_inv-refined
     t_Slice/seq subtyping friction ("Subtyping check failed / incomplete quantifiers",
     rlimit ~6-9 — Z3 gives up, not budget).
   Next attempt: prove the spec-fn-phrased chunk lemma INSIDE the cts substrate via the
   same `norm [delta_only ...]; trefl` tactic that lemma_cts_into_eq_fold uses, so the
   fold term is never named in consumer position; OR wrap the fold in an opaque atom.
   compress_then_serialize_u stays panic_free for now (only consumed by encrypt_c1, which
   is itself deferred as a value+&mut tuple-return).  The leaf chunk lemmas above all verify. *)
