module Libcrux_sha3.Portable.Incremental.Bundle
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Simd.Portable in
  let open Libcrux_sha3.Traits in
  ()

/// SHAKE128 Xof state
type t_Shake128Xof = {
  f_state:Libcrux_sha3.Generic_keccak.Xof.t_KeccakXofState (mk_usize 1) (mk_usize 168) u64
}

/// SHAKE256 Xof state
type t_Shake256Xof = {
  f_state:Libcrux_sha3.Generic_keccak.Xof.t_KeccakXofState (mk_usize 1) (mk_usize 136) u64
}

/// Create a new SHAKE-128 state object.
let shake128_init (_: Prims.unit) : Libcrux_sha3.Portable.t_KeccakState =
  { Libcrux_sha3.Portable.f_state = Libcrux_sha3.Generic_keccak.impl_2__new (mk_usize 1) #u64 () }
  <:
  Libcrux_sha3.Portable.t_KeccakState

/// Absorb
let shake128_absorb_final (s: Libcrux_sha3.Portable.t_KeccakState) (data0: t_Slice u8)
    : Prims.Pure Libcrux_sha3.Portable.t_KeccakState
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data0 <: usize)
          <:
          Hax_lib.Int.t_Int) <
        (168 <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) =
  let s:Libcrux_sha3.Portable.t_KeccakState =
    {
      s with
      Libcrux_sha3.Portable.f_state
      =
      Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 1)
        #u64
        (mk_usize 168)
        (mk_u8 31)
        s.Libcrux_sha3.Portable.f_state
        (let list = [data0] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
          Rust_primitives.Hax.array_of_list 1 list)
        (mk_usize 0)
        (Core_models.Slice.impl__len #u8 data0 <: usize)
    }
    <:
    Libcrux_sha3.Portable.t_KeccakState
  in
  s

/// Squeeze three blocks
let shake128_squeeze_first_three_blocks (s: Libcrux_sha3.Portable.t_KeccakState) (out0: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (504 <: Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (s_future: Libcrux_sha3.Portable.t_KeccakState), (out0_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize)) =
  let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64), (tmp1: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_first_three_blocks (mk_usize 168)
      s.Libcrux_sha3.Portable.f_state
      out0
  in
  let s:Libcrux_sha3.Portable.t_KeccakState =
    { s with Libcrux_sha3.Portable.f_state = tmp0 } <: Libcrux_sha3.Portable.t_KeccakState
  in
  let out0:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  s, out0 <: (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)

/// Squeeze five blocks
let shake128_squeeze_first_five_blocks (s: Libcrux_sha3.Portable.t_KeccakState) (out0: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (840 <: Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (s_future: Libcrux_sha3.Portable.t_KeccakState), (out0_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize)) =
  let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64), (tmp1: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_first_five_blocks (mk_usize 168)
      s.Libcrux_sha3.Portable.f_state
      out0
  in
  let s:Libcrux_sha3.Portable.t_KeccakState =
    { s with Libcrux_sha3.Portable.f_state = tmp0 } <: Libcrux_sha3.Portable.t_KeccakState
  in
  let out0:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  s, out0 <: (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)

/// Squeeze another block
let shake128_squeeze_next_block (s: Libcrux_sha3.Portable.t_KeccakState) (out0: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (168 <: Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (s_future: Libcrux_sha3.Portable.t_KeccakState), (out0_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize)) =
  let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64), (tmp1: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_next_block (mk_usize 168)
      s.Libcrux_sha3.Portable.f_state
      out0
      (mk_usize 0)
  in
  let s:Libcrux_sha3.Portable.t_KeccakState =
    { s with Libcrux_sha3.Portable.f_state = tmp0 } <: Libcrux_sha3.Portable.t_KeccakState
  in
  let out0:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  s, out0 <: (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)

/// Create a new SHAKE-256 state object.
let shake256_init (_: Prims.unit) : Libcrux_sha3.Portable.t_KeccakState =
  { Libcrux_sha3.Portable.f_state = Libcrux_sha3.Generic_keccak.impl_2__new (mk_usize 1) #u64 () }
  <:
  Libcrux_sha3.Portable.t_KeccakState

/// Absorb some data for SHAKE-256 for the last time
let shake256_absorb_final (s: Libcrux_sha3.Portable.t_KeccakState) (data: t_Slice u8)
    : Prims.Pure Libcrux_sha3.Portable.t_KeccakState
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data <: usize)
          <:
          Hax_lib.Int.t_Int) <
        (136 <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) =
  let s:Libcrux_sha3.Portable.t_KeccakState =
    {
      s with
      Libcrux_sha3.Portable.f_state
      =
      Libcrux_sha3.Generic_keccak.impl_2__absorb_final (mk_usize 1)
        #u64
        (mk_usize 136)
        (mk_u8 31)
        s.Libcrux_sha3.Portable.f_state
        (let list = [data] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
          Rust_primitives.Hax.array_of_list 1 list)
        (mk_usize 0)
        (Core_models.Slice.impl__len #u8 data <: usize)
    }
    <:
    Libcrux_sha3.Portable.t_KeccakState
  in
  s

/// Squeeze the first SHAKE-256 block
let shake256_squeeze_first_block (s: Libcrux_sha3.Portable.t_KeccakState) (out: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (136 <: Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (s_future: Libcrux_sha3.Portable.t_KeccakState), (out_future: t_Slice u8) = temp_0_ in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let out:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_first_block (mk_usize 136)
      s.Libcrux_sha3.Portable.f_state
      out
  in
  s, out <: (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)

/// Squeeze the next SHAKE-256 block
let shake256_squeeze_next_block (s: Libcrux_sha3.Portable.t_KeccakState) (out: t_Slice u8)
    : Prims.Pure (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (136 <: Hax_lib.Int.t_Int))
      (ensures
        fun temp_0_ ->
          let (s_future: Libcrux_sha3.Portable.t_KeccakState), (out_future: t_Slice u8) = temp_0_ in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64), (tmp1: t_Slice u8) =
    Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_next_block (mk_usize 136)
      s.Libcrux_sha3.Portable.f_state
      out
      (mk_usize 0)
  in
  let s:Libcrux_sha3.Portable.t_KeccakState =
    { s with Libcrux_sha3.Portable.f_state = tmp0 } <: Libcrux_sha3.Portable.t_KeccakState
  in
  let out:t_Slice u8 = tmp1 in
  let _:Prims.unit = () in
  s, out <: (Libcrux_sha3.Portable.t_KeccakState & t_Slice u8)

class t_Sealed (v_Self: Type0) = { __marker_trait_t_Sealed:Prims.unit }

/// An XOF
class t_Xof (v_Self: Type0) (v_RATE: usize) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Sealed v_Self;
  f_new_pre:Prims.unit -> Type0;
  f_new_post:Prims.unit -> v_Self -> Type0;
  f_new:x0: Prims.unit -> Prims.Pure v_Self (f_new_pre x0) (fun result -> f_new_post x0 result);
  f_absorb_pre:v_Self -> t_Slice u8 -> Type0;
  f_absorb_post:v_Self -> t_Slice u8 -> v_Self -> Type0;
  f_absorb:x0: v_Self -> x1: t_Slice u8
    -> Prims.Pure v_Self (f_absorb_pre x0 x1) (fun result -> f_absorb_post x0 x1 result);
  f_absorb_final_pre:v_Self -> t_Slice u8 -> Type0;
  f_absorb_final_post:v_Self -> t_Slice u8 -> v_Self -> Type0;
  f_absorb_final:x0: v_Self -> x1: t_Slice u8
    -> Prims.Pure v_Self (f_absorb_final_pre x0 x1) (fun result -> f_absorb_final_post x0 x1 result);
  f_squeeze_pre:v_Self -> t_Slice u8 -> Type0;
  f_squeeze_post:v_Self -> t_Slice u8 -> (v_Self & t_Slice u8) -> Type0;
  f_squeeze:x0: v_Self -> x1: t_Slice u8
    -> Prims.Pure (v_Self & t_Slice u8)
        (f_squeeze_pre x0 x1)
        (fun result -> f_squeeze_post x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_RATE:usize) {|i: t_Xof v_Self v_RATE|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__private: t_Sealed t_Shake128Xof = { __marker_trait_t_Sealed = () }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_Xof t_Shake128Xof (mk_usize 168) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_new_pre = (fun (_: Prims.unit) -> true);
    f_new_post
    =
    (fun (_: Prims.unit) (result: t_Shake128Xof) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          result.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_new
    =
    (fun (_: Prims.unit) ->
        { f_state = Libcrux_sha3.Generic_keccak.Xof.impl__new (mk_usize 1) (mk_usize 168) #u64 () }
        <:
        t_Shake128Xof);
    f_absorb_pre
    =
    (fun (self_: t_Shake128Xof) (input: t_Slice u8) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb_post
    =
    (fun (self_: t_Shake128Xof) (input: t_Slice u8) (self_e_future: t_Shake128Xof) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          self_e_future.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb
    =
    (fun (self: t_Shake128Xof) (input: t_Slice u8) ->
        let self:t_Shake128Xof =
          {
            self with
            f_state
            =
            Libcrux_sha3.Generic_keccak.Xof.impl__absorb (mk_usize 1)
              (mk_usize 168)
              #u64
              self.f_state
              (let list = [input] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
          }
          <:
          t_Shake128Xof
        in
        self);
    f_absorb_final_pre
    =
    (fun (self_: t_Shake128Xof) (input: t_Slice u8) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb_final_post
    =
    (fun (self_: t_Shake128Xof) (input: t_Slice u8) (self_e_future: t_Shake128Xof) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          self_e_future.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb_final
    =
    (fun (self: t_Shake128Xof) (input: t_Slice u8) ->
        let self:t_Shake128Xof =
          {
            self with
            f_state
            =
            Libcrux_sha3.Generic_keccak.Xof.impl__absorb_final (mk_usize 1)
              (mk_usize 168)
              #u64
              (mk_u8 31)
              self.f_state
              (let list = [input] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
          }
          <:
          t_Shake128Xof
        in
        self);
    f_squeeze_pre
    =
    (fun (self_: t_Shake128Xof) (out: t_Slice u8) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_squeeze_post
    =
    (fun
        (self_: t_Shake128Xof)
        (out: t_Slice u8)
        (self_e_future, out_future: (t_Shake128Xof & t_Slice u8))
        ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 168)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len &&
        (Core_models.Slice.impl__len #u8 out_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out <: usize));
    f_squeeze
    =
    fun (self: t_Shake128Xof) (out: t_Slice u8) ->
      let
      (tmp0: Libcrux_sha3.Generic_keccak.Xof.t_KeccakXofState (mk_usize 1) (mk_usize 168) u64),
      (tmp1: t_Slice u8) =
        Libcrux_sha3.Generic_keccak.Xof.impl_1__squeeze (mk_usize 168) #u64 self.f_state out
      in
      let self:t_Shake128Xof = { self with f_state = tmp0 } <: t_Shake128Xof in
      let out:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      self, out <: (t_Shake128Xof & t_Slice u8)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl__from__private: t_Sealed t_Shake256Xof = { __marker_trait_t_Sealed = () }

/// Shake256 XOF in absorb state
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: t_Xof t_Shake256Xof (mk_usize 136) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_new_pre = (fun (_: Prims.unit) -> true);
    f_new_post
    =
    (fun (_: Prims.unit) (result: t_Shake256Xof) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          result.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_new
    =
    (fun (_: Prims.unit) ->
        { f_state = Libcrux_sha3.Generic_keccak.Xof.impl__new (mk_usize 1) (mk_usize 136) #u64 () }
        <:
        t_Shake256Xof);
    f_absorb_pre
    =
    (fun (self_: t_Shake256Xof) (input: t_Slice u8) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb_post
    =
    (fun (self_: t_Shake256Xof) (input: t_Slice u8) (self_e_future: t_Shake256Xof) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          self_e_future.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb
    =
    (fun (self: t_Shake256Xof) (input: t_Slice u8) ->
        let self:t_Shake256Xof =
          {
            self with
            f_state
            =
            Libcrux_sha3.Generic_keccak.Xof.impl__absorb (mk_usize 1)
              (mk_usize 136)
              #u64
              self.f_state
              (let list = [input] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
          }
          <:
          t_Shake256Xof
        in
        self);
    f_absorb_final_pre
    =
    (fun (self_: t_Shake256Xof) (input: t_Slice u8) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb_final_post
    =
    (fun (self_: t_Shake256Xof) (input: t_Slice u8) (self_e_future: t_Shake256Xof) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          self_e_future.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_absorb_final
    =
    (fun (self: t_Shake256Xof) (input: t_Slice u8) ->
        let self:t_Shake256Xof =
          {
            self with
            f_state
            =
            Libcrux_sha3.Generic_keccak.Xof.impl__absorb_final (mk_usize 1)
              (mk_usize 136)
              #u64
              (mk_u8 31)
              self.f_state
              (let list = [input] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
          }
          <:
          t_Shake256Xof
        in
        self);
    f_squeeze_pre
    =
    (fun (self_: t_Shake256Xof) (out: t_Slice u8) ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len);
    f_squeeze_post
    =
    (fun
        (self_: t_Shake256Xof)
        (out: t_Slice u8)
        (self_e_future, out_future: (t_Shake256Xof & t_Slice u8))
        ->
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv (mk_usize 136)
          self_.f_state.Libcrux_sha3.Generic_keccak.Xof.f_buf_len &&
        (Core_models.Slice.impl__len #u8 out_future <: usize) =.
        (Core_models.Slice.impl__len #u8 out <: usize));
    f_squeeze
    =
    fun (self: t_Shake256Xof) (out: t_Slice u8) ->
      let
      (tmp0: Libcrux_sha3.Generic_keccak.Xof.t_KeccakXofState (mk_usize 1) (mk_usize 136) u64),
      (tmp1: t_Slice u8) =
        Libcrux_sha3.Generic_keccak.Xof.impl_1__squeeze (mk_usize 136) #u64 self.f_state out
      in
      let self:t_Shake256Xof = { self with f_state = tmp0 } <: t_Shake256Xof in
      let out:t_Slice u8 = tmp1 in
      let _:Prims.unit = () in
      self, out <: (t_Shake256Xof & t_Slice u8)
  }
