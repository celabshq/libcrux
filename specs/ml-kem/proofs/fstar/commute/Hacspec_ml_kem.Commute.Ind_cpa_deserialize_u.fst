module Hacspec_ml_kem.Commute.Ind_cpa_deserialize_u

/// Bridge lemmas for `Libcrux_ml_kem.Ind_cpa.deserialize_then_decompress_u`.
///
/// The impl loops, per row i:
///   u_as_ntt[i] = ntt_vector_u (deserialize_then_decompress_ring_element_u ciphertext[i*block..(i+1)*block])
/// and the spec target is
///   deserialize_then_decompress_u_then_ntt K (ciphertext[..c1_size K]) du
///     = vector_ntt K (deserialize_then_decompress_u K (ciphertext[..c1_size K]) du)
///     = createi K (fun i -> ntt (decompress (byte_decode_dyn (ct.[i*dps .. i*dps+dps]) du) du)).
///
/// KEYSTONE `lemma_ddu_index`: the createi-index of `deserialize_then_decompress_u`
/// through its OUTER `let du_poly_size = E in createi …`.  The outer let blocks
/// `createi_lemma`'s SMTPat (a `let` sits between `Seq.index` and `createi`); `norm_spec`
/// reduces it and exposes the bare `createi` so the SMTPat fires.  (No t_trefl; the
/// `.[]`-Range index is a red herring — the proven `lemma_vector_decode_12_index` uses it too.)
///
/// REUSES the TARGET-GENERIC machinery in `Ind_cpa_sampling`:
///   `cbd_ntt_{row,prefix}_done` atoms + `lemma_cbd_ntt_{row_intro,prefix_init,prefix_step}`
///   + `lemma_vector_ntt_index`, instantiated with target = the dtdu_then_ntt spec value.

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

module S   = Hacspec_ml_kem.Serialize
module P   = Hacspec_ml_kem.Parameters
module VS  = Libcrux_ml_kem.Vector.Spec
module VV  = Libcrux_ml_kem.Vector
module N   = Hacspec_ml_kem.Ntt
module Br  = Hacspec_ml_kem.Commute.Bridges
module CH  = Hacspec_ml_kem.Commute.Chunk
module Sa  = Hacspec_ml_kem.Commute.Ind_cpa_sampling
module C   = Hacspec_ml_kem.Compress
module U   = Rust_primitives.Hax.Monomorphized_update_at

(* ---- KEYSTONE: createi-index of deserialize_then_decompress_u (norm_spec kills the outer let) ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_ddu_index (v_K du: usize) (ct: t_Slice u8) (i: nat)
  : Lemma
    (requires
      v_K <=. mk_usize 4 /\ (du =. mk_usize 10 \/ du =. mk_usize 11) /\
      (Core_models.Slice.impl__len ct <: usize) =.
        (((v_K *! P.v_COEFFICIENTS_IN_RING_ELEMENT) *! du) /! mk_usize 8) /\
      i < v v_K)
    (ensures
       Seq.index (S.deserialize_then_decompress_u v_K ct du) i ==
       C.decompress
         (S.byte_decode_dyn
            (ct.[ { Core_models.Ops.Range.f_start =
                      (mk_usize i) *! ((P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8);
                    Core_models.Ops.Range.f_end =
                      (mk_usize i) *! ((P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8)
                      +! ((P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8) }
                  <: Core_models.Ops.Range.t_Range usize ]) du) du)
  = assert (i == v (mk_usize i));
    FStar.Pervasives.norm_spec
      [delta_only [`%Hacspec_ml_kem.Serialize.deserialize_then_decompress_u]; zeta; iota; primops]
      (S.deserialize_then_decompress_u v_K ct du)
#pop-options

(* ---- the then_ntt wrapper index: target[i] == ntt (dtdu[i]) ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_dtdu_then_ntt_index (v_K du: usize) (ct: t_Slice u8) (i: nat)
  : Lemma
    (requires
      v_K <=. mk_usize 4 /\ (du =. mk_usize 10 \/ du =. mk_usize 11) /\
      (Core_models.Slice.impl__len ct <: usize) =.
        (((v_K *! P.v_COEFFICIENTS_IN_RING_ELEMENT) *! du) /! mk_usize 8) /\
      i < v v_K)
    (ensures
      Seq.index (S.deserialize_then_decompress_u_then_ntt v_K ct du) i ==
      N.ntt (Seq.index (S.deserialize_then_decompress_u v_K ct du) i))
  = let vec = S.deserialize_then_decompress_u v_K ct du in
    Sa.lemma_vector_ntt_index v_K vec i;
    assert (vec.[ mk_usize i ] == Seq.index vec i)
#pop-options

(* ---- ISOLATED DECOMP-congruence: the dtdru post (in ct_c1.[r_block] form, identical inlined
   range to lemma_ddu_index's ensures) gives poly_to_spec deserialized == Seq.index (dtdu …) i.
   Kept in its OWN lemma so the byte_decode_dyn precond is elaborated once (in the requires) and
   the conclusion is byte_decode-free (the `pivot`), keeping every downstream VC clean. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_ddu_deserialized_eq
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K == 2 \/ v v_K == 3 \/ v v_K == 4})
      (du: usize{v du == 10 \/ v du == 11})
      (ct_c1: t_Slice u8)
      (i: nat{i < v v_K})
      (deserialized: VV.t_PolynomialRingElement vV)
    : Lemma
      (requires
        ((Core_models.Slice.impl__len ct_c1 <: usize) =.
           (((v_K *! P.v_COEFFICIENTS_IN_RING_ELEMENT) *! du) /! mk_usize 8) /\
         VS.poly_to_spec #vV deserialized ==
           C.decompress
             (S.byte_decode_dyn
                (ct_c1.[ { Core_models.Ops.Range.f_start =
                             (mk_usize i) *! ((P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8);
                           Core_models.Ops.Range.f_end =
                             (mk_usize i) *! ((P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8)
                             +! ((P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8) }
                         <: Core_models.Ops.Range.t_Range usize ]) du) du))
      (ensures
        VS.poly_to_spec #vV deserialized ==
        Seq.index (S.deserialize_then_decompress_u v_K ct_c1 du) i)
  = lemma_ddu_index v_K du ct_c1 i
#pop-options

(* ---- per-row bridge (CLEAN: pivot hypothesis, no byte_decode_dyn anywhere) ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ddu_per_index
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K == 2 \/ v v_K == 3 \/ v v_K == 4})
      (du: usize{v du == 10 \/ v du == 11})
      (ct_c1: t_Slice u8)
      (i: nat{i < v v_K})
      (deserialized u_ntt: VV.t_PolynomialRingElement vV)
    : Lemma
      (requires
        ((Core_models.Slice.impl__len ct_c1 <: usize) =.
           (((v_K *! P.v_COEFFICIENTS_IN_RING_ELEMENT) *! du) /! mk_usize 8) /\
         Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) u_ntt /\
         VS.poly_to_spec #vV deserialized ==
           Seq.index (S.deserialize_then_decompress_u v_K ct_c1 du) i /\
         CH.to_spec_poly_plain #vV u_ntt == N.ntt (CH.to_spec_poly_plain #vV deserialized)))
      (ensures
        Sa.cbd_ntt_row_done v_K u_ntt
          (S.deserialize_then_decompress_u_then_ntt v_K ct_c1 du) i)
  = let target = S.deserialize_then_decompress_u_then_ntt v_K ct_c1 du in
    let pivot = Seq.index (S.deserialize_then_decompress_u v_K ct_c1 du) i in
    Br.poly_to_spec_eq_to_spec_poly_plain #vV deserialized;
    Br.poly_to_spec_eq_to_spec_poly_plain #vV u_ntt;
    assert (VS.poly_to_spec #vV u_ntt == N.ntt pivot);
    lemma_dtdu_then_ntt_index v_K du ct_c1 i;
    assert (Seq.index target i == N.ntt pivot);
    Sa.lemma_cbd_ntt_row_intro #vV v_K u_ntt target i
#pop-options

(* ---- finalize: loop-exit prefix atom (n = K) -> bound + vector_to_spec == target.
   Target-generic mirror of Sa.lemma_sample_vector_cbd_then_ntt_post (reuses Sa's atoms). ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ddu_finalize
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (re_as_ntt: t_Array (VV.t_PolynomialRingElement vV) v_K)
    : Lemma
      (requires Sa.cbd_ntt_prefix_done v_K re_as_ntt target (v v_K))
      (ensures
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector v_K #vV (mk_usize 3328) re_as_ntt /\
        VS.vector_to_spec v_K #vV re_as_ntt == target)
  = reveal_opaque (`%Hacspec_ml_kem.Commute.Ind_cpa_sampling.cbd_ntt_prefix_done)
      (Sa.cbd_ntt_prefix_done v_K re_as_ntt target (v v_K));
    let aux (j: nat{j < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index re_as_ntt j) /\
       VS.poly_to_spec #vV (Seq.index re_as_ntt j) == Seq.index target j) =
      reveal_opaque (`%Hacspec_ml_kem.Commute.Ind_cpa_sampling.cbd_ntt_row_done)
        (Sa.cbd_ntt_row_done v_K (Seq.index re_as_ntt j) target j)
    in
    Classical.forall_intro aux;
    let arr_bound (k: usize{v k < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328)
        (re_as_ntt.[ k ] <: VV.t_PolynomialRingElement vV)) = () in
    Classical.forall_intro arr_bound;
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro v_K #vV re_as_ntt
      (mk_usize 3328);
    let auxv (j: nat{j < v v_K}) : Lemma
      (Seq.index (VS.vector_to_spec v_K #vV re_as_ntt) j == Seq.index target j) =
      VS.vector_to_spec_index v_K #vV re_as_ntt j
    in
    Classical.forall_intro auxv;
    Seq.lemma_eq_intro (VS.vector_to_spec v_K #vV re_as_ntt) target
#pop-options

(* ---- slice bridge: the impl's full-ciphertext block == the c1-restricted block (Seq.slice_slice) ---- *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 150"
let lemma_ddu_slice_eq (ciphertext: t_Slice u8) (v_K du: usize) (i: nat)
  : Lemma
    (requires
      (v_K =. mk_usize 2 \/ v_K =. mk_usize 3 \/ v_K =. mk_usize 4) /\
      (du =. mk_usize 10 \/ du =. mk_usize 11) /\
      v du == v (P.vector_u_compression_factor v_K) /\
      (P.c1_size v_K <: usize) <=. (Core_models.Slice.impl__len ciphertext <: usize) /\
      i < v v_K)
    (ensures
      (let block = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8 in
       (ciphertext.[ { Core_models.Ops.Range.f_start = (mk_usize i) *! block;
                       Core_models.Ops.Range.f_end = ((mk_usize i) +! mk_usize 1) *! block }
                     <: Core_models.Ops.Range.t_Range usize ])
       == ((ciphertext.[ { Core_models.Ops.Range.f_end = P.c1_size v_K }
                          <: Core_models.Ops.Range.t_RangeTo usize ])
            .[ { Core_models.Ops.Range.f_start = (mk_usize i) *! block;
                 Core_models.Ops.Range.f_end = (mk_usize i) *! block +! block }
               <: Core_models.Ops.Range.t_Range usize ])))
  = let block = (P.v_COEFFICIENTS_IN_RING_ELEMENT *! du) /! mk_usize 8 in
    assert (v block == v (P.c1_block_size v_K));
    assert (v (P.c1_size v_K) == v v_K * v block);
    assert (i == v (mk_usize i));
    assert (v ((mk_usize i) *! block) == i * v block);
    assert (v (((mk_usize i) +! mk_usize 1) *! block) == (i + 1) * v block);
    assert ((i + 1) * v block <= v v_K * v block);
    Seq.slice_slice ciphertext 0 (v (P.c1_size v_K)) (i * v block) (i * v block + v block)
#pop-options
