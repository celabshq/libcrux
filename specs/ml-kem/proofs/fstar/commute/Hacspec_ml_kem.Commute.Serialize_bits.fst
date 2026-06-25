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

(* ================================================================== *)
(* ENCODE-12: byte_encode side (inverse of decode).                    *)
(*   byte_encode 384 3072 p 12                                          *)
(*     = bits_to_bytes 384 3072 (bitvector_from_bounded_ints 256 3072   *)
(*         (createi 256 (fun i -> (p[i]).f_val)) 12)                    *)
(* Mirror of the decode chain but in the encode direction.             *)
(* ================================================================== *)

(* and-1 for u16 (u8 version above is lemma_val_and1) *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 100"
let lemma_val_and1_u16 (y: u16) : Lemma (v (y &. mk_u16 1) == get_bit y (sz 0))
  = let z = y &. mk_u16 1 in
    logand_lemma y (mk_u16 1);
    assert (v z < pow2 1);
    lemma_recon_nat (v z) 1;
    lemma_get_bit_nat_eq z (sz 0);
    lemma_get_bit_nat_eq y (sz 0)
#pop-options

(* E1: bitvector_from_bounded_ints index (direct createi, no fold) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bitvec_from_bounded_index
    (input: t_Array u16 (mk_usize 256)) (p: nat{p < 3072})
  : Lemma (Seq.index (S.bitvector_from_bounded_ints (mk_usize 256) (mk_usize 3072) input (mk_usize 12)) p
           == (get_bit_nat (v (Seq.index input (p / 12))) (p % 12) = 1))
  = let pp = mk_usize p in
    assert (p == v pp);
    assert (Seq.index (S.bitvector_from_bounded_ints (mk_usize 256) (mk_usize 3072) input (mk_usize 12)) (v pp)
            == ((((input.[ pp /! mk_usize 12 <: usize ] <: u16) >>! (pp %! mk_usize 12 <: usize) <: u16)
                 &. mk_u16 1 <: u16) =. mk_u16 1))
      by (FStar.Tactics.norm [delta_only [`%S.bitvector_from_bounded_ints]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    let idx = v (pp /! mk_usize 12 <: usize) in
    let sh  = pp %! mk_usize 12 in
    assert (idx == p / 12);
    assert (v sh == p % 12);
    let byte = Seq.index input idx in
    lemma_val_and1_u16 (byte >>! sh);     // v ((byte>>!sh)&.1) == get_bit (byte>>!sh) 0
    lemma_get_bit_nat_eq byte sh          // get_bit byte sh == get_bit_nat (v byte) (p%12)
    // get_bit_shr SMTPat: get_bit (byte>>!sh) 0 == get_bit byte (sz (0 + v sh))
#pop-options

(* cast (b:bool) <: u8 == (if b then mk_u8 1 else mk_u8 0); its bits *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_get_bit_cast_bool (b: bool) (j: usize{v j < 8})
  : Lemma (get_bit (Rust_primitives.cast #bool #u8 b) j == (if b && v j = 0 then 1 else 0))
          [SMTPat (get_bit (Rust_primitives.cast #bool #u8 b) j)]
  = let c : u8 = Rust_primitives.cast #bool #u8 b in
    assert (v c == (if b then 1 else 0));
    assert_norm (pow2 1 == 2);
    assert (bounded c 1);                 // v c < 2 = pow2 1
    if v j = 0 then begin
      lemma_get_bit_nat_eq c j;           // get_bit c 0 == get_bit_nat (v c) 0 == v c
      assert_norm (pow2 0 == 1)
    end
    // else: lemma_get_bit_bounded SMTPat fires (bounded c 1 /\ v j >= 1) ==> get_bit c j == 0
#pop-options

(* E2: bits_to_bytes bit — bit t of byte m equals bv[8m+t] *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bits_to_bytes_bit
    (bv: t_Array bool (mk_usize 3072)) (m: nat{m < 384}) (t: nat{t < 8})
  : Lemma (get_bit (Seq.index (S.bits_to_bytes (mk_usize 384) (mk_usize 3072) bv) m) (sz t)
           == (if Seq.index bv (8 * m + t) then 1 else 0))
  = let mm = mk_usize m in
    assert (m == v mm);
    assert (Seq.index (S.bits_to_bytes (mk_usize 384) (mk_usize 3072) bv) (v mm)
            == ((((((((Rust_primitives.cast #bool #u8 (bv.[ mk_usize 8 *! mm <: usize ] <: bool) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 1 <: usize ] <: bool) <: u8) <<! mk_i32 1 <: u8) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 2 <: usize ] <: bool) <: u8) <<! mk_i32 2 <: u8) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 3 <: usize ] <: bool) <: u8) <<! mk_i32 3 <: u8) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 4 <: usize ] <: bool) <: u8) <<! mk_i32 4 <: u8) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 5 <: usize ] <: bool) <: u8) <<! mk_i32 5 <: u8) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 6 <: usize ] <: bool) <: u8) <<! mk_i32 6 <: u8) <: u8) |.
                      ((Rust_primitives.cast #bool #u8 (bv.[ (mk_usize 8 *! mm <: usize) +! mk_usize 7 <: usize ] <: bool) <: u8) <<! mk_i32 7 <: u8) <: u8))
      by (FStar.Tactics.norm [delta_only [`%S.bits_to_bytes]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    // index arithmetic: 8*m + s for each s
    assert (v (mk_usize 8 *! mm <: usize) == 8 * m);
    assert (forall (s: nat). s < 8 ==> v ((mk_usize 8 *! mm <: usize) +! mk_usize s <: usize) == 8 * m + s)
#pop-options

(* byte_encode bit: bit t of output byte m == bit (p'%12) of (p[p'/12]).f_val, p'=8m+t *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_byte_encode_bit
    (p: t_Array P.t_FieldElement (mk_usize 256)) (m: nat{m < 384}) (t: nat{t < 8})
  : Lemma
    (get_bit (Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) m) (sz t)
     == get_bit_nat (v (Seq.index p ((8 * m + t) / 12)).Hacspec_ml_kem.Parameters.f_val) ((8 * m + t) % 12))
  = let praw : t_Array u16 (mk_usize 256) =
      P.createi #u16 (mk_usize 256) #(usize -> u16)
        (fun i -> let i:usize = i in (p.[ i ] <: P.t_FieldElement).Hacspec_ml_kem.Parameters.f_val) in
    let bv = S.bitvector_from_bounded_ints (mk_usize 256) (mk_usize 3072) praw (mk_usize 12) in
    assert (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)
            == S.bits_to_bytes (mk_usize 384) (mk_usize 3072) bv)
      by (FStar.Tactics.norm [delta_only [`%S.byte_encode]; zeta; iota; primops];
          FStar.Tactics.trefl ());
    let p' = 8 * m + t in
    lemma_bits_to_bytes_bit bv m t;            // get_bit byte m t == (if bv[8m+t] then 1 else 0)
    lemma_bitvec_from_bounded_index praw p';   // bv[p'] == (get_bit_nat (v praw[p'/12]) (p'%12) = 1)
    let kk = mk_usize (p' / 12) in
    assert (p' / 12 < 256);
    assert (Seq.index praw (v kk) == (p.[ kk ] <: P.t_FieldElement).Hacspec_ml_kem.Parameters.f_val)
      by (FStar.Tactics.l_to_r [`P.createi_lemma]; FStar.Tactics.trefl ())
#pop-options

(* per-byte bridge: output byte (24i+r) == byte_encode byte, via bit extensionality.
   value-match hypothesis (v coefficient[l] == (p[16i+l]).f_val) supplied by the composer
   (to_unsigned_representative post + poly_to_spec_index). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_serialize_byte_eq
    (serialized: t_Array u8 (mk_usize 384))
    (coefficient: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (i: nat{i < 16}) (r: nat{r < 24})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq coefficient 12 (Seq.slice serialized (24 * i) (24 * i + 24) <: t_Array u8 (mk_usize 24)) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index coefficient l) 12) /\
      (forall (l: nat). l < 16 ==> v (Seq.index coefficient l)
                                  == v (Seq.index p (16 * i + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures
      Seq.index serialized (24 * i + r)
      == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * i + r))
  = let m = 24 * i + r in
    let chunk : t_Array u8 (mk_usize 24) = Seq.slice serialized (24 * i) (24 * i + 24) in
    let be = S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12) in
    let aux (t: nat{t < 8}) : Lemma (get_bit (Seq.index serialized m) (sz t) == get_bit (Seq.index be m) (sz t)) =
      let pp = 8 * r + t in
      let l' = pp / 12 in
      assert (pp < 192);
      assert (l' < 16);
      // serialize side: get_bit serialized[m] t == get_bit coefficient[l'] (pp%12)
      assert (bit_vec_of_int_t_array coefficient 12 pp == get_bit (Seq.index coefficient l') (mk_usize (pp % 12)));
      assert (bit_vec_of_int_t_array coefficient 12 pp == bit_vec_of_int_t_array chunk 8 pp);
      BV.int_t_seq_slice_to_bv_sub_lemma serialized (24 * i) (mk_usize 24) 8;
      assert (bit_vec_of_int_t_array chunk 8 pp
              == BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((24 * i) * 8) (24 * 8) pp);
      assert (BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((24 * i) * 8) (24 * 8) pp
              == bit_vec_of_int_t_array serialized 8 (192 * i + pp));
      assert (192 * i + pp == 8 * m + t);
      assert (bit_vec_of_int_t_array serialized 8 (8 * m + t) == get_bit (Seq.index serialized m) (sz t));
      lemma_get_bit_nat_eq (Seq.index coefficient l') (mk_usize (pp % 12));
      // byte_encode side
      lemma_byte_encode_bit p m t;
      assert ((8 * m + t) / 12 == 16 * i + l');
      assert ((8 * m + t) % 12 == pp % 12)
    in
    FStar.Classical.forall_intro aux;
    introduce forall (j: usize {v j < 8}).
        get_bit (Seq.index serialized m) j == get_bit (Seq.index be m) j
    with aux (v j);
    lemma_int_t_eq_via_bits (Seq.index serialized m) (Seq.index be m)
#pop-options

(* per-chunk: all 24 bytes of chunk i agree with byte_encode *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_serialize_chunk_eq_byte_encode_12
    (serialized: t_Array u8 (mk_usize 384))
    (coefficient: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (i: nat{i < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq coefficient 12 (Seq.slice serialized (24 * i) (24 * i + 24) <: t_Array u8 (mk_usize 24)) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index coefficient l) 12) /\
      (forall (l: nat). l < 16 ==> v (Seq.index coefficient l)
                                  == v (Seq.index p (16 * i + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures
      (forall (r: nat). r < 24 ==>
        Seq.index serialized (24 * i + r)
        == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * i + r)))
  = let aux (r: nat) : Lemma (r < 24 ==>
        Seq.index serialized (24 * i + r)
        == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * i + r)) =
      if r < 24 then lemma_serialize_byte_eq serialized coefficient p i r
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ------------------------------------------------------------------ *)
(* opaque per-chunk atom (mirrors chunk_decoded_12) — keeps the         *)
(* bit-vector equality + value-match OUT of the composer's loop VC.     *)
(* ------------------------------------------------------------------ *)

[@@ "opaque_to_smt"]
let chunk_encoded_12 (serialized: t_Array u8 (mk_usize 384))
                     (g: t_Array i16 (mk_usize 16))
                     (p: t_Array P.t_FieldElement (mk_usize 256))
                     (j: nat) : prop =
  j < 16 /\
  BV.int_t_array_bitwise_eq g 12 (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 /\
  (forall (l: nat). l < 16 ==> bounded (Seq.index g l) 12) /\
  (forall (l: nat). l < 16 ==> v (Seq.index g l)
                              == v (Seq.index p (16 * j + l)).Hacspec_ml_kem.Parameters.f_val)

let lemma_chunk_encoded_intro
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g 12 (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index g l) 12) /\
      (forall (l: nat). l < 16 ==> v (Seq.index g l)
                                  == v (Seq.index p (16 * j + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures chunk_encoded_12 serialized g p j)
  = reveal_opaque (`%chunk_encoded_12) chunk_encoded_12

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_chunk_encoded_byte_encode
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat{j < 16})
  : Lemma
    (requires chunk_encoded_12 serialized g p j)
    (ensures
      (forall (r: nat). r < 24 ==>
        Seq.index serialized (24 * j + r)
        == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * j + r)))
  = reveal_opaque (`%chunk_encoded_12) chunk_encoded_12;
    lemma_serialize_chunk_eq_byte_encode_12 serialized g p j
#pop-options

(* mod_q_eq (v x) (v y) ==> i16_to_spec_fe x == i16_to_spec_fe y.
   Used by to_unsigned_field_modulus to expose the value relation the
   encode composers need (the wrapper's bounds-only post drops it). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_i16_to_spec_fe_mod_q_eq (x y: i16)
  : Lemma (requires Hacspec_ml_kem.ModQ.mod_q_eq (v x) (v y))
          (ensures VTS.i16_to_spec_fe x == VTS.i16_to_spec_fe y)
  = Hacspec_ml_kem.ModQ.lemma_mod_q_eq_unfold (v x) (v y);
    assert (v (VTS.i16_to_spec_fe x).Hacspec_ml_kem.Parameters.f_val == v x % 3329);
    assert (v (VTS.i16_to_spec_fe y).Hacspec_ml_kem.Parameters.f_val == v y % 3329)
#pop-options

(* ------------------------------------------------------------------ *)
(* byte-level opaque atom for the serialize_uncompressed composer.      *)
(* Keeps `byte_encode` (heavy transparent `let`) ENTIRELY out of the    *)
(* composer's loop context — the precondition check + createi unfolding *)
(* of byte_encode saturate the per-iteration VC otherwise.  Carries     *)
(* only `serialized` + `p` (poly_to_spec re, an opaque val), no g_j.     *)
(* ------------------------------------------------------------------ *)

[@@ "opaque_to_smt"]
let chunk_byte_enc (serialized: t_Array u8 (mk_usize 384))
                   (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat) : prop =
  j < 16 /\
  (forall (r: nat). r < 24 ==>
    Seq.index serialized (24 * j + r)
    == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * j + r))

(* intro from the bit-vector eq + value-match (delegates to the chunk bridge) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_chunk_byte_enc_intro
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g 12 (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index g l) 12) /\
      (forall (l: nat). l < 16 ==> v (Seq.index g l)
                                  == v (Seq.index p (16 * j + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures chunk_byte_enc serialized p j)
  = reveal_opaque (`%chunk_byte_enc) chunk_byte_enc;
    lemma_serialize_chunk_eq_byte_encode_12 serialized g p j
#pop-options

(* frame: chunk j's bytes lie within [0,bound), and s_new agrees with s_old on that prefix.
   bound is refined <=384 so the slice requires is well-formed; base=24*j is refined so the
   per-byte index bound 24*j+r<384 stays LINEAR (no nonlinear 24*j reasoning in the proof). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_frame
    (s_old s_new: t_Array u8 (mk_usize 384))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat{j < 16}) (bound: nat{bound <= 384})
  : Lemma
    (requires
      chunk_byte_enc s_old p j /\ 24 * j + 24 <= bound /\
      Seq.slice s_new 0 bound == Seq.slice s_old 0 bound)
    (ensures chunk_byte_enc s_new p j)
  = reveal_opaque (`%chunk_byte_enc) chunk_byte_enc;
    FStar.Math.Lemmas.lemma_mult_le_left 24 j 15;       // 24*j <= 360
    let base : (b: nat{b + 24 <= 384}) = 24 * j in
    let be = S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12) in
    introduce forall (r: nat). r < 24 ==> Seq.index s_new (base + r) == Seq.index be (base + r)
    with introduce _ ==> _
    with _. (Seq.lemma_index_slice s_new 0 bound (base + r);
             Seq.lemma_index_slice s_old 0 bound (base + r))
#pop-options

(* extend the per-chunk invariant from [0,i) to [0,i+1) after a sub-slice update.
   Done as a TOP-LEVEL lemma (clean context) so the opaque-atom forall e-matching
   does not saturate in the composer's heavy loop VC (skill: opaque carryover after
   update). The composer just supplies the old invariant + frame + the new chunk. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_extend
    (s_old s_new: t_Array u8 (mk_usize 384))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (i: nat{i < 16})
  : Lemma
    (requires
      (forall (j: nat). j < i ==> chunk_byte_enc s_old p j) /\
      Seq.slice s_new 0 (24 * i) == Seq.slice s_old 0 (24 * i) /\
      chunk_byte_enc s_new p i)
    (ensures (forall (j: nat). j < i + 1 ==> chunk_byte_enc s_new p j))
  = FStar.Math.Lemmas.lemma_mult_le_left 24 i 16;          // 24*i <= 384
    let aux (j: nat{j < i + 1}) : Lemma (chunk_byte_enc s_new p j) =
      if j < i then begin
        FStar.Math.Lemmas.lemma_mult_le_left 24 (j + 1) i;  // 24*(j+1) <= 24*i, i.e. 24*j+24 <= 24*i
        lemma_chunk_byte_enc_frame s_old s_new p j (24 * i)
      end
    in
    FStar.Classical.forall_intro aux
#pop-options

(* per-lane value match, TOP-LEVEL + clean context (single l, NO forall hypothesis):
   seals the `poly_to_spec_index` createi cascade away from the consumer's forall
   (feedback_standalone_lane_cong_createi_cascade). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_enc_value_match_one
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16}) (l: nat{l < 16})
  : Lemma
    (requires
      v (Seq.index g l) >= 0 /\ v (Seq.index g l) < 3329 /\
      VTS.i16_to_spec_fe (Seq.index g l)
      == VTS.i16_to_spec_fe (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
            (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) l))
    (ensures
      bounded (Seq.index g l) 12 /\
      v (Seq.index g l)
      == v (Seq.index (Libcrux_ml_kem.Vector.Spec.poly_to_spec re) (16 * j + l)).Hacspec_ml_kem.Parameters.f_val)
  = FStar.Math.Lemmas.lemma_div_plus l j 16;
    FStar.Math.Lemmas.lemma_mod_plus l j 16;
    Libcrux_ml_kem.Vector.Spec.poly_to_spec_index re (16 * j + l)
#pop-options

(* intro_re: prove chunk_byte_enc from `re` + the to_unsigned-style per-lane FE relation.
   poly_to_spec_index is sealed inside lemma_enc_value_match_one (clean context), so its
   createi cascade never meets this lemma's forall hypothesis. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_intro_re
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (serialized: t_Array u8 (mk_usize 384))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    (g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g 12 (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 /\
      (forall (l: nat). l < 16 ==> v (Seq.index g l) >= 0 /\ v (Seq.index g l) < 3329) /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == VTS.i16_to_spec_fe (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
              (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) l)))
    (ensures chunk_byte_enc serialized (Libcrux_ml_kem.Vector.Spec.poly_to_spec re) j)
  = let p = Libcrux_ml_kem.Vector.Spec.poly_to_spec re in
    let aux (l: nat{l < 16}) : Lemma
      (bounded (Seq.index g l) 12 /\
       v (Seq.index g l) == v (Seq.index p (16 * j + l)).Hacspec_ml_kem.Parameters.f_val) =
      lemma_enc_value_match_one re g j l
    in
    FStar.Classical.forall_intro aux;
    lemma_chunk_byte_enc_intro serialized g p j
#pop-options

(* unfold: the per-byte equality (for the composer finalize) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_unfold
    (serialized: t_Array u8 (mk_usize 384))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat{j < 16})
  : Lemma
    (requires chunk_byte_enc serialized p j)
    (ensures
      (forall (r: nat). r < 24 ==>
        Seq.index serialized (24 * j + r)
        == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * j + r)))
  = reveal_opaque (`%chunk_byte_enc) chunk_byte_enc
#pop-options

(* ================================================================== *)
(* DECODE-12-REDUCED (Track B): deserialize_12 then cond_subtract_3329.*)
(* The reduced composer stores g = cond_subtract_3329 (deserialize_12  *)
(* bytes): mod-q congruence (the trait post's mod_q_eq) preserves the  *)
(* i16_to_spec_fe image, so the chunk still decodes to byte_decode;    *)
(* the exact conditional conjunct bounds the output lanes by 3328.     *)
(* Mirrors chunk_decompressed_d (Serialize_compress): the atom carries *)
(* the bound + the functional whole-array eq.                          *)
(* ================================================================== *)

[@@ "opaque_to_smt"]
let chunk_decoded_12_red (serialized: t_Array u8 (mk_usize 384))
                         (g: t_Array i16 (mk_usize 16)) (j: nat) : prop =
  j < 16 /\
  VTS.is_i16b_array_opaque 3328 g /\
  (forall (l: nat). l < 16 ==>
    VTS.i16_to_spec_fe (Seq.index g l)
    == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * j + l))

(* intro from deserialize_12's byte-bridge facts (over the RAW decoded
   chunk g0) + the cond_subtract_3329 trait post relating g to g0. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_chunk_decoded_red_intro
    (serialized: t_Array u8 (mk_usize 384)) (g0 g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 g0 12 /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index g0 ll) 12) /\
      VTS.cond_subtract_3329_post g0 g)
    (ensures chunk_decoded_12_red serialized g j)
  = reveal_opaque (`%chunk_decoded_12_red) chunk_decoded_12_red;
    reveal_opaque (`%VTS.is_i16b_array_opaque) (VTS.is_i16b_array_opaque 3328 g);
    lemma_deserialize_chunk_eq_byte_decode_12 serialized g0 j;
    assert_norm (mk_usize 384 == mk_usize 32 *! mk_usize 12);
    assert_norm (mk_usize 3072 == mk_usize 256 *! mk_usize 12);
    let bd = S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12) in
    let aux (l: nat{l < 16}) : Lemma
      (VTS.i16_to_spec_fe (Seq.index g l) == Seq.index bd (16 * j + l) /\
       VTS.is_i16b 3328 (Seq.index g l)) =
      lemma_i16_to_spec_fe_mod_q_eq (Seq.index g l) (Seq.index g0 l)
    in
    FStar.Classical.forall_intro aux
#pop-options

(* unfold: the per-lane spec equality (for the composer finalize) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_decoded_red_byte_decode
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16})
  : Lemma
    (requires chunk_decoded_12_red serialized g j)
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * j + l)))
  = reveal_opaque (`%chunk_decoded_12_red) chunk_decoded_12_red
#pop-options

(* the per-chunk bound conjunct *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_decoded_red_bound
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16})
  : Lemma
    (requires chunk_decoded_12_red serialized g j)
    (ensures VTS.is_i16b_array_opaque 3328 g)
  = reveal_opaque (`%chunk_decoded_12_red) chunk_decoded_12_red
#pop-options

(* is_bounded_poly 3328 re from the per-chunk bound conjuncts
   (mirror of Serialize_compress.lemma_is_bounded_poly_of_chunks) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_is_bounded_poly_of_red_chunks
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (serialized: t_Array u8 (mk_usize 384))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (j: nat). j < 16 ==>
        chunk_decoded_12_red serialized
          (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
            (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) j))
    (ensures Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328) re)
  = reveal_opaque (`%Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly)
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) re);
    let aux (i: nat{i < 16}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector (mk_usize 3328)
        (re.Libcrux_ml_kem.Vector.f_coefficients.[ sz i ])) =
      lemma_chunk_decoded_red_bound serialized
        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
          (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients i)) i
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ------------------------------------------------------------------ *)
(* B2 (deserialize_ring_elements_reduced): standalone loop-step lemma. *)
(* Extends the per-row invariant (bound + byte_decode eq over the      *)
(* row's 384-byte chunk) from i to i+1 after the row-i update.  Inline *)
(* maintenance in the fold body saturates the function VC (canceled at *)
(* full rlimit 400) — clean-context standalone lemma per the campaign  *)
(* composer recipe.                                                    *)
(* ------------------------------------------------------------------ *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_row_decoded_maintain
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (v_K: usize)
    (public_key: t_Slice u8)
    (pk_old pk_new: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
    (chunk: t_Slice u8)
    (i: usize)
  : Lemma
    (requires
      v i < v v_K /\
      Seq.length public_key == v v_K * 384 /\
      Seq.length chunk == 384 /\
      chunk == Seq.slice public_key (v i * 384) (v i * 384 + 384) /\
      (forall (j: nat). j < v i ==>
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328) (Seq.index pk_old j) /\
        Libcrux_ml_kem.Vector.Spec.poly_to_spec (Seq.index pk_old j) ==
          S.byte_decode (mk_usize 384) (mk_usize 3072)
            (Seq.slice public_key (j * 384) (j * 384 + 384)) (mk_usize 12)) /\
      (forall (k: nat). k < v v_K /\ k <> v i ==> Seq.index pk_new k == Seq.index pk_old k) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328) (Seq.index pk_new (v i)) /\
      Libcrux_ml_kem.Vector.Spec.poly_to_spec (Seq.index pk_new (v i)) ==
        S.byte_decode (mk_usize 384) (mk_usize 3072) chunk (mk_usize 12))
    (ensures
      (forall (j: nat). j < v i + 1 ==>
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328) (Seq.index pk_new j) /\
        Libcrux_ml_kem.Vector.Spec.poly_to_spec (Seq.index pk_new j) ==
          S.byte_decode (mk_usize 384) (mk_usize 3072)
            (Seq.slice public_key (j * 384) (j * 384 + 384)) (mk_usize 12)))
  = let aux (j: nat{j < v i + 1}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328) (Seq.index pk_new j) /\
       Libcrux_ml_kem.Vector.Spec.poly_to_spec (Seq.index pk_new j) ==
         S.byte_decode (mk_usize 384) (mk_usize 3072)
           (Seq.slice public_key (j * 384) (j * 384 + 384)) (mk_usize 12)) =
      if j < v i
      then assert (Seq.index pk_new j == Seq.index pk_old j)
      else assert (j == v i)
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ================================================================== *)
(* UNCOMPRESSED-12 bound: the honest [0,4095] lane bound carried by    *)
(* `chunk_decoded_12` (`bounded (g l) 12`) lifts to is_i16b 4096        *)
(* ([0,4095] subset [-4096,4096]).  Mirrors lemma_chunk_decoded_red_*   *)
(* but for the NON-reduced atom (no cond_subtract_3329).  Consumed by   *)
(* deserialize_to_uncompressed_ring_element to expose a 4096 bound on   *)
(* the deserialized (unreduced) ByteDecode_12 output.                   *)
(* ================================================================== *)

(* the per-chunk bound conjunct: bounded 12 (lanes in [0,4095]) =>
   is_i16b_array_opaque 4096 *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_decoded_bound
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16})
  : Lemma
    (requires chunk_decoded_12 serialized g j)
    (ensures VTS.is_i16b_array_opaque 4096 g)
  = reveal_opaque (`%chunk_decoded_12) chunk_decoded_12;
    reveal_opaque (`%VTS.is_i16b_array_opaque) (VTS.is_i16b_array_opaque 4096 g);
    assert_norm (pow2 12 == 4096);
    let aux (l: nat{l < 16}) : Lemma (VTS.is_i16b 4096 (Seq.index g l)) =
      assert (bounded (Seq.index g l) 12)
    in
    FStar.Classical.forall_intro aux
#pop-options

(* is_bounded_poly 4096 re from the per-chunk uncompressed bound conjuncts
   (mirror of lemma_is_bounded_poly_of_red_chunks at the unreduced 4096 bound) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_is_bounded_poly_of_chunks_12
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (serialized: t_Array u8 (mk_usize 384))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (j: nat). j < 16 ==>
        chunk_decoded_12 serialized
          (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
            (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) j))
    (ensures Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 4096) re)
  = reveal_opaque (`%Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly)
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 4096) re);
    let aux (i: nat{i < 16}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector (mk_usize 4096)
        (re.Libcrux_ml_kem.Vector.f_coefficients.[ sz i ])) =
      lemma_chunk_decoded_bound serialized
        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
          (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients i)) i
    in
    FStar.Classical.forall_intro aux
#pop-options
