use crate::constants::{Eta, Gamma2};

/// Specs for the proofs
#[cfg(hax)]
pub(crate) mod specs;

// Each field element occupies 32 bits and the size of a simd_unit is 256 bits.
pub(crate) const COEFFICIENTS_IN_SIMD_UNIT: usize = 8;

// Note: For proofs, it is better to use concrete constants instead of const expressions
//COEFFICIENTS_IN_RING_ELEMENT / COEFFICIENTS_IN_SIMD_UNIT;
pub(crate) const SIMD_UNITS_IN_RING_ELEMENT: usize = 32;

pub const FIELD_MODULUS: i32 = 8_380_417;

// FIELD_MODULUS^{-1} mod MONTGOMERY_R
pub const INVERSE_OF_MODULUS_MOD_MONTGOMERY_R: u64 = 58_728_449;

/// If 'x' denotes a value of type `fe`, values having this type hold a
/// representative y ≡ x·MONTGOMERY_R (mod FIELD_MODULUS).
/// We use 'fer' as a shorthand for this type.
pub(crate) type FieldElementTimesMontgomeryR = i32;

#[cfg(hax)]
#[hax_lib::attributes]
pub(crate) trait Repr: Copy + Clone {
    #[cfg(hax)]
    #[requires(true)]
    fn repr(&self) -> [i32; COEFFICIENTS_IN_SIMD_UNIT];
}

#[cfg(not(hax))]
pub trait Repr {}

#[hax_lib::attributes]
pub(crate) trait Operations: Copy + Clone + Repr {
    #[hax_lib::requires(true)]
    #[hax_lib::ensures(|result| result.repr() == [0i32; COEFFICIENTS_IN_SIMD_UNIT])]
    fn zero() -> Self;

    #[hax_lib::requires(array.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[hax_lib::ensures(|result| future(out).repr() == array)]
    fn from_coefficient_array(array: &[i32], out: &mut Self);

    #[hax_lib::requires(out.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[hax_lib::ensures(|result| future(out) == value.repr())]
    fn to_coefficient_array(value: &Self, out: &mut [i32]);

    // Arithmetic
    #[hax_lib::requires(specs::add_pre(&lhs.repr(), &rhs.repr()))]
    #[hax_lib::ensures(|_| specs::add_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn add(lhs: &mut Self, rhs: &Self);

    #[hax_lib::requires(specs::sub_pre(&lhs.repr(), &rhs.repr()))]
    #[hax_lib::ensures(|_| specs::sub_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn subtract(lhs: &mut Self, rhs: &Self);

    #[hax_lib::requires(fstar!(r#"v $bound > 0 /\ 
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${simd_unit})"#))]
    fn infinity_norm_exceeds(simd_unit: &Self, bound: i32) -> bool;

    #[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/ 
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${simd_unit})"#))]
    fn decompose(gamma2: Gamma2, simd_unit: &Self, low: &mut Self, high: &mut Self);

    #[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/ 
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232})"#))]
    #[hax_lib::ensures(|result| result <= 8)]
    fn compute_hint(low: &Self, high: &Self, gamma2: i32, hint: &mut Self) -> usize;

    #[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/ 
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${simd_unit})"#))]
    fn use_hint(gamma2: Gamma2, simd_unit: &Self, hint: &mut Self);

    // Modular operations
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (${rhs.repr()})"#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${lhs}_future) /\
        Spec.MLDSA.Math.(forall i. i < 8 ==> 
            mod_q (v (Seq.index (f_repr ${lhs}_future) i)) == 
            mod_q (v (Seq.index (${lhs.repr()}) i) * v (Seq.index (${rhs.repr()}) i) * 8265825))"#))]
    fn montgomery_multiply(lhs: &mut Self, rhs: &Self);

    // 261631 is the largest x such that x * pow2 13 <= 2143289343 (the barrett reduce input bound)
    #[hax_lib::requires(fstar!(r#"v $SHIFT_BY == 13 /\ 
        (forall i. i < 8 ==> v (Seq.index (f_repr ${simd_unit}) i) >= 0 /\
            v (Seq.index (f_repr ${simd_unit}) i) <= 261631)"#))]
    fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Self);

    // Decomposition operations
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${t0})"#))]
    fn power2round(t0: &mut Self, t1: &mut Self);

    // Sampling
    //
    // In the sampling functions, since each SIMD unit can hold 8 coefficients,
    // we expect that `out` has the capacity for up to 8 coefficients.

    // Since each coefficient could potentially be sampled with 3 bytes, we expect
    // `randomness` to hold 24 bytes.
    #[hax_lib::requires(true)]
    fn rejection_sample_less_than_field_modulus(randomness: &[u8], out: &mut [i32]) -> usize;

    // Since each coefficient could potentially be sampled with half a byte,
    // we expect `randomness` to hold 4 bytes.
    #[hax_lib::requires(true)]
    fn rejection_sample_less_than_eta_equals_2(randomness: &[u8], out: &mut [i32]) -> usize;

    #[hax_lib::requires(true)]
    fn rejection_sample_less_than_eta_equals_4(randomness: &[u8], out: &mut [i32]) -> usize;

    // Encoding operations

    // Gamma1
    #[hax_lib::requires(true)]
    fn gamma1_serialize(simd_unit: &Self, serialized: &mut [u8], gamma1_exponent: usize);
    #[hax_lib::requires(true)]
    fn gamma1_deserialize(serialized: &[u8], out: &mut Self, gamma1_exponent: usize);

    // Commitment
    #[hax_lib::requires(serialized.len() == 4 || serialized.len() == 6)]
    #[hax_lib::ensures(|_| future(serialized).len() == serialized.len())]
    fn commitment_serialize(simd_unit: &Self, serialized: &mut [u8]);

    // Error
    #[hax_lib::requires(true)]
    fn error_serialize(eta: Eta, simd_unit: &Self, serialized: &mut [u8]);
    #[hax_lib::requires(true)]
    fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Self);

    // t0
    #[hax_lib::requires(true)]
    fn t0_serialize(simd_unit: &Self, out: &mut [u8]); // out len 13
    #[hax_lib::requires(true)]
    fn t0_deserialize(serialized: &[u8], out: &mut Self);

    // t1
    #[hax_lib::requires(true)]
    fn t1_serialize(simd_unit: &Self, out: &mut [u8]); // out len 10
    #[hax_lib::requires(true)]
    fn t1_deserialize(serialized: &[u8], out: &mut Self);

    // NTT
    #[hax_lib::requires(fstar!(r#"
        (forall (i:nat). i < 32 ==> 
            Spec.Utils.is_i32b_array_opaque 
            (v ${specs::NTT_BASE_BOUND})
            (f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        (forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) 
            (f_repr (Seq.index ${simd_units}_future i)))
    "#))]
    fn ntt(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);

    // invert NTT and convert to standard domain
    #[hax_lib::requires(fstar!(r#"
        (forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) 
            (f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        (forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) 
            (f_repr (Seq.index ${simd_units}_future i)))
    "#))]
    fn invert_ntt_montgomery(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);

    // Barrett reduce all coefficients
    fn barrett_reduce_simd_unit(simd_unit: &mut Self);
}
