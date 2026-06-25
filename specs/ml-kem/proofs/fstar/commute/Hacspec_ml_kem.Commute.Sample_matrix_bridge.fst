module Hacspec_ml_kem.Commute.Sample_matrix_bridge
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P   = Hacspec_ml_kem.Parameters
module HF  = Hacspec_ml_kem.Parameters.Hash_functions
module HM  = Hacspec_ml_kem.Matrix
module HS  = Hacspec_ml_kem.Sampling
module VS  = Libcrux_ml_kem.Vector.Spec
module VV  = Libcrux_ml_kem.Vector
module T   = Libcrux_ml_kem.Vector.Traits
module KB  = Hacspec_ml_kem.Commute.Keygen_bridge
module CF  = Core_models.Ops.Control_flow
module R   = Core_models.Result
module F   = Rust_primitives.Hax.Folds
module UA  = Rust_primitives.Hax.Monomorphized_update_at

(* ════════════════════════════════════════════════════════════════════════
   FULL functional verification of Libcrux_ml_kem.Matrix.sample_matrix_A.

   Goal: matrix_to_spec A == HM.sample_matrix_A seed[..32] transpose  (Ok case).

   The impl fills A entry-by-entry: at outer iter i it samples sampled[j] from
   XOF(seed‖i‖j) (sample_from_xof), and writes (transpose) A[j][i]=sampled[j]
   else A[i][j]=sampled[j].  The hacspec is a nested fold_range_return over (i,j)
   that short-circuits to Err on the first rejection-sampling failure.

   Design (mirror Compute_dot_bridge.vdot_done):
   - PART 1: a per-entry characterization of the HACSPEC result `lemma_sma_entry`
     (the standalone fold reconstruction), via a generic forward-invariant lemma
     `lemma_frr_inv` applied to the inner and outer folds (reusing Keygen_bridge's
     reconstructed step functions + lemma_sma_unfold).
   - PART 2: an opaque atom `sma_done_ij` tracking "A's done (i,j)-prefix matches
     the hacspec sm's prefix", with base/inner_step/outer_row/finalize lemmas
     (reveal only inside them), threaded through sample_matrix_A's two loops.
   ════════════════════════════════════════════════════════════════════════ *)

(* short type abbreviations (hacspec side: FieldElement matrices) *)
unfold let poly_t = t_Array P.t_FieldElement (mk_usize 256)
unfold let sample_res = R.t_Result poly_t HS.t_BadRejectionSamplingRandomnessError

(* ════════════════ helpers: xof input, write slot, the per-(i,j) sample ════════════════ *)

(* the 34-byte XOF input for entry (i,j): seed[0..32] ‖ cast i ‖ cast j.
   Built on an append base (Z3-transparent indices) rather than copy_from_slice. *)
let sma_xof_base (seed: t_Slice u8 {Seq.length seed == 32}) : t_Array u8 (mk_usize 34) =
  Seq.append seed (Seq.create 2 (mk_u8 0))

let sma_xof_input (seed: t_Slice u8 {Seq.length seed == 32}) (i j: usize) : t_Array u8 (mk_usize 34) =
  let x0 : t_Array u8 (mk_usize 34) = sma_xof_base seed in
  let x1 = UA.update_at_usize x0 (mk_usize 32) (cast (i <: usize) <: u8) in
  let x2 = UA.update_at_usize x1 (mk_usize 33) (cast (j <: usize) <: u8) in
  x2

(* the sample for entry (i,j): sample_ntt(XOF(seed‖i‖j)) — matches the hacspec inner step
   and (via lemma_sma_seeds_step) the impl's sample_from_xof input. *)
let sma_sample (seed: t_Slice u8 {Seq.length seed == 32}) (i j: usize) : sample_res =
  HS.sample_ntt (mk_usize 70) (mk_usize 560) (mk_usize 840) (mk_usize 6720)
    (HF.v_XOF (mk_usize 840) (sma_xof_input seed i j <: t_Slice u8))

(* write slot of (i,j): transpose => A[j][i], else A[i][j] *)
unfold let slot_row (transpose: bool) (i j: usize) : usize = if transpose then j else i
unfold let slot_col (transpose: bool) (i j: usize) : usize = if transpose then i else j

(* ════════════════ helper: impl seeds[j] == sma_xof_input (for matrix.rs seeds loop) ════════════════ *)

(* The impl builds seeds[j] = clone(seed) with [32]:=cast i, [33]:=cast j.  Show this 34-byte
   array equals sma_xof_input (seed[0..32]) i j.  (seed here is the full 34-byte impl seed.) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_sma_seeds_step (seed: t_Array u8 (mk_usize 34)) (i j: usize)
  : Lemma
    (ensures
      (let s32 = (seed.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
                    <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8) in
       UA.update_at_usize (UA.update_at_usize seed (mk_usize 32) (cast (i <: usize) <: u8))
                          (mk_usize 33) (cast (j <: usize) <: u8)
       == sma_xof_input s32 i j))
= let s32 = (seed.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8) in
  let lhs = UA.update_at_usize (UA.update_at_usize seed (mk_usize 32) (cast (i <: usize) <: u8))
                               (mk_usize 33) (cast (j <: usize) <: u8) in
  let rhs = sma_xof_input s32 i j in
  let base = sma_xof_base s32 in
  (* both: [k<32] = seed[k]; [32] = cast i; [33] = cast j.  rhs's [0,32) = base[0,32) = s32 = seed[0..32]. *)
  assert (Seq.length s32 == 32);
  let aux (k: nat {k < 34}) : Lemma (Seq.index lhs k == Seq.index rhs k) =
    if k < 32 then begin
      assert (Seq.index base k == Seq.index s32 k);
      assert (Seq.index s32 k == Seq.index seed k)
    end else () in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   PART 1.a — generic forward invariant for fold_range_return.

   p (v start) acc + (p maintained on Continue, qq established on Break(Break))
   ==> the fold's Continue result satisfies p (v end_), and any Break result
   satisfies qq.  Mirrors KB.lemma_frr_break_q's recursion.
   ════════════════════════════════════════════════════════════════════════ *)
unfold let cf_inv_post (#acc_t #ret_t: Type0) (p: int -> acc_t -> Type0) (qq: ret_t -> Type0)
  (e: int) (x: CF.t_ControlFlow ret_t acc_t) : Type0 =
  match x with
  | CF.ControlFlow_Continue a' -> p e a'
  | CF.ControlFlow_Break r -> qq r

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let rec lemma_frr_inv
      (#acc_t #ret_t: Type0) (#u: inttype)
      (start end_: int_t u)
      (inv: acc_t -> (i:int_t u{F.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (p: int -> acc_t -> Type0)
      (qq: ret_t -> Type0)
      (f: (acc:acc_t -> i:int_t u {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i)}
            -> CF.t_ControlFlow (CF.t_ControlFlow ret_t (unit & acc_t)) acc_t))
      (acc: acc_t)
  : Lemma
    (requires
      v start <= v end_ /\
      p (v start) acc /\
      (forall (a: acc_t) (i: int_t u {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i)}).
         p (v i) a ==>
           (match f a i with
            | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> qq r
            | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a')) -> False
            | CF.ControlFlow_Continue a' -> p (v i + 1) a')))
    (ensures cf_inv_post p qq (v end_) (F.fold_range_return start end_ inv acc f))
    (decreases (v end_ - v start))
= if v start < v end_ then begin
    (* instantiate the maintenance hypothesis at (acc, start) *)
    assert (match f acc start with
            | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> qq r
            | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a2)) -> False
            | CF.ControlFlow_Continue a2 -> p (v start + 1) a2);
    match f acc start with
    | CF.ControlFlow_Continue a' ->
      lemma_frr_inv (start +! mk_int 1) end_ inv p qq f a';
      assert (F.fold_range_return start end_ inv acc f
              == F.fold_range_return (start +! mk_int 1) end_ inv a' f)
    | CF.ControlFlow_Break (CF.ControlFlow_Break r) ->
      assert (F.fold_range_return start end_ inv acc f == CF.ControlFlow_Break r)
    | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a')) -> ()
  end else ()
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   PART 1.b — the hacspec inner step's xof2 equals sma_xof_input (given the
   slice invariant), so its sample equals sma_sample.
   ════════════════════════════════════════════════════════════════════════ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_sma_xof2_eq
      (seed: t_Slice u8 {Seq.length seed == 32})
      (xof: t_Array u8 (mk_usize 34) {Seq.equal (Seq.slice xof 0 32) seed})
      (i j: usize)
  : Lemma
    (ensures
      (let xof1 = UA.update_at_usize xof (mk_usize 32) (cast (i <: usize) <: u8) in
       let xof2 = UA.update_at_usize xof1 (mk_usize 33) (cast (j <: usize) <: u8) in
       xof2 == sma_xof_input seed i j /\ Seq.equal (Seq.slice xof2 0 32) seed))
= let xof1 = UA.update_at_usize xof (mk_usize 32) (cast (i <: usize) <: u8) in
  let xof2 = UA.update_at_usize xof1 (mk_usize 33) (cast (j <: usize) <: u8) in
  let rhs = sma_xof_input seed i j in
  let base = sma_xof_base seed in
  let aux (k: nat {k < 34}) : Lemma (Seq.index xof2 k == Seq.index rhs k) =
    if k < 32 then begin
      assert (Seq.index (Seq.slice xof 0 32) k == Seq.index xof k);
      assert (Seq.index (Seq.slice xof 0 32) k == Seq.index seed k);
      assert (Seq.index base k == Seq.index seed k)
    end else () in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro xof2 rhs;
  let aux2 (k: nat {k < 32}) : Lemma (Seq.index (Seq.slice xof2 0 32) k == Seq.index seed k) =
    assert (Seq.index (Seq.slice xof2 0 32) k == Seq.index xof2 k);
    assert (Seq.index (Seq.slice xof 0 32) k == Seq.index seed k) in
  Classical.forall_intro aux2;
  Seq.lemma_eq_intro (Seq.slice xof2 0 32) seed
#pop-options

(* clean-context per-entry value of a double update_at (= Seq.upd) on a square matrix. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_matrix_upd_entry
      (#x_t: Type0) (#v_K: usize)
      (a: t_Array (t_Array x_t v_K) v_K)
      (r c: usize {v r < v v_K /\ v c < v v_K}) (x: x_t)
      (a2 b2: nat {a2 < v v_K /\ b2 < v v_K})
  : Lemma
    (ensures
      Seq.index (Seq.index (UA.update_at_usize a r (UA.update_at_usize (a.[ r ]) c x)) a2) b2
      == (if a2 = v r && b2 = v c then x else Seq.index (Seq.index a a2) b2))
= ()
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   PART 1.c — inner-row characterization (fixed i).
   ════════════════════════════════════════════════════════════════════════ *)

(* "entry (a,b) was written by the inner row i up to (exclusive) j_done" *)
unfold let written_upto (transpose: bool) (i: usize) (j_done: int) (a b: nat) : bool =
  if transpose then (b = v i && a < j_done) else (a = v i && b < j_done)

(* the inner-fold invariant for row i, over accumulator (A, xof). *)
unfold let inner_p
      (v_RANK: usize) (transpose: bool) (seed: t_Slice u8 {Seq.length seed == 32})
      (i: usize {v i < v v_RANK}) (a0: KB.mat_ty v_RANK)
      (j_done: int) (acc: KB.acc_ty v_RANK)
  : Type0
  = let (a, xof) = acc in
    Seq.equal (Seq.slice xof 0 32) seed /\
    (forall (jj: usize). (v jj < j_done /\ v jj < v v_RANK) ==>
       (match sma_sample seed i jj with
        | R.Result_Ok s ->
          Seq.index (Seq.index a (v (slot_row transpose i jj))) (v (slot_col transpose i jj)) == s
        | R.Result_Err _ -> False)) /\
    (forall (a2 b2: nat). (a2 < v v_RANK /\ b2 < v v_RANK /\ ~(written_upto transpose i j_done a2 b2)) ==>
       Seq.index (Seq.index a a2) b2 == Seq.index (Seq.index a0 a2) b2)

(* clean-context: extend the inner done-forall from jj to jj+1 after writing slot (i,jj). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_inner_done_extend
      (v_RANK: usize) (transpose: bool) (seed: t_Slice u8 {Seq.length seed == 32})
      (i: usize {v i < v v_RANK}) (a a': KB.mat_ty v_RANK) (sampled: poly_t)
      (jj: usize {v jj < v v_RANK})
  : Lemma
    (requires
      v v_RANK <= 4 /\
      sma_sample seed i jj == R.Result_Ok sampled /\
      a' == UA.update_at_usize a (slot_row transpose i jj)
              (UA.update_at_usize (a.[ slot_row transpose i jj ]) (slot_col transpose i jj) sampled) /\
      (forall (jx: usize). (v jx < v jj /\ v jx < v v_RANK) ==>
         (match sma_sample seed i jx with
          | R.Result_Ok s ->
            Seq.index (Seq.index a (v (slot_row transpose i jx))) (v (slot_col transpose i jx)) == s
          | R.Result_Err _ -> False)))
    (ensures
      (forall (jx: usize). (v jx < v jj + 1 /\ v jx < v v_RANK) ==>
         (match sma_sample seed i jx with
          | R.Result_Ok s ->
            Seq.index (Seq.index a' (v (slot_row transpose i jx))) (v (slot_col transpose i jx)) == s
          | R.Result_Err _ -> False)))
= introduce forall (jx: usize). (v jx < v jj + 1 /\ v jx < v v_RANK) ==>
    (match sma_sample seed i jx with
     | R.Result_Ok s ->
       Seq.index (Seq.index a' (v (slot_row transpose i jx))) (v (slot_col transpose i jx)) == s
     | R.Result_Err _ -> False)
  with introduce _ ==> _ with _.
    lemma_matrix_upd_entry #poly_t #v_RANK a (slot_row transpose i jj) (slot_col transpose i jj)
      sampled (v (slot_row transpose i jx)) (v (slot_col transpose i jx))
#pop-options

(* clean-context: extend the inner preserved-forall from jj to jj+1. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_inner_pres_extend
      (v_RANK: usize) (transpose: bool)
      (i: usize {v i < v v_RANK}) (a a0 a': KB.mat_ty v_RANK) (sampled: poly_t)
      (jj: usize {v jj < v v_RANK})
  : Lemma
    (requires
      v v_RANK <= 4 /\
      a' == UA.update_at_usize a (slot_row transpose i jj)
              (UA.update_at_usize (a.[ slot_row transpose i jj ]) (slot_col transpose i jj) sampled) /\
      (forall (a2 b2: nat). (a2 < v v_RANK /\ b2 < v v_RANK /\ ~(written_upto transpose i (v jj) a2 b2)) ==>
         Seq.index (Seq.index a a2) b2 == Seq.index (Seq.index a0 a2) b2))
    (ensures
      (forall (a2 b2: nat). (a2 < v v_RANK /\ b2 < v v_RANK /\ ~(written_upto transpose i (v jj + 1) a2 b2)) ==>
         Seq.index (Seq.index a' a2) b2 == Seq.index (Seq.index a0 a2) b2))
= introduce forall (a2 b2: nat). (a2 < v v_RANK /\ b2 < v v_RANK /\ ~(written_upto transpose i (v jj + 1) a2 b2)) ==>
    Seq.index (Seq.index a' a2) b2 == Seq.index (Seq.index a0 a2) b2
  with introduce _ ==> _ with _.
    lemma_matrix_upd_entry #poly_t #v_RANK a (slot_row transpose i jj) (slot_col transpose i jj)
      sampled a2 b2
#pop-options

(* one inner step maintains inner_p (Continue) / establishes is_err (Break). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_inner_step_maintain
      (v_RANK: usize) (transpose: bool) (seed: t_Slice u8 {Seq.length seed == 32})
      (i: usize {v i < v v_RANK}) (a0: KB.mat_ty v_RANK)
      (acc: KB.acc_ty v_RANK)
      (jj: usize {v jj <= v v_RANK /\ F.fold_range_wf_index (mk_usize 0) v_RANK true (v jj)})
  : Lemma
    (requires v v_RANK <= 4 /\ inner_p v_RANK transpose seed i a0 (v jj) acc)
    (ensures
      (match KB.sma_inner_step v_RANK transpose i acc jj with
       | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> KB.is_err_pred v_RANK r
       | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a')) -> False
       | CF.ControlFlow_Continue a' -> inner_p v_RANK transpose seed i a0 (v jj + 1) a'))
= let (a, xof) = acc in
  let xof1 = UA.update_at_usize xof (mk_usize 32) (cast (i <: usize) <: u8) in
  let xof2 = UA.update_at_usize xof1 (mk_usize 33) (cast (jj <: usize) <: u8) in
  lemma_sma_xof2_eq seed xof i jj;
  match HS.sample_ntt (mk_usize 70) (mk_usize 560) (mk_usize 840) (mk_usize 6720)
          (HF.v_XOF (mk_usize 840) (xof2 <: t_Slice u8)) with
  | R.Result_Err err -> ()
  | R.Result_Ok sampled ->
    assert (sma_sample seed i jj == R.Result_Ok sampled);
    let a' : KB.mat_ty v_RANK = UA.update_at_usize a (slot_row transpose i jj)
               (UA.update_at_usize (a.[ slot_row transpose i jj ]) (slot_col transpose i jj) sampled) in
    (* a' is exactly the step's Continue payload (case on transpose) *)
    assert (a' == (if transpose
                   then UA.update_at_usize a jj (UA.update_at_usize (a.[ jj ]) i sampled)
                   else UA.update_at_usize a i (UA.update_at_usize (a.[ i ]) jj sampled)));
    lemma_inner_done_extend v_RANK transpose seed i a a' sampled jj;
    lemma_inner_pres_extend v_RANK transpose i a a0 a' sampled jj
#pop-options

(* assemble the maintenance forall and run lemma_frr_inv on the inner fold *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_sma_inner_row
      (v_RANK: usize) (transpose: bool) (seed: t_Slice u8 {Seq.length seed == 32})
      (i: usize {v i < v v_RANK}) (a0: KB.mat_ty v_RANK) (xof0: t_Array u8 (mk_usize 34))
  : Lemma
    (requires v v_RANK <= 4 /\ Seq.equal (Seq.slice xof0 0 32) seed)
    (ensures
      (match F.fold_range_return (mk_usize 0) v_RANK (KB.triv_inv v_RANK)
               (a0, xof0) (KB.sma_inner_step v_RANK transpose i) with
       | CF.ControlFlow_Continue (a', xof') ->
         Seq.equal (Seq.slice xof' 0 32) seed /\
         (forall (jj: usize). v jj < v v_RANK ==>
            (match sma_sample seed i jj with
             | R.Result_Ok s ->
               Seq.index (Seq.index a' (v (slot_row transpose i jj))) (v (slot_col transpose i jj)) == s
             | R.Result_Err _ -> False)) /\
         (forall (a2 b2: nat). (a2 < v v_RANK /\ b2 < v v_RANK /\
              ~(if transpose then b2 = v i else a2 = v i)) ==>
            Seq.index (Seq.index a' a2) b2 == Seq.index (Seq.index a0 a2) b2)
       | CF.ControlFlow_Break r -> KB.is_err_pred v_RANK r))
= let p = inner_p v_RANK transpose seed i a0 in
  introduce forall (acc: KB.acc_ty v_RANK)
                   (jj: usize {v jj <= v v_RANK /\ F.fold_range_wf_index (mk_usize 0) v_RANK true (v jj)}).
            p (v jj) acc ==>
              (match KB.sma_inner_step v_RANK transpose i acc jj with
               | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> KB.is_err_pred v_RANK r
               | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a')) -> False
               | CF.ControlFlow_Continue a' -> p (v jj + 1) a')
  with introduce _ ==> _
    with _pf. lemma_inner_step_maintain v_RANK transpose seed i a0 acc jj;
  assert (p 0 (a0, xof0));
  lemma_frr_inv (mk_usize 0) v_RANK (KB.triv_inv v_RANK) p (KB.is_err_pred v_RANK)
    (KB.sma_inner_step v_RANK transpose i) (a0, xof0)
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   PART 1.d — outer characterization, then the per-entry lemma.
   ════════════════════════════════════════════════════════════════════════ *)

(* the outer-fold invariant over accumulator (A, xof). *)
unfold let outer_p
      (v_RANK: usize) (transpose: bool) (seed: t_Slice u8 {Seq.length seed == 32})
      (i_done: int) (acc: KB.acc_ty v_RANK)
  : Type0
  = let (a, xof) = acc in
    Seq.equal (Seq.slice xof 0 32) seed /\
    (forall (ii jj: usize). (v ii < i_done /\ v ii < v v_RANK /\ v jj < v v_RANK) ==>
       (match sma_sample seed ii jj with
        | R.Result_Ok s ->
          Seq.index (Seq.index a (v (slot_row transpose ii jj))) (v (slot_col transpose ii jj)) == s
        | R.Result_Err _ -> False))

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_outer_step_maintain
      (v_RANK: usize) (transpose: bool) (seed: t_Slice u8 {Seq.length seed == 32})
      (acc: KB.acc_ty v_RANK)
      (ii: usize {v ii <= v v_RANK /\ F.fold_range_wf_index (mk_usize 0) v_RANK true (v ii)})
  : Lemma
    (requires v v_RANK <= 4 /\ outer_p v_RANK transpose seed (v ii) acc)
    (ensures
      (match KB.sma_outer_step v_RANK transpose acc ii with
       | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> KB.is_err_pred v_RANK r
       | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a')) -> False
       | CF.ControlFlow_Continue a' -> outer_p v_RANK transpose seed (v ii + 1) a'))
= let (a, xof) = acc in
  lemma_sma_inner_row v_RANK transpose seed ii a xof;
  (* the outer step's inner fold result drives the conclusion; lemma_sma_inner_row gives
     row-ii entries + preservation of non-(row/col)-ii entries, which extends outer_p. *)
  ()
#pop-options

(* per-(i,j) characterization of the hacspec result.  THE standalone fold reconstruction. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400 --split_queries always"
let lemma_sma_entry
      (v_RANK: usize) (seed: t_Slice u8 {Seq.length seed == 32}) (transpose: bool)
  : Lemma
    (requires v v_RANK <= 4)
    (ensures
      (match HM.sample_matrix_A v_RANK seed transpose with
       | R.Result_Ok sm ->
         (forall (i j: usize). (v i < v v_RANK /\ v j < v v_RANK) ==>
            (match sma_sample seed i j with
             | R.Result_Ok s ->
               Seq.index (Seq.index sm (v (slot_row transpose i j))) (v (slot_col transpose i j)) == s
             | R.Result_Err _ -> False))
       | R.Result_Err _ -> True))
= KB.lemma_sma_unfold v_RANK seed transpose;
  let p = outer_p v_RANK transpose seed in
  introduce forall (acc: KB.acc_ty v_RANK)
                   (ii: usize {v ii <= v v_RANK /\ F.fold_range_wf_index (mk_usize 0) v_RANK true (v ii)}).
            p (v ii) acc ==>
              (match KB.sma_outer_step v_RANK transpose acc ii with
               | CF.ControlFlow_Break (CF.ControlFlow_Break r) -> KB.is_err_pred v_RANK r
               | CF.ControlFlow_Break (CF.ControlFlow_Continue (_, a')) -> False
               | CF.ControlFlow_Continue a' -> p (v ii + 1) a')
  with introduce _ ==> _
    with _pf. lemma_outer_step_maintain v_RANK transpose seed acc ii;
  assert (p 0 (KB.sma_init_A v_RANK, KB.sma_init_xof seed));
  lemma_frr_inv (mk_usize 0) v_RANK (KB.triv_inv v_RANK) p (KB.is_err_pred v_RANK)
    (KB.sma_outer_step v_RANK transpose) (KB.sma_init_A v_RANK, KB.sma_init_xof seed)
#pop-options

(* ════════════════════════════════════════════════════════════════════════
   PART 2 — the opaque atom + base/step/outer_row/finalize, threaded by matrix.rs.
   ════════════════════════════════════════════════════════════════════════ *)

(* "the impl matrix `a`'s done (i,j)-prefix matches the hacspec result sm's prefix"
   (vacuously true when the hacspec rejects). *)
[@@ "opaque_to_smt"]
let sma_done_ij
      (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
      (a: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
      (seed: t_Slice u8) (transpose: bool) (i: nat) (j: nat)
  : Type0
  = (Seq.length seed == 32 /\ v v_K <= 4) ==>
    (match HM.sample_matrix_A v_K seed transpose with
     | R.Result_Ok sm ->
       (forall (a2 b2: nat). (a2 < v v_K /\ b2 < v v_K /\
          (if transpose then (b2 < i \/ (b2 = i /\ a2 < j)) else (a2 < i \/ (a2 = i /\ b2 < j)))) ==>
          VS.poly_to_spec #vV (Seq.index (Seq.index a a2) b2) == Seq.index (Seq.index sm a2) b2)
     | R.Result_Err _ -> True)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_sma_base
      (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
      (a: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
      (seed: t_Slice u8) (transpose: bool)
  : Lemma (ensures sma_done_ij #vV #iop #v_K a seed transpose 0 0)
= reveal_opaque (`%sma_done_ij) (sma_done_ij #vV #iop #v_K a seed transpose 0 0)
#pop-options

(* inner step: extend the prefix from (i,j) to (i,j+1) using the new entry. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_sma_inner_step
      (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
      (a_old a_new: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
      (sampled: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (seeds_j: t_Array u8 (mk_usize 34))
      (seed: t_Slice u8) (transpose: bool) (i j: usize)
  : Lemma
    (requires
      Seq.length seed == 32 /\ v v_K <= 4 /\ v i < v v_K /\ v j < v v_K /\
      sma_done_ij #vV #iop #v_K a_old seed transpose (v i) (v j) /\
      seeds_j == sma_xof_input seed i j /\
      (match HS.sample_ntt (mk_usize 70) (mk_usize 560) (mk_usize 840) (mk_usize 6720)
               (HF.v_XOF (mk_usize 840) (seeds_j <: t_Slice u8)) with
       | R.Result_Ok s -> VS.poly_to_spec #vV (Seq.index sampled (v j)) == s
       | R.Result_Err _ -> True) /\
      a_new == (if transpose
                then UA.update_at_usize a_old j (UA.update_at_usize (a_old.[ j ]) i (sampled.[ j ]))
                else UA.update_at_usize a_old i (UA.update_at_usize (a_old.[ i ]) j (sampled.[ j ]))))
    (ensures sma_done_ij #vV #iop #v_K a_new seed transpose (v i) (v j + 1))
= reveal_opaque (`%sma_done_ij) (sma_done_ij #vV #iop #v_K a_old seed transpose (v i) (v j));
  reveal_opaque (`%sma_done_ij) (sma_done_ij #vV #iop #v_K a_new seed transpose (v i) (v j + 1));
  (match HM.sample_matrix_A v_K seed transpose with
   | R.Result_Err _ -> ()
   | R.Result_Ok sm ->
     lemma_sma_entry v_K seed transpose;
     let sr = slot_row transpose i j in
     let sc = slot_col transpose i j in
     (* a_new is the unified update of a_old at slot (sr,sc) := sampled[j] *)
     assert (a_new == UA.update_at_usize a_old sr (UA.update_at_usize (a_old.[ sr ]) sc (sampled.[ j ])));
     (* the impl XOF input equals the hacspec one, so the couplings agree on sma_sample seed i j *)
     assert (sma_sample seed i j
             == HS.sample_ntt (mk_usize 70) (mk_usize 560) (mk_usize 840) (mk_usize 6720)
                  (HF.v_XOF (mk_usize 840) (seeds_j <: t_Slice u8)));
     introduce forall (a2 b2: nat). (a2 < v v_K /\ b2 < v v_K /\
         (if transpose then (b2 < v i \/ (b2 = v i /\ a2 < v j + 1))
                       else (a2 < v i \/ (a2 = v i /\ b2 < v j + 1)))) ==>
         VS.poly_to_spec #vV (Seq.index (Seq.index a_new a2) b2) == Seq.index (Seq.index sm a2) b2
     with introduce _ ==> _ with _.
       (lemma_matrix_upd_entry #(VV.t_PolynomialRingElement vV) #v_K a_old sr sc (sampled.[ j ]) a2 b2;
        (* the new cell (sr,sc) gets its value from lemma_sma_entry at (i,j) + the coupling *)
        assert ((a2 = v sr /\ b2 = v sc) ==>
                (match sma_sample seed i j with
                 | R.Result_Ok s -> Seq.index (Seq.index sm a2) b2 == s
                 | R.Result_Err _ -> False))))
#pop-options

(* outer row transition: prefix (i, v_K) == prefix (i+1, 0). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_sma_outer_row
      (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
      (a: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
      (seed: t_Slice u8) (transpose: bool) (i: usize {v i < v v_K})
  : Lemma
    (requires sma_done_ij #vV #iop #v_K a seed transpose (v i) (v v_K))
    (ensures sma_done_ij #vV #iop #v_K a seed transpose (v i + 1) 0)
= reveal_opaque (`%sma_done_ij) (sma_done_ij #vV #iop #v_K a seed transpose (v i) (v v_K));
  reveal_opaque (`%sma_done_ij) (sma_done_ij #vV #iop #v_K a seed transpose (v i + 1) 0)
#pop-options

(* finalize: full prefix == whole matrix, hence matrix_to_spec a == sm. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_sma_finalize
      (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
      (a: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
      (seed: t_Slice u8) (transpose: bool)
  : Lemma
    (requires Seq.length seed == 32 /\ v v_K <= 4 /\ sma_done_ij #vV #iop #v_K a seed transpose (v v_K) 0)
    (ensures
      (match HM.sample_matrix_A v_K seed transpose with
       | R.Result_Ok sm -> VS.matrix_to_spec v_K #vV a == sm
       | R.Result_Err _ -> True))
= reveal_opaque (`%sma_done_ij) (sma_done_ij #vV #iop #v_K a seed transpose (v v_K) 0);
  (match HM.sample_matrix_A v_K seed transpose with
   | R.Result_Err _ -> ()
   | R.Result_Ok sm ->
     let aux (a2: nat{a2 < v v_K}) : Lemma
       (Seq.index (VS.matrix_to_spec v_K #vV a) a2 == Seq.index sm a2) =
       VS.matrix_to_spec_index v_K #vV a a2;
       let inner (b2: nat{b2 < v v_K}) : Lemma
         (Seq.index (VS.vector_to_spec v_K #vV (Seq.index a a2)) b2 == Seq.index (Seq.index sm a2) b2) =
         VS.vector_to_spec_index v_K #vV (Seq.index a a2) b2 in
       Classical.forall_intro inner;
       Seq.lemma_eq_intro (VS.vector_to_spec v_K #vV (Seq.index a a2)) (Seq.index sm a2) in
     Classical.forall_intro aux;
     Seq.lemma_eq_intro (VS.matrix_to_spec v_K #vV a) sm)
#pop-options
