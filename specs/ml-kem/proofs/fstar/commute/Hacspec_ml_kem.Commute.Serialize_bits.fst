module Hacspec_ml_kem.Commute.Serialize_bits
/// Foundational bit-vector reconciliation for the Serialize composers.
/// Connects the impl core-op posts (BitVecEq.int_t_array_bitwise_eq, i.e.
/// bit_vec_of_int_t_array equality) to the Hacspec spec bit-functions
/// (bytes_to_bits / bitvector_to_bounded_ints).  All generic in bit-width d.
#set-options "--fuel 1 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Rust_primitives.BitVectors

module ML = FStar.Math.Lemmas
module S  = Hacspec_ml_kem.Serialize
module P  = Hacspec_ml_kem.Parameters
module F  = Rust_primitives.Hax.Folds

(* ------------------------------------------------------------------ *)
(* bitsum: value reconstructed from the low d bits given by predicate f *)
(* ------------------------------------------------------------------ *)

let rec bitsum (f: nat -> bool) (d: nat) : Tot nat (decreases d) =
  if d = 0 then 0
  else bitsum f (d - 1) + (if f (d - 1) then pow2 (d - 1) else 0)

let rec bitsum_cong (f g: nat -> bool) (d: nat)
  : Lemma (requires forall (j: nat). j < d ==> f j == g j)
          (ensures bitsum f d == bitsum g d)
          (decreases d)
  = if d = 0 then ()
    else bitsum_cong f g (d - 1)

let rec lemma_bitsum_bound (f: nat -> bool) (d: nat)
  : Lemma (ensures bitsum f d < pow2 d) (decreases d)
  = if d = 0 then ()
    else begin
      lemma_bitsum_bound f (d - 1);
      ML.pow2_plus 1 (d - 1)   // pow2 d == 2 * pow2 (d-1)
    end

(* value of `1us <<! s` for s < 16 *)
let lemma_shl1_u16 (s: nat{s < 16})
  : Lemma (v (mk_u16 1 <<! mk_usize s) == pow2 s)
  = ML.pow2_lt_compat 16 s;
    ML.small_mod (pow2 s) (pow2 16)

(* get_bit_nat x j = (x / pow2 j) % 2 *)

(* For j < m, the j-th bit of (x % 2^m) equals the j-th bit of x. *)
let lemma_get_bit_nat_mod (x: nat) (m: nat) (j: nat)
  : Lemma (requires j < m)
          (ensures get_bit_nat (x % pow2 m) j == get_bit_nat x j)
  = ML.pow2_modulo_division_lemma_1 x j m;
    // (x % pow2 m) / pow2 j == (x / pow2 j) % pow2 (m - j)
    ML.pow2_modulo_modulo_lemma_1 (x / pow2 j) 1 (m - j)
    // ((x/pow2 j) % pow2 (m-j)) % pow2 1 == (x/pow2 j) % pow2 1   (since 1 <= m-j)

(* purely-algebraic combine: no get_bit_nat / div-mod, so Z3 closes it fast.
   bitsum f d = bitsum f (d-1) + (if f(d-1) then pow2(d-1) else 0)
             = lo + (if hi=1 then pow2(d-1) else 0) = lo + hi*pow2(d-1) = x. *)
let lemma_recon_combine (x lo hi: nat) (d: nat) (f: nat -> bool)
  : Lemma (requires
       d > 0 /\ hi < 2 /\
       x == hi * pow2 (d - 1) + lo /\
       lo == bitsum f (d - 1) /\
       (f (d - 1) <==> hi = 1))
     (ensures x == bitsum f d)
  = ()

(* heavy step, non-recursive so --split_queries always is safe (no termination VC) *)
#push-options "--z3rlimit 200 --split_queries always"
let lemma_recon_step (x: nat) (d: nat)
  : Lemma (requires
       d > 0 /\ x < pow2 d /\
       (x % pow2 (d - 1)) == bitsum (fun j -> get_bit_nat (x % pow2 (d - 1)) j = 1) (d - 1))
     (ensures x == bitsum (fun j -> get_bit_nat x j = 1) d)
  = let m = d - 1 in
    let f_x  = (fun (j: nat) -> get_bit_nat x j = 1) in
    ML.pow2_plus 1 m;                      // pow2 d == 2 * pow2 m
    ML.lemma_div_mod x (pow2 m);           // x == (x / pow2 m) * pow2 m + x % pow2 m
    let lo = x % pow2 m in
    let hi = x / pow2 m in
    assert (hi < 2);
    assert (get_bit_nat x m == hi);
    let f_lo = (fun (j: nat) -> get_bit_nat lo j = 1) in
    let aux (j: nat) : Lemma (j < m ==> f_x j == f_lo j) =
      if j < m then lemma_get_bit_nat_mod x m j
    in
    FStar.Classical.forall_intro aux;
    bitsum_cong f_lo f_x m;                // lo == bitsum f_x m
    lemma_recon_combine x lo hi d f_x
#pop-options

#push-options "--z3rlimit 50"
let rec lemma_recon_nat (x: nat) (d: nat)
  : Lemma (requires x < pow2 d)
          (ensures x == bitsum (fun j -> get_bit_nat x j = 1) d)
          (decreases d)
  = if d = 0 then ()
    else (lemma_recon_nat (x % pow2 (d - 1)) (d - 1);
          lemma_recon_step x d)
#pop-options

(* ------------------------------------------------------------------ *)
(* spec-fold: bitvector_to_bounded_ints[k] value == bitsum of its bits  *)
(* (concrete d=12; fuel unfolds the extracted fold_range + bitsum)       *)
(* ------------------------------------------------------------------ *)

(* peel-FIRST one-step unfold of fold_range (definitional); local copy *)
let lemma_fold_range_step
      (#acc_t: Type0)
      (start end_: usize)
      (inv: acc_t -> (i:usize{F.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (init: acc_t {~(F.range_empty start end_) ==> inv init start})
      (f: (acc:acc_t -> i:usize {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i) /\ inv acc i}
                     -> acc':acc_t {(inv acc' (mk_int (v i + 1)))}))
  : Lemma (requires v start < v end_)
      (ensures F.fold_range start end_ inv init f ==
               F.fold_range (start +! mk_usize 1) end_ inv (f init start) f)
  = ()

(* named inv/step for bitvector_to_bounded_ints' inner fold at d=12,
   byte-copied from the spec so the createi body delta/beta-reduces to
   `fold_range 0 12 dec_inv (mk_u16 0) (dec_step input i)`. *)
let dec_inv : u16 -> (j:usize{F.fold_range_wf_index (mk_usize 0) (mk_usize 12) false (v j)}) -> Type0 =
  (fun coefficient j ->
      let coefficient:u16 = coefficient in
      let j:usize = j in
      coefficient <. (mk_u16 1 <<! j <: u16) <: bool)

#push-options "--z3rlimit 300"
let dec_step (input: t_Array bool (mk_usize 3072)) (i: usize{i <. mk_usize 256})
  : (acc:u16 -> j:usize {v j <= 12 /\ F.fold_range_wf_index (mk_usize 0) (mk_usize 12) true (v j) /\ dec_inv acc j}
              -> acc':u16 {dec_inv acc' (mk_int (v j + 1))}) =
  (fun coefficient j ->
      let coefficient:u16 = coefficient in
      let j:usize = j in
      if input.[ (i *! mk_usize 12 <: usize) +! j <: usize ] <: bool
      then
        let coefficient:u16 = coefficient +! (mk_u16 1 <<! j <: u16) in
        coefficient
      else coefficient)
#pop-options

(* the fold value, by upward recursion on the start (peel + IH) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let rec lemma_dec_aux
    (input: t_Array bool (mk_usize 3072)) (i: usize{i <. mk_usize 256})
    (start: nat{start <= 12}) (acc: u16)
  : Lemma
    (requires dec_inv acc (mk_usize start) /\
              v acc == bitsum (fun j -> j < start && Seq.index input (v i * 12 + j)) start)
    (ensures
      v (F.fold_range (mk_usize start) (mk_usize 12) dec_inv acc (dec_step input i))
      == bitsum (fun j -> j < 12 && Seq.index input (v i * 12 + j)) 12)
    (decreases (12 - start))
  = if start = 12 then begin
      bitsum_cong (fun j -> j < start && Seq.index input (v i * 12 + j))
                  (fun j -> j < 12 && Seq.index input (v i * 12 + j)) 12
    end
    else begin
      let f = dec_step input i in
      let ss = mk_usize start in
      lemma_fold_range_step #u16 ss (mk_usize 12) dec_inv acc f;
      let acc' = f acc ss in
      lemma_shl1_u16 start;                       // v (mk_u16 1 <<! ss) == pow2 start
      assert (v acc < pow2 start);                // from dec_inv acc ss
      ML.pow2_le_compat 11 start;                 // pow2 start <= pow2 11  (start <= 11)
      assert_norm (pow2 11 == 2048);
      assert_norm (pow2 16 == 65536);
      // index value & extracted bit
      assert (v ((i *! mk_usize 12 <: usize) +! ss <: usize) == v i * 12 + start);
      let bit : bool = Seq.index input (v i * 12 + start) in
      // dec_step unfold (transparent let): acc' is the if-expression
      assert (acc' == (if bit then acc +! (mk_u16 1 <<! ss) else acc));
      // no-overflow: v acc + pow2 start < pow2 12 <= pow2 16
      assert (v acc' == v acc + (if bit then pow2 start else 0));
      // bitsum unfolds one step at (start+1); the predicates agree below start
      let g = (fun (j: nat) -> j < start + 1 && Seq.index input (v i * 12 + j)) in
      let h = (fun (j: nat) -> j < start && Seq.index input (v i * 12 + j)) in
      bitsum_cong h g start;                      // bitsum h start == bitsum g start
      assert (g start == bit);
      assert (bitsum g (start + 1) == bitsum g start + (if bit then pow2 start else 0));
      assert (v acc' == bitsum g (start + 1));
      lemma_dec_aux input i (start + 1) acc'
    end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bitvec_to_bounded_index_12
  (input: t_Array bool (mk_usize 3072)) (k: nat{k < 256})
  : Lemma (ensures
      v (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize 3072) input (mk_usize 12)) k)
      == bitsum (fun j -> j < 12 && Seq.index input (k * 12 + j)) 12)
  = let kk = mk_usize k in
    assert (k == v kk);
    assert (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize 3072) input (mk_usize 12)) (v kk)
            == F.fold_range (mk_usize 0) (mk_usize 12) dec_inv (mk_u16 0) (dec_step input kk))
      by (FStar.Tactics.norm [delta_only [`%S.bitvector_to_bounded_ints]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    lemma_shl1_u16 0;
    lemma_dec_aux input kk 0 (mk_u16 0);
    // lemma_dec_aux gives the result over predicate (v kk*12+j); rephrase to (k*12+j)
    bitsum_cong (fun j -> j < 12 && Seq.index input (v kk * 12 + j))
                (fun j -> j < 12 && Seq.index input (k * 12 + j)) 12
#pop-options

(* ------------------------------------------------------------------ *)
(* bytes_to_bits <-> get_bit bridge                                     *)
(* ------------------------------------------------------------------ *)

(* nat-level get_bit equals machine get_bit for non-negative ints *)
let lemma_get_bit_nat_eq (#t: inttype) (x: int_t t {v x >= 0}) (j: usize {v j < bits t})
  : Lemma (get_bit x j == get_bit_nat (v x) (v j))
  = reveal_opaque (`%get_bit) (get_bit #t)

(* value of `y &. 1` is bit 0 of y.  Light proof: y &. 1 < 2, so reconstruct at d=1. *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 100"
let lemma_val_and1 (y: u8) : Lemma (v (y &. mk_u8 1) == get_bit y (sz 0))
  = let z = y &. mk_u8 1 in
    logand_lemma y (mk_u8 1);          // v z <= v (mk_u8 1) == 1, and v z >= 0
    assert (v z < pow2 1);
    lemma_recon_nat (v z) 1;           // v z == (if get_bit_nat (v z) 0 = 1 then 1 else 0)
    lemma_get_bit_nat_eq z (sz 0);     // get_bit_nat (v z) 0 == get_bit z (sz 0)
    lemma_get_bit_nat_eq y (sz 0)
    // get_bit z (sz 0) == get_bit y (sz 0) bit_and get_bit (mk_u8 1) (sz 0) == get_bit y (sz 0)
#pop-options

(* bytes_to_bits index, related to get_bit_nat of the source byte *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bytes_to_bits_index (b: t_Array u8 (mk_usize 384)) (m: nat {m < 3072})
  : Lemma (Seq.index (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) b) m
           == (get_bit_nat (v (Seq.index b (m / 8))) (m % 8) = 1))
  = let mm = mk_usize m in
    assert (m == v mm);
    // createi reduction of bytes_to_bits
    assert (Seq.index (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) b) (v mm)
            == (((Seq.index b (v (mm /! mk_usize 8)) >>! (mm %! mk_usize 8)) &. mk_u8 1) = mk_u8 1))
      by (FStar.Tactics.norm [delta_only [`%S.bytes_to_bits]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    let byte = Seq.index b (m / 8) in
    let sh   = mm %! mk_usize 8 in
    // value of the masked shifted byte == get_bit (byte >>! sh) 0 == get_bit byte sh
    lemma_val_and1 (byte >>! sh);
    lemma_get_bit_nat_eq byte sh
    // get_bit (byte >>! sh) (sz 0) == get_bit byte (sz (0 + v sh))  [SMTPat get_bit_shr]
#pop-options

(* ------------------------------------------------------------------ *)
(* per-coefficient bit equality: impl group bit == spec bytes bit      *)
(* ------------------------------------------------------------------ *)
module BV = BitVecEq

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_coeff_bit
    (serialized: t_Array u8 (mk_usize 384))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16}) (l: nat {l < 16}) (j: nat {j < 12})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * i) (24 * i + 24) <: t_Array u8 (mk_usize 24)) 8 group 12 /\
      v (Seq.index group l) >= 0)
    (ensures
      (get_bit_nat (v (Seq.index group l)) j = 1)
      == Seq.index (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) serialized) ((16 * i + l) * 12 + j))
  = let chunk : t_Array u8 (mk_usize 24) = Seq.slice serialized (24 * i) (24 * i + 24) in
    let p = 12 * l + j in
    let m = 192 * i + p in
    assert (p < 192);
    assert (m == (16 * i + l) * 12 + j);
    // (1) get_bit_nat (v group[l]) j == bit_vec_of_int_t_array group 12 p
    lemma_get_bit_nat_eq (Seq.index group l) (mk_usize j);
    assert (p / 12 == l /\ p % 12 == j);
    assert (bit_vec_of_int_t_array group 12 p == get_bit (Seq.index group l) (mk_usize j));
    // (2) int_t_array_bitwise_eq: group 12 == chunk 8 (pointwise at p)
    assert (bit_vec_of_int_t_array group 12 p == bit_vec_of_int_t_array chunk 8 p);
    // (3) slice -> sub of serialized: chunk 8 p == serialized 8 (192 i + p)
    BV.int_t_seq_slice_to_bv_sub_lemma serialized (24 * i) (mk_usize 24) 8;
    assert (bit_vec_of_int_t_array chunk 8 p
            == BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((24 * i) * 8) (24 * 8) p);
    assert (BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((24 * i) * 8) (24 * 8) p
            == bit_vec_of_int_t_array serialized 8 m);
    // (4) bit_vec_of_int_t_array serialized 8 m == get_bit_nat (v serialized[m/8]) (m%8)
    lemma_get_bit_nat_eq (Seq.index serialized (m / 8)) (mk_usize (m % 8));
    assert (bit_vec_of_int_t_array serialized 8 m
            == get_bit (Seq.index serialized (m / 8)) (mk_usize (m % 8)));
    // (5) bytes_to_bits index
    lemma_bytes_to_bits_index serialized m
#pop-options

(* ------------------------------------------------------------------ *)
(* per-coefficient + per-chunk: impl group == byte_decode (12-bit)     *)
(* ------------------------------------------------------------------ *)
module VTS = Libcrux_ml_kem.Vector.Traits.Spec

(* value equality: impl coeff value == spec decoded value (pure bit-reasoning) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_coeff_value
    (serialized: t_Array u8 (mk_usize 384))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16}) (l: nat {l < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * i) (24 * i + 24) <: t_Array u8 (mk_usize 24)) 8 group 12 /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index group ll) 12))
    (ensures
      v (Seq.index group l)
      == v (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize 3072)
              (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) serialized) (mk_usize 12)) (16 * i + l)))
  = let k = 16 * i + l in
    let bv = S.bytes_to_bits (mk_usize 384) (mk_usize 3072) serialized in
    lemma_recon_nat (v (Seq.index group l)) 12;
    let aux (j: nat) : Lemma (j < 12 ==>
        (get_bit_nat (v (Seq.index group l)) j = 1) == (j < 12 && Seq.index bv (k * 12 + j))) =
      if j < 12 then lemma_coeff_bit serialized group i l j
    in
    FStar.Classical.forall_intro aux;
    bitsum_cong (fun j -> get_bit_nat (v (Seq.index group l)) j = 1)
                (fun j -> j < 12 && Seq.index bv (k * 12 + j)) 12;
    lemma_bitvec_to_bounded_index_12 bv k
#pop-options

(* byte_decode[k] == FE.new (decoded[k] % q)  (createi tactic, no bit-reasoning) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_byte_decode_index_12 (serialized: t_Array u8 (mk_usize 384)) (k: nat {k < 256})
  : Lemma
    (Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) k
     == P.impl_FieldElement__new
          (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize 3072)
             (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) serialized) (mk_usize 12)) k
           %! P.v_FIELD_MODULUS))
  = let kk = mk_usize k in
    assert (k == v kk);
    assert (S.byte_decode_generic (mk_usize 32) (mk_usize 256) (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)
            == S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize 3072)
                 (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) serialized) (mk_usize 12))
      by (FStar.Tactics.norm [delta_only [`%S.byte_decode_generic]; zeta; iota; primops];
          FStar.Tactics.trefl ());
    assert (Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (v kk)
            == P.impl_FieldElement__new
                 (Seq.index (S.byte_decode_generic (mk_usize 32) (mk_usize 256) (mk_usize 384)
                               (mk_usize 3072) serialized (mk_usize 12)) (v kk)
                  %! P.v_FIELD_MODULUS))
      by (FStar.Tactics.norm [delta_only [`%S.byte_decode]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ())
#pop-options

(* combine: light FE record equality from the two pieces *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_deserialize_coeff_eq_byte_decode_12
    (serialized: t_Array u8 (mk_usize 384))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16}) (l: nat {l < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * i) (24 * i + 24) <: t_Array u8 (mk_usize 24)) 8 group 12 /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index group ll) 12))
    (ensures
      VTS.i16_to_spec_fe (Seq.index group l)
      == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * i + l))
  = let k = 16 * i + l in
    let decoded = S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize 3072)
                    (S.bytes_to_bits (mk_usize 384) (mk_usize 3072) serialized) (mk_usize 12) in
    lemma_coeff_value serialized group i l;
    lemma_byte_decode_index_12 serialized k;
    let r1 = VTS.i16_to_spec_fe (Seq.index group l) in
    let r2 = Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) k in
    assert (v r1.Hacspec_ml_kem.Parameters.f_val == v (Seq.index group l) % 3329);
    assert (v r2.Hacspec_ml_kem.Parameters.f_val == v (Seq.index decoded k) % 3329);
    assert (r1.Hacspec_ml_kem.Parameters.f_val == r2.Hacspec_ml_kem.Parameters.f_val)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_deserialize_chunk_eq_byte_decode_12
    (serialized: t_Array u8 (mk_usize 384))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * i) (24 * i + 24) <: t_Array u8 (mk_usize 24)) 8 group 12 /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index group ll) 12))
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index group l)
        == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * i + l)))
  = let aux (l: nat) : Lemma (l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index group l)
        == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * i + l)) =
      if l < 16 then lemma_deserialize_coeff_eq_byte_decode_12 serialized group i l
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ------------------------------------------------------------------ *)
(* opaque per-chunk atom: keeps the bit-vector equality out of the      *)
(* composer's loop-body VC (mirrors Matrix_bridge.row_done).            *)
(* ------------------------------------------------------------------ *)

[@@ "opaque_to_smt"]
let chunk_decoded_12 (serialized: t_Array u8 (mk_usize 384))
                     (g: t_Array i16 (mk_usize 16)) (j: nat) : prop =
  j < 16 /\
  BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 g 12 /\
  (forall (l: nat). l < 16 ==> bounded (Seq.index g l) 12)

let lemma_chunk_decoded_intro
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 g 12 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index g l) 12))
    (ensures chunk_decoded_12 serialized g j)
  = reveal_opaque (`%chunk_decoded_12) chunk_decoded_12

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_decoded_byte_decode
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat {j < 16})
  : Lemma
    (requires chunk_decoded_12 serialized g j)
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * j + l)))
  = reveal_opaque (`%chunk_decoded_12) chunk_decoded_12;
    lemma_deserialize_chunk_eq_byte_decode_12 serialized g j
#pop-options
