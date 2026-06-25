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

// The spec is stated through `lane32` — the integer value of the j-th
// 32-bit lane of a `Vec256`, expressed via the canonical i16x16 view as
// unsigned low half + 2^16 · signed high half.  (No i32-lane accessor
// exists in `Avx2_extract`, and adding one there would cascade across
// every crate; `lane32` is the ml-kem-local view.)
//
// History: the previous spec was stated on the i16x16 view directly,
// where the `3328 * pow2 16` bounds are vacuous for i16 (any i16
// satisfies them) and the per-lane residue claim is false on odd
// (high-half) lanes.  It was never caught because the ensures is
// admitted under `panic_free`.  The lane32 form below is the correct
// i32-lane statement, cross-validated against a bit-exact simulation
// of this function's body (2000 random trials).
//
// Ensures, per 32-bit lane j:
//  - the low i16 lane of the result is the Montgomery reduction
//    (bounded by 3328, residue `lane32 vec j * 169 mod q`), and
//  - the full 32-bit lane equals that i16 sign-extended (the final
//    `slli/srai` pair), so the result can feed 32-bit multiplies.
#[inline(always)]
#[cfg_attr(
    hax,
    hax_lib::fstar::options(
        "--fuel 1 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
    )
)]
#[hax_lib::fstar::before(
    interface,
    r#"
[@@ "opaque_to_smt"]
let lane32 (vv: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8}) : int =
  (v (Libcrux_intrinsics.Avx2_extract.get_lane vv (2*j)) % 65536) +
  65536 * v (Libcrux_intrinsics.Avx2_extract.get_lane vv (2*j + 1))

(* Ground-literal per-lane triple: bound + sign-extension + residue.  Stated
   per concrete lane (no quantifier) so consumers chain by congruence only. *)
unfold let mont_red_i32_lane
    (vec result: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8}) : Type0 =
  let r16 = Libcrux_intrinsics.Avx2_extract.get_lane result (2 * j) in
  Spec.Utils.is_i16b 3328 r16 /\ lane32 result j == v r16 /\
  v r16 % 3329 == ((lane32 vec j) * 169) % 3329
"#
)]
#[hax_lib::fstar::before(
    r#"
(* ── A4 montgomery_reduce_i32s proof scaffolding ──────────────────────────── *)

(* lane32 (the intrinsic, transparent i32-lane view) decomposes into its two
   i16 sub-lanes: the @%-into-i16 is the low lane, the floor-/pow2 16 is the high
   lane.  Pure modular arithmetic over the lane32 definition. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_lane32_halves (w: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8})
  : Lemma (ensures
      (Libcrux_intrinsics.Avx2_extract.lane32 w j) @% pow2 16 ==
        v (Libcrux_intrinsics.Avx2_extract.get_lane w (2 * j)) /\
      (Libcrux_intrinsics.Avx2_extract.lane32 w j) / pow2 16 ==
        v (Libcrux_intrinsics.Avx2_extract.get_lane w (2 * j + 1)))
  = let lo = v (Libcrux_intrinsics.Avx2_extract.get_lane w (2 * j)) in
    let hi = v (Libcrux_intrinsics.Avx2_extract.get_lane w (2 * j + 1)) in
    assert_norm (pow2 16 == 65536);
    FStar.Math.Lemmas.lemma_div_plus (lo % pow2 16) hi (pow2 16);
    FStar.Math.Lemmas.small_div (lo % pow2 16) (pow2 16);
    FStar.Math.Lemmas.modulo_addition_lemma (lo % pow2 16) (pow2 16) hi;
    FStar.Math.Lemmas.small_mod (lo % pow2 16) (pow2 16);
    Spec.Utils.lemma_range_at_percent lo (pow2 16)
#pop-options

(* The logical srli-by-16 reproduces, mod 2^16, the arithmetic floor /2^16 of the
   (signed) lane: this is what makes `srli_epi32 16` deliver the high i16 lane. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_srli_hi (vv: int)
  : Lemma (requires - (pow2 31) <= vv /\ vv < pow2 31)
          (ensures ((vv % pow2 32) / pow2 16) @% pow2 16 == vv / pow2 16)
  = assert_norm (pow2 32 == pow2 16 * pow2 16);
    assert_norm (pow2 31 == pow2 16 * pow2 15);
    FStar.Math.Lemmas.lemma_div_lt_nat (if vv >= 0 then vv else vv + pow2 32) 32 16;
    if vv >= 0 then begin
      FStar.Math.Lemmas.small_mod vv (pow2 32);
      FStar.Math.Lemmas.lemma_div_lt_nat vv 31 16;
      Spec.Utils.lemma_range_at_percent (vv / pow2 16) (pow2 16)
    end
    else begin
      FStar.Math.Lemmas.small_mod (vv + pow2 32) (pow2 32);
      FStar.Math.Lemmas.modulo_addition_lemma vv (pow2 32) 1;
      FStar.Math.Lemmas.lemma_div_plus vv (pow2 16) (pow2 16);
      FStar.Math.Lemmas.modulo_addition_lemma (vv / pow2 16) (pow2 16) (pow2 16);
      FStar.Math.Lemmas.small_mod (vv / pow2 16 + pow2 16) (pow2 16);
      Spec.Utils.lemma_range_at_percent (vv / pow2 16) (pow2 16)
    end
#pop-options

(* Ground per-op lane facts (clean single-op context, like Compress's
   slli_lane_nowrap / srli3_lane), so the consumer can cite them as posts
   instead of letting the slli/srai lane-foralls auto-fire and cascade.  The
   slli get-lane facts keep `lane32` atomic so the slli *general* (lane32 @%
   2^32) clause cannot pull in nonlinear work. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60 --using_facts_from '* -Libcrux_intrinsics.Avx2_extract.lane32'"
let lemma_slli16_even
      (vv: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8})
    : Lemma
      (Libcrux_intrinsics.Avx2_extract.get_lane
          (Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 (mk_i32 16) vv) (2 * j) == mk_i16 0)
  = let r = Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 (mk_i32 16) vv in
    ()
let lemma_slli16_odd
      (vv: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8})
    : Lemma
      (Libcrux_intrinsics.Avx2_extract.get_lane
          (Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 (mk_i32 16) vv) (2 * j + 1) ==
        Libcrux_intrinsics.Avx2_extract.get_lane vv (2 * j))
  = let r = Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 (mk_i32 16) vv in
    ()
#pop-options
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_srai16_lane
      (r2 r3: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8})
    : Lemma (requires r3 == Libcrux_intrinsics.Avx2_extract.mm256_srai_epi32 (mk_i32 16) r2)
            (ensures
              Libcrux_intrinsics.Avx2_extract.lane32 r3 j ==
                (Libcrux_intrinsics.Avx2_extract.lane32 r2 j) / pow2 16)
  = ()
#pop-options

(* slli 16 then arithmetic srai 16 sign-extends the even i16 sub-lane `t`
   (the Montgomery result, |t| <= 3328) back into the full 32-bit lane.  The raw
   slli/srai posts are excluded so they cannot auto-fire and cascade; the lane
   facts come from the ground lemmas above. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 -Libcrux_intrinsics.Avx2_extract.mm256_srai_epi32'"
let lemma_sign_extend
      (r1 r2 r3: Libcrux_intrinsics.Avx2_extract.t_Vec256) (j: nat{j < 8}) (t: i16)
    : Lemma
      (requires
        Spec.Utils.is_i16b 3328 t /\
        Libcrux_intrinsics.Avx2_extract.get_lane r1 (2 * j) == t /\
        r2 == Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 (mk_i32 16) r1 /\
        r3 == Libcrux_intrinsics.Avx2_extract.mm256_srai_epi32 (mk_i32 16) r2)
      (ensures
        Libcrux_intrinsics.Avx2_extract.get_lane r3 (2 * j) == t /\
        Libcrux_intrinsics.Avx2_extract.lane32 r3 j == v t)
  = assert_norm (pow2 16 == 65536);
    lemma_slli16_even r1 j;
    lemma_slli16_odd r1 j;
    assert (Libcrux_intrinsics.Avx2_extract.lane32 r2 j == pow2 16 * v t);
    lemma_srai16_lane r2 r3 j;
    FStar.Math.Lemmas.cancel_mul_div (v t) (pow2 16);
    assert (Libcrux_intrinsics.Avx2_extract.lane32 r3 j == v t);
    lemma_lane32_halves r3 j;
    Spec.Utils.lemma_range_at_percent (v t) (pow2 16)
#pop-options

(* Per-32-bit-lane Montgomery reduction: the body's even i16 sub-lane of
   `result_sub` is `mont_red_i32` of the i32 with value `lane32 vec j`; the final
   `slli 16; srai 16` sign-extends it into the full i32 lane. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let mont_reduce_lane
      (vec k ktm vh r1 r2 r3: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (j: nat{j < 8})
    : Lemma
      (requires
        Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec j) /\
        k ==
          Libcrux_intrinsics.Avx2_extract.mm256_mullo_epi16 vec
            (Libcrux_intrinsics.Avx2_extract.mm256_set1_epi32 (cast (Libcrux_ml_kem.Vector.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R
                      <:
                      u32)
                  <:
                  i32)) /\
        ktm ==
          Libcrux_intrinsics.Avx2_extract.mm256_mulhi_epi16 k
            (Libcrux_intrinsics.Avx2_extract.mm256_set1_epi32 (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS
                      <:
                      i16)
                  <:
                  i32)) /\
        vh == Libcrux_intrinsics.Avx2_extract.mm256_srli_epi32 (mk_i32 16) vec /\
        r1 == Libcrux_intrinsics.Avx2_extract.mm256_sub_epi16 vh ktm /\
        r2 == Libcrux_intrinsics.Avx2_extract.mm256_slli_epi32 (mk_i32 16) r1 /\
        r3 == Libcrux_intrinsics.Avx2_extract.mm256_srai_epi32 (mk_i32 16) r2)
      (ensures mont_red_i32_lane vec r3 j)
  = reveal_opaque (`%lane32)
      (lane32);
    let vlo = Libcrux_intrinsics.Avx2_extract.get_lane vec (2 * j) in
    let vhi = Libcrux_intrinsics.Avx2_extract.get_lane vec (2 * j + 1) in
    let vV = Libcrux_intrinsics.Avx2_extract.lane32 vec j in
    lemma_lane32_halves vec j;
    assert_norm (3328 * pow2 15 < pow2 31);
    let pf:i32 = mk_int #i32_inttype vV in
    assert (v pf == vV);
    Spec.Utils.lemma_range_at_percent (v vhi) (pow2 16);
    (* pf's low / high i16 halves are the lane's two i16 sub-lanes. *)
    assert ((cast pf <: i16) == vlo);
    assert ((cast (pf >>! mk_i32 16) <: i16) == vhi);
    (* concrete constant values for the two broadcasts. *)
    assert_norm (v (cast (Libcrux_ml_kem.Vector.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u32) <: i32) == 62209);
    assert_norm (v (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) == 3329);
    assert_norm ((cast (cast (Libcrux_ml_kem.Vector.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u32) <: i32) <: i16) == neg (mk_i16 3327));
    assert_norm ((cast (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) <: i16) == mk_i16 3329);
    (* value_high low lane == high i16 sub-lane of vec. *)
    lemma_srli_hi vV;
    lemma_lane32_halves vh j;
    assert (Libcrux_intrinsics.Avx2_extract.get_lane vh (2 * j) == vhi);
    (* result_sub low lane is exactly mont_red_i32 pf. *)
    assert (Libcrux_intrinsics.Avx2_extract.get_lane r1 (2 * j) == Spec.Utils.mont_red_i32 pf);
    assert (Spec.Utils.is_i32b (3328 * pow2 15) pf);
    Spec.Utils.lemma_mont_red_i32 pf;
    let t = Libcrux_intrinsics.Avx2_extract.get_lane r1 (2 * j) in
    (* slli 16; srai 16 sign-extends t into the full i32 lane (clean context). *)
    lemma_sign_extend r1 r2 r3 j t
#pop-options
"#
)]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
                (Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 0) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 1) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 2) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 3) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 4) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 5) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 6) /\
                 Spec.Utils.is_intb (3328 * pow2 15) (lane32 $vec 7)) ==>
                (mont_red_i32_lane $vec ${result} 0 /\ mont_red_i32_lane $vec ${result} 1 /\
                 mont_red_i32_lane $vec ${result} 2 /\ mont_red_i32_lane $vec ${result} 3 /\
                 mont_red_i32_lane $vec ${result} 4 /\ mont_red_i32_lane $vec ${result} 5 /\
                 mont_red_i32_lane $vec ${result} 6 /\ mont_red_i32_lane $vec ${result} 7)"#)))]
pub(crate) fn montgomery_reduce_i32s(vec: Vec256) -> Vec256 {
    let k = mm256_mullo_epi16(
        vec,
        mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32),
    );
    let k_times_modulus = mm256_mulhi_epi16(k, mm256_set1_epi32(FIELD_MODULUS as i32));

    let value_high = mm256_srli_epi32::<16>(vec);

    let result_sub = mm256_sub_epi16(value_high, k_times_modulus);

    let result_shifted = mm256_slli_epi32::<16>(result_sub);

    let result = mm256_srai_epi32::<16>(result_shifted);

    hax_lib::fstar!(
        r#"
  introduce
    (Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (lane32 vec 7)) ==>
    (mont_red_i32_lane vec result 0 /\
      mont_red_i32_lane vec result 1 /\
      mont_red_i32_lane vec result 2 /\
      mont_red_i32_lane vec result 3 /\
      mont_red_i32_lane vec result 4 /\
      mont_red_i32_lane vec result 5 /\
      mont_red_i32_lane vec result 6 /\
      mont_red_i32_lane vec result 7)
  with _hyp.
    (mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 0;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 1;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 2;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 3;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 4;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 5;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 6;
      mont_reduce_lane vec k k_times_modulus value_high result_sub result_shifted result 7)
"#
    );

    result
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
