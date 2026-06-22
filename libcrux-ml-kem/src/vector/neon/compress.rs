use super::vector_type::*;
use crate::vector::FIELD_MODULUS;
use libcrux_intrinsics::arm64::*;

// Helper lemmas backing the functional postcondition of `compress_1`.
// `repr` is `low(8) ++ high(8)`; `lemma_repr_index` (Vector_type) bridges a
// `repr` index to the corresponding `get_lane_i16x8` of `.f_low` / `.f_high`.
#[hax_lib::fstar::before(
    r#"
(* The modular-reduction core of compress_1 (the step AVX2's
   compress_message_coefficient leaves as `assume`).  For 0 <= vec_i < 3329,
   the message-compression `((vec_i*4+3329)/6658) % 2` equals the indicator
   `833 <= vec_i <= 2496`, which is precisely the bit-15 extraction the SIMD
   chain computes. *)
let lemma_compress_1_arith (vec_i: int) : Lemma
  (requires vec_i >= 0 /\ vec_i < 3329)
  (ensures ((vec_i * 4 + 3329) / 6658) % 2 == (if 833 <= vec_i && vec_i <= 2496 then 1 else 0))
  = ()

(* >>! 15 on i16 (arithmetic shift) is sign extension: -1 if negative, else 0 *)
let lemma_i16_arith_shr_15 (x: i16) : Lemma
  (ensures v (x >>! mk_i32 15) == (if v x < 0 then -1 else 0))
  [SMTPat (x >>! mk_i32 15)]
  = ()

(* xor of an i16 with all-ones (-1) is bitwise NOT (-x-1); xor with 0 is id. *)
let lemma_i16_xor_neg1 (x: i16) : Lemma
  (ensures v (x ^. mk_i16 (-1)) == -(v x) - 1)
  [SMTPat (x ^. mk_i16 (-1))]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 (-1));
    Rust_primitives.Integers.lognot_lemma x

let lemma_i16_xor_zero (x: i16) : Lemma
  (ensures v (x ^. mk_i16 0) == v x)
  [SMTPat (x ^. mk_i16 0)]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 0)

(* xor where the MASK (all-ones / all-zeros) is the FIRST operand, as the
   SIMD chain produces it (`mask ^. shifted`).  Mirrors the sign-mask trick:
   m == -1 gives bitwise NOT of s, m == 0 gives s unchanged. *)
let lemma_i16_xor_mask_left (m s: i16) : Lemma
  (requires v m == -1 \/ v m == 0)
  (ensures v (m ^. s) == (if v m = -1 then -(v s) - 1 else v s))
  = Rust_primitives.Integers.logxor_lemma s s;
    Rust_primitives.Integers.lognot_lemma s;
    if v m = -1
    then assert (m == Rust_primitives.Integers.ones)
    else assert (m == Rust_primitives.Integers.zero)

(* The reinterpret/logical-shr-15/reinterpret tail extracts bit 15 of x's
   16-bit representation: 1 iff x < 0, else 0. *)
let lemma_tail_bit15 (x: i16) : Lemma
  (ensures (let u  = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                       #Rust_primitives.Integers.u16_inttype x in
            let sh = u >>! mk_i32 15 in
            let o  = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
                       #Rust_primitives.Integers.i16_inttype sh in
            v o == (if v x < 0 then 1 else 0)))
  = if v x < 0 then begin
      assert (v (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                   #Rust_primitives.Integers.u16_inttype x) == v x + pow2 16);
      assert ((v x + pow2 16) / pow2 15 == 1)
    end else begin
      assert (v (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                   #Rust_primitives.Integers.u16_inttype x) == v x);
      assert (v x / pow2 15 == 0)
    end

#push-options "--z3rlimit 300 --split_queries always"

(* Per-half characterization of the compress_1 SIMD chain on one i16x8.
   For each lane with input in [0,3329), the chain output lane equals
   `((vec_k*4+3329)/6658) % 2`, and that value is in {0,1}. *)
let lemma_compress_1_half (vin: Libcrux_intrinsics.Arm64_extract.t_e_int16x8_t)
    : Lemma
      (requires
        (forall (k: nat{k < 8}).
            v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vin k) >= 0 /\
            v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vin k) < 3329))
      (ensures
        (let half = Libcrux_intrinsics.Arm64_extract.e_vdupq_n_s16 (mk_i16 1664) in
         let quarter = Libcrux_intrinsics.Arm64_extract.e_vdupq_n_s16 (mk_i16 832) in
         let shifted = Libcrux_intrinsics.Arm64_extract.e_vsubq_s16 half vin in
         let mask = Libcrux_intrinsics.Arm64_extract.e_vshrq_n_s16 (mk_i32 15) shifted in
         let stp = Libcrux_intrinsics.Arm64_extract.e_veorq_s16 mask shifted in
         let spir = Libcrux_intrinsics.Arm64_extract.e_vsubq_s16 stp quarter in
         let out =
           Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_s16_u16
             (Libcrux_intrinsics.Arm64_extract.e_vshrq_n_u16 (mk_i32 15)
               (Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 spir)) in
         forall (k: nat{k < 8}).
           (let vec_k = v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vin k) in
            let res_k = v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 out k) in
            res_k >= 0 /\ res_k < 2 /\ res_k == ((vec_k * 4 + 3329) / 6658) % 2)))
  = let half = Libcrux_intrinsics.Arm64_extract.e_vdupq_n_s16 (mk_i16 1664) in
    let quarter = Libcrux_intrinsics.Arm64_extract.e_vdupq_n_s16 (mk_i16 832) in
    let shifted = Libcrux_intrinsics.Arm64_extract.e_vsubq_s16 half vin in
    let mask = Libcrux_intrinsics.Arm64_extract.e_vshrq_n_s16 (mk_i32 15) shifted in
    let stp = Libcrux_intrinsics.Arm64_extract.e_veorq_s16 mask shifted in
    let spir = Libcrux_intrinsics.Arm64_extract.e_vsubq_s16 stp quarter in
    let u16v = Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_u16_s16 spir in
    let sh = Libcrux_intrinsics.Arm64_extract.e_vshrq_n_u16 (mk_i32 15) u16v in
    let out = Libcrux_intrinsics.Arm64_extract.e_vreinterpretq_s16_u16 sh in
    let aux (k: nat{k < 8}) : Lemma
      (let vec_k = v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vin k) in
       let res_k = v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 out k) in
       res_k >= 0 /\ res_k < 2 /\ res_k == ((vec_k * 4 + 3329) / 6658) % 2) =
      let vec_k = v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 vin k) in
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 half k) == 1664);
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 quarter k) == 832);
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shifted k) == 1664 - vec_k);
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 mask k) ==
              (if 1664 - vec_k < 0 then -1 else 0));
      lemma_i16_xor_mask_left (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 mask k)
                              (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 shifted k);
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 stp k) ==
              (if 1664 - vec_k < 0 then -(1664 - vec_k) - 1 else 1664 - vec_k));
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 spir k) ==
              (if 1664 - vec_k < 0 then vec_k - 2497 else 832 - vec_k));
      assert ((v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 spir k) < 0) ==
              (833 <= vec_k && vec_k <= 2496));
      lemma_tail_bit15 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 spir k);
      assert (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 out k) ==
              (if 833 <= vec_k && vec_k <= 2496 then 1 else 0));
      lemma_compress_1_arith vec_k
    in
    Classical.forall_intro aux

#pop-options
"#
)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall (i: nat).
    i < 16 ==>
    Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) i) >= 0 /\
    Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) i) < 3329"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat).
    i < 16 ==>
    (let res_i = Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result}) i) in
     let vec_i = Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) i) in
     res_i >= 0 /\ res_i < 2 /\ res_i == ((vec_i * 4 + 3329) / 6658) % 2)"#))]
pub(crate) fn compress_1(mut v: SIMD128Vector) -> SIMD128Vector {
    // Per-half functional characterization, established on the inputs before mutation.
    // The two asserts bridge the repr-level precondition to per-lane get_lane bounds
    // (`lemma_repr_index` SMTPat), which discharge `lemma_compress_1_half`'s requires.
    hax_lib::fstar!(
        r#"assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) k ==
                Libcrux_intrinsics.Arm64_extract.get_lane_i16x8
                  ${v}.f_low k);
           assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) (k + 8) ==
                Libcrux_intrinsics.Arm64_extract.get_lane_i16x8
                  ${v}.f_high k);
           lemma_compress_1_half ${v}.f_low; lemma_compress_1_half ${v}.f_high"#
    );
    // This is what we are trying to do in portable:
    // let shifted: i16 = 1664 - (fe as i16);
    // let mask = shifted >> 15;
    // let shifted_to_positive = mask ^ shifted;
    // let shifted_positive_in_range = shifted_to_positive - 832;
    // ((shifted_positive_in_range >> 15) & 1) as u8

    let half = _vdupq_n_s16(1664);
    let quarter = _vdupq_n_s16(832);

    let shifted = _vsubq_s16(half, v.low);
    let mask = _vshrq_n_s16::<15>(shifted);
    let shifted_to_positive = _veorq_s16(mask, shifted);
    let shifted_positive_in_range = _vsubq_s16(shifted_to_positive, quarter);
    v.low = _vreinterpretq_s16_u16(_vshrq_n_u16::<15>(_vreinterpretq_u16_s16(
        shifted_positive_in_range,
    )));

    let shifted = _vsubq_s16(half, v.high);
    let mask = _vshrq_n_s16::<15>(shifted);
    let shifted_to_positive = _veorq_s16(mask, shifted);
    let shifted_positive_in_range = _vsubq_s16(shifted_to_positive, quarter);
    v.high = _vreinterpretq_s16_u16(_vshrq_n_u16::<15>(_vreinterpretq_u16_s16(
        shifted_positive_in_range,
    )));

    // Bridge `repr v` indices (lanes 0..7 from f_low, 8..15 from f_high) to the
    // per-half chain outputs characterized above, via `lemma_repr_index`.
    hax_lib::fstar!(
        r#"assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) k ==
                Libcrux_intrinsics.Arm64_extract.get_lane_i16x8
                  ${v}.f_low k);
           assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) (k + 8) ==
                Libcrux_intrinsics.Arm64_extract.get_lane_i16x8
                  ${v}.f_high k)"#
    );
    v
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"v $coefficient_bits >= 0 /\ v $coefficient_bits < 15"#))]
#[hax_lib::ensures(|result| fstar!(r#"v ${result} == pow2 (v ${coefficient_bits}) - 1"#))]
fn mask_n_least_significant_bits(coefficient_bits: i16) -> i16 {
    match coefficient_bits {
        4 => {
            hax_lib::fstar!(r#"assert_norm (pow2 4 - 1 == 15)"#);
            0x0f
        }
        5 => {
            hax_lib::fstar!(r#"assert_norm (pow2 5 - 1 == 31)"#);
            0x1f
        }
        10 => {
            hax_lib::fstar!(r#"assert_norm (pow2 10 - 1 == 1023)"#);
            0x3ff
        }
        11 => {
            hax_lib::fstar!(r#"assert_norm (pow2 11 - 1 == 2047)"#);
            0x7ff
        }
        x => {
            // catch-all is only reachable for coefficient_bits in [0, 15);
            // pow2 (v x) <= pow2 14 = 16384 < 2^15, so (1 << x) fits i16 and
            // (1 << x) - 1 >= 0 cannot underflow.
            hax_lib::fstar!(
                r#"FStar.Math.Lemmas.pow2_le_compat 14 (v $x);
                   assert_norm (pow2 14 == 16384)"#
            );
            (1 << x) - 1
        }
    }
}

#[inline(always)]
fn compress_int32x4_t<const COEFFICIENT_BITS: i32>(v: _uint32x4_t) -> _uint32x4_t {
    // This is what we are trying to do in portable:
    // let mut compressed = (fe as u64) << coefficient_bits;
    // compressed += 1664 as u64;
    // compressed *= 10_321_340;
    // compressed >>= 35;
    // get_n_least_significant_bits(coefficient_bits, compressed as u32) as FieldElement
    let half = _vdupq_n_u32(1664);
    let compressed = _vshlq_n_u32::<COEFFICIENT_BITS>(v);
    let compressed = _vaddq_u32(compressed, half);
    let compressed = _vreinterpretq_u32_s32(_vqdmulhq_n_s32(
        _vreinterpretq_s32_u32(compressed),
        10_321_340,
    ));
    let compressed = _vshrq_n_u32::<4>(compressed);
    compressed
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v $COEFFICIENT_BITS == 4 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 5 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 10 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 11"#))]
pub(crate) fn compress<const COEFFICIENT_BITS: i32>(mut v: SIMD128Vector) -> SIMD128Vector {
    // This is what we are trying to do in portable:
    // let mut compressed = (fe as u64) << coefficient_bits;
    // compressed += 1664 as u64;
    // compressed *= 10_321_340;
    // compressed >>= 35;
    // get_n_least_significant_bits(coefficient_bits, compressed as u32) as FieldElement

    // The `i32 -> i16` cast preserves the value for COEFFICIENT_BITS in {4,5,10,11},
    // which discharges `mask_n_least_significant_bits`'s `< 15` precondition.
    // `v` is qualified because the vector parameter is named `v`.
    hax_lib::fstar!(
        r#"assert (Rust_primitives.Integers.v (cast ($COEFFICIENT_BITS <: i32) <: i16) ==
            Rust_primitives.Integers.v $COEFFICIENT_BITS)"#
    );

    let mask = _vdupq_n_s16(mask_n_least_significant_bits(COEFFICIENT_BITS as i16));
    let mask16 = _vdupq_n_u32(0xffff);

    let low0 = _vandq_u32(_vreinterpretq_u32_s16(v.low), mask16); //a0, a2, a4, a6
    let low1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.low)); //a1, a3, a5, a7
    let high0 = _vandq_u32(_vreinterpretq_u32_s16(v.high), mask16); //a0, a2, a4, a6
    let high1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.high)); //a1, a3, a5, a7

    let low0 = compress_int32x4_t::<COEFFICIENT_BITS>(low0);
    let low1 = compress_int32x4_t::<COEFFICIENT_BITS>(low1);
    let high0 = compress_int32x4_t::<COEFFICIENT_BITS>(high0);
    let high1 = compress_int32x4_t::<COEFFICIENT_BITS>(high1);

    let low = _vtrn1q_s16(_vreinterpretq_s16_u32(low0), _vreinterpretq_s16_u32(low1));
    let high = _vtrn1q_s16(_vreinterpretq_s16_u32(high0), _vreinterpretq_s16_u32(high1));

    v.low = _vandq_s16(low, mask);
    v.high = _vandq_s16(high, mask);
    v
}

// d-bit Neon decompress: the per-u32-lane decompress core lemmas.  EXACT
// division: each lane computes `(a*3329 + 2^(d-1)) / 2^d`.  No trusted-base
// extension: the Arm64 intrinsic lane models are reused.
#[hax_lib::fstar::before(
    r#"
module NA = Libcrux_intrinsics.Arm64_extract

(* `1 <<! (cb-1)` as u32 has value 2^(d-1). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_neon_twopow_m1 (cb: i32)
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11))
          (ensures Rust_primitives.Integers.v (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32) ==
                   pow2 (v cb - 1))
  = assert_norm (pow2 10 == 1024); assert_norm (pow2 31 == 2147483648);
    assert_norm (pow2 32 == 4294967296);
    FStar.Math.Lemmas.pow2_le_compat 10 (v cb - 1)
#pop-options

(* clean-context bound: (a*3329 + 2^(d-1)) / 2^d < 3329 for a < 2^d. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 80"
let lemma_decompress_u32_bound (a dd: nat)
  : Lemma (requires a < pow2 dd /\ (dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11))
          (ensures a * 3329 + pow2 (dd - 1) < 4294967296 /\
                   (a * 3329 + pow2 (dd - 1)) / pow2 dd < 3329)
  = assert_norm (pow2 4 == 16); assert_norm (pow2 5 == 32);
    assert_norm (pow2 10 == 1024); assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    FStar.Math.Lemmas.pow2_le_compat 10 (dd - 1);
    FStar.Math.Lemmas.lemma_mult_le_right 3329 a (pow2 dd - 1);
    let n = a * 3329 + pow2 (dd - 1) in
    FStar.Math.Lemmas.pow2_double_mult (dd - 1);
    assert (n < 3329 * pow2 dd);
    FStar.Math.Lemmas.lemma_div_plus (-1) 3329 (pow2 dd);
    FStar.Math.Lemmas.lemma_div_le n (3329 * pow2 dd - 1) (pow2 dd)
#pop-options

(* per-u32-lane decompress core, proven standalone (param `vv`, no `v` shadow). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_decompress_u32_lane (vv: NA.t_e_uint32x4_t) (cb: i32) (k: nat{k < 4})
  : Lemma
    (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
              Rust_primitives.Integers.v (NA.get_lane_u32x4 vv k) < pow2 (v cb))
    (ensures
      (let coeff = NA.e_vdupq_n_u32 (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32) in
       let d1 = NA.e_vmulq_n_u32 vv (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: u32) in
       let d2 = NA.e_vaddq_u32 d1 coeff in
       let r = NA.e_vshrq_n_u32 cb d2 in
       let a = Rust_primitives.Integers.v (NA.get_lane_u32x4 vv k) in
       Rust_primitives.Integers.v (NA.get_lane_u32x4 r k) ==
         (a * 3329 + pow2 (v cb - 1)) / pow2 (v cb) /\
       Rust_primitives.Integers.v (NA.get_lane_u32x4 r k) < 3329))
  = let a = Rust_primitives.Integers.v (NA.get_lane_u32x4 vv k) in
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb);
    assert_norm (Rust_primitives.Integers.v
      (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: u32) == 3329);
    lemma_neon_twopow_m1 cb;
    lemma_decompress_u32_bound a (v cb);
    let coeff = NA.e_vdupq_n_u32 (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32) in
    let d1 = NA.e_vmulq_n_u32 vv (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: u32) in
    let d2 = NA.e_vaddq_u32 d1 coeff in
    (* coeff lane k == 2^(d-1) *)
    assert (NA.get_lane_u32x4 coeff k == (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32));
    (* a * 3329 < 2^32 so the u32 mul does not wrap *)
    FStar.Math.Lemmas.lemma_mult_le_right 3329 a 2047;
    assert (Rust_primitives.Integers.v (NA.get_lane_u32x4 d1 k) == a * 3329);
    assert (Rust_primitives.Integers.v (NA.get_lane_u32x4 d2 k) == a * 3329 + pow2 (v cb - 1))
#pop-options
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[inline(always)]
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v $COEFFICIENT_BITS == 4 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 5 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 10 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 11"#))]
#[hax_lib::ensures(|result| fstar!(r#"(forall (k: nat). k < 4 ==>
    Rust_primitives.Integers.v (Libcrux_intrinsics.Arm64_extract.get_lane_u32x4 ${v} k) <
    pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS)) ==>
  (forall (k: nat). k < 4 ==>
    (let a = Rust_primitives.Integers.v (Libcrux_intrinsics.Arm64_extract.get_lane_u32x4 ${v} k) in
     let r = Rust_primitives.Integers.v (Libcrux_intrinsics.Arm64_extract.get_lane_u32x4 ${result} k) in
     r == (a * 3329 + pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS - 1)) /
          pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS) /\
     r < 3329))"#))]
fn decompress_uint32x4_t<const COEFFICIENT_BITS: i32>(v: _uint32x4_t) -> _uint32x4_t {
    let coeff = _vdupq_n_u32(1 << (COEFFICIENT_BITS - 1));
    let decompressed = _vmulq_n_u32(v, FIELD_MODULUS as u32);
    let decompressed = _vaddq_u32(decompressed, coeff);
    let result = _vshrq_n_u32::<COEFFICIENT_BITS>(decompressed);
    hax_lib::fstar!(
        r#"introduce (forall (k: nat). k < 4 ==>
                Rust_primitives.Integers.v (NA.get_lane_u32x4 v k) <
                pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS)) ==>
              (forall (k: nat). k < 4 ==>
                (let a = Rust_primitives.Integers.v (NA.get_lane_u32x4 v k) in
                 Rust_primitives.Integers.v (NA.get_lane_u32x4 result k) ==
                   (a * 3329 + pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS - 1)) /
                   pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS) /\
                 Rust_primitives.Integers.v (NA.get_lane_u32x4 result k) < 3329))
    with _hyp.
      introduce forall (k: nat). k < 4 ==>
        (let a = Rust_primitives.Integers.v (NA.get_lane_u32x4 v k) in
         Rust_primitives.Integers.v (NA.get_lane_u32x4 result k) ==
           (a * 3329 + pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS - 1)) /
           pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS) /\
         Rust_primitives.Integers.v (NA.get_lane_u32x4 result k) < 3329)
      with (if k < 4 then lemma_decompress_u32_lane v v_COEFFICIENT_BITS k)"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::fstar::options("--z3rlimit 200 --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall i. (let x = Seq.index (repr ${a}) i in
    x == mk_i16 0 \/ x == mk_i16 1)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat).
    i < 16 ==>
    (let res_i = Rust_primitives.Integers.v (Seq.index (repr ${result}) i) in
     let a_i = Rust_primitives.Integers.v (Seq.index (repr ${a}) i) in
     (res_i == 0 \/ res_i == 1665) /\ res_i == (2 * a_i * 3329 + 2) / 4)"#))]
pub fn decompress_1(a: SIMD128Vector) -> SIMD128Vector {
    let z = ZERO();
    // z is all-zero, and every lane of `a` is in {0, 1}, so 0 - a_i in {0, -1}
    // satisfies `sub`'s precondition (no signed overflow).
    hax_lib::fstar!(
        r#"assert (forall i. Seq.index (repr ${z}) i == mk_i16 0);
           assert (forall i. Spec.Utils.is_intb (pow2 15 - 1)
             (Rust_primitives.Integers.v (Seq.index (repr ${z}) i) -
              Rust_primitives.Integers.v (Seq.index (repr ${a}) i)))"#
    );
    let s = super::arithmetic::sub(z, &a);
    // sub gives s_i == 0 - a_i, so s_i in {0,-1}: 0 when a_i==0, -1 when a_i==1.
    hax_lib::fstar!(
        r#"assert (forall i.
              Seq.index (repr ${s}) i == mk_i16 0 \/ Seq.index (repr ${s}) i == mk_i16 (- 1));
           assert (forall (i: nat).
              i < 16 ==>
              (let a_i = v (Seq.index (repr ${a}) i) in
               let s_i = v (Seq.index (repr ${s}) i) in
               (a_i == 0 ==> s_i == 0) /\ (a_i == 1 ==> s_i == - 1)))"#
    );
    let res = super::arithmetic::bitwise_and_with_constant(s, 1665);
    // s_i &. 1665: 0 &. 1665 == 0, (-1) &. 1665 == 1665.  Then match to the
    // decompress_d closed form (2*a_i*3329+2)/4 (== 0 for a_i==0, 1665 for a_i==1).
    hax_lib::fstar!(
        r#"assert (forall i.
              Seq.index (repr ${res}) i == mk_i16 0 \/ Seq.index (repr ${res}) i == mk_i16 1665);
           assert (forall (i: nat).
              i < 16 ==>
              (let a_i = v (Seq.index (repr ${a}) i) in
               let res_i = v (Seq.index (repr ${res}) i) in
               (a_i == 0 ==> res_i == 0) /\ (a_i == 1 ==> res_i == 1665)));
           assert (forall (i: nat).
              i < 16 ==>
              (let a_i = v (Seq.index (repr ${a}) i) in
               (a_i == 0 ==> (2 * a_i * 3329 + 2) / 4 == 0) /\
               (a_i == 1 ==> (2 * a_i * 3329 + 2) / 4 == 1665)));
           assert (forall (i: nat).
              i < 16 ==>
              (let a_i = v (Seq.index (repr ${a}) i) in
               let res_i = v (Seq.index (repr ${res}) i) in
               (res_i == 0 \/ res_i == 1665) /\ res_i == (2 * a_i * 3329 + 2) / 4))"#
    );
    res
}

// d-bit Neon decompress: the deinterleave bit lemmas + per-output-lane SIMD-leaf
// chain.  Deinterleave even/odd i16 via `vand 0xffff`/`vshr<16>` of the u32
// reinterpret, decompress each u32 lane, then `vtrn1q` re-interleave.  Reuses the
// scalar bridge `lemma_decompress_ciphertext_coefficient_fe_commute` in the
// dispatcher; no trusted-base extension.
#[hax_lib::fstar::before(
    r#"
(* ---- Reinterpret round-trip bit facts (pure crate-helper, NO trust; mirror
   the analogous lemmas in Vector.Neon.Ntt, which this module cannot import) ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 120"
let lemma_i16_bits_as_u32_bit (a: i16) (i: usize {v i < 32}) : Lemma
  (ensures get_bit (NA.i16_bits_as_u32 a) i == (if v i < 16 then get_bit a i else 0))
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
              #Rust_primitives.Integers.u16_inttype a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 32);
    assert (NA.i16_bits_as_u32 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
              #Rust_primitives.Integers.u32_inttype w)

(* value of i16_bits_as_u32 on a non-negative i16 (so v a < 2^15 < 2^16). *)
let lemma_i16_bits_as_u32_val (a: i16) : Lemma
  (requires 0 <= v a)
  (ensures v (NA.i16_bits_as_u32 a) == v a)
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
              #Rust_primitives.Integers.u16_inttype a in
    FStar.Math.Lemmas.small_mod (v a) (pow2 16);
    assert (NA.i16_bits_as_u32 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
              #Rust_primitives.Integers.u32_inttype w);
    FStar.Math.Lemmas.small_mod (v w) (pow2 32)

(* the deinterleave: AND with 0xffff extracts the low (even) i16 lane, SHR 16 the
   odd lane, from the u32 reinterpret `i16_bits_as_u32 a |. (i16_bits_as_u32 b <<! 16)`. *)
let lemma_deint_lo (a b: i16) : Lemma
  (requires 0 <= v a /\ 0 <= v b)
  (ensures v ((NA.i16_bits_as_u32 a |. (NA.i16_bits_as_u32 b <<! mk_u32 16) <: u32) &. mk_u32 65535)
           == v a)
  = let x = NA.i16_bits_as_u32 a in
    let y = NA.i16_bits_as_u32 b in
    let r = (x |. (y <<! mk_u32 16) <: u32) &. mk_u32 65535 in
    assert_norm (pow2 16 == 65536);
    let aux (i: usize {v i < 32}) : Lemma (get_bit r i == get_bit x i) =
      lemma_i16_bits_as_u32_bit a i;
      lemma_i16_bits_as_u32_bit b i;
      Rust_primitives.BitVectors.get_bit_pow2_minus_one #Rust_primitives.Integers.u32_inttype 16 i
    in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r x;
    lemma_i16_bits_as_u32_val a

let lemma_deint_hi (a b: i16) : Lemma
  (requires 0 <= v a /\ 0 <= v b)
  (ensures v ((NA.i16_bits_as_u32 a |. (NA.i16_bits_as_u32 b <<! mk_u32 16) <: u32) >>! mk_u32 16)
           == v b)
  = let x = NA.i16_bits_as_u32 a in
    let y = NA.i16_bits_as_u32 b in
    let r = (x |. (y <<! mk_u32 16) <: u32) >>! mk_u32 16 in
    let aux (i: usize {v i < 32}) : Lemma (get_bit r i == get_bit y i) =
      lemma_i16_bits_as_u32_bit a (if v i < 16 then sz (v i + 16) else i);
      lemma_i16_bits_as_u32_bit b i
    in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r y;
    lemma_i16_bits_as_u32_val b

(* reinterpret_s16_u32 back: lo16 of a small u32 is its value as i16; hi16 is 0. *)
let lemma_u32_lo16_val (d: u32) : Lemma
  (requires v d < pow2 15)
  (ensures NA.u32_lo16_as_i16 d == mk_i16 (v d) /\ v (NA.u32_lo16_as_i16 d) == v d)
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype
              #Rust_primitives.Integers.u16_inttype d in
    FStar.Math.Lemmas.small_mod (v d) (pow2 16);
    FStar.Math.Lemmas.small_mod (v w) (pow2 16)

let lemma_u32_hi16_zero (d: u32) : Lemma
  (requires v d < pow2 16)
  (ensures NA.u32_hi16_as_i16 d == mk_i16 0)
  = FStar.Math.Lemmas.small_div (v d) (pow2 16);
    assert (v (d >>! mk_u32 16) == 0);
    assert ((d >>! mk_u32 16) == mk_u32 0)
#pop-options

(* the Neon spine value (a*3329+2^(d-1))/2^d equals the bridge / AVX2 / portable
   form (2a*3329+2^d)/(2^d*2) by the 2x/2y == x/y cancellation. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 60"
let lemma_decompress_form_eq (a dd: nat)
  : Lemma (requires dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11)
          (ensures (a * 3329 + pow2 (dd - 1)) / pow2 dd ==
                   (2 * a * 3329 + pow2 dd) / (pow2 dd * 2))
  = FStar.Math.Lemmas.pow2_double_mult (dd - 1);
    let zz = a * 3329 + pow2 (dd - 1) in
    FStar.Math.Lemmas.division_multiplication_lemma (2 * zz) 2 (pow2 dd)
#pop-options

(* per-output-lane assembly: vtrn1q_s16 (reinterpret_s16_u32 l0d) (reinterpret_s16_u32 l1d)
   places the even/odd decompressed u32 lanes back in order.  Free-param (no
   decompress recomputation) so it composes lane-by-lane (no 16-lane saturation). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_assemble_lane (l0d l1d: NA.t_e_uint32x4_t) (j: nat{j < 8}) : Lemma
  (requires v (NA.get_lane_u32x4 l0d (j / 2)) < pow2 15 /\
            v (NA.get_lane_u32x4 l1d (j / 2)) < pow2 15)
  (ensures
    (let out = NA.e_vtrn1q_s16 (NA.e_vreinterpretq_s16_u32 l0d) (NA.e_vreinterpretq_s16_u32 l1d) in
     0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < pow2 15 /\
     v (NA.get_lane_i16x8 out j) ==
       (if j % 2 = 0
        then v (NA.get_lane_u32x4 l0d (j / 2))
        else v (NA.get_lane_u32x4 l1d (j / 2)))))
  = let aa = NA.e_vreinterpretq_s16_u32 l0d in
    let bb = NA.e_vreinterpretq_s16_u32 l1d in
    let k = j / 2 in
    FStar.Math.Lemmas.lemma_div_mod j 2;
    if j % 2 = 0
    then lemma_u32_lo16_val (NA.get_lane_u32x4 l0d k)
    else lemma_u32_lo16_val (NA.get_lane_u32x4 l1d k)
#pop-options

(* clean-context: the 4 deinterleaved lanes equal the even/odd input lanes AND are
   < 2^d.  Proven away from the decompress/assemble context (which otherwise
   saturates when this 4-lane forall is derived inline). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_deint_bounds (hv: NA.t_e_int16x8_t) (cb: i32) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < pow2 (v cb)))
  (ensures
    (let r = NA.e_vreinterpretq_u32_s16 hv in
     let l0 = NA.e_vandq_u32 r (NA.e_vdupq_n_u32 (mk_u32 65535)) in
     let l1 = NA.e_vshrq_n_u32 (mk_i32 16) r in
     forall (m: nat). m < 4 ==>
       v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
       v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
       v (NA.get_lane_u32x4 l0 m) < pow2 (v cb) /\ v (NA.get_lane_u32x4 l1 m) < pow2 (v cb)))
  = let r = NA.e_vreinterpretq_u32_s16 hv in
    let l0 = NA.e_vandq_u32 r (NA.e_vdupq_n_u32 (mk_u32 65535)) in
    let l1 = NA.e_vshrq_n_u32 (mk_i32 16) r in
    let aux (m: nat{m < 4})
      : Lemma (v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
               v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
               v (NA.get_lane_u32x4 l0 m) < pow2 (v cb) /\
               v (NA.get_lane_u32x4 l1 m) < pow2 (v cb)) =
      assert (2 * m < 8 /\ 2 * m + 1 < 8);
      lemma_deint_lo (NA.get_lane_i16x8 hv (2 * m)) (NA.get_lane_i16x8 hv (2 * m + 1));
      lemma_deint_hi (NA.get_lane_i16x8 hv (2 * m)) (NA.get_lane_i16x8 hv (2 * m + 1))
    in
    introduce forall (m: nat). m < 4 ==>
      (v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
       v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
       v (NA.get_lane_u32x4 l0 m) < pow2 (v cb) /\ v (NA.get_lane_u32x4 l1 m) < pow2 (v cb))
    with (if m < 4 then aux m)
#pop-options

(* per-OUTPUT-lane (standalone, clean context — the SIMD-leaf recipe): for one
   output lane j, the deinterleave -> decompress -> reinterpret+vtrn1q computes the
   exact decompress_d value of input lane j.  Factoring this per-lane (instead of an
   8-lane forall in the heavy half-context) avoids the saturation cliff. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_neon_out_lane (hv: NA.t_e_int16x8_t) (cb: i32) (j: nat{j < 8}) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < pow2 (v cb)))
  (ensures
    (let mask16 = NA.e_vdupq_n_u32 (mk_u32 65535) in
     let r = NA.e_vreinterpretq_u32_s16 hv in
     let l0 = NA.e_vandq_u32 r mask16 in
     let l1 = NA.e_vshrq_n_u32 (mk_i32 16) r in
     let l0d = decompress_uint32x4_t cb l0 in
     let l1d = decompress_uint32x4_t cb l1 in
     let out = NA.e_vtrn1q_s16 (NA.e_vreinterpretq_s16_u32 l0d) (NA.e_vreinterpretq_s16_u32 l1d) in
     0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < 3329 /\
     v (NA.get_lane_i16x8 out j) ==
       (v (NA.get_lane_i16x8 hv j) * 3329 + pow2 (v cb - 1)) / pow2 (v cb)))
  = let mask16 = NA.e_vdupq_n_u32 (mk_u32 65535) in
    let r = NA.e_vreinterpretq_u32_s16 hv in
    let l0 = NA.e_vandq_u32 r mask16 in
    let l1 = NA.e_vshrq_n_u32 (mk_i32 16) r in
    let k = j / 2 in
    FStar.Math.Lemmas.lemma_div_mod j 2;
    lemma_deint_bounds hv cb;
    let l0d = decompress_uint32x4_t cb l0 in
    let l1d = decompress_uint32x4_t cb l1 in
    assert_norm (pow2 15 == 32768);
    FStar.Math.Lemmas.pow2_le_compat 15 (v cb);
    assert (v (NA.get_lane_u32x4 l0d k) ==
              (v (NA.get_lane_u32x4 l0 k) * 3329 + pow2 (v cb - 1)) / pow2 (v cb) /\
            v (NA.get_lane_u32x4 l0d k) < 3329);
    assert (v (NA.get_lane_u32x4 l1d k) ==
              (v (NA.get_lane_u32x4 l1 k) * 3329 + pow2 (v cb - 1)) / pow2 (v cb) /\
            v (NA.get_lane_u32x4 l1d k) < 3329);
    lemma_assemble_lane l0d l1d j
#pop-options

(* trivial dispatcher: the per-lane lemma keeps the 8-lane forall light. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --split_queries always"
let lemma_decompress_half_out (hv: NA.t_e_int16x8_t) (cb: i32) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < pow2 (v cb)))
  (ensures
    (let mask16 = NA.e_vdupq_n_u32 (mk_u32 65535) in
     let r = NA.e_vreinterpretq_u32_s16 hv in
     let l0 = NA.e_vandq_u32 r mask16 in
     let l1 = NA.e_vshrq_n_u32 (mk_i32 16) r in
     let l0d = decompress_uint32x4_t cb l0 in
     let l1d = decompress_uint32x4_t cb l1 in
     let out = NA.e_vtrn1q_s16 (NA.e_vreinterpretq_s16_u32 l0d) (NA.e_vreinterpretq_s16_u32 l1d) in
     forall (j: nat). j < 8 ==>
       0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < 3329 /\
       v (NA.get_lane_i16x8 out j) ==
         (v (NA.get_lane_i16x8 hv j) * 3329 + pow2 (v cb - 1)) / pow2 (v cb)))
  = introduce forall (j: nat). j < 8 ==>
      (let mask16 = NA.e_vdupq_n_u32 (mk_u32 65535) in
       let r = NA.e_vreinterpretq_u32_s16 hv in
       let l0 = NA.e_vandq_u32 r mask16 in
       let l1 = NA.e_vshrq_n_u32 (mk_i32 16) r in
       let l0d = decompress_uint32x4_t cb l0 in
       let l1d = decompress_uint32x4_t cb l1 in
       let out = NA.e_vtrn1q_s16 (NA.e_vreinterpretq_s16_u32 l0d) (NA.e_vreinterpretq_s16_u32 l1d) in
       0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < 3329 /\
       v (NA.get_lane_i16x8 out j) ==
         (v (NA.get_lane_i16x8 hv j) * 3329 + pow2 (v cb - 1)) / pow2 (v cb))
    with (if j < 8 then lemma_neon_out_lane hv cb j)
#pop-options
"#
)]
#[inline(always)]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"(Rust_primitives.Integers.v $COEFFICIENT_BITS == 4 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 5 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 10 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 11) /\
    (forall (j: nat). j < 16 ==>
      0 <= Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) j) /\
      Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) j) <
      pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS))"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 16 ==>
    (let a = Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) j) in
     let r = Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result}) j) in
     0 <= r /\ r < 3329 /\
     r == (2 * a * 3329 + pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS)) /
          (pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS) * 2))"#))]
pub(crate) fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    mut v: SIMD128Vector,
) -> SIMD128Vector {
    #[cfg(hax)]
    let v_orig = v;
    let mask16 = _vdupq_n_u32(0xffff);
    let low0 = _vandq_u32(_vreinterpretq_u32_s16(v.low), mask16);
    let low1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.low));
    let high0 = _vandq_u32(_vreinterpretq_u32_s16(v.high), mask16);
    let high1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.high));

    let low0 = decompress_uint32x4_t::<COEFFICIENT_BITS>(low0);
    let low1 = decompress_uint32x4_t::<COEFFICIENT_BITS>(low1);
    let high0 = decompress_uint32x4_t::<COEFFICIENT_BITS>(high0);
    let high1 = decompress_uint32x4_t::<COEFFICIENT_BITS>(high1);

    v.low = _vtrn1q_s16(_vreinterpretq_s16_u32(low0), _vreinterpretq_s16_u32(low1));
    v.high = _vtrn1q_s16(_vreinterpretq_s16_u32(high0), _vreinterpretq_s16_u32(high1));
    let result = v;
    hax_lib::fstar!(
        r#"(* repr append-index bridge for the input snapshot *)
    assert (forall (m: nat). m < 8 ==>
      Seq.index (repr ${v_orig}) m ==
      NA.get_lane_i16x8 (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_low m);
    assert (forall (m: nat). m < 8 ==>
      Seq.index (repr ${v_orig}) (m + 8) ==
      NA.get_lane_i16x8 (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_high m);
    (* input half lane bounds (from the function requires on repr v_orig) *)
    assert (forall (m: nat). m < 8 ==>
      0 <= Rust_primitives.Integers.v (NA.get_lane_i16x8 (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_low m) /\
      Rust_primitives.Integers.v (NA.get_lane_i16x8 (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_low m) <
      pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS));
    assert (forall (m: nat). m < 8 ==>
      0 <= Rust_primitives.Integers.v (NA.get_lane_i16x8 (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_high m) /\
      Rust_primitives.Integers.v (NA.get_lane_i16x8 (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_high m) <
      pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS));
    lemma_decompress_half_out (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_low v_COEFFICIENT_BITS;
    lemma_decompress_half_out (${v_orig}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_high v_COEFFICIENT_BITS;
    (* repr append-index bridge for the result *)
    assert (forall (m: nat). m < 8 ==>
      Seq.index (repr ${result}) m ==
      NA.get_lane_i16x8 (${result}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_low m);
    assert (forall (m: nat). m < 8 ==>
      Seq.index (repr ${result}) (m + 8) ==
      NA.get_lane_i16x8 (${result}).Libcrux_ml_kem.Vector.Neon.Vector_type.f_high m);
    introduce forall (j: nat). j < 16 ==>
      (let a = Rust_primitives.Integers.v (Seq.index (repr ${v_orig}) j) in
       let r = Rust_primitives.Integers.v (Seq.index (repr ${result}) j) in
       0 <= r /\ r < 3329 /\
       r ==
       (2 * a * 3329 + pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS)) /
       (pow2 (Rust_primitives.Integers.v v_COEFFICIENT_BITS) * 2))
    with (if j < 16
          then lemma_decompress_form_eq (Rust_primitives.Integers.v (Seq.index (repr ${v_orig}) j))
                 (Rust_primitives.Integers.v v_COEFFICIENT_BITS))"#
    );
    result
}
