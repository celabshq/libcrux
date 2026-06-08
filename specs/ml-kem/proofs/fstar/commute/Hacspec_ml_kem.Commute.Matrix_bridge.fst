module Hacspec_ml_kem.Commute.Matrix_bridge
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P = Hacspec_ml_kem.Parameters
module N = Hacspec_ml_kem.Ntt
module HP = Hacspec_ml_kem.Polynomial
module MX = Hacspec_ml_kem.Matrix
module T = Libcrux_ml_kem.Vector.Traits
module V = Libcrux_ml_kem.Vector.Traits.Spec
module VV = Libcrux_ml_kem.Vector
module VS = Libcrux_ml_kem.Vector.Spec
module CH = Hacspec_ml_kem.Commute.Chunk
module Br = Hacspec_ml_kem.Commute.Bridges
module Poly = Libcrux_ml_kem.Polynomial
module SB = Hacspec_ml_kem.Commute.Matrix_bilin
module SB2 = Hacspec_ml_kem.Commute.Matrix_bilin2
module ML = FStar.Math.Lemmas

(* ════════════════ Part 1: FE / poly bridges + per-step maintenance ════════════════ *)

(* bridge (a): std lift = R · plain lift  (mirror SB2.lemma_std_arr, but vs plain) *)
#push-options "--z3rlimit 100"
let lemma_std_arr_plain
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (p: VV.t_PolynomialRingElement vV) :
    Lemma (Poly.to_spec_poly_standard #vV p
           == SB.scale_poly SB.r_fe (CH.to_spec_poly_plain #vV p))
= let plain = CH.to_spec_poly_plain #vV p in
  let lhs   = Poly.to_spec_poly_standard #vV p in
  let rhs   = SB.scale_poly SB.r_fe plain in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    SB2.poly_lane_std #vV p j;
    CH.poly_lane_plain #vV p j;
    SB.lemma_scale_poly_index SB.r_fe plain j;
    SB.lemma_std_eq_R_plain (Seq.index (T.f_repr (Seq.index p.VV.f_coefficients (j / 16))) (j % 16))
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* per-lane char of add_polynomials *)
let lemma_add_poly_lane (x y: t_Array P.t_FieldElement (mk_usize 256)) (i: usize {v i < 256}) :
    Lemma (Seq.index (MX.add_polynomials x y) (v i)
           == P.impl_FieldElement__add (Seq.index x (v i)) (Seq.index y (v i)))
= ()

(* bridge (c): add_polynomials == HP.add_to_ring_element  (identical createi body) *)
let lemma_add_poly_eq_HP_add (x y: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (MX.add_polynomials x y == HP.add_to_ring_element x y)
= let lhs = MX.add_polynomials x y in
  let rhs = HP.add_to_ring_element x y in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    lemma_add_poly_lane x y (sz j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs

(* HP.add_standard_error_reduce == add_polynomials  (identical createi body) *)
let lemma_add_std_err_eq_add_poly (x y: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.add_standard_error_reduce x y == MX.add_polynomials x y)
= let lhs = HP.add_standard_error_reduce x y in
  let rhs = MX.add_polynomials x y in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    lemma_add_poly_lane x y (sz j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs

(* bridge (b): scale distributes over add_polynomials *)
#push-options "--z3rlimit 150"
let lemma_scale_add (c: P.t_FieldElement) (x y: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (SB.scale_poly c (MX.add_polynomials x y)
           == MX.add_polynomials (SB.scale_poly c x) (SB.scale_poly c y))
= let lhs = SB.scale_poly c (MX.add_polynomials x y) in
  let rhs = MX.add_polynomials (SB.scale_poly c x) (SB.scale_poly c y) in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    SB.lemma_scale_poly_index c (MX.add_polynomials x y) j;     (* lhs[j] = c · (x[j]+y[j]) *)
    lemma_add_poly_lane x y (sz j);                             (* (x+y)[j] = x[j]+_FE y[j] *)
    lemma_add_poly_lane (SB.scale_poly c x) (SB.scale_poly c y) (sz j);  (* rhs[j] = (c·x[j])+_FE(c·y[j]) *)
    SB.lemma_scale_poly_index c x j;
    SB.lemma_scale_poly_index c y j;
    SB.left_distrib c (Seq.index x j) (Seq.index y j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* ── per-step inner-loop maintenance (standard domain) ──
   acc_next = add_to_ring_element(acc, product); product = ntt_multiply(A_ij, s_j).
   Given inv `std acc == partial`, conclude `std acc_next == add_polynomials partial term`
   where term = HP.ntt_multiply (plain A_ij) (plain s_j). *)
#push-options "--z3rlimit 200"
let lemma_inner_step
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (acc product acc_next a_ij s_j: VV.t_PolynomialRingElement vV)
    (partial: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma
      (requires
        Poly.to_spec_poly_standard #vV acc == partial /\
        CH.to_spec_poly_plain #vV acc_next
          == HP.add_to_ring_element (CH.to_spec_poly_plain #vV acc)
                                    (CH.to_spec_poly_plain #vV product) /\
        CH.to_spec_poly_mont #vV product
          == HP.ntt_multiply (CH.to_spec_poly_mont #vV a_ij) (CH.to_spec_poly_mont #vV s_j))
      (ensures
        Poly.to_spec_poly_standard #vV acc_next
          == MX.add_polynomials partial
               (HP.ntt_multiply (CH.to_spec_poly_plain #vV a_ij) (CH.to_spec_poly_plain #vV s_j)))
= let pa  = CH.to_spec_poly_plain #vV acc in
  let pp  = CH.to_spec_poly_plain #vV product in
  let term = HP.ntt_multiply (CH.to_spec_poly_plain #vV a_ij) (CH.to_spec_poly_plain #vV s_j) in
  (* std product == term *)
  SB2.lemma_ntt_multiply_standard #vV a_ij s_j product;
  (* std lift = R·plain on the three polys *)
  lemma_std_arr_plain #vV acc_next;
  lemma_std_arr_plain #vV acc;
  lemma_std_arr_plain #vV product;
  (* plain acc_next = add_polynomials pa pp *)
  lemma_add_poly_eq_HP_add pa pp;
  (* scale over add *)
  lemma_scale_add SB.r_fe pa pp;
  assert (Poly.to_spec_poly_standard #vV acc_next == SB.scale_poly SB.r_fe (MX.add_polynomials pa pp));
  assert (Poly.to_spec_poly_standard #vV acc_next
          == MX.add_polynomials (SB.scale_poly SB.r_fe pa) (SB.scale_poly SB.r_fe pp));
  assert (SB.scale_poly SB.r_fe pa == partial);
  assert (SB.scale_poly SB.r_fe pp == term)
#pop-options

(* ════════════════ Part 2: spec-side fold (part) + fusion to multiply_matrix_by_column ════════════════ *)

module F = Rust_primitives.Hax.Folds

let zero_poly : t_Array P.t_FieldElement (mk_usize 256) =
  Rust_primitives.Hax.repeat (P.impl_FieldElement__new (mk_u16 0)) (mk_usize 256)

(* peel-last prefix fold of the row-i dot product, abstract spec arrays *)
let rec part (#v_K: usize)
    (arow scol: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (j: nat {j <= v v_K})
  : Tot (t_Array P.t_FieldElement (mk_usize 256)) (decreases j)
= if j = 0 then zero_poly
  else MX.add_polynomials (part #v_K arow scol (j - 1))
         (N.multiply_ntts (Seq.index arow (j - 1)) (Seq.index scol (j - 1)))

(* peel-FIRST one-step unfold of fold_range (definitional) *)
let lemma_fold_range_step
      (#acc_t: Type0)
      (start end_: usize)
      (inv: acc_t -> (i:usize{F.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (init: acc_t {~(F.range_empty start end_) ==> inv init start})
      (f: (acc:acc_t -> i:usize {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i) /\ inv acc i}
                     -> acc':acc_t {(inv acc' (mk_int (v i + 1)))}))
  : Lemma (requires v start < v end_)
      (ensures F.fold_range start end_ inv init f ==
               F.fold_range (start +! mk_usize 1) end_ inv (f init start) f)
  = ()

(* named inv/step for multiply_matrix_by_column's inner fold.
   These are TRANSCRIBED to byte-match the inline lambdas in
   Hacspec_ml_kem.Matrix.multiply_matrix_by_column (including the no-op
   `let result = result`/`let j = j` rebinds, the `<:` ascriptions, and the
   `.[]` index notation), so that the createi body-lambda applied at index i
   delta/beta-reduces to `fold_range 0 v_K mmbc_inv zero_poly
   (mmbc_step matrix vector i)` definitionally — making lemma_part_eq_mmbc's
   createi connection go through. *)
let mmbc_inv (result: t_Array P.t_FieldElement (mk_usize 256)) (temp_1_: usize) : Type0 =
  let result:t_Array P.t_FieldElement (mk_usize 256) = result in
  let _:usize = temp_1_ in
  true

let mmbc_step
    (#v_K: usize)
    (matrix: t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
    (vector: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (i: usize {v i < v v_K})
    (result: t_Array P.t_FieldElement (mk_usize 256))
    (j: usize {v j < v v_K})
  : t_Array P.t_FieldElement (mk_usize 256)
  = let result:t_Array P.t_FieldElement (mk_usize 256) = result in
    let j:usize = j in
    let product:t_Array P.t_FieldElement (mk_usize 256) =
      N.multiply_ntts ((matrix.[ j ]
            <:
            t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K).[ i ]
          <:
          t_Array P.t_FieldElement (mk_usize 256))
        (vector.[ j ] <: t_Array P.t_FieldElement (mk_usize 256))
    in
    let result:t_Array P.t_FieldElement (mk_usize 256) =
      MX.add_polynomials result product
    in
    result

(* Specific fold fusion for multiply_matrix_by_column's inner fold (sha3-style:
   named inv/step, upward recursion on the start k, per-step bridge inline).
   matrix/vector are the spec arrays the fold indexes; arow/scol drive `part`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Hacspec_ml_kem.Ntt -Hacspec_ml_kem.Matrix'"
let rec lemma_mmbc_aux
    (#v_K: usize)
    (arow scol: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (matrix: t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
    (vector: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (i: usize {v i < v v_K})
    (k: nat {k <= v v_K})
  : Lemma
    (requires
      (forall (m: nat). m < v v_K ==>
         N.multiply_ntts (Seq.index (Seq.index matrix m) (v i)) (Seq.index vector m)
         == N.multiply_ntts (Seq.index arow m) (Seq.index scol m)))
    (ensures
      F.fold_range (sz k) v_K mmbc_inv (part #v_K arow scol k) (mmbc_step #v_K matrix vector i)
      == part #v_K arow scol (v v_K))
    (decreases (v v_K - k))
= if k = v v_K then ()
  else begin
    assert (k < v v_K);
    let mk : t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K = Seq.index matrix k in
    (* per-step: matrix[k][i]·vector[k] == arow[k]·scol[k] (hypothesis at m=k) *)
    assert (N.multiply_ntts (Seq.index mk (v i)) (Seq.index vector k)
            == N.multiply_ntts (Seq.index arow k) (Seq.index scol k));
    assert ((sz k) +! mk_usize 1 == sz (k + 1));
    (* the fold's step at k == part(k+1) (part unfold + multiply_ntts eq + add_poly congruence) *)
    assert (mmbc_step #v_K matrix vector i (part #v_K arow scol k) (sz k)
            == part #v_K arow scol (k + 1));
    lemma_fold_range_step #(t_Array P.t_FieldElement (mk_usize 256))
      (sz k) v_K mmbc_inv (part #v_K arow scol k) (mmbc_step #v_K matrix vector i);
    lemma_mmbc_aux #v_K arow scol matrix vector i (k + 1)
  end
#pop-options

(* transpose(matrix_to_spec A)[k][i] == vector_to_spec(A[i])[k] *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_transpose_bridge
    (#v_K: usize) (#vV: Type0) {| iop: T.t_Operations vV |}
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (i k: usize {v i < v v_K /\ v k < v v_K})
  : Lemma
    (ensures
      Seq.index (Seq.index (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A)) (v k)) (v i)
      == Seq.index (VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i))) (v k))
= let mts = VS.matrix_to_spec v_K #vV matrix_A in
  (* transpose mts [k][i] == mts[i][k] via the two createi unfolds *)
  P.createi_lemma #(t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K
    #(usize -> t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (fun (a: usize { a <. v_K }) ->
       P.createi #(t_Array P.t_FieldElement (mk_usize 256)) v_K
         #(usize -> t_Array P.t_FieldElement (mk_usize 256))
         (fun (b: usize { b <. v_K }) ->
            (Seq.index (Seq.index mts (v b)) (v a)) <: t_Array P.t_FieldElement (mk_usize 256))
       <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    k;
  P.createi_lemma #(t_Array P.t_FieldElement (mk_usize 256)) v_K
    #(usize -> t_Array P.t_FieldElement (mk_usize 256))
    (fun (b: usize { b <. v_K }) ->
       (Seq.index (Seq.index mts (v b)) (v k)) <: t_Array P.t_FieldElement (mk_usize 256))
    i;
  VS.matrix_to_spec_index v_K #vV matrix_A (v i)
#pop-options


(* ── fold_range output congruence (fallback recipe) ──
   fold_range never inspects `inv` at runtime (it only constrains types), and its
   result is determined by start/end_/init/f.  So two folds over the SAME
   start/end_/init with pointwise-equal step functions produce equal results.
   We compare OUTPUTS by upward recursion (peeling the first step on both via
   lemma_fold_range_step), which avoids ever comparing the step *functions* as
   values — the obstruction that defeats trefl/SMT on createi-of-fold. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let rec lemma_fold_range_cong
      (#acc_t: Type0)
      (start end_: usize)
      (inv1 inv2: acc_t -> (i:usize{F.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (init: acc_t {(forall (a:acc_t) (j:usize{F.fold_range_wf_index start end_ false (v j)}).
                       inv1 a j /\ inv2 a j)})
      (f1: (acc:acc_t -> i:usize {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i) /\ inv1 acc i}
                      -> acc':acc_t {(inv1 acc' (mk_int (v i + 1)))}))
      (f2: (acc:acc_t -> i:usize {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i) /\ inv2 acc i}
                      -> acc':acc_t {(inv2 acc' (mk_int (v i + 1)))}))
  : Lemma
    (requires
      (forall (a:acc_t) (j:usize{F.fold_range_wf_index start end_ false (v j)}). inv1 a j /\ inv2 a j) /\
      (forall (acc:acc_t) (jj:usize{v jj >= v start /\ v jj < v end_}). f1 acc jj == f2 acc jj))
    (ensures F.fold_range start end_ inv1 init f1 == F.fold_range start end_ inv2 init f2)
    (decreases (v end_ - v start))
= if v start < v end_ then begin
    let init1 = f1 init start in
    assert (init1 == f2 init start);  (* pointwise hyp at jj=start *)
    lemma_fold_range_step #acc_t start end_ inv1 init f1;
    lemma_fold_range_step #acc_t start end_ inv2 init f2;
    lemma_fold_range_cong #acc_t (start +! mk_usize 1) end_ inv1 inv2 init1 f1 f2
  end
  else ()
#pop-options

(* multiply_matrix_by_column[i] == my named fold_range.
   Step 1 (trefl): multiply_matrix_by_column[i] == its spec inline fold
     `fold_range 0 v_K spec_inv zero_poly (spec_step i)` — pure delta-unfold of
     multiply_matrix_by_column (+createi_lemma), no inv/step renaming so no
     binder-refinement drift.
   Step 2 (fold congruence): that spec fold == my `fold_range 0 v_K mmbc_inv
     zero_poly (mmbc_step i)`; spec_step and mmbc_step agree pointwise (both are
     add_polynomials acc (multiply_ntts matrix.[j].[i] vector.[j]), with `.[]` =
     Seq.index a value equality). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mmbc_index
    (#v_K: usize)
    (matrix: t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
    (vector: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (i: usize {v i < v v_K})
  : Lemma
    (ensures
      Seq.index (MX.multiply_matrix_by_column v_K matrix vector) (v i)
      == F.fold_range (mk_usize 0) v_K mmbc_inv zero_poly (mmbc_step #v_K matrix vector i))
= (* spec inline inv/step, byte-copied from multiply_matrix_by_column *)
  let spec_inv : (t_Array P.t_FieldElement (mk_usize 256))
                 -> (j:usize{F.fold_range_wf_index (mk_usize 0) v_K false (v j)}) -> Type0 =
    (fun result temp_1_ ->
        let result:t_Array P.t_FieldElement (mk_usize 256) = result in
        let _:usize = temp_1_ in
        b2t true) in
  let spec_step
      : (acc:(t_Array P.t_FieldElement (mk_usize 256))
         -> j:usize {v j <= v v_K /\ F.fold_range_wf_index (mk_usize 0) v_K true (v j) /\ spec_inv acc j}
         -> acc':(t_Array P.t_FieldElement (mk_usize 256)) {spec_inv acc' (mk_int (v j + 1))}) =
    (fun result j ->
        let result:t_Array P.t_FieldElement (mk_usize 256) = result in
        let j:usize = j in
        let product:t_Array P.t_FieldElement (mk_usize 256) =
          N.multiply_ntts ((matrix.[ j ]
                <:
                t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K).[ i ]
              <:
              t_Array P.t_FieldElement (mk_usize 256))
            (vector.[ j ] <: t_Array P.t_FieldElement (mk_usize 256))
        in
        let result:t_Array P.t_FieldElement (mk_usize 256) =
          MX.add_polynomials result product
        in
        result) in
  (* Step 1: multiply_matrix_by_column[i] == spec inline fold, by trefl. *)
  assert (Seq.index (MX.multiply_matrix_by_column v_K matrix vector) (v i)
          == F.fold_range (mk_usize 0) v_K spec_inv zero_poly spec_step)
    by (FStar.Tactics.norm [delta_only [`%MX.multiply_matrix_by_column];
                            zeta; iota; primops];
        FStar.Tactics.l_to_r [`P.createi_lemma];
        FStar.Tactics.trefl ());
  (* Step 2: spec fold == mmbc fold, by output congruence (pointwise step eq). *)
  let aux (acc: t_Array P.t_FieldElement (mk_usize 256))
          (jj: usize{v jj >= 0 /\ v jj < v v_K})
    : Lemma (spec_step acc jj == mmbc_step #v_K matrix vector i acc jj)
    = () in
  Classical.forall_intro_2 aux;
  lemma_fold_range_cong #(t_Array P.t_FieldElement (mk_usize 256))
    (mk_usize 0) v_K spec_inv mmbc_inv zero_poly spec_step (mmbc_step #v_K matrix vector i)
#pop-options

(* part of the row-i dot product == multiply_matrix_by_column(transpose A, s)[i] *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_part_eq_mmbc
    (#v_K: usize) (#vV: Type0) {| iop: T.t_Operations vV |}
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (i: usize {v i < v v_K})
  : Lemma
    (ensures
      part #v_K (VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i))) (VS.vector_to_spec v_K #vV s) (v v_K)
      == Seq.index (MX.multiply_matrix_by_column v_K
                      (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A))
                      (VS.vector_to_spec v_K #vV s)) (v i))
= let arow = VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i)) in
  let scol = VS.vector_to_spec v_K #vV s in
  let matrix = MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A) in
  let vector = VS.vector_to_spec v_K #vV s in
  let aux (m: nat {m < v v_K}) :
    Lemma (N.multiply_ntts (Seq.index (Seq.index matrix m) (v i)) (Seq.index vector m)
           == N.multiply_ntts (Seq.index arow m) (Seq.index scol m)) =
    lemma_transpose_bridge #v_K #vV matrix_A i (sz m)
  in
  Classical.forall_intro aux;
  lemma_mmbc_aux #v_K arow scol matrix vector i 0;
  assert (part #v_K arow scol 0 == zero_poly);
  lemma_mmbc_index #v_K matrix vector i;
  assert (F.fold_range (mk_usize 0) v_K mmbc_inv zero_poly (mmbc_step #v_K matrix vector i)
          == part #v_K arow scol (v v_K))
#pop-options

(* ════ Inner-loop one-call maintenance: std(acc)==part j  ⟹  std(acc_next)==part (j+1).
   Packages lemma_inner_step + the vector_to_spec_index/poly_to_spec bridges so the
   compute_As_plus_e inner loop body is a single call (keeps the function VC small). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_inner_maintain
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (i j: usize {v i < v v_K /\ v j < v v_K})
    (acc product acc_next: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      Poly.to_spec_poly_standard #vV acc
        == part #v_K (VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i)))
                     (VS.vector_to_spec v_K #vV s) (v j) /\
      CH.to_spec_poly_mont #vV product
        == HP.ntt_multiply (CH.to_spec_poly_mont #vV (Seq.index (Seq.index matrix_A (v i)) (v j)))
                           (CH.to_spec_poly_mont #vV (Seq.index s (v j))) /\
      CH.to_spec_poly_plain #vV acc_next
        == HP.add_to_ring_element (CH.to_spec_poly_plain #vV acc) (CH.to_spec_poly_plain #vV product))
    (ensures
      Poly.to_spec_poly_standard #vV acc_next
        == part #v_K (VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i)))
                     (VS.vector_to_spec v_K #vV s) (v j + 1))
= let arow = VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i)) in
  let scol = VS.vector_to_spec v_K #vV s in
  let a_ij = Seq.index (Seq.index matrix_A (v i)) (v j) in
  let s_j  = Seq.index s (v j) in
  lemma_inner_step #vV acc product acc_next a_ij s_j (part #v_K arow scol (v j));
  (* arow[v j] == to_spec_poly_plain a_ij ; scol[v j] == to_spec_poly_plain s_j *)
  VS.vector_to_spec_index v_K #vV (Seq.index matrix_A (v i)) (v j);
  VS.vector_to_spec_index v_K #vV s (v j);
  Hacspec_ml_kem.Commute.Bridges.poly_to_spec_eq_to_spec_poly_plain #vV a_ij;
  Hacspec_ml_kem.Commute.Bridges.poly_to_spec_eq_to_spec_poly_plain #vV s_j;
  assert (Seq.index arow (v j) == CH.to_spec_poly_plain #vV a_ij);
  assert (Seq.index scol (v j) == CH.to_spec_poly_plain #vV s_j);
  (* part (v j + 1) unfolds (fuel 1) to add_polynomials (part (v j)) (multiply_ntts arow[v j] scol[v j]);
     N.multiply_ntts == HP.ntt_multiply definitionally, matching lemma_inner_step's conclusion. *)
  assert (part #v_K arow scol (v j + 1)
          == MX.add_polynomials (part #v_K arow scol (v j))
               (N.multiply_ntts (Seq.index arow (v j)) (Seq.index scol (v j))))
#pop-options

(* ════ Per-row finalize: inner-exit (std==mmbc_row) + add_standard_error_reduce post
   ⟹ poly_to_spec(t_final) == compute_As_plus_e(transpose A, s, e)[i]. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_outer_row
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s error: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (t_pre t_final: VV.t_PolynomialRingElement vV)
    (i: usize {v i < v v_K})
  : Lemma
    (requires
      Poly.to_spec_poly_standard #vV t_pre
        == Seq.index (MX.multiply_matrix_by_column v_K
                        (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A))
                        (VS.vector_to_spec v_K #vV s)) (v i) /\
      CH.to_spec_poly_plain #vV t_final
        == HP.add_standard_error_reduce (Poly.to_spec_poly_standard #vV t_pre)
                                        (CH.to_spec_poly_plain #vV (Seq.index error (v i))))
    (ensures
      VS.poly_to_spec #vV t_final
        == Seq.index (MX.compute_As_plus_e v_K
                        (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A))
                        (VS.vector_to_spec v_K #vV s)
                        (VS.vector_to_spec v_K #vV error)) (v i))
= let mmbc = MX.multiply_matrix_by_column v_K
               (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A))
               (VS.vector_to_spec v_K #vV s) in
  let ev = VS.vector_to_spec v_K #vV error in
  lemma_add_std_err_eq_add_poly (Poly.to_spec_poly_standard #vV t_pre)
                                (CH.to_spec_poly_plain #vV (Seq.index error (v i)));
  VS.vector_to_spec_index v_K #vV error (v i);
  Hacspec_ml_kem.Commute.Bridges.poly_to_spec_eq_to_spec_poly_plain #vV (Seq.index error (v i));
  Hacspec_ml_kem.Commute.Bridges.poly_to_spec_eq_to_spec_poly_plain #vV t_final;
  (* compute_As_plus_e = add_vectors mmbc ev ; [i] = add_polynomials mmbc[i] ev[i] *)
  assert (Seq.index (MX.compute_As_plus_e v_K
                       (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A))
                       (VS.vector_to_spec v_K #vV s) ev) (v i)
          == MX.add_polynomials (Seq.index mmbc (v i)) (Seq.index ev (v i)))
#pop-options

(* ════ Final assembly: per-row poly_to_spec == target[j] for all j ⟹ vector_to_spec t == target.
   (deserialize_vector pattern; now works because vector_to_spec_index exists.) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_final_assemble
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (t: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (requires (forall (j: nat). j < v v_K ==> VS.poly_to_spec #vV (Seq.index t j) == Seq.index target j))
    (ensures VS.vector_to_spec v_K #vV t == target)
= let lhs = VS.vector_to_spec v_K #vV t in
  let aux (j: nat {j < v v_K}) : Lemma (Seq.index lhs j == Seq.index target j) =
    VS.vector_to_spec_index v_K #vV t j
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs target
#pop-options

(* ════════════════ Opaque composition atoms (compute_As_plus_e wiring) ════════════════
   The compute_As_plus_e proof is a pure composition: each loop step's functional behaviour is
   one of the proven lemmas above.  To keep the FUNCTION's VCs trivial — in particular so the
   transparent createi specs (`compute_As_plus_e` / `part` / `to_spec_poly_*`) never unfold into
   the simple `j *! 3328` panic-freedom / bounds obligations — we wrap each functional invariant
   conjunct in an OPAQUE predicate atom (mirror of `add_std_chunk_done` in add_standard_error_reduce).
   The loop invariants then carry only opaque atoms; the wrapper lemmas reveal them internally. *)

(* row i of t equals the partial dot-product of (vts A[i]) · (vts s) up to index j *)
[@@ "opaque_to_smt"]
let inner_done
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (t_i: VV.t_PolynomialRingElement vV)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (i: usize {v i < v v_K}) (j: nat {j <= v v_K})
  : Type0
  = Poly.to_spec_poly_standard #vV t_i
    == part #v_K (VS.vector_to_spec v_K #vV (Seq.index matrix_A (v i)))
                 (VS.vector_to_spec v_K #vV s) j

(* row j of t equals target row j *)
[@@ "opaque_to_smt"]
let row_done
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (t_j: VV.t_PolynomialRingElement vV)
    (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (j: nat {j < v v_K})
  : Type0
  = VS.poly_to_spec #vV t_j == Seq.index target j

(* base: a zero-bounded row satisfies inner_done at j=0 (std ZERO == zero_poly == part .. 0) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_inner_done_base
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (i: usize {v i < v v_K})
    (t_i: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 0) t_i)
    (ensures inner_done t_i matrix_A s i 0)
= Hacspec_ml_kem.Commute.Matrix_zerolift.lemma_zero_lift #vV t_i;
  reveal_opaque (`%inner_done) (inner_done t_i matrix_A s i 0)
#pop-options

(* step: inner_done j  + the per-step mont/plain posts  ⟹  inner_done (j+1) *)
(* The two impl__ bridge calls are made INSIDE this lemma so the createi-bearing functional posts
   (ntt_multiply / add_to_ring_element) never enter the caller's (compute_As_plus_e's) context —
   the loop body then only chains opaque atoms + clean bound posts. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_inner_done_step
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (i j: usize {v i < v v_K /\ v j < v v_K})
    (acc product acc_next: VV.t_PolynomialRingElement vV) (e_b: usize)
  : Lemma
    (requires
      inner_done acc matrix_A s i (v j) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328)
        (Seq.index (Seq.index matrix_A (v i)) (v j)) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index s (v j)) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV e_b acc /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) product /\
      (e_b <=. (mk_usize 4 *! mk_usize 3328 <: usize)) /\
      product == Poly.impl__ntt_multiply (Seq.index (Seq.index matrix_A (v i)) (v j)) (Seq.index s (v j)) /\
      acc_next == Poly.impl__add_to_ring_element acc product e_b)
    (ensures inner_done acc_next matrix_A s i (v j + 1))
= Poly.lemma_impl_ntt_multiply_spec (Seq.index (Seq.index matrix_A (v i)) (v j)) (Seq.index s (v j));
  Poly.lemma_impl_add_to_ring_element_spec acc product e_b;
  reveal_opaque (`%inner_done) (inner_done acc matrix_A s i (v j));
  lemma_inner_maintain matrix_A s i j acc product acc_next;
  reveal_opaque (`%inner_done) (inner_done acc_next matrix_A s i (v j + 1))
#pop-options

(* finalize: inner_done at v_K + add_standard_error_reduce post ⟹ row_done t_final target i *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_row_done_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s error: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (t_pre t_final: VV.t_PolynomialRingElement vV)
    (i: usize {v i < v v_K})
    (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (requires
      inner_done t_pre matrix_A s i (v v_K) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index error (v i)) /\
      t_final == Poly.impl__add_standard_error_reduce t_pre (Seq.index error (v i)) /\
      target == MX.compute_As_plus_e v_K
                  (MX.transpose v_K (VS.matrix_to_spec v_K #vV matrix_A))
                  (VS.vector_to_spec v_K #vV s)
                  (VS.vector_to_spec v_K #vV error))
    (ensures row_done t_final target (v i))
= Poly.lemma_impl_add_standard_error_reduce_spec t_pre (Seq.index error (v i));
  reveal_opaque (`%inner_done) (inner_done t_pre matrix_A s i (v v_K));
  lemma_part_eq_mmbc matrix_A s i;
  lemma_outer_row matrix_A s error t_pre t_final i;
  reveal_opaque (`%row_done) (row_done t_final target (v i))
#pop-options

(* assemble: forall j. row_done t[j] target j ⟹ vector_to_spec t == target *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_rows_assemble
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (t: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (requires (forall (j: nat). j < v v_K ==> row_done (Seq.index t j) target j))
    (ensures VS.vector_to_spec v_K #vV t == target)
= let aux (j: nat {j < v v_K}) :
    Lemma (VS.poly_to_spec #vV (Seq.index t j) == Seq.index target j) =
    reveal_opaque (`%row_done) (row_done (Seq.index t j) target j)
  in
  Classical.forall_intro aux;
  lemma_final_assemble t target
#pop-options

(* ════ Full inner-step maintenance in a CLEAN context: given inv(j) for tt, prove inv(j+1)
   for the updated array.  The array-modification postconditions of update_at_usize
   (modified index `(upd s i x).[i]==x` + frame `(upd s i x).[k]==s.[k]`, k<>i) discharge here
   because the context is not polluted by the compute_As_plus_e fold WP.  ensures is written in
   the fold's exact inv(j+1) shape (`mk_int (v j + 1)`) so the caller needs no roundtrip. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_inner_step_full
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (matrix_A: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (s: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (i j: usize {v i < v v_K /\ v j < v v_K})
    (tt: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (requires
      v v_K <= 4 /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328)
        (Seq.index (Seq.index matrix_A (v i)) (v j)) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index s (v j)) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (j *! mk_usize 3328 <: usize)
        (Seq.index tt (v i)) /\
      (forall (k: usize). k <. i ==>
         Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index tt (v k))) /\
      (forall (k: nat). k < v i /\ k < v v_K ==> row_done (Seq.index tt k) target k) /\
      inner_done (Seq.index tt (v i)) matrix_A s i (v j))
    (ensures
      (let product = Poly.impl__ntt_multiply (Seq.index (Seq.index matrix_A (v i)) (v j)) (Seq.index s (v j)) in
       let xval = Poly.impl__add_to_ring_element (Seq.index tt (v i)) product (j *! mk_usize 3328 <: usize) in
       let tt' = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize tt i xval in
       Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV
         ((mk_int (v j + 1) <: usize) *! mk_usize 3328 <: usize) (Seq.index tt' (v i)) /\
       (forall (k: usize). k <. i ==>
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index tt' (v k))) /\
       (forall (k: nat). k < v i /\ k < v v_K ==> row_done (Seq.index tt' k) target k) /\
       inner_done (Seq.index tt' (v i)) matrix_A s i (v (mk_int (v j + 1) <: usize))))
= let a_ij = Seq.index (Seq.index matrix_A (v i)) (v j) in
  let s_j = Seq.index s (v j) in
  let acc = Seq.index tt (v i) in
  let product = Poly.impl__ntt_multiply a_ij s_j in
  let xval = Poly.impl__add_to_ring_element acc product (j *! mk_usize 3328 <: usize) in
  let tt' = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize tt i xval in
  (* array-mod postconditions of update_at_usize (clean context: Seq.upd lemmas fire) *)
  assert (tt' == Seq.upd tt (v i) xval);
  assert (Seq.index tt' (v i) == xval);
  (* mk_int roundtrip *)
  assert (v (mk_int (v j + 1) <: usize) == v j + 1);
  (* bound on xval == (j+1)*3328, and (j*3328)+3328 == (mk_int(v j+1))*3328 *)
  assert (v ((j *! mk_usize 3328 <: usize) +! mk_usize 3328 <: usize)
          == v ((mk_int (v j + 1) <: usize) *! mk_usize 3328 <: usize));
  (* functional: inner_done tt'.[i] (v j + 1) *)
  lemma_inner_done_step matrix_A s i j acc product xval (j *! mk_usize 3328 <: usize);
  (* frame for k<i: tt'.[k] == tt.[k], so bounds + row_done carry *)
  let aux_b (k: usize{v k < v i}) :
    Lemma (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index tt' (v k))) =
    Seq.lemma_index_upd2 tt (v i) xval (v k) in
  let aux_r (k: nat{k < v i /\ k < v v_K}) :
    Lemma (row_done (Seq.index tt' k) target k) =
    Seq.lemma_index_upd2 tt (v i) xval k in
  Classical.forall_intro aux_b;
  Classical.forall_intro aux_r
#pop-options
