module BitVecEq
#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"
open Core_models
open FStar.Mul
open MkSeq
open FStar.FunctionalExtensionality

val bit_vec_equal (#n: nat) (bv1 bv2: bit_vec n): Type0
val bit_vec_equal_intro (#n: nat) (bv1 bv2: bit_vec n)
  : Lemma (requires forall i. bv1 i == bv2 i)
          (ensures bit_vec_equal bv1 bv2)
val bit_vec_equal_elim (#n: nat) (bv1 bv2: bit_vec n)
  : Lemma (requires bit_vec_equal #n bv1 bv2)
          (ensures bv1 == bv2)
          [SMTPat (bit_vec_equal #n bv1 bv2)]

let bit_vec_equal_intro_principle ()
  : Lemma (forall n (bv1 bv2: bit_vec n). (forall i. bv1 i == bv2 i) ==> bit_vec_equal #n bv1 bv2)
  = introduce forall n (bv1 bv2: bit_vec n). _
    with introduce (forall i. bv1 i == bv2 i) ==> bit_vec_equal #n bv1 bv2
         with _. bit_vec_equal_intro #n bv1 bv2
  
let bit_vec_equal_elim_principle ()
  : Lemma (forall n (bv1 bv2: bit_vec n). bit_vec_equal #n bv1 bv2 ==> (forall i. bv1 i == bv2 i))
  = introduce forall n (bv1 bv2: bit_vec n). _
    with introduce bit_vec_equal #n bv1 bv2 ==> (forall i. bv1 i == bv2 i)
         with _. bit_vec_equal_elim #n bv1 bv2

let bit_vec_equal_trivial (bv1 bv2: bit_vec 0): Lemma (bv1 == bv2) 
    [SMTPat (eq2 #(bit_vec 0) bv1 bv2)]
  = bit_vec_equal_intro bv1 bv2

let bit_vec_sub #n (bv: bit_vec n) (start: nat) (len: nat {start + len <= n})
  : bit_vec len
  = on (i: nat {i < len})
       (fun i -> bv (start + i))

let bit_vec_equal_trivial_sub_smtpat (bv1: bit_vec 'n)
  : Lemma (forall (bv2: bit_vec 0). bit_vec_sub bv1 0 0 == bv2)
    [SMTPat (bit_vec_sub bv1 0 0)]
  = introduce forall (bv2: bit_vec 0). bit_vec_sub bv1 0 0 == bv2
    with bit_vec_equal_trivial (bit_vec_sub bv1 0 0) bv2

unfold let retype #a #b (#_:unit{a == b})
  (x: a): b
  = x

let bit_vec_sub_all_lemma #n (bv: bit_vec n)
  : Lemma (bit_vec_sub bv 0 n == bv)
          [SMTPat (bit_vec_sub bv 0 n)]
  = bit_vec_equal_intro (bit_vec_sub bv 0 n) bv

let int_t_array_bitwise_eq'
       #t1 #t2 #n1 #n2
       (arr1: t_Array (int_t t1) n1) (d1: lane_bits t1)
       (arr2: t_Array (int_t t2) n2) (d2: lane_bits t2 {v n1 * d1 == v n2 * d2})
     = bit_vec_equal (bit_vec_of_int_t_array arr1 d1)
                     (retype (bit_vec_of_int_t_array arr2 d2))

let int_t_array_bitwise_eq
       #t1 #t2 #n1 #n2
       (arr1: t_Array (int_t t1) n1) (d1: lane_bits t1)
       (arr2: t_Array (int_t t2) n2) (d2: lane_bits t2 {v n1 * d1 == v n2 * d2})
     = bit_vec_of_int_t_array arr1 d1 == bit_vec_of_int_t_array arr2 d2

// let get_bit_intro ()
//   : Lemma (forall (#n: inttype) (x: int_t n) (nth: usize {v nth < bits n}). 
//              get_bit #n x nth == (  if v x >= 0 then get_bit_nat (v x) (v nth)
//                                 else get_bit_nat (pow2 (bits n) + v x) (v nth)))
//   = introduce forall (n: inttype) (x: int_t n) (nth: usize {v nth < bits n}). 
//              get_bit #n x nth == (  if v x >= 0 then get_bit_nat (v x) (v nth)
//                                 else get_bit_nat (pow2 (bits n) + v x) (v nth))
//     with get_bit_intro #n x nth

#push-options "--fuel 0 --ifuel 0 --z3rlimit 150"
/// Rewrite a `bit_vec_of_int_t_array (Seq.slice arr ...)` into a `bit_vec_sub ...`
let int_t_seq_slice_to_bv_sub_lemma #t #n 
  (arr: t_Array (int_t t) n)
  (start: nat) (len: usize {start + v len <= v n})
  (d: lane_bits t) 
  : Lemma ( bit_vec_of_int_t_array (Seq.slice arr start (start + v len) <: t_Array _ len) d
     `bit_vec_equal` bit_vec_sub (bit_vec_of_int_t_array arr d) (start * d) (v len * d))
   [SMTPat (bit_vec_sub (bit_vec_of_int_t_array arr d) (start * d) (v len * d))]
  = let bv1 = bit_vec_of_int_t_array #_ #len (Seq.slice arr start (start + v len)) d in
    let bv2 = bit_vec_sub (bit_vec_of_int_t_array arr d) (start * d) (v len * d) in
    introduce forall i. bv1 i == bv2 i 
    with ( Seq.lemma_index_slice arr start (start + v len) (i / d);
           Math.Lemmas.lemma_div_plus i start d;
           Math.Lemmas.lemma_mod_plus i start d);
    bit_vec_equal_intro bv1 bv2

#push-options "--split_queries always"
let int_t_eq_seq_slice_bv_sub_lemma #t #n1 #n2
  (arr1: t_Array (int_t t) n1) (arr2: t_Array (int_t t) n2)  (d: lane_bits t)
  (start1 start2: nat) (len: nat {start1 + len <= v n1 /\ start2 + len <= v n2})
  : Lemma (requires Seq.slice arr1 start1 (start1 + len) == Seq.slice arr2 start2 (start2 + len))
          (ensures  bit_vec_equal
                       (bit_vec_sub (bit_vec_of_int_t_array arr1 d) (start1 * d) (len * d))
                       (bit_vec_sub (bit_vec_of_int_t_array arr2 d) (start2 * d) (len * d)))
          [SMTPat ((bit_vec_sub (bit_vec_of_int_t_array arr1 d) (start1 * d) (len * d)) ==
                       (bit_vec_sub (bit_vec_of_int_t_array arr2 d) (start2 * d) (len * d)))]
  = let len = sz len in
    int_t_seq_slice_to_bv_sub_lemma arr1 start1 len d;
    int_t_seq_slice_to_bv_sub_lemma arr2 start2 len d;
    // bit_vec_equal_elim_principle ();
    bit_vec_equal_intro_principle ()
#pop-options

let bit_vec_equal_extend #n1 #n2
  (bv1: bit_vec n1) (bv2: bit_vec n2) (start1 start2: nat)
  (len1: nat)
  (len2: nat { start1 + len1 + len2 <= n1 /\ start2 + len1 + len2 <= n2})
  : Lemma 
    (requires 
       bit_vec_sub bv1 start1 len1 == bit_vec_sub bv2 start2 len1
     /\ bit_vec_sub bv1 (start1 + len1) len2 == bit_vec_sub bv2 (start2 + len1) len2)
    (ensures bit_vec_sub bv1 start1 (len1+len2) == bit_vec_sub bv2 start2 (len1+len2))
    // [SMTPat (bit_vec_sub bv1 start1 len1 == bit_vec_sub bv2 start2 len1);
    //  SMTPat ()
    // ]
     // SMTPat (bit_vec_sub bv1 (start1 + len1) len2 == bit_vec_sub bv2 (start2 + len1) len2)]
  = let left1 = bit_vec_sub bv1 start1 len1 in
    let left2 = bit_vec_sub bv2 start2 len1 in
    let right1 = bit_vec_sub bv1 (start1 + len1) len2 in
    let right2 = bit_vec_sub bv2 (start2 + len1) len2 in
    // ()
    // bit_vec_equal_elim left1  left2 ;
    // bit_vec_equal_elim right1 right2;
    let entire1 = bit_vec_sub bv1 start1 (len1 + len2) in
    let entire2 = bit_vec_sub bv2 start2 (len1 + len2) in
    assert (forall (i:nat). i < len1 ==> left1 i == left2 i);
    assert (forall (i:nat). i < len2 ==> right1 i == right2 i);
    introduce forall (i:nat). i < len1 + len2 ==> entire1 i == entire2 i
    with introduce i < len1 + len2 ==> entire1 i == entire2 i
         with _. if i < len1 then assert (left1 i == left2 i)
                             else assert (entire1 i == right1 (i - len1));
    bit_vec_equal_intro entire1 entire2
#pop-options

// let bit_vec_equal_trans (#n: nat) (bv1 bv2 bv3: bit_vec n)
//   : Lemma (requires bv1 `bit_vec_equal` bv2 /\ bv2 `bit_vec_equal` bv3)
//           (ensures  bv1 `bit_vec_equal` bv3)
//   = bit_vec_equal_elim_principle ();
//     bit_vec_equal_intro_principle ()

(*
let int_arr_bitwise_eq_range
       #t1 #t2 #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement2: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement2 x}) n2)
       (d2: lane_bits t2)
       (offset1 offset2: nat)
       (bits: nat {
           offset1 + bits <= v n1 * d1
         /\ offset2 + bits <= v n2 * d2
       })
     = bit_vec_equal #bits (fun i -> bit_vec_of_int_t_array arr1 d1 (i + offset1))
     = forall (k: nat). k < bits ==>
          bit_vec_of_int_t_array arr1 d1 (offset1 + k) 
       == bit_vec_of_int_t_array arr2 d2 (offset2 + k)

let int_arr_bitwise_eq_range_comm
       #t1 #t2 #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement2: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement2 x}) n2)
       (d2: lane_bits t2)
       (offset1 offset2: nat)
       (bits: nat {
           offset1 + bits <= v n1 * d1
         /\ offset2 + bits <= v n2 * d2
       })
    : Lemma (requires int_arr_bitwise_eq_range arr1 d1 arr2 d2 offset1 offset2 bits)
            (ensures int_arr_bitwise_eq_range arr2 d2 arr1 d1 offset2 offset1 bits)
    = ()

// kill that function in favor of range
let int_arr_bitwise_eq_up_to
       #t1 #t2 #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement x}) n2)
       (d2: lane_bits t2 {v n1 * d1 == v n2 * d2})
       (max: nat {max <= v n1 * d1})
     
     = forall i. i < max
       ==> bit_vec_of_int_t_array arr1 d1 i == bit_vec_of_int_t_array arr2 d2 i

let int_arr_bitwise_eq_
       #t1 #t2 #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement x}) n2)
       (d2: lane_bits t2 {v n1 * d1 == v n2 * d2})
     = int_arr_bitwise_eq_up_to arr1 d1 arr2 d2 (v n1 * d1)

// move to fsti
let bit_vec_equal #n (bv1 bv2: bit_vec n)
  = forall i. i < n ==> bv1 i == bv2 i

let int_arr_bitwise_eq
       #t1 #t2 #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement x}) n2)
       (d2: lane_bits t2 {v n1 * d1 == v n2 * d2})
     = forall i. i < v n1 * d1
       ==> bit_vec_of_int_t_array arr1 d1 i == bit_vec_of_int_t_array arr2 d2 i

let int_arr_bitwise_eq_range_transitivity
       #t1 #t2 #t3 #n1 #n2 #n3
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement2: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement2 x}) n2)
       (d2: lane_bits t2)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement3: int_t t3 -> Type0)
       (arr3: t_Array (x: int_t t3 {refinement3 x}) n3)
       (d3: lane_bits t3)
       (offset1 offset2 offset3: nat)
       (bits: nat {
           offset1 + bits <= v n1 * d1
         /\ offset2 + bits <= v n2 * d2
         /\ offset3 + bits <= v n3 * d3
       })
   : Lemma 
     (requires int_arr_bitwise_eq_range #t1 #t2 #n1 #n2 arr1 d1 arr2 d2 offset1 offset2 bits
             /\ int_arr_bitwise_eq_range #t2 #t3 #n2 #n3 arr2 d2 arr3 d3 offset2 offset3 bits)
     (ensures  int_arr_bitwise_eq_range #t1 #t3 #n1 #n3 arr1 d1 arr3 d3 offset1 offset3 bits)
   = ()


let int_arr_bitwise_eq_range_intro
       #t1 #t2 #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t1 -> Type0)
       (arr1: t_Array (x: int_t t1 {refinement1 x}) n1)
       (d1: lane_bits t1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement: int_t t2 -> Type0)
       (arr2: t_Array (x: int_t t2 {refinement x}) n2)
       (d2: lane_bits t2 {v n1 * d1 == v n2 * d2})
   : Lemma 
     (requires int_arr_bitwise_eq arr1 d1 arr2 d2)
     (ensures int_arr_bitwise_eq_range arr1 d1 arr2 d2 0 0 (v n1 * d1))
   = admit ()

let int_arr_bitwise_eq_range_intro_eq_slice
       #t #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement: int_t t -> Type0)
       (arr1: t_Array (x: int_t t {refinement x}) n1)
       (arr2: t_Array (x: int_t t {refinement x}) n2)
       (d: lane_bits t)
       (offset1 offset2: nat)
       (n: nat {offset1 + n < v n1 /\ offset2 + n < v n2})
       (bits: nat {
           offset1 + bits <= v n1 * d
         /\ offset2 + bits <= v n2 * d
         /\ bits <= n * d
       })
   : Lemma (requires Seq.slice arr1 offset1 (offset1 + n) == Seq.slice arr2 offset2 (offset2 + n))
           (ensures int_arr_bitwise_eq_range arr1 d arr2 d offset1 offset2 bits)
 = admit ()
 
let int_arr_bitwise_eq_range_intro_eq
       #t #n1 #n2
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement1: int_t t -> Type0)
       (arr1: t_Array (x: int_t t {refinement1 x}) n1)
       (#[FStar.Tactics.exact (`(fun _ -> True))]refinement2: int_t t -> Type0)
       (arr2: t_Array (x: int_t t {refinement2 x}) n2)
       (d: lane_bits t)
       (n_offset1 n_offset2: nat)
       (n: nat {n_offset1 + n <= v n1 /\ n_offset2 + n <= v n2})
       // (offset1 offset2: nat)
       (bits: nat {
           n_offset1 * d + bits <= v n1 * d
         /\ n_offset2 * d + bits <= v n2 * d
         /\ bits <= n * d
       })
   : Lemma (requires forall (i: nat). i < n ==> Seq.index arr1 (i + n_offset1) == Seq.index arr2 (i + n_offset2))
           (ensures int_arr_bitwise_eq_range arr1 d arr2 d (n_offset1 * d) (n_offset2 * d) bits)
 = admit ()
*)
