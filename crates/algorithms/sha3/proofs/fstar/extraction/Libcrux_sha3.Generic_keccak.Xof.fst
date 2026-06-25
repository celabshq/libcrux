module Libcrux_sha3.Generic_keccak.Xof
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_sha3.Traits in
  ()

#push-options "--split_queries always --z3rlimit 300"

/// The internal keccak state that can also buffer inputs to absorb.
/// This is used in the general xof APIs.
type t_KeccakXofState
  (v_PARALLEL_LANES: usize) (v_RATE: usize) (v_STATE: Type0)
  {| i0: Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES |}
  = {
  f_inner:Libcrux_sha3.Generic_keccak.t_KeccakState v_PARALLEL_LANES v_STATE;
  f_buf:t_Array (t_Array u8 v_RATE) v_PARALLEL_LANES;
  f_buf_len:usize;
  f_sponge:bool
}

let buf_to_slices
      (v_PARALLEL_LANES v_RATE: usize)
      (buf: t_Array (t_Array u8 v_RATE) v_PARALLEL_LANES)
    : t_Array (t_Slice u8) v_PARALLEL_LANES =
  Rust_primitives.Slice.array_from_fn #(t_Slice u8)
    v_PARALLEL_LANES
    #(usize -> u8)
    (fun i -> Core_models.Array.impl_23__as_slice #u8 v_RATE (buf.[ i ]))

/// An all zero block
let impl__zero_block
      (v_PARALLEL_LANES: usize)
      (v_RATE: usize)
      (#v_STATE: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES)
      (_: Prims.unit)
    : t_Array u8 v_RATE = Rust_primitives.Hax.repeat (mk_u8 0) v_RATE

/// Generate a new keccak xof state.
let impl__new
      (v_PARALLEL_LANES: usize)
      (v_RATE: usize)
      (#v_STATE: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES)
      (_: Prims.unit)
    : Prims.Pure (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (requires v_PARALLEL_LANES =. mk_usize 1 && Libcrux_sha3.Proof_utils.valid_rate v_RATE)
      (ensures
        fun result ->
          let result:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = result in
          Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE result.f_buf_len) =
  {
    f_inner = Libcrux_sha3.Generic_keccak.impl_2__new v_PARALLEL_LANES #v_STATE ();
    f_buf
    =
    Rust_primitives.Hax.repeat (impl__zero_block v_PARALLEL_LANES v_RATE #v_STATE ()
        <:
        t_Array u8 v_RATE)
      v_PARALLEL_LANES;
    f_buf_len = mk_usize 0;
    f_sponge = false
  }
  <:
  t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE

/// Try to complete the internal partial buffer by consuming the minimum required
/// number of bytes from the provided `inputs` so that `self.buf` becomes exactly
/// one full block of size `RATE`.
/// Behaviour:
/// - If `self.buf_len` is 0 (no buffered bytes) or already equal to `RATE`
///   (already a full block), or if the combined available bytes in `inputs` are
///   not enough to reach `RATE`, the function does nothing and returns 0.
/// - If `0 < self.buf_len < RATE` and `inputs[..]` contain at least
///   `RATE - self.buf_len` bytes, the function copies exactly
///   `consumed = RATE - self.buf_len` bytes from each lane `inputs[i]` into
///   `self.buf[i]` starting at the current `self.buf_len` offset, sets
///   `self.buf_len = RATE`, and returns `consumed`.
/// Returns the `consumed` bytes from `inputs` if there\'s enough buffered
/// content to consume, and `0` otherwise.
/// If `consumed > 0` is returned, `self.buf` contains a full block to be
/// loaded.
let impl__fill_buffer
      (v_PARALLEL_LANES v_RATE: usize)
      (#v_STATE: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES)
      (self: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (inputs: t_Array (t_Slice u8) v_PARALLEL_LANES)
    : Prims.Pure (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE & usize)
      (requires
        v_PARALLEL_LANES =. mk_usize 1 &&
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self.f_buf_len)
      (ensures
        fun temp_0_ ->
          let (self_e_future: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE), (consumed: usize) =
            temp_0_
          in
          Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self_e_future.f_buf_len &&
          (if consumed =. mk_usize 0
            then
              self_e_future.f_buf_len =. self.f_buf_len &&
              (self.f_buf_len =. mk_usize 0 || self.f_buf_len =. v_RATE ||
              ((Rust_primitives.Hax.Int.from_machine self.f_buf_len <: Hax_lib.Int.t_Int) +
                (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                        (inputs.[ mk_usize 0 ] <: t_Slice u8)
                      <:
                      usize)
                  <:
                  Hax_lib.Int.t_Int)
                <:
                Hax_lib.Int.t_Int) <
              (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int))
            else
              self_e_future.f_buf_len =. v_RATE && consumed =. (v_RATE -! self.f_buf_len <: usize))) =
  let input_len:usize = Core_models.Slice.impl__len #u8 (inputs.[ mk_usize 0 ] <: t_Slice u8) in
  let (self: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE), (hax_temp_output: usize) =
    if self.f_buf_len <>. mk_usize 0 && input_len >=. (v_RATE -! self.f_buf_len <: usize)
    then
      let consumed:usize = v_RATE -! self.f_buf_len in
      let self_buf_len:usize = self.f_buf_len in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
          v_PARALLEL_LANES
          (fun self temp_1_ ->
              let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self in
              let _:usize = temp_1_ in
              self.f_buf_len =. self_buf_len <: bool)
          self
          (fun self i ->
              let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self in
              let i:usize = i in
              let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
                {
                  self with
                  f_buf
                  =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize self.f_buf
                    i
                    (Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from (self.f_buf.[ i
                          ]
                          <:
                          t_Array u8 v_RATE)
                        ({ Core_models.Ops.Range.f_start = self.f_buf_len }
                          <:
                          Core_models.Ops.Range.t_RangeFrom usize)
                        (Core_models.Slice.impl__copy_from_slice #u8
                            ((self.f_buf.[ i ] <: t_Array u8 v_RATE).[ {
                                  Core_models.Ops.Range.f_start = self.f_buf_len
                                }
                                <:
                                Core_models.Ops.Range.t_RangeFrom usize ]
                              <:
                              t_Slice u8)
                            ((inputs.[ i ] <: t_Slice u8).[ {
                                  Core_models.Ops.Range.f_end = consumed
                                }
                                <:
                                Core_models.Ops.Range.t_RangeTo usize ]
                              <:
                              t_Slice u8)
                          <:
                          t_Slice u8)
                      <:
                      t_Array u8 v_RATE)
                }
                <:
                t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
              in
              self)
      in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        { self with f_buf_len = v_RATE } <: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
      in
      self, consumed <: (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE & usize)
    else self, mk_usize 0 <: (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE & usize)
  in
  self, hax_temp_output <: (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE & usize)

let impl__absorb_full
      (v_PARALLEL_LANES v_RATE: usize)
      (#v_STATE: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_sha3.Traits.t_Absorb
            (Libcrux_sha3.Generic_keccak.t_KeccakState v_PARALLEL_LANES v_STATE) v_PARALLEL_LANES)
      (self: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (inputs: t_Array (t_Slice u8) v_PARALLEL_LANES)
    : Prims.Pure (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE & usize)
      (requires
        v_PARALLEL_LANES =. mk_usize 1 &&
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self.f_buf_len)
      (ensures
        fun temp_0_ ->
          let (self_e_future: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE), (remainder: usize)
          =
            temp_0_
          in
          Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self_e_future.f_buf_len &&
          ((Rust_primitives.Hax.Int.from_machine self_e_future.f_buf_len <: Hax_lib.Int.t_Int) +
            (Rust_primitives.Hax.Int.from_machine remainder <: Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) <=
          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit = Hax_lib.v_assert (v_PARALLEL_LANES >. mk_usize 0 <: bool) in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit = Hax_lib.v_assert (self.f_buf_len <=. v_RATE <: bool) in
      ()
  in
  let (tmp0: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE), (out: usize) =
    impl__fill_buffer v_PARALLEL_LANES v_RATE #v_STATE self inputs
  in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = tmp0 in
  let consumed:usize = out in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
    if self.f_buf_len =. v_RATE
    then
      let borrowed:t_Array (t_Slice u8) v_PARALLEL_LANES =
        buf_to_slices v_PARALLEL_LANES v_RATE self.f_buf
      in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        {
          self with
          f_inner
          =
          Libcrux_sha3.Traits.f_load_block #(Libcrux_sha3.Generic_keccak.t_KeccakState
                v_PARALLEL_LANES v_STATE)
            #v_PARALLEL_LANES
            #FStar.Tactics.Typeclasses.solve
            v_RATE
            self.f_inner
            borrowed
            (mk_usize 0)
        }
        <:
        t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
      in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        {
          self with
          f_inner
          =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_PARALLEL_LANES #v_STATE self.f_inner
        }
        <:
        t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
      in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        { self with f_buf_len = mk_usize 0 } <: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
      in
      self
    else self
  in
  let input_to_consume:usize =
    (Core_models.Slice.impl__len #u8 (inputs.[ mk_usize 0 ] <: t_Slice u8) <: usize) -! consumed
  in
  let num_blocks:usize = input_to_consume /! v_RATE in
  let remainder:usize = input_to_consume %! v_RATE in
  let v_end:usize = consumed +! (num_blocks *! v_RATE <: usize) in
  let _:Prims.unit =
    Hax_lib.v_assert (v_end <=.
        (Core_models.Slice.impl__len #u8 (inputs.[ mk_usize 0 ] <: t_Slice u8) <: usize)
        <:
        bool)
  in
  let (self_buf_len: usize), (v_end: usize) = self.f_buf_len, v_end <: (usize & usize) in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      num_blocks
      (fun self temp_1_ ->
          let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self in
          let _:usize = temp_1_ in
          self.f_buf_len =. self_buf_len <: bool)
      self
      (fun self i ->
          let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self in
          let i:usize = i in
          let _:Prims.unit =
            Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i num_blocks v_RATE
          in
          let start:usize = (i *! v_RATE <: usize) +! consumed in
          let _:Prims.unit = Hax_lib.v_assert ((start +! v_RATE <: usize) <=. v_end <: bool) in
          let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
            {
              self with
              f_inner
              =
              Libcrux_sha3.Traits.f_load_block #(Libcrux_sha3.Generic_keccak.t_KeccakState
                    v_PARALLEL_LANES v_STATE)
                #v_PARALLEL_LANES
                #FStar.Tactics.Typeclasses.solve
                v_RATE
                self.f_inner
                inputs
                start
            }
            <:
            t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
          in
          let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
            {
              self with
              f_inner
              =
              Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_PARALLEL_LANES #v_STATE self.f_inner
            }
            <:
            t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
          in
          self)
  in
  let hax_temp_output:usize = remainder in
  self, hax_temp_output <: (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE & usize)

/// Absorb
/// This function takes any number of bytes to absorb and buffers if it\'s not enough.
/// The function assumes that all input slices in `inputs` have the same length.
/// Only a multiple of `RATE` blocks are absorbed.
/// For the remaining bytes [`absorb_final`] needs to be called.
/// This works best with relatively small `inputs`.
let impl__absorb
      (v_PARALLEL_LANES v_RATE: usize)
      (#v_STATE: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_sha3.Traits.t_Absorb
            (Libcrux_sha3.Generic_keccak.t_KeccakState v_PARALLEL_LANES v_STATE) v_PARALLEL_LANES)
      (self: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (inputs: t_Array (t_Slice u8) v_PARALLEL_LANES)
    : Prims.Pure (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (requires
        v_PARALLEL_LANES =. mk_usize 1 &&
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self.f_buf_len)
      (ensures
        fun self_e_future ->
          let self_e_future:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self_e_future in
          Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self_e_future.f_buf_len) =
  let (tmp0: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE), (out: usize) =
    impl__absorb_full v_PARALLEL_LANES v_RATE #v_STATE self inputs
  in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = tmp0 in
  let remainder:usize = out in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
    if remainder >. mk_usize 0
    then
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit =
            Hax_lib.v_assert ((self.f_buf_len =. mk_usize 0 <: bool) ||
                ((self.f_buf_len +! remainder <: usize) <=. v_RATE <: bool))
          in
          ()
      in
      let _:Prims.unit =
        Hax_lib.v_assert (((Rust_primitives.Hax.Int.from_machine remainder <: Hax_lib.Int.t_Int) +
              (Rust_primitives.Hax.Int.from_machine self.f_buf_len <: Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int) <=
            (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
            <:
            bool)
      in
      let input_len:usize = Core_models.Slice.impl__len #u8 (inputs.[ mk_usize 0 ] <: t_Slice u8) in
      let self_buf_len:usize = self.f_buf_len in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
          v_PARALLEL_LANES
          (fun self temp_1_ ->
              let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self in
              let _:usize = temp_1_ in
              self.f_buf_len =. self_buf_len <: bool)
          self
          (fun self i ->
              let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self in
              let i:usize = i in
              let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
                {
                  self with
                  f_buf
                  =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize self.f_buf
                    i
                    (Rust_primitives.Hax.Monomorphized_update_at.update_at_range (self.f_buf.[ i ]
                          <:
                          t_Array u8 v_RATE)
                        ({
                            Core_models.Ops.Range.f_start = self.f_buf_len;
                            Core_models.Ops.Range.f_end = self.f_buf_len +! remainder <: usize
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize)
                        (Core_models.Slice.impl__copy_from_slice #u8
                            ((self.f_buf.[ i ] <: t_Array u8 v_RATE).[ {
                                  Core_models.Ops.Range.f_start = self.f_buf_len;
                                  Core_models.Ops.Range.f_end = self.f_buf_len +! remainder <: usize
                                }
                                <:
                                Core_models.Ops.Range.t_Range usize ]
                              <:
                              t_Slice u8)
                            ((inputs.[ i ] <: t_Slice u8).[ {
                                  Core_models.Ops.Range.f_start = input_len -! remainder <: usize;
                                  Core_models.Ops.Range.f_end = input_len
                                }
                                <:
                                Core_models.Ops.Range.t_Range usize ]
                              <:
                              t_Slice u8)
                          <:
                          t_Slice u8)
                      <:
                      t_Array u8 v_RATE)
                }
                <:
                t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
              in
              self)
      in
      let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
        { self with f_buf_len = self.f_buf_len +! remainder }
        <:
        t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
      in
      self
    else self
  in
  self

/// Absorb a final block.
/// The `inputs` block may be empty. Everything in the `inputs` block beyond
/// `RATE` bytes is ignored.
let impl__absorb_final
      (v_PARALLEL_LANES v_RATE: usize)
      (#v_STATE: Type0)
      (v_DELIMITER: u8)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE v_PARALLEL_LANES)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_sha3.Traits.t_Absorb
            (Libcrux_sha3.Generic_keccak.t_KeccakState v_PARALLEL_LANES v_STATE) v_PARALLEL_LANES)
      (self: t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (inputs: t_Array (t_Slice u8) v_PARALLEL_LANES)
    : Prims.Pure (t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE)
      (requires
        v_PARALLEL_LANES =. mk_usize 1 &&
        Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self.f_buf_len)
      (ensures
        fun self_e_future ->
          let self_e_future:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE = self_e_future in
          Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self_e_future.f_buf_len) =
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
    impl__absorb v_PARALLEL_LANES v_RATE #v_STATE self inputs
  in
  let borrowed:t_Array (t_Slice u8) v_PARALLEL_LANES =
    buf_to_slices v_PARALLEL_LANES v_RATE self.f_buf
  in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
    {
      self with
      f_inner
      =
      Libcrux_sha3.Traits.f_load_last #(Libcrux_sha3.Generic_keccak.t_KeccakState v_PARALLEL_LANES
            v_STATE)
        #v_PARALLEL_LANES
        #FStar.Tactics.Typeclasses.solve
        v_RATE
        v_DELIMITER
        self.f_inner
        borrowed
        (mk_usize 0)
        self.f_buf_len
    }
    <:
    t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
  in
  let self:t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE =
    {
      self with
      f_inner
      =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_PARALLEL_LANES #v_STATE self.f_inner
    }
    <:
    t_KeccakXofState v_PARALLEL_LANES v_RATE v_STATE
  in
  self

/// Squeeze `N` x `LEN` bytes. Only `N = 1` for now.
let impl_1__squeeze
      (v_RATE: usize)
      (#v_STATE: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_sha3.Traits.t_KeccakItem v_STATE (mk_usize 1))
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_sha3.Traits.t_Squeeze
            (Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) v_STATE) v_STATE)
      (self: t_KeccakXofState (mk_usize 1) v_RATE v_STATE)
      (out: t_Slice u8)
    : Prims.Pure (t_KeccakXofState (mk_usize 1) v_RATE v_STATE & t_Slice u8)
      (requires Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self.f_buf_len)
      (ensures
        fun temp_0_ ->
          let
          (self_e_future: t_KeccakXofState (mk_usize 1) v_RATE v_STATE), (out_future: t_Slice u8) =
            temp_0_
          in
          Libcrux_sha3.Proof_utils.keccak_xof_state_inv v_RATE self_e_future.f_buf_len &&
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize)) =
  let out_len:usize = Core_models.Slice.impl__len #u8 out in
  if out_len =. mk_usize 0
  then self, out <: (t_KeccakXofState (mk_usize 1) v_RATE v_STATE & t_Slice u8)
  else
    let self:t_KeccakXofState (mk_usize 1) v_RATE v_STATE =
      if self.f_sponge
      then
        let self:t_KeccakXofState (mk_usize 1) v_RATE v_STATE =
          {
            self with
            f_inner
            =
            Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #v_STATE self.f_inner
          }
          <:
          t_KeccakXofState (mk_usize 1) v_RATE v_STATE
        in
        self
      else self
    in
    let (out: t_Slice u8), (self: t_KeccakXofState (mk_usize 1) v_RATE v_STATE) =
      if out_len >. mk_usize 0
      then
        let blocks:usize = out_len /! v_RATE in
        let last:usize = out_len -! (out_len %! v_RATE <: usize) in
        if blocks =. mk_usize 0
        then
          let out:t_Slice u8 =
            Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1)
                  v_STATE)
              #v_STATE
              #FStar.Tactics.Typeclasses.solve
              v_RATE
              self.f_inner
              out
              (mk_usize 0)
              out_len
          in
          out, self <: (t_Slice u8 & t_KeccakXofState (mk_usize 1) v_RATE v_STATE)
        else
          let out:t_Slice u8 =
            Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1)
                  v_STATE)
              #v_STATE
              #FStar.Tactics.Typeclasses.solve
              v_RATE
              self.f_inner
              out
              (mk_usize 0)
              v_RATE
          in
          let self_buf_len:usize = self.f_buf_len in
          let (out: t_Slice u8), (self: t_KeccakXofState (mk_usize 1) v_RATE v_STATE) =
            Rust_primitives.Hax.Folds.fold_range (mk_usize 1)
              blocks
              (fun temp_0_ temp_1_ ->
                  let (out: t_Slice u8), (self: t_KeccakXofState (mk_usize 1) v_RATE v_STATE) =
                    temp_0_
                  in
                  let _:usize = temp_1_ in
                  ((Core_models.Slice.impl__len #u8 out <: usize) =. out_len <: bool) &&
                  (self_buf_len =. self.f_buf_len <: bool))
              (out, self <: (t_Slice u8 & t_KeccakXofState (mk_usize 1) v_RATE v_STATE))
              (fun temp_0_ i ->
                  let (out: t_Slice u8), (self: t_KeccakXofState (mk_usize 1) v_RATE v_STATE) =
                    temp_0_
                  in
                  let i:usize = i in
                  let _:Prims.unit =
                    Hax_lib.v_assert (((Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int) *
                          (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
                          <:
                          Hax_lib.Int.t_Int) <=
                        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out
                              <:
                              usize)
                          <:
                          Hax_lib.Int.t_Int)
                        <:
                        bool)
                  in
                  let self:t_KeccakXofState (mk_usize 1) v_RATE v_STATE =
                    {
                      self with
                      f_inner
                      =
                      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1)
                        #v_STATE
                        self.f_inner
                    }
                    <:
                    t_KeccakXofState (mk_usize 1) v_RATE v_STATE
                  in
                  let out:t_Slice u8 =
                    Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState
                          (mk_usize 1) v_STATE)
                      #v_STATE
                      #FStar.Tactics.Typeclasses.solve
                      v_RATE
                      self.f_inner
                      out
                      (i *! v_RATE <: usize)
                      v_RATE
                  in
                  out, self <: (t_Slice u8 & t_KeccakXofState (mk_usize 1) v_RATE v_STATE))
          in
          if last <. out_len
          then
            let _:Prims.unit = Libcrux_sha3.Proof_utils.Lemmas.lemma_div_mul_mod out_len v_RATE in
            let self:t_KeccakXofState (mk_usize 1) v_RATE v_STATE =
              {
                self with
                f_inner
                =
                Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #v_STATE self.f_inner
              }
              <:
              t_KeccakXofState (mk_usize 1) v_RATE v_STATE
            in
            let out:t_Slice u8 =
              Libcrux_sha3.Traits.f_squeeze #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1)
                    v_STATE)
                #v_STATE
                #FStar.Tactics.Typeclasses.solve
                v_RATE
                self.f_inner
                out
                last
                (out_len -! last <: usize)
            in
            out, self <: (t_Slice u8 & t_KeccakXofState (mk_usize 1) v_RATE v_STATE)
          else out, self <: (t_Slice u8 & t_KeccakXofState (mk_usize 1) v_RATE v_STATE)
      else out, self <: (t_Slice u8 & t_KeccakXofState (mk_usize 1) v_RATE v_STATE)
    in
    let self:t_KeccakXofState (mk_usize 1) v_RATE v_STATE =
      { self with f_sponge = true } <: t_KeccakXofState (mk_usize 1) v_RATE v_STATE
    in
    self, out <: (t_KeccakXofState (mk_usize 1) v_RATE v_STATE & t_Slice u8)
