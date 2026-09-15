module Libcrux_ml_kem.Vector.Avx2_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Rust_primitives.BitVectors

(* Hand-written proof theory relocated from src/vector/avx2.rs
   `hax_lib::fstar::before` block (byte-exact raw-string contents; get_bit and
   Math.Lemmas qualified for the companion's FStar.Mul+Core_models opens).
   Consumed only by that module. *)

(* Helper: if every bit of `vec` at lane position >= n (within each 16-bit
   lane) is zero, then each i16 lane of `vec256_as_i16x16 vec` is bounded
   to fit in `n` bits.  Used by every `op_deserialize_N_post_bridge` to
   discharge the per-lane `bounded` conjunct of `deserialize_post_N`. *)
let lemma_vec256_lane_bounded
      (vec: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (n: nat{n > 0 /\ n <= 16}) (i: nat{i < 16})
    : Lemma
      (requires forall (b: nat{b < 16}). b >= n ==>
                  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vec (i * 16 + b) == 0)
      (ensures
        Rust_primitives.BitVectors.bounded
          (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vec) i) n)
  = let arr = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vec in
    let lane = Seq.index arr i in
    let aux (b: usize{v b < 16}) : Lemma (v b > n ==> Rust_primitives.Integers.get_bit lane b == 0)
      = if v b > n then begin
          Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
            vec 16 (i * 16 + v b);
          FStar.Math.Lemmas.lemma_mod_plus (v b) i 16;
          FStar.Math.Lemmas.lemma_div_plus (v b) i 16
        end
        else ()
    in
    Classical.forall_intro aux;
    // The lemma_get_bit_bounded' precondition has `forall i. v i > d ==> get_bit lane i == 0`
    // implicitly under `v i < 16` (subtype on `i: usize`).  The Classical.forall_intro
    // gives us the constrained version; the SMTPat-fired lemma will use it.
    Rust_primitives.BitVectors.lemma_get_bit_bounded' lane n

let op_deserialize_1_post_bridge (input: t_Slice u8) (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
    Seq.length input == 2 /\
    (forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v i = (if i % 16 >= 1 then 0
             else let j = (i / 16) * 1 + i % 16 in
                  bit_vec_of_int_t_array (input <: t_Array _ (sz 2)) 8 j)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 input
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    let inp_arr : t_Array u8 (sz 2) = input in
    introduce forall (i: nat{i < 16}).
        bit_vec_of_int_t_array arr 1 i == bit_vec_of_int_t_array inp_arr 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 1 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 1)
      (BitVecEq.retype (bit_vec_of_int_t_array inp_arr 8));
    introduce forall (i: nat). i < 16 ==>
        Rust_primitives.BitVectors.bounded (Seq.index arr i) 1
    with introduce i < 16 ==> Rust_primitives.BitVectors.bounded (Seq.index arr i) 1
    with _. lemma_vec256_lane_bounded v 1 i

let op_serialize_1_pre_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1
              (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  (ensures forall (j: nat{j < 256}). j % 16 >= 1 ==> Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j == 0)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (j: nat{j < 256}). j % 16 >= 1 ==> Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j == 0
    with introduce j % 16 >= 1 ==> Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j == 0
    with _. begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 j;
      // bit_vec_of_int_t_array arr 16 j == v j; lane = j / 16, lane bit = j % 16 >= 1;
      // serialize_pre_N 1 ==> bounded (Seq.index arr (j/16)) 1 ==> the j%16-th bit is 0
      ()
    end

let op_serialize_1_post_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (r: t_Array u8 (mk_usize 2)) : Lemma
  (requires
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) /\
    (forall (i: nat{i < 16}).
      bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v (i * 16)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) r)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (i: nat{i < 16}).
        bit_vec_of_int_t_array arr 1 i == bit_vec_of_int_t_array r 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 1 i
      // bit_vec_of_int_t_array arr 1 i == v ((i/1)*16 + i%1) == v (i * 16)
      //   == bit_vec_of_int_t_array r 8 i   (from the primitive's post)
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 1)
      (BitVecEq.retype (bit_vec_of_int_t_array r 8))

let op_serialize_4_pre_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4
              (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  (ensures forall (j: nat{j < 256}). j % 16 < 4 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (j: nat{j < 256}). j % 16 < 4 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0
    with begin
      if j % 16 < 4 then ()
      else begin
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 j;
        // bit_vec_of_int_t_array arr 16 j == v j
        // arr lane = j / 16, lane bit = j % 16, j % 16 >= 4
        // bounded (Seq.index arr (j/16)) 4 ==> get_bit (Seq.index arr (j/16)) (j%16) == 0
        ()
      end
    end

let op_serialize_4_post_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (r: t_Array u8 (mk_usize 8)) : Lemma
  (requires
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) /\
    (forall (i: nat{i < 64}).
      bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v ((i / 4) * 16 + i % 4)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) r)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (i: nat{i < 64}).
        bit_vec_of_int_t_array arr 4 i == bit_vec_of_int_t_array r 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 4 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 4)
      (BitVecEq.retype (bit_vec_of_int_t_array r 8))

let op_deserialize_4_post_bridge (input: t_Slice u8) (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
    Seq.length input == 8 /\
    (forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v i = (if i % 16 >= 4 then 0
             else let j = (i / 16) * 4 + i % 16 in
                  bit_vec_of_int_t_array (input <: t_Array _ (sz 8)) 8 j)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 input
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    let inp_arr : t_Array u8 (sz 8) = input in
    introduce forall (i: nat{i < 64}).
        bit_vec_of_int_t_array arr 4 i == bit_vec_of_int_t_array inp_arr 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 4 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 4)
      (BitVecEq.retype (bit_vec_of_int_t_array inp_arr 8));
    introduce forall (i: nat). i < 16 ==>
        Rust_primitives.BitVectors.bounded (Seq.index arr i) 4
    with introduce i < 16 ==> Rust_primitives.BitVectors.bounded (Seq.index arr i) 4
    with _. lemma_vec256_lane_bounded v 4 i

let op_serialize_10_pre_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10
              (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  (ensures forall (j: nat{j < 256}). j % 16 < 10 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (j: nat{j < 256}). j % 16 < 10 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0
    with begin
      if j % 16 < 10 then ()
      else begin
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 j;
        ()
      end
    end

let op_serialize_10_post_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (r: t_Array u8 (mk_usize 20)) : Lemma
  (requires
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) /\
    (forall (i: nat{i < 160}).
      bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v ((i / 10) * 16 + i % 10)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) r)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (i: nat{i < 160}).
        bit_vec_of_int_t_array arr 10 i == bit_vec_of_int_t_array r 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 10 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 10)
      (BitVecEq.retype (bit_vec_of_int_t_array r 8))

let op_deserialize_10_post_bridge (input: t_Slice u8) (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
    Seq.length input == 20 /\
    (forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v i = (if i % 16 >= 10 then 0
             else let j = (i / 16) * 10 + i % 16 in
                  bit_vec_of_int_t_array (input <: t_Array _ (sz 20)) 8 j)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 input
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    let inp_arr : t_Array u8 (sz 20) = input in
    introduce forall (i: nat{i < 160}).
        bit_vec_of_int_t_array arr 10 i == bit_vec_of_int_t_array inp_arr 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 10 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 10)
      (BitVecEq.retype (bit_vec_of_int_t_array inp_arr 8));
    introduce forall (i: nat). i < 16 ==>
        Rust_primitives.BitVectors.bounded (Seq.index arr i) 10
    with introduce i < 16 ==> Rust_primitives.BitVectors.bounded (Seq.index arr i) 10
    with _. lemma_vec256_lane_bounded v 10 i

let op_serialize_12_pre_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12
              (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  (ensures forall (j: nat{j < 256}). j % 16 < 12 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (j: nat{j < 256}). j % 16 < 12 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0
    with begin
      if j % 16 < 12 then ()
      else begin
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 j;
        ()
      end
    end

let op_serialize_12_post_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (r: t_Array u8 (mk_usize 24)) : Lemma
  (requires
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) /\
    (forall (i: nat{i < 192}).
      bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v ((i / 12) * 16 + i % 12)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) r)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (i: nat{i < 192}).
        bit_vec_of_int_t_array arr 12 i == bit_vec_of_int_t_array r 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 12 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 12)
      (BitVecEq.retype (bit_vec_of_int_t_array r 8))

let op_deserialize_12_post_bridge (input: t_Slice u8) (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
    Seq.length input == 24 /\
    (forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v i = (if i % 16 >= 12 then 0
             else let j = (i / 16) * 12 + i % 16 in
                  bit_vec_of_int_t_array (input <: t_Array _ (sz 24)) 8 j)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 input
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    let inp_arr : t_Array u8 (sz 24) = input in
    introduce forall (i: nat{i < 192}).
        bit_vec_of_int_t_array arr 12 i == bit_vec_of_int_t_array inp_arr 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 12 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 12)
      (BitVecEq.retype (bit_vec_of_int_t_array inp_arr 8));
    introduce forall (i: nat). i < 16 ==>
        Rust_primitives.BitVectors.bounded (Seq.index arr i) 12
    with introduce i < 16 ==> Rust_primitives.BitVectors.bounded (Seq.index arr i) 12
    with _. lemma_vec256_lane_bounded v 12 i

let op_serialize_11_pre_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11
              (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  (ensures forall (j: nat{j < 256}). j % 16 < 11 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (j: nat{j < 256}). j % 16 < 11 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0
    with begin
      if j % 16 < 11 then ()
      else begin
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 j;
        ()
      end
    end

let op_serialize_11_post_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (r: t_Array u8 (mk_usize 22)) : Lemma
  (requires
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) /\
    (forall (i: nat{i < 176}).
      bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v ((i / 11) * 16 + i % 11)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) r)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (i: nat{i < 176}).
        bit_vec_of_int_t_array arr 11 i == bit_vec_of_int_t_array r 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 11 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 11)
      (BitVecEq.retype (bit_vec_of_int_t_array r 8))

let op_deserialize_11_post_bridge (input: t_Slice u8) (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
    Seq.length input == 22 /\
    (forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v i = (if i % 16 >= 11 then 0
             else let j = (i / 16) * 11 + i % 16 in
                  bit_vec_of_int_t_array (input <: t_Array _ (sz 22)) 8 j)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 input
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    let inp_arr : t_Array u8 (sz 22) = input in
    introduce forall (i: nat{i < 176}).
        bit_vec_of_int_t_array arr 11 i == bit_vec_of_int_t_array inp_arr 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 11 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 11)
      (BitVecEq.retype (bit_vec_of_int_t_array inp_arr 8));
    introduce forall (i: nat). i < 16 ==>
        Rust_primitives.BitVectors.bounded (Seq.index arr i) 11
    with introduce i < 16 ==> Rust_primitives.BitVectors.bounded (Seq.index arr i) 11
    with _. lemma_vec256_lane_bounded v 11 i

let op_serialize_5_pre_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5
              (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  (ensures forall (j: nat{j < 256}). j % 16 < 5 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (j: nat{j < 256}). j % 16 < 5 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v j = 0
    with begin
      if j % 16 < 5 then ()
      else begin
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 j;
        ()
      end
    end

let op_serialize_5_post_bridge (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (r: t_Array u8 (mk_usize 10)) : Lemma
  (requires
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) /\
    (forall (i: nat{i < 80}).
      bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v ((i / 5) * 16 + i % 5)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) r)
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    introduce forall (i: nat{i < 80}).
        bit_vec_of_int_t_array arr 5 i == bit_vec_of_int_t_array r 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 5 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 5)
      (BitVecEq.retype (bit_vec_of_int_t_array r 8))

let op_deserialize_5_post_bridge (input: t_Slice u8) (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
    Seq.length input == 10 /\
    (forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit v i = (if i % 16 >= 5 then 0
             else let j = (i / 16) * 5 + i % 16 in
                  bit_vec_of_int_t_array (input <: t_Array _ (sz 10)) 8 j)))
  (ensures
    Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 input
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v))
  = let arr : t_Array i16 (sz 16) =
      Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v
    in
    let inp_arr : t_Array u8 (sz 10) = input in
    introduce forall (i: nat{i < 80}).
        bit_vec_of_int_t_array arr 5 i == bit_vec_of_int_t_array inp_arr 8 i
    with begin
      Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 5 i
    end;
    BitVecEq.bit_vec_equal_intro
      (bit_vec_of_int_t_array arr 5)
      (BitVecEq.retype (bit_vec_of_int_t_array inp_arr 8));
    introduce forall (i: nat). i < 16 ==>
        Rust_primitives.BitVectors.bounded (Seq.index arr i) 5
    with introduce i < 16 ==> Rust_primitives.BitVectors.bounded (Seq.index arr i) 5
    with _. lemma_vec256_lane_bounded v 5 i
