use crate::{
    constants::Eta,
    hash_functions::{shake128, shake256},
    helper::cloop,
    polynomial::PolynomialRingElement,
    sample::{sample_four_error_ring_elements, sample_up_to_four_ring_elements_flat},
    simd::traits::Operations,
};

#[cfg(hax)]
use crate::simd::traits::specs::*;

/// The x4 sampling implementation that is selected during multiplexing.
//
// `requires(true)` matches the `hash_functions` trait pattern: refines
// the extracted `f_matrix_flat_pre` to `Type0{true ==> pred}` so panic-
// free callers can discharge it.  The ensures combine length-preservation
// (so callers can rebind the mutated-via-&mut, returned-by-value-in-F*
// `matrix` back to a fixed-size array) with a per-coefficient FIELD_MAX
// bound (so `compute_as1_plus_s2`'s `a_as_ntt` precondition discharges
// from the trait method's post).  Class B Chain 1B (NTT-bound chain).
#[hax_lib::attributes]
pub(crate) trait X4Sampler {
    /// Sample the matrix A using platform specific implementation.
    #[requires(true)]
    #[ensures(|_| fstar!(r#"
        Seq.length ${matrix}_future == Seq.length $matrix /\
        (forall (k:nat). k < Seq.length ${matrix}_future ==>
            (forall (j:nat). j < 32 ==>
                Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                    (i1._super_i2.f_repr (Seq.index (Seq.index ${matrix}_future k).f_simd_units j))))
    "#))]
    fn matrix_flat<SIMDUnit: Operations>(
        columns: usize,
        seed: &[u8],
        matrix: &mut [PolynomialRingElement<SIMDUnit>],
    );
}

// Free-fn matrix_flat (called by every X4Sampler impl).  Body chains
// from `sample_up_to_four_ring_elements_flat`'s Class B Chain 1A ensures:
// each iteration writes 1-4 ring elements with all coefficients
// `is_i32b_array_opaque FIELD_MAX`.  Loop invariant tracks bound on the
// entire `matrix` slice (initial zero-fill is FIELD_MAX-bounded; each
// iteration's postulate covers the full slice).
#[inline(always)]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${matrix}_future == Seq.length $matrix /\
    (forall (k:nat). k < Seq.length ${matrix}_future ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index ${matrix}_future k).f_simd_units j))))
"#))]
pub(crate) fn matrix_flat<SIMDUnit: Operations, Shake128: shake128::XofX4>(
    columns: usize,
    seed: &[u8],
    matrix: &mut [PolynomialRingElement<SIMDUnit>],
) {
    let mut rand_stack0 = [0u8; shake128::FIVE_BLOCKS_SIZE];
    let mut rand_stack1 = [0u8; shake128::FIVE_BLOCKS_SIZE];
    let mut rand_stack2 = [0u8; shake128::FIVE_BLOCKS_SIZE];
    let mut rand_stack3 = [0u8; shake128::FIVE_BLOCKS_SIZE];
    let mut tmp_stack = [[0i32; 263], [0i32; 263], [0i32; 263], [0i32; 263]];

    cloop! {
        for start_index in (0..matrix.len()).step_by(4) {
            let elements_requested = if start_index + 4 <= matrix.len() {
                4
            } else {
                matrix.len() - start_index
            };
            sample_up_to_four_ring_elements_flat::<SIMDUnit, Shake128>(
                columns,
                seed,
                matrix,
                &mut rand_stack0,
                &mut rand_stack1,
                &mut rand_stack2,
                &mut rand_stack3,
                &mut tmp_stack,
                start_index,
                elements_requested,
            );
        }
    }
}

/// Portable sampling
pub(crate) mod portable {
    use super::*;

    pub(crate) struct PortableSampler {}
    impl X4Sampler for PortableSampler {
        fn matrix_flat<SIMDUnit: Operations>(
            columns: usize,
            seed: &[u8],
            matrix: &mut [PolynomialRingElement<SIMDUnit>],
        ) {
            matrix_flat::<SIMDUnit, crate::hash_functions::portable::Shake128X4>(
                columns, seed, matrix,
            )
        }
    }
}

/// Neon sampling
#[cfg(feature = "simd128")]
pub(crate) mod neon {
    use super::*;

    pub(crate) struct NeonSampler {}
    impl X4Sampler for NeonSampler {
        #[inline(always)]
        fn matrix_flat<SIMDUnit: Operations>(
            columns: usize,
            seed: &[u8],
            matrix: &mut [PolynomialRingElement<SIMDUnit>],
        ) {
            matrix_flat::<SIMDUnit, crate::hash_functions::neon::Shake128x4>(columns, seed, matrix)
        }
    }
}

/// AVX2 sampling
#[cfg(feature = "simd256")]
pub(crate) mod avx2 {
    use super::*;

    pub(crate) struct AVX2Sampler {}
    impl X4Sampler for AVX2Sampler {
        #[allow(unsafe_code)]
        fn matrix_flat<SIMDUnit: Operations>(
            columns: usize,
            seed: &[u8],
            matrix: &mut [PolynomialRingElement<SIMDUnit>],
        ) {
            #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
            #[allow(unsafe_code)]
            unsafe fn inner<SIMDUnit: Operations>(
                columns: usize,
                seed: &[u8],
                matrix: &mut [PolynomialRingElement<SIMDUnit>],
            ) {
                matrix_flat::<SIMDUnit, crate::hash_functions::simd256::Shake128x4>(
                    columns, seed, matrix,
                )
            }
            unsafe { inner(columns, seed, matrix) };
        }
    }
}

// Not inling this causes a 10x slow-down
#[inline(always)]
// Length-preserving + per-coefficient `is_pos_array_opaque eta` ensures
// (Class B Chain 1B).  Body chains from `sample_four_error_ring_elements`'s
// Class B Chain 1A postulate: each call's post says the entire `s1_s2`
// slice is `is_pos eta` (the function only writes 4 elements but the
// initial zero-fill keeps unwritten indices in [0, 2*eta] = `is_pos eta`).
// Consumed downstream by `signing_key::generate_serialized` (which
// requires `is_pos eta` on `s1_2`) — option (a) per the strict-polarity
// audit, no `Spec.Utils` bridge lemma needed.
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${s1_s2}_future == Seq.length $s1_s2 /\
    (forall (k:nat). k < Seq.length ${s1_s2}_future ==>
        (forall (j:nat). j < 32 ==>
            Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque
                (match $eta with
                 | Libcrux_ml_dsa.Constants.Eta_Two -> 2
                 | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
                (i0._super_i2.f_repr (Seq.index (Seq.index ${s1_s2}_future k).f_simd_units j))))
"#))]
pub(crate) fn sample_s1_and_s2<SIMDUnit: Operations, Shake256X4: shake256::XofX4>(
    eta: Eta,
    seed: &[u8],
    s1_s2: &mut [PolynomialRingElement<SIMDUnit>],
) {
    let len = s1_s2.len();

    // XXX: div_ceil is not implemented in F*.
    for i in 0..len / 4 {
        sample_four_error_ring_elements::<SIMDUnit, Shake256X4>(eta, seed, 4 * i as u16, s1_s2);
    }

    // Do it another time if needed.
    let remainder = len % 4;
    if remainder != 0 {
        sample_four_error_ring_elements::<SIMDUnit, Shake256X4>(
            eta,
            seed,
            (len - remainder) as u16,
            s1_s2,
        );
    }
}
