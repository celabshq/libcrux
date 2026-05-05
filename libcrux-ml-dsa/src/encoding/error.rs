// Functions for serializing and deserializing an error ring element.

use crate::{
    constants::Eta, ntt::ntt, polynomial::PolynomialRingElement, simd::traits::Operations,
};

#[cfg(hax)]
use crate::simd::traits::specs::*;

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length $serialized == 32 * (match $eta with
                                     | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                     | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
    (forall (j:nat). j < 32 ==>
      Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque
        (match $eta with
         | Libcrux_ml_dsa.Constants.Eta_Two -> 2
         | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
        (i0._super_i2.f_repr (Seq.index re.f_simd_units j)))"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
pub(crate) fn serialize<SIMDUnit: Operations>(
    eta: Eta,
    re: &PolynomialRingElement<SIMDUnit>,
    serialized: &mut [u8], // OUTPUT_SIZE
) {
    let output_bytes_per_simd_unit = chunk_size(eta);
    for i in 0..re.simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= 32 /\
              v output_bytes_per_simd_unit == (match eta with
                                               | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                               | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
              Seq.length serialized == 32 * v output_bytes_per_simd_unit"#
        ));
        SIMDUnit::error_serialize(
            eta,
            &re.simd_units[i],
            &mut serialized
                [i * output_bytes_per_simd_unit..(i + 1) * output_bytes_per_simd_unit],
        );
    }
}

#[inline(always)]
fn chunk_size(eta: Eta) -> usize {
    match eta {
        Eta::Two => 3,
        Eta::Four => 4,
    }
}

// F-16 (2026-04-30): strengthen post to expose a per-simd-unit signed
// bound that holds for both eta values.  The trait post on
// `error_deserialize` is the eta-conditional `forall8`
// (`eta=2 ==> -5 <= v <= 2`, `eta=4 ==> -11 <= v <= 4`).  Both cases
// satisfy `is_i32b 11`, so we expose `is_i32b_array_opaque 11` per j.
// `11 < NTT_BASE_BOUND`, so `is_i32b_array_larger` lifts to
// `NTT_BASE_BOUND` at the caller (mirrors T0's `pow2 12 -> NTT_BASE_BOUND`
// pattern but without the SMTPat shortcut, since `forall8` has no
// strict_lower-style intro lemma).
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length $serialized == 32 * (match $eta with
                                     | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                     | Libcrux_ml_dsa.Constants.Eta_Four -> 4)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    (forall (j:nat). j < 32 ==>
      Spec.Utils.is_i32b_array_opaque 11
        (i0._super_i2.f_repr (Seq.index ${result}_future.f_simd_units j)))"#))]
fn deserialize<SIMDUnit: Operations>(
    eta: Eta,
    serialized: &[u8],
    result: &mut PolynomialRingElement<SIMDUnit>,
) {
    let chunk_size = chunk_size(eta);
    for i in 0..result.simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= 32 /\
              v chunk_size == (match eta with
                               | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                               | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
              Seq.length serialized == 32 * v chunk_size /\
              (forall (j:nat). j < v i ==>
                Spec.Utils.is_i32b_array_opaque 11
                  (i0._super_i2.f_repr (Seq.index result.f_simd_units j)))"#
        ));
        SIMDUnit::error_deserialize(
            eta,
            &serialized[i * chunk_size..(i + 1) * chunk_size],
            &mut result.simd_units[i],
        );
        // Bridge: the trait post is `forall8 (eta_two ==> -5 <= v <= 2 /\
        // eta_four ==> -11 <= v <= 4)` on `f_repr (result.simd_units[i])`.
        // Both cases satisfy `is_i32b 11`.  Targeted reveal per AP-3.
        hax_lib::fstar!(
            r#"
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
              (Spec.Utils.is_i32b_array_opaque 11
                 (i0._super_i2.f_repr (Seq.index result.f_simd_units (v i))))"#
        );
    }
}

// F-16 (2026-04-30): close body admit by mirroring T0's recipe.  Per-iter
// `deserialize` call gives `forall j<32. is_i32b_array_opaque 11 (...)`
// (post bridged from eta-conditional `forall8` inside `deserialize`).
// `11 ≤ NTT_BASE_BOUND` is lifted via `is_i32b_array_larger`, then
// `ntt`'s post supplies `is_i32b_array_opaque FIELD_MAX` for that index.
// The loop_invariant tracks the partial-progress FIELD_MAX bound across
// `ring_elements[0..i]`.
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length $serialized == v $ring_element_size * Seq.length ring_elements /\
    Seq.length ring_elements <= 8 /\
    v $ring_element_size == 32 * (match $eta with
                                  | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                  | Libcrux_ml_dsa.Constants.Eta_Four -> 4)"#))]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${ring_elements}_future == Seq.length $ring_elements"#))]
pub(crate) fn deserialize_to_vector_then_ntt<SIMDUnit: Operations>(
    eta: Eta,
    ring_element_size: usize,
    serialized: &[u8],
    ring_elements: &mut [PolynomialRingElement<SIMDUnit>],
) {
    let n = serialized.len() / ring_element_size;
    for i in 0..n {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= v n /\
              v n == Seq.length ring_elements /\
              Seq.length serialized == v ring_element_size * Seq.length ring_elements /\
              v ring_element_size == 32 * (match eta with
                                           | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                           | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
              Seq.length ring_elements <= 8 /\
              (forall (k:nat). k < v i ==>
                (forall (j:nat). j < 32 ==>
                  Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                    (i0._super_i2.f_repr (Seq.index (Seq.index ring_elements k).f_simd_units j))))"#
        ));
        let bytes = &serialized[i * ring_element_size..(i + 1) * ring_element_size];
        deserialize::<SIMDUnit>(eta, bytes, &mut ring_elements[i]);
        // Lift the per-j `is_i32b_array_opaque 11` we just got from
        // `deserialize` to `FIELD_MAX = NTT_BASE_BOUND` per-lane, then to
        // `is_bounded_poly FIELD_MAX` so `ntt`'s (newly is_bounded_poly-form)
        // pre discharges.
        hax_lib::fstar!(
            r#"
            let lemma_lift (j:nat{j < 32}) :
              Lemma (Spec.Utils.is_i32b_array_opaque
                       (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND)
                       (i0._super_i2.f_repr
                         (Seq.index (Seq.index ring_elements (v i)).f_simd_units j))) =
              Spec.Utils.is_i32b_array_larger 11
                (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND)
                (i0._super_i2.f_repr
                  (Seq.index (Seq.index ring_elements (v i)).f_simd_units j))
            in
            Classical.forall_intro lemma_lift;
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
              (mk_usize 8380416) (Seq.index ring_elements (v i))"#
        );
        ntt(&mut ring_elements[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::simd::{self, traits::Operations};

    fn test_deserialize_generic<SIMDUnit: Operations>() {
        let serialized = [
            220, 24, 44, 136, 134, 36, 11, 195, 72, 82, 34, 144, 36, 33, 9, 196, 22, 70, 100, 148,
            65, 32, 163, 1, 210, 40, 14, 224, 38, 72, 33, 41, 136, 156, 146, 80, 25, 37, 50, 92,
            66, 140, 227, 144, 96, 9, 64, 141, 193, 50, 1, 140, 20, 141, 92, 32, 97, 2, 25, 42, 34,
            66, 50, 193, 72, 113, 202, 8, 16, 209, 54, 44, 129, 194, 48, 144, 20, 141, 228, 166,
            141, 228, 20, 144, 146, 54, 12, 99, 4, 140, 226, 18, 12, 194, 38, 97,
        ];

        let expected_coefficients = [
            -2, -1, -1, -2, 1, 2, -1, 1, 2, 1, 0, -1, 2, 1, 1, 1, -1, 1, -2, 1, -2, 1, 0, 0, 0, 0,
            1, 1, 0, 2, -2, -2, -2, -2, -2, 2, 0, 0, 0, 2, -2, 2, -1, -1, 1, -2, 1, 0, -2, -2, 1,
            0, 1, -1, 2, 0, 2, -2, -2, 1, 0, -1, 2, 2, 0, 0, -1, -2, 0, -2, -1, 2, 2, -2, -1, -1,
            0, 2, 0, 0, 1, -2, -2, -2, 0, 2, 0, -2, -2, -1, 0, 1, 1, 1, -2, 0, 1, -1, -2, 0, 0, -2,
            -2, 1, -2, -1, 1, 1, -2, 2, -1, -2, -1, -2, -1, 2, 1, 1, 2, -1, 1, 1, 2, 2, -2, 0, -1,
            -2, 1, 2, -1, 1, -1, 0, 2, 2, -2, 1, 0, 0, 1, 0, -1, -2, -2, -1, 1, 2, 0, 0, 2, -1, 0,
            2, -2, -2, 1, -2, 0, 1, 0, -2, 2, 1, -2, -2, -2, 1, 1, 2, -1, -2, -2, 0, -2, -1, 0, 1,
            -1, -2, 2, 2, -2, 2, 1, 0, -1, -1, -1, 2, -1, 1, 1, 2, 0, 1, -2, 1, -2, 1, 2, 0, 0, 0,
            1, 0, -1, -2, -2, -2, -1, -1, 0, -1, -1, -2, -2, -2, -1, 0, 1, 2, -2, -2, 0, 0, 0, -1,
            -1, 2, -1, 2, -1, -2, 1, 0, 2, 2, -1, -2, 0, -2, -1, 1, 1, 2, -1, 2, 0, 2, -1, -1, 0,
            0, 2, -1,
        ];

        let mut deserialized = PolynomialRingElement::<SIMDUnit>::zero();
        deserialize::<SIMDUnit>(Eta::Two, &serialized, &mut deserialized);
        assert_eq!(deserialized.to_i32_array(), expected_coefficients);

        let serialized = [
            22, 103, 55, 49, 34, 65, 50, 129, 52, 65, 21, 85, 82, 69, 3, 55, 52, 101, 80, 64, 114,
            136, 53, 8, 135, 67, 64, 71, 131, 21, 117, 81, 23, 99, 17, 84, 51, 23, 117, 56, 52, 85,
            131, 17, 22, 117, 85, 68, 34, 113, 87, 24, 65, 81, 2, 80, 118, 53, 34, 8, 32, 51, 51,
            82, 24, 2, 69, 0, 80, 68, 129, 133, 134, 17, 134, 82, 18, 21, 37, 114, 55, 87, 83, 8,
            80, 52, 103, 1, 84, 82, 99, 16, 86, 48, 16, 133, 17, 2, 67, 51, 120, 71, 19, 5, 72, 19,
            21, 103, 114, 115, 69, 36, 97, 68, 115, 56, 18, 83, 99, 32, 83, 88, 37, 71, 35, 82, 24,
            19,
        ];

        let expected_coefficients = [
            -2, 3, -3, -2, -3, 1, 3, 1, 2, 2, 3, 0, 2, 1, 3, -4, 0, 1, 3, 0, -1, 3, -1, -1, 2, -1,
            -1, 0, 1, 4, -3, 1, 0, 1, -1, -2, 4, -1, 4, 0, 2, -3, -4, -4, -1, 1, -4, 4, -3, -4, 1,
            0, 4, 0, -3, 0, 1, -4, -1, 3, -1, -3, 3, -1, -3, 3, 1, -2, 3, 3, 0, -1, 1, 1, -3, 3,
            -1, -3, -4, 1, 0, 1, -1, -1, 1, -4, 3, 3, -2, 3, -1, -3, -1, -1, 0, 0, 2, 2, 3, -3, -3,
            -1, -4, 3, 3, 0, 3, -1, 2, 4, 4, -1, -2, -3, -1, 1, 2, 2, -4, 4, 4, 2, 1, 1, 1, 1, 2,
            -1, -4, 3, 2, 4, -1, 0, 4, 4, 4, -1, 0, 0, 3, -4, -1, -4, -2, -4, 3, 3, -2, -4, 2, -1,
            2, 3, -1, 3, -1, 2, 2, -3, -3, 1, -3, -1, 1, -1, -4, 4, 4, -1, 0, 1, -3, -2, 3, 4, 0,
            -1, 2, -1, 1, -2, 4, 3, -2, -1, 4, 1, 4, 3, -1, -4, 3, 3, 2, 4, 1, 0, 1, 1, -4, -3, -3,
            0, 1, 3, -1, 4, -4, 0, 1, 3, -1, 3, -3, -2, 2, -3, 1, -3, -1, 0, 0, 2, 3, -2, 0, 0, 1,
            -3, -4, 1, 2, 3, 1, -1, 1, -2, 4, 2, 1, -1, -4, -1, -1, 2, -3, 0, 1, 2, 2, -1, -4, 3,
            1, 3,
        ];

        let mut deserialized = PolynomialRingElement::<SIMDUnit>::zero();
        deserialize::<SIMDUnit>(Eta::Four, &serialized, &mut deserialized);
        assert_eq!(deserialized.to_i32_array(), expected_coefficients);
    }

    #[test]
    fn test_deserialize_portable() {
        test_deserialize_generic::<simd::portable::PortableSIMDUnit>();
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn test_deserialize_simd256() {
        test_deserialize_generic::<simd::avx2::AVX2SIMDUnit>();
    }
}
