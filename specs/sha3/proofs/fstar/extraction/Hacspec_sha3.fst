module Hacspec_sha3
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

[@@ "opaque_to_smt"]
let createi
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
    : t_Array v_T v_N
    = Rust_primitives.Arrays.createi v_N f

let createi_lemma
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
      (i: usize{i <. v_N})
     : Lemma (Seq.index (createi #v_T v_N #v_F f) (v i) == f i)
       [SMTPat (Seq.index (createi #v_T v_N #v_F f) (v i))]
     = reveal_opaque (`%createi) (createi #v_T v_N #v_F f)
