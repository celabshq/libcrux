use crate::{
    constants::{BYTES_PER_RING_ELEMENT, SHARED_SECRET_SIZE},
    helper::cloop,
    polynomial::{PolynomialRingElement, VECTORS_IN_RING_ELEMENT},
    vector::Operations,
};

#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::spec::{matrix_to_spec, poly_to_spec, vector_to_spec};

#[cfg(hax)]
use crate::polynomial::spec;

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector (sz 3328) $a"#))]
#[hax_lib::ensures(|result| fstar!(r#"(forall (i:nat). i < 16 ==>
    v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $result) i) >= 0 /\
    v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $result) i) < v ${crate::vector::FIELD_MODULUS}) /\
    (forall (i:nat). i < 16 ==>
       v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $result) i) >= 0 /\
       v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $result) i) < 3329 /\
       Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $result) i)
         == Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $a) i))"#))]
pub(super) fn to_unsigned_field_modulus<Vector: Operations>(a: Vector) -> Vector {
    let result = Vector::to_unsigned_representative(a);
    // Expose the value relation (`i16_to_spec_fe result[i] == i16_to_spec_fe a[i]`)
    // that the encode composers need; the trait post carries it as `mod_q_eq`,
    // which the wrapper would otherwise drop behind a bounds-only ensures.
    hax_lib::fstar!(
        r#"let aux (i:nat{i<16}) : Lemma
             (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $result) i)
              == Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $a) i)) =
             Hacspec_ml_kem.Commute.Serialize_bits.lemma_i16_to_spec_fe_mod_q_eq
               (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $result) i)
               (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr $a) i) in
           FStar.Classical.forall_intro aux"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result ==
        Hacspec_ml_kem.Serialize.compress_then_serialize_message (${poly_to_spec::<Vector>} $re)"#)
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
        Hacspec_ml_kem.Serialize.deserialize_then_decompress_message $serialized /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $result"#)
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
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result ==
        Hacspec_ml_kem.Serialize.byte_encode (sz 384) (sz 3072) (${poly_to_spec::<Vector>} $re) (sz 12)"#)
)]
pub(super) fn serialize_uncompressed_ring_element<Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; BYTES_PER_RING_ELEMENT] {
    hax_lib::fstar!(r#"assert_norm (pow2 12 == 4096)"#);
    let mut serialized = [0u8; BYTES_PER_RING_ELEMENT];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant: each completed chunk carries the opaque per-chunk encode
        // atom (`chunk_byte_enc`) — keeps `byte_encode` (a heavy transparent `let`)
        // and `poly_to_spec_index`'s createi cascade out of the loop-body VC.
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\ Seq.length $serialized == 384 /\
                (v $i < 16 ==> Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re) /\
                (forall (j: nat). j < v $i ==>
                  Hacspec_ml_kem.Commute.Serialize_bits.chunk_byte_enc $serialized
                    (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) j)"#
            )
        });
        hax_lib::fstar!(r#"assert (24 * v $i + 24 <= 384)"#);
        #[cfg(hax)]
        let serialized_old = serialized;
        let coefficient = to_unsigned_field_modulus(re.coefficients[i]);

        let bytes = Vector::serialize_12(coefficient);
        serialized[24 * i..24 * i + 24].copy_from_slice(&bytes);
        // Establish chunk `i`'s atom (intro_re seals poly_to_spec_index's createi
        // cascade in a clean lemma), then extend the invariant to i+1 (a clean
        // standalone lemma — the opaque-atom forall must not run in this VC).
        hax_lib::fstar!(
            r#"let g = Libcrux_ml_kem.Vector.Traits.f_repr $coefficient in
               let ii = v $i in
               assert (Seq.slice $serialized (24 * ii) (24 * ii + 24) == $bytes);
               assert (BitVecEq.int_t_array_bitwise_eq g 12
                         (Seq.slice $serialized (24 * ii) (24 * ii + 24) <: t_Array u8 (mk_usize 24)) 8);
               Hacspec_ml_kem.Commute.Serialize_bits.lemma_chunk_byte_enc_intro_re $serialized $re g ii;
               assert (Seq.slice $serialized 0 (24 * ii) == Seq.slice $serialized_old 0 (24 * ii));
               Hacspec_ml_kem.Commute.Serialize_bits.lemma_chunk_byte_enc_extend $serialized_old
                 $serialized (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) ii"#
        );
    }
    // Finalize: unpack each chunk atom into its 24 per-byte equalities, then
    // conclude array equality with `byte_encode`.
    hax_lib::fstar!(
        r#"let p = Libcrux_ml_kem.Vector.Spec.poly_to_spec $re in
           let be = Hacspec_ml_kem.Serialize.byte_encode (sz 384) (sz 3072) p (sz 12) in
           let aux_final (k:nat) : Lemma (k < 384 ==> Seq.index $serialized k == Seq.index be k) =
             if k < 384 then begin
               Hacspec_ml_kem.Commute.Serialize_bits.lemma_chunk_byte_enc_unfold $serialized p (k / 24);
               assert (24 * (k / 24) + k % 24 == k);
               assert (k % 24 < 24);
               assert (k / 24 < 16)
             end in
           FStar.Classical.forall_intro aux_final;
           Seq.lemma_eq_intro $serialized be"#
    );
    serialized
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    serialized.len() == BYTES_PER_RING_ELEMENT
)]
#[hax_lib::ensures(|result|
    fstar!(r#"${poly_to_spec::<Vector>} $result ==
        Hacspec_ml_kem.Serialize.byte_decode (sz 384) (sz 3072) $serialized (sz 12)"#)
)]
pub(super) fn deserialize_to_uncompressed_ring_element<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(r#"assert (v $BYTES_PER_RING_ELEMENT / 24 == 16)"#);
    let mut re = PolynomialRingElement::<Vector>::ZERO();

    cloop! {
        for (i, bytes) in serialized.chunks_exact(24).enumerate() {
            // Loop invariant: each processed chunk's coefficient vector carries the
            // opaque per-chunk decode atom (`chunk_decoded_12`) — keeps the heavy
            // bit-vector equality out of the loop-body VC.
            hax_lib::loop_invariant!(|i: usize| {
                fstar!(
                    r#"v $i <= 16 /\ Seq.length $serialized == 384 /\
                    (forall (j: nat). j < v $i ==>
                      Hacspec_ml_kem.Commute.Serialize_bits.chunk_decoded_12 $serialized
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                          (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients j)) j)"#
                )
            });
            re.coefficients[i] = Vector::deserialize_12(bytes);
            // Establish the opaque atom for chunk `i` from deserialize_12's post.
            hax_lib::fstar!(
                r#"assert (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients (v $i) ==
                        Libcrux_ml_kem.Vector.Traits.f_deserialize_12_ #v_Vector ${bytes});
                   assert (Seq.slice $serialized (24 * v $i) (24 * v $i + 24) == ${bytes});
                   Hacspec_ml_kem.Commute.Serialize_bits.lemma_chunk_decoded_intro $serialized
                     (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                       (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients (v $i))) (v $i)"#
            );
        }
    }

    hax_lib::fstar!(
        r#"let result = re in
           assert (Seq.length (Libcrux_ml_kem.Vector.Spec.poly_to_spec result) == 256);
           assert (Seq.length (Hacspec_ml_kem.Serialize.byte_decode (sz 384) (sz 3072) $serialized (sz 12)) == 256);
           let aux (k: nat{k < 256}) : Lemma
             (ensures Seq.index (Libcrux_ml_kem.Vector.Spec.poly_to_spec result) k ==
                      Seq.index (Hacspec_ml_kem.Serialize.byte_decode (sz 384) (sz 3072) $serialized (sz 12)) k) =
             assert (k / 16 < 16);
             assert (Hacspec_ml_kem.Commute.Serialize_bits.chunk_decoded_12 $serialized
                       (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                         (Seq.index result.Libcrux_ml_kem.Vector.f_coefficients (k / 16))) (k / 16));
             Libcrux_ml_kem.Vector.Spec.poly_to_spec_index result k;
             Hacspec_ml_kem.Commute.Serialize_bits.lemma_chunk_decoded_byte_decode $serialized
               (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                 (Seq.index result.Libcrux_ml_kem.Vector.f_coefficients (k / 16))) (k / 16)
           in
           FStar.Classical.forall_intro aux;
           Seq.lemma_eq_intro (Libcrux_ml_kem.Vector.Spec.poly_to_spec result)
             (Hacspec_ml_kem.Serialize.byte_decode (sz 384) (sz 3072) $serialized (sz 12))"#
    );

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
    fstar!(r#"(forall (i:nat). i < v $K ==>
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index $result i)) /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector $K #$:Vector (sz 3328) $result"#)
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
    fstar!(r#"${vector_to_spec::<K, Vector>} $K ${deserialized_pk}_future ==
        Hacspec_ml_kem.Serialize.vector_decode_12_ $K $public_key /\
        (forall (i:nat). i < v $K ==>
            Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index ${deserialized_pk}_future i))"#)
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
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"v $OUT_LEN == 320 /\ Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328)  $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result == Hacspec_ml_kem.Serialize.byte_encode $OUT_LEN (sz 256 *! sz 10)
        (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} $re) (sz 10)) (sz 10)"#)
)]
fn compress_then_serialize_10<const OUT_LEN: usize, Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    hax_lib::fstar!(r#"assert_norm (pow2 10 == 1024)"#);
    let mut serialized = [0u8; OUT_LEN];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant: each completed 20-byte chunk carries the opaque per-chunk
        // encode atom (`chunk_byte_enc_d` at d=10) for `compress (poly_to_spec re) 10`
        // — keeps `byte_encode` + the compress value-match out of the loop-body VC.
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\ v $OUT_LEN == 320 /\
            (v $i < 16 ==> Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re) /\
            (forall (j: nat). j < v $i ==>
              Hacspec_ml_kem.Commute.Serialize_compress.chunk_byte_enc_d (sz 10) $OUT_LEN $serialized
                (Hacspec_ml_kem.Compress.compress (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) (sz 10)) j)"#
            )
        });
        hax_lib::fstar!(r#"assert (20 * v $i + 20 <= 320)"#);
        #[cfg(hax)]
        let serialized_old = serialized;
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
        // Establish chunk `i`'s atom from the compress + serialize_10 posts (the
        // compress value-match + bit-vector eq are sealed inside the commute lemma),
        // then extend the invariant to i+1 (clean standalone lemma).
        hax_lib::fstar!(
            r#"let g = Libcrux_ml_kem.Vector.Traits.f_repr $coefficient in
               let inp = Libcrux_ml_kem.Vector.Traits.f_repr $unreduced in
               let ii = v $i in
               assert (Libcrux_ml_kem.Vector.Traits.Spec.compress_post inp (mk_i32 10) g);
               assert (Seq.slice $serialized (20 * ii) (20 * ii + 20) == $bytes);
               assert (BitVecEq.int_t_array_bitwise_eq g 10
                         (Seq.slice $serialized (20 * ii) (20 * ii + 20) <: t_Array u8 (mk_usize 20)) 8);
               Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_byte_enc_intro_compress_post
                 (mk_i32 10) $OUT_LEN $serialized $re inp g ii;
               assert (Seq.slice $serialized 0 (20 * ii) == Seq.slice $serialized_old 0 (20 * ii));
               Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_byte_enc_extend_d
                 (sz 10) $OUT_LEN $serialized_old $serialized
                 (Hacspec_ml_kem.Compress.compress (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) (sz 10)) ii"#
        );
    }
    // Finalize: unpack each chunk atom into its 20 per-byte equalities, then
    // conclude array equality with `byte_encode (compress ...)`.
    hax_lib::fstar!(
        r#"let p = Hacspec_ml_kem.Compress.compress (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) (sz 10) in
           let be = Hacspec_ml_kem.Serialize.byte_encode (mk_usize 320) (mk_usize 2560) p (sz 10) in
           assert ($OUT_LEN == mk_usize 320);
           let aux_final (k:nat) : Lemma (k < 320 ==> Seq.index $serialized k == Seq.index be k) =
             if k < 320 then begin
               Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_byte_enc_unfold_d
                 (sz 10) $OUT_LEN $serialized p (k / 20);
               assert (20 * (k / 20) + k % 20 == k);
               assert (k % 20 < 20);
               assert (k / 20 < 16)
             end in
           FStar.Classical.forall_intro aux_final;
           Seq.lemma_eq_intro $serialized be"#
    );
    serialized
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"v $OUT_LEN == 352 /\ Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328)  $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result == Hacspec_ml_kem.Serialize.byte_encode $OUT_LEN (sz 256 *! sz 11)
        (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} $re) (sz 11)) (sz 11)"#)
)]
fn compress_then_serialize_11<const OUT_LEN: usize, Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> [u8; OUT_LEN] {
    hax_lib::fstar!(r#"assert_norm (pow2 11 == 2048)"#);
    let mut serialized = [0u8; OUT_LEN];
    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\ v $OUT_LEN == 352 /\
            (v $i < 16 ==> Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re) /\
            (forall (j: nat). j < v $i ==>
              Hacspec_ml_kem.Commute.Serialize_compress.chunk_byte_enc_d (sz 11) $OUT_LEN $serialized
                (Hacspec_ml_kem.Compress.compress (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) (sz 11)) j)"#
            )
        });
        hax_lib::fstar!(r#"assert (22 * v $i + 22 <= 352)"#);
        #[cfg(hax)]
        let serialized_old = serialized;
        let unreduced = to_unsigned_field_modulus(re.coefficients[i]);
        // Intro direction for compress::<11>'s pre.
        hax_lib::fstar!(
            r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                  (mk_i16 0) (mk_i16 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_repr ${unreduced})"#
        );
        let coefficient = Vector::compress::<11>(unreduced);

        let bytes = Vector::serialize_11(coefficient);
        serialized[22 * i..22 * i + 22].copy_from_slice(&bytes);
        // Establish chunk `i`'s atom (compress + serialize_11 posts sealed in the
        // commute lemma), then extend the invariant to i+1.
        hax_lib::fstar!(
            r#"let g = Libcrux_ml_kem.Vector.Traits.f_repr $coefficient in
               let inp = Libcrux_ml_kem.Vector.Traits.f_repr $unreduced in
               let ii = v $i in
               assert (Libcrux_ml_kem.Vector.Traits.Spec.compress_post inp (mk_i32 11) g);
               assert (Seq.slice $serialized (22 * ii) (22 * ii + 22) == $bytes);
               assert (BitVecEq.int_t_array_bitwise_eq g 11
                         (Seq.slice $serialized (22 * ii) (22 * ii + 22) <: t_Array u8 (mk_usize 22)) 8);
               Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_byte_enc_intro_compress_post
                 (mk_i32 11) $OUT_LEN $serialized $re inp g ii;
               assert (Seq.slice $serialized 0 (22 * ii) == Seq.slice $serialized_old 0 (22 * ii));
               Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_byte_enc_extend_d
                 (sz 11) $OUT_LEN $serialized_old $serialized
                 (Hacspec_ml_kem.Compress.compress (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) (sz 11)) ii"#
        );
    }
    // Finalize: unpack each chunk atom into its 22 per-byte equalities, then
    // conclude array equality with `byte_encode (compress ...)`.
    hax_lib::fstar!(
        r#"let p = Hacspec_ml_kem.Compress.compress (Libcrux_ml_kem.Vector.Spec.poly_to_spec $re) (sz 11) in
           let be = Hacspec_ml_kem.Serialize.byte_encode (mk_usize 352) (mk_usize 2816) p (sz 11) in
           assert ($OUT_LEN == mk_usize 352);
           let aux_final (k:nat) : Lemma (k < 352 ==> Seq.index $serialized k == Seq.index be k) =
             if k < 352 then begin
               Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_byte_enc_unfold_d
                 (sz 11) $OUT_LEN $serialized p (k / 22);
               assert (22 * (k / 22) + k % 22 == k);
               assert (k % 22 < 22);
               assert (k / 22 < 16)
             end in
           FStar.Classical.forall_intro aux_final;
           Seq.lemma_eq_intro $serialized be"#
    );
    serialized
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"(v $COMPRESSION_FACTOR == 10 \/ v $COMPRESSION_FACTOR == 11) /\
    v $OUT_LEN == 32 * v $COMPRESSION_FACTOR /\ Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|result|
    fstar!(r#"$result == Hacspec_ml_kem.Serialize.byte_encode $OUT_LEN (sz 256 *! $COMPRESSION_FACTOR)
        (Hacspec_ml_kem.Compress.compress (${poly_to_spec::<Vector>} $re) $COMPRESSION_FACTOR)
        $COMPRESSION_FACTOR"#)
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
        // Each branch's post is `byte_encode OUT_LEN (256*d) (compress (poly_to_spec re) d) d`
        // with the concrete d; the requires `v CF ∈ {10,11}` plus the branch guard pin
        // `CF == sz d`, so the dispatcher's symbolic-CF ensures follows.
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
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Seq.length $serialized == 160 /\
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re"#))]
#[hax_lib::ensures(|_|
    fstar!(r#"${serialized_future.len()} == ${serialized.len()}"#)
)]
fn compress_then_serialize_5<Vector: Operations>(
    re: PolynomialRingElement<Vector>,
    serialized: &mut [u8],
) {
    hax_lib::fstar!(r#"assert_norm (pow2 5 == 32)"#);
    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $i >= 0 /\ v $i <= 16 /\
            v $i < 16 ==> (Seq.length serialized == 160 /\
                           Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) $re)"#
            )
        });
        hax_lib::fstar!(r#"assert (10 * v $i + 10 <= 160)"#);
        let unreduced = to_unsigned_field_modulus(re.coefficients[i]);
        hax_lib::fstar!(
            r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                  (mk_i16 0) (mk_i16 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_repr ${unreduced})"#
        );
        let coefficient = Vector::compress::<5>(unreduced);

        let bytes = Vector::serialize_5(coefficient);
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
        ${out}_future == Hacspec_ml_kem.Serialize.compress_then_serialize_v $OUT_LEN
            (${poly_to_spec::<Vector>} $re) $COMPRESSION_FACTOR"#)
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
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    serialized.len() == 320
)]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &result)
    & fstar!(r#"${poly_to_spec::<Vector>} $result ==
        Hacspec_ml_kem.Compress.decompress
            (Hacspec_ml_kem.Serialize.byte_decode_dyn $serialized (sz 10)) (sz 10)"#)
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
            // Loop invariant: each completed coefficient vector carries the opaque
            // per-chunk decompressed atom (`chunk_decompressed_d` at d=10).
            hax_lib::loop_invariant!(|i: usize| {
                fstar!(
                    r#"v $i <= 16 /\ Seq.length $serialized == 320 /\
                    (forall (j: nat). j < v $i ==>
                      Hacspec_ml_kem.Commute.Serialize_compress.chunk_decompressed_d (sz 10) $serialized
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                          (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients j)) j)"#
                )
            });
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
            // Establish chunk `i`'s decompressed atom from the deserialize_10 +
            // decompress_ciphertext_coefficient posts (byte-bridge + decompress
            // value-match are sealed inside the commute lemma).
            hax_lib::fstar!(
                r#"let grp = Libcrux_ml_kem.Vector.Traits.f_repr $coefficient in
                   let g = Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                             (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients (v $i)) in
                   let ii = v $i in
                   assert (Libcrux_ml_kem.Vector.Traits.Spec.decompress_ciphertext_coefficient_post
                             grp (mk_i32 10) g);
                   assert (Seq.slice $serialized (20 * ii) (20 * ii + 20) == $bytes);
                   assert (BitVecEq.int_t_array_bitwise_eq
                             (Seq.slice $serialized (20 * ii) (20 * ii + 20) <: t_Array u8 (mk_usize 20)) 8 grp 10);
                   Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_decompressed_intro_post_d
                     (sz 10) (mk_i32 10) $serialized grp g ii"#
            );
        }
    }
    // Finalize: poly_to_spec re == decompress (byte_decode_dyn serialized 10) 10
    // and is_bounded_poly 3328 re, from the per-chunk atoms.
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Serialize_compress.lemma_byte_decode_dyn_eq $serialized (sz 10);
           Hacspec_ml_kem.Commute.Serialize_compress.lemma_poly_to_spec_eq_decompress (sz 10) $serialized $re;
           Hacspec_ml_kem.Commute.Serialize_compress.lemma_is_bounded_poly_of_chunks (sz 10) $serialized $re"#
    );
    re
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    serialized.len() == 352
)]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &result)
    & fstar!(r#"${poly_to_spec::<Vector>} $result ==
        Hacspec_ml_kem.Compress.decompress
            (Hacspec_ml_kem.Serialize.byte_decode_dyn $serialized (sz 11)) (sz 11)"#)
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
            hax_lib::loop_invariant!(|i: usize| {
                fstar!(
                    r#"v $i <= 16 /\ Seq.length $serialized == 352 /\
                    (forall (j: nat). j < v $i ==>
                      Hacspec_ml_kem.Commute.Serialize_compress.chunk_decompressed_d (sz 11) $serialized
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                          (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients j)) j)"#
                )
            });
            let coefficient = Vector::deserialize_11(bytes);
            // Intro: deserialize_11 post `forall j. bounded c[j] 11` -> trait pre
            // `bounded_pos_i16_array 11` (= `bounded_i16_array 0 2047`).
            hax_lib::fstar!(
                r#"assert_norm (pow2 11 - 1 == 2047);
                   Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                     (mk_i16 0) (mk_i16 2047)
                     (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient})"#
            );
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<11>(coefficient);
            hax_lib::fstar!(
                r#"let grp = Libcrux_ml_kem.Vector.Traits.f_repr $coefficient in
                   let g = Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                             (Seq.index ${re}.Libcrux_ml_kem.Vector.f_coefficients (v $i)) in
                   let ii = v $i in
                   assert (Libcrux_ml_kem.Vector.Traits.Spec.decompress_ciphertext_coefficient_post
                             grp (mk_i32 11) g);
                   assert (Seq.slice $serialized (22 * ii) (22 * ii + 22) == $bytes);
                   assert (BitVecEq.int_t_array_bitwise_eq
                             (Seq.slice $serialized (22 * ii) (22 * ii + 22) <: t_Array u8 (mk_usize 22)) 8 grp 11);
                   Hacspec_ml_kem.Commute.Serialize_compress.lemma_chunk_decompressed_intro_post_d
                     (sz 11) (mk_i32 11) $serialized grp g ii"#
            );
        }
    }
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Serialize_compress.lemma_byte_decode_dyn_eq $serialized (sz 11);
           Hacspec_ml_kem.Commute.Serialize_compress.lemma_poly_to_spec_eq_decompress (sz 11) $serialized $re;
           Hacspec_ml_kem.Commute.Serialize_compress.lemma_is_bounded_poly_of_chunks (sz 11) $serialized $re"#
    );
    re
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(
    (COMPRESSION_FACTOR == 10 || COMPRESSION_FACTOR == 11) &&
    serialized.len() == 32 * COMPRESSION_FACTOR
)]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &result)
    & fstar!(r#"${poly_to_spec::<Vector>} $result ==
        Hacspec_ml_kem.Compress.decompress
            (Hacspec_ml_kem.Serialize.byte_decode_dyn $serialized $COMPRESSION_FACTOR)
            $COMPRESSION_FACTOR"#)
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
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    serialized.len() == 128
)]
#[hax_lib::ensures(|result| spec::is_bounded_poly(4095, &result))]
fn deserialize_then_decompress_4<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 4) /! sz 8) == 128)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();
    // Lift `is_bounded_poly 0 re` (from ZERO ensures) to
    // `is_bounded_poly 4095 re` so the loop invariant holds on entry.
    #[cfg(hax)]
    spec::is_bounded_poly_higher(&re, 0, 4095);

    cloop! {
        for (i, bytes) in serialized.chunks_exact(8).enumerate() {
            hax_lib::loop_invariant!(|_i: usize| {
                spec::is_bounded_poly(4095, &re)
            });
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
            // Lift the strengthened trait post `bounded_i16_array 0 3328
            // (f_repr re.coefficients[i])` to `is_bounded_vector 4095
            // (re.coefficients[i])` so the loop invariant on `re` is
            // preserved by this iteration.
            #[cfg(hax)]
            spec::lemma_decompress_post_to_is_bounded_vector(&re.coefficients[i], 4095);
        }
    }
    re
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    serialized.len() == 160
)]
#[hax_lib::ensures(|result| spec::is_bounded_poly(4095, &result))]
fn deserialize_then_decompress_5<Vector: Operations>(
    serialized: &[u8],
) -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"assert (v ((${crate::constants::COEFFICIENTS_IN_RING_ELEMENT} *! sz 5) /! sz 8) == 160)"#
    );
    let mut re = PolynomialRingElement::<Vector>::ZERO();
    // Lift `is_bounded_poly 0 re` (from ZERO ensures) to
    // `is_bounded_poly 4095 re` so the loop invariant holds on entry.
    #[cfg(hax)]
    spec::is_bounded_poly_higher(&re, 0, 4095);

    cloop! {
        for (i, bytes) in serialized.chunks_exact(10).enumerate() {
            hax_lib::loop_invariant!(|_i: usize| {
                spec::is_bounded_poly(4095, &re)
            });
            let coefficient = Vector::deserialize_5(bytes);
            // Intro: deserialize_5 post `forall j. bounded c[j] 5` -> trait pre
            // `bounded_pos_i16_array 5` (= `bounded_i16_array 0 31`).
            hax_lib::fstar!(
                r#"assert_norm (pow2 5 - 1 == 31);
                   Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
                     (mk_i16 0) (mk_i16 31)
                     (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient})"#
            );
            re.coefficients[i] = Vector::decompress_ciphertext_coefficient::<5>(coefficient);
            // Lift the strengthened trait post `bounded_i16_array 0 3328
            // (f_repr re.coefficients[i])` to `is_bounded_vector 4095
            // (re.coefficients[i])` so the loop invariant on `re` is
            // preserved by this iteration.
            #[cfg(hax)]
            spec::lemma_decompress_post_to_is_bounded_vector(&re.coefficients[i], 4095);
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
    spec::is_bounded_poly(4095, &result)
    & fstar!(r#"${poly_to_spec::<Vector>} $result ==
        Hacspec_ml_kem.Serialize.deserialize_then_decompress_v $serialized $COMPRESSION_FACTOR"#)
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
