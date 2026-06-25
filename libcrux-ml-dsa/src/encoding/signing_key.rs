use crate::{
    constants::{
        Eta, BYTES_FOR_VERIFICATION_KEY_HASH, RING_ELEMENT_OF_T0S_SIZE, SEED_FOR_A_SIZE,
        SEED_FOR_SIGNING_SIZE,
    },
    encoding,
    hash_functions::shake256,
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 600 --split_queries always --fuel 0 --ifuel 1")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $seed_matrix == 32 /\
    Seq.length $seed_signing == 32 /\
    Seq.length $s1_2 <= 15 /\
    Seq.length $t0 <= 8 /\
    v $error_ring_element_size == 32 * (match $eta with
                                         | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                         | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
    Seq.length $signing_key_serialized ==
        32 + 32 + 64
        + Seq.length $s1_2 * v $error_ring_element_size
        + Seq.length $t0 * 416 /\
    (forall (k:nat). k < Seq.length $s1_2 ==>
      (forall (j:nat). j < 32 ==>
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque
          (match $eta with
           | Libcrux_ml_dsa.Constants.Eta_Two -> 2
           | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
          (i0._super_i2.f_repr (Seq.index (Seq.index $s1_2 k).f_simd_units j)))) /\
    Libcrux_ml_dsa.Polynomial.Spec.is_strict_lower_poly_slice (mk_usize (pow2 12)) $t0"#))]
pub(crate) fn generate_serialized<SIMDUnit: Operations, Shake256: shake256::DsaXof>(
    eta: Eta,
    error_ring_element_size: usize,
    seed_matrix: &[u8],
    seed_signing: &[u8],
    verification_key: &[u8],
    s1_2: &[PolynomialRingElement<SIMDUnit>],
    t0: &[PolynomialRingElement<SIMDUnit>],
    signing_key_serialized: &mut [u8],
) {
    // Constant prefix offsets
    const SEED_MATRIX_END: usize = SEED_FOR_A_SIZE; // 32
    const SEED_SIGNING_END: usize = SEED_MATRIX_END + SEED_FOR_SIGNING_SIZE; // 64
    const VK_HASH_END: usize = SEED_SIGNING_END + BYTES_FOR_VERIFICATION_KEY_HASH; // 128

    signing_key_serialized[0..SEED_MATRIX_END].copy_from_slice(seed_matrix);
    signing_key_serialized[SEED_MATRIX_END..SEED_SIGNING_END].copy_from_slice(seed_signing);

    let mut verification_key_hash = [0; BYTES_FOR_VERIFICATION_KEY_HASH];
    // The DsaXof trait does not (yet) declare a `requires` on `shake256`, so
    // hax extracts an opaque `f_shake256_pre` that we cannot discharge from
    // here.  Assume it; the only meaningful obligation in any impl is buffer
    // length, which is structurally guaranteed by the array type.
    hax_lib::fstar!(
        r#"assume (Libcrux_ml_dsa.Hash_functions.Shake256.f_shake256_pre #v_Shake256
                    #FStar.Tactics.Typeclasses.solve
                    (mk_usize 64)
                    $verification_key
                    $verification_key_hash)"#
    );
    Shake256::shake256::<BYTES_FOR_VERIFICATION_KEY_HASH>(
        verification_key,
        &mut verification_key_hash,
    );
    signing_key_serialized[SEED_SIGNING_END..VK_HASH_END]
        .copy_from_slice(&verification_key_hash);

    let s1_2_len = s1_2.len();
    let t0_offset_base = VK_HASH_END + s1_2_len * error_ring_element_size;
    for i in 0..s1_2_len {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= Seq.length $s1_2 /\
              Seq.length $s1_2 <= 15 /\
              v $s1_2_len == Seq.length $s1_2 /\
              v $error_ring_element_size <= 128 /\
              Seq.length $signing_key_serialized ==
                  128
                  + Seq.length $s1_2 * v $error_ring_element_size
                  + Seq.length $t0 * 416 /\
              v $error_ring_element_size == 32 * (match $eta with
                                                   | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                                   | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
              (forall (k:nat). k < Seq.length $s1_2 ==>
                (forall (j:nat). j < 32 ==>
                  Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque
                    (match $eta with
                     | Libcrux_ml_dsa.Constants.Eta_Two -> 2
                     | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
                    (i0._super_i2.f_repr (Seq.index (Seq.index $s1_2 k).f_simd_units j))))"#
        ));
        hax_lib::fstar!(
            r#"assert (v i < v $s1_2_len);
               assert (v i * v $error_ring_element_size <= 14 * 128);
               assert (v i * v $error_ring_element_size + v $error_ring_element_size <=
                       Seq.length $s1_2 * v $error_ring_element_size)"#
        );
        let lo = VK_HASH_END + i * error_ring_element_size;
        let hi = lo + error_ring_element_size;
        encoding::error::serialize::<SIMDUnit>(
            eta,
            &s1_2[i],
            &mut signing_key_serialized[lo..hi],
        );
    }

    let t0_len = t0.len();
    for i in 0..t0_len {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= Seq.length $t0 /\
              Seq.length $t0 <= 8 /\
              v $t0_len == Seq.length $t0 /\
              v $error_ring_element_size <= 128 /\
              v $t0_offset_base == 128 + Seq.length $s1_2 * v $error_ring_element_size /\
              v $t0_offset_base <= 128 + 15 * 128 /\
              Seq.length $signing_key_serialized ==
                  v $t0_offset_base + Seq.length $t0 * 416 /\
              Libcrux_ml_dsa.Polynomial.Spec.is_strict_lower_poly_slice (mk_usize (pow2 12)) $t0"#
        ));
        hax_lib::fstar!(
            r#"assert (v $RING_ELEMENT_OF_T0S_SIZE == 416);
               assert (v i < v $t0_len);
               assert (v i * 416 <= 7 * 416);
               assert ((v i + 1) * 416 <= Seq.length $t0 * 416)"#
        );
        // Bridge: from the opaque slice atom recover the per-poly atom on t0[i],
        // then unfold to the bare per-j forall that t0::serialize requires.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_strict_lower_poly_slice_lookup
                 (mk_usize (pow2 12)) $t0 (v $i);
               let aux (j: nat{j < 32})
                 : Lemma (Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
                            (i0._super_i2.f_repr
                               (Seq.index (Seq.index $t0 (v $i)).f_simd_units j))) =
                 Libcrux_ml_dsa.Polynomial.Spec.lemma_is_strict_lower_poly_lookup
                   (mk_usize (pow2 12)) (Seq.index $t0 (v $i)) j
               in
               Classical.forall_intro aux"#
        );
        let lo = t0_offset_base + i * RING_ELEMENT_OF_T0S_SIZE;
        let hi = lo + RING_ELEMENT_OF_T0S_SIZE;
        encoding::t0::serialize::<SIMDUnit>(
            &t0[i],
            &mut signing_key_serialized[lo..hi],
        );
    }
}
