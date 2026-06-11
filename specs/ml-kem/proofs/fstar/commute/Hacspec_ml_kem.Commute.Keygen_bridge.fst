module Hacspec_ml_kem.Commute.Keygen_bridge
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P   = Hacspec_ml_kem.Parameters
module HF  = Hacspec_ml_kem.Parameters.Hash_functions
module SU  = Spec.Utils
module HM  = Hacspec_ml_kem.Matrix

(* ════════════════════════════════════════════════════════════════════════
   KEYGEN composition bridges for Libcrux_ml_kem.Ind_cpa.generate_keypair_unpacked.

   The impl samples the matrix A with `transpose = TRUE` and stores it directly
   in f_A; the hacspec reference samples with `transpose = FALSE` (into v_A_as_ntt)
   and keeps it un-transposed. compute_As_plus_e's contract ALREADY re-applies the
   transpose, so BOTH the A-conjunct and the tt-conjunct of the post collapse onto
   ONE spec fact: sample_matrix_A_spec(seed,TRUE) == transpose(sample_matrix_A_spec(
   seed,FALSE)) (+ is_Ok coupling + transpose_involutive).

   The v_G/seed bridges are FO-glue, in the same (assumed, sound) category as
   Ind_cca_bridge.lemma_v_G_bridge / lemma_slice_to_array_id_32.
   ════════════════════════════════════════════════════════════════════════ *)

(* ── transpose is involutive on square (v_K × v_K) matrices ──
   Needed for the tt-conjunct: compute_As_plus_e's post gives
   vector_to_spec(tt) == HM.compute_As_plus_e(transpose(matrix_to_spec A), s, e);
   matrix_to_spec A == transpose(v_A_as_ntt) (transpose lemma below), so the
   transposed argument is transpose(transpose v_A_as_ntt) == v_A_as_ntt. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_transpose_involutive
      (v_K: usize)
      (m: t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
  : Lemma (requires v v_K <= 4)
          (ensures HM.transpose v_K (HM.transpose v_K m) == m)
  = let mt = HM.transpose v_K m in
    let mtt = HM.transpose v_K mt in
    (* mtt[a] == m[a] for each row a, by extensionality over columns b:
       mtt[a][b] == mt[b][a] == m[a][b]  (two createi unfolds each, SMTPat-driven).
       nat indices + mk_usize witnesses so the createi `(v i)` SMTPat can fire. *)
    let aux (a: nat{a < v v_K}) : Lemma (Seq.index mtt a == Seq.index m a) =
      let ai = mk_usize a in
      let inner (b: nat{b < v v_K})
        : Lemma (Seq.index (Seq.index mtt a) b == Seq.index (Seq.index m a) b) =
        let bi = mk_usize b in
        assert (Seq.index (Seq.index mtt (v ai)) (v bi)
                == Seq.index (Seq.index m (v ai)) (v bi)) in
      Classical.forall_intro inner;
      Seq.lemma_eq_intro (Seq.index mtt a) (Seq.index m a)
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro mtt m
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   Generic relational lemma for fold_range_return.

   Two `fold_range_return`s over the SAME range/inv, whose step functions map
   `rel`-related accumulators to control-flow states that AGREE (both Break with
   equal return, both break-continue with rel-related accs, or both Continue with
   rel-related accs), produce rel-related FINAL control-flow states.  Proved by
   induction on (end_ - start), mirroring fold_range_return's own recursion.
   ════════════════════════════════════════════════════════════════════════ *)
module CF = Core_models.Ops.Control_flow

unfold let cf_rel (#a #r: Type0) (rel: a -> a -> Type0)
  (xT xF: CF.t_ControlFlow r a) : Type0 =
  match xT, xF with
  | CF.ControlFlow_Break bT, CF.ControlFlow_Break bF -> bT == bF
  | CF.ControlFlow_Continue cT, CF.ControlFlow_Continue cF -> rel cT cF
  | _, _ -> False

unfold let step_rel (#a #r: Type0) (rel: a -> a -> Type0)
  (xT xF: CF.t_ControlFlow (CF.t_ControlFlow r (unit & a)) a) : Type0 =
  match xT, xF with
  | CF.ControlFlow_Break (CF.ControlFlow_Break bT), CF.ControlFlow_Break (CF.ControlFlow_Break bF) ->
    bT == bF
  | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, cT)),
    CF.ControlFlow_Break (CF.ControlFlow_Continue (_, cF)) -> rel cT cF
  | CF.ControlFlow_Continue cT, CF.ControlFlow_Continue cF -> rel cT cF
  | _, _ -> False

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let rec lemma_frr_rel
      (#acc_t #ret_t: Type0) (#u: inttype)
      (start end_: int_t u)
      (inv: acc_t -> (i:int_t u{Rust_primitives.Hax.Folds.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (rel: acc_t -> acc_t -> Type0)
      (fT fF: (acc:acc_t -> i:int_t u {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i)}
                  -> CF.t_ControlFlow (CF.t_ControlFlow ret_t (unit & acc_t)) acc_t))
      (accT accF: acc_t)
  : Lemma
    (requires
      rel accT accF /\
      (forall (a b: acc_t)
         (i: int_t u {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i)}).
         rel a b ==> step_rel rel (fT a i) (fF b i)))
    (ensures
      cf_rel rel (Rust_primitives.Hax.Folds.fold_range_return start end_ inv accT fT)
                 (Rust_primitives.Hax.Folds.fold_range_return start end_ inv accF fF))
    (decreases (v end_ - v start))
  = if v start < v end_ then begin
      match fT accT start, fF accF start with
      | CF.ControlFlow_Continue aT, CF.ControlFlow_Continue aF ->
        lemma_frr_rel (start +! mk_int 1) end_ inv rel fT fF aT aF
      | _, _ -> ()
    end else ()
#pop-options

(* ── per-cell transpose-write commute (the algebraic core of the transpose lemma) ──
   If a_t == transpose a_f, then writing `s` at a_t[j][i] (the TRUE write slot) and at
   a_f[i][j] (the FALSE write slot) preserves the transpose relation.  This is the inner
   step-commute fact that, plugged into lemma_frr_rel (twice, inner+outer), yields
   lemma_sample_matrix_A_transpose. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_transpose_write
      (v_K: usize)
      (a_t a_f: t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
      (i j: usize)
      (s: t_Array P.t_FieldElement (mk_usize 256))
  : Lemma (requires v v_K <= 4 /\ v i < v v_K /\ v j < v v_K /\ a_t == HM.transpose v_K a_f)
          (ensures
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize a_t j
              (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (a_t.[ j ]) i s)
            == HM.transpose v_K
              (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize a_f i
                (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (a_f.[ i ]) j s)))
  = let at' = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize a_t j
                (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (a_t.[ j ]) i s) in
    let af' = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize a_f i
                (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (a_f.[ i ]) j s) in
    let tf' = HM.transpose v_K af' in
    let aux (a: nat{a < v v_K}) : Lemma (Seq.index at' a == Seq.index tf' a) =
      let ai = mk_usize a in
      let inner (b: nat{b < v v_K})
        : Lemma (Seq.index (Seq.index at' a) b == Seq.index (Seq.index tf' a) b) =
        let bi = mk_usize b in
        (* transpose hypothesis, per-cell: a_t[a][b] == a_f[b][a] *)
        assert (Seq.index (Seq.index a_t (v ai)) (v bi)
                == Seq.index (Seq.index a_f (v bi)) (v ai));
        (* tf'[a][b] == af'[b][a] by createi unfold; at'/af' by Seq.upd case-split *)
        assert (Seq.index (Seq.index tf' (v ai)) (v bi)
                == Seq.index (Seq.index af' (v bi)) (v ai)) in
      Classical.forall_intro inner;
      Seq.lemma_eq_intro (Seq.index at' a) (Seq.index tf' a)
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro at' tf'
#pop-options

(* ── sample_matrix_A transpose bridge (THE hard lemma) ──
   The XOF input is keyed on (i,j) identically for transpose TRUE and FALSE, so the
   two runs hit the rejection-sampling Err on exactly the same (i,j) (is_Ok coupling)
   and the Ok-results are transposes (A[i][j] vs A[j][i] write slot).

   PROOF PLAN (next session): instantiate lemma_frr_rel TWICE with
   rel (a_t,xof_t) (a_f,xof_f) := (a_t == transpose v_K a_f /\ xof_t == xof_f):
     - INNER fold (fixed i): step-commute hypothesis is lemma_transpose_write (Ok branch)
       + same-Err (both Break(Break(Err err)) since same xof => same sample_ntt).
     - OUTER fold: step-commute uses the inner lemma_frr_rel result; outer step maps
       Break ret -> Break(Break ret), Continue -> Continue.
     - Initial rel: zero matrix is its own transpose; xof prefixes equal.
     - Final: Continue (a,_) -> Ok a, Break r -> r gives the is_Ok coupling + value eq.
   The remaining work is the verbatim reconstruction of the two step lambdas (so the
   fold terms match HM.sample_matrix_A's unfolding) — mechanical but ~200 lines. *)
(* ── reconstructed step functions for HM.sample_matrix_A's nested fold_range_return.
   Byte-copied from Hacspec_ml_kem.Matrix.sample_matrix_A (Matrix.fst:207-551), with the
   fold-expected binder refinements made explicit, so that lemma_sma_unfold's
   `norm [delta_only]; trefl` matches HM.sample_matrix_A's unfolding. *)

let triv_inv (v_RANK: usize)
    : (t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_RANK) v_RANK &
       t_Array u8 (mk_usize 34))
      -> (i: usize {Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK false (v i)})
      -> Type0 =
  fun temp_0_ temp_1_ ->
    let
    (v_A_as_ntt:
      t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
        v_RANK),
    (xof_input: t_Array u8 (mk_usize 34)) =
      temp_0_
    in
    let _:usize = temp_1_ in
    true

let sma_inner_step
      (v_RANK: usize)
      (transpose: bool)
      (i: usize {v i < v v_RANK})
      (temp_0_:
        (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK &
          t_Array u8 (mk_usize 34)))
      (j: usize {v j <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v j)})
    : Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Ops.Control_flow.t_ControlFlow
            (Core_models.Result.t_Result
                (t_Array
                    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                    v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
            (Prims.unit &
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                  v_RANK &
                t_Array u8 (mk_usize 34))))
        (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK &
          t_Array u8 (mk_usize 34)) =
  let
  (v_A_as_ntt:
    t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK),
  (xof_input: t_Array u8 (mk_usize 34)) =
    temp_0_
  in
  let j:usize = j in
  let xof_input:t_Array u8 (mk_usize 34) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
      (mk_usize 32)
      (cast (i <: usize) <: u8)
  in
  let xof_input:t_Array u8 (mk_usize 34) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
      (mk_usize 33)
      (cast (j <: usize) <: u8)
  in
  let (xof_bytes: t_Array u8 (mk_usize 840)):t_Array u8 (mk_usize 840) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_XOF (mk_usize 840)
      (xof_input <: t_Slice u8)
  in
  match
    Hacspec_ml_kem.Sampling.sample_ntt (mk_usize 70)
      (mk_usize 560)
      (mk_usize 840)
      (mk_usize 6720)
      xof_bytes
    <:
    Core_models.Result.t_Result
      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok sampled ->
    if transpose
    then
      let v_A_as_ntt:t_Array
        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
            v_RANK) v_RANK =
        Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A_as_ntt
          j
          (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A_as_ntt.[
                  j ]
                <:
                t_Array
                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                  v_RANK)
              i
              sampled
            <:
            t_Array
              (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
              v_RANK)
      in
      Core_models.Ops.Control_flow.ControlFlow_Continue
      (v_A_as_ntt, xof_input
        <:
        (t_Array
            (t_Array
                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                v_RANK) v_RANK &
          t_Array u8 (mk_usize 34)))
      <:
      Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Ops.Control_flow.t_ControlFlow
            (Core_models.Result.t_Result
                (t_Array
                    (t_Array
                        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                            (mk_usize 256)) v_RANK) v_RANK)
                Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
            (Prims.unit &
              (t_Array
                  (t_Array
                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                          (mk_usize 256)) v_RANK) v_RANK &
                t_Array u8 (mk_usize 34))))
        (t_Array
            (t_Array
                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                v_RANK) v_RANK &
          t_Array u8 (mk_usize 34))
    else
      let v_A_as_ntt:t_Array
        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
            v_RANK) v_RANK =
        Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A_as_ntt
          i
          (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A_as_ntt.[
                  i ]
                <:
                t_Array
                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                  v_RANK)
              j
              sampled
            <:
            t_Array
              (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
              v_RANK)
      in
      Core_models.Ops.Control_flow.ControlFlow_Continue
      (v_A_as_ntt, xof_input
        <:
        (t_Array
            (t_Array
                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                v_RANK) v_RANK &
          t_Array u8 (mk_usize 34)))
      <:
      Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Ops.Control_flow.t_ControlFlow
            (Core_models.Result.t_Result
                (t_Array
                    (t_Array
                        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                            (mk_usize 256)) v_RANK) v_RANK)
                Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
            (Prims.unit &
              (t_Array
                  (t_Array
                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                          (mk_usize 256)) v_RANK) v_RANK &
                t_Array u8 (mk_usize 34))))
        (t_Array
            (t_Array
                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                v_RANK) v_RANK &
          t_Array u8 (mk_usize 34))
  | Core_models.Result.Result_Err err ->
    Core_models.Ops.Control_flow.ControlFlow_Break
    (Core_models.Ops.Control_flow.ControlFlow_Break
      (Core_models.Result.Result_Err err
        <:
        Core_models.Result.t_Result
          (t_Array
              (t_Array
                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                  v_RANK) v_RANK)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      <:
      Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Result.t_Result
            (t_Array
                (t_Array
                    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
                    ) v_RANK) v_RANK)
            Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
        (Prims.unit &
          (t_Array
              (t_Array
                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                  v_RANK) v_RANK &
            t_Array u8 (mk_usize 34))))
    <:
    Core_models.Ops.Control_flow.t_ControlFlow
      (Core_models.Ops.Control_flow.t_ControlFlow
          (Core_models.Result.t_Result
              (t_Array
                  (t_Array
                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                          (mk_usize 256)) v_RANK) v_RANK)
              Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
          (Prims.unit &
            (t_Array
                (t_Array
                    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
                    ) v_RANK) v_RANK &
              t_Array u8 (mk_usize 34))))
      (t_Array
          (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
              v_RANK) v_RANK &
        t_Array u8 (mk_usize 34))

let sma_outer_step
      (v_RANK: usize)
      (transpose: bool)
      (temp_0_:
        (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK &
          t_Array u8 (mk_usize 34)))
      (i: usize {v i <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v i)})
    : Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Ops.Control_flow.t_ControlFlow
            (Core_models.Result.t_Result
                (t_Array
                    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                    v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
            (Prims.unit &
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                  v_RANK &
                t_Array u8 (mk_usize 34))))
        (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK &
          t_Array u8 (mk_usize 34)) =
  let
  (v_A_as_ntt:
    t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK),
  (xof_input: t_Array u8 (mk_usize 34)) =
    temp_0_
  in
  let i:usize = i in
  match
    Rust_primitives.Hax.Folds.fold_range_return (mk_usize 0)
      v_RANK
      (triv_inv v_RANK)
      (v_A_as_ntt, xof_input
        <:
        (t_Array
            (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK &
          t_Array u8 (mk_usize 34)))
      (sma_inner_step v_RANK transpose i)
    <:
    Core_models.Ops.Control_flow.t_ControlFlow
      (Core_models.Result.t_Result
          (t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 34))
  with
  | Core_models.Ops.Control_flow.ControlFlow_Break ret ->
    Core_models.Ops.Control_flow.ControlFlow_Break
    (Core_models.Ops.Control_flow.ControlFlow_Break ret
      <:
      Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Result.t_Result
            (t_Array
                (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                    v_RANK) v_RANK)
            Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
        (Prims.unit &
          (t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                  v_RANK) v_RANK &
            t_Array u8 (mk_usize 34))))
    <:
    Core_models.Ops.Control_flow.t_ControlFlow
      (Core_models.Ops.Control_flow.t_ControlFlow
          (Core_models.Result.t_Result
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                      v_RANK) v_RANK)
              Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
          (Prims.unit &
            (t_Array
                (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                    v_RANK) v_RANK &
              t_Array u8 (mk_usize 34))))
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 34))
  | Core_models.Ops.Control_flow.ControlFlow_Continue loop_res ->
    Core_models.Ops.Control_flow.ControlFlow_Continue loop_res
    <:
    Core_models.Ops.Control_flow.t_ControlFlow
      (Core_models.Ops.Control_flow.t_ControlFlow
          (Core_models.Result.t_Result
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                      v_RANK) v_RANK)
              Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
          (Prims.unit &
            (t_Array
                (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                    v_RANK) v_RANK &
              t_Array u8 (mk_usize 34))))
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 34))

let sma_init_A (v_RANK: usize)
    : t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_RANK) v_RANK =
  Rust_primitives.Hax.repeat (Rust_primitives.Hax.repeat (Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new
                (mk_u16 0)
              <:
              Hacspec_ml_kem.Parameters.t_FieldElement)
            (mk_usize 256)
          <:
          t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
        v_RANK
      <:
      t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    v_RANK

let sma_init_xof (seed_for_A: t_Slice u8 {Seq.length seed_for_A == 32}) : t_Array u8 (mk_usize 34) =
  let xof_input:t_Array u8 (mk_usize 34) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 34) in
  let xof_input:t_Array u8 (mk_usize 34) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to xof_input
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (xof_input.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          seed_for_A
        <:
        t_Slice u8)
  in
  xof_input

(* ── trefl: HM.sample_matrix_A unfolds to the named-step fold. ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_sma_unfold (v_RANK: usize) (seed_for_A: t_Slice u8) (transpose: bool)
  : Lemma (requires Seq.length seed_for_A == 32 /\ v v_RANK <= 4)
          (ensures
            HM.sample_matrix_A v_RANK seed_for_A transpose ==
            (match Rust_primitives.Hax.Folds.fold_range_return (mk_usize 0) v_RANK (triv_inv v_RANK)
                     (sma_init_A v_RANK, sma_init_xof seed_for_A) (sma_outer_step v_RANK transpose)
             with
             | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
             | Core_models.Ops.Control_flow.ControlFlow_Continue (v_A_as_ntt, xof_input) ->
               Core_models.Result.Result_Ok v_A_as_ntt))
  = assert (HM.sample_matrix_A v_RANK seed_for_A transpose ==
            (match Rust_primitives.Hax.Folds.fold_range_return (mk_usize 0) v_RANK (triv_inv v_RANK)
                     (sma_init_A v_RANK, sma_init_xof seed_for_A) (sma_outer_step v_RANK transpose)
             with
             | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
             | Core_models.Ops.Control_flow.ControlFlow_Continue (v_A_as_ntt, xof_input) ->
               Core_models.Result.Result_Ok v_A_as_ntt))
      by (FStar.Tactics.norm [delta_only [`%HM.sample_matrix_A; `%sma_outer_step; `%sma_inner_step;
                                          `%triv_inv; `%sma_init_A; `%sma_init_xof];
                              zeta; iota; primops];
          FStar.Tactics.trefl ())
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   Relational machinery: relate the TRUE and FALSE runs of sample_matrix_A.
   ════════════════════════════════════════════════════════════════════════ *)
unfold let mat_ty (v_RANK: usize) =
  t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_RANK) v_RANK
unfold let acc_ty (v_RANK: usize) = (mat_ty v_RANK & t_Array u8 (mk_usize 34))
unfold let res_ty (v_RANK: usize) =
  Core_models.Result.t_Result (mat_ty v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

(* the coupling: TRUE matrix is the transpose of the FALSE matrix; xof states agree *)
let rel_pred (v_RANK: usize) (p q: acc_ty v_RANK) : Type0 =
  let ap, xp = p in
  let aq, xq = q in
  ap == HM.transpose v_RANK aq /\ xp == xq

(* break value predicate: the result is an Err (used to rule out a Break carrying Ok) *)
let is_err_pred (v_RANK: usize) (r: res_ty v_RANK) : Type0 =
  Core_models.Result.impl__is_ok r == false

(* ── zero matrix is its own transpose (the initial rel) ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_sma_init_self_transpose (v_RANK: usize)
  : Lemma (requires v v_RANK <= 4)
          (ensures sma_init_A v_RANK == HM.transpose v_RANK (sma_init_A v_RANK))
  = let m = sma_init_A v_RANK in
    let mt = HM.transpose v_RANK m in
    let aux (a: nat{a < v v_RANK}) : Lemma (Seq.index mt a == Seq.index m a) =
      let ai = mk_usize a in
      let inner (b: nat{b < v v_RANK})
        : Lemma (Seq.index (Seq.index mt a) b == Seq.index (Seq.index m a) b) =
        let bi = mk_usize b in
        (* transpose unfold (createi SMTPat): mt[a][b] == m[b][a]; m is constant so m[b][a]==m[a][b] *)
        assert (Seq.index (Seq.index mt (v ai)) (v bi) == Seq.index (Seq.index m (v bi)) (v ai)) in
      Classical.forall_intro inner;
      Seq.lemma_eq_intro (Seq.index mt a) (Seq.index m a)
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro mt m
#pop-options

(* ── inner step-commute: rel-related accs ⟹ step_rel-related inner-step results ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 250"
let lemma_inner_commute
      (v_RANK: usize) (i: usize)
      (a b: acc_ty v_RANK)
      (j: usize {v j <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v j)})
  : Lemma (requires v v_RANK <= 4 /\ v i < v v_RANK /\ rel_pred v_RANK a b)
          (ensures step_rel (rel_pred v_RANK)
                            (sma_inner_step v_RANK true i a j)
                            (sma_inner_step v_RANK false i b j))
  = let at, xt = a in
    let af, xf = b in
    let xof1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xt (mk_usize 32) (cast (i <: usize) <: u8) in
    let xof2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof1 (mk_usize 33) (cast (j <: usize) <: u8) in
    let xof_bytes = Hacspec_ml_kem.Parameters.Hash_functions.v_XOF (mk_usize 840) (xof2 <: t_Slice u8) in
    match Hacspec_ml_kem.Sampling.sample_ntt (mk_usize 70) (mk_usize 560) (mk_usize 840)
            (mk_usize 6720) xof_bytes with
    | Core_models.Result.Result_Ok sampled -> lemma_transpose_write v_RANK at af i j sampled
    | Core_models.Result.Result_Err err -> ()
#pop-options

(* ── outer step-commute: via lemma_frr_rel applied to the inner fold ── *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_outer_commute
      (v_RANK: usize)
      (a b: acc_ty v_RANK)
      (i: usize {v i <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v i)})
  : Lemma (requires v v_RANK <= 4 /\ rel_pred v_RANK a b)
          (ensures step_rel (rel_pred v_RANK)
                            (sma_outer_step v_RANK true a i)
                            (sma_outer_step v_RANK false b i))
  = let at, xt = a in
    let af, xf = b in
    introduce forall (p q: acc_ty v_RANK)
                     (jj: usize {v jj <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v jj)}).
              rel_pred v_RANK p q ==>
              step_rel (rel_pred v_RANK) (sma_inner_step v_RANK true i p jj) (sma_inner_step v_RANK false i q jj)
    with introduce _ ==> _
      with _pf. lemma_inner_commute v_RANK i p q jj;
    lemma_frr_rel (mk_usize 0) v_RANK (triv_inv v_RANK) (rel_pred v_RANK)
      (sma_inner_step v_RANK true i) (sma_inner_step v_RANK false i)
      (at, xt) (af, xf)
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   Break-is-Err machinery: the fold's Break value is always a Result_Err, so a
   Break can never carry Result_Ok.  Mirrors lemma_frr_rel's recursion.
   ════════════════════════════════════════════════════════════════════════ *)
unfold let cf_break_q (#acc_t #ret_t: Type0) (qq: ret_t -> Type0)
  (x: CF.t_ControlFlow ret_t acc_t) : Type0 =
  match x with
  | CF.ControlFlow_Break r -> qq r
  | CF.ControlFlow_Continue _ -> True

unfold let step_break_q (#acc_t #ret_t: Type0) (qq: ret_t -> Type0)
  (x: CF.t_ControlFlow (CF.t_ControlFlow ret_t (unit & acc_t)) acc_t) : Type0 =
  match x with
  | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> qq r
  | CF.ControlFlow_Break (CF.ControlFlow_Continue _) -> False
  | CF.ControlFlow_Continue _ -> True

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let rec lemma_frr_break_q
      (#acc_t #ret_t: Type0) (#u: inttype)
      (start end_: int_t u)
      (inv: acc_t -> (i:int_t u{Rust_primitives.Hax.Folds.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (qq: ret_t -> Type0)
      (f: (acc:acc_t -> i:int_t u {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i)}
            -> CF.t_ControlFlow (CF.t_ControlFlow ret_t (unit & acc_t)) acc_t))
      (acc: acc_t)
  : Lemma
    (requires
      (forall (a: acc_t) (i: int_t u {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i)}).
         step_break_q qq (f a i)))
    (ensures cf_break_q qq (Rust_primitives.Hax.Folds.fold_range_return start end_ inv acc f))
    (decreases (v end_ - v start))
  = if v start < v end_ then begin
      match f acc start with
      | CF.ControlFlow_Continue a -> lemma_frr_break_q (start +! mk_int 1) end_ inv qq f a
      | _ -> ()
    end else ()
#pop-options

(* inner step never breaks with Ok *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_inner_step_break
      (v_RANK: usize) (transpose: bool) (i: usize {v i < v v_RANK})
      (a: acc_ty v_RANK)
      (j: usize {v j <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v j)})
  : Lemma (ensures step_break_q (is_err_pred v_RANK) (sma_inner_step v_RANK transpose i a j))
  = let at, xt = a in
    let xof1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xt (mk_usize 32) (cast (i <: usize) <: u8) in
    let xof2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof1 (mk_usize 33) (cast (j <: usize) <: u8) in
    let xof_bytes = Hacspec_ml_kem.Parameters.Hash_functions.v_XOF (mk_usize 840) (xof2 <: t_Slice u8) in
    match Hacspec_ml_kem.Sampling.sample_ntt (mk_usize 70) (mk_usize 560) (mk_usize 840)
            (mk_usize 6720) xof_bytes with
    | Core_models.Result.Result_Ok sampled -> ()
    | Core_models.Result.Result_Err err -> ()
#pop-options

(* outer step never breaks with Ok (its break value = inner fold's break value) *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_outer_step_break
      (v_RANK: usize) (transpose: bool)
      (a: acc_ty v_RANK)
      (i: usize {v i <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v i)})
  : Lemma (ensures step_break_q (is_err_pred v_RANK) (sma_outer_step v_RANK transpose a i))
  = let at, xt = a in
    introduce forall (p: acc_ty v_RANK)
                     (jj: usize {v jj <= v v_RANK /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_RANK true (v jj)}).
              step_break_q (is_err_pred v_RANK) (sma_inner_step v_RANK transpose i p jj)
    with lemma_inner_step_break v_RANK transpose i p jj;
    lemma_frr_break_q (mk_usize 0) v_RANK (triv_inv v_RANK) (is_err_pred v_RANK)
      (sma_inner_step v_RANK transpose i) (at, xt)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400 --split_queries always"
let lemma_sample_matrix_A_transpose
      (v_K: usize)
      (seed: t_Slice u8)
  : Lemma (requires Seq.length seed == 32 /\ v v_K <= 4)
          (ensures
            (Core_models.Result.impl__is_ok (HM.sample_matrix_A v_K seed true) ==
             Core_models.Result.impl__is_ok (HM.sample_matrix_A v_K seed false)) /\
            (match HM.sample_matrix_A v_K seed false with
             | Core_models.Result.Result_Ok aF ->
               HM.sample_matrix_A v_K seed true ==
                 (Core_models.Result.Result_Ok (HM.transpose v_K aF)
                   <: Core_models.Result.t_Result
                        (t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
                        Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
             | Core_models.Result.Result_Err _ -> True))
  = lemma_sma_unfold v_K seed true;
    lemma_sma_unfold v_K seed false;
    lemma_sma_init_self_transpose v_K;
    (* outer step-commute hypothesis for lemma_frr_rel *)
    introduce forall (a b: acc_ty v_K)
                     (i: usize {v i <= v v_K /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_K true (v i)}).
              rel_pred v_K a b ==>
              step_rel (rel_pred v_K) (sma_outer_step v_K true a i) (sma_outer_step v_K false b i)
    with introduce _ ==> _
      with _pf. lemma_outer_commute v_K a b i;
    lemma_frr_rel (mk_usize 0) v_K (triv_inv v_K) (rel_pred v_K)
      (sma_outer_step v_K true) (sma_outer_step v_K false)
      (sma_init_A v_K, sma_init_xof seed) (sma_init_A v_K, sma_init_xof seed);
    (* the FALSE outer fold never breaks with Ok *)
    introduce forall (p: acc_ty v_K)
                     (ii: usize {v ii <= v v_K /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_usize 0) v_K true (v ii)}).
              step_break_q (is_err_pred v_K) (sma_outer_step v_K false p ii)
    with lemma_outer_step_break v_K false p ii;
    lemma_frr_break_q (mk_usize 0) v_K (triv_inv v_K) (is_err_pred v_K)
      (sma_outer_step v_K false) (sma_init_A v_K, sma_init_xof seed)
#pop-options

(* ── construction bridge: the spec's 33-byte g_input (= seed ‖ [cast K], built by
   repeat 0; copy seed into [..32]; write cast K at [32]) equals Seq.append seed
   [cast K].  Lets HF.v_G(g_input) be rewritten to HF.v_G(append seed [cast K]) so
   it can meet the Variant post (SU.v_G(append seed [cast K])) via lemma_v_G_bridge.
   Mirrors Ind_cca_bridge.lemma_dk_build.  The g_input term is copied VERBATIM from
   Hacspec_ml_kem.Ind_cpa.generate_keypair_unpacked so it matches definitionally. *)
let lemma_g_input_build (v_K: usize) (seed: t_Slice u8)
  : Lemma (requires Seq.length seed == 32)
          (ensures
            (let g0:t_Array u8 (mk_usize 33) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33) in
             let g1:t_Array u8 (mk_usize 33) =
               Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to g0
                 ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
                 (Core_models.Slice.impl__copy_from_slice #u8
                     (g0.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
                         <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
                     seed
                   <: t_Slice u8)
             in
             let g2:t_Array u8 (mk_usize 33) =
               Rust_primitives.Hax.Monomorphized_update_at.update_at_usize g1
                 (mk_usize 32)
                 (cast (v_K <: usize) <: u8)
             in
             (g2 <: t_Slice u8) == Seq.append seed (Seq.create 1 (cast v_K <: u8))))
  = let g0:t_Array u8 (mk_usize 33) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33) in
    let c1:t_Slice u8 =
      Core_models.Slice.impl__copy_from_slice #u8
        (g0.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
            <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
        seed
    in
    assert (c1 == seed);
    let g1:t_Array u8 (mk_usize 33) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to g0
        ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
        c1
    in
    let g2:t_Array u8 (mk_usize 33) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize g1
        (mk_usize 32) (cast (v_K <: usize) <: u8)
    in
    (* g1 writes seed into [0,32); g2 = Seq.upd g1 32 (cast K). *)
    assert (Seq.slice g1 0 32 == c1);
    assert (Seq.slice g2 0 32 `Seq.equal` seed);
    assert (Seq.slice g2 32 33 `Seq.equal` (Seq.create 1 (cast v_K <: u8)));
    Rust_primitives.Arrays.lemma_slice_append (g2 <: t_Slice u8) seed
      (Seq.create 1 (cast v_K <: u8))

(* ── seed_for_A bridge: impl writes unwrap(try_into s); spec writes
   copy_from_slice(repeat 0 32, s).  For a 32-byte slice s both denote s. ── *)
let lemma_seed_for_A_eq (s: t_Slice u8)
  : Lemma (requires Seq.length s == 32)
          (ensures
            (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
                #Core_models.Array.t_TryFromSliceError
                (Core_models.Convert.f_try_into #(t_Slice u8)
                    #(t_Array u8 (mk_usize 32))
                    #FStar.Tactics.Typeclasses.solve
                    s)
              <: t_Array u8 (mk_usize 32)) ==
            (Core_models.Slice.impl__copy_from_slice #u8
                (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32))
                s
              <: t_Array u8 (mk_usize 32)))
  = (* unwrap(try_into s) == s (slice/array coercion glue); copy_from_slice out == src. *)
    Hacspec_ml_kem.Commute.Ind_cca_bridge.lemma_slice_to_array_id_32 s
