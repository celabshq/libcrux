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

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v $COEFFICIENT_BITS >= 1 /\
    Rust_primitives.Integers.v $COEFFICIENT_BITS <= 32"#))]
fn decompress_uint32x4_t<const COEFFICIENT_BITS: i32>(v: _uint32x4_t) -> _uint32x4_t {
    let coeff = _vdupq_n_u32(1 << (COEFFICIENT_BITS - 1));
    let decompressed = _vmulq_n_u32(v, FIELD_MODULUS as u32);
    let decompressed = _vaddq_u32(decompressed, coeff);
    let decompressed = _vshrq_n_u32::<COEFFICIENT_BITS>(decompressed);

    decompressed
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

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v $COEFFICIENT_BITS == 4 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 5 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 10 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 11"#))]
pub(crate) fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    mut v: SIMD128Vector,
) -> SIMD128Vector {
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
    v
}
