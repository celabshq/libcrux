use libcrux_intrinsics::avx2::*;

#[hax_lib::ensures(|result| fstar!(r#"
      let open Spec.Intrinsics in
      forall (i: nat {i < 48}).
         (if i < 24 then result.(mk_int i) else result.(mk_int (128 + i - 24)))
      == (${simd_unit}.(mk_int ((i / 6) * 32 + i % 6)))
    "#
    ))]
#[hax_lib::fstar::options("--ifuel 0 --z3rlimit 380")]
#[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
#[inline(always)]
// This function is purely AVX2 operations, which makes the SMT proof with F* easy.
fn serialize_6(simd_unit: &Vec256) -> Vec256 {
    let adjacent_2_combined =
        mm256_sllv_epi32(*simd_unit, mm256_set_epi32(0, 26, 0, 26, 0, 26, 0, 26));
    let adjacent_2_combined = mm256_srli_epi64::<26>(adjacent_2_combined);

    let adjacent_3_combined = mm256_shuffle_epi8(
        adjacent_2_combined,
        mm256_set_epi8(
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 8, 1, 0, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, 9, 8, 1, 0,
        ),
    );

    let adjacent_3_combined = mm256_mullo_epi16(
        adjacent_3_combined,
        // Note: the explicit style `1i16 << N` matters for F* proofs here.
        mm256_set_epi16(
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 4,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 0,
            1i16 << 4,
        ),
    );
    mm256_srlv_epi32(adjacent_3_combined, mm256_set_epi32(0, 0, 0, 4, 0, 0, 0, 4))
}

#[hax_lib::fstar::options("--ifuel 0 --z3rlimit 380")]
#[hax_lib::ensures(|result| fstar!(r"
        let open Spec.Intrinsics in
          forall (i: nat{i < 32}).
                ${result}.(mk_int i) == simd_unit.(mk_int ((i / 4) * 32 + i % 4))
    "))]
#[inline(always)]
// This function is purely AVX2 operations, which makes the SMT proof with F* easy.
fn serialize_4(simd_unit: &Vec256) -> Vec128 {
    let adjacent_2_combined =
        mm256_sllv_epi32(*simd_unit, mm256_set_epi32(0, 28, 0, 28, 0, 28, 0, 28));
    let adjacent_2_combined = mm256_srli_epi64::<28>(adjacent_2_combined);

    let adjacent_4_combined =
        mm256_permutevar8x32_epi32(adjacent_2_combined, mm256_set_epi32(0, 0, 0, 0, 6, 2, 4, 0));
    let adjacent_4_combined = mm256_castsi256_si128(adjacent_4_combined);
    let adjacent_4_combined = mm_shuffle_epi8(
        adjacent_4_combined,
        mm_set_epi8(
            -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, 12, 4, 8, 0,
        ),
    );
    adjacent_4_combined
}

#[inline(always)]
#[hax_lib::requires(out.len() == 4 || out.len() == 6)]
#[hax_lib::ensures(|_result| fstar!(r"
  let open Spec.Intrinsics in
  let bytes = Seq.length out in
     Seq.length out_future == Seq.length out
  /\ (forall (i: nat {i < bytes * 8}). u8_to_bv out_future.[mk_usize (i / 8)] (mk_int (i % 8)) == simd_unit.(mk_int (i / bytes * 32 + i % bytes)))
"))]
pub(in crate::simd::avx2) fn serialize(simd_unit: &Vec256, out: &mut [u8]) {
    let mut serialized = [0u8; 19];

    match out.len() as u8 {
        4 => {
            mm_storeu_bytes_si128(&mut serialized[0..16], serialize_4(simd_unit));
            out.copy_from_slice(&serialized[0..4]);
        }

        6 => {
            let adjacent_3_combined = serialize_6(simd_unit);

            // We now have 24 bits starting at position 0 in the lower 128-bit lane, ...
            let lower_3 = mm256_castsi256_si128(adjacent_3_combined);
            // and 24 bits starting at position 0 in the upper 128-bit lane.
            let upper_3 = mm256_extracti128_si256::<1>(adjacent_3_combined);

            mm_storeu_bytes_si128(&mut serialized[0..16], lower_3);
            mm_storeu_bytes_si128(&mut serialized[3..19], upper_3);

            out.copy_from_slice(&serialized[0..6]);
        }

        _ => unreachable!(),
    }
}
