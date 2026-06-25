use crate::{parameters::*, serialize::*};

#[derive(Debug)]
pub struct BadRejectionSamplingRandomnessError;

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
/// One iteration of rejection sampling.
///
/// Decodes 24 bytes into 16 u16 values using 12-bit little-endian packing
/// (FIPS 203 Algorithm 6, inner loop body), keeps those `< q`, and returns
/// the accepted values packed at the start of a 16-element array together
/// with the count.  The unused tail of the array is filled with zeros.
///
/// This is the natural unit of rejection sampling at the trait boundary
/// (`Operations::rej_sample` consumes exactly 24 bytes); `sample_ntt`
/// below repeatedly applies this step until 256 coefficients have been
/// accepted or the byte stream is exhausted.
pub fn rej_sample_step(bytes: [u8; 24]) -> ([FieldElement; 16], usize) {
    let decoded = byte_decode_generic::<2, 16, 24, 192>(&bytes, 12);
    let mut result = [FieldElement::new(0); 16];
    let mut count: usize = 0;
    for i in 0..16 {
        hax_lib::loop_invariant!(|i: usize| count <= i);
        if decoded[i] < FIELD_MODULUS {
            result[count] = FieldElement::new(decoded[i]);
            count += 1;
        }
    }
    (result, count)
}

#[hax_lib::requires(
    N <= MAX_BYTES / 96
    && N8 == N * 8
    && N12 == N * 12
    && N96 == N12 * 8
)]
pub fn sample_ntt<const N: usize, const N8: usize, const N12: usize, const N96: usize>(
    bytes: [u8; N12],
) -> Result<Polynomial, BadRejectionSamplingRandomnessError> {
    let decoded = byte_decode_generic::<N, N8, N12, N96>(&bytes, 12);
    let mut result = [FieldElement::new(0); 256];
    let mut sampled_coefficients: usize = 0;
    for i in 0..N8 {
        if decoded[i] < FIELD_MODULUS && sampled_coefficients < 256 {
            result[sampled_coefficients] = FieldElement::new(decoded[i]);
            sampled_coefficients += 1
        }
    }
    if sampled_coefficients == 256 {
        Ok(result)
    } else {
        Err(BadRejectionSamplingRandomnessError)
    }
}

#[hax_lib::requires(eta <= 4 && coins.len() == eta)]
#[hax_lib::ensures(|r| r.val <= eta as u16)]
fn sum_coins(eta: usize, coins: &[bool]) -> FieldElement {
    hax_lib::debug_assert!(eta <= 4 && coins.len() == eta);
    let mut sum: u16 = 0;
    for i in 0..eta {
        hax_lib::loop_invariant!(|i: usize| sum <= (i as u16));
        sum += coins[i] as u16;
    }
    FieldElement::new(sum)
}

/// Given a series of uniformly random bytes in `randomness`, sample
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
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(eta <= 4 && ETA64 == eta * 64 && ETA512 == eta * 512)]
pub fn sample_poly_cbd<const ETA64: usize, const ETA512: usize>(
    eta: usize,
    bytes: &[u8; ETA64],
) -> Polynomial {
    hax_lib::debug_assert!(ETA64 == eta * 64 && ETA512 == eta * 512);
    let bits: [bool; ETA512] = bytes_to_bits(bytes);
    createi(|i| {
        let x: FieldElement = sum_coins(eta, &bits[(2 * i * eta)..(2 * i * eta + eta)]);
        let y: FieldElement = sum_coins(eta, &bits[(2 * i * eta + eta)..(2 * i * eta + 2 * eta)]);
        FieldElement::new((x.val + FIELD_MODULUS - y.val) % FIELD_MODULUS)
    })
}

#[cfg(test)]
mod tests {
    use proptest::collection::vec;
    use proptest::prelude::*;

    use super::*;

    fn field_coefficient_to_binomial_sample(sampling_coins: usize, coefficient: u16) -> f64 {
        if sampling_coins == 2 {
            match coefficient {
                3327 => -2.0,
                3328 => -1.0,
                0 => 0.0,
                1 => 1.0,
                2 => 2.0,
                other => panic!(
                    "{} should not be a coefficient when ETA = {}.",
                    other, sampling_coins
                ),
            }
        } else if sampling_coins == 3 {
            match coefficient {
                3326 => -3.0,
                3327 => -2.0,
                3328 => -1.0,
                0 => 0.0,
                1 => 1.0,
                2 => 2.0,
                3 => 3.0,
                other => panic!(
                    "{} should not be a coefficient when ETA = {}.",
                    other, sampling_coins
                ),
            }
        } else {
            panic!("ETA is neither 2 nor 3.");
        }
    }

    fn percentage_error(actual: f64, expected: f64) -> f64 {
        ((actual - expected).abs() / expected.abs()) * 100.0
    }

    const REJECTION_SAMPLING_ATTEMPTS: usize = 3;

    #[test]
    fn uniform_sample_from_all_zeros() {
        let r = sample_ntt::<105, 840, 1260, 10080>([0; 1260]).unwrap();
        for coefficient in r.into_iter() {
            assert_eq!(coefficient, FieldElement::new(0));
        }
    }

    #[test]
    #[should_panic]
    fn uniform_sample_from_all_u8_max() {
        let _ = sample_ntt::<105, 840, 1260, 10080>([u8::MAX; 1260]).unwrap();
    }

    proptest! {
        #[test]
        #[ignore = "see https://github.com/cryspen/libcrux/issues/112"]
        fn uniform_sampler_mean_and_variance(randomness in vec(any::<u8>(), 3 * 1260)) {
            let mut sampled_ring_element = [FieldElement::new(0); 256];

            let mut sampling_attempts = (0..REJECTION_SAMPLING_ATTEMPTS).peekable();
            while let Some(attempt) = sampling_attempts.next() {
                let sampled = sample_ntt::<105, 840, 1260, 10080>(randomness[attempt*1260..(attempt+1)*1260].try_into().unwrap());

                if sampled.is_ok() {
                    sampled_ring_element = sampled.unwrap();
                    break;
                } else if sampling_attempts.peek().is_none() {
                    panic!("Unable to uniformly sample a ring element with the given randomness.");
                }
            }

            // Mean = (a + b) / 2 = (0 + 3328) / 2
            let expected_mean : f64 = 1664.0;

            // Variance = (n^2 - 1) / 12 where:
            // n = b - a + 1 = 3328 - 0 + 1 = 3329
            let expected_variance : f64 = 923_520.0;

            let mut mean : f64 = 0.0;
            let mut variance : f64 = 0.0;

            for coefficient in sampled_ring_element.iter() {
                mean += f64::from(coefficient.val);
            }
            mean /= sampled_ring_element.len() as f64;

            for coefficient in sampled_ring_element.iter() {
                let coefficient_value : f64 = f64::from(coefficient.val);
                variance += (coefficient_value - mean) * (coefficient_value - mean);
            }
            variance /= (sampled_ring_element.len() - 1) as f64;

            // We use looser tolerances as compared to in the binomial
            // sampling test since we're not sampling as many elements here
            // (we can never be sure what randomness we have will allow for
            // a ring element to be sampled uniformly).
            assert!(percentage_error(mean, expected_mean) < 25.0, "The variance is {}.", variance);
            assert!(percentage_error(variance, expected_variance) < 25.0, "The variance is {}.", variance);
        }

        #[test]
        #[ignore = "see https://github.com/cryspen/libcrux/issues/112"]
        fn binomial_sampler_mean_and_variance(
            randomness in vec(any::<u8>(), 2 * 2 * 64)
            )
            {
            // TODO: Generalize sampling coins to be in {2, 3}
            let sampling_coins : usize = 2;
            let expected_variance : f64 = (sampling_coins as f64) / 2.0;

            let mut mean : f64 = 0.0;
            let mut variance : f64 = 0.0;

            let ring_element_1 = sample_poly_cbd::<128, 1024>(sampling_coins, &randomness[0..sampling_coins * 64].try_into().unwrap());
            let ring_element_2 = sample_poly_cbd::<128, 1024>(sampling_coins, &randomness[sampling_coins * 64..].try_into().unwrap());

            let all_coefficients = ring_element_1
                                   .into_iter()
                                   .chain(ring_element_2.into_iter())
                                   .collect::<Vec<FieldElement>>();

            for coefficient in &all_coefficients {
                mean += field_coefficient_to_binomial_sample(sampling_coins, coefficient.val);
            }
            mean /= all_coefficients.len() as f64;

            for coefficient in &all_coefficients {
                let binomial_sample = field_coefficient_to_binomial_sample(sampling_coins, coefficient.val);
                variance += (binomial_sample - mean) * (binomial_sample - mean);
            }
            variance /= (all_coefficients.len() - 1) as f64;

            assert!(mean < 0.3, "The mean is {}.", mean); // The expected mean is 0
            assert!(percentage_error(variance, expected_variance) < 26.0, "The variance is {}.", variance);
        }
    }
}
