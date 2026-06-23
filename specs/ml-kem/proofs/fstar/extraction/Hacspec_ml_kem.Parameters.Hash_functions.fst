module Hacspec_ml_kem.Parameters.Hash_functions
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

assume
val v_G': input: t_Slice u8 -> t_Array u8 (mk_usize 64)

unfold
let v_G = v_G'

let v_H_DIGEST_SIZE: usize = mk_usize 32

assume
val v_H': input: t_Slice u8 -> t_Array u8 (mk_usize 32)

unfold
let v_H = v_H'

assume
val v_PRF': v_LEN: usize -> input: t_Slice u8 -> t_Array u8 v_LEN

unfold
let v_PRF (v_LEN: usize) = v_PRF' v_LEN

assume
val v_XOF': v_LEN: usize -> input: t_Slice u8 -> t_Array u8 v_LEN

unfold
let v_XOF (v_LEN: usize) = v_XOF' v_LEN

assume
val v_J': v_LEN: usize -> input: t_Slice u8 -> t_Array u8 v_LEN

unfold
let v_J (v_LEN: usize) = v_J' v_LEN
