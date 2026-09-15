use crate::{
    arithmetic::{
        decompose_vector, make_hint, power2round_vector, use_hint, vector_infinity_norm_exceeds,
    },
    constants::*,
    encoding::{self},
    hash_functions::{shake128, shake256},
    matrix::{
        add_vectors, compute_as1_plus_s2, compute_matrix_x_mask, compute_w_approx,
        subtract_vectors, vector_times_ring_element,
    },
    ntt::ntt,
    polynomial::PolynomialRingElement,
    pre_hash::{DomainSeparationContext, PreHash},
    sample::{sample_challenge_ring_element, sample_mask_vector},
    samplex4::{self, X4Sampler},
    simd::traits::Operations,
    types::*,
    MLDSASignature,
};

pub(crate) mod instantiations;

#[cfg(not(eurydice))]
pub(crate) mod multiplexing;

#[libcrux_macros::ml_dsa_parameter_sets(44, 65, 87)]
pub(crate) mod generic {
    use super::*;

    // Derived constants
    const ROW_COLUMN: usize = ROWS_IN_A + COLUMNS_IN_A;
    const ROW_X_COLUMN: usize = ROWS_IN_A * COLUMNS_IN_A;
    const ERROR_RING_ELEMENT_SIZE: usize = error_ring_element_size(BITS_PER_ERROR_COEFFICIENT);
    const GAMMA1_RING_ELEMENT_SIZE: usize = gamma1_ring_element_size(BITS_PER_GAMMA1_COEFFICIENT);
    const COMMITMENT_RING_ELEMENT_SIZE: usize =
        commitment_ring_element_size(BITS_PER_COMMITMENT_COEFFICIENT);

    const BETA: i32 = beta(ONES_IN_VERIFIER_CHALLENGE, ETA);
    const COMMITMENT_VECTOR_SIZE: usize =
        commitment_vector_size(BITS_PER_COMMITMENT_COEFFICIENT, ROWS_IN_A);
    pub(crate) const SIGNING_KEY_SIZE: usize =
        signing_key_size(ROWS_IN_A, COLUMNS_IN_A, ERROR_RING_ELEMENT_SIZE);
    pub(crate) const VERIFICATION_KEY_SIZE: usize = verification_key_size(ROWS_IN_A);
    pub(crate) const SIGNATURE_SIZE: usize = signature_size(
        ROWS_IN_A,
        COLUMNS_IN_A,
        MAX_ONES_IN_HINT,
        COMMITMENT_HASH_SIZE,
        BITS_PER_GAMMA1_COEFFICIENT,
    );

    #[inline(always)]
    #[libcrux_macros::trusted(
        panic_free,
        "pending-proof(campaign): top-level signing body verified panic-free; functional post admitted"
    )]
    #[cfg_attr(
        hax,
        hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")
    )]
    // The requires clause
    //   signing_key.len() == SIGNING_KEY_SIZE && verification_key.len() == VERIFICATION_KEY_SIZE
    // is deliberately omitted here: the wrapper modules
    // (Ml_dsa_generic.Instantiations.{Avx2,Portable,Neon}.Ml_dsa_*_) call this function
    // with arbitrary &[u8] slices and have no analogous precondition to discharge it from.
    // TODO: add it once the wrapper Rust functions also surface the length precondition
    // (or once the function takes fixed-size arrays).
    #[cfg_attr(hax, hax_lib::ensures(|_| {
        let (pk_spec, sk_spec) = hacspec_ml_dsa::keygen_internal::<
            { HACSPEC_PARAMS.k },
            { HACSPEC_PARAMS.l },
            VERIFICATION_KEY_SIZE,
            SIGNING_KEY_SIZE,
        >(&randomness, &HACSPEC_PARAMS);
        future(signing_key).len() == signing_key.len()
            && future(verification_key).len() == verification_key.len()
            && future(signing_key) == &sk_spec[..]
            && future(verification_key) == &pk_spec[..]
    }))]
    #[libcrux_macros::trusted(inline-admit)]
    pub(crate) fn generate_key_pair<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
        signing_key: &mut [u8],
        verification_key: &mut [u8],
    ) {
        // The functional post is admitted: q60 of this function cliffs at rlimit
        // 400 (~65s), with k!63 producing ~624K quantifier instances. TODO: remove
        // the admit once trait-surface opacity is tightened and the q60 profile is
        // clean.
        trusted_admit!(
            "pending-proof(campaign): keygen q60 cliffs at rlimit 400 (k!63 ~624K \
             instances); re-admitted 2026-05-08 pending trait-surface opacity fixes"
        );
        // Check key sizes
        #[cfg(not(eurydice))]
        debug_assert!(signing_key.len() == SIGNING_KEY_SIZE);
        #[cfg(not(eurydice))]
        debug_assert!(verification_key.len() == VERIFICATION_KEY_SIZE);

        // 128 = SEED_FOR_A_SIZE + SEED_FOR_ERROR_VECTORS_SIZE + SEED_FOR_SIGNING_SIZE
        let mut seed_expanded = [0; 128];
        {
            let mut shake = Shake256Xof::init();
            shake.absorb(&randomness);
            shake.absorb_final(&[ROWS_IN_A as u8, COLUMNS_IN_A as u8]);
            shake.squeeze(&mut seed_expanded);
        }

        let (seed_for_a, seed_expanded) = seed_expanded.split_at(SEED_FOR_A_SIZE);
        let (seed_for_error_vectors, seed_for_signing) =
            seed_expanded.split_at(SEED_FOR_ERROR_VECTORS_SIZE);

        let mut s1_s2 = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_COLUMN];
        samplex4::sample_s1_and_s2::<SIMDUnit, Shake256X4>(ETA, seed_for_error_vectors, &mut s1_s2);

        // Bridge `sample_s1_and_s2`'s post (asymmetric opaque atom
        // `is_lane_range_poly_slice 0 eta_val s1_s2`, with eta_val ∈ {2, 4})
        // to the symmetric forms downstream consumers want:
        //   - `is_bounded_poly_slice 4 s1_s2` for `compute_as1_plus_s2`'s pre
        //     (Step 2's tight chain to power2round_vector)
        //   - `is_bounded_poly_slice 8380416 s1_s2` for the per-element
        //     `ntt` pre on `s1_ntt[i]` (after copy_from_slice from s1_s2)
        // The bridge lemma handles both bridging (asymmetric → symmetric)
        // and widening (b1 ≤ b2) in one call.
        proof!(
            r#"
            let eta_val : usize = match ${ETA} with
                                   | Libcrux_ml_dsa.Constants.Eta_Two -> mk_usize 2
                                   | Libcrux_ml_dsa.Constants.Eta_Four -> mk_usize 4 in
            Libcrux_ml_dsa.Polynomial.Spec.lemma_lane_range_pos_to_bounded_poly_slice
              eta_val (mk_usize 4) ${s1_s2};
            Libcrux_ml_dsa.Polynomial.Spec.lemma_lane_range_pos_to_bounded_poly_slice
              eta_val (mk_usize 8380416) ${s1_s2}
            "#
        );

        let mut t0 = [PolynomialRingElement::<SIMDUnit>::zero(); ROWS_IN_A];
        {
            let mut a_as_ntt = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_X_COLUMN];
            Sampler::matrix_flat::<SIMDUnit>(COLUMNS_IN_A, seed_for_a, &mut a_as_ntt);

            let mut s1_ntt = [PolynomialRingElement::<SIMDUnit>::zero(); COLUMNS_IN_A];
            s1_ntt.copy_from_slice(&s1_s2[0..COLUMNS_IN_A]);
            // Lift `is_bounded_poly_slice 8380416 s1_s2` (in scope from
            // an earlier bridge) to `is_bounded_poly_slice 8380416 s1_ntt`
            // via the copy_from_slice frame: s1_ntt[k] == s1_s2[k] for
            // k in [0, COLUMNS_IN_A).
            proof!(
                r#"
                let _:Prims.unit =
                  let aux (k: nat{k < Seq.length ${s1_ntt}}) :
                    Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                             (mk_usize 8380416) (Seq.index ${s1_ntt} k)) =
                    assert (Seq.index ${s1_ntt} k == Seq.index ${s1_s2} k);
                    Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
                      (mk_usize 8380416) ${s1_s2} k
                  in
                  Classical.forall_intro aux
                in
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
                  (mk_usize 8380416) ${s1_ntt}
                "#
            );
            for i in 0..s1_ntt.len() {
                // Truthful split: processed entries [0,i) are NTT_OUTPUT_BOUND
                // (forward ntt output, not reduced), unprocessed [i,len) are
                // still FIELD_MAX (sampled s1). After the loop the whole slice
                // is NTT_OUTPUT_BOUND, matching compute_as1_plus_s2's widened
                // s1_ntt pre. (Body is admit()-ed; this keeps the claim truthful.)
                hax_lib::loop_invariant!(|i: usize| fstar!(
                    r#"v $i <= Seq.length ${s1_ntt} /\
                       Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                           (mk_usize 75423744) (mk_usize 0) $i ${s1_ntt} /\
                       (forall (k:nat). v $i <= k /\ k < Seq.length ${s1_ntt} ==>
                          Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                              (mk_usize 8380416) (Seq.index ${s1_ntt} k))"#
                ));
                ntt(&mut s1_ntt[i]);
            }
            compute_as1_plus_s2::<SIMDUnit>(
                ROWS_IN_A,
                COLUMNS_IN_A,
                &mut a_as_ntt,
                &s1_ntt,
                &s1_s2,
                &mut t0,
            );
        }

        let mut t1 = [PolynomialRingElement::<SIMDUnit>::zero(); ROWS_IN_A];
        power2round_vector::<SIMDUnit>(&mut t0, &mut t1);

        // Bridge `is_lane_range_poly_slice 0 eta_val s1_s2` (post of
        // sample_s1_and_s2) to the bare `forall k j. is_pos_array_opaque
        // (match eta with...) ...` form that signing_key's `s1_2` pre wants.
        // One-shot via `lemma_lane_range_pos_to_pos_array_slice`.
        proof!(
            r#"
            let eta_val : usize = match ${ETA} with
                                   | Libcrux_ml_dsa.Constants.Eta_Two -> mk_usize 2
                                   | Libcrux_ml_dsa.Constants.Eta_Four -> mk_usize 4 in
            Libcrux_ml_dsa.Polynomial.Spec.lemma_lane_range_pos_to_pos_array_slice
              eta_val ${s1_s2}
            "#
        );

        // Write out the keys
        encoding::verification_key::generate_serialized::<SIMDUnit>(
            seed_for_a,
            &t1,
            verification_key,
        );
        encoding::signing_key::generate_serialized::<SIMDUnit, Shake256>(
            ETA,
            ERROR_RING_ELEMENT_SIZE,
            seed_for_a,
            seed_for_signing,
            verification_key,
            &s1_s2,
            &t0,
            signing_key,
        );
    }

    #[inline(always)]
    // The signing key arrives as a `&[u8]` slice (the fixed-array length is
    // erased at the generic boundary), so we need its exact length to prove the
    // five `split_at` calls below are panic-free and to discharge the
    // `t0_serialized` length precondition of `t0::deserialize_to_vector_then_ntt`.
    // The top-level API always passes a `[u8; SIGNING_KEY_SIZE]`, so the
    // instantiation wrappers discharge this from the array type.
    #[cfg_attr(hax, hax_lib::requires(fstar!(r#"Seq.length $signing_key == v ${SIGNING_KEY_SIZE}"#)))]
    // Like verify_internal, the pre-loop VC (5 split_at + s1/s2/t0 deserialize +
    // matrix_flat + derive_message_representative + shake) saturates the module
    // default rlimit as one monolithic query; split + z3refresh lands it.
    #[cfg_attr(
        hax,
        hax_lib::fstar::before(r#"#restart-solver"#),
        hax_lib::fstar::options("--z3rlimit 800 --split_queries always")
    )]
    // The rejection loop's invariant hint clause uses the companion predicate
    // `Ml_dsa_generic_theory.hint_count_bounded` (see that module for why it
    // must be a top-level `match` atom — F* error 114 otherwise).
    #[libcrux_macros::trusted(inline-admit)]
    pub(crate) fn sign_internal<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        signing_key: &[u8],
        message: &[u8],
        domain_separation_context: Option<DomainSeparationContext>,
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
        signature: &mut [u8; SIGNATURE_SIZE],
    ) -> Result<(), SigningError> {
        // Split the signing key into its parts.
        let (seed_for_a, remaining_serialized) = signing_key.split_at(SEED_FOR_A_SIZE);
        let (seed_for_signing, remaining_serialized) =
            remaining_serialized.split_at(SEED_FOR_SIGNING_SIZE);
        let (verification_key_hash, remaining_serialized) =
            remaining_serialized.split_at(BYTES_FOR_VERIFICATION_KEY_HASH);

        let (s1_serialized, remaining_serialized) =
            remaining_serialized.split_at(ERROR_RING_ELEMENT_SIZE * COLUMNS_IN_A);
        let (s2_serialized, t0_serialized) =
            remaining_serialized.split_at(ERROR_RING_ELEMENT_SIZE * ROWS_IN_A);

        // The T0 remainder length must chain back through all five splits to
        // SIGNING_KEY_SIZE.  Unfolding `signing_key_size` (via assert_norm on the
        // constant relationship) lets the ERROR_RING_ELEMENT_SIZE terms cancel so
        // t0_serialized's length equals RING_ELEMENT_OF_T0S_SIZE * ROWS_IN_A,
        // which is exactly what t0::deserialize_to_vector_then_ntt requires
        // (t0_as_ntt has length ROWS_IN_A).  Placed BEFORE the ring-element array
        // declarations so the five-slice reconciliation runs in a small typing
        // context — for ml-dsa-87 (k=8/l=7) the same assert after the arrays
        // saturates rlimit 800; here it lands at ~73.
        proof!(
            r#"
            assert_norm (v ${SIGNING_KEY_SIZE} ==
                v ${SEED_FOR_A_SIZE} +
                v ${SEED_FOR_SIGNING_SIZE} +
                v ${BYTES_FOR_VERIFICATION_KEY_HASH} +
                v ${ERROR_RING_ELEMENT_SIZE} * v ${COLUMNS_IN_A} +
                v ${ERROR_RING_ELEMENT_SIZE} * v ${ROWS_IN_A} +
                v ${RING_ELEMENT_OF_T0S_SIZE} * v ${ROWS_IN_A});
            assert (Seq.length ${t0_serialized} ==
                v ${RING_ELEMENT_OF_T0S_SIZE} * v ${ROWS_IN_A})
            "#
        );

        // Deserialize s1, s2, and t0.
        let mut s1_as_ntt = [PolynomialRingElement::zero(); COLUMNS_IN_A];
        let mut s2_as_ntt = [PolynomialRingElement::zero(); ROWS_IN_A];
        let mut t0_as_ntt = [PolynomialRingElement::zero(); ROWS_IN_A];

        encoding::error::deserialize_to_vector_then_ntt::<SIMDUnit>(
            ETA,
            ERROR_RING_ELEMENT_SIZE,
            s1_serialized,
            &mut s1_as_ntt,
        );
        encoding::error::deserialize_to_vector_then_ntt::<SIMDUnit>(
            ETA,
            ERROR_RING_ELEMENT_SIZE,
            s2_serialized,
            &mut s2_as_ntt,
        );
        encoding::t0::deserialize_to_vector_then_ntt::<SIMDUnit>(t0_serialized, &mut t0_as_ntt);

        // Sample matrix A.
        let mut matrix = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_X_COLUMN];
        Sampler::matrix_flat::<SIMDUnit>(COLUMNS_IN_A, seed_for_a, &mut matrix);

        let mut message_representative = [0; MESSAGE_REPRESENTATIVE_SIZE];
        derive_message_representative::<Shake256Xof>(
            verification_key_hash,
            &domain_separation_context,
            message,
            &mut message_representative,
        );

        let mut mask_seed = [0; MASK_SEED_SIZE];
        {
            let mut shake = Shake256Xof::init();
            shake.absorb(seed_for_signing);
            shake.absorb(&randomness);
            shake.absorb_final(&message_representative);

            shake.squeeze(&mut mask_seed);
        }

        let mut domain_separator_for_mask: u16 = 0;
        let mut attempt = 0;

        // Return values.
        // Required because we can't return early.
        // See https://github.com/hacspec/hax/issues/1171
        let mut commitment_hash = None;
        let mut signer_response = None;
        let mut hint = None;

        // As specified in [FIPS 204, Appendix C], the minimum number of
        // attempts in this rejection sampling loop is 814. This puts the
        // probability of failure at 2⁻²⁵⁶ or less.
        //
        // [FIPS 204, Appendix C]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf#appendix.C
        // Rejection-sampling loop.  The invariant tracks: `attempt` stays
        // within its bound (well-foundedness of the decreases measure); the
        // mask domain separator grows by at most COLUMNS_IN_A per attempt; and
        // — once a signature is accepted — the hint's Hamming weight is within
        // MAX_ONES_IN_HINT (consumed by the post-loop `serialize` precondition).
        // The loop-body callee preconditions + invariant maintenance remain a
        // follow-up (body admit below); the invariant establishment, decreases,
        // and the whole post-loop ARE discharged (pre-loop admit removed).
        while attempt < REJECTION_SAMPLE_BOUND_SIGN {
            hax_lib::loop_invariant!(fstar!(
                r#"
                v ${attempt} <= v ${REJECTION_SAMPLE_BOUND_SIGN} /\
                v ${domain_separator_for_mask} <= v ${attempt} * v ${COLUMNS_IN_A} /\
                Libcrux_ml_dsa.Ml_dsa_generic_theory.hint_count_bounded ${hint} ${MAX_ONES_IN_HINT} /\
                Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) ${s1_as_ntt} /\
                Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) ${s2_as_ntt} /\
                Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) ${t0_as_ntt} /\
                Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${matrix}
                "#
            ));
            hax_lib::loop_decreases!(REJECTION_SAMPLE_BOUND_SIGN - attempt);
            // The loop-body callee preconditions up to `decompose_vector` are
            // discharged below (sample_mask_vector, the mask NTT, compute_matrix_x_mask,
            // decompose_vector).  The remaining ones — from `serialize_vector` onward —
            // and the per-iteration invariant maintenance are still admitted (see the
            // `admit ()` before `serialize_vector` below).
            attempt += 1;

            let mut mask = [PolynomialRingElement::zero(); COLUMNS_IN_A];
            let mut w0 = [PolynomialRingElement::zero(); ROWS_IN_A];
            let mut commitment = [PolynomialRingElement::zero(); ROWS_IN_A];

            sample_mask_vector::<SIMDUnit, Shake256, Shake256X4>(
                COLUMNS_IN_A,
                GAMMA1_EXPONENT,
                &mask_seed,
                &mut domain_separator_for_mask,
                &mut mask,
            );

            {
                let mut a_x_mask = [PolynomialRingElement::zero(); ROWS_IN_A];
                let mut mask_ntt = mask.clone();
                // NTT the mask.  Loop invariant mirrors `vector_times_ring_element`:
                // processed prefix is `is_bounded_poly_range 75423744` (ntt's output
                // bound), the untouched suffix still equals `mask` (which carries the
                // `is_bounded_poly_slice 8380416` bound sample_mask_vector produced),
                // giving ntt's `is_bounded_poly 8380416` precond on each element.
                proof!(
                    r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                         (mk_usize 75423744) (mk_usize 0) (mk_usize 0) ${mask_ntt}"#
                );
                for i in 0..mask_ntt.len() {
                    hax_lib::loop_invariant!(|i: usize| fstar!(
                        r#"v $i <= Seq.length ${mask_ntt} /\
                          Seq.length ${mask_ntt} == Seq.length ${mask} /\
                          Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${mask} /\
                          Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                              (mk_usize 75423744) (mk_usize 0) $i ${mask_ntt} /\
                          (forall (k: nat). v $i <= k /\ k < Seq.length ${mask_ntt} ==>
                              Seq.index ${mask_ntt} k == Seq.index ${mask} k)"#
                    ));
                    #[cfg(hax)]
                    let iter_start: &[PolynomialRingElement<SIMDUnit>] =
                        mask_ntt.to_vec().as_slice();
                    proof!(
                        r#"assert (Seq.index ${mask_ntt} (v $i) == Seq.index ${mask} (v $i));
                           Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
                             (mk_usize 8380416) ${mask} (v $i)"#
                    );
                    ntt(&mut mask_ntt[i]);
                    proof!(
                        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
                             (mk_usize 75423744) $i iter_start ${mask_ntt}"#
                    );
                }
                // compute_matrix_x_mask precond: per-element bounds on matrix (from
                // the loop invariant's `is_bounded_poly_slice 8380416 matrix`) and
                // mask_ntt (from the NTT loop's exit range), plus the length link
                // `Seq.length matrix == ROWS_IN_A * COLUMNS_IN_A` — pin the two param
                // constants to their literals via normalize_term so the product reduces.
                proof!(
                    r#"
                    let _:Prims.unit =
                      let aux (k: nat{k < Seq.length ${matrix}}) :
                        Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416) (Seq.index ${matrix} k)) =
                        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) ${matrix} k
                      in Classical.forall_intro aux
                    in
                    let _:Prims.unit =
                      let aux (k: nat{k < Seq.length ${mask_ntt}}) :
                        Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) (Seq.index ${mask_ntt} k)) =
                        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup (mk_usize 75423744) (mk_usize 0)
                          (Core_models.Slice.impl__len (${mask_ntt} <: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)))
                          ${mask_ntt} k
                      in Classical.forall_intro aux
                    in
                    assert_norm (v ${ROWS_IN_A} == normalize_term (v ${ROWS_IN_A}));
                    assert_norm (v ${COLUMNS_IN_A} == normalize_term (v ${COLUMNS_IN_A}))"#
                );
                compute_matrix_x_mask::<SIMDUnit>(
                    ROWS_IN_A,
                    COLUMNS_IN_A,
                    &matrix,
                    &mask_ntt,
                    &mut a_x_mask,
                );
                // decompose_vector precond: `is_bounded_poly_slice 8380416 a_x_mask`
                // (from compute_matrix_x_mask's per-row post) and the gamma2 disjunction
                // (GAMMA2 is a param-set literal; normalize the whole disjunction).
                proof!(
                    r#"
                    let _:Prims.unit =
                      let aux (k: nat{k < Seq.length ${a_x_mask}}) :
                        Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416) (Seq.index ${a_x_mask} k)) = ()
                      in Classical.forall_intro aux;
                      Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro (mk_usize 8380416) ${a_x_mask}
                    in
                    assert_norm (v ${GAMMA2} == v Libcrux_ml_dsa.Constants.v_GAMMA2_V261_888_ \/
                                 v ${GAMMA2} == v Libcrux_ml_dsa.Constants.v_GAMMA2_V95_232_)"#
                );
                decompose_vector::<SIMDUnit>(
                    ROWS_IN_A,
                    GAMMA2,
                    &a_x_mask,
                    &mut w0,
                    &mut commitment,
                );
            }

            let mut commitment_hash_candidate = [0; COMMITMENT_HASH_SIZE];
            {
                let mut commitment_serialized = [0u8; COMMITMENT_VECTOR_SIZE];
                // decompose_vector gives `commitment` (its
                // HighBits output) the tight non-negative lane range
                // `is_lane_range_poly_slice 0 (use_hint_serialize_bound GAMMA2)`.
                // Mirror the verify path (verify_internal serialize_vector block):
                // pin `use_hint_serialize_bound GAMMA2 == pow2(COMMITMENT_RING_
                // ELEMENT_SIZE/32) - 1`, convert the lane range to serialize_vector's
                // per-unit `is_pos_array_opaque` precondition, and pin the length
                // constants so `Seq.length serialized == COMMITMENT_RING_ELEMENT_SIZE *
                // Seq.length commitment` reduces to the literal COMMITMENT_VECTOR_SIZE.
                proof!(
                    r#"assert_norm (v (Libcrux_ml_dsa.Arithmetic.use_hint_serialize_bound ${GAMMA2}) == pow2 (v ${COMMITMENT_RING_ELEMENT_SIZE} / 32) - 1)"#
                );
                proof!(
                    r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_lane_range_pos_to_pos_array_slice
                         (Libcrux_ml_dsa.Arithmetic.use_hint_serialize_bound ${GAMMA2}) ${commitment}"#
                );
                proof!(
                    r#"assert_norm (v ${COMMITMENT_RING_ELEMENT_SIZE} == normalize_term (v ${COMMITMENT_RING_ELEMENT_SIZE}));
                       assert_norm (v ${ROWS_IN_A} == normalize_term (v ${ROWS_IN_A}))"#
                );
                encoding::commitment::serialize_vector::<SIMDUnit>(
                    COMMITMENT_RING_ELEMENT_SIZE,
                    &commitment,
                    &mut commitment_serialized,
                );
                let mut shake = Shake256Xof::init();
                shake.absorb(&message_representative);
                shake.absorb_final(&commitment_serialized);

                shake.squeeze(&mut commitment_hash_candidate);
            }
            // The shake absorb/absorb_final/squeeze XOF ops above are verified:
            // Shake256Xof::{init,absorb,absorb_final,squeeze} all carry
            // `requires(true)`, mirroring verify_internal's proven shake block and
            // sign_internal's own mask_seed shake block (both above this point).
            let mut verifier_challenge = PolynomialRingElement::zero();
            sample_challenge_ring_element::<SIMDUnit, Shake256>(
                &commitment_hash_candidate,
                ONES_IN_VERIFIER_CHALLENGE,
                &mut verifier_challenge,
            );
            ntt(&mut verifier_challenge);
            // The sample_challenge_ring_element + ntt calls above are verified:
            // sample_challenge carries `requires(true)` and ensures
            // `is_bounded_poly 8380416 verifier_challenge`, which discharges ntt's
            // precond `is_bounded_poly 8380416` directly (same opaque atom, same
            // term); ntt then yields `is_bounded_poly 75423744 verifier_challenge`.
            // Still admitted: the three vector_infinity_norm_exceeds calls,
            // add_vectors(w0, challenge_times_t0), make_hint, the Bundle t_Option
            // invariant, and the per-iteration invariant maintenance.

            // We need to clone here in case we need s1_as_ntt or s2_as_ntt again in
            // another iteration of the loop.
            let mut challenge_times_s1 = s1_as_ntt.clone();
            let mut challenge_times_s2 = s2_as_ntt.clone();

            vector_times_ring_element::<SIMDUnit>(&mut challenge_times_s1, &verifier_challenge);
            vector_times_ring_element::<SIMDUnit>(&mut challenge_times_s2, &verifier_challenge);
            // The two vector_times_ring_element calls above are verified.
            // Each requires `is_bounded_poly_slice 75423744 vector` on the (cloned) s1/s2
            // operand and `is_bounded_poly 75423744 ring_element` on verifier_challenge.
            // The loop invariant carries `is_bounded_poly_slice 75423744 {s1,s2}_as_ntt`;
            // the array clone's return type is refined `r == self`, so the bound transfers
            // to challenge_times_s{1,2} by congruence. verifier_challenge carries
            // `is_bounded_poly 75423744` from the ntt post above. Both vtre calls then
            // yield `is_bounded_poly_slice 8380416 challenge_times_s{1,2}`.

            add_vectors::<SIMDUnit>(COLUMNS_IN_A, &mut mask, &challenge_times_s1);
            subtract_vectors::<SIMDUnit>(ROWS_IN_A, &mut w0, &challenge_times_s2);
            // The add_vectors + subtract_vectors calls above are verified.
            // Both share the contract `requires is_bounded_poly_slice 8380416 {lhs,rhs}`,
            // `ensures is_bounded_poly_slice 16760832 lhs_future`. All four operand bounds
            // are already in scope: `mask` carries `is_bounded_poly_slice 8380416` from
            // sample_mask_vector (the mask NTT loop mutated the clone `mask_ntt`, not
            // `mask`); `w0` carries it from decompose_vector's low post; and
            // challenge_times_s{1,2} carry it from the vector_times_ring_element post
            // above. Lengths are COLUMNS_IN_A / ROWS_IN_A respectively. After these,
            // `mask` and `w0` both carry `is_bounded_poly_slice 16760832` (= 2q),
            // matching the 2q-relaxed precond of the vector_infinity_norm_exceeds
            // wrappers below.
            trusted_admit!(
                "pending-proof(campaign): sign_internal rejection-loop chunk-7 body \
                 (vector_infinity_norm_exceeds region) proof pending; chunks 1-6 verified"
            );

            if vector_infinity_norm_exceeds::<SIMDUnit>(&mask, (1 << GAMMA1_EXPONENT) - BETA) {
                // XXX: https://github.com/hacspec/hax/issues/1171
                // continue;
            } else {
                if vector_infinity_norm_exceeds::<SIMDUnit>(&w0, GAMMA2 - BETA) {
                    // XXX: https://github.com/hacspec/hax/issues/1171
                    // continue;
                } else {
                    // We need to clone here in case we need t0_as_ntt again in another iteration
                    // of the loop.
                    let mut challenge_times_t0 = t0_as_ntt.clone();
                    vector_times_ring_element::<SIMDUnit>(
                        &mut challenge_times_t0,
                        &verifier_challenge,
                    );
                    if vector_infinity_norm_exceeds::<SIMDUnit>(&challenge_times_t0, GAMMA2) {
                        // XXX: https://github.com/hacspec/hax/issues/1171
                        // continue;
                    } else {
                        add_vectors::<SIMDUnit>(ROWS_IN_A, &mut w0, &challenge_times_t0);
                        let mut hint_candidate = [[0; COEFFICIENTS_IN_RING_ELEMENT]; ROWS_IN_A];
                        let ones_in_hint =
                            make_hint::<SIMDUnit>(&w0, &commitment, GAMMA2, &mut hint_candidate);

                        if ones_in_hint > MAX_ONES_IN_HINT {
                            // XXX: https://github.com/hacspec/hax/issues/1171
                            // continue;
                        } else {
                            attempt = REJECTION_SAMPLE_BOUND_SIGN; // exit loop now
                            commitment_hash = Some(commitment_hash_candidate);
                            signer_response = Some(mask);
                            hint = Some(hint_candidate);
                        }
                    }
                }
            }
        }

        let commitment_hash = match commitment_hash {
            Some(commitment_hash) => commitment_hash,
            None => return Err(SigningError::RejectionSamplingError),
        };

        let signer_response = match signer_response {
            Some(signer_response) => signer_response,
            None => return Err(SigningError::RejectionSamplingError),
        };

        let hint = match hint {
            Some(hint) => hint,
            None => return Err(SigningError::RejectionSamplingError),
        };

        // Discharge serialize's length preconditions: the concrete parameter
        // constants satisfy gamma1_ring_element_size = 32·(1+gamma1_exponent),
        // and SIGNATURE_SIZE decomposes into the commitment-hash / response /
        // hint sections.  `norm [delta]` fully reduces `signature_size` to its
        // literal (plain SMT reduction of it is flaky across parameter sets),
        // then SMT closes the array-length link via the type refinement.
        proof!(
            r#"
            assert_norm (v ${GAMMA1_EXPONENT} == 17 \/ v ${GAMMA1_EXPONENT} == 19);
            assert_norm (v ${GAMMA1_RING_ELEMENT_SIZE} == 32 * (1 + v ${GAMMA1_EXPONENT}));
            assert_norm (v ${SIGNATURE_SIZE} ==
                v ${COMMITMENT_HASH_SIZE} +
                v ${GAMMA1_RING_ELEMENT_SIZE} * v ${COLUMNS_IN_A} +
                v ${MAX_ONES_IN_HINT} + v ${ROWS_IN_A});
            assert (Seq.length ${signature} == v ${SIGNATURE_SIZE})
              by (FStar.Tactics.norm [primops; iota; zeta; delta]; FStar.Tactics.smt ());
            assert_norm (v ${MAX_ONES_IN_HINT} + v ${ROWS_IN_A} <= max_usize)
            "#
        );

        encoding::signature::serialize::<SIMDUnit>(
            &commitment_hash,
            &signer_response,
            &hint,
            COMMITMENT_HASH_SIZE,
            COLUMNS_IN_A,
            ROWS_IN_A,
            GAMMA1_EXPONENT,
            GAMMA1_RING_ELEMENT_SIZE,
            MAX_ONES_IN_HINT,
            signature,
        );

        Ok(())
    }

    // Move the signer response into the NTT domain.  Loop invariant: the
    // processed prefix [0,i) is NTT_OUTPUT_BOUND (75423744), the unprocessed
    // suffix [i,len) is still FIELD_MAX (8380416, from the caller's precondition
    // = signature::deserialize's post); after the loop the whole slice is
    // 75423744, matching compute_w_approx's signer_response precondition.
    //
    // Factored out of verify_internal so the `fold_range` term does not thread
    // through verify_internal's top-level VC: at the compute_w_approx call site
    // the response is an opaque value carrying only `is_bounded_poly_slice
    // 75423744` (this fn's post), not the whole per-row NTT fold.  That keeps
    // verify_internal's top-level VC size-independent — the ML-DSA-87 (k=8/l=7)
    // saturation was the compute_w_approx precondition sub-query carrying this
    // fold term.  (Mirror of generate_key_pair's s1_ntt loop.)
    #[inline(always)]
    #[cfg_attr(
        hax,
        hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")
    )]
    #[cfg_attr(hax, hax_lib::requires(fstar!(r#"
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $signer_response"#)))]
    #[cfg_attr(hax, hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${signer_response}_future == Seq.length $signer_response /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) ${signer_response}_future"#)))]
    fn ntt_signer_response<SIMDUnit: Operations>(
        signer_response: &mut [PolynomialRingElement<SIMDUnit>; COLUMNS_IN_A],
    ) {
        proof!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                 (mk_usize 75423744) (mk_usize 0) (mk_usize 0) ${signer_response}"#
        );
        for i in 0..signer_response.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"v $i <= Seq.length ${signer_response} /\
                   Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                       (mk_usize 75423744) (mk_usize 0) $i ${signer_response} /\
                   (forall (k:nat). v $i <= k /\ k < Seq.length ${signer_response} ==>
                      Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                          (mk_usize 8380416) (Seq.index ${signer_response} k))"#
            ));
            // The suffix bound at k=i is exactly ntt's FIELD_MAX precondition.
            proof!(
                r#"assert (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                            (mk_usize 8380416) (Seq.index ${signer_response} (v $i)))"#
            );
            #[cfg(hax)]
            let iter_start: &[PolynomialRingElement<SIMDUnit>] =
                signer_response.to_vec().as_slice();
            ntt(&mut signer_response[i]);
            // ntt's post gives is_bounded_poly 75423744 on the updated entry; extend
            // the processed range from [0,i) to [0,i+1). The F* lemma logic lives
            // in Polynomial.Spec.
            proof!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
                     (mk_usize 75423744) $i iter_start ${signer_response}"#
            );
        }
        // After the loop the processed range covers [0,len); lift range -> slice.
        proof!(
            r#"
            let _:Prims.unit =
              let aux (k:nat{k < Seq.length ${signer_response}}) :
                Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                         (mk_usize 75423744) (Seq.index ${signer_response} k)) =
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
                  (mk_usize 75423744) (mk_usize 0)
                  (Core_models.Slice.impl__len ${signer_response})
                  ${signer_response} k
              in Classical.forall_intro aux
            in
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
              (mk_usize 75423744) ${signer_response}"#
        );
    }

    // Fixed-length wrapper around `compute_w_approx`.  `compute_w_approx` takes
    // *slices*, so it requires `Seq.length matrix == rows_in_a * columns_in_a`
    // (a nonlinear product) as an explicit precondition.  Discharging that
    // conjunct inside verify_internal's large ML-DSA-87 (k=8/l=7) context
    // saturates Z3 — context pollution poisons even the trivial ground fact
    // `56 == 8 * 7` (this was the verify_internal cliff, localized to
    // compute_w_approx's precondition sub-query).  Taking the operands as
    // fixed-size arrays lets the `Seq.length` facts reduce definitionally and
    // the product discharge by `assert_norm` in *this* helper's minimal
    // context; verify_internal's call site is then left only the (opaque,
    // cheap) bound atoms — keeping its top-level VC size-independent.
    #[inline(always)]
    #[cfg_attr(
        hax,
        hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")
    )]
    #[cfg_attr(hax, hax_lib::requires(fstar!(r#"
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $matrix /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $signer_response /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $verifier_challenge_as_ntt /\
        Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice (mk_usize 0) (mk_usize 261631) $t1"#)))]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[cfg_attr(hax, hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${t1}_future == Seq.length $t1 /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 4211177) ${t1}_future"#)))]
    fn compute_w_approx_fixed<SIMDUnit: Operations>(
        matrix: &[PolynomialRingElement<SIMDUnit>; ROW_X_COLUMN],
        signer_response: &[PolynomialRingElement<SIMDUnit>; COLUMNS_IN_A],
        verifier_challenge_as_ntt: &PolynomialRingElement<SIMDUnit>,
        t1: &mut [PolynomialRingElement<SIMDUnit>; ROWS_IN_A],
    ) {
        // `ROW_X_COLUMN` is *defined* as `ROWS_IN_A * COLUMNS_IN_A`; assert_norm
        // reduces each per monomorphization, discharging compute_w_approx's
        // `Seq.length matrix == rows * cols` (matrix has fixed length ROW_X_COLUMN).
        proof!(
            r#"assert_norm (v ${ROWS_IN_A} <= 8);
               assert_norm (v ${COLUMNS_IN_A} <= 7);
               assert_norm (v ${ROW_X_COLUMN} == v ${ROWS_IN_A} * v ${COLUMNS_IN_A})"#
        );
        compute_w_approx::<SIMDUnit>(
            ROWS_IN_A,
            COLUMNS_IN_A,
            matrix,
            signer_response,
            verifier_challenge_as_ntt,
            t1,
        );
    }

    /// The internal verification API.
    ///
    /// If no `domain_separation_context` is supplied, it is assumed that
    /// `message` already contains the domain separation.
    #[allow(non_snake_case)]
    #[inline(always)]
    // verify_internal's monolithic VC splits into ~160 sub-queries; the heaviest
    // (compute_w_approx's precondition in the ML-DSA-87 context, k=8/l=7) is
    // budget-bound.  `#restart-solver` is REQUIRED alongside `--split_queries always`:
    // without it, Z3's state accumulates across sub-queries in the full-module build
    // and that one query drifts past 800 (flaky cold); with a fresh solver per
    // sub-query it lands at ~640/800 deterministically.  (44/65 use <45.)
    #[cfg_attr(
        hax,
        hax_lib::fstar::before(r#"#restart-solver"#),
        hax_lib::fstar::options("--z3rlimit 800 --split_queries always")
    )]
    pub(crate) fn verify_internal<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
    >(
        verification_key: &[u8; VERIFICATION_KEY_SIZE],
        message: &[u8],
        domain_separation_context: Option<DomainSeparationContext>,
        signature_serialized: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), VerificationError> {
        // Concrete-constant facts, stated GENERICALLY across the 44/65/87
        // parameter sets (each is either definitional — e.g. ROW_X_COLUMN is
        // *defined* as ROWS_IN_A*COLUMNS_IN_A — or a disjunction/inequality that
        // holds for every set), so `assert_norm` reduces each per monomorphization.
        // They discharge the concrete-constant preconditions of the callees below
        // (signature::deserialize, compute_w_approx, use_hint, serialize_vector,
        // vector_infinity_norm_exceeds's bound arithmetic).
        proof!(
            r#"
            assert_norm (v ${GAMMA1_EXPONENT} == 17 \/ v ${GAMMA1_EXPONENT} == 19);
            assert_norm (v ${GAMMA1_RING_ELEMENT_SIZE} == 32 * (1 + v ${GAMMA1_EXPONENT}));
            assert_norm (v ${ROWS_IN_A} > 0);
            assert_norm (v ${ROWS_IN_A} <= 8);
            assert_norm (v ${COLUMNS_IN_A} <= 7);
            assert_norm (v ${ROW_X_COLUMN} == v ${ROWS_IN_A} * v ${COLUMNS_IN_A});
            assert_norm (v ${BETA} >= 0 /\ v ${BETA} <= 524288);
            assert_norm (v ${SIGNATURE_SIZE} == v ${COMMITMENT_HASH_SIZE} + v ${GAMMA1_RING_ELEMENT_SIZE} * v ${COLUMNS_IN_A} + v ${MAX_ONES_IN_HINT} + v ${ROWS_IN_A});
            assert_norm (v ${GAMMA2} == v ${crate::constants::GAMMA2_V95_232} \/ v ${GAMMA2} == v ${crate::constants::GAMMA2_V261_888});
            assert_norm (v ${COMMITMENT_RING_ELEMENT_SIZE} == 128 \/ v ${COMMITMENT_RING_ELEMENT_SIZE} == 192);
            assert_norm (v (Libcrux_ml_dsa.Arithmetic.use_hint_serialize_bound ${GAMMA2}) == pow2 (v ${COMMITMENT_RING_ELEMENT_SIZE} / 32) - 1)
            "#
        );
        // Per FIPS 204 §3.6.2, an implementation that accepts inputs for σ
        // or pk of any other length than specified shall return false.  The
        // typed arguments enforce this at compile time for direct Rust
        // callers; these asserts mirror the keygen pattern (lines 68-70)
        // and document the invariant for FFI / C-extraction surfaces where
        // the array length may be erased.
        #[cfg(not(eurydice))]
        debug_assert!(verification_key.len() == VERIFICATION_KEY_SIZE);
        #[cfg(not(eurydice))]
        debug_assert!(signature_serialized.len() == SIGNATURE_SIZE);

        let (seed_for_a, t1_serialized) = verification_key.split_at(SEED_FOR_A_SIZE);
        let mut t1 = [PolynomialRingElement::<SIMDUnit>::zero(); ROWS_IN_A];
        encoding::verification_key::deserialize::<SIMDUnit>(
            ROWS_IN_A,
            VERIFICATION_KEY_SIZE,
            t1_serialized,
            &mut t1,
        );

        let mut deserialized_commitment_hash = [0u8; COMMITMENT_HASH_SIZE];
        let mut deserialized_signer_response = [PolynomialRingElement::zero(); COLUMNS_IN_A];
        let mut deserialized_hint = [[0i32; COEFFICIENTS_IN_RING_ELEMENT]; ROWS_IN_A];

        match encoding::signature::deserialize::<SIMDUnit>(
            COLUMNS_IN_A,
            ROWS_IN_A,
            COMMITMENT_HASH_SIZE,
            GAMMA1_EXPONENT,
            GAMMA1_RING_ELEMENT_SIZE,
            MAX_ONES_IN_HINT,
            SIGNATURE_SIZE,
            signature_serialized,
            &mut deserialized_commitment_hash,
            &mut deserialized_signer_response,
            &mut deserialized_hint,
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        // `vector_infinity_norm_exceeds`'s input precondition was relaxed from
        // `FIELD_MAX` to `2 * FIELD_MAX = 16760832` (so `sign_internal` can
        // pass its `< 2q`-bounded `w0`/`mask`).  `deserialize` gives the tight
        // `is_bounded_poly_slice 8380416` post; weaken it to the `16760832`
        // pre here (monotone in the bound: `q - 1 <= 2 * (q - 1)`).
        proof!(
            "Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_higher (mk_usize 8380416) (mk_usize 16760832) $deserialized_signer_response"
        );
        // We use if-else branches because early returns will not go through hax.
        if vector_infinity_norm_exceeds::<SIMDUnit>(
            &deserialized_signer_response,
            (1 << GAMMA1_EXPONENT) - BETA,
        ) {
            return Err(VerificationError::SignerResponseExceedsBoundError);
        }
        let mut matrix = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_X_COLUMN];
        Sampler::matrix_flat::<SIMDUnit>(COLUMNS_IN_A, seed_for_a, &mut matrix);

        let mut verification_key_hash = [0; BYTES_FOR_VERIFICATION_KEY_HASH];
        Shake256::shake256(verification_key, &mut verification_key_hash);

        let mut message_representative = [0; MESSAGE_REPRESENTATIVE_SIZE];
        derive_message_representative::<Shake256Xof>(
            &verification_key_hash,
            &domain_separation_context,
            message,
            &mut message_representative,
        );

        let mut verifier_challenge = PolynomialRingElement::zero();
        sample_challenge_ring_element::<SIMDUnit, Shake256>(
            &deserialized_commitment_hash,
            ONES_IN_VERIFIER_CHALLENGE,
            &mut verifier_challenge,
        );
        ntt(&mut verifier_challenge);

        // Move signer response into the NTT domain.  Factored into a helper so
        // the per-row NTT `fold_range` term does not thread through this
        // function's top-level VC (see `ntt_signer_response`); at the
        // compute_w_approx call site the response is opaque, carrying only
        // `is_bounded_poly_slice 75423744` — this keeps verify_internal's VC
        // size-independent (fixes the ML-DSA-87 k=8/l=7 saturation).
        ntt_signer_response::<SIMDUnit>(&mut deserialized_signer_response);
        // Widen t1's lane range 0..1023 (verification_key::deserialize post) to
        // 0..261631 (compute_w_approx's precondition; [0,1023] subset [0,261631]).
        proof!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_widen
                 (mk_usize 0) (mk_usize 1023) (mk_usize 261631) ${t1}"#
        );
        // compute_w_approx requires `Seq.length matrix == rows_in_a * columns_in_a`
        // (a nonlinear product) because it takes slices; discharging that in this
        // large k=8/l=7 context saturates Z3 (the ML-DSA-87 verify_internal cliff).
        // The fixed-array wrapper `compute_w_approx_fixed` discharges the length
        // facts in its own minimal context, so here only the (opaque, cheap) bound
        // atoms remain — keeping this VC size-independent.
        compute_w_approx_fixed::<SIMDUnit>(
            &matrix,
            &deserialized_signer_response,
            &verifier_challenge,
            &mut t1,
        );

        // Compute the commitment hash again to validate the signature.
        let mut recomputed_commitment_hash = [0; COMMITMENT_HASH_SIZE];
        {
            // Weaken compute_w_approx's `is_bounded_poly_slice 4211177 t1` to the
            // FIELD_MAX bound (8380416) that use_hint's precondition wants
            // (4211177 <= 8380416), per element then re-introduce the slice atom.
            proof!(
                r#"
                let _:Prims.unit =
                  let aux (k:nat{k < Seq.length ${t1}}) :
                    Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416) (Seq.index ${t1} k)) =
                    Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 4211177) ${t1} k;
                    Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher (mk_usize 4211177) (mk_usize 8380416) (Seq.index ${t1} k)
                  in Classical.forall_intro aux
                in
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro (mk_usize 8380416) ${t1}"#
            );
            use_hint::<SIMDUnit>(GAMMA2, &deserialized_hint, &mut t1);
            let mut commitment_serialized = [0u8; COMMITMENT_VECTOR_SIZE];
            // use_hint's post `is_lane_range_poly_slice 0 (use_hint_serialize_bound GAMMA2) t1`
            // -> the per-simd-unit non-negative `is_pos_array_opaque` bound that
            // serialize_vector requires.  The top-of-body assert_norm established
            // `use_hint_serialize_bound GAMMA2 == pow2 (COMMITMENT_RING_ELEMENT_SIZE/32) - 1`,
            // so the two bounds coincide.
            proof!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_lane_range_pos_to_pos_array_slice
                     (Libcrux_ml_dsa.Arithmetic.use_hint_serialize_bound ${GAMMA2}) ${t1}"#
            );
            // serialize_vector's `Seq.length serialized == ring_element_size * Seq.length vector`
            // sees serialized/t1 lengths as resolved literals; pin the constants so
            // COMMITMENT_RING_ELEMENT_SIZE * (len t1 = ROWS_IN_A) reduces to the
            // literal COMMITMENT_VECTOR_SIZE.
            proof!(
                r#"assert_norm (v ${COMMITMENT_RING_ELEMENT_SIZE} == normalize_term (v ${COMMITMENT_RING_ELEMENT_SIZE}));
                   assert_norm (v ${ROWS_IN_A} == normalize_term (v ${ROWS_IN_A}))"#
            );
            encoding::commitment::serialize_vector::<SIMDUnit>(
                COMMITMENT_RING_ELEMENT_SIZE,
                &t1,
                &mut commitment_serialized,
            );

            let mut shake = Shake256Xof::init();
            shake.absorb(&message_representative);
            shake.absorb_final(&commitment_serialized);

            shake.squeeze(&mut recomputed_commitment_hash);
        }

        // Check if this is a valid signature by comparing the hashes.
        if deserialized_commitment_hash == recomputed_commitment_hash {
            return Ok(());
        }

        return Err(VerificationError::CommitmentHashesDontMatchError);
    }

    #[inline(always)]
    #[libcrux_macros::trusted(inline-admit)]
    pub(crate) fn sign_pre_hashed_mut<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128: shake128::Xof,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
        PH: PreHash,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        pre_hash_buffer: &mut [u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
        signature: &mut [u8; SIGNATURE_SIZE],
    ) -> Result<(), SigningError> {
        trusted_admit!(
            "pending-proof(campaign): body wraps admitted sign_internal; \
             discharge after sign frontier lands"
        );
        if context.len() > CONTEXT_MAX_LEN {
            return Err(SigningError::ContextTooLongError);
        }
        PH::hash::<Shake128>(message, pre_hash_buffer);
        let domain_separation_context = match DomainSeparationContext::new(context, Some(PH::oid()))
        {
            Ok(dsc) => dsc,
            Err(_) => return Err(SigningError::ContextTooLongError),
        };
        sign_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof, Shake256X4>(
            signing_key,
            pre_hash_buffer,
            Some(domain_separation_context),
            randomness,
            signature,
        )
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::ensures(|_| future(pre_hash_buffer).len() == pre_hash_buffer.len()))]
    #[libcrux_macros::trusted(inline-admit)]
    pub(crate) fn sign_pre_hashed<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128: shake128::Xof,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
        PH: PreHash,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        pre_hash_buffer: &mut [u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
    ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
        trusted_admit!(
            "pending-proof(campaign): body wraps admitted sign_pre_hashed_mut; \
             discharge after sign frontier lands"
        );
        let mut signature = MLDSASignature::zero();

        // [eurydice] doesn't support ?
        // https://github.com/AeneasVerif/eurydice/issues/105
        match sign_pre_hashed_mut::<
            SIMDUnit,
            Sampler,
            Shake128,
            Shake128X4,
            Shake256,
            Shake256Xof,
            Shake256X4,
            PH,
        >(
            signing_key,
            message,
            context,
            pre_hash_buffer,
            randomness,
            &mut signature.value,
        ) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    // Propagates sign_internal's signing-key length precondition; discharged by
    // the fixed-array instantiation wrappers.
    #[cfg_attr(hax, hax_lib::requires(fstar!(r#"Seq.length $signing_key == v ${SIGNING_KEY_SIZE}"#)))]
    pub(crate) fn sign_mut<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
        signature: &mut [u8; SIGNATURE_SIZE],
    ) -> Result<(), SigningError> {
        let domain_separation_context = match DomainSeparationContext::new(context, None) {
            Ok(dsc) => dsc,
            Err(_) => return Err(SigningError::ContextTooLongError),
        };
        sign_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof, Shake256X4>(
            signing_key,
            message,
            Some(domain_separation_context),
            randomness,
            signature,
        )
    }

    #[inline(always)]
    #[libcrux_macros::trusted(
        panic_free,
        "pending-proof(campaign): signing wrapper verified panic-free; result-ok ensures admitted"
    )]
    #[cfg_attr(hax, hax_lib::ensures(|result| {
        hax_lib::implies(
            context.len() <= 255
                && message.len() <= 8192
                && signing_key.len() >= SIGNING_KEY_SIZE,
            result.is_ok()
                == hacspec_ml_dsa::sign::<
                    { HACSPEC_PARAMS.k },
                    { HACSPEC_PARAMS.l },
                    SIGNATURE_SIZE,
                    COMMITMENT_VECTOR_SIZE,
                    { HACSPEC_PARAMS.lambda / 4 },
                >(
                    signing_key,
                    message,
                    context,
                    &randomness,
                    &HACSPEC_PARAMS,
                ).is_ok(),
        )
    }))]
    // Propagates sign_mut's signing-key length precondition; discharged by the
    // fixed-array instantiation wrappers.  The functional ensures above already
    // treats `signing_key.len() >= SIGNING_KEY_SIZE` as a hypothesis, so this
    // exact-length requires is strictly compatible.
    #[cfg_attr(hax, hax_lib::requires(fstar!(r#"Seq.length $signing_key == v ${SIGNING_KEY_SIZE}"#)))]
    pub(crate) fn sign<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
    ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
        let mut signature = MLDSASignature::zero();

        // [eurydice] doesn't support ?
        // https://github.com/AeneasVerif/eurydice/issues/105
        match sign_mut::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof, Shake256X4>(
            signing_key,
            message,
            context,
            randomness,
            &mut signature.value,
        ) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    #[libcrux_macros::trusted(
        panic_free,
        "pending-proof(campaign): signing wrapper verified panic-free; result-ok ensures admitted"
    )]
    #[cfg_attr(hax, hax_lib::ensures(|result| {
        hax_lib::implies(
            context.len() <= 255 && message.len() <= 8192,
            result.is_ok()
                == hacspec_ml_dsa::verify::<
                    { HACSPEC_PARAMS.k },
                    { HACSPEC_PARAMS.l },
                    { HACSPEC_PARAMS.lambda / 4 },
                    COMMITMENT_VECTOR_SIZE,
                >(
                    verification_key_serialized,
                    message,
                    signature_serialized,
                    context,
                    &HACSPEC_PARAMS,
                ).is_ok(),
        )
    }))]
    pub(crate) fn verify<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
    >(
        verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
        message: &[u8],
        context: &[u8],
        signature_serialized: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), VerificationError> {
        // We manually do the matching here to make Eurydice happy.
        let domain_separation_context = match DomainSeparationContext::new(context, None) {
            Ok(dsc) => dsc,
            Err(_) => return Err(VerificationError::VerificationContextTooLongError),
        };
        verify_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof>(
            verification_key_serialized,
            message,
            Some(domain_separation_context),
            signature_serialized,
        )
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::ensures(|_| future(pre_hash_buffer).len() == pre_hash_buffer.len()))]
    #[libcrux_macros::trusted(inline-admit)]
    pub(crate) fn verify_pre_hashed<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128: shake128::Xof,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        PH: PreHash,
    >(
        verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
        message: &[u8],
        context: &[u8],
        pre_hash_buffer: &mut [u8],
        signature_serialized: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), VerificationError> {
        trusted_admit!(
            "pending-proof(campaign): PH::hash + DomainSeparationContext body proof \
             pending; verify_internal itself verified"
        );
        PH::hash::<Shake128>(message, pre_hash_buffer);
        let domain_separation_context = match DomainSeparationContext::new(context, Some(PH::oid()))
        {
            Ok(dsc) => dsc,
            Err(_) => return Err(VerificationError::VerificationContextTooLongError),
        };
        verify_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof>(
            verification_key_serialized,
            pre_hash_buffer,
            Some(domain_separation_context),
            signature_serialized,
        )
    }
}

/// This corresponds to line 6 in algorithm 7 in FIPS 204 (line 7 in algorithm
/// 8, resp.).
///
/// If `domain_separation_context` is supplied, applies domain
/// separation and length encoding to the context string,
/// before appending the message (in the regular variant) or the
/// pre-hash OID as well as the pre-hashed message digest. Otherwise,
/// it is assumed that `message` already contains domain separation
/// information.
///
/// In FIPS 204 M' is the concatenation of the domain separated context, any
/// potential pre-hash OID and the message (or the message pre-hash). We do not
/// explicitely construct the concatenation in memory since it is of statically unknown
/// length, but feed its components directly into the incremental XOF.
///
/// Refer to line 10 of Algorithm 2 (and line 5 of Algorithm 3, resp.) in [FIPS
/// 204](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf#section.5)
/// for details on the domain separation for regular ML-DSA. Line
/// 23 of Algorithm 4 (and line 18 of Algorithm 5,resp.) describe domain separation for the HashMl-DSA
/// variant.
#[inline(always)]
#[libcrux_macros::trusted(inline-admit)]
fn derive_message_representative<Shake256Xof: shake256::Xof>(
    verification_key_hash: &[u8],
    domain_separation_context: &Option<DomainSeparationContext>,
    message: &[u8],
    message_representative: &mut [u8; 64],
) {
    trusted_admit!(
        "pending-proof(campaign): shake256 absorb/squeeze body (message representative) \
         proof pending"
    );
    #[cfg(not(eurydice))]
    debug_assert!(verification_key_hash.len() == 64);

    let mut shake = Shake256Xof::init();
    shake.absorb(verification_key_hash);
    if let Some(domain_separation_context) = domain_separation_context {
        shake.absorb(&[domain_separation_context.pre_hash_oid().is_some() as u8]);
        shake.absorb(&[domain_separation_context.context().len() as u8]);
        shake.absorb(domain_separation_context.context());
        if let Some(pre_hash_oid) = domain_separation_context.pre_hash_oid() {
            // FIPS 204 Alg 4 line 23 / Alg 5 line 18: absorb the OID
            // verbatim (no extra IntegerToBytes(|OID|, 1) prefix).  The
            // OID is already DER-encoded with tag+length (see the
            // `SHAKE128_OID` constant in pre_hash.rs), so it carries
            // its own length information.
            shake.absorb(pre_hash_oid)
        }
    }

    shake.absorb_final(message);
    shake.squeeze(message_representative);
}
