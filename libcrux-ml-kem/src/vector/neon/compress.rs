use super::vector_type::*;
use crate::vector::FIELD_MODULUS;
use libcrux_intrinsics::arm64::*;

// Helper lemmas backing the functional postcondition of `compress_1`.
// `repr` is `low(8) ++ high(8)`; `lemma_repr_index` (Vector_type) bridges a
// `repr` index to the corresponding `get_lane_i16x8` of `.f_low` / `.f_high`.
#[hax_lib::fstar::before(
    r#"
module NA = Libcrux_intrinsics.Arm64_ml_kem_views
open Libcrux_ml_kem.Vector.Neon.Compress_theory
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
    proof!(
        r#"assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) k ==
                Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8
                  ${v}.f_low k);
           assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) (k + 8) ==
                Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8
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
    proof!(
        r#"assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) k ==
                Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8
                  ${v}.f_low k);
           assert (forall (k: nat{k < 8}).
              Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${v}) (k + 8) ==
                Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8
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
            proof!(r#"assert_norm (pow2 4 - 1 == 15)"#);
            0x0f
        }
        5 => {
            proof!(r#"assert_norm (pow2 5 - 1 == 31)"#);
            0x1f
        }
        10 => {
            proof!(r#"assert_norm (pow2 10 - 1 == 1023)"#);
            0x3ff
        }
        11 => {
            proof!(r#"assert_norm (pow2 11 - 1 == 2047)"#);
            0x7ff
        }
        x => {
            // catch-all is only reachable for coefficient_bits in [0, 15);
            // pow2 (v x) <= pow2 14 = 16384 < 2^15, so (1 << x) fits i16 and
            // (1 << x) - 1 >= 0 cannot underflow.
            proof!(
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

// A3 compress functional proof, part 1: per-u32-lane compress core.  The rest of
// the proof (deint/assemble/mask + the Barrett post `cmp_compress_post`) is in a
// `fstar::after` block on `decompress_ciphertext_coefficient` (it reuses the
// generic deinterleave/reinterpret lemmas defined in that fn's `before` block).
// cmp_compress_u32_lane cites this module's own compress_int32x4_t; a companion
// would create an F* module cycle (Error 308, per the Neon.Compress_theory header).
// proof-residence: locked(module-cycle) — cites own compress_int32x4_t
#[hax_lib::fstar::before(
    r#"
module NC = Libcrux_intrinsics.Arm64_ml_kem_views

(* per-u32-lane compress core: compress_int32x4_t lane k computes the Barrett
   (unmasked) compression value, bounded below 2^15.  Uses the validated arm64
   s32<->u32 lane reinterpret bridge NC.e_vreinterpret_i32_u32_lane_bridge. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let cmp_compress_u32_lane (vv: NC.t_e_uint32x4_t) (cb: i32) (k: nat{k < 4}) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            v (NC.get_lane_u32x4 vv k) < 3329)
  (ensures
    (let r = compress_int32x4_t cb vv in
     let a = v (NC.get_lane_u32x4 vv k) in
     v (NC.get_lane_u32x4 r k) == (((a * pow2 (v cb) + 1664) * 10321340) / pow2 35) /\
     0 <= v (NC.get_lane_u32x4 r k) /\ v (NC.get_lane_u32x4 r k) < pow2 15))
  = let a = v (NC.get_lane_u32x4 vv k) in
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb);
    let half = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 1664) in
    let c1 = Libcrux_intrinsics.Arm64.e_vshlq_n_u32 cb vv in
    let c2 = Libcrux_intrinsics.Arm64.e_vaddq_u32 c1 half in
    let s32 = Libcrux_intrinsics.Arm64.e_vreinterpretq_s32_u32 c2 in
    let mulres = Libcrux_intrinsics.Arm64.e_vqdmulhq_n_s32 s32 (mk_i32 10321340) in
    let u32res = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s32 mulres in
    let r = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 4) u32res in
    assert (v (NC.get_lane_u32x4 half k) == 1664);
    FStar.Math.Lemmas.lemma_mult_le_right (pow2 (v cb)) a 3328;
    assert_norm (3328 * 2048 < pow2 32);
    assert (a * pow2 (v cb) <= 3328 * 2048);
    assert (v (NC.get_lane_u32x4 c1 k) == a * pow2 (v cb));
    let bb = a * pow2 (v cb) + 1664 in
    assert (v (NC.get_lane_u32x4 c2 k) == bb);
    assert_norm (pow2 31 == 2147483648);
    assert (bb < pow2 31);
    NC.e_vreinterpret_i32_u32_lane_bridge c2 k;
    assert (s32 == c2);
    assert (v (NC.get_lane_i32x4 s32 k) == bb);
    assert_norm (pow2 47 == 140737488355328);
    FStar.Math.Lemmas.lemma_mult_le_right 10321340 bb (3328 * 2048 + 1664);
    assert (bb * 10321340 < pow2 47);
    let pp = (bb * 10321340) / pow2 31 in
    FStar.Math.Lemmas.lemma_div_lt_nat (bb * 10321340) 47 31;
    assert (pp < pow2 16);
    assert ((((cast (NC.get_lane_i32x4 s32 k) <: i64) *. (cast (mk_i32 10321340) <: i64)) >>! (mk_i32 31))
            == mk_i64 pp);
    assert (v (NC.get_lane_i32x4 mulres k) == pp);
    NC.e_vreinterpret_i32_u32_lane_bridge mulres k;
    FStar.Math.Lemmas.small_mod pp (pow2 32);
    assert (v (NC.get_lane_u32x4 u32res k) == pp);
    assert (v (NC.get_lane_u32x4 r k) == pp / pow2 4);
    FStar.Math.Lemmas.division_multiplication_lemma (bb * 10321340) (pow2 31) (pow2 4);
    FStar.Math.Lemmas.pow2_plus 31 4;
    assert (v (NC.get_lane_u32x4 r k) == (bb * 10321340) / pow2 35);
    FStar.Math.Lemmas.lemma_div_lt_nat (bb * 10321340) 47 35
#pop-options
"#
)]
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
    proof!(
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
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[inline(always)]
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v $COEFFICIENT_BITS == 4 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 5 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 10 \/
    Rust_primitives.Integers.v $COEFFICIENT_BITS == 11"#))]
#[hax_lib::ensures(|result| fstar!(r#"(forall (k: nat). k < 4 ==>
    Rust_primitives.Integers.v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_u32x4 ${v} k) <
    pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS)) ==>
  (forall (k: nat). k < 4 ==>
    (let a = Rust_primitives.Integers.v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_u32x4 ${v} k) in
     let r = Rust_primitives.Integers.v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_u32x4 ${result} k) in
     r == (a * 3329 + pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS - 1)) /
          pow2 (Rust_primitives.Integers.v $COEFFICIENT_BITS) /\
     r < 3329))"#))]
fn decompress_uint32x4_t<const COEFFICIENT_BITS: i32>(v: _uint32x4_t) -> _uint32x4_t {
    let coeff = _vdupq_n_u32(1 << (COEFFICIENT_BITS - 1));
    let decompressed = _vmulq_n_u32(v, FIELD_MODULUS as u32);
    let decompressed = _vaddq_u32(decompressed, coeff);
    let result = _vshrq_n_u32::<COEFFICIENT_BITS>(decompressed);
    proof!(
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
    proof!(
        r#"assert (forall i. Seq.index (repr ${z}) i == mk_i16 0);
           assert (forall i. Spec.Utils.is_intb (pow2 15 - 1)
             (Rust_primitives.Integers.v (Seq.index (repr ${z}) i) -
              Rust_primitives.Integers.v (Seq.index (repr ${a}) i)))"#
    );
    let s = super::arithmetic::sub(z, &a);
    // sub gives s_i == 0 - a_i, so s_i in {0,-1}: 0 when a_i==0, -1 when a_i==1.
    proof!(
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
    proof!(
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
// lemma_neon_out_lane cites this module's own decompress_uint32x4_t; a companion
// would create an F* module cycle (Error 308, per the Neon.Compress_theory header).
// proof-residence: locked(module-cycle) — cites own decompress_uint32x4_t
#[hax_lib::fstar::before(
    r#"
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
    (let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
     let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
     let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
     let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
     let l0d = decompress_uint32x4_t cb l0 in
     let l1d = decompress_uint32x4_t cb l1 in
     let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0d) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1d) in
     0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < 3329 /\
     v (NA.get_lane_i16x8 out j) ==
       (v (NA.get_lane_i16x8 hv j) * 3329 + pow2 (v cb - 1)) / pow2 (v cb)))
  = let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
    let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
    let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
    let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
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
    (let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
     let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
     let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
     let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
     let l0d = decompress_uint32x4_t cb l0 in
     let l1d = decompress_uint32x4_t cb l1 in
     let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0d) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1d) in
     forall (j: nat). j < 8 ==>
       0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < 3329 /\
       v (NA.get_lane_i16x8 out j) ==
         (v (NA.get_lane_i16x8 hv j) * 3329 + pow2 (v cb - 1)) / pow2 (v cb)))
  = introduce forall (j: nat). j < 8 ==>
      (let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
       let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
       let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
       let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
       let l0d = decompress_uint32x4_t cb l0 in
       let l1d = decompress_uint32x4_t cb l1 in
       let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0d) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1d) in
       0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < 3329 /\
       v (NA.get_lane_i16x8 out j) ==
         (v (NA.get_lane_i16x8 hv j) * 3329 + pow2 (v cb - 1)) / pow2 (v cb))
    with (if j < 8 then lemma_neon_out_lane hv cb j)
#pop-options
"#
)]
// A3 compress functional proof, part 2 (after decompress so the generic
// deinterleave/reinterpret lemmas in decompress's `before` block are in scope):
// the Barrett-form functional post `cmp_compress_post` for `compress`, which
// `Vector.Neon.op_compress` calls and bridges to the spec `compress_post`.
#[hax_lib::fstar::after(
    interface,
    r#"
val cmp_compress_post (cb: i32) (vin: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    : Lemma
      (requires
        (Rust_primitives.Integers.v cb == 4 \/ Rust_primitives.Integers.v cb == 5 \/
          Rust_primitives.Integers.v cb == 10 \/ Rust_primitives.Integers.v cb == 11) /\
        (forall (i: nat).
            i < 16 ==>
            0 <= Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr vin) i) /\
            Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr vin) i) < 3329))
      (ensures
        (let result = compress cb vin in
          forall (i: nat).
            i < 16 ==>
            (let ri = Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr result) i) in
              let vi = Rust_primitives.Integers.v (Seq.index (Libcrux_ml_kem.Vector.Neon.Vector_type.repr vin) i) in
              0 <= ri /\ ri < pow2 (Rust_primitives.Integers.v cb) /\
              ri == (((vi * pow2 (Rust_primitives.Integers.v cb) + 1664) * 10321340) / pow2 35) %
                    pow2 (Rust_primitives.Integers.v cb))))
"#
)]
// The cmp_deint_bounds / cmp_out_lane / cmp_compress_post cluster cites this module's
// own compress_int32x4_t and t_SIMD128Vector fields; a companion would create an F*
// module cycle (Error 308, per the Neon.Compress_theory header).
// proof-residence: locked(module-cycle) — cites own compress_int32x4_t / SIMD128 fields
#[hax_lib::fstar::after(
    r#"
(* ===================== A3 compress functional post ===================== *)

(* deinterleave for compress: even/odd input lanes (< 3329) extracted to u32 lanes. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let cmp_deint_bounds (hv: NA.t_e_int16x8_t) : Lemma
  (requires (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < 3329))
  (ensures
    (let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
     let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r (Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535)) in
     let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
     forall (m: nat). m < 4 ==>
       v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
       v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
       v (NA.get_lane_u32x4 l0 m) < 3329 /\ v (NA.get_lane_u32x4 l1 m) < 3329))
  = let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
    let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r (Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535)) in
    let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
    let aux (m: nat{m < 4})
      : Lemma (v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
               v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
               v (NA.get_lane_u32x4 l0 m) < 3329 /\ v (NA.get_lane_u32x4 l1 m) < 3329) =
      assert (2 * m < 8 /\ 2 * m + 1 < 8);
      // The l1 (vshrq_n_u32 16) lane fact is not auto-injected over core-models,
      // so call the whole-vector shift op-fact + the per-lane reinterpret bridge
      // EXPLICITLY (no SMTPat).
      NA.lemma_e_vshrq_n_u32 (mk_i32 16) r;
      NA.lemma_e_vreinterpretq_u32_s16_lane hv m;
      lemma_deint_lo (NA.get_lane_i16x8 hv (2 * m)) (NA.get_lane_i16x8 hv (2 * m + 1));
      lemma_deint_hi (NA.get_lane_i16x8 hv (2 * m)) (NA.get_lane_i16x8 hv (2 * m + 1))
    in
    introduce forall (m: nat). m < 4 ==>
      (v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
       v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
       v (NA.get_lane_u32x4 l0 m) < 3329 /\ v (NA.get_lane_u32x4 l1 m) < 3329)
    with (if m < 4 then aux m)
#pop-options

(* per-output-lane: deint -> compress_int32x4_t -> reinterpret+vtrn1q computes the
   Barrett (unmasked) compression value of input lane j, bounded < 2^15. *)
#restart-solver
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let cmp_out_lane (hv: NA.t_e_int16x8_t) (cb: i32) (j: nat{j < 8}) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < 3329))
  (ensures
    (let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
     let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
     let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
     let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
     let l0c = compress_int32x4_t cb l0 in
     let l1c = compress_int32x4_t cb l1 in
     let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0c) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1c) in
     0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < pow2 15 /\
     v (NA.get_lane_i16x8 out j) ==
       (((v (NA.get_lane_i16x8 hv j) * pow2 (v cb) + 1664) * 10321340) / pow2 35)))
  = let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
    let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
    let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
    let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
    let k = j / 2 in
    FStar.Math.Lemmas.lemma_div_mod j 2;
    cmp_deint_bounds hv;
    let l0c = compress_int32x4_t cb l0 in
    let l1c = compress_int32x4_t cb l1 in
    cmp_compress_u32_lane l0 cb k;
    cmp_compress_u32_lane l1 cb k;
    lemma_assemble_lane l0c l1c j
#pop-options

(* trivial dispatcher: per-lane lemma keeps the 8-lane forall light. *)
#restart-solver
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --split_queries always"
let cmp_half_out (hv: NA.t_e_int16x8_t) (cb: i32) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < 3329))
  (ensures
    (let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
     let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
     let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
     let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
     let l0c = compress_int32x4_t cb l0 in
     let l1c = compress_int32x4_t cb l1 in
     let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0c) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1c) in
     forall (j: nat). j < 8 ==>
       0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < pow2 15 /\
       v (NA.get_lane_i16x8 out j) ==
         (((v (NA.get_lane_i16x8 hv j) * pow2 (v cb) + 1664) * 10321340) / pow2 35)))
  = introduce forall (j: nat). j < 8 ==>
      (let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
       let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
       let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r mask16 in
       let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
       let l0c = compress_int32x4_t cb l0 in
       let l1c = compress_int32x4_t cb l1 in
       let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0c) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1c) in
       0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < pow2 15 /\
       v (NA.get_lane_i16x8 out j) ==
         (((v (NA.get_lane_i16x8 hv j) * pow2 (v cb) + 1664) * 10321340) / pow2 35))
    with (if j < 8 then cmp_out_lane hv cb j)
#pop-options

(* final mask: x &. (2^cb - 1) == x % 2^cb for x >= 0. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let cmp_mask_lemma (x: i16) (cb: i32) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\ 0 <= v x)
  (ensures (let m = mask_n_least_significant_bits (cast (cb <: i32) <: i16) in
            v (x &. m) == (v x) % pow2 (v cb) /\
            0 <= (v x) % pow2 (v cb) /\ (v x) % pow2 (v cb) < pow2 (v cb)))
  = let dd = v cb in
    assert (v (cast (cb <: i32) <: i16) == v cb);
    let m = mask_n_least_significant_bits (cast (cb <: i32) <: i16) in
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    assert (v m == pow2 dd - 1);
    assert (m == Rust_primitives.Integers.sub #i16_inttype (mk_i16 (pow2 dd)) (mk_i16 1));
    Rust_primitives.Integers.logand_mask_lemma x dd
#pop-options

(* vdupq_n_s16 broadcast lane value (clean context, avoids context-pruning drop). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let cmp_dup_lane (x: i16) (m: nat{m < 8}) : Lemma
  (ensures NA.get_lane_i16x8 (Libcrux_intrinsics.Arm64.e_vdupq_n_s16 x) m == x)
  = FStar.Seq.Base.lemma_index_create 8 x m
#pop-options

(* clean-context per-lane mask: lane i of (vandq_s16 lowv (vdupq (2^cb-1)))
   == (lowv lane i) % 2^cb, bounded.  Isolates the dup-broadcast + mask reasoning
   away from cmp_compress_post's heavy (context-pruned) body. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let cmp_masked_lane (lowv: NA.t_e_int16x8_t) (cb: i32) (i: nat{i < 8}) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            0 <= v (NA.get_lane_i16x8 lowv i))
  (ensures
    (let m = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mask_n_least_significant_bits (cast (cb <: i32) <: i16)) in
     v (NA.get_lane_i16x8 (Libcrux_intrinsics.Arm64.e_vandq_s16 lowv m) i) == (v (NA.get_lane_i16x8 lowv i)) % pow2 (v cb) /\
     0 <= (v (NA.get_lane_i16x8 lowv i)) % pow2 (v cb) /\
     (v (NA.get_lane_i16x8 lowv i)) % pow2 (v cb) < pow2 (v cb)))
  = cmp_dup_lane (mask_n_least_significant_bits (cast (cb <: i32) <: i16)) i;
    cmp_mask_lemma (NA.get_lane_i16x8 lowv i) cb
#pop-options

(* isolate the (expensive) compress-body unfold: result halves == masked vtrn1q
   outputs.  Proven once; cmp_compress_post then reasons over `low`/`high`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let cmp_compress_unfold (cb: i32)
    (vin: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector) : Lemma
  (requires v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11)
  (ensures
    (let flo = vin.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
     let fhi = vin.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
     let mask = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mask_n_least_significant_bits (cast (cb <: i32) <: i16)) in
     let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
     let rlo = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 flo in
     let low = Libcrux_intrinsics.Arm64.e_vtrn1q_s16
       (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 (compress_int32x4_t cb (Libcrux_intrinsics.Arm64.e_vandq_u32 rlo mask16)))
       (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 (compress_int32x4_t cb (Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) rlo))) in
     let rhi = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 fhi in
     let high = Libcrux_intrinsics.Arm64.e_vtrn1q_s16
       (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 (compress_int32x4_t cb (Libcrux_intrinsics.Arm64.e_vandq_u32 rhi mask16)))
       (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 (compress_int32x4_t cb (Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) rhi))) in
     let result = compress cb vin in
     result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low == Libcrux_intrinsics.Arm64.e_vandq_s16 low mask /\
     result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high == Libcrux_intrinsics.Arm64.e_vandq_s16 high mask))
  = ()
#pop-options

(* full Barrett-form functional post for `compress`.
   op_compress (Vector.Neon) calls this, then bridges to the spec compress_post. *)
#restart-solver
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let cmp_compress_post (cb: i32) (vin: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (i: nat). i < 16 ==>
              0 <= v (Seq.index (repr vin) i) /\ v (Seq.index (repr vin) i) < 3329))
  (ensures
    (let result = compress cb vin in
     forall (i: nat). i < 16 ==>
       (let ri = v (Seq.index (repr result) i) in
        let vi = v (Seq.index (repr vin) i) in
        0 <= ri /\ ri < pow2 (v cb) /\
        ri == (((vi * pow2 (v cb) + 1664) * 10321340) / pow2 35) % pow2 (v cb))))
  = let flo = vin.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
    let fhi = vin.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let result = compress cb vin in
    let mask = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mask_n_least_significant_bits (cast (cb <: i32) <: i16)) in
    let mask16 = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535) in
    let rlo = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 flo in
    let llo0 = Libcrux_intrinsics.Arm64.e_vandq_u32 rlo mask16 in
    let llo1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) rlo in
    let llo0c = compress_int32x4_t cb llo0 in
    let llo1c = compress_int32x4_t cb llo1 in
    let low = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 llo0c) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 llo1c) in
    let rhi = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 fhi in
    let lhi0 = Libcrux_intrinsics.Arm64.e_vandq_u32 rhi mask16 in
    let lhi1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) rhi in
    let lhi0c = compress_int32x4_t cb lhi0 in
    let lhi1c = compress_int32x4_t cb lhi1 in
    let high = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 lhi0c) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 lhi1c) in
    cmp_compress_unfold cb vin;
    assert (result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low == Libcrux_intrinsics.Arm64.e_vandq_s16 low mask);
    assert (result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high == Libcrux_intrinsics.Arm64.e_vandq_s16 high mask);
    assert (forall (m: nat). m < 8 ==> Seq.index (repr vin) m == NA.get_lane_i16x8 flo m);
    assert (forall (m: nat). m < 8 ==> Seq.index (repr vin) (m + 8) == NA.get_lane_i16x8 fhi m);
    assert (forall (m: nat). m < 8 ==>
      Seq.index (repr result) m ==
      NA.get_lane_i16x8 (result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low) m);
    assert (forall (m: nat). m < 8 ==>
      Seq.index (repr result) (m + 8) ==
      NA.get_lane_i16x8 (result.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high) m);
    assert (forall (m: nat). m < 8 ==>
      0 <= v (NA.get_lane_i16x8 flo m) /\ v (NA.get_lane_i16x8 flo m) < 3329);
    assert (forall (m: nat). m < 8 ==>
      0 <= v (NA.get_lane_i16x8 fhi m) /\ v (NA.get_lane_i16x8 fhi m) < 3329);
    cmp_half_out flo cb;
    cmp_half_out fhi cb;
    let aux (i: nat{i < 16}) : Lemma
      (let ri = v (Seq.index (repr result) i) in
       let vi = v (Seq.index (repr vin) i) in
       0 <= ri /\ ri < pow2 (v cb) /\
       ri == (((vi * pow2 (v cb) + 1664) * 10321340) / pow2 35) % pow2 (v cb)) =
      if i < 8
      then cmp_masked_lane low cb i
      else cmp_masked_lane high cb (i - 8)
    in
    Classical.forall_intro aux
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
    proof!(
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
