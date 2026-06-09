use super::vector_type::*;
use crate::vector::traits::{
    BARRETT_R, BARRETT_SHIFT, FIELD_ELEMENTS_IN_VECTOR, FIELD_MODULUS,
    INVERSE_OF_MODULUS_MOD_MONTGOMERY_R,
};
use libcrux_secrets::*;

#[cfg(hax)]
use crate::vector::traits::spec;

/// If 'x' denotes a value of type `fe`, values having this type hold a
/// representative y ≡ x·MONTGOMERY_R^(-1) (mod FIELD_MODULUS).
/// We use 'mfe' as a shorthand for this type
pub type MontgomeryFieldElement = I16;

/// If 'x' denotes a value of type `fe`, values having this type hold a
/// representative y ≡ x·MONTGOMERY_R (mod FIELD_MODULUS).
/// We use 'fer' as a shorthand for this type.
pub type FieldElementTimesMontgomeryR = I16;

pub(crate) const MONTGOMERY_SHIFT: u8 = 16;

#[cfg(hax)]
pub(crate) const MONTGOMERY_R: i32 = 1 << MONTGOMERY_SHIFT;

/// This is calculated as ⌊(BARRETT_R / FIELD_MODULUS) + 1/2⌋
pub(crate) const BARRETT_MULTIPLIER: i32 = 20159;

#[hax_lib::fstar::options("--z3rlimit 150 --split_queries always")]
#[cfg_attr(hax, hax_lib::requires(n <= 16))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"v result == v value % pow2(v n)"#)))]
#[inline(always)]
pub(crate) fn get_n_least_significant_bits(n: u8, value: U32) -> U32 {
    let res = value & ((1 << n) - 1);

    hax_lib::fstar!(
        "calc (==) {
    v res;
    (==) { }
    v (logand value (((mk_u32 1) <<! n) -! (mk_u32 1)));
    (==) {} 
    v (logand value (((mk_int 1) <<! n) -! (mk_int 1)));
    (==) { }
    v (logand value (mk_int ((1 * pow2 (v n)) % pow2 32) -! (mk_int 1)));
    (==) {Math.Lemmas.small_mod (pow2 (v n)) (pow2 32); Math.Lemmas.pow2_lt_compat 32 (v n)}
    v (logand value ((mk_int (pow2 (v n))) -! (mk_int 1)));
    (==) {Math.Lemmas.pow2_lt_compat 32 (v n); logand_mask_lemma value (v n)}
    v value % (pow2 (v n));
    }"
    );

    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(fstar!(r#"${spec::add_pre} ${lhs}.f_elements ${rhs}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::add_post} ${lhs}.f_elements ${rhs}.f_elements ${result}.f_elements"#))]
pub fn add(mut lhs: PortableVector, rhs: &PortableVector) -> PortableVector {
    #[cfg(hax)]
    let _lhs0 = lhs;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==> (Seq.index ${lhs}.f_elements j) == 
                                     (Seq.index ${_lhs0}.f_elements j) +! (Seq.index ${rhs}.f_elements j)) /\
              (forall j. j >= v i ==> (Seq.index ${lhs}.f_elements j) == (Seq.index ${_lhs0}.f_elements j))"#
            )
        });

        lhs.elements[i] += rhs.elements[i];
    }

    hax_lib::fstar!(
        "assert (forall i. v (Seq.index ${lhs}.f_elements i) ==
    			          v (Seq.index ${_lhs0}.f_elements i) + v (Seq.index ${rhs}.f_elements i))"
    );
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );

    lhs
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::sub_pre} ${lhs}.f_elements ${rhs}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::sub_post} ${lhs}.f_elements ${rhs}.f_elements ${result}.f_elements"#))]
pub fn sub(mut lhs: PortableVector, rhs: &PortableVector) -> PortableVector {
    #[cfg(hax)]
    let _lhs0 = lhs;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==> (Seq.index ${lhs}.f_elements j) == 
                                     (Seq.index ${_lhs0}.f_elements j) -! (Seq.index ${rhs}.f_elements j)) /\
              (forall j. j >= v i ==> (Seq.index ${lhs}.f_elements j) == (Seq.index ${_lhs0}.f_elements j))"#
            )
        });

        lhs.elements[i] -= rhs.elements[i];
    }

    hax_lib::fstar!(
        "assert (forall i. v (Seq.index ${lhs}.f_elements i) ==
    			          v (Seq.index ${_lhs0}.f_elements i) - v (Seq.index ${rhs}.f_elements i))"
    );
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );

    lhs
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::multiply_by_constant_pre} ${vec}.f_elements c"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::multiply_by_constant_post} ${vec}.f_elements c ${result}.f_elements"#))]
pub fn multiply_by_constant(mut vec: PortableVector, c: i16) -> PortableVector {
    #[cfg(hax)]
    let _vec0 = vec;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==> (Seq.index ${vec}.f_elements j) == 
                                     (Seq.index ${_vec0}.f_elements j) *! c) /\
              (forall j. j >= v i ==> (Seq.index ${vec}.f_elements j) == (Seq.index ${_vec0}.f_elements j))"#
            )
        });

        vec.elements[i] *= c;
    }

    hax_lib::fstar!(
        "assert (forall i. v (Seq.index ${vec}.f_elements i) ==
    			          v (Seq.index ${_vec0}.f_elements i) * v c)"
    );

    vec
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"${result}.f_elements == Spec.Utils.map_array (fun x -> x &. c) (${vec}.f_elements)"#))]
pub fn bitwise_and_with_constant(mut vec: PortableVector, c: i16) -> PortableVector {
    #[cfg(hax)]
    let _vec0 = vec;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==> Seq.index ${vec}.f_elements j == 
                                     (Seq.index ${_vec0}.f_elements j &. c)) /\
              (forall j. j >= v i ==> Seq.index ${vec}.f_elements j == Seq.index ${_vec0}.f_elements j)"#
            )
        });

        vec.elements[i] &= c;
    }

    hax_lib::fstar!(
        r#"Seq.lemma_eq_intro ${vec}.f_elements (Spec.Utils.map_array (fun x -> x &. c) ${_vec0}.f_elements)"#
    );

    vec
}

#[inline(always)]
#[hax_lib::requires(SHIFT_BY >= 0 && SHIFT_BY < 16)]
#[hax_lib::ensures(|result| fstar!(r#"(v_SHIFT_BY >=. (mk_i32 0) /\ v_SHIFT_BY <. (mk_i32 16)) ==> 
        ${result}.f_elements == Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) (${vec}.f_elements)"#))]
pub fn shift_right<const SHIFT_BY: i32>(mut vec: PortableVector) -> PortableVector {
    #[cfg(hax)]
    let _vec0 = vec;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==> Seq.index ${vec}.f_elements j == 
                                     (Seq.index ${_vec0}.f_elements j >>! ${SHIFT_BY})) /\
              (forall j. j >= v i ==> Seq.index ${vec}.f_elements j == Seq.index ${_vec0}.f_elements j)"#
            )
        });

        vec.elements[i] = vec.elements[i] >> SHIFT_BY;
    }

    hax_lib::fstar!(
        r#"Seq.lemma_eq_intro ${vec}.f_elements (Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) ${_vec0}.f_elements)"#
    );

    vec
}

/// Note: This function is not secret independent
/// Only use with public values.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array (pow2 12 - 1) ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i.
    let x = Seq.index ${vec}.f_elements i in
    let y = Seq.index ${result}.f_elements i in
    ((v y == v x - 3329 \/ v y == v x) /\
     (v y % 3329 == v x % 3329) /\
     v y == (if v x >= 3329 then v x - 3329 else v x))"#))]
pub fn cond_subtract_3329(mut vec: PortableVector) -> PortableVector {
    #[cfg(hax)]
    let _vec0 = vec;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==> Seq.index ${vec}.f_elements j == 
                                     (let x = Seq.index ${_vec0}.f_elements j in
                                      if x >=. (mk_i16 3329) then x -! (mk_i16 3329) else x)) /\
              (forall j. j >= v i ==> Seq.index ${vec}.f_elements j == Seq.index ${_vec0}.f_elements j)"#
            )
        });
        if vec.elements[i].declassify() >= 3329 {
            vec.elements[i] -= 3329
        }
    }

    hax_lib::fstar!(
        r#"let aux (j: nat) : Lemma (j < 16 ==>
          (let x = Seq.index ${_vec0}.f_elements j in
           let y = Seq.index ${vec}.f_elements j in
           ((v y == v x - 3329 \/ v y == v x) /\
            (v y % 3329 == v x % 3329) /\
            v y == (if v x >= 3329 then v x - 3329 else v x))))
          = if j < 16 then begin
              let x = Seq.index ${_vec0}.f_elements j in
              if x >=. mk_i16 3329 then
                FStar.Math.Lemmas.lemma_mod_sub (v x) 3329 1
              else ()
            end in
        Classical.forall_intro aux"#
    );

    vec
}

/// Signed Barrett Reduction
///
/// Given an input `value`, `barrett_reduce` outputs a representative `result`
/// such that:
///
/// - result ≡ value (mod FIELD_MODULUS)
/// - the absolute value of `result` is bound as follows:
///
/// `|result| ≤ FIELD_MODULUS / 2 · (|value|/BARRETT_R + 1)
///
/// Note: The input bound is 28296 to prevent overflow in the multiplication of quotient by FIELD_MODULUS
///
#[hax_lib::fstar::options("--z3rlimit 150 --ext context_pruning")]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 28296 value"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b 3328 result /\
                v result % 3329 == v value % 3329"#)))]
pub(crate) fn barrett_reduce_element(value: FieldElement) -> FieldElement {
    let t = (value.as_i32() * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    hax_lib::fstar!(
        r#"
        assert_norm (v v_BARRETT_MULTIPLIER == (pow2 27 + 3329) / (2*3329));
        assert (v t = v value * v v_BARRETT_MULTIPLIER + pow2 25);
        assert (v t / pow2 26 < 9);
        assert (v t / pow2 26 > - 9)"#
    );
    hax_lib::fstar!(r#"assert (v t / pow2 26 < 9)"#);
    hax_lib::fstar!(r#"assert (v t / pow2 26 > - 9)"#);

    let quotient = (t >> BARRETT_SHIFT).as_i16();

    hax_lib::fstar!(r#"assert (v quotient = v t / pow2 26)"#);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b 9 quotient)"#);

    let result = value - (quotient * FIELD_MODULUS);

    hax_lib::fstar!(
        "calc (==) {
    v result % 3329;
    (==) { }
    (v value - (v quotient * 3329)) % 3329;
    (==) {Math.Lemmas.lemma_mod_sub_distr (v value) (v quotient * 3329) 3329}
    (v value - (v quotient * 3329) % 3329) % 3329;
    (==) {Math.Lemmas.cancel_mul_mod (v quotient) 3329}
    (v value - 0) % 3329;    
    (==) {}
    (v value) % 3329;    
    }"
    );

    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 28296 ${vec}.f_elements"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 ${result}.f_elements /\
                (forall i. (v (Seq.index ${result}.f_elements i) % 3329) == 
                           (v (Seq.index ${vec}.f_elements i) % 3329))"#)))]
pub(crate) fn barrett_reduce(mut vec: PortableVector) -> PortableVector {
    #[cfg(hax)]
    let _vec0 = vec;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                (forall j. j < v i ==> (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements j) /\
                                        v (Seq.index ${vec}.f_elements j) % 3329 == (v (Seq.index ${_vec0}.f_elements j) % 3329))) /\
                (forall j. j >= v i ==> (Seq.index ${vec}.f_elements j == Seq.index ${_vec0}.f_elements j /\
                                         Spec.Utils.is_i16b 28296 (Seq.index ${vec}.f_elements j)))"#
            )
        });

        let vi = barrett_reduce_element(vec.elements[i]);
        vec.elements[i] = vi;

        hax_lib::fstar!(
            r#"assert (v (mk_int #usize_inttype (v i + 1)) == v i + 1);
                         assert (forall j. j < v i ==> Spec.Utils.is_i16b 3328 (Seq.index vec.f_elements j));
                         assert(Spec.Utils.is_i16b 3328 vi);
                         assert(Spec.Utils.is_i16b 3328 (Seq.index vec.f_elements (v i)));
                         assert (forall j. j < v i + 1 ==> Spec.Utils.is_i16b 3328 (Seq.index vec.f_elements j))"#
        );
    }
    vec
}

/// Signed Montgomery Reduction
///
/// Given an input `value`, `montgomery_reduce` outputs a representative `o`
/// such that:
///
/// - o ≡ value · MONTGOMERY_R^(-1) (mod FIELD_MODULUS)
/// - the absolute value of `o` is bound as follows:
///
/// `|result| ≤ ceil(|value| / MONTGOMERY_R) + 1665
///
/// In particular, if `|value| ≤ FIELD_MODULUS-1 * FIELD_MODULUS-1`, then `|o| <= FIELD_MODULUS-1`.
/// And, if `|value| ≤ pow2 16 * FIELD_MODULUS-1`, then `|o| <= FIELD_MODULUS + 1664
///
#[hax_lib::fstar::options("--z3rlimit 300")]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b (3328 * pow2 16) value "#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b (3328 + 1665) result /\
                (Spec.Utils.is_i32b (3328 * pow2 15) value ==> Spec.Utils.is_i16b 3328 result) /\
                v result % 3329 == (v value * 169) % 3329"#)))]
pub(crate) fn montgomery_reduce_element(value: I32) -> MontgomeryFieldElement {
    // This forces hax to extract code for MONTGOMERY_R before it extracts code
    // for this function. The removal of this line is being tracked in:
    // https://github.com/cryspen/libcrux/issues/134
    #[cfg(hax)]
    let _ = MONTGOMERY_R;

    let k = (value.as_i16()).as_i32() * (INVERSE_OF_MODULUS_MOD_MONTGOMERY_R.classify().as_i32());

    hax_lib::fstar!(
        r#"assert(v (cast (cast (value <: i32) <: i16) <: i32) == v value @% pow2 16);
                     assert(v k == (v value @% pow2 16) * 62209);
                     assert(v (cast (cast (k <: i32) <: i16) <: i32) == v k @% pow2 16);
                     assert(v (cast (cast (k <: i32) <: i16) <: i32) < pow2 15);
                     assert(v (cast (cast (k <: i32) <: i16) <: i32) >= -pow2 15);
                     assert(v (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: i32) == 3329)"#
    );

    let k_times_modulus = (k.as_i16().as_i32()) * (FIELD_MODULUS.classify().as_i32());

    hax_lib::fstar!(
        r#"assert_norm (pow2 15 * 3329 < pow2 31);
           Spec.Utils.lemma_mul_i16b (pow2 15) (3329) (cast (k <: i32) <: i16) Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS;
                     assert (Spec.Utils.is_i32b (pow2 15 * 3329) k_times_modulus)"#
    );

    let c = (k_times_modulus >> MONTGOMERY_SHIFT).as_i16();

    hax_lib::fstar!(
        "assert (v k_times_modulus < pow2 31);
                     assert (v k_times_modulus / pow2 16 < pow2 15);
                     assert (v c == (v k_times_modulus / pow2 16) @% pow2 16);
                     assert(v c == v k_times_modulus / pow2 16); 
                     assert(Spec.Utils.is_i16b 1665 c)"
    );

    let value_high = (value >> MONTGOMERY_SHIFT).as_i16();

    hax_lib::fstar!(
        r#"assert (v value < pow2 31);
                     assert (v value / pow2 16 < pow2 15);
                     assert (v value_high == (v value / pow2 16) @% pow2 16);
                     Spec.Utils.lemma_div_at_percent (v value) (pow2 16);
                     assert (v value_high == (v value / pow2 16)); 
                     assert(Spec.Utils.is_i32b (3328 * 3328) value ==> Spec.Utils.is_i16b 169 value_high);
                     assert(Spec.Utils.is_i16b 3328 value_high)"#
    );

    let res = value_high - c;

    hax_lib::fstar!(
        r#"
        assert(Spec.Utils.is_i16b (3328 + 1665) res);
        assert(Spec.Utils.is_i32b (3328 * pow2 15) value ==> Spec.Utils.is_i16b 3328 res)
        "#
    );
    hax_lib::fstar!(
        r#"calc ( == ) {
        v k_times_modulus % pow2 16;
          ( == ) { assert (v k_times_modulus == (v k @% pow2 16) * 3329) }
        ((v k @% pow2 16) * 3329) % pow2 16;
          ( == ) { assert (v k = (v value @% pow2 16) * 62209) }
        ((((v value @% pow2 16) * 62209) @% pow2 16) * 3329) % pow2 16;
          ( == ) {  Math.Lemmas.lemma_mod_sub ((((v value @% pow2 16) * 62209) % pow2 16) * 3329) (pow2 16) 3329 }
        ((((v value @% pow2 16) * 62209) % pow2 16) * 3329) % pow2 16;
          ( == ) {  Math.Lemmas.lemma_mod_mul_distr_l ((v value @% pow2 16) * 62209) 3329 (pow2 16) }
        ((((v value @% pow2 16) * 62209) * 3329) % pow2 16);
          ( == ) { }
        (((v value @% pow2 16) * (62209 * 3329)) % pow2 16);
          ( == ) { Math.Lemmas.lemma_mod_mul_distr_r (v value @% pow2 16) (62209 * 3329) (pow2 16) }
        (((v value @% pow2 16) * ((62209 * 3329) % pow2 16)) % pow2 16);
          ( == ) { assert_norm((62209 * 3329) % pow2 16 == 1) }
        ((v value @% pow2 16) % pow2 16);
          ( == ) { Math.Lemmas.lemma_mod_sub (v value) (pow2 16) 1 }
        (v value) % pow2 16;
        };
      Math.Lemmas.modulo_add (pow2 16) (- (v k_times_modulus)) (v value) (v k_times_modulus);
      assert ((v value - v k_times_modulus) % pow2 16 == 0)"#
    );
    hax_lib::fstar!(
        r#"calc ( == ) {
        v res % 3329;
            ( == ) { assert (v res == v value_high - v c) }
        (v value / pow2 16 - v k_times_modulus / pow2 16) % 3329 ;
            ( == ) { Math.Lemmas.lemma_div_exact (v value - v k_times_modulus) (pow2 16) }
        ((v value - v k_times_modulus) / pow2 16) % 3329;
            ( == ) { assert ((pow2 16 * 169) % 3329 == 1) }
        (((v value - v k_times_modulus) / pow2 16) * ((pow2 16 * 169) % 3329)) % 3329;
            ( == ) { Math.Lemmas.lemma_mod_mul_distr_r ((v value - v k_times_modulus) / pow2 16) (pow2 16 * 169) 3329}
        (((v value - v k_times_modulus) / pow2 16) * pow2 16 * 169) % 3329;
            ( == ) { Math.Lemmas.lemma_div_exact (v value - v k_times_modulus) (pow2 16)}
        ((v value - v k_times_modulus) * 169) % 3329;
            ( == ) { assert (v k_times_modulus == (v k @% pow2 16) * 3329) }
        ((v value * 169) - ((v k @% pow2 16) * 3329 * 169)) % 3329; 
            ( == ) { Math.Lemmas.lemma_mod_sub (v value * 169) 3329 ((v k @% pow2 16) * 169)}
        (v value * 169) % 3329;  
        }"#
    );

    res
}

/// If `fe` is some field element 'x' of the Kyber field and `fer` is congruent to
/// `y · MONTGOMERY_R`, this procedure outputs a value that is congruent to
/// `x · y`, as follows:
///
///    `fe · fer ≡ x · y · MONTGOMERY_R (mod FIELD_MODULUS)`
///
/// `montgomery_reduce` takes the value `x · y · MONTGOMERY_R` and outputs a representative
/// `x · y · MONTGOMERY_R * MONTGOMERY_R^{-1} ≡ x · y (mod FIELD_MODULUS)`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 fer"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b 3328 result /\
                v result % 3329 == (v fe * v fer * 169) % 3329"#)))]
pub(crate) fn montgomery_multiply_fe_by_fer(
    fe: FieldElement,
    fer: FieldElementTimesMontgomeryR,
) -> FieldElement {
    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i16b (pow2 15) (1664) fe fer"#);

    let product = (fe.as_i32()) * (fer.as_i32());
    montgomery_reduce_element(product)
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 c"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
Spec.Utils.is_i16b_array 3328 ${result}.f_elements /\
(forall i. i < 16 ==> 
    (v (Seq.index ${result}.f_elements i) % 3329 == 
       (v (Seq.index ${vec}.f_elements i) * v c * 169) %3329))"#)))]
pub(crate) fn montgomery_multiply_by_constant(mut vec: PortableVector, c: I16) -> PortableVector {
    let _vec0 = vec;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v i ==>
	      	  (let vecj = Seq.index ${vec}.f_elements j in
		       (Spec.Utils.is_i16b 3328 vecj /\
                v vecj % 3329 == (v (Seq.index ${_vec0}.f_elements j) * v c * 169) % 3329))) /\
              (forall j. j >= v i ==> (Seq.index ${vec}.f_elements j) == (Seq.index ${_vec0}.f_elements j))"#
            )
        });

        vec.elements[i] = montgomery_multiply_fe_by_fer(vec.elements[i], c)
    }
    vec
}

#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 3328 ${a}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i.
                                       (let x = Seq.index ${a}.f_elements i in
                                        let y = Seq.index ${result}.f_elements i in
                                        (v y >= 0 /\ v y <= 3328 /\ (v y % 3329 == v x % 3329)))"#))]
#[inline(always)]
pub(crate) fn to_unsigned_representative(a: PortableVector) -> PortableVector {
    let t = shift_right::<15>(a);

    hax_lib::fstar!(
        r#"
        assert (forall i. Seq.index ${t}.f_elements i == ((Seq.index ${a}.f_elements i) >>! (mk_i32 15)));
        assert (forall i. Seq.index ${a}.f_elements i >=. mk_i16 0 ==> Seq.index ${t}.f_elements i == mk_i16 0);
        assert (forall i. Seq.index ${a}.f_elements i <. mk_i16 0 ==> Seq.index ${t}.f_elements i == mk_i16 (-1))
    "#
    );

    let fm = bitwise_and_with_constant(t, FIELD_MODULUS);

    hax_lib::fstar!(
        r#"
  assert (forall i. Seq.index ${fm}.f_elements i == (Seq.index ${t}.f_elements i &. Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS));
  assert (forall i. Seq.index ${a}.f_elements i >=. mk_i16 0 ==> Seq.index ${fm}.f_elements i == mk_i16 0);
  assert (forall i. Seq.index ${a}.f_elements i <. mk_i16 0 ==> Seq.index ${fm}.f_elements i == mk_i16 3329)
    "#
    );

    add(a, &fm)
}
