module Libcrux_sha3.Generic_keccak
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Traits in
  ()

type t_KeccakState (v_N: usize) (v_T: Type0) {| i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |} = {
  f_st:t_Array v_T (mk_usize 25)
}

let impl_1
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Clone.t_Clone v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
    : Core_models.Clone.t_Clone (t_KeccakState v_N v_T) =
  { f_clone = (fun x -> x); f_clone_pre = (fun _ -> True); f_clone_post = (fun _ _ -> True) }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl':
    v_N: usize ->
    #v_T: Type0 ->
    {| i0: Core_models.Marker.t_Copy v_T |} ->
    {| i1: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
  -> Core_models.Marker.t_Copy (t_KeccakState v_N v_T)

unfold
let impl
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Marker.t_Copy v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
     = impl' v_N #v_T #i0 #i1

/// Create a new Shake128 x4 state.
let impl_2__new
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (_: Prims.unit)
    : t_KeccakState v_N v_T =
  {
    f_st
    =
    Rust_primitives.Hax.repeat (Libcrux_sha3.Traits.f_zero #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          ()
        <:
        v_T)
      (mk_usize 25)
  }
  <:
  t_KeccakState v_N v_T

/// Set element `[i, j] = v`.
let impl_2__set
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (i j: usize)
      (v: v_T)
    : Prims.Pure (t_KeccakState v_N v_T)
      (requires i <. mk_usize 5 && j <. mk_usize 5)
      (fun _ -> Prims.l_True) =
  let self:t_KeccakState v_N v_T =
    { self with f_st = Libcrux_sha3.Traits.set_ij v_N #v_T self.f_st i j v }
    <:
    t_KeccakState v_N v_T
  in
  self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
    : Core_models.Ops.Index.t_Index (t_KeccakState v_N v_T) (usize & usize) =
  {
    f_Output = v_T;
    f_index_pre
    =
    (fun (self_: t_KeccakState v_N v_T) (index: (usize & usize)) ->
        index._1 <. mk_usize 5 && index._2 <. mk_usize 5);
    f_index_post = (fun (self: t_KeccakState v_N v_T) (index: (usize & usize)) (out: v_T) -> true);
    f_index
    =
    fun (self: t_KeccakState v_N v_T) (index: (usize & usize)) ->
      Libcrux_sha3.Traits.get_ij v_N #v_T self.f_st index._1 index._2
  }

let impl_2__theta
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
    : (t_KeccakState v_N v_T & t_Array v_T (mk_usize 5)) =
  let (c: t_Array v_T (mk_usize 5)):t_Array v_T (mk_usize 5) =
    let list =
      [
        Libcrux_sha3.Traits.f_xor5 #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 0 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 1, mk_usize 0 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 2, mk_usize 0 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 3, mk_usize 0 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 4, mk_usize 0 <: (usize & usize) ] <: v_T);
        Libcrux_sha3.Traits.f_xor5 #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 1 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 1, mk_usize 1 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 2, mk_usize 1 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 3, mk_usize 1 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 4, mk_usize 1 <: (usize & usize) ] <: v_T);
        Libcrux_sha3.Traits.f_xor5 #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 2 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 1, mk_usize 2 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 2, mk_usize 2 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 3, mk_usize 2 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 4, mk_usize 2 <: (usize & usize) ] <: v_T);
        Libcrux_sha3.Traits.f_xor5 #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 3 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 1, mk_usize 3 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 2, mk_usize 3 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 3, mk_usize 3 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 4, mk_usize 3 <: (usize & usize) ] <: v_T);
        Libcrux_sha3.Traits.f_xor5 #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 4 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 1, mk_usize 4 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 2, mk_usize 4 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 3, mk_usize 4 <: (usize & usize) ] <: v_T)
          (self.[ mk_usize 4, mk_usize 4 <: (usize & usize) ] <: v_T)
      ]
    in
    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 5);
    Rust_primitives.Hax.array_of_list 5 list
  in
  let hax_temp_output:t_Array v_T (mk_usize 5) =
    let list =
      [
        Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (c.[ (mk_usize 0 +! mk_usize 4 <: usize) %! mk_usize 5 <: usize ] <: v_T)
          (c.[ (mk_usize 0 +! mk_usize 1 <: usize) %! mk_usize 5 <: usize ] <: v_T);
        Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (c.[ (mk_usize 1 +! mk_usize 4 <: usize) %! mk_usize 5 <: usize ] <: v_T)
          (c.[ (mk_usize 1 +! mk_usize 1 <: usize) %! mk_usize 5 <: usize ] <: v_T);
        Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (c.[ (mk_usize 2 +! mk_usize 4 <: usize) %! mk_usize 5 <: usize ] <: v_T)
          (c.[ (mk_usize 2 +! mk_usize 1 <: usize) %! mk_usize 5 <: usize ] <: v_T);
        Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (c.[ (mk_usize 3 +! mk_usize 4 <: usize) %! mk_usize 5 <: usize ] <: v_T)
          (c.[ (mk_usize 3 +! mk_usize 1 <: usize) %! mk_usize 5 <: usize ] <: v_T);
        Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (c.[ (mk_usize 4 +! mk_usize 4 <: usize) %! mk_usize 5 <: usize ] <: v_T)
          (c.[ (mk_usize 4 +! mk_usize 1 <: usize) %! mk_usize 5 <: usize ] <: v_T)
      ]
    in
    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 5);
    Rust_primitives.Hax.array_of_list 5 list
  in
  self, hax_temp_output <: (t_KeccakState v_N v_T & t_Array v_T (mk_usize 5))

let impl_2__rho_0_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (t: t_Array v_T (mk_usize 5))
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 0)
      (Libcrux_sha3.Traits.f_xor #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 0 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 0 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 0)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 36)
          (mk_i32 28)
          (self.[ mk_usize 1, mk_usize 0 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 0 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 0)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 3)
          (mk_i32 61)
          (self.[ mk_usize 2, mk_usize 0 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 0 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 0)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 41)
          (mk_i32 23)
          (self.[ mk_usize 3, mk_usize 0 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 0 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 0)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 18)
          (mk_i32 46)
          (self.[ mk_usize 4, mk_usize 0 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 0 ] <: v_T)
        <:
        v_T)
  in
  self

let impl_2__rho_1_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (t: t_Array v_T (mk_usize 5))
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 1)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 1)
          (mk_i32 63)
          (self.[ mk_usize 0, mk_usize 1 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 1 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 1)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 44)
          (mk_i32 20)
          (self.[ mk_usize 1, mk_usize 1 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 1 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 1)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 10)
          (mk_i32 54)
          (self.[ mk_usize 2, mk_usize 1 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 1 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 1)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 45)
          (mk_i32 19)
          (self.[ mk_usize 3, mk_usize 1 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 1 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 1)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 2)
          (mk_i32 62)
          (self.[ mk_usize 4, mk_usize 1 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 1 ] <: v_T)
        <:
        v_T)
  in
  self

let impl_2__rho_2_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (t: t_Array v_T (mk_usize 5))
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 2)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 62)
          (mk_i32 2)
          (self.[ mk_usize 0, mk_usize 2 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 2 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 2)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 6)
          (mk_i32 58)
          (self.[ mk_usize 1, mk_usize 2 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 2 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 2)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 43)
          (mk_i32 21)
          (self.[ mk_usize 2, mk_usize 2 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 2 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 2)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 15)
          (mk_i32 49)
          (self.[ mk_usize 3, mk_usize 2 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 2 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 2)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 61)
          (mk_i32 3)
          (self.[ mk_usize 4, mk_usize 2 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 2 ] <: v_T)
        <:
        v_T)
  in
  self

let impl_2__rho_3_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (t: t_Array v_T (mk_usize 5))
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 3)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 28)
          (mk_i32 36)
          (self.[ mk_usize 0, mk_usize 3 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 3 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 3)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 55)
          (mk_i32 9)
          (self.[ mk_usize 1, mk_usize 3 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 3 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 3)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 25)
          (mk_i32 39)
          (self.[ mk_usize 2, mk_usize 3 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 3 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 3)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 21)
          (mk_i32 43)
          (self.[ mk_usize 3, mk_usize 3 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 3 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 3)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 56)
          (mk_i32 8)
          (self.[ mk_usize 4, mk_usize 3 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 3 ] <: v_T)
        <:
        v_T)
  in
  self

let impl_2__rho_4_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (t: t_Array v_T (mk_usize 5))
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 4)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 27)
          (mk_i32 37)
          (self.[ mk_usize 0, mk_usize 4 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 4 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 4)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 20)
          (mk_i32 44)
          (self.[ mk_usize 1, mk_usize 4 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 4 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 4)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 39)
          (mk_i32 25)
          (self.[ mk_usize 2, mk_usize 4 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 4 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 4)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 8)
          (mk_i32 56)
          (self.[ mk_usize 3, mk_usize 4 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 4 ] <: v_T)
        <:
        v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 4)
      (Libcrux_sha3.Traits.f_xor_and_rotate #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (mk_i32 14)
          (mk_i32 50)
          (self.[ mk_usize 4, mk_usize 4 <: (usize & usize) ] <: v_T)
          (t.[ mk_usize 4 ] <: v_T)
        <:
        v_T)
  in
  self

let impl_2__rho
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (t: t_Array v_T (mk_usize 5))
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T = impl_2__rho_0_ v_N #v_T self t in
  let self:t_KeccakState v_N v_T = impl_2__rho_1_ v_N #v_T self t in
  let self:t_KeccakState v_N v_T = impl_2__rho_2_ v_N #v_T self t in
  let self:t_KeccakState v_N v_T = impl_2__rho_3_ v_N #v_T self t in
  let self:t_KeccakState v_N v_T = impl_2__rho_4_ v_N #v_T self t in
  self

let impl_2__pi_0_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self old: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 0)
      (old.[ mk_usize 0, mk_usize 3 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 0)
      (old.[ mk_usize 0, mk_usize 1 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 0)
      (old.[ mk_usize 0, mk_usize 4 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 0)
      (old.[ mk_usize 0, mk_usize 2 <: (usize & usize) ] <: v_T)
  in
  self

let impl_2__pi_1_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self old: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 1)
      (old.[ mk_usize 1, mk_usize 1 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 1)
      (old.[ mk_usize 1, mk_usize 4 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 1)
      (old.[ mk_usize 1, mk_usize 2 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 1)
      (old.[ mk_usize 1, mk_usize 0 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 1)
      (old.[ mk_usize 1, mk_usize 3 <: (usize & usize) ] <: v_T)
  in
  self

let impl_2__pi_2_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self old: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 2)
      (old.[ mk_usize 2, mk_usize 2 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 2)
      (old.[ mk_usize 2, mk_usize 0 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 2)
      (old.[ mk_usize 2, mk_usize 3 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 2)
      (old.[ mk_usize 2, mk_usize 1 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 2)
      (old.[ mk_usize 2, mk_usize 4 <: (usize & usize) ] <: v_T)
  in
  self

let impl_2__pi_3_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self old: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 3)
      (old.[ mk_usize 3, mk_usize 3 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 3)
      (old.[ mk_usize 3, mk_usize 1 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 3)
      (old.[ mk_usize 3, mk_usize 4 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 3)
      (old.[ mk_usize 3, mk_usize 2 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 3)
      (old.[ mk_usize 3, mk_usize 0 <: (usize & usize) ] <: v_T)
  in
  self

let impl_2__pi_4_
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self old: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 4)
      (old.[ mk_usize 4, mk_usize 4 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 1)
      (mk_usize 4)
      (old.[ mk_usize 4, mk_usize 2 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 2)
      (mk_usize 4)
      (old.[ mk_usize 4, mk_usize 0 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 3)
      (mk_usize 4)
      (old.[ mk_usize 4, mk_usize 3 <: (usize & usize) ] <: v_T)
  in
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 4)
      (mk_usize 4)
      (old.[ mk_usize 4, mk_usize 1 <: (usize & usize) ] <: v_T)
  in
  self

let impl_2__pi
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let (old: t_KeccakState v_N v_T):t_KeccakState v_N v_T = self in
  let self:t_KeccakState v_N v_T = impl_2__pi_0_ v_N #v_T self old in
  let self:t_KeccakState v_N v_T = impl_2__pi_1_ v_N #v_T self old in
  let self:t_KeccakState v_N v_T = impl_2__pi_2_ v_N #v_T self old in
  let self:t_KeccakState v_N v_T = impl_2__pi_3_ v_N #v_T self old in
  let self:t_KeccakState v_N v_T = impl_2__pi_4_ v_N #v_T self old in
  self

let impl_2__chi
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let old:t_KeccakState v_N v_T = self in
  let self:t_KeccakState v_N v_T =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 5)
      (fun self temp_1_ ->
          let self:t_KeccakState v_N v_T = self in
          let _:usize = temp_1_ in
          true)
      self
      (fun self i ->
          let self:t_KeccakState v_N v_T = self in
          let i:usize = i in
          Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
            (mk_usize 5)
            (fun self temp_1_ ->
                let self:t_KeccakState v_N v_T = self in
                let _:usize = temp_1_ in
                true)
            self
            (fun self j ->
                let self:t_KeccakState v_N v_T = self in
                let j:usize = j in
                impl_2__set v_N
                  #v_T
                  self
                  i
                  j
                  (Libcrux_sha3.Traits.f_and_not_xor #v_T
                      #v_N
                      #FStar.Tactics.Typeclasses.solve
                      (self.[ i, j <: (usize & usize) ] <: v_T)
                      (old.[ i, ((j +! mk_usize 2 <: usize) %! mk_usize 5 <: usize)
                          <:
                          (usize & usize) ]
                        <:
                        v_T)
                      (old.[ i, ((j +! mk_usize 1 <: usize) %! mk_usize 5 <: usize)
                          <:
                          (usize & usize) ]
                        <:
                        v_T)
                    <:
                    v_T)
                <:
                t_KeccakState v_N v_T)
          <:
          t_KeccakState v_N v_T)
  in
  self

let impl_2__iota
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
      (i: usize)
    : Prims.Pure (t_KeccakState v_N v_T)
      (requires
        i <.
        (Core_models.Slice.impl__len #u64
            (Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS <: t_Slice u64)
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let self:t_KeccakState v_N v_T =
    impl_2__set v_N
      #v_T
      self
      (mk_usize 0)
      (mk_usize 0)
      (Libcrux_sha3.Traits.f_xor_constant #v_T
          #v_N
          #FStar.Tactics.Typeclasses.solve
          (self.[ mk_usize 0, mk_usize 0 <: (usize & usize) ] <: v_T)
          (Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS.[ i ] <: u64)
        <:
        v_T)
  in
  self

let impl_2__keccakf1600
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (self: t_KeccakState v_N v_T)
    : t_KeccakState v_N v_T =
  let self:t_KeccakState v_N v_T =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 24)
      (fun self temp_1_ ->
          let self:t_KeccakState v_N v_T = self in
          let _:usize = temp_1_ in
          true)
      self
      (fun self i ->
          let self:t_KeccakState v_N v_T = self in
          let i:usize = i in
          let (tmp0: t_KeccakState v_N v_T), (out: t_Array v_T (mk_usize 5)) =
            impl_2__theta v_N #v_T self
          in
          let self:t_KeccakState v_N v_T = tmp0 in
          let t:t_Array v_T (mk_usize 5) = out in
          let self:t_KeccakState v_N v_T = impl_2__rho v_N #v_T self t in
          let self:t_KeccakState v_N v_T = impl_2__pi v_N #v_T self in
          let self:t_KeccakState v_N v_T = impl_2__chi v_N #v_T self in
          let self:t_KeccakState v_N v_T = impl_2__iota v_N #v_T self i in
          self)
  in
  self

let impl_2__absorb_block
      (v_N: usize)
      (#v_T: Type0)
      (v_RATE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_sha3.Traits.t_Absorb (t_KeccakState v_N v_T) v_N)
      (self: t_KeccakState v_N v_T)
      (input: t_Array (t_Slice u8) v_N)
      (start: usize)
    : Prims.Pure (t_KeccakState v_N v_T)
      (requires
        b2t
        ((v_N <>. mk_usize 0 <: bool) && (Libcrux_sha3.Proof_utils.valid_rate v_RATE <: bool) &&
          (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
              (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int) <=
            (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                    (input.[ mk_usize 0 ] <: t_Slice u8)
                  <:
                  usize)
              <:
              Hax_lib.Int.t_Int)
            <:
            bool)) /\ Libcrux_sha3.Proof_utils.slices_same_len v_N input)
      (fun _ -> Prims.l_True) =
  let self:t_KeccakState v_N v_T =
    Libcrux_sha3.Traits.f_load_block #(t_KeccakState v_N v_T)
      #v_N
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      self
      input
      start
  in
  let self:t_KeccakState v_N v_T = impl_2__keccakf1600 v_N #v_T self in
  self

let impl_2__absorb_final
      (v_N: usize)
      (#v_T: Type0)
      (v_RATE: usize)
      (v_DELIM: u8)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_sha3.Traits.t_KeccakItem v_T v_N)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_sha3.Traits.t_Absorb (t_KeccakState v_N v_T) v_N)
      (self: t_KeccakState v_N v_T)
      (input: t_Array (t_Slice u8) v_N)
      (start len: usize)
    : Prims.Pure (t_KeccakState v_N v_T)
      (requires
        b2t
        ((v_N <>. mk_usize 0 <: bool) && (Libcrux_sha3.Proof_utils.valid_rate v_RATE <: bool) &&
          (len <. v_RATE <: bool) &&
          (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
              (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int) <=
            (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                    (input.[ mk_usize 0 ] <: t_Slice u8)
                  <:
                  usize)
              <:
              Hax_lib.Int.t_Int)
            <:
            bool)) /\ Libcrux_sha3.Proof_utils.slices_same_len v_N input)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((v_N >. mk_usize 0 <: bool) && (len <. v_RATE <: bool))
      in
      ()
  in
  let self:t_KeccakState v_N v_T =
    Libcrux_sha3.Traits.f_load_last #(t_KeccakState v_N v_T)
      #v_N
      #FStar.Tactics.Typeclasses.solve
      v_RATE
      v_DELIM
      self
      input
      start
      len
  in
  let self:t_KeccakState v_N v_T = impl_2__keccakf1600 v_N #v_T self in
  self
