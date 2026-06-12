use core::array::from_fn;

#[cfg(hax)]
use hax_lib::prop::ToProp;

use crate::{
    constants::{
        ranked_bytes_per_ring_element, BYTES_PER_RING_ELEMENT, COEFFICIENTS_IN_RING_ELEMENT,
        SHARED_SECRET_SIZE,
    },
    hash_functions::Hash,
    helper::cloop,
    matrix::*,
    ntt::{ntt_binomially_sampled_ring_element, ntt_vector_u},
    polynomial::PolynomialRingElement,
    sampling::sample_from_binomial_distribution,
    serialize::{
        compress_then_serialize_message, compress_then_serialize_ring_element_u,
        compress_then_serialize_ring_element_v, deserialize_ring_elements_reduced,
        deserialize_then_decompress_message, deserialize_then_decompress_ring_element_u,
        deserialize_then_decompress_ring_element_v, deserialize_to_uncompressed_ring_element,
        serialize_uncompressed_ring_element,
    },
    utils::{into_padded_array, prf_input_inc},
    variant::Variant,
    vector::Operations,
};

#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::spec::{matrix_to_spec, poly_to_spec, vector_to_spec};

#[cfg(hax)]
use crate::polynomial::spec;

/// Types for the unpacked API.
#[allow(non_snake_case)]
pub(crate) mod unpacked {
    use crate::{polynomial::PolynomialRingElement, vector::traits::Operations};

    /// An unpacked ML-KEM IND-CPA Private Key
    pub(crate) struct IndCpaPrivateKeyUnpacked<const K: usize, Vector: Operations> {
        pub(crate) secret_as_ntt: [PolynomialRingElement<Vector>; K],
    }

    impl<const K: usize, Vector: Operations> Default for IndCpaPrivateKeyUnpacked<K, Vector> {
        fn default() -> Self {
            Self {
                secret_as_ntt: [PolynomialRingElement::<Vector>::ZERO(); K],
            }
        }
    }

    /// An unpacked ML-KEM IND-CPA Public Key
    #[derive(Clone)]
    pub(crate) struct IndCpaPublicKeyUnpacked<const K: usize, Vector: Operations> {
        pub(crate) t_as_ntt: [PolynomialRingElement<Vector>; K],
        pub(crate) seed_for_A: [u8; 32],
        pub(crate) A: [[PolynomialRingElement<Vector>; K]; K],
    }

    impl<const K: usize, Vector: Operations> Default for IndCpaPublicKeyUnpacked<K, Vector> {
        fn default() -> Self {
            Self {
                t_as_ntt: [PolynomialRingElement::<Vector>::ZERO(); K],
                seed_for_A: [0u8; 32],
                A: [[PolynomialRingElement::<Vector>::ZERO(); K]; K],
            }
        }
    }
}
use unpacked::*;

/// Concatenate `t` and `ρ` into the public key.
#[inline(always)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        && seed_for_a.len() == 32).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, t_as_ntt)
)]
#[hax_lib::ensures(|res|
    res == hacspec_ml_kem::serialize::serialize_public_key::<K, PUBLIC_KEY_SIZE>(
        &crate::vector::spec::vector_to_spec(t_as_ntt),
        seed_for_a,
    )
)]
pub(crate) fn serialize_public_key<
    const K: usize,
    const PUBLIC_KEY_SIZE: usize,
    Vector: Operations,
>(
    t_as_ntt: &[PolynomialRingElement<Vector>; K],
    seed_for_a: &[u8],
) -> [u8; PUBLIC_KEY_SIZE] {
    let mut public_key_serialized = [0u8; PUBLIC_KEY_SIZE];
    serialize_public_key_mut::<K, PUBLIC_KEY_SIZE, Vector>(
        t_as_ntt,
        seed_for_a,
        &mut public_key_serialized,
    );
    public_key_serialized
}

/// Concatenate `t` and `ρ` into the public key.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning")]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        && seed_for_a.len() == 32).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, t_as_ntt)
)]
#[hax_lib::ensures(|()| fstar!(r#"${serialized}_future ==
    Hacspec_ml_kem.Serialize.serialize_public_key $K $PUBLIC_KEY_SIZE
      (${vector_to_spec::<K, Vector>} $K $t_as_ntt)
      $seed_for_a"#))]
pub(crate) fn serialize_public_key_mut<
    const K: usize,
    const PUBLIC_KEY_SIZE: usize,
    Vector: Operations,
>(
    t_as_ntt: &[PolynomialRingElement<Vector>; K],
    seed_for_a: &[u8],
    serialized: &mut [u8; PUBLIC_KEY_SIZE],
) {
    serialize_vector::<K, Vector>(
        t_as_ntt,
        &mut serialized[0..ranked_bytes_per_ring_element(K)],
    );

    serialized[ranked_bytes_per_ring_element(K)..].copy_from_slice(seed_for_a);

    hax_lib::fstar!(
        r#"assert (Libcrux_ml_kem.Constants.ranked_bytes_per_ring_element $K ==
                    $K *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT);
           assert (v $PUBLIC_KEY_SIZE ==
                    v $K * v Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT + 32);
           Hacspec_ml_kem.Commute.Ind_cpa_serialize.lemma_serialize_public_key_mut_finalize
             $K $PUBLIC_KEY_SIZE $serialized
             (${vector_to_spec::<K, Vector>} $K $t_as_ntt)
             $seed_for_a"#
    );
}

/// Call [`serialize_uncompressed_ring_element`] for each ring element.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning")]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && out.len() == hacspec_ml_kem::parameters::ranked_bytes_per_ring_element(K)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, key)
)]
#[hax_lib::ensures(|()| fstar!(r#"${out}_future ==
    Hacspec_ml_kem.Serialize.serialize_secret_key $K ($K *! sz 384)
      (${vector_to_spec::<K, Vector>} $K $key)"#))]
pub(crate) fn serialize_vector<const K: usize, Vector: Operations>(
    key: &[PolynomialRingElement<Vector>; K],
    out: &mut [u8],
) {

    cloop! {
        for (i, re) in key.into_iter().enumerate() {
            hax_lib::loop_invariant!(|i: usize| {
                fstar!(r#"${out.len()} == Hacspec_ml_kem.Parameters.ranked_bytes_per_ring_element $K /\
                    (v $i < v $K ==>
                    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index $key (v $i))) /\
                    (forall (j: nat). j < v $i ==>
                    (j + 1) * v $BYTES_PER_RING_ELEMENT <= Seq.length $out /\
                    (Seq.slice $out (j * v $BYTES_PER_RING_ELEMENT) ((j + 1) * v $BYTES_PER_RING_ELEMENT) ==
                        Hacspec_ml_kem.Serialize.byte_encode (sz 384) (sz 3072) (${poly_to_spec::<Vector>} (Seq.index $key j)) (sz 12)))"#
                )
            });

            out[i * BYTES_PER_RING_ELEMENT..(i + 1) * BYTES_PER_RING_ELEMENT]
            .copy_from_slice(&serialize_uncompressed_ring_element(&re));

            hax_lib::fstar!(r#"
                let lemma_aux (j: nat{ j < v $i }) : Lemma
                (Seq.slice out (j * v $BYTES_PER_RING_ELEMENT) ((j + 1) * v $BYTES_PER_RING_ELEMENT) ==
                    Hacspec_ml_kem.Serialize.byte_encode (sz 384) (sz 3072) (${poly_to_spec::<Vector>} (Seq.index $key j)) (sz 12)) =
                eq_intro (Seq.slice out (j * v $BYTES_PER_RING_ELEMENT) ((j + 1) * v $BYTES_PER_RING_ELEMENT))
                (Hacspec_ml_kem.Serialize.byte_encode (sz 384) (sz 3072) (${poly_to_spec::<Vector>} (Seq.index $key j)) (sz 12))
                in
                Classical.forall_intro lemma_aux"#
            );
        }
    }

    hax_lib::fstar!(
        r#"assert (v $K > 0 /\ v $K <= 4);
           assert (Seq.length $out == v $K * v $BYTES_PER_RING_ELEMENT);
           Hacspec_ml_kem.Commute.Ind_cpa_serialize.lemma_serialize_vector_finalize
             $K #$:Vector $out $key"#
    );
}

/// Sample a vector of ring elements from a centered binomial distribution.
//
// NOTE (2026-06-11): `panic_free` — body (incl. the cascade-fixed scaffold: cbd_prefix_done opaque
// loop invariant + prefix init/step + finalize lemma) IS verified; only the functional return-post is
// admitted. That post is NOT yet proven — it fails as "incomplete quantifiers" (the inlined
// tuple-return projector-matching problem; see proofs/agent-status/ind_cpa-sample_ring_element_cbd-status.md).
// To resume: remove `verification_status(panic_free)` and close the return-post.
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options(
    "--z3rlimit 300 --ext context_pruning --split_queries always --z3refresh --using_facts_from '* -Hacspec_ml_kem.Parameters.createi_lemma -Libcrux_ml_kem.Polynomial.Spec'"
)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
        && ETA2 == hacspec_ml_kem::parameters::eta2(K)
        && (domain_separator as usize) < 2 * K
        && (domain_separator as usize) + K < 256).to_prop()
)]
#[hax_lib::ensures(|ds| fstar!(r#"
    b2t ($ds =. ($domain_separator +! (cast ($K <: usize) <: u8) <: u8)) /\
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector $K #$:Vector (mk_usize 3) error_1_future /\
    b2t ((Libcrux_ml_kem.Vector.Spec.vector_to_spec $K #$:Vector error_1_future
            <: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) $K) =.
         (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2
            (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
            $domain_separator
            <: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) $K))"#))]
fn sample_ring_element_cbd<
    const K: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    prf_input: &[u8; 33],
    domain_separator: u8,
    error_1: &mut [PolynomialRingElement<Vector>; K],
) -> u8 {
    let mut prf_inputs = [prf_input.clone(); K];

    // Unshadow domain_separator: bind the incremented result to a fresh name so the
    // parameter (= initial ds, used by the ensures' sample_vector_cbd) stays nameable post-loop.
    let domain_separator_future = prf_input_inc::<K>(&mut prf_inputs, domain_separator);
    hax_lib::fstar!(
        r#"assert (v $domain_separator_future == v $domain_separator + v $K);
           Hacspec_ml_kem.Commute.Ind_cpa_sampling.lemma_prf_inputs_struct
             $K $prf_input $prf_inputs $domain_separator"#
    );
    let prf_outputs: [[u8; ETA2_RANDOMNESS_SIZE]; K] = Hasher::PRFxN(&prf_inputs);
    // Establish the opaque loop-invariant atom at the empty prefix (vacuous).
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ind_cpa_sampling.lemma_cbd_prefix_init
             $K ${error_1}
             (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2
                (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8) $domain_separator)"#
    );
    for i in 0..K {
        // Invariant carries ONE opaque atom (cbd_prefix_done) wrapping the `forall j` — so hax does
        // NOT bake a raw forall into the fold-accumulator refinement type (that re-fires ~20k times
        // in the whole-function post VC → "incomplete quantifiers").  See Ind_cpa_sampling.fst.
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"Hacspec_ml_kem.Commute.Ind_cpa_sampling.cbd_prefix_done $K ${error_1}
                    (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2
                       (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8) $domain_separator) (v $i)"#
            )
        });
        #[cfg(hax)]
        let error_1_old = *error_1;
        let sampled = sample_from_binomial_distribution::<ETA2, Vector>(&prf_outputs[i]);
        error_1[i] = sampled;
        hax_lib::fstar!(
            r#"Hacspec_ml_kem.Commute.Ind_cpa_sampling.lemma_per_index_cbd
                 #$:Vector $K $ETA2 (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
                 $domain_separator $ETA2_RANDOMNESS_SIZE $prf_inputs $prf_outputs $sampled (v $i);
               Hacspec_ml_kem.Commute.Ind_cpa_sampling.lemma_cbd_row_intro
                 $K $sampled
                 (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2
                    (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8) $domain_separator) (v $i);
               Hacspec_ml_kem.Commute.Ind_cpa_sampling.lemma_cbd_prefix_step
                 #$:Vector $K
                 (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2
                    (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8) $domain_separator)
                 $error_1_old $i $sampled"#
        );
    }

    // Single full-post establisher: yields the exact ensures (all 3 conjuncts as one opaque atom)
    // from the loop-exit opaque prefix atom + the ds increment value-fact (asserted above).
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ind_cpa_sampling.lemma_sample_ring_element_cbd_post
             #$:Vector $K $ETA2 (${prf_input}.[ { Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
             $domain_separator $domain_separator_future $error_1"#
    );

    domain_separator_future
}

/// Sample a vector of ring elements from a centered binomial distribution and
/// convert them into their NTT representations.
//
// NOTE (2026-06-11): `panic_free` — body verified, functional post admitted. Same value+&mut
// tuple-return-post wall as sample_ring_element_cbd (see
// proofs/agent-status/ind_cpa-sample_ring_element_cbd-status.md), and additionally needs ntt.rs
// strengthened to a functional NTT post. Deferred to the tuple-return follow-up.
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options(
    "--max_fuel 25 --z3rlimit 400 --ext context_pruning --split_queries always"
)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && ETA_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
        && ETA == hacspec_ml_kem::parameters::eta1(K)
        && (domain_separator as usize) < 2 * K
        && (domain_separator as usize) + K < 256).to_prop()
)]
#[hax_lib::ensures(|ds|
    (ds == domain_separator + (K as u8)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, future(re_as_ntt))
    & (crate::vector::spec::vector_to_spec(future(re_as_ntt)) ==
        hacspec_ml_kem::ind_cpa::sample_vector_cbd_then_ntt::<K>(
            ETA, &prf_input[..32], domain_separator)).to_prop()
)]
fn sample_vector_cbd_then_ntt<
    const K: usize,
    const ETA: usize,
    const ETA_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    re_as_ntt: &mut [PolynomialRingElement<Vector>; K],
    prf_input: &[u8; 33],
    mut domain_separator: u8,
) -> u8 {
    let mut prf_inputs = [prf_input.clone(); K];

    #[cfg(hax)]
    let _domain_separator_init = domain_separator;

    domain_separator = prf_input_inc::<K>(&mut prf_inputs, domain_separator);
    let prf_outputs: [[u8; ETA_RANDOMNESS_SIZE]; K] = Hasher::PRFxN(&prf_inputs);
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"forall (j:nat). j < v $i ==>
            Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #$:Vector (sz 3328) re_as_ntt.[ sz j ]"#
            )
        });
        re_as_ntt[i] = sample_from_binomial_distribution::<ETA, Vector>(&prf_outputs[i]);
        ntt_binomially_sampled_ring_element(&mut re_as_ntt[i]);
    }
    domain_separator
}

/// This function implements most of <strong>Algorithm 12</strong> of the
/// NIST FIPS 203 specification; this is the Kyber CPA-PKE key generation algorithm.
///
/// We say "most of" since Algorithm 12 samples the required randomness within
/// the function itself, whereas this implementation expects it to be provided
/// through the `key_generation_seed` parameter.
///
/// Algorithm 12 is reproduced below:
///
/// ```plaintext
/// Output: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Output: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
///
/// d ←$ B
/// (ρ,σ) ← G(d)
/// N ← 0
/// for (i ← 0; i < k; i++)
///     for(j ← 0; j < k; j++)
///         Â[i,j] ← SampleNTT(XOF(ρ, i, j))
///     end for
/// end for
/// for(i ← 0; i < k; i++)
///     s[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
///     N ← N + 1
/// end for
/// for(i ← 0; i < k; i++)
///     e[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(σ,N))
///     N ← N + 1
/// end for
/// ŝ ← NTT(s)
/// ê ← NTT(e)
/// t̂ ← Â◦ŝ + ê
/// ekₚₖₑ ← ByteEncode₁₂(t̂) ‖ ρ
/// dkₚₖₑ ← ByteEncode₁₂(ŝ)
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[allow(non_snake_case)]
#[hax_lib::fstar::before(r#"[@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 500 --ext context_pruning --using_facts_from '* -Hacspec_ml_kem.Matrix -Hacspec_ml_kem.Ind_cpa.sample_vector_cbd_then_ntt'")]
// Use .to_prop() & to create logical (l_and) conjunction so F* can propagate
// is_rank(K) as a hypothesis when type-checking eta1_randomness_size(K)'s precondition.
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K).to_prop()
    & (ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
        && ETA1 == hacspec_ml_kem::parameters::eta1(K)
        && key_generation_seed.len() == hacspec_ml_kem::parameters::CPA_KEY_GENERATION_SEED_SIZE).to_prop()
)]
#[hax_lib::ensures(|()|
    crate::polynomial::spec::is_bounded_polynomial_vector(3328, &future(public_key).t_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &future(private_key).secret_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, &future(public_key).A)
    & (match hacspec_ml_kem::ind_cpa::generate_keypair_unpacked::<K>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        key_generation_seed,
    ) {
        Ok((secret_as_ntt, t_as_ntt, A_as_ntt, seed_for_A)) => {
            crate::vector::spec::vector_to_spec(&future(public_key).t_as_ntt) == t_as_ntt
            && future(public_key).seed_for_A == seed_for_A
            && crate::vector::spec::matrix_to_spec(&future(public_key).A)
                == hacspec_ml_kem::matrix::transpose(&A_as_ntt)
            && crate::vector::spec::vector_to_spec(&future(private_key).secret_as_ntt) == secret_as_ntt
        }
        Err(_) => true,
    }).to_prop()
)]
#[inline(always)]
pub(crate) fn generate_keypair_unpacked<
    const K: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
    Scheme: Variant,
>(
    key_generation_seed: &[u8],
    private_key: &mut IndCpaPrivateKeyUnpacked<K, Vector>,
    public_key: &mut IndCpaPublicKeyUnpacked<K, Vector>,
) {
    // (ρ,σ) := G(d) for Kyber, (ρ,σ) := G(d || K) for ML-KEM
    let hashed = Scheme::cpa_keygen_seed::<K, Hasher>(key_generation_seed);
    let (seed_for_A, seed_for_secret_and_error) = hashed.split_at(32);

    hax_lib::fstar!(
        "eq_intro $seed_for_A
        (Seq.slice (Libcrux_ml_kem.Utils.into_padded_array (sz 34) $seed_for_A) 0 32)"
    );
    sample_matrix_A::<K, Vector, Hasher>(&mut public_key.A, &into_padded_array(seed_for_A), true);

    let prf_input: [u8; 33] = into_padded_array(seed_for_secret_and_error);
    hax_lib::fstar!("eq_intro $seed_for_secret_and_error (Seq.slice $prf_input 0 32)");
    let domain_separator =
        sample_vector_cbd_then_ntt::<K, ETA1, ETA1_RANDOMNESS_SIZE, Vector, Hasher>(
            &mut private_key.secret_as_ntt,
            &prf_input,
            0,
        );

    let mut error_as_ntt = from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());
    let _ = sample_vector_cbd_then_ntt::<K, ETA1, ETA1_RANDOMNESS_SIZE, Vector, Hasher>(
        &mut error_as_ntt,
        &prf_input,
        domain_separator,
    );

    // tˆ := Aˆ ◦ sˆ + eˆ
    compute_As_plus_e(
        &mut public_key.t_as_ntt,
        &public_key.A,
        &private_key.secret_as_ntt,
        &error_as_ntt,
    );

    public_key.seed_for_A = seed_for_A.try_into().unwrap();

    // For encapsulation, we need to store A not Aˆ, and so we untranspose A
    // However, we pass A_transpose here and let the IND-CCA layer do the untranspose.
    // We could do it here, but then we would pay the performance cost (if any) for the packed API as well.

    // Discharge the functional ensures: the impl matrix A is the transpose of the
    // spec's (false) sample_matrix_A, and the spec re-transposes — so both the A
    // and tt conjuncts collapse onto lemma_sample_matrix_A_transpose; bounds folded
    // into the opaque is_bounded_polynomial_{vector,matrix} atoms.
    hax_lib::fstar!(
        r#"(* (1) hashed (impl) == hashed_spec: Variant post gives hashed == SU.v_G(seed++[K]);
       lemma_v_G_bridge: SU.v_G == HF.v_G; lemma_g_input_build: spec's g_input == seed++[K]. *)
    Hacspec_ml_kem.Commute.Ind_cca_bridge.lemma_v_G_bridge
      (Seq.append $key_generation_seed (Seq.create 1 (cast $K <: u8)));
    Hacspec_ml_kem.Commute.Keygen_bridge.lemma_g_input_build $K $key_generation_seed;
    (* (2) seed_for_A field: unwrap(try_into s) == copy_from_slice(repeat 0 32, s). *)
    Hacspec_ml_kem.Commute.Keygen_bridge.lemma_seed_for_A_eq $seed_for_A;
    (* (3) sample_matrix_A TRUE == transpose(sample_matrix_A FALSE) + is_Ok coupling. *)
    Hacspec_ml_kem.Commute.Keygen_bridge.lemma_sample_matrix_A_transpose $K $seed_for_A;
    (* (4) transpose involutive on the FALSE result (for the tt-conjunct). *)
    (match Hacspec_ml_kem.Matrix.sample_matrix_A $K $seed_for_A false with
      | Core_models.Result.Result_Ok aF ->
        Hacspec_ml_kem.Commute.Keygen_bridge.lemma_transpose_involutive $K aF
      | Core_models.Result.Result_Err _ -> ());
    (* (5) bounds: tt_as_ntt vector intro. *)
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro $K
      #$:Vector
      (${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt)
      (mk_usize 3328);
    (* (6) bounds: A matrix intro (per-row then matrix), as in build_unpacked_public_key_mut. *)
    (let aux (i: usize { v i < v $K })
          : Lemma
          (Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector $K
              #$:Vector
              (mk_usize 3328)
              (${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A.[ i ])) =
        Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro $K
          #$:Vector
          (${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A.[ i ])
          (mk_usize 3328)
      in
      FStar.Classical.forall_intro aux);
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_matrix_intro $K
      #$:Vector
      (${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A)
      (mk_usize 3328)"#
    );
}

#[allow(non_snake_case)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && PRIVATE_KEY_SIZE == hacspec_ml_kem::parameters::cpa_private_key_size(K)
    && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && key_generation_seed.len() == hacspec_ml_kem::parameters::CPA_KEY_GENERATION_SEED_SIZE
)]
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::ind_cpa::generate_keypair::<K, PUBLIC_KEY_SIZE, PRIVATE_KEY_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        key_generation_seed,
    ) {
        Ok((ek, dk)) => result.0 == dk && result.1 == ek,
        Err(_) => true,
    }
)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[inline(always)]
pub(crate) fn generate_keypair<
    const K: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
    Scheme: Variant,
>(
    key_generation_seed: &[u8],
) -> ([u8; PRIVATE_KEY_SIZE], [u8; PUBLIC_KEY_SIZE]) {
    let mut private_key = IndCpaPrivateKeyUnpacked::default();
    let mut public_key = IndCpaPublicKeyUnpacked::default();

    generate_keypair_unpacked::<K, ETA1, ETA1_RANDOMNESS_SIZE, Vector, Hasher, Scheme>(
        key_generation_seed,
        &mut private_key,
        &mut public_key,
    );

    let result = serialize_unpacked_secret_key::<K, PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE, Vector>(
        &public_key,
        &private_key,
    );

    // The spec's `ek` is built from `serialize_secret_key(tt_as_ntt) || seed_for_A`;
    // the impl serializes via `serialize_public_key`.  Bridge the two, and connect the
    // unpacked outputs (vector_to_spec t̂/ŝ, seed_for_A) to the spec's unpacked result.
    hax_lib::fstar!(
        r#"
    (match
        Hacspec_ml_kem.Ind_cpa.generate_keypair_unpacked $K
          (Hacspec_ml_kem.Parameters.rank_to_params $K)
          $key_generation_seed
      with
      | Core_models.Result.Result_Ok (secret_as_ntt, tt_as_ntt, e_A, seed_for_A) ->
        assert (Seq.length seed_for_A == 32);
        assert (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K #$:Vector
                  ${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt == tt_as_ntt);
        assert (${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A == seed_for_A);
        assert (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K #$:Vector
                  ${private_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_secret_as_ntt == secret_as_ntt);
        Hacspec_ml_kem.Commute.Serialize.lemma_ek_eq_serialize_public_key $K tt_as_ntt seed_for_A
      | Core_models.Result.Result_Err _ -> ())
"#
    );

    result
}

/// Serialize the secret key from the unpacked key pair generation.
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        && PRIVATE_KEY_SIZE == hacspec_ml_kem::parameters::ranked_bytes_per_ring_element(K)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &public_key.t_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &private_key.secret_as_ntt)
)]
#[hax_lib::ensures(|result| fstar!(r#"
    ${result}._1 ==
      Hacspec_ml_kem.Serialize.serialize_secret_key $K ($K *! sz 384)
        (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K #$:Vector
          ${private_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_secret_as_ntt) /\
    ${result}._2 ==
      Hacspec_ml_kem.Serialize.serialize_public_key $K $PUBLIC_KEY_SIZE
        (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K #$:Vector
          ${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt)
        (${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A <: t_Slice u8)
"#))]
pub(crate) fn serialize_unpacked_secret_key<
    const K: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    Vector: Operations,
>(
    public_key: &IndCpaPublicKeyUnpacked<K, Vector>,
    private_key: &IndCpaPrivateKeyUnpacked<K, Vector>,
) -> ([u8; PRIVATE_KEY_SIZE], [u8; PUBLIC_KEY_SIZE]) {
    // pk := (Encode_12(tˆ mod^{+}q) || ρ)
    let public_key_serialized = serialize_public_key::<K, PUBLIC_KEY_SIZE, Vector>(
        &public_key.t_as_ntt,
        &public_key.seed_for_A,
    );

    // sk := Encode_12(sˆ mod^{+}q)
    let mut secret_key_serialized = [0u8; PRIVATE_KEY_SIZE];
    serialize_vector(&private_key.secret_as_ntt, &mut secret_key_serialized);

    (secret_key_serialized, public_key_serialized)
}

/// Call [`compress_then_serialize_ring_element_u`] on each ring element.
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning")]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && OUT_LEN == hacspec_ml_kem::parameters::c1_size(K)
        && COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
        && BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
        && out.len() == OUT_LEN).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &input)
)]
#[hax_lib::ensures(|()| fstar!(r#"(${out}_future <: t_Slice u8) ==
    (Hacspec_ml_kem.Serialize.compress_then_serialize_u $K $OUT_LEN
       (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K #$:Vector $input)
       $COMPRESSION_FACTOR
     <: t_Slice u8)"#))]
#[inline(always)]
fn compress_then_serialize_u<
    const K: usize,
    const OUT_LEN: usize,
    const COMPRESSION_FACTOR: usize,
    const BLOCK_LEN: usize,
    Vector: Operations,
>(
    input: [PolynomialRingElement<Vector>; K],
    out: &mut [u8],
) {
    hax_lib::fstar!(
        "assert (v (sz 32 *! $COMPRESSION_FACTOR) == 32 * v $COMPRESSION_FACTOR);
        assert (v ($OUT_LEN /! $K) == v $OUT_LEN / v $K);
        assert (v $OUT_LEN / v $K == 32 * v $COMPRESSION_FACTOR)"
    );
    cloop! {
        for (i, re) in input.into_iter().enumerate() {
            hax_lib::loop_invariant!(|i: usize| { fstar!(r#"(v $i < v $K ==> Seq.length out == v $OUT_LEN /\
                Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index $input (v $i))) /\
            (forall (j: nat). j < v $i ==>
                Seq.length out == v $OUT_LEN /\
                (j + 1) * (v $OUT_LEN / v $K) <= Seq.length out /\
                (Seq.slice out (j * (v $OUT_LEN / v $K)) (((j + 1)) * (v $OUT_LEN / v $K)) == 
                    Hacspec_ml_kem.Serialize.byte_encode (sz 32 *! $COMPRESSION_FACTOR) (sz 256 *! $COMPRESSION_FACTOR)
                        (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} (Seq.index $input j)) $COMPRESSION_FACTOR)
                        $COMPRESSION_FACTOR))"#) });
            hax_lib::fstar!(r#"assert (forall (j: nat). j < v $i ==>
                ((Seq.slice out (j * (v $OUT_LEN / v $K)) (((j + 1)) * (v $OUT_LEN / v $K)) ==
                Hacspec_ml_kem.Serialize.byte_encode (sz 32 *! $COMPRESSION_FACTOR) (sz 256 *! $COMPRESSION_FACTOR)
                    (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} (Seq.index $input j)) $COMPRESSION_FACTOR)
                    $COMPRESSION_FACTOR)))"#);
            out[i * (OUT_LEN / K)..(i + 1) * (OUT_LEN / K)].copy_from_slice(
                &compress_then_serialize_ring_element_u::<COMPRESSION_FACTOR, BLOCK_LEN, Vector>(&re),
            );
            hax_lib::fstar!(r#"let lemma_aux (j: nat{ j < v $i }) : Lemma
                (Seq.slice out (j * (v $OUT_LEN / v $K)) (((j + 1)) * (v $OUT_LEN / v $K)) ==
                Hacspec_ml_kem.Serialize.byte_encode (sz 32 *! $COMPRESSION_FACTOR) (sz 256 *! $COMPRESSION_FACTOR)
                    (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} (Seq.index $input j)) $COMPRESSION_FACTOR)
                    $COMPRESSION_FACTOR) =
                eq_intro
                (Seq.slice out (j * (v $OUT_LEN / v $K)) (((j + 1)) * (v $OUT_LEN / v $K)))
                (Hacspec_ml_kem.Serialize.byte_encode (sz 32 *! $COMPRESSION_FACTOR) (sz 256 *! $COMPRESSION_FACTOR)
                    (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} (Seq.index $input j)) $COMPRESSION_FACTOR)
                    $COMPRESSION_FACTOR)
            in
            Classical.forall_intro lemma_aux"#);
        }
    };
}

/// This function implements <strong>Algorithm 13</strong> of the
/// NIST FIPS 203 specification; this is the Kyber CPA-PKE encryption algorithm.
///
/// Algorithm 13 is reproduced below:
///
/// ```plaintext
/// Input: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Input: message m ∈ 𝔹^{32}.
/// Input: encryption randomness r ∈ 𝔹^{32}.
/// Output: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
///
/// N ← 0
/// t̂ ← ByteDecode₁₂(ekₚₖₑ[0:384k])
/// ρ ← ekₚₖₑ[384k: 384k + 32]
/// for (i ← 0; i < k; i++)
///     for(j ← 0; j < k; j++)
///         Â[i,j] ← SampleNTT(XOF(ρ, i, j))
///     end for
/// end for
/// for(i ← 0; i < k; i++)
///     r[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(r,N))
///     N ← N + 1
/// end for
/// for(i ← 0; i < k; i++)
///     e₁[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
///     N ← N + 1
/// end for
/// e₂ ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
/// r̂ ← NTT(r)
/// u ← NTT-¹(Âᵀ ◦ r̂) + e₁
/// μ ← Decompress₁(ByteDecode₁(m)))
/// v ← NTT-¹(t̂ᵀ ◦ rˆ) + e₂ + μ
/// c₁ ← ByteEncode_{dᵤ}(Compress_{dᵤ}(u))
/// c₂ ← ByteEncode_{dᵥ}(Compress_{dᵥ}(v))
/// return c ← (c₁ ‖ c₂)
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[allow(non_snake_case)]
// FUNCTIONAL POST, proven compositionally via `Encrypt_bridge.lemma_encrypt_unpacked_finalize`
// (CONCAT form): it takes the two byte segments DIRECTLY (`tmp0` = encrypt_c1's c1 write, and
// the c2 segment as `Seq.slice ciphertext C1_LEN CIPHERTEXT_SIZE` — a single-`update_at` slice)
// plus `ciphertext` in its raw two-`update_at`-write form, and discharges `ciphertext == Ok-value`
// via `lemma_impl_concat` + `lemma_spec_concat`.  The earlier slice-form finalize (which required
// `Seq.slice ciphertext 0 C1_LEN == compress_u`, i.e. slicing the c1 segment THROUGH the second
// update) saturated the full no-admit build; passing `tmp0` directly moves that reasoning into the
// bridge (where the segments are explicit), so encrypt_unpacked now verifies at rlimit ~132.
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K).to_prop()
    & (ETA1 == hacspec_ml_kem::parameters::eta1(K)
        && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
        && ETA2 == hacspec_ml_kem::parameters::eta2(K)
        && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
        && C1_LEN == hacspec_ml_kem::parameters::c1_size(K)
        && C2_LEN == hacspec_ml_kem::parameters::c2_size(K)
        && U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
        && V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
        && BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
        && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
        && randomness.len() == hacspec_ml_kem::parameters::SHARED_SECRET_SIZE).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, &public_key.A)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &public_key.t_as_ntt)
)]
// NOTE: the impl computes u via `compute_vector_u(matrix_to_spec public_key.A)` (row-wise),
// so the matrix arg to the spec is `matrix_to_spec(public_key.A)` WITHOUT a transpose — the
// stored `A` already plays the role the spec's `compute_vector_u` consumes directly (cf.
// `build_unpacked_public_key`: `matrix_to_spec(A) == sample_matrix_A(..,false)`, which is
// exactly what `Hacspec.encrypt` feeds `encrypt_unpacked`).
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::ind_cpa::encrypt_unpacked::<K, C1_LEN, C2_LEN, CIPHERTEXT_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        &crate::vector::spec::vector_to_spec(&public_key.t_as_ntt),
        &crate::vector::spec::matrix_to_spec(&public_key.A),
        message,
        randomness,
    ) {
        Ok(ct) => result == ct,
        Err(_) => true,
    }
)]
#[inline(always)]
pub(crate) fn encrypt_unpacked<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_LEN: usize,
    const C2_LEN: usize,
    const U_COMPRESSION_FACTOR: usize,
    const V_COMPRESSION_FACTOR: usize,
    const BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    public_key: &IndCpaPublicKeyUnpacked<K, Vector>,
    message: &[u8; SHARED_SECRET_SIZE],
    randomness: &[u8],
) -> [u8; CIPHERTEXT_SIZE] {
    hax_lib::fstar!(
        r#"assert_norm (v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 2)) +
                       v (Hacspec_ml_kem.Parameters.c2_size (mk_usize 2)) ==
                       v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size (mk_usize 2)));
           assert_norm (v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 3)) +
                       v (Hacspec_ml_kem.Parameters.c2_size (mk_usize 3)) ==
                       v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size (mk_usize 3)));
           assert_norm (v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 4)) +
                       v (Hacspec_ml_kem.Parameters.c2_size (mk_usize 4)) ==
                       v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size (mk_usize 4)))"#
    );
    let mut ciphertext = [0u8; CIPHERTEXT_SIZE];

    let (r_as_ntt, error_2) = encrypt_c1::<
        K,
        C1_LEN,
        U_COMPRESSION_FACTOR,
        BLOCK_LEN,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        Vector,
        Hasher,
    >(randomness, &public_key.A, &mut ciphertext[0..C1_LEN]);

    encrypt_c2::<K, V_COMPRESSION_FACTOR, C2_LEN, Vector>(
        &public_key.t_as_ntt,
        &r_as_ntt,
        &error_2,
        message,
        &mut ciphertext[C1_LEN..],
    );

    // Compose encrypt_c1's c1 segment (`tmp0`, the &mut write) + encrypt_c2's c2 segment
    // (the tail slice of the final `ciphertext`) into `c == c1 ‖ c2 == Hacspec.encrypt_unpacked(...)`.
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Encrypt_bridge.lemma_encrypt_unpacked_finalize
             $K $C1_LEN $C2_LEN $CIPHERTEXT_SIZE $U_COMPRESSION_FACTOR $V_COMPRESSION_FACTOR
             (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K
                ${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt)
             (Libcrux_ml_kem.Vector.Spec.matrix_to_spec $K
                ${public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A)
             $message $randomness
             tmp0
             (Seq.slice ${ciphertext} (v $C1_LEN) (v $CIPHERTEXT_SIZE))
             ${ciphertext}"#
    );

    ciphertext
}

// Trust-base bridge: the impl-side abstract PRF (`Spec.Utils.v_PRF`, proven by
// every backend) equals the spec-side abstract PRF (`Hacspec.Parameters.Hash_functions.v_PRF`)
// via the (already-wired) `Commute.Prf_bridge.lemma_prf_identification`.  Lifts the
// proven `sample_from_binomial_distribution` post (sample_poly_cbd ∘ Spec.Utils.v_PRF)
// to the spec `sample_secret`.  Proven once in clean context; called by encrypt_c1.
#[cfg_attr(hax, hax_lib::fstar::before(r#"
#push-options "--fuel 1 --ifuel 1 --z3rlimit 40"

let lemma_ds_cast (v_K: usize)
    : Lemma (requires b2t (Hacspec_ml_kem.Parameters.is_rank v_K))
            (ensures ((cast v_K <: u8) +! (cast v_K <: u8)) == (cast (v_K *! mk_usize 2) <: u8))
  = ()

let lemma_error_2_sample_secret
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (v_K v_ETA2 v_ETA2_RANDOMNESS_SIZE: usize)
      (prf_input: t_Array u8 (mk_usize 33))
      (prf_output: t_Array u8 v_ETA2_RANDOMNESS_SIZE)
      (error_2: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Lemma
      (requires
        b2t (Hacspec_ml_kem.Parameters.is_rank v_K) /\
        v_ETA2 == Hacspec_ml_kem.Parameters.eta2 v_K /\
        v_ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size v_K /\
        prf_output == Spec.Utils.v_PRF v_ETA2_RANDOMNESS_SIZE prf_input /\
        Libcrux_ml_kem.Vector.Spec.poly_to_spec error_2 ==
          Hacspec_ml_kem.Sampling.sample_poly_cbd (v_ETA2 *! mk_usize 64) (v_ETA2 *! mk_usize 512)
            v_ETA2 prf_output)
      (ensures
        Libcrux_ml_kem.Vector.Spec.poly_to_spec error_2 ==
          Hacspec_ml_kem.Ind_cpa.sample_secret v_ETA2 prf_input)
  = assert (v_ETA2 == mk_usize 2);
    assert (v_ETA2_RANDOMNESS_SIZE == mk_usize 128);
    assert (v_ETA2_RANDOMNESS_SIZE == v_ETA2 *! mk_usize 64);
    Hacspec_ml_kem.Commute.Prf_bridge.lemma_prf_identification (v_ETA2 *! mk_usize 64) prf_input

#pop-options
"#))]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K).to_prop()
    & (ETA1 == hacspec_ml_kem::parameters::eta1(K)
        && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
        && ETA2 == hacspec_ml_kem::parameters::eta2(K)
        && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
        && BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
        && C1_LEN == hacspec_ml_kem::parameters::c1_size(K)
        && U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
        && randomness.len() == hacspec_ml_kem::parameters::SHARED_SECRET_SIZE
        && ciphertext.len() == hacspec_ml_kem::parameters::c1_size(K)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, matrix)
)]
#[hax_lib::ensures(|(r_as_ntt_out, error_2_out)|
    (future(ciphertext).len() == ciphertext.len()).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &r_as_ntt_out)
    & crate::polynomial::spec::is_bounded_poly(3328, &error_2_out)
    & fstar!(r#"Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $r_as_ntt_out ==
          Hacspec_ml_kem.Ind_cpa.sample_vector_cbd_then_ntt $K $ETA1 $randomness (mk_u8 0)"#)
    & fstar!(r#"Libcrux_ml_kem.Vector.Spec.poly_to_spec $error_2_out ==
          Hacspec_ml_kem.Ind_cpa.sample_secret $ETA2
            (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize
               (Libcrux_ml_kem.Utils.into_padded_array (mk_usize 33) $randomness)
               (mk_usize 32) (cast ($K *! mk_usize 2) <: u8))"#)
    & fstar!(r#"(${ciphertext}_future <: t_Slice u8) ==
          (Hacspec_ml_kem.Serialize.compress_then_serialize_u $K $C1_LEN
             (Hacspec_ml_kem.Matrix.compute_vector_u $K
                (Libcrux_ml_kem.Vector.Spec.matrix_to_spec $K $matrix)
                (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd_then_ntt $K $ETA1 $randomness (mk_u8 0))
                (Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2 $randomness (cast $K <: u8)))
             $U_COMPRESSION_FACTOR
           <: t_Slice u8)"#)
)]
#[inline(always)]
pub(crate) fn encrypt_c1<
    const K: usize,
    const C1_LEN: usize,
    const U_COMPRESSION_FACTOR: usize,
    const BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    randomness: &[u8],
    matrix: &[[PolynomialRingElement<Vector>; K]; K],
    ciphertext: &mut [u8], // C1_LEN
) -> (
    [PolynomialRingElement<Vector>; K],
    PolynomialRingElement<Vector>,
) {
    // for i from 0 to k−1 do
    //     r[i] := CBD{η1}(PRF(r, N))
    //     N := N + 1
    // end for
    // rˆ := NTT(r)
    let mut prf_input: [u8; 33] = into_padded_array(randomness);

    let mut r_as_ntt = from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());
    let domain_separator =
        sample_vector_cbd_then_ntt::<K, ETA1, ETA1_RANDOMNESS_SIZE, Vector, Hasher>(
            &mut r_as_ntt,
            &prf_input,
            0,
        );
    hax_lib::fstar!(
        "eq_intro $randomness (Seq.slice $prf_input 0 32);
        assert (v $domain_separator == v $K)"
    );

    // for i from 0 to k−1 do
    //     e1[i] := CBD_{η2}(PRF(r,N))
    //     N := N + 1
    // end for
    let mut error_1 = from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());
    let domain_separator = sample_ring_element_cbd::<K, ETA2_RANDOMNESS_SIZE, ETA2, Vector, Hasher>(
        &prf_input,
        domain_separator,
        &mut error_1,
    );

    // e_2 := CBD{η2}(PRF(r, N))
    prf_input[32] = domain_separator;
    hax_lib::fstar!(
        "assert (Seq.equal $prf_input (Seq.append $randomness (Seq.create 1 $domain_separator)));
        assert ($prf_input == Seq.append $randomness (Seq.create 1 $domain_separator))"
    );
    let prf_output: [u8; ETA2_RANDOMNESS_SIZE] = Hasher::PRF(&prf_input);
    let error_2 = sample_from_binomial_distribution::<ETA2, Vector>(&prf_output);

    // u := NTT^{-1}(AˆT ◦ rˆ) + e_1
    // Widen error_1's per-element bound from 3 (sampler post) to 7
    // (compute_vector_u pre).  The matrix bound on `matrix` chains through
    // the SMTPat'd `lemma_is_bounded_polynomial_{matrix,vector}_elim`
    // automatically, so only the eta1→7 widening on error_1 is explicit.
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_higher
            $K #$:Vector $error_1 (sz 3) (sz 7)"#
    );
    let u = compute_vector_u(matrix, &r_as_ntt, &error_1);

    // Fold compute_vector_u's per-element forall ensures into the opaque
    // `is_bounded_polynomial_vector 3328 u` atom required by
    // compress_then_serialize_u.
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro
            $K #$:Vector $u (sz 3328)"#
    );

    // c_1 := Encode_{du}(Compress_q(u,d_u))
    compress_then_serialize_u::<K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, Vector>(u, ciphertext);

    // Widen error_2's per-element bound from 3 (sampler post) to 3328 (the
    // returned-value ensures, matching encrypt_c2's t_as_ntt/r_as_ntt bound).
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly_higher #$:Vector $error_2 (sz 3) (sz 3328)"#
    );

    // Functional post: lift the sampler/compute_vector_u/compress_then_serialize_u
    // callee posts into the spec form expected by encrypt_unpacked.
    hax_lib::fstar!(
        r#"assert (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $r_as_ntt ==
              Hacspec_ml_kem.Ind_cpa.sample_vector_cbd_then_ntt $K $ETA1 $randomness (mk_u8 0));
           assert (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $error_1 ==
              Hacspec_ml_kem.Ind_cpa.sample_vector_cbd $K $ETA2 $randomness (cast $K <: u8));
           assert ($prf_output == Spec.Utils.v_PRF $ETA2_RANDOMNESS_SIZE $prf_input);
           lemma_ds_cast $K;
           lemma_error_2_sample_secret #$:Vector $K $ETA2 $ETA2_RANDOMNESS_SIZE $prf_input $prf_output
             $error_2"#
    );

    (r_as_ntt, error_2)
}

#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K).to_prop()
    & (V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
        && C2_LEN == hacspec_ml_kem::parameters::c2_size(K)
        && ciphertext.len() == hacspec_ml_kem::parameters::c2_size(K)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, t_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, r_as_ntt)
    & crate::polynomial::spec::is_bounded_poly(3328, error_2)
)]
#[hax_lib::ensures(|()| (future(ciphertext).len() == ciphertext.len()).to_prop()
    & fstar!(r#"${ciphertext}_future ==
        Hacspec_ml_kem.Serialize.compress_then_serialize_v $C2_LEN
          (Hacspec_ml_kem.Matrix.compute_ring_element_v $K
              (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $t_as_ntt)
              (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $r_as_ntt)
              (Libcrux_ml_kem.Vector.Spec.poly_to_spec $error_2)
              (Hacspec_ml_kem.Serialize.deserialize_then_decompress_message $message))
          $V_COMPRESSION_FACTOR"#))]
#[inline(always)]
pub(crate) fn encrypt_c2<
    const K: usize,
    const V_COMPRESSION_FACTOR: usize,
    const C2_LEN: usize,
    Vector: Operations,
>(
    t_as_ntt: &[PolynomialRingElement<Vector>; K],
    r_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_2: &PolynomialRingElement<Vector>,
    message: &[u8; SHARED_SECRET_SIZE],
    ciphertext: &mut [u8],
) {
    // v := NTT^{−1}(tˆT ◦ rˆ) + e_2 + Decompress_q(Decode_1(m),1)
    let message_as_ring_element = deserialize_then_decompress_message(message);
    let v = compute_ring_element_v(t_as_ntt, r_as_ntt, error_2, &message_as_ring_element);
    hax_lib::fstar!("assert ($C2_LEN = Hacspec_ml_kem.Parameters.c2_size v_K)");

    // c_2 := Encode_{dv}(Compress_q(v,d_v))
    compress_then_serialize_ring_element_v::<K, V_COMPRESSION_FACTOR, C2_LEN, Vector>(
        v, ciphertext,
    );
    // Functional post: compose compress_then_serialize_ring_element_v's post
    // (out == compress_then_serialize_v(poly_to_spec v)) with compute_ring_element_v's
    // (poly_to_spec v == Hacspec.compute_ring_element_v(...)) — all proven callees.
    hax_lib::fstar!(
        r#"assert (${ciphertext} ==
        Hacspec_ml_kem.Serialize.compress_then_serialize_v $C2_LEN
          (Hacspec_ml_kem.Matrix.compute_ring_element_v $K
              (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $t_as_ntt)
              (Libcrux_ml_kem.Vector.Spec.vector_to_spec $K $r_as_ntt)
              (Libcrux_ml_kem.Vector.Spec.poly_to_spec $error_2)
              (Hacspec_ml_kem.Serialize.deserialize_then_decompress_message $message))
          $V_COMPRESSION_FACTOR)"#
    );
}

#[allow(non_snake_case)]
// FORWARDER (functional): composes `build_unpacked_public_key`'s posts
// (`vector_to_spec t̂ == vector_decode_12_(pk[..T_ENC])`, `matrix_to_spec A == sample_matrix_A(pk[T_ENC..],false)`)
// with `encrypt_unpacked`'s functional post.  `Encrypt_bridge.lemma_rank_to_params` supplies the
// param-field facts so the spec's `impl_MlKemParams__tt_as_ntt_encoded_size(rank_to_params K)` and
// `deserialize_ring_elements_reduced (== vector_decode_12_)` line up; the rest is delta-reduction.
#[hax_lib::fstar::options("--z3rlimit 500 --ext context_pruning")]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K).to_prop()
    & (ETA1 == hacspec_ml_kem::parameters::eta1(K)
        && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
        && ETA2 == hacspec_ml_kem::parameters::eta2(K)
        && BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
        && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
        && U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
        && V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
        && public_key.len() == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        && randomness.len() == hacspec_ml_kem::parameters::SHARED_SECRET_SIZE
        && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
        && T_AS_NTT_ENCODED_SIZE == hacspec_ml_kem::parameters::t_as_ntt_encoded_size(K)
        && C1_LEN == hacspec_ml_kem::parameters::c1_size(K)
        && C2_LEN == hacspec_ml_kem::parameters::c2_size(K)).to_prop()
)]
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::ind_cpa::encrypt::<K, C1_LEN, C2_LEN, CIPHERTEXT_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        public_key,
        message,
        randomness,
    ) {
        Ok(expected) => result == expected,
        Err(_) => true,
    }
)]
#[inline(always)]
pub(crate) fn encrypt<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_LEN: usize,
    const C2_LEN: usize,
    const U_COMPRESSION_FACTOR: usize,
    const V_COMPRESSION_FACTOR: usize,
    const BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    public_key: &[u8],
    message: &[u8; SHARED_SECRET_SIZE],
    randomness: &[u8],
) -> [u8; CIPHERTEXT_SIZE] {
    let unpacked_public_key =
        build_unpacked_public_key::<K, T_AS_NTT_ENCODED_SIZE, Vector, Hasher>(public_key);

    // After unpacking the public key we can now call the unpacked encryption.
    let result = encrypt_unpacked::<
        K,
        CIPHERTEXT_SIZE,
        T_AS_NTT_ENCODED_SIZE,
        C1_LEN,
        C2_LEN,
        U_COMPRESSION_FACTOR,
        V_COMPRESSION_FACTOR,
        BLOCK_LEN,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        Vector,
        Hasher,
    >(&unpacked_public_key, message, randomness);

    // The spec `Hacspec.encrypt` decodes t̂ (== build_unpacked) then matches `sample_matrix_A`
    // (== build_unpacked) and forwards to `encrypt_unpacked`; the param-field facts make the two
    // line up (deserialize_ring_elements_reduced == vector_decode_12_ is definitional).
    hax_lib::fstar!("Hacspec_ml_kem.Commute.Encrypt_bridge.lemma_rank_to_params $K");

    result
}

#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && T_AS_NTT_ENCODED_SIZE == hacspec_ml_kem::parameters::t_as_ntt_encoded_size(K)
    && public_key.len() == hacspec_ml_kem::parameters::cpa_public_key_size(K)
)]
#[hax_lib::ensures(|result| {
    crate::vector::spec::vector_to_spec(&result.t_as_ntt)
        == hacspec_ml_kem::serialize::vector_decode_12::<K>(
            &public_key[..T_AS_NTT_ENCODED_SIZE],
        )
    && match hacspec_ml_kem::matrix::sample_matrix_A::<K>(
        &public_key[T_AS_NTT_ENCODED_SIZE..],
        false,
    ) {
        Ok(A_as_ntt) => crate::vector::spec::matrix_to_spec(&result.A) == A_as_ntt,
        Err(_) => true,
    }
})]
fn build_unpacked_public_key<
    const K: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    public_key: &[u8],
) -> IndCpaPublicKeyUnpacked<K, Vector> {
    let mut unpacked_public_key = IndCpaPublicKeyUnpacked::<K, Vector>::default();
    build_unpacked_public_key_mut::<K, T_AS_NTT_ENCODED_SIZE, Vector, Hasher>(
        public_key,
        &mut unpacked_public_key,
    );
    unpacked_public_key
}

#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && T_AS_NTT_ENCODED_SIZE == hacspec_ml_kem::parameters::t_as_ntt_encoded_size(K)
    && public_key.len() == hacspec_ml_kem::parameters::cpa_public_key_size(K)
)]
#[hax_lib::ensures(|()| {
    crate::polynomial::spec::is_bounded_polynomial_vector(3328, &future(unpacked_public_key).t_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, &future(unpacked_public_key).A)
    & (crate::vector::spec::vector_to_spec(&future(unpacked_public_key).t_as_ntt)
        == hacspec_ml_kem::serialize::vector_decode_12::<K>(
            &public_key[..T_AS_NTT_ENCODED_SIZE],
        )
    && match hacspec_ml_kem::matrix::sample_matrix_A::<K>(
        &public_key[T_AS_NTT_ENCODED_SIZE..],
        false,
    ) {
        Ok(A_as_ntt) => crate::vector::spec::matrix_to_spec(&future(unpacked_public_key).A) == A_as_ntt,
        Err(_) => true,
    }).to_prop()
})]
pub(crate) fn build_unpacked_public_key_mut<
    const K: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    public_key: &[u8],
    unpacked_public_key: &mut IndCpaPublicKeyUnpacked<K, Vector>,
) {
    // tˆ := Decode_12(pk)
    deserialize_ring_elements_reduced::<K, Vector>(
        &public_key[..T_AS_NTT_ENCODED_SIZE],
        &mut unpacked_public_key.t_as_ntt,
    );

    // ρ := pk + 12·k·n / 8
    // for i from 0 to k−1 do
    //     for j from 0 to k − 1 do
    //         AˆT[i][j] := Parse(XOF(ρ, i, j))
    //     end for
    // end for
    let seed = &public_key[T_AS_NTT_ENCODED_SIZE..];
    hax_lib::fstar!(
        "eq_intro $seed
      (Seq.slice (Libcrux_ml_kem.Utils.into_padded_array (sz 34) $seed) 0 32)"
    );
    sample_matrix_A::<K, Vector, Hasher>(
        &mut unpacked_public_key.A,
        &into_padded_array(seed),
        false,
    );

    // Fold the callee posts' per-element bounds into the opaque
    // is_bounded_polynomial_{vector,matrix} atoms required by the ensures.
    // (The functional vector_to_spec / matrix_to_spec equalities flow straight
    // from the callee posts; the seed-slice eq_intro above bridges the A spec.)
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro $K #$:Vector
             (${unpacked_public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt) (mk_usize 3328);
           (let aux (i: usize { v i < v $K })
               : Lemma
                 (Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector $K #$:Vector
                    (mk_usize 3328)
                    (${unpacked_public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A.[ i ])) =
             Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro $K #$:Vector
               (${unpacked_public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A.[ i ]) (mk_usize 3328)
           in
           FStar.Classical.forall_intro aux);
           Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_matrix_intro $K #$:Vector
             (${unpacked_public_key}.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A) (mk_usize 3328)"#
    );
}

/// Call [`deserialize_then_decompress_ring_element_u`] on each ring element
/// in the `ciphertext`.
// FOLLOW-UP (Phase F Stream 2 retry 2026-05-06): ntt_vector_u now has the
// functional ensures (src/ntt.rs line 561-563), so the original blocker is
// gone. New blockers (~10 errors at extracted Ind_cpa.fst:1050-1106 under
// panic_free):
//   1. deserialize_then_decompress_ring_element_u's ensures
//      (`poly_to_spec result == decompress(byte_decode_dyn ...)`) lacks the
//      `is_bounded_poly(3328, result)` conjunct that ntt_vector_u requires.
//      Decompress's spec output is in [0, q-1], but the impl-level bound
//      doesn't propagate from poly_to_spec.
//   2. The loop invariant uses `poly_to_spec` while ntt_vector_u uses
//      `to_spec_poly_plain`. Bridge lemma `poly_to_spec_eq_to_spec_poly_plain`
//      exists in Hacspec_ml_kem.Commute.Bridges but must be applied each
//      iteration to convert.
//   3. The functional ensures requires loop-invariant maintenance through
//      both deserialize_then_decompress_ring_element_u AND ntt_vector_u
//      composing correctly. 60+ min dedicated session.
#[hax_lib::fstar::verification_status(panic_free)]
#[inline(always)]
// Local-suppression band-aid: drop the `is_bounded_poly` equation
// (whose body cascades on every `Seq.index p.f_coefficients i` access via
// the array_from_fn refinement, ~66k instantiations in this query) and
// the `array_from_fn` refinement itself.  Verifies cleanly without
// these.  The principled fix — making `is_bounded_poly` opaque + adding
// SMTPat'd elim/intro/higher lemmas — breaks ~10 functions across
// Ntt/Invert_ntt that need explicit reveal+intro updates; deferred to a
// separate sprint.
// The functional conjunct below is stated as a TRUST BOUNDARY (kept `panic_free`, so admitted):
// the impl fuses deserialize+decompress+NTT per element, matching the spec's
// `vector_ntt ∘ deserialize_then_decompress_u`.  Consumed by `decrypt_unpacked`.  (Provable for
// real via a per-element functional loop invariant + the poly_to_spec ↔ to_spec_poly_plain bridge;
// deferred.)  The Hacspec spec modules are added to the `using_facts_from` prune list because the
// functional conjunct's spec-function references otherwise pollute the band-aided loop sub-query and
// make it saturate cold (rlimit 400/canceled); the body is admitted so pruning their facts is sound.
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always --using_facts_from '* -Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly -Rust_primitives.Slice.array_from_fn -Hacspec_ml_kem.Serialize -Hacspec_ml_kem.Ntt -Hacspec_ml_kem.Compress'")]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
    && U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
)]
#[hax_lib::ensures(|res|
    spec::is_bounded_polynomial_vector(3328, &res)
    & (crate::vector::spec::vector_to_spec(&res)
        == hacspec_ml_kem::serialize::deserialize_then_decompress_u_then_ntt::<K>(
            &ciphertext[..hacspec_ml_kem::parameters::c1_size(K)],
            U_COMPRESSION_FACTOR,
        ))
)]
fn deserialize_then_decompress_u<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const U_COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    ciphertext: &[u8; CIPHERTEXT_SIZE],
) -> [PolynomialRingElement<Vector>; K] {
    hax_lib::fstar!(
        "assert (v (($COEFFICIENTS_IN_RING_ELEMENT *! $U_COMPRESSION_FACTOR ) /!
        sz 8) == v (Hacspec_ml_kem.Parameters.c1_block_size $K))"
    );
    let mut u_as_ntt: [PolynomialRingElement<Vector>; K] =
        hacspec_ml_kem::parameters::createi(|_| PolynomialRingElement::<Vector>::ZERO());
    let block_size = (COEFFICIENTS_IN_RING_ELEMENT * U_COMPRESSION_FACTOR) / 8;
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| { fstar!(r#"forall (j:nat). j < v $i ==>
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #$:Vector (sz 3328) u_as_ntt.[ sz j ]"#) });
        let u_bytes = &ciphertext[i * block_size..(i + 1) * block_size];
        u_as_ntt[i] = deserialize_then_decompress_ring_element_u::<U_COMPRESSION_FACTOR, Vector>(u_bytes);
        ntt_vector_u::<U_COMPRESSION_FACTOR, Vector>(&mut u_as_ntt[i]);
    }
    // Fold per-element bound forall into the opaque
    // is_bounded_polynomial_vector atom required by the ensures.
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro
            $K #$:Vector $u_as_ntt (sz 3328)"#
    );
    u_as_ntt
}

/// Call [`deserialize_to_uncompressed_ring_element`] for each ring element.
#[hax_lib::fstar::verification_status(panic_free)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning")]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && secret_key.len() == hacspec_ml_kem::parameters::cpa_private_key_size(K)
)]
#[hax_lib::ensures(|()|
    spec::is_bounded_polynomial_vector(3328, future(secret_as_ntt))
    & (crate::vector::spec::vector_to_spec(future(secret_as_ntt)) ==
       hacspec_ml_kem::serialize::vector_decode_12::<K>(secret_key))
)]
pub(crate) fn deserialize_vector<const K: usize, Vector: Operations>(
    secret_key: &[u8],
    secret_as_ntt: &mut [PolynomialRingElement<Vector>; K],
) {

    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"forall (j: nat). j < v $i ==>
                j * v $BYTES_PER_RING_ELEMENT + v $BYTES_PER_RING_ELEMENT <=
                    v (Hacspec_ml_kem.Parameters.cpa_private_key_size $K) /\
                ${poly_to_spec::<Vector>} (Seq.index $secret_as_ntt j) ==
                    Hacspec_ml_kem.Serialize.byte_decode (sz 384) (sz 3072) (Seq.slice $secret_key
                        (j * v $BYTES_PER_RING_ELEMENT)
                        (j * v $BYTES_PER_RING_ELEMENT + v $BYTES_PER_RING_ELEMENT)) (sz 12)"#
            )
        });
        secret_as_ntt[i] = deserialize_to_uncompressed_ring_element(
            &secret_key[i * BYTES_PER_RING_ELEMENT..(i + 1) * BYTES_PER_RING_ELEMENT],
        );
    }
    hax_lib::fstar!(
        r#"let lemma_post (j: nat) : Lemma
            (requires j < v $K)
            (ensures
              Seq.index (${vector_to_spec::<K, Vector>} $K $secret_as_ntt) j ==
              Seq.index (Hacspec_ml_kem.Serialize.vector_decode_12_ $K $secret_key) j) =
          let slice = Seq.slice $secret_key
              (j * v $BYTES_PER_RING_ELEMENT)
              (j * v $BYTES_PER_RING_ELEMENT + v $BYTES_PER_RING_ELEMENT) in
          let chunk:t_Array u8 (mk_usize 384) =
            Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 384))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice u8)
                  #(t_Array u8 (mk_usize 384))
                  #FStar.Tactics.Typeclasses.solve
                  slice) in
          eq_intro (chunk <: Seq.seq u8) slice
        in Classical.forall_intro (Classical.move_requires lemma_post);
        eq_intro
        (${vector_to_spec::<K, Vector>} $K $secret_as_ntt)
        (Hacspec_ml_kem.Serialize.vector_decode_12_ $K $secret_key)"#
    );
}

/// This function implements <strong>Algorithm 14</strong> of the
/// NIST FIPS 203 specification; this is the Kyber CPA-PKE decryption algorithm.
///
/// Algorithm 14 is reproduced below:
///
/// ```plaintext
/// Input: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
/// Input: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
/// Output: message m ∈ 𝔹^{32}.
///
/// c₁ ← c[0 : 32dᵤk]
/// c₂ ← c[32dᵤk : 32(dᵤk + dᵥ)]
/// u ← Decompress_{dᵤ}(ByteDecode_{dᵤ}(c₁))
/// v ← Decompress_{dᵥ}(ByteDecode_{dᵥ}(c₂))
/// ŝ ← ByteDecode₁₂(dkₚₖₑ)
/// w ← v - NTT-¹(ŝᵀ ◦ NTT(u))
/// m ← ByteEncode₁(Compress₁(w))
/// return m
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[allow(non_snake_case)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning")]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
        && U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
        && V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
        && VECTOR_U_ENCODED_SIZE == hacspec_ml_kem::parameters::c1_size(K)).to_prop()
    & spec::is_bounded_polynomial_vector(3328, &secret_key.secret_as_ntt)
)]
#[hax_lib::ensures(|result|
    result == hacspec_ml_kem::ind_cpa::decrypt_unpacked::<K>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        &crate::vector::spec::vector_to_spec(&secret_key.secret_as_ntt),
        ciphertext,
    )
)]
#[inline(always)]
pub(crate) fn decrypt_unpacked<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const VECTOR_U_ENCODED_SIZE: usize,
    const U_COMPRESSION_FACTOR: usize,
    const V_COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    secret_key: &IndCpaPrivateKeyUnpacked<K, Vector>,
    ciphertext: &[u8; CIPHERTEXT_SIZE],
) -> [u8; SHARED_SECRET_SIZE] {
    hax_lib::fstar!(
        r#"assert_norm (v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size (mk_usize 2)) -
                       v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 2)) ==
                       32 * v (Hacspec_ml_kem.Parameters.vector_v_compression_factor (mk_usize 2)));
           assert_norm (v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size (mk_usize 3)) -
                       v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 3)) ==
                       32 * v (Hacspec_ml_kem.Parameters.vector_v_compression_factor (mk_usize 3)));
           assert_norm (v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size (mk_usize 4)) -
                       v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 4)) ==
                       32 * v (Hacspec_ml_kem.Parameters.vector_v_compression_factor (mk_usize 4)))"#
    );
    // u := Decompress_q(Decode_{d_u}(c), d_u)
    let u_as_ntt = deserialize_then_decompress_u::<K, CIPHERTEXT_SIZE, U_COMPRESSION_FACTOR, Vector>(
        ciphertext,
    );

    // v := Decompress_q(Decode_{d_v}(c + d_u·k·n / 8), d_v)
    let v = deserialize_then_decompress_ring_element_v::<K, V_COMPRESSION_FACTOR, Vector>(
        &ciphertext[VECTOR_U_ENCODED_SIZE..],
    );

    // m := Encode_1(Compress_q(v − NTT^{−1}(sˆT ◦ NTT(u)) , 1))
    let message = compute_message(&v, &secret_key.secret_as_ntt, &u_as_ntt);
    let result = compress_then_serialize_message(message);

    // Compose the four callee functional posts into `result == Hacspec.decrypt_unpacked(...)`:
    // deserialize_then_decompress_u (u), deserialize_then_decompress_ring_element_v (v),
    // compute_message (w), compress_then_serialize_message.  `lemma_rank_to_params` supplies the
    // param-field facts; the assert_norms equate the spec's `u_encoded_size` with `c1_size`
    // (`Rust_primitives.Integers.v` is fully-qualified because the local `v` binder shadows it).
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Encrypt_bridge.lemma_rank_to_params $K;
        assert_norm (Rust_primitives.Integers.v (Hacspec_ml_kem.Parameters.impl_MlKemParams__u_encoded_size
                (Hacspec_ml_kem.Parameters.rank_to_params (mk_usize 2))) ==
            Rust_primitives.Integers.v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 2)));
        assert_norm (Rust_primitives.Integers.v (Hacspec_ml_kem.Parameters.impl_MlKemParams__u_encoded_size
                (Hacspec_ml_kem.Parameters.rank_to_params (mk_usize 3))) ==
            Rust_primitives.Integers.v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 3)));
        assert_norm (Rust_primitives.Integers.v (Hacspec_ml_kem.Parameters.impl_MlKemParams__u_encoded_size
                (Hacspec_ml_kem.Parameters.rank_to_params (mk_usize 4))) ==
            Rust_primitives.Integers.v (Hacspec_ml_kem.Parameters.c1_size (mk_usize 4)))"#
    );

    result
}

#[allow(non_snake_case)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && secret_key.len() == hacspec_ml_kem::parameters::cpa_private_key_size(K)
    && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
    && VECTOR_U_ENCODED_SIZE == hacspec_ml_kem::parameters::c1_size(K)
    && U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
    && V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
)]
#[hax_lib::ensures(|result|
    result == hacspec_ml_kem::ind_cpa::decrypt::<K>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        secret_key,
        ciphertext,
    )
)]
#[hax_lib::fstar::verification_status(panic_free)]
#[inline(always)]
pub(crate) fn decrypt<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const VECTOR_U_ENCODED_SIZE: usize,
    const U_COMPRESSION_FACTOR: usize,
    const V_COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    secret_key: &[u8],
    ciphertext: &[u8; CIPHERTEXT_SIZE],
) -> [u8; SHARED_SECRET_SIZE] {
    // sˆ := Decode_12(sk)
    let mut secret_key_unpacked = IndCpaPrivateKeyUnpacked {
        secret_as_ntt: from_fn(|_| PolynomialRingElement::<Vector>::ZERO()),
    };
    deserialize_vector::<K, Vector>(secret_key, &mut secret_key_unpacked.secret_as_ntt);

    decrypt_unpacked::<
        K,
        CIPHERTEXT_SIZE,
        VECTOR_U_ENCODED_SIZE,
        U_COMPRESSION_FACTOR,
        V_COMPRESSION_FACTOR,
        Vector,
    >(&secret_key_unpacked, ciphertext)
}
