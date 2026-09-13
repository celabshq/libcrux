module Hacspec_sha3
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let createi
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
    : t_Array v_T v_N
    = Rust_primitives.Arrays.createi v_N f
