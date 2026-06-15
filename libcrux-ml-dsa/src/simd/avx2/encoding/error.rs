use libcrux_intrinsics::avx2::*;

use crate::simd::avx2::Eta;

#[inline(always)]
#[hax_lib::fstar::before("open Spec.Intrinsics")]
#[hax_lib::fstar::options(
    "--fuel 0 --ifuel 0 --z3rlimit 5000 --z3smtopt '(set-option :smt.arith.nl false)'"
)]
#[hax_lib::requires(
    fstar!(r"forall (i: nat {i < 256}). i % 32 >= 3 ==> ${simd_unit_shifted}.(mk_int i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero")
)]
#[hax_lib::ensures(|result| {
    fstar!(r"forall (i:nat{i < 24}). ${result}.(mk_int i) == ${simd_unit_shifted}.(mk_int ((i / 3) * 32 + i % 3))")
})]
// `serialize_when_eta_is_2_aux` contains the AVX2-only pure operations.
// This split is required for the F* proof to go through.
fn serialize_when_eta_is_2_aux(simd_unit_shifted: Vec256) -> Vec128 {
    let adjacent_2_combined = mm256_sllv_epi32(
        simd_unit_shifted,
        mm256_set_epi32(0, 29, 0, 29, 0, 29, 0, 29),
    );
    let adjacent_2_combined = mm256_srli_epi64::<29>(adjacent_2_combined);

    let adjacent_4_combined = mm256_shuffle_epi8(
        adjacent_2_combined,
        mm256_set_epi8(
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 8, -1, 0, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, 8, -1, 0,
        ),
    );
    let adjacent_4_combined = mm256_madd_epi16(
        adjacent_4_combined,
        mm256_set_epi16(0, 0, 0, 0, 0, 0, 1 << 6, 1, 0, 0, 0, 0, 0, 0, 1 << 6, 1),
    );

    let adjacent_6_combined =
        mm256_permutevar8x32_epi32(adjacent_4_combined, mm256_set_epi32(0, 0, 0, 0, 0, 0, 4, 0));
    let adjacent_6_combined = mm256_castsi256_si128(adjacent_6_combined);

    let adjacent_6_combined = mm_sllv_epi32(adjacent_6_combined, mm_set_epi32(0, 0, 0, 20));
    let adjacent_6_combined = mm_srli_epi64::<20>(adjacent_6_combined);

    adjacent_6_combined
}

const ETA_2: i32 = 2;

#[inline(always)]
#[hax_lib::requires(fstar!("forall i. let x = (v $ETA_2 - v (to_i32x8 simd_unit i)) in x >= 0 && x <= 7"))]
#[hax_lib::ensures(|_result| fstar!(r#"
     Seq.length ${out}_future == 3
  /\ (forall (i:nat{i < 24}). u8_to_bv (Seq.index ${out}_future (i / 8)) (mk_int (i % 8))
                   == i32_to_bv ($ETA_2 -! to_i32x8 $simd_unit (mk_int (i / 3))) (mk_int (i % 3)))
"#))]
fn serialize_when_eta_is_2(simd_unit: &Vec256, out: &mut [u8]) {
    let mut serialized = [0u8; 16];

    let simd_unit_shifted = mm256_sub_epi32(mm256_set1_epi32(ETA_2), *simd_unit);

    hax_lib::fstar!("reveal_opaque_arithmetic_ops #I32");
    hax_lib::fstar!("i32_lt_pow2_n_to_bit_zero_lemma 3 $simd_unit_shifted");
    let adjacent_6_combined = serialize_when_eta_is_2_aux(simd_unit_shifted);

    hax_lib::fstar!("assert(forall (i:nat{i < 24}). to_i32x8 $simd_unit_shifted (mk_int (i / 3)) == mk_int 2 `sub_mod` to_i32x8 $simd_unit (mk_int (i / 3)))");
    hax_lib::fstar!("assert(forall i. mk_int 2 `sub_mod` to_i32x8 simd_unit i == mk_int 2 -! to_i32x8 simd_unit i)");

    mm_storeu_bytes_si128(&mut serialized[0..16], adjacent_6_combined);
    out.copy_from_slice(&serialized[0..3]);
}

#[inline(always)]
// `serialize_when_eta_is_4_aux` contains the AVX2-only pure operations.
// This split makes the F* proof much faster and stable (seconds versus about a minute).
#[hax_lib::requires(
    fstar!(r"forall (i: nat {i < 256}). i % 32 >= 4 ==> ${simd_unit_shifted}.(mk_int i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero")
)]
#[hax_lib::ensures(|result| {
    fstar!(r"forall (i:nat{i < 32}). ${result}.(mk_int i) == ${simd_unit_shifted}.(mk_int ((i / 4) * 32 + i % 4))")
})]
fn serialize_when_eta_is_4_aux(simd_unit_shifted: Vec256) -> Vec128 {
    let adjacent_2_combined = mm256_sllv_epi32(
        simd_unit_shifted,
        mm256_set_epi32(0, 28, 0, 28, 0, 28, 0, 28),
    );
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

const ETA_4: i32 = 4;

#[inline(always)]
#[hax_lib::requires(fstar!("forall i. let x = (v $ETA_4 - v (to_i32x8 simd_unit i)) in x >= 0 && x <= 15"))]
#[hax_lib::ensures(|_result| fstar!(r#"
     Seq.length ${out}_future == 4
  /\ (forall (i:nat{i < 32}). u8_to_bv (Seq.index ${out}_future (i / 8)) (mk_int (i % 8))
                   == i32_to_bv ($ETA_4 -! to_i32x8 $simd_unit (mk_int (i / 4))) (mk_int (i % 4)))
"#))]
#[hax_lib::fstar::options("--split_queries always")]
fn serialize_when_eta_is_4(simd_unit: &Vec256, out: &mut [u8]) {
    let mut serialized = [0u8; 16];

    let simd_unit_shifted = mm256_sub_epi32(mm256_set1_epi32(ETA_4), *simd_unit);

    hax_lib::fstar!("reveal_opaque_arithmetic_ops #I32");
    hax_lib::fstar!("i32_lt_pow2_n_to_bit_zero_lemma 4 $simd_unit_shifted");
    let adjacent_4_combined = serialize_when_eta_is_4_aux(simd_unit_shifted);

    hax_lib::fstar!("assert(forall (i:nat{i < 32}). to_i32x8 $simd_unit_shifted (mk_int (i / 4)) == mk_int 4 `sub_mod` to_i32x8 $simd_unit (mk_int (i / 4)))");
    hax_lib::fstar!("assert(forall i. mk_int 4 `sub_mod` to_i32x8 simd_unit i == mk_int 4 -! to_i32x8 simd_unit i)");

    mm_storeu_bytes_si128(&mut serialized[0..16], adjacent_4_combined);

    out.copy_from_slice(&serialized[0..4])
}

#[hax_lib::requires(
    fstar!("forall i. let x = (v (${eta as u8}) - v (to_i32x8 simd_unit i)) in x >= 0 && x <= (pow2 (v (${eta as u8})) - 1)")
)]
#[hax_lib::ensures(|_result| {
    let bytes = match eta {
        Eta::Two => 3,
        Eta::Four => 4,
    };
    fstar!(r#"
          Seq.length ${serialized}_future == v $bytes
       /\ (forall (i:nat{i < v $bytes * 8}).
              u8_to_bv (Seq.index ${serialized}_future (i / 8)) (mk_int (i % 8))
           == i32_to_bv ((${eta as i32}) -! to_i32x8 $simd_unit (mk_int (i / v $bytes))) (mk_int (i % v $bytes)))
    "#)
})]
#[inline(always)]
pub fn serialize(eta: Eta, simd_unit: &Vec256, serialized: &mut [u8]) {
    // [eurydice] injects an unused variable here in the C code for some reason.
    match eta {
        Eta::Two => serialize_when_eta_is_2(simd_unit, serialized),
        Eta::Four => serialize_when_eta_is_4(simd_unit, serialized),
    }
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 3)]
#[hax_lib::ensures(|result| fstar!(r#"
  (forall (i: nat {i <  24}). u8_to_bv ${bytes}.[mk_usize (i / 8)] (mk_int (i % 8)) == ${result}.(mk_int (i / 3 * 32 + i % 3)))
/\ (forall (i: nat {i < 256}). i % 32 >= 3 ==> Libcrux_core_models.Abstractions.Bit.Bit_Zero? ${result}.(mk_int i))
"#))]
fn deserialize_to_unsigned_when_eta_is_2(bytes: &[u8]) -> Vec256 {
    #[cfg(not(eurydice))]
    debug_assert!(bytes.len() == 3);

    const COEFFICIENT_MASK: i32 = (1 << 3) - 1;

    let bytes_in_simd_unit = mm256_set_epi32(
        bytes[2] as i32,
        bytes[2] as i32,
        ((bytes[2] as i32) << 8) | (bytes[1] as i32),
        bytes[1] as i32,
        bytes[1] as i32,
        ((bytes[1] as i32) << 8) | (bytes[0] as i32),
        bytes[0] as i32,
        bytes[0] as i32,
    );

    let coefficients =
        mm256_srlv_epi32(bytes_in_simd_unit, mm256_set_epi32(5, 2, 7, 4, 1, 6, 3, 0));

    mm256_and_si256(coefficients, mm256_set1_epi32(COEFFICIENT_MASK))
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 4)]
#[hax_lib::ensures(|result| fstar!(r#"
  (forall (i: nat {i <  32}). u8_to_bv ${bytes}.[mk_usize (i / 8)] (mk_int (i % 8)) == ${result}.(mk_int (i / 4 * 32 + i % 4)))
/\ (forall (i: nat {i < 256}). i % 32 >= 4 ==> Libcrux_core_models.Abstractions.Bit.Bit_Zero? ${result}.(mk_int i))
"#))]
fn deserialize_to_unsigned_when_eta_is_4(bytes: &[u8]) -> Vec256 {
    #[cfg(not(eurydice))]
    debug_assert!(bytes.len() == 4);

    const COEFFICIENT_MASK: i32 = (1 << 4) - 1;

    let bytes_in_simd_unit = mm256_set_epi32(
        bytes[3] as i32,
        bytes[3] as i32,
        bytes[2] as i32,
        bytes[2] as i32,
        bytes[1] as i32,
        bytes[1] as i32,
        bytes[0] as i32,
        bytes[0] as i32,
    );

    let coefficients =
        mm256_srlv_epi32(bytes_in_simd_unit, mm256_set_epi32(4, 0, 4, 0, 4, 0, 4, 0));

    mm256_and_si256(coefficients, mm256_set1_epi32(COEFFICIENT_MASK))
}

#[inline(always)]
#[hax_lib::fstar::before(r#"
let deserialize_to_unsigned_post
  (eta: Libcrux_ml_dsa.Constants.t_Eta)
  (serialized: t_Slice u8{Seq.length serialized == (match eta with | Libcrux_ml_dsa.Constants.Eta_Two  -> 3 | Libcrux_ml_dsa.Constants.Eta_Four -> 4)})
  (result: bv256)
  = let bytes = Seq.length serialized in
    (forall (i: nat{i < bytes * 8}).
       u8_to_bv serialized.[ mk_usize (i / 8) ] (mk_int (i % 8)) ==
       result.(mk_int ((i / bytes) * 32 + i % bytes))) /\
    (forall (i: nat{i < 256}).
       i % 32 >= bytes ==> Libcrux_core_models.Abstractions.Bit.Bit_Zero? result.(mk_int i))
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(serialized.len() == match eta {
    Eta::Two => 3,
    Eta::Four => 4,
})]
#[hax_lib::ensures(|result| fstar!("deserialize_to_unsigned_post $eta $serialized $result"))]
pub(crate) fn deserialize_to_unsigned(eta: Eta, serialized: &[u8]) -> Vec256 {
    match eta {
        Eta::Two => deserialize_to_unsigned_when_eta_is_2(serialized),
        Eta::Four => deserialize_to_unsigned_when_eta_is_4(serialized),
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"
module C = Libcrux_ml_dsa.Constants
let deserialize_post (eta: C.t_Eta)
         (serialized: t_Slice u8 {Seq.length serialized == (match eta with | C.Eta_Two  -> 3 | C.Eta_Four -> 4)})
         (result: bv256)
    = let eta_i32:i32 = match eta <: C.t_Eta with | C.Eta_Two  -> mk_i32 2 | C.Eta_Four -> mk_i32 4 in
      let bytes = Seq.length serialized in
      (forall i. v (to_i32x8 result i) > minint I32)
    /\ ( let out_reverted = mk_i32x8 (fun i -> neg (to_i32x8 result i) `add_mod` eta_i32) in
        deserialize_to_unsigned_post eta serialized out_reverted)
"#)]
#[hax_lib::requires(serialized.len() == match eta {
    Eta::Two => 3,
    Eta::Four => 4,
})]
#[hax_lib::ensures(|result| fstar!("deserialize_post $eta $serialized ${out}_future"))]
pub(crate) fn deserialize(eta: Eta, serialized: &[u8], out: &mut Vec256) {
    let unsigned = deserialize_to_unsigned(eta, serialized);

    // [eurydice]: https://github.com/AeneasVerif/eurydice/issues/122
    let eta_v = match eta {
        Eta::Two => 2,
        Eta::Four => 4,
    };
    *out = mm256_sub_epi32(mm256_set1_epi32(eta_v), unsigned);
    hax_lib::fstar!(
        r"
    i32_bit_zero_lemma_to_lt_pow2_n_weak 4 $unsigned;
    reveal_opaque_arithmetic_ops #I32;
    let out_reverted: bv256 = mk_i32x8 (fun i -> neg (to_i32x8 $out i) `add_mod` $eta_v) in
    introduce forall i. neg (to_i32x8 out i) `add_mod` $eta_v == to_i32x8 $unsigned i
    with rewrite_eq_sub_mod (to_i32x8 out i) $eta_v (to_i32x8 $unsigned i);
    to_i32x8_eq_to_bv_eq $unsigned out_reverted;
    assert_norm (deserialize_post $eta $serialized $out == ((forall i. v (to_i32x8 out i) > minint I32) /\ deserialize_to_unsigned_post $eta $serialized out_reverted))
    "
    );
}
