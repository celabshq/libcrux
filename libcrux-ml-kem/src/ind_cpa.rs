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
#[hax_lib::fstar::verification_status(panic_free)]
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
}

/// Call [`serialize_uncompressed_ring_element`] for each ring element.
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
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
}

/// Sample a vector of ring elements from a centered binomial distribution.
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options(
    "--max_fuel 15 --z3rlimit 400 --ext context_pruning --split_queries always"
)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
        && ETA2 == hacspec_ml_kem::parameters::eta2(K)
        && (domain_separator as usize) < 2 * K
        && (domain_separator as usize) + K < 256).to_prop()
)]
#[hax_lib::ensures(|ds|
    (ds == domain_separator + (K as u8)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(7, future(error_1))
    & (crate::vector::spec::vector_to_spec(future(error_1)) ==
        hacspec_ml_kem::ind_cpa::sample_vector_cbd::<K>(
            ETA2, &prf_input[..32], domain_separator)).to_prop()
)]
fn sample_ring_element_cbd<
    const K: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    prf_input: &[u8; 33],
    mut domain_separator: u8,
    error_1: &mut [PolynomialRingElement<Vector>; K],
) -> u8 {
    let mut prf_inputs = [prf_input.clone(); K];

    #[cfg(hax)]
    let _domain_separator_init = domain_separator;

    domain_separator = prf_input_inc::<K>(&mut prf_inputs, domain_separator);
    let prf_outputs: [[u8; ETA2_RANDOMNESS_SIZE]; K] = Hasher::PRFxN(&prf_inputs);
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                forall (j:nat). j < v $i ==>
                    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 7) (Seq.index ${error_1} j)"#
            )
        });
        error_1[i] = sample_from_binomial_distribution::<ETA2, Vector>(&prf_outputs[i]);
    }

    domain_separator
}

/// Sample a vector of ring elements from a centered binomial distribution and
/// convert them into their NTT representations.
#[inline(always)]
// FOLLOW-UP (Phase D): body fails panic_free precondition check on
// ntt_binomially_sampled_ring_element call (line 352 in extracted F*) — needs
// loop-invariant strengthening of bounds on accumulator. Stays lax.
#[hax_lib::fstar::verification_status(lax)]
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
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::before(r#"[@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 500 --ext context_pruning")]
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
#[hax_lib::fstar::verification_status(panic_free)]
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

    serialize_unpacked_secret_key::<K, PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE, Vector>(
        &public_key,
        &private_key,
    )
}

/// Serialize the secret key from the unpacked key pair generation.
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        && PRIVATE_KEY_SIZE == hacspec_ml_kem::parameters::ranked_bytes_per_ring_element(K)).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &public_key.t_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &private_key.secret_as_ntt)
)]
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
// FOLLOW-UP (Phase D): body has eq_intro spec-equality assertion that fails
// to discharge under panic_free at rlimit 800 (same pattern as serialize_vector).
// Stays lax.
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning")]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
        && OUT_LEN == hacspec_ml_kem::parameters::c1_size(K)
        && COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
        && BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
        && out.len() == OUT_LEN).to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &input)
)]
#[hax_lib::ensures(|()| {
    let mut expected = [0u8; 1408]; // max K=4, du=11: 4 * 256 * 11 / 8
    let len = (K * 256 * COMPRESSION_FACTOR) / 8;
    hacspec_ml_kem::serialize::compress_then_serialize_u_into::<K>(
        &crate::vector::spec::vector_to_spec(&input),
        COMPRESSION_FACTOR,
        &mut expected[..len],
    );
    future(out)[..] == expected[..len]
})]
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
    hax_lib::fstar!(
        r#"eq_intro out
        (Hacspec_ml_kem.Serialize.compress_then_serialize_u $K $OUT_LEN
            (${vector_to_spec::<K, Vector>} $K $input) $COMPRESSION_FACTOR)"#
    )
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
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning")]
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
)]
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::ind_cpa::encrypt_unpacked::<K, C1_LEN, C2_LEN, CIPHERTEXT_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        &crate::vector::spec::vector_to_spec(&public_key.t_as_ntt),
        &hacspec_ml_kem::matrix::transpose(&crate::vector::spec::matrix_to_spec(&public_key.A)),
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

    ciphertext
}

// FOLLOW-UP (Phase D): body fails panic_free precondition check on
// into_padded_array call — randomness slice bound not propagated. Stays lax.
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::ensures(|(ciphertext_out, _)| ciphertext_out.len() == ciphertext.len())]
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
    let u = compute_vector_u(matrix, &r_as_ntt, &error_1);

    // c_1 := Encode_{du}(Compress_q(u,d_u))
    compress_then_serialize_u::<K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, Vector>(u, ciphertext);

    (r_as_ntt, error_2)
}

// FOLLOW-UP (Phase D): body fails panic_free precondition check on
// Matrix.compute_ring_element_v call — bounds on inputs not propagated. Stays lax.
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::ensures(|()| future(ciphertext).len() == ciphertext.len())]
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
}

#[allow(non_snake_case)]
#[hax_lib::fstar::verification_status(panic_free)]
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

    // After unpacking the public key we can now call the unpacked decryption.
    encrypt_unpacked::<
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
    >(&unpacked_public_key, message, randomness)
}

#[hax_lib::fstar::verification_status(panic_free)]
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
    crate::vector::spec::vector_to_spec(&future(unpacked_public_key).t_as_ntt)
        == hacspec_ml_kem::serialize::vector_decode_12::<K>(
            &public_key[..T_AS_NTT_ENCODED_SIZE],
        )
    && match hacspec_ml_kem::matrix::sample_matrix_A::<K>(
        &public_key[T_AS_NTT_ENCODED_SIZE..],
        false,
    ) {
        Ok(A_as_ntt) => crate::vector::spec::matrix_to_spec(&future(unpacked_public_key).A) == A_as_ntt,
        Err(_) => true,
    }
})]
#[hax_lib::fstar::verification_status(panic_free)]
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
}

/// Call [`deserialize_then_decompress_ring_element_u`] on each ring element
/// in the `ciphertext`.
// FOLLOW-UP (Phase F Stream 2): tried the lemma_post + Classical.move_requires
// + Seq.slice-of-slice eq_intro pattern (matches lighthouse deserialize_vector).
// Proof body discharged the eq_intro under panic_free at rlimit 800, but the
// body's loop-invariant maintenance cannot be re-established because
// `ntt_vector_u` lacks a functional ensures of the form
//   poly_to_spec(future(re)) == Hacspec_ml_kem.Ntt.ntt(poly_to_spec(re))
// (the ensures is commented out in src/ntt.rs).  Without it, after
// `ntt_vector_u(&mut u_as_ntt[i])` the loop invariant cannot be maintained
// under panic_free.  Stays lax pending strengthening of ntt_vector_u's
// ensures (out of this Stream's scope; would touch src/ntt.rs and
// require establishing the ntt body proof).
#[hax_lib::fstar::verification_status(lax)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning")]
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
    let mut u_as_ntt = from_fn(|_| PolynomialRingElement::<Vector>::ZERO());
    cloop! {
        for (i, u_bytes) in ciphertext
            .chunks_exact((COEFFICIENTS_IN_RING_ELEMENT * U_COMPRESSION_FACTOR) / 8)
            .enumerate()
        {
            hax_lib::loop_invariant!(|i: usize| { fstar!(r#"forall (j: nat). j < v $i ==>
              j * v (Hacspec_ml_kem.Parameters.c1_block_size $K) + v (Hacspec_ml_kem.Parameters.c1_block_size $K) <= v $CIPHERTEXT_SIZE /\
              ${poly_to_spec::<Vector>} (Seq.index $u_as_ntt j) ==
                Hacspec_ml_kem.Ntt.ntt (Hacspec_ml_kem.Compress.decompress
                  (Hacspec_ml_kem.Serialize.byte_decode_dyn
                    (Seq.slice $ciphertext (j * v (Hacspec_ml_kem.Parameters.c1_block_size $K))
                      (j * v (Hacspec_ml_kem.Parameters.c1_block_size $K) + v (Hacspec_ml_kem.Parameters.c1_block_size $K)))
                    $U_COMPRESSION_FACTOR)
                  $U_COMPRESSION_FACTOR)"#) });
            u_as_ntt[i]  = deserialize_then_decompress_ring_element_u::<U_COMPRESSION_FACTOR, Vector>(u_bytes);
            ntt_vector_u::<U_COMPRESSION_FACTOR, Vector>(&mut u_as_ntt[i]);
        }
    }
    hax_lib::fstar!(
        r#"eq_intro
        (${vector_to_spec::<K, Vector>} $K $u_as_ntt)
        (Hacspec_ml_kem.Ntt.vector_ntt $K
          (Hacspec_ml_kem.Serialize.deserialize_then_decompress_u $K
            (Seq.slice $ciphertext 0 (v (Hacspec_ml_kem.Parameters.c1_size $K)))
            $U_COMPRESSION_FACTOR))"#
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
#[hax_lib::fstar::verification_status(panic_free)]
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
    compress_then_serialize_message(message)
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
