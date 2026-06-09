module Hacspec_ml_kem.Commute.Serialize_compress
#set-options "--fuel 1 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Rust_primitives.BitVectors

module ML = FStar.Math.Lemmas
module S  = Hacspec_ml_kem.Serialize
module P  = Hacspec_ml_kem.Parameters
module F  = Rust_primitives.Hax.Folds
module BV = BitVecEq
module VTS = Libcrux_ml_kem.Vector.Traits.Spec
module SB = Hacspec_ml_kem.Commute.Serialize_bits

(* E1 generic-in-d: bitvector_from_bounded_ints index *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bitvec_from_bounded_index_d
    (d: usize{v d > 0 /\ v d <= 12})
    (input: t_Array u16 (mk_usize 256)) (p: nat{p < 256 * v d})
  : Lemma (Seq.index (S.bitvector_from_bounded_ints (mk_usize 256) (mk_usize (256 * v d)) input d) p
           == (get_bit_nat (v (Seq.index input (p / v d))) (p % v d) = 1))
  = let pp = mk_usize p in
    assert (p == v pp);
    assert (Seq.index (S.bitvector_from_bounded_ints (mk_usize 256) (mk_usize (256 * v d)) input d) (v pp)
            == ((((input.[ pp /! d <: usize ] <: u16) >>! (pp %! d <: usize) <: u16)
                 &. mk_u16 1 <: u16) =. mk_u16 1))
      by (FStar.Tactics.norm [delta_only [`%S.bitvector_from_bounded_ints]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    let idx = v (pp /! d <: usize) in
    let sh  = pp %! d in
    assert (idx == p / v d);
    assert (v sh == p % v d);
    let byte = Seq.index input idx in
    SB.lemma_val_and1_u16 (byte >>! sh);
    SB.lemma_get_bit_nat_eq byte sh
#pop-options

(* E2 generic byte-count: bits_to_bytes bit *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bits_to_bytes_bit_d
    (d: usize{v d > 0 /\ v d <= 12})
    (bv: t_Array bool (mk_usize (256 * v d))) (m: nat{m < 32 * v d}) (t: nat{t < 8})
  : Lemma (get_bit (Seq.index (S.bits_to_bytes (mk_usize (32 * v d)) (mk_usize (256 * v d)) bv) m) (sz t)
           == (if Seq.index bv (8 * m + t) then 1 else 0))
  = let mm = mk_usize m in
    assert (m == v mm);
    assert (Seq.index (S.bits_to_bytes (mk_usize (32 * v d)) (mk_usize (256 * v d)) bv) (v mm)
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
    assert (v (mk_usize 8 *! mm <: usize) == 8 * m);
    assert (forall (s: nat). s < 8 ==> v ((mk_usize 8 *! mm <: usize) +! mk_usize s <: usize) == 8 * m + s)
#pop-options

(* byte_encode bit, generic-in-d *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_byte_encode_bit_d
    (d: usize{v d > 0 /\ v d <= 12})
    (p: t_Array P.t_FieldElement (mk_usize 256)) (m: nat{m < 32 * v d}) (t: nat{t < 8})
  : Lemma
    (get_bit (Seq.index (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d) m) (sz t)
     == get_bit_nat (v (Seq.index p ((8 * m + t) / v d)).Hacspec_ml_kem.Parameters.f_val) ((8 * m + t) % v d))
  = let praw : t_Array u16 (mk_usize 256) =
      P.createi #u16 (mk_usize 256) #(usize -> u16)
        (fun i -> let i:usize = i in (p.[ i ] <: P.t_FieldElement).Hacspec_ml_kem.Parameters.f_val) in
    let bv = S.bitvector_from_bounded_ints (mk_usize 256) (mk_usize (256 * v d)) praw d in
    assert (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d
            == S.bits_to_bytes (mk_usize (32 * v d)) (mk_usize (256 * v d)) bv)
      by (FStar.Tactics.norm [delta_only [`%S.byte_encode]; zeta; iota; primops];
          FStar.Tactics.trefl ());
    let p' = 8 * m + t in
    lemma_bits_to_bytes_bit_d d bv m t;
    lemma_bitvec_from_bounded_index_d d praw p';
    let kk = mk_usize (p' / v d) in
    assert (p' / v d < 256);
    assert (Seq.index praw (v kk) == (p.[ kk ] <: P.t_FieldElement).Hacspec_ml_kem.Parameters.f_val)
      by (FStar.Tactics.l_to_r [`P.createi_lemma]; FStar.Tactics.trefl ())
#pop-options

(* per-byte bridge, generic-in-d AND generic-in-p *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_byte_eq_d
    (d: usize{v d > 0 /\ v d <= 12})
    (out_len: usize{v out_len == 32 * v d})
    (serialized: t_Array u8 out_len)
    (coefficient: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (i: nat{i < 16}) (r: nat{r < 2 * v d})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq coefficient (v d) (Seq.slice serialized (2 * v d * i) (2 * v d * i + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index coefficient l) (v d)) /\
      (forall (l: nat). l < 16 ==> v (Seq.index coefficient l)
                                  == v (Seq.index p (16 * i + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures
      Seq.index serialized (2 * v d * i + r)
      == Seq.index (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d) (2 * v d * i + r))
  = let dd = v d in
    let m = 2 * dd * i + r in
    let chunk : t_Array u8 (mk_usize (2 * dd)) = Seq.slice serialized (2 * dd * i) (2 * dd * i + 2 * dd) in
    let be = S.byte_encode (mk_usize (32 * dd)) (mk_usize (256 * dd)) p d in
    let aux (t: nat{t < 8}) : Lemma (get_bit (Seq.index serialized m) (sz t) == get_bit (Seq.index be m) (sz t)) =
      let pp = 8 * r + t in
      let l' = pp / dd in
      assert (pp < 16 * dd);
      assert (l' < 16);
      assert (bit_vec_of_int_t_array coefficient dd pp == get_bit (Seq.index coefficient l') (mk_usize (pp % dd)));
      assert (bit_vec_of_int_t_array coefficient dd pp == bit_vec_of_int_t_array chunk 8 pp);
      BV.int_t_seq_slice_to_bv_sub_lemma serialized (2 * dd * i) (mk_usize (2 * dd)) 8;
      assert (bit_vec_of_int_t_array chunk 8 pp
              == BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((2 * dd * i) * 8) (2 * dd * 8) pp);
      assert ((2 * dd * i) * 8 == 16 * dd * i);
      assert (BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((2 * dd * i) * 8) (2 * dd * 8) pp
              == bit_vec_of_int_t_array serialized 8 ((2 * dd * i) * 8 + pp));
      assert (16 * dd * i + pp == 8 * m + t);
      assert (bit_vec_of_int_t_array serialized 8 (8 * m + t) == get_bit (Seq.index serialized m) (sz t));
      SB.lemma_get_bit_nat_eq (Seq.index coefficient l') (mk_usize (pp % dd));
      lemma_byte_encode_bit_d d p m t;
      ML.lemma_div_plus pp (16 * i) dd;   // (pp + 16*i*dd)/dd == pp/dd + 16*i
      ML.lemma_mod_plus pp (16 * i) dd;   // (pp + 16*i*dd)%dd == pp%dd
      assert ((8 * m + t) / dd == 16 * i + l');
      assert ((8 * m + t) % dd == pp % dd)
    in
    FStar.Classical.forall_intro aux;
    introduce forall (j: usize {v j < 8}).
        get_bit (Seq.index serialized m) j == get_bit (Seq.index be m) j
    with aux (v j);
    lemma_int_t_eq_via_bits (Seq.index serialized m) (Seq.index be m)
#pop-options

(* per-chunk: all 2d bytes of chunk i agree with byte_encode *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_serialize_chunk_eq_byte_encode_d
    (d: usize{v d > 0 /\ v d <= 12})
    (out_len: usize{v out_len == 32 * v d})
    (serialized: t_Array u8 out_len)
    (coefficient: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (i: nat{i < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq coefficient (v d) (Seq.slice serialized (2 * v d * i) (2 * v d * i + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index coefficient l) (v d)) /\
      (forall (l: nat). l < 16 ==> v (Seq.index coefficient l)
                                  == v (Seq.index p (16 * i + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures
      (forall (r: nat). r < 2 * v d ==>
        Seq.index serialized (2 * v d * i + r)
        == Seq.index (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d) (2 * v d * i + r)))
  = let aux (r: nat) : Lemma (r < 2 * v d ==>
        Seq.index serialized (2 * v d * i + r)
        == Seq.index (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d) (2 * v d * i + r)) =
      if r < 2 * v d then lemma_serialize_byte_eq_d d out_len serialized coefficient p i r
    in
    FStar.Classical.forall_intro aux
#pop-options

module C = Hacspec_ml_kem.Compress
module VS = Libcrux_ml_kem.Vector.Spec
module VT = Libcrux_ml_kem.Vector.Traits

(* ---- p-generic, d-generic opaque per-chunk encode atom ---- *)
[@@ "opaque_to_smt"]
let chunk_byte_enc_d (d: usize{v d > 0 /\ v d <= 12})
                   (out_len: usize{v out_len == 32 * v d})
                   (serialized: t_Array u8 out_len)
                   (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat) : prop =
  j < 16 /\
  (forall (r: nat). r < 2 * v d ==>
    Seq.index serialized (2 * v d * j + r)
    == Seq.index (S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d) (2 * v d * j + r))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_chunk_byte_enc_intro_d
    (d: usize{v d > 0 /\ v d <= 12})
    (out_len: usize{v out_len == 32 * v d})
    (serialized: t_Array u8 out_len) (g: t_Array i16 (mk_usize 16))
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g (v d) (Seq.slice serialized (2 * v d * j) (2 * v d * j + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 /\
      (forall (l: nat). l < 16 ==> bounded (Seq.index g l) (v d)) /\
      (forall (l: nat). l < 16 ==> v (Seq.index g l)
                                  == v (Seq.index p (16 * j + l)).Hacspec_ml_kem.Parameters.f_val))
    (ensures chunk_byte_enc_d d out_len serialized p j)
  = reveal_opaque (`%chunk_byte_enc_d) chunk_byte_enc_d;
    lemma_serialize_chunk_eq_byte_encode_d d out_len serialized g p j
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_frame_d
    (d: usize{v d > 0 /\ v d <= 12})
    (out_len: usize{v out_len == 32 * v d})
    (s_old s_new: t_Array u8 out_len)
    (p: t_Array P.t_FieldElement (mk_usize 256)) (j: nat{j < 16}) (bound: nat{bound <= v out_len})
  : Lemma
    (requires
      chunk_byte_enc_d d out_len s_old p j /\ 2 * v d * j + 2 * v d <= bound /\
      Seq.slice s_new 0 bound == Seq.slice s_old 0 bound)
    (ensures chunk_byte_enc_d d out_len s_new p j)
  = reveal_opaque (`%chunk_byte_enc_d) chunk_byte_enc_d;
    ML.lemma_mult_le_left (2 * v d) j 15;
    let base : (b: nat{b + 2 * v d <= v out_len}) = 2 * v d * j in
    let be = S.byte_encode (mk_usize (32 * v d)) (mk_usize (256 * v d)) p d in
    introduce forall (r: nat). r < 2 * v d ==> Seq.index s_new (base + r) == Seq.index be (base + r)
    with introduce _ ==> _
    with _. (Seq.lemma_index_slice s_new 0 bound (base + r);
             Seq.lemma_index_slice s_old 0 bound (base + r))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_extend_d
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
  = ML.lemma_mult_le_left (2 * v d) i 16;
    let aux (j: nat{j < i + 1}) : Lemma (chunk_byte_enc_d d out_len s_new p j) =
      if j < i then begin
        ML.lemma_mult_le_left (2 * v d) (j + 1) i;
        lemma_chunk_byte_enc_frame_d d out_len s_old s_new p j (2 * v d * i)
      end
    in
    FStar.Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_byte_enc_unfold_d
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
  = reveal_opaque (`%chunk_byte_enc_d) chunk_byte_enc_d
#pop-options

(* ---- compress createi index ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_compress_index
    (re: t_Array P.t_FieldElement (mk_usize 256)) (d: usize{d <. mk_usize 12}) (k: nat{k < 256})
  : Lemma (Seq.index (C.compress re d) k == C.compress_d (Seq.index re k) d)
  = let kk = mk_usize k in
    assert (Seq.index (C.compress re d) (v kk) == C.compress_d (re.[ kk ] <: P.t_FieldElement) d)
      by (FStar.Tactics.norm [delta_only [`%C.compress]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ())
#pop-options

(* ---- compress value-match (clean context, single l) ----
   Input form matches what the composer holds: g = f_repr (compress unreduced),
   inp = f_repr unreduced, with the to_unsigned bridge inp ~ re.coefficients[j]. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_compress_value_match_one
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (d: usize{v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11})
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    (g inp: t_Array i16 (mk_usize 16)) (j: nat{j < 16}) (l: nat{l < 16})
  : Lemma
    (requires
      v (Seq.index g l) >= 0 /\ v (Seq.index g l) < pow2 (v d) /\
      VTS.i16_to_spec_fe (Seq.index g l)
      == C.compress_d (VTS.i16_to_spec_fe (Seq.index inp l)) d /\
      VTS.i16_to_spec_fe (Seq.index inp l)
      == VTS.i16_to_spec_fe (Seq.index (VT.f_repr
            (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) l))
    (ensures
      bounded (Seq.index g l) (v d) /\
      v (Seq.index g l)
      == v (Seq.index (C.compress (VS.poly_to_spec re) d) (16 * j + l)).Hacspec_ml_kem.Parameters.f_val)
  = let p = VS.poly_to_spec re in
    ML.lemma_div_plus l j 16;
    ML.lemma_mod_plus l j 16;
    VS.poly_to_spec_index re (16 * j + l);
    lemma_compress_index p d (16 * j + l);
    assert (pow2 (v d) <= pow2 11);
    assert_norm (pow2 11 == 2048)
#pop-options

(* ---- compress intro: establish the atom for p = compress (poly_to_spec re) d ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_chunk_byte_enc_intro_compress
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: VT.t_Operations v_Vector)
    (d: usize{v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11})
    (out_len: usize{v out_len == 32 * v d})
    (serialized: t_Array u8 out_len)
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    (inp g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      BV.int_t_array_bitwise_eq g (v d) (Seq.slice serialized (2 * v d * j) (2 * v d * j + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 /\
      (forall (l: nat). l < 16 ==> v (Seq.index g l) >= 0 /\ v (Seq.index g l) < pow2 (v d)) /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l) == C.compress_d (VTS.i16_to_spec_fe (Seq.index inp l)) d) /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index inp l)
        == VTS.i16_to_spec_fe (Seq.index (VT.f_repr
              (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients j)) l)))
    (ensures chunk_byte_enc_d d out_len serialized (C.compress (VS.poly_to_spec re) d) j)
  = let p = C.compress (VS.poly_to_spec re) d in
    let aux (l: nat{l < 16}) : Lemma
      (bounded (Seq.index g l) (v d) /\
       v (Seq.index g l) == v (Seq.index p (16 * j + l)).Hacspec_ml_kem.Parameters.f_val) =
      lemma_compress_value_match_one d re g inp j l
    in
    FStar.Classical.forall_intro aux;
    lemma_chunk_byte_enc_intro_d d out_len serialized g p j
#pop-options

(* targeted (per-application) reveal of compress_d_lane_post — tiny clean VC,
   no universal reveal (which saturates Z3 in any non-trivial context). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_compress_lane_eq
    (d: usize{v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11}) (input result: i16)
  : Lemma (requires VTS.compress_d_lane_post d input result)
          (ensures VTS.i16_to_spec_fe result == C.compress_d (VTS.i16_to_spec_fe input) d)
  = reveal_opaque (`%VTS.compress_d_lane_post) (VTS.compress_d_lane_post d input result)
#pop-options

(* Extract the per-lane compress equations from `compress_post` in a MINIMAL
   context (only compress_post in scope) — the `match l` dispatch + targeted
   reveals saturate if the heavy bitvec/value-match hypotheses are also present. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_compress_post_lanes
    (cb: i32{v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11})
    (inp g: t_Array i16 (mk_usize 16))
  : Lemma
    (requires VTS.compress_post inp cb g)
    (ensures
      (forall (l: nat). l < 16 ==> v (Seq.index g l) >= 0 /\ v (Seq.index g l) < pow2 (v cb)) /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == C.compress_d (VTS.i16_to_spec_fe (Seq.index inp l)) (mk_usize (v cb))))
  = let d : usize = mk_usize (v cb) in
    let aux_eq (l: nat{l < 16}) : Lemma
      (VTS.i16_to_spec_fe (Seq.index g l) == C.compress_d (VTS.i16_to_spec_fe (Seq.index inp l)) d) =
      (match l with
       | 0  -> lemma_compress_lane_eq d (Seq.index inp 0)  (Seq.index g 0)
       | 1  -> lemma_compress_lane_eq d (Seq.index inp 1)  (Seq.index g 1)
       | 2  -> lemma_compress_lane_eq d (Seq.index inp 2)  (Seq.index g 2)
       | 3  -> lemma_compress_lane_eq d (Seq.index inp 3)  (Seq.index g 3)
       | 4  -> lemma_compress_lane_eq d (Seq.index inp 4)  (Seq.index g 4)
       | 5  -> lemma_compress_lane_eq d (Seq.index inp 5)  (Seq.index g 5)
       | 6  -> lemma_compress_lane_eq d (Seq.index inp 6)  (Seq.index g 6)
       | 7  -> lemma_compress_lane_eq d (Seq.index inp 7)  (Seq.index g 7)
       | 8  -> lemma_compress_lane_eq d (Seq.index inp 8)  (Seq.index g 8)
       | 9  -> lemma_compress_lane_eq d (Seq.index inp 9)  (Seq.index g 9)
       | 10 -> lemma_compress_lane_eq d (Seq.index inp 10) (Seq.index g 10)
       | 11 -> lemma_compress_lane_eq d (Seq.index inp 11) (Seq.index g 11)
       | 12 -> lemma_compress_lane_eq d (Seq.index inp 12) (Seq.index g 12)
       | 13 -> lemma_compress_lane_eq d (Seq.index inp 13) (Seq.index g 13)
       | 14 -> lemma_compress_lane_eq d (Seq.index inp 14) (Seq.index g 14)
       | _  -> lemma_compress_lane_eq d (Seq.index inp 15) (Seq.index g 15))
    in
    FStar.Classical.forall_intro aux_eq
#pop-options

(* ---- compress intro from the RAW trait `compress_post` ----
   The composer's heavy loop body never carries a reveal: the lane extraction
   happens in `lemma_compress_post_lanes` (minimal context). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_chunk_byte_enc_intro_compress_post
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
  = let d : usize = mk_usize (v cb) in
    lemma_compress_post_lanes cb inp g;
    lemma_chunk_byte_enc_intro_compress d out_len serialized re inp g j
#pop-options
