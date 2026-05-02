use crate::{
    constants::{BYTES_PER_RING_ELEMENT, SHARED_SECRET_SIZE},
    helper::cloop,
    polynomial::{PolynomialRingElement, VECTORS_IN_RING_ELEMENT},
    vector::Operations,
};

#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::spec::{matrix_to_spec, poly_to_spec, vector_to_spec};

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector (sz 3328) $a"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
    v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $result) i) >= 0 /\
    v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $result) i) < v ${crate::vector::FIELD_MODULUS}"#))]
pub(super) fn to_unsigned_field_modulus<Vector: Operations>(a: Vector) -> Vector {
    Vector::to_unsigned_representative(a)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result ==
        Spec.MLKEM.compress_then_encode_message (${poly_to_spec::<Vector>} $re)"#)
)]
pub(super) fn compress_then_serialize_message<Vector: Operations>(
    re: PolynomialRingElement<Vector>,
) -> [u8; SHARED_SECRET_SIZE] {
    let mut serialized = [0u8; SHARED_SECRET_SIZE];
    for i in 0..16 {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!("v $i < 16 ==> Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re")
        });
        hax_lib::fstar!(r#"assert (2 * v $i + 2 <= 32)"#);

        let coefficient = to_unsigned_field_modulus(re.coefficients[i]);
        let coefficient_compressed = Vector::compress_1(coefficient);
        // Bridge: compress_1's post `bounded_pos_i16_array 1` (=
        // `bounded_i16_array (mk_i16 0) (mk_i16 1)`) to serialize_1's pre
        // `serialize_pre_N 1 r` (= `forall j. bounded r[j] 1`).  Targeted
        // reveal (Rule SD4) — unfolds the opaque only for THIS instance,
        // not universally; previously the global form polluted Z3 every
        // loop iteration with the unbound forall.
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                       (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array (mk_i16 0) (mk_i16 1)
                         (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient_compressed}));
               assert_norm (pow2 1 == 2);
               assert (forall (k: nat). {:pattern Seq.index
                       (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient_compressed}) k}
                  k < 16 ==> Rust_primitives.BitVectors.bounded
                    (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient_compressed}) k) 1)"#
        );

        let bytes = Vector::serialize_1(coefficient_compressed);
        serialized[2 * i..2 * i + 2].copy_from_slice(&bytes);
    }

    serialized
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::ensures(|result|
    fstar!(r#"${poly_to_spec::<Vector>} $result ==
        Spec.MLKEM.decode_then_decompress_message $serialized"#)
)]
pub(super) fn deserialize_then_decompress_message<Vector: Operations>(
    serialized: &[u8; SHARED_SECRET_SIZE],
) -> PolynomialRingElement<Vector> {
    let mut re = PolynomialRingElement::<Vector>::ZERO();
    for i in 0..16 {
        let coefficient_compressed = Vector::deserialize_1(&serialized[2 * i..2 * i + 2]);
        // Bridge: deserialize_1's post `forall j. bounded r[j] 1` (=
        // `forall j. 0 <= v r[j] < 2`) to decompress_1's pre
        // `bounded_pos_i16_array 1 r` (= `bounded_i16_array 0 1 r`).
        // Targeted reveal (Rule SD4) — only the specific instance.
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                       (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array (mk_i16 0) (mk_i16 1)
                         (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient_compressed}));
               assert_norm (pow2 1 == 2)"#
        );
        re.coefficients[i] = Vector::decompress_1(coefficient_compressed);
    }
    re
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result ==
        Spec.MLKEM.byte_encode 12 (${poly_to_spec::<Vector>} $re)"#)
)]
pub(super) fn serialize_uncompressed_ring_element<Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; BYTES_PER_RING_ELEMENT] {
    hax_lib::fstar!(r#"assert_norm (pow2 12 == 4096)"#);
    let mut serialized = [0u8; BYTES_PER_RING_ELEMENT];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#
            )
        });
        hax_lib::fstar!(r#"assert (24 * v $i + 24 <= 384)"#);
        let coefficient = to_unsigned_field_modulus(re.coefficients[i]);

        let bytes = Vector::serialize_12(coefficient);
        serialized[24 * i..24 * i + 24].copy_from_slice(&bytes);
    }
    serialized
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(
    serialized.len() == BYTES_PER_RING_ELEMENT
)]
#[hax_lib::ensures(|result|
    fstar!(r#"${poly_to_spec::<Vector>} $result == 
        Spec.MLKEM.byte_decode 12 $serialized"#)
)]
pub(super) fn deserialize_to_uncompressed_ring_element<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(r#"assert (v $BYTES_PER_RING_ELEMENT / 24 == 16)"#);
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(24).enumerate() {
            re.coefficients[i] = Vector::deserialize_12(bytes);
        }
    }

    re
}

/// Only use with public values.
///
/// This MUST NOT be used with secret inputs, like its caller `deserialize_ring_elements_reduced`.
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(
    serialized.len() == BYTES_PER_RING_ELEMENT
)]
fn deserialize_to_reduced_ring_element<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(r#"assert (v $BYTES_PER_RING_ELEMENT / 24 == 16)"#);
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(24).enumerate() {
            let coefficient = Vector::deserialize_12(bytes);
            hax_lib::fstar!(
                r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 4095)"#
        )   ;
            re.coefficients[i] = Vector::cond_subtract_3329(coefficient);
        }
    }
    re
}

/// This function deserializes ring elements and reduces the result by the field
/// modulus.
///
/// This function MUST NOT be used on secret inputs.
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(
    fstar!(r#"Hacspec_ml_kem.Parameters.is_rank v_K /\ 
            Seq.length public_key == v (Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size v_K)"#)
)]
#[hax_lib::ensures(|result|
    fstar!(r#"forall (i:nat). i < v $K ==>
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index $result i)"#)
)]
pub(super) fn deserialize_ring_elements_reduced_out<const K: usize, Vector: Operations>(
    public_key: &[u8],
) -> [PolynomialRingElement<Vector>; K] {
    let mut deserialized_pk = core::array::from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());
    deserialize_ring_elements_reduced::<K, Vector>(public_key, &mut deserialized_pk);
    deserialized_pk
}

/// See [deserialize_ring_elements_reduced_out].
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(
    fstar!(r#"Hacspec_ml_kem.Parameters.is_rank v_K /\ 
            Seq.length public_key == v (Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size v_K)"#)
)]
#[hax_lib::ensures(|_|
    fstar!(r#"${vector_to_spec::<K, Vector>} (mk_usize $K) ${deserialized_pk}_future == 
        Spec.MLKEM.vector_decode_12 #$K $public_key"#)
)]
pub(super) fn deserialize_ring_elements_reduced<const K: usize, Vector: Operations>(
    public_key: &[u8],
    deserialized_pk: &mut [PolynomialRingElement<Vector>; K],
) {
    cloop! {
        for (i, ring_element) in public_key
            .chunks_exact(BYTES_PER_RING_ELEMENT)
            .enumerate()
        {
            deserialized_pk[i] = deserialize_to_reduced_ring_element(ring_element);
        }
    };
    ()
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"v $OUT_LEN == 320 /\ Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328)  $re"#))]
fn compress_then_serialize_10<const OUT_LEN: usize, Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    hax_lib::fstar!(r#"assert_norm (pow2 10 == 1024)"#);
    let mut serialized = [0u8; OUT_LEN];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#
            )
        });
        hax_lib::fstar!(r#"assert (20 * v $i + 20 <= 320)"#);
        let unreduced = to_unsigned_field_modulus(re.coefficients[i]);
        // Intro direction: prove `bounded_i16_array (mk_i16 0) (mk_i16 3328)`
        // from `to_unsigned_field_modulus`'s post (forall j. 0 <= v r[j] <= 3328)
        // for `compress::<10>`'s pre.  Use the named lemma (no global SMTPat).
        hax_lib::fstar!(
            r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                  (mk_i16 0) (mk_i16 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_repr ${unreduced})"#
        );
        let coefficient = Vector::compress::<10>(unreduced);

        let bytes = Vector::serialize_10(coefficient);
        serialized[20 * i..20 * i + 20].copy_from_slice(&bytes);
    }
    serialized
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
fn compress_then_serialize_11<const OUT_LEN: usize, Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    let mut serialized = [0u8; OUT_LEN];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        let coefficient =
            Vector::compress::<11>(Vector::to_unsigned_representative(re.coefficients[i]));

        let bytes = Vector::serialize_11(coefficient);
        serialized[22 * i..22 * i + 22].copy_from_slice(&bytes);
    }
    serialized
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"(v $COMPRESSION_FACTOR == 10 \/ v $COMPRESSION_FACTOR == 11) /\
    v $OUT_LEN == 32 * v $COMPRESSION_FACTOR /\ Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result == Spec.MLKEM.compress_then_byte_encode (v $COMPRESSION_FACTOR)
        (${poly_to_spec::<Vector>} $re)"#)
)]
pub(super) fn compress_then_serialize_ring_element_u<
    const COMPRESSION_FACTOR: usize,
    const OUT_LEN: usize,
    Vector: Operations,
>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    hax_lib::fstar!(
        r#"assert (
        (v (cast $COMPRESSION_FACTOR <: u32) == 10) \/
        (v (cast $COMPRESSION_FACTOR <: u32) == 11))"#
    );
    match COMPRESSION_FACTOR as u32 {
        10 => compress_then_serialize_10(re),
        11 => compress_then_serialize_11(re),
        _ => unreachable!(),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Seq.length $serialized == 128 /\
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|_|
    fstar!(r#"${serialized_future.len()} == ${serialized.len()}"#)
)]
fn compress_then_serialize_4<Vector: Operations>(
    re: PolynomialRingElement<Vector>,
    serialized: &mut [u8],
) {
    hax_lib::fstar!(r#"assert_norm (pow2 4 == 16)"#);
    for i in 0..VECTORS_IN_RING_ELEMENT {
        // NOTE: Using `$serialized` in loop_invariant doesn't work here
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> (Seq.length serialized == 128 /\ 
                           Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re)"#
            )
        });
        hax_lib::fstar!(r#"assert (8 * v $i + 8 <= 128)"#);
        let unreduced = to_unsigned_field_modulus(re.coefficients[i]);
        // Intro direction for compress::<4>'s pre.
        hax_lib::fstar!(
            r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                  (mk_i16 0) (mk_i16 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_repr ${unreduced})"#
        );
        let coefficient = Vector::compress::<4>(unreduced);

        let bytes = Vector::serialize_4(coefficient);
        serialized[8 * i..8 * i + 8].copy_from_slice(&bytes);
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(
    serialized.len() == 160
)]
#[hax_lib::ensures(|_|
    fstar!(r#"${serialized_future.len()} == ${serialized.len()}"#)
)]
fn compress_then_serialize_5<Vector: Operations>(
    re: PolynomialRingElement<Vector>,
    serialized: &mut [u8],
) {
    for i in 0..VECTORS_IN_RING_ELEMENT {
        let coefficients =
            Vector::compress::<5>(Vector::to_unsigned_representative(re.coefficients[i]));

        let bytes = Vector::serialize_5(coefficients);
        serialized[10 * i..10 * i + 10].copy_from_slice(&bytes);
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank v_K /\ 
    $COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor v_K /\
    Seq.length $out == v $OUT_LEN /\ v $OUT_LEN == 32 * v $COMPRESSION_FACTOR /\
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|_|
    fstar!(r#"${out_future.len()} == ${out.len()} /\
        ${out}_future == Spec.MLKEM.compress_then_encode_v #v_K
            (${poly_to_spec::<Vector>} $re)"#)
)]
pub(super) fn compress_then_serialize_ring_element_v<
    const K: usize,
    const COMPRESSION_FACTOR: usize,
    const OUT_LEN: usize,
    Vector: Operations,
>(
    re: PolynomialRingElement<Vector>,
    out: &mut [u8],
) {
    hax_lib::fstar!(
        r#"assert (
        (v (cast $COMPRESSION_FACTOR <: u32) == 4) \/
        (v (cast $COMPRESSION_FACTOR <: u32) == 5))"#
    );
    match COMPRESSION_FACTOR as u32 {
        4 => compress_then_serialize_4(re, out),
        5 => compress_then_serialize_5(re, out),
        _ => unreachable!(),
    }
}

#[inline(always)]
#[hax_lib::requires(
    serialized.len() == 320
)]
fn deserialize_then_decompress_10<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 10) /! sz 8) == 320)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(20).enumerate() {
            let coefficient = Vector::deserialize_10(bytes);
            // Intro: deserialize_10 post `forall j. bounded c[j] 10` -> trait pre
            // `bounded_pos_i16_array 10` (= `bounded_i16_array 0 1023`).
            hax_lib::fstar!(
                r#"assert_norm (pow2 10 - 1 == 1023);
                   Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                     (mk_i16 0) (mk_i16 1023)
                     (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient})"#
            );
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<10>(coefficient);
        }
    }
    re
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(
    serialized.len() == 352
)]
fn deserialize_then_decompress_11<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 11) /! sz 8) == 352)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(22).enumerate() {
            let coefficient = Vector::deserialize_11(bytes);
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<11>(coefficient);
        }
    }

    re
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(
    (COMPRESSION_FACTOR == 10 || COMPRESSION_FACTOR == 11) &&
    serialized.len() == 32 * COMPRESSION_FACTOR
)]
#[hax_lib::ensures(|result|
    fstar!(r#"${poly_to_spec::<Vector>} $result == 
        Spec.MLKEM.byte_decode_then_decompress (v $COMPRESSION_FACTOR) $serialized"#)
)]
/// Decompress + decode the ciphertext-u ring element.  Output lanes are
/// in **plain** form (`v c ≡ decompress_d(byte_decode(input)) mod q`).
/// This is fed into `ntt_vector_u` which preserves plain form (Mont-form
/// zetas cancel with `mont_mul`'s `·R⁻¹`).
pub(super) fn deserialize_then_decompress_ring_element_u<
    const COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (
        (v (cast $COMPRESSION_FACTOR <: u32) == 10) \/
        (v (cast $COMPRESSION_FACTOR <: u32) == 11))"#
    );
    match COMPRESSION_FACTOR as u32 {
        10 => deserialize_then_decompress_10(serialized),
        11 => deserialize_then_decompress_11(serialized),
        _ => unreachable!(),
    }
}

#[inline(always)]
#[hax_lib::requires(
    serialized.len() == 128
)]
fn deserialize_then_decompress_4<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 4) /! sz 8) == 128)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(8).enumerate() {
            let coefficient = Vector::deserialize_4(bytes);
            // Intro: deserialize_4 post `forall j. bounded c[j] 4` -> trait pre
            // `bounded_pos_i16_array 4` (= `bounded_i16_array 0 15`).
            hax_lib::fstar!(
                r#"assert_norm (pow2 4 - 1 == 15);
                   Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                     (mk_i16 0) (mk_i16 15)
                     (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient})"#
            );
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<4>(coefficient);
        }
    }
    re
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(
    serialized.len() == 160
)]
fn deserialize_then_decompress_5<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 5) /! sz 8) == 160)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(10).enumerate() {
            re.coefficients[i] = Vector::deserialize_5(bytes);
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<5>(re.coefficients[i]);
        }
    }
    re
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\ 
    $COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor $K /\
    Seq.length $serialized == 32 * v $COMPRESSION_FACTOR"#)
)]
#[hax_lib::ensures(|result|
    fstar!(r#"${poly_to_spec::<Vector>} $result == 
        Spec.MLKEM.decode_then_decompress_v #${K} $serialized"#)
)]
/// Decompress + decode the ciphertext-v ring element.  Output lanes are
/// in **plain** form (`v c ≡ decompress_d(byte_decode(input)) mod q`).
/// This is consumed by `subtract_reduce` (in polynomial.rs) as `myself`,
/// the LHS in `v - InvNTT(s · u)` per FIPS-203 Algorithm 14.
pub(super) fn deserialize_then_decompress_ring_element_v<
    const K: usize,
    const COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (
        (v (cast $COMPRESSION_FACTOR <: u32) == 4) \/
        (v (cast $COMPRESSION_FACTOR <: u32) == 5))"#
    );
    match COMPRESSION_FACTOR as u32 {
        4 => deserialize_then_decompress_4(serialized),
        5 => deserialize_then_decompress_5(serialized),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod cross_spec_tests {
    use super::*;
    use crate::polynomial::cross_spec_tests::{lift_poly, unlift_poly};
    use crate::vector::portable::PortableVector;
    use hacspec_ml_kem::parameters::{self as spec, FieldElement, Polynomial};

    /// compress_then_serialize_message: impl matches spec.
    #[test]
    fn compress_then_serialize_message_matches_spec() {
        for pattern in [0x00u8, 0x55, 0xAA, 0xFF] {
            // Create a polynomial with coefficients in [0, q)
            let spec_poly: Polynomial = spec::createi(|i| {
                FieldElement::new(
                    ((i as u16).wrapping_mul(pattern as u16).wrapping_add(42))
                        % spec::FIELD_MODULUS,
                )
            });
            let impl_poly = unlift_poly(&spec_poly);

            let impl_bytes = compress_then_serialize_message::<PortableVector>(impl_poly);
            let spec_bytes = hacspec_ml_kem::serialize::compress_then_serialize_message(spec_poly);

            assert_eq!(
                impl_bytes, spec_bytes,
                "compress_then_serialize_message mismatch for pattern=0x{pattern:02X}"
            );
        }
    }

    /// deserialize_then_decompress_message: impl matches spec.
    #[test]
    fn deserialize_then_decompress_message_matches_spec() {
        for pattern in [0x00u8, 0x55, 0xAA, 0xFF] {
            let bytes = [pattern; 32];

            let impl_poly = deserialize_then_decompress_message::<PortableVector>(&bytes);
            let spec_poly = hacspec_ml_kem::serialize::deserialize_then_decompress_message(&bytes);

            assert_eq!(
                lift_poly(&impl_poly),
                spec_poly,
                "deserialize_then_decompress_message mismatch for pattern=0x{pattern:02X}"
            );
        }
    }

    /// serialize_uncompressed_ring_element (12-bit encode): impl matches spec.
    #[test]
    fn serialize_uncompressed_matches_spec() {
        let spec_poly: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 13 + 7) % spec::FIELD_MODULUS));
        let impl_poly = unlift_poly(&spec_poly);

        let impl_bytes = serialize_uncompressed_ring_element::<PortableVector>(&impl_poly);
        let spec_bytes = hacspec_ml_kem::serialize::serialize_uncompressed_ring_element(&spec_poly);

        assert_eq!(impl_bytes, spec_bytes);
    }

    /// deserialize_to_uncompressed_ring_element (12-bit decode): impl matches spec.
    #[test]
    fn deserialize_uncompressed_matches_spec() {
        // Create valid 12-bit encoded bytes via spec
        let spec_poly: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 17 + 3) % spec::FIELD_MODULUS));
        let bytes = hacspec_ml_kem::serialize::serialize_uncompressed_ring_element(&spec_poly);

        let impl_poly = deserialize_to_uncompressed_ring_element::<PortableVector>(&bytes);
        let spec_decoded =
            hacspec_ml_kem::serialize::deserialize_to_uncompressed_ring_element(&bytes);

        assert_eq!(
            lift_poly(&impl_poly),
            spec_decoded,
            "deserialize_to_uncompressed mismatch"
        );
    }

    /// Message roundtrip: serialize then deserialize recovers the message bytes.
    #[test]
    fn message_roundtrip() {
        let msg = [0xABu8; 32];
        let poly = deserialize_then_decompress_message::<PortableVector>(&msg);
        let recovered = compress_then_serialize_message::<PortableVector>(poly);
        assert_eq!(msg, recovered);
    }
}
