use crate::parameters::*;
use hax_lib::ToProp;

/// According to the NIST FIPS 203 standard (Page 10, Lines 536 - 539),
/// compressing a polynomial ring element is accomplished by `compress()`ing its
/// constituent field coefficients.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(bits_per_compressed_coefficient < 12)]
pub fn compress(re: Polynomial, bits_per_compressed_coefficient: usize) -> Polynomial {
    createi(|i| compress_d(re[i], bits_per_compressed_coefficient))
}

/// According to the NIST FIPS 203 standard (Page 10, Lines 536 - 539),
/// compressing a polynomial ring element is accomplished by `decompress()`ing
/// its constituent field coefficients.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires((bits_per_compressed_coefficient < 12).to_prop() &
        hax_lib::forall(|i: usize|
            hax_lib::implies(i < 256, re[i].val < (1u16 << bits_per_compressed_coefficient))))]
pub fn decompress(re: Polynomial, bits_per_compressed_coefficient: usize) -> Polynomial {
    createi(|i| decompress_d(re[i], bits_per_compressed_coefficient))
}

/// This function implements the `Compress` function specified in the NIST FIPS
/// 203 standard (Page 18, Expression 4.5), which is defined as:
///
/// ```plaintext
/// Compress_d: ℤq -> ℤ_{2ᵈ}
/// Compress_d(x) = ⌈(2ᵈ/q)·x⌋
/// ```
///
/// Since `⌈x⌋ = ⌊x + 1/2⌋` we have:
///
/// ```plaintext
/// Compress_d(x) = ⌊(2ᵈ/q)·x + 1/2⌋
///               = ⌊(2^{d+1}·x + q) / 2q⌋
/// ```
///
/// this latter expression is what the code computes, since it enables us to
/// avoid the use of floating point computations as required by the standard.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::fstar::options("--z3rlimit 1500")]
#[hax_lib::requires(to_bit_size < 12)]
fn compress_d(fe: FieldElement, to_bit_size: usize) -> FieldElement {
    hax_lib::debug_assert!(to_bit_size < 12);
    let two_pow_bit_size = 2u32.pow(to_bit_size as u32);

    let compressed =
        (fe.val as u32 * 2 * two_pow_bit_size + FIELD_MODULUS as u32) / (2 * FIELD_MODULUS as u32);

    FieldElement::new((compressed % two_pow_bit_size) as u16)
}

/// This function implements the `Decompress` function specified in the NIST FIPS
/// 203 standard (Page 18, Expression 4.6), which is defined as:
///
/// ```plaintext
/// Decompress_d: ℤ_{2ᵈ} -> ℤq
/// Decompress_d(y) = ⌈(q/2ᵈ)·y⌋
/// ```
///
/// Since `⌈x⌋ = ⌊x + 1/2⌋` we have:
///
/// ```plaintext
/// Decompress_d(y) = ⌊(q/2ᵈ)·y + 1/2⌋
///                 = ⌊(2·y·q + 2ᵈ) / 2^{d+1})⌋
/// ```
///
/// this latter expression is what the code computes, since it enables us to
/// avoid the use of floating point computations as required by the standard.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(to_bit_size < 12 && fe.val < (1u16 << to_bit_size))]
fn decompress_d(fe: FieldElement, to_bit_size: usize) -> FieldElement {
    hax_lib::debug_assert!(to_bit_size < 12 && fe.val < (1u16 << to_bit_size));
    let two_pow_bit_size = 2u32.pow(to_bit_size as u32);
    let numerator = 2 * fe.val as u32 * FIELD_MODULUS as u32 + two_pow_bit_size;
    let decompressed = numerator / (two_pow_bit_size * 2);
    FieldElement::new(decompressed as u16)
}

#[cfg(test)]
pub mod tests {
    use proptest::collection::vec;
    use proptest::prelude::*;

    use crate::{
        compress::{compress, decompress},
        parameters::*,
    };

    prop_compose! {
        fn arb_field_element(bit_size : usize) (
            representative in 0..FIELD_MODULUS) -> FieldElement {
                FieldElement::new(representative & ((1 << bit_size) - 1))
        }
    }

    prop_compose! {
        pub fn arb_ring_element(bits_per_coefficient : usize) (arb_ring_coefficients in vec(arb_field_element(bits_per_coefficient), COEFFICIENTS_IN_RING_ELEMENT)) -> Polynomial {
                createi(|i| arb_ring_coefficients[i])
        }
    }

    use crate::compress::{compress_d, decompress_d};

    #[test]
    fn compress_d_zero_maps_to_zero() {
        for d in 1..12 {
            assert_eq!(
                compress_d(FieldElement::new(0), d),
                FieldElement::new(0),
                "compress_d(0, {}) should be 0",
                d
            );
        }
    }

    #[test]
    fn decompress_d_zero_maps_to_zero() {
        for d in 1..12 {
            assert_eq!(
                decompress_d(FieldElement::new(0), d),
                FieldElement::new(0),
                "decompress_d(0, {}) should be 0",
                d
            );
        }
    }

    #[test]
    fn compress_d_known_values() {
        // d=1: compress_1(x) = round(2x/q) mod 2
        // Midpoint of [0, q-1] is ~1664, so values near q/2 map to 1
        assert_eq!(compress_d(FieldElement::new(0), 1), FieldElement::new(0));
        assert_eq!(compress_d(FieldElement::new(1664), 1), FieldElement::new(1));
        assert_eq!(compress_d(FieldElement::new(3328), 1), FieldElement::new(0)); // near q ≈ 0

        // d=4: compress_4(208) should be 1
        // decompress_4(1) = round(q/16) = round(208.06) = 208
        // So compress_4(208) = 1
        assert_eq!(compress_d(FieldElement::new(208), 4), FieldElement::new(1));
        // compress_4(1665) = round(16*1665/3329) = round(8.0024) = 8
        assert_eq!(compress_d(FieldElement::new(1665), 4), FieldElement::new(8));
    }

    #[test]
    fn decompress_d_known_values() {
        // decompress_d(y, d) = round(q·y / 2^d)
        // d=4, y=1: round(3329/16) = round(208.0625) = 208
        assert_eq!(
            decompress_d(FieldElement::new(1), 4),
            FieldElement::new(208)
        );
        // d=4, y=8: round(3329*8/16) = round(1664.5) = 1665
        assert_eq!(
            decompress_d(FieldElement::new(8), 4),
            FieldElement::new(1665)
        );
        // d=1, y=1: round(3329/2) = round(1664.5) = 1665
        assert_eq!(
            decompress_d(FieldElement::new(1), 1),
            FieldElement::new(1665)
        );
    }

    #[test]
    fn compress_d_output_in_range() {
        for d in 1..12 {
            let upper = 1u16 << d;
            for x in (0..FIELD_MODULUS).step_by(17) {
                let c = compress_d(FieldElement::new(x), d);
                assert!(
                    c.val < upper,
                    "compress_d({}, {}) = {} not in [0, {})",
                    x,
                    d,
                    c.val,
                    upper
                );
            }
        }
    }

    #[test]
    fn decompress_d_output_in_range() {
        for d in 1..12 {
            let upper = 1u16 << d;
            for y in 0..upper {
                let dec = decompress_d(FieldElement::new(y), d);
                assert!(
                    dec.val < FIELD_MODULUS,
                    "decompress_d({}, {}) = {} not in [0, q)",
                    y,
                    d,
                    dec.val
                );
            }
        }
    }

    #[test]
    fn compress_decompress_roundtrip_is_identity_on_decompressed() {
        // compress(decompress(y, d), d) should equal y for all y in [0, 2^d)
        for d in 1..12 {
            let upper = 1u16 << d;
            for y in 0..upper {
                let recovered = compress_d(decompress_d(FieldElement::new(y), d), d);
                assert_eq!(
                    recovered,
                    FieldElement::new(y),
                    "compress_d(decompress_d({}, {}), {}) = {:?} != {}",
                    y,
                    d,
                    d,
                    recovered,
                    y
                );
            }
        }
    }

    #[test]
    fn roundtrip_error_is_bounded() {
        // FIPS 203 guarantees: |decompress(compress(x)) - x| mod q ≤ B_q
        // where B_q = round(q / 2^{d+1})
        let q = FIELD_MODULUS as i32;
        for d in 1..12usize {
            let two_pow_d_plus_1 = 1u32 << (d + 1);
            let b_q = ((q as u32 + two_pow_d_plus_1 / 2) / two_pow_d_plus_1) as i32;

            for x in 0..FIELD_MODULUS {
                let roundtripped = decompress_d(compress_d(FieldElement::new(x), d), d);
                let mut error = (roundtripped.val as i32 - x as i32).rem_euclid(q);
                if error > q / 2 {
                    error = q - error;
                }
                assert!(
                    error <= b_q,
                    "d={}: |decompress(compress({})) - {}| = {} > B_q = {}",
                    d,
                    x,
                    x,
                    error,
                    b_q
                );
            }
        }
    }

    proptest! {
        #[test]
        fn compress_to_zero_bits(ring_element in arb_ring_element(12)) {
            let compressed = compress(ring_element, 0);
            for i in 0..256 {
                let coefficient = compressed[i];
                assert_eq!(coefficient, FieldElement::new(0));
            }
        }

        fn compress_and_decompress_are_inverses_when_no_compression(ring_element in arb_ring_element(12)) {
            let compressed = compress(ring_element, 12);
            let decompressed = decompress(compressed, 12);

            assert_eq!(compressed, decompressed);
        }
    }
}
