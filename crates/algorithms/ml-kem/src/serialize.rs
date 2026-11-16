use crate::{
    constants::{BYTES_PER_RING_ELEMENT, SHARED_SECRET_SIZE},
    helper::cloop,
    polynomial::{PolynomialRingElement, VECTORS_IN_RING_ELEMENT},
    vector::Operations,
};

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.is_bounded_vector 3328 $a"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
    v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $result) i) >= 0 /\
    v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $result) i) < v ${crate::vector::FIELD_MODULUS}"#)))]
pub(super) fn to_unsigned_field_modulus<Vector: Operations>(a: Vector) -> Vector {
    Vector::to_unsigned_representative(a)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"$result ==
        Spec.MLKEM.compress_then_encode_message (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re)"#)
))]
pub(super) fn compress_then_serialize_message<Vector: Operations>(
    re: PolynomialRingElement<Vector>,
) -> [u8; SHARED_SECRET_SIZE] {
    let mut serialized = [0u8; SHARED_SECRET_SIZE];
    for i in 0..16 {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!("v $i < 16 ==> Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re")
        });
        #[cfg(hax)]
        hax_lib::fstar!(r#"assert (2 * v $i + 2 <= 32)"#);

        let coefficient = to_unsigned_field_modulus(re.coefficients[i]);
        let coefficient_compressed = Vector::compress_1(coefficient);

        let bytes = Vector::serialize_1(coefficient_compressed);
        serialized[2 * i..2 * i + 2].copy_from_slice(&bytes);
    }

    serialized
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $result ==
        Spec.MLKEM.decode_then_decompress_message $serialized"#)
))]
pub(super) fn deserialize_then_decompress_message<Vector: Operations>(
    serialized: &[u8; SHARED_SECRET_SIZE],
) -> PolynomialRingElement<Vector> {
    let mut re = PolynomialRingElement::<Vector>::ZERO();
    for i in 0..16 {
        let coefficient_compressed = Vector::deserialize_1(&serialized[2 * i..2 * i + 2]);
        re.coefficients[i] = Vector::decompress_1(coefficient_compressed);
    }
    re
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"$result ==
        Spec.MLKEM.byte_encode 12 (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re)"#)
))]
pub(super) fn serialize_uncompressed_ring_element<Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; BYTES_PER_RING_ELEMENT] {
    #[cfg(hax)]
    hax_lib::fstar!(r#"assert_norm (pow2 12 == 4096)"#);
    let mut serialized = [0u8; BYTES_PER_RING_ELEMENT];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#
            )
        });
        #[cfg(hax)]
        hax_lib::fstar!(r#"assert (24 * v $i + 24 <= 384)"#);
        let coefficient = to_unsigned_field_modulus(re.coefficients[i]);

        let bytes = Vector::serialize_12(coefficient);
        serialized[24 * i..24 * i + 24].copy_from_slice(&bytes);
    }
    serialized
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == BYTES_PER_RING_ELEMENT
))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $result == 
        Spec.MLKEM.byte_decode 12 $serialized"#)
))]
pub(super) fn deserialize_to_uncompressed_ring_element<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == BYTES_PER_RING_ELEMENT
))]
fn deserialize_to_reduced_ring_element<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    hax_lib::fstar!(r#"assert (v $BYTES_PER_RING_ELEMENT / 24 == 16)"#);
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(24).enumerate() {
            let coefficient = Vector::deserialize_12(bytes);
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(
    fstar!(r#"Spec.MLKEM.is_rank v_K /\ 
            Seq.length public_key == v (Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE v_K)"#)
))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"forall (i:nat). i < v $K ==>
        Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index $result i)"#)
))]
pub(super) fn deserialize_ring_elements_reduced_out<const K: usize, Vector: Operations>(
    public_key: &[u8],
) -> [PolynomialRingElement<Vector>; K] {
    let mut deserialized_pk = core::array::from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());
    deserialize_ring_elements_reduced::<K, Vector>(public_key, &mut deserialized_pk);
    deserialized_pk
}

/// See [deserialize_ring_elements_reduced_out].
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(
    fstar!(r#"Spec.MLKEM.is_rank v_K /\ 
            Seq.length public_key == v (Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE v_K)"#)
))]
#[cfg_attr(hax, hax_lib::ensures(|_|
    fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector ${deserialized_pk}_future == 
        Spec.MLKEM.vector_decode_12 #$K $public_key"#)
))]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v $OUT_LEN == 320 /\ Libcrux_ml_kem.Polynomial.is_bounded_poly 3328  $re"#)))]
fn compress_then_serialize_10<const OUT_LEN: usize, Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    #[cfg(hax)]
    hax_lib::fstar!(r#"assert_norm (pow2 10 == 1024)"#);
    let mut serialized = [0u8; OUT_LEN];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#
            )
        });
        #[cfg(hax)]
        hax_lib::fstar!(r#"assert (20 * v $i + 20 <= 320)"#);
        let coefficient = Vector::compress::<10>(to_unsigned_field_modulus(re.coefficients[i]));

        let bytes = Vector::serialize_10(coefficient);
        serialized[20 * i..20 * i + 20].copy_from_slice(&bytes);
    }
    serialized
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"(v $COMPRESSION_FACTOR == 10 \/ v $COMPRESSION_FACTOR == 11) /\
    v $OUT_LEN == 32 * v $COMPRESSION_FACTOR /\ Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"$result == Spec.MLKEM.compress_then_byte_encode (v $COMPRESSION_FACTOR)
        (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re)"#)
))]
pub(super) fn compress_then_serialize_ring_element_u<
    const COMPRESSION_FACTOR: usize,
    const OUT_LEN: usize,
    Vector: Operations,
>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    #[cfg(hax)]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Seq.length $serialized == 128 /\
    Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|_|
    fstar!(r#"${serialized_future.len()} == ${serialized.len()}"#)
))]
fn compress_then_serialize_4<Vector: Operations>(
    re: PolynomialRingElement<Vector>,
    serialized: &mut [u8],
) {
    #[cfg(hax)]
    hax_lib::fstar!(r#"assert_norm (pow2 4 == 16)"#);
    for i in 0..VECTORS_IN_RING_ELEMENT {
        // NOTE: Using `$serialized` in loop_invariant doesn't work here
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> (Seq.length serialized == 128 /\ 
                           Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re)"#
            )
        });
        #[cfg(hax)]
        hax_lib::fstar!(r#"assert (8 * v $i + 8 <= 128)"#);
        let coefficient = Vector::compress::<4>(to_unsigned_field_modulus(re.coefficients[i]));

        let bytes = Vector::serialize_4(coefficient);
        serialized[8 * i..8 * i + 8].copy_from_slice(&bytes);
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == 160
))]
#[cfg_attr(hax, hax_lib::ensures(|_|
    fstar!(r#"${serialized_future.len()} == ${serialized.len()}"#)
))]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank v_K /\ 
    $COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR v_K /\
    Seq.length $out == v $OUT_LEN /\ v $OUT_LEN == 32 * v $COMPRESSION_FACTOR /\
    Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|_|
    fstar!(r#"${out_future.len()} == ${out.len()} /\
        ${out}_future == Spec.MLKEM.compress_then_encode_v #v_K
            (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re)"#)
))]
pub(super) fn compress_then_serialize_ring_element_v<
    const K: usize,
    const COMPRESSION_FACTOR: usize,
    const OUT_LEN: usize,
    Vector: Operations,
>(
    re: PolynomialRingElement<Vector>,
    out: &mut [u8],
) {
    #[cfg(hax)]
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
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == 320
))]
fn deserialize_then_decompress_10<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 10) /! sz 8) == 320)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(20).enumerate() {
            let coefficient = Vector::deserialize_10(bytes);
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<10>(coefficient);
        }
    }
    re
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == 352
))]
fn deserialize_then_decompress_11<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(
    (COMPRESSION_FACTOR == 10 || COMPRESSION_FACTOR == 11) &&
    serialized.len() == 32 * COMPRESSION_FACTOR
))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $result == 
        Spec.MLKEM.byte_decode_then_decompress (v $COMPRESSION_FACTOR) $serialized"#)
))]
pub(super) fn deserialize_then_decompress_ring_element_u<
    const COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
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
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == 128
))]
fn deserialize_then_decompress_4<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 4) /! sz 8) == 128)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(8).enumerate() {
            let coefficient = Vector::deserialize_4(bytes);
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<4>(coefficient);
        }
    }
    re
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(
    serialized.len() == 160
))]
fn deserialize_then_decompress_5<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
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
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\ 
    $COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR $K /\
    Seq.length $serialized == 32 * v $COMPRESSION_FACTOR"#)
))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $result == 
        Spec.MLKEM.decode_then_decompress_v #${K} $serialized"#)
))]
pub(super) fn deserialize_then_decompress_ring_element_v<
    const K: usize,
    const COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
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
