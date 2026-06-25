use crate::{
    constants::{RING_ELEMENT_OF_T1S_SIZE, SEED_FOR_A_SIZE},
    encoding::t1,
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --fuel 0 --ifuel 1")]
#[hax_lib::requires(fstar!(r#"
    Seq.length seed == 32 /\
    Seq.length t1 <= 8 /\
    Seq.length $verification_key_serialized == 32 + Seq.length t1 * 320 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
        (mk_usize 0) (mk_usize 1023) $t1"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${verification_key_serialized}_future == Seq.length ${verification_key_serialized}"#))]
pub(crate) fn generate_serialized<SIMDUnit: Operations>(
    seed: &[u8],
    t1: &[PolynomialRingElement<SIMDUnit>],
    verification_key_serialized: &mut [u8],
) {
    verification_key_serialized[0..SEED_FOR_A_SIZE].copy_from_slice(seed);
    let t1_len = t1.len();
    for i in 0..t1_len {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= Seq.length $t1 /\
              Seq.length $t1 <= 8 /\
              v $t1_len == Seq.length $t1 /\
              Seq.length $verification_key_serialized == 32 + Seq.length $t1 * 320 /\
              Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
                  (mk_usize 0) (mk_usize 1023) $t1"#
        ));
        hax_lib::fstar!(
            r#"assert_norm (v $RING_ELEMENT_OF_T1S_SIZE == 320);
               assert (v i < v $t1_len);
               assert (v i * 320 <= 7 * 320);
               assert ((v i + 1) * 320 <= Seq.length $t1 * 320)"#
        );
        let offset = SEED_FOR_A_SIZE + (i * RING_ELEMENT_OF_T1S_SIZE);
        t1::serialize::<SIMDUnit>(
            &t1[i],
            &mut verification_key_serialized[offset..offset + RING_ELEMENT_OF_T1S_SIZE],
        );
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"
    v $rows_in_a <= 8 /\
    v $rows_in_a == Seq.length t1 /\
    v $verification_key_size == 32 + v $rows_in_a * 320 /\
    Seq.length $serialized == v $rows_in_a * 320"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${t1}_future == Seq.length t1 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
        (mk_usize 0) (mk_usize 1023) ${t1}_future"#))]
pub(crate) fn deserialize<SIMDUnit: Operations>(
    rows_in_a: usize,
    verification_key_size: usize,
    serialized: &[u8],
    t1: &mut [PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == verification_key_size - SEED_FOR_A_SIZE);

    // Inline literal `320` for the slice index (= `RING_ELEMENT_OF_T1S_SIZE`,
    // but that constant extracts to the opaque arithmetic
    // `(13 *! 256) /! 8` which Z3 cannot reduce under `fuel 0`). With the
    // literal plus the loop_invariant tracking length preservation +
    // `rows_in_a <= 8` multiplication bound, the slice index well-
    // formedness `(i+1) * 320 <= rows_in_a * 320` discharges; the
    // per-lane `< pow2 10` post is delegated to the trailing
    // `admit () (* Panic freedom *)` produced by
    // `verification_status(panic_free)`.
    for i in 0..rows_in_a {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= v rows_in_a /\
              v rows_in_a <= 8 /\
              v rows_in_a == Seq.length t1 /\
              Seq.length serialized == v rows_in_a * 320"#
        ));
        t1::deserialize::<SIMDUnit>(&serialized[i * 320..(i + 1) * 320], &mut t1[i]);
    }
}
