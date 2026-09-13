module Hacspec_ml_kem.Commute.Serialize_compress
/// Abstract-interface firewall over the generic-in-d compress/decompress byte
/// bridge (the Serialize composers' compress theory).  Exposes only the 13
/// surface decls consumed by Libcrux_ml_kem.Serialize (12) + Sampling_cbd
/// (lemma_bytes_to_bits_index_d).  The heavy per-coefficient/per-byte proofs
/// (lemma_dec_aux_d, lemma_coeff_*_d, lemma_serialize_byte_eq_d, the createi
/// tactic index lemmas, the compress/decompress lane-post machinery, ...) stay
/// private so they never contaminate a consumer's SMT context.
///   * The 2 opaque per-chunk atoms (chunk_byte_enc_d, chunk_decompressed_d)
///     are ABSTRACT vals — consumers use them only via the intro / unfold /
///     finalize lemmas, never reveal_opaque them (verified: no
///     `reveal_opaque (`%<pred>)` in Serialize.fst).
///   * ZERO exposed SMTPats (Serialize_compress.fst has none).
///   * Uses Serialize_bits ONLY in its .fst bodies (via the SB alias) — no
///     surface signature references it, so this interface does not depend on
///     Serialize_bits.
#set-options "--fuel 1 --ifuel 1 --z3rlimit 200"
open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Rust_primitives.BitVectors

module S   = Hacspec_ml_kem.Serialize
module P   = Hacspec_ml_kem.Parameters
module BV  = BitVecEq
module VTS = Libcrux_ml_kem.Vector.Traits.Spec
module C   = Hacspec_ml_kem.Compress
module VS  = Libcrux_ml_kem.Vector.Spec
module VT  = Libcrux_ml_kem.Vector.Traits

val chunk_byte_enc_d (d: usize{v d > 0 /\ v d <= 12})
                     (out_len: usize{v out_len == 32 * v d})
                     (serialized: t_Array u8 out_len)
                     (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat) : prop

val lemma_chunk_byte_enc_extend_d
    (d: usize{v d > 0 /\ v d <= 12})
    (out_len: usize{v out_len == 32 * v d})
    (s_old s_new: t_Array u8 out_len)
    (p: t_Array P.t_FieldElement (mk_usize 256)) (i: nat{i < 16})
  : Lemma
    (requires
      (forall (j: nat). j < i ==> chunk_byte_enc_d d out_len s_old p j) /\
      Seq.slice s_new 0 (2 * v d * i) == Seq.slice s_old 0 (2 * v d * i) /\
      chunk_byte_enc_d d out_len s_new p i)
    (ensures (forall (j: nat). j < i + 1 ==> chunk_byte_enc_d d out_len s_new p j))

val lemma_chunk_byte_enc_unfold_d
    (d: usize{v d > 0 /\ v d <= 12})
    (out_len: usize{v out_len == 32 * v d})
    (serialized: t_Array u8 out_len)
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat{j < 16})
  : Lemma
    (requires chunk_byte_enc_d d out_len serialized p j)
    (ensures
      (forall (r: nat). r < 2 * v d ==>
        Seq.index serialized (2 * v d * j + r)
        == Seq.index (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d) (2 * v d * j + r)))

val lemma_chunk_byte_enc_intro_compress_post
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (cb: i32{v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11})
    (out_len: usize{v out_len == 32 * v cb})
    (serialized: t_Array u8 out_len)
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    (inp g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g (v cb) (Seq.slice serialized (2 * v cb * j) (2 * v cb * j + 2 * v cb) <: t_Array u8 (mk_usize (2 * v cb))) 8 /\
      VTS.compress_post inp cb g /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index inp l)
        == VTS.i16_to_spec_fe (Seq.index (VT.f_repr
              (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) l)))
    (ensures chunk_byte_enc_d (mk_usize (v cb)) out_len serialized
               (C.compress (VS.poly_to_spec re) (mk_usize (v cb))) j)

val lemma_byte_decode_dyn_eq (serialized: t_Slice u8) (d: usize{v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11})
  : Lemma (requires Seq.length serialized == 32 * v d)
          (ensures S.byte_decode_dyn serialized d
                   == S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d)

val lemma_bytes_to_bits_index_d (d: usize{v d > 0 /\ v d <= 12})
    (b: t_Array u8 (mk_usize (32 * v d))) (m: nat {m < 256 * v d})
  : Lemma (Seq.index (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) b) m
           == (get_bit_nat (v (Seq.index b (m / 8))) (m % 8) = 1))

val chunk_decompressed_d (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (g: t_Array i16 (mk_usize 16)) (j: nat) : prop

val lemma_chunk_decompressed_intro_post_d
    (d: usize{v d > 0 /\ v d < 12})
    (cb: i32{(v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\ v cb == v d})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (grp g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (2 * v d * j) (2 * v d * j + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 grp (v d) /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index grp ll) (v d)) /\
      VTS.decompress_ciphertext_coefficient_post grp cb g)
    (ensures chunk_decompressed_d d serialized g j)

val lemma_poly_to_spec_eq_decompress
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (j: nat). j < 16 ==>
        chunk_decompressed_d d serialized
          (VT.f_to_i16_array (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) j))
    (ensures
      VS.poly_to_spec re
      == C.decompress (S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d) d)

val lemma_is_bounded_poly_of_chunks
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (j: nat). j < 16 ==>
        chunk_decompressed_d d serialized
          (VT.f_to_i16_array (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) j))
    (ensures Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328) re)

val lemma_chunk_byte_enc_intro_compress_1_post
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (d1: usize{v d1 == 1 /\ d1 == mk_usize 1})
    (out_len: usize{v out_len == 32 * v d1})
    (serialized: t_Array u8 out_len)
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    (inp g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g (v d1) (Seq.slice serialized (2 * v d1 * j) (2 * v d1 * j + 2 * v d1) <: t_Array u8 (mk_usize (2 * v d1))) 8 /\
      VTS.compress_1_post inp g /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index inp l)
        == VTS.i16_to_spec_fe (Seq.index (VT.f_repr
              (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) l)))
    (ensures chunk_byte_enc_d d1 out_len serialized
               (C.compress (VS.poly_to_spec re) d1) j)

val lemma_chunk_decompressed_intro_1_post
    (d1: usize{v d1 == 1 /\ d1 == mk_usize 1})
    (serialized: t_Array u8 (mk_usize (32 * v d1)))
    (grp g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (2 * v d1 * j) (2 * v d1 * j + 2 * v d1) <: t_Array u8 (mk_usize (2 * v d1))) 8 grp (v d1) /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index grp ll) (v d1)) /\
      VTS.decompress_1_post grp g)
    (ensures chunk_decompressed_d d1 serialized g j)

val lemma_vector_to_spec_decode_12_finalize
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (v_K: usize{v v_K == 2 \/ v v_K == 3 \/ v v_K == 4})
    (public_key: t_Slice u8)
    (pk: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
  : Lemma
    (requires
      Seq.length public_key == v v_K * 384 /\
      (forall (j: nat). j < v v_K ==>
        VS.poly_to_spec (Seq.index pk j) ==
          S.byte_decode (mk_usize 384) (mk_usize 3072)
            (Seq.slice public_key (j * 384) (j * 384 + 384)) (mk_usize 12)))
    (ensures VS.vector_to_spec v_K pk == S.vector_decode_12_ v_K public_key)

(* Round-trip identity for the compress_then_serialize_4_/5_
   `serialized_old = serialized.to_vec().as_slice()` pre-update snapshot.  Tight
   SMTPat (fires only on the exact round-trip term) — does not cascade.  See the
   .fst for the full rationale; mirror of sha3's `lemma_as_slice_to_vec_id_u8`. *)
val lemma_as_slice_to_vec_id_u8 (s: t_Slice u8)
  : Lemma
      (ensures
        Alloc.Vec.impl_1__as_slice #u8 #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #u8 s) == s)
      [SMTPat (Alloc.Vec.impl_1__as_slice #u8 #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #u8 s))]
