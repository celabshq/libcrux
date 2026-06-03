module Libcrux_traits.Digest.Arrayref
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// Error indicating that hashing failed.
type t_HashError =
  | HashError_InvalidPayloadLength : t_HashError
  | HashError_Unknown : t_HashError

let t_HashError_cast_to_repr (x: t_HashError) : isize =
  match x <: t_HashError with
  | HashError_InvalidPayloadLength  -> mk_isize 0
  | HashError_Unknown  -> mk_isize 1

/// A trait for oneshot hashing, where the output is written into a provided buffer.
class t_Hash (v_Self: Type0) (v_OUTPUT_LEN: usize) = {
  f_hash_pre:t_Array u8 v_OUTPUT_LEN -> t_Slice u8 -> Type0;
  f_hash_post:
      t_Array u8 v_OUTPUT_LEN ->
      t_Slice u8 ->
      (t_Array u8 v_OUTPUT_LEN & Core_models.Result.t_Result Prims.unit t_HashError)
    -> Type0;
  f_hash:x0: t_Array u8 v_OUTPUT_LEN -> x1: t_Slice u8
    -> Prims.Pure (t_Array u8 v_OUTPUT_LEN & Core_models.Result.t_Result Prims.unit t_HashError)
        (f_hash_pre x0 x1)
        (fun result -> f_hash_post x0 x1 result)
}
