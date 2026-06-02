module Libcrux_ml_kem.Ntt
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_kem.Vector.Traits in
  ()

#push-options "--z3rlimit 800 --ext context_pruning --split_queries always"

let ntt_at_layer_1_
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (zeta_i: usize)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (e_initial_coefficient_bound: usize)
     =
  let e_zeta_i_init:usize = zeta_i in
  let e_re_init:t_Array v_Vector (mk_usize 16) = re.Libcrux_ml_kem.Vector.f_coefficients in
  let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 16)
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          b2t (zeta_i =. (e_zeta_i_init +! (round *! mk_usize 4 <: usize) <: usize) <: bool) /\
          (forall (i: usize).
              if i <. mk_usize 16
              then
                if i >=. round
                then
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    e_initial_coefficient_bound
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector) /\
                  Seq.index re.f_coefficients (v i) == Seq.index e_re_init (v i)
                else
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector) /\
                  Seq.index re.f_coefficients (v i) ==
                  Libcrux_ml_kem.Vector.Traits.f_ntt_layer_1_step #v_Vector
                    (Seq.index e_re_init (v i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! i))
              else b2t true))
      (re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          let zeta_i:usize = zeta_i +! mk_usize 1 in
          let _:Prims.unit =
            reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
              (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))
          in
          let _:Prims.unit =
            assert (zeta_i == mk_usize 64 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 1 == mk_usize 65 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 2 == mk_usize 66 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 3 == mk_usize 67 +! mk_usize 4 *! round)
          in
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            {
              re with
              Libcrux_ml_kem.Vector.f_coefficients
              =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                  .Libcrux_ml_kem.Vector.f_coefficients
                round
                (Libcrux_ml_kem.Vector.Traits.f_ntt_layer_1_step #v_Vector
                    #FStar.Tactics.Typeclasses.solve
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ round ] <: v_Vector)
                    (Libcrux_ml_kem.Polynomial.zeta zeta_i <: i16)
                    (Libcrux_ml_kem.Polynomial.zeta (zeta_i +! mk_usize 1 <: usize) <: i16)
                    (Libcrux_ml_kem.Polynomial.zeta (zeta_i +! mk_usize 2 <: usize) <: i16)
                    (Libcrux_ml_kem.Polynomial.zeta (zeta_i +! mk_usize 3 <: usize) <: i16)
                  <:
                  v_Vector)
            }
            <:
            Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
          in
          let zeta_i:usize = zeta_i +! mk_usize 3 in
          re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
  in
  let _:Prims.unit =
    let aux (j: nat)
        : Lemma
        (j < 16 ==>
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #v_Vector
            e_re_init
            re.f_coefficients
            (mk_usize 2)
            (Libcrux_ml_kem.Vector.Traits.Spec.zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +!
                      mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j)))
            j) =
      if j < 16
      then
        (reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
            (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7 * 3328)
                (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (Seq.index e_re_init j)));
          Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_1_step_to_hacspec #v_Vector
            (Seq.index e_re_init j)
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz j))
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j));
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector
            e_re_init
            re.f_coefficients
            (mk_usize 2)
            (Libcrux_ml_kem.Vector.Traits.Spec.zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +!
                      mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j)))
            j)
    in
    Classical.forall_intro aux
  in
  zeta_i, re <: (usize & Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)

#pop-options

#push-options "--z3rlimit 800 --ext context_pruning --split_queries always"

let ntt_at_layer_2_
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (zeta_i: usize)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (e_initial_coefficient_bound: usize)
     =
  let e_zeta_i_init:usize = zeta_i in
  let e_re_init:t_Array v_Vector (mk_usize 16) = re.Libcrux_ml_kem.Vector.f_coefficients in
  let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 16)
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          b2t (zeta_i =. (e_zeta_i_init +! (round *! mk_usize 2 <: usize) <: usize) <: bool) /\
          (forall (i: usize).
              if i <. mk_usize 16
              then
                if i >=. round
                then
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    e_initial_coefficient_bound
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector) /\
                  Seq.index re.f_coefficients (v i) == Seq.index e_re_init (v i)
                else
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector) /\
                  Seq.index re.f_coefficients (v i) ==
                  Libcrux_ml_kem.Vector.Traits.f_ntt_layer_2_step #v_Vector
                    (Seq.index e_re_init (v i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! i))
              else b2t true))
      (re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          let zeta_i:usize = zeta_i +! mk_usize 1 in
          let _:Prims.unit =
            reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
              (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))
          in
          let _:Prims.unit =
            assert (zeta_i == mk_usize 32 +! mk_usize 2 *! round);
            assert (zeta_i +! mk_usize 1 == mk_usize 33 +! mk_usize 2 *! round)
          in
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            {
              re with
              Libcrux_ml_kem.Vector.f_coefficients
              =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                  .Libcrux_ml_kem.Vector.f_coefficients
                round
                (Libcrux_ml_kem.Vector.Traits.f_ntt_layer_2_step #v_Vector
                    #FStar.Tactics.Typeclasses.solve
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ round ] <: v_Vector)
                    (Libcrux_ml_kem.Polynomial.zeta zeta_i <: i16)
                    (Libcrux_ml_kem.Polynomial.zeta (zeta_i +! mk_usize 1 <: usize) <: i16)
                  <:
                  v_Vector)
            }
            <:
            Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
          in
          let zeta_i:usize = zeta_i +! mk_usize 1 in
          re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
  in
  let _:Prims.unit =
    let aux (j: nat)
        : Lemma
        (j < 16 ==>
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #v_Vector
            e_re_init
            re.f_coefficients
            (mk_usize 4)
            (Libcrux_ml_kem.Vector.Traits.Spec.zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +!
                      mk_usize 2 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j)))
            j) =
      if j < 16
      then
        (reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
            (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6 * 3328)
                (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (Seq.index e_re_init j)));
          Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_2_step_to_hacspec #v_Vector
            (Seq.index e_re_init j)
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz j))
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j));
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector
            e_re_init
            re.f_coefficients
            (mk_usize 4)
            (Libcrux_ml_kem.Vector.Traits.Spec.zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +!
                      mk_usize 2 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j)))
            j)
    in
    Classical.forall_intro aux
  in
  zeta_i, re <: (usize & Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)

#pop-options

#push-options "--z3rlimit 800 --ext context_pruning --split_queries always"

let ntt_at_layer_3_
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (zeta_i: usize)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (e_initial_coefficient_bound: usize)
     =
  let e_zeta_i_init:usize = zeta_i in
  let e_re_init:t_Array v_Vector (mk_usize 16) = re.Libcrux_ml_kem.Vector.f_coefficients in
  let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 16)
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          b2t (zeta_i =. (e_zeta_i_init +! round <: usize) <: bool) /\
          (forall (i: usize).
              if i <. mk_usize 16
              then
                if i >=. round
                then
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    e_initial_coefficient_bound
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector) /\
                  Seq.index re.f_coefficients (v i) == Seq.index e_re_init (v i)
                else
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector) /\
                  Seq.index re.f_coefficients (v i) ==
                  Libcrux_ml_kem.Vector.Traits.f_ntt_layer_3_step #v_Vector
                    (Seq.index e_re_init (v i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! i))
              else b2t true))
      (re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          let zeta_i:usize = zeta_i +! mk_usize 1 in
          let _:Prims.unit =
            reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
              (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))
          in
          let _:Prims.unit = assert (zeta_i == mk_usize 16 +! round) in
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            {
              re with
              Libcrux_ml_kem.Vector.f_coefficients
              =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                  .Libcrux_ml_kem.Vector.f_coefficients
                round
                (Libcrux_ml_kem.Vector.Traits.f_ntt_layer_3_step #v_Vector
                    #FStar.Tactics.Typeclasses.solve
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ round ] <: v_Vector)
                    (Libcrux_ml_kem.Polynomial.zeta zeta_i <: i16)
                  <:
                  v_Vector)
            }
            <:
            Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
          in
          re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
  in
  let _:Prims.unit =
    let aux (j: nat)
        : Lemma
        (j < 16 ==>
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #v_Vector
            e_re_init
            re.f_coefficients
            (mk_usize 8)
            (Libcrux_ml_kem.Vector.Traits.Spec.zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +!
                      sz j)))
            j) =
      if j < 16
      then
        (reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
            (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5 * 3328)
                (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (Seq.index e_re_init j)));
          Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_3_step_to_hacspec #v_Vector
            (Seq.index e_re_init j)
            (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz j));
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector
            e_re_init
            re.f_coefficients
            (mk_usize 8)
            (Libcrux_ml_kem.Vector.Traits.Spec.zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +!
                      sz j)))
            j)
    in
    Classical.forall_intro aux
  in
  zeta_i, re <: (usize & Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)

#pop-options

let ntt_layer_int_vec_step
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (a b: v_Vector)
      (zeta_r: i16)
      (e_initial_coefficient_bound: usize)
     =
  let t:v_Vector =
    Libcrux_ml_kem.Vector.Traits.f_montgomery_multiply_by_constant #v_Vector
      #FStar.Tactics.Typeclasses.solve
      b
      zeta_r
  in
  let b:v_Vector =
    Libcrux_ml_kem.Polynomial.sub_bounded #v_Vector a e_initial_coefficient_bound t (mk_usize 3328)
  in
  let a:v_Vector =
    Libcrux_ml_kem.Polynomial.add_bounded #v_Vector a e_initial_coefficient_bound t (mk_usize 3328)
  in
  a, b <: (v_Vector & v_Vector)

#push-options "--z3rlimit 300 --ext context_pruning --split_queries always"

let ntt_at_layer_4_plus
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (zeta_i: usize)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (layer e_initial_coefficient_bound: usize)
     =
  let step:usize = mk_usize 1 <<! layer in
  let e_zeta_i_init:usize = zeta_i in
  let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 128 >>! layer <: usize)
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          b2t (zeta_i =. (e_zeta_i_init +! round <: usize) <: bool) /\
          (forall (i: usize).
              if i <. mk_usize 16
              then
                if i >=. (((round *! step <: usize) *! mk_usize 2 <: usize) /! mk_usize 16 <: usize)
                then
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    e_initial_coefficient_bound
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector)
                else
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                    (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector)
              else b2t true))
      (re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
      (fun temp_0_ round ->
          let (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector), (zeta_i: usize) =
            temp_0_
          in
          let round:usize = round in
          let zeta_i:usize = zeta_i +! mk_usize 1 in
          let offset:usize = (round *! step <: usize) *! mk_usize 2 in
          let offset_vec:usize = offset /! mk_usize 16 in
          let step_vec:usize = step /! mk_usize 16 in
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            Rust_primitives.Hax.Folds.fold_range offset_vec
              (offset_vec +! step_vec <: usize)
              (fun re j ->
                  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
                  let j:usize = j in
                  forall (i: usize).
                    if i <. mk_usize 16
                    then
                      if
                        i >=. j && i <. (offset_vec +! step_vec <: usize) ||
                        i >=. (j +! step_vec <: usize)
                      then
                        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                          e_initial_coefficient_bound
                          (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector)
                      else
                        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                          (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
                          (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector)
                    else b2t true)
              re
              (fun re j ->
                  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
                  let j:usize = j in
                  let (x: v_Vector), (y: v_Vector) =
                    ntt_layer_int_vec_step #v_Vector
                      (re.Libcrux_ml_kem.Vector.f_coefficients.[ j ] <: v_Vector)
                      (re.Libcrux_ml_kem.Vector.f_coefficients.[ j +! step_vec <: usize ]
                        <:
                        v_Vector)
                      (Libcrux_ml_kem.Polynomial.zeta zeta_i <: i16)
                      e_initial_coefficient_bound
                  in
                  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
                    {
                      re with
                      Libcrux_ml_kem.Vector.f_coefficients
                      =
                      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                          .Libcrux_ml_kem.Vector.f_coefficients
                        j
                        x
                    }
                    <:
                    Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
                  in
                  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
                    {
                      re with
                      Libcrux_ml_kem.Vector.f_coefficients
                      =
                      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                          .Libcrux_ml_kem.Vector.f_coefficients
                        (j +! step_vec <: usize)
                        y
                    }
                    <:
                    Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
                  in
                  re)
          in
          re, zeta_i <: (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector & usize))
  in
  zeta_i, re <: (usize & Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)

#pop-options

#push-options "--z3rlimit 200 --ext context_pruning"

#push-options "--z3rlimit 300 --ext context_pruning --split_queries always"

let ntt_at_layer_7_
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
     =
  let step:usize = Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT /! mk_usize 2 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      step
      (fun re j ->
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
          let j:usize = j in
          forall (i: usize).
            if i <. mk_usize 16
            then
              if i >=. j && i <. step || i >=. (j +! step <: usize)
              then
                Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                  (mk_usize 3)
                  (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector)
              else
                Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                  (mk_usize 4803)
                  (re.Libcrux_ml_kem.Vector.f_coefficients.[ i ] <: v_Vector)
            else b2t true)
      re
      (fun re j ->
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
          let j:usize = j in
          let _:Prims.unit = assume (v (Core_models.Num.impl_i16__abs (mk_i16 (- 1600))) == 1600) in
          let t:v_Vector =
            Libcrux_ml_kem.Polynomial.multiply_by_constant_bounded #v_Vector
              (re.Libcrux_ml_kem.Vector.f_coefficients.[ j +! step <: usize ] <: v_Vector)
              (mk_usize 3)
              (mk_i16 (-1600))
          in
          let _:Prims.unit =
            assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 4800) t)
          in
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            {
              re with
              Libcrux_ml_kem.Vector.f_coefficients
              =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                  .Libcrux_ml_kem.Vector.f_coefficients
                (j +! step <: usize)
                (Libcrux_ml_kem.Polynomial.sub_bounded #v_Vector
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ j ] <: v_Vector)
                    (mk_usize 3)
                    t
                    (mk_usize 4800)
                  <:
                  v_Vector)
            }
            <:
            Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
          in
          let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            {
              re with
              Libcrux_ml_kem.Vector.f_coefficients
              =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
                  .Libcrux_ml_kem.Vector.f_coefficients
                j
                (Libcrux_ml_kem.Polynomial.add_bounded #v_Vector
                    (re.Libcrux_ml_kem.Vector.f_coefficients.[ j ] <: v_Vector)
                    (mk_usize 3)
                    t
                    (mk_usize 4800)
                  <:
                  v_Vector)
            }
            <:
            Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector
          in
          re)
  in
  re

#pop-options

#pop-options

#push-options "--z3rlimit 200"

let ntt_binomially_sampled_ring_element
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
     =
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = ntt_at_layer_7_ #v_Vector re in
  let zeta_i:usize = mk_usize 1 in
  let _:Prims.unit =
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly_higher #v_Vector
      re
      (mk_usize 4803)
      (mk_usize 2 *! mk_usize 3328 <: usize)
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 6) (mk_usize 2 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 5) (mk_usize 3 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 4) (mk_usize 4 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_3_ #v_Vector zeta_i re (mk_usize 5 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_2_ #v_Vector zeta_i re (mk_usize 6 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_1_ #v_Vector zeta_i re (mk_usize 7 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let _:Prims.unit =
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly_higher #v_Vector
      re
      (mk_usize 8 *! mk_usize 3328 <: usize)
      (mk_usize 28296)
  in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
    Libcrux_ml_kem.Polynomial.impl__poly_barrett_reduce #v_Vector re
  in
  re

#pop-options

#push-options "--z3rlimit 400 --split_queries always"

let ntt_vector_u
      (v_VECTOR_U_COMPRESSION_FACTOR: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
     =
  let re0:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let (bnd28296: usize):usize = mk_usize 28296 in
  let _:Prims.unit = assert (v bnd28296 == 28296) in
  let zeta_i:usize = mk_usize 0 in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 7) (mk_usize 3328)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re1:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    assume (Hacspec_ml_kem.Commute.Ntt_bridge.poly_step #v_Vector re0 re1 (mk_usize 7))
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 6) (mk_usize 2 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re2:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    assume (Hacspec_ml_kem.Commute.Ntt_bridge.poly_step #v_Vector re1 re2 (mk_usize 6))
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 5) (mk_usize 3 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re3:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    assume (Hacspec_ml_kem.Commute.Ntt_bridge.poly_step #v_Vector re2 re3 (mk_usize 5))
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_4_plus #v_Vector zeta_i re (mk_usize 4) (mk_usize 4 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re4:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    assume (Hacspec_ml_kem.Commute.Ntt_bridge.poly_step #v_Vector re3 re4 (mk_usize 4))
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_3_ #v_Vector zeta_i re (mk_usize 5 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re5:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer3_to_poly_step #v_Vector re4 re5
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_2_ #v_Vector zeta_i re (mk_usize 6 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re6:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer2_to_poly_step #v_Vector re5 re6
  in
  let (tmp0: usize), (tmp1: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
    ntt_at_layer_1_ #v_Vector zeta_i re (mk_usize 7 *! mk_usize 3328 <: usize)
  in
  let zeta_i:usize = tmp0 in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = tmp1 in
  let _:Prims.unit = () in
  let re7:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let _:Prims.unit =
    Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer1_to_poly_step #v_Vector re6 re7
  in
  let _:Prims.unit =
    Hacspec_ml_kem.Commute.Ntt_bridge.lemma_compose_7 #v_Vector re0 re1 re2 re3 re4 re5 re6 re7
  in
  let _:Prims.unit =
    assume (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector bnd28296 re)
  in
  let _:Prims.unit =
    assume (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector
          (Libcrux_ml_kem.Polynomial.impl__poly_barrett_reduce #v_Vector re) ==
        Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector re7)
  in
  let re:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
    Libcrux_ml_kem.Polynomial.impl__poly_barrett_reduce #v_Vector re
  in
  re

#pop-options
