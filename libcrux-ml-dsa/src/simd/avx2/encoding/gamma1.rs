use libcrux_intrinsics::avx2::*;

#[inline(always)]
#[hax_lib::fstar::before(r#"
open Spec.Intrinsics
let lemma_mm256_add_epi64_lemma_weaker lhs rhs (i: u64 {v i < 256})
  : Lemma
    (requires forall i. Libcrux_core_models.Abstractions.Bit.Bit_Zero? lhs.(i) \/ Libcrux_core_models.Abstractions.Bit.Bit_Zero? rhs.(i))
    (ensures (Libcrux_core_models.Abstractions.Bit.Bit_Zero? lhs.(i) ==> (Libcrux_intrinsics.Avx2.mm256_add_epi64 lhs rhs).(i) == rhs.(i))
           /\ (Libcrux_core_models.Abstractions.Bit.Bit_Zero? rhs.(i) ==> (Libcrux_intrinsics.Avx2.mm256_add_epi64 lhs rhs).(i) == lhs.(i)))
    [SMTPat (Libcrux_intrinsics.Avx2.mm256_add_epi64 lhs rhs).(i)]
    = Spec.Intrinsics.mm256_add_epi64_lemma lhs rhs i
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"forall i. v i % 32 >= 18 ==> simd_unit_shifted.(i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero"#))]
#[hax_lib::ensures(|result| fstar!(r#"
forall (i: nat {i < 8}) (j: nat {j < 18}).
  let offset = if i >= 4 then 56 else 0 in
  ${result}.(mk_int (i * 18 + j + offset)) == ${simd_unit_shifted}.(mk_int (i * 32 + j))
"#))]
#[hax_lib::fstar::options("--fuel 0 --ifuel 0 --z3rlimit 100")]
// `serialize_when_gamma1_is_2_pow_17_aux` contains the AVX2-only pure operations.
// This split is required for the F* proof to go through.
fn serialize_when_gamma1_is_2_pow_17_aux(simd_unit_shifted: Vec256) -> Vec256 {
    let adjacent_2_combined = mm256_sllv_epi32(
        simd_unit_shifted,
        mm256_set_epi32(0, 14, 0, 14, 0, 14, 0, 14),
    );
    let adjacent_2_combined = mm256_srli_epi64::<14>(adjacent_2_combined);

    let every_second_element = mm256_bsrli_epi128::<8>(adjacent_2_combined);
    let every_second_element_shifted = mm256_slli_epi64::<36>(every_second_element);

    let adjacent_4_combined = mm256_add_epi64(adjacent_2_combined, every_second_element_shifted);
    let adjacent_4_combined = mm256_srlv_epi64(adjacent_4_combined, mm256_set_epi64x(28, 0, 28, 0));
    adjacent_4_combined
}

const GAMMA1_2_POW_17: i32 = 1 << 17;

#[inline(always)]
#[hax_lib::fstar::options(r#"--ifuel 0 --z3rlimit 140 --split_queries always"#)]
#[hax_lib::requires(fstar!(r#"forall i. let x = (v ${GAMMA1_2_POW_17} - v (to_i32x8 $simd_unit i)) in x >= 0 && x < pow2 18"#))]
#[hax_lib::ensures(|_result| fstar!(r#"
      Seq.length ${out}_future == 18
    /\ (forall (i:nat{i < 144}).
      u8_to_bv (Seq.index ${out}_future (i / 8)) (mk_int (i % 8))
   == i32_to_bv (${GAMMA1_2_POW_17} `sub_mod` to_i32x8 $simd_unit (mk_int (i / 18))) (mk_int (i % 18)))
"#))]
fn serialize_when_gamma1_is_2_pow_17(simd_unit: &Vec256, out: &mut [u8]) {
    let mut serialized = [0u8; 32];

    let simd_unit_shifted = mm256_sub_epi32(mm256_set1_epi32(GAMMA1_2_POW_17), *simd_unit);
    hax_lib::fstar!("i32_lt_pow2_n_to_bit_zero_lemma 18 $simd_unit_shifted");
    hax_lib::fstar!("reveal_opaque_arithmetic_ops #I32");
    let adjacent_4_combined = serialize_when_gamma1_is_2_pow_17_aux(simd_unit_shifted);

    let lower_4 = mm256_castsi256_si128(adjacent_4_combined);
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_4);

    let upper_4 = mm256_extracti128_si256::<1>(adjacent_4_combined);
    mm_storeu_bytes_si128(&mut serialized[9..25], upper_4);

    hax_lib::fstar!(
        r#"
  // Thunk the two proofs, so that Z3 check them in parallel, and the SMT context is not polluted
  let spec (i:nat{i < 144}) = u8_to_bv (Seq.index $serialized (i / 8)) (mk_int (i % 8))
            == i32_to_bv (${GAMMA1_2_POW_17} `sub_mod` to_i32x8 $simd_unit (mk_int (i / 18))) (mk_int (i % 18)) in
  let proof: squash (forall i. spec i) =
    assert (forall (i:nat{i < 8}). to_i32x8 $simd_unit_shifted (mk_int ((i / 4) * 4 + i % 4)) == ${GAMMA1_2_POW_17} `sub_mod` to_i32x8 $simd_unit (mk_int i));
    let proof_72 (): squash (forall i. i < 72 ==> spec i) = () in
    let proof_144 (): squash (forall i. i >= 72 ==> spec i) = () in
    proof_72 (); proof_144 ()
  in ()
    "#
    );

    out.copy_from_slice(&serialized[0..18]);
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"forall i. v i % 32 >= 20 ==> ${simd_unit_shifted}.(i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero"#))]
#[hax_lib::ensures(|result| fstar!(r#"
forall (i: nat {i < 8}) (j: nat {j < 20}).
       ${result}.(mk_int ((if i >= 4 then 48 else 0) + i * 20 + j))
    == ${simd_unit_shifted}.(mk_int (i * 32 + j))
"#))]
// `serialize_when_gamma1_is_2_pow_19_aux` contains the AVX2-only pure operations.
// This split is required for the F* proof to go through.
fn serialize_when_gamma1_is_2_pow_19_aux(simd_unit_shifted: Vec256) -> Vec256 {
    let adjacent_2_combined = mm256_sllv_epi32(
        simd_unit_shifted,
        mm256_set_epi32(0, 12, 0, 12, 0, 12, 0, 12),
    );
    let adjacent_2_combined = mm256_srli_epi64::<12>(adjacent_2_combined);

    let adjacent_4_combined = mm256_shuffle_epi8(
        adjacent_2_combined,
        mm256_set_epi8(
            -1, -1, -1, -1, -1, -1, 12, 11, 10, 9, 8, 4, 3, 2, 1, 0, -1, -1, -1, -1, -1, -1, 12,
            11, 10, 9, 8, 4, 3, 2, 1, 0,
        ),
    );
    adjacent_4_combined
}

const GAMMA1_2_POW_19: i32 = 1 << 19;
#[inline(always)]
#[hax_lib::fstar::options(r#"--ifuel 0 --z3rlimit 140 --split_queries always"#)]
#[hax_lib::requires(fstar!(r#"forall i. let x = (v ${GAMMA1_2_POW_19} - v (to_i32x8 $simd_unit i)) in x >= 0 && x < pow2 20"#))]
#[hax_lib::ensures(|_result| fstar!(r#"
      Seq.length ${out}_future == 20
    /\ (forall (i:nat{i < 160}).
      u8_to_bv (Seq.index ${out}_future (i / 8)) (mk_int (i % 8))
   == i32_to_bv (${GAMMA1_2_POW_19} `sub_mod` to_i32x8 $simd_unit (mk_int (i / 20))) (mk_int (i % 20)))
"#))]
fn serialize_when_gamma1_is_2_pow_19(simd_unit: &Vec256, out: &mut [u8]) {
    let mut serialized = [0u8; 32];

    let simd_unit_shifted = mm256_sub_epi32(mm256_set1_epi32(GAMMA1_2_POW_19), *simd_unit);
    hax_lib::fstar!("i32_lt_pow2_n_to_bit_zero_lemma 20 $simd_unit_shifted");
    hax_lib::fstar!("reveal_opaque_arithmetic_ops #I32");
    let adjacent_4_combined = serialize_when_gamma1_is_2_pow_19_aux(simd_unit_shifted);

    // We now have 80 bits starting at position 0 in the lower 128-bit lane, ...
    let lower_4 = mm256_castsi256_si128(adjacent_4_combined);
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_4);

    // ... and the second 80 bits at position 0 in the upper 128-bit lane
    let upper_4 = mm256_extracti128_si256::<1>(adjacent_4_combined);
    mm_storeu_bytes_si128(&mut serialized[10..26], upper_4);

    hax_lib::fstar!(
        r#"
  // Thunk the two proofs, so that Z3 check them in parallel, and the SMT context is not polluted
  let spec (i:nat{i < 160}) = u8_to_bv (Seq.index $serialized (i / 8)) (mk_int (i % 8))
            == i32_to_bv (${GAMMA1_2_POW_19} `sub_mod` to_i32x8 $simd_unit (mk_int (i / 20))) (mk_int (i % 20)) in
  let proof: squash (forall i. spec i) =
    assert (forall (i:nat{i < 8}). to_i32x8 $simd_unit_shifted (mk_int ((i / 4) * 4 + i % 4)) == ${GAMMA1_2_POW_19} `sub_mod` to_i32x8 $simd_unit (mk_int i));
    let proof_80 (): squash (forall i. i < 80 ==> spec i) = () in
    let proof_160 (): squash (forall i. i >= 80 ==> spec i) = () in
    proof_80 (); proof_160 ()
  in ()
    "#
    );

    out.copy_from_slice(&serialized[0..20])
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
     (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19)
  /\ (Seq.length serialized == v $gamma1_exponent + 1)
  /\ (
    let gamma_pow = match v $gamma1_exponent with
    | 17 -> $GAMMA1_2_POW_17
    | 19 -> $GAMMA1_2_POW_19
    in
    forall i. let x = (v gamma_pow - v (to_i32x8 $simd_unit i)) in x >= 0 && x < pow2 (v $gamma1_exponent + 1)
  )
"#))
]
#[hax_lib::ensures(|_result| fstar!(r#"
    let gamma_pow = match v $gamma1_exponent with
    | 17 -> $GAMMA1_2_POW_17
    | 19 -> $GAMMA1_2_POW_19
    in
    let n_bytes = v $gamma1_exponent + 1 in
      Seq.length ${serialized}_future == n_bytes
    /\ (forall (i:nat{i < n_bytes * 8}).
      u8_to_bv (Seq.index ${serialized}_future (i / 8)) (mk_int (i % 8))
   == i32_to_bv (gamma_pow -! to_i32x8 $simd_unit (mk_int (i / n_bytes))) (mk_int (i % n_bytes)))
"#))]
pub(crate) fn serialize(simd_unit: &Vec256, serialized: &mut [u8], gamma1_exponent: usize) {
    match gamma1_exponent {
        17 => serialize_when_gamma1_is_2_pow_17(simd_unit, serialized),
        19 => serialize_when_gamma1_is_2_pow_19(simd_unit, serialized),
        _ => unreachable!(),
    }
}

const GAMMA1_17: i32 = 1 << 17;
const GAMMA1_17_TIMES_2_MASK: i32 = (GAMMA1_17 << 1) - 1;

const GAMMA1_19: i32 = 1 << 19;
const GAMMA1_19_TIMES_2_MASK: i32 = (GAMMA1_19 << 1) - 1;

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
type gamma1_exp = g:usize {v g == 17 \/ v g == 19}
unfold let serialized_len (g: gamma1_exp) = v g + 1
unfold let gamma1_of_exp (g: gamma1_exp) = match v g with | 17 -> $GAMMA1_17 | 19 -> $GAMMA1_19

let deserialize_unsigned_post
  (g: gamma1_exp)
  (serialized: t_Slice u8{Seq.length serialized == serialized_len g})
  (result: bv256)
  = let bytes = Seq.length serialized in
    (forall (i: nat{i < bytes * 8}).
       u8_to_bv serialized.[ mk_usize (i / 8) ] (mk_int (i % 8)) ==
       result.(mk_int ((i / bytes) * 32 + i % bytes))) /\
    (forall (i: nat{i < 256}).
       i % 32 >= bytes ==> Libcrux_core_models.Abstractions.Bit.Bit_Zero? result.(mk_int i))
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(serialized.len() == 18)]
#[hax_lib::ensures(|_result| fstar!("deserialize_unsigned_post (mk_int 17) $serialized ${out}_future"))]
fn deserialize_when_gamma1_is_2_pow_17_unsigned(serialized: &[u8], out: &mut Vec256) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 18);

    let serialized_lower = mm_loadu_si128(&serialized[0..16]);
    let serialized_upper = mm_loadu_si128(&serialized[2..18]);

    let serialized_vec = mm256_set_m128i(serialized_upper, serialized_lower);

    // XXX: use out here
    let coefficients = mm256_shuffle_epi8(
        serialized_vec,
        mm256_set_epi8(
            -1, 15, 14, 13, -1, 13, 12, 11, -1, 11, 10, 9, -1, 9, 8, 7, -1, 8, 7, 6, -1, 6, 5, 4,
            -1, 4, 3, 2, -1, 2, 1, 0,
        ),
    );

    let coefficients = mm256_srlv_epi32(coefficients, mm256_set_epi32(6, 4, 2, 0, 6, 4, 2, 0));
    let coefficients = mm256_and_si256(coefficients, mm256_set1_epi32(GAMMA1_17_TIMES_2_MASK));
    hax_lib::fstar!(
        r#"let (): squash (deserialize_unsigned_post (mk_int 17) $serialized $coefficients) =
             i32_to_bv_pow2_min_one_lemma_fa 18
           in ()"#
    );
    *out = coefficients
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(serialized.len() == 20)]
#[hax_lib::ensures(|result| fstar!("deserialize_unsigned_post (mk_int 19) $serialized ${out}_future"))]
fn deserialize_when_gamma1_is_2_pow_19_unsigned(serialized: &[u8], out: &mut Vec256) {
    // Each set of 5 bytes deserializes to 2 coefficients, and since each Vec256
    // can hold 8 such coefficients, we process 5 * (8 / 2) = 20 bytes in this
    // function.
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 20);

    let serialized_lower = mm_loadu_si128(&serialized[0..16]);
    let serialized_upper = mm_loadu_si128(&serialized[4..20]);

    let serialized_vec = mm256_set_m128i(serialized_upper, serialized_lower);

    let coefficients = mm256_shuffle_epi8(
        serialized_vec,
        mm256_set_epi8(
            -1, 15, 14, 13, -1, 13, 12, 11, -1, 10, 9, 8, -1, 8, 7, 6, -1, 9, 8, 7, -1, 7, 6, 5,
            -1, 4, 3, 2, -1, 2, 1, 0,
        ),
    );

    let coefficients = mm256_srlv_epi32(coefficients, mm256_set_epi32(4, 0, 4, 0, 4, 0, 4, 0));
    let coefficients = mm256_and_si256(coefficients, mm256_set1_epi32(GAMMA1_19_TIMES_2_MASK));
    hax_lib::fstar!("i32_to_bv_pow2_min_one_lemma_fa 20");
    *out = coefficients
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let deserialize_post (gamma1_exponent: gamma1_exp)
         (serialized: t_Slice u8 {Seq.length serialized == v gamma1_exponent + 1})
         (result: bv256)
    = (forall i. v (to_i32x8 result i) > minint I32)
    /\ ( let out_reverted = mk_i32x8 (fun i -> neg (to_i32x8 result i) `add_mod` gamma1_of_exp gamma1_exponent) in
        deserialize_unsigned_post gamma1_exponent serialized out_reverted)
"#
)]
#[hax_lib::requires( match gamma1_exponent {
        17 => serialized.len() == 18,
        19 => serialized.len() == 20,
        _ => false,
})]
#[hax_lib::ensures(|result| fstar!("deserialize_post $gamma1_exponent $serialized ${out}_future"))]
pub(crate) fn deserialize(serialized: &[u8], out: &mut Vec256, gamma1_exponent: usize) {
    match gamma1_exponent as u8 {
        17 => deserialize_when_gamma1_is_2_pow_17_unsigned(serialized, out),
        19 => deserialize_when_gamma1_is_2_pow_19_unsigned(serialized, out),
        _ => unreachable!(),
    };

    #[cfg(hax)]
    let unsigned = out.clone();

    let gamma1 = match gamma1_exponent as u8 {
        17 => GAMMA1_17,
        19 => GAMMA1_19,
        _ => unreachable!(),
    };

    *out = mm256_sub_epi32(mm256_set1_epi32(gamma1), *out);

    hax_lib::fstar!(
        r#"
    i32_bit_zero_lemma_to_lt_pow2_n_weak (v $gamma1_exponent + 1) $unsigned;
    reveal_opaque_arithmetic_ops #I32;
    let out_reverted: bv256 = mk_i32x8 (fun i -> neg (to_i32x8 $out i) `add_mod` gamma1_of_exp $gamma1_exponent) in
    introduce forall i. neg (to_i32x8 out i) `add_mod` gamma1_of_exp $gamma1_exponent == to_i32x8 $unsigned i
    with rewrite_eq_sub_mod (to_i32x8 out i) (gamma1_of_exp $gamma1_exponent) (to_i32x8 $unsigned i);
    to_i32x8_eq_to_bv_eq $unsigned out_reverted;
    assert_norm (deserialize_post $gamma1_exponent $serialized out ==
        ((forall i. v (to_i32x8 out i) > minint I32) /\
          deserialize_unsigned_post $gamma1_exponent $serialized out_reverted))
    "#
    );
}
