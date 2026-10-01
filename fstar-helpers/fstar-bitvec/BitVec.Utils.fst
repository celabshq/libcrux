module BitVec.Utils

open Core_models
open FStar.FunctionalExtensionality
open BitVec.Equality
open Rust_primitives.BitVectors

let mk_bv #len (f: (i:nat{i < len}) -> bit) = on (i:nat {i < len}) f

let mk_list_32 #a (x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13 x14 x15 x16 x17 x18 x19 x20 x21 x22 x23 x24 x25 x26 x27 x28 x29 x30 x31: a)
  : (l:list a {List.Tot.length l == 32})
  = let l = [x0;x1;x2;x3;x4;x5;x6;x7;x8;x9;x10;x11;x12;x13;x14;x15;x16;x17;x18;x19;x20;x21;x22;x23;x24;x25;x26;x27;x28;x29;x30;x31] in
    assert_norm (List.Tot.length l == 32);
    l

let mk_list_16 #a (x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13 x14 x15: a)
  : (l:list a {List.Tot.length l == 16})
  = let l = [x0;x1;x2;x3;x4;x5;x6;x7;x8;x9;x10;x11;x12;x13;x14;x15] in
    assert_norm (List.Tot.length l == 16);
    l

let mk_list_8 #a (x0 x1 x2 x3 x4 x5 x6 x7: a)
  : (l:list a {List.Tot.length l == 8})
  = let l = [x0;x1;x2;x3;x4;x5;x6;x7] in
    assert_norm (List.Tot.length l == 8);
    l

let rw_get_bit_cast #t #u
  (x: int_t t) (nth: usize)
  : Lemma (requires v nth < bits u /\ v nth < bits u)
          (ensures eq2 #bit (get_bit (cast_mod #t #u x) nth) (if v nth < bits t then get_bit x nth else 0))
          [SMTPat (get_bit (cast_mod #t #u x) nth)]
  = ()

let rw_get_bit_shr #t #u (x: int_t t) (y: int_t u) (i: usize {v i < bits t})
  : Lemma (requires v y >= 0 /\ v y < bits t)
          (ensures eq2 #bit (get_bit (x >>! y) i )
                (if v i < bits t - v y
                    then get_bit x (mk_int (v i + v y))
                    else if signed t
                         then get_bit x (mk_int (bits t - 1))
                         else 0))
  = ()

unfold type forall_sig (n: nat) = pred: ((i:nat{i < n}) -> bool)
                              -> r: bool {r <==> (forall i. pred i)}

let forall8: forall_sig 8 = fun pred -> pred 0 && pred 1 && pred 2 && pred 3
                                  && pred 4 && pred 5 && pred 6 && pred 7

#push-options "--z3rlimit 400"
let forall16: forall_sig 16 = fun pred -> forall8 pred && forall8 (fun i -> pred (i + 8))
let forall32: forall_sig 32 = fun pred -> forall16 pred && forall16 (fun i -> pred (i + 16))
let forall64: forall_sig 64 = fun pred -> forall32 pred && forall32 (fun i -> pred (i + 32))
let forall128: forall_sig 128 = fun pred -> forall64 pred && forall64 (fun i -> pred (i + 64))
let forall256: forall_sig 256 = fun pred -> forall128 pred && forall128 (fun i -> pred (i + 128))
#pop-options

let forall_n (n:nat{n <= 256}): forall_sig n = fun pred -> forall256 (fun i -> if i < n then pred i else true)

let bit_vec_to_int_t_lemma
    #t (d: lane_bits t) (bv: bit_vec d)
    i
  : Lemma (get_bit (bit_vec_to_int_t d bv) (sz i) == bv i)
          [SMTPat (get_bit (bit_vec_to_int_t d bv) (sz i))]
  = bit_vec_to_int_t_lemma d bv i

