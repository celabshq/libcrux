use crate::{
    constants::COEFFICIENTS_IN_RING_ELEMENT, hash_functions::*, helper::cloop,
    polynomial::PolynomialRingElement, vector::Operations,
};

#[cfg(hax)]
use crate::polynomial::spec;

#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::spec::{matrix_to_spec, poly_to_spec, vector_to_spec};

#[cfg(hax)]
use hax_lib::prop::ToProp;

/// If `bytes` contains a set of uniformly random bytes, this function
/// uniformly samples a ring element `â` that is treated as being the NTT representation
/// of the corresponding polynomial `a`.
///
/// Since rejection sampling is used, it is possible the supplied bytes are
/// not enough to sample the element, in which case an `Err` is returned and the
/// caller must try again with a fresh set of bytes.
///
/// This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
/// We say "partially" because this implementation only accepts a finite set of
/// bytes as input and returns an error if the set is not enough; Algorithm 6 of
/// the FIPS 203 standard on the other hand samples from an infinite stream of bytes
/// until the ring element is filled. Algorithm 6 is reproduced below:
///
/// ```plaintext
/// Input: byte stream B ∈ 𝔹*.
/// Output: array â ∈ ℤ₂₅₆.
///
/// i ← 0
/// j ← 0
/// while j < 256 do
///     d₁ ← B[i] + 256·(B[i+1] mod 16)
///     d₂ ← ⌊B[i+1]/16⌋ + 16·B[i+2]
///     if d₁ < q then
///         â[j] ← d₁
///         j ← j + 1
///     end if
///     if d₂ < q and j < 256 then
///         â[j] ← d₂
///         j ← j + 1
///     end if
///     i ← i + 3
/// end while
/// return â
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning")]
#[hax_lib::requires(hax_lib::forall(
    |i: usize| hax_lib::implies(i < K,
        (sampled_coefficients[i] <= COEFFICIENTS_IN_RING_ELEMENT).to_prop().and(
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < sampled_coefficients[i],
                    out[i][j] >= 0 && out[i][j] <= 3328))))))]
#[hax_lib::ensures(|_| hax_lib::forall(
    |i: usize| hax_lib::implies(i < K,
        (future(sampled_coefficients)[i] <= COEFFICIENTS_IN_RING_ELEMENT).to_prop().and(
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < future(sampled_coefficients)[i],
                    out[i][j] >= 0 && out[i][j] <= 3328))))))]
fn sample_from_uniform_distribution_next<Vector: Operations, const K: usize, const N: usize>(
    randomness: &[[u8; N]; K],
    sampled_coefficients: &mut [usize; K],
    out: &mut [[i16; 272]; K],
) -> bool {
    // Would be great to trigger auto-vectorization or at least loop unrolling here
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| hax_lib::implies(
            j < K,
            (if j < i {
                sampled_coefficients[j] <= COEFFICIENTS_IN_RING_ELEMENT + 16
            } else {
                sampled_coefficients[j] <= COEFFICIENTS_IN_RING_ELEMENT
            })
            .to_prop()
            .and(hax_lib::forall(|k: usize| hax_lib::implies(
                k < sampled_coefficients[j],
                out[j][k] >= 0 && out[j][k] <= 3328
            )))
        )));

        for r in 0..N / 24 {
            hax_lib::loop_invariant!(|r: usize| hax_lib::forall(|j: usize| hax_lib::implies(
                j < K,
                (if j <= i {
                    sampled_coefficients[j] <= COEFFICIENTS_IN_RING_ELEMENT + 16
                } else {
                    sampled_coefficients[j] <= COEFFICIENTS_IN_RING_ELEMENT
                })
                .to_prop()
                .and(hax_lib::forall(|k: usize| hax_lib::implies(
                    k < sampled_coefficients[j],
                    out[j][k] >= 0 && out[j][k] <= 3328
                )))
            )));

            if sampled_coefficients[i] < COEFFICIENTS_IN_RING_ELEMENT {
                let sampled = Vector::rej_sample(
                    &randomness[i][r * 24..(r * 24) + 24],
                    &mut out[i][sampled_coefficients[i]..sampled_coefficients[i] + 16],
                );
                sampled_coefficients[i] += sampled;
            }
        }
    }
    let mut done = true;
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| hax_lib::implies(
            j < K,
            (if j < i {
                sampled_coefficients[j] <= COEFFICIENTS_IN_RING_ELEMENT
            } else {
                sampled_coefficients[j] <= COEFFICIENTS_IN_RING_ELEMENT + 16
            })
            .to_prop()
            .and(hax_lib::forall(|k: usize| hax_lib::implies(
                k < sampled_coefficients[j],
                out[j][k] >= 0 && out[j][k] <= 3328
            )))
        )));

        if sampled_coefficients[i] >= COEFFICIENTS_IN_RING_ELEMENT {
            sampled_coefficients[i] = COEFFICIENTS_IN_RING_ELEMENT;
        } else {
            done = false
        }
    }
    done
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::ensures(|result| hax_lib::forall(|i:usize| hax_lib::implies(i < K,
                                spec::is_bounded_poly(3328, &result[i]))))]
pub(super) fn sample_from_xof<const K: usize, Vector: Operations, Hasher: Hash<K>>(
    seeds: &[[u8; 34]; K],
) -> [PolynomialRingElement<Vector>; K] {
    let mut sampled_coefficients: [usize; K] = [0; K];
    let mut out: [[i16; 272]; K] = [[0; 272]; K];

    let mut xof_state = Hasher::shake128_init_absorb_final(seeds);
    let randomness = xof_state.shake128_squeeze_first_three_blocks();

    let mut done = sample_from_uniform_distribution_next::<Vector, K, THREE_BLOCKS>(
        &randomness,
        &mut sampled_coefficients,
        &mut out,
    );

    // Requiring more than 5 blocks to sample a ring element should be very
    // unlikely according to:
    // https://eprint.iacr.org/2023/708.pdf
    // To avoid failing here, we squeeze more blocks out of the state until
    // we have enough.
    while !done {
        let randomness = xof_state.shake128_squeeze_next_block();
        done = sample_from_uniform_distribution_next::<Vector, K, BLOCK_SIZE>(
            &randomness,
            &mut sampled_coefficients,
            &mut out,
        );
    }

    out.map(|s| PolynomialRingElement::<Vector>::from_i16_array(&s[0..256]))
}

/// Given a series of uniformly random bytes in `randomness`, for some number `eta`,
/// the `sample_from_binomial_distribution_{eta}` functions sample
/// a ring element from a binomial distribution centered at 0 that uses two sets
/// of `eta` coin flips. If, for example,
/// `eta = ETA`, each ring coefficient is a value `v` such
/// such that `v ∈ {-ETA, -ETA + 1, ..., 0, ..., ETA + 1, ETA}` and:
///
/// ```plaintext
/// - If v < 0, Pr[v] = Pr[-v]
/// - If v >= 0, Pr[v] = BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2 ^ (2 * ETA)
/// ```
///
/// The values `v < 0` are mapped to the appropriate `KyberFieldElement`.
///
/// The expected value is:
///
/// ```plaintext
/// E[X] = (-ETA)Pr[-ETA] + (-(ETA - 1))Pr[-(ETA - 1)] + ... + (ETA - 1)Pr[ETA - 1] + (ETA)Pr[ETA]
///      = 0 since Pr[-v] = Pr[v] when v < 0.
/// ```
///
/// And the variance is:
///
/// ```plaintext
/// Var(X) = E[(X - E[X])^2]
///        = E[X^2]
///        = sum_(v=-ETA to ETA)v^2 * (BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2^(2 * ETA))
///        = ETA / 2
/// ```
///
/// This function implements <strong>Algorithm 7</strong> of the NIST FIPS 203 standard, which is
/// reproduced below:
///
/// ```plaintext
/// Input: byte array B ∈ 𝔹^{64η}.
/// Output: array f ∈ ℤ₂₅₆.
///
/// b ← BytesToBits(B)
/// for (i ← 0; i < 256; i++)
///     x ← ∑(j=0 to η - 1) b[2iη + j]
///     y ← ∑(j=0 to η - 1) b[2iη + η + j]
///     f[i] ← x−y mod q
/// end for
/// return f
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::requires(randomness.len() == 2 * 64)]
// TODO: Remove or replace with something that works and is useful for the proof.
// #[cfg_attr(hax, hax_lib::ensures(|result|
//     hax_lib::forall(|i:usize|
//         hax_lib::implies(i < result.coefficients.len(), || result.coefficients[i].abs() <= 2
// ))))]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800")]
fn sample_from_binomial_distribution_2<Vector: Operations>(
    randomness: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        "assert (v (sz 2 *! sz 64) == 128);
        assert (Seq.length $randomness == 128)"
    );
    let mut sampled_i16s = [0i16; 256];

    cloop! {
        for (chunk_number, byte_chunk) in randomness.chunks_exact(4).enumerate() {
            let random_bits_as_u32: u32 = (byte_chunk[0] as u32)
                | (byte_chunk[1] as u32) << 8
                | (byte_chunk[2] as u32) << 16
                | (byte_chunk[3] as u32) << 24;

            let even_bits = random_bits_as_u32 & 0x55555555;
            let odd_bits = (random_bits_as_u32 >> 1) & 0x55555555;
            hax_lib::fstar!(r#"logand_lemma $random_bits_as_u32 (mk_u32 1431655765);
                logand_lemma ($random_bits_as_u32 >>! (mk_i32 1)) (mk_u32 1431655765)"#);
            let coin_toss_outcomes = even_bits + odd_bits;

            cloop! {
                for outcome_set in (0..32u32).step_by(4) { // u32::BITS
                    let outcome_1 = ((coin_toss_outcomes >> outcome_set) & 0x3) as i16;
                    let outcome_2 = ((coin_toss_outcomes >> (outcome_set + 2)) & 0x3) as i16;
                    hax_lib::fstar!(r#"logand_lemma ($coin_toss_outcomes >>! $outcome_set <: u32) (mk_u32 3);
                        logand_lemma ($coin_toss_outcomes >>! ($outcome_set +! (mk_u32 2) <: u32) <: u32) (mk_u32 3);
                        assert (v $outcome_1 >= 0 /\ v $outcome_1 <= 3);
                        assert (v $outcome_2 >= 0 /\ v $outcome_2 <= 3);
                        assert (v $chunk_number <= 31);
                        assert (v (sz 8 *! $chunk_number <: usize) <= 248);
                        assert (v (cast ($outcome_set >>! (mk_i32 2) <: u32) <: usize) <= 7)"#);

                    let offset = (outcome_set >> 2) as usize;
                    sampled_i16s[8 * chunk_number + offset] = outcome_1 - outcome_2;
                }
            }
        }
    }
    PolynomialRingElement::from_i16_array(&sampled_i16s)
}

#[hax_lib::requires(randomness.len() == 3 * 64)]
// TODO: Remove or replace with something that works and is useful for the proof.
// #[cfg_attr(hax, hax_lib::ensures(|result|
//     hax_lib::forall(|i:usize|
//         hax_lib::implies(i < result.coefficients.len(), || result.coefficients[i].abs() <= 3
// ))))]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800")]
fn sample_from_binomial_distribution_3<Vector: Operations>(
    randomness: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        "assert (v (sz 3 *! sz 64) == 192);
        assert (Seq.length $randomness == 192)"
    );
    let mut sampled_i16s = [0i16; 256];

    cloop! {
        for (chunk_number, byte_chunk) in randomness.chunks_exact(3).enumerate() {
            let random_bits_as_u24: u32 =
                (byte_chunk[0] as u32) | (byte_chunk[1] as u32) << 8 | (byte_chunk[2] as u32) << 16;

            let first_bits = random_bits_as_u24 & 0x00249249;
            let second_bits = (random_bits_as_u24 >> 1) & 0x00249249;
            let third_bits = (random_bits_as_u24 >> 2) & 0x00249249;
            hax_lib::fstar!(r#"logand_lemma $random_bits_as_u24 (mk_u32 2396745);
                logand_lemma ($random_bits_as_u24 >>! (mk_i32 1) <: u32) (mk_u32 2396745);
                logand_lemma ($random_bits_as_u24 >>! (mk_i32 2) <: u32) (mk_u32 2396745)"#);

            let coin_toss_outcomes = first_bits + second_bits + third_bits;

            cloop! {
                for outcome_set in (0..24).step_by(6) {
                    let outcome_1 = ((coin_toss_outcomes >> outcome_set) & 0x7) as i16;
                    let outcome_2 = ((coin_toss_outcomes >> (outcome_set + 3)) & 0x7) as i16;
                    hax_lib::fstar!(r#"logand_lemma ($coin_toss_outcomes >>! $outcome_set <: u32) (mk_u32 7);
                        logand_lemma ($coin_toss_outcomes >>! ($outcome_set +! (mk_i32 3) <: i32) <: u32) (mk_u32 7);
                        assert (v $outcome_1 >= 0 /\ v $outcome_1 <= 7);
                        assert (v $outcome_2 >= 0 /\ v $outcome_2 <= 7);
                        assert (v $chunk_number <= 63);
                        assert (v (sz 4 *! $chunk_number <: usize) <= 252);
                        assert (v (cast ($outcome_set /! (mk_i32 6) <: i32) <: usize) <= 3)"#);

                    let offset = (outcome_set / 6) as usize;
                    sampled_i16s[4 * chunk_number + offset] = outcome_1 - outcome_2;
                }
            }
        }
    }
    PolynomialRingElement::from_i16_array(&sampled_i16s)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires((ETA == 2 || ETA == 3) && randomness.len() == ETA * 64)]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3) ${result} /\
    ${poly_to_spec::<Vector>} $result ==
        Hacspec_ml_kem.Sampling.sample_poly_cbd ($ETA *! sz 64) ($ETA *! sz 512) $ETA $randomness"#))]
/// CBD sampling.  Output lanes are in **plain** form (`v c ≡ α mod q`)
/// where α is a small CBD-distributed integer (η ∈ {2, 3}).  No Montgomery
/// scaling is applied — sampled values feed directly into NTT, and
/// `ntt_binomially_sampled_ring_element` preserves plain form (see
/// `src/ntt.rs` and the `ntt_matches_spec` cross-spec test).
pub(super) fn sample_from_binomial_distribution<const ETA: usize, Vector: Operations>(
    randomness: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (
        (v (cast $ETA <: u32) == 2) \/
        (v (cast $ETA <: u32) == 3))"#
    );
    match ETA as u32 {
        2 => sample_from_binomial_distribution_2(randomness),
        3 => sample_from_binomial_distribution_3(randomness),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod cross_spec_tests {
    use super::*;
    use crate::polynomial::cross_spec_tests::lift_poly;
    use crate::vector::portable::PortableVector;
    use hacspec_ml_kem::parameters::FieldElement;

    /// CBD eta=2: impl and spec produce identical polynomials from the same bytes.
    #[test]
    fn cbd_eta2_matches_spec() {
        for pattern in [0x00u8, 0x55, 0xAA, 0xFF] {
            let bytes = [pattern; 128];
            let impl_poly = sample_from_binomial_distribution::<2, PortableVector>(&bytes);
            let spec_poly = hacspec_ml_kem::sampling::sample_poly_cbd::<128, 1024>(2, &bytes);
            assert_eq!(
                lift_poly(&impl_poly),
                spec_poly,
                "CBD eta=2 mismatch for pattern 0x{pattern:02X}"
            );
        }
    }

    /// CBD eta=3: impl and spec produce identical polynomials from the same bytes.
    #[test]
    fn cbd_eta3_matches_spec() {
        for pattern in [0x00u8, 0x55, 0xAA, 0xFF] {
            let bytes = [pattern; 192];
            let impl_poly = sample_from_binomial_distribution::<3, PortableVector>(&bytes);
            let spec_poly = hacspec_ml_kem::sampling::sample_poly_cbd::<192, 1536>(3, &bytes);
            assert_eq!(
                lift_poly(&impl_poly),
                spec_poly,
                "CBD eta=3 mismatch for pattern 0x{pattern:02X}"
            );
        }
    }

    /// CBD with varying byte patterns to stress different bit combinations.
    #[test]
    fn cbd_eta2_sequential_bytes() {
        let bytes: [u8; 128] = core::array::from_fn(|i| i as u8);
        let impl_poly = sample_from_binomial_distribution::<2, PortableVector>(&bytes);
        let spec_poly = hacspec_ml_kem::sampling::sample_poly_cbd::<128, 1024>(2, &bytes);
        assert_eq!(lift_poly(&impl_poly), spec_poly);
    }

    #[test]
    fn cbd_eta3_sequential_bytes() {
        let bytes: [u8; 192] = core::array::from_fn(|i| i as u8);
        let impl_poly = sample_from_binomial_distribution::<3, PortableVector>(&bytes);
        let spec_poly = hacspec_ml_kem::sampling::sample_poly_cbd::<192, 1536>(3, &bytes);
        assert_eq!(lift_poly(&impl_poly), spec_poly);
    }
}
