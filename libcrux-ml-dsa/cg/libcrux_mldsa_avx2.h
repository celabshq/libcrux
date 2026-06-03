/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: e656e17bff6ca5efac8ab6919b9b74cb9a8dd8ad
 * Eurydice: aaa9fa657fb6f09802edb890252040d94cd93982
 * Karamel: 8c19d41458ce5cbfea029ebc03334ba96d149039
 * F*: unset
 * Libcrux: ae092f4921933e45dec31610a1465191171e5b3f
 */

#ifndef libcrux_mldsa_avx2_H
#define libcrux_mldsa_avx2_H

#include "eurydice_glue.h"
#include "intrinsics/libcrux_intrinsics_avx2.h"
#include "libcrux_mldsa_core.h"
#include "libcrux_mldsa_portable.h"
#include "libcrux_sha3.h"
#include "libcrux_sha3_avx2.h"
#include "libcrux_sha3_internal.h"

typedef libcrux_sha3_avx2_x4_incremental_KeccakState
    libcrux_ml_dsa_hash_functions_simd256_Shake128x4;

typedef libcrux_sha3_portable_KeccakState
    libcrux_ml_dsa_hash_functions_simd256_Shake256;

typedef libcrux_sha3_avx2_x4_incremental_KeccakState
    libcrux_ml_dsa_hash_functions_simd256_Shake256x4;

/**
 Init the state and absorb 4 blocks in parallel.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c4
libcrux_ml_dsa_hash_functions_simd256_init_absorb(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  Eurydice_arr_c4 state = libcrux_sha3_avx2_x4_incremental_init();
  libcrux_sha3_avx2_x4_incremental_shake128_absorb_final(&state, input0, input1,
                                                         input2, input3);
  return state;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_ml_dsa_hash_functions_simd256_init_absorb_final_shake256(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_7c state = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state, input);
  return state;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c4
libcrux_ml_dsa_hash_functions_simd256_init_absorb_x4(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  Eurydice_arr_c4 state = libcrux_sha3_avx2_x4_incremental_init();
  libcrux_sha3_avx2_x4_incremental_shake256_absorb_final(&state, input0, input1,
                                                         input2, input3);
  return state;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_shake256(
    Eurydice_arr_7c *state) {
  Eurydice_arr_ff out = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
      state, Eurydice_array_to_slice_mut_58(&out));
  return out;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_x4(
    Eurydice_arr_c4 *state) {
  Eurydice_arr_ff out0 = {{0U}};
  Eurydice_arr_ff out1 = {{0U}};
  Eurydice_arr_ff out2 = {{0U}};
  Eurydice_arr_ff out3 = {{0U}};
  libcrux_sha3_avx2_x4_incremental_shake256_squeeze_first_block(
      state, Eurydice_array_to_slice_mut_58(&out0),
      Eurydice_array_to_slice_mut_58(&out1),
      Eurydice_array_to_slice_mut_58(&out2),
      Eurydice_array_to_slice_mut_58(&out3));
  return (Eurydice_arr_ff_x4{out0, out1, out2, out3});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_five_blocks(
    Eurydice_arr_c4 *state, Eurydice_arr_d1 *out0, Eurydice_arr_d1 *out1,
    Eurydice_arr_d1 *out2, Eurydice_arr_d1 *out3) {
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_five_blocks(
      state, Eurydice_array_to_slice_mut_4c(out0),
      Eurydice_array_to_slice_mut_4c(out1),
      Eurydice_array_to_slice_mut_4c(out2),
      Eurydice_array_to_slice_mut_4c(out3));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c5_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block(
    Eurydice_arr_c4 *state) {
  Eurydice_arr_c5 out0 = {{0U}};
  Eurydice_arr_c5 out1 = {{0U}};
  Eurydice_arr_c5 out2 = {{0U}};
  Eurydice_arr_c5 out3 = {{0U}};
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_next_block(
      state, Eurydice_array_to_slice_mut_2c(&out0),
      Eurydice_array_to_slice_mut_2c(&out1),
      Eurydice_array_to_slice_mut_2c(&out2),
      Eurydice_array_to_slice_mut_2c(&out3));
  return (Eurydice_arr_c5_x4{out0, out1, out2, out3});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_shake256(
    Eurydice_arr_7c *state) {
  Eurydice_arr_ff out = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
      state, Eurydice_array_to_slice_mut_58(&out));
  return out;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4(
    Eurydice_arr_c4 *state) {
  Eurydice_arr_ff out0 = {{0U}};
  Eurydice_arr_ff out1 = {{0U}};
  Eurydice_arr_ff out2 = {{0U}};
  Eurydice_arr_ff out3 = {{0U}};
  libcrux_sha3_avx2_x4_incremental_shake256_squeeze_next_block(
      state, Eurydice_array_to_slice_mut_58(&out0),
      Eurydice_array_to_slice_mut_58(&out1),
      Eurydice_array_to_slice_mut_58(&out2),
      Eurydice_array_to_slice_mut_58(&out3));
  return (Eurydice_arr_ff_x4{out0, out1, out2, out3});
}

/**
 Init the state and absorb 4 blocks in parallel.
*/
/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake128x4}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c4
libcrux_ml_dsa_hash_functions_simd256_init_absorb_3b(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  return libcrux_ml_dsa_hash_functions_simd256_init_absorb(input0, input1,
                                                           input2, input3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake128x4}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_five_blocks_3b(
    Eurydice_arr_c4 *self, Eurydice_arr_d1 *out0, Eurydice_arr_d1 *out1,
    Eurydice_arr_d1 *out2, Eurydice_arr_d1 *out3) {
  libcrux_ml_dsa_hash_functions_simd256_squeeze_first_five_blocks(
      self, out0, out1, out2, out3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake128x4}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c5_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_3b(
    Eurydice_arr_c4 *self) {
  return libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_ml_dsa_hash_functions_simd256_init_absorb_final_8c(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_dsa_hash_functions_simd256_init_absorb_final_shake256(
      input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_8c(
    Eurydice_arr_7c *self) {
  return libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_shake256(
      self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_8c(
    Eurydice_arr_7c *self) {
  return libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_shake256(
      self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c4
libcrux_ml_dsa_hash_functions_simd256_init_absorb_x4_ad(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  return libcrux_ml_dsa_hash_functions_simd256_init_absorb_x4(input0, input1,
                                                              input2, input3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_x4_ad(
    Eurydice_arr_c4 *self) {
  return libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_x4(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4_ad(
    Eurydice_arr_c4 *self) {
  return libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4(self);
}

typedef __m256i libcrux_ml_dsa_simd_avx2_vector_type_Vec256;

/**
 Create an all-zero vector coefficient
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i libcrux_ml_dsa_simd_avx2_vector_type_zero(void) {
  return libcrux_intrinsics_avx2_mm256_setzero_si256();
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_dsa_simd_avx2_zero_a2(void) {
  return libcrux_ml_dsa_simd_avx2_vector_type_zero();
}

/**
 Create a coefficient from an `i32` array
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_vector_type_from_coefficient_array(
    Eurydice_dst_ref_shared_83 coefficient_array, __m256i *out) {
  out[0U] = libcrux_intrinsics_avx2_mm256_loadu_si256_i32(coefficient_array);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_from_coefficient_array_a2(
    Eurydice_dst_ref_shared_83 coefficient_array, __m256i *out) {
  libcrux_ml_dsa_simd_avx2_vector_type_from_coefficient_array(coefficient_array,
                                                              out);
}

/**
 Write out the coefficient to an `i32` array
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_vector_type_to_coefficient_array(
    const __m256i *value, Eurydice_dst_ref_mut_83 out) {
  libcrux_intrinsics_avx2_mm256_storeu_si256_i32(out, value[0U]);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_to_coefficient_array_a2(
    const __m256i *value, Eurydice_dst_ref_mut_83 out) {
  libcrux_ml_dsa_simd_avx2_vector_type_to_coefficient_array(value, out);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_arithmetic_add(__m256i *lhs,
                                                           const __m256i *rhs) {
  lhs[0U] = libcrux_intrinsics_avx2_mm256_add_epi32(lhs[0U], rhs[0U]);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_add_a2(
    __m256i *lhs, const __m256i *rhs) {
  libcrux_ml_dsa_simd_avx2_arithmetic_add(lhs, rhs);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_arithmetic_subtract(
    __m256i *lhs, const __m256i *rhs) {
  lhs[0U] = libcrux_intrinsics_avx2_mm256_sub_epi32(lhs[0U], rhs[0U]);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_subtract_a2(
    __m256i *lhs, const __m256i *rhs) {
  libcrux_ml_dsa_simd_avx2_arithmetic_subtract(lhs, rhs);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline bool libcrux_ml_dsa_simd_avx2_arithmetic_infinity_norm_exceeds(
    const __m256i *simd_unit, int32_t bound) {
  __m256i absolute_values =
      libcrux_intrinsics_avx2_mm256_abs_epi32(simd_unit[0U]);
  __m256i bound0 = libcrux_intrinsics_avx2_mm256_set1_epi32(bound - 1);
  __m256i compare_with_bound =
      libcrux_intrinsics_avx2_mm256_cmpgt_epi32(absolute_values, bound0);
  int32_t result = libcrux_intrinsics_avx2_mm256_testz_si256(
      compare_with_bound, compare_with_bound);
  return result != 1;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool libcrux_ml_dsa_simd_avx2_infinity_norm_exceeds_a2(
    const __m256i *simd_unit, int32_t bound) {
  return libcrux_ml_dsa_simd_avx2_arithmetic_infinity_norm_exceeds(simd_unit,
                                                                   bound);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i
libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives_ret(
    const __m256i *t) {
  __m256i signs = libcrux_intrinsics_avx2_mm256_srai_epi32(31, t[0U], __m256i);
  __m256i conditional_add_field_modulus =
      libcrux_intrinsics_avx2_mm256_and_si256(
          signs, libcrux_intrinsics_avx2_mm256_set1_epi32(
                     LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS));
  return libcrux_intrinsics_avx2_mm256_add_epi32(t[0U],
                                                 conditional_add_field_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_arithmetic_decompose(
    int32_t gamma2, const __m256i *r, __m256i *r0, __m256i *r1) {
  __m256i r2 =
      libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives_ret(r);
  __m256i ceil_of_r_by_128 = libcrux_intrinsics_avx2_mm256_add_epi32(
      r2, libcrux_intrinsics_avx2_mm256_set1_epi32(127));
  __m256i ceil_of_r_by_1280 =
      libcrux_intrinsics_avx2_mm256_srai_epi32(7, ceil_of_r_by_128, __m256i);
  switch (gamma2) {
    case 95232: {
      __m256i result = libcrux_intrinsics_avx2_mm256_mullo_epi32(
          ceil_of_r_by_1280, libcrux_intrinsics_avx2_mm256_set1_epi32(11275));
      __m256i result0 = libcrux_intrinsics_avx2_mm256_add_epi32(
          result, libcrux_intrinsics_avx2_mm256_set1_epi32(
                      (int32_t)((uint32_t)1 << 23U)));
      __m256i result1 =
          libcrux_intrinsics_avx2_mm256_srai_epi32(24, result0, __m256i);
      __m256i mask = libcrux_intrinsics_avx2_mm256_sub_epi32(
          libcrux_intrinsics_avx2_mm256_set1_epi32(43), result1);
      __m256i mask0 =
          libcrux_intrinsics_avx2_mm256_srai_epi32(31, mask, __m256i);
      __m256i not_result =
          libcrux_intrinsics_avx2_mm256_xor_si256(result1, mask0);
      r1[0U] = libcrux_intrinsics_avx2_mm256_and_si256(result1, not_result);
      break;
    }
    case 261888: {
      __m256i result = libcrux_intrinsics_avx2_mm256_mullo_epi32(
          ceil_of_r_by_1280, libcrux_intrinsics_avx2_mm256_set1_epi32(1025));
      __m256i result0 = libcrux_intrinsics_avx2_mm256_add_epi32(
          result, libcrux_intrinsics_avx2_mm256_set1_epi32(
                      (int32_t)((uint32_t)1 << 21U)));
      __m256i result1 =
          libcrux_intrinsics_avx2_mm256_srai_epi32(22, result0, __m256i);
      r1[0U] = libcrux_intrinsics_avx2_mm256_and_si256(
          result1, libcrux_intrinsics_avx2_mm256_set1_epi32(15));
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  int32_t alpha = gamma2 * 2;
  __m256i r0_tmp = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      r1[0U], libcrux_intrinsics_avx2_mm256_set1_epi32(alpha));
  __m256i r0_tmp0 = libcrux_intrinsics_avx2_mm256_sub_epi32(r2, r0_tmp);
  __m256i field_modulus_halved = libcrux_intrinsics_avx2_mm256_set1_epi32(
      (LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS - 1) / 2);
  __m256i mask =
      libcrux_intrinsics_avx2_mm256_sub_epi32(field_modulus_halved, r0_tmp0);
  __m256i mask0 = libcrux_intrinsics_avx2_mm256_srai_epi32(31, mask, __m256i);
  __m256i field_modulus_and_mask = libcrux_intrinsics_avx2_mm256_and_si256(
      mask0, libcrux_intrinsics_avx2_mm256_set1_epi32(
                 LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS));
  r0[0U] =
      libcrux_intrinsics_avx2_mm256_sub_epi32(r0_tmp0, field_modulus_and_mask);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_decompose_a2(
    int32_t gamma2, const __m256i *simd_unit, __m256i *low, __m256i *high) {
  libcrux_ml_dsa_simd_avx2_arithmetic_decompose(gamma2, simd_unit, low, high);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t libcrux_ml_dsa_simd_avx2_arithmetic_compute_hint(
    const __m256i *low, const __m256i *high, int32_t gamma2, __m256i *hint) {
  __m256i minus_gamma2 = libcrux_intrinsics_avx2_mm256_set1_epi32(-gamma2);
  __m256i gamma20 = libcrux_intrinsics_avx2_mm256_set1_epi32(gamma2);
  __m256i low_within_bound = libcrux_intrinsics_avx2_mm256_cmpgt_epi32(
      libcrux_intrinsics_avx2_mm256_abs_epi32(low[0U]), gamma20);
  __m256i low_equals_minus_gamma2 =
      libcrux_intrinsics_avx2_mm256_cmpeq_epi32(low[0U], minus_gamma2);
  __m256i low_equals_minus_gamma2_and_high_is_nonzero =
      libcrux_intrinsics_avx2_mm256_sign_epi32(low_equals_minus_gamma2,
                                               high[0U]);
  hint[0U] = libcrux_intrinsics_avx2_mm256_or_si256(
      low_within_bound, low_equals_minus_gamma2_and_high_is_nonzero);
  int32_t hints_mask = libcrux_intrinsics_avx2_mm256_movemask_ps(
      libcrux_intrinsics_avx2_mm256_castsi256_ps(hint[0U]));
  hint[0U] = libcrux_intrinsics_avx2_mm256_and_si256(
      hint[0U], libcrux_intrinsics_avx2_mm256_set1_epi32(1));
  return (size_t)core_num__i32__count_ones(hints_mask);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t libcrux_ml_dsa_simd_avx2_compute_hint_a2(
    const __m256i *low, const __m256i *high, int32_t gamma2, __m256i *hint) {
  return libcrux_ml_dsa_simd_avx2_arithmetic_compute_hint(low, high, gamma2,
                                                          hint);
}

typedef struct core_core_arch_x86___m256i_x2_s {
  __m256i fst;
  __m256i snd;
} core_core_arch_x86___m256i_x2;

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_arithmetic_use_hint(
    int32_t gamma2, const __m256i *r, __m256i *hint) {
  core_core_arch_x86___m256i_x2 uu____0 = {
      libcrux_intrinsics_avx2_mm256_setzero_si256(),
      libcrux_intrinsics_avx2_mm256_setzero_si256()};
  __m256i r0 = uu____0.fst;
  __m256i r1 = uu____0.snd;
  libcrux_ml_dsa_simd_avx2_arithmetic_decompose(gamma2, r, &r0, &r1);
  __m256i all_zeros = libcrux_intrinsics_avx2_mm256_setzero_si256();
  __m256i r0_gt_zero = libcrux_intrinsics_avx2_mm256_cmpgt_epi32(r0, all_zeros);
  __m256i negate_hints = libcrux_intrinsics_avx2_vec256_blendv_epi32(
      hint[0U], all_zeros, r0_gt_zero);
  __m256i negate_hints0 =
      libcrux_intrinsics_avx2_mm256_slli_epi32(1, negate_hints, __m256i);
  __m256i hints =
      libcrux_intrinsics_avx2_mm256_sub_epi32(hint[0U], negate_hints0);
  __m256i r1_plus_hints = libcrux_intrinsics_avx2_mm256_add_epi32(r1, hints);
  switch (gamma2) {
    case 95232: {
      __m256i max = libcrux_intrinsics_avx2_mm256_set1_epi32(43);
      r1_plus_hints = libcrux_intrinsics_avx2_vec256_blendv_epi32(
          r1_plus_hints, max, r1_plus_hints);
      __m256i greater_than_or_equal_to_max =
          libcrux_intrinsics_avx2_mm256_cmpgt_epi32(r1_plus_hints, max);
      hint[0U] = libcrux_intrinsics_avx2_vec256_blendv_epi32(
          r1_plus_hints, all_zeros, greater_than_or_equal_to_max);
      break;
    }
    case 261888: {
      hint[0U] = libcrux_intrinsics_avx2_mm256_and_si256(
          r1_plus_hints, libcrux_intrinsics_avx2_mm256_set1_epi32(15));
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_use_hint_a2(
    int32_t gamma2, const __m256i *simd_unit, __m256i *hint) {
  libcrux_ml_dsa_simd_avx2_arithmetic_use_hint(gamma2, simd_unit, hint);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_aux(
    __m256i field_modulus, __m256i inverse_of_modulus_mod_montgomery_r,
    __m256i *lhs, const __m256i *rhs) {
  __m256i prod02 = libcrux_intrinsics_avx2_mm256_mul_epi32(lhs[0U], rhs[0U]);
  __m256i prod13 = libcrux_intrinsics_avx2_mm256_mul_epi32(
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, lhs[0U], __m256i),
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, rhs[0U], __m256i));
  __m256i k02 = libcrux_intrinsics_avx2_mm256_mul_epi32(
      prod02, inverse_of_modulus_mod_montgomery_r);
  __m256i k13 = libcrux_intrinsics_avx2_mm256_mul_epi32(
      prod13, inverse_of_modulus_mod_montgomery_r);
  __m256i c02 = libcrux_intrinsics_avx2_mm256_mul_epi32(k02, field_modulus);
  __m256i c13 = libcrux_intrinsics_avx2_mm256_mul_epi32(k13, field_modulus);
  __m256i res02 = libcrux_intrinsics_avx2_mm256_sub_epi32(prod02, c02);
  __m256i res13 = libcrux_intrinsics_avx2_mm256_sub_epi32(prod13, c13);
  __m256i res02_shifted =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, res02, __m256i);
  lhs[0U] = libcrux_intrinsics_avx2_mm256_blend_epi32(170, res02_shifted, res13,
                                                      __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(__m256i *lhs,
                                                        const __m256i *rhs) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi32(
      LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  __m256i inverse_of_modulus_mod_montgomery_r =
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          (int32_t)
              LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_aux(
      field_modulus, inverse_of_modulus_mod_montgomery_r, lhs, rhs);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_montgomery_multiply_a2(
    __m256i *lhs, const __m256i *rhs) {
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(lhs, rhs);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_arithmetic_barrett_reduce_simd_unit(
    __m256i *simd_unit) {
  __m256i quotient = libcrux_intrinsics_avx2_mm256_add_epi32(
      simd_unit[0U],
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << 22U)));
  __m256i quotient0 =
      libcrux_intrinsics_avx2_mm256_srai_epi32(23, quotient, __m256i);
  __m256i quotient_times_field_modulus =
      libcrux_intrinsics_avx2_mm256_mullo_epi32(
          quotient0, libcrux_intrinsics_avx2_mm256_set1_epi32(
                         LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS));
  simd_unit[0U] = libcrux_intrinsics_avx2_mm256_sub_epi32(
      simd_unit[0U], quotient_times_field_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives(__m256i *t) {
  t[0U] =
      libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives_ret(t);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_arithmetic_power2round(
    __m256i *r0, __m256i *r1) {
  libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives(r0);
  r1[0U] = libcrux_intrinsics_avx2_mm256_add_epi32(
      r0[0U],
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          (int32_t)((uint32_t)1
                    << (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T -
                                  (size_t)1U)) -
          1));
  r1[0U] = libcrux_intrinsics_avx2_mm256_srai_epi32(13, r1[0U], __m256i);
  __m256i tmp = libcrux_intrinsics_avx2_mm256_slli_epi32(13, r1[0U], __m256i);
  r0[0U] = libcrux_intrinsics_avx2_mm256_sub_epi32(r0[0U], tmp);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_power2round_a2(
    __m256i *t0, __m256i *t1) {
  libcrux_ml_dsa_simd_avx2_arithmetic_power2round(t0, t1);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_LESS_THAN_FIELD_MODULUS_BYTESTREAM_TO_POTENTIAL_COEFFICIENTS_COEFFICIENT_MASK \
  ((int32_t)((uint32_t)1 << 23U) - 1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_bytestream_to_potential_coefficients(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_ec serialized_extended = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_to_mut_21(&serialized_extended, (size_t)24U),
      serialized, uint8_t);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_loadu_si256_u8(
      Eurydice_array_to_slice_shared_01(&serialized_extended));
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 5, 4, 3, 0, 2, 1, 0));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      coefficients0, libcrux_intrinsics_avx2_mm256_set_epi8(
                         -1, 11, 10, 9, -1, 8, 7, 6, -1, 5, 4, 3, -1, 2, 1, 0,
                         -1, 11, 10, 9, -1, 8, 7, 6, -1, 5, 4, 3, -1, 2, 1, 0));
  return libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients1,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_LESS_THAN_FIELD_MODULUS_BYTESTREAM_TO_POTENTIAL_COEFFICIENTS_COEFFICIENT_MASK));
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE  \
  ((Eurydice_arr_a30{{{{255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,  \
                        255U, 255U, 255U, 255U, 255U, 255U, 255U}},            \
                      {{0U, 1U, 2U, 3U, 255U, 255U, 255U, 255U, 255U, 255U,    \
                        255U, 255U, 255U, 255U, 255U, 255U}},                  \
                      {{4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U,    \
                        255U, 255U, 255U, 255U, 255U, 255U}},                  \
                      {{0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 255U, 255U, 255U,      \
                        255U, 255U, 255U, 255U, 255U}},                        \
                      {{8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U,  \
                        255U, 255U, 255U, 255U, 255U, 255U}},                  \
                      {{0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,    \
                        255U, 255U, 255U, 255U, 255U}},                        \
                      {{4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,    \
                        255U, 255U, 255U, 255U, 255U}},                        \
                      {{0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U,      \
                        255U, 255U, 255U, 255U}},                              \
                      {{12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U,      \
                        255U, 255U, 255U, 255U, 255U, 255U, 255U}},            \
                      {{0U, 1U, 2U, 3U, 12U, 13U, 14U, 15U, 255U, 255U, 255U,  \
                        255U, 255U, 255U, 255U, 255U}},                        \
                      {{4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U,  \
                        255U, 255U, 255U, 255U, 255U}},                        \
                      {{0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U,    \
                        255U, 255U, 255U, 255U}},                              \
                      {{8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U,      \
                        255U, 255U, 255U, 255U, 255U, 255U}},                  \
                      {{0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U,  \
                        255U, 255U, 255U, 255U}},                              \
                      {{4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U,  \
                        255U, 255U, 255U, 255U}},                              \
                      {{0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, \
                        13U, 14U, 15U}}}}))

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_sample(
    Eurydice_borrow_slice_u8 input, Eurydice_dst_ref_mut_83 output) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi32(
      LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  __m256i potential_coefficients =
      libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_bytestream_to_potential_coefficients(
          input);
  __m256i compare_with_field_modulus =
      libcrux_intrinsics_avx2_mm256_cmpgt_epi32(field_modulus,
                                                potential_coefficients);
  int32_t good = libcrux_intrinsics_avx2_mm256_movemask_ps(
      libcrux_intrinsics_avx2_mm256_castsi256_ps(compare_with_field_modulus));
  int32_t good_lower_half = good & 15;
  int32_t good_upper_half = good >> 4U;
  Eurydice_arr_b2 lower_shuffles =
      LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE
          .data[(size_t)good_lower_half];
  __m128i lower_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&lower_shuffles));
  __m128i lower_coefficients =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(potential_coefficients);
  __m128i lower_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      lower_coefficients, lower_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128_i32(
      Eurydice_slice_subslice_mut_47(
          output, (core_ops_range_Range_87{(size_t)0U, (size_t)4U})),
      lower_coefficients0);
  size_t sampled_count = (size_t)core_num__i32__count_ones(good_lower_half);
  Eurydice_arr_b2 upper_shuffles =
      LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE
          .data[(size_t)good_upper_half];
  __m128i upper_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&upper_shuffles));
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      1, potential_coefficients, __m128i);
  __m128i upper_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      upper_coefficients, upper_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128_i32(
      Eurydice_slice_subslice_mut_47(
          output,
          (core_ops_range_Range_87{sampled_count, sampled_count + (size_t)4U})),
      upper_coefficients0);
  size_t uu____0 = sampled_count;
  return uu____0 + (size_t)core_num__i32__count_ones(good_upper_half);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_a2(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_83 out) {
  return libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_sample(
      randomness, out);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_DESERIALIZE_TO_UNSIGNED_WHEN_ETA_IS_4_COEFFICIENT_MASK \
  ((int32_t)((uint32_t)1 << 4U) - 1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned_when_eta_is_4(
    Eurydice_borrow_slice_u8 bytes) {
  __m256i bytes_in_simd_unit = libcrux_intrinsics_avx2_mm256_set_epi32(
      (int32_t)(uint32_t)bytes.ptr[3U], (int32_t)(uint32_t)bytes.ptr[3U],
      (int32_t)(uint32_t)bytes.ptr[2U], (int32_t)(uint32_t)bytes.ptr[2U],
      (int32_t)(uint32_t)bytes.ptr[1U], (int32_t)(uint32_t)bytes.ptr[1U],
      (int32_t)(uint32_t)bytes.ptr[0U], (int32_t)(uint32_t)bytes.ptr[0U]);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_srlv_epi32(
      bytes_in_simd_unit,
      libcrux_intrinsics_avx2_mm256_set_epi32(4, 0, 4, 0, 4, 0, 4, 0));
  return libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_DESERIALIZE_TO_UNSIGNED_WHEN_ETA_IS_4_COEFFICIENT_MASK));
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_DESERIALIZE_TO_UNSIGNED_WHEN_ETA_IS_2_COEFFICIENT_MASK \
  ((int32_t)((uint32_t)1 << 3U) - 1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned_when_eta_is_2(
    Eurydice_borrow_slice_u8 bytes) {
  __m256i bytes_in_simd_unit = libcrux_intrinsics_avx2_mm256_set_epi32(
      (int32_t)(uint32_t)bytes.ptr[2U], (int32_t)(uint32_t)bytes.ptr[2U],
      (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[2U] << 8U) |
          (int32_t)(uint32_t)bytes.ptr[1U],
      (int32_t)(uint32_t)bytes.ptr[1U], (int32_t)(uint32_t)bytes.ptr[1U],
      (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[1U] << 8U) |
          (int32_t)(uint32_t)bytes.ptr[0U],
      (int32_t)(uint32_t)bytes.ptr[0U], (int32_t)(uint32_t)bytes.ptr[0U]);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_srlv_epi32(
      bytes_in_simd_unit,
      libcrux_intrinsics_avx2_mm256_set_epi32(5, 2, 7, 4, 1, 6, 3, 0));
  return libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_DESERIALIZE_TO_UNSIGNED_WHEN_ETA_IS_2_COEFFICIENT_MASK));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      return libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned_when_eta_is_4(
          serialized);
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  return libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned_when_eta_is_2(
      serialized);
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types core_core_arch_x86___m256i

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_secrets_mem_requests_ct_declassify_17(
    const __m256i *val) {}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.shift_interval with
const generics
- ETA= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_shift_interval_af(
    __m256i coefficients) {
  __m256i uu____0;
  __m256i quotient = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      coefficients, libcrux_intrinsics_avx2_mm256_set1_epi32(26));
  __m256i quotient0 =
      libcrux_intrinsics_avx2_mm256_srai_epi32(7, quotient, __m256i);
  __m256i quotient1 = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      quotient0, libcrux_intrinsics_avx2_mm256_set1_epi32(5));
  __m256i coefficients_mod_5 =
      libcrux_intrinsics_avx2_mm256_sub_epi32(coefficients, quotient1);
  uu____0 = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)(size_t)2U),
      coefficients_mod_5);
  return uu____0;
}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.sample with const
generics
- ETA= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_sample_af(
    Eurydice_borrow_slice_u8 input, Eurydice_dst_ref_mut_83 output) {
  __m256i potential_coefficients =
      libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned(
          libcrux_ml_dsa_constants_Eta_Four, input);
  int32_t interval_boundary;
  interval_boundary = 15;
  __m256i compare_with_interval_boundary =
      libcrux_intrinsics_avx2_mm256_cmpgt_epi32(
          libcrux_intrinsics_avx2_mm256_set1_epi32(interval_boundary),
          potential_coefficients);
  int32_t good = libcrux_intrinsics_avx2_mm256_movemask_ps(
      libcrux_intrinsics_avx2_mm256_castsi256_ps(
          compare_with_interval_boundary));
  int32_t good_lower_half = good & 15;
  int32_t good_upper_half = good >> 4U;
  __m256i shifted =
      libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_shift_interval_af(
          potential_coefficients);
  Eurydice_arr_b2 lower_shuffles =
      LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE
          .data[(size_t)good_lower_half];
  __m128i lower_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&lower_shuffles));
  __m128i lower_coefficients =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(shifted);
  __m128i lower_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      lower_coefficients, lower_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128_i32(
      Eurydice_slice_subslice_mut_47(
          output, (core_ops_range_Range_87{(size_t)0U, (size_t)4U})),
      lower_coefficients0);
  size_t sampled_count = (size_t)core_num__i32__count_ones(good_lower_half);
  Eurydice_arr_b2 upper_shuffles =
      LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE
          .data[(size_t)good_upper_half];
  __m128i upper_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&upper_shuffles));
  __m128i upper_coefficients =
      libcrux_intrinsics_avx2_mm256_extracti128_si256(1, shifted, __m128i);
  __m128i upper_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      upper_coefficients, upper_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128_i32(
      Eurydice_slice_subslice_mut_47(
          output,
          (core_ops_range_Range_87{sampled_count, sampled_count + (size_t)4U})),
      upper_coefficients0);
  size_t uu____0 = sampled_count;
  return uu____0 + (size_t)core_num__i32__count_ones(good_upper_half);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_equals_2_a2(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_83 out) {
  return libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_sample_af(
      randomness, out);
}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.shift_interval with
const generics
- ETA= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_shift_interval_23(
    __m256i coefficients) {
  return libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)(size_t)4U),
      coefficients);
}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.sample with const
generics
- ETA= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_sample_23(
    Eurydice_borrow_slice_u8 input, Eurydice_dst_ref_mut_83 output) {
  __m256i potential_coefficients =
      libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned(
          libcrux_ml_dsa_constants_Eta_Four, input);
  int32_t interval_boundary;
  interval_boundary = 9;
  __m256i compare_with_interval_boundary =
      libcrux_intrinsics_avx2_mm256_cmpgt_epi32(
          libcrux_intrinsics_avx2_mm256_set1_epi32(interval_boundary),
          potential_coefficients);
  int32_t good = libcrux_intrinsics_avx2_mm256_movemask_ps(
      libcrux_intrinsics_avx2_mm256_castsi256_ps(
          compare_with_interval_boundary));
  int32_t good_lower_half = good & 15;
  int32_t good_upper_half = good >> 4U;
  __m256i shifted =
      libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_shift_interval_23(
          potential_coefficients);
  Eurydice_arr_b2 lower_shuffles =
      LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE
          .data[(size_t)good_lower_half];
  __m128i lower_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&lower_shuffles));
  __m128i lower_coefficients =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(shifted);
  __m128i lower_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      lower_coefficients, lower_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128_i32(
      Eurydice_slice_subslice_mut_47(
          output, (core_ops_range_Range_87{(size_t)0U, (size_t)4U})),
      lower_coefficients0);
  size_t sampled_count = (size_t)core_num__i32__count_ones(good_lower_half);
  Eurydice_arr_b2 upper_shuffles =
      LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE
          .data[(size_t)good_upper_half];
  __m128i upper_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&upper_shuffles));
  __m128i upper_coefficients =
      libcrux_intrinsics_avx2_mm256_extracti128_si256(1, shifted, __m128i);
  __m128i upper_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      upper_coefficients, upper_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128_i32(
      Eurydice_slice_subslice_mut_47(
          output,
          (core_ops_range_Range_87{sampled_count, sampled_count + (size_t)4U})),
      upper_coefficients0);
  size_t uu____0 = sampled_count;
  return uu____0 + (size_t)core_num__i32__count_ones(good_upper_half);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_equals_4_a2(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_83 out) {
  return libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_sample_23(
      randomness, out);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_2_POW_19 \
  ((int32_t)((uint32_t)1 << 19U))

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_19_aux(
    __m256i simd_unit_shifted) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit_shifted,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 12, 0, 12, 0, 12, 0, 12));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      12, adjacent_2_combined, __m256i);
  return libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      adjacent_2_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          -1, -1, -1, -1, -1, -1, 12, 11, 10, 9, 8, 4, 3, 2, 1, 0, -1, -1, -1,
          -1, -1, -1, 12, 11, 10, 9, 8, 4, 3, 2, 1, 0));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_ec serialized = {{0U}};
  __m256i simd_unit_shifted = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_2_POW_19),
      simd_unit[0U]);
  __m256i adjacent_4_combined =
      libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_19_aux(
          simd_unit_shifted);
  __m128i lower_4 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_4_combined);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d44(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
      lower_4);
  __m128i upper_4 = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      1, adjacent_4_combined, __m128i);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d44(
          &serialized, (core_ops_range_Range_87{(size_t)10U, (size_t)26U})),
      upper_4);
  Eurydice_slice_copy(
      out,
      Eurydice_array_to_subslice_shared_d4(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)20U})),
      uint8_t);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_2_POW_17 \
  ((int32_t)((uint32_t)1 << 17U))

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_17_aux(
    __m256i simd_unit_shifted) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit_shifted,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 14, 0, 14, 0, 14, 0, 14));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      14, adjacent_2_combined, __m256i);
  __m256i every_second_element = libcrux_intrinsics_avx2_mm256_bsrli_epi128(
      8, adjacent_2_combined0, __m256i);
  __m256i every_second_element_shifted =
      libcrux_intrinsics_avx2_mm256_slli_epi64(36, every_second_element,
                                               __m256i);
  __m256i adjacent_4_combined = libcrux_intrinsics_avx2_mm256_add_epi64(
      adjacent_2_combined0, every_second_element_shifted);
  return libcrux_intrinsics_avx2_mm256_srlv_epi64(
      adjacent_4_combined,
      libcrux_intrinsics_avx2_mm256_set_epi64x(28LL, 0LL, 28LL, 0LL));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_ec serialized = {{0U}};
  __m256i simd_unit_shifted = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_2_POW_17),
      simd_unit[0U]);
  __m256i adjacent_4_combined =
      libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_17_aux(
          simd_unit_shifted);
  __m128i lower_4 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_4_combined);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d44(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
      lower_4);
  __m128i upper_4 = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      1, adjacent_4_combined, __m128i);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d44(
          &serialized, (core_ops_range_Range_87{(size_t)9U, (size_t)25U})),
      upper_4);
  Eurydice_slice_copy(
      out,
      Eurydice_array_to_subslice_shared_d4(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)18U})),
      uint8_t);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 serialized,
    size_t gamma1_exponent) {
  switch (gamma1_exponent) {
    case 17U: {
      break;
    }
    case 19U: {
      libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(
          simd_unit, serialized);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(
      simd_unit, serialized);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_gamma1_serialize_a2(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 serialized,
    size_t gamma1_exponent) {
  libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize(simd_unit, serialized,
                                                     gamma1_exponent);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17 \
  ((int32_t)((uint32_t)1 << 17U))

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17_TIMES_2_MASK   \
  ((int32_t)((uint32_t)LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17 \
             << 1U) -                                                     \
   1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17_unsigned(
    Eurydice_borrow_slice_u8 serialized, __m256i *out) {
  __m128i serialized_lower =
      libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_c8(
          serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})));
  __m128i serialized_upper =
      libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_c8(
          serialized, (core_ops_range_Range_87{(size_t)2U, (size_t)18U})));
  __m256i serialized_vec = libcrux_intrinsics_avx2_mm256_set_m128i(
      serialized_upper, serialized_lower);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      serialized_vec,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          -1, 15, 14, 13, -1, 13, 12, 11, -1, 11, 10, 9, -1, 9, 8, 7, -1, 8, 7,
          6, -1, 6, 5, 4, -1, 4, 3, 2, -1, 2, 1, 0));
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_srlv_epi32(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi32(6, 4, 2, 0, 6, 4, 2, 0));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients0,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17_TIMES_2_MASK));
  out[0U] = coefficients1;
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19 \
  ((int32_t)((uint32_t)1 << 19U))

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19_TIMES_2_MASK   \
  ((int32_t)((uint32_t)LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19 \
             << 1U) -                                                     \
   1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19_unsigned(
    Eurydice_borrow_slice_u8 serialized, __m256i *out) {
  __m128i serialized_lower =
      libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_c8(
          serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})));
  __m128i serialized_upper =
      libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_c8(
          serialized, (core_ops_range_Range_87{(size_t)4U, (size_t)20U})));
  __m256i serialized_vec = libcrux_intrinsics_avx2_mm256_set_m128i(
      serialized_upper, serialized_lower);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      serialized_vec,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          -1, 15, 14, 13, -1, 13, 12, 11, -1, 10, 9, 8, -1, 8, 7, 6, -1, 9, 8,
          7, -1, 7, 6, 5, -1, 4, 3, 2, -1, 2, 1, 0));
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_srlv_epi32(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi32(4, 0, 4, 0, 4, 0, 4, 0));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients0,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19_TIMES_2_MASK));
  out[0U] = coefficients1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize(
    Eurydice_borrow_slice_u8 serialized, __m256i *out, size_t gamma1_exponent) {
  switch ((uint32_t)(uint8_t)gamma1_exponent) {
    case 17U: {
      libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17_unsigned(
          serialized, out);
      break;
    }
    case 19U: {
      libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19_unsigned(
          serialized, out);
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  int32_t gamma1;
  switch ((uint32_t)(uint8_t)gamma1_exponent) {
    case 17U: {
      gamma1 = LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17;
      break;
    }
    case 19U: {
      gamma1 = LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19;
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  out[0U] = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32(gamma1), out[0U]);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_gamma1_deserialize_a2(
    Eurydice_borrow_slice_u8 serialized, __m256i *out, size_t gamma1_exponent) {
  libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize(serialized, out,
                                                       gamma1_exponent);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m128i
libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize_4(
    const __m256i *simd_unit) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit[0U],
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 28, 0, 28, 0, 28, 0, 28));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      28, adjacent_2_combined, __m256i);
  __m256i adjacent_4_combined =
      libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
          adjacent_2_combined0,
          libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 0, 6, 2, 4, 0));
  __m128i adjacent_4_combined0 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_4_combined);
  return libcrux_intrinsics_avx2_mm_shuffle_epi8(
      adjacent_4_combined0, libcrux_intrinsics_avx2_mm_set_epi8(
                                -16, -16, -16, -16, -16, -16, -16, -16, -16,
                                -16, -16, -16, 12, 4, 8, 0));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize_6(
    const __m256i *simd_unit) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit[0U],
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 26, 0, 26, 0, 26, 0, 26));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      26, adjacent_2_combined, __m256i);
  __m256i adjacent_3_combined = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      adjacent_2_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 8, 1, 0, -1, -1,
          -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 8, 1, 0));
  __m256i adjacent_3_combined0 = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      adjacent_3_combined,
      libcrux_intrinsics_avx2_mm256_set_epi16(
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 0U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 0U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 0U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 4U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 0U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 0U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 0U),
          (int16_t)((uint32_t)1 << 0U), (int16_t)((uint32_t)1 << 4U)));
  return libcrux_intrinsics_avx2_mm256_srlv_epi32(
      adjacent_3_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 4, 0, 0, 0, 4));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_38 serialized = {{0U}};
  switch ((uint32_t)(uint8_t)out.meta) {
    case 4U: {
      libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
          Eurydice_array_to_subslice_mut_d47(
              &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
          libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize_4(simd_unit));
      Eurydice_slice_copy(
          out,
          Eurydice_array_to_subslice_shared_d41(
              &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)4U})),
          uint8_t);
      break;
    }
    case 6U: {
      __m256i adjacent_3_combined =
          libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize_6(simd_unit);
      __m128i lower_3 =
          libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_3_combined);
      __m128i upper_3 = libcrux_intrinsics_avx2_mm256_extracti128_si256(
          1, adjacent_3_combined, __m128i);
      libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
          Eurydice_array_to_subslice_mut_d47(
              &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
          lower_3);
      libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
          Eurydice_array_to_subslice_mut_d47(
              &serialized, (core_ops_range_Range_87{(size_t)3U, (size_t)19U})),
          upper_3);
      Eurydice_slice_copy(
          out,
          Eurydice_array_to_subslice_shared_d41(
              &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)6U})),
          uint8_t);
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_commitment_serialize_a2(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize(simd_unit, serialized);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_ETA_4 (4)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m128i
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_4_aux(
    __m256i simd_unit_shifted) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit_shifted,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 28, 0, 28, 0, 28, 0, 28));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      28, adjacent_2_combined, __m256i);
  __m256i adjacent_4_combined =
      libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
          adjacent_2_combined0,
          libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 0, 6, 2, 4, 0));
  __m128i adjacent_4_combined0 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_4_combined);
  return libcrux_intrinsics_avx2_mm_shuffle_epi8(
      adjacent_4_combined0, libcrux_intrinsics_avx2_mm_set_epi8(
                                -16, -16, -16, -16, -16, -16, -16, -16, -16,
                                -16, -16, -16, 12, 4, 8, 0));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_4(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_b2 serialized = {{0U}};
  __m256i simd_unit_shifted = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_ETA_4),
      simd_unit[0U]);
  __m128i adjacent_4_combined =
      libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_4_aux(
          simd_unit_shifted);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d48(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
      adjacent_4_combined);
  Eurydice_slice_copy(
      out,
      Eurydice_array_to_subslice_shared_d42(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)4U})),
      uint8_t);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_ETA_2 (2)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m128i
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_2_aux(
    __m256i simd_unit_shifted) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit_shifted,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 29, 0, 29, 0, 29, 0, 29));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      29, adjacent_2_combined, __m256i);
  __m256i adjacent_4_combined = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      adjacent_2_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 8, -1, 0, -1, -1,
          -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 8, -1, 0));
  __m256i adjacent_4_combined0 = libcrux_intrinsics_avx2_mm256_madd_epi16(
      adjacent_4_combined,
      libcrux_intrinsics_avx2_mm256_set_epi16(
          0, 0, 0, 0, 0, 0, (int16_t)((uint32_t)1 << 6U), 1, 0, 0, 0, 0, 0, 0,
          (int16_t)((uint32_t)1 << 6U), 1));
  __m256i adjacent_6_combined =
      libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
          adjacent_4_combined0,
          libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 0, 0, 0, 4, 0));
  __m128i adjacent_6_combined0 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_6_combined);
  __m128i adjacent_6_combined1 = libcrux_intrinsics_avx2_mm_sllv_epi32(
      adjacent_6_combined0, libcrux_intrinsics_avx2_mm_set_epi32(0, 0, 0, 20));
  return libcrux_intrinsics_avx2_mm_srli_epi64(20, adjacent_6_combined1,
                                               __m128i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_2(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_b2 serialized = {{0U}};
  __m256i simd_unit_shifted = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_ETA_2),
      simd_unit[0U]);
  __m128i adjacent_6_combined =
      libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_2_aux(
          simd_unit_shifted);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d48(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
      adjacent_6_combined);
  Eurydice_slice_copy(
      out,
      Eurydice_array_to_subslice_shared_d42(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)3U})),
      uint8_t);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_error_serialize(
    libcrux_ml_dsa_constants_Eta eta, const __m256i *simd_unit,
    Eurydice_mut_borrow_slice_u8 serialized) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_4(
          simd_unit, serialized);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_2(simd_unit,
                                                                  serialized);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_error_serialize_a2(
    libcrux_ml_dsa_constants_Eta eta, const __m256i *simd_unit,
    Eurydice_mut_borrow_slice_u8 serialized) {
  libcrux_ml_dsa_simd_avx2_encoding_error_serialize(eta, simd_unit, serialized);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_error_deserialize(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized,
    __m256i *out) {
  __m256i unsigned0 =
      libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned(
          eta, serialized);
  int32_t eta_v;
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      eta_v = 2;
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      eta_v = 4;
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  out[0U] = libcrux_intrinsics_avx2_mm256_sub_epi32(
      libcrux_intrinsics_avx2_mm256_set1_epi32(eta_v), unsigned0);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_error_deserialize_a2(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized,
    __m256i *out) {
  libcrux_ml_dsa_simd_avx2_encoding_error_deserialize(eta, serialized, out);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T0_POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE \
  ((int32_t)((uint32_t)1                                                             \
             << (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T -        \
                           (size_t)1U)))

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_encoding_t0_change_interval(const __m256i *simd_unit) {
  __m256i interval_end = libcrux_intrinsics_avx2_mm256_set1_epi32(
      LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T0_POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE);
  return libcrux_intrinsics_avx2_mm256_sub_epi32(interval_end, simd_unit[0U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m128i
libcrux_ml_dsa_simd_avx2_encoding_t0_serialize_aux(__m256i simd_unit) {
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 19, 0, 19, 0, 19, 0, 19));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      19, adjacent_2_combined, __m256i);
  __m256i adjacent_4_combined =
      libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
          adjacent_2_combined0,
          libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 0, 6, 4, 2, 0));
  __m256i adjacent_4_combined0 = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      adjacent_4_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 6, 0, 6, 0, 6, 0, 6));
  __m256i adjacent_4_combined1 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      6, adjacent_4_combined0, __m256i);
  __m256i second_4_combined = libcrux_intrinsics_avx2_mm256_bsrli_epi128(
      8, adjacent_4_combined1, __m256i);
  __m256i least_12_bits_shifted_up =
      libcrux_intrinsics_avx2_mm256_slli_epi64(52, second_4_combined, __m256i);
  __m256i bits_sequential = libcrux_intrinsics_avx2_mm256_add_epi64(
      adjacent_4_combined1, least_12_bits_shifted_up);
  __m256i bits_sequential0 = libcrux_intrinsics_avx2_mm256_srlv_epi64(
      bits_sequential,
      libcrux_intrinsics_avx2_mm256_set_epi64x(0LL, 0LL, 12LL, 0LL));
  return libcrux_intrinsics_avx2_mm256_castsi256_si128(bits_sequential0);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_t0_serialize(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_b2 serialized = {{0U}};
  __m256i simd_unit_changed =
      libcrux_ml_dsa_simd_avx2_encoding_t0_change_interval(simd_unit);
  __m128i bits_sequential =
      libcrux_ml_dsa_simd_avx2_encoding_t0_serialize_aux(simd_unit_changed);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_slice_mut_29(&serialized), bits_sequential);
  Eurydice_slice_copy(
      out,
      Eurydice_array_to_subslice_shared_d42(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)13U})),
      uint8_t);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_t0_serialize_a2(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_dsa_simd_avx2_encoding_t0_serialize(simd_unit, out);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T0_DESERIALIZE_UNSIGNED_COEFFICIENT_MASK \
  ((int32_t)((uint32_t)1 << 13U) - 1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_encoding_t0_deserialize_unsigned(
    Eurydice_borrow_slice_u8 serialized, __m256i *out) {
  Eurydice_arr_b2 serialized_extended = {{0U}};
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d48(
                          &serialized_extended,
                          (core_ops_range_Range_87{(size_t)0U, (size_t)13U})),
                      serialized, uint8_t);
  __m128i serialized0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&serialized_extended));
  __m256i serialized1 =
      libcrux_intrinsics_avx2_mm256_set_m128i(serialized0, serialized0);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      serialized1, libcrux_intrinsics_avx2_mm256_set_epi8(
                       -1, -1, 12, 11, -1, 11, 10, 9, -1, -1, 9, 8, -1, 8, 7, 6,
                       -1, 6, 5, 4, -1, -1, 4, 3, -1, 3, 2, 1, -1, -1, 1, 0));
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_srlv_epi32(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi32(3, 6, 1, 4, 7, 2, 5, 0));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients0,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T0_DESERIALIZE_UNSIGNED_COEFFICIENT_MASK));
  out[0U] = coefficients1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_t0_deserialize(
    Eurydice_borrow_slice_u8 serialized, __m256i *out) {
  libcrux_ml_dsa_simd_avx2_encoding_t0_deserialize_unsigned(serialized, out);
  out[0U] = libcrux_ml_dsa_simd_avx2_encoding_t0_change_interval(out);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_t0_deserialize_a2(
    Eurydice_borrow_slice_u8 serialized, __m256i *out) {
  libcrux_ml_dsa_simd_avx2_encoding_t0_deserialize(serialized, out);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_t1_serialize(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  Eurydice_arr_94 serialized = {{0U}};
  __m256i adjacent_2_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      simd_unit[0U],
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 22, 0, 22, 0, 22, 0, 22));
  __m256i adjacent_2_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      22, adjacent_2_combined, __m256i);
  __m256i adjacent_4_combined =
      libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
          adjacent_2_combined0,
          libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 6, 4, 0, 0, 2, 0));
  __m256i adjacent_4_combined0 = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      adjacent_4_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 12, 0, 12, 0, 12, 0, 12));
  __m256i adjacent_4_combined1 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      12, adjacent_4_combined0, __m256i);
  __m128i lower_4 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_4_combined1);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d49(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)16U})),
      lower_4);
  __m128i upper_4 = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      1, adjacent_4_combined1, __m128i);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_d49(
          &serialized, (core_ops_range_Range_87{(size_t)5U, (size_t)21U})),
      upper_4);
  Eurydice_slice_copy(
      out,
      Eurydice_array_to_subslice_shared_d43(
          &serialized, (core_ops_range_Range_87{(size_t)0U, (size_t)10U})),
      uint8_t);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_t1_serialize_a2(
    const __m256i *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_dsa_simd_avx2_encoding_t1_serialize(simd_unit, out);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T1_DESERIALIZE_COEFFICIENT_MASK \
  ((int32_t)((uint32_t)1 << 10U) - 1)

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_encoding_t1_deserialize(
    Eurydice_borrow_slice_u8 bytes, __m256i *out) {
  Eurydice_arr_b2 bytes_extended = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d48(
          &bytes_extended, (core_ops_range_Range_87{(size_t)0U, (size_t)10U})),
      bytes, uint8_t);
  __m128i bytes_loaded = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_29(&bytes_extended));
  __m256i bytes_loaded0 =
      libcrux_intrinsics_avx2_mm256_set_m128i(bytes_loaded, bytes_loaded);
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      bytes_loaded0,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          -1, -1, 9, 8, -1, -1, 8, 7, -1, -1, 7, 6, -1, -1, 6, 5, -1, -1, 4, 3,
          -1, -1, 3, 2, -1, -1, 2, 1, -1, -1, 1, 0));
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_srlv_epi32(
      coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi32(6, 4, 2, 0, 6, 4, 2, 0));
  out[0U] = libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients0,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T1_DESERIALIZE_COEFFICIENT_MASK));
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_t1_deserialize_a2(
    Eurydice_borrow_slice_u8 serialized, __m256i *out) {
  libcrux_ml_dsa_simd_avx2_encoding_t1_deserialize(serialized, out);
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_60_s {
  __m256i data[32U];
} Eurydice_arr_60;

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
    Eurydice_arr_60 *re, size_t index, __m256i zeta, size_t step_by,
    __m256i field_modulus, __m256i inverse_of_modulus_mod_montgomery_r) {
  __m256i t = re->data[index + step_by];
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_aux(
      field_modulus, inverse_of_modulus_mod_montgomery_r, &t, &zeta);
  re->data[index + step_by] = re->data[index];
  libcrux_ml_dsa_simd_avx2_arithmetic_subtract(&re->data[index + step_by], &t);
  libcrux_ml_dsa_simd_avx2_arithmetic_add(&re->data[index], &t);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7 \
  ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6 ((size_t)8U)

/**
 This is equivalent to the pqclean 0 and 1

 This does 32 Montgomery multiplications (192 multiplications).
 This is the same as in pqclean. The only difference is locality of registers.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6(
    Eurydice_arr_60 *re) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi32(
      LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  __m256i inverse_of_modulus_mod_montgomery_r =
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          (int32_t)
              LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R);
  __m256i zeta7 = libcrux_intrinsics_avx2_mm256_set1_epi32(25847);
  __m256i zeta60 = libcrux_intrinsics_avx2_mm256_set1_epi32(-2608894);
  __m256i zeta61 = libcrux_intrinsics_avx2_mm256_set1_epi32(-518909);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U + (size_t)1U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U + (size_t)2U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U + (size_t)3U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)8U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)8U + (size_t)1U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)8U + (size_t)2U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)8U + (size_t)3U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U + (size_t)1U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U + (size_t)2U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)0U + (size_t)3U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)16U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)16U + (size_t)1U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)16U + (size_t)2U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)16U + (size_t)3U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U + (size_t)1U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U + (size_t)2U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U + (size_t)3U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)12U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)12U + (size_t)1U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)12U + (size_t)2U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)12U + (size_t)3U, zeta7,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U + (size_t)1U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U + (size_t)2U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)4U + (size_t)3U, zeta60,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)20U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)20U + (size_t)1U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)20U + (size_t)2U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
      re, (size_t)20U + (size_t)3U, zeta61,
      LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6,
      field_modulus, inverse_of_modulus_mod_montgomery_r);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.ntt.ntt_at_layer_5_to_3.round
with const generics
- STEP= 32
- STEP_BY= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_90(Eurydice_arr_60 *re,
                                                          size_t index,
                                                          int32_t zeta) {
  __m256i rhs = libcrux_intrinsics_avx2_mm256_set1_epi32(zeta);
  size_t offset = index * (size_t)32U * (size_t)2U /
                  LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT;
  for (size_t i = offset; i < offset + (size_t)4U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(
        &re->data[j + (size_t)4U], &rhs);
    __m256i tmp = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j], re->data[j + (size_t)4U]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)4U]);
    re->data[j + (size_t)4U] = tmp;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.ntt.ntt_at_layer_5_to_3.round
with const generics
- STEP= 16
- STEP_BY= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(Eurydice_arr_60 *re,
                                                          size_t index,
                                                          int32_t zeta) {
  __m256i rhs = libcrux_intrinsics_avx2_mm256_set1_epi32(zeta);
  size_t offset = index * (size_t)16U * (size_t)2U /
                  LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT;
  for (size_t i = offset; i < offset + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(
        &re->data[j + (size_t)2U], &rhs);
    __m256i tmp = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] = tmp;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.ntt.ntt_at_layer_5_to_3.round
with const generics
- STEP= 8
- STEP_BY= 1
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(Eurydice_arr_60 *re,
                                                          size_t index,
                                                          int32_t zeta) {
  __m256i rhs = libcrux_intrinsics_avx2_mm256_set1_epi32(zeta);
  size_t offset = index * (size_t)8U * (size_t)2U /
                  LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT;
  for (size_t i = offset; i < offset + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(
        &re->data[j + (size_t)1U], &rhs);
    __m256i tmp = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] = tmp;
  }
}

/**
 Layer 5, 4, 3

 Each layer does 16 Montgomery multiplications -> 3*16 = 48 total
 pqclean does 4 * 4 on each layer -> 48 total | plus 4 * 4 shuffles every time
 (48)
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_90(re, (size_t)0U,
                                                            237124);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_90(re, (size_t)1U,
                                                            -777960);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_90(re, (size_t)2U,
                                                            -876248);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_90(re, (size_t)3U,
                                                            466468);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)0U,
                                                            1826347);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)1U,
                                                            2353451);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)2U,
                                                            -359251);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)3U,
                                                            -2091905);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)4U,
                                                            3119733);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)5U,
                                                            -2884855);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)6U,
                                                            3111497);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(re, (size_t)7U,
                                                            2680103);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)0U,
                                                            2725464);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)1U,
                                                            1024112);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)2U,
                                                            -1079900);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)3U,
                                                            3585928);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)4U,
                                                            -549488);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)5U,
                                                            -1119584);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)6U,
                                                            2619752);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)7U,
                                                            -2108549);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)8U,
                                                            -2118186);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)9U,
                                                            -3859737);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)10U,
                                                            -1399561);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)11U,
                                                            -3277672);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)12U,
                                                            1757237);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)13U,
                                                            -19422);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)14U,
                                                            4010497);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(re, (size_t)15U,
                                                            280005);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(
    Eurydice_arr_60 *re, size_t index, int32_t zeta0, int32_t zeta1) {
  __m256i re0 = re->data[index];
  __m256i re1 = re->data[index + (size_t)1U];
  __m256i summands = libcrux_intrinsics_avx2_mm256_set_m128i(
      libcrux_intrinsics_avx2_mm256_castsi256_si128(re1),
      libcrux_intrinsics_avx2_mm256_castsi256_si128(re0));
  __m256i zeta_products =
      libcrux_intrinsics_avx2_mm256_permute2x128_si256(19, re1, re0, __m256i);
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi32(
      zeta1, zeta1, zeta1, zeta1, zeta0, zeta0, zeta0, zeta0);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(&zeta_products,
                                                          &zetas);
  __m256i sub_terms =
      libcrux_intrinsics_avx2_mm256_sub_epi32(summands, zeta_products);
  __m256i add_terms =
      libcrux_intrinsics_avx2_mm256_add_epi32(summands, zeta_products);
  __m256i nre0 = libcrux_intrinsics_avx2_mm256_set_m128i(
      libcrux_intrinsics_avx2_mm256_castsi256_si128(sub_terms),
      libcrux_intrinsics_avx2_mm256_castsi256_si128(add_terms));
  __m256i nre1 = libcrux_intrinsics_avx2_mm256_permute2x128_si256(
      19, sub_terms, add_terms, __m256i);
  re->data[index] = nre0;
  re->data[index + (size_t)1U] = nre1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_2(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)0U, 2706023, 95776);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)2U, 3077325, 3530437);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)4U, -1661693, -3592148);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)6U, -2537516, 3915439);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)8U, -3861115, -3043716);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)10U, 3574422, -2867647);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)12U, 3539968, -300467);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)14U, 2348700, -539299);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)16U, -1699267, -1643818);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)18U, 3505694, -3821735);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)20U, 3507263, -2140649);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)22U, -1600420, 3699596);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)24U, 811944, 531354);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)26U, 954230, 3881043);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)28U, 3900724, -2556880);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(re, (size_t)30U, 2071892, -2797779);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(
    Eurydice_arr_60 *re, size_t index, int32_t zeta_a0, int32_t zeta_a1,
    int32_t zeta_b0, int32_t zeta_b1) {
  __m256i re0 = re->data[index];
  __m256i re1 = re->data[index + (size_t)1U];
  __m256i summands = libcrux_intrinsics_avx2_mm256_unpacklo_epi64(re0, re1);
  __m256i zeta_products =
      libcrux_intrinsics_avx2_mm256_unpackhi_epi64(re0, re1);
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi32(
      zeta_b1, zeta_b1, zeta_a1, zeta_a1, zeta_b0, zeta_b0, zeta_a0, zeta_a0);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(&zeta_products,
                                                          &zetas);
  __m256i sub_terms =
      libcrux_intrinsics_avx2_mm256_sub_epi32(summands, zeta_products);
  __m256i add_terms =
      libcrux_intrinsics_avx2_mm256_add_epi32(summands, zeta_products);
  __m256i nre0 =
      libcrux_intrinsics_avx2_mm256_unpacklo_epi64(add_terms, sub_terms);
  __m256i nre1 =
      libcrux_intrinsics_avx2_mm256_unpackhi_epi64(add_terms, sub_terms);
  re->data[index] = nre0;
  re->data[index + (size_t)1U] = nre1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_1(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)0U, -3930395, -1528703,
                                           -3677745, -3041255);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)2U, -1452451, 3475950,
                                           2176455, -1585221);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)4U, -1257611, 1939314,
                                           -4083598, -1000202);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)6U, -3190144, -3157330,
                                           -3632928, 126922);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)8U, 3412210, -983419,
                                           2147896, 2715295);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)10U, -2967645, -3693493,
                                           -411027, -2477047);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)12U, -671102, -1228525,
                                           -22981, -1308169);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)14U, -381987, 1349076,
                                           1852771, -1430430);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)16U, -3343383, 264944,
                                           508951, 3097992);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)18U, 44288, -1100098,
                                           904516, 3958618);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)20U, -3724342, -8578,
                                           1653064, -3249728);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)22U, 2389356, -210977,
                                           759969, -1316856);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)24U, 189548, -3553272,
                                           3159746, -1851402);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)26U, -2409325, -177440,
                                           1315589, 1341330);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)28U, 1285669, -1584928,
                                           -812732, -1439742);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(re, (size_t)30U, -3019102, -3881060,
                                           -3628969, 3839961);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(
    Eurydice_arr_60 *re, size_t index, int32_t zeta_a0, int32_t zeta_a1,
    int32_t zeta_a2, int32_t zeta_a3, int32_t zeta_b0, int32_t zeta_b1,
    int32_t zeta_b2, int32_t zeta_b3) {
  __m256i re0 = re->data[index];
  __m256i re1 = re->data[index + (size_t)1U];
  __m256i a = libcrux_intrinsics_avx2_mm256_shuffle_epi32(216, re0, __m256i);
  __m256i b = libcrux_intrinsics_avx2_mm256_shuffle_epi32(216, re1, __m256i);
  __m256i summands = libcrux_intrinsics_avx2_mm256_unpacklo_epi64(a, b);
  __m256i zeta_products = libcrux_intrinsics_avx2_mm256_unpackhi_epi64(a, b);
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi32(
      zeta_b3, zeta_b2, zeta_a3, zeta_a2, zeta_b1, zeta_b0, zeta_a1, zeta_a0);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(&zeta_products,
                                                          &zetas);
  __m256i sub_terms =
      libcrux_intrinsics_avx2_mm256_sub_epi32(summands, zeta_products);
  __m256i add_terms =
      libcrux_intrinsics_avx2_mm256_add_epi32(summands, zeta_products);
  __m256i a_terms_shuffled =
      libcrux_intrinsics_avx2_mm256_unpacklo_epi64(add_terms, sub_terms);
  __m256i b_terms_shuffled =
      libcrux_intrinsics_avx2_mm256_unpackhi_epi64(add_terms, sub_terms);
  __m256i nre0 = libcrux_intrinsics_avx2_mm256_shuffle_epi32(
      216, a_terms_shuffled, __m256i);
  __m256i nre1 = libcrux_intrinsics_avx2_mm256_shuffle_epi32(
      216, b_terms_shuffled, __m256i);
  re->data[index] = nre0;
  re->data[index + (size_t)1U] = nre1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_0(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)0U, 2091667, 3407706,
                                           2316500, 3817976, -3342478, 2244091,
                                           -2446433, -3562462);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)2U, 266997, 2434439,
                                           -1235728, 3513181, -3520352,
                                           -3759364, -1197226, -3193378);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)4U, 900702, 1859098,
                                           909542, 819034, 495491, -1613174,
                                           -43260, -522500);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)6U, -655327, -3122442,
                                           2031748, 3207046, -3556995, -525098,
                                           -768622, -3595838);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)8U, 342297, 286988,
                                           -2437823, 4108315, 3437287, -3342277,
                                           1735879, 203044);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)10U, 2842341, 2691481,
                                           -2590150, 1265009, 4055324, 1247620,
                                           2486353, 1595974);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)12U, -3767016, 1250494,
                                           2635921, -3548272, -2994039, 1869119,
                                           1903435, -1050970);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)14U, -1333058, 1237275,
                                           -3318210, -1430225, -451100, 1312455,
                                           3306115, -1962642);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)16U, -1279661, 1917081,
                                           -2546312, -1374803, 1500165, 777191,
                                           2235880, 3406031);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)18U, -542412, -2831860,
                                           -1671176, -1846953, -2584293,
                                           -3724270, 594136, -3776993);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)20U, -2013608, 2432395,
                                           2454455, -164721, 1957272, 3369112,
                                           185531, -1207385);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)22U, -3183426, 162844,
                                           1616392, 3014001, 810149, 1652634,
                                           -3694233, -1799107);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)24U, -3038916, 3523897,
                                           3866901, 269760, 2213111, -975884,
                                           1717735, 472078);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)26U, -426683, 1723600,
                                           -1803090, 1910376, -1667432,
                                           -1104333, -260646, -3833893);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)28U, -2939036, -2235985,
                                           -420899, -2286327, 183443, -976891,
                                           1612842, -3545687);
  libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(re, (size_t)30U, -554416, 3919660,
                                           -48306, -1362209, 3937738, 1400424,
                                           -846154, 1976782);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_ntt_ntt_avx2_ntt(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6(re);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3(re);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_2(re);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_1(re);
  libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_0(re);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_ntt_ntt(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_ntt_avx2_ntt(re);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_ntt_a2(
    Eurydice_arr_60 *simd_units) {
  libcrux_ml_dsa_simd_avx2_ntt_ntt(simd_units);
}

typedef struct libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2_s {
  __m256i fst;
  __m256i snd;
} libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2;

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2
libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_0(
    __m256i simd_unit0, __m256i simd_unit1, int32_t zeta00, int32_t zeta01,
    int32_t zeta02, int32_t zeta03, int32_t zeta10, int32_t zeta11,
    int32_t zeta12, int32_t zeta13) {
  __m256i a_shuffled =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(216, simd_unit0, __m256i);
  __m256i b_shuffled0 =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(216, simd_unit1, __m256i);
  __m256i lo_values =
      libcrux_intrinsics_avx2_mm256_unpacklo_epi64(a_shuffled, b_shuffled0);
  __m256i hi_values =
      libcrux_intrinsics_avx2_mm256_unpackhi_epi64(a_shuffled, b_shuffled0);
  __m256i differences = hi_values;
  libcrux_ml_dsa_simd_avx2_arithmetic_subtract(&differences, &lo_values);
  libcrux_ml_dsa_simd_avx2_arithmetic_add(&lo_values, &hi_values);
  __m256i sums = lo_values;
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi32(
      zeta13, zeta12, zeta03, zeta02, zeta11, zeta10, zeta01, zeta00);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(&differences, &zetas);
  __m256i a_shuffled0 =
      libcrux_intrinsics_avx2_mm256_unpacklo_epi64(sums, differences);
  __m256i b_shuffled =
      libcrux_intrinsics_avx2_mm256_unpackhi_epi64(sums, differences);
  __m256i a =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(216, a_shuffled0, __m256i);
  __m256i b =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(216, b_shuffled, __m256i);
  return (libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2{a, b});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
    Eurydice_arr_60 *re, size_t index, int32_t zeta00, int32_t zeta01,
    int32_t zeta02, int32_t zeta03, int32_t zeta10, int32_t zeta11,
    int32_t zeta12, int32_t zeta13) {
  libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2 uu____0 =
      libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_0(
          re->data[index], re->data[index + (size_t)1U], zeta00, zeta01, zeta02,
          zeta03, zeta10, zeta11, zeta12, zeta13);
  __m256i uu____1 = uu____0.snd;
  re->data[index] = uu____0.fst;
  re->data[index + (size_t)1U] = uu____1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)0U, 1976782, -846154, 1400424, 3937738, -1362209, -48306,
      3919660, -554416);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)2U, -3545687, 1612842, -976891, 183443, -2286327, -420899,
      -2235985, -2939036);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)4U, -3833893, -260646, -1104333, -1667432, 1910376, -1803090,
      1723600, -426683);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)6U, 472078, 1717735, -975884, 2213111, 269760, 3866901,
      3523897, -3038916);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)8U, -1799107, -3694233, 1652634, 810149, 3014001, 1616392,
      162844, -3183426);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)10U, -1207385, 185531, 3369112, 1957272, -164721, 2454455,
      2432395, -2013608);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)12U, -3776993, 594136, -3724270, -2584293, -1846953, -1671176,
      -2831860, -542412);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)14U, 3406031, 2235880, 777191, 1500165, -1374803, -2546312,
      1917081, -1279661);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)16U, -1962642, 3306115, 1312455, -451100, -1430225, -3318210,
      1237275, -1333058);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)18U, -1050970, 1903435, 1869119, -2994039, -3548272, 2635921,
      1250494, -3767016);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)20U, 1595974, 2486353, 1247620, 4055324, 1265009, -2590150,
      2691481, 2842341);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)22U, 203044, 1735879, -3342277, 3437287, 4108315, -2437823,
      286988, 342297);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)24U, -3595838, -768622, -525098, -3556995, 3207046, 2031748,
      -3122442, -655327);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)26U, -522500, -43260, -1613174, 495491, 819034, 909542,
      1859098, 900702);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)28U, -3193378, -1197226, -3759364, -3520352, 3513181,
      -1235728, 2434439, 266997);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)30U, -3562462, -2446433, 2244091, -3342478, 3817976, 2316500,
      3407706, 2091667);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2
libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_1(
    __m256i simd_unit0, __m256i simd_unit1, int32_t zeta00, int32_t zeta01,
    int32_t zeta10, int32_t zeta11) {
  __m256i lo_values =
      libcrux_intrinsics_avx2_mm256_unpacklo_epi64(simd_unit0, simd_unit1);
  __m256i hi_values =
      libcrux_intrinsics_avx2_mm256_unpackhi_epi64(simd_unit0, simd_unit1);
  __m256i differences = hi_values;
  libcrux_ml_dsa_simd_avx2_arithmetic_subtract(&differences, &lo_values);
  libcrux_ml_dsa_simd_avx2_arithmetic_add(&lo_values, &hi_values);
  __m256i sums = lo_values;
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi32(
      zeta11, zeta11, zeta01, zeta01, zeta10, zeta10, zeta00, zeta00);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(&differences, &zetas);
  __m256i a = libcrux_intrinsics_avx2_mm256_unpacklo_epi64(sums, differences);
  __m256i b = libcrux_intrinsics_avx2_mm256_unpackhi_epi64(sums, differences);
  return (libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2{a, b});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
    Eurydice_arr_60 *re, size_t index, int32_t zeta_00, int32_t zeta_01,
    int32_t zeta_10, int32_t zeta_11) {
  libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2 uu____0 =
      libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_1(
          re->data[index], re->data[index + (size_t)1U], zeta_00, zeta_01,
          zeta_10, zeta_11);
  __m256i uu____1 = uu____0.snd;
  re->data[index] = uu____0.fst;
  re->data[index + (size_t)1U] = uu____1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)0U, 3839961, -3628969, -3881060, -3019102);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)2U, -1439742, -812732, -1584928, 1285669);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)4U, 1341330, 1315589, -177440, -2409325);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)6U, -1851402, 3159746, -3553272, 189548);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)8U, -1316856, 759969, -210977, 2389356);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)10U, -3249728, 1653064, -8578, -3724342);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)12U, 3958618, 904516, -1100098, 44288);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)14U, 3097992, 508951, 264944, -3343383);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)16U, -1430430, 1852771, 1349076, -381987);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)18U, -1308169, -22981, -1228525, -671102);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)20U, -2477047, -411027, -3693493, -2967645);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)22U, 2715295, 2147896, -983419, 3412210);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)24U, 126922, -3632928, -3157330, -3190144);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)26U, -1000202, -4083598, 1939314, -1257611);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)28U, -1585221, 2176455, 3475950, -1452451);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)30U, -3041255, -3677745, -1528703, -3930395);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2
libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_2(
    __m256i simd_unit0, __m256i simd_unit1, int32_t zeta0, int32_t zeta1) {
  __m256i lo_values = libcrux_intrinsics_avx2_mm256_permute2x128_si256(
      32, simd_unit0, simd_unit1, __m256i);
  __m256i hi_values = libcrux_intrinsics_avx2_mm256_permute2x128_si256(
      49, simd_unit0, simd_unit1, __m256i);
  __m256i differences = hi_values;
  libcrux_ml_dsa_simd_avx2_arithmetic_subtract(&differences, &lo_values);
  libcrux_ml_dsa_simd_avx2_arithmetic_add(&lo_values, &hi_values);
  __m256i sums = lo_values;
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi32(
      zeta1, zeta1, zeta1, zeta1, zeta0, zeta0, zeta0, zeta0);
  libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(&differences, &zetas);
  __m256i a = libcrux_intrinsics_avx2_mm256_permute2x128_si256(
      32, sums, differences, __m256i);
  __m256i b = libcrux_intrinsics_avx2_mm256_permute2x128_si256(
      49, sums, differences, __m256i);
  return (libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2{a, b});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(Eurydice_arr_60 *re,
                                                            size_t index,
                                                            int32_t zeta1,
                                                            int32_t zeta2) {
  libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2 uu____0 =
      libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_2(
          re->data[index], re->data[index + (size_t)1U], zeta1, zeta2);
  __m256i uu____1 = uu____0.snd;
  re->data[index] = uu____0.fst;
  re->data[index + (size_t)1U] = uu____1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)0U, -2797779, 2071892);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)2U, -2556880, 3900724);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(re, (size_t)4U,
                                                              3881043, 954230);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(re, (size_t)6U,
                                                              531354, 811944);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)8U, 3699596, -1600420);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)10U, -2140649, 3507263);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)12U, -3821735, 3505694);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)14U, -1643818, -1699267);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(re, (size_t)16U,
                                                              -539299, 2348700);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(re, (size_t)18U,
                                                              -300467, 3539968);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)20U, -2867647, 3574422);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)22U, -3043716, -3861115);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)24U, 3915439, -2537516);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)26U, -3592148, -1661693);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(re, (size_t)28U,
                                                              3530437, 3077325);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(re, (size_t)30U,
                                                              95776, 2706023);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
    __m256i lhs, int32_t constant) {
  __m256i rhs = libcrux_intrinsics_avx2_mm256_set1_epi32(constant);
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi32(
      LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  __m256i inverse_of_modulus_mod_montgomery_r =
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          (int32_t)
              LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R);
  __m256i prod02 = libcrux_intrinsics_avx2_mm256_mul_epi32(lhs, rhs);
  __m256i prod13 = libcrux_intrinsics_avx2_mm256_mul_epi32(
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, lhs, __m256i),
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, rhs, __m256i));
  __m256i k02 = libcrux_intrinsics_avx2_mm256_mul_epi32(
      prod02, inverse_of_modulus_mod_montgomery_r);
  __m256i k13 = libcrux_intrinsics_avx2_mm256_mul_epi32(
      prod13, inverse_of_modulus_mod_montgomery_r);
  __m256i c02 = libcrux_intrinsics_avx2_mm256_mul_epi32(k02, field_modulus);
  __m256i c13 = libcrux_intrinsics_avx2_mm256_mul_epi32(k13, field_modulus);
  __m256i res02 = libcrux_intrinsics_avx2_mm256_sub_epi32(prod02, c02);
  __m256i res13 = libcrux_intrinsics_avx2_mm256_sub_epi32(prod13, c13);
  __m256i res02_shifted =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, res02, __m256i);
  return libcrux_intrinsics_avx2_mm256_blend_epi32(170, res02_shifted, res13,
                                                   __m256i);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 1
- ZETA= 280005
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_30(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 280005);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 2
- STEP_BY= 1
- ZETA= 4010497
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_25(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)2U; i < (size_t)2U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 4010497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 1
- ZETA= -19422
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_43(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -19422);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 6
- STEP_BY= 1
- ZETA= 1757237
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_f4(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)6U; i < (size_t)6U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 1757237);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 1
- ZETA= -3277672
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_82(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -3277672);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 10
- STEP_BY= 1
- ZETA= -1399561
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_1d(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)10U; i < (size_t)10U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -1399561);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 1
- ZETA= -3859737
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_ea(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -3859737);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 14
- STEP_BY= 1
- ZETA= -2118186
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_d8(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)14U; i < (size_t)14U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -2118186);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 1
- ZETA= -2108549
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_42(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -2108549);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 18
- STEP_BY= 1
- ZETA= 2619752
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_60(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)18U; i < (size_t)18U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 2619752);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 1
- ZETA= -1119584
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_61(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -1119584);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 22
- STEP_BY= 1
- ZETA= -549488
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_29(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)22U; i < (size_t)22U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -549488);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 1
- ZETA= 3585928
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 3585928);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 26
- STEP_BY= 1
- ZETA= -1079900
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_9d(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)26U; i < (size_t)26U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -1079900);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 1
- ZETA= 1024112
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_38(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 1024112);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 30
- STEP_BY= 1
- ZETA= 2725464
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_5f(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)30U; i < (size_t)30U + (size_t)1U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)1U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)1U]);
    re->data[j + (size_t)1U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 2725464);
  }
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_3(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_30(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_25(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_43(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_f4(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_82(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_1d(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_ea(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_d8(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_42(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_60(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_61(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_29(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_9d(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_38(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_5f(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 2
- ZETA= 2680103
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_300(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 2680103);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 2
- ZETA= 3111497
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_430(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 3111497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 2
- ZETA= -2884855
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_820(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -2884855);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 2
- ZETA= 3119733
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_ea0(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 3119733);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 2
- ZETA= -2091905
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_420(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -2091905);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 2
- ZETA= -359251
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_610(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -359251);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 2
- ZETA= 2353451
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe0(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 2353451);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 2
- ZETA= 1826347
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_380(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)2U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)2U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)2U]);
    re->data[j + (size_t)2U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 1826347);
  }
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_4(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_300(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_430(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_820(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_ea0(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_420(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_610(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe0(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_380(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 4
- ZETA= 466468
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_301(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)4U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)4U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)4U]);
    re->data[j + (size_t)4U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 466468);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 4
- ZETA= -876248
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_821(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)4U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)4U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)4U]);
    re->data[j + (size_t)4U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -876248);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 4
- ZETA= -777960
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_421(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)4U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)4U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)4U]);
    re->data[j + (size_t)4U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -777960);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 4
- ZETA= 237124
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe1(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)4U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)4U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)4U]);
    re->data[j + (size_t)4U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 237124);
  }
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_5(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_301(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_821(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_421(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe1(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 8
- ZETA= -518909
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_302(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)8U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)8U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)8U]);
    re->data[j + (size_t)8U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -518909);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 8
- ZETA= -2608894
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_422(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)8U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)8U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)8U]);
    re->data[j + (size_t)8U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, -2608894);
  }
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_6(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_302(re);
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_422(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 16
- ZETA= 25847
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_303(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)16U; i++) {
    size_t j = i;
    __m256i a_minus_b = libcrux_intrinsics_avx2_mm256_sub_epi32(
        re->data[j + (size_t)16U], re->data[j]);
    re->data[j] = libcrux_intrinsics_avx2_mm256_add_epi32(
        re->data[j], re->data[j + (size_t)16U]);
    re->data[j + (size_t)16U] =
        libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
            a_minus_b, 25847);
  }
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_7(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_303(re);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_MONTGOMERY_INV_INNER_FACTOR \
  (41978)

KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_montgomery_inv_inner(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_3(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_4(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_5(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_6(re);
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_7(re);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    re->data
        [i0] = libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
        re->data[i0],
        LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_MONTGOMERY_INV_INNER_FACTOR);
  }
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_montgomery(Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_montgomery_inv_inner(re);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_avx2_invert_ntt_montgomery_a2(
    Eurydice_arr_60 *simd_units) {
  libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_montgomery(simd_units);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_barrett_reduce_simd_unit_a2(__m256i *simd_unit) {
  libcrux_ml_dsa_simd_avx2_arithmetic_barrett_reduce_simd_unit(simd_unit);
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b
with const generics
- $11size_t
*/
typedef struct Eurydice_arr_4f_s {
  Eurydice_arr_60 data[11U];
} Eurydice_arr_4f;

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.zero_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_60 libcrux_ml_dsa_polynomial_zero_ff_64(void) {
  Eurydice_arr_60 lit;
  __m256i repeat_expression[32U];
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    repeat_expression[i] = libcrux_ml_dsa_simd_avx2_zero_a2();
  }
  memcpy(lit.data, repeat_expression, (size_t)32U * sizeof(__m256i));
  return lit;
}

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b, size_t

*/
typedef struct Eurydice_dst_ref_mut_ba_s {
  Eurydice_arr_60 *ptr;
  size_t meta;
} Eurydice_dst_ref_mut_ba;

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b, size_t

*/
typedef struct Eurydice_dst_ref_shared_ba_s {
  const Eurydice_arr_60 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_ba;

/**
A monomorphic instance of
libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_4 with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_64(
    Eurydice_borrow_slice_u8 randomness, size_t *sampled_coefficients,
    Eurydice_arr_d0 *out) {
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)4U; i++) {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8 random_bytes = Eurydice_slice_subslice_shared_c8(
        randomness,
        (core_ops_range_Range_87{_cloop_i * (size_t)4U,
                                 _cloop_i * (size_t)4U + (size_t)4U}));
    if (!done) {
      size_t sampled =
          libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_equals_4_a2(
              random_bytes, Eurydice_array_to_subslice_from_mut_11(
                                out, sampled_coefficients[0U]));
      sampled_coefficients[0U] += sampled;
      if (sampled_coefficients[0U] >=
          LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        done = true;
      }
    }
  }
  return done;
}

/**
A monomorphic instance of
libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_2 with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_64(
    Eurydice_borrow_slice_u8 randomness, size_t *sampled_coefficients,
    Eurydice_arr_d0 *out) {
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)4U; i++) {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8 random_bytes = Eurydice_slice_subslice_shared_c8(
        randomness,
        (core_ops_range_Range_87{_cloop_i * (size_t)4U,
                                 _cloop_i * (size_t)4U + (size_t)4U}));
    if (!done) {
      size_t sampled =
          libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_equals_2_a2(
              random_bytes, Eurydice_array_to_subslice_from_mut_11(
                                out, sampled_coefficients[0U]));
      sampled_coefficients[0U] += sampled;
      if (sampled_coefficients[0U] >=
          LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        done = true;
      }
    }
  }
  return done;
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 randomness,
    size_t *sampled, Eurydice_arr_d0 *out) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      return libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_64(
          randomness, sampled, out);
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  return libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_64(
      randomness, sampled, out);
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.from_i32_array_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_polynomial_from_i32_array_ff_64(
    Eurydice_dst_ref_shared_83 array, Eurydice_arr_60 *result) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_DSA_SIMD_TRAITS_SIMD_UNITS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_from_coefficient_array_a2(
        Eurydice_slice_subslice_shared_47(
            array,
            (core_ops_range_Range_87{
                i0 * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_four_error_ring_elements
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_four_error_ring_elements_fc(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 seed,
    uint16_t start_index, Eurydice_dst_ref_mut_ba re) {
  Eurydice_arr_91 seed0 =
      libcrux_ml_dsa_sample_add_error_domain_separator(seed, start_index);
  Eurydice_arr_91 seed1 = libcrux_ml_dsa_sample_add_error_domain_separator(
      seed, (uint32_t)start_index + 1U);
  Eurydice_arr_91 seed2 = libcrux_ml_dsa_sample_add_error_domain_separator(
      seed, (uint32_t)start_index + 2U);
  Eurydice_arr_91 seed3 = libcrux_ml_dsa_sample_add_error_domain_separator(
      seed, (uint32_t)start_index + 3U);
  Eurydice_arr_c4 state =
      libcrux_ml_dsa_hash_functions_simd256_init_absorb_x4_ad(
          Eurydice_array_to_slice_shared_f1(&seed0),
          Eurydice_array_to_slice_shared_f1(&seed1),
          Eurydice_array_to_slice_shared_f1(&seed2),
          Eurydice_array_to_slice_shared_f1(&seed3));
  Eurydice_arr_ff_x4 randomnesses0 =
      libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_x4_ad(&state);
  Eurydice_arr_93 out = {{{{0U}}, {{0U}}, {{0U}}, {{0U}}}};
  size_t sampled0 = (size_t)0U;
  size_t sampled1 = (size_t)0U;
  size_t sampled2 = (size_t)0U;
  size_t sampled3 = (size_t)0U;
  libcrux_ml_dsa_constants_Eta uu____0 = eta;
  bool done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
      uu____0, Eurydice_array_to_slice_shared_58(&randomnesses0.fst), &sampled0,
      out.data);
  libcrux_ml_dsa_constants_Eta uu____1 = eta;
  bool done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
      uu____1, Eurydice_array_to_slice_shared_58(&randomnesses0.snd), &sampled1,
      &out.data[1U]);
  libcrux_ml_dsa_constants_Eta uu____2 = eta;
  bool done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
      uu____2, Eurydice_array_to_slice_shared_58(&randomnesses0.thd), &sampled2,
      &out.data[2U]);
  libcrux_ml_dsa_constants_Eta uu____3 = eta;
  bool done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
      uu____3, Eurydice_array_to_slice_shared_58(&randomnesses0.f3), &sampled3,
      &out.data[3U]);
  while (true) {
    if (done0) {
      if (done1) {
        if (done2) {
          if (done3) {
            break;
          } else {
            Eurydice_arr_ff_x4 randomnesses =
                libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4_ad(
                    &state);
            if (!done0) {
              libcrux_ml_dsa_constants_Eta uu____4 = eta;
              done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                  uu____4, Eurydice_array_to_slice_shared_58(&randomnesses.fst),
                  &sampled0, out.data);
            }
            if (!done1) {
              libcrux_ml_dsa_constants_Eta uu____5 = eta;
              done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                  uu____5, Eurydice_array_to_slice_shared_58(&randomnesses.snd),
                  &sampled1, &out.data[1U]);
            }
            if (!done2) {
              libcrux_ml_dsa_constants_Eta uu____6 = eta;
              done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                  uu____6, Eurydice_array_to_slice_shared_58(&randomnesses.thd),
                  &sampled2, &out.data[2U]);
            }
            if (!done3) {
              libcrux_ml_dsa_constants_Eta uu____7 = eta;
              done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                  uu____7, Eurydice_array_to_slice_shared_58(&randomnesses.f3),
                  &sampled3, &out.data[3U]);
            }
          }
        } else {
          Eurydice_arr_ff_x4 randomnesses =
              libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4_ad(
                  &state);
          if (!done0) {
            libcrux_ml_dsa_constants_Eta uu____8 = eta;
            done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                uu____8, Eurydice_array_to_slice_shared_58(&randomnesses.fst),
                &sampled0, out.data);
          }
          if (!done1) {
            libcrux_ml_dsa_constants_Eta uu____9 = eta;
            done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                uu____9, Eurydice_array_to_slice_shared_58(&randomnesses.snd),
                &sampled1, &out.data[1U]);
          }
          if (!done2) {
            libcrux_ml_dsa_constants_Eta uu____10 = eta;
            done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                uu____10, Eurydice_array_to_slice_shared_58(&randomnesses.thd),
                &sampled2, &out.data[2U]);
          }
          if (!done3) {
            libcrux_ml_dsa_constants_Eta uu____11 = eta;
            done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
                uu____11, Eurydice_array_to_slice_shared_58(&randomnesses.f3),
                &sampled3, &out.data[3U]);
          }
        }
      } else {
        Eurydice_arr_ff_x4 randomnesses =
            libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4_ad(
                &state);
        if (!done0) {
          libcrux_ml_dsa_constants_Eta uu____12 = eta;
          done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
              uu____12, Eurydice_array_to_slice_shared_58(&randomnesses.fst),
              &sampled0, out.data);
        }
        if (!done1) {
          libcrux_ml_dsa_constants_Eta uu____13 = eta;
          done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
              uu____13, Eurydice_array_to_slice_shared_58(&randomnesses.snd),
              &sampled1, &out.data[1U]);
        }
        if (!done2) {
          libcrux_ml_dsa_constants_Eta uu____14 = eta;
          done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
              uu____14, Eurydice_array_to_slice_shared_58(&randomnesses.thd),
              &sampled2, &out.data[2U]);
        }
        if (!done3) {
          libcrux_ml_dsa_constants_Eta uu____15 = eta;
          done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
              uu____15, Eurydice_array_to_slice_shared_58(&randomnesses.f3),
              &sampled3, &out.data[3U]);
        }
      }
    } else {
      Eurydice_arr_ff_x4 randomnesses =
          libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4_ad(
              &state);
      if (!done0) {
        libcrux_ml_dsa_constants_Eta uu____16 = eta;
        done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
            uu____16, Eurydice_array_to_slice_shared_58(&randomnesses.fst),
            &sampled0, out.data);
      }
      if (!done1) {
        libcrux_ml_dsa_constants_Eta uu____17 = eta;
        done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
            uu____17, Eurydice_array_to_slice_shared_58(&randomnesses.snd),
            &sampled1, &out.data[1U]);
      }
      if (!done2) {
        libcrux_ml_dsa_constants_Eta uu____18 = eta;
        done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
            uu____18, Eurydice_array_to_slice_shared_58(&randomnesses.thd),
            &sampled2, &out.data[2U]);
      }
      if (!done3) {
        libcrux_ml_dsa_constants_Eta uu____19 = eta;
        done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_64(
            uu____19, Eurydice_array_to_slice_shared_58(&randomnesses.f3),
            &sampled3, &out.data[3U]);
      }
    }
  }
  size_t max0 = (size_t)(uint32_t)start_index + (size_t)4U;
  size_t max;
  if (re.meta < max0) {
    max = re.meta;
  } else {
    max = max0;
  }
  for (size_t i = (size_t)(uint32_t)start_index; i < max; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_from_i32_array_ff_64(
        Eurydice_array_to_slice_shared_2c0(&out.data[i0 % (size_t)4U]),
        &re.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.sample_s1_and_s2
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_samplex4_sample_s1_and_s2_fc(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_ba s1_s2) {
  size_t len = s1_s2.meta;
  for (size_t i = (size_t)0U; i < len / (size_t)4U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_sample_sample_four_error_ring_elements_fc(
        eta, seed, 4U * (uint32_t)(uint16_t)i0, s1_s2);
  }
  size_t remainder = len % (size_t)4U;
  if (remainder != (size_t)0U) {
    libcrux_ml_dsa_sample_sample_four_error_ring_elements_fc(
        eta, seed, (uint16_t)(len - remainder), s1_s2);
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_mut_ba Eurydice_array_to_slice_mut_203(
    Eurydice_arr_4f *a) {
  Eurydice_dst_ref_mut_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b
with const generics
- $6size_t
*/
typedef struct Eurydice_arr_e2_s {
  Eurydice_arr_60 data[6U];
} Eurydice_arr_e2;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b
with const generics
- $30size_t
*/
typedef struct Eurydice_arr_9a_s {
  Eurydice_arr_60 data[30U];
} Eurydice_arr_9a;

/**
A monomorphic instance of
libcrux_ml_dsa.sample.rejection_sample_less_than_field_modulus with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
    Eurydice_borrow_slice_u8 randomness, size_t *sampled_coefficients,
    Eurydice_arr_d0 *out) {
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)24U; i++) {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8 random_bytes = Eurydice_slice_subslice_shared_c8(
        randomness,
        (core_ops_range_Range_87{_cloop_i * (size_t)24U,
                                 _cloop_i * (size_t)24U + (size_t)24U}));
    if (!done) {
      size_t sampled =
          libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_a2(
              random_bytes, Eurydice_array_to_subslice_from_mut_11(
                                out, sampled_coefficients[0U]));
      sampled_coefficients[0U] += sampled;
      if (sampled_coefficients[0U] >=
          LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        done = true;
      }
    }
  }
  return done;
}

/**
 Sample and write out up to four ring elements.

 If i <= `elements_requested`, a field element with domain separated
 seed according to the provided index is generated in
 `tmp_stack[i]`. After successful rejection sampling in
 `tmp_stack[i]`, the ring element is written to `matrix` at the
 provided index in `indices[i]`.
 `rand_stack` is a working buffer that holds initial Shake output.
*/
/**
A monomorphic instance of
libcrux_ml_dsa.sample.sample_up_to_four_ring_elements_flat with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_0a(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_ba matrix, Eurydice_arr_d1 *rand_stack0,
    Eurydice_arr_d1 *rand_stack1, Eurydice_arr_d1 *rand_stack2,
    Eurydice_arr_d1 *rand_stack3, Eurydice_dst_ref_mut_33 tmp_stack,
    size_t start_index, size_t elements_requested) {
  Eurydice_arr_31 seed0 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index, columns));
  Eurydice_arr_31 seed1 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index + (size_t)1U, columns));
  Eurydice_arr_31 seed2 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index + (size_t)2U, columns));
  Eurydice_arr_31 seed3 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index + (size_t)3U, columns));
  Eurydice_arr_c4 state = libcrux_ml_dsa_hash_functions_simd256_init_absorb_3b(
      Eurydice_array_to_slice_shared_e9(&seed0),
      Eurydice_array_to_slice_shared_e9(&seed1),
      Eurydice_array_to_slice_shared_e9(&seed2),
      Eurydice_array_to_slice_shared_e9(&seed3));
  libcrux_ml_dsa_hash_functions_simd256_squeeze_first_five_blocks_3b(
      &state, rand_stack0, rand_stack1, rand_stack2, rand_stack3);
  size_t sampled0 = (size_t)0U;
  size_t sampled1 = (size_t)0U;
  size_t sampled2 = (size_t)0U;
  size_t sampled3 = (size_t)0U;
  bool done0 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
          Eurydice_array_to_slice_shared_4c(rand_stack0), &sampled0,
          tmp_stack.ptr);
  bool done1 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
          Eurydice_array_to_slice_shared_4c(rand_stack1), &sampled1,
          &tmp_stack.ptr[1U]);
  bool done2 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
          Eurydice_array_to_slice_shared_4c(rand_stack2), &sampled2,
          &tmp_stack.ptr[2U]);
  bool done3 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
          Eurydice_array_to_slice_shared_4c(rand_stack3), &sampled3,
          &tmp_stack.ptr[3U]);
  while (true) {
    if (done0) {
      if (done1) {
        if (done2) {
          if (done3) {
            break;
          } else {
            Eurydice_arr_c5_x4 randomnesses =
                libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_3b(
                    &state);
            if (!done0) {
              done0 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                      Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
                      &sampled0, tmp_stack.ptr);
            }
            if (!done1) {
              done1 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                      Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
                      &sampled1, &tmp_stack.ptr[1U]);
            }
            if (!done2) {
              done2 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                      Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
                      &sampled2, &tmp_stack.ptr[2U]);
            }
            if (!done3) {
              done3 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                      Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
                      &sampled3, &tmp_stack.ptr[3U]);
            }
          }
        } else {
          Eurydice_arr_c5_x4 randomnesses =
              libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_3b(
                  &state);
          if (!done0) {
            done0 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                    Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
                    &sampled0, tmp_stack.ptr);
          }
          if (!done1) {
            done1 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                    Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
                    &sampled1, &tmp_stack.ptr[1U]);
          }
          if (!done2) {
            done2 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                    Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
                    &sampled2, &tmp_stack.ptr[2U]);
          }
          if (!done3) {
            done3 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                    Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
                    &sampled3, &tmp_stack.ptr[3U]);
          }
        }
      } else {
        Eurydice_arr_c5_x4 randomnesses =
            libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_3b(&state);
        if (!done0) {
          done0 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                  Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
                  &sampled0, tmp_stack.ptr);
        }
        if (!done1) {
          done1 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                  Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
                  &sampled1, &tmp_stack.ptr[1U]);
        }
        if (!done2) {
          done2 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                  Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
                  &sampled2, &tmp_stack.ptr[2U]);
        }
        if (!done3) {
          done3 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                  Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
                  &sampled3, &tmp_stack.ptr[3U]);
        }
      }
    } else {
      Eurydice_arr_c5_x4 randomnesses =
          libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_3b(&state);
      if (!done0) {
        done0 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                Eurydice_array_to_slice_shared_2c(&randomnesses.fst), &sampled0,
                tmp_stack.ptr);
      }
      if (!done1) {
        done1 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                Eurydice_array_to_slice_shared_2c(&randomnesses.snd), &sampled1,
                &tmp_stack.ptr[1U]);
      }
      if (!done2) {
        done2 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                Eurydice_array_to_slice_shared_2c(&randomnesses.thd), &sampled2,
                &tmp_stack.ptr[2U]);
      }
      if (!done3) {
        done3 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_64(
                Eurydice_array_to_slice_shared_2c(&randomnesses.f3), &sampled3,
                &tmp_stack.ptr[3U]);
      }
    }
  }
  for (size_t i = (size_t)0U; i < elements_requested; i++) {
    size_t k = i;
    libcrux_ml_dsa_polynomial_from_i32_array_ff_64(
        Eurydice_array_to_slice_shared_2c0(&tmp_stack.ptr[k]),
        &matrix.ptr[start_index + k]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.matrix_flat
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_samplex4_matrix_flat_0a(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_ba matrix) {
  Eurydice_arr_d1 rand_stack0 = {{0U}};
  Eurydice_arr_d1 rand_stack1 = {{0U}};
  Eurydice_arr_d1 rand_stack2 = {{0U}};
  Eurydice_arr_d1 rand_stack3 = {{0U}};
  Eurydice_arr_93 tmp_stack = {{{{0U}}, {{0U}}, {{0U}}, {{0U}}}};
  for (size_t i = (size_t)0U; i < matrix.meta / (size_t)4U + (size_t)1U; i++) {
    size_t start_index = i;
    size_t start_index0 = start_index * (size_t)4U;
    if (start_index0 >= matrix.meta) {
      break;
    }
    size_t elements_requested;
    if (start_index0 + (size_t)4U <= matrix.meta) {
      elements_requested = (size_t)4U;
    } else {
      elements_requested = matrix.meta - start_index0;
    }
    libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_0a(
        columns, seed, matrix, &rand_stack0, &rand_stack1, &rand_stack2,
        &rand_stack3, Eurydice_array_to_slice_mut_7e(&tmp_stack), start_index0,
        elements_requested);
  }
}

/**
This function found in impl {libcrux_ml_dsa::samplex4::X4Sampler for
libcrux_ml_dsa::samplex4::avx2::AVX2Sampler}
*/
/**
A monomorphic instance of libcrux_ml_dsa.samplex4.avx2.matrix_flat.inner_e8
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_samplex4_avx2_matrix_flat_inner_e8_64(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_ba matrix) {
  libcrux_ml_dsa_samplex4_matrix_flat_0a(columns, seed, matrix);
}

/**
This function found in impl {libcrux_ml_dsa::samplex4::X4Sampler for
libcrux_ml_dsa::samplex4::avx2::AVX2Sampler}
*/
/**
A monomorphic instance of libcrux_ml_dsa.samplex4.avx2.matrix_flat_e8
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_samplex4_avx2_matrix_flat_e8_64(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_ba matrix) {
  libcrux_ml_dsa_samplex4_avx2_matrix_flat_inner_e8_64(columns, seed, matrix);
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 30
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_mut_ba Eurydice_array_to_slice_mut_204(
    Eurydice_arr_9a *a) {
  Eurydice_dst_ref_mut_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)30U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_77_s {
  Eurydice_arr_60 data[5U];
} Eurydice_arr_77;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_mut_ba Eurydice_array_to_slice_mut_205(
    Eurydice_arr_77 *a) {
  Eurydice_dst_ref_mut_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)5U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256, core_ops_range_Range size_t,
Eurydice_derefed_slice libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_ba Eurydice_array_to_subslice_shared_250(
    const Eurydice_arr_4f *a, core_ops_range_Range_87 r) {
  return (Eurydice_dst_ref_shared_ba{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_ntt_64(Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_ntt_a2(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt_multiply_montgomery
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_ntt_multiply_montgomery_64(
    Eurydice_arr_60 *lhs, const Eurydice_arr_60 *rhs) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_montgomery_multiply_a2(&lhs->data[i0],
                                                    &rhs->data[i0]);
  }
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.add_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_polynomial_add_ff_64(
    Eurydice_arr_60 *self, const Eurydice_arr_60 *rhs) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_add_a2(&self->data[i0], &rhs->data[i0]);
  }
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.barrett_reduce_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_polynomial_barrett_reduce_ff_64(
    Eurydice_arr_60 *self) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_barrett_reduce_simd_unit_a2(&self->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.invert_ntt_montgomery
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_invert_ntt_montgomery_64(
    Eurydice_arr_60 *re) {
  libcrux_ml_dsa_simd_avx2_invert_ntt_montgomery_a2(re);
}

/**
 Compute InvertNTT(Â ◦ ŝ₁) + s₂
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_as1_plus_s2
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_compute_as1_plus_s2_64(
    size_t rows_in_a, size_t columns_in_a, Eurydice_dst_ref_mut_ba a_as_ntt,
    Eurydice_dst_ref_shared_ba s1_ntt, Eurydice_dst_ref_shared_ba s1_s2,
    Eurydice_dst_ref_mut_ba result) {
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < columns_in_a; i++) {
      size_t j = i;
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_64(
          &a_as_ntt.ptr[i1 * columns_in_a + j], &s1_ntt.ptr[j]);
      libcrux_ml_dsa_polynomial_add_ff_64(&result.ptr[i1],
                                          &a_as_ntt.ptr[i1 * columns_in_a + j]);
    }
  }
  for (size_t i = (size_t)0U; i < result.meta; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_barrett_reduce_ff_64(&result.ptr[i0]);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_64(&result.ptr[i0]);
    libcrux_ml_dsa_polynomial_add_ff_64(&result.ptr[i0],
                                        &s1_s2.ptr[columns_in_a + i0]);
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_ba Eurydice_array_to_slice_shared_203(
    const Eurydice_arr_77 *a) {
  Eurydice_dst_ref_shared_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)5U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_ba Eurydice_array_to_slice_shared_204(
    const Eurydice_arr_4f *a) {
  Eurydice_dst_ref_shared_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 6
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_mut_ba Eurydice_array_to_slice_mut_206(
    Eurydice_arr_e2 *a) {
  Eurydice_dst_ref_mut_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256[[$6size_t]]

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_secrets_mem_requests_ct_declassify_b20(
    const Eurydice_arr_e2 *val) {}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.power2round_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_power2round_vector_64(
    Eurydice_dst_ref_mut_ba t, Eurydice_dst_ref_mut_ba t1) {
  for (size_t i0 = (size_t)0U; i0 < t.meta; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
      size_t j = i;
      libcrux_ml_dsa_simd_avx2_power2round_a2(&t.ptr[i1].data[j],
                                              &t1.ptr[i1].data[j]);
    }
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_t1_serialize_64(
    const Eurydice_arr_60 *re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    const __m256i *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_avx2_t1_serialize_a2(
        simd_unit,
        Eurydice_slice_subslice_mut_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 *
                    LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT})));
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.verification_key.generate_serialized with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_verification_key_generate_serialized_64(
    Eurydice_borrow_slice_u8 seed, Eurydice_dst_ref_shared_ba t1,
    Eurydice_mut_borrow_slice_u8 verification_key_serialized) {
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_c8(
          verification_key_serialized,
          (core_ops_range_Range_87{(size_t)0U,
                                   LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE})),
      seed, uint8_t);
  for (size_t i = (size_t)0U; i < t1.meta; i++) {
    size_t i0 = i;
    const Eurydice_arr_60 *ring_element = &t1.ptr[i0];
    size_t offset = LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE +
                    i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE;
    libcrux_ml_dsa_encoding_t1_serialize_64(
        ring_element,
        Eurydice_slice_subslice_mut_c8(
            verification_key_serialized,
            (core_ops_range_Range_87{
                offset,
                offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE})));
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 6
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_ba Eurydice_array_to_slice_shared_205(
    const Eurydice_arr_e2 *a) {
  Eurydice_dst_ref_shared_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256
with const generics
- OUTPUT_LENGTH= 64
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_simd256_shake256_c9(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_c7 *out) {
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_17(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_8c
with const generics
- OUTPUT_LENGTH= 64
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_8c_c9(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_c7 *out) {
  libcrux_ml_dsa_hash_functions_simd256_shake256_c9(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_error_serialize_64(
    libcrux_ml_dsa_constants_Eta eta, const Eurydice_arr_60 *re,
    Eurydice_mut_borrow_slice_u8 serialized) {
  size_t output_bytes_per_simd_unit =
      libcrux_ml_dsa_encoding_error_chunk_size(eta);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    const __m256i *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_avx2_error_serialize_a2(
        eta, simd_unit,
        Eurydice_slice_subslice_mut_c8(
            serialized, (core_ops_range_Range_87{
                            i0 * output_bytes_per_simd_unit,
                            (i0 + (size_t)1U) * output_bytes_per_simd_unit})));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_t0_serialize_64(
    const Eurydice_arr_60 *re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    const __m256i *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_avx2_t0_serialize_a2(
        simd_unit,
        Eurydice_slice_subslice_mut_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT})));
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.signing_key.generate_serialized with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_signing_key_generate_serialized_18(
    libcrux_ml_dsa_constants_Eta eta, size_t error_ring_element_size,
    Eurydice_borrow_slice_u8 seed_matrix, Eurydice_borrow_slice_u8 seed_signing,
    Eurydice_borrow_slice_u8 verification_key, Eurydice_dst_ref_shared_ba s1_2,
    Eurydice_dst_ref_shared_ba t0,
    Eurydice_mut_borrow_slice_u8 signing_key_serialized) {
  size_t offset = (size_t)0U;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_c8(
          signing_key_serialized,
          (core_ops_range_Range_87{
              offset, offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE})),
      seed_matrix, uint8_t);
  offset += LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_c8(
          signing_key_serialized,
          (core_ops_range_Range_87{
              offset,
              offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE})),
      seed_signing, uint8_t);
  offset += LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE;
  Eurydice_arr_c7 verification_key_hash = {{0U}};
  libcrux_ml_dsa_hash_functions_simd256_shake256_8c_c9(verification_key,
                                                       &verification_key_hash);
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_c8(
          signing_key_serialized,
          (core_ops_range_Range_87{
              offset,
              offset +
                  LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH})),
      Eurydice_array_to_slice_shared_17(&verification_key_hash), uint8_t);
  offset += LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH;
  for (size_t i = (size_t)0U; i < s1_2.meta; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_error_serialize_64(
        eta, &s1_2.ptr[i0],
        Eurydice_slice_subslice_mut_c8(
            signing_key_serialized,
            (core_ops_range_Range_87{offset,
                                     offset + error_ring_element_size})));
    offset += error_ring_element_size;
  }
  for (size_t i = (size_t)0U; i < t0.meta; i++) {
    size_t _cloop_j = i;
    const Eurydice_arr_60 *ring_element = &t0.ptr[_cloop_j];
    libcrux_ml_dsa_encoding_t0_serialize_64(
        ring_element,
        Eurydice_slice_subslice_mut_c8(
            signing_key_serialized,
            (core_ops_range_Range_87{
                offset,
                offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE})));
    offset += LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.generate_key_pair with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_07(
    Eurydice_arr_ec randomness, Eurydice_mut_borrow_slice_u8 signing_key,
    Eurydice_mut_borrow_slice_u8 verification_key) {
  Eurydice_arr_89 seed_expanded0 = {{0U}};
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d shake =
      libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(
      &shake, Eurydice_array_to_slice_shared_01(&randomness));
  /* original Rust expression is not an lvalue in C */
  Eurydice_array_u8x2 lvalue = {
      {(uint8_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
       (uint8_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A}};
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
      &shake, Eurydice_array_to_slice_shared_82(&lvalue));
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(
      &shake, Eurydice_array_to_slice_mut_78(&seed_expanded0));
  Eurydice_borrow_slice_u8_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_78(&seed_expanded0),
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_expanded = uu____0.snd;
  Eurydice_borrow_slice_u8_x2 uu____1 = Eurydice_slice_split_at(
      seed_expanded, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_ERROR_VECTORS_SIZE,
      uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_error_vectors = uu____1.fst;
  Eurydice_borrow_slice_u8 seed_for_signing = uu____1.snd;
  Eurydice_arr_4f s1_s2;
  Eurydice_arr_60 repeat_expression0[11U];
  for (size_t i = (size_t)0U; i < (size_t)11U; i++) {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(s1_s2.data, repeat_expression0, (size_t)11U * sizeof(Eurydice_arr_60));
  libcrux_ml_dsa_samplex4_sample_s1_and_s2_fc(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA, seed_for_error_vectors,
      Eurydice_array_to_slice_mut_203(&s1_s2));
  Eurydice_arr_e2 t0;
  Eurydice_arr_60 repeat_expression1[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(t0.data, repeat_expression1, (size_t)6U * sizeof(Eurydice_arr_60));
  Eurydice_arr_9a a_as_ntt;
  Eurydice_arr_60 repeat_expression2[30U];
  for (size_t i = (size_t)0U; i < (size_t)30U; i++) {
    repeat_expression2[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(a_as_ntt.data, repeat_expression2,
         (size_t)30U * sizeof(Eurydice_arr_60));
  libcrux_ml_dsa_samplex4_avx2_matrix_flat_e8_64(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, seed_for_a,
      Eurydice_array_to_slice_mut_204(&a_as_ntt));
  Eurydice_arr_77 s1_ntt;
  Eurydice_arr_60 repeat_expression3[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    repeat_expression3[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(s1_ntt.data, repeat_expression3, (size_t)5U * sizeof(Eurydice_arr_60));
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_205(&s1_ntt),
      Eurydice_array_to_subslice_shared_250(
          &s1_s2,
          (core_ops_range_Range_87{
              (size_t)0U, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A})),
      Eurydice_arr_60);
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_ntt_64(&s1_ntt.data[i0]);
  }
  libcrux_ml_dsa_matrix_compute_as1_plus_s2_64(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      Eurydice_array_to_slice_mut_204(&a_as_ntt),
      Eurydice_array_to_slice_shared_203(&s1_ntt),
      Eurydice_array_to_slice_shared_204(&s1_s2),
      Eurydice_array_to_slice_mut_206(&t0));
  Eurydice_arr_e2 t1;
  Eurydice_arr_60 repeat_expression[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(t1.data, repeat_expression, (size_t)6U * sizeof(Eurydice_arr_60));
  libcrux_ml_dsa_arithmetic_power2round_vector_64(
      Eurydice_array_to_slice_mut_206(&t0),
      Eurydice_array_to_slice_mut_206(&t1));
  libcrux_ml_dsa_encoding_verification_key_generate_serialized_64(
      seed_for_a, Eurydice_array_to_slice_shared_205(&t1), verification_key);
  libcrux_ml_dsa_encoding_signing_key_generate_serialized_18(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
      seed_for_a, seed_for_signing,
      (Eurydice_borrow_slice_u8{verification_key.ptr, verification_key.meta}),
      Eurydice_array_to_slice_shared_204(&s1_s2),
      Eurydice_array_to_slice_shared_205(&t0), signing_key);
}

/**
 Key Generation.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair__inner(
    Eurydice_arr_ec randomness, Eurydice_mut_borrow_slice_u8 signing_key,
    Eurydice_mut_borrow_slice_u8 verification_key) {
  libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_07(
      randomness, signing_key, verification_key);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair(
    Eurydice_arr_ec randomness, Eurydice_mut_borrow_slice_u8 signing_key,
    Eurydice_mut_borrow_slice_u8 verification_key) {
  libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair__inner(
      randomness, signing_key, verification_key);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_error_deserialize_64(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized,
    Eurydice_arr_60 *result) {
  size_t chunk_size = libcrux_ml_dsa_encoding_error_chunk_size(eta);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_error_deserialize_a2(
        eta,
        Eurydice_slice_subslice_shared_c8(
            serialized, (core_ops_range_Range_87{
                            i0 * chunk_size, (i0 + (size_t)1U) * chunk_size})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.error.deserialize_to_vector_then_ntt with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_64(
    libcrux_ml_dsa_constants_Eta eta, size_t ring_element_size,
    Eurydice_borrow_slice_u8 serialized,
    Eurydice_dst_ref_mut_ba ring_elements) {
  for (size_t i = (size_t)0U; i < serialized.meta / ring_element_size; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_c8(
        serialized,
        (core_ops_range_Range_87{i0 * ring_element_size,
                                 i0 * ring_element_size + ring_element_size}));
    libcrux_ml_dsa_encoding_error_deserialize_64(eta, bytes,
                                                 &ring_elements.ptr[i0]);
    libcrux_ml_dsa_ntt_ntt_64(&ring_elements.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_t0_deserialize_64(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_60 *result) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_t0_deserialize_a2(
        Eurydice_slice_subslice_shared_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.t0.deserialize_to_vector_then_ntt with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_64(
    Eurydice_borrow_slice_u8 serialized,
    Eurydice_dst_ref_mut_ba ring_elements) {
  for (size_t i = (size_t)0U;
       i < serialized.meta / LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_c8(
        serialized,
        (core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE,
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE +
                LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE}));
    libcrux_ml_dsa_encoding_t0_deserialize_64(bytes, &ring_elements.ptr[i0]);
    libcrux_ml_dsa_ntt_ntt_64(&ring_elements.ptr[i0]);
  }
}

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_77

*/
typedef struct core_option_Option_61_s {
  core_option_Option_87_tags tag;
  Eurydice_arr_77 f0;
} core_option_Option_61;

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4
with const generics
- OUT_LEN= 576
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_5a(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3,
    Eurydice_arr_220 *out0, Eurydice_arr_220 *out1, Eurydice_arr_220 *out2,
    Eurydice_arr_220 *out3) {
  libcrux_sha3_avx2_x4_shake256(input0, input1, input2, input3,
                                Eurydice_array_to_slice_mut_8a(out0),
                                Eurydice_array_to_slice_mut_8a(out1),
                                Eurydice_array_to_slice_mut_8a(out2),
                                Eurydice_array_to_slice_mut_8a(out3));
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4_ad
with const generics
- OUT_LEN= 576
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_ad_5a(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3,
    Eurydice_arr_220 *out0, Eurydice_arr_220 *out1, Eurydice_arr_220 *out2,
    Eurydice_arr_220 *out3) {
  libcrux_ml_dsa_hash_functions_simd256_shake256_x4_5a(
      input0, input1, input2, input3, out0, out1, out2, out3);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_gamma1_deserialize_64(
    size_t gamma1_exponent, Eurydice_borrow_slice_u8 serialized,
    Eurydice_arr_60 *result) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_gamma1_deserialize_a2(
        Eurydice_slice_subslice_shared_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 * (gamma1_exponent + (size_t)1U),
                (i0 + (size_t)1U) * (gamma1_exponent + (size_t)1U)})),
        &result->data[i0], gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4
with const generics
- OUT_LEN= 640
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_0e(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3,
    Eurydice_arr_20 *out0, Eurydice_arr_20 *out1, Eurydice_arr_20 *out2,
    Eurydice_arr_20 *out3) {
  libcrux_sha3_avx2_x4_shake256(input0, input1, input2, input3,
                                Eurydice_array_to_slice_mut_4f(out0),
                                Eurydice_array_to_slice_mut_4f(out1),
                                Eurydice_array_to_slice_mut_4f(out2),
                                Eurydice_array_to_slice_mut_4f(out3));
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4_ad
with const generics
- OUT_LEN= 640
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_ad_0e(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3,
    Eurydice_arr_20 *out0, Eurydice_arr_20 *out1, Eurydice_arr_20 *out2,
    Eurydice_arr_20 *out3) {
  libcrux_ml_dsa_hash_functions_simd256_shake256_x4_0e(
      input0, input1, input2, input3, out0, out1, out2, out3);
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256
with const generics
- OUTPUT_LENGTH= 640
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_simd256_shake256_0e(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_20 *out) {
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_4f(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_8c
with const generics
- OUTPUT_LENGTH= 640
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_8c_0e(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_20 *out) {
  libcrux_ml_dsa_hash_functions_simd256_shake256_0e(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256
with const generics
- OUTPUT_LENGTH= 576
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_simd256_shake256_5a(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_220 *out) {
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_8a(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_8c
with const generics
- OUTPUT_LENGTH= 576
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_simd256_shake256_8c_5a(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_220 *out) {
  libcrux_ml_dsa_hash_functions_simd256_shake256_5a(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_ring_element
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_sample_sample_mask_ring_element_18(
    const Eurydice_arr_91 *seed, Eurydice_arr_60 *result,
    size_t gamma1_exponent) {
  switch (gamma1_exponent) {
    case 17U: {
      break;
    }
    case 19U: {
      Eurydice_arr_20 out = {{0U}};
      libcrux_ml_dsa_hash_functions_simd256_shake256_8c_0e(
          Eurydice_array_to_slice_shared_f1(seed), &out);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_4f(&out), result);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  Eurydice_arr_220 out = {{0U}};
  libcrux_ml_dsa_hash_functions_simd256_shake256_8c_5a(
      Eurydice_array_to_slice_shared_f1(seed), &out);
  libcrux_ml_dsa_encoding_gamma1_deserialize_64(
      gamma1_exponent, Eurydice_array_to_slice_shared_8a(&out), result);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_sample_sample_mask_vector_f4(
    size_t dimension, size_t gamma1_exponent, const Eurydice_arr_c7 *seed,
    uint16_t *domain_separator, Eurydice_dst_ref_mut_ba mask) {
  Eurydice_arr_91 seed0 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_17(seed), domain_separator[0U]);
  Eurydice_arr_91 seed1 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_17(seed),
      (uint32_t)domain_separator[0U] + 1U);
  Eurydice_arr_91 seed2 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_17(seed),
      (uint32_t)domain_separator[0U] + 2U);
  Eurydice_arr_91 seed3 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_17(seed),
      (uint32_t)domain_separator[0U] + 3U);
  domain_separator[0U] = (uint32_t)domain_separator[0U] + 4U;
  switch (gamma1_exponent) {
    case 17U: {
      Eurydice_arr_220 out0 = {{0U}};
      Eurydice_arr_220 out1 = {{0U}};
      Eurydice_arr_220 out2 = {{0U}};
      Eurydice_arr_220 out3 = {{0U}};
      libcrux_ml_dsa_hash_functions_simd256_shake256_x4_ad_5a(
          Eurydice_array_to_slice_shared_f1(&seed0),
          Eurydice_array_to_slice_shared_f1(&seed1),
          Eurydice_array_to_slice_shared_f1(&seed2),
          Eurydice_array_to_slice_shared_f1(&seed3), &out0, &out1, &out2,
          &out3);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_8a(&out0), mask.ptr);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_8a(&out1),
          &mask.ptr[1U]);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_8a(&out2),
          &mask.ptr[2U]);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_8a(&out3),
          &mask.ptr[3U]);
      break;
    }
    case 19U: {
      Eurydice_arr_20 out0 = {{0U}};
      Eurydice_arr_20 out1 = {{0U}};
      Eurydice_arr_20 out2 = {{0U}};
      Eurydice_arr_20 out3 = {{0U}};
      libcrux_ml_dsa_hash_functions_simd256_shake256_x4_ad_0e(
          Eurydice_array_to_slice_shared_f1(&seed0),
          Eurydice_array_to_slice_shared_f1(&seed1),
          Eurydice_array_to_slice_shared_f1(&seed2),
          Eurydice_array_to_slice_shared_f1(&seed3), &out0, &out1, &out2,
          &out3);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_4f(&out0), mask.ptr);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_4f(&out1),
          &mask.ptr[1U]);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_4f(&out2),
          &mask.ptr[2U]);
      libcrux_ml_dsa_encoding_gamma1_deserialize_64(
          gamma1_exponent, Eurydice_array_to_slice_shared_4f(&out3),
          &mask.ptr[3U]);
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  for (size_t i = (size_t)4U; i < dimension; i++) {
    size_t i0 = i;
    Eurydice_arr_91 seed4 = libcrux_ml_dsa_sample_add_error_domain_separator(
        Eurydice_array_to_slice_shared_17(seed), domain_separator[0U]);
    domain_separator[0U] = (uint32_t)domain_separator[0U] + 1U;
    libcrux_ml_dsa_sample_sample_mask_ring_element_18(&seed4, &mask.ptr[i0],
                                                      gamma1_exponent);
  }
}

/**
 Compute InvertNTT(Â ◦ ŷ)
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_matrix_x_mask
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_compute_matrix_x_mask_64(
    size_t rows_in_a, size_t columns_in_a, Eurydice_dst_ref_shared_ba matrix,
    Eurydice_dst_ref_shared_ba mask, Eurydice_dst_ref_mut_ba result) {
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < columns_in_a; i++) {
      size_t j = i;
      Eurydice_arr_60 product = mask.ptr[j];
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_64(
          &product, &matrix.ptr[i1 * columns_in_a + j]);
      libcrux_ml_dsa_polynomial_add_ff_64(&result.ptr[i1], &product);
    }
    libcrux_ml_dsa_polynomial_barrett_reduce_ff_64(&result.ptr[i1]);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_64(&result.ptr[i1]);
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_avx2_vector_type_Vec256 with const generics
- N= 30
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_ba Eurydice_array_to_slice_shared_206(
    const Eurydice_arr_9a *a) {
  Eurydice_dst_ref_shared_ba lit;
  lit.ptr = a->data;
  lit.meta = (size_t)30U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.decompose_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_decompose_vector_64(
    size_t dimension, int32_t gamma2, Eurydice_dst_ref_shared_ba t,
    Eurydice_dst_ref_mut_ba low, Eurydice_dst_ref_mut_ba high) {
  for (size_t i0 = (size_t)0U; i0 < dimension; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
      size_t j = i;
      libcrux_ml_dsa_simd_avx2_decompose_a2(gamma2, &t.ptr[i1].data[j],
                                            &low.ptr[i1].data[j],
                                            &high.ptr[i1].data[j]);
    }
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_commitment_serialize_64(
    const Eurydice_arr_60 *re, Eurydice_mut_borrow_slice_u8 serialized) {
  size_t output_bytes_per_simd_unit =
      serialized.meta / ((size_t)8U * (size_t)4U);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    const __m256i *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_avx2_commitment_serialize_a2(
        simd_unit,
        Eurydice_slice_subslice_mut_c8(
            serialized, (core_ops_range_Range_87{
                            i0 * output_bytes_per_simd_unit,
                            (i0 + (size_t)1U) * output_bytes_per_simd_unit})));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_commitment_serialize_vector_64(
    size_t ring_element_size, Eurydice_dst_ref_shared_ba vector,
    Eurydice_mut_borrow_slice_u8 serialized) {
  size_t offset = (size_t)0U;
  for (size_t i = (size_t)0U; i < vector.meta; i++) {
    size_t _cloop_j = i;
    const Eurydice_arr_60 *ring_element = &vector.ptr[_cloop_j];
    libcrux_ml_dsa_encoding_commitment_serialize_64(
        ring_element,
        Eurydice_slice_subslice_mut_c8(
            serialized,
            (core_ops_range_Range_87{offset, offset + ring_element_size})));
    offset += ring_element_size;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_challenge_ring_element
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_hash_functions_simd256_Shake256 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_challenge_ring_element_18(
    Eurydice_borrow_slice_u8 seed, size_t number_of_ones, Eurydice_arr_60 *re) {
  Eurydice_arr_7c state =
      libcrux_ml_dsa_hash_functions_simd256_init_absorb_final_8c(seed);
  Eurydice_arr_ff randomness0 =
      libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_8c(&state);
  Eurydice_array_u8x8 arr;
  memcpy(arr.data,
         Eurydice_array_to_subslice_shared_d40(
             &randomness0, (core_ops_range_Range_87{(size_t)0U, (size_t)8U}))
             .ptr,
         (size_t)8U * sizeof(uint8_t));
  uint64_t signs = core_num__u64__from_le_bytes(
      core_result_unwrap_26_e0(core_result_Result_8e_s(
          core_result_Ok, &core_result_Result_8e_s::U::case_Ok, arr)));
  Eurydice_arr_6c result = {{0U}};
  size_t out_index = (size_t)256U - number_of_ones;
  bool done = libcrux_ml_dsa_sample_inside_out_shuffle(
      Eurydice_array_to_subslice_from_shared_5f(&randomness0, (size_t)8U),
      &out_index, &signs, &result);
  while (true) {
    if (done) {
      break;
    } else {
      Eurydice_arr_ff randomness =
          libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_8c(&state);
      done = libcrux_ml_dsa_sample_inside_out_shuffle(
          Eurydice_array_to_slice_shared_58(&randomness), &out_index, &signs,
          &result);
    }
  }
  libcrux_ml_dsa_polynomial_from_i32_array_ff_64(
      Eurydice_array_to_slice_shared_af(&result), re);
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.vector_times_ring_element
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_vector_times_ring_element_64(
    Eurydice_dst_ref_mut_ba vector, const Eurydice_arr_60 *ring_element) {
  for (size_t i = (size_t)0U; i < vector.meta; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_ntt_multiply_montgomery_64(&vector.ptr[i0],
                                                  ring_element);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_64(&vector.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.add_vectors
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_add_vectors_64(
    size_t dimension, Eurydice_dst_ref_mut_ba lhs,
    Eurydice_dst_ref_shared_ba rhs) {
  for (size_t i = (size_t)0U; i < dimension; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_add_ff_64(&lhs.ptr[i0], &rhs.ptr[i0]);
  }
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.subtract_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_polynomial_subtract_ff_64(
    Eurydice_arr_60 *self, const Eurydice_arr_60 *rhs) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_subtract_a2(&self->data[i0], &rhs->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.subtract_vectors
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_subtract_vectors_64(
    size_t dimension, Eurydice_dst_ref_mut_ba lhs,
    Eurydice_dst_ref_shared_ba rhs) {
  for (size_t i = (size_t)0U; i < dimension; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_subtract_ff_64(&lhs.ptr[i0], &rhs.ptr[i0]);
  }
}

/**
 CAUTION: This function must only be called with inputs for
 which it is safe to leak the index of a violating coefficient.

 For all norm checks during ML-DSA signature generation it is
 safe to leak the index of a violating coefficient.
*/
/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.infinity_norm_exceeds_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_dsa_polynomial_infinity_norm_exceeds_ff_64(
    const Eurydice_arr_60 *self, int32_t bound) {
  bool result = false;
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    bool coeff_exceeds = libcrux_ml_dsa_simd_avx2_infinity_norm_exceeds_a2(
        &self->data[i0], bound);
    bool uu____0;
    if (result) {
      uu____0 = true;
    } else {
      uu____0 = coeff_exceeds;
    }
    result = uu____0;
  }
  return result;
}

/**
 CAUTION: This function must only be called with inputs for
 which it is safe to leak the index of a violating coefficient.

 For all norm checks during ML-DSA signature generation it is
 safe to leak the index of a violating coefficient.
*/
/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.vector_infinity_norm_exceeds
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_64(
    Eurydice_dst_ref_shared_ba vector, int32_t bound) {
  bool result = false;
  for (size_t i = (size_t)0U; i < vector.meta; i++) {
    size_t i0 = i;
    bool uu____0;
    if (result) {
      uu____0 = true;
    } else {
      uu____0 = libcrux_ml_dsa_polynomial_infinity_norm_exceeds_ff_64(
          &vector.ptr[i0], bound);
    }
    result = uu____0;
  }
  return result;
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.to_i32_array_ff
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_6c libcrux_ml_dsa_polynomial_to_i32_array_ff_64(
    const Eurydice_arr_60 *self) {
  Eurydice_arr_6c result = {{0U}};
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_to_coefficient_array_a2(
        &self->data[i0],
        Eurydice_array_to_subslice_mut_44(
            &result,
            (core_ops_range_Range_87{
                i0 * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT})));
  }
  return result;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.make_hint
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t libcrux_ml_dsa_arithmetic_make_hint_64(
    Eurydice_dst_ref_shared_ba low, Eurydice_dst_ref_shared_ba high,
    int32_t gamma2, Eurydice_dst_ref_mut_20 hint) {
  size_t true_hints = (size_t)0U;
  Eurydice_arr_60 hint_simd = libcrux_ml_dsa_polynomial_zero_ff_64();
  for (size_t i0 = (size_t)0U; i0 < low.meta; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
      size_t j = i;
      size_t one_hints_count = libcrux_ml_dsa_simd_avx2_compute_hint_a2(
          &low.ptr[i1].data[j], &high.ptr[i1].data[j], gamma2,
          &hint_simd.data[j]);
      true_hints += one_hints_count;
    }
    Eurydice_arr_6c uu____0 =
        libcrux_ml_dsa_polynomial_to_i32_array_ff_64(&hint_simd);
    hint.ptr[i1] = uu____0;
  }
  return true_hints;
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_gamma1_serialize_64(
    const Eurydice_arr_60 *re, Eurydice_mut_borrow_slice_u8 serialized,
    size_t gamma1_exponent) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    const __m256i *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_avx2_gamma1_serialize_a2(
        simd_unit,
        Eurydice_slice_subslice_mut_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 * (gamma1_exponent + (size_t)1U),
                (i0 + (size_t)1U) * (gamma1_exponent + (size_t)1U)})),
        gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_signature_serialize_64(
    Eurydice_borrow_slice_u8 commitment_hash,
    Eurydice_dst_ref_shared_ba signer_response, Eurydice_dst_ref_shared_20 hint,
    size_t commitment_hash_size, size_t columns_in_a, size_t rows_in_a,
    size_t gamma1_exponent, size_t gamma1_ring_element_size,
    size_t max_ones_in_hint, Eurydice_mut_borrow_slice_u8 signature) {
  size_t offset = (size_t)0U;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_c8(
          signature,
          (core_ops_range_Range_87{offset, offset + commitment_hash_size})),
      commitment_hash, uint8_t);
  offset += commitment_hash_size;
  for (size_t i = (size_t)0U; i < columns_in_a; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_gamma1_serialize_64(
        &signer_response.ptr[i0],
        Eurydice_slice_subslice_mut_c8(
            signature, (core_ops_range_Range_87{
                           offset, offset + gamma1_ring_element_size})),
        gamma1_exponent);
    offset += gamma1_ring_element_size;
  }
  size_t true_hints_seen = (size_t)0U;
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)256U; i++) {
      size_t j = i;
      if (hint.ptr[i1].data[j] == 1) {
        signature.ptr[offset + true_hints_seen] = (uint8_t)j;
        true_hints_seen++;
      }
    }
    signature.ptr[offset + max_ones_in_hint + i1] = (uint8_t)true_hints_seen;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_07(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    core_option_Option_84 domain_separation_context, Eurydice_arr_ec randomness,
    Eurydice_arr_0c *signature) {
  Eurydice_borrow_slice_u8_x2 uu____0 = Eurydice_slice_split_at(
      signing_key, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 remaining_serialized0 = uu____0.snd;
  Eurydice_borrow_slice_u8_x2 uu____1 = Eurydice_slice_split_at(
      remaining_serialized0, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE,
      uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_signing = uu____1.fst;
  Eurydice_borrow_slice_u8 remaining_serialized1 = uu____1.snd;
  Eurydice_borrow_slice_u8_x2 uu____2 = Eurydice_slice_split_at(
      remaining_serialized1,
      LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 verification_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 remaining_serialized2 = uu____2.snd;
  Eurydice_borrow_slice_u8_x2 uu____3 = Eurydice_slice_split_at(
      remaining_serialized2,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE *
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 s1_serialized = uu____3.fst;
  Eurydice_borrow_slice_u8 remaining_serialized = uu____3.snd;
  Eurydice_borrow_slice_u8_x2 uu____4 = Eurydice_slice_split_at(
      remaining_serialized,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE *
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 s2_serialized = uu____4.fst;
  Eurydice_borrow_slice_u8 t0_serialized = uu____4.snd;
  Eurydice_arr_77 s1_as_ntt;
  Eurydice_arr_60 repeat_expression0[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(s1_as_ntt.data, repeat_expression0,
         (size_t)5U * sizeof(Eurydice_arr_60));
  Eurydice_arr_e2 s2_as_ntt;
  Eurydice_arr_60 repeat_expression1[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(s2_as_ntt.data, repeat_expression1,
         (size_t)6U * sizeof(Eurydice_arr_60));
  Eurydice_arr_e2 t0_as_ntt;
  Eurydice_arr_60 repeat_expression2[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression2[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(t0_as_ntt.data, repeat_expression2,
         (size_t)6U * sizeof(Eurydice_arr_60));
  libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_64(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
      s1_serialized, Eurydice_array_to_slice_mut_205(&s1_as_ntt));
  libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_64(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
      s2_serialized, Eurydice_array_to_slice_mut_206(&s2_as_ntt));
  libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_64(
      t0_serialized, Eurydice_array_to_slice_mut_206(&t0_as_ntt));
  Eurydice_arr_9a matrix;
  Eurydice_arr_60 repeat_expression3[30U];
  for (size_t i = (size_t)0U; i < (size_t)30U; i++) {
    repeat_expression3[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(matrix.data, repeat_expression3,
         (size_t)30U * sizeof(Eurydice_arr_60));
  libcrux_ml_dsa_samplex4_avx2_matrix_flat_e8_64(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, seed_for_a,
      Eurydice_array_to_slice_mut_204(&matrix));
  Eurydice_arr_c7 message_representative = {{0U}};
  libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(
      verification_key_hash, &domain_separation_context, message,
      &message_representative);
  Eurydice_arr_c7 mask_seed = {{0U}};
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d shake0 =
      libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake0, seed_for_signing);
  libcrux_ml_dsa_hash_functions_portable_absorb_26(
      &shake0, Eurydice_array_to_slice_shared_01(&randomness));
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
      &shake0, Eurydice_array_to_slice_shared_17(&message_representative));
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(
      &shake0, Eurydice_array_to_slice_mut_17(&mask_seed));
  uint16_t domain_separator_for_mask = 0U;
  size_t attempt = (size_t)0U;
  core_option_Option_81 commitment_hash0 = {core_option_None};
  core_option_Option_61 signer_response0 = {core_option_None};
  core_option_Option_05 hint0 = {core_option_None};
  while (attempt < LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN) {
    attempt++;
    Eurydice_arr_77 mask;
    Eurydice_arr_60 repeat_expression4[5U];
    for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
      repeat_expression4[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
    }
    memcpy(mask.data, repeat_expression4, (size_t)5U * sizeof(Eurydice_arr_60));
    Eurydice_arr_e2 w0;
    Eurydice_arr_60 repeat_expression5[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
      repeat_expression5[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
    }
    memcpy(w0.data, repeat_expression5, (size_t)6U * sizeof(Eurydice_arr_60));
    Eurydice_arr_e2 commitment;
    Eurydice_arr_60 repeat_expression6[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
      repeat_expression6[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
    }
    memcpy(commitment.data, repeat_expression6,
           (size_t)6U * sizeof(Eurydice_arr_60));
    libcrux_ml_dsa_sample_sample_mask_vector_f4(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT, &mask_seed,
        &domain_separator_for_mask, Eurydice_array_to_slice_mut_205(&mask));
    Eurydice_arr_e2 a_x_mask;
    Eurydice_arr_60 repeat_expression[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
      repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
    }
    memcpy(a_x_mask.data, repeat_expression,
           (size_t)6U * sizeof(Eurydice_arr_60));
    Eurydice_arr_77 mask_ntt = core_array__core__clone__Clone_for__T__N___clone(
        (size_t)5U, &mask, Eurydice_arr_60, Eurydice_arr_77);
    for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
      size_t i0 = i;
      libcrux_ml_dsa_ntt_ntt_64(&mask_ntt.data[i0]);
    }
    libcrux_ml_dsa_matrix_compute_matrix_x_mask_64(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        Eurydice_array_to_slice_shared_206(&matrix),
        Eurydice_array_to_slice_shared_203(&mask_ntt),
        Eurydice_array_to_slice_mut_206(&a_x_mask));
    libcrux_ml_dsa_arithmetic_decompose_vector_64(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
        Eurydice_array_to_slice_shared_205(&a_x_mask),
        Eurydice_array_to_slice_mut_206(&w0),
        Eurydice_array_to_slice_mut_206(&commitment));
    Eurydice_arr_65 commitment_hash_candidate = {{0U}};
    Eurydice_arr_d2 commitment_serialized = {{0U}};
    libcrux_ml_dsa_encoding_commitment_serialize_vector_64(
        LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE,
        Eurydice_array_to_slice_shared_205(&commitment),
        Eurydice_array_to_slice_mut_27(&commitment_serialized));
    libcrux_sha3_generic_keccak_xof_KeccakXofState_8d shake =
        libcrux_ml_dsa_hash_functions_portable_init_26();
    libcrux_ml_dsa_hash_functions_portable_absorb_26(
        &shake, Eurydice_array_to_slice_shared_17(&message_representative));
    libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
        &shake, Eurydice_array_to_slice_shared_27(&commitment_serialized));
    libcrux_ml_dsa_hash_functions_portable_squeeze_26(
        &shake, Eurydice_array_to_slice_mut_9f(&commitment_hash_candidate));
    Eurydice_arr_60 verifier_challenge = libcrux_ml_dsa_polynomial_zero_ff_64();
    libcrux_ml_dsa_sample_sample_challenge_ring_element_18(
        Eurydice_array_to_slice_shared_9f(&commitment_hash_candidate),
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE,
        &verifier_challenge);
    libcrux_ml_dsa_ntt_ntt_64(&verifier_challenge);
    Eurydice_arr_77 challenge_times_s1 =
        core_array__core__clone__Clone_for__T__N___clone(
            (size_t)5U, &s1_as_ntt, Eurydice_arr_60, Eurydice_arr_77);
    Eurydice_arr_e2 challenge_times_s2 =
        core_array__core__clone__Clone_for__T__N___clone(
            (size_t)6U, &s2_as_ntt, Eurydice_arr_60, Eurydice_arr_e2);
    libcrux_ml_dsa_matrix_vector_times_ring_element_64(
        Eurydice_array_to_slice_mut_205(&challenge_times_s1),
        &verifier_challenge);
    libcrux_ml_dsa_matrix_vector_times_ring_element_64(
        Eurydice_array_to_slice_mut_206(&challenge_times_s2),
        &verifier_challenge);
    libcrux_ml_dsa_matrix_add_vectors_64(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        Eurydice_array_to_slice_mut_205(&mask),
        Eurydice_array_to_slice_shared_203(&challenge_times_s1));
    libcrux_ml_dsa_matrix_subtract_vectors_64(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        Eurydice_array_to_slice_mut_206(&w0),
        Eurydice_array_to_slice_shared_205(&challenge_times_s2));
    if (!libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_64(
            Eurydice_array_to_slice_shared_203(&mask),
            (int32_t)((uint32_t)1 << (uint32_t)
                          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT) -
                LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)) {
      if (!libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_64(
              Eurydice_array_to_slice_shared_205(&w0),
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2 -
                  LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)) {
        Eurydice_arr_e2 challenge_times_t0 =
            core_array__core__clone__Clone_for__T__N___clone(
                (size_t)6U, &t0_as_ntt, Eurydice_arr_60, Eurydice_arr_e2);
        libcrux_ml_dsa_matrix_vector_times_ring_element_64(
            Eurydice_array_to_slice_mut_206(&challenge_times_t0),
            &verifier_challenge);
        if (!libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_64(
                Eurydice_array_to_slice_shared_205(&challenge_times_t0),
                LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2)) {
          libcrux_ml_dsa_matrix_add_vectors_64(
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
              Eurydice_array_to_slice_mut_206(&w0),
              Eurydice_array_to_slice_shared_205(&challenge_times_t0));
          Eurydice_arr_5d0 hint_candidate = {
              {{{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}}};
          size_t ones_in_hint = libcrux_ml_dsa_arithmetic_make_hint_64(
              Eurydice_array_to_slice_shared_205(&w0),
              Eurydice_array_to_slice_shared_205(&commitment),
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
              Eurydice_array_to_slice_mut_86(&hint_candidate));
          if (!(ones_in_hint >
                LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT)) {
            attempt = LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN;
            commitment_hash0 = (core_option_Option_81{
                core_option_Some, commitment_hash_candidate});
            signer_response0 = (core_option_Option_61{core_option_Some, mask});
            hint0 = (core_option_Option_05{core_option_Some, hint_candidate});
          }
        }
      }
    }
  }
  core_result_Result_53 uu____5;
  if (commitment_hash0.tag == core_option_None) {
    uu____5 = (core_result_Result_53{
        core_result_Err,
        libcrux_ml_dsa_types_SigningError_RejectionSamplingError});
  } else {
    Eurydice_arr_65 commitment_hash = commitment_hash0.f0;
    Eurydice_arr_65 commitment_hash1 = commitment_hash;
    if (signer_response0.tag == core_option_None) {
      uu____5 = (core_result_Result_53{
          core_result_Err,
          libcrux_ml_dsa_types_SigningError_RejectionSamplingError});
    } else {
      Eurydice_arr_77 signer_response = signer_response0.f0;
      Eurydice_arr_77 signer_response1 = signer_response;
      if (!(hint0.tag == core_option_None)) {
        Eurydice_arr_5d0 hint = hint0.f0;
        Eurydice_arr_5d0 hint1 = hint;
        libcrux_ml_dsa_encoding_signature_serialize_64(
            Eurydice_array_to_slice_shared_9f(&commitment_hash1),
            Eurydice_array_to_slice_shared_203(&signer_response1),
            Eurydice_array_to_slice_shared_86(&hint1),
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
            LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,
            Eurydice_array_to_slice_mut_6b(signature));
        return (core_result_Result_53{core_result_Ok});
      }
      uu____5 = (core_result_Result_53{
          core_result_Err,
          libcrux_ml_dsa_types_SigningError_RejectionSamplingError});
    }
  }
  return uu____5;
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_07(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness,
    Eurydice_arr_0c *signature) {
  core_result_Result_a8 uu____0 = libcrux_ml_dsa_pre_hash_new_88(
      context, (core_option_Option_57{core_option_None}));
  if (!(uu____0.tag == core_result_Ok)) {
    return (core_result_Result_53{
        core_result_Err,
        libcrux_ml_dsa_types_SigningError_ContextTooLongError});
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
      dsc;
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_07(
      signing_key, message,
      (core_option_Option_84{core_option_Some, domain_separation_context}),
      randomness, signature);
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4 with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_07(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness) {
  Eurydice_arr_0c signature = libcrux_ml_dsa_types_zero_c5_5c();
  core_result_Result_53 uu____0 =
      libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_07(
          signing_key, message, context, randomness, &signature);
  core_result_Result_8c uu____1;
  if (uu____0.tag == core_result_Ok) {
    uu____1 = core_result_Result_8c_s(
        core_result_Ok, &core_result_Result_8c_s::U::case_Ok, signature);
  } else {
    libcrux_ml_dsa_types_SigningError e = uu____0.f0;
    uu____1 = core_result_Result_8c_s(core_result_Err,
                                      &core_result_Result_8c_s::U::case_Err, e);
  }
  return uu____1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign__inner(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_07(
      Eurydice_array_to_slice_shared_98(signing_key), message, context,
      randomness);
}

/**
 Sign.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign__inner(
      signing_key, message, context, randomness);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_mut__inner(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness,
    Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_07(
      Eurydice_array_to_slice_shared_98(signing_key), message, context,
      randomness, signature);
}

/**
 Sign.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_mut(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness,
    Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_mut__inner(
      signing_key, message, context, randomness, signature);
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed_mut with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_portable_Shake128,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4,
libcrux_ml_dsa_pre_hash_SHAKE128_PH with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_37(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_ec randomness,
    Eurydice_arr_0c *signature) {
  if (!(context.meta > LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN)) {
    libcrux_ml_dsa_pre_hash_hash_30_83(message, pre_hash_buffer);
    core_result_Result_a8 uu____0 = libcrux_ml_dsa_pre_hash_new_88(
        context, (core_option_Option_57{core_option_Some,
                                        libcrux_ml_dsa_pre_hash_oid_30()}));
    if (!(uu____0.tag == core_result_Ok)) {
      return (core_result_Result_53{
          core_result_Err,
          libcrux_ml_dsa_types_SigningError_ContextTooLongError});
    }
    libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
    libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
        dsc;
    return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_07(
        signing_key,
        (Eurydice_borrow_slice_u8{pre_hash_buffer.ptr, pre_hash_buffer.meta}),
        (core_option_Option_84{core_option_Some, domain_separation_context}),
        randomness, signature);
  }
  return (core_result_Result_53{
      core_result_Err, libcrux_ml_dsa_types_SigningError_ContextTooLongError});
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_portable_Shake128,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_simd256_Shake256x4,
libcrux_ml_dsa_pre_hash_SHAKE128_PH with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_37(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_ec randomness) {
  Eurydice_arr_0c signature = libcrux_ml_dsa_types_zero_c5_5c();
  core_result_Result_53 uu____0 =
      libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_37(
          signing_key, message, context, pre_hash_buffer, randomness,
          &signature);
  core_result_Result_8c uu____1;
  if (uu____0.tag == core_result_Ok) {
    uu____1 = core_result_Result_8c_s(
        core_result_Ok, &core_result_Result_8c_s::U::case_Ok, signature);
  } else {
    libcrux_ml_dsa_types_SigningError e = uu____0.f0;
    uu____1 = core_result_Result_8c_s(core_result_Err,
                                      &core_result_Result_8c_s::U::case_Err, e);
  }
  return uu____1;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_pre_hashed_shake128__inner(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_ec randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_37(
      Eurydice_array_to_slice_shared_98(signing_key), message, context,
      pre_hash_buffer, randomness);
}

/**
 Sign (pre-hashed).
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_pre_hashed_shake128(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_ec randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_pre_hashed_shake128__inner(
      signing_key, message, context, pre_hash_buffer, randomness);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_encoding_t1_deserialize_64(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_60 *result) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_t1_deserialize_a2(
        Eurydice_slice_subslice_shared_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 * LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.verification_key.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_verification_key_deserialize_64(
    size_t rows_in_a, size_t verification_key_size,
    Eurydice_borrow_slice_u8 serialized, Eurydice_dst_ref_mut_ba t1) {
  for (size_t i = (size_t)0U; i < rows_in_a; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_t1_deserialize_64(
        Eurydice_slice_subslice_shared_c8(
            serialized,
            (core_ops_range_Range_87{
                i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE})),
        &t1.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_encoding_signature_deserialize_64(
    size_t columns_in_a, size_t rows_in_a, size_t commitment_hash_size,
    size_t gamma1_exponent, size_t gamma1_ring_element_size,
    size_t max_ones_in_hint, size_t signature_size,
    Eurydice_borrow_slice_u8 serialized,
    Eurydice_mut_borrow_slice_u8 out_commitment_hash,
    Eurydice_dst_ref_mut_ba out_signer_response,
    Eurydice_dst_ref_mut_20 out_hint) {
  Eurydice_borrow_slice_u8_x2 uu____0 = Eurydice_slice_split_at(
      serialized, commitment_hash_size, uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 commitment_hash = uu____0.fst;
  Eurydice_borrow_slice_u8 rest_of_serialized = uu____0.snd;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_c8(
          out_commitment_hash,
          (core_ops_range_Range_87{(size_t)0U, commitment_hash_size})),
      commitment_hash, uint8_t);
  Eurydice_borrow_slice_u8_x2 uu____1 = Eurydice_slice_split_at(
      rest_of_serialized, gamma1_ring_element_size * columns_in_a, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 signer_response_serialized = uu____1.fst;
  Eurydice_borrow_slice_u8 hint_serialized = uu____1.snd;
  for (size_t i = (size_t)0U; i < columns_in_a; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_gamma1_deserialize_64(
        gamma1_exponent,
        Eurydice_slice_subslice_shared_c8(
            signer_response_serialized,
            (core_ops_range_Range_87{
                i0 * gamma1_ring_element_size,
                (i0 + (size_t)1U) * gamma1_ring_element_size})),
        &out_signer_response.ptr[i0]);
  }
  size_t previous_true_hints_seen = (size_t)0U;
  bool malformed_hint = false;
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    size_t current_true_hints_seen =
        (size_t)(uint32_t)hint_serialized.ptr[max_ones_in_hint + i1];
    if (current_true_hints_seen < previous_true_hints_seen) {
      malformed_hint = true;
      break;
    }
    if (current_true_hints_seen > max_ones_in_hint) {
      malformed_hint = true;
      break;
    }
    for (size_t i = previous_true_hints_seen; i < current_true_hints_seen;
         i++) {
      size_t j = i;
      if (j > previous_true_hints_seen) {
        if (hint_serialized.ptr[j] <= hint_serialized.ptr[j - (size_t)1U]) {
          malformed_hint = true;
          break;
        }
      }
      libcrux_ml_dsa_encoding_signature_set_hint(
          out_hint, i1, (size_t)(uint32_t)hint_serialized.ptr[j]);
    }
    if (malformed_hint) {
      break;
    }
    previous_true_hints_seen = current_true_hints_seen;
  }
  for (size_t i = previous_true_hints_seen; i < max_ones_in_hint; i++) {
    size_t j = i;
    if (hint_serialized.ptr[j] != 0U) {
      malformed_hint = true;
      break;
    }
  }
  core_result_Result_41 uu____2;
  if (malformed_hint) {
    uu____2 = (core_result_Result_41{
        core_result_Err,
        libcrux_ml_dsa_types_VerificationError_MalformedHintError});
  } else {
    uu____2 = (core_result_Result_41{core_result_Ok});
  }
  return uu____2;
}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.avx2.arithmetic.shift_left_then_reduce with const generics
- SHIFT_BY= 13
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_dsa_simd_avx2_arithmetic_shift_left_then_reduce_84(
    __m256i *simd_unit) {
  __m256i shifted =
      libcrux_intrinsics_avx2_mm256_slli_epi32(13, simd_unit[0U], __m256i);
  libcrux_ml_dsa_simd_avx2_arithmetic_barrett_reduce_simd_unit(&shifted);
  simd_unit[0U] = shifted;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.shift_left_then_reduce_a2
with const generics
- SHIFT_BY= 13
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_avx2_shift_left_then_reduce_a2_84(__m256i *simd_unit) {
  libcrux_ml_dsa_simd_avx2_arithmetic_shift_left_then_reduce_84(simd_unit);
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.shift_left_then_reduce
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics
- SHIFT_BY= 13
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_shift_left_then_reduce_3a(
    Eurydice_arr_60 *re) {
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_avx2_shift_left_then_reduce_a2_84(&re->data[i0]);
  }
}

/**
 Compute InvertNTT(Â ◦ ẑ - ĉ ◦ NTT(t₁2ᵈ))
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_w_approx
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_compute_w_approx_64(
    size_t rows_in_a, size_t columns_in_a, Eurydice_dst_ref_shared_ba matrix,
    Eurydice_dst_ref_shared_ba signer_response,
    const Eurydice_arr_60 *verifier_challenge_as_ntt,
    Eurydice_dst_ref_mut_ba t1) {
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    Eurydice_arr_60 inner_result = libcrux_ml_dsa_polynomial_zero_ff_64();
    for (size_t i = (size_t)0U; i < columns_in_a; i++) {
      size_t j = i;
      Eurydice_arr_60 product = matrix.ptr[i1 * columns_in_a + j];
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_64(&product,
                                                    &signer_response.ptr[j]);
      libcrux_ml_dsa_polynomial_add_ff_64(&inner_result, &product);
    }
    libcrux_ml_dsa_arithmetic_shift_left_then_reduce_3a(&t1.ptr[i1]);
    libcrux_ml_dsa_ntt_ntt_64(&t1.ptr[i1]);
    libcrux_ml_dsa_ntt_ntt_multiply_montgomery_64(&t1.ptr[i1],
                                                  verifier_challenge_as_ntt);
    libcrux_ml_dsa_polynomial_subtract_ff_64(&inner_result, &t1.ptr[i1]);
    t1.ptr[i1] = inner_result;
    libcrux_ml_dsa_polynomial_barrett_reduce_ff_64(&t1.ptr[i1]);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_64(&t1.ptr[i1]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.use_hint
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_use_hint_64(
    int32_t gamma2, Eurydice_dst_ref_shared_20 hint,
    Eurydice_dst_ref_mut_ba re_vector) {
  for (size_t i0 = (size_t)0U; i0 < re_vector.meta; i0++) {
    size_t i1 = i0;
    Eurydice_arr_60 tmp = libcrux_ml_dsa_polynomial_zero_ff_64();
    libcrux_ml_dsa_polynomial_from_i32_array_ff_64(
        Eurydice_array_to_slice_shared_af(&hint.ptr[i1]), &tmp);
    for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
      size_t j = i;
      libcrux_ml_dsa_simd_avx2_use_hint_a2(gamma2, &re_vector.ptr[i1].data[j],
                                           &tmp.data[j]);
    }
    re_vector.ptr[i1] = tmp;
  }
}

/**
 The internal verification API.

 If no `domain_separation_context` is supplied, it is assumed that
 `message` already contains the domain separation.
*/
/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_internal with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_07(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    core_option_Option_84 domain_separation_context,
    const Eurydice_arr_0c *signature_serialized) {
  Eurydice_borrow_slice_u8_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_37(verification_key),
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 t1_serialized = uu____0.snd;
  Eurydice_arr_e2 t1;
  Eurydice_arr_60 repeat_expression0[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(t1.data, repeat_expression0, (size_t)6U * sizeof(Eurydice_arr_60));
  libcrux_ml_dsa_encoding_verification_key_deserialize_64(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_VERIFICATION_KEY_SIZE,
      t1_serialized, Eurydice_array_to_slice_mut_206(&t1));
  Eurydice_arr_65 deserialized_commitment_hash = {{0U}};
  Eurydice_arr_77 deserialized_signer_response;
  Eurydice_arr_60 repeat_expression1[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
  }
  memcpy(deserialized_signer_response.data, repeat_expression1,
         (size_t)5U * sizeof(Eurydice_arr_60));
  Eurydice_arr_5d0 deserialized_hint = {
      {{{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}}};
  core_result_Result_41 uu____1 =
      libcrux_ml_dsa_encoding_signature_deserialize_64(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNATURE_SIZE,
          Eurydice_array_to_slice_shared_6b(signature_serialized),
          Eurydice_array_to_slice_mut_9f(&deserialized_commitment_hash),
          Eurydice_array_to_slice_mut_205(&deserialized_signer_response),
          Eurydice_array_to_slice_mut_86(&deserialized_hint));
  core_result_Result_41 uu____2;
  if (uu____1.tag == core_result_Ok) {
    if (libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_64(
            Eurydice_array_to_slice_shared_203(&deserialized_signer_response),
            (int32_t)((uint32_t)1 << (uint32_t)
                          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT) -
                LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)) {
      uu____2 = (core_result_Result_41{
          core_result_Err,
          libcrux_ml_dsa_types_VerificationError_SignerResponseExceedsBoundError});
    } else {
      Eurydice_arr_9a matrix;
      Eurydice_arr_60 repeat_expression[30U];
      for (size_t i = (size_t)0U; i < (size_t)30U; i++) {
        repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_64();
      }
      memcpy(matrix.data, repeat_expression,
             (size_t)30U * sizeof(Eurydice_arr_60));
      libcrux_ml_dsa_samplex4_avx2_matrix_flat_e8_64(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, seed_for_a,
          Eurydice_array_to_slice_mut_204(&matrix));
      Eurydice_arr_c7 verification_key_hash = {{0U}};
      libcrux_ml_dsa_hash_functions_simd256_shake256_8c_c9(
          Eurydice_array_to_slice_shared_37(verification_key),
          &verification_key_hash);
      Eurydice_arr_c7 message_representative = {{0U}};
      libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(
          Eurydice_array_to_slice_shared_17(&verification_key_hash),
          &domain_separation_context, message, &message_representative);
      Eurydice_arr_60 verifier_challenge =
          libcrux_ml_dsa_polynomial_zero_ff_64();
      libcrux_ml_dsa_sample_sample_challenge_ring_element_18(
          Eurydice_array_to_slice_shared_9f(&deserialized_commitment_hash),
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE,
          &verifier_challenge);
      libcrux_ml_dsa_ntt_ntt_64(&verifier_challenge);
      for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
        size_t i0 = i;
        libcrux_ml_dsa_ntt_ntt_64(&deserialized_signer_response.data[i0]);
      }
      libcrux_ml_dsa_matrix_compute_w_approx_64(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
          Eurydice_array_to_slice_shared_206(&matrix),
          Eurydice_array_to_slice_shared_203(&deserialized_signer_response),
          &verifier_challenge, Eurydice_array_to_slice_mut_206(&t1));
      Eurydice_arr_65 recomputed_commitment_hash = {{0U}};
      libcrux_ml_dsa_arithmetic_use_hint_64(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
          Eurydice_array_to_slice_shared_86(&deserialized_hint),
          Eurydice_array_to_slice_mut_206(&t1));
      Eurydice_arr_d2 commitment_serialized = {{0U}};
      libcrux_ml_dsa_encoding_commitment_serialize_vector_64(
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE,
          Eurydice_array_to_slice_shared_205(&t1),
          Eurydice_array_to_slice_mut_27(&commitment_serialized));
      libcrux_sha3_generic_keccak_xof_KeccakXofState_8d shake =
          libcrux_ml_dsa_hash_functions_portable_init_26();
      libcrux_ml_dsa_hash_functions_portable_absorb_26(
          &shake, Eurydice_array_to_slice_shared_17(&message_representative));
      libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
          &shake, Eurydice_array_to_slice_shared_27(&commitment_serialized));
      libcrux_ml_dsa_hash_functions_portable_squeeze_26(
          &shake, Eurydice_array_to_slice_mut_9f(&recomputed_commitment_hash));
      if (Eurydice_array_eq((size_t)48U, &deserialized_commitment_hash,
                            &recomputed_commitment_hash, uint8_t)) {
        uu____2 = (core_result_Result_41{core_result_Ok});
      } else {
        uu____2 = (core_result_Result_41{
            core_result_Err,
            libcrux_ml_dsa_types_VerificationError_CommitmentHashesDontMatchError});
      }
    }
  } else {
    libcrux_ml_dsa_types_VerificationError e = uu____1.f0;
    uu____2 = (core_result_Result_41{core_result_Err, e});
  }
  return uu____2;
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_07(
    const Eurydice_arr_29 *verification_key_serialized,
    Eurydice_borrow_slice_u8 message, Eurydice_borrow_slice_u8 context,
    const Eurydice_arr_0c *signature_serialized) {
  core_result_Result_a8 uu____0 = libcrux_ml_dsa_pre_hash_new_88(
      context, (core_option_Option_57{core_option_None}));
  if (!(uu____0.tag == core_result_Ok)) {
    return (core_result_Result_41{
        core_result_Err,
        libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError});
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
      dsc;
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_07(
      verification_key_serialized, message,
      (core_option_Option_84{core_option_Some, domain_separation_context}),
      signature_serialized);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify__inner(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_07(
      verification_key, message, context, signature);
}

/**
 Verify.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify__inner(
      verification_key, message, context, signature);
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_pre_hashed with types
libcrux_ml_dsa_simd_avx2_vector_type_Vec256,
libcrux_ml_dsa_samplex4_avx2_AVX2Sampler,
libcrux_ml_dsa_hash_functions_portable_Shake128,
libcrux_ml_dsa_hash_functions_simd256_Shake128x4,
libcrux_ml_dsa_hash_functions_simd256_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_pre_hash_SHAKE128_PH with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_37(
    const Eurydice_arr_29 *verification_key_serialized,
    Eurydice_borrow_slice_u8 message, Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
    const Eurydice_arr_0c *signature_serialized) {
  libcrux_ml_dsa_pre_hash_hash_30_83(message, pre_hash_buffer);
  core_result_Result_a8 uu____0 = libcrux_ml_dsa_pre_hash_new_88(
      context, (core_option_Option_57{core_option_Some,
                                      libcrux_ml_dsa_pre_hash_oid_30()}));
  if (!(uu____0.tag == core_result_Ok)) {
    return (core_result_Result_41{
        core_result_Err,
        libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError});
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
      dsc;
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_07(
      verification_key_serialized,
      (Eurydice_borrow_slice_u8{pre_hash_buffer.ptr, pre_hash_buffer.meta}),
      (core_option_Option_84{core_option_Some, domain_separation_context}),
      signature_serialized);
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify_pre_hashed_shake128__inner(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
    const Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_37(
      verification_key, message, context, pre_hash_buffer, signature);
}

/**
 Verify (pre-hashed with SHAKE-128).
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify_pre_hashed_shake128(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
    const Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify_pre_hashed_shake128__inner(
      verification_key, message, context, pre_hash_buffer, signature);
}

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_3_STEP ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_3_STEP_BY \
  ((size_t)1U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_4_STEP ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_4_STEP_BY \
  ((size_t)2U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_5_STEP ((size_t)32U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_5_STEP_BY \
  ((size_t)4U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_6_STEP ((size_t)64U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_6_STEP_BY \
  ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_7_STEP \
  ((size_t)128U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_7_STEP_BY \
  ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_SIMD_UNIT_INVERT_NTT_AT_LAYER_0_SHUFFLE \
  (216)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_BUTTERFLY_2_SHUFFLE (216)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP ((size_t)1U << 5U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_1 \
  ((size_t)1U << 4U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_2 \
  ((size_t)1U << 3U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_BY \
  (LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP /       \
   LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_BY_1 \
  (LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_1 /       \
   LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_BY_2 \
  (LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_2 /       \
   LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT)

KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_dsa_simd_avx2_rejection_sample_shuffle_table_is_bit_set(
    size_t number, uint8_t bit_position) {
  return (number & (size_t)1U << (uint32_t)bit_position) >>
             (uint32_t)bit_position ==
         (size_t)1U;
}

KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_a30
libcrux_ml_dsa_simd_avx2_rejection_sample_shuffle_table_generate_shuffle_table(
    void) {
  Eurydice_arr_a30 byte_shuffles;
  Eurydice_arr_b2 repeat_expression0[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    Eurydice_arr_b2 lit;
    uint8_t repeat_expression[16U];
    memset(repeat_expression, 255U, (size_t)16U * sizeof(uint8_t));
    memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(uint8_t));
    repeat_expression0[i] = lit;
  }
  memcpy(byte_shuffles.data, repeat_expression0,
         (size_t)16U * sizeof(Eurydice_arr_b2));
  for (size_t i0 = (size_t)0U; i0 < (size_t)1U << 4U; i0++) {
    size_t bit_pattern = i0;
    size_t byte_shuffles_index = (size_t)0U;
    for (uint8_t i = 0U; i < 4U; i = (uint32_t)i + 1U) {
      uint8_t bit_position = i;
      if (libcrux_ml_dsa_simd_avx2_rejection_sample_shuffle_table_is_bit_set(
              bit_pattern, bit_position)) {
        byte_shuffles.data[bit_pattern].data[byte_shuffles_index] =
            (uint32_t)bit_position * 4U;
        byte_shuffles_index++;
        byte_shuffles.data[bit_pattern].data[byte_shuffles_index] =
            (uint32_t)bit_position * 4U + 1U;
        byte_shuffles_index++;
        byte_shuffles.data[bit_pattern].data[byte_shuffles_index] =
            (uint32_t)bit_position * 4U + 2U;
        byte_shuffles_index++;
        byte_shuffles.data[bit_pattern].data[byte_shuffles_index] =
            (uint32_t)bit_position * 4U + 3U;
        byte_shuffles_index++;
      }
    }
  }
  return byte_shuffles;
}

typedef Eurydice_arr_60 libcrux_ml_dsa_simd_avx2_vector_type_AVX2RingElement;

/**
This function found in impl {core::clone::Clone for
libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i libcrux_ml_dsa_simd_avx2_vector_type_clone_be(
    const __m256i *self) {
  return self[0U];
}

#define libcrux_mldsa_avx2_H_DEFINED
#endif /* libcrux_mldsa_avx2_H */
