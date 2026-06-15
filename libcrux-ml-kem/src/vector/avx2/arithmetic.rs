use crate::vector::{traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R, FIELD_MODULUS};

use super::*;

#[inline(always)]
#[hax_lib::fstar::before(interface, "open Libcrux_intrinsics.Avx2_extract")]
#[hax_lib::fstar::before(
    r#"
let lemma_add_i (lhs rhs: t_Vec256) (i:nat): Lemma 
  (requires (i < 16 /\ Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane lhs i) + v (get_lane rhs i))))
  (ensures (v (add_mod (get_lane lhs i) (get_lane rhs i)) ==
            (v (get_lane lhs i) + v (get_lane rhs i))))
  [SMTPat (v (add_mod (get_lane lhs i) (get_lane rhs i)))] = ()"#
)]
#[hax_lib::requires(fstar!(r#"forall i. i < 16 ==> 
    Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane $lhs i) + v (get_lane $rhs i))"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i. i < 16 ==> 
    v (get_lane $result i) == (v (get_lane $lhs i) + v (get_lane $rhs i))"#))]
pub(crate) fn add(lhs: Vec256, rhs: Vec256) -> Vec256 {
    let result = mm256_add_epi16(lhs, rhs);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane result i == get_lane lhs i +. get_lane rhs i);
                     assert (forall i. v (get_lane result i) == v (get_lane lhs i) + v (get_lane rhs i))"#
    );

    result
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let lemma_sub_i (lhs rhs: t_Vec256) (i:nat):  Lemma 
  (requires (i < 16 /\ Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane lhs i) - v (get_lane rhs i))))
  (ensures (v (sub_mod (get_lane lhs i) (get_lane rhs i)) ==
            (v (get_lane lhs i) - v (get_lane rhs i))))
  [SMTPat (v (sub_mod (get_lane lhs i) (get_lane rhs i)))] = ()"#
)]
#[hax_lib::requires(fstar!(r#"forall i. i < 16 ==> 
    Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane $lhs i) - v (get_lane $rhs i))"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i. i < 16 ==> 
    v (get_lane $result i) == (v (get_lane $lhs i) - v (get_lane $rhs i))"#))]
pub(crate) fn sub(lhs: Vec256, rhs: Vec256) -> Vec256 {
    let result = mm256_sub_epi16(lhs, rhs);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane result i == get_lane lhs i -. get_lane rhs i);
                     assert (forall i. v (get_lane result i) == v (get_lane lhs i) - v (get_lane rhs i))"#
    );

    result
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let lemma_mul_i (lhs: t_Vec256) (i:nat) (c:i16):  Lemma 
  (requires (i < 16 /\ Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane lhs i) * v c)))
  (ensures (v (mul_mod (get_lane lhs i) c) ==
            (v (get_lane lhs i) * v c)))
  [SMTPat (v (mul_mod (get_lane lhs i) c))] = ()"#
)]
#[hax_lib::requires(fstar!(r#"forall i. i < 16 ==> 
    Spec.Utils.is_intb (pow2 15 - 1) (v (get_lane $vector i) * v constant)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i. i < 16 ==> 
    v (get_lane $result i) == (v (get_lane $vector i) * v constant)"#))]
pub(crate) fn multiply_by_constant(vector: Vec256, constant: i16) -> Vec256 {
    let cv = mm256_set1_epi16(constant);
    let result = mm256_mullo_epi16(vector, cv);

    hax_lib::fstar!(
        r#"Seq.lemma_eq_intro (vec256_as_i16x16 ${result})
                        (Spec.Utils.map_array (fun x -> x *. $constant) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vector))"#
    );

    hax_lib::fstar!(
        r#"assert (forall i. get_lane result i == get_lane vector i *. constant);
                     assert (forall i. v (get_lane vector i *. constant) == v (get_lane vector i) * v constant);
                     assert (forall i. v (get_lane result i) == v (get_lane vector i) * v constant)"#
    );

    result
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result == 
                           Spec.Utils.map_array (fun x -> x &. $constant) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vector)"#))]
pub(crate) fn bitwise_and_with_constant(vector: Vec256, constant: i16) -> Vec256 {
    let cv = mm256_set1_epi16(constant);
    let result = mm256_and_si256(vector, cv);

    hax_lib::fstar!(
        r#"Seq.lemma_eq_intro (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
                        (Spec.Utils.map_array (fun x -> x &. $constant) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vector))"#
    );

    result
}

#[inline(always)]
#[hax_lib::requires(SHIFT_BY >= 0 && SHIFT_BY < 16)]
#[hax_lib::ensures(|result| fstar!(r#"(v_SHIFT_BY >=. (mk_i32 0) /\ v_SHIFT_BY <. (mk_i32 16)) ==> 
                            Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result == 
                            Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vector)"#))]
pub(crate) fn shift_right<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    let result = mm256_srai_epi16::<{ SHIFT_BY }>(vector);

    hax_lib::fstar!(
        "Seq.lemma_eq_intro (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result})
                        (Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) 
                           (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vector))"
    );

    result
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 100"))]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array (pow2 12 - 1) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vector)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i. i < 16 ==> 
                get_lane $result i == 
                (if (get_lane $vector i) >=. (mk_i16 3329) then get_lane $vector i -! (mk_i16 3329) else get_lane $vector i)"#))]
pub(crate) fn cond_subtract_3329(vector: Vec256) -> Vec256 {
    let field_modulus = mm256_set1_epi16(FIELD_MODULUS);

    hax_lib::fstar!(r#"assert (forall i. get_lane $field_modulus i == (mk_i16 3329))"#);

    // Compute v_i - Q and crate a mask from the sign bit of each of these
    // quantities.
    let v_minus_field_modulus = mm256_sub_epi16(vector, field_modulus);

    hax_lib::fstar!(
        "assert (forall i. get_lane $v_minus_field_modulus i == get_lane $vector i -. (mk_i16 3329))"
    );

    let sign_mask = mm256_srai_epi16::<15>(v_minus_field_modulus);

    hax_lib::fstar!(
        "assert (forall i. get_lane $sign_mask i == (get_lane $v_minus_field_modulus i >>! (mk_i32 15)))"
    );

    // If v_i - Q < 0 then add back Q to (v_i - Q).
    let conditional_add_field_modulus = mm256_and_si256(sign_mask, field_modulus);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $conditional_add_field_modulus i == (get_lane $sign_mask i &. (mk_i16 3329)))"#
    );

    let result = mm256_add_epi16(v_minus_field_modulus, conditional_add_field_modulus);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $result i == (get_lane $v_minus_field_modulus i +. get_lane $conditional_add_field_modulus i));
                     assert (forall i. get_lane $result i == Spec.Utils.cond_sub (get_lane $vector i));
                     assert (forall i. get_lane $result i == (if (get_lane $vector i) >=. (mk_i16 3329) then get_lane $vector i -! (mk_i16 3329) else get_lane $vector i))"#
    );

    result
}

const BARRETT_MULTIPLIER: i16 = 20159;

/// See Section 3.2 of the implementation notes document for an explanation
/// of this code.
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 200"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 28296 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
                (forall i. i < 16 ==> v (get_lane $result i) % 3329 == 
                                      (v (get_lane $vector i) % 3329))"#)))]
pub(crate) fn barrett_reduce(vector: Vec256) -> Vec256 {
    let t0 = mm256_mulhi_epi16(vector, mm256_set1_epi16(BARRETT_MULTIPLIER));

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $t0 i == (cast (((cast (get_lane $vector i) <: i32) *. (cast v_BARRETT_MULTIPLIER <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let t512 = mm256_set1_epi16(512);

    hax_lib::fstar!(r#"assert (forall i. get_lane $t512 i == (mk_i16 512))"#);

    let t1 = mm256_add_epi16(t0, t512);

    hax_lib::fstar!(r#"assert (forall i. get_lane $t1 i == get_lane $t0 i +. (mk_i16 512))"#);

    let quotient = mm256_srai_epi16::<10>(t1);

    hax_lib::fstar!(
        "assert (forall i. get_lane $quotient i == (((get_lane $t1 i) <: i16) >>! ((mk_i32 10) <: i32)))"
    );

    let quotient_times_field_modulus = mm256_mullo_epi16(quotient, mm256_set1_epi16(FIELD_MODULUS));

    hax_lib::fstar!(
        "assert (forall i. get_lane $quotient_times_field_modulus i ==
                     get_lane $quotient i *. Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS)"
    );
    let result = mm256_sub_epi16(vector, quotient_times_field_modulus);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $result i ==
                                       get_lane $vector i -.  get_lane $quotient_times_field_modulus i);
                    assert (forall i. get_lane $result i == Spec.Utils.barrett_red (get_lane $vector i));
                    assert (forall i. v (get_lane $result i) % 3329 == v (get_lane $vector i) % 3329);
                    assert (forall i. Spec.Utils.is_i16b 3328 (get_lane $result i));
                    assert (forall (i:nat). i < 16 ==> Spec.Utils.is_i16b 3328 (get_lane $result i));
                    assert (Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result))"#
    );

    result
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 100 --ext context_pruning"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 constant"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
                (forall i. i < 16 ==> v (get_lane $result i) % 3329 == 
                                      ((v (get_lane $vector i) * v constant * 169) % 3329))"#)))]
pub(crate) fn montgomery_multiply_by_constant(vector: Vec256, constant: i16) -> Vec256 {
    let vec_constant = mm256_set1_epi16(constant);

    hax_lib::fstar!(r#"assert (forall i. get_lane $vec_constant i == $constant)"#);
    let value_low = mm256_mullo_epi16(vector, vec_constant);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $value_low i == get_lane $vector i *. $constant)"#
    );
    let k = mm256_mullo_epi16(
        value_low,
        mm256_set1_epi16(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i16),
    );

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $k i == get_lane $value_low i *. (neg (mk_i16 3327)))"#
    );

    let modulus = mm256_set1_epi16(FIELD_MODULUS);

    hax_lib::fstar!(r#"assert (forall i. get_lane $modulus i == (mk_i16 3329))"#);

    let k_times_modulus = mm256_mulhi_epi16(k, modulus);

    hax_lib::fstar!(
        r#"assert (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $k_times_modulus == 
                        Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
                                (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $k)
                                (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $modulus));
                     assert (forall i. get_lane $k_times_modulus i == 
                        (cast (((cast (get_lane $k i) <: i32) *. (cast (get_lane $modulus i) <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let value_high = mm256_mulhi_epi16(vector, vec_constant);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $value_high i == 
        (cast (((cast (get_lane $vector i) <: i32) *. (cast (get_lane $vec_constant i) <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let result = mm256_sub_epi16(value_high, k_times_modulus);

    hax_lib::fstar!(
        r#"Spec.Utils.lemma_range_at_percent 3329 (pow2 32);
                    assert (v (cast (mk_i16 3329) <: i32) == (3329 @% pow2 32));
                    assert (v (cast (mk_i16 3329) <: i32) == 3329);
                    assert ((cast (mk_i16 3329) <: i32) == (mk_i32 3329));
                    assert (forall i. get_lane $result i == (get_lane $value_high i) -. (get_lane $k_times_modulus i));
                    assert (forall i. get_lane $result i == Spec.Utils.mont_mul_red_i16 (get_lane $vector i) $constant);
                    assert (forall i. Spec.Utils.is_i16b 3328 (get_lane $result i));
                    assert (forall (i:nat). i < 16 ==> Spec.Utils.is_i16b 3328 (get_lane $result i));
                    assert (Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result));
                    assert (forall i. v (get_lane $result i) % 3329 == ((v (get_lane $vector i) * v $constant * 169) % 3329))"#
    );
    result
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 100"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $constants))"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
                (forall i. i < 16 ==> v (get_lane $result i) % 3329 == 
                                      ((v (get_lane $vec i) * v (get_lane $constants i) * 169) % 3329))"#)))]
pub(crate) fn montgomery_multiply_by_constants(vec: Vec256, constants: Vec256) -> Vec256 {
    let value_low = mm256_mullo_epi16(vec, constants);

    hax_lib::fstar!(
        "assert (forall i. get_lane $value_low i == get_lane $vec i *. get_lane $constants i)"
    );

    let k = mm256_mullo_epi16(
        value_low,
        mm256_set1_epi16(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i16),
    );

    hax_lib::fstar!(
        r#"assert (forall i. get_lane $k i == get_lane $value_low i *. (neg (mk_i16 3327)))"#
    );

    let modulus = mm256_set1_epi16(FIELD_MODULUS);
    hax_lib::fstar!(r#"assert (forall i. get_lane $modulus i == (mk_i16 3329))"#);

    let k_times_modulus = mm256_mulhi_epi16(k, modulus);

    hax_lib::fstar!(
        r#"assert (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $k_times_modulus == 
                        Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
                                (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $k)
                                (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $modulus));
                    assert (forall i. get_lane $k_times_modulus i == 
                        (cast (((cast (get_lane $k i) <: i32) *. (cast (get_lane $modulus i) <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let value_high = mm256_mulhi_epi16(vec, constants);
    hax_lib::fstar!(
        r#"assert (forall i. get_lane $value_high i == 
            (cast (((cast (get_lane $vec i) <: i32) *. (cast (get_lane $constants i) <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let result = mm256_sub_epi16(value_high, k_times_modulus);

    hax_lib::fstar!(
        r#"Spec.Utils.lemma_range_at_percent 3329 (pow2 32);
                    assert (v (cast (mk_i16 3329) <: i32) == (3329 @% pow2 32));
                    assert (v (cast (mk_i16 3329) <: i32) == 3329);
                    assert ((cast (mk_i16 3329) <: i32) == (mk_i32 3329));
                    assert (forall i. get_lane $result i == (get_lane $value_high i) -. (get_lane $k_times_modulus i));
                    assert (forall i. get_lane $result i == Spec.Utils.mont_mul_red_i16 (get_lane $vec i) (get_lane $constants i));
                    assert (forall i. Spec.Utils.is_i16b 3328 (get_lane $result i));
                    assert (forall (i:nat). i < 16 ==> Spec.Utils.is_i16b 3328 (get_lane $result i));
                    assert (Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result));
                    assert (forall i. v (get_lane $result i) % 3329 == ((v (get_lane $vec i) * v (get_lane $constants i) * 169) % 3329))"#
    );

    result
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array (3328 * pow2 16) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vec))"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array (3328 + 1665) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${result}) /\
                (Spec.Utils.is_i16b_array (3328 * pow2 15) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $vec) ==>
                 Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result)) /\
                (forall i. i < 16 ==> v (get_lane $result i) % 3329 == 
                                      ((v (get_lane $vec i) * 169) % 3329))"#)))]
pub(crate) fn montgomery_reduce_i32s(vec: Vec256) -> Vec256 {
    let k = mm256_mullo_epi16(
        vec,
        mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32),
    );
    let k_times_modulus = mm256_mulhi_epi16(k, mm256_set1_epi32(FIELD_MODULUS as i32));

    let value_high = mm256_srli_epi32::<16>(vec);

    let result = mm256_sub_epi16(value_high, k_times_modulus);

    let result = mm256_slli_epi32::<16>(result);

    mm256_srai_epi32::<16>(result)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 100"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 1664 (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 $constants))"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 ${result}) /\
                (forall i. i < 8 ==> v (get_lane128 $result i) % 3329 == 
                                      ((v (get_lane128 $vec i) * v (get_lane128 $constants i) * 169) % 3329))"#)))]
pub(crate) fn montgomery_multiply_m128i_by_constants(vec: Vec128, constants: Vec128) -> Vec128 {
    let value_low = mm_mullo_epi16(vec, constants);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane128 $value_low i == get_lane128 $vec i *. get_lane128 $constants i)"#
    );

    let k = mm_mullo_epi16(
        value_low,
        mm_set1_epi16(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i16),
    );

    hax_lib::fstar!(
        "assert (forall i. get_lane128 $k i == get_lane128 $value_low i *. (neg (mk_i16 3327)))"
    );

    let modulus = mm_set1_epi16(FIELD_MODULUS);

    hax_lib::fstar!(r#"assert (forall i. get_lane128 $modulus i == (mk_i16 3329))"#);

    let k_times_modulus = mm_mulhi_epi16(k, modulus);

    hax_lib::fstar!(
        r#"assert (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 $k_times_modulus == 
                        Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
                                (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 $k)
                                (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 $modulus));
                    assert (forall i. get_lane128 $k_times_modulus i == 
                        (cast (((cast (get_lane128 $k i) <: i32) *. (cast (get_lane128 $modulus i) <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let value_high = mm_mulhi_epi16(vec, constants);

    hax_lib::fstar!(
        r#"assert (forall i. get_lane128 $value_high i == 
                        (cast (((cast (get_lane128 $vec i) <: i32) *. (cast (get_lane128 $constants i) <: i32)) >>! (mk_i32 16)) <: i16))"#
    );

    let result = mm_sub_epi16(value_high, k_times_modulus);

    hax_lib::fstar!(
        r#"Spec.Utils.lemma_range_at_percent 3329 (pow2 32);
                    assert (v (cast (mk_i16 3329) <: i32) == (3329 @% pow2 32));
                    assert (v (cast (mk_i16 3329) <: i32) == 3329);
                    assert ((cast (mk_i16 3329) <: i32) == (mk_i32 3329));
                    assert (forall i. get_lane128 $result i == (get_lane128 $value_high i) -. (get_lane128 $k_times_modulus i));
                    assert (forall i. get_lane128 $result i == Spec.Utils.mont_mul_red_i16 (get_lane128 $vec i) (get_lane128 $constants i));
                    assert (forall i. Spec.Utils.is_i16b 3328 (get_lane128 $result i));
                    assert (forall (i:nat). i < 8 ==> Spec.Utils.is_i16b 3328 (get_lane128 $result i));
                    assert (Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 $result));
                    assert (forall i. v (get_lane128 $result i) % 3329 == ((v (get_lane128 $vec i) * v (get_lane128 $constants i) * 169) % 3329))"#
    );

    result
}

#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i.
                                       (let x = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i in
                                        let y = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $result) i in
                                        (v y >= 0 /\ v y <= 3328 /\ (v y % 3329 == v x % 3329)))"#))]
#[inline(always)]
pub(crate) fn to_unsigned_representative(a: Vec256) -> Vec256 {
    let t = shift_right::<15>(a);

    hax_lib::fstar!(
        r#"
  assert (forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $t) i == 
                    ((Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i) >>! (mk_i32 15)));
  assert (forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i >=. mk_i16 0 ==> Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $t) i == mk_i16 0);
  assert (forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i <. mk_i16 0 ==> Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $t) i == mk_i16 (-1))
    "#
    );

    let fm = bitwise_and_with_constant(t, FIELD_MODULUS);

    hax_lib::fstar!(
        r#"
  assert (forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${fm}) i == (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $t) i &. Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS));
  assert (forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i >=. mk_i16 0 ==> Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${fm}) i == mk_i16 0);
  assert (forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i <. mk_i16 0 ==> Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${fm}) i == mk_i16 3329)
    "#
    );

    add(a, fm)
}
