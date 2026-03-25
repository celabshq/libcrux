/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 146b7dce58cb11ca8010b1c947c3437a959dcd88
 * Eurydice: cdf02f9d8ed0d73f88c0a495c5b79359a51398fc
 * Karamel: 8e7262955105599e91f3a99c9ab3d3387f7046f2
 * F*: 89901492c020c74b82d811d27f3149c222d9b8b5
 * Libcrux: 4faeb5fdd7bdf29c1f30136c8f32fe24c06cfab0
 */

#ifndef libcrux_mlkem768_avx2_H
#define libcrux_mlkem768_avx2_H

#include "eurydice_glue.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include "intrinsics/libcrux_intrinsics_avx2.h"
#include "libcrux_ct_ops.h"
#include "libcrux_mlkem768_portable.h"
#include "libcrux_mlkem_core.h"
#include "libcrux_sha3_avx2.h"
#include "libcrux_sha3_portable.h"

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_06
libcrux_ml_kem_hash_functions_avx2_G(Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_06 digest = {.data = {0U}};
  libcrux_sha3_portable_sha512(Eurydice_array_to_slice_mut_d8(&digest), input);
  return digest;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_hash_functions_avx2_H(Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_600 digest = {.data = {0U}};
  libcrux_sha3_portable_sha256(Eurydice_array_to_slice_mut_6e(&digest), input);
  return digest;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_vec_zero(void) {
  return libcrux_intrinsics_avx2_mm256_setzero_si256();
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ZERO_f5(void) {
  return libcrux_ml_kem_vector_avx2_vec_zero();
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_vec_from_i16_array(Eurydice_borrow_slice_i16 array) {
  return libcrux_intrinsics_avx2_mm256_loadu_si256_i16(array);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_from_i16_array_f5(Eurydice_borrow_slice_i16 array) {
  return libcrux_ml_kem_vector_avx2_vec_from_i16_array(array);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_add(__m256i lhs, __m256i rhs) {
  return libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_add_f5(__m256i lhs, const __m256i *rhs) {
  return libcrux_ml_kem_vector_avx2_arithmetic_add(lhs, rhs[0U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_sub(__m256i lhs, __m256i rhs) {
  return libcrux_intrinsics_avx2_mm256_sub_epi16(lhs, rhs);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_sub_f5(__m256i lhs, const __m256i *rhs) {
  return libcrux_ml_kem_vector_avx2_arithmetic_sub(lhs, rhs[0U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_multiply_by_constant(__m256i vector,
                                                           int16_t constant) {
  __m256i cv = libcrux_intrinsics_avx2_mm256_set1_epi16(constant);
  return libcrux_intrinsics_avx2_mm256_mullo_epi16(vector, cv);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_multiply_by_constant_f5(__m256i vec, int16_t c) {
  return libcrux_ml_kem_vector_avx2_arithmetic_multiply_by_constant(vec, c);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_cond_subtract_3329(__m256i vector) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi16(
      LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i v_minus_field_modulus =
      libcrux_intrinsics_avx2_mm256_sub_epi16(vector, field_modulus);
  __m256i sign_mask = libcrux_intrinsics_avx2_mm256_srai_epi16(
      (int32_t)15, v_minus_field_modulus, __m256i);
  __m256i conditional_add_field_modulus =
      libcrux_intrinsics_avx2_mm256_and_si256(sign_mask, field_modulus);
  return libcrux_intrinsics_avx2_mm256_add_epi16(v_minus_field_modulus,
                                                 conditional_add_field_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_cond_subtract_3329(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_arithmetic_cond_subtract_3329(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_cond_subtract_3329_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_cond_subtract_3329(vector);
}

#define LIBCRUX_ML_KEM_VECTOR_AVX2_ARITHMETIC_BARRETT_MULTIPLIER \
  ((int16_t)20159)

/**
 See Section 3.2 of the implementation notes document for an explanation
 of this code.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_barrett_reduce(__m256i vector) {
  __m256i t0 = libcrux_intrinsics_avx2_mm256_mulhi_epi16(
      vector, libcrux_intrinsics_avx2_mm256_set1_epi16(
                  LIBCRUX_ML_KEM_VECTOR_AVX2_ARITHMETIC_BARRETT_MULTIPLIER));
  __m256i t512 = libcrux_intrinsics_avx2_mm256_set1_epi16((int16_t)512);
  __m256i t1 = libcrux_intrinsics_avx2_mm256_add_epi16(t0, t512);
  __m256i quotient =
      libcrux_intrinsics_avx2_mm256_srai_epi16((int32_t)10, t1, __m256i);
  __m256i quotient_times_field_modulus =
      libcrux_intrinsics_avx2_mm256_mullo_epi16(
          quotient, libcrux_intrinsics_avx2_mm256_set1_epi16(
                        LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
  return libcrux_intrinsics_avx2_mm256_sub_epi16(vector,
                                                 quotient_times_field_modulus);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_barrett_reduce_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_arithmetic_barrett_reduce(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constant(
    __m256i vector, int16_t constant) {
  __m256i vec_constant = libcrux_intrinsics_avx2_mm256_set1_epi16(constant);
  __m256i value_low =
      libcrux_intrinsics_avx2_mm256_mullo_epi16(vector, vec_constant);
  __m256i k = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      value_low,
      libcrux_intrinsics_avx2_mm256_set1_epi16(
          (int16_t)
              LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m256i modulus = libcrux_intrinsics_avx2_mm256_set1_epi16(
      LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i k_times_modulus =
      libcrux_intrinsics_avx2_mm256_mulhi_epi16(k, modulus);
  __m256i value_high =
      libcrux_intrinsics_avx2_mm256_mulhi_epi16(vector, vec_constant);
  return libcrux_intrinsics_avx2_mm256_sub_epi16(value_high, k_times_modulus);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(
    __m256i vector, int16_t constant) {
  return libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constant(
      vector, constant);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_bitwise_and_with_constant(
    __m256i vector, int16_t constant) {
  __m256i cv = libcrux_intrinsics_avx2_mm256_set1_epi16(constant);
  return libcrux_intrinsics_avx2_mm256_and_si256(vector, cv);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.arithmetic.shift_right
with const generics
- SHIFT_BY= 15
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_shift_right_ef(__m256i vector) {
  return libcrux_intrinsics_avx2_mm256_srai_epi16((int32_t)15, vector, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_to_unsigned_representative(__m256i a) {
  __m256i t = libcrux_ml_kem_vector_avx2_arithmetic_shift_right_ef(a);
  __m256i fm = libcrux_ml_kem_vector_avx2_arithmetic_bitwise_and_with_constant(
      t, LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  return libcrux_ml_kem_vector_avx2_arithmetic_add(a, fm);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i libcrux_ml_kem_vector_avx2_to_unsigned_representative_f5(
    __m256i a) {
  return libcrux_ml_kem_vector_avx2_arithmetic_to_unsigned_representative(a);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_message_coefficient(
    __m256i vector) {
  __m256i field_modulus_halved = libcrux_intrinsics_avx2_mm256_set1_epi16(
      (LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS - (int16_t)1) / (int16_t)2);
  __m256i field_modulus_quartered = libcrux_intrinsics_avx2_mm256_set1_epi16(
      (LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS - (int16_t)1) / (int16_t)4);
  __m256i shifted =
      libcrux_intrinsics_avx2_mm256_sub_epi16(field_modulus_halved, vector);
  __m256i mask =
      libcrux_intrinsics_avx2_mm256_srai_epi16((int32_t)15, shifted, __m256i);
  __m256i shifted_to_positive =
      libcrux_intrinsics_avx2_mm256_xor_si256(mask, shifted);
  __m256i shifted_to_positive_in_range =
      libcrux_intrinsics_avx2_mm256_sub_epi16(shifted_to_positive,
                                              field_modulus_quartered);
  return libcrux_intrinsics_avx2_mm256_srli_epi16(
      (int32_t)15, shifted_to_positive_in_range, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_1(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_compress_message_coefficient(
      vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_1_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_1(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(__m256i lhs,
                                                      __m256i rhs) {
  __m256i prod02 = libcrux_intrinsics_avx2_mm256_mul_epu32(lhs, rhs);
  __m256i prod13 = libcrux_intrinsics_avx2_mm256_mul_epu32(
      libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)245, lhs, __m256i),
      libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)245, rhs, __m256i));
  return libcrux_intrinsics_avx2_mm256_unpackhi_epi64(
      libcrux_intrinsics_avx2_mm256_unpacklo_epi32(prod02, prod13),
      libcrux_intrinsics_avx2_mm256_unpackhi_epi32(prod02, prod13));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_1(__m256i a) {
  __m256i z = libcrux_intrinsics_avx2_mm256_setzero_si256();
  __m256i s = libcrux_ml_kem_vector_avx2_arithmetic_sub(z, a);
  return libcrux_ml_kem_vector_avx2_arithmetic_bitwise_and_with_constant(
      s, (int16_t)1665);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i libcrux_ml_kem_vector_avx2_decompress_1_f5(__m256i a) {
  return libcrux_ml_kem_vector_avx2_compress_decompress_1(a);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(
    __m256i vec, __m256i constants) {
  __m256i value_low = libcrux_intrinsics_avx2_mm256_mullo_epi16(vec, constants);
  __m256i k = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      value_low,
      libcrux_intrinsics_avx2_mm256_set1_epi16(
          (int16_t)
              LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m256i modulus = libcrux_intrinsics_avx2_mm256_set1_epi16(
      LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i k_times_modulus =
      libcrux_intrinsics_avx2_mm256_mulhi_epi16(k, modulus);
  __m256i value_high =
      libcrux_intrinsics_avx2_mm256_mulhi_epi16(vec, constants);
  return libcrux_intrinsics_avx2_mm256_sub_epi16(value_high, k_times_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_ntt_layer_1_step(
    __m256i vector, int16_t zeta0, int16_t zeta1, int16_t zeta2,
    int16_t zeta3) {
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi16(
      -zeta3, -zeta3, zeta3, zeta3, -zeta2, -zeta2, zeta2, zeta2, -zeta1,
      -zeta1, zeta1, zeta1, -zeta0, -zeta0, zeta0, zeta0);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)245,
                                                            vector, __m256i);
  __m256i rhs0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(
          rhs, zetas);
  __m256i lhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)160,
                                                            vector, __m256i);
  return libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_layer_1_step(
    __m256i vector, int16_t zeta0, int16_t zeta1, int16_t zeta2,
    int16_t zeta3) {
  return libcrux_ml_kem_vector_avx2_ntt_ntt_layer_1_step(vector, zeta0, zeta1,
                                                         zeta2, zeta3);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_layer_1_step_f5(
    __m256i vector, int16_t zeta0, int16_t zeta1, int16_t zeta2,
    int16_t zeta3) {
  return libcrux_ml_kem_vector_avx2_ntt_layer_1_step(vector, zeta0, zeta1,
                                                     zeta2, zeta3);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_ntt_layer_2_step(
    __m256i vector, int16_t zeta0, int16_t zeta1) {
  __m256i zetas = libcrux_intrinsics_avx2_mm256_set_epi16(
      -zeta1, -zeta1, -zeta1, -zeta1, zeta1, zeta1, zeta1, zeta1, -zeta0,
      -zeta0, -zeta0, -zeta0, zeta0, zeta0, zeta0, zeta0);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)238,
                                                            vector, __m256i);
  __m256i rhs0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(
          rhs, zetas);
  __m256i lhs =
      libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)68, vector, __m256i);
  return libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_layer_2_step(
    __m256i vector, int16_t zeta0, int16_t zeta1) {
  return libcrux_ml_kem_vector_avx2_ntt_ntt_layer_2_step(vector, zeta0, zeta1);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_layer_2_step_f5(
    __m256i vector, int16_t zeta0, int16_t zeta1) {
  return libcrux_ml_kem_vector_avx2_ntt_layer_2_step(vector, zeta0, zeta1);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m128i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_m128i_by_constants(
    __m128i vec, __m128i constants) {
  __m128i value_low = libcrux_intrinsics_avx2_mm_mullo_epi16(vec, constants);
  __m128i k = libcrux_intrinsics_avx2_mm_mullo_epi16(
      value_low,
      libcrux_intrinsics_avx2_mm_set1_epi16(
          (int16_t)
              LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m128i modulus = libcrux_intrinsics_avx2_mm_set1_epi16(
      LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m128i k_times_modulus = libcrux_intrinsics_avx2_mm_mulhi_epi16(k, modulus);
  __m128i value_high = libcrux_intrinsics_avx2_mm_mulhi_epi16(vec, constants);
  return libcrux_intrinsics_avx2_mm_sub_epi16(value_high, k_times_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_ntt_layer_3_step(__m256i vector, int16_t zeta) {
  __m128i rhs = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, vector, __m128i);
  __m128i rhs0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_m128i_by_constants(
          rhs, libcrux_intrinsics_avx2_mm_set1_epi16(zeta));
  __m128i lhs = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m128i lower_coefficients = libcrux_intrinsics_avx2_mm_add_epi16(lhs, rhs0);
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm_sub_epi16(lhs, rhs0);
  __m256i combined =
      libcrux_intrinsics_avx2_mm256_castsi128_si256(lower_coefficients);
  return libcrux_intrinsics_avx2_mm256_inserti128_si256(
      (int32_t)1, combined, upper_coefficients, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_3_step(__m256i vector, int16_t zeta) {
  return libcrux_ml_kem_vector_avx2_ntt_ntt_layer_3_step(vector, zeta);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_3_step_f5(__m256i vector, int16_t zeta) {
  return libcrux_ml_kem_vector_avx2_ntt_layer_3_step(vector, zeta);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_1_step(__m256i vector,
                                                    int16_t zeta0,
                                                    int16_t zeta1,
                                                    int16_t zeta2,
                                                    int16_t zeta3) {
  __m256i lhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)245,
                                                            vector, __m256i);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32((int32_t)160,
                                                            vector, __m256i);
  __m256i rhs0 = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      rhs, libcrux_intrinsics_avx2_mm256_set_epi16(
               (int16_t)-1, (int16_t)-1, (int16_t)1, (int16_t)1, (int16_t)-1,
               (int16_t)-1, (int16_t)1, (int16_t)1, (int16_t)-1, (int16_t)-1,
               (int16_t)1, (int16_t)1, (int16_t)-1, (int16_t)-1, (int16_t)1,
               (int16_t)1));
  __m256i sum0 = libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
  __m256i sum_times_zetas =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(
          sum0, libcrux_intrinsics_avx2_mm256_set_epi16(
                    zeta3, zeta3, (int16_t)0, (int16_t)0, zeta2, zeta2,
                    (int16_t)0, (int16_t)0, zeta1, zeta1, (int16_t)0,
                    (int16_t)0, zeta0, zeta0, (int16_t)0, (int16_t)0));
  __m256i sum = libcrux_ml_kem_vector_avx2_arithmetic_barrett_reduce(sum0);
  return libcrux_intrinsics_avx2_mm256_blend_epi16((int32_t)204, sum,
                                                   sum_times_zetas, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step(
    __m256i vector, int16_t zeta0, int16_t zeta1, int16_t zeta2,
    int16_t zeta3) {
  return libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_1_step(
      vector, zeta0, zeta1, zeta2, zeta3);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step_f5(__m256i vector,
                                                   int16_t zeta0, int16_t zeta1,
                                                   int16_t zeta2,
                                                   int16_t zeta3) {
  return libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step(vector, zeta0, zeta1,
                                                         zeta2, zeta3);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_2_step(__m256i vector,
                                                    int16_t zeta0,
                                                    int16_t zeta1) {
  __m256i lhs = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(
      (int32_t)245, vector, __m256i);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(
      (int32_t)160, vector, __m256i);
  __m256i rhs0 = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      rhs, libcrux_intrinsics_avx2_mm256_set_epi16(
               (int16_t)-1, (int16_t)-1, (int16_t)-1, (int16_t)-1, (int16_t)1,
               (int16_t)1, (int16_t)1, (int16_t)1, (int16_t)-1, (int16_t)-1,
               (int16_t)-1, (int16_t)-1, (int16_t)1, (int16_t)1, (int16_t)1,
               (int16_t)1));
  __m256i sum = libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
  __m256i sum_times_zetas =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(
          sum, libcrux_intrinsics_avx2_mm256_set_epi16(
                   zeta1, zeta1, zeta1, zeta1, (int16_t)0, (int16_t)0,
                   (int16_t)0, (int16_t)0, zeta0, zeta0, zeta0, zeta0,
                   (int16_t)0, (int16_t)0, (int16_t)0, (int16_t)0));
  return libcrux_intrinsics_avx2_mm256_blend_epi16((int32_t)240, sum,
                                                   sum_times_zetas, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step(
    __m256i vector, int16_t zeta0, int16_t zeta1) {
  return libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_2_step(vector, zeta0,
                                                             zeta1);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step_f5(__m256i vector,
                                                   int16_t zeta0,
                                                   int16_t zeta1) {
  return libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step(vector, zeta0, zeta1);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_3_step(__m256i vector,
                                                    int16_t zeta) {
  __m128i lhs = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, vector, __m128i);
  __m128i rhs = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m128i lower_coefficients = libcrux_intrinsics_avx2_mm_add_epi16(lhs, rhs);
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm_sub_epi16(lhs, rhs);
  __m128i upper_coefficients0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_m128i_by_constants(
          upper_coefficients, libcrux_intrinsics_avx2_mm_set1_epi16(zeta));
  __m256i combined =
      libcrux_intrinsics_avx2_mm256_castsi128_si256(lower_coefficients);
  return libcrux_intrinsics_avx2_mm256_inserti128_si256(
      (int32_t)1, combined, upper_coefficients0, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step(__m256i vector, int16_t zeta) {
  return libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_3_step(vector, zeta);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step_f5(__m256i vector,
                                                   int16_t zeta) {
  return libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step(vector, zeta);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(__m256i vec) {
  __m256i k = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      vec,
      libcrux_intrinsics_avx2_mm256_set1_epi32(
          (int32_t)
              LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m256i k_times_modulus = libcrux_intrinsics_avx2_mm256_mulhi_epi16(
      k, libcrux_intrinsics_avx2_mm256_set1_epi32(
             (int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
  __m256i value_high =
      libcrux_intrinsics_avx2_mm256_srli_epi32((int32_t)16, vec, __m256i);
  __m256i result =
      libcrux_intrinsics_avx2_mm256_sub_epi16(value_high, k_times_modulus);
  __m256i result0 =
      libcrux_intrinsics_avx2_mm256_slli_epi32((int32_t)16, result, __m256i);
  return libcrux_intrinsics_avx2_mm256_srai_epi32((int32_t)16, result0,
                                                  __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_ntt_multiply(
    __m256i lhs, __m256i rhs, int16_t zeta0, int16_t zeta1, int16_t zeta2,
    int16_t zeta3) {
  __m256i shuffle_with = libcrux_intrinsics_avx2_mm256_set_epi8(
      (int8_t)15, (int8_t)14, (int8_t)11, (int8_t)10, (int8_t)7, (int8_t)6,
      (int8_t)3, (int8_t)2, (int8_t)13, (int8_t)12, (int8_t)9, (int8_t)8,
      (int8_t)5, (int8_t)4, (int8_t)1, (int8_t)0, (int8_t)15, (int8_t)14,
      (int8_t)11, (int8_t)10, (int8_t)7, (int8_t)6, (int8_t)3, (int8_t)2,
      (int8_t)13, (int8_t)12, (int8_t)9, (int8_t)8, (int8_t)5, (int8_t)4,
      (int8_t)1, (int8_t)0);
  __m256i lhs_shuffled =
      libcrux_intrinsics_avx2_mm256_shuffle_epi8(lhs, shuffle_with);
  __m256i lhs_shuffled0 = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(
      (int32_t)216, lhs_shuffled, __m256i);
  __m128i lhs_evens =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(lhs_shuffled0);
  __m256i lhs_evens0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(lhs_evens);
  __m128i lhs_odds = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, lhs_shuffled0, __m128i);
  __m256i lhs_odds0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(lhs_odds);
  __m256i rhs_shuffled =
      libcrux_intrinsics_avx2_mm256_shuffle_epi8(rhs, shuffle_with);
  __m256i rhs_shuffled0 = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(
      (int32_t)216, rhs_shuffled, __m256i);
  __m128i rhs_evens =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(rhs_shuffled0);
  __m256i rhs_evens0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(rhs_evens);
  __m128i rhs_odds = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, rhs_shuffled0, __m128i);
  __m256i rhs_odds0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(rhs_odds);
  __m256i left =
      libcrux_intrinsics_avx2_mm256_mullo_epi32(lhs_evens0, rhs_evens0);
  __m256i right =
      libcrux_intrinsics_avx2_mm256_mullo_epi32(lhs_odds0, rhs_odds0);
  __m256i right0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(right);
  __m256i right1 = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      right0,
      libcrux_intrinsics_avx2_mm256_set_epi32(
          -(int32_t)zeta3, (int32_t)zeta3, -(int32_t)zeta2, (int32_t)zeta2,
          -(int32_t)zeta1, (int32_t)zeta1, -(int32_t)zeta0, (int32_t)zeta0));
  __m256i products_left = libcrux_intrinsics_avx2_mm256_add_epi32(left, right1);
  __m256i products_left0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(
          products_left);
  __m256i rhs_adjacent_swapped = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      rhs,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          (int8_t)13, (int8_t)12, (int8_t)15, (int8_t)14, (int8_t)9, (int8_t)8,
          (int8_t)11, (int8_t)10, (int8_t)5, (int8_t)4, (int8_t)7, (int8_t)6,
          (int8_t)1, (int8_t)0, (int8_t)3, (int8_t)2, (int8_t)13, (int8_t)12,
          (int8_t)15, (int8_t)14, (int8_t)9, (int8_t)8, (int8_t)11, (int8_t)10,
          (int8_t)5, (int8_t)4, (int8_t)7, (int8_t)6, (int8_t)1, (int8_t)0,
          (int8_t)3, (int8_t)2));
  __m256i products_right =
      libcrux_intrinsics_avx2_mm256_madd_epi16(lhs, rhs_adjacent_swapped);
  __m256i products_right0 =
      libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(
          products_right);
  __m256i products_right1 = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)16, products_right0, __m256i);
  return libcrux_intrinsics_avx2_mm256_blend_epi16((int32_t)170, products_left0,
                                                   products_right1, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_multiply(
    const __m256i *lhs, const __m256i *rhs, int16_t zeta0, int16_t zeta1,
    int16_t zeta2, int16_t zeta3) {
  return libcrux_ml_kem_vector_avx2_ntt_ntt_multiply(lhs[0U], rhs[0U], zeta0,
                                                     zeta1, zeta2, zeta3);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i libcrux_ml_kem_vector_avx2_ntt_multiply_f5(
    const __m256i *lhs, const __m256i *rhs, int16_t zeta0, int16_t zeta1,
    int16_t zeta2, int16_t zeta3) {
  return libcrux_ml_kem_vector_avx2_ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2,
                                                 zeta3);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_avx2_serialize_serialize_1(__m256i vector) {
  __m256i lsb_to_msb =
      libcrux_intrinsics_avx2_mm256_slli_epi16((int32_t)15, vector, __m256i);
  __m128i low_msbs = libcrux_intrinsics_avx2_mm256_castsi256_si128(lsb_to_msb);
  __m128i high_msbs = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, lsb_to_msb, __m128i);
  __m128i msbs = libcrux_intrinsics_avx2_mm_packs_epi16(low_msbs, high_msbs);
  int32_t bits_packed = libcrux_intrinsics_avx2_mm_movemask_epi8(msbs);
  Eurydice_array_u8x2 result = {
      .data = {(uint8_t)bits_packed, (uint8_t)(bits_packed >> 8U)}};
  return result;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_avx2_serialize_1(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_serialize_1(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_avx2_serialize_1_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_1(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_i16s(
    int16_t a, int16_t b) {
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_set_epi16(
      b, b, b, b, b, b, b, b, a, a, a, a, a, a, a, a);
  __m256i coefficients_in_msb = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      coefficients, libcrux_intrinsics_avx2_mm256_set_epi16(
                        (int16_t)1 << 8U, (int16_t)1 << 9U, (int16_t)1 << 10U,
                        (int16_t)1 << 11U, (int16_t)1 << 12U, (int16_t)1 << 13U,
                        (int16_t)1 << 14U, (int16_t)-32768, (int16_t)1 << 8U,
                        (int16_t)1 << 9U, (int16_t)1 << 10U, (int16_t)1 << 11U,
                        (int16_t)1 << 12U, (int16_t)1 << 13U, (int16_t)1 << 14U,
                        (int16_t)-32768));
  return libcrux_intrinsics_avx2_mm256_srli_epi16((int32_t)15,
                                                  coefficients_in_msb, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_u8s(
    uint8_t a, uint8_t b) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_i16s(
      (int16_t)a, (int16_t)b);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_1(
    Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_u8s(
      Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_1(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_1(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_1_f5(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_deserialize_1(bytes);
}

/**
 `mm256_concat_pairs_n(n, x)` is then a sequence of 32 bits packets
 of the shape `0b0…0b₁…bₙa₁…aₙ`, if `x` is a sequence of pairs of
 16 bits, of the shape `(0b0…0a₁…aₙ, 0b0…0b₁…bₙ)` (where the last
 `n` bits are non-zero).
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(uint8_t n,
                                                          __m256i x) {
  int16_t n0 = (int16_t)1 << (uint32_t)n;
  return libcrux_intrinsics_avx2_mm256_madd_epi16(
      x, libcrux_intrinsics_avx2_mm256_set_epi16(
             n0, (int16_t)1, n0, (int16_t)1, n0, (int16_t)1, n0, (int16_t)1, n0,
             (int16_t)1, n0, (int16_t)1, n0, (int16_t)1, n0, (int16_t)1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_avx2_serialize_serialize_4(__m256i vector) {
  Eurydice_arr_88 serialized = {.data = {0U}};
  __m256i adjacent_2_combined =
      libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(4U, vector);
  __m256i adjacent_8_combined = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      adjacent_2_combined,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1,
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1,
          (int8_t)-1, (int8_t)-1, (int8_t)12, (int8_t)8, (int8_t)4, (int8_t)0,
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1,
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1,
          (int8_t)-1, (int8_t)-1, (int8_t)12, (int8_t)8, (int8_t)4, (int8_t)0));
  __m256i combined = libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(
      adjacent_8_combined, libcrux_intrinsics_avx2_mm256_set_epi32(
                               (int32_t)0, (int32_t)0, (int32_t)0, (int32_t)0,
                               (int32_t)0, (int32_t)0, (int32_t)4, (int32_t)0));
  __m128i combined0 = libcrux_intrinsics_avx2_mm256_castsi256_si128(combined);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_slice_mut_46(&serialized), combined0);
  Eurydice_array_u8x8 arr;
  memcpy(arr.data,
         Eurydice_array_to_subslice_shared_365(
             &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                              .start = (size_t)0U, .end = (size_t)8U}))
             .ptr,
         (size_t)8U * sizeof(uint8_t));
  return unwrap_26_ab(
      (KRML_CLITERAL(Result_a4){.tag = Ok, .val = {.case_Ok = arr}}));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_avx2_serialize_4(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_serialize_4(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_avx2_serialize_4_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_4(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_i16s(
    int16_t b0, int16_t b1, int16_t b2, int16_t b3, int16_t b4, int16_t b5,
    int16_t b6, int16_t b7) {
  __m256i coefficients = libcrux_intrinsics_avx2_mm256_set_epi16(
      b7, b7, b6, b6, b5, b5, b4, b4, b3, b3, b2, b2, b1, b1, b0, b0);
  __m256i coefficients_in_msb = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      coefficients, libcrux_intrinsics_avx2_mm256_set_epi16(
                        (int16_t)1 << 0U, (int16_t)1 << 4U, (int16_t)1 << 0U,
                        (int16_t)1 << 4U, (int16_t)1 << 0U, (int16_t)1 << 4U,
                        (int16_t)1 << 0U, (int16_t)1 << 4U, (int16_t)1 << 0U,
                        (int16_t)1 << 4U, (int16_t)1 << 0U, (int16_t)1 << 4U,
                        (int16_t)1 << 0U, (int16_t)1 << 4U, (int16_t)1 << 0U,
                        (int16_t)1 << 4U));
  __m256i coefficients_in_lsb = libcrux_intrinsics_avx2_mm256_srli_epi16(
      (int32_t)4, coefficients_in_msb, __m256i);
  return libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients_in_lsb, libcrux_intrinsics_avx2_mm256_set1_epi16(
                               ((int16_t)1 << 4U) - (int16_t)1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_u8s(
    uint8_t b0, uint8_t b1, uint8_t b2, uint8_t b3, uint8_t b4, uint8_t b5,
    uint8_t b6, uint8_t b7) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_i16s(
      (int16_t)b0, (int16_t)b1, (int16_t)b2, (int16_t)b3, (int16_t)b4,
      (int16_t)b5, (int16_t)b6, (int16_t)b7);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_4(
    Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_u8s(
      Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)4U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)5U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)6U, uint8_t),
      Eurydice_slice_index_shared(bytes, (size_t)7U, uint8_t));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_4(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_4(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_4_f5(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_deserialize_4(bytes);
}

/**
 We cannot model `mm256_inserti128_si256` on its own: it produces a
 Vec256 where the upper 128 bits are undefined. Thus
 `mm256_inserti128_si256` is not pure.

 Luckily, we always call `mm256_castsi128_si256` right after
 `mm256_inserti128_si256`: this composition sets the upper bits,
 making the whole computation pure again.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(__m128i lower,
                                                                __m128i upper) {
  return libcrux_intrinsics_avx2_mm256_inserti128_si256(
      (int32_t)1, libcrux_intrinsics_avx2_mm256_castsi128_si256(lower), upper,
      __m256i);
}

typedef struct core_core_arch_x86___m128i_x2_s {
  __m128i fst;
  __m128i snd;
} core_core_arch_x86___m128i_x2;

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_core_arch_x86___m128i_x2
libcrux_ml_kem_vector_avx2_serialize_serialize_10_serialize_10_vec(
    __m256i vector) {
  __m256i adjacent_2_combined =
      libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(10U, vector);
  __m256i adjacent_4_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      adjacent_2_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(
          (int32_t)0, (int32_t)12, (int32_t)0, (int32_t)12, (int32_t)0,
          (int32_t)12, (int32_t)0, (int32_t)12));
  __m256i adjacent_4_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      (int32_t)12, adjacent_4_combined, __m256i);
  __m256i adjacent_8_combined = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      adjacent_4_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1,
          (int8_t)-1, (int8_t)12, (int8_t)11, (int8_t)10, (int8_t)9, (int8_t)8,
          (int8_t)4, (int8_t)3, (int8_t)2, (int8_t)1, (int8_t)0, (int8_t)-1,
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1,
          (int8_t)12, (int8_t)11, (int8_t)10, (int8_t)9, (int8_t)8, (int8_t)4,
          (int8_t)3, (int8_t)2, (int8_t)1, (int8_t)0));
  __m128i lower_8 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_8_combined);
  __m128i upper_8 = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, adjacent_8_combined, __m128i);
  return (KRML_CLITERAL(core_core_arch_x86___m128i_x2){.fst = lower_8,
                                                       .snd = upper_8});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_dc
libcrux_ml_kem_vector_avx2_serialize_serialize_10(__m256i vector) {
  core_core_arch_x86___m128i_x2 uu____0 =
      libcrux_ml_kem_vector_avx2_serialize_serialize_10_serialize_10_vec(
          vector);
  __m128i lower_8 = uu____0.fst;
  __m128i upper_8 = uu____0.snd;
  Eurydice_arr_600 serialized = {.data = {0U}};
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_364(
          &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                           .start = (size_t)0U, .end = (size_t)16U})),
      lower_8);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_364(
          &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                           .start = (size_t)10U, .end = (size_t)26U})),
      upper_8);
  Eurydice_arr_dc arr;
  memcpy(arr.data,
         Eurydice_array_to_subslice_shared_362(
             &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                              .start = (size_t)0U, .end = (size_t)20U}))
             .ptr,
         (size_t)20U * sizeof(uint8_t));
  return unwrap_26_51(
      (KRML_CLITERAL(Result_6e){.tag = Ok, .val = {.case_Ok = arr}}));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_dc
libcrux_ml_kem_vector_avx2_serialize_10(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_serialize_10(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_dc
libcrux_ml_kem_vector_avx2_serialize_10_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_10(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_10_deserialize_10_vec(
    __m128i lower_coefficients0, __m128i upper_coefficients0) {
  __m128i lower_coefficients = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      lower_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(
          (int8_t)9, (int8_t)8, (int8_t)8, (int8_t)7, (int8_t)7, (int8_t)6,
          (int8_t)6, (int8_t)5, (int8_t)4, (int8_t)3, (int8_t)3, (int8_t)2,
          (int8_t)2, (int8_t)1, (int8_t)1, (int8_t)0));
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      upper_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(
          (int8_t)15, (int8_t)14, (int8_t)14, (int8_t)13, (int8_t)13,
          (int8_t)12, (int8_t)12, (int8_t)11, (int8_t)10, (int8_t)9, (int8_t)9,
          (int8_t)8, (int8_t)8, (int8_t)7, (int8_t)7, (int8_t)6));
  __m256i coefficients =
      libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(
          lower_coefficients, upper_coefficients);
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      coefficients, libcrux_intrinsics_avx2_mm256_set_epi16(
                        (int16_t)1 << 0U, (int16_t)1 << 2U, (int16_t)1 << 4U,
                        (int16_t)1 << 6U, (int16_t)1 << 0U, (int16_t)1 << 2U,
                        (int16_t)1 << 4U, (int16_t)1 << 6U, (int16_t)1 << 0U,
                        (int16_t)1 << 2U, (int16_t)1 << 4U, (int16_t)1 << 6U,
                        (int16_t)1 << 0U, (int16_t)1 << 2U, (int16_t)1 << 4U,
                        (int16_t)1 << 6U));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_srli_epi16(
      (int32_t)6, coefficients0, __m256i);
  return libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients1, libcrux_intrinsics_avx2_mm256_set1_epi16(
                         ((int16_t)1 << 10U) - (int16_t)1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_10(
    Eurydice_borrow_slice_u8 bytes) {
  Eurydice_borrow_slice_u8 lower_coefficients =
      Eurydice_slice_subslice_shared_7e(
          bytes, (KRML_CLITERAL(core_ops_range_Range_08){.start = (size_t)0U,
                                                         .end = (size_t)16U}));
  Eurydice_borrow_slice_u8 upper_coefficients =
      Eurydice_slice_subslice_shared_7e(
          bytes, (KRML_CLITERAL(core_ops_range_Range_08){.start = (size_t)4U,
                                                         .end = (size_t)20U}));
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_10_deserialize_10_vec(
      libcrux_intrinsics_avx2_mm_loadu_si128(lower_coefficients),
      libcrux_intrinsics_avx2_mm_loadu_si128(upper_coefficients));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_10(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_10(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_10_f5(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_deserialize_10(bytes);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_core_arch_x86___m128i_x2
libcrux_ml_kem_vector_avx2_serialize_serialize_12_serialize_12_vec(
    __m256i vector) {
  __m256i adjacent_2_combined =
      libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(12U, vector);
  __m256i adjacent_4_combined = libcrux_intrinsics_avx2_mm256_sllv_epi32(
      adjacent_2_combined, libcrux_intrinsics_avx2_mm256_set_epi32(
                               (int32_t)0, (int32_t)8, (int32_t)0, (int32_t)8,
                               (int32_t)0, (int32_t)8, (int32_t)0, (int32_t)8));
  __m256i adjacent_4_combined0 = libcrux_intrinsics_avx2_mm256_srli_epi64(
      (int32_t)8, adjacent_4_combined, __m256i);
  __m256i adjacent_8_combined = libcrux_intrinsics_avx2_mm256_shuffle_epi8(
      adjacent_4_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)13,
          (int8_t)12, (int8_t)11, (int8_t)10, (int8_t)9, (int8_t)8, (int8_t)5,
          (int8_t)4, (int8_t)3, (int8_t)2, (int8_t)1, (int8_t)0, (int8_t)-1,
          (int8_t)-1, (int8_t)-1, (int8_t)-1, (int8_t)13, (int8_t)12,
          (int8_t)11, (int8_t)10, (int8_t)9, (int8_t)8, (int8_t)5, (int8_t)4,
          (int8_t)3, (int8_t)2, (int8_t)1, (int8_t)0));
  __m128i lower_8 =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_8_combined);
  __m128i upper_8 = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, adjacent_8_combined, __m128i);
  return (KRML_CLITERAL(core_core_arch_x86___m128i_x2){.fst = lower_8,
                                                       .snd = upper_8});
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_ml_kem_vector_avx2_serialize_serialize_12(__m256i vector) {
  Eurydice_arr_600 serialized = {.data = {0U}};
  core_core_arch_x86___m128i_x2 uu____0 =
      libcrux_ml_kem_vector_avx2_serialize_serialize_12_serialize_12_vec(
          vector);
  __m128i lower_8 = uu____0.fst;
  __m128i upper_8 = uu____0.snd;
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_364(
          &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                           .start = (size_t)0U, .end = (size_t)16U})),
      lower_8);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(
      Eurydice_array_to_subslice_mut_364(
          &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                           .start = (size_t)12U, .end = (size_t)28U})),
      upper_8);
  Eurydice_arr_6d arr;
  memcpy(arr.data,
         Eurydice_array_to_subslice_shared_362(
             &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                              .start = (size_t)0U, .end = (size_t)24U}))
             .ptr,
         (size_t)24U * sizeof(uint8_t));
  return unwrap_26_a9(
      (KRML_CLITERAL(Result_16){.tag = Ok, .val = {.case_Ok = arr}}));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_ml_kem_vector_avx2_serialize_12(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_serialize_12(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_ml_kem_vector_avx2_serialize_12_f5(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_serialize_12(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_12_deserialize_12_vec(
    __m128i lower_coefficients0, __m128i upper_coefficients0) {
  __m128i lower_coefficients = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      lower_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(
          (int8_t)11, (int8_t)10, (int8_t)10, (int8_t)9, (int8_t)8, (int8_t)7,
          (int8_t)7, (int8_t)6, (int8_t)5, (int8_t)4, (int8_t)4, (int8_t)3,
          (int8_t)2, (int8_t)1, (int8_t)1, (int8_t)0));
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      upper_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(
          (int8_t)15, (int8_t)14, (int8_t)14, (int8_t)13, (int8_t)12,
          (int8_t)11, (int8_t)11, (int8_t)10, (int8_t)9, (int8_t)8, (int8_t)8,
          (int8_t)7, (int8_t)6, (int8_t)5, (int8_t)5, (int8_t)4));
  __m256i coefficients =
      libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(
          lower_coefficients, upper_coefficients);
  __m256i coefficients0 = libcrux_intrinsics_avx2_mm256_mullo_epi16(
      coefficients, libcrux_intrinsics_avx2_mm256_set_epi16(
                        (int16_t)1 << 0U, (int16_t)1 << 4U, (int16_t)1 << 0U,
                        (int16_t)1 << 4U, (int16_t)1 << 0U, (int16_t)1 << 4U,
                        (int16_t)1 << 0U, (int16_t)1 << 4U, (int16_t)1 << 0U,
                        (int16_t)1 << 4U, (int16_t)1 << 0U, (int16_t)1 << 4U,
                        (int16_t)1 << 0U, (int16_t)1 << 4U, (int16_t)1 << 0U,
                        (int16_t)1 << 4U));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_srli_epi16(
      (int32_t)4, coefficients0, __m256i);
  return libcrux_intrinsics_avx2_mm256_and_si256(
      coefficients1, libcrux_intrinsics_avx2_mm256_set1_epi16(
                         ((int16_t)1 << 12U) - (int16_t)1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_12(
    Eurydice_borrow_slice_u8 bytes) {
  __m128i lower_coefficients =
      libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_7e(
          bytes, (KRML_CLITERAL(core_ops_range_Range_08){.start = (size_t)0U,
                                                         .end = (size_t)16U})));
  __m128i upper_coefficients =
      libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_7e(
          bytes, (KRML_CLITERAL(core_ops_range_Range_08){.start = (size_t)8U,
                                                         .end = (size_t)24U})));
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_12_deserialize_12_vec(
      lower_coefficients, upper_coefficients);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_12(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_12(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_12_f5(Eurydice_borrow_slice_u8 bytes) {
  return libcrux_ml_kem_vector_avx2_deserialize_12(bytes);
}

#define LIBCRUX_ML_KEM_VECTOR_REJ_SAMPLE_TABLE_REJECTION_SAMPLE_SHUFFLE_TABLE  \
  ((KRML_CLITERAL(Eurydice_arr_ef){                                            \
      .data = {{.data = {255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {2U, 3U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 2U, 3U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U,     \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U,     \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U,     \
                         12U, 13U, 255U, 255U}},                               \
               {.data = {14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U,     \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U,     \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U, 255U}},           \
               {.data = {0U, 1U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U,   \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U,     \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U,   \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U,     \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {2U, 3U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 2U, 3U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U,   \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U,     \
                         255U, 255U, 255U, 255U, 255U, 255U}},                 \
               {.data = {0U, 1U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U,   \
                         255U, 255U, 255U, 255U, 255U}},                       \
               {.data = {0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, \
                         255U, 255U, 255U, 255U}},                             \
               {.data = {0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U,   \
                         14U, 15U, 255U, 255U}},                               \
               {.data = {0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U,     \
                         12U, 13U, 14U, 15U}}}}))

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_kem_vector_avx2_sampling_rejection_sample(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_i16 output) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi16(
      LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i potential_coefficients =
      libcrux_ml_kem_vector_avx2_serialize_deserialize_12(input);
  __m256i compare_with_field_modulus =
      libcrux_intrinsics_avx2_mm256_cmpgt_epi16(field_modulus,
                                                potential_coefficients);
  Eurydice_array_u8x2 good = libcrux_ml_kem_vector_avx2_serialize_serialize_1(
      compare_with_field_modulus);
  Eurydice_arr_88 lower_shuffles =
      LIBCRUX_ML_KEM_VECTOR_REJ_SAMPLE_TABLE_REJECTION_SAMPLE_SHUFFLE_TABLE
          .data[(size_t)good.data[0U]];
  __m128i lower_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_46(&lower_shuffles));
  __m128i lower_coefficients =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(potential_coefficients);
  __m128i lower_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      lower_coefficients, lower_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128(output, lower_coefficients0);
  size_t sampled_count = (size_t)core_num__u8__count_ones(good.data[0U]);
  Eurydice_arr_88 upper_shuffles =
      LIBCRUX_ML_KEM_VECTOR_REJ_SAMPLE_TABLE_REJECTION_SAMPLE_SHUFFLE_TABLE
          .data[(size_t)good.data[1U]];
  __m128i upper_shuffles0 = libcrux_intrinsics_avx2_mm_loadu_si128(
      Eurydice_array_to_slice_shared_46(&upper_shuffles));
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, potential_coefficients, __m128i);
  __m128i upper_coefficients0 = libcrux_intrinsics_avx2_mm_shuffle_epi8(
      upper_coefficients, upper_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128(
      Eurydice_slice_subslice_mut_76(
          output,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = sampled_count, .end = sampled_count + (size_t)8U})),
      upper_coefficients0);
  size_t uu____0 = sampled_count;
  return uu____0 + (size_t)core_num__u8__count_ones(good.data[1U]);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t libcrux_ml_kem_vector_avx2_rej_sample_f5(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_i16 output) {
  return libcrux_ml_kem_vector_avx2_sampling_rejection_sample(input, output);
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_51_s {
  __m256i data[16U];
} Eurydice_arr_51;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- $3size_t
*/
typedef struct Eurydice_arr_260_s {
  Eurydice_arr_51 data[3U];
} Eurydice_arr_260;

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51 libcrux_ml_kem_polynomial_ZERO_d6_84(void) {
  Eurydice_arr_51 lit;
  __m256i repeat_expression[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    repeat_expression[i] = libcrux_ml_kem_vector_avx2_ZERO_f5();
  }
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(__m256i));
  return lit;
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for libcrux_ml_kem::ind_cpa::decrypt::closure<Vector, K,
CIPHERTEXT_SIZE, VECTOR_U_ENCODED_SIZE, U_COMPRESSION_FACTOR,
V_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt.call_mut_0b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- VECTOR_U_ENCODED_SIZE= 960
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51 libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_2f(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_to_uncompressed_ring_element with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_84(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_51 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_08){
            .start = i0 * (size_t)24U, .end = i0 * (size_t)24U + (size_t)24U}));
    re.data[i0] = libcrux_ml_kem_vector_avx2_deserialize_12_f5(bytes);
  }
  return re;
}

/**
 Call [`deserialize_to_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_vector
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_deserialize_vector_ab(
    Eurydice_borrow_slice_u8 secret_key, Eurydice_arr_260 *secret_as_ntt) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_51 uu____0 =
        libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_84(
            Eurydice_slice_subslice_shared_7e(
                secret_key,
                (KRML_CLITERAL(core_ops_range_Range_08){
                    .start =
                        i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                    .end = (i0 + (size_t)1U) *
                           LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT})));
    secret_as_ntt->data[i0] = uu____0;
  }
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for
libcrux_ml_kem::ind_cpa::deserialize_then_decompress_u::closure<Vector, K,
CIPHERTEXT_SIZE, U_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u.call_mut_35 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_ed(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.avx2.compress.decompress_ciphertext_coefficient with const
generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_ef(
    __m256i vector) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi32(
      (int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i two_pow_coefficient_bits = libcrux_intrinsics_avx2_mm256_set1_epi32(
      (int32_t)1 << (uint32_t)(int32_t)10);
  __m128i coefficients_low =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i decompressed_low = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      coefficients_low0, field_modulus);
  __m256i decompressed_low0 = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)1, decompressed_low, __m256i);
  __m256i decompressed_low1 = libcrux_intrinsics_avx2_mm256_add_epi32(
      decompressed_low0, two_pow_coefficient_bits);
  __m256i decompressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)10, decompressed_low1, __m256i);
  __m256i decompressed_low3 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)1, decompressed_low2, __m256i);
  __m128i coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, vector, __m128i);
  __m256i coefficients_high0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i decompressed_high = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      coefficients_high0, field_modulus);
  __m256i decompressed_high0 = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)1, decompressed_high, __m256i);
  __m256i decompressed_high1 = libcrux_intrinsics_avx2_mm256_add_epi32(
      decompressed_high0, two_pow_coefficient_bits);
  __m256i decompressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)10, decompressed_high1, __m256i);
  __m256i decompressed_high3 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)1, decompressed_high2, __m256i);
  __m256i compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(
      decompressed_low3, decompressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64((int32_t)216,
                                                         compressed, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of
libcrux_ml_kem.vector.avx2.decompress_ciphertext_coefficient_f5 with const
generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_ef(
    __m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_ef(
      vector);
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_10 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_then_decompress_10_84(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_51 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)20U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_08){
            .start = i0 * (size_t)20U, .end = i0 * (size_t)20U + (size_t)20U}));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_10_f5(bytes);
    re.data[i0] =
        libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_ef(
            coefficient);
  }
  return re;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_u with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_ee(
    Eurydice_borrow_slice_u8 serialized) {
  return libcrux_ml_kem_serialize_deserialize_then_decompress_10_84(serialized);
}

typedef struct libcrux_ml_kem_vector_avx2_SIMD256Vector_x2_s {
  __m256i fst;
  __m256i snd;
} libcrux_ml_kem_vector_avx2_SIMD256Vector_x2;

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_layer_int_vec_step
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_vector_avx2_SIMD256Vector_x2
libcrux_ml_kem_ntt_ntt_layer_int_vec_step_84(__m256i a, __m256i b,
                                             int16_t zeta_r) {
  __m256i t =
      libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(b, zeta_r);
  b = libcrux_ml_kem_vector_avx2_sub_f5(a, &t);
  a = libcrux_ml_kem_vector_avx2_add_f5(a, &t);
  return (KRML_CLITERAL(libcrux_ml_kem_vector_avx2_SIMD256Vector_x2){.fst = a,
                                                                     .snd = b});
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(
    size_t *zeta_i, Eurydice_arr_51 *re, size_t layer,
    size_t _initial_coefficient_bound) {
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++) {
    size_t round = i0;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec = offset / (size_t)16U;
    size_t step_vec = step / (size_t)16U;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++) {
      size_t j = i;
      libcrux_ml_kem_vector_avx2_SIMD256Vector_x2 uu____0 =
          libcrux_ml_kem_ntt_ntt_layer_int_vec_step_84(
              re->data[j], re->data[j + step_vec],
              libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
      __m256i x = uu____0.fst;
      __m256i y = uu____0.snd;
      re->data[j] = x;
      re->data[j + step_vec] = y;
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_3
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_3_84(
    size_t *zeta_i, Eurydice_arr_51 *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_avx2_ntt_layer_3_step_f5(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_2_84(
    size_t *zeta_i, Eurydice_arr_51 *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_avx2_ntt_layer_2_step_f5(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U));
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_1_84(
    size_t *zeta_i, Eurydice_arr_51 *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_avx2_ntt_layer_1_step_f5(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)3U));
    zeta_i[0U] = zeta_i[0U] + (size_t)3U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_poly_barrett_reduce_84(
    Eurydice_arr_51 *myself) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    myself->data[i0] =
        libcrux_ml_kem_vector_avx2_barrett_reduce_f5(myself->data[i0]);
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(
    Eurydice_arr_51 *self) {
  libcrux_ml_kem_polynomial_poly_barrett_reduce_84(self);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_vector_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- VECTOR_U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_vector_u_ee(
    Eurydice_arr_51 *re) {
  size_t zeta_i = (size_t)0U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)7U,
                                            (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U,
                                            (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U,
                                            (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)4U,
                                            (size_t)4U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_3_84(&zeta_i, re, (size_t)5U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_2_84(&zeta_i, re, (size_t)6U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_1_84(&zeta_i, re, (size_t)7U * (size_t)3328U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
 Call [`deserialize_then_decompress_ring_element_u`] on each ring element
 in the `ciphertext`.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_260
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_ed(
    const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_ed(
            &lvalue, i);
  }
  Eurydice_arr_260 u_as_ntt = arr_struct;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_42(ciphertext),
                              uint8_t) /
               (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                (size_t)10U / (size_t)8U);
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 u_bytes = Eurydice_array_to_subslice_shared_36(
        ciphertext,
        (KRML_CLITERAL(core_ops_range_Range_08){
            .start =
                i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                      (size_t)10U / (size_t)8U),
            .end = i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                         (size_t)10U / (size_t)8U) +
                   LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                       (size_t)10U / (size_t)8U}));
    u_as_ntt.data[i0] =
        libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_ee(
            u_bytes);
    libcrux_ml_kem_ntt_ntt_vector_u_ee(&u_as_ntt.data[i0]);
  }
  return u_as_ntt;
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.avx2.compress.decompress_ciphertext_coefficient with const
generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_d1(
    __m256i vector) {
  __m256i field_modulus = libcrux_intrinsics_avx2_mm256_set1_epi32(
      (int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i two_pow_coefficient_bits = libcrux_intrinsics_avx2_mm256_set1_epi32(
      (int32_t)1 << (uint32_t)(int32_t)4);
  __m128i coefficients_low =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i decompressed_low = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      coefficients_low0, field_modulus);
  __m256i decompressed_low0 = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)1, decompressed_low, __m256i);
  __m256i decompressed_low1 = libcrux_intrinsics_avx2_mm256_add_epi32(
      decompressed_low0, two_pow_coefficient_bits);
  __m256i decompressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)4, decompressed_low1, __m256i);
  __m256i decompressed_low3 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)1, decompressed_low2, __m256i);
  __m128i coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, vector, __m128i);
  __m256i coefficients_high0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i decompressed_high = libcrux_intrinsics_avx2_mm256_mullo_epi32(
      coefficients_high0, field_modulus);
  __m256i decompressed_high0 = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)1, decompressed_high, __m256i);
  __m256i decompressed_high1 = libcrux_intrinsics_avx2_mm256_add_epi32(
      decompressed_high0, two_pow_coefficient_bits);
  __m256i decompressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)4, decompressed_high1, __m256i);
  __m256i decompressed_high3 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)1, decompressed_high2, __m256i);
  __m256i compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(
      decompressed_low3, decompressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64((int32_t)216,
                                                         compressed, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of
libcrux_ml_kem.vector.avx2.decompress_ciphertext_coefficient_f5 with const
generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_d1(
    __m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_d1(
      vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_4
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_then_decompress_4_84(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_51 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)8U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_08){
            .start = i0 * (size_t)8U, .end = i0 * (size_t)8U + (size_t)8U}));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_4_f5(bytes);
    re.data[i0] =
        libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_d1(
            coefficient);
  }
  return re;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_v with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_ed(
    Eurydice_borrow_slice_u8 serialized) {
  return libcrux_ml_kem_serialize_deserialize_then_decompress_4_84(serialized);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51 libcrux_ml_kem_polynomial_ZERO_84(void) {
  Eurydice_arr_51 lit;
  __m256i repeat_expression[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    repeat_expression[i] = libcrux_ml_kem_vector_avx2_ZERO_f5();
  }
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(__m256i));
  return lit;
}

/**
 Given two `KyberPolynomialRingElement`s in their NTT representations,
 compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
 the `iᵗʰ` coefficient of the product `k̂` is determined by the calculation:

 ```plaintext
 ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X²
 - ζ^(2·BitRev₇(i) + 1))
 ```

 This function almost implements <strong>Algorithm 10</strong> of the
 NIST FIPS 203 standard, which is reproduced below:

 ```plaintext
 Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
 Output: An array ĥ ∈ ℤq.

 for(i ← 0; i < 128; i++)
     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1],
 ζ^(2·BitRev₇(i) + 1)) end for return ĥ
 ```
 We say "almost" because the coefficients of the ring element output by
 this function are in the Montgomery domain.

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ntt_multiply
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_ntt_multiply_84(const Eurydice_arr_51 *myself,
                                          const Eurydice_arr_51 *rhs) {
  Eurydice_arr_51 out = libcrux_ml_kem_polynomial_ZERO_84();
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    out.data[i0] = libcrux_ml_kem_vector_avx2_ntt_multiply_f5(
        &myself->data[i0], &rhs->data[i0],
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 +
                                       (size_t)1U),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 +
                                       (size_t)2U),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 +
                                       (size_t)3U));
  }
  return out;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ntt_multiply_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_ntt_multiply_d6_84(const Eurydice_arr_51 *self,
                                             const Eurydice_arr_51 *rhs) {
  return libcrux_ml_kem_polynomial_ntt_multiply_84(self, rhs);
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, size_t

*/
typedef struct Eurydice_dst_ref_shared_4b_s {
  const __m256i *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_4b;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- N= 16
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_4b Eurydice_array_to_slice_shared_85(
    const Eurydice_arr_51 *a) {
  Eurydice_dst_ref_shared_4b lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_to_ring_element_ab(
    Eurydice_arr_51 *myself, const Eurydice_arr_51 *rhs) {
  for (size_t i = (size_t)0U;
       i <
       Eurydice_slice_len(Eurydice_array_to_slice_shared_85(myself), __m256i);
       i++) {
    size_t i0 = i;
    myself->data[i0] =
        libcrux_ml_kem_vector_avx2_add_f5(myself->data[i0], &rhs->data[i0]);
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_to_ring_element_d6_ab(
    Eurydice_arr_51 *self, const Eurydice_arr_51 *rhs) {
  libcrux_ml_kem_polynomial_add_to_ring_element_ab(self, rhs);
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_84(
    size_t *zeta_i, Eurydice_arr_51 *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step_f5(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)3U));
    zeta_i[0U] = zeta_i[0U] - (size_t)3U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_84(
    size_t *zeta_i, Eurydice_arr_51 *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step_f5(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U));
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_3
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_84(
    size_t *zeta_i, Eurydice_arr_51 *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step_f5(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
  }
}

/**
A monomorphic instance of
libcrux_ml_kem.invert_ntt.inv_ntt_layer_int_vec_step_reduce with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_vector_avx2_SIMD256Vector_x2
libcrux_ml_kem_invert_ntt_inv_ntt_layer_int_vec_step_reduce_84(__m256i a,
                                                               __m256i b,
                                                               int16_t zeta_r) {
  __m256i a_minus_b = libcrux_ml_kem_vector_avx2_sub_f5(b, &a);
  a = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(
      libcrux_ml_kem_vector_avx2_add_f5(a, &b));
  b = libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(a_minus_b,
                                                                    zeta_r);
  return (KRML_CLITERAL(libcrux_ml_kem_vector_avx2_SIMD256Vector_x2){.fst = a,
                                                                     .snd = b});
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(size_t *zeta_i,
                                                        Eurydice_arr_51 *re,
                                                        size_t layer) {
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++) {
    size_t round = i0;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec =
        offset / LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR;
    size_t step_vec =
        step / LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++) {
      size_t j = i;
      libcrux_ml_kem_vector_avx2_SIMD256Vector_x2 uu____0 =
          libcrux_ml_kem_invert_ntt_inv_ntt_layer_int_vec_step_reduce_84(
              re->data[j], re->data[j + step_vec],
              libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
      __m256i x = uu____0.fst;
      __m256i y = uu____0.snd;
      re->data[j] = x;
      re->data[j + step_vec] = y;
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_montgomery
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_ab(
    Eurydice_arr_51 *re) {
  size_t zeta_i =
      LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)2U;
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re,
                                                          (size_t)4U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re,
                                                          (size_t)5U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re,
                                                          (size_t)6U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re,
                                                          (size_t)7U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.subtract_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_subtract_reduce_84(const Eurydice_arr_51 *myself,
                                             Eurydice_arr_51 b) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    __m256i coefficient_normal_form =
        libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(
            b.data[i0], (int16_t)1441);
    __m256i diff = libcrux_ml_kem_vector_avx2_sub_f5(myself->data[i0],
                                                     &coefficient_normal_form);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(diff);
    b.data[i0] = red;
  }
  return b;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.subtract_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_subtract_reduce_d6_84(const Eurydice_arr_51 *self,
                                                Eurydice_arr_51 b) {
  return libcrux_ml_kem_polynomial_subtract_reduce_84(self, b);
}

/**
 The following functions compute various expressions involving
 vectors and matrices. The computation of these expressions has been
 abstracted away into these functions in order to save on loop iterations.
 Compute v − InverseNTT(sᵀ ◦ NTT(u))
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_message
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51 libcrux_ml_kem_matrix_compute_message_ab(
    const Eurydice_arr_51 *v, const Eurydice_arr_260 *secret_as_ntt,
    const Eurydice_arr_260 *u_as_ntt) {
  Eurydice_arr_51 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_51 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(
        &secret_as_ntt->data[i0], &u_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_ab(&result, &product);
  }
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_ab(&result);
  return libcrux_ml_kem_polynomial_subtract_reduce_d6_84(v, result);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.to_unsigned_field_modulus
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(__m256i a) {
  return libcrux_ml_kem_vector_avx2_to_unsigned_representative_f5(a);
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_message with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_serialize_compress_then_serialize_message_84(
    Eurydice_arr_51 re) {
  Eurydice_arr_600 serialized = {.data = {0U}};
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    __m256i coefficient =
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re.data[i0]);
    __m256i coefficient_compressed =
        libcrux_ml_kem_vector_avx2_compress_1_f5(coefficient);
    Eurydice_array_u8x2 bytes =
        libcrux_ml_kem_vector_avx2_serialize_1_f5(coefficient_compressed);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_364(
            &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                             .start = (size_t)2U * i0,
                             .end = (size_t)2U * i0 + (size_t)2U})),
        Eurydice_array_to_slice_shared_26(&bytes), uint8_t);
  }
  return serialized;
}

/**
 This function implements <strong>Algorithm 14</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE decryption algorithm.

 Algorithm 14 is reproduced below:

 ```plaintext
 Input: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
 Input: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
 Output: message m ∈ 𝔹^{32}.

 c₁ ← c[0 : 32dᵤk]
 c₂ ← c[32dᵤk : 32(dᵤk + dᵥ)]
 u ← Decompress_{dᵤ}(ByteDecode_{dᵤ}(c₁))
 v ← Decompress_{dᵥ}(ByteDecode_{dᵥ}(c₂))
 ŝ ← ByteDecode₁₂(dkₚₖₑ)
 w ← v - NTT-¹(ŝᵀ ◦ NTT(u))
 m ← ByteEncode₁(Compress₁(w))
 return m
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt_unpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- VECTOR_U_ENCODED_SIZE= 960
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_ind_cpa_decrypt_unpacked_2f(const Eurydice_arr_260 *secret_key,
                                           const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_260 u_as_ntt =
      libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_ed(ciphertext);
  Eurydice_arr_51 v =
      libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_ed(
          Eurydice_array_to_subslice_from_shared_8c(ciphertext, (size_t)960U));
  Eurydice_arr_51 message =
      libcrux_ml_kem_matrix_compute_message_ab(&v, secret_key, &u_as_ntt);
  return libcrux_ml_kem_serialize_compress_then_serialize_message_84(message);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- VECTOR_U_ENCODED_SIZE= 960
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600 libcrux_ml_kem_ind_cpa_decrypt_2f(
    Eurydice_borrow_slice_u8 secret_key, const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_2f(&lvalue, i);
  }
  Eurydice_arr_260 secret_key_unpacked = arr_struct;
  libcrux_ml_kem_ind_cpa_deserialize_vector_ab(secret_key,
                                               &secret_key_unpacked);
  return libcrux_ml_kem_ind_cpa_decrypt_unpacked_2f(&secret_key_unpacked,
                                                    ciphertext);
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.G_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_06
libcrux_ml_kem_hash_functions_avx2_G_41_e0(Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_avx2_G(input);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF
with const generics
- LEN= 32
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_hash_functions_avx2_PRF_9e(Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_600 digest = {.data = {0U}};
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_6e(&digest),
                                 input);
  return digest;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 3
- LEN= 32
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_hash_functions_avx2_PRF_41_41(Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_avx2_PRF_9e(input);
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector[[$3size_t]] with const generics
- $3size_t
*/
typedef struct Eurydice_arr_aa0_s {
  Eurydice_arr_260 data[3U];
} Eurydice_arr_aa0;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63_s {
  Eurydice_arr_260 t_as_ntt;
  Eurydice_arr_600 seed_for_A;
  Eurydice_arr_aa0 A;
} libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63;

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_8b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
libcrux_ml_kem_ind_cpa_unpacked_default_8b_ab(void) {
  Eurydice_arr_260 uu____0;
  Eurydice_arr_51 repeat_expression0[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression0[i] = libcrux_ml_kem_polynomial_ZERO_d6_84();
  }
  memcpy(uu____0.data, repeat_expression0,
         (size_t)3U * sizeof(Eurydice_arr_51));
  Eurydice_arr_600 uu____1 = {.data = {0U}};
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63 lit0;
  lit0.t_as_ntt = uu____0;
  lit0.seed_for_A = uu____1;
  Eurydice_arr_260 repeat_expression1[3U];
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    Eurydice_arr_260 lit;
    Eurydice_arr_51 repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84();
    }
    memcpy(lit.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_51));
    repeat_expression1[i0] = lit;
  }
  memcpy(lit0.A.data, repeat_expression1,
         (size_t)3U * sizeof(Eurydice_arr_260));
  return lit0;
}

/**
 Only use with public values.

 This MUST NOT be used with secret inputs, like its caller
 `deserialize_ring_elements_reduced`.
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_to_reduced_ring_element with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_84(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_51 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_08){
            .start = i0 * (size_t)24U, .end = i0 * (size_t)24U + (size_t)24U}));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_12_f5(bytes);
    re.data[i0] = libcrux_ml_kem_vector_avx2_cond_subtract_3329_f5(coefficient);
  }
  return re;
}

/**
 See [deserialize_ring_elements_reduced_out].
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_ring_elements_reduced with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_ab(
    Eurydice_borrow_slice_u8 public_key, Eurydice_arr_260 *deserialized_pk) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(public_key, uint8_t) /
               LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT;
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 ring_element = Eurydice_slice_subslice_shared_7e(
        public_key,
        (KRML_CLITERAL(core_ops_range_Range_08){
            .start = i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            .end = i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT +
                   LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT}));
    Eurydice_arr_51 uu____0 =
        libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_84(
            ring_element);
    deserialized_pk->data[i0] = uu____0;
  }
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_05
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_e0(
    const Eurydice_arr_84 *input) {
  Eurydice_arr_05 state = libcrux_sha3_avx2_x4_incremental_init();
  libcrux_sha3_avx2_x4_incremental_shake128_absorb_final(
      &state, Eurydice_array_to_slice_shared_8d(input->data),
      Eurydice_array_to_slice_shared_8d(&input->data[1U]),
      Eurydice_array_to_slice_shared_8d(&input->data[2U]),
      Eurydice_array_to_slice_shared_8d(input->data));
  return state;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final_41 with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_05
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_e0(
    const Eurydice_arr_84 *input) {
  return libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_e0(
      input);
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks with
const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_35
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_e0(
    Eurydice_arr_05 *st) {
  Eurydice_arr_35 out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  Eurydice_arr_b0 out0 = {.data = {0U}};
  Eurydice_arr_b0 out1 = {.data = {0U}};
  Eurydice_arr_b0 out2 = {.data = {0U}};
  Eurydice_arr_b0 out3 = {.data = {0U}};
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_three_blocks(
      st, Eurydice_array_to_slice_mut_85(&out0),
      Eurydice_array_to_slice_mut_85(&out1),
      Eurydice_array_to_slice_mut_85(&out2),
      Eurydice_array_to_slice_mut_85(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks_41 with
const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_35
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_e0(
    Eurydice_arr_05 *self) {
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_e0(
      self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT
 representation of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm
 6</strong> of the NIST FIPS 203 standard, We say "partially" because this
 implementation only accepts a finite set of bytes as input and returns an error
 if the set is not enough; Algorithm 6 of the FIPS 203 standard on the other
 hand samples from an infinite stream of bytes until the ring element is filled.
 Algorithm 6 is reproduced below:

 ```plaintext
 Input: byte stream B ∈ 𝔹*.
 Output: array â ∈ ℤ₂₅₆.

 i ← 0
 j ← 0
 while j < 256 do
     d₁ ← B[i] + 256·(B[i+1] mod 16)
     d₂ ← ⌊B[i+1]/16⌋ + 16·B[i+2]
     if d₁ < q then
         â[j] ← d₁
         j ← j + 1
     end if
     if d₂ < q and j < 256 then
         â[j] ← d₂
         j ← j + 1
     end if
     i ← i + 3
 end while
 return â
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_uniform_distribution_next with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- N= 504
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ed(
    const Eurydice_arr_35 *randomness, Eurydice_arr_c8 *sampled_coefficients,
    Eurydice_arr_d4 *out) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)504U / (size_t)24U; i++) {
      size_t r = i;
      if (sampled_coefficients->data[i1] <
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        size_t sampled = libcrux_ml_kem_vector_avx2_rej_sample_f5(
            Eurydice_array_to_subslice_shared_360(
                &randomness->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_08){
                    .start = r * (size_t)24U,
                    .end = r * (size_t)24U + (size_t)24U})),
            Eurydice_array_to_subslice_mut_85(
                &out->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_08){
                    .start = sampled_coefficients->data[i1],
                    .end = sampled_coefficients->data[i1] + (size_t)16U})));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] =
            sampled_coefficients->data[uu____0] + sampled;
      }
    }
  }
  bool done = true;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >=
        LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
      sampled_coefficients->data[i0] =
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    } else {
      done = false;
    }
  }
  return done;
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_e0(
    Eurydice_arr_05 *st) {
  Eurydice_arr_d6 out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  Eurydice_arr_27 out0 = {.data = {0U}};
  Eurydice_arr_27 out1 = {.data = {0U}};
  Eurydice_arr_27 out2 = {.data = {0U}};
  Eurydice_arr_27 out3 = {.data = {0U}};
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_next_block(
      st, Eurydice_array_to_slice_mut_7b(&out0),
      Eurydice_array_to_slice_mut_7b(&out1),
      Eurydice_array_to_slice_mut_7b(&out2),
      Eurydice_array_to_slice_mut_7b(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block_41 with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_e0(
    Eurydice_arr_05 *self) {
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_e0(
      self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT
 representation of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm
 6</strong> of the NIST FIPS 203 standard, We say "partially" because this
 implementation only accepts a finite set of bytes as input and returns an error
 if the set is not enough; Algorithm 6 of the FIPS 203 standard on the other
 hand samples from an infinite stream of bytes until the ring element is filled.
 Algorithm 6 is reproduced below:

 ```plaintext
 Input: byte stream B ∈ 𝔹*.
 Output: array â ∈ ℤ₂₅₆.

 i ← 0
 j ← 0
 while j < 256 do
     d₁ ← B[i] + 256·(B[i+1] mod 16)
     d₂ ← ⌊B[i+1]/16⌋ + 16·B[i+2]
     if d₁ < q then
         â[j] ← d₁
         j ← j + 1
     end if
     if d₂ < q and j < 256 then
         â[j] ← d₂
         j ← j + 1
     end if
     i ← i + 3
 end while
 return â
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_uniform_distribution_next with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- N= 168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ed0(
    const Eurydice_arr_d6 *randomness, Eurydice_arr_c8 *sampled_coefficients,
    Eurydice_arr_d4 *out) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)24U; i++) {
      size_t r = i;
      if (sampled_coefficients->data[i1] <
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        size_t sampled = libcrux_ml_kem_vector_avx2_rej_sample_f5(
            Eurydice_array_to_subslice_shared_361(
                &randomness->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_08){
                    .start = r * (size_t)24U,
                    .end = r * (size_t)24U + (size_t)24U})),
            Eurydice_array_to_subslice_mut_85(
                &out->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_08){
                    .start = sampled_coefficients->data[i1],
                    .end = sampled_coefficients->data[i1] + (size_t)16U})));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] =
            sampled_coefficients->data[uu____0] + sampled;
      }
    }
  }
  bool done = true;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >=
        LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
      sampled_coefficients->data[i0] =
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    } else {
      done = false;
    }
  }
  return done;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.from_i16_array
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_from_i16_array_84(Eurydice_borrow_slice_i16 a) {
  Eurydice_arr_51 result = libcrux_ml_kem_polynomial_ZERO_84();
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    result.data[i0] = libcrux_ml_kem_vector_avx2_from_i16_array_f5(
        Eurydice_slice_subslice_shared_76(
            a, (KRML_CLITERAL(core_ops_range_Range_08){
                   .start = i0 * (size_t)16U,
                   .end = (i0 + (size_t)1U) * (size_t)16U})));
  }
  return result;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.from_i16_array_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_borrow_slice_i16 a) {
  return libcrux_ml_kem_polynomial_from_i16_array_84(a);
}

/**
This function found in impl {core::ops::function::FnMut<(@Array<i16, 272usize>),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector,
Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_e7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_sampling_sample_from_xof_call_mut_e7_6c(
    void **_, Eurydice_arr_a00 tupled_args) {
  Eurydice_arr_a00 s = tupled_args;
  return libcrux_ml_kem_polynomial_from_i16_array_d6_84(
      Eurydice_array_to_subslice_shared_850(
          &s, (KRML_CLITERAL(core_ops_range_Range_08){.start = (size_t)0U,
                                                      .end = (size_t)256U})));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_260
libcrux_ml_kem_sampling_sample_from_xof_6c(const Eurydice_arr_84 *seeds) {
  Eurydice_arr_c8 sampled_coefficients = {.data = {0U}};
  Eurydice_arr_d4 out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  Eurydice_arr_05 xof_state =
      libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_e0(
          seeds);
  Eurydice_arr_35 randomness0 =
      libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_e0(
          &xof_state);
  bool done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ed(
      &randomness0, &sampled_coefficients, &out);
  while (true) {
    if (done) {
      break;
    } else {
      Eurydice_arr_d6 randomness =
          libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_e0(
              &xof_state);
      done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ed0(
          &randomness, &sampled_coefficients, &out);
    }
  }
  Eurydice_arr_260 arr_mapped_str;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
        libcrux_ml_kem_sampling_sample_from_xof_call_mut_e7_6c(&lvalue,
                                                               out.data[i]);
  }
  return arr_mapped_str;
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector, size_t

*/
typedef struct Eurydice_dst_ref_shared_5a_s {
  const Eurydice_arr_51 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_5a;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- N= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_5a Eurydice_array_to_slice_shared_cf0(
    const Eurydice_arr_260 *a) {
  Eurydice_dst_ref_shared_5a lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_sample_matrix_A_6c(
    Eurydice_arr_aa0 *A_transpose, const Eurydice_arr_48 *seed,
    bool transpose) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    Eurydice_arr_84 seeds;
    Eurydice_arr_48 repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] =
          core_array__core__clone__Clone_for__Array_T__N___clone(
              (size_t)34U, seed, uint8_t, Eurydice_arr_48);
    }
    memcpy(seeds.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_48));
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;
    }
    Eurydice_arr_260 sampled =
        libcrux_ml_kem_sampling_sample_from_xof_6c(&seeds);
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf0(&sampled),
                                Eurydice_arr_51);
         i++) {
      size_t j = i;
      Eurydice_arr_51 sample = sampled.data[j];
      if (transpose) {
        A_transpose->data[j].data[i1] = sample;
      } else {
        A_transpose->data[i1].data[j] = sample;
      }
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_fa(
    Eurydice_borrow_slice_u8 public_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
        *unpacked_public_key) {
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_ab(
      Eurydice_slice_subslice_to_shared_c6(public_key, (size_t)1152U),
      &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8 seed =
      Eurydice_slice_subslice_from_shared_6b(public_key, (size_t)1152U);
  Eurydice_arr_aa0 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue = libcrux_ml_kem_utils_into_padded_array_b6(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_6c(uu____0, &lvalue, false);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
    libcrux_ml_kem_ind_cpa_build_unpacked_public_key_fa(
        Eurydice_borrow_slice_u8 public_key) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
      unpacked_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_ab();
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_fa(public_key,
                                                          &unpacked_public_key);
  return unpacked_public_key;
}

/**
A monomorphic instance of K.
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector[[$3size_t]],
libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector

*/
typedef struct tuple_880_s {
  Eurydice_arr_260 fst;
  Eurydice_arr_51 snd;
} tuple_880;

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure<Vector, Hasher,
K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2,
ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2,
TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_f1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- C1_LEN= 960
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51 libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_48(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN
with const generics
- K= 3
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_db
libcrux_ml_kem_hash_functions_avx2_PRFxN_41(const Eurydice_arr_46 *input) {
  Eurydice_arr_db out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  Eurydice_arr_d1 out0 = {.data = {0U}};
  Eurydice_arr_d1 out1 = {.data = {0U}};
  Eurydice_arr_d1 out2 = {.data = {0U}};
  Eurydice_arr_d1 out3 = {.data = {0U}};
  libcrux_sha3_avx2_x4_shake256(
      Eurydice_array_to_slice_shared_61(input->data),
      Eurydice_array_to_slice_shared_61(&input->data[1U]),
      Eurydice_array_to_slice_shared_61(&input->data[2U]),
      Eurydice_array_to_slice_shared_61(input->data),
      Eurydice_array_to_slice_mut_18(&out0),
      Eurydice_array_to_slice_mut_18(&out1),
      Eurydice_array_to_slice_mut_18(&out2),
      Eurydice_array_to_slice_mut_18(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN_41
with const generics
- K= 3
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_db
libcrux_ml_kem_hash_functions_avx2_PRFxN_41_41(const Eurydice_arr_46 *input) {
  return libcrux_ml_kem_hash_functions_avx2_PRFxN_41(input);
}

/**
 Given a series of uniformly random bytes in `randomness`, for some number
 `eta`, the `sample_from_binomial_distribution_{eta}` functions sample a ring
 element from a binomial distribution centered at 0 that uses two sets of `eta`
 coin flips. If, for example, `eta = ETA`, each ring coefficient is a value `v`
 such such that `v ∈ {-ETA, -ETA + 1, ..., 0, ..., ETA + 1, ETA}` and:

 ```plaintext
 - If v < 0, Pr[v] = Pr[-v]
 - If v >= 0, Pr[v] = BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2 ^ (2 * ETA)
 ```

 The values `v < 0` are mapped to the appropriate `KyberFieldElement`.

 The expected value is:

 ```plaintext
 E[X] = (-ETA)Pr[-ETA] + (-(ETA - 1))Pr[-(ETA - 1)] + ... + (ETA - 1)Pr[ETA - 1]
 + (ETA)Pr[ETA] = 0 since Pr[-v] = Pr[v] when v < 0.
 ```

 And the variance is:

 ```plaintext
 Var(X) = E[(X - E[X])^2]
        = E[X^2]
        = sum_(v=-ETA to ETA)v^2 * (BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) /
 2^(2 * ETA)) = ETA / 2
 ```

 This function implements <strong>Algorithm 7</strong> of the NIST FIPS 203
 standard, which is reproduced below:

 ```plaintext
 Input: byte array B ∈ 𝔹^{64η}.
 Output: array f ∈ ℤ₂₅₆.

 b ← BytesToBits(B)
 for (i ← 0; i < 256; i++)
     x ← ∑(j=0 to η - 1) b[2iη + j]
     y ← ∑(j=0 to η - 1) b[2iη + η + j]
     f[i] ← x−y mod q
 end for
 return f
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_binomial_distribution_2 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_84(
    Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_c1 sampled_i16s = {.data = {0U}};
  for (size_t i0 = (size_t)0U;
       i0 < Eurydice_slice_len(randomness, uint8_t) / (size_t)4U; i0++) {
    size_t chunk_number = i0;
    Eurydice_borrow_slice_u8 byte_chunk = Eurydice_slice_subslice_shared_7e(
        randomness, (KRML_CLITERAL(core_ops_range_Range_08){
                        .start = chunk_number * (size_t)4U,
                        .end = chunk_number * (size_t)4U + (size_t)4U}));
    uint32_t random_bits_as_u32 =
        (((uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)0U,
                                                uint8_t) |
          (uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)1U, uint8_t)
              << 8U) |
         (uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)2U, uint8_t)
             << 16U) |
        (uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)3U, uint8_t)
            << 24U;
    uint32_t even_bits = random_bits_as_u32 & 1431655765U;
    uint32_t odd_bits = random_bits_as_u32 >> 1U & 1431655765U;
    uint32_t coin_toss_outcomes = even_bits + odd_bits;
    for (uint32_t i = 0U; i < 32U / 4U; i++) {
      uint32_t outcome_set = i;
      uint32_t outcome_set0 = outcome_set * 4U;
      int16_t outcome_1 =
          (int16_t)(coin_toss_outcomes >> (uint32_t)outcome_set0 & 3U);
      int16_t outcome_2 =
          (int16_t)(coin_toss_outcomes >> (uint32_t)(outcome_set0 + 2U) & 3U);
      size_t offset = (size_t)(outcome_set0 >> 2U);
      sampled_i16s.data[(size_t)8U * chunk_number + offset] =
          outcome_1 - outcome_2;
    }
  }
  return libcrux_ml_kem_polynomial_from_i16_array_d6_84(
      Eurydice_array_to_slice_shared_1a(&sampled_i16s));
}

/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_binomial_distribution with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- ETA= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_sampling_sample_from_binomial_distribution_89(
    Eurydice_borrow_slice_u8 randomness) {
  return libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_84(
      randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_7_84(
    Eurydice_arr_51 *re) {
  size_t step = LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT / (size_t)2U;
  for (size_t i = (size_t)0U; i < step; i++) {
    size_t j = i;
    __m256i t = libcrux_ml_kem_vector_avx2_multiply_by_constant_f5(
        re->data[j + step], (int16_t)-1600);
    re->data[j + step] = libcrux_ml_kem_vector_avx2_sub_f5(re->data[j], &t);
    re->data[j] = libcrux_ml_kem_vector_avx2_add_f5(re->data[j], &t);
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_binomially_sampled_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_84(Eurydice_arr_51 *re) {
  libcrux_ml_kem_ntt_ntt_at_layer_7_84(re);
  size_t zeta_i = (size_t)1U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U,
                                            (size_t)11207U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U,
                                            (size_t)11207U + (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(
      &zeta_i, re, (size_t)4U, (size_t)11207U + (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_3_84(
      &zeta_i, re, (size_t)11207U + (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_2_84(
      &zeta_i, re, (size_t)11207U + (size_t)4U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_1_84(
      &zeta_i, re, (size_t)11207U + (size_t)5U * (size_t)3328U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
 Sample a vector of ring elements from a centered binomial distribution and
 convert them into their NTT representations.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_vector_cbd_then_ntt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- ETA= 2
- ETA_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_b4(
    Eurydice_arr_260 *re_as_ntt, const Eurydice_arr_3e *prf_input,
    uint8_t domain_separator) {
  Eurydice_arr_46 prf_inputs;
  Eurydice_arr_3e repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)33U, prf_input, uint8_t, Eurydice_arr_3e);
  }
  memcpy(prf_inputs.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_3e));
  domain_separator =
      libcrux_ml_kem_utils_prf_input_inc_e0(&prf_inputs, domain_separator);
  Eurydice_arr_db prf_outputs =
      libcrux_ml_kem_hash_functions_avx2_PRFxN_41_41(&prf_inputs);
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_51 uu____0 =
        libcrux_ml_kem_sampling_sample_from_binomial_distribution_89(
            Eurydice_array_to_slice_shared_18(&prf_outputs.data[i0]));
    re_as_ntt->data[i0] = uu____0;
    libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_84(
        &re_as_ntt->data[i0]);
  }
  return domain_separator;
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure#1<Vector,
Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE,
ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2,
TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- C1_LEN= 960
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51 libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_48(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Sample a vector of ring elements from a centered binomial distribution.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_ring_element_cbd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- ETA2_RANDOMNESS_SIZE= 128
- ETA2= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_b4(
    const Eurydice_arr_3e *prf_input, uint8_t domain_separator,
    Eurydice_arr_260 *error_1) {
  Eurydice_arr_46 prf_inputs;
  Eurydice_arr_3e repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)33U, prf_input, uint8_t, Eurydice_arr_3e);
  }
  memcpy(prf_inputs.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_3e));
  domain_separator =
      libcrux_ml_kem_utils_prf_input_inc_e0(&prf_inputs, domain_separator);
  Eurydice_arr_db prf_outputs =
      libcrux_ml_kem_hash_functions_avx2_PRFxN_41_41(&prf_inputs);
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_51 uu____0 =
        libcrux_ml_kem_sampling_sample_from_binomial_distribution_89(
            Eurydice_array_to_slice_shared_18(&prf_outputs.data[i0]));
    error_1->data[i0] = uu____0;
  }
  return domain_separator;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF
with const generics
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_hash_functions_avx2_PRF_a6(Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_d1 digest = {.data = {0U}};
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_18(&digest),
                                 input);
  return digest;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 3
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_hash_functions_avx2_PRF_41_410(Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_avx2_PRF_a6(input);
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for libcrux_ml_kem::matrix::compute_vector_u::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u.call_mut_a8
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_ab(void **_,
                                                      size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector[[$3size_t]], size_t

*/
typedef struct Eurydice_dst_ref_shared_940_s {
  const Eurydice_arr_260 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_940;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_avx2_SIMD256Vector[[$3size_t]] with const generics
- N= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_dst_ref_shared_940 Eurydice_array_to_slice_shared_b50(
    const Eurydice_arr_aa0 *a) {
  Eurydice_dst_ref_shared_940 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_error_reduce_84(
    Eurydice_arr_51 *myself, const Eurydice_arr_51 *error) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t j = i;
    __m256i coefficient_normal_form =
        libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(
            myself->data[j], (int16_t)1441);
    __m256i sum = libcrux_ml_kem_vector_avx2_add_f5(coefficient_normal_form,
                                                    &error->data[j]);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(sum);
    myself->data[j] = red;
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_error_reduce_d6_84(
    Eurydice_arr_51 *self, const Eurydice_arr_51 *error) {
  libcrux_ml_kem_polynomial_add_error_reduce_84(self, error);
}

/**
 Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_260
libcrux_ml_kem_matrix_compute_vector_u_ab(const Eurydice_arr_aa0 *a_as_ntt,
                                          const Eurydice_arr_260 *r_as_ntt,
                                          const Eurydice_arr_260 *error_1) {
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_ab(&lvalue, i);
  }
  Eurydice_arr_260 result = arr_struct;
  for (size_t i0 = (size_t)0U;
       i0 < Eurydice_slice_len(Eurydice_array_to_slice_shared_b50(a_as_ntt),
                               Eurydice_arr_260);
       i0++) {
    size_t i1 = i0;
    const Eurydice_arr_260 *row = &a_as_ntt->data[i1];
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf0(row),
                                Eurydice_arr_51);
         i++) {
      size_t j = i;
      const Eurydice_arr_51 *a_element = &row->data[j];
      Eurydice_arr_51 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(
          a_element, &r_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_ab(&result.data[i1],
                                                          &product);
    }
    libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_ab(&result.data[i1]);
    libcrux_ml_kem_polynomial_add_error_reduce_d6_84(&result.data[i1],
                                                     &error_1->data[i1]);
  }
  return result;
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.avx2.compress.compress_ciphertext_coefficient with const
generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_ef(
    __m256i vector) {
  __m256i field_modulus_halved = libcrux_intrinsics_avx2_mm256_set1_epi32(
      ((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS - (int32_t)1) /
      (int32_t)2);
  __m256i compression_factor =
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)10321340);
  __m256i coefficient_bits_mask = libcrux_intrinsics_avx2_mm256_set1_epi32(
      ((int32_t)1 << (uint32_t)(int32_t)10) - (int32_t)1);
  __m128i coefficients_low =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i compressed_low = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)10, coefficients_low0, __m256i);
  __m256i compressed_low0 = libcrux_intrinsics_avx2_mm256_add_epi32(
      compressed_low, field_modulus_halved);
  __m256i compressed_low1 =
      libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_low0,
                                                            compression_factor);
  __m256i compressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)3, compressed_low1, __m256i);
  __m256i compressed_low3 = libcrux_intrinsics_avx2_mm256_and_si256(
      compressed_low2, coefficient_bits_mask);
  __m128i coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, vector, __m128i);
  __m256i coefficients_high0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i compressed_high = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)10, coefficients_high0, __m256i);
  __m256i compressed_high0 = libcrux_intrinsics_avx2_mm256_add_epi32(
      compressed_high, field_modulus_halved);
  __m256i compressed_high1 =
      libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_high0,
                                                            compression_factor);
  __m256i compressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)3, compressed_high1, __m256i);
  __m256i compressed_high3 = libcrux_intrinsics_avx2_mm256_and_si256(
      compressed_high2, coefficient_bits_mask);
  __m256i compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(
      compressed_low3, compressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64((int32_t)216,
                                                         compressed, __m256i);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_ef(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_ef(
      vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress_f5
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f5_ef(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_ef(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_10
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- OUT_LEN= 320
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b7
libcrux_ml_kem_serialize_compress_then_serialize_10_0e(
    const Eurydice_arr_51 *re) {
  Eurydice_arr_b7 serialized = {.data = {0U}};
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    __m256i coefficient = libcrux_ml_kem_vector_avx2_compress_f5_ef(
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re->data[i0]));
    Eurydice_arr_dc bytes =
        libcrux_ml_kem_vector_avx2_serialize_10_f5(coefficient);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_369(
            &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                             .start = (size_t)20U * i0,
                             .end = (size_t)20U * i0 + (size_t)20U})),
        Eurydice_array_to_slice_shared_c2(&bytes), uint8_t);
  }
  return serialized;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_ring_element_u with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- COMPRESSION_FACTOR= 10
- OUT_LEN= 320
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b7
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_a4(
    const Eurydice_arr_51 *re) {
  return libcrux_ml_kem_serialize_compress_then_serialize_10_0e(re);
}

/**
 Call [`compress_then_serialize_ring_element_u`] on each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.compress_then_serialize_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- OUT_LEN= 960
- COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_compress_then_serialize_u_8c(
    Eurydice_arr_260 input, Eurydice_mut_borrow_slice_u8 out) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf0(&input),
                              Eurydice_arr_51);
       i++) {
    size_t i0 = i;
    Eurydice_arr_51 re = input.data[i0];
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_7e(
        out, (KRML_CLITERAL(core_ops_range_Range_08){
                 .start = i0 * ((size_t)960U / (size_t)3U),
                 .end = (i0 + (size_t)1U) * ((size_t)960U / (size_t)3U)}));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b7 lvalue =
        libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_a4(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_d3(&lvalue),
                        uint8_t);
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- C1_LEN= 960
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_880 libcrux_ml_kem_ind_cpa_encrypt_c1_48(
    Eurydice_borrow_slice_u8 randomness, const Eurydice_arr_aa0 *matrix,
    Eurydice_mut_borrow_slice_u8 ciphertext) {
  Eurydice_arr_3e prf_input =
      libcrux_ml_kem_utils_into_padded_array_c8(randomness);
  Eurydice_arr_260 arr_struct0;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct0.data[i] =
        libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_48(&lvalue, i);
  }
  Eurydice_arr_260 r_as_ntt = arr_struct0;
  uint8_t domain_separator0 =
      libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_b4(&r_as_ntt,
                                                           &prf_input, 0U);
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_48(&lvalue, i);
  }
  Eurydice_arr_260 error_1 = arr_struct;
  uint8_t domain_separator = libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_b4(
      &prf_input, domain_separator0, &error_1);
  prf_input.data[32U] = domain_separator;
  Eurydice_arr_d1 prf_output = libcrux_ml_kem_hash_functions_avx2_PRF_41_410(
      Eurydice_array_to_slice_shared_61(&prf_input));
  Eurydice_arr_51 error_2 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_89(
          Eurydice_array_to_slice_shared_18(&prf_output));
  Eurydice_arr_260 u =
      libcrux_ml_kem_matrix_compute_vector_u_ab(matrix, &r_as_ntt, &error_1);
  libcrux_ml_kem_ind_cpa_compress_then_serialize_u_8c(u, ciphertext);
  return (KRML_CLITERAL(tuple_880){.fst = r_as_ntt, .snd = error_2});
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_message with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_then_decompress_message_84(
    const Eurydice_arr_600 *serialized) {
  Eurydice_arr_51 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    __m256i coefficient_compressed =
        libcrux_ml_kem_vector_avx2_deserialize_1_f5(
            Eurydice_array_to_subslice_shared_362(
                serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                                .start = (size_t)2U * i0,
                                .end = (size_t)2U * i0 + (size_t)2U})));
    re.data[i0] =
        libcrux_ml_kem_vector_avx2_decompress_1_f5(coefficient_compressed);
  }
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_message_error_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_add_message_error_reduce_84(
    const Eurydice_arr_51 *myself, const Eurydice_arr_51 *message,
    Eurydice_arr_51 result) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    __m256i coefficient_normal_form =
        libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(
            result.data[i0], (int16_t)1441);
    __m256i sum1 =
        libcrux_ml_kem_vector_avx2_add_f5(myself->data[i0], &message->data[i0]);
    __m256i sum2 =
        libcrux_ml_kem_vector_avx2_add_f5(coefficient_normal_form, &sum1);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(sum2);
    result.data[i0] = red;
  }
  return result;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_message_error_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_polynomial_add_message_error_reduce_d6_84(
    const Eurydice_arr_51 *self, const Eurydice_arr_51 *message,
    Eurydice_arr_51 result) {
  return libcrux_ml_kem_polynomial_add_message_error_reduce_84(self, message,
                                                               result);
}

/**
 Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_51
libcrux_ml_kem_matrix_compute_ring_element_v_ab(
    const Eurydice_arr_260 *t_as_ntt, const Eurydice_arr_260 *r_as_ntt,
    const Eurydice_arr_51 *error_2, const Eurydice_arr_51 *message) {
  Eurydice_arr_51 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_51 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(
        &t_as_ntt->data[i0], &r_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_ab(&result, &product);
  }
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_ab(&result);
  return libcrux_ml_kem_polynomial_add_message_error_reduce_d6_84(
      error_2, message, result);
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.avx2.compress.compress_ciphertext_coefficient with const
generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_d1(
    __m256i vector) {
  __m256i field_modulus_halved = libcrux_intrinsics_avx2_mm256_set1_epi32(
      ((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS - (int32_t)1) /
      (int32_t)2);
  __m256i compression_factor =
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)10321340);
  __m256i coefficient_bits_mask = libcrux_intrinsics_avx2_mm256_set1_epi32(
      ((int32_t)1 << (uint32_t)(int32_t)4) - (int32_t)1);
  __m128i coefficients_low =
      libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i compressed_low = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)4, coefficients_low0, __m256i);
  __m256i compressed_low0 = libcrux_intrinsics_avx2_mm256_add_epi32(
      compressed_low, field_modulus_halved);
  __m256i compressed_low1 =
      libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_low0,
                                                            compression_factor);
  __m256i compressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)3, compressed_low1, __m256i);
  __m256i compressed_low3 = libcrux_intrinsics_avx2_mm256_and_si256(
      compressed_low2, coefficient_bits_mask);
  __m128i coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(
      (int32_t)1, vector, __m128i);
  __m256i coefficients_high0 =
      libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i compressed_high = libcrux_intrinsics_avx2_mm256_slli_epi32(
      (int32_t)4, coefficients_high0, __m256i);
  __m256i compressed_high0 = libcrux_intrinsics_avx2_mm256_add_epi32(
      compressed_high, field_modulus_halved);
  __m256i compressed_high1 =
      libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_high0,
                                                            compression_factor);
  __m256i compressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(
      (int32_t)3, compressed_high1, __m256i);
  __m256i compressed_high3 = libcrux_intrinsics_avx2_mm256_and_si256(
      compressed_high2, coefficient_bits_mask);
  __m256i compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(
      compressed_low3, compressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64((int32_t)216,
                                                         compressed, __m256i);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_d1(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_d1(
      vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress_f5
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f5_d1(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_compress_d1(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_4
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_4_84(
    Eurydice_arr_51 re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    __m256i coefficient = libcrux_ml_kem_vector_avx2_compress_f5_d1(
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re.data[i0]));
    Eurydice_array_u8x8 bytes =
        libcrux_ml_kem_vector_avx2_serialize_4_f5(coefficient);
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_7e(
            serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                            .start = (size_t)8U * i0,
                            .end = (size_t)8U * i0 + (size_t)8U})),
        Eurydice_array_to_slice_shared_41(&bytes), uint8_t);
  }
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_ring_element_v with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- COMPRESSION_FACTOR= 4
- OUT_LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_ed(
    Eurydice_arr_51 re, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_kem_serialize_compress_then_serialize_4_84(re, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- V_COMPRESSION_FACTOR= 4
- C2_LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_encrypt_c2_ed(
    const Eurydice_arr_260 *t_as_ntt, const Eurydice_arr_260 *r_as_ntt,
    const Eurydice_arr_51 *error_2, const Eurydice_arr_600 *message,
    Eurydice_mut_borrow_slice_u8 ciphertext) {
  Eurydice_arr_51 message_as_ring_element =
      libcrux_ml_kem_serialize_deserialize_then_decompress_message_84(message);
  Eurydice_arr_51 v = libcrux_ml_kem_matrix_compute_ring_element_v_ab(
      t_as_ntt, r_as_ntt, error_2, &message_as_ring_element);
  libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_ed(
      v, ciphertext);
}

/**
 This function implements <strong>Algorithm 13</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE encryption algorithm.

 Algorithm 13 is reproduced below:

 ```plaintext
 Input: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
 Input: message m ∈ 𝔹^{32}.
 Input: encryption randomness r ∈ 𝔹^{32}.
 Output: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.

 N ← 0
 t̂ ← ByteDecode₁₂(ekₚₖₑ[0:384k])
 ρ ← ekₚₖₑ[384k: 384k + 32]
 for (i ← 0; i < k; i++)
     for(j ← 0; j < k; j++)
         Â[i,j] ← SampleNTT(XOF(ρ, i, j))
     end for
 end for
 for(i ← 0; i < k; i++)
     r[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(r,N))
     N ← N + 1
 end for
 for(i ← 0; i < k; i++)
     e₁[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
     N ← N + 1
 end for
 e₂ ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
 r̂ ← NTT(r)
 u ← NTT-¹(Âᵀ ◦ r̂) + e₁
 μ ← Decompress₁(ByteDecode₁(m)))
 v ← NTT-¹(t̂ᵀ ◦ rˆ) + e₂ + μ
 c₁ ← ByteEncode_{dᵤ}(Compress_{dᵤ}(u))
 c₂ ← ByteEncode_{dᵥ}(Compress_{dᵥ}(v))
 return c ← (c₁ ‖ c₂)
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_unpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_LEN= 960
- C2_LEN= 128
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_2c
libcrux_ml_kem_ind_cpa_encrypt_unpacked_74(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
        *public_key,
    const Eurydice_arr_600 *message, Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_2c ciphertext = {.data = {0U}};
  tuple_880 uu____0 = libcrux_ml_kem_ind_cpa_encrypt_c1_48(
      randomness, &public_key->A,
      Eurydice_array_to_subslice_mut_3610(
          &ciphertext, (KRML_CLITERAL(core_ops_range_Range_08){
                           .start = (size_t)0U, .end = (size_t)960U})));
  Eurydice_arr_260 r_as_ntt = uu____0.fst;
  Eurydice_arr_51 error_2 = uu____0.snd;
  libcrux_ml_kem_ind_cpa_encrypt_c2_ed(
      &public_key->t_as_ntt, &r_as_ntt, &error_2, message,
      Eurydice_array_to_subslice_from_mut_8c1(&ciphertext, (size_t)960U));
  return ciphertext;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_LEN= 960
- C2_LEN= 128
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_2c libcrux_ml_kem_ind_cpa_encrypt_74(
    Eurydice_borrow_slice_u8 public_key, const Eurydice_arr_600 *message,
    Eurydice_borrow_slice_u8 randomness) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
      unpacked_public_key =
          libcrux_ml_kem_ind_cpa_build_unpacked_public_key_fa(public_key);
  return libcrux_ml_kem_ind_cpa_encrypt_unpacked_74(&unpacked_public_key,
                                                    message, randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for
libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.kdf_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600 libcrux_ml_kem_variant_kdf_39_ae(
    Eurydice_borrow_slice_u8 shared_secret, const Eurydice_arr_2c *_) {
  Eurydice_arr_600 out = {.data = {0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(&out), shared_secret,
                      uint8_t);
  return out;
}

/**
 This code verifies on some machines, runs out of memory on others
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600 libcrux_ml_kem_ind_cca_decapsulate_a1(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x4 uu____0 =
      libcrux_ml_kem_types_unpack_private_key_b4(
          Eurydice_array_to_slice_shared_ec(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  Eurydice_arr_600 decrypted =
      libcrux_ml_kem_ind_cpa_decrypt_2f(ind_cpa_secret_key, ciphertext);
  Eurydice_arr_06 to_hash0 = libcrux_ml_kem_utils_into_padded_array_24(
      Eurydice_array_to_slice_shared_6e(&decrypted));
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_from_mut_8c(
          &to_hash0, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE),
      ind_cpa_public_key_hash, uint8_t);
  Eurydice_arr_06 hashed = libcrux_ml_kem_hash_functions_avx2_G_41_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash0));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret0 = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_480 to_hash =
      libcrux_ml_kem_utils_into_padded_array_15(implicit_rejection_value);
  Eurydice_mut_borrow_slice_u8 uu____2 =
      Eurydice_array_to_subslice_from_mut_8c0(
          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_d3_80(ciphertext),
                      uint8_t);
  Eurydice_arr_600 implicit_rejection_shared_secret =
      libcrux_ml_kem_hash_functions_avx2_PRF_41_41(
          Eurydice_array_to_slice_shared_74(&to_hash));
  Eurydice_arr_2c expected_ciphertext = libcrux_ml_kem_ind_cpa_encrypt_74(
      ind_cpa_public_key, &decrypted, pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 =
      Eurydice_array_to_slice_shared_6e(&implicit_rejection_shared_secret);
  Eurydice_arr_600 implicit_rejection_shared_secret0 =
      libcrux_ml_kem_variant_kdf_39_ae(
          uu____3, libcrux_ml_kem_types_as_slice_a9_80(ciphertext));
  Eurydice_arr_600 shared_secret = libcrux_ml_kem_variant_kdf_39_ae(
      shared_secret0, libcrux_ml_kem_types_as_slice_a9_80(ciphertext));
  Eurydice_borrow_slice_u8 uu____4 =
      libcrux_ml_kem_types_as_ref_d3_80(ciphertext);
  return libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(
      uu____4, Eurydice_array_to_slice_shared_42(&expected_ciphertext),
      Eurydice_array_to_slice_shared_6e(&shared_secret),
      Eurydice_array_to_slice_shared_6e(&implicit_rejection_shared_secret0));
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2 with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_35(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_decapsulate_a1(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_35(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_35(
      private_key, ciphertext);
}

/**
 Decapsulate ML-KEM 768

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem768PrivateKey`] and an
 [`MlKem768Ciphertext`].
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600 libcrux_ml_kem_mlkem768_avx2_decapsulate(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_35(private_key,
                                                                   ciphertext);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for
libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.entropy_preprocess_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_variant_entropy_preprocess_39_be(
    Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_600 out = {.data = {0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(&out), randomness,
                      uint8_t);
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.H_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_hash_functions_avx2_H_41_e0(Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_avx2_H(input);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_56 libcrux_ml_kem_ind_cca_encapsulate_70(
    const Eurydice_arr_74 *public_key, const Eurydice_arr_600 *randomness) {
  Eurydice_arr_600 randomness0 =
      libcrux_ml_kem_variant_entropy_preprocess_39_be(
          Eurydice_array_to_slice_shared_6e(randomness));
  Eurydice_arr_06 to_hash = libcrux_ml_kem_utils_into_padded_array_24(
      Eurydice_array_to_slice_shared_6e(&randomness0));
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_from_mut_8c(
      &to_hash, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE);
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_600 lvalue = libcrux_ml_kem_hash_functions_avx2_H_41_e0(
      Eurydice_array_to_slice_shared_45(
          libcrux_ml_kem_types_as_slice_e6_d0(public_key)));
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_6e(&lvalue),
                      uint8_t);
  Eurydice_arr_06 hashed = libcrux_ml_kem_hash_functions_avx2_G_41_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_2c ciphertext = libcrux_ml_kem_ind_cpa_encrypt_74(
      Eurydice_array_to_slice_shared_45(
          libcrux_ml_kem_types_as_slice_e6_d0(public_key)),
      &randomness0, pseudorandomness);
  Eurydice_arr_2c uu____2 = libcrux_ml_kem_types_from_e0_80(ciphertext);
  return (KRML_CLITERAL(tuple_56){
      .fst = uu____2,
      .snd = libcrux_ml_kem_variant_kdf_39_ae(shared_secret, &ciphertext)});
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2 with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_56
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_cd(
    const Eurydice_arr_74 *public_key, const Eurydice_arr_600 *randomness) {
  return libcrux_ml_kem_ind_cca_encapsulate_70(public_key, randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_56
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_cd(
    const Eurydice_arr_74 *public_key, const Eurydice_arr_600 *randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_cd(
      public_key, randomness);
}

/**
 Encapsulate ML-KEM 768

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_56 libcrux_ml_kem_mlkem768_avx2_encapsulate(
    const Eurydice_arr_74 *public_key, Eurydice_arr_600 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_cd(public_key,
                                                                   &randomness);
}

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cpa::unpacked::IndCpaPrivateKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_70
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_260 libcrux_ml_kem_ind_cpa_unpacked_default_70_ab(
    void) {
  Eurydice_arr_260 lit;
  Eurydice_arr_51 repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84();
  }
  memcpy(lit.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_51));
  return lit;
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for
libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.cpa_keygen_seed_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_06
libcrux_ml_kem_variant_cpa_keygen_seed_39_be(
    Eurydice_borrow_slice_u8 key_generation_seed) {
  Eurydice_arr_3e seed = {.data = {0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_368(
          &seed,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = (size_t)0U,
              .end =
                  LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE})),
      key_generation_seed, uint8_t);
  seed.data[LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE] =
      (uint8_t)(size_t)3U;
  return libcrux_ml_kem_hash_functions_avx2_G_41_e0(
      Eurydice_array_to_slice_shared_61(&seed));
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@3]> for
libcrux_ml_kem::ind_cpa::generate_keypair_unpacked::closure<Vector, Hasher,
Scheme, K, ETA1, ETA1_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1,
TraitClause@2, TraitClause@3, TraitClause@4, TraitClause@5]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.generate_keypair_unpacked.call_mut_73 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_22(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.to_standard_domain
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_polynomial_to_standard_domain_84(__m256i vector) {
  return libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(
      vector,
      LIBCRUX_ML_KEM_VECTOR_TRAITS_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_standard_error_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_standard_error_reduce_84(
    Eurydice_arr_51 *myself, const Eurydice_arr_51 *error) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t j = i;
    __m256i coefficient_normal_form =
        libcrux_ml_kem_polynomial_to_standard_domain_84(myself->data[j]);
    __m256i sum = libcrux_ml_kem_vector_avx2_add_f5(coefficient_normal_form,
                                                    &error->data[j]);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(sum);
    myself->data[j] = red;
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_standard_error_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_84(
    Eurydice_arr_51 *self, const Eurydice_arr_51 *error) {
  libcrux_ml_kem_polynomial_add_standard_error_reduce_84(self, error);
}

/**
 Compute Â ◦ ŝ + ê
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_As_plus_e
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_compute_As_plus_e_ab(
    Eurydice_arr_260 *t_as_ntt, const Eurydice_arr_aa0 *matrix_A,
    const Eurydice_arr_260 *s_as_ntt, const Eurydice_arr_260 *error_as_ntt) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_b50(matrix_A),
                              Eurydice_arr_260);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_260 *row = &matrix_A->data[i0];
    Eurydice_arr_51 uu____0 = libcrux_ml_kem_polynomial_ZERO_d6_84();
    t_as_ntt->data[i0] = uu____0;
    for (size_t i1 = (size_t)0U;
         i1 < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf0(row),
                                 Eurydice_arr_51);
         i1++) {
      size_t j = i1;
      const Eurydice_arr_51 *matrix_element = &row->data[j];
      Eurydice_arr_51 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(
          matrix_element, &s_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_ab(&t_as_ntt->data[i0],
                                                          &product);
    }
    libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_84(
        &t_as_ntt->data[i0], &error_as_ntt->data[i0]);
  }
}

/**
 This function implements most of <strong>Algorithm 12</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE key generation
 algorithm.

 We say "most of" since Algorithm 12 samples the required randomness within
 the function itself, whereas this implementation expects it to be provided
 through the `key_generation_seed` parameter.

 Algorithm 12 is reproduced below:

 ```plaintext
 Output: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
 Output: decryption key dkₚₖₑ ∈ 𝔹^{384k}.

 d ←$ B
 (ρ,σ) ← G(d)
 N ← 0
 for (i ← 0; i < k; i++)
     for(j ← 0; j < k; j++)
         Â[i,j] ← SampleNTT(XOF(ρ, i, j))
     end for
 end for
 for(i ← 0; i < k; i++)
     s[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
     N ← N + 1
 end for
 for(i ← 0; i < k; i++)
     e[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(σ,N))
     N ← N + 1
 end for
 ŝ ← NTT(s)
 ê ← NTT(e)
 t̂ ← Â◦ŝ + ê
 ekₚₖₑ ← ByteEncode₁₂(t̂) ‖ ρ
 dkₚₖₑ ← ByteEncode₁₂(ŝ)
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair_unpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_22(
    Eurydice_borrow_slice_u8 key_generation_seed, Eurydice_arr_260 *private_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63 *public_key) {
  Eurydice_arr_06 hashed =
      libcrux_ml_kem_variant_cpa_keygen_seed_39_be(key_generation_seed);
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed), (size_t)32U, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_A = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_for_secret_and_error = uu____0.snd;
  Eurydice_arr_aa0 *uu____1 = &public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue0 =
      libcrux_ml_kem_utils_into_padded_array_b6(seed_for_A);
  libcrux_ml_kem_matrix_sample_matrix_A_6c(uu____1, &lvalue0, true);
  Eurydice_arr_3e prf_input =
      libcrux_ml_kem_utils_into_padded_array_c8(seed_for_secret_and_error);
  uint8_t domain_separator =
      libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_b4(private_key,
                                                           &prf_input, 0U);
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_22(&lvalue,
                                                                        i);
  }
  Eurydice_arr_260 error_as_ntt = arr_struct;
  libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_b4(
      &error_as_ntt, &prf_input, domain_separator);
  libcrux_ml_kem_matrix_compute_As_plus_e_ab(
      &public_key->t_as_ntt, &public_key->A, private_key, &error_as_ntt);
  Eurydice_arr_600 arr;
  memcpy(arr.data, seed_for_A.ptr, (size_t)32U * sizeof(uint8_t));
  Eurydice_arr_600 uu____2 = unwrap_26_07(
      (KRML_CLITERAL(Result_95){.tag = Ok, .val = {.case_Ok = arr}}));
  public_key->seed_for_A = uu____2;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.serialize_uncompressed_ring_element with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_cc
libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_84(
    const Eurydice_arr_51 *re) {
  Eurydice_arr_cc serialized = {.data = {0U}};
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    __m256i coefficient =
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re->data[i0]);
    Eurydice_arr_6d bytes =
        libcrux_ml_kem_vector_avx2_serialize_12_f5(coefficient);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_3611(
            &serialized, (KRML_CLITERAL(core_ops_range_Range_08){
                             .start = (size_t)24U * i0,
                             .end = (size_t)24U * i0 + (size_t)24U})),
        Eurydice_array_to_slice_shared_0b(&bytes), uint8_t);
  }
  return serialized;
}

/**
 Call [`serialize_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_vector
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_serialize_vector_ab(
    const Eurydice_arr_260 *key, Eurydice_mut_borrow_slice_u8 out) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf0(key),
                              Eurydice_arr_51);
       i++) {
    size_t i0 = i;
    Eurydice_arr_51 re = key->data[i0];
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_7e(
        out, (KRML_CLITERAL(core_ops_range_Range_08){
                 .start = i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                 .end = (i0 + (size_t)1U) *
                        LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT}));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_cc lvalue =
        libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_84(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_fe(&lvalue),
                        uint8_t);
  }
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_serialize_public_key_mut_ed(
    const Eurydice_arr_260 *t_as_ntt, Eurydice_borrow_slice_u8 seed_for_a,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cpa_serialize_vector_ab(
      t_as_ntt,
      Eurydice_array_to_subslice_mut_3612(
          serialized,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = (size_t)0U,
              .end = libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)})));
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_from_mut_8c2(
          serialized,
          libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)3U)),
      seed_for_a, uint8_t);
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_74
libcrux_ml_kem_ind_cpa_serialize_public_key_ed(
    const Eurydice_arr_260 *t_as_ntt, Eurydice_borrow_slice_u8 seed_for_a) {
  Eurydice_arr_74 public_key_serialized = {.data = {0U}};
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_ed(t_as_ntt, seed_for_a,
                                                     &public_key_serialized);
  return public_key_serialized;
}

/**
 Serialize the secret key from the unpacked key pair generation.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_unpacked_secret_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PRIVATE_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_utils_extraction_helper_Keypair768
libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_ed(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
        *public_key,
    const Eurydice_arr_260 *private_key) {
  Eurydice_arr_74 public_key_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_ed(
          &public_key->t_as_ntt,
          Eurydice_array_to_slice_shared_6e(&public_key->seed_for_A));
  Eurydice_arr_60 secret_key_serialized = {.data = {0U}};
  libcrux_ml_kem_ind_cpa_serialize_vector_ab(
      private_key, Eurydice_array_to_slice_mut_06(&secret_key_serialized));
  return (KRML_CLITERAL(libcrux_ml_kem_utils_extraction_helper_Keypair768){
      .fst = secret_key_serialized, .snd = public_key_serialized});
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- PRIVATE_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_utils_extraction_helper_Keypair768
libcrux_ml_kem_ind_cpa_generate_keypair_5d(
    Eurydice_borrow_slice_u8 key_generation_seed) {
  Eurydice_arr_260 private_key =
      libcrux_ml_kem_ind_cpa_unpacked_default_70_ab();
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63 public_key =
      libcrux_ml_kem_ind_cpa_unpacked_default_8b_ab();
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_22(
      key_generation_seed, &private_key, &public_key);
  return libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_ed(&public_key,
                                                                 &private_key);
}

/**
 Serialize the secret key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key_mut
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_ae(
    Eurydice_borrow_slice_u8 private_key, Eurydice_borrow_slice_u8 public_key,
    Eurydice_borrow_slice_u8 implicit_rejection_value,
    Eurydice_arr_ea *serialized) {
  size_t pointer = (size_t)0U;
  Eurydice_arr_ea *uu____0 = serialized;
  size_t uu____1 = pointer;
  size_t uu____2 = pointer;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_3613(
          uu____0,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = uu____1,
              .end = uu____2 + Eurydice_slice_len(private_key, uint8_t)})),
      private_key, uint8_t);
  pointer = pointer + Eurydice_slice_len(private_key, uint8_t);
  Eurydice_arr_ea *uu____3 = serialized;
  size_t uu____4 = pointer;
  size_t uu____5 = pointer;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_3613(
          uu____3,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = uu____4,
              .end = uu____5 + Eurydice_slice_len(public_key, uint8_t)})),
      public_key, uint8_t);
  pointer = pointer + Eurydice_slice_len(public_key, uint8_t);
  Eurydice_mut_borrow_slice_u8 uu____6 = Eurydice_array_to_subslice_mut_3613(
      serialized,
      (KRML_CLITERAL(core_ops_range_Range_08){
          .start = pointer,
          .end = pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE}));
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_600 lvalue =
      libcrux_ml_kem_hash_functions_avx2_H_41_e0(public_key);
  Eurydice_slice_copy(uu____6, Eurydice_array_to_slice_shared_6e(&lvalue),
                      uint8_t);
  pointer = pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE;
  Eurydice_arr_ea *uu____7 = serialized;
  size_t uu____8 = pointer;
  size_t uu____9 = pointer;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_3613(
          uu____7,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = uu____8,
              .end = uu____9 +
                     Eurydice_slice_len(implicit_rejection_value, uint8_t)})),
      implicit_rejection_value, uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ea
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_ae(
    Eurydice_borrow_slice_u8 private_key, Eurydice_borrow_slice_u8 public_key,
    Eurydice_borrow_slice_u8 implicit_rejection_value) {
  Eurydice_arr_ea out = {.data = {0U}};
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_ae(
      private_key, public_key, implicit_rejection_value, &out);
  return out;
}

/**
 Packed API

 Generate a key pair.

 Depending on the `Vector` and `Hasher` used, this requires different hardware
 features
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_generate_keypair_bb(const Eurydice_arr_06 *randomness) {
  Eurydice_borrow_slice_u8 ind_cpa_keypair_randomness =
      Eurydice_array_to_subslice_shared_363(
          randomness,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = (size_t)0U,
              .end =
                  LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE}));
  Eurydice_borrow_slice_u8 implicit_rejection_value =
      Eurydice_array_to_subslice_from_shared_8c0(
          randomness,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_utils_extraction_helper_Keypair768 uu____0 =
      libcrux_ml_kem_ind_cpa_generate_keypair_5d(ind_cpa_keypair_randomness);
  Eurydice_arr_60 ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_74 public_key = uu____0.snd;
  Eurydice_arr_ea secret_key_serialized =
      libcrux_ml_kem_ind_cca_serialize_kem_secret_key_ae(
          Eurydice_array_to_slice_shared_06(&ind_cpa_private_key),
          Eurydice_array_to_slice_shared_45(&public_key),
          implicit_rejection_value);
  Eurydice_arr_ea private_key =
      libcrux_ml_kem_types_from_77_28(secret_key_serialized);
  return libcrux_ml_kem_types_from_17_74(
      private_key, libcrux_ml_kem_types_from_fd_d0(public_key));
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2 with const
generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_ce(
    const Eurydice_arr_06 *randomness) {
  return libcrux_ml_kem_ind_cca_generate_keypair_bb(randomness);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_ce(
    const Eurydice_arr_06 *randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_ce(
      randomness);
}

/**
 Generate ML-KEM 768 Key Pair
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_mlkem768_avx2_generate_key_pair(Eurydice_arr_06 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_ce(
      &randomness);
}

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key_only
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_private_key_only_ae(
    const Eurydice_arr_ea *private_key) {
  Eurydice_arr_600 t = libcrux_ml_kem_hash_functions_avx2_H_41_e0(
      Eurydice_array_to_subslice_shared_364(
          private_key, (KRML_CLITERAL(core_ops_range_Range_08){
                           .start = (size_t)384U * (size_t)3U,
                           .end = (size_t)768U * (size_t)3U + (size_t)32U})));
  Eurydice_borrow_slice_u8 expected = Eurydice_array_to_subslice_shared_364(
      private_key, (KRML_CLITERAL(core_ops_range_Range_08){
                       .start = (size_t)768U * (size_t)3U + (size_t)32U,
                       .end = (size_t)768U * (size_t)3U + (size_t)64U}));
  return Eurydice_array_eq_slice((size_t)32U, &t, &expected, uint8_t, bool);
}

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
 Note that the size checks in 7.2 1 and 2 are covered by the `SECRET_KEY_SIZE`
 and `CIPHERTEXT_SIZE` in the `private_key` and `ciphertext` types.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_private_key_12(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *_ciphertext) {
  return libcrux_ml_kem_ind_cca_validate_private_key_only_ae(private_key);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2 with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_31(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_validate_private_key_12(private_key,
                                                        ciphertext);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_31(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_31(
      private_key, ciphertext);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool libcrux_ml_kem_mlkem768_avx2_validate_private_key(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_31(
      private_key, ciphertext);
}

/**
 Private key validation
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_41(
    const Eurydice_arr_ea *private_key) {
  return libcrux_ml_kem_ind_cca_validate_private_key_only_ae(private_key);
}

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool libcrux_ml_kem_mlkem768_avx2_validate_private_key_only(
    const Eurydice_arr_ea *private_key) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_41(
      private_key);
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for
libcrux_ml_kem::serialize::deserialize_ring_elements_reduced_out::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out.call_mut_0b with
types libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_ab(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 This function deserializes ring elements and reduces the result by the field
 modulus.

 This function MUST NOT be used on secret inputs.
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_260
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_ab(
    Eurydice_borrow_slice_u8 public_key) {
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_ab(
            &lvalue, i);
  }
  Eurydice_arr_260 deserialized_pk = arr_struct;
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_ab(
      public_key, &deserialized_pk);
  return deserialized_pk;
}

/**
 Validate an ML-KEM public key.

 This implements the Modulus check in 7.2 2.
 Note that the size check in 7.2 1 is covered by the `PUBLIC_KEY_SIZE` in the
 `public_key` type.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_public_key_ed(
    const Eurydice_arr_74 *public_key) {
  Eurydice_arr_260 deserialized_pk =
      libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_ab(
          Eurydice_array_to_subslice_to_shared_6e0(
              public_key,
              libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)));
  Eurydice_arr_74 public_key_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_ed(
          &deserialized_pk,
          Eurydice_array_to_subslice_from_shared_8c1(
              public_key,
              libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)));
  return Eurydice_array_eq((size_t)1184U, public_key, &public_key_serialized,
                           uint8_t);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2 with const
generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_41(
    const Eurydice_arr_74 *public_key) {
  return libcrux_ml_kem_ind_cca_validate_public_key_ed(public_key);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key with const
generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_41(
    const Eurydice_arr_74 *public_key) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_41(
      public_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool libcrux_ml_kem_mlkem768_avx2_validate_public_key(
    const Eurydice_arr_74 *public_key) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_41(
      public_key);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_63_s {
  Eurydice_arr_260 ind_cpa_private_key;
  Eurydice_arr_600 implicit_rejection_value;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_63;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63_s {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63 ind_cpa_public_key;
  Eurydice_arr_600 public_key_hash;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63;

typedef struct libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked_s {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_63 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 public_key;
} libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_ind_cca_unpacked_decapsulate_12(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_600 decrypted = libcrux_ml_kem_ind_cpa_decrypt_unpacked_2f(
      &key_pair->private_key.ind_cpa_private_key, ciphertext);
  Eurydice_arr_06 to_hash0 = libcrux_ml_kem_utils_into_padded_array_24(
      Eurydice_array_to_slice_shared_6e(&decrypted));
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_from_mut_8c(
      &to_hash0, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(
      uu____0,
      Eurydice_array_to_slice_shared_6e(&key_pair->public_key.public_key_hash),
      uint8_t);
  Eurydice_arr_06 hashed = libcrux_ml_kem_hash_functions_avx2_G_41_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash0));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_480 to_hash = libcrux_ml_kem_utils_into_padded_array_15(
      Eurydice_array_to_slice_shared_6e(
          &key_pair->private_key.implicit_rejection_value));
  Eurydice_mut_borrow_slice_u8 uu____2 =
      Eurydice_array_to_subslice_from_mut_8c0(
          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_d3_80(ciphertext),
                      uint8_t);
  Eurydice_arr_600 implicit_rejection_shared_secret =
      libcrux_ml_kem_hash_functions_avx2_PRF_41_41(
          Eurydice_array_to_slice_shared_74(&to_hash));
  Eurydice_arr_2c expected_ciphertext =
      libcrux_ml_kem_ind_cpa_encrypt_unpacked_74(
          &key_pair->public_key.ind_cpa_public_key, &decrypted,
          pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 =
      libcrux_ml_kem_types_as_ref_d3_80(ciphertext);
  uint8_t selector =
      libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
          uu____3, Eurydice_array_to_slice_shared_42(&expected_ciphertext));
  return libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
      shared_secret,
      Eurydice_array_to_slice_shared_6e(&implicit_rejection_shared_secret),
      selector);
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2 with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_35(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_12(key_pair, ciphertext);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_35(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_35(
      key_pair, ciphertext);
}

/**
 Decapsulate ML-KEM 768 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type
 [`MlKem768KeyPairUnpacked`] and an [`MlKem768Ciphertext`].
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_mlkem768_avx2_unpacked_decapsulate(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *private_key,
    const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_35(
      private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encaps_prepare
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_06 libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_be(
    Eurydice_borrow_slice_u8 randomness, Eurydice_borrow_slice_u8 pk_hash) {
  Eurydice_arr_06 to_hash =
      libcrux_ml_kem_utils_into_padded_array_24(randomness);
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_8c(
                          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE),
                      pk_hash, uint8_t);
  return libcrux_ml_kem_hash_functions_avx2_G_41_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash));
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_56 libcrux_ml_kem_ind_cca_unpacked_encapsulate_70(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *public_key,
    const Eurydice_arr_600 *randomness) {
  Eurydice_arr_06 hashed = libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_be(
      Eurydice_array_to_slice_shared_6e(randomness),
      Eurydice_array_to_slice_shared_6e(&public_key->public_key_hash));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____0.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____0.snd;
  Eurydice_arr_2c ciphertext = libcrux_ml_kem_ind_cpa_encrypt_unpacked_74(
      &public_key->ind_cpa_public_key, randomness, pseudorandomness);
  Eurydice_arr_600 shared_secret_array = {.data = {0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(&shared_secret_array),
                      shared_secret, uint8_t);
  return (KRML_CLITERAL(tuple_56){
      .fst = libcrux_ml_kem_types_from_e0_80(ciphertext),
      .snd = shared_secret_array});
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2 with const
generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_56
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_cd(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *public_key,
    const Eurydice_arr_600 *randomness) {
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_70(public_key, randomness);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate with const
generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_56
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_cd(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *public_key,
    const Eurydice_arr_600 *randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_cd(
      public_key, randomness);
}

/**
 Encapsulate ML-KEM 768 (unpacked)

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type
 [`MlKem768PublicKeyUnpacked`], the SHA3-256 hash of this public key, and
 [`SHARED_SECRET_SIZE`] bytes of `randomness`.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_56 libcrux_ml_kem_mlkem768_avx2_unpacked_encapsulate(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *public_key,
    Eurydice_arr_600 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_cd(
      public_key, &randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for
libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.transpose_a.closure.call_mut_b4 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_ab(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
@Array<libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1], K>> for
libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.transpose_a.call_mut_7b with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_260
libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_7b_ab(void **_,
                                                           size_t tupled_args) {
  Eurydice_arr_260 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_ab(
            &lvalue, i);
  }
  return arr_struct;
}

/**
This function found in impl {core::clone::Clone for
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.clone_c1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51 libcrux_ml_kem_polynomial_clone_c1_84(
    const Eurydice_arr_51 *self) {
  return core_array__core__clone__Clone_for__Array_T__N___clone(
      (size_t)16U, self, __m256i, Eurydice_arr_51);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_aa0 libcrux_ml_kem_ind_cca_unpacked_transpose_a_ab(
    Eurydice_arr_aa0 ind_cpa_a) {
  Eurydice_arr_aa0 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_7b_ab(&lvalue, i);
  }
  Eurydice_arr_aa0 A = arr_struct;
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      Eurydice_arr_51 uu____0 =
          libcrux_ml_kem_polynomial_clone_c1_84(&ind_cpa_a.data[j].data[i1]);
      A.data[i1].data[j] = uu____0;
    }
  }
  return A;
}

/**
 Generate Unpacked Keys
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cca_unpacked_generate_keypair_bb(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out) {
  Eurydice_borrow_slice_u8 ind_cpa_keypair_randomness =
      Eurydice_array_to_subslice_shared_363(
          &randomness,
          (KRML_CLITERAL(core_ops_range_Range_08){
              .start = (size_t)0U,
              .end =
                  LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE}));
  Eurydice_borrow_slice_u8 implicit_rejection_value =
      Eurydice_array_to_subslice_from_shared_8c0(
          &randomness,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_22(
      ind_cpa_keypair_randomness, &out->private_key.ind_cpa_private_key,
      &out->public_key.ind_cpa_public_key);
  Eurydice_arr_aa0 A = libcrux_ml_kem_ind_cca_unpacked_transpose_a_ab(
      out->public_key.ind_cpa_public_key.A);
  out->public_key.ind_cpa_public_key.A = A;
  Eurydice_arr_74 pk_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_ed(
          &out->public_key.ind_cpa_public_key.t_as_ntt,
          Eurydice_array_to_slice_shared_6e(
              &out->public_key.ind_cpa_public_key.seed_for_A));
  Eurydice_arr_600 uu____0 = libcrux_ml_kem_hash_functions_avx2_H_41_e0(
      Eurydice_array_to_slice_shared_45(&pk_serialized));
  out->public_key.public_key_hash = uu____0;
  Eurydice_arr_600 arr;
  memcpy(arr.data, implicit_rejection_value.ptr, (size_t)32U * sizeof(uint8_t));
  Eurydice_arr_600 uu____1 = unwrap_26_07(
      (KRML_CLITERAL(Result_95){.tag = Ok, .val = {.case_Ok = arr}}));
  out->private_key.implicit_rejection_value = uu____1;
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2 with
const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_ce(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out) {
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_bb(randomness, out);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair with const
generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_ce(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out) {
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_ce(
      randomness, out);
}

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair) {
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_ce(
      randomness, key_pair);
}

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
libcrux_ml_kem_ind_cca_unpacked_default_30_ab(void) {
  return (
      KRML_CLITERAL(libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63){
          .ind_cpa_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_ab(),
          .public_key_hash = {.data = {0U}}});
}

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
    libcrux_ml_kem_ind_cca_unpacked_default_7b_ab(void) {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_63 uu____0 = {
      .ind_cpa_private_key = libcrux_ml_kem_ind_cpa_unpacked_default_70_ab(),
      .implicit_rejection_value = {.data = {0U}}};
  return (KRML_CLITERAL(
      libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked){
      .private_key = uu____0,
      .public_key = libcrux_ml_kem_ind_cca_unpacked_default_30_ab()});
}

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair(
    Eurydice_arr_06 randomness) {
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_ind_cca_unpacked_default_7b_ab();
  libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(randomness,
                                                              &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair(void) {
  return libcrux_ml_kem_ind_cca_unpacked_default_7b_ab();
}

/**
 Create a new, empty unpacked public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
libcrux_ml_kem_mlkem768_avx2_unpacked_init_public_key(void) {
  return libcrux_ml_kem_ind_cca_unpacked_default_30_ab();
}

/**
This function found in impl {core::ops::function::FnMut<(@Array<i16, 272usize>),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector,
Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_e7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_51
libcrux_ml_kem_sampling_sample_from_xof_call_mut_e7_b3(
    void **_, Eurydice_arr_a00 tupled_args) {
  Eurydice_arr_a00 s = tupled_args;
  return libcrux_ml_kem_polynomial_from_i16_array_d6_84(
      Eurydice_array_to_subslice_shared_850(
          &s, (KRML_CLITERAL(core_ops_range_Range_08){.start = (size_t)0U,
                                                      .end = (size_t)256U})));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_260
libcrux_ml_kem_sampling_sample_from_xof_b3(const Eurydice_arr_84 *seeds) {
  Eurydice_arr_c8 sampled_coefficients = {.data = {0U}};
  Eurydice_arr_d4 out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  Eurydice_arr_e4 xof_state =
      libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_e0(
          seeds);
  Eurydice_arr_35 randomness0 =
      libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_e0(
          &xof_state);
  bool done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ed(
      &randomness0, &sampled_coefficients, &out);
  while (true) {
    if (done) {
      break;
    } else {
      Eurydice_arr_d6 randomness =
          libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_e0(
              &xof_state);
      done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ed0(
          &randomness, &sampled_coefficients, &out);
    }
  }
  Eurydice_arr_260 arr_mapped_str;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
        libcrux_ml_kem_sampling_sample_from_xof_call_mut_e7_b3(&lvalue,
                                                               out.data[i]);
  }
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_sample_matrix_A_b3(
    Eurydice_arr_aa0 *A_transpose, const Eurydice_arr_48 *seed,
    bool transpose) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    Eurydice_arr_84 seeds;
    Eurydice_arr_48 repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] =
          core_array__core__clone__Clone_for__Array_T__N___clone(
              (size_t)34U, seed, uint8_t, Eurydice_arr_48);
    }
    memcpy(seeds.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_48));
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;
    }
    Eurydice_arr_260 sampled =
        libcrux_ml_kem_sampling_sample_from_xof_b3(&seeds);
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf0(&sampled),
                                Eurydice_arr_51);
         i++) {
      size_t j = i;
      Eurydice_arr_51 sample = sampled.data[j];
      if (transpose) {
        A_transpose->data[j].data[i1] = sample;
      } else {
        A_transpose->data[i1].data[j] = sample;
      }
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_bf(
    Eurydice_borrow_slice_u8 public_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
        *unpacked_public_key) {
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_ab(
      Eurydice_slice_subslice_to_shared_c6(public_key, (size_t)1152U),
      &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8 seed =
      Eurydice_slice_subslice_from_shared_6b(public_key, (size_t)1152U);
  Eurydice_arr_aa0 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue = libcrux_ml_kem_utils_into_padded_array_b6(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_b3(uu____0, &lvalue, false);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.keys_from_private_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_2f(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x4 uu____0 =
      libcrux_ml_kem_types_unpack_private_key_b4(
          Eurydice_array_to_slice_shared_ec(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  libcrux_ml_kem_ind_cpa_deserialize_vector_ab(
      ind_cpa_secret_key, &key_pair->private_key.ind_cpa_private_key);
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_bf(
      ind_cpa_public_key, &key_pair->public_key.ind_cpa_public_key);
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_6e(&key_pair->public_key.public_key_hash),
      ind_cpa_public_key_hash, uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(
                          &key_pair->private_key.implicit_rejection_value),
                      implicit_rejection_value, uint8_t);
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_6e(
          &key_pair->public_key.ind_cpa_public_key.seed_for_A),
      Eurydice_slice_subslice_from_shared_6b(ind_cpa_public_key, (size_t)1152U),
      uint8_t);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_fd(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair) {
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_2f(private_key,
                                                           key_pair);
}

/**
 Get an unpacked key from a private key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_from_private_mut(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair) {
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_fd(
      private_key, key_pair);
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_8c(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self,
    Eurydice_arr_ea *serialized) {
  libcrux_ml_kem_utils_extraction_helper_Keypair768 uu____0 =
      libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_ed(
          &self->public_key.ind_cpa_public_key,
          &self->private_key.ind_cpa_private_key);
  Eurydice_arr_60 ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_74 ind_cpa_public_key = uu____0.snd;
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_d6(
      Eurydice_array_to_slice_shared_06(&ind_cpa_private_key),
      Eurydice_array_to_slice_shared_45(&ind_cpa_public_key),
      Eurydice_array_to_slice_shared_6e(
          &self->private_key.implicit_rejection_value),
      serialized);
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ea
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_8c(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self) {
  Eurydice_arr_ea sk = libcrux_ml_kem_types_default_d3_28();
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_8c(self, &sk);
  return sk;
}

/**
 Get the serialized private key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ea
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_private_key(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_8c(key_pair);
}

/**
 Get the serialized private key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_private_key_mut(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    Eurydice_arr_ea *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_8c(key_pair,
                                                                   serialized);
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_74
libcrux_ml_kem_ind_cca_unpacked_serialized_dd_ed(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *self) {
  return libcrux_ml_kem_types_from_fd_d0(
      libcrux_ml_kem_ind_cpa_serialize_public_key_ed(
          &self->ind_cpa_public_key.t_as_ntt,
          Eurydice_array_to_slice_shared_6e(
              &self->ind_cpa_public_key.seed_for_A)));
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_74
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_ed(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_dd_ed(&self->public_key);
}

/**
 Get the serialized public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_74
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_public_key(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_ed(key_pair);
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ed(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *self,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_ed(
      &self->ind_cpa_public_key.t_as_ntt,
      Eurydice_array_to_slice_shared_6e(&self->ind_cpa_public_key.seed_for_A),
      serialized);
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11 with types
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_ed(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ed(&self->public_key,
                                                       serialized);
}

/**
 Get the serialized public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_public_key_mut(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_ed(key_pair,
                                                                  serialized);
}

/**
This function found in impl {core::clone::Clone for
libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.clone_91
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63
libcrux_ml_kem_ind_cpa_unpacked_clone_91_ab(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63 *self) {
  Eurydice_arr_260 uu____0 =
      core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)3U, &self->t_as_ntt, Eurydice_arr_51, Eurydice_arr_260);
  Eurydice_arr_600 uu____1 =
      core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)32U, &self->seed_for_A, uint8_t, Eurydice_arr_600);
  return (
      KRML_CLITERAL(libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63){
          .t_as_ntt = uu____0,
          .seed_for_A = uu____1,
          .A = core_array__core__clone__Clone_for__Array_T__N___clone(
              (size_t)3U, &self->A, Eurydice_arr_260, Eurydice_arr_aa0)});
}

/**
This function found in impl {core::clone::Clone for
libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.clone_d7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
libcrux_ml_kem_ind_cca_unpacked_clone_d7_ab(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *self) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_63 uu____0 =
      libcrux_ml_kem_ind_cpa_unpacked_clone_91_ab(&self->ind_cpa_public_key);
  return (KRML_CLITERAL(
      libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63){
      .ind_cpa_public_key = uu____0,
      .public_key_hash = core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)32U, &self->public_key_hash, uint8_t, Eurydice_arr_600)});
}

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *
libcrux_ml_kem_ind_cca_unpacked_public_key_11_ab(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self) {
  return &self->public_key;
}

/**
 Get the unpacked public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_kem_mlkem768_avx2_unpacked_public_key(
    const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *pk) {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 uu____0 =
      libcrux_ml_kem_ind_cca_unpacked_clone_d7_ab(
          libcrux_ml_kem_ind_cca_unpacked_public_key_11_ab(key_pair));
  pk[0U] = uu____0;
}

/**
 Get the serialized public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_kem_mlkem768_avx2_unpacked_serialized_public_key(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63 *public_key,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ed(public_key, serialized);
}

/**
 Generate an unpacked key from a serialized key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.unpack_public_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash,
libcrux_ml_kem_vector_avx2_SIMD256Vector with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_fb(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
        *unpacked_public_key) {
  Eurydice_borrow_slice_u8 uu____0 =
      Eurydice_array_to_subslice_to_shared_6e0(public_key, (size_t)1152U);
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_ab(
      uu____0, &unpacked_public_key->ind_cpa_public_key.t_as_ntt);
  unpacked_public_key->ind_cpa_public_key.seed_for_A =
      libcrux_ml_kem_utils_into_padded_array_9e(
          Eurydice_array_to_subslice_from_shared_8c1(public_key,
                                                     (size_t)1152U));
  Eurydice_arr_aa0 *uu____2 = &unpacked_public_key->ind_cpa_public_key.A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue = libcrux_ml_kem_utils_into_padded_array_b6(
      Eurydice_array_to_subslice_from_shared_8c1(public_key, (size_t)1152U));
  libcrux_ml_kem_matrix_sample_matrix_A_6c(uu____2, &lvalue, false);
  Eurydice_arr_600 uu____3 = libcrux_ml_kem_hash_functions_avx2_H_41_e0(
      Eurydice_array_to_slice_shared_45(
          libcrux_ml_kem_types_as_slice_e6_d0(public_key)));
  unpacked_public_key->public_key_hash = uu____3;
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2 with
const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_31(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_fb(public_key,
                                                       unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key with const
generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_31(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_31(
      public_key, unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_kem_mlkem768_avx2_unpacked_unpacked_public_key(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_31(
      public_key, unpacked_public_key);
}

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_63
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768PublicKeyUnpacked;

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem768_avx2_H_DEFINED
#endif /* libcrux_mlkem768_avx2_H */
