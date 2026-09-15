module Hacspec_ml_kem.Commute.Serialize_bits
/// Abstract-interface firewall over the foundational bit-vector reconciliation
/// module (the Serialize composers' bit theory).  Exposes only the surface
/// consumed by Libcrux_ml_kem.Serialize (the 14 decode/encode atoms + lemmas)
/// plus the foundational bit helpers consumed by Serialize_compress (bitsum +
/// 7 lemmas via the SB alias) and Sampling_cbd (lemma_get_bit_nat_eq).  The
/// heavy per-coefficient/per-byte bit proofs (lemma_dec_aux, lemma_coeff_*,
/// lemma_serialize_byte_eq, the createi-tactic index lemmas, ...) stay private
/// so they never contaminate a consumer's SMT context.
///   * `bitsum` is TRANSPARENT: Serialize_compress unfolds its one-step
///     recursion (bitsum g (s+1) == bitsum g s + ...) directly.
///   * The 3 opaque per-chunk atoms (chunk_decoded_12 / chunk_byte_enc /
///     chunk_decoded_12_red) are ABSTRACT vals — consumers use them only via
///     the intro / byte_decode / unfold lemmas, never reveal_opaque them
///     (verified: no `reveal_opaque (`%<pred>)` in Serialize.fst).
///   * ZERO exposed SMTPats (the module's one SMTPat is on the private helper
///     lemma_get_bit_cast_bool, not in the surface).
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Rust_primitives.BitVectors

module ML  = FStar.Math.Lemmas
module S   = Hacspec_ml_kem.Serialize
module P   = Hacspec_ml_kem.Parameters
module F   = Rust_primitives.Hax.Folds
module BV  = BitVecEq
module VTS = Libcrux_ml_kem.Vector.Traits.Spec

(* transparent: Serialize_compress unfolds the one-step recursion *)
let rec bitsum (f: nat -> bool) (d: nat) : Tot nat (decreases d) =
  if d = 0 then 0
  else bitsum f (d - 1) + (if f (d - 1) then pow2 (d - 1) else 0)

val bitsum_cong (f g: nat -> bool) (d: nat)
  : Lemma (requires forall (j: nat). j < d ==> f j == g j)
          (ensures bitsum f d == bitsum g d)

val lemma_shl1_u16 (s: nat{s < 16})
  : Lemma (v (mk_u16 1 <<! mk_usize s) == pow2 s)

val lemma_recon_nat (x: nat) (d: nat)
  : Lemma (requires x < pow2 d)
          (ensures x == bitsum (fun j -> get_bit_nat x j = 1) d)

val lemma_fold_range_step
      (#acc_t: Type0)
      (start end_: usize)
      (inv: acc_t -> (i:usize{F.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (init: acc_t {~(F.range_empty start end_) ==> inv init start})
      (f: (acc:acc_t -> i:usize {v i <= v end_ /\ F.fold_range_wf_index start end_ true (v i) /\ inv acc i}
                     -> acc':acc_t {(inv acc' (mk_int (v i + 1)))}))
  : Lemma (requires v start < v end_)
      (ensures F.fold_range start end_ inv init f ==
               F.fold_range (start +! mk_usize 1) end_ inv (f init start) f)

val lemma_get_bit_nat_eq (#t: inttype) (x: int_t t {v x >= 0}) (j: usize {v j < bits t})
  : Lemma (get_bit x j == get_bit_nat (v x) (v j))

val lemma_val_and1 (y: u8) : Lemma (v (y &. mk_u8 1) == get_bit y (sz 0))

val chunk_decoded_12 (serialized: t_Array u8 (mk_usize 384))
                     (g: t_Array i16 (mk_usize 16)) (j: nat) : prop

val lemma_chunk_decoded_intro
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 g 12 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index g l) 12))
    (ensures chunk_decoded_12 serialized g j)

val lemma_chunk_decoded_byte_decode
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat {j < 16})
  : Lemma
    (requires chunk_decoded_12 serialized g j)
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * j + l)))

val lemma_val_and1_u16 (y: u16) : Lemma (v (y &. mk_u16 1) == get_bit y (sz 0))

(* NO SMTPat here (mandate: 0 exposed SMTPats).  The .fst `let` keeps its
   [SMTPat (get_bit (cast #bool #u8 b) j)] so it still auto-fires WITHIN
   Serialize_bits.fst; the cross-module consumer (Serialize_compress's
   lemma_bits_to_bytes_bit_d) re-injects the fact via an explicit
   FStar.Classical.forall_intro_2 SB.lemma_get_bit_cast_bool. *)
val lemma_get_bit_cast_bool (b: bool) (j: usize{v j < 8})
  : Lemma (get_bit (Rust_primitives.cast #bool #u8 b) j == (if b && v j = 0 then 1 else 0))

val lemma_i16_to_spec_fe_mod_q_eq (x y: i16)
  : Lemma (requires Hacspec_ml_kem.ModQ.mod_q_eq (v x) (v y))
          (ensures VTS.i16_to_spec_fe x == VTS.i16_to_spec_fe y)

val chunk_byte_enc (serialized: t_Array u8 (mk_usize 384))
                   (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat) : prop

val lemma_chunk_byte_enc_extend
    (s_old s_new: t_Array u8 (mk_usize 384))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (i: nat{i < 16})
  : Lemma
    (requires
      (forall (j: nat). j < i ==> chunk_byte_enc s_old p j) /\
      Seq.slice s_new 0 (24 * i) == Seq.slice s_old 0 (24 * i) /\
      chunk_byte_enc s_new p i)
    (ensures (forall (j: nat). j < i + 1 ==> chunk_byte_enc s_new p j))

val lemma_chunk_byte_enc_intro_re
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

val lemma_chunk_byte_enc_unfold
    (serialized: t_Array u8 (mk_usize 384))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat{j < 16})
  : Lemma
    (requires chunk_byte_enc serialized p j)
    (ensures
      (forall (r: nat). r < 24 ==>
        Seq.index serialized (24 * j + r)
        == Seq.index (S.byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12)) (24 * j + r)))

val chunk_decoded_12_red (serialized: t_Array u8 (mk_usize 384))
                         (g: t_Array i16 (mk_usize 16)) (j: nat) : prop

val lemma_chunk_decoded_red_intro
    (serialized: t_Array u8 (mk_usize 384)) (g0 g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (24 * j) (24 * j + 24) <: t_Array u8 (mk_usize 24)) 8 g0 12 /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index g0 ll) 12) /\
      VTS.cond_subtract_3329_post g0 g)
    (ensures chunk_decoded_12_red serialized g j)

val lemma_chunk_decoded_red_byte_decode
    (serialized: t_Array u8 (mk_usize 384)) (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16})
  : Lemma
    (requires chunk_decoded_12_red serialized g j)
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == Seq.index (S.byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)) (16 * j + l)))

val lemma_is_bounded_poly_of_red_chunks
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

val lemma_row_decoded_maintain
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

val lemma_is_bounded_poly_of_chunks_12
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
