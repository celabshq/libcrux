use super::*;
use crate::vector::portable::PortableVector;
use libcrux_intrinsics::arm64::*;

#[cfg(hax)]
use crate::vector::traits::spec;

#[inline(always)]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::fstar::before(
    r#"
(* Bridge: loading the two halves of a 16-element i16 array into a
   SIMD128Vector reproduces that array under `repr`.  `e_vld1q_s16`'s
   post gives the per-lane equality on each 8-lane half; `lemma_repr_index`
   stitches the two halves back via Seq.append. *)
let lemma_repr_of_two_loads (array: t_Array i16 (mk_usize 16))
    : Lemma
      (ensures
        Libcrux_ml_kem.Vector.Neon.Vector_type.repr
          ({ Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
               = Libcrux_intrinsics.Arm64_extract.e_vld1q_s16 (Seq.slice array 0 8 <: t_Slice i16);
             Libcrux_ml_kem.Vector.Neon.Vector_type.f_high
               = Libcrux_intrinsics.Arm64_extract.e_vld1q_s16 (Seq.slice array 8 16 <: t_Slice i16) }
           <: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
        == array) =
  let low = Libcrux_intrinsics.Arm64_extract.e_vld1q_s16 (Seq.slice array 0 8 <: t_Slice i16) in
  let high = Libcrux_intrinsics.Arm64_extract.e_vld1q_s16 (Seq.slice array 8 16 <: t_Slice i16) in
  let r = { Libcrux_ml_kem.Vector.Neon.Vector_type.f_low = low;
            Libcrux_ml_kem.Vector.Neon.Vector_type.f_high = high }
          <: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector in
  let lhs = Libcrux_ml_kem.Vector.Neon.Vector_type.repr r in
  assert (forall (j: nat{j < 16}). Seq.index lhs j == Seq.index array j);
  Seq.lemma_eq_intro lhs array

(* Single-lane bit-extraction through arm_sshl_i16 + AND-1.
   Bit 0 of `(arm_sshl_i16 (cast byte) shk) &. 1` equals bit k of `byte`
   (shifter shk extracts bit k), and the AND-1 makes the lane bounded to 1 bit. *)
let lemma_deser1_lane (byte: u8) (k: nat{k < 8}) (shk: i16)
    : Lemma
      (requires v (shk %! mk_i16 256) == (if k = 0 then 0 else 256 - k))
      (ensures
        (let lane = (Libcrux_intrinsics.Arm64_extract.arm_sshl_i16 (cast byte <: i16) shk) &. mk_i16 1 in
         Rust_primitives.Integers.get_bit lane (mk_usize 0)
           == Rust_primitives.Integers.get_bit byte (mk_usize k) /\
         Rust_primitives.BitVectors.bounded lane 1)) =
  let lane = (Libcrux_intrinsics.Arm64_extract.arm_sshl_i16 (cast byte <: i16) shk) &. mk_i16 1 in
  assert (forall (nth: usize {v nth < bits i16_inttype}).
            v nth > 1 ==> Rust_primitives.Integers.get_bit lane nth == 0);
  Rust_primitives.BitVectors.lemma_get_bit_bounded' lane 1

(* Per-lane value of one half of the deserialize_1 result vector, in clean
   context: lane j of (vandq (vshlq (vdupq pre) shift) one) is
   (arm_sshl_i16 pre shift[j]) &. 1.  Factored out so the consumer's per-bit
   forall does not re-instantiate the four intrinsic lane foralls each time. *)
let lemma_deser1_half_lane
      (pre: i16) (shift one: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (shifter: t_Array i16 (mk_usize 8)) (j: nat{j < 8})
    : Lemma
      (requires
        (forall (m: nat{m < 8}). Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 one m == mk_i16 1) /\
        (forall (m: nat{m < 8}). Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift m == Seq.index shifter m))
      (ensures
        (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8
          (Libcrux_intrinsics.Arm64_extract.e_vandq_s16
             (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16
                (Libcrux_intrinsics.Arm64_extract.e_vdupq_n_s16 pre) shift) one) j <: i16)
        == (((Libcrux_intrinsics.Arm64_extract.arm_sshl_i16 pre (Seq.index shifter j)) &. mk_i16 1) <: i16)) =
  ()
"#
)]
#[hax_lib::fstar::before(
    r#"
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let rec ser1_bitsum (c: nat -> nat) (d: nat) : Tot nat (decreases d) =
  if d = 0 then 0 else ser1_bitsum c (d - 1) + c (d - 1) * pow2 (d - 1)

let rec lemma_ser1_bitsum_bound (c: nat -> nat) (d: nat)
    : Lemma (requires forall (j: nat). j < d ==> c j < 2)
            (ensures ser1_bitsum c d < pow2 d)
            (decreases d) =
  if d = 0 then ()
  else (lemma_ser1_bitsum_bound c (d - 1); FStar.Math.Lemmas.pow2_double_sum (d - 1))

let rec lemma_ser1_bitsum_bit (c: nat -> nat) (d: nat) (k: nat{k < d})
    : Lemma (requires forall (j: nat). j < d ==> c j < 2)
            (ensures Rust_primitives.Integers.get_bit_nat (ser1_bitsum c d) k == c k)
            (decreases d) =
  lemma_ser1_bitsum_bound c (d - 1);
  let lo = ser1_bitsum c (d - 1) in
  let hi = c (d - 1) in
  if k = d - 1 then begin
    FStar.Math.Lemmas.lemma_div_plus lo hi (pow2 (d - 1));
    FStar.Math.Lemmas.small_div lo (pow2 (d - 1))
  end
  else begin
    lemma_ser1_bitsum_bit c (d - 1) k;
    FStar.Math.Lemmas.pow2_plus k (d - 1 - k);
    assert (hi * pow2 (d - 1) == (hi * pow2 (d - 1 - k)) * pow2 k);
    FStar.Math.Lemmas.lemma_div_plus lo (hi * pow2 (d - 1 - k)) (pow2 k);
    FStar.Math.Lemmas.pow2_plus 1 (d - 2 - k);
    assert (hi * pow2 (d - 1 - k) == (hi * pow2 (d - 2 - k)) * 2);
    FStar.Math.Lemmas.lemma_mod_plus (lo / pow2 k) (hi * pow2 (d - 2 - k)) 2
  end
#pop-options

(* One lane: shifting a 1-bit value c left by s (0<=s<8) yields c * 2^s. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ser1_shift_lane (c shk: i16) (s: nat{s < 8})
    : Lemma
      (requires Rust_primitives.BitVectors.bounded c 1 /\ v (shk %! mk_i16 256) == s)
      (ensures v (Libcrux_intrinsics.Arm64_extract.arm_sshl_i16 c shk) == (v c) * pow2 s) =
  FStar.Math.Lemmas.pow2_le_compat 7 s
#pop-options

(* reveal the opaque `get_bit` for a nonnegative integer. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_get_bit_val (#t: inttype) (x: int_t t) (i: usize{v i < bits t})
    : Lemma (requires v x >= 0)
            (ensures Rust_primitives.Integers.get_bit x i == Rust_primitives.Integers.get_bit_nat (v x) (v i)) =
  reveal_opaque (`%Rust_primitives.Integers.get_bit) (Rust_primitives.Integers.get_bit #t)
#pop-options

(* ser1_bitsum at d=8 is the flat weighted sum (pure nat, clean context). *)
#push-options "--fuel 9 --ifuel 1 --z3rlimit 100"
let lemma_ser1_bitsum_flat (c: nat -> nat)
    : Lemma
      (ensures
        ser1_bitsum c 8
        == c 0 * 1 + c 1 * 2 + c 2 * 4 + c 3 * 8 + c 4 * 16 + c 5 * 32 + c 6 * 64 + c 7 * 128) =
  ()
#pop-options

(* Value of one packed byte: vaddvq of vshlq(half, [0..7]) equals the no-carry
   binary (weighted) sum of the 8 one-bit lanes. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_ser1_half (half shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
    : Lemma
      (requires
        (forall (j: nat{j < 8}).
            Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half j) 1) /\
        (forall (j: nat{j < 8}).
            v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j))
      (ensures
        v (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
              (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 half shift))
        == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 0)) * 1
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 1)) * 2
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 2)) * 4
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 3)) * 8
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 4)) * 16
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 5)) * 32
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 6)) * 64
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 7)) * 128) =
  let sh = Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 half shift in
  let aux (j: nat{j < 8})
      : Lemma
        (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh j)
          == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half j)) * pow2 j) =
    lemma_ser1_shift_lane (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half j)
      (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) j
  in
  Classical.forall_intro aux;
  assert_norm (pow2 0 == 1 /\ pow2 1 == 2 /\ pow2 2 == 4 /\ pow2 3 == 8 /\
               pow2 4 == 16 /\ pow2 5 == 32 /\ pow2 6 == 64 /\ pow2 7 == 128);
  let l0 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 0 in
  let l1 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 1 in
  let l2 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 2 in
  let l3 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 3 in
  let l4 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 4 in
  let l5 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 5 in
  let l6 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 6 in
  let l7 = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 sh 7 in
  (* lane values + small bounds (each lane < 256, so the nested adds do not wrap) *)
  assert (v l0 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 0)) * 1 /\ v l0 <= 1);
  assert (v l1 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 1)) * 2 /\ v l1 <= 2);
  assert (v l2 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 2)) * 4 /\ v l2 <= 4);
  assert (v l3 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 3)) * 8 /\ v l3 <= 8);
  assert (v l4 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 4)) * 16 /\ v l4 <= 16);
  assert (v l5 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 5)) * 32 /\ v l5 <= 32);
  assert (v l6 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 6)) * 64 /\ v l6 <= 64);
  assert (v l7 == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half 7)) * 128 /\ v l7 <= 128);
  (* nested vaddvq sum is exact (no i16 wrap) *)
  assert (v (l0 +. l1) == v l0 + v l1);
  assert (v (l2 +. l3) == v l2 + v l3);
  assert (v (l4 +. l5) == v l4 + v l5);
  assert (v (l6 +. l7) == v l6 + v l7);
  assert (v ((l0 +. l1) +. (l2 +. l3)) == v l0 + v l1 + v l2 + v l3);
  assert (v ((l4 +. l5) +. (l6 +. l7)) == v l4 + v l5 + v l6 + v l7);
  assert (v (((l0 +. l1) +. (l2 +. l3)) +. ((l4 +. l5) +. (l6 +. l7)))
          == v l0 + v l1 + v l2 + v l3 + v l4 + v l5 + v l6 + v l7)
#pop-options

(* Bit k of the packed byte (cast of the vaddvq sum) equals bit 0 of lane k. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_ser1_byte (half shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t) (byte: u8) (k: nat{k < 8})
    : Lemma
      (requires
        (forall (j: nat{j < 8}).
            Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half j) 1) /\
        (forall (j: nat{j < 8}).
            v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j) /\
        byte == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                        (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 half shift)) <: u8))
      (ensures
        Rust_primitives.Integers.get_bit byte (sz k)
        == Rust_primitives.Integers.get_bit
             (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half k) (mk_usize 0)) =
  let c:(nat -> nat) =
    (fun (j: nat) ->
        if j < 8
        then (let x = v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half j) in
              if x >= 0 then x else 0)
        else 0)
  in
  lemma_ser1_half half shift;
  lemma_ser1_bitsum_flat c;
  lemma_ser1_bitsum_bound c 8;
  lemma_ser1_bitsum_bit c 8 k;
  let s = Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
            (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 half shift)
  in
  Rust_primitives.Integers.get_bit_cast #i16_inttype #u8_inttype s (sz k);
  lemma_get_bit_val #i16_inttype s (sz k);
  lemma_get_bit_val #i16_inttype (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half k) (mk_usize 0)
#pop-options

(* Both 8-lane bound foralls, derived once from serialize_pre_N in a CLEAN
   context (no cast/shift terms). The f_high half uses the +8 Seq.append
   offset (app2), fragile to derive amid the cast soup. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser1_bounded_halves (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    : Lemma
      (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (repr vec))
      (ensures
        (forall (j: nat{j < 8}).
            Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low j) 1) /\
        (forall (j: nat{j < 8}).
            Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high j) 1)) =
  assert (forall (j: nat{j < 8}).
        Rust_primitives.BitVectors.bounded (Seq.index (repr vec) j) 1 /\
        Rust_primitives.BitVectors.bounded (Seq.index (repr vec) (j + 8)) 1)
#pop-options

(* The i/8, i%8 euclidean facts, proven in a CLEAN context (only the bound
   on i, no cast soup): deriving i%8==i-8 from lemma_div_mod is flaky amid
   lemma_ser1_bit's heavy hypotheses, so isolate it. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_idx8 (i: nat{i < 16})
    : Lemma
      (ensures
        (i < 8 ==> (i / 8 == 0 /\ i % 8 == i)) /\
        (i >= 8 ==> (i / 8 == 1 /\ i % 8 == i - 8))) =
  if i < 8 then (FStar.Math.Lemmas.small_div i 8; FStar.Math.Lemmas.small_mod i 8)
  else FStar.Math.Lemmas.lemma_div_mod i 8
#pop-options

(* Per-coefficient bit equality for SYMBOLIC i (AVX2-style): an internal
   if i<8 split selects the low/high half; the i/8, i%8 reductions come from
   the clean lemma_idx8, and the repr-append index SMTPat bridges
   (repr vec).[i] to the lane.  One lemma VC -> the dispatcher is a single
   introduce-forall call (no 16-way match, no monolithic saturation). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ser1_bit (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (result: t_Array u8 (mk_usize 2))
      (i: nat{i < 16})
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (repr vec) /\
        (forall (j: nat{j < 8}).
            v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j) /\
        Seq.index result 0
          == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                     (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low shift)) <: u8) /\
        Seq.index result 1
          == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                     (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high shift)) <: u8))
      (ensures
        Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
        == Rust_primitives.Integers.get_bit (Seq.index (repr vec) i) (mk_usize 0)) =
  lemma_ser1_bounded_halves vec;
  lemma_idx8 i;
  if i < 8 then
    lemma_ser1_byte vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low shift (Seq.index result 0) i
  else
    lemma_ser1_byte vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high shift (Seq.index result 1) (i - 8)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ser1_bits
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (result: t_Array u8 (mk_usize 2))
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (repr vec) /\
        (forall (j: nat{j < 8}).
            v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j) /\
        Seq.index result 0
          == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                     (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low shift)) <: u8) /\
        Seq.index result 1
          == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                     (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high shift)) <: u8))
      (ensures
        forall (i: nat{i < 16}).
          Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit (Seq.index (repr vec) i) (mk_usize 0)) =
  introduce forall (i: nat{i < 16}).
      Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
      == Rust_primitives.Integers.get_bit (Seq.index (repr vec) i) (mk_usize 0)
  with lemma_ser1_bit vec shift result i
#pop-options

(* BitVec bridge, proven in a CLEAN context (only the per-coefficient
   get_bit forall): the on-applied bit_vec_of_int_t_array unfolding +
   bit_vec_equal_intro are fragile amid lemma_ser1_bits's repr-append forall
   and the cast/shift result-defs, so isolate them here. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_bv_eq_from_bits
      (arr: t_Array i16 (mk_usize 16))
      (result: t_Array u8 (mk_usize 2))
    : Lemma
      (requires
        forall (i: nat{i < 16}).
          Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit (Seq.index arr i) (mk_usize 0))
      (ensures
        BitVecEq.int_t_array_bitwise_eq #i16_inttype #u8_inttype #(mk_usize 16) #(mk_usize 2)
          arr 1 result 8) =
  introduce forall (i: nat{i < 16}).
      Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr 1 i
      == Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 2) result 8 i
  with (assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr 1 i
            == Rust_primitives.Integers.get_bit (Seq.index arr i) (mk_usize 0));
        assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 2) result 8 i
            == Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))));
  BitVecEq.bit_vec_equal_intro
    (Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr 1)
    (BitVecEq.retype (Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 2) result 8))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_serialize_1_post
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (result: t_Array u8 (mk_usize 2))
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (repr vec) /\
        (forall (j: nat{j < 8}).
            v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j) /\
        Seq.index result 0
          == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                     (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16
                        vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low shift)) <: u8) /\
        Seq.index result 1
          == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddvq_s16
                     (Libcrux_intrinsics.Arm64_extract.e_vshlq_s16
                        vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high shift)) <: u8))
      (ensures Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (repr vec) result) =
  lemma_ser1_bits vec shift result;
  lemma_bv_eq_from_bits (repr vec) result
#pop-options

(* The 8 shift-amount facts from the concrete shifter, proven in a CLEAN
   context (only the e_vld1q lane relationship + the 8 ground shifter
   values): the 8-way match enumeration saturates if done inline in the
   serialize_1_ body amid the array_of_list / vaddvq / cast machinery. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ser1_shift_amounts (shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (shifter: t_Array i16 (mk_usize 8))
    : Lemma
      (requires
        (forall (j: nat{j < 8}).
            Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j == Seq.index shifter j) /\
        Seq.index shifter 0 == mk_i16 0 /\ Seq.index shifter 1 == mk_i16 1 /\
        Seq.index shifter 2 == mk_i16 2 /\ Seq.index shifter 3 == mk_i16 3 /\
        Seq.index shifter 4 == mk_i16 4 /\ Seq.index shifter 5 == mk_i16 5 /\
        Seq.index shifter 6 == mk_i16 6 /\ Seq.index shifter 7 == mk_i16 7)
      (ensures
        forall (j: nat{j < 8}).
          v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j) =
  introduce forall (j: nat{j < 8}).
      v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == j
  with (match j with
        | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | _ -> ())
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_1(v: SIMD128Vector) -> [u8; 2] {
    let shifter: [i16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let shift = _vld1q_s16(&shifter);
    let low = _vshlq_s16(v.low, shift);
    let high = _vshlq_s16(v.high, shift);
    let low = _vaddvq_s16(low);
    let high = _vaddvq_s16(high);
    let result = [low as u8, high as u8];
    hax_lib::fstar!(
        r#"(* shift-amount facts from the concrete shifter (lane j shifts by j) *)
assert (forall (j: nat{j < 8}).
      Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${shift} j == Seq.index ${shifter} j);
assert (Seq.index ${shifter} 0 == mk_i16 0 /\ Seq.index ${shifter} 1 == mk_i16 1 /\
        Seq.index ${shifter} 2 == mk_i16 2 /\ Seq.index ${shifter} 3 == mk_i16 3 /\
        Seq.index ${shifter} 4 == mk_i16 4 /\ Seq.index ${shifter} 5 == mk_i16 5 /\
        Seq.index ${shifter} 6 == mk_i16 6 /\ Seq.index ${shifter} 7 == mk_i16 7);
lemma_ser1_shift_amounts ${shift} ${shifter};
assert (Seq.index ${result} 0 == (cast (${low} <: i16) <: u8));
assert (Seq.index ${result} 1 == (cast (${high} <: i16) <: u8));
lemma_serialize_1_post ${v} ${shift} ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(a.len() == 2)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_1_post} ${a} (repr ${result})"#))]
pub(crate) fn deserialize_1(a: &[u8]) -> SIMD128Vector {
    let one = _vdupq_n_s16(1);
    let low = _vdupq_n_s16(a[0] as i16);
    let high = _vdupq_n_s16(a[1] as i16);
    let shifter: [i16; 8] = [0, 0xff, -2, -3, -4, -5, -6, -7];
    let shift = _vld1q_s16(&shifter);
    let low = _vshlq_s16(low, shift);
    let high = _vshlq_s16(high, shift);
    let result = SIMD128Vector {
        low: _vandq_s16(low, one),
        high: _vandq_s16(high, one),
    };
    hax_lib::fstar!(
        r#"
let rr : t_Array i16 (mk_usize 16) = Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result} in
let pre0 : i16 = cast (${a}.[ mk_usize 0 ] <: u8) <: i16 in
let pre1 : i16 = cast (${a}.[ mk_usize 1 ] <: u8) <: i16 in
assert (forall (m: nat{m < 8}). Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${one} m == mk_i16 1);
assert (forall (m: nat{m < 8}).
          Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${shift} m == Seq.index ${shifter} m);
let aux (i: nat{i < 16}) : Lemma
  (Rust_primitives.Integers.get_bit (Seq.index rr i) (mk_usize 0)
     == Rust_primitives.Integers.get_bit (Seq.index ${a} (i / 8)) (sz (i % 8))
   /\ Rust_primitives.BitVectors.bounded (Seq.index rr i) 1) =
  let k : nat = i % 8 in
  let byte = if i < 8 then (${a}.[ mk_usize 0 ] <: u8) else (${a}.[ mk_usize 1 ] <: u8) in
  let pre_lane : i16 = if i < 8 then pre0 else pre1 in
  let shk = Seq.index ${shifter} k in
  (match k with
   | 0 -> assert (v (shk %! mk_i16 256) == 0)
   | 1 -> assert (v (shk %! mk_i16 256) == 255)
   | 2 -> assert (v (shk %! mk_i16 256) == 254)
   | 3 -> assert (v (shk %! mk_i16 256) == 253)
   | 4 -> assert (v (shk %! mk_i16 256) == 252)
   | 5 -> assert (v (shk %! mk_i16 256) == 251)
   | 6 -> assert (v (shk %! mk_i16 256) == 250)
   | _ -> assert (v (shk %! mk_i16 256) == 249));
  assert (v (shk %! mk_i16 256) == (if k = 0 then 0 else 256 - k));
  (if i < 8
   then lemma_deser1_half_lane pre0 ${shift} ${one} ${shifter} k
   else lemma_deser1_half_lane pre1 ${shift} ${one} ${shifter} k);
  assert (Seq.index rr i
          == (Libcrux_intrinsics.Arm64_extract.arm_sshl_i16 pre_lane shk &. mk_i16 1));
  assert (byte == Seq.index ${a} (i / 8));
  assert (k == i % 8);
  assert (cast byte == pre_lane);
  lemma_deser1_lane byte k shk
in
Classical.forall_intro aux;
let bv_in : Rust_primitives.BitVectors.bit_vec 16 =
  Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 2) (${a} <: t_Array u8 (mk_usize 2)) 8 in
let bv_out : Rust_primitives.BitVectors.bit_vec 16 =
  Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) rr 1 in
introduce forall (i: nat{i < 16}). bv_in i == bv_out i
with (assert (bv_in i == Rust_primitives.Integers.get_bit (Seq.index ${a} (i / 8)) (sz (i % 8)));
      assert (bv_out i == Rust_primitives.Integers.get_bit (Seq.index rr i) (mk_usize 0)));
BitVecEq.bit_vec_equal_intro bv_in (BitVecEq.retype bv_out)
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"(* ===================== serialize_4 proof =====================
   Neon serialize_4_ packs 16 4-bit coefficients into 8 bytes:
   vshlq_u16 (shifter [0;4;8;12;0;4;8;12]) then vaddv_u16 (4-lane horiz add) x4
   -> four u16 group sums -> u64 (|<<16|<<32|<<48) -> to_le_bytes.
   Each group sum g (g<4) = Sum_{m<4} coeff_{4g+m} * 2^{4m}, and the whole u64
   is Sum_{c<16} coeff_c * 2^{4c} (base-16); byte b holds coeffs 2b, 2b+1. *)

(* TRUSTED AXIOM modeling Core_models.Num.impl_u64__to_le_bytes (a bare
   `assume val` in hax-lib core proof-libs with NO functional ensures): byte b
   of the little-endian encoding is (x / 2^(8b)) mod 2^8.  VALIDATED bit-exact
   vs Rust std u64::to_le_bytes (24,000,072 checks, 0 fails):
   ~/hax-fstar-mcp/libcrux-notes/agent-status/u64_to_le_bytes_validate-2026-06-23.rs *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
assume
val lemma_u64_to_le_bytes_index (x: u64) (b: nat{b < 8})
    : Lemma
      (ensures
        v (Seq.index (Core_models.Num.impl_u64__to_le_bytes x) b)
        == (v x / pow2 (8 * b)) % pow2 8)
#pop-options

(* Bit p of byte b of to_le_bytes(x) is bit (8b+p) of x. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_byte_extract_bit (n: nat) (b: nat) (p: nat{p < 8})
    : Lemma
      (ensures
        Rust_primitives.Integers.get_bit_nat ((n / pow2 (8 * b)) % pow2 8) p
        == Rust_primitives.Integers.get_bit_nat n (8 * b + p)) =
  let q = n / pow2 (8 * b) in
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 q p 8;
  FStar.Math.Lemmas.pow2_modulo_modulo_lemma_1 (q / pow2 p) 1 (8 - p);
  FStar.Math.Lemmas.division_multiplication_lemma n (pow2 (8 * b)) (pow2 p);
  FStar.Math.Lemmas.pow2_plus (8 * b) p
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_to_le_bytes_bit (x: u64) (b: nat{b < 8}) (p: nat{p < 8})
    : Lemma
      (ensures
        Rust_primitives.Integers.get_bit (Seq.index (Core_models.Num.impl_u64__to_le_bytes x) b) (sz p)
        == Rust_primitives.Integers.get_bit x (sz (8 * b + p))) =
  let result = Core_models.Num.impl_u64__to_le_bytes x in
  let byte = Seq.index result b in
  lemma_u64_to_le_bytes_index x b;
  lemma_get_bit_val #u8_inttype byte (sz p);
  lemma_get_bit_val #u64_inttype x (sz (8 * b + p));
  lemma_byte_extract_bit (v x) b p
#pop-options

(* Base-16 digit sum (analog of ser1_bitsum at base 2^4). *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let rec ser4_nibsum (c: nat -> nat) (d: nat) : Tot nat (decreases d) =
  if d = 0 then 0 else ser4_nibsum c (d - 1) + c (d - 1) * pow2 (4 * (d - 1))

let rec lemma_ser4_nibsum_bound (c: nat -> nat) (d: nat)
    : Lemma (requires forall (j: nat). j < d ==> c j < 16)
            (ensures ser4_nibsum c d < pow2 (4 * d))
            (decreases d) =
  if d = 0 then ()
  else begin
    lemma_ser4_nibsum_bound c (d - 1);
    assert_norm (pow2 4 == 16);
    FStar.Math.Lemmas.pow2_plus 4 (4 * (d - 1))
  end
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let rec lemma_ser4_nibsum_bit (c: nat -> nat) (d: nat) (k: nat{k < 4 * d})
    : Lemma (requires forall (j: nat). j < d ==> c j < 16)
            (ensures
              Rust_primitives.Integers.get_bit_nat (ser4_nibsum c d) k
              == Rust_primitives.Integers.get_bit_nat (c (k / 4)) (k % 4))
            (decreases d) =
  lemma_ser4_nibsum_bound c (d - 1);
  let lo = ser4_nibsum c (d - 1) in
  let hi = c (d - 1) in
  let e = 4 * (d - 1) in
  if k >= e then begin
    FStar.Math.Lemmas.lemma_div_plus (k - e) (d - 1) 4;
    FStar.Math.Lemmas.small_div (k - e) 4;
    FStar.Math.Lemmas.lemma_mod_plus (k - e) (d - 1) 4;
    FStar.Math.Lemmas.small_mod (k - e) 4;
    FStar.Math.Lemmas.pow2_plus e (k - e);
    FStar.Math.Lemmas.division_multiplication_lemma (lo + hi * pow2 e) (pow2 e) (pow2 (k - e));
    FStar.Math.Lemmas.lemma_div_plus lo hi (pow2 e);
    FStar.Math.Lemmas.small_div lo (pow2 e)
  end
  else begin
    lemma_ser4_nibsum_bit c (d - 1) k;
    FStar.Math.Lemmas.pow2_plus k (e - k);
    assert (hi * pow2 e == (hi * pow2 (e - k)) * pow2 k);
    FStar.Math.Lemmas.lemma_div_plus lo (hi * pow2 (e - k)) (pow2 k);
    FStar.Math.Lemmas.pow2_plus 1 (e - k - 1);
    assert (hi * pow2 (e - k) == (hi * pow2 (e - k - 1)) * 2);
    FStar.Math.Lemmas.lemma_mod_plus (lo / pow2 k) (hi * pow2 (e - k - 1)) 2
  end
#pop-options

#push-options "--fuel 5 --ifuel 1 --z3rlimit 100"
let lemma_ser4_nibsum_flat (c: nat -> nat)
    : Lemma (ensures ser4_nibsum c 4 == c 0 * 1 + c 1 * 16 + c 2 * 256 + c 3 * 4096) =
  assert_norm (pow2 0 == 1 /\ pow2 4 == 16 /\ pow2 8 == 256 /\ pow2 12 == 4096)
#pop-options

(* Bit m of a 4-nibble base-16 byte-pair sum picks bit (m%4) of nibble (m/4). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_nib4_bit (n0 n1 n2 n3: nat) (m: nat{m < 16})
    : Lemma
      (requires n0 < 16 /\ n1 < 16 /\ n2 < 16 /\ n3 < 16)
      (ensures
        Rust_primitives.Integers.get_bit_nat (n0 * 1 + n1 * 16 + n2 * 256 + n3 * 4096) m
        == Rust_primitives.Integers.get_bit_nat (if m < 4 then n0 else if m < 8 then n1 else if m < 12 then n2 else n3)
             (m % 4)) =
  let c:(nat -> nat) =
    (fun (j: nat) -> if j = 0 then n0 else if j = 1 then n1 else if j = 2 then n2 else if j = 3 then n3 else 0)
  in
  lemma_ser4_nibsum_flat c;
  lemma_ser4_nibsum_bit c 4 m;
  assert (m / 4 == (if m < 4 then 0 else if m < 8 then 1 else if m < 12 then 2 else 3))
#pop-options

(* Per-lane value: reinterpret coeff (bounded 4) to u16 then shift left by
   the shifter amount 4*(j%4) gives coeff * 2^{4*(j%4)} (no u16 wrap). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_ser4_lane (vsrc shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t) (j: nat{j < 8})
    : Lemma
      (requires
        Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) 4 /\
        v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4))
      (ensures
        v (Libcrux_intrinsics.Arm64_extract.get_lane_u16x8
              (Libcrux_intrinsics.Arm64_extract.e_vshlq_u16
                 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift) j)
        == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j)) * pow2 (4 * (j % 4))) =
  let coeff = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j in
  let rv = Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc in
  let u16coeff = Libcrux_intrinsics.Arm64_extract.get_lane_u16x8 rv j in
  let s = 4 * (j % 4) in
  assert (u16coeff == Rust_primitives.Integers.cast_mod #i16_inttype #u16_inttype coeff);
  FStar.Math.Lemmas.small_mod (v coeff) (pow2 16);
  assert (v u16coeff == v coeff);
  assert (v coeff < pow2 4);
  FStar.Math.Lemmas.pow2_le_compat 12 s;
  FStar.Math.Lemmas.pow2_plus 4 12;
  FStar.Math.Lemmas.lemma_mult_lt_right (pow2 s) (v coeff) (pow2 4);
  FStar.Math.Lemmas.lemma_mult_le_left (pow2 4) (pow2 s) (pow2 12);
  FStar.Math.Lemmas.small_mod ((v coeff) * pow2 s) (pow2 16)
#pop-options

(* vaddv_u16 over 4 lanes holding (n0, n1*16, n2*256, n3*4096) with each
   n<16 is the exact flat sum (no u16 wrap; max = 15*4369 = 65535). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_vaddv (a: Libcrux_intrinsics.Arm64_extract.t_e_uint16x4_t) (n0 n1 n2 n3: nat)
    : Lemma
      (requires
        n0 < 16 /\ n1 < 16 /\ n2 < 16 /\ n3 < 16 /\
        v (Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 0) == n0 * 1 /\
        v (Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 1) == n1 * 16 /\
        v (Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 2) == n2 * 256 /\
        v (Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 3) == n3 * 4096)
      (ensures
        v (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16 a)
        == n0 * 1 + n1 * 16 + n2 * 256 + n3 * 4096) =
  let l0 = Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 0 in
  let l1 = Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 1 in
  let l2 = Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 2 in
  let l3 = Libcrux_intrinsics.Arm64_extract.get_lane_u16x4 a 3 in
  assert (v (l0 +. l1) == v l0 + v l1);
  assert (v (l2 +. l3) == v l2 + v l3);
  assert (v ((l0 +. l1) +. (l2 +. l3)) == v l0 + v l1 + v l2 + v l3)
#pop-options

(* Low half (lanes 0-3) group sum value. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_ser4_low_sum (vsrc shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
    : Lemma
      (requires
        (forall (j: nat{j < 8}). Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) 4) /\
        (forall (j: nat{j < 8}). v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)))
      (ensures
        v (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16
              (Libcrux_intrinsics.Arm64_extract.e_vget_low_u16
                 (Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift)))
        == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 0)) * 1
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 1)) * 16
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 2)) * 256
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 3)) * 4096) =
  let lowhi = Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift in
  let a = Libcrux_intrinsics.Arm64_extract.e_vget_low_u16 lowhi in
  assert_norm (pow2 0 == 1 /\ pow2 4 == 16 /\ pow2 8 == 256 /\ pow2 12 == 4096);
  lemma_ser4_lane vsrc shift 0;
  lemma_ser4_lane vsrc shift 1;
  lemma_ser4_lane vsrc shift 2;
  lemma_ser4_lane vsrc shift 3;
  lemma_ser4_vaddv a
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 0))
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 1))
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 2))
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 3))
#pop-options

(* High half (lanes 4-7) group sum value. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_ser4_high_sum (vsrc shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
    : Lemma
      (requires
        (forall (j: nat{j < 8}). Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) 4) /\
        (forall (j: nat{j < 8}). v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)))
      (ensures
        v (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16
              (Libcrux_intrinsics.Arm64_extract.e_vget_high_u16
                 (Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift)))
        == (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 4)) * 1
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 5)) * 16
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 6)) * 256
         + (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 7)) * 4096) =
  let lowhi = Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift in
  let a = Libcrux_intrinsics.Arm64_extract.e_vget_high_u16 lowhi in
  assert_norm (pow2 0 == 1 /\ pow2 4 == 16 /\ pow2 8 == 256 /\ pow2 12 == 4096);
  lemma_ser4_lane vsrc shift 4;
  lemma_ser4_lane vsrc shift 5;
  lemma_ser4_lane vsrc shift 6;
  lemma_ser4_lane vsrc shift 7;
  lemma_ser4_vaddv a
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 4))
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 5))
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 6))
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc 7))
#pop-options

(* The 8 shift-amount facts from the concrete shifter [0;4;8;12;0;4;8;12]. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ser4_shift_amounts (shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (shifter: t_Array i16 (mk_usize 8))
    : Lemma
      (requires
        (forall (j: nat{j < 8}).
            Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j == Seq.index shifter j) /\
        Seq.index shifter 0 == mk_i16 0 /\ Seq.index shifter 1 == mk_i16 4 /\
        Seq.index shifter 2 == mk_i16 8 /\ Seq.index shifter 3 == mk_i16 12 /\
        Seq.index shifter 4 == mk_i16 0 /\ Seq.index shifter 5 == mk_i16 4 /\
        Seq.index shifter 6 == mk_i16 8 /\ Seq.index shifter 7 == mk_i16 12)
      (ensures
        forall (j: nat{j < 8}).
          v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)) =
  introduce forall (j: nat{j < 8}).
      v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)
  with (match j with
        | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | _ -> ())
#pop-options

(* Per-lane 4-bit bounds for both halves, from serialize_pre_N 4. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_bounded (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    : Lemma
      (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr vec))
      (ensures
        (forall (j: nat{j < 8}).
            Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low j) 4) /\
        (forall (j: nat{j < 8}).
            Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high j) 4)) =
  assert (forall (j: nat{j < 8}).
        Rust_primitives.BitVectors.bounded (Seq.index (repr vec) j) 4 /\
        Rust_primitives.BitVectors.bounded (Seq.index (repr vec) (j + 8)) 4)
#pop-options

(* Euclidean facts for a global bit i < 64 (clean context). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_idx_ser4 (i: nat{i < 64})
    : Lemma
      (ensures
        8 * (i / 8) + i % 8 == i /\ i / 8 < 8 /\ i % 8 < 8 /\
        16 * (i / 16) + i % 16 == i /\ i / 16 < 4 /\ i % 16 < 16 /\
        i / 4 < 16 /\ i % 4 < 4 /\ i % 16 / 4 < 4 /\
        4 * (i / 16) + (i % 16) / 4 == i / 4 /\
        (i % 16) % 4 == i % 4) =
  FStar.Math.Lemmas.lemma_div_mod i 8;
  FStar.Math.Lemmas.lemma_div_mod i 16;
  FStar.Math.Lemmas.lemma_div_plus (i % 16) (4 * (i / 16)) 4;
  FStar.Math.Lemmas.lemma_mod_plus (i % 16) (4 * (i / 16)) 4
#pop-options

(* The packed u64 sum (sum0 | sum1<<16 | sum2<<32 | sum3<<48) bit i is the
   corresponding within-group bit of sum_{i/16} (groups are disjoint 16-bit
   ranges; each sum_g < 2^16). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_pack_bit (sum0 sum1 sum2 sum3 sum: u64) (i: nat{i < 64})
    : Lemma
      (requires
        v sum0 < pow2 16 /\ v sum1 < pow2 16 /\ v sum2 < pow2 16 /\ v sum3 < pow2 16 /\
        sum ==
        (((sum0 |. (sum1 <<! mk_i32 16 <: u64) <: u64) |. (sum2 <<! mk_i32 32 <: u64) <: u64) |.
          (sum3 <<! mk_i32 48 <: u64)))
      (ensures
        Rust_primitives.Integers.get_bit sum (sz i) ==
        (if i < 16 then Rust_primitives.Integers.get_bit sum0 (sz i)
         else if i < 32 then Rust_primitives.Integers.get_bit sum1 (sz (i - 16))
         else if i < 48 then Rust_primitives.Integers.get_bit sum2 (sz (i - 32))
         else Rust_primitives.Integers.get_bit sum3 (sz (i - 48)))) =
  assert (Rust_primitives.BitVectors.bounded sum0 16);
  assert (Rust_primitives.BitVectors.bounded sum1 16);
  assert (Rust_primitives.BitVectors.bounded sum2 16);
  assert (Rust_primitives.BitVectors.bounded sum3 16)
#pop-options

(* serialize_pre_N 4 -> per-coeff 4-bit bounds, proven in a CLEAN context. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_ser4_prebounds (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    : Lemma
      (requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr vec))
      (ensures forall (k: nat{k < 16}). Rust_primitives.BitVectors.bounded (Seq.index (repr vec) k) 4) =
  ()
#pop-options

(* CLEAN per-group bit fact: bit m of group sum sg (= base-16 sum of its 4
   coeffs) is bit (m%4) of coeff (4g + m/4).  Only nibsum machinery, no OR/
   to_le_bytes terms -> small context. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_group_bit
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (sg: u64) (g: nat{g < 4}) (m: nat{m < 16})
    : Lemma
      (requires
        (forall (k: nat{k < 16}). Rust_primitives.BitVectors.bounded (Seq.index (repr vec) k) 4) /\
        v sg == (v (Seq.index (repr vec) (4 * g))) * 1 + (v (Seq.index (repr vec) (4 * g + 1))) * 16
              + (v (Seq.index (repr vec) (4 * g + 2))) * 256 + (v (Seq.index (repr vec) (4 * g + 3))) * 4096)
      (ensures
        Rust_primitives.Integers.get_bit sg (sz m)
        == Rust_primitives.Integers.get_bit (Seq.index (repr vec) (4 * g + m / 4)) (sz (m % 4))) =
  assert_norm (pow2 4 == 16);
  FStar.Math.Lemmas.lemma_div_mod m 4;
  let c:(nat -> nat) = (fun (j: nat) -> if j < 4 then v (Seq.index (repr vec) (4 * g + j)) else 0) in
  lemma_ser4_nibsum_flat c;
  lemma_ser4_nibsum_bit c 4 m;
  lemma_get_bit_val #u64_inttype sg (sz m);
  lemma_get_bit_val #i16_inttype (Seq.index (repr vec) (4 * g + m / 4)) (sz (m % 4))
#pop-options

(* CLEAN result-byte to group-bit fact: byte bit (i%8) of byte (i/8) of
   to_le_bytes(packed sum) is bit (i%16) of group sum_{i/16}.  Only OR/shift/
   to_le_bytes machinery, no nibsum terms -> small context. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_ser4_result_bit
      (sum0 sum1 sum2 sum3 sum: u64)
      (result: t_Array u8 (mk_usize 8))
      (i: nat{i < 64})
    : Lemma
      (requires
        v sum0 < pow2 16 /\ v sum1 < pow2 16 /\ v sum2 < pow2 16 /\ v sum3 < pow2 16 /\
        sum ==
        (((sum0 |. (sum1 <<! mk_i32 16 <: u64) <: u64) |. (sum2 <<! mk_i32 32 <: u64) <: u64) |.
          (sum3 <<! mk_i32 48 <: u64)) /\
        result == Core_models.Num.impl_u64__to_le_bytes sum)
      (ensures
        Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8)) ==
        (if i < 16 then Rust_primitives.Integers.get_bit sum0 (sz (i % 16))
         else if i < 32 then Rust_primitives.Integers.get_bit sum1 (sz (i % 16))
         else if i < 48 then Rust_primitives.Integers.get_bit sum2 (sz (i % 16))
         else Rust_primitives.Integers.get_bit sum3 (sz (i % 16)))) =
  lemma_idx_ser4 i;
  lemma_to_le_bytes_bit sum (i / 8) (i % 8);
  lemma_pack_bit sum0 sum1 sum2 sum3 sum i
#pop-options

(* Per-group chainer with MINIMAL context: takes the already-collapsed
   result-byte fact (get_bit result == get_bit sg) as a hypothesis, so no
   OR/to_le_bytes/if-ladder machinery enters; just chains group_bit. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_ser4_bit_grp
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (sg: u64)
      (result: t_Array u8 (mk_usize 8))
      (i: nat{i < 64})
      (g: nat{g < 4})
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr vec) /\
        16 * g <= i /\ i < 16 * g + 16 /\
        v sg == (v (Seq.index (repr vec) (4 * g))) * 1 + (v (Seq.index (repr vec) (4 * g + 1))) * 16
              + (v (Seq.index (repr vec) (4 * g + 2))) * 256 + (v (Seq.index (repr vec) (4 * g + 3))) * 4096 /\
        Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
        == Rust_primitives.Integers.get_bit sg (sz (i % 16)))
      (ensures
        Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
        == Rust_primitives.Integers.get_bit (Seq.index (repr vec) (i / 4)) (sz (i % 4))) =
  lemma_idx_ser4 i;
  lemma_ser4_prebounds vec;
  lemma_ser4_group_bit vec sg g (i % 16)
#pop-options

(* Master per-bit equality (symbolic i): concrete 4-way dispatch.  In each
   branch the result_bit if-ladder collapses (the range hypothesis is known),
   giving the collapsed fact passed to the minimal-context grp. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_bit
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (sum0 sum1 sum2 sum3 sum: u64)
      (result: t_Array u8 (mk_usize 8))
      (i: nat{i < 64})
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr vec) /\
        v sum0 < pow2 16 /\ v sum1 < pow2 16 /\ v sum2 < pow2 16 /\ v sum3 < pow2 16 /\
        v sum0 == (v (Seq.index (repr vec) 0)) * 1 + (v (Seq.index (repr vec) 1)) * 16 + (v (Seq.index (repr vec) 2)) * 256 + (v (Seq.index (repr vec) 3)) * 4096 /\
        v sum1 == (v (Seq.index (repr vec) 4)) * 1 + (v (Seq.index (repr vec) 5)) * 16 + (v (Seq.index (repr vec) 6)) * 256 + (v (Seq.index (repr vec) 7)) * 4096 /\
        v sum2 == (v (Seq.index (repr vec) 8)) * 1 + (v (Seq.index (repr vec) 9)) * 16 + (v (Seq.index (repr vec) 10)) * 256 + (v (Seq.index (repr vec) 11)) * 4096 /\
        v sum3 == (v (Seq.index (repr vec) 12)) * 1 + (v (Seq.index (repr vec) 13)) * 16 + (v (Seq.index (repr vec) 14)) * 256 + (v (Seq.index (repr vec) 15)) * 4096 /\
        sum ==
        (((sum0 |. (sum1 <<! mk_i32 16 <: u64) <: u64) |. (sum2 <<! mk_i32 32 <: u64) <: u64) |.
          (sum3 <<! mk_i32 48 <: u64)) /\
        result == Core_models.Num.impl_u64__to_le_bytes sum)
      (ensures
        Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
        == Rust_primitives.Integers.get_bit (Seq.index (repr vec) (i / 4)) (sz (i % 4))) =
  lemma_ser4_result_bit sum0 sum1 sum2 sum3 sum result i;
  if i < 16 then
    (assert (Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit sum0 (sz (i % 16)));
     lemma_ser4_bit_grp vec sum0 result i 0)
  else if i < 32 then
    (assert (Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit sum1 (sz (i % 16)));
     lemma_ser4_bit_grp vec sum1 result i 1)
  else if i < 48 then
    (assert (Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit sum2 (sz (i % 16)));
     lemma_ser4_bit_grp vec sum2 result i 2)
  else
    (assert (Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit sum3 (sz (i % 16)));
     lemma_ser4_bit_grp vec sum3 result i 3)
#pop-options

(* Dispatcher: one introduce-forall over symbolic i. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_bits
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (sum0 sum1 sum2 sum3 sum: u64)
      (result: t_Array u8 (mk_usize 8))
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr vec) /\
        v sum0 < pow2 16 /\ v sum1 < pow2 16 /\ v sum2 < pow2 16 /\ v sum3 < pow2 16 /\
        v sum0 == (v (Seq.index (repr vec) 0)) * 1 + (v (Seq.index (repr vec) 1)) * 16 + (v (Seq.index (repr vec) 2)) * 256 + (v (Seq.index (repr vec) 3)) * 4096 /\
        v sum1 == (v (Seq.index (repr vec) 4)) * 1 + (v (Seq.index (repr vec) 5)) * 16 + (v (Seq.index (repr vec) 6)) * 256 + (v (Seq.index (repr vec) 7)) * 4096 /\
        v sum2 == (v (Seq.index (repr vec) 8)) * 1 + (v (Seq.index (repr vec) 9)) * 16 + (v (Seq.index (repr vec) 10)) * 256 + (v (Seq.index (repr vec) 11)) * 4096 /\
        v sum3 == (v (Seq.index (repr vec) 12)) * 1 + (v (Seq.index (repr vec) 13)) * 16 + (v (Seq.index (repr vec) 14)) * 256 + (v (Seq.index (repr vec) 15)) * 4096 /\
        sum ==
        (((sum0 |. (sum1 <<! mk_i32 16 <: u64) <: u64) |. (sum2 <<! mk_i32 32 <: u64) <: u64) |.
          (sum3 <<! mk_i32 48 <: u64)) /\
        result == Core_models.Num.impl_u64__to_le_bytes sum)
      (ensures
        forall (i: nat{i < 64}).
          Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit (Seq.index (repr vec) (i / 4)) (sz (i % 4))) =
  introduce forall (i: nat{i < 64}).
      Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
      == Rust_primitives.Integers.get_bit (Seq.index (repr vec) (i / 4)) (sz (i % 4))
  with (lemma_idx_ser4 i; lemma_ser4_bit vec sum0 sum1 sum2 sum3 sum result i)
#pop-options

(* BitVec stitch for d=4 (mirror lemma_bv_eq_from_bits, base 16). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_bv_eq_from_bits4
      (arr: t_Array i16 (mk_usize 16))
      (result: t_Array u8 (mk_usize 8))
    : Lemma
      (requires
        forall (i: nat{i < 64}).
          Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))
          == Rust_primitives.Integers.get_bit (Seq.index arr (i / 4)) (sz (i % 4)))
      (ensures
        BitVecEq.int_t_array_bitwise_eq #i16_inttype #u8_inttype #(mk_usize 16) #(mk_usize 8)
          arr 4 result 8) =
  introduce forall (i: nat{i < 64}).
      Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr 4 i
      == Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 8) result 8 i
  with (assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr 4 i
            == Rust_primitives.Integers.get_bit (Seq.index arr (i / 4)) (sz (i % 4)));
        assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 8) result 8 i
            == Rust_primitives.Integers.get_bit (Seq.index result (i / 8)) (sz (i % 8))));
  BitVecEq.bit_vec_equal_intro
    (Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr 4)
    (BitVecEq.retype (Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 8) result 8))
#pop-options

(* CLEAN: low-half group sum value in (repr vec) terms (cast u16->u64 +
   append bridge), proven in a small context. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_sumval_lo
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (vsrc shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (sg: u64) (base: nat{base + 7 < 16})
    : Lemma
      (requires
        (forall (j: nat{j < 8}). Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) 4) /\
        (forall (j: nat{j < 8}). v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)) /\
        (forall (j: nat{j < 8}). v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) == v (Seq.index (repr vec) (base + j))) /\
        sg == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16
                       (Libcrux_intrinsics.Arm64_extract.e_vget_low_u16
                          (Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift)) <: u16) <: u64))
      (ensures
        v sg < pow2 16 /\
        v sg == (v (Seq.index (repr vec) base)) * 1 + (v (Seq.index (repr vec) (base + 1))) * 16
              + (v (Seq.index (repr vec) (base + 2))) * 256 + (v (Seq.index (repr vec) (base + 3))) * 4096) =
  lemma_ser4_low_sum vsrc shift
#pop-options

(* CLEAN: high-half group sum value in (repr vec) terms. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_ser4_sumval_hi
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (vsrc shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (sg: u64) (base: nat{base + 7 < 16})
    : Lemma
      (requires
        (forall (j: nat{j < 8}). Rust_primitives.BitVectors.bounded (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) 4) /\
        (forall (j: nat{j < 8}). v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)) /\
        (forall (j: nat{j < 8}). v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vsrc j) == v (Seq.index (repr vec) (base + j))) /\
        sg == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16
                       (Libcrux_intrinsics.Arm64_extract.e_vget_high_u16
                          (Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vsrc) shift)) <: u16) <: u64))
      (ensures
        v sg < pow2 16 /\
        v sg == (v (Seq.index (repr vec) (base + 4))) * 1 + (v (Seq.index (repr vec) (base + 5))) * 16
              + (v (Seq.index (repr vec) (base + 6))) * 256 + (v (Seq.index (repr vec) (base + 7))) * 4096) =
  lemma_ser4_high_sum vsrc shift
#pop-options

(* Top-level post for the leaf, from the structural definitions. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_serialize_4_post
      (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (shift: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
      (sum0 sum1 sum2 sum3 sum: u64)
      (result: t_Array u8 (mk_usize 8))
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr vec) /\
        (forall (j: nat{j < 8}).
            v ((Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shift j) %! mk_i16 256) == 4 * (j % 4)) /\
        (let lowt = Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low) shift in
         let hight = Libcrux_intrinsics.Arm64_extract.e_vshlq_u16 (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high) shift in
         sum0 == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16 (Libcrux_intrinsics.Arm64_extract.e_vget_low_u16 lowt) <: u16) <: u64) /\
         sum1 == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16 (Libcrux_intrinsics.Arm64_extract.e_vget_high_u16 lowt) <: u16) <: u64) /\
         sum2 == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16 (Libcrux_intrinsics.Arm64_extract.e_vget_low_u16 hight) <: u16) <: u64) /\
         sum3 == (cast (Libcrux_intrinsics.Arm64_extract.e_vaddv_u16 (Libcrux_intrinsics.Arm64_extract.e_vget_high_u16 hight) <: u16) <: u64)) /\
        sum ==
        (((sum0 |. (sum1 <<! mk_i32 16 <: u64) <: u64) |. (sum2 <<! mk_i32 32 <: u64) <: u64) |.
          (sum3 <<! mk_i32 48 <: u64)) /\
        result == Core_models.Num.impl_u64__to_le_bytes sum)
      (ensures Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (repr vec) result) =
  lemma_ser4_bounded vec;
  (* (repr vec).[k] bridges to the f_low/f_high lanes via the append index SMTPat *)
  assert (forall (j: nat{j < 8}).
        v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low j) == v (Seq.index (repr vec) (0 + j)));
  assert (forall (j: nat{j < 8}).
        v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high j) == v (Seq.index (repr vec) (8 + j)));
  lemma_ser4_sumval_lo vec vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low shift sum0 0;
  lemma_ser4_sumval_hi vec vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low shift sum1 0;
  lemma_ser4_sumval_lo vec vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high shift sum2 8;
  lemma_ser4_sumval_hi vec vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high shift sum3 8;
  lemma_ser4_bits vec sum0 sum1 sum2 sum3 sum result;
  lemma_bv_eq_from_bits4 (repr vec) result
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_4(v: SIMD128Vector) -> [u8; 8] {
    let shifter: [i16; 8] = [0, 4, 8, 12, 0, 4, 8, 12];
    let shift = _vld1q_s16(&shifter);
    let lowt = _vshlq_u16(_vreinterpretq_u16_s16(v.low), shift);
    let hight = _vshlq_u16(_vreinterpretq_u16_s16(v.high), shift);
    let sum0 = _vaddv_u16(_vget_low_u16(lowt)) as u64;
    let sum1 = _vaddv_u16(_vget_high_u16(lowt)) as u64;
    let sum2 = _vaddv_u16(_vget_low_u16(hight)) as u64;
    let sum3 = _vaddv_u16(_vget_high_u16(hight)) as u64;
    let sum = sum0 | (sum1 << 16) | (sum2 << 32) | (sum3 << 48);
    let result = sum.to_le_bytes();
    hax_lib::fstar!(
        r#"assert (forall (j: nat{j < 8}).
      Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${shift} j == Seq.index ${shifter} j);
assert (Seq.index ${shifter} 0 == mk_i16 0 /\ Seq.index ${shifter} 1 == mk_i16 4 /\
        Seq.index ${shifter} 2 == mk_i16 8 /\ Seq.index ${shifter} 3 == mk_i16 12 /\
        Seq.index ${shifter} 4 == mk_i16 0 /\ Seq.index ${shifter} 5 == mk_i16 4 /\
        Seq.index ${shifter} 6 == mk_i16 8 /\ Seq.index ${shifter} 7 == mk_i16 12);
lemma_ser4_shift_amounts ${shift} ${shifter};
lemma_serialize_4_post ${v} ${shift} ${sum0} ${sum1} ${sum2} ${sum3} ${sum} ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(v.len() == 8)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_4_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_4(v: &[u8]) -> SIMD128Vector {
    let input = PortableVector::deserialize_4(v);
    let input_i16s = PortableVector::to_i16_array(input);
    let result = SIMD128Vector {
        low: _vld1q_s16(&input_i16s[0..8]),
        high: _vld1q_s16(&input_i16s[8..16]),
    };
    hax_lib::fstar!(
        r#"
assert (${input_i16s}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                           Core_models.Ops.Range.f_end = mk_usize 8 }
                         <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${input_i16s} 0 8);
assert (${input_i16s}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                           Core_models.Ops.Range.f_end = mk_usize 16 }
                         <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${input_i16s} 8 16);
lemma_repr_of_two_loads ${input_i16s}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::serialize_5_pre} (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::serialize_5_post} (repr ${v}) ${result}"#))]
pub(crate) fn serialize_5(v: SIMD128Vector) -> [u8; 10] {
    let out_i16s = to_i16_array(v);
    let out = PortableVector::from_i16_array(&out_i16s);
    PortableVector::serialize_5(out)
}

#[inline(always)]
#[hax_lib::requires(v.len() == 10)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_5_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_5(v: &[u8]) -> SIMD128Vector {
    let output = PortableVector::deserialize_5(v);
    let array = PortableVector::to_i16_array(output);
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    hax_lib::fstar!(
        r#"
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = mk_usize 8 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 0 8);
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 16 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 8 16);
lemma_repr_of_two_loads ${array}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"
module NI = Libcrux_intrinsics.Arm64_extract
module BV = Rust_primitives.BitVectors
module I = Rust_primitives.Integers
module VT = Libcrux_ml_kem.Vector.Neon.Vector_type
module Spec = Libcrux_ml_kem.Vector.Traits.Spec

(* ===================== Block A: spine get_bit lemmas ===================== *)

(* A1: bit r of the low-d-bits mask (2^d - 1) is 1 iff r < d. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_mask_nat_bit (d: nat) (r: nat)
    : Lemma (ensures I.get_bit_nat (pow2 d - 1) r == (if r < d then 1 else 0)) =
  if r < d then begin
    FStar.Math.Lemmas.pow2_plus r (d - r);
    (* pow2 d - 1 = (pow2 (d-r) - 1) * pow2 r + (pow2 r - 1) *)
    FStar.Math.Lemmas.lemma_div_plus (pow2 r - 1) (pow2 (d - r) - 1) (pow2 r);
    FStar.Math.Lemmas.small_div (pow2 r - 1) (pow2 r);
    (* (pow2 d - 1)/pow2 r == pow2 (d-r) - 1, which is odd *)
    FStar.Math.Lemmas.pow2_plus 1 (d - r - 1)
  end
  else begin
    FStar.Math.Lemmas.pow2_le_compat r d;
    FStar.Math.Lemmas.small_div (pow2 d - 1) (pow2 r)
  end
#pop-options

(* A1-i32: bit r of arm_low_mask_i32 d *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_low_mask_i32_bit (d: nat{d < 32}) (r: nat{r < 32})
    : Lemma (I.get_bit (NI.arm_low_mask_i32 d) (sz r) == (if r < d then 1 else 0)) =
  assert (v (NI.arm_low_mask_i32 d) == pow2 d - 1);
  lemma_get_bit_val #i32_inttype (NI.arm_low_mask_i32 d) (sz r);
  lemma_mask_nat_bit d r
#pop-options

(* A1-i64: bit r of arm_low_mask_i64 d *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_low_mask_i64_bit (d: nat{d < 64}) (r: nat{r < 64})
    : Lemma (I.get_bit (NI.arm_low_mask_i64 d) (sz r) == (if r < d then 1 else 0)) =
  assert (v (NI.arm_low_mask_i64 d) == pow2 d - 1);
  lemma_get_bit_val #i64_inttype (NI.arm_low_mask_i64 d) (sz r);
  lemma_mask_nat_bit d r
#pop-options

(* A2: bit r (<16) of (i16x2_as_i32 lo hi) is bit r of lo. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_i16x2_lo_bit (lo hi: i16) (r: nat{r < 16})
    : Lemma (I.get_bit (NI.i16x2_as_i32 lo hi) (sz r) == I.get_bit lo (sz r)) =
  FStar.Math.Lemmas.small_mod (v (I.cast_mod #i16_inttype #u16_inttype lo)) (pow2 32);
  assert (I.cast #u16_inttype #u32_inttype (I.cast_mod #i16_inttype #u16_inttype lo)
       == I.cast_mod #u16_inttype #u32_inttype (I.cast_mod #i16_inttype #u16_inttype lo))
#pop-options

(* A3: bit r (<32) of (i32x2_as_i64 lo hi) is bit r of lo. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_i32x2_lo_bit (lo hi: i32) (r: nat{r < 32})
    : Lemma (I.get_bit (NI.i32x2_as_i64 lo hi) (sz r) == I.get_bit lo (sz r)) =
  FStar.Math.Lemmas.small_mod (v (I.cast_mod #i32_inttype #u32_inttype lo)) (pow2 64);
  assert (I.cast #u32_inttype #u64_inttype (I.cast_mod #i32_inttype #u32_inttype lo)
       == I.cast_mod #u32_inttype #u64_inttype (I.cast_mod #i32_inttype #u32_inttype lo))
#pop-options

(* A4: the i32 vsli layer.  mixt = (i16x2_as_i32 ae ae & mask_d) | (i16x2_as_i32 ao ao << d).
   bit r (<2d) of mixt is bit r of ae (r<d) or bit (r-d) of ao (d<=r<2d). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_vsli32_lane_bit (ae ao: i16) (d: nat{0 < d /\ d <= 12}) (r: nat{r < 2 * d})
    : Lemma
      (ensures
        (let mixt = (NI.i16x2_as_i32 ae ae &. NI.arm_low_mask_i32 d) |.
                    (NI.i16x2_as_i32 ao ao <<! mk_i32 d) in
         I.get_bit mixt (sz r) == (if r < d then I.get_bit ae (sz r) else I.get_bit ao (sz (r - d))))) =
  lemma_low_mask_i32_bit d r;
  (if r < d then lemma_i16x2_lo_bit ae ae r
   else lemma_i16x2_lo_bit ao ao (r - d))
#pop-options

(* A5: the i64 vsli layer. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_vsli64_lane_bit (me mo: i32) (d2: nat{0 < d2 /\ d2 <= 24}) (q: nat{q < 2 * d2})
    : Lemma
      (ensures
        (let lane = (NI.i32x2_as_i64 me me &. NI.arm_low_mask_i64 d2) |.
                    (NI.i32x2_as_i64 mo mo <<! mk_i32 d2) in
         I.get_bit lane (sz q) == (if q < d2 then I.get_bit me (sz q) else I.get_bit mo (sz (q - d2))))) =
  lemma_low_mask_i64_bit d2 q;
  (if q < d2 then lemma_i32x2_lo_bit me me q
   else lemma_i32x2_lo_bit mo mo (q - d2))
#pop-options

(* q div/mod by d, when q lives in band [m*d, (m+1)*d). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_q_band (d: nat{d > 0}) (q: nat) (m: nat)
    : Lemma (requires m * d <= q /\ q < (m + 1) * d)
            (ensures q / d == m /\ q % d == q - m * d) =
  let r = q - m * d in
  FStar.Math.Lemmas.small_div r d;
  FStar.Math.Lemmas.small_mod r d;
  FStar.Math.Lemmas.lemma_div_plus r m d;
  FStar.Math.Lemmas.lemma_mod_plus r m d
#pop-options

(* A6: combine both layers — bit q (<4d) of the packed lane is bit (q%d) of coeff q/d. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 250 --split_queries always"
let lemma_lane_bit (c0 c1 c2 c3: i16) (me mo: i32) (lane: i64) (d: nat{0 < d /\ d <= 12}) (q: nat{q < 4 * d})
    : Lemma
      (requires
        (let me32 = (NI.i16x2_as_i32 c0 c0 &. NI.arm_low_mask_i32 d) |. (NI.i16x2_as_i32 c1 c1 <<! mk_i32 d) in
         let mo32 = (NI.i16x2_as_i32 c2 c2 &. NI.arm_low_mask_i32 d) |. (NI.i16x2_as_i32 c3 c3 <<! mk_i32 d) in
         me == me32 /\ mo == mo32 /\
         lane == ((NI.i32x2_as_i64 me me &. NI.arm_low_mask_i64 (2 * d)) |. (NI.i32x2_as_i64 mo mo <<! mk_i32 (2 * d)))))
      (ensures
        I.get_bit lane (sz q) ==
        (if q < d then I.get_bit c0 (sz q)
         else if q < 2 * d then I.get_bit c1 (sz (q - d))
         else if q < 3 * d then I.get_bit c2 (sz (q - 2 * d))
         else I.get_bit c3 (sz (q - 3 * d)))) =
  lemma_vsli64_lane_bit me mo (2 * d) q;
  if q < 2 * d then lemma_vsli32_lane_bit c0 c1 d q
  else lemma_vsli32_lane_bit c2 c3 d (q - 2 * d)
#pop-options

(* B1: bit p (<8) of byte e (<8) of an i64 is bit (8e+p) of the i64. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_i64_byte_bit (x: i64) (e: nat{e < 8}) (p: nat{p < 8})
    : Lemma (I.get_bit (NI.i64_byte x e) (sz p) == I.get_bit x (sz (8 * e + p))) =
  ()
#pop-options

(* ===================== Block B: recompute spine + stitch ===================== *)

(* opaque atoms: the s32 mixt and full i64x2 packed spine of one 8-coeff half. *)
[@@ "opaque_to_smt"]
let half_mixt (half: NI.t_e_int16x8_t) (d: nat{0 < d /\ d <= 12}) : NI.t_e_int32x4_t =
  NI.e_vsliq_n_s32 (mk_i32 d)
    (NI.e_vreinterpretq_s32_s16 (NI.e_vtrn1q_s16 half half))
    (NI.e_vreinterpretq_s32_s16 (NI.e_vtrn2q_s16 half half))

[@@ "opaque_to_smt"]
let half_spine (half: NI.t_e_int16x8_t) (d: nat{0 < d /\ d <= 12}) : NI.t_e_int64x2_t =
  NI.e_vsliq_n_s64 (mk_i32 (2 * d))
    (NI.e_vreinterpretq_s64_s32 (NI.e_vtrn1q_s32 (half_mixt half d) (half_mixt half d)))
    (NI.e_vreinterpretq_s64_s32 (NI.e_vtrn2q_s32 (half_mixt half d) (half_mixt half d)))

(* B2: the recompute lemma — bit q (<4d) of lane g (g<2) of the packed i64x2
   `low_mix == half_spine half d` is bit (q%d) of coeff (4g + q/d) of half. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_half_lane_bit (half: NI.t_e_int16x8_t) (low_mix: NI.t_e_int64x2_t)
      (g: nat{g < 2}) (d: nat{0 < d /\ d <= 12}) (q: nat{q < 4 * d})
    : Lemma
      (requires low_mix == half_spine half d)
      (ensures
        I.get_bit (NI.get_lane_i64x2 low_mix g) (sz q) ==
        I.get_bit (NI.get_lane_i16x8 half (4 * g + q / d)) (sz (q % d))) =
  reveal_opaque (`%half_spine) (half_spine half d);
  reveal_opaque (`%half_mixt) (half_mixt half d);
  let mixt = half_mixt half d in
  let c0 = NI.get_lane_i16x8 half (4 * g) in
  let c1 = NI.get_lane_i16x8 half (4 * g + 1) in
  let c2 = NI.get_lane_i16x8 half (4 * g + 2) in
  let c3 = NI.get_lane_i16x8 half (4 * g + 3) in
  let me = NI.get_lane_i32x4 mixt (2 * g) in
  let mo = NI.get_lane_i32x4 mixt (2 * g + 1) in
  let lane = NI.get_lane_i64x2 low_mix g in
  assert (me == ((NI.i16x2_as_i32 c0 c0 &. NI.arm_low_mask_i32 d) |. (NI.i16x2_as_i32 c1 c1 <<! mk_i32 d)));
  assert (mo == ((NI.i16x2_as_i32 c2 c2 &. NI.arm_low_mask_i32 d) |. (NI.i16x2_as_i32 c3 c3 <<! mk_i32 d)));
  assert (lane == ((NI.i32x2_as_i64 me me &. NI.arm_low_mask_i64 (2 * d)) |. (NI.i32x2_as_i64 mo mo <<! mk_i32 (2 * d))));
  lemma_lane_bit c0 c1 c2 c3 me mo lane d q;
  (if q < d then lemma_q_band d q 0
   else if q < 2 * d then lemma_q_band d q 1
   else if q < 3 * d then lemma_q_band d q 2
   else lemma_q_band d q 3)
#pop-options

(* lane selector: groups 0,1 from low_mix; groups 2,3 from high_mix. *)
let lane_of (low_mix high_mix: NI.t_e_int64x2_t) (g: nat{g < 4}) : i64 =
  if g < 2 then NI.get_lane_i64x2 low_mix g else NI.get_lane_i64x2 high_mix (g - 2)

(* B3: per-global-bit fact for the byte stream. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_serd_bit
      (vec: VT.t_SIMD128Vector)
      (low_mix high_mix: NI.t_e_int64x2_t)
      (v_N: nat{0 < v_N /\ v_N <= 6})
      (result: t_Array u8 (mk_usize (4 * v_N)))
      (i: nat{i < 32 * v_N})
    : Lemma
      (requires
        (forall (g: nat{g < 4}) (q: nat{q < 8 * v_N}).
           I.get_bit (lane_of low_mix high_mix g) (sz q) ==
           I.get_bit (Seq.index (VT.repr vec) (4 * g + q / (2 * v_N))) (sz (q % (2 * v_N)))) /\
        (forall (g: nat{g < 4}) (e: nat{e < v_N}).
           Seq.index result (v_N * g + e) == NI.i64_byte (lane_of low_mix high_mix g) e))
      (ensures
        I.get_bit (Seq.index result (i / 8)) (sz (i % 8)) ==
        I.get_bit (Seq.index (VT.repr vec) (i / (2 * v_N))) (sz (i % (2 * v_N)))) =
  let beta = i / 8 in
  let p = i % 8 in
  let n = v_N in
  FStar.Math.Lemmas.lemma_div_mod i 8;
  let g = beta / n in
  let e = beta % n in
  FStar.Math.Lemmas.lemma_div_mod beta n;
  let q = 8 * e + p in
  assert (beta < 4 * n);
  assert (g < 4);
  assert (e < n);
  assert (q < 8 * n);
  assert (Seq.index result (n * g + e) == NI.i64_byte (lane_of low_mix high_mix g) e);
  lemma_i64_byte_bit (lane_of low_mix high_mix g) e p;
  assert (i == q + (4 * g) * (2 * n));
  FStar.Math.Lemmas.lemma_div_plus q (4 * g) (2 * n);
  FStar.Math.Lemmas.lemma_mod_plus q (4 * g) (2 * n)
#pop-options

(* B-stitch: per-bit equality -> int_t_array_bitwise_eq (mirror lemma_bv_eq_from_bits4). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_bv_eq_from_bitsN (v_N: nat{0 < v_N /\ v_N <= 6})
      (arr: t_Array i16 (mk_usize 16))
      (result: t_Array u8 (mk_usize (4 * v_N)))
    : Lemma
      (requires
        (forall (i: nat{i < 32 * v_N}).
          I.get_bit (Seq.index result (i / 8)) (sz (i % 8)) ==
          I.get_bit (Seq.index arr (i / (2 * v_N))) (sz (i % (2 * v_N)))))
      (ensures
        BitVecEq.int_t_array_bitwise_eq #i16_inttype #u8_inttype #(mk_usize 16) #(mk_usize (4 * v_N))
          arr (2 * v_N) result 8) =
  introduce forall (i: nat{i < 32 * v_N}).
      BV.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr (2 * v_N) i
      == BV.bit_vec_of_int_t_array #u8_inttype #(mk_usize (4 * v_N)) result 8 i
  with (assert (BV.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr (2 * v_N) i
            == I.get_bit (Seq.index arr (i / (2 * v_N))) (sz (i % (2 * v_N))));
        assert (BV.bit_vec_of_int_t_array #u8_inttype #(mk_usize (4 * v_N)) result 8 i
            == I.get_bit (Seq.index result (i / 8)) (sz (i % 8))));
  BitVecEq.bit_vec_equal_intro
    (BV.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) arr (2 * v_N))
    (BitVecEq.retype (BV.bit_vec_of_int_t_array #u8_inttype #(mk_usize (4 * v_N)) result 8))
#pop-options

(* B4: top chainer — the leaf post from the spine + result-byte mapping. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_serialize_post
      (vec: VT.t_SIMD128Vector)
      (low_mix high_mix: NI.t_e_int64x2_t)
      (v_N: nat{0 < v_N /\ v_N <= 6})
      (result: t_Array u8 (mk_usize (4 * v_N)))
    : Lemma
      (requires
        Spec.serialize_pre_N (2 * v_N) (VT.repr vec) /\
        (forall (g: nat{g < 4}) (q: nat{q < 8 * v_N}).
           I.get_bit (lane_of low_mix high_mix g) (sz q) ==
           I.get_bit (Seq.index (VT.repr vec) (4 * g + q / (2 * v_N))) (sz (q % (2 * v_N)))) /\
        (forall (g: nat{g < 4}) (e: nat{e < v_N}).
           Seq.index result (v_N * g + e) == NI.i64_byte (lane_of low_mix high_mix g) e))
      (ensures Spec.serialize_post_N (2 * v_N) (VT.repr vec) result) =
  introduce forall (i: nat{i < 32 * v_N}).
      I.get_bit (Seq.index result (i / 8)) (sz (i % 8)) ==
      I.get_bit (Seq.index (VT.repr vec) (i / (2 * v_N))) (sz (i % (2 * v_N)))
  with lemma_serd_bit vec low_mix high_mix v_N result i;
  lemma_bv_eq_from_bitsN v_N (VT.repr vec) result
#pop-options

(* framing SMTPat: a sub-slice [a,b) that ends at/before the written range's start
   is preserved by update_at_range.  Chains through the nested updates automatically. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_uar_sub_before (#t: Type0) (s: t_Slice t)
      (i: Core_models.Ops.Range.t_Range usize) (x: t_Slice t) (a b: nat)
    : Lemma
      (requires
        v i.Core_models.Ops.Range.f_start <= Seq.length s /\
        v i.Core_models.Ops.Range.f_end <= Seq.length s /\
        Seq.length x == v i.Core_models.Ops.Range.f_end - v i.Core_models.Ops.Range.f_start /\
        a <= b /\ b <= v i.Core_models.Ops.Range.f_start)
      (ensures
        Seq.slice (Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x) a b == Seq.slice s a b)
      [SMTPat (Seq.slice (Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x) a b)] =
  let res = Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x in
  let fs = v i.Core_models.Ops.Range.f_start in
  FStar.Seq.Properties.slice_slice res 0 fs a b;
  FStar.Seq.Properties.slice_slice s 0 fs a b
#pop-options

(* R3: result-byte mapping, from the two 16-byte stores + four copy_from_slice. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_result_mapping
      (low_mix high_mix: NI.t_e_int64x2_t)
      (v_N: nat{0 < v_N /\ v_N <= 6})
      (result32: t_Array u8 (mk_usize 32))
      (result: t_Array u8 (mk_usize (4 * v_N)))
    : Lemma
      (requires
        (forall (k: nat{k < 16}).
           Seq.index result32 k == NI.i64_byte (NI.get_lane_i64x2 low_mix (k / 8)) (k % 8)) /\
        (forall (k: nat{k < 16}).
           Seq.index result32 (16 + k) == NI.i64_byte (NI.get_lane_i64x2 high_mix (k / 8)) (k % 8)) /\
        Seq.slice result 0 v_N == Seq.slice result32 0 v_N /\
        Seq.slice result v_N (v_N + v_N) == Seq.slice result32 8 (8 + v_N) /\
        Seq.slice result (2 * v_N) (2 * v_N + v_N) == Seq.slice result32 16 (16 + v_N) /\
        Seq.slice result (3 * v_N) (3 * v_N + v_N) == Seq.slice result32 24 (24 + v_N))
      (ensures
        (forall (g: nat{g < 4}) (e: nat{e < v_N}).
           Seq.index result (v_N * g + e) == NI.i64_byte (lane_of low_mix high_mix g) e)) =
  let aux (g: nat{g < 4}) (e: nat{e < v_N}) : Lemma
      (Seq.index result (v_N * g + e) == NI.i64_byte (lane_of low_mix high_mix g) e) =
    if g = 0 then begin
      Seq.lemma_index_slice result 0 v_N e;
      Seq.lemma_index_slice result32 0 v_N e;
      FStar.Math.Lemmas.small_div e 8; FStar.Math.Lemmas.small_mod e 8
    end
    else if g = 1 then begin
      Seq.lemma_index_slice result v_N (v_N + v_N) e;
      Seq.lemma_index_slice result32 8 (8 + v_N) e;
      FStar.Math.Lemmas.small_div e 8; FStar.Math.Lemmas.small_mod e 8;
      FStar.Math.Lemmas.lemma_div_plus e 1 8; FStar.Math.Lemmas.lemma_mod_plus e 1 8
    end
    else if g = 2 then begin
      Seq.lemma_index_slice result (2 * v_N) (2 * v_N + v_N) e;
      Seq.lemma_index_slice result32 16 (16 + v_N) e;
      assert (Seq.index result32 (16 + e) == NI.i64_byte (NI.get_lane_i64x2 high_mix (e / 8)) (e % 8));
      FStar.Math.Lemmas.small_div e 8; FStar.Math.Lemmas.small_mod e 8
    end
    else begin
      Seq.lemma_index_slice result (3 * v_N) (3 * v_N + v_N) e;
      Seq.lemma_index_slice result32 24 (24 + v_N) e;
      assert (24 + e == 16 + (8 + e));
      assert (Seq.index result32 (16 + (8 + e)) ==
              NI.i64_byte (NI.get_lane_i64x2 high_mix ((8 + e) / 8)) ((8 + e) % 8));
      FStar.Math.Lemmas.small_div e 8; FStar.Math.Lemmas.small_mod e 8;
      FStar.Math.Lemmas.lemma_div_plus e 1 8; FStar.Math.Lemmas.lemma_mod_plus e 1 8
    end
  in
  Classical.forall_intro_2 aux
#pop-options

(* store-byte conversion: slice-content (get_lane_u8x16 of the reinterpret) -> i64_byte facts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_store32_to_bytes (low_mix high_mix: NI.t_e_int64x2_t) (result32: t_Array u8 (mk_usize 32))
    : Lemma
      (requires
        (forall (k: nat{k < 16}).
           Seq.index (Seq.slice result32 0 16) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 low_mix) k) /\
        (forall (k: nat{k < 16}).
           Seq.index (Seq.slice result32 16 32) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 high_mix) k))
      (ensures
        (forall (k: nat{k < 16}).
           Seq.index result32 k == NI.i64_byte (NI.get_lane_i64x2 low_mix (k / 8)) (k % 8)) /\
        (forall (k: nat{k < 16}).
           Seq.index result32 (16 + k) == NI.i64_byte (NI.get_lane_i64x2 high_mix (k / 8)) (k % 8))) =
  let auxl (k: nat{k < 16}) : Lemma
      (Seq.index result32 k == NI.i64_byte (NI.get_lane_i64x2 low_mix (k / 8)) (k % 8)) =
    Seq.lemma_index_slice result32 0 16 k in
  let auxh (k: nat{k < 16}) : Lemma
      (Seq.index result32 (16 + k) == NI.i64_byte (NI.get_lane_i64x2 high_mix (k / 8)) (k % 8)) =
    Seq.lemma_index_slice result32 16 32 k in
  Classical.forall_intro auxl;
  Classical.forall_intro auxh
#pop-options

(* R2: spine fact for both halves, in clean context (atomic half_spine match). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_spine_r2 (vec: VT.t_SIMD128Vector) (low_mix high_mix: NI.t_e_int64x2_t) (d: nat{0 < d /\ d <= 12})
    : Lemma
      (requires low_mix == half_spine vec.VT.f_low d /\ high_mix == half_spine vec.VT.f_high d)
      (ensures
        (forall (g: nat{g < 4}) (q: nat{q < 4 * d}).
           I.get_bit (lane_of low_mix high_mix g) (sz q) ==
           I.get_bit (Seq.index (VT.repr vec) (4 * g + q / d)) (sz (q % d)))) =
  introduce forall (g: nat{g < 4}) (q: nat{q < 4 * d}).
      I.get_bit (lane_of low_mix high_mix g) (sz q) ==
      I.get_bit (Seq.index (VT.repr vec) (4 * g + q / d)) (sz (q % d))
  with (if g < 2 then lemma_half_lane_bit vec.VT.f_low low_mix g d q
        else lemma_half_lane_bit vec.VT.f_high high_mix (g - 2) d q)
#pop-options
"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_10(v: SIMD128Vector) -> [u8; 20] {
    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.low, v.low)); // a0, a0, a2, a2, a4, a4, a6, a6
    let low1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.low, v.low)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<10>(low0, low1); // a1a0, a3a2, a5a4, a7a6

    let low0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let low1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let low_mix = _vsliq_n_s64::<20>(low0, low1); // a3a2a1a0, a7a6a5a4

    let high0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.high, v.high)); // a0, a0, a2, a2, a4, a4, a6, a6
    let high1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.high, v.high)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<10>(high0, high1); // a1a0, a3a2, a5a4, a7a6

    let high0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let high1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let high_mix = _vsliq_n_s64::<20>(high0, high1); // a3a2a1a0, a7a6a5a4

    let mut result32 = [0u8; 32];
    _vst1q_u8(&mut result32[0..16], _vreinterpretq_u8_s64(low_mix));
    _vst1q_u8(&mut result32[16..32], _vreinterpretq_u8_s64(high_mix));
    let mut result = [0u8; 20];
    result[0..5].copy_from_slice(&result32[0..5]);
    result[5..10].copy_from_slice(&result32[8..13]);
    result[10..15].copy_from_slice(&result32[16..21]);
    result[15..20].copy_from_slice(&result32[24..29]);
    hax_lib::fstar!(
        r#"
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${low_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${low_mix}) k
      with ()
    in
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${high_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${high_mix}) k
      with ()
    in
    lemma_store32_to_bytes ${low_mix} ${high_mix} ${result32};
    assert (Seq.slice ${result} 0 5 == Seq.slice ${result32} 0 5);
    assert (Seq.slice ${result} 5 10 == Seq.slice ${result32} 8 13);
    assert (Seq.slice ${result} 10 15 == Seq.slice ${result32} 16 21);
    assert (Seq.slice ${result} 15 20 == Seq.slice ${result32} 24 29);
    lemma_result_mapping ${low_mix} ${high_mix} 5 ${result32} ${result};
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 10);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 10);
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 10);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 10);
    assert (${low_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 10);
    assert (${high_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 10);
    lemma_spine_r2 ${v} ${low_mix} ${high_mix} 10;
    lemma_serialize_post ${v} ${low_mix} ${high_mix} 5 ${result}
    "#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(v.len() == 20)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_10_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_10(v: &[u8]) -> SIMD128Vector {
    let output = PortableVector::deserialize_10(v);
    let array = PortableVector::to_i16_array(output);
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    hax_lib::fstar!(
        r#"
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = mk_usize 8 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 0 8);
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 16 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 8 16);
lemma_repr_of_two_loads ${array}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::serialize_11_pre} (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::serialize_11_post} (repr ${v}) ${result}"#))]
pub(crate) fn serialize_11(v: SIMD128Vector) -> [u8; 22] {
    let out_i16s = to_i16_array(v);
    let out = PortableVector::from_i16_array(&out_i16s);
    PortableVector::serialize_11(out)
}

#[inline(always)]
#[hax_lib::requires(v.len() == 22)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_11_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_11(v: &[u8]) -> SIMD128Vector {
    let output = PortableVector::deserialize_11(v);
    let array = PortableVector::to_i16_array(output);
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    hax_lib::fstar!(
        r#"
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = mk_usize 8 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 0 8);
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 16 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 8 16);
lemma_repr_of_two_loads ${array}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_12(v: SIMD128Vector) -> [u8; 24] {
    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.low, v.low)); // a0, a0, a2, a2, a4, a4, a6, a6
    let low1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.low, v.low)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<12>(low0, low1); // a1a0, a3a2, a5a4, a7a6

    let low0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let low1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let low_mix = _vsliq_n_s64::<24>(low0, low1); // a3a2a1a0, a7a6a5a4

    let high0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.high, v.high)); // a0, a0, a2, a2, a4, a4, a6, a6
    let high1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.high, v.high)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<12>(high0, high1); // a1a0, a3a2, a5a4, a7a6

    let high0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let high1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let high_mix = _vsliq_n_s64::<24>(high0, high1); // a3a2a1a0, a7a6a5a4

    let mut result32 = [0u8; 32];
    _vst1q_u8(&mut result32[0..16], _vreinterpretq_u8_s64(low_mix));
    _vst1q_u8(&mut result32[16..32], _vreinterpretq_u8_s64(high_mix));
    let mut result = [0u8; 24];
    result[0..6].copy_from_slice(&result32[0..6]);
    result[6..12].copy_from_slice(&result32[8..14]);
    result[12..18].copy_from_slice(&result32[16..22]);
    result[18..24].copy_from_slice(&result32[24..30]);
    hax_lib::fstar!(
        r#"
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${low_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${low_mix}) k
      with ()
    in
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${high_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (NI.e_vreinterpretq_u8_s64 ${high_mix}) k
      with ()
    in
    lemma_store32_to_bytes ${low_mix} ${high_mix} ${result32};
    assert (Seq.slice ${result} 0 6 == Seq.slice ${result32} 0 6);
    assert (Seq.slice ${result} 6 12 == Seq.slice ${result32} 8 14);
    assert (Seq.slice ${result} 12 18 == Seq.slice ${result32} 16 22);
    assert (Seq.slice ${result} 18 24 == Seq.slice ${result32} 24 30);
    lemma_result_mapping ${low_mix} ${high_mix} 6 ${result32} ${result};
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 12);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 12);
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 12);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 12);
    assert (${low_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 12);
    assert (${high_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 12);
    lemma_spine_r2 ${v} ${low_mix} ${high_mix} 12;
    lemma_serialize_post ${v} ${low_mix} ${high_mix} 6 ${result}
    "#
    );
    result
}

#[hax_lib::fstar::before(
    r#"
module NI = Libcrux_intrinsics.Arm64_extract
module BV = Rust_primitives.BitVectors
module I = Rust_primitives.Integers

(* bit b (<12) of ((n >> s) & (2^12-1)) is bit (s+b) of n. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_window_bit (n: nat) (s: nat) (b: nat{b < 12})
    : Lemma
      (ensures I.get_bit_nat ((n / pow2 s) % pow2 12) b == I.get_bit_nat n (s + b)) =
  let q = n / pow2 s in
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 q b 12;
  FStar.Math.Lemmas.pow2_modulo_modulo_lemma_1 (q / pow2 b) 1 (12 - b);
  FStar.Math.Lemmas.division_multiplication_lemma n (pow2 s) (pow2 b);
  FStar.Math.Lemmas.pow2_plus s b
#pop-options

(* bit b (<12) of the all-12-bits-set u16 mask is 1. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_mask12_bit (b: nat{b < 12}) : Lemma (I.get_bit (mk_u16 4095) (sz b) == 1) =
  lemma_get_bit_val #u16_inttype (mk_u16 4095) (sz b);
  assert (v (mk_u16 4095) == 4095);
  (match b with
   | 0  -> assert_norm (I.get_bit_nat 4095 0  == 1)
   | 1  -> assert_norm (I.get_bit_nat 4095 1  == 1)
   | 2  -> assert_norm (I.get_bit_nat 4095 2  == 1)
   | 3  -> assert_norm (I.get_bit_nat 4095 3  == 1)
   | 4  -> assert_norm (I.get_bit_nat 4095 4  == 1)
   | 5  -> assert_norm (I.get_bit_nat 4095 5  == 1)
   | 6  -> assert_norm (I.get_bit_nat 4095 6  == 1)
   | 7  -> assert_norm (I.get_bit_nat 4095 7  == 1)
   | 8  -> assert_norm (I.get_bit_nat 4095 8  == 1)
   | 9  -> assert_norm (I.get_bit_nat 4095 9  == 1)
   | 10 -> assert_norm (I.get_bit_nat 4095 10 == 1)
   | _  -> assert_norm (I.get_bit_nat 4095 11 == 1))
#pop-options

(* bit i (<16) of the little-endian 2-byte value lo + hi*256. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_u8x2_as_u16_bit (lo hi: u8) (i: nat{i < 16})
    : Lemma
      (ensures
        I.get_bit (NI.u8x2_as_u16 lo hi) (sz i) ==
        (if i < 8 then I.get_bit lo (sz i) else I.get_bit hi (sz (i - 8)))) =
  FStar.Math.Lemmas.small_mod (v lo) (pow2 16);
  FStar.Math.Lemmas.small_mod (v hi) (pow2 16);
  assert (I.cast #u8_inttype #u16_inttype lo == I.cast_mod #u8_inttype #u16_inttype lo);
  assert (I.cast #u8_inttype #u16_inttype hi == I.cast_mod #u8_inttype #u16_inttype hi)
#pop-options

(* One lane value: cast_mod_u16->i16 of ((arm_ushl w shift) & 0xFFF) is the
   shifted-masked window (v w / 2^s) mod 2^12, and it is 12-bit bounded. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_deser12_lane_val (w: u16) (shift_lane: i16) (s: nat)
    : Lemma
      (requires
        (s == 0 /\ v (shift_lane %! mk_i16 256) == 0) \/
        (s == 4 /\ v (shift_lane %! mk_i16 256) == 252))
      (ensures
        (let res = I.cast_mod #u16_inttype #i16_inttype
                     ((NI.arm_ushl_u16 w shift_lane) &. mk_u16 4095) in
         v res == (v w / pow2 s) % pow2 12 /\ BV.bounded res 12)) =
  let z = NI.arm_ushl_u16 w shift_lane in
  assert (v z == v w / pow2 s);
  I.logand_mask_lemma #u16_inttype z 12;
  assert_norm (mk_u16 4095 == I.sub #u16_inttype (mk_int #u16_inttype (pow2 12)) (mk_int #u16_inttype 1));
  let m = z &. mk_u16 4095 in
  assert (v m == v z % pow2 12);
  assert (v m < pow2 12);
  assert_norm (pow2 12 <= pow2 15)
#pop-options

(* One lane bit: bit b (<12) of the result lane equals bit (s+b) of the window w. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_deser12_lane_bit (w: u16) (shift_lane: i16) (s: nat) (b: nat{b < 12})
    : Lemma
      (requires
        (s == 0 /\ v (shift_lane %! mk_i16 256) == 0) \/
        (s == 4 /\ v (shift_lane %! mk_i16 256) == 252))
      (ensures
        (let res = I.cast_mod #u16_inttype #i16_inttype
                     ((NI.arm_ushl_u16 w shift_lane) &. mk_u16 4095) in
         BV.bounded res 12 /\
         I.get_bit res (sz b) == I.get_bit w (sz (s + b)))) =
  let res = I.cast_mod #u16_inttype #i16_inttype ((NI.arm_ushl_u16 w shift_lane) &. mk_u16 4095) in
  lemma_deser12_lane_val w shift_lane s;
  lemma_get_bit_val #i16_inttype res (sz b);
  lemma_window_bit (v w) s b;
  lemma_get_bit_val #u16_inttype w (sz (s + b))
#pop-options

(* One output lane c of a 12-bit deserialize half: the vqtbl byte-gather selects
   bytes (idxA,idxB) from input_vec, vreinterpret reads the LE window, vshl/vand
   shift-mask; bit b of the result lane == bit (s+b) of the window u8x2(vA,vB). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 250 --split_queries always --z3refresh"
let lemma_deser12_out_lane
      (input_vec index_vec: NI.t_e_uint8x16_t)
      (shift_vec: NI.t_e_int16x8_t) (mask12: NI.t_e_uint16x8_t)
      (c: nat{c < 8}) (idxA idxB: nat) (vA vB: u8) (s: nat) (b: nat{b < 12})
    : Lemma
      (requires
        v (NI.get_lane_u8x16 index_vec (2 * c)) == idxA /\
        v (NI.get_lane_u8x16 index_vec (2 * c + 1)) == idxB /\
        idxA < 16 /\ idxB < 16 /\
        NI.get_lane_u8x16 input_vec idxA == vA /\
        NI.get_lane_u8x16 input_vec idxB == vB /\
        (forall (i: nat{i < 8}). NI.get_lane_u16x8 mask12 i == mk_u16 4095) /\
        ((s == 0 /\ v (NI.get_lane_i16x8 shift_vec c %! mk_i16 256) == 0) \/
         (s == 4 /\ v (NI.get_lane_i16x8 shift_vec c %! mk_i16 256) == 252)))
      (ensures
        (let low = NI.e_vreinterpretq_s16_u16
                     (NI.e_vandq_u16
                        (NI.e_vshlq_u16
                           (NI.e_vreinterpretq_u16_u8 (NI.e_vqtbl1q_u8 input_vec index_vec))
                           shift_vec)
                        mask12) in
         BV.bounded (NI.get_lane_i16x8 low c) 12 /\
         I.get_bit (NI.get_lane_i16x8 low c) (sz b) ==
         I.get_bit (NI.u8x2_as_u16 vA vB) (sz (s + b)))) =
  let tbl = NI.e_vqtbl1q_u8 input_vec index_vec in
  let moved = NI.e_vreinterpretq_u16_u8 tbl in
  let shifted = NI.e_vshlq_u16 moved shift_vec in
  let masked = NI.e_vandq_u16 shifted mask12 in
  let low = NI.e_vreinterpretq_s16_u16 masked in
  assert (NI.get_lane_u8x16 tbl (2 * c) == vA);
  assert (NI.get_lane_u8x16 tbl (2 * c + 1) == vB);
  assert (NI.get_lane_u16x8 moved c == NI.u8x2_as_u16 vA vB);
  assert (NI.get_lane_u16x8 shifted c ==
          NI.arm_ushl_u16 (NI.get_lane_u16x8 moved c) (NI.get_lane_i16x8 shift_vec c));
  assert (NI.get_lane_u16x8 mask12 c == mk_u16 4095);
  assert (NI.get_lane_u16x8 masked c ==
          (NI.get_lane_u16x8 shifted c &. mk_u16 4095));
  assert (NI.get_lane_i16x8 low c ==
          I.cast_mod #u16_inttype #i16_inttype
            ((NI.arm_ushl_u16 (NI.u8x2_as_u16 vA vB) (NI.get_lane_i16x8 shift_vec c)) &. mk_u16 4095));
  lemma_deser12_lane_bit (NI.u8x2_as_u16 vA vB) (NI.get_lane_i16x8 shift_vec c) s b
#pop-options

(* Coefficient cc (<16) of the packed byte stream: ties the output lane's bit b to
   the global byte-stream bit (12*cc+b), given the window invariant 8*byteA+s==12*cc
   and vA=inp[byteA], vB=inp[byteA+1]. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 250 --split_queries always --z3refresh"
let lemma_deser12_coeff_bit
      (inp: t_Slice u8)
      (input_vec index_vec: NI.t_e_uint8x16_t)
      (shift_vec: NI.t_e_int16x8_t) (mask12: NI.t_e_uint16x8_t)
      (c: nat{c < 8}) (idxA idxB: nat) (vA vB: u8) (s: nat) (byteA: nat) (cc: nat{cc < 16})
      (b: nat{b < 12})
    : Lemma
      (requires
        v (NI.get_lane_u8x16 index_vec (2 * c)) == idxA /\
        v (NI.get_lane_u8x16 index_vec (2 * c + 1)) == idxB /\
        idxA < 16 /\ idxB < 16 /\
        NI.get_lane_u8x16 input_vec idxA == vA /\
        NI.get_lane_u8x16 input_vec idxB == vB /\
        (forall (i: nat{i < 8}). NI.get_lane_u16x8 mask12 i == mk_u16 4095) /\
        ((s == 0 /\ v (NI.get_lane_i16x8 shift_vec c %! mk_i16 256) == 0) \/
         (s == 4 /\ v (NI.get_lane_i16x8 shift_vec c %! mk_i16 256) == 252)) /\
        byteA + 1 < Seq.length inp /\
        8 * byteA + s == 12 * cc /\
        vA == Seq.index inp byteA /\ vB == Seq.index inp (byteA + 1))
      (ensures
        (let low = NI.e_vreinterpretq_s16_u16
                     (NI.e_vandq_u16
                        (NI.e_vshlq_u16
                           (NI.e_vreinterpretq_u16_u8 (NI.e_vqtbl1q_u8 input_vec index_vec))
                           shift_vec)
                        mask12) in
         BV.bounded (NI.get_lane_i16x8 low c) 12 /\
         I.get_bit (NI.get_lane_i16x8 low c) (sz b) ==
         I.get_bit (Seq.index inp ((12 * cc + b) / 8)) (sz ((12 * cc + b) % 8)))) =
  lemma_deser12_out_lane input_vec index_vec shift_vec mask12 c idxA idxB vA vB s b;
  lemma_u8x2_as_u16_bit vA vB (s + b);
  let r = s + b in
  assert (12 * cc + b == r + byteA * 8);
  FStar.Math.Lemmas.lemma_div_plus r byteA 8;
  FStar.Math.Lemmas.lemma_mod_plus r byteA 8;
  (if r < 8
   then (FStar.Math.Lemmas.small_div r 8; FStar.Math.Lemmas.small_mod r 8)
   else (FStar.Math.Lemmas.lemma_div_plus (r - 8) 1 8;
         FStar.Math.Lemmas.lemma_mod_plus (r - 8) 1 8;
         FStar.Math.Lemmas.small_div (r - 8) 8;
         FStar.Math.Lemmas.small_mod (r - 8) 8))
#pop-options

(* Concrete contents of the gather-index / shift tables.  Proven on the explicit
   array_of_list literal so the seq_of_list index SMTPat fires (fuel reduces
   List.index); callers transfer by ground congruence (their array == this literal). *)
#push-options "--fuel 20 --ifuel 1 --z3rlimit 200"
let lemma_deser12_index_vals (u: unit)
    : Lemma
      (ensures
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 0 == mk_u8 0 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 1 == mk_u8 1 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 2 == mk_u8 1 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 3 == mk_u8 2 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 4 == mk_u8 3 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 5 == mk_u8 4 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 6 == mk_u8 4 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 7 == mk_u8 5 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 8 == mk_u8 6 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 9 == mk_u8 7 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 10 == mk_u8 7 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 11 == mk_u8 8 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 12 == mk_u8 9 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 13 == mk_u8 10 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 14 == mk_u8 10 /\
         Seq.index (Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ]) 15 == mk_u8 11) =
  let l = [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ] in
  assert_norm (List.Tot.length l == 16);
  FStar.Seq.Properties.lemma_seq_of_list_index l 0;
  FStar.Seq.Properties.lemma_seq_of_list_index l 1;
  FStar.Seq.Properties.lemma_seq_of_list_index l 2;
  FStar.Seq.Properties.lemma_seq_of_list_index l 3;
  FStar.Seq.Properties.lemma_seq_of_list_index l 4;
  FStar.Seq.Properties.lemma_seq_of_list_index l 5;
  FStar.Seq.Properties.lemma_seq_of_list_index l 6;
  FStar.Seq.Properties.lemma_seq_of_list_index l 7;
  FStar.Seq.Properties.lemma_seq_of_list_index l 8;
  FStar.Seq.Properties.lemma_seq_of_list_index l 9;
  FStar.Seq.Properties.lemma_seq_of_list_index l 10;
  FStar.Seq.Properties.lemma_seq_of_list_index l 11;
  FStar.Seq.Properties.lemma_seq_of_list_index l 12;
  FStar.Seq.Properties.lemma_seq_of_list_index l 13;
  FStar.Seq.Properties.lemma_seq_of_list_index l 14;
  FStar.Seq.Properties.lemma_seq_of_list_index l 15;
  assert_norm (List.Tot.index l 0 == mk_u8 0 /\
                List.Tot.index l 1 == mk_u8 1 /\
                List.Tot.index l 2 == mk_u8 1 /\
                List.Tot.index l 3 == mk_u8 2 /\
                List.Tot.index l 4 == mk_u8 3 /\
                List.Tot.index l 5 == mk_u8 4 /\
                List.Tot.index l 6 == mk_u8 4 /\
                List.Tot.index l 7 == mk_u8 5 /\
                List.Tot.index l 8 == mk_u8 6 /\
                List.Tot.index l 9 == mk_u8 7 /\
                List.Tot.index l 10 == mk_u8 7 /\
                List.Tot.index l 11 == mk_u8 8 /\
                List.Tot.index l 12 == mk_u8 9 /\
                List.Tot.index l 13 == mk_u8 10 /\
                List.Tot.index l 14 == mk_u8 10 /\
                List.Tot.index l 15 == mk_u8 11)
#pop-options

#push-options "--fuel 20 --ifuel 1 --z3rlimit 200"
let lemma_deser12_shift_vals (u: unit)
    : Lemma
      (ensures
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 0 == mk_i16 0 /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 1 == mk_i16 (-4) /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 2 == mk_i16 0 /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 3 == mk_i16 (-4) /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 4 == mk_i16 0 /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 5 == mk_i16 (-4) /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 6 == mk_i16 0 /\
         Seq.index (Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)]) 7 == mk_i16 (-4)) =
  let l = [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)] in
  assert_norm (List.Tot.length l == 8);
  FStar.Seq.Properties.lemma_seq_of_list_index l 0;
  FStar.Seq.Properties.lemma_seq_of_list_index l 1;
  FStar.Seq.Properties.lemma_seq_of_list_index l 2;
  FStar.Seq.Properties.lemma_seq_of_list_index l 3;
  FStar.Seq.Properties.lemma_seq_of_list_index l 4;
  FStar.Seq.Properties.lemma_seq_of_list_index l 5;
  FStar.Seq.Properties.lemma_seq_of_list_index l 6;
  FStar.Seq.Properties.lemma_seq_of_list_index l 7;
  assert_norm (List.Tot.index l 0 == mk_i16 0 /\
                List.Tot.index l 1 == mk_i16 (-4) /\
                List.Tot.index l 2 == mk_i16 0 /\
                List.Tot.index l 3 == mk_i16 (-4) /\
                List.Tot.index l 4 == mk_i16 0 /\
                List.Tot.index l 5 == mk_i16 (-4) /\
                List.Tot.index l 6 == mk_i16 0 /\
                List.Tot.index l 7 == mk_i16 (-4))
#pop-options

#push-options "--fuel 20 --ifuel 1 --z3rlimit 100"
let lemma_deser12_index_lanes (index_vec: NI.t_e_uint8x16_t) (indexes: t_Array u8 (mk_usize 16))
    : Lemma
      (requires
        (forall (i: nat{i < 16}). NI.get_lane_u8x16 index_vec i == Seq.index indexes i) /\
        indexes == Rust_primitives.Hax.array_of_list 16 [ mk_u8 0; mk_u8 1; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 10; mk_u8 11 ])
      (ensures
        v (NI.get_lane_u8x16 index_vec 0) == 0 /\
        v (NI.get_lane_u8x16 index_vec 1) == 1 /\
        v (NI.get_lane_u8x16 index_vec 2) == 1 /\
        v (NI.get_lane_u8x16 index_vec 3) == 2 /\
        v (NI.get_lane_u8x16 index_vec 4) == 3 /\
        v (NI.get_lane_u8x16 index_vec 5) == 4 /\
        v (NI.get_lane_u8x16 index_vec 6) == 4 /\
        v (NI.get_lane_u8x16 index_vec 7) == 5 /\
        v (NI.get_lane_u8x16 index_vec 8) == 6 /\
        v (NI.get_lane_u8x16 index_vec 9) == 7 /\
        v (NI.get_lane_u8x16 index_vec 10) == 7 /\
        v (NI.get_lane_u8x16 index_vec 11) == 8 /\
        v (NI.get_lane_u8x16 index_vec 12) == 9 /\
        v (NI.get_lane_u8x16 index_vec 13) == 10 /\
        v (NI.get_lane_u8x16 index_vec 14) == 10 /\
        v (NI.get_lane_u8x16 index_vec 15) == 11) =
  lemma_deser12_index_vals ()
#pop-options

#push-options "--fuel 20 --ifuel 1 --z3rlimit 100"
let lemma_deser12_shift_lanes (shift_vec: NI.t_e_int16x8_t) (shifts: t_Array i16 (mk_usize 8))
    : Lemma
      (requires
        (forall (i: nat{i < 8}). NI.get_lane_i16x8 shift_vec i == Seq.index shifts i) /\
        shifts == Rust_primitives.Hax.array_of_list 8 [mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4); mk_i16 0; mk_i16 (-4)])
      (ensures
        v (NI.get_lane_i16x8 shift_vec 0 %! mk_i16 256) == 0 /\
        v (NI.get_lane_i16x8 shift_vec 1 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 2 %! mk_i16 256) == 0 /\
        v (NI.get_lane_i16x8 shift_vec 3 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 4 %! mk_i16 256) == 0 /\
        v (NI.get_lane_i16x8 shift_vec 5 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 6 %! mk_i16 256) == 0 /\
        v (NI.get_lane_i16x8 shift_vec 7 %! mk_i16 256) == 252) =
  lemma_deser12_shift_vals ()
#pop-options

(* Clean-context per-coefficient dispatcher: given the loaded vectors with their
   concrete per-lane facts (index table, shifts, mask, input bytes), coefficient
   cc of repr(result) is 12-bit bounded and its bits track the packed byte stream. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_deser12_dispatch
      (inp: t_Slice u8) (result: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (index_vec input_vec0 input_vec1: NI.t_e_uint8x16_t)
      (shift_vec: NI.t_e_int16x8_t) (mask12: NI.t_e_uint16x8_t)
      (cc: nat{cc < 16})
    : Lemma
      (requires
        Seq.length inp == 24 /\
        result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low ==
          NI.e_vreinterpretq_s16_u16 (NI.e_vandq_u16 (NI.e_vshlq_u16 (NI.e_vreinterpretq_u16_u8 (NI.e_vqtbl1q_u8 input_vec0 index_vec)) shift_vec) mask12) /\
        result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high ==
          NI.e_vreinterpretq_s16_u16 (NI.e_vandq_u16 (NI.e_vshlq_u16 (NI.e_vreinterpretq_u16_u8 (NI.e_vqtbl1q_u8 input_vec1 index_vec)) shift_vec) mask12) /\
        v (NI.get_lane_u8x16 index_vec 0) == 0 /\ v (NI.get_lane_u8x16 index_vec 1) == 1 /\
        v (NI.get_lane_u8x16 index_vec 2) == 1 /\ v (NI.get_lane_u8x16 index_vec 3) == 2 /\
        v (NI.get_lane_u8x16 index_vec 4) == 3 /\ v (NI.get_lane_u8x16 index_vec 5) == 4 /\
        v (NI.get_lane_u8x16 index_vec 6) == 4 /\ v (NI.get_lane_u8x16 index_vec 7) == 5 /\
        v (NI.get_lane_u8x16 index_vec 8) == 6 /\ v (NI.get_lane_u8x16 index_vec 9) == 7 /\
        v (NI.get_lane_u8x16 index_vec 10) == 7 /\ v (NI.get_lane_u8x16 index_vec 11) == 8 /\
        v (NI.get_lane_u8x16 index_vec 12) == 9 /\ v (NI.get_lane_u8x16 index_vec 13) == 10 /\
        v (NI.get_lane_u8x16 index_vec 14) == 10 /\ v (NI.get_lane_u8x16 index_vec 15) == 11 /\
        (forall (i: nat{i < 8}). NI.get_lane_u16x8 mask12 i == mk_u16 4095) /\
        v (NI.get_lane_i16x8 shift_vec 0 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 1 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 2 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 3 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 4 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 5 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 6 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 7 %! mk_i16 256) == 252 /\
        (forall (i: nat{i < 12}). NI.get_lane_u8x16 input_vec0 i == Seq.index inp i) /\
        (forall (i: nat{i < 12}). NI.get_lane_u8x16 input_vec1 i == Seq.index inp (12 + i)))
      (ensures
        (let rr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr result in
         BV.bounded (Seq.index rr cc) 12 /\
         (forall (b: nat{b < 12}). I.get_bit (Seq.index rr cc) (sz b) ==
            I.get_bit (Seq.index inp ((12 * cc + b) / 8)) (sz ((12 * cc + b) % 8))))) =
  let rr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr result in
  let _ = Seq.index rr cc in
  (match cc with
   | 0  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 0 0 1 (Seq.index inp 0) (Seq.index inp 1) 0 0 0 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 0) (sz b) == I.get_bit (Seq.index inp ((12 * 0 + b) / 8)) (sz ((12 * 0 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 0 0 1 (Seq.index inp 0) (Seq.index inp 1) 0 0 0 b
   | 1  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 1 1 2 (Seq.index inp 1) (Seq.index inp 2) 4 1 1 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 1) (sz b) == I.get_bit (Seq.index inp ((12 * 1 + b) / 8)) (sz ((12 * 1 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 1 1 2 (Seq.index inp 1) (Seq.index inp 2) 4 1 1 b
   | 2  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 2 3 4 (Seq.index inp 3) (Seq.index inp 4) 0 3 2 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 2) (sz b) == I.get_bit (Seq.index inp ((12 * 2 + b) / 8)) (sz ((12 * 2 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 2 3 4 (Seq.index inp 3) (Seq.index inp 4) 0 3 2 b
   | 3  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 3 4 5 (Seq.index inp 4) (Seq.index inp 5) 4 4 3 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 3) (sz b) == I.get_bit (Seq.index inp ((12 * 3 + b) / 8)) (sz ((12 * 3 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 3 4 5 (Seq.index inp 4) (Seq.index inp 5) 4 4 3 b
   | 4  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 4 6 7 (Seq.index inp 6) (Seq.index inp 7) 0 6 4 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 4) (sz b) == I.get_bit (Seq.index inp ((12 * 4 + b) / 8)) (sz ((12 * 4 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 4 6 7 (Seq.index inp 6) (Seq.index inp 7) 0 6 4 b
   | 5  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 5 7 8 (Seq.index inp 7) (Seq.index inp 8) 4 7 5 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 5) (sz b) == I.get_bit (Seq.index inp ((12 * 5 + b) / 8)) (sz ((12 * 5 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 5 7 8 (Seq.index inp 7) (Seq.index inp 8) 4 7 5 b
   | 6  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 6 9 10 (Seq.index inp 9) (Seq.index inp 10) 0 9 6 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 6) (sz b) == I.get_bit (Seq.index inp ((12 * 6 + b) / 8)) (sz ((12 * 6 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 6 9 10 (Seq.index inp 9) (Seq.index inp 10) 0 9 6 b
   | 7  -> lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 7 10 11 (Seq.index inp 10) (Seq.index inp 11) 4 10 7 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 7) (sz b) == I.get_bit (Seq.index inp ((12 * 7 + b) / 8)) (sz ((12 * 7 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec0 index_vec shift_vec mask12 7 10 11 (Seq.index inp 10) (Seq.index inp 11) 4 10 7 b
   | 8  -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 0 0 1 (Seq.index inp 12) (Seq.index inp 13) 0 12 8 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 8) (sz b) == I.get_bit (Seq.index inp ((12 * 8 + b) / 8)) (sz ((12 * 8 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 0 0 1 (Seq.index inp 12) (Seq.index inp 13) 0 12 8 b
   | 9  -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 1 1 2 (Seq.index inp 13) (Seq.index inp 14) 4 13 9 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 9) (sz b) == I.get_bit (Seq.index inp ((12 * 9 + b) / 8)) (sz ((12 * 9 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 1 1 2 (Seq.index inp 13) (Seq.index inp 14) 4 13 9 b
   | 10 -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 2 3 4 (Seq.index inp 15) (Seq.index inp 16) 0 15 10 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 10) (sz b) == I.get_bit (Seq.index inp ((12 * 10 + b) / 8)) (sz ((12 * 10 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 2 3 4 (Seq.index inp 15) (Seq.index inp 16) 0 15 10 b
   | 11 -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 3 4 5 (Seq.index inp 16) (Seq.index inp 17) 4 16 11 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 11) (sz b) == I.get_bit (Seq.index inp ((12 * 11 + b) / 8)) (sz ((12 * 11 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 3 4 5 (Seq.index inp 16) (Seq.index inp 17) 4 16 11 b
   | 12 -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 4 6 7 (Seq.index inp 18) (Seq.index inp 19) 0 18 12 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 12) (sz b) == I.get_bit (Seq.index inp ((12 * 12 + b) / 8)) (sz ((12 * 12 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 4 6 7 (Seq.index inp 18) (Seq.index inp 19) 0 18 12 b
   | 13 -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 5 7 8 (Seq.index inp 19) (Seq.index inp 20) 4 19 13 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 13) (sz b) == I.get_bit (Seq.index inp ((12 * 13 + b) / 8)) (sz ((12 * 13 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 5 7 8 (Seq.index inp 19) (Seq.index inp 20) 4 19 13 b
   | 14 -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 6 9 10 (Seq.index inp 21) (Seq.index inp 22) 0 21 14 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 14) (sz b) == I.get_bit (Seq.index inp ((12 * 14 + b) / 8)) (sz ((12 * 14 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 6 9 10 (Seq.index inp 21) (Seq.index inp 22) 0 21 14 b
   | _  -> lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 7 10 11 (Seq.index inp 22) (Seq.index inp 23) 4 22 15 0;
           introduce forall (b: nat{b < 12}). I.get_bit (Seq.index rr 15) (sz b) == I.get_bit (Seq.index inp ((12 * 15 + b) / 8)) (sz ((12 * 15 + b) % 8))
           with lemma_deser12_coeff_bit inp input_vec1 index_vec shift_vec mask12 7 10 11 (Seq.index inp 22) (Seq.index inp 23) 4 22 15 b)
#pop-options

(* Assemble deserialize_post_N from the per-coefficient dispatcher + BitVec stitch,
   in clean context (off the leaf's heavy construction WP). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_deser12_post
      (inp: t_Slice u8) (result: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (index_vec input_vec0 input_vec1: NI.t_e_uint8x16_t)
      (shift_vec: NI.t_e_int16x8_t) (mask12: NI.t_e_uint16x8_t)
    : Lemma
      (requires
        Seq.length inp == 24 /\
        result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low ==
          NI.e_vreinterpretq_s16_u16 (NI.e_vandq_u16 (NI.e_vshlq_u16 (NI.e_vreinterpretq_u16_u8 (NI.e_vqtbl1q_u8 input_vec0 index_vec)) shift_vec) mask12) /\
        result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high ==
          NI.e_vreinterpretq_s16_u16 (NI.e_vandq_u16 (NI.e_vshlq_u16 (NI.e_vreinterpretq_u16_u8 (NI.e_vqtbl1q_u8 input_vec1 index_vec)) shift_vec) mask12) /\
        v (NI.get_lane_u8x16 index_vec 0) == 0 /\ v (NI.get_lane_u8x16 index_vec 1) == 1 /\
        v (NI.get_lane_u8x16 index_vec 2) == 1 /\ v (NI.get_lane_u8x16 index_vec 3) == 2 /\
        v (NI.get_lane_u8x16 index_vec 4) == 3 /\ v (NI.get_lane_u8x16 index_vec 5) == 4 /\
        v (NI.get_lane_u8x16 index_vec 6) == 4 /\ v (NI.get_lane_u8x16 index_vec 7) == 5 /\
        v (NI.get_lane_u8x16 index_vec 8) == 6 /\ v (NI.get_lane_u8x16 index_vec 9) == 7 /\
        v (NI.get_lane_u8x16 index_vec 10) == 7 /\ v (NI.get_lane_u8x16 index_vec 11) == 8 /\
        v (NI.get_lane_u8x16 index_vec 12) == 9 /\ v (NI.get_lane_u8x16 index_vec 13) == 10 /\
        v (NI.get_lane_u8x16 index_vec 14) == 10 /\ v (NI.get_lane_u8x16 index_vec 15) == 11 /\
        (forall (i: nat{i < 8}). NI.get_lane_u16x8 mask12 i == mk_u16 4095) /\
        v (NI.get_lane_i16x8 shift_vec 0 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 1 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 2 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 3 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 4 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 5 %! mk_i16 256) == 252 /\
        v (NI.get_lane_i16x8 shift_vec 6 %! mk_i16 256) == 0 /\ v (NI.get_lane_i16x8 shift_vec 7 %! mk_i16 256) == 252 /\
        (forall (i: nat{i < 12}). NI.get_lane_u8x16 input_vec0 i == Seq.index inp i) /\
        (forall (i: nat{i < 12}). NI.get_lane_u8x16 input_vec1 i == Seq.index inp (12 + i)))
      (ensures
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 inp
          (Libcrux_ml_kem.Vector.Neon.Vector_type.repr result)) =
  let rr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr result in
  let aux (cc: nat{cc < 16}) : Lemma
      (BV.bounded (Seq.index rr cc) 12 /\
       (forall (b: nat{b < 12}). I.get_bit (Seq.index rr cc) (sz b) ==
          I.get_bit (Seq.index inp ((12 * cc + b) / 8)) (sz ((12 * cc + b) % 8)))) =
    lemma_deser12_dispatch inp result index_vec input_vec0 input_vec1 shift_vec mask12 cc
  in
  Classical.forall_intro aux;
  let n8: usize = sz (Seq.length inp) in
  let input_arr: t_Array u8 n8 = inp in
  let bv_in: BV.bit_vec (v n8 * 8) = BV.bit_vec_of_int_t_array #u8_inttype #n8 input_arr 8 in
  let bv_out: BV.bit_vec (16 * 12) = BV.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) rr 12 in
  introduce forall (i: nat{i < 192}). bv_in i == bv_out i
  with (FStar.Math.Lemmas.lemma_div_mod i 12;
        assert (12 * (i / 12) + (i % 12) == i);
        assert (bv_out i == I.get_bit (Seq.index rr (i / 12)) (sz (i % 12)));
        assert (bv_in i == I.get_bit (Seq.index inp (i / 8)) (sz (i % 8))));
  BitVecEq.bit_vec_equal_intro bv_in (BitVecEq.retype bv_out)
#pop-options
"#
)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always --z3refresh")]
#[hax_lib::requires(v.len() == 24)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_12_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_12(v: &[u8]) -> SIMD128Vector {
    let indexes: [u8; 16] = [0, 1, 1, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 10, 10, 11];
    let index_vec = _vld1q_u8(&indexes);
    let shifts: [i16; 8] = [0, -4, 0, -4, 0, -4, 0, -4];
    let shift_vec = _vld1q_s16(&shifts);
    let mask12 = _vdupq_n_u16(0xfff);

    let mut input0 = [0u8; 16];
    input0[0..12].copy_from_slice(&v[0..12]);
    let input_vec0 = _vld1q_u8(&input0);

    let mut input1 = [0u8; 16];
    input1[0..12].copy_from_slice(&v[12..24]);
    let input_vec1 = _vld1q_u8(&input1);

    let moved0 = _vreinterpretq_u16_u8(_vqtbl1q_u8(input_vec0, index_vec));
    let shifted0 = _vshlq_u16(moved0, shift_vec);
    let low = _vreinterpretq_s16_u16(_vandq_u16(shifted0, mask12));

    let moved1 = _vreinterpretq_u16_u8(_vqtbl1q_u8(input_vec1, index_vec));
    let shifted1 = _vshlq_u16(moved1, shift_vec);
    let high = _vreinterpretq_s16_u16(_vandq_u16(shifted1, mask12));

    let result = SIMD128Vector { low, high };
    hax_lib::fstar!(
        r#"(* mask lanes *)
assert (forall (i: nat{i < 8}). NI.get_lane_u16x8 ${mask12} i == mk_u16 4095);
(* ${index_vec} lanes + concrete index values *)
assert (forall (i: nat{i < 16}). NI.get_lane_u8x16 ${index_vec} i == Seq.index ${indexes} i);
lemma_deser12_index_lanes ${index_vec} ${indexes};
(* shift lanes + concrete shift values *)
assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${shift_vec} i == Seq.index ${shifts} i);
lemma_deser12_shift_lanes ${shift_vec} ${shifts};
(* input0 / input1 vs ${v} *)
assert ((${v}.[ { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 12 } <: Core_models.Ops.Range.t_Range usize ]) == Seq.slice ${v} 0 12);
assert ((${v}.[ { Core_models.Ops.Range.f_start = mk_usize 12; Core_models.Ops.Range.f_end = mk_usize 24 } <: Core_models.Ops.Range.t_Range usize ]) == Seq.slice ${v} 12 24);
Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range #u8
  (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16))
  ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 12 } <: Core_models.Ops.Range.t_Range usize)
  (Seq.slice ${v} 0 12);
Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range #u8
  (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16))
  ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 12 } <: Core_models.Ops.Range.t_Range usize)
  (Seq.slice ${v} 12 24);
assert (forall (i: nat{i < 12}). NI.get_lane_u8x16 ${input_vec0} i == Seq.index ${v} i);
assert (forall (i: nat{i < 12}). NI.get_lane_u8x16 ${input_vec1} i == Seq.index ${v} (12 + i));
lemma_deser12_post ${v} ${result} ${index_vec} ${input_vec0} ${input_vec1} ${shift_vec} ${mask12}"#
    );
    result
}
