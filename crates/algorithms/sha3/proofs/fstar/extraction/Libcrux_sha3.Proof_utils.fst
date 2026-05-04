module Libcrux_sha3.Proof_utils
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// Checks if all slices in an array have the same length.
let slices_same_len (v_N: usize) (slices: t_Array (t_Slice u8) v_N) : Hax_lib.Prop.t_Prop =
  forall (i: usize).
    b2t (i <. v_N <: bool) ==>
    b2t
    ((Core_models.Slice.impl__len #u8 (slices.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
      (Core_models.Slice.impl__len #u8 (slices.[ i ] <: t_Slice u8) <: usize)
      <:
      bool)

let valid_rate (rate: usize) : bool =
  rate >. mk_usize 32 && rate <. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
  ((rate %! mk_usize 32 <: usize) =. mk_usize 8 || (rate %! mk_usize 32 <: usize) =. mk_usize 16)

/// XOF state invariant: validates that buffer length and rate are valid.
let keccak_xof_state_inv (rate buf_len: usize) : bool = valid_rate rate && buf_len <=. rate
