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
    (d: usize{v d > 0 /\ v d < 12})
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
    (d: usize{v d > 0 /\ v d < 12})
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

(* ================================================================== *)
(* DECODE-U bridge (deserialize_then_decompress 10/11).  Promoted from   *)
(* the validated Scratch_du development.  Generic-in-d FOLD byte-bridge  *)
(* + byte_decode_dyn reconciliation + decompress value-match + atom.     *)
(* ================================================================== *)
(* ================================================================== *)
(* Part A: byte_decode_dyn reconciliation                              *)
(*   byte_decode_dyn b d == byte_decode (32d) (256d) b d  for the      *)
(*   slice b with len == 32d.  Reduces to unwrap(try_into b) == b.     *)
(* ================================================================== *)

(* unwrap (try_into b) == b for a length-matching slice -> array.
   try_from (impl_2) returns Ok (array_from_fn n (slice_index b)) when
   len b == n; unwrap (Ok x) == x; array_from_fn is pointwise = b. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 100"
let lemma_slice_to_array_id (n: usize) (array: t_Slice u8)
  : Lemma (requires Seq.length array == v n)
          (ensures Core_models.Result.impl__unwrap #(t_Array u8 n)
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 n)
                #FStar.Tactics.Typeclasses.solve array) == array)
  = let a : t_Array u8 n = Core_models.Result.impl__unwrap #(t_Array u8 n)
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 n)
                #FStar.Tactics.Typeclasses.solve array) in
    assert (Core_models.Slice.impl__len #u8 array == n);
    Seq.lemma_eq_intro a array
#pop-options

(* byte_decode_dyn b d == byte_decode (32d) (256d) b d, for d in {4,5,10,11}. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_byte_decode_dyn_eq (serialized: t_Slice u8) (d: usize{v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11})
  : Lemma (requires Seq.length serialized == 32 * v d)
          (ensures S.byte_decode_dyn serialized d
                   == S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d)
  = if v d = 4 then lemma_slice_to_array_id (mk_usize 128) serialized
    else if v d = 5 then lemma_slice_to_array_id (mk_usize 160) serialized
    else if v d = 10 then lemma_slice_to_array_id (mk_usize 320) serialized
    else lemma_slice_to_array_id (mk_usize 352) serialized
#pop-options

(* ================================================================== *)
(* Part 2: generic-in-d DECODE byte-bridge (the FOLD side).            *)
(*   Generalize Serialize_bits dec_inv/dec_step/lemma_dec_aux/         *)
(*   lemma_bitvec_to_bounded_index_12 from d=12 to symbolic d.         *)
(* ================================================================== *)

(* named inv/step for bitvector_to_bounded_ints' inner fold at width d,
   byte-copied from the spec (12 -> d) so the createi body delta/beta-
   reduces to `fold_range 0 d dec_inv_d (mk_u16 0) (dec_step_d input i)`. *)
let dec_inv_d (d: usize{v d > 0 /\ v d <= 12})
  : u16 -> (j:usize{F.fold_range_wf_index (mk_usize 0) d false (v j)}) -> Type0 =
  (fun coefficient j ->
      let coefficient:u16 = coefficient in
      let j:usize = j in
      coefficient <. (mk_u16 1 <<! j <: u16) <: bool)

#push-options "--z3rlimit 300"
let dec_step_d (d: usize{v d > 0 /\ v d <= 12})
               (input: t_Array bool (mk_usize (256 * v d))) (i: usize{i <. mk_usize 256})
  : (acc:u16 -> j:usize {v j <= v d /\ F.fold_range_wf_index (mk_usize 0) d true (v j) /\ dec_inv_d d acc j}
              -> acc':u16 {dec_inv_d d acc' (mk_int (v j + 1))}) =
  (fun coefficient j ->
      let coefficient:u16 = coefficient in
      let j:usize = j in
      if input.[ (i *! d <: usize) +! j <: usize ] <: bool
      then
        let coefficient:u16 = coefficient +! (mk_u16 1 <<! j <: u16) in
        coefficient
      else coefficient)
#pop-options

(* the fold value, by upward recursion on the start (peel + IH) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let rec lemma_dec_aux_d
    (d: usize{v d > 0 /\ v d <= 12})
    (input: t_Array bool (mk_usize (256 * v d))) (i: usize{i <. mk_usize 256})
    (start: nat{start <= v d}) (acc: u16)
  : Lemma
    (requires dec_inv_d d acc (mk_usize start) /\
              v acc == SB.bitsum (fun j -> j < start && Seq.index input (v i * v d + j)) start)
    (ensures
      v (F.fold_range (mk_usize start) d (dec_inv_d d) acc (dec_step_d d input i))
      == SB.bitsum (fun j -> j < v d && Seq.index input (v i * v d + j)) (v d))
    (decreases (v d - start))
  = if start = v d then begin
      SB.bitsum_cong (fun j -> j < start && Seq.index input (v i * v d + j))
                     (fun j -> j < v d && Seq.index input (v i * v d + j)) (v d)
    end
    else begin
      let f = dec_step_d d input i in
      let ss = mk_usize start in
      SB.lemma_fold_range_step #u16 ss d (dec_inv_d d) acc f;
      let acc' = f acc ss in
      SB.lemma_shl1_u16 start;                    // v (mk_u16 1 <<! ss) == pow2 start
      assert (v acc < pow2 start);                // from dec_inv_d acc ss
      ML.pow2_le_compat 11 start;                 // pow2 start <= pow2 11  (start <= 11)
      assert_norm (pow2 11 == 2048);
      assert_norm (pow2 16 == 65536);
      // index value & extracted bit
      assert (v ((i *! d <: usize) +! ss <: usize) == v i * v d + start);
      let bit : bool = Seq.index input (v i * v d + start) in
      assert (acc' == (if bit then acc +! (mk_u16 1 <<! ss) else acc));
      assert (v acc' == v acc + (if bit then pow2 start else 0));
      let g = (fun (j: nat) -> j < start + 1 && Seq.index input (v i * v d + j)) in
      let h = (fun (j: nat) -> j < start && Seq.index input (v i * v d + j)) in
      SB.bitsum_cong h g start;                   // bitsum h start == bitsum g start
      assert (g start == bit);
      assert (SB.bitsum g (start + 1) == SB.bitsum g start + (if bit then pow2 start else 0));
      assert (v acc' == SB.bitsum g (start + 1));
      lemma_dec_aux_d d input i (start + 1) acc'
    end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bitvec_to_bounded_index_d
  (d: usize{v d > 0 /\ v d <= 12})
  (input: t_Array bool (mk_usize (256 * v d))) (k: nat{k < 256})
  : Lemma (ensures
      v (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize (256 * v d)) input d) k)
      == SB.bitsum (fun j -> j < v d && Seq.index input (k * v d + j)) (v d))
  = let kk = mk_usize k in
    assert (k == v kk);
    assert (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize (256 * v d)) input d) (v kk)
            == F.fold_range (mk_usize 0) d (dec_inv_d d) (mk_u16 0) (dec_step_d d input kk))
      by (FStar.Tactics.norm [delta_only [`%S.bitvector_to_bounded_ints]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    SB.lemma_shl1_u16 0;
    lemma_dec_aux_d d input kk 0 (mk_u16 0);
    SB.bitsum_cong (fun j -> j < v d && Seq.index input (v kk * v d + j))
                   (fun j -> j < v d && Seq.index input (k * v d + j)) (v d)
#pop-options

(* ---- byte-bridge upper layers, generic-in-d (mirror Serialize_bits 229-373) ---- *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_bytes_to_bits_index_d (d: usize{v d > 0 /\ v d <= 12})
    (b: t_Array u8 (mk_usize (32 * v d))) (m: nat {m < 256 * v d})
  : Lemma (Seq.index (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) b) m
           == (get_bit_nat (v (Seq.index b (m / 8))) (m % 8) = 1))
  = let mm = mk_usize m in
    assert (m == v mm);
    assert (Seq.index (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) b) (v mm)
            == (((Seq.index b (v (mm /! mk_usize 8)) >>! (mm %! mk_usize 8)) &. mk_u8 1) = mk_u8 1))
      by (FStar.Tactics.norm [delta_only [`%S.bytes_to_bits]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    let byte = Seq.index b (m / 8) in
    let sh   = mm %! mk_usize 8 in
    SB.lemma_val_and1 (byte >>! sh);
    SB.lemma_get_bit_nat_eq byte sh
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_coeff_bit_d (d: usize{v d > 0 /\ v d <= 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16}) (l: nat {l < 16}) (j: nat {j < v d})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (2 * v d * i) (2 * v d * i + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 group (v d) /\
      v (Seq.index group l) >= 0)
    (ensures
      (get_bit_nat (v (Seq.index group l)) j = 1)
      == Seq.index (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized) ((16 * i + l) * v d + j))
  = let dd = v d in
    let chunk : t_Array u8 (mk_usize (2 * dd)) = Seq.slice serialized (2 * dd * i) (2 * dd * i + 2 * dd) in
    let p = dd * l + j in
    let m = (16 * dd) * i + p in
    assert (p < 16 * dd);
    assert (m == (16 * i + l) * dd + j);
    SB.lemma_get_bit_nat_eq (Seq.index group l) (mk_usize j);
    assert (p / dd == l /\ p % dd == j);
    assert (bit_vec_of_int_t_array group dd p == get_bit (Seq.index group l) (mk_usize j));
    assert (bit_vec_of_int_t_array group dd p == bit_vec_of_int_t_array chunk 8 p);
    BV.int_t_seq_slice_to_bv_sub_lemma serialized (2 * dd * i) (mk_usize (2 * dd)) 8;
    assert ((2 * dd * i) * 8 == 16 * dd * i);
    assert (bit_vec_of_int_t_array chunk 8 p
            == BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((2 * dd * i) * 8) (2 * dd * 8) p);
    assert (BV.bit_vec_sub (bit_vec_of_int_t_array serialized 8) ((2 * dd * i) * 8) (2 * dd * 8) p
            == bit_vec_of_int_t_array serialized 8 m);
    SB.lemma_get_bit_nat_eq (Seq.index serialized (m / 8)) (mk_usize (m % 8));
    assert (bit_vec_of_int_t_array serialized 8 m
            == get_bit (Seq.index serialized (m / 8)) (mk_usize (m % 8)));
    lemma_bytes_to_bits_index_d d serialized m
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_coeff_value_d (d: usize{v d > 0 /\ v d <= 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16}) (l: nat {l < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (2 * v d * i) (2 * v d * i + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 group (v d) /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index group ll) (v d)))
    (ensures
      v (Seq.index group l)
      == v (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize (256 * v d))
              (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized) d) (16 * i + l)))
  = let dd = v d in
    let k = 16 * i + l in
    let bv = S.bytes_to_bits (mk_usize (32 * dd)) (mk_usize (256 * dd)) serialized in
    assert (Seq.length bv == 256 * dd);
    ML.lemma_mult_le_right dd (k + 1) 256;          // (k+1)*dd <= 256*dd
    // hand Z3 the index bound as a TYPING FACT (refined binder), not a strict
    // inequality to re-derive by simplex (which saturates in full-module context).
    let kd : (m:nat{m == k * dd /\ m + dd <= 256 * dd}) = k * dd in
    SB.lemma_recon_nat (v (Seq.index group l)) dd;
    let aux (j: nat) : Lemma (j < dd ==>
        (get_bit_nat (v (Seq.index group l)) j = 1) == (j < dd && Seq.index bv (kd + j))) =
      if j < dd then lemma_coeff_bit_d d serialized group i l j
    in
    FStar.Classical.forall_intro aux;
    SB.bitsum_cong (fun j -> get_bit_nat (v (Seq.index group l)) j = 1)
                   (fun j -> j < dd && Seq.index bv (kd + j)) dd;
    lemma_bitvec_to_bounded_index_d d bv k
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --z3refresh"
let lemma_byte_decode_index_d (d: usize{v d > 0 /\ v d <= 12})
    (serialized: t_Array u8 (mk_usize (32 * v d))) (k: nat {k < 256})
  : Lemma
    (Seq.index (S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d) k
     == P.impl_FieldElement__new
          (Seq.index (S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize (256 * v d))
             (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized) d) k
           %! P.v_FIELD_MODULUS))
  = let kk = mk_usize k in
    assert (k == v kk);
    // byte_decode's well-formedness precond is `v_D32 =. 32 *! d` / `v_D256 =. 256 *! d`
    // (nonlinear at symbolic d).  Hand Z3 these as ground facts so the tactic-asserts'
    // SMT side-goals don't saturate the createi context.
    assert (mk_usize 32 *! d == mk_usize (32 * v d));
    assert (mk_usize 256 *! d == mk_usize (256 * v d));
    assert (S.byte_decode_generic (mk_usize 32) (mk_usize 256) (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d
            == S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize (256 * v d))
                 (S.bytes_to_bits (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized) d)
      by (FStar.Tactics.norm [delta_only [`%S.byte_decode_generic]; zeta; iota; primops];
          FStar.Tactics.trefl ());
    assert (Seq.index (S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d) (v kk)
            == P.impl_FieldElement__new
                 (Seq.index (S.byte_decode_generic (mk_usize 32) (mk_usize 256) (mk_usize (32 * v d))
                               (mk_usize (256 * v d)) serialized d) (v kk)
                  %! P.v_FIELD_MODULUS))
      by (FStar.Tactics.norm [delta_only [`%S.byte_decode]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ())
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_deserialize_coeff_eq_byte_decode_d (d: usize{v d > 0 /\ v d <= 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (group: t_Array i16 (mk_usize 16))
    (i: nat {i < 16}) (l: nat {l < 16})
  : Lemma
    (requires
      BV.int_t_array_bitwise_eq (Seq.slice serialized (2 * v d * i) (2 * v d * i + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 group (v d) /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index group ll) (v d)))
    (ensures
      VTS.i16_to_spec_fe (Seq.index group l)
      == Seq.index (S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d) (16 * i + l))
  = let dd = v d in
    let k = 16 * i + l in
    let decoded = S.bitvector_to_bounded_ints (mk_usize 256) (mk_usize (256 * dd))
                    (S.bytes_to_bits (mk_usize (32 * dd)) (mk_usize (256 * dd)) serialized) d in
    lemma_coeff_value_d d serialized group i l;
    lemma_byte_decode_index_d d serialized k;
    let r1 = VTS.i16_to_spec_fe (Seq.index group l) in
    let r2 = Seq.index (S.byte_decode (mk_usize (32 * dd)) (mk_usize (256 * dd)) serialized d) k in
    assert (v r1.Hacspec_ml_kem.Parameters.f_val == v (Seq.index group l) % 3329);
    assert (v r2.Hacspec_ml_kem.Parameters.f_val == v (Seq.index decoded k) % 3329);
    assert (r1.Hacspec_ml_kem.Parameters.f_val == r2.Hacspec_ml_kem.Parameters.f_val)
#pop-options

(* ================================================================== *)
(* Part 3: decompress value-match (mirror of the compress side).       *)
(* ================================================================== *)


(* per-lane bound, proven in a clean context (no decompress hypothesis -> the
   decompress createi-lambda quantifier does not pollute the instantiation). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_fe_bound_at
    (re: t_Array P.t_FieldElement (mk_usize 256)) (d: usize{d <. mk_usize 12}) (k: nat{k < 256})
  : Lemma (requires (forall (i: usize). b2t (i <. mk_usize 256) ==>
              b2t ((re.[ i ]).Hacspec_ml_kem.Parameters.f_val <. (mk_u16 1 <<! d <: u16))))
          (ensures (re.[ mk_usize k ] <: P.t_FieldElement).Hacspec_ml_kem.Parameters.f_val <. (mk_u16 1 <<! d <: u16))
  = ()
#pop-options

(* decompress createi index.  `dec` is supplied as a hypothesis (== decompress
   re d) so the ensures does NOT re-form `decompress re d`.  The per-k bound is
   an explicit requires (provided by the caller via `lemma_fe_bound_at`) so the
   ensures RHS `decompress_d (Seq.index re k) d` is well-typed WITHOUT having to
   instantiate the forall (whose `re.[i]` pattern can't match `Seq.index re k`). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_decompress_index
    (re: t_Array P.t_FieldElement (mk_usize 256)) (d: usize{d <. mk_usize 12})
    (dec: t_Array P.t_FieldElement (mk_usize 256)) (k: nat{k < 256})
  : Lemma
    (requires
      (forall (i: usize). b2t (i <. mk_usize 256) ==>
        b2t ((re.[ i ]).Hacspec_ml_kem.Parameters.f_val <. (mk_u16 1 <<! d <: u16))) /\
      (Seq.index re k <: P.t_FieldElement).Hacspec_ml_kem.Parameters.f_val <. (mk_u16 1 <<! d <: u16) /\
      dec == C.decompress re d)
    (ensures Seq.index dec k == C.decompress_d (Seq.index re k) d)
  = let kk = mk_usize k in
    assert (Seq.index (C.decompress re d) (v kk) == C.decompress_d (re.[ kk ] <: P.t_FieldElement) d)
      by (FStar.Tactics.norm [delta_only [`%C.decompress]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ())
#pop-options

(* targeted (per-application) reveal of decompress_d_lane_post (clean VC) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_decompress_lane_eq
    (d: usize{v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11}) (input result: i16)
  : Lemma (requires VTS.decompress_d_lane_post d input result /\ v input >= 0 /\ v input < pow2 (v d))
          (ensures VTS.i16_to_spec_fe result == C.decompress_d (VTS.i16_to_spec_fe input) d)
  = reveal_opaque (`%VTS.decompress_d_lane_post) (VTS.decompress_d_lane_post d input result)
#pop-options

(* extract per-lane decompress value equations from decompress_ciphertext_coefficient_post,
   in a MINIMAL context (16-arm match dispatch; universal reveal saturates). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_decompress_post_lanes
    (cb: i32{v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11})
    (grp result: t_Array i16 (mk_usize 16))
  : Lemma
    (requires VTS.decompress_ciphertext_coefficient_post grp cb result /\
              (forall (l: nat). l < 16 ==> v (Seq.index grp l) >= 0 /\ v (Seq.index grp l) < pow2 (v cb)))
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index result l)
        == C.decompress_d (VTS.i16_to_spec_fe (Seq.index grp l)) (mk_usize (v cb))))
  = let d : usize = mk_usize (v cb) in
    let aux_eq (l: nat{l < 16}) : Lemma
      (VTS.i16_to_spec_fe (Seq.index result l) == C.decompress_d (VTS.i16_to_spec_fe (Seq.index grp l)) d) =
      (match l with
       | 0  -> lemma_decompress_lane_eq d (Seq.index grp 0)  (Seq.index result 0)
       | 1  -> lemma_decompress_lane_eq d (Seq.index grp 1)  (Seq.index result 1)
       | 2  -> lemma_decompress_lane_eq d (Seq.index grp 2)  (Seq.index result 2)
       | 3  -> lemma_decompress_lane_eq d (Seq.index grp 3)  (Seq.index result 3)
       | 4  -> lemma_decompress_lane_eq d (Seq.index grp 4)  (Seq.index result 4)
       | 5  -> lemma_decompress_lane_eq d (Seq.index grp 5)  (Seq.index result 5)
       | 6  -> lemma_decompress_lane_eq d (Seq.index grp 6)  (Seq.index result 6)
       | 7  -> lemma_decompress_lane_eq d (Seq.index grp 7)  (Seq.index result 7)
       | 8  -> lemma_decompress_lane_eq d (Seq.index grp 8)  (Seq.index result 8)
       | 9  -> lemma_decompress_lane_eq d (Seq.index grp 9)  (Seq.index result 9)
       | 10 -> lemma_decompress_lane_eq d (Seq.index grp 10) (Seq.index result 10)
       | 11 -> lemma_decompress_lane_eq d (Seq.index grp 11) (Seq.index result 11)
       | 12 -> lemma_decompress_lane_eq d (Seq.index grp 12) (Seq.index result 12)
       | 13 -> lemma_decompress_lane_eq d (Seq.index grp 13) (Seq.index result 13)
       | 14 -> lemma_decompress_lane_eq d (Seq.index grp 14) (Seq.index result 14)
       | _  -> lemma_decompress_lane_eq d (Seq.index grp 15) (Seq.index result 15))
    in
    FStar.Classical.forall_intro aux_eq
#pop-options

(* ================================================================== *)
(* Combined "decompressed chunk" atom + intro/unfold + finalize.       *)
(* ================================================================== *)

(* Atom uses the WHOLE-ARRAY `decompress` (no per-element decompress_d precond,
   which would re-trigger the byte_decode-ensures instantiation in the def's
   well-formedness).  The per-index reduction lives in the intro. *)
[@@ "opaque_to_smt"]
let chunk_decompressed_d (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (g: t_Array i16 (mk_usize 16)) (j: nat) : prop =
  j < 16 /\
  VTS.is_i16b_array_opaque 3328 g /\
  (forall (l: nat). l < 16 ==>
    VTS.i16_to_spec_fe (Seq.index g l)
    == Seq.index (C.decompress (S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d) d) (16 * j + l))

(* intro from the byte-bridge (grp <-> bytes) + the decompress value form (g <-> grp). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_chunk_decompressed_intro_d
    (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (grp g: t_Array i16 (mk_usize 16)) (j: nat)
  : Lemma
    (requires
      j < 16 /\
      VTS.is_i16b_array_opaque 3328 g /\
      BV.int_t_array_bitwise_eq (Seq.slice serialized (2 * v d * j) (2 * v d * j + 2 * v d) <: t_Array u8 (mk_usize (2 * v d))) 8 grp (v d) /\
      (forall (ll: nat). ll < 16 ==> bounded (Seq.index grp ll) (v d)) /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == C.decompress_d (VTS.i16_to_spec_fe (Seq.index grp l)) d))
    (ensures chunk_decompressed_d d serialized g j)
  = reveal_opaque (`%chunk_decompressed_d) chunk_decompressed_d;
    let bd = S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d in
    let target = C.decompress bd d in
    let aux (l: nat{l < 16}) : Lemma
      (VTS.i16_to_spec_fe (Seq.index g l) == Seq.index target (16 * j + l)) =
      lemma_deserialize_coeff_eq_byte_decode_d d serialized grp j l;  // i16_to_spec_fe grp[l] == bd[16j+l]
      assert (16 * j + l < 256);
      lemma_fe_bound_at bd d (16 * j + l);
      lemma_decompress_index bd d target (16 * j + l)  // target[16j+l] == decompress_d (bd[16j+l]) d
    in
    FStar.Classical.forall_intro aux
#pop-options

(* bounded_i16_array 0 3328 -> is_i16b_array_opaque 3328 (the |x| <= 3328 form). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_is_i16b_array_opaque_of_bounded (g: t_Array i16 (mk_usize 16))
  : Lemma (requires VTS.bounded_i16_array (mk_i16 0) (mk_i16 3328) g)
          (ensures VTS.is_i16b_array_opaque 3328 g)
  = reveal_opaque (`%VTS.is_i16b_array_opaque) (VTS.is_i16b_array_opaque 3328 g);
    let aux (i: nat{i < 16}) : Lemma (VTS.is_i16b 3328 (Seq.index g i)) =
      VTS.lemma_bounded_i16_array_lookup (mk_i16 0) (mk_i16 3328) g i
    in
    FStar.Classical.forall_intro aux
#pop-options

(* intro from the RAW trait `decompress_ciphertext_coefficient_post`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_chunk_decompressed_intro_post_d
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
  = lemma_decompress_post_lanes cb grp g;
    assert (mk_usize (v cb) == d);
    // decompress_ciphertext_coefficient_post gives `bounded_i16_array 0 3328 g`
    // (v cb in {4,5,10,11} fires the implication) -> is_i16b_array_opaque 3328 g.
    lemma_is_i16b_array_opaque_of_bounded g;
    lemma_chunk_decompressed_intro_d d serialized grp g j
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_decompressed_unfold_d
    (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16})
  : Lemma
    (requires chunk_decompressed_d d serialized g j)
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == Seq.index (C.decompress (S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d) d) (16 * j + l)))
  = reveal_opaque (`%chunk_decompressed_d) chunk_decompressed_d
#pop-options

(* expose the bound conjunct of the atom *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunk_decompressed_bound_d
    (d: usize{v d > 0 /\ v d < 12})
    (serialized: t_Array u8 (mk_usize (32 * v d)))
    (g: t_Array i16 (mk_usize 16)) (j: nat{j < 16})
  : Lemma
    (requires chunk_decompressed_d d serialized g j)
    (ensures VTS.is_i16b_array_opaque 3328 g)
  = reveal_opaque (`%chunk_decompressed_d) chunk_decompressed_d
#pop-options

(* finalize: poly_to_spec re == decompress (byte_decode (32d) (256d) serialized d) d *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_poly_to_spec_eq_decompress
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
  = let bd = S.byte_decode (mk_usize (32 * v d)) (mk_usize (256 * v d)) serialized d in
    let target = C.decompress bd d in
    assert (Seq.length (VS.poly_to_spec re) == 256);
    assert (Seq.length target == 256);
    let aux (k: nat{k < 256}) : Lemma (Seq.index (VS.poly_to_spec re) k == Seq.index target k) =
      assert (k / 16 < 16);
      assert (16 * (k / 16) + (k % 16) == k);
      lemma_chunk_decompressed_unfold_d d serialized
        (VT.f_to_i16_array (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients (k / 16))) (k / 16);
      VS.poly_to_spec_index re k
    in
    FStar.Classical.forall_intro aux;
    Seq.lemma_eq_intro (VS.poly_to_spec re) target
#pop-options

(* is_bounded_poly 3328 re from the per-chunk bound conjuncts *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_is_bounded_poly_of_chunks
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
  = reveal_opaque (`%Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly)
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) re);
    let aux (i: nat{i < 16}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector (mk_usize 3328)
        (re.Libcrux_ml_kem.Vector.f_coefficients.[ sz i ])) =
      lemma_chunk_decompressed_bound_d d serialized
        (VT.f_to_i16_array (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients i)) i
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ================================================================== *)
(* E (message) bridge: d=1 lane extraction + intros for compress_1 /  *)
(* decompress_1.  Their lane posts (compress_1_lane_post /            *)
(* decompress_1_lane_post) are separate from the d-in-{4,5,10,11}     *)
(* family, so they need their own targeted reveals; the generic       *)
(* chunk atoms / frame / unfold / finalize machinery above is already *)
(* symbolic in d and covers d=1.                                      *)
(* ================================================================== *)

(* targeted (per-application) reveal of compress_1_lane_post (clean VC) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_compress_1_lane_eq (input result: i16)
  : Lemma (requires VTS.compress_1_lane_post input result)
          (ensures VTS.i16_to_spec_fe result == C.compress_d (VTS.i16_to_spec_fe input) (mk_usize 1))
  = reveal_opaque (`%VTS.compress_1_lane_post) (VTS.compress_1_lane_post input result)
#pop-options

(* extract the per-lane compress_1 equations + bounds from `compress_1_post`
   in a MINIMAL context (16-arm match dispatch, mirror of
   lemma_compress_post_lanes).  The bound comes from the post's
   `bounded_pos_i16_array 1` conjunct via a targeted reveal. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_compress_1_post_lanes (inp g: t_Array i16 (mk_usize 16))
  : Lemma
    (requires VTS.compress_1_post inp g)
    (ensures
      (forall (l: nat). l < 16 ==> v (Seq.index g l) >= 0 /\ v (Seq.index g l) < pow2 1) /\
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == C.compress_d (VTS.i16_to_spec_fe (Seq.index inp l)) (mk_usize 1)))
  = assert_norm (pow2 1 - 1 == 1);
    assert_norm (pow2 1 == 2);
    reveal_opaque (`%VTS.bounded_i16_array)
      (VTS.bounded_i16_array (mk_i16 0) (mk_i16 (pow2 1 - 1)) g);
    let aux_eq (l: nat{l < 16}) : Lemma
      (VTS.i16_to_spec_fe (Seq.index g l)
       == C.compress_d (VTS.i16_to_spec_fe (Seq.index inp l)) (mk_usize 1)) =
      (match l with
       | 0  -> lemma_compress_1_lane_eq (Seq.index inp 0)  (Seq.index g 0)
       | 1  -> lemma_compress_1_lane_eq (Seq.index inp 1)  (Seq.index g 1)
       | 2  -> lemma_compress_1_lane_eq (Seq.index inp 2)  (Seq.index g 2)
       | 3  -> lemma_compress_1_lane_eq (Seq.index inp 3)  (Seq.index g 3)
       | 4  -> lemma_compress_1_lane_eq (Seq.index inp 4)  (Seq.index g 4)
       | 5  -> lemma_compress_1_lane_eq (Seq.index inp 5)  (Seq.index g 5)
       | 6  -> lemma_compress_1_lane_eq (Seq.index inp 6)  (Seq.index g 6)
       | 7  -> lemma_compress_1_lane_eq (Seq.index inp 7)  (Seq.index g 7)
       | 8  -> lemma_compress_1_lane_eq (Seq.index inp 8)  (Seq.index g 8)
       | 9  -> lemma_compress_1_lane_eq (Seq.index inp 9)  (Seq.index g 9)
       | 10 -> lemma_compress_1_lane_eq (Seq.index inp 10) (Seq.index g 10)
       | 11 -> lemma_compress_1_lane_eq (Seq.index inp 11) (Seq.index g 11)
       | 12 -> lemma_compress_1_lane_eq (Seq.index inp 12) (Seq.index g 12)
       | 13 -> lemma_compress_1_lane_eq (Seq.index inp 13) (Seq.index g 13)
       | 14 -> lemma_compress_1_lane_eq (Seq.index inp 14) (Seq.index g 14)
       | _  -> lemma_compress_1_lane_eq (Seq.index inp 15) (Seq.index g 15))
    in
    FStar.Classical.forall_intro aux_eq
#pop-options

(* d=1 intro from the RAW trait `compress_1_post` (mirror of
   lemma_chunk_byte_enc_intro_compress_post at the message width).
   `d1`/`out_len` are refined PARAMETERS (not inline literals), exactly
   like the {10,11} sibling — inlining `sz 1`/`mk_usize 32` makes the
   ensures-WF and call-arg subtyping VCs fire as ground sub-queries
   inside this decl's heavy hypothesis context, where they saturate. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_chunk_byte_enc_intro_compress_1_post
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
  = lemma_compress_1_post_lanes inp g;
    lemma_chunk_byte_enc_intro_compress d1 out_len serialized re inp g j
#pop-options

(* targeted (per-application) reveal of decompress_1_lane_post (clean VC) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_decompress_1_lane_eq (input result: i16)
  : Lemma (requires VTS.decompress_1_lane_post input result /\ v input >= 0 /\ v input < 2)
          (ensures VTS.i16_to_spec_fe result == C.decompress_d (VTS.i16_to_spec_fe input) (mk_usize 1))
  = reveal_opaque (`%VTS.decompress_1_lane_post) (VTS.decompress_1_lane_post input result)
#pop-options

(* per-lane decompress_1 value equations from `decompress_1_post`
   (16-arm match dispatch, mirror of lemma_decompress_post_lanes). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_decompress_1_post_lanes (grp g: t_Array i16 (mk_usize 16))
  : Lemma
    (requires VTS.decompress_1_post grp g /\
              (forall (l: nat). l < 16 ==> v (Seq.index grp l) >= 0 /\ v (Seq.index grp l) < 2))
    (ensures
      (forall (l: nat). l < 16 ==>
        VTS.i16_to_spec_fe (Seq.index g l)
        == C.decompress_d (VTS.i16_to_spec_fe (Seq.index grp l)) (mk_usize 1)))
  = let aux_eq (l: nat{l < 16}) : Lemma
      (VTS.i16_to_spec_fe (Seq.index g l)
       == C.decompress_d (VTS.i16_to_spec_fe (Seq.index grp l)) (mk_usize 1)) =
      (match l with
       | 0  -> lemma_decompress_1_lane_eq (Seq.index grp 0)  (Seq.index g 0)
       | 1  -> lemma_decompress_1_lane_eq (Seq.index grp 1)  (Seq.index g 1)
       | 2  -> lemma_decompress_1_lane_eq (Seq.index grp 2)  (Seq.index g 2)
       | 3  -> lemma_decompress_1_lane_eq (Seq.index grp 3)  (Seq.index g 3)
       | 4  -> lemma_decompress_1_lane_eq (Seq.index grp 4)  (Seq.index g 4)
       | 5  -> lemma_decompress_1_lane_eq (Seq.index grp 5)  (Seq.index g 5)
       | 6  -> lemma_decompress_1_lane_eq (Seq.index grp 6)  (Seq.index g 6)
       | 7  -> lemma_decompress_1_lane_eq (Seq.index grp 7)  (Seq.index g 7)
       | 8  -> lemma_decompress_1_lane_eq (Seq.index grp 8)  (Seq.index g 8)
       | 9  -> lemma_decompress_1_lane_eq (Seq.index grp 9)  (Seq.index g 9)
       | 10 -> lemma_decompress_1_lane_eq (Seq.index grp 10) (Seq.index g 10)
       | 11 -> lemma_decompress_1_lane_eq (Seq.index grp 11) (Seq.index g 11)
       | 12 -> lemma_decompress_1_lane_eq (Seq.index grp 12) (Seq.index g 12)
       | 13 -> lemma_decompress_1_lane_eq (Seq.index grp 13) (Seq.index g 13)
       | 14 -> lemma_decompress_1_lane_eq (Seq.index grp 14) (Seq.index g 14)
       | _  -> lemma_decompress_1_lane_eq (Seq.index grp 15) (Seq.index g 15))
    in
    FStar.Classical.forall_intro aux_eq
#pop-options

(* d=1 intro from the RAW trait `decompress_1_post` (mirror of
   lemma_chunk_decompressed_intro_post_d).  The bound conjunct
   `bounded_i16_array 0 3328 g` is supplied by the (strengthened)
   decompress_1_post itself.  `d1` is a refined PARAMETER (not an inline
   `sz 1` literal), exactly like the {4,5,10,11} sibling — inlining the
   literal makes the callee-arg subtyping/requires VCs fire as ground
   sub-queries inside this decl's heavy hypothesis context, where they
   saturate (same failure mode the compress-side d=1 intro had). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_chunk_decompressed_intro_1_post
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
  = assert_norm (pow2 1 == 2);
    lemma_decompress_1_post_lanes grp g;
    lemma_is_i16b_array_opaque_of_bounded g;
    lemma_chunk_decompressed_intro_d d1 serialized grp g j
#pop-options

(* ================================================================== *)
(* B2 finalize (deserialize_ring_elements_reduced): vector_to_spec pk  *)
(* == vector_decode_12_ from the per-row byte_decode invariant.        *)
(* Standalone clean-context lemmas — the in-function lemma_post shape  *)
(* fails its own statement WF ("incomplete quantifiers" on Seq.index   *)
(* bounds) under the saturated composer VC.                            *)
(* ================================================================== *)

(* per-row: vector_decode_12_'s createi row reduces to byte_decode of the
   row's 384-byte slice (try_into chunk == slice via lemma_slice_to_array_id). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --z3refresh"
let lemma_vector_decode_12_index
    (v_K: usize{v v_K == 2 \/ v v_K == 3 \/ v v_K == 4})
    (public_key: t_Slice u8)
    (j: nat{j < v v_K})
  : Lemma
    (requires Seq.length public_key == v v_K * 384)
    (ensures
      Seq.index (S.vector_decode_12_ v_K public_key) j ==
      S.byte_decode (mk_usize 384) (mk_usize 3072)
        (Seq.slice public_key (j * 384) (j * 384 + 384)) (mk_usize 12))
  = let jj : usize = mk_usize j in
    let start : usize = jj *! P.v_BYTES_PER_RING_ELEMENT in
    assert (v start == j * 384);
    assert (v (start +! mk_usize 384) == j * 384 + 384);
    let slice : t_Slice u8 = public_key.[ {
        Core_models.Ops.Range.f_start = start;
        Core_models.Ops.Range.f_end = start +! mk_usize 384 <: usize }
      <: Core_models.Ops.Range.t_Range usize ] in
    assert (slice == Seq.slice public_key (j * 384) (j * 384 + 384));
    lemma_slice_to_array_id (mk_usize 384) slice;
    assert (j == v jj);
    assert (Seq.index (S.vector_decode_12_ v_K public_key) (v jj) ==
            S.byte_decode (mk_usize 384) (mk_usize 3072)
              (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 384))
                #Core_models.Array.t_TryFromSliceError
                (Core_models.Convert.f_try_into #(t_Slice u8) #(t_Array u8 (mk_usize 384))
                  #FStar.Tactics.Typeclasses.solve slice)) (mk_usize 12))
#pop-options

(* whole-vector finalize *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_vector_to_spec_decode_12_finalize
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
  = let target = S.vector_decode_12_ v_K public_key in
    assert (Seq.length (VS.vector_to_spec v_K pk) == v v_K);
    assert (Seq.length target == v v_K);
    let aux (j: nat{j < v v_K}) : Lemma
      (Seq.index (VS.vector_to_spec v_K pk) j == Seq.index target j) =
      VS.vector_to_spec_index v_K #v_Vector pk j;
      lemma_vector_decode_12_index v_K public_key j
    in
    FStar.Classical.forall_intro aux;
    Seq.lemma_eq_intro (VS.vector_to_spec v_K pk) target
#pop-options
