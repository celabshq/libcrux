use libcrux_intrinsics::avx2::*;

use crate::constants::BITS_IN_LOWER_PART_OF_T;

const POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE: i32 = 1 << (BITS_IN_LOWER_PART_OF_T - 1);

#[inline(always)]
fn change_interval(simd_unit: &Vec256) -> Vec256 {
    let interval_end = mm256_set1_epi32(POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE);

    mm256_sub_epi32(interval_end, *simd_unit)
}

#[inline(always)]
#[hax_lib::fstar::before("open Spec.Intrinsics")]
#[hax_lib::fstar::before(r#"
let mm256_add_epi64_lemma_smtpat lhs rhs (i: u64 {v i < 256})
  : Lemma
    (requires
      forall (j:nat{j < v i % 64}). Libcrux_core_models.Abstractions.Bit.Bit_Zero? lhs.(mk_int ((v i / 64) * 64 + j))
                         \/ Libcrux_core_models.Abstractions.Bit.Bit_Zero? rhs.(mk_int ((v i / 64) * 64 + j))
    )
    (ensures
      (Libcrux_core_models.Abstractions.Bit.Bit_Zero? lhs.(i) ==> (Libcrux_intrinsics.Avx2.mm256_add_epi64 lhs rhs).(i) == rhs.(i)) /\
      (Libcrux_core_models.Abstractions.Bit.Bit_Zero? rhs.(i) ==> (Libcrux_intrinsics.Avx2.mm256_add_epi64 lhs rhs).(i) == lhs.(i))
    )
    [SMTPat (Libcrux_intrinsics.Avx2.mm256_add_epi64 lhs rhs).(i)]
    = mm256_add_epi64_lemma lhs rhs i
"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 0 --z3rlimit 500")]
#[hax_lib::requires(fstar!(r#"forall i. v i % 32 >= 13 ==> ${simd_unit}.(i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero"#))]
#[hax_lib::ensures(|out|fstar!(r#"
forall (i: nat {i < 8}) (j: nat {j < 13}). ${out}.(mk_int (i * 13 + j)) == ${simd_unit}.(mk_int (i * 32 + j))
"#))]
// `serialize_aux` contains the AVX2-only pure operations.
// This split is required for the F* proof to go through.
pub(crate) fn serialize_aux(simd_unit: Vec256) -> Vec128 {
    let adjacent_2_combined =
        mm256_sllv_epi32(simd_unit, mm256_set_epi32(0, 19, 0, 19, 0, 19, 0, 19));
    let adjacent_2_combined = mm256_srli_epi64::<19>(adjacent_2_combined);

    let adjacent_4_combined =
        mm256_permutevar8x32_epi32(adjacent_2_combined, mm256_set_epi32(0, 0, 0, 0, 6, 4, 2, 0));
    let adjacent_4_combined =
        mm256_sllv_epi32(adjacent_4_combined, mm256_set_epi32(0, 6, 0, 6, 0, 6, 0, 6));
    let adjacent_4_combined = mm256_srli_epi64::<6>(adjacent_4_combined);

    let second_4_combined = mm256_bsrli_epi128::<8>(adjacent_4_combined);
    let least_12_bits_shifted_up = mm256_slli_epi64::<52>(second_4_combined);

    let bits_sequential = mm256_add_epi64(adjacent_4_combined, least_12_bits_shifted_up);
    let bits_sequential = mm256_srlv_epi64(bits_sequential, mm256_set_epi64x(0, 0, 12, 0));

    mm256_castsi256_si128(bits_sequential)
}

#[inline(always)]
#[hax_lib::fstar::options(r#"--ifuel 0 --z3rlimit 340 --split_queries always"#)]
#[hax_lib::requires(fstar!(r#"forall i. let x = (v $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE - v (to_i32x8 $simd_unit i)) in x >= 0 && x < pow2 13"#))]
#[hax_lib::ensures(|_result| fstar!(r#"
      Seq.length ${out}_future == 13
    /\ (forall (i:nat{i < 8 * 13}).
      u8_to_bv (Seq.index ${out}_future (i / 8)) (mk_int (i % 8))
   == i32_to_bv (         $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE
                `sub_mod` to_i32x8 $simd_unit (mk_int (i / 13))) (mk_int (i % 13)))
"#))]
pub(crate) fn serialize(simd_unit: &Vec256, out: &mut [u8]) {
    let mut serialized = [0u8; 16];

    let simd_unit_changed = change_interval(simd_unit);

    hax_lib::fstar!("i32_lt_pow2_n_to_bit_zero_lemma 13 $simd_unit_changed");
    hax_lib::fstar!("reveal_opaque_arithmetic_ops #I32");
    let bits_sequential = serialize_aux(simd_unit_changed);
    mm_storeu_bytes_si128(&mut serialized, bits_sequential);

    hax_lib::fstar!(
        r"
  assert(forall (i:nat{i < 104}). to_i32x8 $simd_unit_changed (mk_int (i / 13))
       == $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE `sub_mod` to_i32x8 $simd_unit (mk_int (i / 13)));
  assert(forall i. $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE `sub_mod` to_i32x8 $simd_unit i
       == $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE -! to_i32x8 $simd_unit i)
"
    );

    out.copy_from_slice(&serialized[0..13])
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let deserialize_unsigned_post
  (serialized: t_Slice u8{Seq.length serialized == 13})
  (result: bv256)
  = let bytes = 13 in
    (forall (i: nat{i < bytes * 8}).
       u8_to_bv serialized.[ mk_usize (i / 8) ] (mk_int (i % 8)) ==
       result.(mk_int ((i / bytes) * 32 + i % bytes))) /\
    (forall (i: nat{i < 256}).
       i % 32 >= bytes ==> Libcrux_core_models.Abstractions.Bit.Bit_Zero? result.(mk_int i))
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(serialized.len() == 13)]
#[hax_lib::ensures(|_result| fstar!("deserialize_unsigned_post $serialized ${out}_future"))]
fn deserialize_unsigned(serialized: &[u8], out: &mut Vec256) {
    const COEFFICIENT_MASK: i32 = (1 << 13) - 1;

    let mut serialized_extended = [0u8; 16];
    serialized_extended[0..13].copy_from_slice(serialized);

    let serialized = mm_loadu_si128(&serialized_extended);
    let serialized = mm256_set_m128i(serialized, serialized);

    // XXX: re-use out variable
    let coefficients = mm256_shuffle_epi8(
        serialized,
        mm256_set_epi8(
            -1, -1, 12, 11, -1, 11, 10, 9, -1, -1, 9, 8, -1, 8, 7, 6, -1, 6, 5, 4, -1, -1, 4, 3,
            -1, 3, 2, 1, -1, -1, 1, 0,
        ),
    );

    let coefficients = mm256_srlv_epi32(coefficients, mm256_set_epi32(3, 6, 1, 4, 7, 2, 5, 0));
    let coefficients = mm256_and_si256(coefficients, mm256_set1_epi32(COEFFICIENT_MASK));
    hax_lib::fstar!("i32_to_bv_pow2_min_one_lemma_fa 13");
    *out = coefficients
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let deserialize_post
         (serialized: t_Slice u8 {Seq.length serialized == 13})
         (result: bv256)
    = (forall i. v (to_i32x8 result i) > minint I32)
    /\ ( let out_reverted = mk_i32x8 (fun i -> neg (to_i32x8 result i) `add_mod` $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE) in
        deserialize_unsigned_post serialized out_reverted)
"#
)]
#[hax_lib::requires(serialized.len() == 13)]
#[hax_lib::ensures(|result| fstar!("deserialize_post $serialized ${out}_future"))]
pub(crate) fn deserialize(serialized: &[u8], out: &mut Vec256) {
    #[cfg(not(eurydice))]
    debug_assert_eq!(serialized.len(), 13);
    deserialize_unsigned(serialized, out);
    #[cfg(hax)]
    let unsigned = out.clone();
    *out = change_interval(out);
    hax_lib::fstar!(
        r"
    i32_bit_zero_lemma_to_lt_pow2_n_weak 13 $unsigned;
    reveal_opaque_arithmetic_ops #I32;
    let out_reverted: bv256 = mk_i32x8 (fun i -> neg (to_i32x8 $out i) `add_mod` $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE) in
    introduce forall i. neg (to_i32x8 out i) `add_mod` $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE == to_i32x8 $unsigned i
    with rewrite_eq_sub_mod (to_i32x8 out i) $POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE (to_i32x8 $unsigned i);
    to_i32x8_eq_to_bv_eq $unsigned out_reverted;
    assert_norm (deserialize_post $serialized $out == ((forall i. v (to_i32x8 out i) > minint I32) /\ deserialize_unsigned_post $serialized out_reverted))
    "
    );
}
