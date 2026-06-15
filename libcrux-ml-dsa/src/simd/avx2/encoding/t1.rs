use libcrux_intrinsics::avx2::*;

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800")]
#[hax_lib::fstar::before("open Spec.Intrinsics")]
#[hax_lib::requires(out.len() == 10)]
#[hax_lib::ensures(|_result| fstar!(r"
   Seq.length ${out}_future == 10 /\
   (forall (i:nat).
     i < 80 ==>
       ${simd_unit}.(mk_int (32*(i/10) + (i%10))) == (u8_to_bv (Seq.index ${out}_future (i/8)))(mk_int (i % 8)))
"))]
pub(crate) fn serialize(simd_unit: &Vec256, out: &mut [u8]) {
    #[cfg(not(eurydice))]
    debug_assert!(out.len() == 10);

    let mut serialized = [0u8; 24];

    let adjacent_2_combined =
        mm256_sllv_epi32(*simd_unit, mm256_set_epi32(0, 22, 0, 22, 0, 22, 0, 22));
    let adjacent_2_combined = mm256_srli_epi64::<22>(adjacent_2_combined);

    let adjacent_4_combined =
        mm256_permutevar8x32_epi32(adjacent_2_combined, mm256_set_epi32(0, 0, 6, 4, 0, 0, 2, 0));
    let adjacent_4_combined = mm256_sllv_epi32(
        adjacent_4_combined,
        mm256_set_epi32(0, 12, 0, 12, 0, 12, 0, 12),
    );
    let adjacent_4_combined = mm256_srli_epi64::<12>(adjacent_4_combined);

    // We now have 40 bits starting at position 0 in the lower 128-bit lane, ...
    let lower_4 = mm256_castsi256_si128(adjacent_4_combined);
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_4);

    // and 40 bits starting at position 0 in the upper 128-bit lane.
    let upper_4 = mm256_extracti128_si256::<1>(adjacent_4_combined);
    mm_storeu_bytes_si128(&mut serialized[5..21], upper_4);

    out.copy_from_slice(&serialized[0..10]);
}

#[inline(always)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 300")]
#[hax_lib::fstar::before("open Spec.Intrinsics")]
#[hax_lib::requires(bytes.len() == 10)]
#[hax_lib::ensures(|_result| fstar!(r"
  (forall (i:nat).
    i < 80 ==>
      (u8_to_bv (Seq.index $bytes (i/8)))(mk_int (i%8)) == ${out}_future.(mk_int (32*(i/10) + (i%10)))
  ) /\ (
   forall (j:nat).
    j < 256 ==>
    j % 32 > 10 ==>
       ${out}_future.(mk_int j) == Libcrux_core_models.Abstractions.Bit.Bit_Zero
  )"))]
pub(crate) fn deserialize(bytes: &[u8], out: &mut Vec256) {
    #[cfg(not(eurydice))]
    debug_assert_eq!(bytes.len(), 10);

    const COEFFICIENT_MASK: i32 = (1 << 10) - 1;

    let mut bytes_extended = [0u8; 16];
    bytes_extended[0..10].copy_from_slice(bytes);

    let bytes_loaded = mm_loadu_si128(&bytes_extended);
    let bytes_loaded = mm256_set_m128i(bytes_loaded, bytes_loaded);

    // XXX: re-use out
    let coefficients = mm256_shuffle_epi8(
        bytes_loaded,
        mm256_set_epi8(
            -1, -1, 9, 8, -1, -1, 8, 7, -1, -1, 7, 6, -1, -1, 6, 5, -1, -1, 4, 3, -1, -1, 3, 2, -1,
            -1, 2, 1, -1, -1, 1, 0,
        ),
    );

    let coefficients = mm256_srlv_epi32(coefficients, mm256_set_epi32(6, 4, 2, 0, 6, 4, 2, 0));

    *out = mm256_and_si256(coefficients, mm256_set1_epi32(COEFFICIENT_MASK));
}
