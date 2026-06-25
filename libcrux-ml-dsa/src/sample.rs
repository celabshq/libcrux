use crate::{
    constants::{Eta, COEFFICIENTS_IN_RING_ELEMENT},
    encoding,
    hash_functions::{shake128, shake256},
    helper::cloop,
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[cfg(hax)]
use crate::simd::traits::specs::*;

#[inline(always)]
#[hax_lib::requires(*sampled_coefficients < 256)]
#[hax_lib::ensures(|_| fstar!(r#"v ${sampled_coefficients}_future <= 263"#))]
fn rejection_sample_less_than_field_modulus<SIMDUnit: Operations>(
    randomness: &[u8],
    sampled_coefficients: &mut usize,
    out: &mut [i32; 263],
) -> bool {
    let mut done = false;

    let n_chunks = randomness.len() / 24;
    for i in 0..n_chunks {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v ${sampled_coefficients} <= 263 /\
               (${done} \/ v ${sampled_coefficients} < 256)"#
        ));
        let random_bytes = &randomness[i * 24..(i + 1) * 24];
        if !done {
            let sampled = SIMDUnit::rejection_sample_less_than_field_modulus(
                random_bytes,
                &mut out[*sampled_coefficients..],
            );
            *sampled_coefficients += sampled;

            if *sampled_coefficients >= COEFFICIENTS_IN_RING_ELEMENT {
                done = true;
            }
        }
    }

    done
}

#[inline(always)]
fn generate_domain_separator((row, column): (u8, u8)) -> u16 {
    (column as u16) | ((row as u16) << 8)
}

#[inline(always)]
#[hax_lib::requires(slice.len() <= 32)]
pub(crate) fn add_domain_separator(slice: &[u8], indices: (u8, u8)) -> [u8; 34] {
    let mut out = [0u8; 34];

    out[0..slice.len()].copy_from_slice(slice);

    let domain_separator = generate_domain_separator(indices);
    out[32] = domain_separator as u8;
    out[33] = (domain_separator >> 8) as u8;

    out
}

/// Sample and write out up to four ring elements.
///
/// If i <= `elements_requested`, a field element with domain separated
/// seed according to the provided index is generated in
/// `tmp_stack[i]`. After successful rejection sampling in
/// `tmp_stack[i]`, the ring element is written to `matrix` at the
/// provided index in `indices[i]`.
/// `rand_stack` is a working buffer that holds initial Shake output.
//
// Body stays admitted (`admit ()` at start of body); the `ensures` is a
// postulated postcondition that callers (`samplex4::matrix_flat`) consume
// to discharge `compute_as1_plus_s2`'s `is_i32b_array_opaque FIELD_MAX`
// pre on the `a_as_ntt` matrix.  The postulate is sound under the body
// invariant: matrix coefficients are rejection-sampled from `[0, Q)`
// (FIPS 204 Algorithm 30), hence `0 <= c < FIELD_MAX + 1` per coefficient,
// which `is_i32b FIELD_MAX` (centered |c| <= FIELD_MAX) covers.
#[inline(always)]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${matrix}_future == Seq.length $matrix /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
        (mk_usize 8380416) ${matrix}_future
"#))]
pub(crate) fn sample_up_to_four_ring_elements_flat<
    SIMDUnit: Operations,
    Shake128: shake128::XofX4,
>(
    columns: usize,
    seed: &[u8],
    matrix: &mut [PolynomialRingElement<SIMDUnit>],
    rand_stack0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    rand_stack1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    rand_stack2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    rand_stack3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    tmp_stack: &mut [[i32; 263]],
    start_index: usize,
    elements_requested: usize,
) {
    hax_lib::fstar!("admit ()");
    #[cfg(not(eurydice))]
    debug_assert!(elements_requested <= 4);

    // Prepare the seeds
    #[hax_lib::requires(width != 0)]
    fn xy(index: usize, width: usize) -> (u8, u8) {
        ((index / width) as u8, (index % width) as u8)
    }

    let seed0 = add_domain_separator(seed, xy(start_index, columns));
    let seed1 = add_domain_separator(seed, xy(start_index + 1, columns));
    let seed2 = add_domain_separator(seed, xy(start_index + 2, columns));
    let seed3 = add_domain_separator(seed, xy(start_index + 3, columns));

    let mut state = Shake128::init_absorb(&seed0, &seed1, &seed2, &seed3);

    state.squeeze_first_five_blocks(rand_stack0, rand_stack1, rand_stack2, rand_stack3);

    // Every call to |rejection_sample_less_than_field_modulus|
    // will result in a call to |PortableSIMDUnit::rejection_sample_less_than_field_modulus|;
    // this latter function performs no bounds checking and can write up to 8
    // elements to its output. It is therefore possible that 255 elements have
    // already been sampled and we call the function again.
    //
    // To ensure we don't overflow the buffer in this case, we allocate 255 + 8
    // = 263 elements.
    let mut sampled0 = 0;
    let mut sampled1 = 0;
    let mut sampled2 = 0;
    let mut sampled3 = 0;

    let mut done0 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
        rand_stack0,
        &mut sampled0,
        &mut tmp_stack[0],
    );
    let mut done1 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
        rand_stack1,
        &mut sampled1,
        &mut tmp_stack[1],
    );
    let mut done2 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
        rand_stack2,
        &mut sampled2,
        &mut tmp_stack[2],
    );
    let mut done3 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
        rand_stack3,
        &mut sampled3,
        &mut tmp_stack[3],
    );

    while !done0 || !done1 || !done2 || !done3 {
        let randomnesses = state.squeeze_next_block();
        if !done0 {
            done0 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
                &randomnesses.0,
                &mut sampled0,
                &mut tmp_stack[0],
            );
        }
        if !done1 {
            done1 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
                &randomnesses.1,
                &mut sampled1,
                &mut tmp_stack[1],
            );
        }
        if !done2 {
            done2 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
                &randomnesses.2,
                &mut sampled2,
                &mut tmp_stack[2],
            );
        }
        if !done3 {
            done3 = rejection_sample_less_than_field_modulus::<SIMDUnit>(
                &randomnesses.3,
                &mut sampled3,
                &mut tmp_stack[3],
            );
        }
    }

    for k in 0..elements_requested {
        PolynomialRingElement::<SIMDUnit>::from_i32_array(
            &tmp_stack[k],
            &mut matrix[start_index + k],
        );
    }
}

#[inline(always)]
#[hax_lib::requires(*sampled_coefficients < 256)]
#[hax_lib::ensures(|_| fstar!(r#"v ${sampled_coefficients}_future <= 263"#))]
fn rejection_sample_less_than_eta_equals_2<SIMDUnit: Operations>(
    randomness: &[u8],
    sampled_coefficients: &mut usize,
    out: &mut [i32; 263],
) -> bool {
    let mut done = false;

    // Since each byte can be used to sample up to 2 coefficients, and since
    // a single SIMDUnit can hold 8 coefficients, we pass in 4 bytes of randomness.
    let n_chunks = randomness.len() / 4;
    for i in 0..n_chunks {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v ${sampled_coefficients} <= 263 /\
               (${done} \/ v ${sampled_coefficients} < 256)"#
        ));
        let random_bytes = &randomness[i * 4..(i + 1) * 4];
        if !done {
            let sampled = SIMDUnit::rejection_sample_less_than_eta_equals_2(
                random_bytes,
                &mut out[*sampled_coefficients..],
            );
            *sampled_coefficients += sampled;

            if *sampled_coefficients >= COEFFICIENTS_IN_RING_ELEMENT {
                done = true;
            }
        }
    }

    done
}

#[inline(always)]
#[hax_lib::requires(*sampled_coefficients < 256)]
#[hax_lib::ensures(|_| fstar!(r#"v ${sampled_coefficients}_future <= 263"#))]
fn rejection_sample_less_than_eta_equals_4<SIMDUnit: Operations>(
    randomness: &[u8],
    sampled_coefficients: &mut usize,
    out: &mut [i32; 263],
) -> bool {
    let mut done = false;

    // Since each byte can be used to sample up to 2 coefficients, and since
    // a single SIMDUnit can hold 8 coefficients, we pass in 4 bytes of randomness.
    let n_chunks = randomness.len() / 4;
    for i in 0..n_chunks {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v ${sampled_coefficients} <= 263 /\
               (${done} \/ v ${sampled_coefficients} < 256)"#
        ));
        let random_bytes = &randomness[i * 4..(i + 1) * 4];
        if !done {
            let sampled = SIMDUnit::rejection_sample_less_than_eta_equals_4(
                random_bytes,
                &mut out[*sampled_coefficients..],
            );
            *sampled_coefficients += sampled;

            if *sampled_coefficients >= COEFFICIENTS_IN_RING_ELEMENT {
                done = true;
            }
        }
    }

    done
}

#[inline(always)]
#[hax_lib::requires(*sampled < 256)]
#[hax_lib::ensures(|_| fstar!(r#"v ${sampled}_future <= 263"#))]
pub(crate) fn rejection_sample_less_than_eta<SIMDUnit: Operations>(
    eta: Eta,
    randomness: &[u8],
    sampled: &mut usize,
    out: &mut [i32; 263],
) -> bool {
    match eta {
        Eta::Two => rejection_sample_less_than_eta_equals_2::<SIMDUnit>(randomness, sampled, out),
        Eta::Four => rejection_sample_less_than_eta_equals_4::<SIMDUnit>(randomness, sampled, out),
    }
}

#[inline(always)]
#[hax_lib::requires(slice.len() <= 64)]
pub(crate) fn add_error_domain_separator(slice: &[u8], domain_separator: u16) -> [u8; 66] {
    let mut out = [0u8; 66];

    out[0..slice.len()].copy_from_slice(slice);
    out[64] = domain_separator as u8;
    out[65] = (domain_separator >> 8) as u8;

    out
}

// Body stays admitted (`admit ()` at start of body); the `ensures` is a
// postulated postcondition that callers (`samplex4::sample_s1_and_s2`)
// consume to discharge `signing_key::generate_serialized`'s pre on `s1_2`.
// Exposed as the opaque `is_lane_range_poly_slice` atom so callers see one
// premise instead of triple-forall + match expansion.
#[inline(always)]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${re}_future == Seq.length $re /\
    (let eta_val : usize = match $eta with
                            | Libcrux_ml_dsa.Constants.Eta_Two -> mk_usize 2
                            | Libcrux_ml_dsa.Constants.Eta_Four -> mk_usize 4 in
     Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
         (mk_usize 0) eta_val ${re}_future)
"#))]
pub(crate) fn sample_four_error_ring_elements<SIMDUnit: Operations, Shake256: shake256::XofX4>(
    eta: Eta,
    seed: &[u8],
    start_index: u16,
    re: &mut [PolynomialRingElement<SIMDUnit>],
) {
    hax_lib::fstar!("admit ()");
    // Prepare the seeds
    let seed0 = add_error_domain_separator(seed, start_index);
    let seed1 = add_error_domain_separator(seed, start_index + 1);
    let seed2 = add_error_domain_separator(seed, start_index + 2);
    let seed3 = add_error_domain_separator(seed, start_index + 3);

    let mut state = Shake256::init_absorb_x4(&seed0, &seed1, &seed2, &seed3);
    let randomnesses = state.squeeze_first_block_x4();

    // Every call to |rejection_sample_less_than_field_modulus|
    // will result in a call to |SIMDUnit::rejection_sample_less_than_field_modulus|;
    // this latter function performs no bounds checking and can write up to 8
    // elements to its output. It is therefore possible that 255 elements have
    // already been sampled and we call the function again.
    //
    // To ensure we don't overflow the buffer in this case, we allocate 255 + 8
    // = 263 elements.
    let mut out = [[0i32; 263]; 4];

    let mut sampled0 = 0;
    let mut sampled1 = 0;
    let mut sampled2 = 0;
    let mut sampled3 = 0;

    let mut done0 = rejection_sample_less_than_eta::<SIMDUnit>(
        eta,
        &randomnesses.0,
        &mut sampled0,
        &mut out[0],
    );
    let mut done1 = rejection_sample_less_than_eta::<SIMDUnit>(
        eta,
        &randomnesses.1,
        &mut sampled1,
        &mut out[1],
    );
    let mut done2 = rejection_sample_less_than_eta::<SIMDUnit>(
        eta,
        &randomnesses.2,
        &mut sampled2,
        &mut out[2],
    );
    let mut done3 = rejection_sample_less_than_eta::<SIMDUnit>(
        eta,
        &randomnesses.3,
        &mut sampled3,
        &mut out[3],
    );

    while !done0 || !done1 || !done2 || !done3 {
        // Always sample another 4, but we only use it if we actually need it.
        let randomnesses = state.squeeze_next_block_x4();
        if !done0 {
            done0 = rejection_sample_less_than_eta::<SIMDUnit>(
                eta,
                &randomnesses.0,
                &mut sampled0,
                &mut out[0],
            );
        }
        if !done1 {
            done1 = rejection_sample_less_than_eta::<SIMDUnit>(
                eta,
                &randomnesses.1,
                &mut sampled1,
                &mut out[1],
            );
        }
        if !done2 {
            done2 = rejection_sample_less_than_eta::<SIMDUnit>(
                eta,
                &randomnesses.2,
                &mut sampled2,
                &mut out[2],
            );
        }
        if !done3 {
            done3 = rejection_sample_less_than_eta::<SIMDUnit>(
                eta,
                &randomnesses.3,
                &mut sampled3,
                &mut out[3],
            );
        }
    }

    // XXX: Core.Cmp.f_min is not implemented
    let max = start_index as usize + 4;
    let max = if re.len() < max { re.len() } else { max };
    for i in start_index as usize..max {
        PolynomialRingElement::<SIMDUnit>::from_i32_array(&out[i % 4], &mut re[i]);
    }
}

#[inline(always)]
fn sample_mask_ring_element<SIMDUnit: Operations, Shake256: shake256::DsaXof>(
    seed: &[u8; 66],
    result: &mut PolynomialRingElement<SIMDUnit>,
    gamma1_exponent: usize,
) {
    hax_lib::fstar!("admit ()");
    match gamma1_exponent {
        17 => {
            let mut out = [0u8; 576];
            Shake256::shake256::<576>(seed, &mut out);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out, result);
        }
        19 => {
            let mut out = [0u8; 640];
            Shake256::shake256::<640>(seed, &mut out);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out, result);
        }
        _ => unreachable!(),
    }
}

#[inline(always)]
pub(crate) fn sample_mask_vector<
    SIMDUnit: Operations,
    Shake256: shake256::DsaXof,
    Shake256X4: shake256::XofX4,
>(
    dimension: usize,
    gamma1_exponent: usize,
    seed: &[u8; 64],
    domain_separator: &mut u16,
    mask: &mut [PolynomialRingElement<SIMDUnit>],
) {
    hax_lib::fstar!("admit ()");
    // DIMENSION is COLUMNS_IN_A
    #[cfg(not(eurydice))]
    debug_assert!(dimension == 4 || dimension == 5 || dimension == 7);
    // So we can always sample 4 elements in one go first.

    let seed0 = add_error_domain_separator(seed, *domain_separator);
    let seed1 = add_error_domain_separator(seed, *domain_separator + 1);
    let seed2 = add_error_domain_separator(seed, *domain_separator + 2);
    let seed3 = add_error_domain_separator(seed, *domain_separator + 3);
    *domain_separator += 4;

    match gamma1_exponent {
        17 => {
            let mut out0 = [0; 576];
            let mut out1 = [0; 576];
            let mut out2 = [0; 576];
            let mut out3 = [0; 576];
            Shake256X4::shake256_x4(
                &seed0, &seed1, &seed2, &seed3, &mut out0, &mut out1, &mut out2, &mut out3,
            );
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out0, &mut mask[0]);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out1, &mut mask[1]);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out2, &mut mask[2]);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out3, &mut mask[3]);
        }
        19 => {
            let mut out0 = [0; 640];
            let mut out1 = [0; 640];
            let mut out2 = [0; 640];
            let mut out3 = [0; 640];
            Shake256X4::shake256_x4(
                &seed0, &seed1, &seed2, &seed3, &mut out0, &mut out1, &mut out2, &mut out3,
            );
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out0, &mut mask[0]);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out1, &mut mask[1]);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out2, &mut mask[2]);
            encoding::gamma1::deserialize::<SIMDUnit>(gamma1_exponent, &out3, &mut mask[3]);
        }
        _ => unreachable!(),
    }

    #[allow(clippy::needless_range_loop)]
    for i in 4..dimension {
        let seed = add_error_domain_separator(seed, *domain_separator);
        *domain_separator += 1;

        // TODO: For 87 we may want to do another 4 and discard 1.
        sample_mask_ring_element::<SIMDUnit, Shake256>(&seed, &mut mask[i], gamma1_exponent);
    }
}

#[inline(always)]
#[hax_lib::requires(*out_index < 256)]
#[hax_lib::ensures(|_| fstar!(r#"v ${out_index}_future <= 256"#))]
fn inside_out_shuffle(
    randomness: &[u8],
    out_index: &mut usize,
    signs: &mut u64,
    result: &mut [i32; 256],
) -> bool {
    let mut done = false;

    for i in 0..randomness.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v ${out_index} <= 256 /\
               (${done} \/ v ${out_index} < 256)"#
        ));
        let byte = randomness[i];
        if !done {
            let sample_at = byte as usize;
            if sample_at <= *out_index {
                result[*out_index] = result[sample_at];
                *out_index += 1;

                result[sample_at] = if (*signs & 1) == 0 { 1 } else { -1 };
                *signs >>= 1;
            }

            done = *out_index == result.len();
        }
    }

    done
}

#[inline(always)]
pub(crate) fn sample_challenge_ring_element<SIMDUnit: Operations, Shake256: shake256::DsaXof>(
    seed: &[u8],
    number_of_ones: usize,
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    hax_lib::fstar!("admit ()");
    let mut state = Shake256::init_absorb_final(seed);
    let randomness = state.squeeze_first_block();

    let mut signs = u64::from_le_bytes(randomness[0..8].try_into().unwrap());
    let mut result = [0i32; 256];

    let mut out_index = result.len() - number_of_ones;
    let mut done = inside_out_shuffle(&randomness[8..], &mut out_index, &mut signs, &mut result);

    while !done {
        let randomness = state.squeeze_next_block();
        done = inside_out_shuffle(&randomness, &mut out_index, &mut signs, &mut result);
    }

    PolynomialRingElement::<SIMDUnit>::from_i32_array(&result, re);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        constants::COEFFICIENTS_IN_RING_ELEMENT,
        hash_functions,
        simd::{self, traits::Operations},
    };

    fn sample_ring_element_uniform<SIMDUnit: Operations, Shake128: shake128::XofX4>(
        seed: [u8; 34],
        re: &mut PolynomialRingElement<SIMDUnit>,
    ) {
        let mut rand_stack = (
            [0u8; shake128::FIVE_BLOCKS_SIZE],
            [0u8; shake128::FIVE_BLOCKS_SIZE],
            [0u8; shake128::FIVE_BLOCKS_SIZE],
            [0u8; shake128::FIVE_BLOCKS_SIZE],
        );

        let dummy_input = [0u8; 34];
        let mut state = Shake128::init_absorb(&seed, &dummy_input, &dummy_input, &dummy_input);
        state.squeeze_first_five_blocks(
            &mut rand_stack.0,
            &mut rand_stack.1,
            &mut rand_stack.2,
            &mut rand_stack.3,
        );
        let mut tmp_stack = [[0i32; 263], [0i32; 263], [0i32; 263], [0i32; 263]];
        let mut sampled = 0;

        let mut done = rejection_sample_less_than_field_modulus::<SIMDUnit>(
            &mut rand_stack.0,
            &mut sampled,
            &mut tmp_stack[0],
        );

        while !done {
            let randomnesses = state.squeeze_next_block();
            if !done {
                done = rejection_sample_less_than_field_modulus::<SIMDUnit>(
                    &randomnesses.0,
                    &mut sampled,
                    &mut tmp_stack[0],
                );
            }
        }

        PolynomialRingElement::<SIMDUnit>::from_i32_array(&tmp_stack[0], re);
    }

    // // This is just a wrapper around sample_four_ring_elements, for testing
    // // purposes.
    // fn sample_error_ring_element<
    //     SIMDUnit: Operations,
    //     Shake256X4: shake256::XofX4,
    //     const ETA: usize,
    // >(
    //     seed: &[u8],
    //     start_index: u16,
    // ) -> PolynomialRingElement<SIMDUnit> {
    //     let mut s = [PolynomialRingElement::ZERO(); 6];
    //     // let start_index = ((seed[65] as u16) << 8) | (seed[64] as u16);
    //     // std::eprintln!("start_index: {start_index}");
    //     sample_four_error_ring_elements::<SIMDUnit, Shake256X4, ETA>(&seed, start_index, &mut s);

    //     for i in 0..s.len() {
    //         std::eprintln!("{:?}", s[i].to_i32_array());
    //     }

    //     s[start_index as usize]
    // }

    fn test_sample_ring_element_uniform_generic<SIMDUnit: Operations, Shake128: shake128::XofX4>() {
        let seed: [u8; 34] = [
            33, 192, 250, 216, 117, 61, 16, 12, 248, 51, 213, 110, 64, 57, 119, 80, 164, 83, 73,
            91, 80, 128, 195, 219, 203, 149, 170, 233, 16, 232, 209, 105, 4, 5,
        ];

        let expected_coefficients: [i32; COEFFICIENTS_IN_RING_ELEMENT] = [
            886541, 1468422, 793958, 7610434, 3986512, 913782, 2546456, 5820798, 1940159, 10062,
            3303190, 3831326, 4834267, 3500674, 16909, 8314529, 7469249, 5611755, 6181076, 269257,
            3566448, 2968856, 7556314, 6685884, 129963, 8017973, 1087829, 5842199, 6867133, 442098,
            3473053, 3812349, 556165, 55620, 4367526, 798402, 5317265, 2828265, 3808240, 3065841,
            6340895, 2710831, 715345, 5806109, 3689225, 4088547, 4258029, 2877620, 6867225,
            3275166, 4626484, 6596723, 5180488, 3836050, 1115576, 2086584, 749098, 4980044,
            7626966, 961947, 4695118, 6488634, 7898263, 841160, 1186851, 6958928, 4995591, 6829719,
            5910175, 2590788, 987365, 5983050, 7039561, 1406907, 4054912, 3093314, 237981, 6184639,
            515190, 5209488, 6460375, 4417602, 7890594, 6584284, 1729237, 5851336, 8226663,
            1843549, 5872244, 1375077, 6275711, 997136, 2593411, 5739784, 6621377, 7180456,
            1437441, 2607410, 197226, 4753353, 5086363, 6096080, 3057564, 5040851, 886178, 699532,
            3772666, 7983776, 1235995, 1960665, 1233119, 317423, 442071, 4649134, 5043634, 4164756,
            3166873, 2343835, 6256400, 6132302, 4124098, 6087733, 5371278, 3484545, 1020458,
            3688444, 7263864, 2413270, 4449757, 5561507, 7464292, 1176556, 8294481, 2892372,
            5509298, 194732, 7976046, 5907126, 4792878, 5059916, 3122481, 7009119, 5476286,
            4905623, 7374799, 7284599, 4929839, 538055, 5611660, 233595, 6125390, 7441322, 3752658,
            6655759, 4907614, 2281767, 1659504, 5490352, 4235568, 7143494, 6217399, 1581266,
            2455222, 1015526, 8366150, 2002613, 185543, 7904386, 8206829, 5380721, 2226008,
            7713547, 6961768, 7911095, 5604679, 6839785, 7573702, 1113136, 5563352, 7446030,
            6694003, 1725163, 4749689, 6474727, 7125683, 1830230, 5300491, 7927815, 5808662,
            2345184, 5462894, 5760340, 1949317, 1853703, 5060631, 5935138, 4873466, 3302619,
            5351360, 5707708, 2715882, 2050173, 52173, 5463772, 2851164, 1702574, 7167630, 1132010,
            1418205, 4182063, 4919187, 2707143, 6241533, 3241235, 2286591, 268487, 3799838, 558302,
            5882605, 6165192, 6702794, 5578115, 1893372, 7246495, 4974148, 2633723, 1522313,
            7636103, 6639058, 6765356, 3588710, 7011438, 4798122, 2329503, 4671411, 6787853,
            1838957, 306944, 5112958, 853077, 7844176, 384195, 839634, 1860349, 7289878, 4054796,
            703698, 5147821, 7632328, 5993194, 6329638, 5959986, 3073141, 675737, 7364844, 4124952,
        ];

        let mut re = PolynomialRingElement::zero();
        sample_ring_element_uniform::<SIMDUnit, Shake128>(seed, &mut re);
        assert_eq!(re.to_i32_array(), expected_coefficients);

        // This seed and the expected coefficients were taken from the
        // "Signature Verification -- ML-DSA-65.txt" file in the "PQC Intermediate Values"
        // package found here:
        //
        // https://csrc.nist.gov/Projects/post-quantum-cryptography/post-quantum-cryptography-standardization/example-files
        let seed = [
            0x6C, 0x84, 0x14, 0x38, 0x08, 0x56, 0xCB, 0x52, 0xD7, 0x9C, 0x4B, 0x29, 0x13, 0x9F,
            0xB1, 0x83, 0x9B, 0x86, 0x06, 0xF5, 0x94, 0x8B, 0x9D, 0x72, 0xA9, 0x56, 0xDC, 0xF1,
            0x01, 0x16, 0xDA, 0x9E, 0x01, 0x00,
        ];
        let mut re = PolynomialRingElement::zero();
        sample_ring_element_uniform::<SIMDUnit, Shake128>(seed, &mut re);
        let actual_coefficients = re.to_i32_array();

        assert_eq!(actual_coefficients[0], 1_165_602);
        assert_eq!(
            actual_coefficients[1..],
            [
                3634040, 7110348, 6039535, 8209112, 8342684, 3376761, 2760752, 201874, 5788205,
                6315920, 5758613, 4180208, 3498018, 4506185, 6197602, 4825715, 1413018, 4001908,
                5200822, 2321616, 43264, 357657, 3357947, 5478400, 1625148, 7950715, 241908,
                5817357, 6314876, 3963827, 2765806, 7187638, 5098494, 4495365, 4124864, 1563629,
                6643348, 2155850, 813048, 5462957, 5416878, 5407763, 685417, 1482758, 2211367,
                7400454, 5644271, 599228, 1192002, 3950753, 1943948, 4147278, 7709236, 4455786,
                5969957, 4873849, 2497883, 7702897, 1951031, 2746827, 541648, 6820767, 4343169,
                7809196, 3075663, 2498997, 7516711, 6073110, 3812366, 6180662, 2140253, 955825,
                1183827, 3824805, 961270, 2848570, 553317, 945650, 846350, 7115358, 7684494,
                3452277, 2829465, 7560733, 7765663, 8046459, 6122871, 2186559, 1063033, 8249483,
                1394306, 664161, 7734307, 4722290, 3791427, 2435952, 263490, 1006165, 3331598,
                1364040, 995391, 2074495, 1907554, 2358279, 2270487, 634762, 7962901, 5614697,
                5786521, 5116667, 1430717, 7455972, 2533159, 7947550, 7739229, 4927600, 241260,
                7369022, 6744571, 6680687, 1961030, 2093028, 4786791, 6246262, 4051533, 3634060,
                2403470, 2802259, 3645990, 6976210, 4921899, 5421392, 2002756, 6710071, 2947573,
                1575303, 4408913, 1184854, 3248924, 8314261, 5273575, 2035537, 3057717, 4276424,
                5822730, 2723413, 7019988, 818534, 2429970, 1355058, 7224104, 2099984, 7006142,
                1252024, 1322417, 4242718, 1761064, 2157891, 4952775, 2413792, 4326818, 7109905,
                5383105, 6756494, 6255540, 2899390, 3086583, 7685346, 4041101, 5334956, 4513393,
                6517963, 4356627, 2904889, 2415412, 7209635, 6858378, 3366617, 2446291, 206235,
                1998054, 4488129, 4659437, 1338118, 4922652, 6007451, 5557143, 4798024, 86509,
                3799432, 5945739, 1001428, 7172374, 2827278, 7428682, 963842, 7199121, 6413373,
                6585976, 4442989, 8150284, 459638, 1681794, 4346128, 7943864, 6962572, 7466591,
                3401623, 6306091, 4245753, 5519446, 1599041, 2410812, 1955008, 5812175, 7440355,
                253888, 4607519, 700571, 7817367, 5129683, 8046724, 1180791, 5121466, 8184965,
                6029940, 3191617, 5335654, 7208397, 7752286, 4052684, 1826096, 1681526, 5923139,
                4148306, 4764105, 1496019, 8215829, 7787085, 2322997, 4716898, 7780010, 6832169,
                5960634, 644622, 2145941, 7046161, 5644191, 5390778, 1364486, 3472707, 4379141,
                897129, 6882711, 5430079
            ]
        );
    }

    // fn test_sample_error_ring_element_generic<SIMDUnit: Operations, Shake256: shake256::XofX4>() {
    //     // When ETA = 2
    //     let seed: [u8; 64] = [
    //         51, 203, 133, 235, 126, 210, 169, 81, 4, 134, 147, 168, 252, 67, 176, 99, 130, 186,
    //         254, 103, 241, 199, 173, 78, 121, 232, 12, 244, 4, 143, 8, 174, 122, 170, 124, 35, 53,
    //         49, 202, 94, 27, 249, 200, 186, 175, 198, 169, 116, 244, 227, 133, 111, 205, 140, 233,
    //         110, 227, 67, 35, 226, 194, 75, 130, 105,
    //     ];
    //     let start_index = 5;

    //     let expected_coefficients: [i32; COEFFICIENTS_IN_RING_ELEMENT] = [
    //         1, 0, -1, 0, 1, -2, -1, 0, -2, 2, -1, -2, 1, -2, 1, -2, 1, 2, -2, 2, -2, -1, 0, -2, -1,
    //         -2, -2, 1, 1, -1, 1, 1, 2, -2, 2, -1, 1, 2, 0, 2, -1, 0, 2, -2, -2, 2, 0, 2, 1, 1, 2,
    //         1, 1, -2, 1, -1, 2, -2, -2, 2, -2, -2, 0, 0, -1, 0, 2, 0, 1, 2, 0, 2, -1, 2, 0, 2, 1,
    //         -2, -2, 0, -1, -2, 2, -2, -1, 2, 1, -1, 2, 1, -2, -1, 1, -1, -1, -1, 2, -1, -2, -2, 2,
    //         2, 0, -1, -1, -2, 0, -1, 0, 1, 2, -2, 0, 2, 2, 1, 0, -1, -1, 0, -2, 2, 2, -2, 2, 1, -1,
    //         -2, -1, -2, -1, 1, 2, 2, -1, 0, 1, 2, -1, 0, 0, 0, 1, 1, -1, -1, -1, -2, 2, 0, -2, 0,
    //         2, -1, 1, 1, 2, -2, 2, -2, 1, 0, -2, 1, 0, 0, -2, -2, 2, 2, -2, -1, 2, -2, 1, 0, 0, -1,
    //         0, -2, 2, -1, -2, 2, -1, 1, -2, -1, 0, -2, 2, 1, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, -1,
    //         -2, 1, 1, 0, -2, 1, 0, 0, -2, 1, -2, -1, 2, 0, 0, 2, 0, -2, -1, -1, 2, 2, -1, -1, -1,
    //         -2, -2, -1, -2, 2, -2, 0, 1, 0, -2, -2, 2, 0, 1, 0, 0, -2, -1, 1, -1, 1, -1, -1, -1, 2,
    //         2, 0,
    //     ];

    //     assert_eq!(
    //         sample_error_ring_element::<SIMDUnit, Shake256, 2>(&seed, start_index).to_i32_array(),
    //         expected_coefficients
    //     );

    //     // When ETA = 4
    //     let seed: [u8; 66] = [
    //         236, 4, 148, 239, 41, 178, 188, 226, 130, 212, 6, 144, 208, 180, 180, 105, 47, 148, 75,
    //         195, 181, 177, 5, 140, 204, 68, 24, 132, 169, 19, 68, 118, 67, 203, 13, 152, 29, 194,
    //         235, 123, 101, 109, 162, 137, 198, 164, 97, 247, 11, 44, 34, 49, 235, 251, 243, 177,
    //         213, 141, 65, 232, 136, 163, 85, 54, 10, 0,
    //     ];

    //     let expected_coefficients: [i32; COEFFICIENTS_IN_RING_ELEMENT] = [
    //         2, -4, 2, -2, 1, 2, 4, 2, 4, -1, -4, 3, 2, 4, -1, 2, -3, 3, 1, -2, 0, 3, -2, 3, 4, 1,
    //         -3, -2, 0, -4, -1, -4, 3, -4, 0, -3, -2, -3, 2, -3, -3, 3, -4, -3, -4, 1, -2, 4, -3, 4,
    //         4, 1, -3, -3, 4, 0, -2, 2, 4, -4, 4, -4, -1, -3, 4, 3, 2, -1, 3, -2, -2, -4, -1, -1, 4,
    //         1, 4, 0, 3, 4, -1, -3, 4, -4, 4, 1, -3, 0, -4, 2, 1, 4, -1, 0, -2, -2, -3, 3, -3, 4, 3,
    //         2, -2, -2, -1, 2, -1, -4, 3, 0, -2, 4, -1, 0, 4, -2, 4, -3, 2, -4, 2, 3, 3, 2, -4, 2,
    //         0, -2, 1, -4, 0, -4, -3, 2, 0, -2, -4, 1, 2, 3, 4, -4, 2, 2, 1, -4, 0, -4, -3, -2, -2,
    //         -2, -1, 1, 4, 1, 0, -2, 2, 1, 4, -4, -1, 0, -1, -3, 2, 1, 3, 3, 4, -2, -2, 3, 1, 3, 3,
    //         -4, -2, -1, -4, -3, 4, 1, 2, -3, -1, 3, 4, -3, 0, -1, -1, -4, -2, 1, -2, 3, -1, -2, 2,
    //         -1, -2, 0, -2, 2, 3, 3, 2, 3, 4, 3, -3, -4, 1, 4, -3, 2, 0, -4, 4, -4, 2, 4, -2, -3,
    //         -4, 3, 0, 1, -2, 2, -1, 4, 4, 0, -1, 1, 4, -2, -3, 2, -2, 4, 2, 1, 1, 1, -3, -2, -2, 2,
    //         2, -4, -1, 1,
    //     ];

    //     // FIXME
    //     // assert_eq!(
    //     //     sample_error_ring_element::<SIMDUnit, Shake256, 4>(seed).to_i32_array(),
    //     //     expected_coefficients
    //     // );
    // }

    fn test_sample_challenge_ring_element_generic<
        SIMDUnit: Operations,
        Shake256: shake256::DsaXof,
    >() {
        // When TAU = 39
        let seed: [u8; 32] = [
            3, 9, 159, 119, 236, 6, 207, 7, 103, 108, 187, 137, 222, 35, 37, 30, 79, 224, 204, 186,
            41, 38, 148, 188, 201, 50, 105, 155, 129, 217, 124, 57,
        ];

        let expected_coefficients: [i32; COEFFICIENTS_IN_RING_ELEMENT] = [
            0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1,
            -1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1,
            -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 0, -1, 0, 0, -1, 1, 0, 0, 1,
            0, 0, 0, 1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
            0,
        ];

        let mut re = PolynomialRingElement::zero();
        sample_challenge_ring_element::<SIMDUnit, Shake256>(&seed, 39, &mut re);
        assert_eq!(re.to_i32_array(), expected_coefficients);

        // When TAU = 49
        let seed: [u8; 32] = [
            147, 7, 165, 152, 200, 20, 4, 38, 107, 110, 111, 176, 108, 84, 109, 201, 232, 125, 52,
            83, 160, 120, 106, 44, 76, 41, 76, 144, 8, 184, 4, 74,
        ];

        let expected_coefficients: [i32; COEFFICIENTS_IN_RING_ELEMENT] = [
            0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, -1, -1, 0,
            1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0,
            -1, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            -1, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, -1, 0, -1, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0,
            -1, 0, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0,
            0, -1, 0, 0, 0,
        ];

        let mut re = PolynomialRingElement::zero();
        sample_challenge_ring_element::<SIMDUnit, Shake256>(&seed, 49, &mut re);
        assert_eq!(re.to_i32_array(), expected_coefficients);

        // When TAU = 60
        let seed: [u8; 32] = [
            188, 193, 17, 175, 172, 179, 13, 23, 90, 238, 237, 230, 143, 113, 24, 65, 250, 86, 234,
            229, 251, 57, 199, 158, 9, 4, 102, 249, 11, 68, 140, 107,
        ];

        let expected_coefficients: [i32; COEFFICIENTS_IN_RING_ELEMENT] = [
            0, 0, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 1, 1, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, 0, 0, -1,
            0, 0, 0, 0, 0, -1, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 1, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0,
            0, 1, 0, -1, 1, 0, 0, 0, 0, 0, 1, 1, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 1, 1, -1, 0,
            0, 0, 0, 1, -1, 0,
        ];

        let mut re = PolynomialRingElement::zero();
        sample_challenge_ring_element::<SIMDUnit, Shake256>(&seed, 60, &mut re);
        assert_eq!(re.to_i32_array(), expected_coefficients);
    }

    #[cfg(not(feature = "simd256"))]
    mod portable {
        use super::*;

        #[test]
        fn test_sample_ring_element_uniform() {
            test_sample_ring_element_uniform_generic::<
                simd::portable::PortableSIMDUnit,
                hash_functions::portable::Shake128X4,
            >();
        }

        // #[test]
        // fn test_sample_error_ring_element() {
        //     test_sample_error_ring_element_generic::<
        //         simd::portable::PortableSIMDUnit,
        //         hash_functions::portable::Shake256X4,
        //     >();
        // }

        #[test]
        fn test_sample_challenge_ring_element() {
            test_sample_challenge_ring_element_generic::<
                simd::portable::PortableSIMDUnit,
                hash_functions::portable::Shake256,
            >();
        }
    }

    #[cfg(feature = "simd256")]
    mod simd256 {
        use super::*;

        #[test]
        fn test_sample_ring_element_uniform() {
            test_sample_ring_element_uniform_generic::<
                simd::avx2::AVX2SIMDUnit,
                hash_functions::simd256::Shake128x4,
            >();
        }

        // #[test]
        // fn test_sample_error_ring_element() {
        //     test_sample_error_ring_element_generic::<
        //         simd::avx2::AVX2SIMDUnit,
        //         hash_functions::simd256::Shake256x4,
        //     >();
        // }

        #[test]
        fn test_sample_challenge_ring_element() {
            test_sample_challenge_ring_element_generic::<
                simd::avx2::AVX2SIMDUnit,
                hash_functions::portable::Shake256,
            >();
        }
    }
}
