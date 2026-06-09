use super::vector_type::{Coefficients, FieldElement};
#[cfg(hax)]
use crate::simd::traits::{specs::*, COEFFICIENTS_IN_SIMD_UNIT};
use crate::{
    constants::{Gamma2, BITS_IN_LOWER_PART_OF_T, GAMMA2_V261_888, GAMMA2_V95_232},
    simd::traits::{
        FieldElementTimesMontgomeryR, FIELD_MODULUS, INVERSE_OF_MODULUS_MOD_MONTGOMERY_R,
    },
};

pub(crate) const MONTGOMERY_SHIFT: u8 = 32;

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(add_pre(&lhs.values, &rhs.values))]
#[hax_lib::ensures(|result| add_post(&lhs.values, &rhs.values, &(future(lhs).values)))]
pub fn add(lhs: &mut Coefficients, rhs: &Coefficients) {
    #[cfg(hax)]
    let _lhs0 = lhs.clone();
    hax_lib::fstar!("reveal_opaque (`%$add_pre) ($add_pre)");
    hax_lib::fstar!("reveal_opaque (`%$add_post) ($add_post)");

    for i in 0..lhs.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            hax_lib::forall(|j: usize| {
                hax_lib::implies(j < i, lhs.values[j] == _lhs0.values[j] + rhs.values[j])
            }) & hax_lib::forall(|j: usize| {
                hax_lib::implies(
                    j >= i && j < COEFFICIENTS_IN_SIMD_UNIT,
                    lhs.values[j] == _lhs0.values[j],
                )
            })
        });

        lhs.values[i] += rhs.values[i];
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(sub_pre(&lhs.values, &rhs.values))]
#[hax_lib::ensures(|result| sub_post(&lhs.values, &rhs.values, &(future(lhs).values)))]
pub fn subtract(lhs: &mut Coefficients, rhs: &Coefficients) {
    #[cfg(hax)]
    let _lhs0 = lhs.clone();
    hax_lib::fstar!("reveal_opaque (`%$sub_pre) ($sub_pre)");
    hax_lib::fstar!("reveal_opaque (`%$sub_post) ($sub_post)");

    for i in 0..lhs.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            hax_lib::forall(|j: usize| {
                hax_lib::implies(j < i, lhs.values[j] == _lhs0.values[j] - rhs.values[j])
            }) & hax_lib::forall(|j: usize| {
                hax_lib::implies(
                    j >= i && j < COEFFICIENTS_IN_SIMD_UNIT,
                    lhs.values[j] == _lhs0.values[j],
                )
            })
        });

        lhs.values[i] -= rhs.values[i];
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(n <= 32)]
#[hax_lib::ensures(|result| fstar!(r#"v result == v value % pow2(v n)"#))]
pub(crate) fn get_n_least_significant_bits(n: u8, value: u64) -> u64 {
    let res = value & ((1 << n) - 1);

    hax_lib::fstar!(
        "calc (==) {
            v res;
            (==) { }
            v (logand value (((mk_u64 1) <<! n) -! (mk_u64 1)));
            (==) {} 
            v (logand value (((mk_int 1) <<! n) -! (mk_int 1)));
            (==) { }
            v (logand value (mk_int ((1 * pow2 (v n)) % pow2 64) -! (mk_int 1)));
            (==) {Math.Lemmas.small_mod (pow2 (v n)) (pow2 64); Math.Lemmas.pow2_lt_compat 64 (v n)}
            v (logand value ((mk_int (pow2 (v n))) -! (mk_int 1)));
            (==) {Math.Lemmas.pow2_lt_compat 64 (v n); logand_mask_lemma value (v n)}
            v value % (pow2 (v n));
            }"
    );

    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 900 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i64b (8380416 * pow2 32) value "#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i32b (8380416 + 4190209) result /\
                (Spec.Utils.is_i64b (8380416 * pow2 31) value ==> Spec.Utils.is_i32b 8380416 result) /\
                Spec.MLDSA.Math.(mod_q (v result) == mod_q (v value * 8265825))"#))]
pub(crate) fn montgomery_reduce_element(value: i64) -> FieldElementTimesMontgomeryR {
    let t = get_n_least_significant_bits(MONTGOMERY_SHIFT, value as u64)
        * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;

    hax_lib::fstar!(r#"assert (v $t == (v $value % pow2 32) * 58728449)"#);

    let k = get_n_least_significant_bits(MONTGOMERY_SHIFT, t) as i32;

    hax_lib::fstar!(
        r#"assert (v $k == v $t @% pow2 32);
        assert(v (cast ($k <: i32) <: i64) == v $k);
        assert(v (cast ($k <: i32) <: i64) < pow2 31);
        assert(v (cast ($k <: i32) <: i64) >= -pow2 31);
        assert(v (cast ($FIELD_MODULUS <: i32) <: i64) == 8380417)"#
    );

    let k_times_modulus = (k as i64) * (FIELD_MODULUS as i64);

    hax_lib::fstar!(
        r#"Spec.Utils.lemma_mul_i32b (pow2 31) (8380417) $k $FIELD_MODULUS;
        assert (Spec.Utils.is_i64b (pow2 31 * 8380417) $k_times_modulus)"#
    );

    let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i32;

    hax_lib::fstar!(
        r#"assert (v $k_times_modulus < pow2 63);
        assert (v $k_times_modulus / pow2 32 < pow2 31);
        assert (v $c == (v $k_times_modulus / pow2 32) @% pow2 32);
        assert(v $c == v $k_times_modulus / pow2 32); 
        assert(Spec.Utils.is_i32b 4190209 $c)"#
    );

    let value_high = (value >> MONTGOMERY_SHIFT) as i32;

    hax_lib::fstar!(
        r#"assert (v $value < pow2 63);
        assert (v $value / pow2 32 < pow2 31);
        assert (v $value_high == (v $value / pow2 32) @% pow2 32);
        Spec.Utils.lemma_div_at_percent (v $value) (pow2 32);
        assert (v $value_high == (v $value / pow2 32));
        assert (Spec.Utils.is_i64b (8380416 * 8380416) $value ==> Spec.Utils.is_i32b 8265825 $value_high);
        assert(Spec.Utils.is_i32b 8380416 $value_high)"#
    );

    let res = value_high - c;

    hax_lib::fstar!(
        r#"assert(Spec.Utils.is_i32b (8380416 + 4190209) $res);
        assert(Spec.Utils.is_i64b (8380416 * pow2 31) $value ==> Spec.Utils.is_i32b 58728448 $res)"#
    );
    hax_lib::fstar!(
        r#"calc ( == ) {
            v $k_times_modulus % pow2 32;
            ( == ) { assert (v $k_times_modulus == v $k * 8380417) }
            (v $k * 8380417) % pow2 32;
            ( == ) { assert (v $k = ((v $value % pow2 32) * 58728449) @% pow2 32) }
            ((((v $value % pow2 32) * 58728449) @% pow2 32) * 8380417) % pow2 32;
            ( == ) {  Math.Lemmas.lemma_mod_sub ((((v $value % pow2 32) * 58728449) % pow2 32) * 8380417) (pow2 32) 8380417 }
            ((((v $value % pow2 32) * 58728449) % pow2 32) * 8380417) % pow2 32;
            ( == ) {  Math.Lemmas.lemma_mod_mul_distr_l ((v $value % pow2 32) * 58728449) 8380417 (pow2 32) }
            ((((v $value % pow2 32) * 58728449) * 8380417) % pow2 32);
            ( == ) {  Math.Lemmas.lemma_mod_mul_distr_r (v $value % pow2 32) (58728449 * 8380417) (pow2 32) }
            ((v $value % pow2 32) % pow2 32);
            ( == ) { Math.Lemmas.lemma_mod_sub (v $value) (pow2 32) 1 }
            (v $value) % pow2 32;
        };
        Math.Lemmas.modulo_add (pow2 32) (- (v $k_times_modulus)) (v $value) (v $k_times_modulus);
        assert ((v $value - v $k_times_modulus) % pow2 32 == 0)"#
    );
    hax_lib::fstar!(
        r#"calc ( == ) {
            v $res % 8380417;
            ( == ) { assert (v $res == v $value_high - v $c) }
            (v $value / pow2 32 - v $k_times_modulus / pow2 32) % 8380417;
            ( == ) { Math.Lemmas.lemma_div_exact (v $value - v $k_times_modulus) (pow2 32) }
            ((v $value - v $k_times_modulus) / pow2 32) % 8380417;
            ( == ) { assert ((pow2 32 * 8265825) % 8380417 == 1) }
            (((v $value - v $k_times_modulus) / pow2 32) * ((pow2 32 * 8265825) % 8380417)) % 8380417;
            ( == ) { Math.Lemmas.lemma_mod_mul_distr_r ((v $value - v $k_times_modulus) / pow2 32)
            (pow2 32 * 8265825)
            8380417 }
            (((v $value - v $k_times_modulus) / pow2 32) * pow2 32 * 8265825) % 8380417;
            ( == ) { Math.Lemmas.lemma_div_exact (v $value - v $k_times_modulus) (pow2 32) }
            ((v $value - v $k_times_modulus) * 8265825) % 8380417;
            ( == ) { assert (v $k_times_modulus == (v $k @% pow2 32) * 8380417) }
            ((v $value * 8265825) - ((v $k @% pow2 32) * 8380417 * 8265825)) % 8380417;
            ( == ) { Math.Lemmas.lemma_mod_sub (v $value * 8265825) 8380417 ((v $k @% pow2 32) * 8265825) }
            (v $value * 8265825) % 8380417;
        }"#
    );
    hax_lib::fstar!(r#"reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q)"#);
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b 4190208 fer"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i32b 8380416 $result /\
    Spec.MLDSA.Math.(mod_q (v result) == mod_q (v fe * v fer * 8265825))"#))]
pub(crate) fn montgomery_multiply_fe_by_fer(
    fe: FieldElement,
    fer: FieldElementTimesMontgomeryR,
) -> FieldElement {
    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i32b (pow2 31) (4190208) fe fer"#);

    montgomery_reduce_element((fe as i64) * (fer as i64))
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b 4190208 $c"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i32b_array_opaque 8380416 ${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values /\
    Spec.MLDSA.Math.(forall i. i < 8 ==> 
        mod_q (v (Seq.index ${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) == 
        mod_q (v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) * v $c * 8265825))"#))]
pub(crate) fn montgomery_multiply_by_constant(simd_unit: &mut Coefficients, c: i32) {
    #[cfg(hax)]
    let _simd_unit0 = simd_unit.clone();
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    for i in 0..simd_unit.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v $i ==>
	      	  (let vecj = Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j in
		       (Spec.Utils.is_i32b 8380416 vecj /\
                Spec.MLDSA.Math.mod_q (v vecj) == Spec.MLDSA.Math.mod_q (v (Seq.index ${_simd_unit0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) * v $c * 8265825)))) /\
              (forall j. j >= v $i ==> (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) == (Seq.index ${_simd_unit0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j))"#
            )
        });
        hax_lib::fstar!(
            r#"Spec.Utils.lemma_mul_i32b (pow2 31) (4190208) ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values.[ $i ] $c"#
        );

        simd_unit.values[i] = montgomery_reduce_element((simd_unit.values[i] as i64) * (c as i64))
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b_array_opaque 8380416 ${rhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i32b_array_opaque 8380416 ${lhs}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values /\
    Spec.MLDSA.Math.(forall i. i < 8 ==> 
        mod_q (v (Seq.index ${lhs}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) == 
        mod_q (v (Seq.index ${lhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) * v (Seq.index ${rhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) * 8265825))"#))]
pub(crate) fn montgomery_multiply(lhs: &mut Coefficients, rhs: &Coefficients) {
    #[cfg(hax)]
    let _lhs0 = lhs.clone();
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    for i in 0..lhs.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              (forall j. j < v $i ==>
	      	  (let vecj = Seq.index ${lhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j in
		       (Spec.Utils.is_i32b 8380416 vecj /\
                Spec.MLDSA.Math.mod_q (v vecj) == Spec.MLDSA.Math.mod_q (v (Seq.index ${_lhs0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) * v (Seq.index ${rhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) * 8265825)))) /\
              (forall j. j >= v $i ==> (Seq.index ${lhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) == (Seq.index ${_lhs0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j))"#
            )
        });
        hax_lib::fstar!(
            r#"Spec.Utils.lemma_mul_i32b (pow2 31) (8380416) ${lhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values.[ $i ] ${rhs}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values.[ $i ]"#
        );

        lhs.values[i] = montgomery_reduce_element((lhs.values[i] as i64) * (rhs.values[i] as i64))
    }
}

// Splits t ∈ {0, ..., q-1} into t0 and t1 with a = t1*2ᴰ + t0
// and -2ᴰ⁻¹ < t0 < 2ᴰ⁻¹.  Returns t0 and t1 computed as.
//
// - t0 = t mod± 2ᵈ
// - t1 = (t - t0) / 2ᵈ.
//
// We assume the input t is in the signed representative range and convert it
// to the standard unsigned range.
#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3refresh --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) $t"#))]
#[hax_lib::ensures(|(t0,t1)| fstar!(r#"let (t0_s, t1_s) = Spec.MLDSA.Math.power2round (v $t) in
    v $t0 == t0_s /\ v $t1 == t1_s /\ Spec.Utils.is_intb_bt (pow2 (v $BITS_IN_LOWER_PART_OF_T - 1)) (v $t0)"#))]
fn power2round_element(t: i32) -> (i32, i32) {
    hax_lib::fstar!("logand_lemma $FIELD_MODULUS (t >>! mk_i32 31)");
    #[cfg(hax)]
    let _t = t;

    // Convert the signed representative to the standard unsigned one.
    let t = t + ((t >> 31) & FIELD_MODULUS);

    hax_lib::fstar!("assert (v $t == v $_t % v $FIELD_MODULUS)");

    // t0 = t - (2^{BITS_IN_LOWER_PART_OF_T} * t1)
    // t1 = ⌊(t - 1)/2^{BITS_IN_LOWER_PART_OF_T} + 1/2⌋
    //
    // See Lemma 10 of the implementation notes document for more information
    // on what these compute.
    let t1 = (t - 1 + (1 << (BITS_IN_LOWER_PART_OF_T - 1))) >> BITS_IN_LOWER_PART_OF_T;

    hax_lib::fstar!(
        "assert (v $t1 == (v $t - 1 + pow2 12) / pow2 13);
        assert ((v $t - (Spec.Utils.mod_p (v $t) (pow2 13))) / pow2 13 ==
            (v $t / pow2 13 - (Spec.Utils.mod_p (v $t) (pow2 13)) / pow2 13));
        if v $t % pow2 13 > pow2 12 then
            (assert (Spec.Utils.mod_p (v $t) (pow2 13) == v $t % pow2 13 - pow2 13);
            assert ((Spec.Utils.mod_p (v $t) (pow2 13)) / pow2 13 == (v $t % pow2 13 - pow2 13) / pow2 13);
            assert ((v $t % pow2 13 - pow2 13) / pow2 13 == (v $t % pow2 13) / pow2 13 - pow2 13 / pow2 13);
            assert ((v $t % pow2 13) / pow2 13 - pow2 13 / pow2 13 == -1);
            assert ((v $t - (Spec.Utils.mod_p (v $t) (pow2 13))) / pow2 13 == v $t / pow2 13 + 1))
        else
            (assert ((v $t - (Spec.Utils.mod_p (v $t) (pow2 13))) / pow2 13 == v $t / pow2 13);
            assert ((v $t - 1 + pow2 12) / pow2 13 == v $t / pow2 13));
        assert (v $t1 == (v $t - (Spec.Utils.mod_p (v $t) (pow2 13))) / pow2 13)");

    let t0 = t - (t1 << BITS_IN_LOWER_PART_OF_T);

    hax_lib::fstar!(
        "assert (v $t0 == v $t - ((v $t - (Spec.Utils.mod_p (v $t) (pow2 13))) / pow2 13) * pow2 13);
        assert (v $t0 == v $t - (v $t - (Spec.Utils.mod_p (v $t) (pow2 13))));
        assert (v $t0 == Spec.Utils.mod_p (v $t) (pow2 13))");

    (t0, t1)
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b_array_opaque (v $FIELD_MODULUS - 1) ${t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    forall i. i < 8 ==>
        (let t0_v = v (Seq.index ${t0}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) in
        let (t0_s, t1_s) = Spec.MLDSA.Math.power2round (v (Seq.index ${t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) in
        t0_v == t0_s /\ v (Seq.index ${t1}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) == t1_s /\
        Spec.Utils.is_intb_bt (pow2 (v $BITS_IN_LOWER_PART_OF_T - 1)) t0_v)"#))]
pub(super) fn power2round(t0: &mut Coefficients, t1: &mut Coefficients) {
    #[cfg(hax)]
    let _t0: Coefficients = t0.clone();
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    for i in 0..t0.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                (forall j. j < v i ==> (let t0_v = v (Seq.index ${t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) in
                    let (t0_s, t1_s) = Spec.MLDSA.Math.power2round (v (Seq.index ${_t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)) in
                    t0_v == t0_s /\ v (Seq.index ${t1}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) == t1_s /\
                    Spec.Utils.is_intb_bt (pow2 (v $BITS_IN_LOWER_PART_OF_T - 1)) t0_v)) /\
                (forall j. j >= v i ==> (Seq.index ${t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j == Seq.index ${_t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j /\
                    Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) (Seq.index ${t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)))"#
            )
        });

        (t0.values[i], t1.values[i]) = power2round_element(t0.values[i]);
    }
}

// TODO: Revisit this function when doing the range analysis and testing
// additional KATs.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $bound > 0 /\ 
        Spec.Utils.is_i32b_array_opaque (v $FIELD_MODULUS - 1) ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    $result == false ==> 
        Spec.Utils.is_i32b_array_opaque (v $bound - 1) ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values"#))]
pub(super) fn infinity_norm_exceeds(simd_unit: &Coefficients, bound: i32) -> bool {
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    let mut result = false;
    // It is ok to leak which coefficient violates the bound since
    // the probability for each coefficient is independent of secret
    // data but we must not leak the sign of the centralized representative.
    for i in 0..simd_unit.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                $result == false ==> (forall j. j < v $i ==>
                    abs (v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)) < v $bound)"#
            )
        });

        let coefficient = simd_unit.values[i];
        // This norm is calculated using the absolute value of the
        // signed representative in the range:
        //
        // -FIELD_MODULUS / 2 < r <= FIELD_MODULUS / 2.
        //
        // So if the coefficient is negative, get its absolute value, but
        // don't convert it into a different representation.
        let sign = coefficient >> 31;

        hax_lib::fstar!("logand_lemma (mk_i32 2 *! $coefficient) $sign");

        let normalized = coefficient - (sign & (2 * coefficient));

        hax_lib::fstar!("assert (v $normalized == abs (v $coefficient))");

        // FIXME: return
        // [hax] https://github.com/hacspec/hax/issues/1204
        result = result || normalized >= bound;
    }

    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b 2143289343 $fe"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i32b 8380416 $result /\
    Spec.MLDSA.Math.mod_q (v $result) == Spec.MLDSA.Math.mod_q (v $fe)"#))]
fn barrett_reduce_element(fe: FieldElement) -> FieldElement {
    let quotient = (fe + (1 << 22)) >> 23;
    let result = fe - (quotient * FIELD_MODULUS);

    hax_lib::fstar!(
        "calc (==) {
        v $result % 8380417;
        (==) { }
        (v $fe - (v $quotient * 8380417)) % 8380417;
        (==) {Math.Lemmas.lemma_mod_sub_distr (v $fe) (v $quotient * 8380417) 8380417}
        (v $fe - (v $quotient * 8380417) % 8380417) % 8380417;
        (==) {Math.Lemmas.cancel_mul_mod (v $quotient) 8380417}
        (v $fe - 0) % 8380417;
        (==) {}
        (v $fe) % 8380417; 
    }"
    );

    hax_lib::fstar!(r#"reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q)"#);
    result
}

#[inline]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i32b_array_opaque 2143289343 (${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array_opaque 8380416 (${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) /\
    (forall i. i < 8 ==> Spec.MLDSA.Math.(
        mod_q (v (Seq.index ${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) ==
        mod_q (v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)))))"#))
]
pub(crate) fn barrett_reduce_simd_unit(simd_unit: &mut Coefficients) {
    #[cfg(hax)]
    let _simd_unit0 = simd_unit.clone();

    for i in 0..simd_unit.values.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
                (forall j. j < v i ==> (Spec.Utils.is_i32b 8380416 (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) /\
                                        Spec.MLDSA.Math.mod_q (v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j))
                                        == Spec.MLDSA.Math.mod_q (v (Seq.index ${_simd_unit0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)))) /\
                (forall j. j >= v i ==> Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j == Seq.index ${_simd_unit0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)"#
        ));

        simd_unit.values[i] = barrett_reduce_element(simd_unit.values[i]);
    }
}
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $SHIFT_BY == 13 /\ 
    (forall i. i < 8 ==> v (Seq.index (${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) i) >= 0 /\
        v (Seq.index (${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) i) <= 261631)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array_opaque 8380416 (${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) /\
    (forall i. i < 8 ==> Spec.MLDSA.Math.(
        mod_q (v (Seq.index ${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) ==
        mod_q (v ((Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) <<! v_SHIFT_BY))))"#))]
pub(super) fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Coefficients) {
    #[cfg(hax)]
    let _simd_unit0 = simd_unit.clone();
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    for i in 0..simd_unit.values.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
                (forall j. j < v i ==> Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j == 
                                       (Seq.index ${_simd_unit0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j <<! v_SHIFT_BY)) /\
                (forall j. j >= v i ==> Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j == Seq.index ${_simd_unit0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)"#
        ));

        simd_unit.values[i] = simd_unit.values[i] << SHIFT_BY;
    }

    barrett_reduce_simd_unit(simd_unit);
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232"#))]
#[hax_lib::ensures(|result| fstar!(r#"v $result = Spec.MLDSA.Math.compute_one_hint (v $low) (v $high) (v $gamma2)"#))]
fn compute_one_hint(low: i32, high: i32, gamma2: i32) -> i32 {
    if (low > gamma2) || (low < -gamma2) || (low == -gamma2 && high != 0) {
        1
    } else {
        0
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    (forall i. i < 8 ==> (v (Seq.index ${hint}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) =
        Spec.MLDSA.Math.compute_one_hint (v (Seq.index ${low}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i))
            (v (Seq.index ${high}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) (v $gamma2))) /\
    v $result == Spec.MLDSA.Math.compute_hint ${hint}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values"#
))]
pub(super) fn compute_hint(
    low: &Coefficients,
    high: &Coefficients,
    gamma2: i32,
    hint: &mut Coefficients,
) -> usize {
    let mut one_hints_count = 0;

    hax_lib::fstar!(
        r#"Spec.Utils.eq_repeati0 (sz 0) (Spec.MLDSA.Math.hint_counter ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) (0)"#
    );

    for i in 0..hint.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                v $i >= 0 /\ v $i <= 8 /\
                (forall j. j < v i ==> (v (Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) =
                    Spec.MLDSA.Math.compute_one_hint (v (Seq.index ${low}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j))
                        (v (Seq.index ${high}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)) (v $gamma2))) /\
                v $one_hints_count <= v $i /\
                v $one_hints_count == Spec.Utils.repeati ($i) (Spec.MLDSA.Math.hint_counter ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) (0)"#
            )
        });

        #[cfg(hax)]
        let _hint_values = hint.values;

        hint.values[i] = compute_one_hint(low.values[i], high.values[i], gamma2);

        hax_lib::fstar!(
            r#"Spec.MLDSA.Math.hint_counter_loop ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values $_hint_values (v i);
            Spec.Utils.unfold_repeati ($i +! sz 1) (Spec.MLDSA.Math.hint_counter ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values) (0) ($i)"#
        );

        one_hints_count += hint.values[i] as usize;
    }

    one_hints_count
}

// Take a representative -q < r < q and convert it
// to the standard unsigned one in the interval [0, q).
//
// Splits this representative r into r₀ and r₁ such that:
//
// - r = r₁*α + r₀
// - -α/2 < r₀ ≤ α/2
//
// except when r₁ = (q-1)/α; in this case:
//
// - r₁ is set to 0 is taken
// - α/2 ≤ r₀ < 0.
//
// Note that 0 ≤ r₁ < (q-1)/α.
#[inline(always)]
#[hax_lib::fstar::options(
    "--fuel 3 --z3rlimit 1500 --ext context_pruning --z3refresh --split_queries always"
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"(v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232) /\
    Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) $r"#))]
#[hax_lib::ensures(|(r0,r1)| fstar!(r#"
    let (r0_s, r1_s, cond) = Spec.MLDSA.Math.decompose (v $gamma2) (v $r) in
    v $r0 = r0_s /\ v $r1 = r1_s /\
    (if cond then
        (v $r0 >= -(v $gamma2) /\ v $r0 < 0)
    else
        (v $r0 > -(v $gamma2) /\ v $r0 <= v $gamma2)) /\
    (v $r1 >= 0 /\ v $r1 < (v $FIELD_MODULUS - 1) / (v $gamma2 * 2))"#))]
fn decompose_element(gamma2: Gamma2, r: i32) -> (i32, i32) {
    #[cfg(hax)]
    let _r = r;
    hax_lib::fstar!(r#"logand_lemma $FIELD_MODULUS ($r >>! mk_i32 31)"#);

    // Convert the signed representative to the standard unsigned one.
    let r = r + ((r >> 31) & FIELD_MODULUS);

    hax_lib::fstar!(r#"assert (v $r == v $_r % v $FIELD_MODULUS)"#);

    let r1 = {
        // Compute ⌈r / 128⌉
        let ceil_of_r_by_128 = (r + 127) >> 7;

        hax_lib::fstar!(r#"assert (v $ceil_of_r_by_128 == (v $r + 127) / 128)"#);

        match gamma2 {
            GAMMA2_V95_232 => {
                // We approximate 1 / 1488 as:
                // ⌊2²⁴ / 1488⌋ / 2²⁴ = 11,275 / 2²⁴
                let result = ((ceil_of_r_by_128 * 11_275) + (1 << 23)) >> 24;

                hax_lib::fstar!(
                    r#"assert (v $result == ((v $ceil_of_r_by_128 * 11275) + pow2 23) / pow2 24);
                    assert (v $result == ((((v $r + 127) / 128) * 11275) + pow2 23) / pow2 24);
                    assert (v $result == (v $r - 1 + 95232) / 190464);
                    assert (v $result == (v $r - (Spec.Utils.mod_p (v $r) 190464)) / 190464);
                    assert (v $result >= 0 /\ v $result <= 44)"#
                );
                hax_lib::fstar!(
                    r#"logxor_lemma $result ((mk_i32 43 -! $result) >>! mk_i32 31);
                    lognot_lemma $result;
                    logand_lemma ($result ^. ((mk_i32 43 -! $result) >>! mk_i32 31)) $result"#
                );

                // For the corner-case a₁ = (q-1)/α = 44, we have to set a₁=0.
                let result_0 = (result ^ (43 - result) >> 31) & result;

                hax_lib::fstar!(
                    r#"assert (v $result == 44 ==> v $result_0 == 0);
                    assert (v $result < 44 ==> v $result_0 == v $result)"#
                );

                result_0
            }
            GAMMA2_V261_888 => {
                // We approximate 1 / 4092 as:
                // ⌊2²² / 4092⌋ / 2²² = 1025 / 2²²
                let result = (ceil_of_r_by_128 * 1025 + (1 << 21)) >> 22;

                hax_lib::fstar!(
                    r#"assert (v $result == ((v $ceil_of_r_by_128 * 1025) + pow2 21) / pow2 22);
                    assert (v $result == ((((v $r + 127) / 128) * 1025) + pow2 21) / pow2 22);
                    assert (v $result == (v $r - 1 + 261888) / 523776);
                    assert (v $result == (v $r - (Spec.Utils.mod_p (v $r) 523776)) / 523776);
                    assert (v $result >= 0 /\ v $result <= 16)"#
                );
                hax_lib::fstar!(r#"logand_mask_lemma $result 4"#);

                // For the corner-case a₁ = (q-1)/α = 16, we have to set a₁=0.
                let result_0 = result & 15;

                hax_lib::fstar!(
                    r#"assert (v $result == 16 ==> v $result_0 == 0);
                    assert (v $result < 16 ==> v $result_0 == v $result)"#
                );

                result_0
            }
            _ => unreachable!(),
        }
    };

    let alpha = gamma2 * 2;
    let mut r0 = r - (r1 * alpha);

    #[cfg(hax)]
    let _r0 = r0;

    hax_lib::fstar!(
        r#"logand_lemma (((($FIELD_MODULUS -! mk_i32 1) /! mk_i32 2) -! $r0) >>! mk_i32 31)
            $FIELD_MODULUS"#
    );

    // In the corner-case, when we set a₁=0, we will incorrectly
    // have a₀ > (q-1)/2 and we'll need to subtract q.  As we
    // return a₀ + q, that comes down to adding q if a₀ < (q-1)/2.
    r0 -= (((FIELD_MODULUS - 1) / 2 - r0) >> 31) & FIELD_MODULUS;

    hax_lib::fstar!(
        r#"assert (v $_r0 > 4190208 ==> v $r0 == v $_r0 - 8380417);
        assert (v $_r0 <= 4190208 ==> v $r0 == v $_r0);
        if v $r - (Spec.Utils.mod_p (v $r) (v $alpha)) = 8380416 then
            (assert (v $r1 == 0);
            assert (v $r0 == (Spec.Utils.mod_p (v $r) (v $alpha)) - 1);
            assert (v $r0 >= -(v $gamma2) /\ v $r0 < 0))
        else
            (assert (v $r1 == (v $r - (Spec.Utils.mod_p (v $r) (v $alpha))) / v $alpha);
            assert (v $r0 == Spec.Utils.mod_p (v $r) (v $alpha));
            assert (v $r0 > -(v $gamma2) /\ v $r0 <= v $gamma2));
        assert (v $r1 >= 0 /\ v $r1 < 8380416 / (v $alpha))"#
    );

    (r0, r1)
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3refresh --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"(v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232) /\
    Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) $r /\
    (v $hint == 0 \/ v $hint == 1)"#))]
#[hax_lib::ensures(|result| fstar!(r#"v $result == Spec.MLDSA.Math.use_one_hint (v $gamma2) (v $r) (v $hint)"#))]
pub(crate) fn use_one_hint(gamma2: Gamma2, r: i32, hint: i32) -> i32 {
    let (r0, r1) = decompose_element(gamma2, r);
    if hint == 0 {
        return r1;
    }

    match gamma2 {
        GAMMA2_V95_232 => {
            if r0 > 0 {
                if r1 == 43 {
                    0
                } else {
                    r1 + hint
                }
            } else if r1 == 0 {
                43
            } else {
                r1 - hint
            }
        }

        GAMMA2_V261_888 => {
            hax_lib::fstar!(
                r#"logand_mask_lemma ($r1 +! $hint) 4;
                logand_mask_lemma ($r1 -! $hint) 4"#
            );

            if r0 > 0 {
                (r1 + hint) & 15
            } else {
                (r1 - hint) & 15
            }
        }

        _ => unreachable!(),
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"(v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232) /\
    Spec.Utils.is_i32b_array_opaque (v $FIELD_MODULUS - 1) ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values"#))]
#[hax_lib::ensures(|_| fstar!(r#"forall i. i < 8 ==>
    (let r = v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) in
    let r0 = v (Seq.index ${low}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) in
    let r1 = v (Seq.index ${high}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) in
    let (r0_s, r1_s, cond) = Spec.MLDSA.Math.decompose (v $gamma2) r in
    r0 = r0_s /\ r1 = r1_s /\
    (if cond then
        (r0 >= -(v $gamma2) /\ r0 < 0)
    else
        (r0 > -(v $gamma2) /\ r0 <= v $gamma2)) /\
    (r1 >= 0 /\ r1 < (v $FIELD_MODULUS - 1) / (v $gamma2 * 2)))"#))]
pub fn decompose(
    gamma2: Gamma2,
    simd_unit: &Coefficients,
    low: &mut Coefficients,
    high: &mut Coefficients,
) {
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    for i in 0..low.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                forall j. j < v i ==> (let r = v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) in
                    let r0 = v (Seq.index ${low}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) in
                    let r1 = v (Seq.index ${high}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) in
                    let (r0_s, r1_s, cond) = Spec.MLDSA.Math.decompose (v $gamma2) r in
                    r0 = r0_s /\ r1 = r1_s /\
                    (if cond then
                        (r0 >= -(v $gamma2) /\ r0 < 0)
                    else
                        (r0 > -(v $gamma2) /\ r0 <= v $gamma2)) /\
                    (r1 >= 0 /\ r1 < (v $FIELD_MODULUS - 1) / (v $gamma2 * 2)))"#
            )
        });

        (low.values[i], high.values[i]) = decompose_element(gamma2, simd_unit.values[i]);
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"(v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232) /\
    Spec.Utils.is_i32b_array_opaque (v $FIELD_MODULUS - 1) ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values /\
    (forall i. i < 8 ==> v (Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) == 0 \/ v (Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) == 1)"#))]
#[hax_lib::ensures(|_| fstar!(r#"forall i. i < 8 ==>
    (let h = Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i in
    let result = Seq.index ${hint}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i in
    v result = Spec.MLDSA.Math.use_one_hint (v $gamma2) (v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i)) (v h))"#))]
pub fn use_hint(gamma2: Gamma2, simd_unit: &Coefficients, hint: &mut Coefficients) {
    #[cfg(hax)]
    let _hint0 = hint.clone();
    hax_lib::fstar!(
        "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
    );

    for i in 0..hint.values.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                (forall j. j < v i ==> (let h = Seq.index ${_hint0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j in
                    let result = Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j in
                    v result = Spec.MLDSA.Math.use_one_hint (v $gamma2) (v (Seq.index ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j)) (v h))) /\
                (forall j. j >= v i ==> (Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j == Seq.index ${_hint0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j /\
                    (v (Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) == 0 \/ v (Seq.index ${hint}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) == 1)))"#
            )
        });

        hint.values[i] = use_one_hint(gamma2, simd_unit.values[i], hint.values[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_montgomery_reduce_element() {
        assert_eq!(montgomery_reduce_element(10933346042510), -1553279);
        assert_eq!(montgomery_reduce_element(-20392060523118), 1331779);
        assert_eq!(montgomery_reduce_element(13704140696092), -1231016);
        assert_eq!(montgomery_reduce_element(-631922212176), -2580954);
    }

    #[test]
    fn test_use_one_hint() {
        assert_eq!(use_one_hint(GAMMA2_V95_232, 7622170, 0), 40);
        assert_eq!(use_one_hint(GAMMA2_V95_232, 2332762, 1), 13);

        assert_eq!(use_one_hint(GAMMA2_V261_888, 7691572, 0), 15);
        assert_eq!(use_one_hint(GAMMA2_V261_888, 6635697, 1), 12);
    }
}
