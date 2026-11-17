/*
 * SPDX-FileCopyrightText: 2026 CE Labs
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: e656e17bff6ca5efac8ab6919b9b74cb9a8dd8ad
 * Eurydice: aaa9fa657fb6f09802edb890252040d94cd93982
 * Karamel: 8c19d41458ce5cbfea029ebc03334ba96d149039
 * F*: 7b347386330d0e5a331a220535b6f15288903234
 * Libcrux: dirty
 */


#ifndef libcrux_mlkem_avx2_H
#define libcrux_mlkem_avx2_H

#include "eurydice_glue.h"



#include "intrinsics/libcrux_intrinsics_avx2.h"

#include "libcrux_sha3_portable.h"
#include "libcrux_sha3_avx2.h"
#include "libcrux_mlkem_portable.h"
#include "libcrux_mlkem_core.h"
#include "libcrux_ct_ops.h"
#include "combined_core.h"

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_hash_functions_avx2_G(Eurydice_borrow_slice_u8 input)
{
  Eurydice_arr_c7 digest = { { 0U } };
  libcrux_sha3_portable_sha512(Eurydice_array_to_slice_mut_17(&digest), input);
  return digest;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_H(Eurydice_borrow_slice_u8 input)
{
  Eurydice_arr_ec digest = { { 0U } };
  libcrux_sha3_portable_sha256(Eurydice_array_to_slice_mut_01(&digest), input);
  return digest;
}

typedef libcrux_sha3_avx2_x4_incremental_KeccakState
libcrux_ml_kem_hash_functions_avx2_Simd256Hash;

typedef __m256i libcrux_ml_kem_vector_avx2_SIMD256Vector;

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_vec_zero(void)
{
  return libcrux_intrinsics_avx2_mm256_setzero_si256();
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ZERO_f5(void)
{
  return libcrux_ml_kem_vector_avx2_vec_zero();
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_vec_from_i16_array(Eurydice_borrow_slice_i16 array)
{
  return libcrux_intrinsics_avx2_mm256_loadu_si256_i16(array);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_from_i16_array_f5(Eurydice_borrow_slice_i16 array)
{
  return libcrux_ml_kem_vector_avx2_vec_from_i16_array(array);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_avx2_vec_to_i16_array(__m256i v)
{
  Eurydice_arr_d6 output = { { 0U } };
  libcrux_intrinsics_avx2_mm256_storeu_si256_i16(Eurydice_array_to_slice_mut_8a(&output), v);
  return output;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_avx2_to_i16_array_f5(__m256i x)
{
  return libcrux_ml_kem_vector_avx2_vec_to_i16_array(x);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_from_bytes(Eurydice_borrow_slice_u8 array)
{
  return
    libcrux_intrinsics_avx2_mm256_loadu_si256_u8(Eurydice_slice_subslice_shared_c8(array,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)32U })));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_from_bytes_f5(Eurydice_borrow_slice_u8 array)
{
  return libcrux_ml_kem_vector_avx2_from_bytes(array);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_vector_avx2_to_bytes(__m256i x, Eurydice_mut_borrow_slice_u8 bytes)
{
  libcrux_intrinsics_avx2_mm256_storeu_si256_u8(Eurydice_slice_subslice_mut_c8(bytes,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)32U })),
    x);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_vector_avx2_to_bytes_f5(__m256i x, Eurydice_mut_borrow_slice_u8 bytes)
{
  libcrux_ml_kem_vector_avx2_to_bytes(x, bytes);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_add(__m256i lhs, __m256i rhs)
{
  return libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_add_f5(__m256i lhs, const __m256i *rhs)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_add(lhs, rhs[0U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_sub(__m256i lhs, __m256i rhs)
{
  return libcrux_intrinsics_avx2_mm256_sub_epi16(lhs, rhs);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_sub_f5(__m256i lhs, const __m256i *rhs)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_sub(lhs, rhs[0U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_multiply_by_constant(__m256i vector, int16_t constant)
{
  __m256i cv = libcrux_intrinsics_avx2_mm256_set1_epi16(constant);
  return libcrux_intrinsics_avx2_mm256_mullo_epi16(vector, cv);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_multiply_by_constant_f5(__m256i vec, int16_t c)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_multiply_by_constant(vec, c);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_cond_subtract_3329(__m256i vector)
{
  __m256i
  field_modulus =
    libcrux_intrinsics_avx2_mm256_set1_epi16(LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i v_minus_field_modulus = libcrux_intrinsics_avx2_mm256_sub_epi16(vector, field_modulus);
  __m256i
  sign_mask = libcrux_intrinsics_avx2_mm256_srai_epi16(15, v_minus_field_modulus, __m256i);
  __m256i
  conditional_add_field_modulus =
    libcrux_intrinsics_avx2_mm256_and_si256(sign_mask,
      field_modulus);
  return
    libcrux_intrinsics_avx2_mm256_add_epi16(v_minus_field_modulus,
      conditional_add_field_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_cond_subtract_3329(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_cond_subtract_3329(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_cond_subtract_3329_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_cond_subtract_3329(vector);
}

#define LIBCRUX_ML_KEM_VECTOR_AVX2_ARITHMETIC_BARRETT_MULTIPLIER (20159)

/**
 See Section 3.2 of the implementation notes document for an explanation
 of this code.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_barrett_reduce(__m256i vector)
{
  __m256i
  t0 =
    libcrux_intrinsics_avx2_mm256_mulhi_epi16(vector,
      libcrux_intrinsics_avx2_mm256_set1_epi16(LIBCRUX_ML_KEM_VECTOR_AVX2_ARITHMETIC_BARRETT_MULTIPLIER));
  __m256i t512 = libcrux_intrinsics_avx2_mm256_set1_epi16(512);
  __m256i t1 = libcrux_intrinsics_avx2_mm256_add_epi16(t0, t512);
  __m256i quotient = libcrux_intrinsics_avx2_mm256_srai_epi16(10, t1, __m256i);
  __m256i
  quotient_times_field_modulus =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(quotient,
      libcrux_intrinsics_avx2_mm256_set1_epi16(LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
  return libcrux_intrinsics_avx2_mm256_sub_epi16(vector, quotient_times_field_modulus);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_barrett_reduce_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_barrett_reduce(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constant(
  __m256i vector,
  int16_t constant
)
{
  __m256i vec_constant = libcrux_intrinsics_avx2_mm256_set1_epi16(constant);
  __m256i value_low = libcrux_intrinsics_avx2_mm256_mullo_epi16(vector, vec_constant);
  __m256i
  k =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(value_low,
      libcrux_intrinsics_avx2_mm256_set1_epi16((int16_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m256i
  modulus = libcrux_intrinsics_avx2_mm256_set1_epi16(LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i k_times_modulus = libcrux_intrinsics_avx2_mm256_mulhi_epi16(k, modulus);
  __m256i value_high = libcrux_intrinsics_avx2_mm256_mulhi_epi16(vector, vec_constant);
  return libcrux_intrinsics_avx2_mm256_sub_epi16(value_high, k_times_modulus);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(__m256i vector, int16_t constant)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constant(vector, constant);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_bitwise_and_with_constant(
  __m256i vector,
  int16_t constant
)
{
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
libcrux_ml_kem_vector_avx2_arithmetic_shift_right_ef(__m256i vector)
{
  return libcrux_intrinsics_avx2_mm256_srai_epi16(15, vector, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_to_unsigned_representative(__m256i a)
{
  __m256i t = libcrux_ml_kem_vector_avx2_arithmetic_shift_right_ef(a);
  __m256i
  fm =
    libcrux_ml_kem_vector_avx2_arithmetic_bitwise_and_with_constant(t,
      LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  return libcrux_ml_kem_vector_avx2_arithmetic_add(a, fm);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i
libcrux_ml_kem_vector_avx2_to_unsigned_representative_f5(__m256i a)
{
  return libcrux_ml_kem_vector_avx2_arithmetic_to_unsigned_representative(a);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_message_coefficient(__m256i vector)
{
  __m256i
  field_modulus_halved =
    libcrux_intrinsics_avx2_mm256_set1_epi16((LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS - 1) / 2);
  __m256i
  field_modulus_quartered =
    libcrux_intrinsics_avx2_mm256_set1_epi16((LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS - 1) / 4);
  __m256i shifted = libcrux_intrinsics_avx2_mm256_sub_epi16(field_modulus_halved, vector);
  __m256i mask = libcrux_intrinsics_avx2_mm256_srai_epi16(15, shifted, __m256i);
  __m256i shifted_to_positive = libcrux_intrinsics_avx2_mm256_xor_si256(mask, shifted);
  __m256i
  shifted_to_positive_in_range =
    libcrux_intrinsics_avx2_mm256_sub_epi16(shifted_to_positive,
      field_modulus_quartered);
  return libcrux_intrinsics_avx2_mm256_srli_epi16(15, shifted_to_positive_in_range, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_1(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_compress_message_coefficient(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_1_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_1(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(__m256i lhs, __m256i rhs)
{
  __m256i prod02 = libcrux_intrinsics_avx2_mm256_mul_epu32(lhs, rhs);
  __m256i
  prod13 =
    libcrux_intrinsics_avx2_mm256_mul_epu32(libcrux_intrinsics_avx2_mm256_shuffle_epi32(245,
        lhs,
        __m256i),
      libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, rhs, __m256i));
  return
    libcrux_intrinsics_avx2_mm256_unpackhi_epi64(libcrux_intrinsics_avx2_mm256_unpacklo_epi32(prod02,
        prod13),
      libcrux_intrinsics_avx2_mm256_unpackhi_epi32(prod02, prod13));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_1(__m256i a)
{
  __m256i z = libcrux_intrinsics_avx2_mm256_setzero_si256();
  __m256i s = libcrux_ml_kem_vector_avx2_arithmetic_sub(z, a);
  return libcrux_ml_kem_vector_avx2_arithmetic_bitwise_and_with_constant(s, 1665);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i
libcrux_ml_kem_vector_avx2_decompress_1_f5(__m256i a)
{
  return libcrux_ml_kem_vector_avx2_compress_decompress_1(a);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(
  __m256i vec,
  __m256i constants
)
{
  __m256i value_low = libcrux_intrinsics_avx2_mm256_mullo_epi16(vec, constants);
  __m256i
  k =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(value_low,
      libcrux_intrinsics_avx2_mm256_set1_epi16((int16_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m256i
  modulus = libcrux_intrinsics_avx2_mm256_set1_epi16(LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i k_times_modulus = libcrux_intrinsics_avx2_mm256_mulhi_epi16(k, modulus);
  __m256i value_high = libcrux_intrinsics_avx2_mm256_mulhi_epi16(vec, constants);
  return libcrux_intrinsics_avx2_mm256_sub_epi16(value_high, k_times_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_ntt_layer_1_step(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  __m256i
  zetas =
    libcrux_intrinsics_avx2_mm256_set_epi16(-zeta3,
      -zeta3,
      zeta3,
      zeta3,
      -zeta2,
      -zeta2,
      zeta2,
      zeta2,
      -zeta1,
      -zeta1,
      zeta1,
      zeta1,
      -zeta0,
      -zeta0,
      zeta0,
      zeta0);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, vector, __m256i);
  __m256i
  rhs0 = libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(rhs, zetas);
  __m256i lhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32(160, vector, __m256i);
  return libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_1_step(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  return libcrux_ml_kem_vector_avx2_ntt_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_1_step_f5(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  return libcrux_ml_kem_vector_avx2_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_ntt_layer_2_step(__m256i vector, int16_t zeta0, int16_t zeta1)
{
  __m256i
  zetas =
    libcrux_intrinsics_avx2_mm256_set_epi16(-zeta1,
      -zeta1,
      -zeta1,
      -zeta1,
      zeta1,
      zeta1,
      zeta1,
      zeta1,
      -zeta0,
      -zeta0,
      -zeta0,
      -zeta0,
      zeta0,
      zeta0,
      zeta0,
      zeta0);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32(238, vector, __m256i);
  __m256i
  rhs0 = libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(rhs, zetas);
  __m256i lhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32(68, vector, __m256i);
  return libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_2_step(__m256i vector, int16_t zeta0, int16_t zeta1)
{
  return libcrux_ml_kem_vector_avx2_ntt_ntt_layer_2_step(vector, zeta0, zeta1);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_2_step_f5(__m256i vector, int16_t zeta0, int16_t zeta1)
{
  return libcrux_ml_kem_vector_avx2_ntt_layer_2_step(vector, zeta0, zeta1);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m128i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_m128i_by_constants(
  __m128i vec,
  __m128i constants
)
{
  __m128i value_low = libcrux_intrinsics_avx2_mm_mullo_epi16(vec, constants);
  __m128i
  k =
    libcrux_intrinsics_avx2_mm_mullo_epi16(value_low,
      libcrux_intrinsics_avx2_mm_set1_epi16((int16_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m128i
  modulus = libcrux_intrinsics_avx2_mm_set1_epi16(LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m128i k_times_modulus = libcrux_intrinsics_avx2_mm_mulhi_epi16(k, modulus);
  __m128i value_high = libcrux_intrinsics_avx2_mm_mulhi_epi16(vec, constants);
  return libcrux_intrinsics_avx2_mm_sub_epi16(value_high, k_times_modulus);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_ntt_layer_3_step(__m256i vector, int16_t zeta)
{
  __m128i rhs = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m128i
  rhs0 =
    libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_m128i_by_constants(rhs,
      libcrux_intrinsics_avx2_mm_set1_epi16(zeta));
  __m128i lhs = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m128i lower_coefficients = libcrux_intrinsics_avx2_mm_add_epi16(lhs, rhs0);
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm_sub_epi16(lhs, rhs0);
  __m256i combined = libcrux_intrinsics_avx2_mm256_castsi128_si256(lower_coefficients);
  return
    libcrux_intrinsics_avx2_mm256_inserti128_si256(1,
      combined,
      upper_coefficients,
      __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_3_step(__m256i vector, int16_t zeta)
{
  return libcrux_ml_kem_vector_avx2_ntt_ntt_layer_3_step(vector, zeta);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_layer_3_step_f5(__m256i vector, int16_t zeta)
{
  return libcrux_ml_kem_vector_avx2_ntt_layer_3_step(vector, zeta);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_1_step(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  __m256i lhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32(245, vector, __m256i);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_shuffle_epi32(160, vector, __m256i);
  __m256i
  rhs0 =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(rhs,
      libcrux_intrinsics_avx2_mm256_set_epi16(-1,
        -1,
        1,
        1,
        -1,
        -1,
        1,
        1,
        -1,
        -1,
        1,
        1,
        -1,
        -1,
        1,
        1));
  __m256i sum0 = libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
  __m256i
  sum_times_zetas =
    libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(sum0,
      libcrux_intrinsics_avx2_mm256_set_epi16(zeta3,
        zeta3,
        0,
        0,
        zeta2,
        zeta2,
        0,
        0,
        zeta1,
        zeta1,
        0,
        0,
        zeta0,
        zeta0,
        0,
        0));
  __m256i sum = libcrux_ml_kem_vector_avx2_arithmetic_barrett_reduce(sum0);
  return libcrux_intrinsics_avx2_mm256_blend_epi16(204, sum, sum_times_zetas, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  return libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step_f5(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  return libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_2_step(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1
)
{
  __m256i lhs = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(245, vector, __m256i);
  __m256i rhs = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(160, vector, __m256i);
  __m256i
  rhs0 =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(rhs,
      libcrux_intrinsics_avx2_mm256_set_epi16(-1,
        -1,
        -1,
        -1,
        1,
        1,
        1,
        1,
        -1,
        -1,
        -1,
        -1,
        1,
        1,
        1,
        1));
  __m256i sum = libcrux_intrinsics_avx2_mm256_add_epi16(lhs, rhs0);
  __m256i
  sum_times_zetas =
    libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_by_constants(sum,
      libcrux_intrinsics_avx2_mm256_set_epi16(zeta1,
        zeta1,
        zeta1,
        zeta1,
        0,
        0,
        0,
        0,
        zeta0,
        zeta0,
        zeta0,
        zeta0,
        0,
        0,
        0,
        0));
  return libcrux_intrinsics_avx2_mm256_blend_epi16(240, sum, sum_times_zetas, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step(__m256i vector, int16_t zeta0, int16_t zeta1)
{
  return libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_2_step(vector, zeta0, zeta1);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step_f5(
  __m256i vector,
  int16_t zeta0,
  int16_t zeta1
)
{
  return libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step(vector, zeta0, zeta1);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_3_step(__m256i vector, int16_t zeta)
{
  __m128i lhs = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m128i rhs = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m128i lower_coefficients = libcrux_intrinsics_avx2_mm_add_epi16(lhs, rhs);
  __m128i upper_coefficients = libcrux_intrinsics_avx2_mm_sub_epi16(lhs, rhs);
  __m128i
  upper_coefficients0 =
    libcrux_ml_kem_vector_avx2_arithmetic_montgomery_multiply_m128i_by_constants(upper_coefficients,
      libcrux_intrinsics_avx2_mm_set1_epi16(zeta));
  __m256i combined = libcrux_intrinsics_avx2_mm256_castsi128_si256(lower_coefficients);
  return
    libcrux_intrinsics_avx2_mm256_inserti128_si256(1,
      combined,
      upper_coefficients0,
      __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step(__m256i vector, int16_t zeta)
{
  return libcrux_ml_kem_vector_avx2_ntt_inv_ntt_layer_3_step(vector, zeta);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step_f5(__m256i vector, int16_t zeta)
{
  return libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step(vector, zeta);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(__m256i vec)
{
  __m256i
  k =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(vec,
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  __m256i
  k_times_modulus =
    libcrux_intrinsics_avx2_mm256_mulhi_epi16(k,
      libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
  __m256i value_high = libcrux_intrinsics_avx2_mm256_srli_epi32(16, vec, __m256i);
  __m256i result = libcrux_intrinsics_avx2_mm256_sub_epi16(value_high, k_times_modulus);
  __m256i result0 = libcrux_intrinsics_avx2_mm256_slli_epi32(16, result, __m256i);
  return libcrux_intrinsics_avx2_mm256_srai_epi32(16, result0, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_ntt_multiply(
  __m256i lhs,
  __m256i rhs,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  __m256i
  shuffle_with =
    libcrux_intrinsics_avx2_mm256_set_epi8(15,
      14,
      11,
      10,
      7,
      6,
      3,
      2,
      13,
      12,
      9,
      8,
      5,
      4,
      1,
      0,
      15,
      14,
      11,
      10,
      7,
      6,
      3,
      2,
      13,
      12,
      9,
      8,
      5,
      4,
      1,
      0);
  __m256i lhs_shuffled = libcrux_intrinsics_avx2_mm256_shuffle_epi8(lhs, shuffle_with);
  __m256i
  lhs_shuffled0 = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, lhs_shuffled, __m256i);
  __m128i lhs_evens = libcrux_intrinsics_avx2_mm256_castsi256_si128(lhs_shuffled0);
  __m256i lhs_evens0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(lhs_evens);
  __m128i lhs_odds = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, lhs_shuffled0, __m128i);
  __m256i lhs_odds0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(lhs_odds);
  __m256i rhs_shuffled = libcrux_intrinsics_avx2_mm256_shuffle_epi8(rhs, shuffle_with);
  __m256i
  rhs_shuffled0 = libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, rhs_shuffled, __m256i);
  __m128i rhs_evens = libcrux_intrinsics_avx2_mm256_castsi256_si128(rhs_shuffled0);
  __m256i rhs_evens0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(rhs_evens);
  __m128i rhs_odds = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, rhs_shuffled0, __m128i);
  __m256i rhs_odds0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(rhs_odds);
  __m256i left = libcrux_intrinsics_avx2_mm256_mullo_epi32(lhs_evens0, rhs_evens0);
  __m256i right = libcrux_intrinsics_avx2_mm256_mullo_epi32(lhs_odds0, rhs_odds0);
  __m256i right0 = libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(right);
  __m256i
  right1 =
    libcrux_intrinsics_avx2_mm256_mullo_epi32(right0,
      libcrux_intrinsics_avx2_mm256_set_epi32(-(int32_t)zeta3,
        (int32_t)zeta3,
        -(int32_t)zeta2,
        (int32_t)zeta2,
        -(int32_t)zeta1,
        (int32_t)zeta1,
        -(int32_t)zeta0,
        (int32_t)zeta0));
  __m256i products_left = libcrux_intrinsics_avx2_mm256_add_epi32(left, right1);
  __m256i
  products_left0 = libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(products_left);
  __m256i
  rhs_adjacent_swapped =
    libcrux_intrinsics_avx2_mm256_shuffle_epi8(rhs,
      libcrux_intrinsics_avx2_mm256_set_epi8(13,
        12,
        15,
        14,
        9,
        8,
        11,
        10,
        5,
        4,
        7,
        6,
        1,
        0,
        3,
        2,
        13,
        12,
        15,
        14,
        9,
        8,
        11,
        10,
        5,
        4,
        7,
        6,
        1,
        0,
        3,
        2));
  __m256i products_right = libcrux_intrinsics_avx2_mm256_madd_epi16(lhs, rhs_adjacent_swapped);
  __m256i
  products_right0 = libcrux_ml_kem_vector_avx2_arithmetic_montgomery_reduce_i32s(products_right);
  __m256i
  products_right1 = libcrux_intrinsics_avx2_mm256_slli_epi32(16, products_right0, __m256i);
  return
    libcrux_intrinsics_avx2_mm256_blend_epi16(170,
      products_left0,
      products_right1,
      __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_multiply(
  const __m256i *lhs,
  const __m256i *rhs,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  return
    libcrux_ml_kem_vector_avx2_ntt_ntt_multiply(lhs[0U],
      rhs[0U],
      zeta0,
      zeta1,
      zeta2,
      zeta3);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_ntt_multiply_f5(
  const __m256i *lhs,
  const __m256i *rhs,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
)
{
  return libcrux_ml_kem_vector_avx2_ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2, zeta3);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_avx2_serialize_serialize_1(__m256i vector)
{
  __m256i lsb_to_msb = libcrux_intrinsics_avx2_mm256_slli_epi16(15, vector, __m256i);
  __m128i low_msbs = libcrux_intrinsics_avx2_mm256_castsi256_si128(lsb_to_msb);
  __m128i high_msbs = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, lsb_to_msb, __m128i);
  __m128i msbs = libcrux_intrinsics_avx2_mm_packs_epi16(low_msbs, high_msbs);
  int32_t bits_packed = libcrux_intrinsics_avx2_mm_movemask_epi8(msbs);
  Eurydice_array_u8x2 result = { { (uint8_t)bits_packed, (uint8_t)(bits_packed >> 8U) } };
  return result;
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_avx2_serialize_1(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_serialize_1(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_avx2_serialize_1_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_1(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_i16s(int16_t a, int16_t b)
{
  __m256i
  coefficients =
    libcrux_intrinsics_avx2_mm256_set_epi16(b,
      b,
      b,
      b,
      b,
      b,
      b,
      b,
      a,
      a,
      a,
      a,
      a,
      a,
      a,
      a);
  __m256i
  coefficients_in_msb =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi16((int16_t)((uint32_t)1 << 8U),
        (int16_t)((uint32_t)1 << 9U),
        (int16_t)((uint32_t)1 << 10U),
        (int16_t)((uint32_t)1 << 11U),
        (int16_t)((uint32_t)1 << 12U),
        (int16_t)((uint32_t)1 << 13U),
        (int16_t)((uint32_t)1 << 14U),
        -32768,
        (int16_t)((uint32_t)1 << 8U),
        (int16_t)((uint32_t)1 << 9U),
        (int16_t)((uint32_t)1 << 10U),
        (int16_t)((uint32_t)1 << 11U),
        (int16_t)((uint32_t)1 << 12U),
        (int16_t)((uint32_t)1 << 13U),
        (int16_t)((uint32_t)1 << 14U),
        -32768));
  return libcrux_intrinsics_avx2_mm256_srli_epi16(15, coefficients_in_msb, __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_u8s(uint8_t a, uint8_t b)
{
  return
    libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_i16s((int16_t)(uint32_t)a,
      (int16_t)(uint32_t)b);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_1(Eurydice_borrow_slice_u8 bytes)
{
  return
    libcrux_ml_kem_vector_avx2_serialize_deserialize_1_deserialize_1_u8s(bytes.ptr[0U],
      bytes.ptr[1U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_1(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_1(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_1_f5(Eurydice_borrow_slice_u8 bytes)
{
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
libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(uint8_t n, __m256i x)
{
  int16_t n0 = (int16_t)((uint32_t)1 << (uint32_t)n);
  return
    libcrux_intrinsics_avx2_mm256_madd_epi16(x,
      libcrux_intrinsics_avx2_mm256_set_epi16(n0,
        1,
        n0,
        1,
        n0,
        1,
        n0,
        1,
        n0,
        1,
        n0,
        1,
        n0,
        1,
        n0,
        1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_avx2_serialize_serialize_4(__m256i vector)
{
  Eurydice_arr_b2 serialized = { { 0U } };
  __m256i
  adjacent_2_combined = libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(4U, vector);
  __m256i
  adjacent_8_combined =
    libcrux_intrinsics_avx2_mm256_shuffle_epi8(adjacent_2_combined,
      libcrux_intrinsics_avx2_mm256_set_epi8(-1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        12,
        8,
        4,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        12,
        8,
        4,
        0));
  __m256i
  combined =
    libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(adjacent_8_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 0, 0, 0, 4, 0));
  __m128i combined0 = libcrux_intrinsics_avx2_mm256_castsi256_si128(combined);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_slice_mut_29(&serialized),
    combined0);
  Eurydice_array_u8x8 arr;
  memcpy(arr.data,
    Eurydice_array_to_subslice_shared_d42(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)8U })).ptr,
    (size_t)8U * sizeof (uint8_t));
  return
    core_result_unwrap_26_e0(core_result_Result_8e_s(core_result_Ok,
        &core_result_Result_8e_s::U::case_Ok,
        arr));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_avx2_serialize_4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_serialize_4(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_avx2_serialize_4_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_4(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_i16s(
  int16_t b0,
  int16_t b1,
  int16_t b2,
  int16_t b3,
  int16_t b4,
  int16_t b5,
  int16_t b6,
  int16_t b7
)
{
  __m256i
  coefficients =
    libcrux_intrinsics_avx2_mm256_set_epi16(b7,
      b7,
      b6,
      b6,
      b5,
      b5,
      b4,
      b4,
      b3,
      b3,
      b2,
      b2,
      b1,
      b1,
      b0,
      b0);
  __m256i
  coefficients_in_msb =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi16((int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U)));
  __m256i
  coefficients_in_lsb = libcrux_intrinsics_avx2_mm256_srli_epi16(4, coefficients_in_msb, __m256i);
  return
    libcrux_intrinsics_avx2_mm256_and_si256(coefficients_in_lsb,
      libcrux_intrinsics_avx2_mm256_set1_epi16((int16_t)((uint32_t)1 << 4U) - 1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_u8s(
  uint8_t b0,
  uint8_t b1,
  uint8_t b2,
  uint8_t b3,
  uint8_t b4,
  uint8_t b5,
  uint8_t b6,
  uint8_t b7
)
{
  return
    libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_i16s((int16_t)(uint32_t)b0,
      (int16_t)(uint32_t)b1,
      (int16_t)(uint32_t)b2,
      (int16_t)(uint32_t)b3,
      (int16_t)(uint32_t)b4,
      (int16_t)(uint32_t)b5,
      (int16_t)(uint32_t)b6,
      (int16_t)(uint32_t)b7);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_4(Eurydice_borrow_slice_u8 bytes)
{
  return
    libcrux_ml_kem_vector_avx2_serialize_deserialize_4_deserialize_4_u8s(bytes.ptr[0U],
      bytes.ptr[1U],
      bytes.ptr[2U],
      bytes.ptr[3U],
      bytes.ptr[4U],
      bytes.ptr[5U],
      bytes.ptr[6U],
      bytes.ptr[7U]);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_4(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_4(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_4_f5(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_deserialize_4(bytes);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_ml_kem_vector_avx2_serialize_serialize_5(__m256i vector)
{
  Eurydice_arr_ec serialized = { { 0U } };
  __m256i
  adjacent_2_combined =
    libcrux_intrinsics_avx2_mm256_madd_epi16(vector,
      libcrux_intrinsics_avx2_mm256_set_epi16((int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1,
        (int16_t)((uint32_t)1 << 5U),
        1));
  __m256i
  adjacent_4_combined =
    libcrux_intrinsics_avx2_mm256_sllv_epi32(adjacent_2_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 22, 0, 22, 0, 22, 0, 22));
  __m256i
  adjacent_4_combined0 =
    libcrux_intrinsics_avx2_mm256_srli_epi64(22,
      adjacent_4_combined,
      __m256i);
  __m256i
  adjacent_8_combined =
    libcrux_intrinsics_avx2_mm256_shuffle_epi32(8,
      adjacent_4_combined0,
      __m256i);
  __m256i
  adjacent_8_combined0 =
    libcrux_intrinsics_avx2_mm256_sllv_epi32(adjacent_8_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 0, 0, 12, 0, 0, 0, 12));
  __m256i
  adjacent_8_combined1 =
    libcrux_intrinsics_avx2_mm256_srli_epi64(12,
      adjacent_8_combined0,
      __m256i);
  __m128i lower_8 = libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_8_combined1);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_subslice_mut_d44(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)16U })),
    lower_8);
  __m128i
  upper_8 = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, adjacent_8_combined1, __m128i);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_subslice_mut_d44(&serialized,
      (core_ops_range_Range_87{ (size_t)5U, (size_t)21U })),
    upper_8);
  Eurydice_arr_6d arr;
  memcpy(arr.data,
    Eurydice_array_to_subslice_shared_d41(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)10U })).ptr,
    (size_t)10U * sizeof (uint8_t));
  return
    core_result_unwrap_26_63(core_result_Result_80_s(core_result_Ok,
        &core_result_Result_80_s::U::case_Ok,
        arr));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_ml_kem_vector_avx2_serialize_5_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_serialize_5(vector);
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
libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(__m128i lower, __m128i upper)
{
  return
    libcrux_intrinsics_avx2_mm256_inserti128_si256(1,
      libcrux_intrinsics_avx2_mm256_castsi128_si256(lower),
      upper,
      __m256i);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_5(Eurydice_borrow_slice_u8 bytes)
{
  __m128i
  coefficients =
    libcrux_intrinsics_avx2_mm_set_epi8((int8_t)(uint32_t)bytes.ptr[9U],
      (int8_t)(uint32_t)bytes.ptr[8U],
      (int8_t)(uint32_t)bytes.ptr[8U],
      (int8_t)(uint32_t)bytes.ptr[7U],
      (int8_t)(uint32_t)bytes.ptr[7U],
      (int8_t)(uint32_t)bytes.ptr[6U],
      (int8_t)(uint32_t)bytes.ptr[6U],
      (int8_t)(uint32_t)bytes.ptr[5U],
      (int8_t)(uint32_t)bytes.ptr[4U],
      (int8_t)(uint32_t)bytes.ptr[3U],
      (int8_t)(uint32_t)bytes.ptr[3U],
      (int8_t)(uint32_t)bytes.ptr[2U],
      (int8_t)(uint32_t)bytes.ptr[2U],
      (int8_t)(uint32_t)bytes.ptr[1U],
      (int8_t)(uint32_t)bytes.ptr[1U],
      (int8_t)(uint32_t)bytes.ptr[0U]);
  __m256i
  coefficients_loaded =
    libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(coefficients,
      coefficients);
  __m256i
  coefficients0 =
    libcrux_intrinsics_avx2_mm256_shuffle_epi8(coefficients_loaded,
      libcrux_intrinsics_avx2_mm256_set_epi8(15,
        14,
        15,
        14,
        13,
        12,
        13,
        12,
        11,
        10,
        11,
        10,
        9,
        8,
        9,
        8,
        7,
        6,
        7,
        6,
        5,
        4,
        5,
        4,
        3,
        2,
        3,
        2,
        1,
        0,
        1,
        0));
  __m256i
  coefficients1 =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(coefficients0,
      libcrux_intrinsics_avx2_mm256_set_epi16((int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 5U),
        (int16_t)((uint32_t)1 << 2U),
        (int16_t)((uint32_t)1 << 7U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 9U),
        (int16_t)((uint32_t)1 << 6U),
        (int16_t)((uint32_t)1 << 11U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 5U),
        (int16_t)((uint32_t)1 << 2U),
        (int16_t)((uint32_t)1 << 7U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 9U),
        (int16_t)((uint32_t)1 << 6U),
        (int16_t)((uint32_t)1 << 11U)));
  return libcrux_intrinsics_avx2_mm256_srli_epi16(11, coefficients1, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_5_f5(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_5(bytes);
}

typedef struct core_core_arch_x86___m128i_x2_s
{
  __m128i fst;
  __m128i snd;
}
core_core_arch_x86___m128i_x2;

KRML_ATTRIBUTE_TARGET("avx2")
static inline core_core_arch_x86___m128i_x2
libcrux_ml_kem_vector_avx2_serialize_serialize_10_serialize_10_vec(__m256i vector)
{
  __m256i
  adjacent_2_combined = libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(10U, vector);
  __m256i
  adjacent_4_combined =
    libcrux_intrinsics_avx2_mm256_sllv_epi32(adjacent_2_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 12, 0, 12, 0, 12, 0, 12));
  __m256i
  adjacent_4_combined0 =
    libcrux_intrinsics_avx2_mm256_srli_epi64(12,
      adjacent_4_combined,
      __m256i);
  __m256i
  adjacent_8_combined =
    libcrux_intrinsics_avx2_mm256_shuffle_epi8(adjacent_4_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(-1,
        -1,
        -1,
        -1,
        -1,
        -1,
        12,
        11,
        10,
        9,
        8,
        4,
        3,
        2,
        1,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        12,
        11,
        10,
        9,
        8,
        4,
        3,
        2,
        1,
        0));
  __m128i lower_8 = libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_8_combined);
  __m128i
  upper_8 = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, adjacent_8_combined, __m128i);
  return (core_core_arch_x86___m128i_x2{ lower_8, upper_8 });
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_fc
libcrux_ml_kem_vector_avx2_serialize_serialize_10(__m256i vector)
{
  core_core_arch_x86___m128i_x2
  uu____0 = libcrux_ml_kem_vector_avx2_serialize_serialize_10_serialize_10_vec(vector);
  __m128i lower_8 = uu____0.fst;
  __m128i upper_8 = uu____0.snd;
  Eurydice_arr_ec serialized = { { 0U } };
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_subslice_mut_d44(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)16U })),
    lower_8);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_subslice_mut_d44(&serialized,
      (core_ops_range_Range_87{ (size_t)10U, (size_t)26U })),
    upper_8);
  Eurydice_arr_fc arr;
  memcpy(arr.data,
    Eurydice_array_to_subslice_shared_d41(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)20U })).ptr,
    (size_t)20U * sizeof (uint8_t));
  return
    core_result_unwrap_26_7d(core_result_Result_83_s(core_result_Ok,
        &core_result_Result_83_s::U::case_Ok,
        arr));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_fc
libcrux_ml_kem_vector_avx2_serialize_10(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_serialize_10(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_fc
libcrux_ml_kem_vector_avx2_serialize_10_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_10(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_10_deserialize_10_vec(
  __m128i lower_coefficients0,
  __m128i upper_coefficients0
)
{
  __m128i
  lower_coefficients =
    libcrux_intrinsics_avx2_mm_shuffle_epi8(lower_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(9, 8, 8, 7, 7, 6, 6, 5, 4, 3, 3, 2, 2, 1, 1, 0));
  __m128i
  upper_coefficients =
    libcrux_intrinsics_avx2_mm_shuffle_epi8(upper_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(15, 14, 14, 13, 13, 12, 12, 11, 10, 9, 9, 8, 8, 7, 7, 6));
  __m256i
  coefficients =
    libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(lower_coefficients,
      upper_coefficients);
  __m256i
  coefficients0 =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi16((int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 2U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 6U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 2U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 6U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 2U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 6U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 2U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 6U)));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_srli_epi16(6, coefficients0, __m256i);
  return
    libcrux_intrinsics_avx2_mm256_and_si256(coefficients1,
      libcrux_intrinsics_avx2_mm256_set1_epi16((int16_t)((uint32_t)1 << 10U) - 1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_10(Eurydice_borrow_slice_u8 bytes)
{
  Eurydice_borrow_slice_u8
  lower_coefficients =
    Eurydice_slice_subslice_shared_c8(bytes,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)16U }));
  Eurydice_borrow_slice_u8
  upper_coefficients =
    Eurydice_slice_subslice_shared_c8(bytes,
      (core_ops_range_Range_87{ (size_t)4U, (size_t)20U }));
  return
    libcrux_ml_kem_vector_avx2_serialize_deserialize_10_deserialize_10_vec(libcrux_intrinsics_avx2_mm_loadu_si128(lower_coefficients),
      libcrux_intrinsics_avx2_mm_loadu_si128(upper_coefficients));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_10(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_10(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_10_f5(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_deserialize_10(bytes);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_80
libcrux_ml_kem_vector_avx2_serialize_serialize_11(__m256i vector)
{
  Eurydice_arr_d6 array = { { 0U } };
  libcrux_intrinsics_avx2_mm256_storeu_si256_i16(Eurydice_array_to_slice_mut_8a(&array), vector);
  Eurydice_arr_d6
  input =
    libcrux_ml_kem_vector_portable_from_i16_array_b8(Eurydice_array_to_slice_shared_8a(&array));
  return libcrux_ml_kem_vector_portable_serialize_11_b8(input);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_80
libcrux_ml_kem_vector_avx2_serialize_11_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_serialize_11(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_11(Eurydice_borrow_slice_u8 bytes)
{
  Eurydice_arr_d6 output = libcrux_ml_kem_vector_portable_deserialize_11_b8(bytes);
  Eurydice_arr_d6 array = libcrux_ml_kem_vector_portable_to_i16_array_b8(output);
  return
    libcrux_intrinsics_avx2_mm256_loadu_si256_i16(Eurydice_array_to_slice_shared_8a(&array));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_11_f5(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_11(bytes);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE core_core_arch_x86___m128i_x2
libcrux_ml_kem_vector_avx2_serialize_serialize_12_serialize_12_vec(__m256i vector)
{
  __m256i
  adjacent_2_combined = libcrux_ml_kem_vector_avx2_serialize_mm256_concat_pairs_n(12U, vector);
  __m256i
  adjacent_4_combined =
    libcrux_intrinsics_avx2_mm256_sllv_epi32(adjacent_2_combined,
      libcrux_intrinsics_avx2_mm256_set_epi32(0, 8, 0, 8, 0, 8, 0, 8));
  __m256i
  adjacent_4_combined0 =
    libcrux_intrinsics_avx2_mm256_srli_epi64(8,
      adjacent_4_combined,
      __m256i);
  __m256i
  adjacent_8_combined =
    libcrux_intrinsics_avx2_mm256_shuffle_epi8(adjacent_4_combined0,
      libcrux_intrinsics_avx2_mm256_set_epi8(-1,
        -1,
        -1,
        -1,
        13,
        12,
        11,
        10,
        9,
        8,
        5,
        4,
        3,
        2,
        1,
        0,
        -1,
        -1,
        -1,
        -1,
        13,
        12,
        11,
        10,
        9,
        8,
        5,
        4,
        3,
        2,
        1,
        0));
  __m128i lower_8 = libcrux_intrinsics_avx2_mm256_castsi256_si128(adjacent_8_combined);
  __m128i
  upper_8 = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, adjacent_8_combined, __m128i);
  return (core_core_arch_x86___m128i_x2{ lower_8, upper_8 });
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_94
libcrux_ml_kem_vector_avx2_serialize_serialize_12(__m256i vector)
{
  Eurydice_arr_ec serialized = { { 0U } };
  core_core_arch_x86___m128i_x2
  uu____0 = libcrux_ml_kem_vector_avx2_serialize_serialize_12_serialize_12_vec(vector);
  __m128i lower_8 = uu____0.fst;
  __m128i upper_8 = uu____0.snd;
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_subslice_mut_d44(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)16U })),
    lower_8);
  libcrux_intrinsics_avx2_mm_storeu_bytes_si128(Eurydice_array_to_subslice_mut_d44(&serialized,
      (core_ops_range_Range_87{ (size_t)12U, (size_t)28U })),
    upper_8);
  Eurydice_arr_94 arr;
  memcpy(arr.data,
    Eurydice_array_to_subslice_shared_d41(&serialized,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)24U })).ptr,
    (size_t)24U * sizeof (uint8_t));
  return
    core_result_unwrap_26_78(core_result_Result_57_s(core_result_Ok,
        &core_result_Result_57_s::U::case_Ok,
        arr));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_94
libcrux_ml_kem_vector_avx2_serialize_12(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_serialize_12(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_94
libcrux_ml_kem_vector_avx2_serialize_12_f5(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_serialize_12(vector);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_12_deserialize_12_vec(
  __m128i lower_coefficients0,
  __m128i upper_coefficients0
)
{
  __m128i
  lower_coefficients =
    libcrux_intrinsics_avx2_mm_shuffle_epi8(lower_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(11, 10, 10, 9, 8, 7, 7, 6, 5, 4, 4, 3, 2, 1, 1, 0));
  __m128i
  upper_coefficients =
    libcrux_intrinsics_avx2_mm_shuffle_epi8(upper_coefficients0,
      libcrux_intrinsics_avx2_mm_set_epi8(15, 14, 14, 13, 12, 11, 11, 10, 9, 8, 8, 7, 6, 5, 5, 4));
  __m256i
  coefficients =
    libcrux_ml_kem_vector_avx2_serialize_mm256_si256_from_two_si128(lower_coefficients,
      upper_coefficients);
  __m256i
  coefficients0 =
    libcrux_intrinsics_avx2_mm256_mullo_epi16(coefficients,
      libcrux_intrinsics_avx2_mm256_set_epi16((int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U),
        (int16_t)((uint32_t)1 << 0U),
        (int16_t)((uint32_t)1 << 4U)));
  __m256i coefficients1 = libcrux_intrinsics_avx2_mm256_srli_epi16(4, coefficients0, __m256i);
  return
    libcrux_intrinsics_avx2_mm256_and_si256(coefficients1,
      libcrux_intrinsics_avx2_mm256_set1_epi16((int16_t)((uint32_t)1 << 12U) - 1));
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_serialize_deserialize_12(Eurydice_borrow_slice_u8 bytes)
{
  __m128i
  lower_coefficients =
    libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_c8(bytes,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)16U })));
  __m128i
  upper_coefficients =
    libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_slice_subslice_shared_c8(bytes,
        (core_ops_range_Range_87{ (size_t)8U, (size_t)24U })));
  return
    libcrux_ml_kem_vector_avx2_serialize_deserialize_12_deserialize_12_vec(lower_coefficients,
      upper_coefficients);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_12(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_serialize_deserialize_12(bytes);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_deserialize_12_f5(Eurydice_borrow_slice_u8 bytes)
{
  return libcrux_ml_kem_vector_avx2_deserialize_12(bytes);
}

KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_kem_vector_avx2_sampling_rejection_sample(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_i16 output
)
{
  __m256i
  field_modulus =
    libcrux_intrinsics_avx2_mm256_set1_epi16(LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i potential_coefficients = libcrux_ml_kem_vector_avx2_serialize_deserialize_12(input);
  __m256i
  compare_with_field_modulus =
    libcrux_intrinsics_avx2_mm256_cmpgt_epi16(field_modulus,
      potential_coefficients);
  Eurydice_array_u8x2
  good = libcrux_ml_kem_vector_avx2_serialize_serialize_1(compare_with_field_modulus);
  Eurydice_arr_b2
  lower_shuffles =
    LIBCRUX_ML_KEM_VECTOR_REJ_SAMPLE_TABLE_REJECTION_SAMPLE_SHUFFLE_TABLE.data[(size_t)(uint32_t)good.data[0U]];
  __m128i
  lower_shuffles0 =
    libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_array_to_slice_shared_29(&lower_shuffles));
  __m128i
  lower_coefficients = libcrux_intrinsics_avx2_mm256_castsi256_si128(potential_coefficients);
  __m128i
  lower_coefficients0 =
    libcrux_intrinsics_avx2_mm_shuffle_epi8(lower_coefficients,
      lower_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128(output, lower_coefficients0);
  size_t sampled_count = (size_t)core_num__u8__count_ones(good.data[0U]);
  Eurydice_arr_b2
  upper_shuffles =
    LIBCRUX_ML_KEM_VECTOR_REJ_SAMPLE_TABLE_REJECTION_SAMPLE_SHUFFLE_TABLE.data[(size_t)(uint32_t)good.data[1U]];
  __m128i
  upper_shuffles0 =
    libcrux_intrinsics_avx2_mm_loadu_si128(Eurydice_array_to_slice_shared_29(&upper_shuffles));
  __m128i
  upper_coefficients =
    libcrux_intrinsics_avx2_mm256_extracti128_si256(1,
      potential_coefficients,
      __m128i);
  __m128i
  upper_coefficients0 =
    libcrux_intrinsics_avx2_mm_shuffle_epi8(upper_coefficients,
      upper_shuffles0);
  libcrux_intrinsics_avx2_mm_storeu_si128(Eurydice_slice_subslice_mut_a6(output,
      (core_ops_range_Range_87{ sampled_count, sampled_count + (size_t)8U })),
    upper_coefficients0);
  size_t uu____0 = sampled_count;
  return uu____0 + (size_t)core_num__u8__count_ones(good.data[1U]);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE size_t
libcrux_ml_kem_vector_avx2_rej_sample_f5(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_i16 output
)
{
  return libcrux_ml_kem_vector_avx2_sampling_rejection_sample(input, output);
}

#define LIBCRUX_ML_KEM_VECTOR_AVX2_NTT_NTT_MULTIPLY_PERMUTE_WITH (216)

/**
This function found in impl {core::clone::Clone for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline __m256i
libcrux_ml_kem_vector_avx2_clone_fd(const __m256i *self)
{
  return self[0U];
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_13_s { __m256i data[16U]; } Eurydice_arr_13;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_f6
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_600_s { Eurydice_arr_13 data[3U]; } Eurydice_arr_600;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_600
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_601_s { Eurydice_arr_600 data[3U]; } Eurydice_arr_601;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef_s
{
  Eurydice_arr_600 t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_601 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef;

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_polynomial_ZERO_d6_84(void)
{
  Eurydice_arr_13 lit;
  __m256i repeat_expression[16U];
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    repeat_expression[i] = libcrux_ml_kem_vector_avx2_ZERO_f5(););
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof (__m256i));
  return lit;
}

/**
 Only use with public values.

 This MUST NOT be used with secret inputs, like its caller `deserialize_ring_elements_reduced`.
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_to_reduced_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_84(
  Eurydice_borrow_slice_u8 serialized
)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)24U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)24U, i0 * (size_t)24U + (size_t)24U }));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_12_f5(bytes);
    re.data[i0] = libcrux_ml_kem_vector_avx2_cond_subtract_3329_f5(coefficient);
  }
  return re;
}

/**
 See [deserialize_ring_elements_reduced_out].
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_e3(
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_arr_600 *deserialized_pk
)
{
  for
  (size_t
    i = (size_t)0U;
    i < public_key.meta / LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT;
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    ring_element =
      Eurydice_slice_subslice_shared_c8(public_key,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT +
              LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
          }
        ));
    Eurydice_arr_13
    uu____0 = libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_84(ring_element);
    deserialized_pk->data[i0] = uu____0;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_78(const Eurydice_arr_810 *input)
{
  Eurydice_arr_c40 state = libcrux_sha3_avx2_x4_incremental_init();
  libcrux_sha3_avx2_x4_incremental_shake128_absorb_final(&state,
    Eurydice_array_to_slice_shared_e9(input->data),
    Eurydice_array_to_slice_shared_e9(&input->data[1U]),
    Eurydice_array_to_slice_shared_e9(&input->data[2U]),
    Eurydice_array_to_slice_shared_e9(input->data));
  return state;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_78(
  const Eurydice_arr_810 *input
)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_78(input);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7e
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_78(Eurydice_arr_c40 *st)
{
  Eurydice_arr_7e out = { { { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_79 out0 = { { 0U } };
  Eurydice_arr_79 out1 = { { 0U } };
  Eurydice_arr_79 out2 = { { 0U } };
  Eurydice_arr_79 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_three_blocks(st,
    Eurydice_array_to_slice_mut_48(&out0),
    Eurydice_array_to_slice_mut_48(&out1),
    Eurydice_array_to_slice_mut_48(&out2),
    Eurydice_array_to_slice_mut_48(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7e
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_78(
  Eurydice_arr_c40 *self
)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_78(self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT representation
 of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
 We say "partially" because this implementation only accepts a finite set of
 bytes as input and returns an error if the set is not enough; Algorithm 6 of
 the FIPS 203 standard on the other hand samples from an infinite stream of bytes
 until the ring element is filled. Algorithm 6 is reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_uniform_distribution_next
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- N= 504
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_79(
  const Eurydice_arr_7e *randomness,
  Eurydice_arr_eb0 *sampled_coefficients,
  Eurydice_arr_b1 *out
)
{
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)504U / (size_t)24U; i++)
    {
      size_t r = i;
      if (sampled_coefficients->data[i1] < LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        size_t
        sampled =
          libcrux_ml_kem_vector_avx2_rej_sample_f5(Eurydice_array_to_subslice_shared_d46(&randomness->data[i1],
              (core_ops_range_Range_87{ r * (size_t)24U, r * (size_t)24U + (size_t)24U })),
            Eurydice_array_to_subslice_mut_e7(&out->data[i1],
              (
                core_ops_range_Range_87{
                  sampled_coefficients->data[i1],
                  sampled_coefficients->data[i1] + (size_t)16U
                }
              )));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    });
  bool done = true;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >= LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
    {
      sampled_coefficients->data[i0] = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    }
    else
    {
      done = false;
    });
  return done;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_2c
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_78(Eurydice_arr_c40 *st)
{
  Eurydice_arr_2c out = { { { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_c5 out0 = { { 0U } };
  Eurydice_arr_c5 out1 = { { 0U } };
  Eurydice_arr_c5 out2 = { { 0U } };
  Eurydice_arr_c5 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_next_block(st,
    Eurydice_array_to_slice_mut_2c(&out0),
    Eurydice_array_to_slice_mut_2c(&out1),
    Eurydice_array_to_slice_mut_2c(&out2),
    Eurydice_array_to_slice_mut_2c(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_2c
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_78(Eurydice_arr_c40 *self)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_78(self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT representation
 of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
 We say "partially" because this implementation only accepts a finite set of
 bytes as input and returns an error if the set is not enough; Algorithm 6 of
 the FIPS 203 standard on the other hand samples from an infinite stream of bytes
 until the ring element is filled. Algorithm 6 is reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_uniform_distribution_next
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- N= 168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_790(
  const Eurydice_arr_2c *randomness,
  Eurydice_arr_eb0 *sampled_coefficients,
  Eurydice_arr_b1 *out
)
{
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)24U; i++)
    {
      size_t r = i;
      if (sampled_coefficients->data[i1] < LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        size_t
        sampled =
          libcrux_ml_kem_vector_avx2_rej_sample_f5(Eurydice_array_to_subslice_shared_d40(&randomness->data[i1],
              (core_ops_range_Range_87{ r * (size_t)24U, r * (size_t)24U + (size_t)24U })),
            Eurydice_array_to_subslice_mut_e7(&out->data[i1],
              (
                core_ops_range_Range_87{
                  sampled_coefficients->data[i1],
                  sampled_coefficients->data[i1] + (size_t)16U
                }
              )));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    });
  bool done = true;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >= LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
    {
      sampled_coefficients->data[i0] = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    }
    else
    {
      done = false;
    });
  return done;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_polynomial_ZERO_84(void)
{
  Eurydice_arr_13 lit;
  __m256i repeat_expression[16U];
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    repeat_expression[i] = libcrux_ml_kem_vector_avx2_ZERO_f5(););
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof (__m256i));
  return lit;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.from_i16_array
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_from_i16_array_84(Eurydice_borrow_slice_i16 a)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_84();
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    result.data[i0] =
      libcrux_ml_kem_vector_avx2_from_i16_array_f5(Eurydice_slice_subslice_shared_a6(a,
          (core_ops_range_Range_87{ i0 * (size_t)16U, (i0 + (size_t)1U) * (size_t)16U })));
  }
  return result;
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.from_i16_array_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_borrow_slice_i16 a)
{
  return libcrux_ml_kem_polynomial_from_i16_array_84(a);
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector, Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_281(void **_, Eurydice_arr_5b tupled_args)
{
  Eurydice_arr_5b s = tupled_args;
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_subslice_shared_e70(&s,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)256U })));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_sampling_sample_from_xof_281(const Eurydice_arr_810 *seeds)
{
  Eurydice_arr_eb0 sampled_coefficients = { { 0U } };
  Eurydice_arr_b1 out = { { { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_c40
  xof_state = libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_78(seeds);
  Eurydice_arr_7e
  randomness0 =
    libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_78(&xof_state);
  bool
  done =
    libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_79(&randomness0,
      &sampled_coefficients,
      &out);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_2c
      randomness = libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_78(&xof_state);
      done =
        libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_790(&randomness,
          &sampled_coefficients,
          &out);
    }
  }
  Eurydice_arr_600 arr_mapped_str;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
      libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_281(&lvalue,
        out.data[i]););
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_sample_matrix_A_281(
  Eurydice_arr_601 *A_transpose,
  const Eurydice_arr_31 *seed,
  bool transpose
)
{
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i1 = i0;
    Eurydice_arr_810 seeds;
    Eurydice_arr_31 repeat_expression[3U];
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      repeat_expression[i] =
        core_array__core__clone__Clone_for__T__N___clone((size_t)34U,
          seed,
          uint8_t,
          Eurydice_arr_31););
    memcpy(seeds.data, repeat_expression, (size_t)3U * sizeof (Eurydice_arr_31));
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;);
    Eurydice_arr_600 sampled = libcrux_ml_kem_sampling_sample_from_xof_281(&seeds);
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 sample = sampled.data[j];
      if (transpose)
      {
        A_transpose->data[j].data[i1] = sample;
      }
      else
      {
        A_transpose->data[i1].data[j] = sample;
      }););
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.H_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_H_41_78(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_H(input);
}

/**
 Generate an unpacked key from a serialized key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.unpack_public_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a1(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
)
{
  Eurydice_borrow_slice_u8
  uu____0 = Eurydice_array_to_subslice_to_shared_213(public_key, (size_t)1152U);
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_e3(uu____0,
    &unpacked_public_key->ind_cpa_public_key.t_as_ntt);
  unpacked_public_key->ind_cpa_public_key.seed_for_A =
    libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_array_to_subslice_from_shared_5f4(public_key,
        (size_t)1152U));
  Eurydice_arr_601 *uu____2 = &unpacked_public_key->ind_cpa_public_key.A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31
  lvalue =
    libcrux_ml_kem_utils_into_padded_array_de(Eurydice_array_to_subslice_from_shared_5f4(public_key,
        (size_t)1152U));
  libcrux_ml_kem_matrix_sample_matrix_A_281(uu____2, &lvalue, false);
  Eurydice_arr_ec
  uu____3 =
    libcrux_ml_kem_hash_functions_avx2_H_41_78(Eurydice_array_to_slice_shared_ff(libcrux_ml_kem_types_as_slice_e6_3d(public_key)));
  unpacked_public_key->public_key_hash = uu____3;
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_d3(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a1(public_key, unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_d3(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_d3(public_key,
    unpacked_public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef_s
{
  Eurydice_arr_600 ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef;

typedef struct libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef public_key;
}
libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked;

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE const
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
*libcrux_ml_kem_ind_cca_unpacked_public_key_11_e3(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self
)
{
  return &self->public_key;
}

/**
This function found in impl {core::clone::Clone for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.clone_91
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cpa_unpacked_clone_91_e3(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef *self
)
{
  Eurydice_arr_600
  uu____0 =
    core_array__core__clone__Clone_for__T__N___clone((size_t)3U,
      &self->t_as_ntt,
      Eurydice_arr_13,
      Eurydice_arr_600);
  Eurydice_arr_ec
  uu____1 =
    core_array__core__clone__Clone_for__T__N___clone((size_t)32U,
      &self->seed_for_A,
      uint8_t,
      Eurydice_arr_ec);
  return
    (
      libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef{
        uu____0,
        uu____1,
        core_array__core__clone__Clone_for__T__N___clone((size_t)3U,
          &self->A,
          Eurydice_arr_600,
          Eurydice_arr_601)
      }
    );
}

/**
This function found in impl {core::clone::Clone for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.clone_d7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cca_unpacked_clone_d7_e3(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *self
)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
  uu____0 = libcrux_ml_kem_ind_cpa_unpacked_clone_91_e3(&self->ind_cpa_public_key);
  return
    (
      libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef{
        uu____0,
        core_array__core__clone__Clone_for__T__N___clone((size_t)32U,
          &self->public_key_hash,
          uint8_t,
          Eurydice_arr_ec)
      }
    );
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.to_unsigned_field_modulus
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(__m256i a)
{
  return libcrux_ml_kem_vector_avx2_to_unsigned_representative_f5(a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.serialize_uncompressed_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b20
libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_84(const Eurydice_arr_13 *re)
{
  Eurydice_arr_b20 serialized = { { 0U } };
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i coefficient = libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re->data[i0]);
    Eurydice_arr_94 bytes = libcrux_ml_kem_vector_avx2_serialize_12_f5(coefficient);
    Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d415(&serialized,
        (core_ops_range_Range_87{ (size_t)24U * i0, (size_t)24U * i0 + (size_t)24U })),
      Eurydice_array_to_slice_shared_ed(&bytes),
      uint8_t);
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_serialize_vector_e3(
  const Eurydice_arr_600 *key,
  Eurydice_mut_borrow_slice_u8 out
)
{
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13 re = key->data[i0];
    Eurydice_mut_borrow_slice_u8
    uu____0 =
      Eurydice_slice_subslice_mut_c8(out,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            (i0 + (size_t)1U) * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
          }
        ));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b20 lvalue = libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_84(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_a9(&lvalue), uint8_t););
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_serialize_public_key_mut_79(
  const Eurydice_arr_600 *t_as_ntt,
  Eurydice_borrow_slice_u8 seed_for_a,
  Eurydice_arr_5f *serialized
)
{
  libcrux_ml_kem_ind_cpa_serialize_vector_e3(t_as_ntt,
    Eurydice_array_to_subslice_mut_d419(serialized,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)3U)
        }
      )));
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f6(serialized,
      libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)3U)),
    seed_for_a,
    uint8_t);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
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
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_79(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *self,
  Eurydice_arr_5f *serialized
)
{
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_79(&self->ind_cpa_public_key.t_as_ntt,
    Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A),
    serialized);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_79(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self,
  Eurydice_arr_5f *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_79(&self->public_key, serialized);
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
static KRML_MUSTINLINE Eurydice_arr_5f
libcrux_ml_kem_ind_cpa_serialize_public_key_79(
  const Eurydice_arr_600 *t_as_ntt,
  Eurydice_borrow_slice_u8 seed_for_a
)
{
  Eurydice_arr_5f public_key_serialized = { { 0U } };
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_79(t_as_ntt,
    seed_for_a,
    &public_key_serialized);
  return public_key_serialized;
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_5f
libcrux_ml_kem_ind_cca_unpacked_serialized_dd_79(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *self
)
{
  return
    libcrux_ml_kem_types_from_51_3d(libcrux_ml_kem_ind_cpa_serialize_public_key_79(&self->ind_cpa_public_key.t_as_ntt,
        Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A)));
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_5f
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_79(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_dd_79(&self->public_key);
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
libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_15(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_600 *private_key
)
{
  Eurydice_arr_5f
  public_key_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_79(&public_key->t_as_ntt,
      Eurydice_array_to_slice_shared_01(&public_key->seed_for_A));
  Eurydice_arr_0e secret_key_serialized = { { 0U } };
  libcrux_ml_kem_ind_cpa_serialize_vector_e3(private_key,
    Eurydice_array_to_slice_mut_f4(&secret_key_serialized));
  return
    (
      libcrux_ml_kem_utils_extraction_helper_Keypair768{
        secret_key_serialized,
        public_key_serialized
      }
    );
}

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_d4(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self,
  Eurydice_arr_7d *serialized
)
{
  libcrux_ml_kem_utils_extraction_helper_Keypair768
  uu____0 =
    libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_15(&self->public_key.ind_cpa_public_key,
      &self->private_key.ind_cpa_private_key);
  Eurydice_arr_0e ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_5f ind_cpa_public_key = uu____0.snd;
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_52(Eurydice_array_to_slice_shared_f4(&ind_cpa_private_key),
    Eurydice_array_to_slice_shared_ff(&ind_cpa_public_key),
    Eurydice_array_to_slice_shared_01(&self->private_key.implicit_rejection_value),
    serialized);
}

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7d
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_d4(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self
)
{
  Eurydice_arr_7d sk = libcrux_ml_kem_types_default_d3_79();
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_d4(self, &sk);
  return sk;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_to_uncompressed_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_84(
  Eurydice_borrow_slice_u8 serialized
)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)24U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)24U, i0 * (size_t)24U + (size_t)24U }));
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_deserialize_vector_e3(
  Eurydice_borrow_slice_u8 secret_key,
  Eurydice_arr_600 *secret_as_ntt
)
{
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_84(Eurydice_slice_subslice_shared_c8(secret_key,
          (
            core_ops_range_Range_87{
              i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
              (i0 + (size_t)1U) * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
            }
          )));
    secret_as_ntt->data[i0] = uu____0;);
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector, Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_7e1(void **_, Eurydice_arr_5b tupled_args)
{
  Eurydice_arr_5b s = tupled_args;
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_subslice_shared_e70(&s,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)256U })));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_sampling_sample_from_xof_7e1(const Eurydice_arr_810 *seeds)
{
  Eurydice_arr_eb0 sampled_coefficients = { { 0U } };
  Eurydice_arr_b1 out = { { { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_1b0
  xof_state = libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_78(seeds);
  Eurydice_arr_7e
  randomness0 =
    libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_78(&xof_state);
  bool
  done =
    libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_79(&randomness0,
      &sampled_coefficients,
      &out);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_2c
      randomness =
        libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_78(&xof_state);
      done =
        libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_790(&randomness,
          &sampled_coefficients,
          &out);
    }
  }
  Eurydice_arr_600 arr_mapped_str;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
      libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_7e1(&lvalue,
        out.data[i]););
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_sample_matrix_A_7e1(
  Eurydice_arr_601 *A_transpose,
  const Eurydice_arr_31 *seed,
  bool transpose
)
{
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i1 = i0;
    Eurydice_arr_810 seeds;
    Eurydice_arr_31 repeat_expression[3U];
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      repeat_expression[i] =
        core_array__core__clone__Clone_for__T__N___clone((size_t)34U,
          seed,
          uint8_t,
          Eurydice_arr_31););
    memcpy(seeds.data, repeat_expression, (size_t)3U * sizeof (Eurydice_arr_31));
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;);
    Eurydice_arr_600 sampled = libcrux_ml_kem_sampling_sample_from_xof_7e1(&seeds);
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 sample = sampled.data[j];
      if (transpose)
      {
        A_transpose->data[j].data[i1] = sample;
      }
      else
      {
        A_transpose->data[i1].data[j] = sample;
      }););
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_101(
  Eurydice_borrow_slice_u8 public_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef *unpacked_public_key
)
{
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_e3(Eurydice_slice_subslice_to_shared_72(public_key,
      (size_t)1152U),
    &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8
  seed = Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)1152U);
  Eurydice_arr_601 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_7e1(uu____0, &lvalue, false);
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
libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_3e(
  const Eurydice_arr_7d *private_key,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
)
{
  Eurydice_borrow_slice_u8_x4
  uu____0 =
    libcrux_ml_kem_types_unpack_private_key_64(Eurydice_array_to_slice_shared_51(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  libcrux_ml_kem_ind_cpa_deserialize_vector_e3(ind_cpa_secret_key,
    &key_pair->private_key.ind_cpa_private_key);
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_101(ind_cpa_public_key,
    &key_pair->public_key.ind_cpa_public_key);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->public_key.public_key_hash),
    ind_cpa_public_key_hash,
    uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->private_key.implicit_rejection_value),
    implicit_rejection_value,
    uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->public_key.ind_cpa_public_key.seed_for_A),
    Eurydice_slice_subslice_from_shared_6d(ind_cpa_public_key, (size_t)1152U),
    uint8_t);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_71(
  const Eurydice_arr_7d *private_key,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_3e(private_key, key_pair);
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPrivateKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_70
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cpa_unpacked_default_70_e3(void)
{
  Eurydice_arr_600 lit;
  Eurydice_arr_13 repeat_expression[3U];
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
  memcpy(lit.data, repeat_expression, (size_t)3U * sizeof (Eurydice_arr_13));
  return lit;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_8b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cpa_unpacked_default_8b_e3(void)
{
  Eurydice_arr_600 uu____0;
  Eurydice_arr_13 repeat_expression0[3U];
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    repeat_expression0[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
  memcpy(uu____0.data, repeat_expression0, (size_t)3U * sizeof (Eurydice_arr_13));
  Eurydice_arr_ec uu____1 = { { 0U } };
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef lit0;
  lit0.t_as_ntt = uu____0;
  lit0.seed_for_A = uu____1;
  Eurydice_arr_600 repeat_expression1[3U];
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    Eurydice_arr_600 lit;
    Eurydice_arr_13 repeat_expression[3U];
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
    memcpy(lit.data, repeat_expression, (size_t)3U * sizeof (Eurydice_arr_13));
    repeat_expression1[i0] = lit;);
  memcpy(lit0.A.data, repeat_expression1, (size_t)3U * sizeof (Eurydice_arr_600));
  return lit0;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cca_unpacked_default_30_e3(void)
{
  return
    (
      libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef{
        libcrux_ml_kem_ind_cpa_unpacked_default_8b_e3(),
        { { 0U } }
      }
    );
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_e3(void)
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef
  uu____0 = { libcrux_ml_kem_ind_cpa_unpacked_default_70_e3(), { { 0U } } };
  return
    (
      libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked{
        uu____0,
        libcrux_ml_kem_ind_cca_unpacked_default_30_e3()
      }
    );
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.G_41
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_hash_functions_avx2_G_41_78(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_G(input);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.cpa_keygen_seed_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_variant_cpa_keygen_seed_39_b6(Eurydice_borrow_slice_u8 key_generation_seed)
{
  Eurydice_arr_fa seed = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d412(&seed,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      )),
    key_generation_seed,
    uint8_t);
  seed.data[LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE] = (uint8_t)(size_t)3U;
  return libcrux_ml_kem_hash_functions_avx2_G_41_78(Eurydice_array_to_slice_shared_b5(&seed));
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN
with const generics
- K= 3
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_58
libcrux_ml_kem_hash_functions_avx2_PRFxN_3b(const Eurydice_arr_801 *input)
{
  Eurydice_arr_58 out = { { { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_89 out0 = { { 0U } };
  Eurydice_arr_89 out1 = { { 0U } };
  Eurydice_arr_89 out2 = { { 0U } };
  Eurydice_arr_89 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_shake256(Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_shared_b5(&input->data[1U]),
    Eurydice_array_to_slice_shared_b5(&input->data[2U]),
    Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_mut_78(&out0),
    Eurydice_array_to_slice_mut_78(&out1),
    Eurydice_array_to_slice_mut_78(&out2),
    Eurydice_array_to_slice_mut_78(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN_41
with const generics
- K= 3
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_58
libcrux_ml_kem_hash_functions_avx2_PRFxN_41_3b(const Eurydice_arr_801 *input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRFxN_3b(input);
}

/**
 Given a series of uniformly random bytes in `randomness`, for some number `eta`,
 the `sample_from_binomial_distribution_{eta}` functions sample
 a ring element from a binomial distribution centered at 0 that uses two sets
 of `eta` coin flips. If, for example,
 `eta = ETA`, each ring coefficient is a value `v` such
 such that `v ∈ {-ETA, -ETA + 1, ..., 0, ..., ETA + 1, ETA}` and:

 ```plaintext
 - If v < 0, Pr[v] = Pr[-v]
 - If v >= 0, Pr[v] = BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2 ^ (2 * ETA)
 ```

 The values `v < 0` are mapped to the appropriate `KyberFieldElement`.

 The expected value is:

 ```plaintext
 E[X] = (-ETA)Pr[-ETA] + (-(ETA - 1))Pr[-(ETA - 1)] + ... + (ETA - 1)Pr[ETA - 1] + (ETA)Pr[ETA]
      = 0 since Pr[-v] = Pr[v] when v < 0.
 ```

 And the variance is:

 ```plaintext
 Var(X) = E[(X - E[X])^2]
        = E[X^2]
        = sum_(v=-ETA to ETA)v^2 * (BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2^(2 * ETA))
        = ETA / 2
 ```

 This function implements <strong>Algorithm 7</strong> of the NIST FIPS 203 standard, which is
 reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_binomial_distribution_2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_84(
  Eurydice_borrow_slice_u8 randomness
)
{
  Eurydice_arr_04 sampled_i16s = { { 0U } };
  for (size_t i0 = (size_t)0U; i0 < randomness.meta / (size_t)4U; i0++)
  {
    size_t chunk_number = i0;
    Eurydice_borrow_slice_u8
    byte_chunk =
      Eurydice_slice_subslice_shared_c8(randomness,
        (
          core_ops_range_Range_87{
            chunk_number * (size_t)4U,
            chunk_number * (size_t)4U + (size_t)4U
          }
        ));
    uint32_t
    random_bits_as_u32 =
      (((uint32_t)byte_chunk.ptr[0U] | (uint32_t)byte_chunk.ptr[1U] << 8U) |
        (uint32_t)byte_chunk.ptr[2U] << 16U)
      | (uint32_t)byte_chunk.ptr[3U] << 24U;
    uint32_t even_bits = random_bits_as_u32 & 1431655765U;
    uint32_t odd_bits = random_bits_as_u32 >> 1U & 1431655765U;
    uint32_t coin_toss_outcomes = even_bits + odd_bits;
    for (uint32_t i = 0U; i < 32U / 4U; i++)
    {
      uint32_t outcome_set = i;
      uint32_t outcome_set0 = outcome_set * 4U;
      int16_t outcome_1 = (int16_t)(coin_toss_outcomes >> (uint32_t)outcome_set0 & 3U);
      int16_t outcome_2 = (int16_t)(coin_toss_outcomes >> (uint32_t)(outcome_set0 + 2U) & 3U);
      size_t offset = (size_t)(outcome_set0 >> 2U);
      sampled_i16s.data[(size_t)8U * chunk_number + offset] = outcome_1 - outcome_2;
    }
  }
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_slice_shared_990(&sampled_i16s));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_binomial_distribution_3
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_binomial_distribution_3_84(
  Eurydice_borrow_slice_u8 randomness
)
{
  Eurydice_arr_04 sampled_i16s = { { 0U } };
  for (size_t i0 = (size_t)0U; i0 < randomness.meta / (size_t)3U; i0++)
  {
    size_t chunk_number = i0;
    Eurydice_borrow_slice_u8
    byte_chunk =
      Eurydice_slice_subslice_shared_c8(randomness,
        (
          core_ops_range_Range_87{
            chunk_number * (size_t)3U,
            chunk_number * (size_t)3U + (size_t)3U
          }
        ));
    uint32_t
    random_bits_as_u24 =
      ((uint32_t)byte_chunk.ptr[0U] | (uint32_t)byte_chunk.ptr[1U] << 8U) |
        (uint32_t)byte_chunk.ptr[2U] << 16U;
    uint32_t first_bits = random_bits_as_u24 & 2396745U;
    uint32_t second_bits = random_bits_as_u24 >> 1U & 2396745U;
    uint32_t third_bits = random_bits_as_u24 >> 2U & 2396745U;
    uint32_t coin_toss_outcomes = first_bits + second_bits + third_bits;
    for (int32_t i = 0; i < 24 / 6; i++)
    {
      int32_t outcome_set = i;
      int32_t outcome_set0 = outcome_set * 6;
      int16_t outcome_1 = (int16_t)(coin_toss_outcomes >> (uint32_t)outcome_set0 & 7U);
      int16_t outcome_2 = (int16_t)(coin_toss_outcomes >> (uint32_t)(outcome_set0 + 3) & 7U);
      size_t offset = (size_t)(outcome_set0 / 6);
      sampled_i16s.data[(size_t)4U * chunk_number + offset] = outcome_1 - outcome_2;
    }
  }
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_slice_shared_990(&sampled_i16s));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_binomial_distribution
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- ETA= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(
  Eurydice_borrow_slice_u8 randomness
)
{
  return libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_84(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_at_layer_7_84(Eurydice_arr_13 *re)
{
  size_t step = LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT / (size_t)2U;
  for (size_t i = (size_t)0U; i < step; i++)
  {
    size_t j = i;
    __m256i t = libcrux_ml_kem_vector_avx2_multiply_by_constant_f5(re->data[j + step], -1600);
    re->data[j + step] = libcrux_ml_kem_vector_avx2_sub_f5(re->data[j], &t);
    re->data[j] = libcrux_ml_kem_vector_avx2_add_f5(re->data[j], &t);
  }
}

typedef struct libcrux_ml_kem_vector_avx2_SIMD256Vector_x2_s
{
  __m256i fst;
  __m256i snd;
}
libcrux_ml_kem_vector_avx2_SIMD256Vector_x2;

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_layer_int_vec_step
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_vector_avx2_SIMD256Vector_x2
libcrux_ml_kem_ntt_ntt_layer_int_vec_step_84(__m256i a, __m256i b, int16_t zeta_r)
{
  __m256i t = libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(b, zeta_r);
  b = libcrux_ml_kem_vector_avx2_sub_f5(a, &t);
  a = libcrux_ml_kem_vector_avx2_add_f5(a, &t);
  return (libcrux_ml_kem_vector_avx2_SIMD256Vector_x2{ a, b });
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(
  size_t *zeta_i,
  Eurydice_arr_13 *re,
  size_t layer,
  size_t _initial_coefficient_bound
)
{
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++)
  {
    size_t round = i0;
    zeta_i[0U]++;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec = offset / (size_t)16U;
    size_t step_vec = step / (size_t)16U;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++)
    {
      size_t j = i;
      libcrux_ml_kem_vector_avx2_SIMD256Vector_x2
      uu____0 =
        libcrux_ml_kem_ntt_ntt_layer_int_vec_step_84(re->data[j],
          re->data[j + step_vec],
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_at_layer_3_84(
  size_t *zeta_i,
  Eurydice_arr_13 *re,
  size_t _initial_coefficient_bound
)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t round = i;
    zeta_i[0U]++;
    re->data[round] =
      libcrux_ml_kem_vector_avx2_ntt_layer_3_step_f5(re->data[round],
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U])););
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_at_layer_2_84(
  size_t *zeta_i,
  Eurydice_arr_13 *re,
  size_t _initial_coefficient_bound
)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t round = i;
    zeta_i[0U]++;
    re->data[round] =
      libcrux_ml_kem_vector_avx2_ntt_layer_2_step_f5(re->data[round],
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U));
    zeta_i[0U]++;);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_at_layer_1_84(
  size_t *zeta_i,
  Eurydice_arr_13 *re,
  size_t _initial_coefficient_bound
)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t round = i;
    zeta_i[0U]++;
    re->data[round] =
      libcrux_ml_kem_vector_avx2_ntt_layer_1_step_f5(re->data[round],
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)3U));
    zeta_i[0U] += (size_t)3U;);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_poly_barrett_reduce_84(Eurydice_arr_13 *myself)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    myself->data[i0] = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(myself->data[i0]);
  }
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(Eurydice_arr_13 *self)
{
  libcrux_ml_kem_polynomial_poly_barrett_reduce_84(self);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_binomially_sampled_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_84(Eurydice_arr_13 *re)
{
  libcrux_ml_kem_ntt_ntt_at_layer_7_84(re);
  size_t zeta_i = (size_t)1U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U, (size_t)11207U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i,
    re,
    (size_t)5U,
    (size_t)11207U + (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i,
    re,
    (size_t)4U,
    (size_t)11207U + (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_3_84(&zeta_i, re, (size_t)11207U + (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_2_84(&zeta_i, re, (size_t)11207U + (size_t)4U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_1_84(&zeta_i, re, (size_t)11207U + (size_t)5U * (size_t)3328U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
 Sample a vector of ring elements from a centered binomial distribution and
 convert them into their NTT representations.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_vector_cbd_then_ntt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- ETA= 2
- ETA_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d61(
  Eurydice_arr_600 *re_as_ntt,
  const Eurydice_arr_fa *prf_input,
  uint8_t domain_separator
)
{
  Eurydice_arr_801 prf_inputs;
  Eurydice_arr_fa repeat_expression[3U];
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    repeat_expression[i] =
      core_array__core__clone__Clone_for__T__N___clone((size_t)33U,
        prf_input,
        uint8_t,
        Eurydice_arr_fa););
  memcpy(prf_inputs.data, repeat_expression, (size_t)3U * sizeof (Eurydice_arr_fa));
  domain_separator = libcrux_ml_kem_utils_prf_input_inc_78(&prf_inputs, domain_separator);
  Eurydice_arr_58 prf_outputs = libcrux_ml_kem_hash_functions_avx2_PRFxN_41_3b(&prf_inputs);
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    re_as_ntt->data[i0] = uu____0;
    libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_84(&re_as_ntt->data[i0]););
  return domain_separator;
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@3]> for libcrux_ml_kem::ind_cpa::generate_keypair_unpacked::closure<Vector, Hasher, Scheme, K, ETA1, ETA1_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3, TraitClause@4, TraitClause@5]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair_unpacked.call_mut_73
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_ab1(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Given two `KyberPolynomialRingElement`s in their NTT representations,
 compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
 the `iᵗʰ` coefficient of the product `k̂` is determined by the calculation:

 ```plaintext
 ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X² - ζ^(2·BitRev₇(i) + 1))
 ```

 This function almost implements <strong>Algorithm 10</strong> of the
 NIST FIPS 203 standard, which is reproduced below:

 ```plaintext
 Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
 Output: An array ĥ ∈ ℤq.

 for(i ← 0; i < 128; i++)
     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1], ζ^(2·BitRev₇(i) + 1))
 end for
 return ĥ
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
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_ntt_multiply_84(
  const Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *rhs
)
{
  Eurydice_arr_13 out = libcrux_ml_kem_polynomial_ZERO_84();
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    out.data[i0] =
      libcrux_ml_kem_vector_avx2_ntt_multiply_f5(&myself->data[i0],
        &rhs->data[i0],
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 + (size_t)1U),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 + (size_t)2U),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 + (size_t)3U));
  }
  return out;
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ntt_multiply_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_ntt_multiply_d6_84(
  const Eurydice_arr_13 *self,
  const Eurydice_arr_13 *rhs
)
{
  return libcrux_ml_kem_polynomial_ntt_multiply_84(self, rhs);
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
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_to_ring_element_e3(
  Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *rhs
)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t i0 = i;
    myself->data[i0] = libcrux_ml_kem_vector_avx2_add_f5(myself->data[i0], &rhs->data[i0]););
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_to_ring_element_d6_e3(
  Eurydice_arr_13 *self,
  const Eurydice_arr_13 *rhs
)
{
  libcrux_ml_kem_polynomial_add_to_ring_element_e3(self, rhs);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.to_standard_domain
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_polynomial_to_standard_domain_84(__m256i vector)
{
  return
    libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(vector,
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
  Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *error
)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t j = i;
    __m256i
    coefficient_normal_form = libcrux_ml_kem_polynomial_to_standard_domain_84(myself->data[j]);
    __m256i sum = libcrux_ml_kem_vector_avx2_add_f5(coefficient_normal_form, &error->data[j]);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(sum);
    myself->data[j] = red;
  }
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_standard_error_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_84(
  Eurydice_arr_13 *self,
  const Eurydice_arr_13 *error
)
{
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
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_compute_As_plus_e_e3(
  Eurydice_arr_600 *t_as_ntt,
  const Eurydice_arr_601 *matrix_A,
  const Eurydice_arr_600 *s_as_ntt,
  const Eurydice_arr_600 *error_as_ntt
)
{
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    const Eurydice_arr_600 *row = &matrix_A->data[i0];
    Eurydice_arr_13 uu____0 = libcrux_ml_kem_polynomial_ZERO_d6_84();
    t_as_ntt->data[i0] = uu____0;
    KRML_MAYBE_FOR3(i1,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i1;
      const Eurydice_arr_13 *matrix_element = &row->data[j];
      Eurydice_arr_13
      product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(matrix_element, &s_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_e3(&t_as_ntt->data[i0], &product););
    libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_84(&t_as_ntt->data[i0],
      &error_as_ntt->data[i0]););
}

/**
 This function implements most of <strong>Algorithm 12</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE key generation algorithm.

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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab1(
  Eurydice_borrow_slice_u8 key_generation_seed,
  Eurydice_arr_600 *private_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef *public_key
)
{
  Eurydice_arr_c7 hashed = libcrux_ml_kem_variant_cpa_keygen_seed_39_b6(key_generation_seed);
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      (size_t)32U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_A = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_for_secret_and_error = uu____0.snd;
  Eurydice_arr_601 *uu____1 = &public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue0 = libcrux_ml_kem_utils_into_padded_array_de(seed_for_A);
  libcrux_ml_kem_matrix_sample_matrix_A_281(uu____1, &lvalue0, true);
  Eurydice_arr_fa
  prf_input = libcrux_ml_kem_utils_into_padded_array_29(seed_for_secret_and_error);
  uint8_t
  domain_separator =
    libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d61(private_key,
      &prf_input,
      0U);
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_ab1(&lvalue,
        i););
  Eurydice_arr_600 error_as_ntt = arr_struct;
  libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d61(&error_as_ntt,
    &prf_input,
    domain_separator);
  libcrux_ml_kem_matrix_compute_As_plus_e_e3(&public_key->t_as_ntt,
    &public_key->A,
    &private_key[0U],
    &error_as_ntt);
  Eurydice_arr_ec arr;
  memcpy(arr.data, seed_for_A.ptr, (size_t)32U * sizeof (uint8_t));
  Eurydice_arr_ec
  uu____2 =
    core_result_unwrap_26_39(core_result_Result_07_s(core_result_Ok,
        &core_result_Result_07_s::U::case_Ok,
        arr));
  public_key->seed_for_A = uu____2;
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a.closure.call_mut_b4
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_e3(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize), [libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]; K]> for libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a.call_mut_22
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_e3(void **_, size_t tupled_args)
{
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_e3(&lvalue,
        i););
  return arr_struct;
}

/**
This function found in impl {core::clone::Clone for libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.clone_c1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_polynomial_clone_c1_84(const Eurydice_arr_13 *self)
{
  return
    core_array__core__clone__Clone_for__T__N___clone((size_t)16U,
      self,
      __m256i,
      Eurydice_arr_13);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_601
libcrux_ml_kem_ind_cca_unpacked_transpose_a_e3(Eurydice_arr_601 ind_cpa_a)
{
  Eurydice_arr_601 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_e3(&lvalue, i););
  Eurydice_arr_601 A = arr_struct;
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i1 = i0;
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 uu____0 = libcrux_ml_kem_polynomial_clone_c1_84(&ind_cpa_a.data[j].data[i1]);
      A.data[i1].data[j] = uu____0;););
  return A;
}

/**
 Generate Unpacked Keys
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db1(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out
)
{
  Eurydice_borrow_slice_u8
  ind_cpa_keypair_randomness =
    Eurydice_array_to_subslice_shared_d47(&randomness,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      ));
  Eurydice_borrow_slice_u8
  implicit_rejection_value =
    Eurydice_array_to_subslice_from_shared_5f1(&randomness,
      LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab1(ind_cpa_keypair_randomness,
    &out->private_key.ind_cpa_private_key,
    &out->public_key.ind_cpa_public_key);
  Eurydice_arr_601
  A = libcrux_ml_kem_ind_cca_unpacked_transpose_a_e3(out->public_key.ind_cpa_public_key.A);
  out->public_key.ind_cpa_public_key.A = A;
  Eurydice_arr_5f
  pk_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_79(&out->public_key.ind_cpa_public_key.t_as_ntt,
      Eurydice_array_to_slice_shared_01(&out->public_key.ind_cpa_public_key.seed_for_A));
  Eurydice_arr_ec
  uu____0 =
    libcrux_ml_kem_hash_functions_avx2_H_41_78(Eurydice_array_to_slice_shared_ff(&pk_serialized));
  out->public_key.public_key_hash = uu____0;
  Eurydice_arr_ec arr;
  memcpy(arr.data, implicit_rejection_value.ptr, (size_t)32U * sizeof (uint8_t));
  Eurydice_arr_ec
  uu____1 =
    core_result_unwrap_26_39(core_result_Result_07_s(core_result_Ok,
        &core_result_Result_07_s::U::case_Ok,
        arr));
  out->private_key.implicit_rejection_value = uu____1;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_e9(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db1(randomness, out);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_e9(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_e9(randomness, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encaps_prepare
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_c7
libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_b6(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_borrow_slice_u8 pk_hash
)
{
  Eurydice_arr_c7 to_hash = libcrux_ml_kem_utils_into_padded_array_c9(randomness);
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f1(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE),
    pk_hash,
    uint8_t);
  return libcrux_ml_kem_hash_functions_avx2_G_41_78(Eurydice_array_to_slice_shared_17(&to_hash));
}

/**
A monomorphic instance of n-tuple
with types Eurydice_arr_600, libcrux_ml_kem_polynomial_PolynomialRingElement_f6

*/
typedef struct tuple_24_s
{
  Eurydice_arr_600 fst;
  Eurydice_arr_13 snd;
}
tuple_24;

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure<Vector, Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_f1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
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
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_781(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure#1<Vector, Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
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
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_781(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Sample a vector of ring elements from a centered binomial distribution.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_ring_element_cbd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- ETA2_RANDOMNESS_SIZE= 128
- ETA2= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_d61(
  const Eurydice_arr_fa *prf_input,
  uint8_t domain_separator,
  Eurydice_arr_600 *error_1
)
{
  Eurydice_arr_801 prf_inputs;
  Eurydice_arr_fa repeat_expression[3U];
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    repeat_expression[i] =
      core_array__core__clone__Clone_for__T__N___clone((size_t)33U,
        prf_input,
        uint8_t,
        Eurydice_arr_fa););
  memcpy(prf_inputs.data, repeat_expression, (size_t)3U * sizeof (Eurydice_arr_fa));
  domain_separator = libcrux_ml_kem_utils_prf_input_inc_78(&prf_inputs, domain_separator);
  Eurydice_arr_58 prf_outputs = libcrux_ml_kem_hash_functions_avx2_PRFxN_41_3b(&prf_inputs);
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    error_1->data[i0] = uu____0;);
  return domain_separator;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF
with const generics
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_89
libcrux_ml_kem_hash_functions_avx2_PRF_ec(Eurydice_borrow_slice_u8 input)
{
  Eurydice_arr_89 digest = { { 0U } };
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_78(&digest), input);
  return digest;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 3
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_89
libcrux_ml_kem_hash_functions_avx2_PRF_41_3b0(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRF_ec(input);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::matrix::compute_vector_u::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u.call_mut_a8
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_e3(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_84(size_t *zeta_i, Eurydice_arr_13 *re)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t round = i;
    zeta_i[0U]--;
    re->data[round] =
      libcrux_ml_kem_vector_avx2_inv_ntt_layer_1_step_f5(re->data[round],
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)3U));
    zeta_i[0U] -= (size_t)3U;);
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_84(size_t *zeta_i, Eurydice_arr_13 *re)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t round = i;
    zeta_i[0U]--;
    re->data[round] =
      libcrux_ml_kem_vector_avx2_inv_ntt_layer_2_step_f5(re->data[round],
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U));
    zeta_i[0U]--;);
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_3
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_84(size_t *zeta_i, Eurydice_arr_13 *re)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t round = i;
    zeta_i[0U]--;
    re->data[round] =
      libcrux_ml_kem_vector_avx2_inv_ntt_layer_3_step_f5(re->data[round],
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U])););
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.inv_ntt_layer_int_vec_step_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_vector_avx2_SIMD256Vector_x2
libcrux_ml_kem_invert_ntt_inv_ntt_layer_int_vec_step_reduce_84(
  __m256i a,
  __m256i b,
  int16_t zeta_r
)
{
  __m256i a_minus_b = libcrux_ml_kem_vector_avx2_sub_f5(b, &a);
  a = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(libcrux_ml_kem_vector_avx2_add_f5(a, &b));
  b = libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(a_minus_b, zeta_r);
  return (libcrux_ml_kem_vector_avx2_SIMD256Vector_x2{ a, b });
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(
  size_t *zeta_i,
  Eurydice_arr_13 *re,
  size_t layer
)
{
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++)
  {
    size_t round = i0;
    zeta_i[0U]--;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec = offset / LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR;
    size_t step_vec = step / LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++)
    {
      size_t j = i;
      libcrux_ml_kem_vector_avx2_SIMD256Vector_x2
      uu____0 =
        libcrux_ml_kem_invert_ntt_inv_ntt_layer_int_vec_step_reduce_84(re->data[j],
          re->data[j + step_vec],
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
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_e3(Eurydice_arr_13 *re)
{
  size_t zeta_i = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)2U;
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)4U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)7U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_error_reduce_84(
  Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *error
)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t j = i;
    __m256i
    coefficient_normal_form =
      libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(myself->data[j],
        1441);
    __m256i sum = libcrux_ml_kem_vector_avx2_add_f5(coefficient_normal_form, &error->data[j]);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(sum);
    myself->data[j] = red;
  }
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_error_reduce_d6_84(
  Eurydice_arr_13 *self,
  const Eurydice_arr_13 *error
)
{
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
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_matrix_compute_vector_u_e3(
  const Eurydice_arr_601 *a_as_ntt,
  const Eurydice_arr_600 *r_as_ntt,
  const Eurydice_arr_600 *error_1
)
{
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_e3(&lvalue, i););
  Eurydice_arr_600 result = arr_struct;
  KRML_MAYBE_FOR3(i0,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i1 = i0;
    const Eurydice_arr_600 *row = &a_as_ntt->data[i1];
    KRML_MAYBE_FOR3(i,
      (size_t)0U,
      (size_t)3U,
      (size_t)1U,
      size_t j = i;
      const Eurydice_arr_13 *a_element = &row->data[j];
      Eurydice_arr_13
      product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(a_element, &r_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_e3(&result.data[i1], &product););
    libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_e3(&result.data[i1]);
    libcrux_ml_kem_polynomial_add_error_reduce_d6_84(&result.data[i1], &error_1->data[i1]););
  return result;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.compress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_ef(__m256i vector)
{
  __m256i
  field_modulus_halved =
    libcrux_intrinsics_avx2_mm256_set1_epi32(((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS -
        1)
      / 2);
  __m256i compression_factor = libcrux_intrinsics_avx2_mm256_set1_epi32(10321340);
  __m256i
  coefficient_bits_mask =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)10) - 1);
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  compressed_low = libcrux_intrinsics_avx2_mm256_slli_epi32(10, coefficients_low0, __m256i);
  __m256i
  compressed_low0 = libcrux_intrinsics_avx2_mm256_add_epi32(compressed_low, field_modulus_halved);
  __m256i
  compressed_low1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_low0,
      compression_factor);
  __m256i
  compressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_low1, __m256i);
  __m256i
  compressed_low3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_low2,
      coefficient_bits_mask);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  compressed_high = libcrux_intrinsics_avx2_mm256_slli_epi32(10, coefficients_high0, __m256i);
  __m256i
  compressed_high0 =
    libcrux_intrinsics_avx2_mm256_add_epi32(compressed_high,
      field_modulus_halved);
  __m256i
  compressed_high1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_high0,
      compression_factor);
  __m256i
  compressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_high1, __m256i);
  __m256i
  compressed_high3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_high2,
      coefficient_bits_mask);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(compressed_low3, compressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_ef(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_ef(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress_f5
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f5_ef(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_ef(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_10
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- OUT_LEN= 320
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b0
libcrux_ml_kem_serialize_compress_then_serialize_10_03(const Eurydice_arr_13 *re)
{
  Eurydice_arr_b0 serialized = { { 0U } };
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i
    coefficient =
      libcrux_ml_kem_vector_avx2_compress_f5_ef(libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re->data[i0]));
    Eurydice_arr_fc bytes = libcrux_ml_kem_vector_avx2_serialize_10_f5(coefficient);
    Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d413(&serialized,
        (core_ops_range_Range_87{ (size_t)20U * i0, (size_t)20U * i0 + (size_t)20U })),
      Eurydice_array_to_slice_shared_8f(&bytes),
      uint8_t);
  }
  return serialized;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.compress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_c4(__m256i vector)
{
  __m256i
  field_modulus_halved =
    libcrux_intrinsics_avx2_mm256_set1_epi32(((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS -
        1)
      / 2);
  __m256i compression_factor = libcrux_intrinsics_avx2_mm256_set1_epi32(10321340);
  __m256i
  coefficient_bits_mask =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)11) - 1);
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  compressed_low = libcrux_intrinsics_avx2_mm256_slli_epi32(11, coefficients_low0, __m256i);
  __m256i
  compressed_low0 = libcrux_intrinsics_avx2_mm256_add_epi32(compressed_low, field_modulus_halved);
  __m256i
  compressed_low1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_low0,
      compression_factor);
  __m256i
  compressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_low1, __m256i);
  __m256i
  compressed_low3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_low2,
      coefficient_bits_mask);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  compressed_high = libcrux_intrinsics_avx2_mm256_slli_epi32(11, coefficients_high0, __m256i);
  __m256i
  compressed_high0 =
    libcrux_intrinsics_avx2_mm256_add_epi32(compressed_high,
      field_modulus_halved);
  __m256i
  compressed_high1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_high0,
      compression_factor);
  __m256i
  compressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_high1, __m256i);
  __m256i
  compressed_high3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_high2,
      coefficient_bits_mask);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(compressed_low3, compressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress
with const generics
- COEFFICIENT_BITS= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_c4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_c4(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress_f5
with const generics
- COEFFICIENT_BITS= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f5_c4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_c4(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_ring_element_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- COMPRESSION_FACTOR= 10
- OUT_LEN= 320
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b0
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_81(const Eurydice_arr_13 *re)
{
  return libcrux_ml_kem_serialize_compress_then_serialize_10_03(re);
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_compress_then_serialize_u_d4(
  Eurydice_arr_600 input,
  Eurydice_mut_borrow_slice_u8 out
)
{
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13 re = input.data[i0];
    Eurydice_mut_borrow_slice_u8
    uu____0 =
      Eurydice_slice_subslice_mut_c8(out,
        (
          core_ops_range_Range_87{
            i0 * ((size_t)960U / (size_t)3U),
            (i0 + (size_t)1U) * ((size_t)960U / (size_t)3U)
          }
        ));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b0
    lvalue = libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_81(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_56(&lvalue), uint8_t););
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
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
static KRML_MUSTINLINE tuple_24
libcrux_ml_kem_ind_cpa_encrypt_c1_781(
  Eurydice_borrow_slice_u8 randomness,
  const Eurydice_arr_601 *matrix,
  Eurydice_mut_borrow_slice_u8 ciphertext
)
{
  Eurydice_arr_fa prf_input = libcrux_ml_kem_utils_into_padded_array_29(randomness);
  Eurydice_arr_600 arr_struct0;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct0.data[i] = libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_781(&lvalue, i););
  Eurydice_arr_600 r_as_ntt = arr_struct0;
  uint8_t
  domain_separator0 =
    libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d61(&r_as_ntt,
      &prf_input,
      0U);
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_781(&lvalue, i););
  Eurydice_arr_600 error_1 = arr_struct;
  uint8_t
  domain_separator =
    libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_d61(&prf_input,
      domain_separator0,
      &error_1);
  prf_input.data[32U] = domain_separator;
  Eurydice_arr_89
  prf_output =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_3b0(Eurydice_array_to_slice_shared_b5(&prf_input));
  Eurydice_arr_13
  error_2 =
    libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_output));
  Eurydice_arr_600 u = libcrux_ml_kem_matrix_compute_vector_u_e3(matrix, &r_as_ntt, &error_1);
  libcrux_ml_kem_ind_cpa_compress_then_serialize_u_d4(u, ciphertext);
  return (tuple_24{ r_as_ntt, error_2 });
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_message
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_message_84(
  const Eurydice_arr_ec *serialized
)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t i0 = i;
    __m256i
    coefficient_compressed =
      libcrux_ml_kem_vector_avx2_deserialize_1_f5(Eurydice_array_to_subslice_shared_d41(serialized,
          (core_ops_range_Range_87{ (size_t)2U * i0, (size_t)2U * i0 + (size_t)2U })));
    re.data[i0] = libcrux_ml_kem_vector_avx2_decompress_1_f5(coefficient_compressed););
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_message_error_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_add_message_error_reduce_84(
  const Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *message,
  Eurydice_arr_13 result
)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i
    coefficient_normal_form =
      libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(result.data[i0],
        1441);
    __m256i sum1 = libcrux_ml_kem_vector_avx2_add_f5(myself->data[i0], &message->data[i0]);
    __m256i sum2 = libcrux_ml_kem_vector_avx2_add_f5(coefficient_normal_form, &sum1);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(sum2);
    result.data[i0] = red;
  }
  return result;
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_message_error_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_add_message_error_reduce_d6_84(
  const Eurydice_arr_13 *self,
  const Eurydice_arr_13 *message,
  Eurydice_arr_13 result
)
{
  return libcrux_ml_kem_polynomial_add_message_error_reduce_84(self, message, result);
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
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_matrix_compute_ring_element_v_e3(
  const Eurydice_arr_600 *t_as_ntt,
  const Eurydice_arr_600 *r_as_ntt,
  const Eurydice_arr_13 *error_2,
  const Eurydice_arr_13 *message
)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    product =
      libcrux_ml_kem_polynomial_ntt_multiply_d6_84(&t_as_ntt->data[i0],
        &r_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_e3(&result, &product););
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_e3(&result);
  return libcrux_ml_kem_polynomial_add_message_error_reduce_d6_84(error_2, message, result);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.compress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_d1(__m256i vector)
{
  __m256i
  field_modulus_halved =
    libcrux_intrinsics_avx2_mm256_set1_epi32(((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS -
        1)
      / 2);
  __m256i compression_factor = libcrux_intrinsics_avx2_mm256_set1_epi32(10321340);
  __m256i
  coefficient_bits_mask =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)4) - 1);
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  compressed_low = libcrux_intrinsics_avx2_mm256_slli_epi32(4, coefficients_low0, __m256i);
  __m256i
  compressed_low0 = libcrux_intrinsics_avx2_mm256_add_epi32(compressed_low, field_modulus_halved);
  __m256i
  compressed_low1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_low0,
      compression_factor);
  __m256i
  compressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_low1, __m256i);
  __m256i
  compressed_low3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_low2,
      coefficient_bits_mask);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  compressed_high = libcrux_intrinsics_avx2_mm256_slli_epi32(4, coefficients_high0, __m256i);
  __m256i
  compressed_high0 =
    libcrux_intrinsics_avx2_mm256_add_epi32(compressed_high,
      field_modulus_halved);
  __m256i
  compressed_high1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_high0,
      compression_factor);
  __m256i
  compressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_high1, __m256i);
  __m256i
  compressed_high3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_high2,
      coefficient_bits_mask);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(compressed_low3, compressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_d1(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_d1(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress_f5
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f5_d1(__m256i vector)
{
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
  Eurydice_arr_13 re,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i
    coefficient =
      libcrux_ml_kem_vector_avx2_compress_f5_d1(libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re.data[i0]));
    Eurydice_array_u8x8 bytes = libcrux_ml_kem_vector_avx2_serialize_4_f5(coefficient);
    Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(serialized,
        (core_ops_range_Range_87{ (size_t)8U * i0, (size_t)8U * i0 + (size_t)8U })),
      Eurydice_array_to_slice_shared_6e(&bytes),
      uint8_t);
  }
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.compress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_f4(__m256i vector)
{
  __m256i
  field_modulus_halved =
    libcrux_intrinsics_avx2_mm256_set1_epi32(((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS -
        1)
      / 2);
  __m256i compression_factor = libcrux_intrinsics_avx2_mm256_set1_epi32(10321340);
  __m256i
  coefficient_bits_mask =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)5) - 1);
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  compressed_low = libcrux_intrinsics_avx2_mm256_slli_epi32(5, coefficients_low0, __m256i);
  __m256i
  compressed_low0 = libcrux_intrinsics_avx2_mm256_add_epi32(compressed_low, field_modulus_halved);
  __m256i
  compressed_low1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_low0,
      compression_factor);
  __m256i
  compressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_low1, __m256i);
  __m256i
  compressed_low3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_low2,
      coefficient_bits_mask);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  compressed_high = libcrux_intrinsics_avx2_mm256_slli_epi32(5, coefficients_high0, __m256i);
  __m256i
  compressed_high0 =
    libcrux_intrinsics_avx2_mm256_add_epi32(compressed_high,
      field_modulus_halved);
  __m256i
  compressed_high1 =
    libcrux_ml_kem_vector_avx2_compress_mulhi_mm256_epi32(compressed_high0,
      compression_factor);
  __m256i
  compressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(3, compressed_high1, __m256i);
  __m256i
  compressed_high3 =
    libcrux_intrinsics_avx2_mm256_and_si256(compressed_high2,
      coefficient_bits_mask);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(compressed_low3, compressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress
with const generics
- COEFFICIENT_BITS= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_compress_ciphertext_coefficient_f4(vector);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress_f5
with const generics
- COEFFICIENT_BITS= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_f5_f4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_f4(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_5
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_5_84(
  Eurydice_arr_13 re,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i
    coefficients =
      libcrux_ml_kem_vector_avx2_compress_f5_f4(libcrux_ml_kem_vector_avx2_to_unsigned_representative_f5(re.data[i0]));
    Eurydice_arr_6d bytes = libcrux_ml_kem_vector_avx2_serialize_5_f5(coefficients);
    Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(serialized,
        (core_ops_range_Range_87{ (size_t)10U * i0, (size_t)10U * i0 + (size_t)10U })),
      Eurydice_array_to_slice_shared_30(&bytes),
      uint8_t);
  }
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- COMPRESSION_FACTOR= 4
- OUT_LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_15(
  Eurydice_arr_13 re,
  Eurydice_mut_borrow_slice_u8 out
)
{
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_encrypt_c2_15(
  const Eurydice_arr_600 *t_as_ntt,
  const Eurydice_arr_600 *r_as_ntt,
  const Eurydice_arr_13 *error_2,
  const Eurydice_arr_ec *message,
  Eurydice_mut_borrow_slice_u8 ciphertext
)
{
  Eurydice_arr_13
  message_as_ring_element =
    libcrux_ml_kem_serialize_deserialize_then_decompress_message_84(message);
  Eurydice_arr_13
  v =
    libcrux_ml_kem_matrix_compute_ring_element_v_e3(t_as_ntt,
      r_as_ntt,
      error_2,
      &message_as_ring_element);
  libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_15(v, ciphertext);
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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
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
static KRML_MUSTINLINE Eurydice_arr_2b
libcrux_ml_kem_ind_cpa_encrypt_unpacked_281(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *message,
  Eurydice_borrow_slice_u8 randomness
)
{
  Eurydice_arr_2b ciphertext = { { 0U } };
  tuple_24
  uu____0 =
    libcrux_ml_kem_ind_cpa_encrypt_c1_781(randomness,
      &public_key->A,
      Eurydice_array_to_subslice_mut_d418(&ciphertext,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)960U })));
  Eurydice_arr_600 r_as_ntt = uu____0.fst;
  Eurydice_arr_13 error_2 = uu____0.snd;
  libcrux_ml_kem_ind_cpa_encrypt_c2_15(&public_key->t_as_ntt,
    &r_as_ntt,
    &error_2,
    message,
    Eurydice_array_to_subslice_from_mut_5f5(&ciphertext, (size_t)960U));
  return ciphertext;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
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
static KRML_MUSTINLINE tuple_f4
libcrux_ml_kem_ind_cca_unpacked_encapsulate_a81(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *randomness
)
{
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_b6(Eurydice_array_to_slice_shared_01(randomness),
      Eurydice_array_to_slice_shared_01(&public_key->public_key_hash));
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____0.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____0.snd;
  Eurydice_arr_2b
  ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_unpacked_281(&public_key->ind_cpa_public_key,
      randomness,
      pseudorandomness);
  Eurydice_arr_ec shared_secret_array = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&shared_secret_array),
    shared_secret,
    uint8_t);
  return (tuple_f4{ libcrux_ml_kem_types_from_19_52(ciphertext), shared_secret_array });
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_26(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a81(public_key, randomness);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_26(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_26(public_key,
      randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cpa::deserialize_then_decompress_u::closure<Vector, K, CIPHERTEXT_SIZE, U_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u.call_mut_35
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_15(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.decompress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_ef(__m256i vector)
{
  __m256i
  field_modulus =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i
  two_pow_coefficient_bits =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)10));
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  decompressed_low = libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_low0, field_modulus);
  __m256i
  decompressed_low0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_low, __m256i);
  __m256i
  decompressed_low1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_low0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(10, decompressed_low1, __m256i);
  __m256i
  decompressed_low3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_low2, __m256i);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  decompressed_high =
    libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_high0,
      field_modulus);
  __m256i
  decompressed_high0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_high, __m256i);
  __m256i
  decompressed_high1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_high0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(10, decompressed_high1, __m256i);
  __m256i
  decompressed_high3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_high2, __m256i);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(decompressed_low3, decompressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.decompress_ciphertext_coefficient_f5
with const generics
- COEFFICIENT_BITS= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_ef(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_ef(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_10
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_10_84(Eurydice_borrow_slice_u8 serialized)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)20U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)20U, i0 * (size_t)20U + (size_t)20U }));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_10_f5(bytes);
    re.data[i0] = libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_ef(coefficient);
  }
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.decompress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_c4(__m256i vector)
{
  __m256i
  field_modulus =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i
  two_pow_coefficient_bits =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)11));
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  decompressed_low = libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_low0, field_modulus);
  __m256i
  decompressed_low0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_low, __m256i);
  __m256i
  decompressed_low1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_low0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(11, decompressed_low1, __m256i);
  __m256i
  decompressed_low3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_low2, __m256i);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  decompressed_high =
    libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_high0,
      field_modulus);
  __m256i
  decompressed_high0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_high, __m256i);
  __m256i
  decompressed_high1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_high0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(11, decompressed_high1, __m256i);
  __m256i
  decompressed_high3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_high2, __m256i);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(decompressed_low3, decompressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.decompress_ciphertext_coefficient_f5
with const generics
- COEFFICIENT_BITS= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_c4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_c4(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_11_84(Eurydice_borrow_slice_u8 serialized)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)22U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)22U, i0 * (size_t)22U + (size_t)22U }));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_11_f5(bytes);
    re.data[i0] = libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_c4(coefficient);
  }
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_d0(
  Eurydice_borrow_slice_u8 serialized
)
{
  return libcrux_ml_kem_serialize_deserialize_then_decompress_10_84(serialized);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_vector_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- VECTOR_U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_vector_u_d0(Eurydice_arr_13 *re)
{
  size_t zeta_i = (size_t)0U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)7U, (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U, (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U, (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)4U, (size_t)4U * (size_t)3328U);
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
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_15(const Eurydice_arr_2b *ciphertext)
{
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_15(&lvalue,
        i););
  Eurydice_arr_600 u_as_ntt = arr_struct;
  for
  (size_t
    i = (size_t)0U;
    i <
      (size_t)1088U /
        (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U);
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    u_bytes =
      Eurydice_array_to_subslice_shared_d49(ciphertext,
        (
          core_ops_range_Range_87{
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U),
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U)
            + LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U
          }
        ));
    u_as_ntt.data[i0] =
      libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_d0(u_bytes);
    libcrux_ml_kem_ntt_ntt_vector_u_d0(&u_as_ntt.data[i0]);
  }
  return u_as_ntt;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.decompress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_d1(__m256i vector)
{
  __m256i
  field_modulus =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i
  two_pow_coefficient_bits =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)4));
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  decompressed_low = libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_low0, field_modulus);
  __m256i
  decompressed_low0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_low, __m256i);
  __m256i
  decompressed_low1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_low0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(4, decompressed_low1, __m256i);
  __m256i
  decompressed_low3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_low2, __m256i);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  decompressed_high =
    libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_high0,
      field_modulus);
  __m256i
  decompressed_high0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_high, __m256i);
  __m256i
  decompressed_high1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_high0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(4, decompressed_high1, __m256i);
  __m256i
  decompressed_high3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_high2, __m256i);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(decompressed_low3, decompressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.decompress_ciphertext_coefficient_f5
with const generics
- COEFFICIENT_BITS= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_d1(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_d1(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_4
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_4_84(Eurydice_borrow_slice_u8 serialized)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)8U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)8U, i0 * (size_t)8U + (size_t)8U }));
    __m256i coefficient = libcrux_ml_kem_vector_avx2_deserialize_4_f5(bytes);
    re.data[i0] = libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_d1(coefficient);
  }
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.compress.decompress_ciphertext_coefficient
with const generics
- COEFFICIENT_BITS= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_f4(__m256i vector)
{
  __m256i
  field_modulus =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  __m256i
  two_pow_coefficient_bits =
    libcrux_intrinsics_avx2_mm256_set1_epi32((int32_t)((uint32_t)1 << (uint32_t)5));
  __m128i coefficients_low = libcrux_intrinsics_avx2_mm256_castsi256_si128(vector);
  __m256i coefficients_low0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_low);
  __m256i
  decompressed_low = libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_low0, field_modulus);
  __m256i
  decompressed_low0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_low, __m256i);
  __m256i
  decompressed_low1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_low0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_low2 = libcrux_intrinsics_avx2_mm256_srli_epi32(5, decompressed_low1, __m256i);
  __m256i
  decompressed_low3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_low2, __m256i);
  __m128i
  coefficients_high = libcrux_intrinsics_avx2_mm256_extracti128_si256(1, vector, __m128i);
  __m256i coefficients_high0 = libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(coefficients_high);
  __m256i
  decompressed_high =
    libcrux_intrinsics_avx2_mm256_mullo_epi32(coefficients_high0,
      field_modulus);
  __m256i
  decompressed_high0 = libcrux_intrinsics_avx2_mm256_slli_epi32(1, decompressed_high, __m256i);
  __m256i
  decompressed_high1 =
    libcrux_intrinsics_avx2_mm256_add_epi32(decompressed_high0,
      two_pow_coefficient_bits);
  __m256i
  decompressed_high2 = libcrux_intrinsics_avx2_mm256_srli_epi32(5, decompressed_high1, __m256i);
  __m256i
  decompressed_high3 = libcrux_intrinsics_avx2_mm256_srli_epi32(1, decompressed_high2, __m256i);
  __m256i
  compressed = libcrux_intrinsics_avx2_mm256_packs_epi32(decompressed_low3, decompressed_high3);
  return libcrux_intrinsics_avx2_mm256_permute4x64_epi64(216, compressed, __m256i);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::avx2::SIMD256Vector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.avx2.decompress_ciphertext_coefficient_f5
with const generics
- COEFFICIENT_BITS= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE __m256i
libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_f4(__m256i vector)
{
  return libcrux_ml_kem_vector_avx2_compress_decompress_ciphertext_coefficient_f4(vector);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_5
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_5_84(Eurydice_borrow_slice_u8 serialized)
{
  Eurydice_arr_13 re = libcrux_ml_kem_polynomial_ZERO_d6_84();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)10U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)10U, i0 * (size_t)10U + (size_t)10U }));
    re.data[i0] = libcrux_ml_kem_vector_avx2_deserialize_5_f5(bytes);
    re.data[i0] = libcrux_ml_kem_vector_avx2_decompress_ciphertext_coefficient_f5_f4(re.data[i0]);
  }
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_79(
  Eurydice_borrow_slice_u8 serialized
)
{
  return libcrux_ml_kem_serialize_deserialize_then_decompress_4_84(serialized);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.subtract_reduce
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_subtract_reduce_84(const Eurydice_arr_13 *myself, Eurydice_arr_13 b)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i
    coefficient_normal_form =
      libcrux_ml_kem_vector_avx2_montgomery_multiply_by_constant_f5(b.data[i0],
        1441);
    __m256i diff = libcrux_ml_kem_vector_avx2_sub_f5(myself->data[i0], &coefficient_normal_form);
    __m256i red = libcrux_ml_kem_vector_avx2_barrett_reduce_f5(diff);
    b.data[i0] = red;
  }
  return b;
}

/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.subtract_reduce_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_polynomial_subtract_reduce_d6_84(const Eurydice_arr_13 *self, Eurydice_arr_13 b)
{
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
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_matrix_compute_message_e3(
  const Eurydice_arr_13 *v,
  const Eurydice_arr_600 *secret_as_ntt,
  const Eurydice_arr_600 *u_as_ntt
)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    product =
      libcrux_ml_kem_polynomial_ntt_multiply_d6_84(&secret_as_ntt->data[i0],
        &u_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_e3(&result, &product););
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_e3(&result);
  return libcrux_ml_kem_polynomial_subtract_reduce_d6_84(v, result);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_message
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics

*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_serialize_compress_then_serialize_message_84(Eurydice_arr_13 re)
{
  Eurydice_arr_ec serialized = { { 0U } };
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t i0 = i;
    __m256i coefficient = libcrux_ml_kem_serialize_to_unsigned_field_modulus_84(re.data[i0]);
    __m256i coefficient_compressed = libcrux_ml_kem_vector_avx2_compress_1_f5(coefficient);
    Eurydice_array_u8x2 bytes = libcrux_ml_kem_vector_avx2_serialize_1_f5(coefficient_compressed);
    Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d44(&serialized,
        (core_ops_range_Range_87{ (size_t)2U * i0, (size_t)2U * i0 + (size_t)2U })),
      Eurydice_array_to_slice_shared_82(&bytes),
      uint8_t););
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_unpacked_3e(
  const Eurydice_arr_600 *secret_key,
  const Eurydice_arr_2b *ciphertext
)
{
  Eurydice_arr_600
  u_as_ntt = libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_15(ciphertext);
  Eurydice_arr_13
  v =
    libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_79(Eurydice_array_to_subslice_from_shared_5f3(ciphertext,
        (size_t)960U));
  Eurydice_arr_13 message = libcrux_ml_kem_matrix_compute_message_e3(&v, secret_key, &u_as_ntt);
  return libcrux_ml_kem_serialize_compress_then_serialize_message_84(message);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF
with const generics
- LEN= 32
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_PRF_ce(Eurydice_borrow_slice_u8 input)
{
  Eurydice_arr_ec digest = { { 0U } };
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_01(&digest), input);
  return digest;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 3
- LEN= 32
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_PRF_41_3b(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRF_ce(input);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_unpacked_decapsulate_d91(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
)
{
  Eurydice_arr_ec
  decrypted =
    libcrux_ml_kem_ind_cpa_decrypt_unpacked_3e(&key_pair->private_key.ind_cpa_private_key,
      ciphertext);
  Eurydice_arr_c7
  to_hash0 =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_from_mut_5f1(&to_hash0,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____0,
    Eurydice_array_to_slice_shared_01(&key_pair->public_key.public_key_hash),
    uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_78(Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_af
  to_hash =
    libcrux_ml_kem_utils_into_padded_array_66(Eurydice_array_to_slice_shared_01(&key_pair->private_key.implicit_rejection_value));
  Eurydice_mut_borrow_slice_u8
  uu____2 =
    Eurydice_array_to_subslice_from_mut_5f4(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_52(ciphertext), uint8_t);
  Eurydice_arr_ec
  implicit_rejection_shared_secret =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_3b(Eurydice_array_to_slice_shared_81(&to_hash));
  Eurydice_arr_2b
  expected_ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_unpacked_281(&key_pair->public_key.ind_cpa_public_key,
      &decrypted,
      pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 = libcrux_ml_kem_types_as_ref_c1_52(ciphertext);
  uint8_t
  selector =
    libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(uu____3,
      Eurydice_array_to_slice_shared_06(&expected_ciphertext));
  return
    libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(shared_secret,
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret),
      selector);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_19(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_d91(key_pair, ciphertext);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_19(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_19(key_pair,
      ciphertext);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::serialize::deserialize_ring_elements_reduced_out::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out.call_mut_0b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_e3(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 This function deserializes ring elements and reduces the result by the field
 modulus.

 This function MUST NOT be used on secret inputs.
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_e3(
  Eurydice_borrow_slice_u8 public_key
)
{
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_e3(&lvalue,
        i););
  Eurydice_arr_600 deserialized_pk = arr_struct;
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_e3(public_key, &deserialized_pk);
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
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_public_key_79(const Eurydice_arr_5f *public_key)
{
  Eurydice_arr_600
  deserialized_pk =
    libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_e3(Eurydice_array_to_subslice_to_shared_213(public_key,
        libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)3U)));
  Eurydice_arr_5f
  public_key_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_79(&deserialized_pk,
      Eurydice_array_to_subslice_from_shared_5f4(public_key,
        libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)3U)));
  return Eurydice_array_eq((size_t)1184U, public_key, &public_key_serialized, uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_3b(
  const Eurydice_arr_5f *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_79(public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_3b(
  const Eurydice_arr_5f *public_key
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_3b(public_key);
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
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_private_key_only_a4(const Eurydice_arr_7d *private_key)
{
  Eurydice_arr_ec
  t =
    libcrux_ml_kem_hash_functions_avx2_H_41_78(Eurydice_array_to_subslice_shared_d410(private_key,
        (
          core_ops_range_Range_87{
            (size_t)384U * (size_t)3U,
            (size_t)768U * (size_t)3U + (size_t)32U
          }
        )));
  Eurydice_borrow_slice_u8
  expected =
    Eurydice_array_to_subslice_shared_d410(private_key,
      (
        core_ops_range_Range_87{
          (size_t)768U * (size_t)3U + (size_t)32U,
          (size_t)768U * (size_t)3U + (size_t)64U
        }
      ));
  return Eurydice_array_eq_slice_shared((size_t)32U, &t, &expected, uint8_t, bool);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_3b(
  const Eurydice_arr_7d *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_a4(private_key);
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
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_private_key_d50(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *_ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_a4(private_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_d3(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_d50(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_d3(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_d3(private_key,
      ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 3
- PRIVATE_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_utils_extraction_helper_Keypair768
libcrux_ml_kem_ind_cpa_generate_keypair_cc1(Eurydice_borrow_slice_u8 key_generation_seed)
{
  Eurydice_arr_600 private_key = libcrux_ml_kem_ind_cpa_unpacked_default_70_e3();
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
  public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_e3();
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab1(key_generation_seed,
    &private_key,
    &public_key);
  return libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_15(&public_key, &private_key);
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
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_a4(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value,
  Eurydice_arr_7d *serialized
)
{
  size_t pointer = (size_t)0U;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d420(serialized,
      (core_ops_range_Range_87{ pointer, pointer + private_key.meta })),
    private_key,
    uint8_t);
  pointer += private_key.meta;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d420(serialized,
      (core_ops_range_Range_87{ pointer, pointer + public_key.meta })),
    public_key,
    uint8_t);
  pointer += public_key.meta;
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_mut_d420(serialized,
      (core_ops_range_Range_87{ pointer, pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE }));
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec lvalue = libcrux_ml_kem_hash_functions_avx2_H_41_78(public_key);
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue), uint8_t);
  pointer += LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d420(serialized,
      (core_ops_range_Range_87{ pointer, pointer + implicit_rejection_value.meta })),
    implicit_rejection_value,
    uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7d
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_a4(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value
)
{
  Eurydice_arr_7d out = { { 0U } };
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_a4(private_key,
    public_key,
    implicit_rejection_value,
    &out);
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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
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
libcrux_ml_kem_ind_cca_generate_keypair_db1(const Eurydice_arr_c7 *randomness)
{
  Eurydice_borrow_slice_u8
  ind_cpa_keypair_randomness =
    Eurydice_array_to_subslice_shared_d47(randomness,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      ));
  Eurydice_borrow_slice_u8
  implicit_rejection_value =
    Eurydice_array_to_subslice_from_shared_5f1(randomness,
      LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_utils_extraction_helper_Keypair768
  uu____0 = libcrux_ml_kem_ind_cpa_generate_keypair_cc1(ind_cpa_keypair_randomness);
  Eurydice_arr_0e ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_5f public_key = uu____0.snd;
  Eurydice_arr_7d
  secret_key_serialized =
    libcrux_ml_kem_ind_cca_serialize_kem_secret_key_a4(Eurydice_array_to_slice_shared_f4(&ind_cpa_private_key),
      Eurydice_array_to_slice_shared_ff(&public_key),
      implicit_rejection_value);
  Eurydice_arr_7d private_key = libcrux_ml_kem_types_from_b2_79(secret_key_serialized);
  return
    libcrux_ml_kem_types_from_17_bc(private_key,
      libcrux_ml_kem_types_from_51_3d(public_key));
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_e9(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_db1(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_e9(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_e9(randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.entropy_preprocess_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_entropy_preprocess_39_b6(Eurydice_borrow_slice_u8 randomness)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), randomness, uint8_t);
  return out;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_911(
  Eurydice_borrow_slice_u8 public_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef *unpacked_public_key
)
{
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_e3(Eurydice_slice_subslice_to_shared_72(public_key,
      (size_t)1152U),
    &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8
  seed = Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)1152U);
  Eurydice_arr_601 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_281(uu____0, &lvalue, false);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_911(Eurydice_borrow_slice_u8 public_key)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
  unpacked_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_e3();
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_911(public_key, &unpacked_public_key);
  return unpacked_public_key;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
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
static KRML_MUSTINLINE Eurydice_arr_2b
libcrux_ml_kem_ind_cpa_encrypt_281(
  Eurydice_borrow_slice_u8 public_key,
  const Eurydice_arr_ec *message,
  Eurydice_borrow_slice_u8 randomness
)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef
  unpacked_public_key = libcrux_ml_kem_ind_cpa_build_unpacked_public_key_911(public_key);
  return libcrux_ml_kem_ind_cpa_encrypt_unpacked_281(&unpacked_public_key, message, randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.kdf_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_kdf_39_a4(
  Eurydice_borrow_slice_u8 shared_secret,
  const Eurydice_arr_2b *_
)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), shared_secret, uint8_t);
  return out;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
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
static KRML_MUSTINLINE tuple_f4
libcrux_ml_kem_ind_cca_encapsulate_a11(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
)
{
  Eurydice_arr_ec
  randomness0 =
    libcrux_ml_kem_variant_entropy_preprocess_39_b6(Eurydice_array_to_slice_shared_01(randomness));
  Eurydice_arr_c7
  to_hash =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&randomness0));
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_from_mut_5f1(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE);
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec
  lvalue =
    libcrux_ml_kem_hash_functions_avx2_H_41_78(Eurydice_array_to_slice_shared_ff(libcrux_ml_kem_types_as_slice_e6_3d(public_key)));
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue), uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_78(Eurydice_array_to_slice_shared_17(&to_hash));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_2b
  ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_281(Eurydice_array_to_slice_shared_ff(libcrux_ml_kem_types_as_slice_e6_3d(public_key)),
      &randomness0,
      pseudorandomness);
  Eurydice_arr_2b uu____2 = libcrux_ml_kem_types_from_19_52(ciphertext);
  return (tuple_f4{ uu____2, libcrux_ml_kem_variant_kdf_39_a4(shared_secret, &ciphertext) });
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_26(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_a11(public_key, randomness);
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_26(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_26(public_key, randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cpa::decrypt::closure<Vector, K, CIPHERTEXT_SIZE, VECTOR_U_ENCODED_SIZE, U_COMPRESSION_FACTOR, V_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
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
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_3e(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_3e(
  Eurydice_borrow_slice_u8 secret_key,
  const Eurydice_arr_2b *ciphertext
)
{
  Eurydice_arr_600 arr_struct;
  KRML_MAYBE_FOR3(i,
    (size_t)0U,
    (size_t)3U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_3e(&lvalue, i););
  Eurydice_arr_600 secret_key_unpacked = arr_struct;
  libcrux_ml_kem_ind_cpa_deserialize_vector_e3(secret_key, &secret_key_unpacked);
  return libcrux_ml_kem_ind_cpa_decrypt_unpacked_3e(&secret_key_unpacked, ciphertext);
}

/**
 This code verifies on some machines, runs out of memory on others
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_decapsulate_661(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  Eurydice_borrow_slice_u8_x4
  uu____0 =
    libcrux_ml_kem_types_unpack_private_key_64(Eurydice_array_to_slice_shared_51(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  Eurydice_arr_ec decrypted = libcrux_ml_kem_ind_cpa_decrypt_3e(ind_cpa_secret_key, ciphertext);
  Eurydice_arr_c7
  to_hash0 =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f1(&to_hash0,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE),
    ind_cpa_public_key_hash,
    uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_78(Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret0 = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_af to_hash = libcrux_ml_kem_utils_into_padded_array_66(implicit_rejection_value);
  Eurydice_mut_borrow_slice_u8
  uu____2 =
    Eurydice_array_to_subslice_from_mut_5f4(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_52(ciphertext), uint8_t);
  Eurydice_arr_ec
  implicit_rejection_shared_secret =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_3b(Eurydice_array_to_slice_shared_81(&to_hash));
  Eurydice_arr_2b
  expected_ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_281(ind_cpa_public_key,
      &decrypted,
      pseudorandomness);
  Eurydice_borrow_slice_u8
  uu____3 = Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret);
  Eurydice_arr_ec
  implicit_rejection_shared_secret0 =
    libcrux_ml_kem_variant_kdf_39_a4(uu____3,
      libcrux_ml_kem_types_as_slice_a9_52(ciphertext));
  Eurydice_arr_ec
  shared_secret =
    libcrux_ml_kem_variant_kdf_39_a4(shared_secret0,
      libcrux_ml_kem_types_as_slice_a9_52(ciphertext));
  Eurydice_borrow_slice_u8 uu____4 = libcrux_ml_kem_types_as_ref_c1_52(ciphertext);
  return
    libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(uu____4,
      Eurydice_array_to_slice_shared_06(&expected_ciphertext),
      Eurydice_array_to_slice_shared_01(&shared_secret),
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret0));
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_19(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_661(private_key, ciphertext);
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_19(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_19(private_key, ciphertext);
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_f6
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_3b_s { Eurydice_arr_13 data[4U]; } Eurydice_arr_3b;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_3b
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_cd0_s { Eurydice_arr_3b data[4U]; } Eurydice_arr_cd0;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4_s
{
  Eurydice_arr_3b t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_cd0 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4;

/**
 See [deserialize_ring_elements_reduced_out].
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_5b(
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_arr_3b *deserialized_pk
)
{
  for
  (size_t
    i = (size_t)0U;
    i < public_key.meta / LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT;
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    ring_element =
      Eurydice_slice_subslice_shared_c8(public_key,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT +
              LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
          }
        ));
    Eurydice_arr_13
    uu____0 = libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_84(ring_element);
    deserialized_pk->data[i0] = uu____0;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_23(const Eurydice_arr_56 *input)
{
  Eurydice_arr_c40 state = libcrux_sha3_avx2_x4_incremental_init();
  libcrux_sha3_avx2_x4_incremental_shake128_absorb_final(&state,
    Eurydice_array_to_slice_shared_e9(input->data),
    Eurydice_array_to_slice_shared_e9(&input->data[1U]),
    Eurydice_array_to_slice_shared_e9(&input->data[2U]),
    Eurydice_array_to_slice_shared_e9(&input->data[3U]));
  return state;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final_41
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_23(
  const Eurydice_arr_56 *input
)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_23(input);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7c0
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_23(Eurydice_arr_c40 *st)
{
  Eurydice_arr_7c0 out = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_79 out0 = { { 0U } };
  Eurydice_arr_79 out1 = { { 0U } };
  Eurydice_arr_79 out2 = { { 0U } };
  Eurydice_arr_79 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_three_blocks(st,
    Eurydice_array_to_slice_mut_48(&out0),
    Eurydice_array_to_slice_mut_48(&out1),
    Eurydice_array_to_slice_mut_48(&out2),
    Eurydice_array_to_slice_mut_48(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  out.data[3U] = out3;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks_41
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_7c0
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_23(
  Eurydice_arr_c40 *self
)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_23(self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT representation
 of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
 We say "partially" because this implementation only accepts a finite set of
 bytes as input and returns an error if the set is not enough; Algorithm 6 of
 the FIPS 203 standard on the other hand samples from an infinite stream of bytes
 until the ring element is filled. Algorithm 6 is reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_uniform_distribution_next
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- N= 504
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_74(
  const Eurydice_arr_7c0 *randomness,
  Eurydice_arr_cc *sampled_coefficients,
  Eurydice_arr_240 *out
)
{
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)504U / (size_t)24U; i++)
    {
      size_t r = i;
      if (sampled_coefficients->data[i1] < LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        size_t
        sampled =
          libcrux_ml_kem_vector_avx2_rej_sample_f5(Eurydice_array_to_subslice_shared_d46(&randomness->data[i1],
              (core_ops_range_Range_87{ r * (size_t)24U, r * (size_t)24U + (size_t)24U })),
            Eurydice_array_to_subslice_mut_e7(&out->data[i1],
              (
                core_ops_range_Range_87{
                  sampled_coefficients->data[i1],
                  sampled_coefficients->data[i1] + (size_t)16U
                }
              )));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    });
  bool done = true;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >= LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
    {
      sampled_coefficients->data[i0] = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    }
    else
    {
      done = false;
    });
  return done;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_9c
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_23(Eurydice_arr_c40 *st)
{
  Eurydice_arr_9c out = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_c5 out0 = { { 0U } };
  Eurydice_arr_c5 out1 = { { 0U } };
  Eurydice_arr_c5 out2 = { { 0U } };
  Eurydice_arr_c5 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_next_block(st,
    Eurydice_array_to_slice_mut_2c(&out0),
    Eurydice_array_to_slice_mut_2c(&out1),
    Eurydice_array_to_slice_mut_2c(&out2),
    Eurydice_array_to_slice_mut_2c(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  out.data[3U] = out3;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block_41
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_9c
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_23(Eurydice_arr_c40 *self)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_23(self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT representation
 of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
 We say "partially" because this implementation only accepts a finite set of
 bytes as input and returns an error if the set is not enough; Algorithm 6 of
 the FIPS 203 standard on the other hand samples from an infinite stream of bytes
 until the ring element is filled. Algorithm 6 is reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_uniform_distribution_next
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- N= 168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_740(
  const Eurydice_arr_9c *randomness,
  Eurydice_arr_cc *sampled_coefficients,
  Eurydice_arr_240 *out
)
{
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)24U; i++)
    {
      size_t r = i;
      if (sampled_coefficients->data[i1] < LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        size_t
        sampled =
          libcrux_ml_kem_vector_avx2_rej_sample_f5(Eurydice_array_to_subslice_shared_d40(&randomness->data[i1],
              (core_ops_range_Range_87{ r * (size_t)24U, r * (size_t)24U + (size_t)24U })),
            Eurydice_array_to_subslice_mut_e7(&out->data[i1],
              (
                core_ops_range_Range_87{
                  sampled_coefficients->data[i1],
                  sampled_coefficients->data[i1] + (size_t)16U
                }
              )));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    });
  bool done = true;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >= LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
    {
      sampled_coefficients->data[i0] = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    }
    else
    {
      done = false;
    });
  return done;
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector, Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_280(void **_, Eurydice_arr_5b tupled_args)
{
  Eurydice_arr_5b s = tupled_args;
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_subslice_shared_e70(&s,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)256U })));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b
libcrux_ml_kem_sampling_sample_from_xof_280(const Eurydice_arr_56 *seeds)
{
  Eurydice_arr_cc sampled_coefficients = { { 0U } };
  Eurydice_arr_240 out = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_c40
  xof_state = libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_23(seeds);
  Eurydice_arr_7c0
  randomness0 =
    libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_23(&xof_state);
  bool
  done =
    libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_74(&randomness0,
      &sampled_coefficients,
      &out);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_9c
      randomness = libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_23(&xof_state);
      done =
        libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_740(&randomness,
          &sampled_coefficients,
          &out);
    }
  }
  Eurydice_arr_3b arr_mapped_str;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
      libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_280(&lvalue,
        out.data[i]););
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_sample_matrix_A_280(
  Eurydice_arr_cd0 *A_transpose,
  const Eurydice_arr_31 *seed,
  bool transpose
)
{
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i1 = i0;
    Eurydice_arr_56 seeds;
    Eurydice_arr_31 repeat_expression[4U];
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      repeat_expression[i] =
        core_array__core__clone__Clone_for__T__N___clone((size_t)34U,
          seed,
          uint8_t,
          Eurydice_arr_31););
    memcpy(seeds.data, repeat_expression, (size_t)4U * sizeof (Eurydice_arr_31));
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;);
    Eurydice_arr_3b sampled = libcrux_ml_kem_sampling_sample_from_xof_280(&seeds);
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 sample = sampled.data[j];
      if (transpose)
      {
        A_transpose->data[j].data[i1] = sample;
      }
      else
      {
        A_transpose->data[i1].data[j] = sample;
      }););
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.H_41
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_H_41_23(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_H(input);
}

/**
 Generate an unpacked key from a serialized key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.unpack_public_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a0(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  Eurydice_borrow_slice_u8
  uu____0 = Eurydice_array_to_subslice_to_shared_214(public_key, (size_t)1536U);
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_5b(uu____0,
    &unpacked_public_key->ind_cpa_public_key.t_as_ntt);
  unpacked_public_key->ind_cpa_public_key.seed_for_A =
    libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_array_to_subslice_from_shared_5f5(public_key,
        (size_t)1536U));
  Eurydice_arr_cd0 *uu____2 = &unpacked_public_key->ind_cpa_public_key.A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31
  lvalue =
    libcrux_ml_kem_utils_into_padded_array_de(Eurydice_array_to_subslice_from_shared_5f5(public_key,
        (size_t)1536U));
  libcrux_ml_kem_matrix_sample_matrix_A_280(uu____2, &lvalue, false);
  Eurydice_arr_ec
  uu____3 =
    libcrux_ml_kem_hash_functions_avx2_H_41_23(Eurydice_array_to_slice_shared_b50(libcrux_ml_kem_types_as_slice_e6_d9(public_key)));
  unpacked_public_key->public_key_hash = uu____3;
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_43(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a0(public_key, unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_43(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_43(public_key,
    unpacked_public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_d4_s
{
  Eurydice_arr_3b ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_d4;

typedef struct libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_d4 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 public_key;
}
libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked;

/**
 Call [`serialize_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_vector
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_serialize_vector_5b(
  const Eurydice_arr_3b *key,
  Eurydice_mut_borrow_slice_u8 out
)
{
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13 re = key->data[i0];
    Eurydice_mut_borrow_slice_u8
    uu____0 =
      Eurydice_slice_subslice_mut_c8(out,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            (i0 + (size_t)1U) * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
          }
        ));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b20 lvalue = libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_84(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_a9(&lvalue), uint8_t););
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_serialize_public_key_mut_74(
  const Eurydice_arr_3b *t_as_ntt,
  Eurydice_borrow_slice_u8 seed_for_a,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cpa_serialize_vector_5b(t_as_ntt,
    Eurydice_array_to_subslice_mut_d423(serialized,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)4U)
        }
      )));
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f8(serialized,
      libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)4U)),
    seed_for_a,
    uint8_t);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_74(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *self,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_74(&self->ind_cpa_public_key.t_as_ntt,
    Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A),
    serialized);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_74(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_74(&self->public_key, serialized);
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_ind_cpa_serialize_public_key_74(
  const Eurydice_arr_3b *t_as_ntt,
  Eurydice_borrow_slice_u8 seed_for_a
)
{
  Eurydice_arr_d1 public_key_serialized = { { 0U } };
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_74(t_as_ntt,
    seed_for_a,
    &public_key_serialized);
  return public_key_serialized;
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_ind_cca_unpacked_serialized_dd_74(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *self
)
{
  return
    libcrux_ml_kem_types_from_51_d9(libcrux_ml_kem_ind_cpa_serialize_public_key_74(&self->ind_cpa_public_key.t_as_ntt,
        Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A)));
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_74(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_dd_74(&self->public_key);
}

/**
 Serialize the secret key from the unpacked key pair generation.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_unpacked_secret_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PRIVATE_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_utils_extraction_helper_Keypair1024
libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_72(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_3b *private_key
)
{
  Eurydice_arr_d1
  public_key_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_74(&public_key->t_as_ntt,
      Eurydice_array_to_slice_shared_01(&public_key->seed_for_A));
  Eurydice_arr_df secret_key_serialized = { { 0U } };
  libcrux_ml_kem_ind_cpa_serialize_vector_5b(private_key,
    Eurydice_array_to_slice_mut_2f(&secret_key_serialized));
  return
    (
      libcrux_ml_kem_utils_extraction_helper_Keypair1024{
        secret_key_serialized,
        public_key_serialized
      }
    );
}

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_f8(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self,
  Eurydice_arr_a8 *serialized
)
{
  libcrux_ml_kem_utils_extraction_helper_Keypair1024
  uu____0 =
    libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_72(&self->public_key.ind_cpa_public_key,
      &self->private_key.ind_cpa_private_key);
  Eurydice_arr_df ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_d1 ind_cpa_public_key = uu____0.snd;
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_4c(Eurydice_array_to_slice_shared_2f0(&ind_cpa_private_key),
    Eurydice_array_to_slice_shared_b50(&ind_cpa_public_key),
    Eurydice_array_to_slice_shared_01(&self->private_key.implicit_rejection_value),
    serialized);
}

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_a8
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_f8(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self
)
{
  Eurydice_arr_a8 sk = libcrux_ml_kem_types_default_d3_0e();
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_f8(self, &sk);
  return sk;
}

/**
 Call [`deserialize_to_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_vector
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_deserialize_vector_5b(
  Eurydice_borrow_slice_u8 secret_key,
  Eurydice_arr_3b *secret_as_ntt
)
{
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_84(Eurydice_slice_subslice_shared_c8(secret_key,
          (
            core_ops_range_Range_87{
              i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
              (i0 + (size_t)1U) * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
            }
          )));
    secret_as_ntt->data[i0] = uu____0;);
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector, Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]]
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_7e0(void **_, Eurydice_arr_5b tupled_args)
{
  Eurydice_arr_5b s = tupled_args;
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_subslice_shared_e70(&s,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)256U })));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]]
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b
libcrux_ml_kem_sampling_sample_from_xof_7e0(const Eurydice_arr_56 *seeds)
{
  Eurydice_arr_cc sampled_coefficients = { { 0U } };
  Eurydice_arr_240 out = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_4a
  xof_state = libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_23(seeds);
  Eurydice_arr_7c0
  randomness0 =
    libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_23(&xof_state);
  bool
  done =
    libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_74(&randomness0,
      &sampled_coefficients,
      &out);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_9c
      randomness =
        libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_23(&xof_state);
      done =
        libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_740(&randomness,
          &sampled_coefficients,
          &out);
    }
  }
  Eurydice_arr_3b arr_mapped_str;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
      libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_7e0(&lvalue,
        out.data[i]););
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]]
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_sample_matrix_A_7e0(
  Eurydice_arr_cd0 *A_transpose,
  const Eurydice_arr_31 *seed,
  bool transpose
)
{
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i1 = i0;
    Eurydice_arr_56 seeds;
    Eurydice_arr_31 repeat_expression[4U];
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      repeat_expression[i] =
        core_array__core__clone__Clone_for__T__N___clone((size_t)34U,
          seed,
          uint8_t,
          Eurydice_arr_31););
    memcpy(seeds.data, repeat_expression, (size_t)4U * sizeof (Eurydice_arr_31));
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;);
    Eurydice_arr_3b sampled = libcrux_ml_kem_sampling_sample_from_xof_7e0(&seeds);
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 sample = sampled.data[j];
      if (transpose)
      {
        A_transpose->data[j].data[i1] = sample;
      }
      else
      {
        A_transpose->data[i1].data[j] = sample;
      }););
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]]
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_100(
  Eurydice_borrow_slice_u8 public_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_5b(Eurydice_slice_subslice_to_shared_72(public_key,
      (size_t)1536U),
    &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8
  seed = Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)1536U);
  Eurydice_arr_cd0 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_7e0(uu____0, &lvalue, false);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.keys_from_private_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_b2(
  const Eurydice_arr_a8 *private_key,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  Eurydice_borrow_slice_u8_x4
  uu____0 =
    libcrux_ml_kem_types_unpack_private_key_e3(Eurydice_array_to_slice_shared_680(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  libcrux_ml_kem_ind_cpa_deserialize_vector_5b(ind_cpa_secret_key,
    &key_pair->private_key.ind_cpa_private_key);
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_100(ind_cpa_public_key,
    &key_pair->public_key.ind_cpa_public_key);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->public_key.public_key_hash),
    ind_cpa_public_key_hash,
    uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->private_key.implicit_rejection_value),
    implicit_rejection_value,
    uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->public_key.ind_cpa_public_key.seed_for_A),
    Eurydice_slice_subslice_from_shared_6d(ind_cpa_public_key, (size_t)1536U),
    uint8_t);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_39(
  const Eurydice_arr_a8 *private_key,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_b2(private_key, key_pair);
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPrivateKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_70
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_3b
libcrux_ml_kem_ind_cpa_unpacked_default_70_5b(void)
{
  Eurydice_arr_3b lit;
  Eurydice_arr_13 repeat_expression[4U];
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
  memcpy(lit.data, repeat_expression, (size_t)4U * sizeof (Eurydice_arr_13));
  return lit;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_8b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4
libcrux_ml_kem_ind_cpa_unpacked_default_8b_5b(void)
{
  Eurydice_arr_3b uu____0;
  Eurydice_arr_13 repeat_expression0[4U];
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    repeat_expression0[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
  memcpy(uu____0.data, repeat_expression0, (size_t)4U * sizeof (Eurydice_arr_13));
  Eurydice_arr_ec uu____1 = { { 0U } };
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 lit0;
  lit0.t_as_ntt = uu____0;
  lit0.seed_for_A = uu____1;
  Eurydice_arr_3b repeat_expression1[4U];
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    Eurydice_arr_3b lit;
    Eurydice_arr_13 repeat_expression[4U];
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
    memcpy(lit.data, repeat_expression, (size_t)4U * sizeof (Eurydice_arr_13));
    repeat_expression1[i0] = lit;);
  memcpy(lit0.A.data, repeat_expression1, (size_t)4U * sizeof (Eurydice_arr_3b));
  return lit0;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4
libcrux_ml_kem_ind_cca_unpacked_default_30_5b(void)
{
  return
    (
      libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4{
        libcrux_ml_kem_ind_cpa_unpacked_default_8b_5b(),
        { { 0U } }
      }
    );
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_5b(void)
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_d4
  uu____0 = { libcrux_ml_kem_ind_cpa_unpacked_default_70_5b(), { { 0U } } };
  return
    (
      libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked{
        uu____0,
        libcrux_ml_kem_ind_cca_unpacked_default_30_5b()
      }
    );
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.G_41
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_hash_functions_avx2_G_41_23(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_G(input);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.cpa_keygen_seed_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_variant_cpa_keygen_seed_39_39(Eurydice_borrow_slice_u8 key_generation_seed)
{
  Eurydice_arr_fa seed = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d412(&seed,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      )),
    key_generation_seed,
    uint8_t);
  seed.data[LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE] = (uint8_t)(size_t)4U;
  return libcrux_ml_kem_hash_functions_avx2_G_41_23(Eurydice_array_to_slice_shared_b5(&seed));
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN
with const generics
- K= 4
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b0
libcrux_ml_kem_hash_functions_avx2_PRFxN_f5(const Eurydice_arr_890 *input)
{
  Eurydice_arr_3b0 out = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  Eurydice_arr_89 out0 = { { 0U } };
  Eurydice_arr_89 out1 = { { 0U } };
  Eurydice_arr_89 out2 = { { 0U } };
  Eurydice_arr_89 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_shake256(Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_shared_b5(&input->data[1U]),
    Eurydice_array_to_slice_shared_b5(&input->data[2U]),
    Eurydice_array_to_slice_shared_b5(&input->data[3U]),
    Eurydice_array_to_slice_mut_78(&out0),
    Eurydice_array_to_slice_mut_78(&out1),
    Eurydice_array_to_slice_mut_78(&out2),
    Eurydice_array_to_slice_mut_78(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  out.data[2U] = out2;
  out.data[3U] = out3;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN_41
with const generics
- K= 4
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b0
libcrux_ml_kem_hash_functions_avx2_PRFxN_41_f5(const Eurydice_arr_890 *input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRFxN_f5(input);
}

/**
 Sample a vector of ring elements from a centered binomial distribution and
 convert them into their NTT representations.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_vector_cbd_then_ntt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- ETA= 2
- ETA_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d60(
  Eurydice_arr_3b *re_as_ntt,
  const Eurydice_arr_fa *prf_input,
  uint8_t domain_separator
)
{
  Eurydice_arr_890 prf_inputs;
  Eurydice_arr_fa repeat_expression[4U];
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    repeat_expression[i] =
      core_array__core__clone__Clone_for__T__N___clone((size_t)33U,
        prf_input,
        uint8_t,
        Eurydice_arr_fa););
  memcpy(prf_inputs.data, repeat_expression, (size_t)4U * sizeof (Eurydice_arr_fa));
  domain_separator = libcrux_ml_kem_utils_prf_input_inc_23(&prf_inputs, domain_separator);
  Eurydice_arr_3b0 prf_outputs = libcrux_ml_kem_hash_functions_avx2_PRFxN_41_f5(&prf_inputs);
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    re_as_ntt->data[i0] = uu____0;
    libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_84(&re_as_ntt->data[i0]););
  return domain_separator;
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@3]> for libcrux_ml_kem::ind_cpa::generate_keypair_unpacked::closure<Vector, Hasher, Scheme, K, ETA1, ETA1_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3, TraitClause@4, TraitClause@5]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair_unpacked.call_mut_73
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_ab0(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_to_ring_element_5b(
  Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *rhs
)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t i0 = i;
    myself->data[i0] = libcrux_ml_kem_vector_avx2_add_f5(myself->data[i0], &rhs->data[i0]););
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_to_ring_element_d6_5b(
  Eurydice_arr_13 *self,
  const Eurydice_arr_13 *rhs
)
{
  libcrux_ml_kem_polynomial_add_to_ring_element_5b(self, rhs);
}

/**
 Compute Â ◦ ŝ + ê
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_As_plus_e
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_compute_As_plus_e_5b(
  Eurydice_arr_3b *t_as_ntt,
  const Eurydice_arr_cd0 *matrix_A,
  const Eurydice_arr_3b *s_as_ntt,
  const Eurydice_arr_3b *error_as_ntt
)
{
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    const Eurydice_arr_3b *row = &matrix_A->data[i0];
    Eurydice_arr_13 uu____0 = libcrux_ml_kem_polynomial_ZERO_d6_84();
    t_as_ntt->data[i0] = uu____0;
    KRML_MAYBE_FOR4(i1,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i1;
      const Eurydice_arr_13 *matrix_element = &row->data[j];
      Eurydice_arr_13
      product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(matrix_element, &s_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_5b(&t_as_ntt->data[i0], &product););
    libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_84(&t_as_ntt->data[i0],
      &error_as_ntt->data[i0]););
}

/**
 This function implements most of <strong>Algorithm 12</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE key generation algorithm.

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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab0(
  Eurydice_borrow_slice_u8 key_generation_seed,
  Eurydice_arr_3b *private_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 *public_key
)
{
  Eurydice_arr_c7 hashed = libcrux_ml_kem_variant_cpa_keygen_seed_39_39(key_generation_seed);
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      (size_t)32U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_A = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_for_secret_and_error = uu____0.snd;
  Eurydice_arr_cd0 *uu____1 = &public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue0 = libcrux_ml_kem_utils_into_padded_array_de(seed_for_A);
  libcrux_ml_kem_matrix_sample_matrix_A_280(uu____1, &lvalue0, true);
  Eurydice_arr_fa
  prf_input = libcrux_ml_kem_utils_into_padded_array_29(seed_for_secret_and_error);
  uint8_t
  domain_separator =
    libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d60(private_key,
      &prf_input,
      0U);
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_ab0(&lvalue,
        i););
  Eurydice_arr_3b error_as_ntt = arr_struct;
  libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d60(&error_as_ntt,
    &prf_input,
    domain_separator);
  libcrux_ml_kem_matrix_compute_As_plus_e_5b(&public_key->t_as_ntt,
    &public_key->A,
    &private_key[0U],
    &error_as_ntt);
  Eurydice_arr_ec arr;
  memcpy(arr.data, seed_for_A.ptr, (size_t)32U * sizeof (uint8_t));
  Eurydice_arr_ec
  uu____2 =
    core_result_unwrap_26_39(core_result_Result_07_s(core_result_Ok,
        &core_result_Result_07_s::U::case_Ok,
        arr));
  public_key->seed_for_A = uu____2;
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a.closure.call_mut_b4
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_5b(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize), [libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]; K]> for libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a.call_mut_22
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_3b
libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_5b(void **_, size_t tupled_args)
{
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_5b(&lvalue,
        i););
  return arr_struct;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_cd0
libcrux_ml_kem_ind_cca_unpacked_transpose_a_5b(Eurydice_arr_cd0 ind_cpa_a)
{
  Eurydice_arr_cd0 arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_5b(&lvalue, i););
  Eurydice_arr_cd0 A = arr_struct;
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i1 = i0;
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 uu____0 = libcrux_ml_kem_polynomial_clone_c1_84(&ind_cpa_a.data[j].data[i1]);
      A.data[i1].data[j] = uu____0;););
  return A;
}

/**
 Generate Unpacked Keys
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db0(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *out
)
{
  Eurydice_borrow_slice_u8
  ind_cpa_keypair_randomness =
    Eurydice_array_to_subslice_shared_d47(&randomness,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      ));
  Eurydice_borrow_slice_u8
  implicit_rejection_value =
    Eurydice_array_to_subslice_from_shared_5f1(&randomness,
      LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab0(ind_cpa_keypair_randomness,
    &out->private_key.ind_cpa_private_key,
    &out->public_key.ind_cpa_public_key);
  Eurydice_arr_cd0
  A = libcrux_ml_kem_ind_cca_unpacked_transpose_a_5b(out->public_key.ind_cpa_public_key.A);
  out->public_key.ind_cpa_public_key.A = A;
  Eurydice_arr_d1
  pk_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_74(&out->public_key.ind_cpa_public_key.t_as_ntt,
      Eurydice_array_to_slice_shared_01(&out->public_key.ind_cpa_public_key.seed_for_A));
  Eurydice_arr_ec
  uu____0 =
    libcrux_ml_kem_hash_functions_avx2_H_41_23(Eurydice_array_to_slice_shared_b50(&pk_serialized));
  out->public_key.public_key_hash = uu____0;
  Eurydice_arr_ec arr;
  memcpy(arr.data, implicit_rejection_value.ptr, (size_t)32U * sizeof (uint8_t));
  Eurydice_arr_ec
  uu____1 =
    core_result_unwrap_26_39(core_result_Result_07_s(core_result_Ok,
        &core_result_Result_07_s::U::case_Ok,
        arr));
  out->private_key.implicit_rejection_value = uu____1;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b3(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db0(randomness, out);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b3(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b3(randomness, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encaps_prepare
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_c7
libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_39(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_borrow_slice_u8 pk_hash
)
{
  Eurydice_arr_c7 to_hash = libcrux_ml_kem_utils_into_padded_array_c9(randomness);
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f1(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE),
    pk_hash,
    uint8_t);
  return libcrux_ml_kem_hash_functions_avx2_G_41_23(Eurydice_array_to_slice_shared_17(&to_hash));
}

/**
A monomorphic instance of n-tuple
with types Eurydice_arr_3b, libcrux_ml_kem_polynomial_PolynomialRingElement_f6

*/
typedef struct tuple_03_s
{
  Eurydice_arr_3b fst;
  Eurydice_arr_13 snd;
}
tuple_03;

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure<Vector, Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_f1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- C1_LEN= 1408
- U_COMPRESSION_FACTOR= 11
- BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_780(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure#1<Vector, Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- C1_LEN= 1408
- U_COMPRESSION_FACTOR= 11
- BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_780(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Sample a vector of ring elements from a centered binomial distribution.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_ring_element_cbd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- ETA2_RANDOMNESS_SIZE= 128
- ETA2= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_d60(
  const Eurydice_arr_fa *prf_input,
  uint8_t domain_separator,
  Eurydice_arr_3b *error_1
)
{
  Eurydice_arr_890 prf_inputs;
  Eurydice_arr_fa repeat_expression[4U];
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    repeat_expression[i] =
      core_array__core__clone__Clone_for__T__N___clone((size_t)33U,
        prf_input,
        uint8_t,
        Eurydice_arr_fa););
  memcpy(prf_inputs.data, repeat_expression, (size_t)4U * sizeof (Eurydice_arr_fa));
  domain_separator = libcrux_ml_kem_utils_prf_input_inc_23(&prf_inputs, domain_separator);
  Eurydice_arr_3b0 prf_outputs = libcrux_ml_kem_hash_functions_avx2_PRFxN_41_f5(&prf_inputs);
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    error_1->data[i0] = uu____0;);
  return domain_separator;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 4
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_89
libcrux_ml_kem_hash_functions_avx2_PRF_41_f50(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRF_ec(input);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::matrix::compute_vector_u::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u.call_mut_a8
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_5b(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_montgomery
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_5b(Eurydice_arr_13 *re)
{
  size_t zeta_i = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)2U;
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)4U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)7U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
 Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b
libcrux_ml_kem_matrix_compute_vector_u_5b(
  const Eurydice_arr_cd0 *a_as_ntt,
  const Eurydice_arr_3b *r_as_ntt,
  const Eurydice_arr_3b *error_1
)
{
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_5b(&lvalue, i););
  Eurydice_arr_3b result = arr_struct;
  KRML_MAYBE_FOR4(i0,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i1 = i0;
    const Eurydice_arr_3b *row = &a_as_ntt->data[i1];
    KRML_MAYBE_FOR4(i,
      (size_t)0U,
      (size_t)4U,
      (size_t)1U,
      size_t j = i;
      const Eurydice_arr_13 *a_element = &row->data[j];
      Eurydice_arr_13
      product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(a_element, &r_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_5b(&result.data[i1], &product););
    libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_5b(&result.data[i1]);
    libcrux_ml_kem_polynomial_add_error_reduce_d6_84(&result.data[i1], &error_1->data[i1]););
  return result;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- OUT_LEN= 352
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_e7
libcrux_ml_kem_serialize_compress_then_serialize_11_fe(const Eurydice_arr_13 *re)
{
  Eurydice_arr_e7 serialized = { { 0U } };
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    __m256i
    coefficient =
      libcrux_ml_kem_vector_avx2_compress_f5_c4(libcrux_ml_kem_vector_avx2_to_unsigned_representative_f5(re->data[i0]));
    Eurydice_arr_80 bytes = libcrux_ml_kem_vector_avx2_serialize_11_f5(coefficient);
    Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d422(&serialized,
        (core_ops_range_Range_87{ (size_t)22U * i0, (size_t)22U * i0 + (size_t)22U })),
      Eurydice_array_to_slice_shared_980(&bytes),
      uint8_t);
  }
  return serialized;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_ring_element_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- COMPRESSION_FACTOR= 11
- OUT_LEN= 352
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_e7
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_d4(const Eurydice_arr_13 *re)
{
  return libcrux_ml_kem_serialize_compress_then_serialize_11_fe(re);
}

/**
 Call [`compress_then_serialize_ring_element_u`] on each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.compress_then_serialize_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- OUT_LEN= 1408
- COMPRESSION_FACTOR= 11
- BLOCK_LEN= 352
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_compress_then_serialize_u_f8(
  Eurydice_arr_3b input,
  Eurydice_mut_borrow_slice_u8 out
)
{
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13 re = input.data[i0];
    Eurydice_mut_borrow_slice_u8
    uu____0 =
      Eurydice_slice_subslice_mut_c8(out,
        (
          core_ops_range_Range_87{
            i0 * ((size_t)1408U / (size_t)4U),
            (i0 + (size_t)1U) * ((size_t)1408U / (size_t)4U)
          }
        ));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_e7
    lvalue = libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_d4(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_25(&lvalue), uint8_t););
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- C1_LEN= 1408
- U_COMPRESSION_FACTOR= 11
- BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_03
libcrux_ml_kem_ind_cpa_encrypt_c1_780(
  Eurydice_borrow_slice_u8 randomness,
  const Eurydice_arr_cd0 *matrix,
  Eurydice_mut_borrow_slice_u8 ciphertext
)
{
  Eurydice_arr_fa prf_input = libcrux_ml_kem_utils_into_padded_array_29(randomness);
  Eurydice_arr_3b arr_struct0;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct0.data[i] = libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_780(&lvalue, i););
  Eurydice_arr_3b r_as_ntt = arr_struct0;
  uint8_t
  domain_separator0 =
    libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d60(&r_as_ntt,
      &prf_input,
      0U);
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_780(&lvalue, i););
  Eurydice_arr_3b error_1 = arr_struct;
  uint8_t
  domain_separator =
    libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_d60(&prf_input,
      domain_separator0,
      &error_1);
  prf_input.data[32U] = domain_separator;
  Eurydice_arr_89
  prf_output =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_f50(Eurydice_array_to_slice_shared_b5(&prf_input));
  Eurydice_arr_13
  error_2 =
    libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_output));
  Eurydice_arr_3b u = libcrux_ml_kem_matrix_compute_vector_u_5b(matrix, &r_as_ntt, &error_1);
  libcrux_ml_kem_ind_cpa_compress_then_serialize_u_f8(u, ciphertext);
  return (tuple_03{ r_as_ntt, error_2 });
}

/**
 Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_matrix_compute_ring_element_v_5b(
  const Eurydice_arr_3b *t_as_ntt,
  const Eurydice_arr_3b *r_as_ntt,
  const Eurydice_arr_13 *error_2,
  const Eurydice_arr_13 *message
)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    product =
      libcrux_ml_kem_polynomial_ntt_multiply_d6_84(&t_as_ntt->data[i0],
        &r_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_5b(&result, &product););
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_5b(&result);
  return libcrux_ml_kem_polynomial_add_message_error_reduce_d6_84(error_2, message, result);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- COMPRESSION_FACTOR= 5
- OUT_LEN= 160
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_72(
  Eurydice_arr_13 re,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_ml_kem_serialize_compress_then_serialize_5_84(re, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- V_COMPRESSION_FACTOR= 5
- C2_LEN= 160
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_encrypt_c2_72(
  const Eurydice_arr_3b *t_as_ntt,
  const Eurydice_arr_3b *r_as_ntt,
  const Eurydice_arr_13 *error_2,
  const Eurydice_arr_ec *message,
  Eurydice_mut_borrow_slice_u8 ciphertext
)
{
  Eurydice_arr_13
  message_as_ring_element =
    libcrux_ml_kem_serialize_deserialize_then_decompress_message_84(message);
  Eurydice_arr_13
  v =
    libcrux_ml_kem_matrix_compute_ring_element_v_5b(t_as_ntt,
      r_as_ntt,
      error_2,
      &message_as_ring_element);
  libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_72(v, ciphertext);
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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_LEN= 1408
- C2_LEN= 160
- U_COMPRESSION_FACTOR= 11
- V_COMPRESSION_FACTOR= 5
- BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_ind_cpa_encrypt_unpacked_280(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *message,
  Eurydice_borrow_slice_u8 randomness
)
{
  Eurydice_arr_d1 ciphertext = { { 0U } };
  tuple_03
  uu____0 =
    libcrux_ml_kem_ind_cpa_encrypt_c1_780(randomness,
      &public_key->A,
      Eurydice_array_to_subslice_mut_d423(&ciphertext,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)1408U })));
  Eurydice_arr_3b r_as_ntt = uu____0.fst;
  Eurydice_arr_13 error_2 = uu____0.snd;
  libcrux_ml_kem_ind_cpa_encrypt_c2_72(&public_key->t_as_ntt,
    &r_as_ntt,
    &error_2,
    message,
    Eurydice_array_to_subslice_from_mut_5f8(&ciphertext, (size_t)1408U));
  return ciphertext;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- VECTOR_U_BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_25
libcrux_ml_kem_ind_cca_unpacked_encapsulate_a80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_39(Eurydice_array_to_slice_shared_01(randomness),
      Eurydice_array_to_slice_shared_01(&public_key->public_key_hash));
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____0.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____0.snd;
  Eurydice_arr_d1
  ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_unpacked_280(&public_key->ind_cpa_public_key,
      randomness,
      pseudorandomness);
  Eurydice_arr_ec shared_secret_array = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&shared_secret_array),
    shared_secret,
    uint8_t);
  return (tuple_25{ libcrux_ml_kem_types_from_19_d9(ciphertext), shared_secret_array });
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- VECTOR_U_BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_07(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a80(public_key, randomness);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- VECTOR_U_BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_07(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_07(public_key,
      randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cpa::deserialize_then_decompress_u::closure<Vector, K, CIPHERTEXT_SIZE, U_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u.call_mut_35
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- U_COMPRESSION_FACTOR= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_72(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- COMPRESSION_FACTOR= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_95(
  Eurydice_borrow_slice_u8 serialized
)
{
  return libcrux_ml_kem_serialize_deserialize_then_decompress_11_84(serialized);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_vector_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- VECTOR_U_COMPRESSION_FACTOR= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_vector_u_95(Eurydice_arr_13 *re)
{
  size_t zeta_i = (size_t)0U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)7U, (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U, (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U, (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)4U, (size_t)4U * (size_t)3328U);
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
- K= 4
- CIPHERTEXT_SIZE= 1568
- U_COMPRESSION_FACTOR= 11
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_72(const Eurydice_arr_d1 *ciphertext)
{
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_72(&lvalue,
        i););
  Eurydice_arr_3b u_as_ntt = arr_struct;
  for
  (size_t
    i = (size_t)0U;
    i <
      (size_t)1568U /
        (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)11U / (size_t)8U);
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    u_bytes =
      Eurydice_array_to_subslice_shared_d411(ciphertext,
        (
          core_ops_range_Range_87{
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)11U / (size_t)8U),
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)11U / (size_t)8U)
            + LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)11U / (size_t)8U
          }
        ));
    u_as_ntt.data[i0] =
      libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_95(u_bytes);
    libcrux_ml_kem_ntt_ntt_vector_u_95(&u_as_ntt.data[i0]);
  }
  return u_as_ntt;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- COMPRESSION_FACTOR= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_74(
  Eurydice_borrow_slice_u8 serialized
)
{
  return libcrux_ml_kem_serialize_deserialize_then_decompress_5_84(serialized);
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
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_matrix_compute_message_5b(
  const Eurydice_arr_13 *v,
  const Eurydice_arr_3b *secret_as_ntt,
  const Eurydice_arr_3b *u_as_ntt
)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    product =
      libcrux_ml_kem_polynomial_ntt_multiply_d6_84(&secret_as_ntt->data[i0],
        &u_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_5b(&result, &product););
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_5b(&result);
  return libcrux_ml_kem_polynomial_subtract_reduce_d6_84(v, result);
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
- K= 4
- CIPHERTEXT_SIZE= 1568
- VECTOR_U_ENCODED_SIZE= 1408
- U_COMPRESSION_FACTOR= 11
- V_COMPRESSION_FACTOR= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_unpacked_b2(
  const Eurydice_arr_3b *secret_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  Eurydice_arr_3b u_as_ntt = libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_72(ciphertext);
  Eurydice_arr_13
  v =
    libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_74(Eurydice_array_to_subslice_from_shared_5f5(ciphertext,
        (size_t)1408U));
  Eurydice_arr_13 message = libcrux_ml_kem_matrix_compute_message_5b(&v, secret_key, &u_as_ntt);
  return libcrux_ml_kem_serialize_compress_then_serialize_message_84(message);
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 4
- LEN= 32
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_PRF_41_f5(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRF_ce(input);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_unpacked_decapsulate_d90(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
)
{
  Eurydice_arr_ec
  decrypted =
    libcrux_ml_kem_ind_cpa_decrypt_unpacked_b2(&key_pair->private_key.ind_cpa_private_key,
      ciphertext);
  Eurydice_arr_c7
  to_hash0 =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_from_mut_5f1(&to_hash0,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____0,
    Eurydice_array_to_slice_shared_01(&key_pair->public_key.public_key_hash),
    uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_23(Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_14
  to_hash =
    libcrux_ml_kem_utils_into_padded_array_49(Eurydice_array_to_slice_shared_01(&key_pair->private_key.implicit_rejection_value));
  Eurydice_mut_borrow_slice_u8
  uu____2 =
    Eurydice_array_to_subslice_from_mut_5f7(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_d9(ciphertext), uint8_t);
  Eurydice_arr_ec
  implicit_rejection_shared_secret =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_f5(Eurydice_array_to_slice_shared_720(&to_hash));
  Eurydice_arr_d1
  expected_ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_unpacked_280(&key_pair->public_key.ind_cpa_public_key,
      &decrypted,
      pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 = libcrux_ml_kem_types_as_ref_c1_d9(ciphertext);
  uint8_t
  selector =
    libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(uu____3,
      Eurydice_array_to_slice_shared_b50(&expected_ciphertext));
  return
    libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(shared_secret,
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret),
      selector);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_85(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_d90(key_pair, ciphertext);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_85(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_85(key_pair,
      ciphertext);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::serialize::deserialize_ring_elements_reduced_out::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out.call_mut_0b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_5b(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 This function deserializes ring elements and reduces the result by the field
 modulus.

 This function MUST NOT be used on secret inputs.
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_3b
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_5b(
  Eurydice_borrow_slice_u8 public_key
)
{
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_5b(&lvalue,
        i););
  Eurydice_arr_3b deserialized_pk = arr_struct;
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_5b(public_key, &deserialized_pk);
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
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_public_key_74(const Eurydice_arr_d1 *public_key)
{
  Eurydice_arr_3b
  deserialized_pk =
    libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_5b(Eurydice_array_to_subslice_to_shared_214(public_key,
        libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)4U)));
  Eurydice_arr_d1
  public_key_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_74(&deserialized_pk,
      Eurydice_array_to_subslice_from_shared_5f5(public_key,
        libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)4U)));
  return Eurydice_array_eq((size_t)1568U, public_key, &public_key_serialized, uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_f5(
  const Eurydice_arr_d1 *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_74(public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_f5(
  const Eurydice_arr_d1 *public_key
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_f5(public_key);
}

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key_only
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_private_key_only_f8(const Eurydice_arr_a8 *private_key)
{
  Eurydice_arr_ec
  t =
    libcrux_ml_kem_hash_functions_avx2_H_41_23(Eurydice_array_to_subslice_shared_d412(private_key,
        (
          core_ops_range_Range_87{
            (size_t)384U * (size_t)4U,
            (size_t)768U * (size_t)4U + (size_t)32U
          }
        )));
  Eurydice_borrow_slice_u8
  expected =
    Eurydice_array_to_subslice_shared_d412(private_key,
      (
        core_ops_range_Range_87{
          (size_t)768U * (size_t)4U + (size_t)32U,
          (size_t)768U * (size_t)4U + (size_t)64U
        }
      ));
  return Eurydice_array_eq_slice_shared((size_t)32U, &t, &expected, uint8_t, bool);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_f5(
  const Eurydice_arr_a8 *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_f8(private_key);
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
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_private_key_b3(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *_ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_f8(private_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_43(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_b3(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_43(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_43(private_key,
      ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- PRIVATE_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_utils_extraction_helper_Keypair1024
libcrux_ml_kem_ind_cpa_generate_keypair_cc0(Eurydice_borrow_slice_u8 key_generation_seed)
{
  Eurydice_arr_3b private_key = libcrux_ml_kem_ind_cpa_unpacked_default_70_5b();
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4
  public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_5b();
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab0(key_generation_seed,
    &private_key,
    &public_key);
  return libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_72(&public_key, &private_key);
}

/**
 Serialize the secret key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key_mut
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- SERIALIZED_KEY_LEN= 3168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_f8(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value,
  Eurydice_arr_a8 *serialized
)
{
  size_t pointer = (size_t)0U;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d424(serialized,
      (core_ops_range_Range_87{ pointer, pointer + private_key.meta })),
    private_key,
    uint8_t);
  pointer += private_key.meta;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d424(serialized,
      (core_ops_range_Range_87{ pointer, pointer + public_key.meta })),
    public_key,
    uint8_t);
  pointer += public_key.meta;
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_mut_d424(serialized,
      (core_ops_range_Range_87{ pointer, pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE }));
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec lvalue = libcrux_ml_kem_hash_functions_avx2_H_41_23(public_key);
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue), uint8_t);
  pointer += LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d424(serialized,
      (core_ops_range_Range_87{ pointer, pointer + implicit_rejection_value.meta })),
    implicit_rejection_value,
    uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- SERIALIZED_KEY_LEN= 3168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_a8
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_f8(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value
)
{
  Eurydice_arr_a8 out = { { 0U } };
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_f8(private_key,
    public_key,
    implicit_rejection_value,
    &out);
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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_generate_keypair_db0(const Eurydice_arr_c7 *randomness)
{
  Eurydice_borrow_slice_u8
  ind_cpa_keypair_randomness =
    Eurydice_array_to_subslice_shared_d47(randomness,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      ));
  Eurydice_borrow_slice_u8
  implicit_rejection_value =
    Eurydice_array_to_subslice_from_shared_5f1(randomness,
      LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_utils_extraction_helper_Keypair1024
  uu____0 = libcrux_ml_kem_ind_cpa_generate_keypair_cc0(ind_cpa_keypair_randomness);
  Eurydice_arr_df ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_d1 public_key = uu____0.snd;
  Eurydice_arr_a8
  secret_key_serialized =
    libcrux_ml_kem_ind_cca_serialize_kem_secret_key_f8(Eurydice_array_to_slice_shared_2f0(&ind_cpa_private_key),
      Eurydice_array_to_slice_shared_b50(&public_key),
      implicit_rejection_value);
  Eurydice_arr_a8 private_key = libcrux_ml_kem_types_from_b2_0e(secret_key_serialized);
  return
    libcrux_ml_kem_types_from_17_70(private_key,
      libcrux_ml_kem_types_from_51_d9(public_key));
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b3(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_db0(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b3(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b3(randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.entropy_preprocess_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_entropy_preprocess_39_39(Eurydice_borrow_slice_u8 randomness)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), randomness, uint8_t);
  return out;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_910(
  Eurydice_borrow_slice_u8 public_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_5b(Eurydice_slice_subslice_to_shared_72(public_key,
      (size_t)1536U),
    &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8
  seed = Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)1536U);
  Eurydice_arr_cd0 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_280(uu____0, &lvalue, false);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_910(Eurydice_borrow_slice_u8 public_key)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4
  unpacked_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_5b();
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_910(public_key, &unpacked_public_key);
  return unpacked_public_key;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_LEN= 1408
- C2_LEN= 160
- U_COMPRESSION_FACTOR= 11
- V_COMPRESSION_FACTOR= 5
- BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d1
libcrux_ml_kem_ind_cpa_encrypt_280(
  Eurydice_borrow_slice_u8 public_key,
  const Eurydice_arr_ec *message,
  Eurydice_borrow_slice_u8 randomness
)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_d4
  unpacked_public_key = libcrux_ml_kem_ind_cpa_build_unpacked_public_key_910(public_key);
  return libcrux_ml_kem_ind_cpa_encrypt_unpacked_280(&unpacked_public_key, message, randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.kdf_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_kdf_39_f8(
  Eurydice_borrow_slice_u8 shared_secret,
  const Eurydice_arr_d1 *_
)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), shared_secret, uint8_t);
  return out;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_25
libcrux_ml_kem_ind_cca_encapsulate_a10(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  Eurydice_arr_ec
  randomness0 =
    libcrux_ml_kem_variant_entropy_preprocess_39_39(Eurydice_array_to_slice_shared_01(randomness));
  Eurydice_arr_c7
  to_hash =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&randomness0));
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_from_mut_5f1(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE);
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec
  lvalue =
    libcrux_ml_kem_hash_functions_avx2_H_41_23(Eurydice_array_to_slice_shared_b50(libcrux_ml_kem_types_as_slice_e6_d9(public_key)));
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue), uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_23(Eurydice_array_to_slice_shared_17(&to_hash));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_d1
  ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_280(Eurydice_array_to_slice_shared_b50(libcrux_ml_kem_types_as_slice_e6_d9(public_key)),
      &randomness0,
      pseudorandomness);
  Eurydice_arr_d1 uu____2 = libcrux_ml_kem_types_from_19_d9(ciphertext);
  return (tuple_25{ uu____2, libcrux_ml_kem_variant_kdf_39_f8(shared_secret, &ciphertext) });
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- VECTOR_U_BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_07(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_a10(public_key, randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- VECTOR_U_BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_07(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_07(public_key, randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cpa::decrypt::closure<Vector, K, CIPHERTEXT_SIZE, VECTOR_U_ENCODED_SIZE, U_COMPRESSION_FACTOR, V_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt.call_mut_0b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- VECTOR_U_ENCODED_SIZE= 1408
- U_COMPRESSION_FACTOR= 11
- V_COMPRESSION_FACTOR= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_b2(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- VECTOR_U_ENCODED_SIZE= 1408
- U_COMPRESSION_FACTOR= 11
- V_COMPRESSION_FACTOR= 5
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_b2(
  Eurydice_borrow_slice_u8 secret_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  Eurydice_arr_3b arr_struct;
  KRML_MAYBE_FOR4(i,
    (size_t)0U,
    (size_t)4U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_b2(&lvalue, i););
  Eurydice_arr_3b secret_key_unpacked = arr_struct;
  libcrux_ml_kem_ind_cpa_deserialize_vector_5b(secret_key, &secret_key_unpacked);
  return libcrux_ml_kem_ind_cpa_decrypt_unpacked_b2(&secret_key_unpacked, ciphertext);
}

/**
 This code verifies on some machines, runs out of memory on others
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_decapsulate_660(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  Eurydice_borrow_slice_u8_x4
  uu____0 =
    libcrux_ml_kem_types_unpack_private_key_e3(Eurydice_array_to_slice_shared_680(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  Eurydice_arr_ec decrypted = libcrux_ml_kem_ind_cpa_decrypt_b2(ind_cpa_secret_key, ciphertext);
  Eurydice_arr_c7
  to_hash0 =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f1(&to_hash0,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE),
    ind_cpa_public_key_hash,
    uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_23(Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret0 = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_14 to_hash = libcrux_ml_kem_utils_into_padded_array_49(implicit_rejection_value);
  Eurydice_mut_borrow_slice_u8
  uu____2 =
    Eurydice_array_to_subslice_from_mut_5f7(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_d9(ciphertext), uint8_t);
  Eurydice_arr_ec
  implicit_rejection_shared_secret =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_f5(Eurydice_array_to_slice_shared_720(&to_hash));
  Eurydice_arr_d1
  expected_ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_280(ind_cpa_public_key,
      &decrypted,
      pseudorandomness);
  Eurydice_borrow_slice_u8
  uu____3 = Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret);
  Eurydice_arr_ec
  implicit_rejection_shared_secret0 =
    libcrux_ml_kem_variant_kdf_39_f8(uu____3,
      libcrux_ml_kem_types_as_slice_a9_d9(ciphertext));
  Eurydice_arr_ec
  shared_secret =
    libcrux_ml_kem_variant_kdf_39_f8(shared_secret0,
      libcrux_ml_kem_types_as_slice_a9_d9(ciphertext));
  Eurydice_borrow_slice_u8 uu____4 = libcrux_ml_kem_types_as_ref_c1_d9(ciphertext);
  return
    libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(uu____4,
      Eurydice_array_to_slice_shared_b50(&expected_ciphertext),
      Eurydice_array_to_slice_shared_01(&shared_secret),
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret0));
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_85(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_660(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_85(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_85(private_key, ciphertext);
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_f6
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_ee_s { Eurydice_arr_13 data[2U]; } Eurydice_arr_ee;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_ee
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_e21_s { Eurydice_arr_ee data[2U]; } Eurydice_arr_e21;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $2size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7_s
{
  Eurydice_arr_ee t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_e21 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $2size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7;

/**
 See [deserialize_ring_elements_reduced_out].
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_16(
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_arr_ee *deserialized_pk
)
{
  for
  (size_t
    i = (size_t)0U;
    i < public_key.meta / LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT;
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    ring_element =
      Eurydice_slice_subslice_shared_c8(public_key,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT +
              LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
          }
        ));
    Eurydice_arr_13
    uu____0 = libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_84(ring_element);
    deserialized_pk->data[i0] = uu____0;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_af(const Eurydice_arr_bf *input)
{
  Eurydice_arr_c40 state = libcrux_sha3_avx2_x4_incremental_init();
  libcrux_sha3_avx2_x4_incremental_shake128_absorb_final(&state,
    Eurydice_array_to_slice_shared_e9(input->data),
    Eurydice_array_to_slice_shared_e9(&input->data[1U]),
    Eurydice_array_to_slice_shared_e9(input->data),
    Eurydice_array_to_slice_shared_e9(input->data));
  return state;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_init_absorb_final_41
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_af(
  const Eurydice_arr_bf *input
)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_af(input);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b8
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_af(Eurydice_arr_c40 *st)
{
  Eurydice_arr_b8 out = { { { { 0U } }, { { 0U } } } };
  Eurydice_arr_79 out0 = { { 0U } };
  Eurydice_arr_79 out1 = { { 0U } };
  Eurydice_arr_79 out2 = { { 0U } };
  Eurydice_arr_79 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_three_blocks(st,
    Eurydice_array_to_slice_mut_48(&out0),
    Eurydice_array_to_slice_mut_48(&out1),
    Eurydice_array_to_slice_mut_48(&out2),
    Eurydice_array_to_slice_mut_48(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_first_three_blocks_41
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_b8
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_af(
  Eurydice_arr_c40 *self
)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_af(self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT representation
 of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
 We say "partially" because this implementation only accepts a finite set of
 bytes as input and returns an error if the set is not enough; Algorithm 6 of
 the FIPS 203 standard on the other hand samples from an infinite stream of bytes
 until the ring element is filled. Algorithm 6 is reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_uniform_distribution_next
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- N= 504
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ce(
  const Eurydice_arr_b8 *randomness,
  Eurydice_arr_850 *sampled_coefficients,
  Eurydice_arr_800 *out
)
{
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)504U / (size_t)24U; i++)
    {
      size_t r = i;
      if (sampled_coefficients->data[i1] < LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        size_t
        sampled =
          libcrux_ml_kem_vector_avx2_rej_sample_f5(Eurydice_array_to_subslice_shared_d46(&randomness->data[i1],
              (core_ops_range_Range_87{ r * (size_t)24U, r * (size_t)24U + (size_t)24U })),
            Eurydice_array_to_subslice_mut_e7(&out->data[i1],
              (
                core_ops_range_Range_87{
                  sampled_coefficients->data[i1],
                  sampled_coefficients->data[i1] + (size_t)16U
                }
              )));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    });
  bool done = true;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >= LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
    {
      sampled_coefficients->data[i0] = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    }
    else
    {
      done = false;
    });
  return done;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_5b0
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_af(Eurydice_arr_c40 *st)
{
  Eurydice_arr_5b0 out = { { { { 0U } }, { { 0U } } } };
  Eurydice_arr_c5 out0 = { { 0U } };
  Eurydice_arr_c5 out1 = { { 0U } };
  Eurydice_arr_c5 out2 = { { 0U } };
  Eurydice_arr_c5 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_incremental_shake128_squeeze_next_block(st,
    Eurydice_array_to_slice_mut_2c(&out0),
    Eurydice_array_to_slice_mut_2c(&out1),
    Eurydice_array_to_slice_mut_2c(&out2),
    Eurydice_array_to_slice_mut_2c(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.shake128_squeeze_next_block_41
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_5b0
libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_af(Eurydice_arr_c40 *self)
{
  return libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_af(self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT representation
 of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
 We say "partially" because this implementation only accepts a finite set of
 bytes as input and returns an error if the set is not enough; Algorithm 6 of
 the FIPS 203 standard on the other hand samples from an infinite stream of bytes
 until the ring element is filled. Algorithm 6 is reproduced below:

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
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_uniform_distribution_next
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- N= 168
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ce0(
  const Eurydice_arr_5b0 *randomness,
  Eurydice_arr_850 *sampled_coefficients,
  Eurydice_arr_800 *out
)
{
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)24U; i++)
    {
      size_t r = i;
      if (sampled_coefficients->data[i1] < LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        size_t
        sampled =
          libcrux_ml_kem_vector_avx2_rej_sample_f5(Eurydice_array_to_subslice_shared_d40(&randomness->data[i1],
              (core_ops_range_Range_87{ r * (size_t)24U, r * (size_t)24U + (size_t)24U })),
            Eurydice_array_to_subslice_mut_e7(&out->data[i1],
              (
                core_ops_range_Range_87{
                  sampled_coefficients->data[i1],
                  sampled_coefficients->data[i1] + (size_t)16U
                }
              )));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    });
  bool done = true;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >= LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
    {
      sampled_coefficients->data[i0] = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    }
    else
    {
      done = false;
    });
  return done;
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector, Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_28(void **_, Eurydice_arr_5b tupled_args)
{
  Eurydice_arr_5b s = tupled_args;
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_subslice_shared_e70(&s,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)256U })));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ee
libcrux_ml_kem_sampling_sample_from_xof_28(const Eurydice_arr_bf *seeds)
{
  Eurydice_arr_850 sampled_coefficients = { { 0U } };
  Eurydice_arr_800 out = { { { { 0U } }, { { 0U } } } };
  Eurydice_arr_c40
  xof_state = libcrux_ml_kem_hash_functions_avx2_shake128_init_absorb_final_41_af(seeds);
  Eurydice_arr_b8
  randomness0 =
    libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_first_three_blocks_41_af(&xof_state);
  bool
  done =
    libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ce(&randomness0,
      &sampled_coefficients,
      &out);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_5b0
      randomness = libcrux_ml_kem_hash_functions_avx2_shake128_squeeze_next_block_41_af(&xof_state);
      done =
        libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ce0(&randomness,
          &sampled_coefficients,
          &out);
    }
  }
  Eurydice_arr_ee arr_mapped_str;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
      libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_28(&lvalue,
        out.data[i]););
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_sample_matrix_A_28(
  Eurydice_arr_e21 *A_transpose,
  const Eurydice_arr_31 *seed,
  bool transpose
)
{
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i1 = i0;
    Eurydice_arr_bf seeds;
    Eurydice_arr_31 repeat_expression[2U];
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      repeat_expression[i] =
        core_array__core__clone__Clone_for__T__N___clone((size_t)34U,
          seed,
          uint8_t,
          Eurydice_arr_31););
    memcpy(seeds.data, repeat_expression, (size_t)2U * sizeof (Eurydice_arr_31));
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;);
    Eurydice_arr_ee sampled = libcrux_ml_kem_sampling_sample_from_xof_28(&seeds);
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 sample = sampled.data[j];
      if (transpose)
      {
        A_transpose->data[j].data[i1] = sample;
      }
      else
      {
        A_transpose->data[i1].data[j] = sample;
      }););
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.H_41
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_H_41_af(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_H(input);
}

/**
 Generate an unpacked key from a serialized key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.unpack_public_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  Eurydice_borrow_slice_u8
  uu____0 = Eurydice_array_to_subslice_to_shared_212(public_key, (size_t)768U);
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_16(uu____0,
    &unpacked_public_key->ind_cpa_public_key.t_as_ntt);
  unpacked_public_key->ind_cpa_public_key.seed_for_A =
    libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_array_to_subslice_from_shared_5f2(public_key,
        (size_t)768U));
  Eurydice_arr_e21 *uu____2 = &unpacked_public_key->ind_cpa_public_key.A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31
  lvalue =
    libcrux_ml_kem_utils_into_padded_array_de(Eurydice_array_to_subslice_from_shared_5f2(public_key,
        (size_t)768U));
  libcrux_ml_kem_matrix_sample_matrix_A_28(uu____2, &lvalue, false);
  Eurydice_arr_ec
  uu____3 =
    libcrux_ml_kem_hash_functions_avx2_H_41_af(Eurydice_array_to_slice_shared_3b(libcrux_ml_kem_types_as_slice_e6_df(public_key)));
  unpacked_public_key->public_key_hash = uu____3;
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_25(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a(public_key, unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_25(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_25(public_key,
    unpacked_public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $2size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_c7_s
{
  Eurydice_arr_ee ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_c7;

typedef struct libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_c7 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 public_key;
}
libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked;

/**
 Call [`serialize_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_vector
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_serialize_vector_16(
  const Eurydice_arr_ee *key,
  Eurydice_mut_borrow_slice_u8 out
)
{
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13 re = key->data[i0];
    Eurydice_mut_borrow_slice_u8
    uu____0 =
      Eurydice_slice_subslice_mut_c8(out,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            (i0 + (size_t)1U) * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
          }
        ));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b20 lvalue = libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_84(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_a9(&lvalue), uint8_t););
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_serialize_public_key_mut_ce(
  const Eurydice_arr_ee *t_as_ntt,
  Eurydice_borrow_slice_u8 seed_for_a,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cpa_serialize_vector_16(t_as_ntt,
    Eurydice_array_to_subslice_mut_d411(serialized,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)2U)
        }
      )));
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f2(serialized,
      libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)2U)),
    seed_for_a,
    uint8_t);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ce(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *self,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_ce(&self->ind_cpa_public_key.t_as_ntt,
    Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A),
    serialized);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_ce(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ce(&self->public_key, serialized);
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_03
libcrux_ml_kem_ind_cpa_serialize_public_key_ce(
  const Eurydice_arr_ee *t_as_ntt,
  Eurydice_borrow_slice_u8 seed_for_a
)
{
  Eurydice_arr_03 public_key_serialized = { { 0U } };
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_ce(t_as_ntt,
    seed_for_a,
    &public_key_serialized);
  return public_key_serialized;
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_03
libcrux_ml_kem_ind_cca_unpacked_serialized_dd_ce(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *self
)
{
  return
    libcrux_ml_kem_types_from_51_df(libcrux_ml_kem_ind_cpa_serialize_public_key_ce(&self->ind_cpa_public_key.t_as_ntt,
        Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A)));
}

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_03
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_ce(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_dd_ce(&self->public_key);
}

/**
 Serialize the secret key from the unpacked key pair generation.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_unpacked_secret_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PRIVATE_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_utils_extraction_helper_Keypair512
libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_e5(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ee *private_key
)
{
  Eurydice_arr_03
  public_key_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_ce(&public_key->t_as_ntt,
      Eurydice_array_to_slice_shared_01(&public_key->seed_for_A));
  Eurydice_arr_d2 secret_key_serialized = { { 0U } };
  libcrux_ml_kem_ind_cpa_serialize_vector_16(private_key,
    Eurydice_array_to_slice_mut_27(&secret_key_serialized));
  return
    (
      libcrux_ml_kem_utils_extraction_helper_Keypair512{
        secret_key_serialized,
        public_key_serialized
      }
    );
}

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_4e(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self,
  Eurydice_arr_ab0 *serialized
)
{
  libcrux_ml_kem_utils_extraction_helper_Keypair512
  uu____0 =
    libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_e5(&self->public_key.ind_cpa_public_key,
      &self->private_key.ind_cpa_private_key);
  Eurydice_arr_d2 ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_03 ind_cpa_public_key = uu____0.snd;
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_e2(Eurydice_array_to_slice_shared_27(&ind_cpa_private_key),
    Eurydice_array_to_slice_shared_3b(&ind_cpa_public_key),
    Eurydice_array_to_slice_shared_01(&self->private_key.implicit_rejection_value),
    serialized);
}

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ab0
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_4e(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self
)
{
  Eurydice_arr_ab0 sk = libcrux_ml_kem_types_default_d3_be();
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_4e(self, &sk);
  return sk;
}

/**
 Call [`deserialize_to_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_vector
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_deserialize_vector_16(
  Eurydice_borrow_slice_u8 secret_key,
  Eurydice_arr_ee *secret_as_ntt
)
{
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_84(Eurydice_slice_subslice_shared_c8(secret_key,
          (
            core_ops_range_Range_87{
              i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
              (i0 + (size_t)1U) * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT
            }
          )));
    secret_as_ntt->data[i0] = uu____0;);
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector, Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$2size_t]]
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_7e(void **_, Eurydice_arr_5b tupled_args)
{
  Eurydice_arr_5b s = tupled_args;
  return
    libcrux_ml_kem_polynomial_from_i16_array_d6_84(Eurydice_array_to_subslice_shared_e70(&s,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)256U })));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$2size_t]]
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ee
libcrux_ml_kem_sampling_sample_from_xof_7e(const Eurydice_arr_bf *seeds)
{
  Eurydice_arr_850 sampled_coefficients = { { 0U } };
  Eurydice_arr_800 out = { { { { 0U } }, { { 0U } } } };
  Eurydice_arr_e3
  xof_state = libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_af(seeds);
  Eurydice_arr_b8
  randomness0 =
    libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_af(&xof_state);
  bool
  done =
    libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ce(&randomness0,
      &sampled_coefficients,
      &out);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_5b0
      randomness =
        libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_af(&xof_state);
      done =
        libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_ce0(&randomness,
          &sampled_coefficients,
          &out);
    }
  }
  Eurydice_arr_ee arr_mapped_str;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
      libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_7e(&lvalue,
        out.data[i]););
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$2size_t]]
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_sample_matrix_A_7e(
  Eurydice_arr_e21 *A_transpose,
  const Eurydice_arr_31 *seed,
  bool transpose
)
{
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i1 = i0;
    Eurydice_arr_bf seeds;
    Eurydice_arr_31 repeat_expression[2U];
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      repeat_expression[i] =
        core_array__core__clone__Clone_for__T__N___clone((size_t)34U,
          seed,
          uint8_t,
          Eurydice_arr_31););
    memcpy(seeds.data, repeat_expression, (size_t)2U * sizeof (Eurydice_arr_31));
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;);
    Eurydice_arr_ee sampled = libcrux_ml_kem_sampling_sample_from_xof_7e(&seeds);
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 sample = sampled.data[j];
      if (transpose)
      {
        A_transpose->data[j].data[i1] = sample;
      }
      else
      {
        A_transpose->data[i1].data[j] = sample;
      }););
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_portable_PortableHash[[$2size_t]]
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_10(
  Eurydice_borrow_slice_u8 public_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_16(Eurydice_slice_subslice_to_shared_72(public_key,
      (size_t)768U),
    &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8
  seed = Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)768U);
  Eurydice_arr_e21 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_7e(uu____0, &lvalue, false);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.keys_from_private_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_a4(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  Eurydice_borrow_slice_u8_x4
  uu____0 =
    libcrux_ml_kem_types_unpack_private_key_e0(Eurydice_array_to_slice_shared_99(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  libcrux_ml_kem_ind_cpa_deserialize_vector_16(ind_cpa_secret_key,
    &key_pair->private_key.ind_cpa_private_key);
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_10(ind_cpa_public_key,
    &key_pair->public_key.ind_cpa_public_key);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->public_key.public_key_hash),
    ind_cpa_public_key_hash,
    uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->private_key.implicit_rejection_value),
    implicit_rejection_value,
    uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&key_pair->public_key.ind_cpa_public_key.seed_for_A),
    Eurydice_slice_subslice_from_shared_6d(ind_cpa_public_key, (size_t)768U),
    uint8_t);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_c3(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_a4(private_key, key_pair);
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPrivateKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_70
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ee
libcrux_ml_kem_ind_cpa_unpacked_default_70_16(void)
{
  Eurydice_arr_ee lit;
  Eurydice_arr_13 repeat_expression[2U];
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
  memcpy(lit.data, repeat_expression, (size_t)2U * sizeof (Eurydice_arr_13));
  return lit;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_8b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7
libcrux_ml_kem_ind_cpa_unpacked_default_8b_16(void)
{
  Eurydice_arr_ee uu____0;
  Eurydice_arr_13 repeat_expression0[2U];
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    repeat_expression0[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
  memcpy(uu____0.data, repeat_expression0, (size_t)2U * sizeof (Eurydice_arr_13));
  Eurydice_arr_ec uu____1 = { { 0U } };
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 lit0;
  lit0.t_as_ntt = uu____0;
  lit0.seed_for_A = uu____1;
  Eurydice_arr_ee repeat_expression1[2U];
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    Eurydice_arr_ee lit;
    Eurydice_arr_13 repeat_expression[2U];
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_84(););
    memcpy(lit.data, repeat_expression, (size_t)2U * sizeof (Eurydice_arr_13));
    repeat_expression1[i0] = lit;);
  memcpy(lit0.A.data, repeat_expression1, (size_t)2U * sizeof (Eurydice_arr_ee));
  return lit0;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7
libcrux_ml_kem_ind_cca_unpacked_default_30_16(void)
{
  return
    (
      libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7{
        libcrux_ml_kem_ind_cpa_unpacked_default_8b_16(),
        { { 0U } }
      }
    );
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_16(void)
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_c7
  uu____0 = { libcrux_ml_kem_ind_cpa_unpacked_default_70_16(), { { 0U } } };
  return
    (
      libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked{
        uu____0,
        libcrux_ml_kem_ind_cca_unpacked_default_30_16()
      }
    );
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.G_41
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_hash_functions_avx2_G_41_af(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_G(input);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.cpa_keygen_seed_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_variant_cpa_keygen_seed_39_b1(Eurydice_borrow_slice_u8 key_generation_seed)
{
  Eurydice_arr_fa seed = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d412(&seed,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      )),
    key_generation_seed,
    uint8_t);
  seed.data[LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE] = (uint8_t)(size_t)2U;
  return libcrux_ml_kem_hash_functions_avx2_G_41_af(Eurydice_array_to_slice_shared_b5(&seed));
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN
with const generics
- K= 2
- LEN= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_eb
libcrux_ml_kem_hash_functions_avx2_PRFxN_d5(const Eurydice_arr_4d0 *input)
{
  Eurydice_arr_eb out = { { { { 0U } }, { { 0U } } } };
  Eurydice_arr_1c out0 = { { 0U } };
  Eurydice_arr_1c out1 = { { 0U } };
  Eurydice_arr_1c out2 = { { 0U } };
  Eurydice_arr_1c out3 = { { 0U } };
  libcrux_sha3_avx2_x4_shake256(Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_shared_b5(&input->data[1U]),
    Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_mut_d9(&out0),
    Eurydice_array_to_slice_mut_d9(&out1),
    Eurydice_array_to_slice_mut_d9(&out2),
    Eurydice_array_to_slice_mut_d9(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN_41
with const generics
- K= 2
- LEN= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_eb
libcrux_ml_kem_hash_functions_avx2_PRFxN_41_d5(const Eurydice_arr_4d0 *input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRFxN_d5(input);
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_binomial_distribution
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- ETA= 3
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_sampling_sample_from_binomial_distribution_e3(
  Eurydice_borrow_slice_u8 randomness
)
{
  return libcrux_ml_kem_sampling_sample_from_binomial_distribution_3_84(randomness);
}

/**
 Sample a vector of ring elements from a centered binomial distribution and
 convert them into their NTT representations.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_vector_cbd_then_ntt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- ETA= 3
- ETA_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d6(
  Eurydice_arr_ee *re_as_ntt,
  const Eurydice_arr_fa *prf_input,
  uint8_t domain_separator
)
{
  Eurydice_arr_4d0 prf_inputs;
  Eurydice_arr_fa repeat_expression[2U];
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    repeat_expression[i] =
      core_array__core__clone__Clone_for__T__N___clone((size_t)33U,
        prf_input,
        uint8_t,
        Eurydice_arr_fa););
  memcpy(prf_inputs.data, repeat_expression, (size_t)2U * sizeof (Eurydice_arr_fa));
  domain_separator = libcrux_ml_kem_utils_prf_input_inc_af(&prf_inputs, domain_separator);
  Eurydice_arr_eb prf_outputs = libcrux_ml_kem_hash_functions_avx2_PRFxN_41_d5(&prf_inputs);
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_e3(Eurydice_array_to_slice_shared_d9(&prf_outputs.data[i0]));
    re_as_ntt->data[i0] = uu____0;
    libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_84(&re_as_ntt->data[i0]););
  return domain_separator;
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@3]> for libcrux_ml_kem::ind_cpa::generate_keypair_unpacked::closure<Vector, Hasher, Scheme, K, ETA1, ETA1_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3, TraitClause@4, TraitClause@5]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair_unpacked.call_mut_73
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_ab(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_to_ring_element_16(
  Eurydice_arr_13 *myself,
  const Eurydice_arr_13 *rhs
)
{
  KRML_MAYBE_FOR16(i,
    (size_t)0U,
    (size_t)16U,
    (size_t)1U,
    size_t i0 = i;
    myself->data[i0] = libcrux_ml_kem_vector_avx2_add_f5(myself->data[i0], &rhs->data[i0]););
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
This function found in impl {libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element_d6
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_to_ring_element_d6_16(
  Eurydice_arr_13 *self,
  const Eurydice_arr_13 *rhs
)
{
  libcrux_ml_kem_polynomial_add_to_ring_element_16(self, rhs);
}

/**
 Compute Â ◦ ŝ + ê
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_As_plus_e
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_matrix_compute_As_plus_e_16(
  Eurydice_arr_ee *t_as_ntt,
  const Eurydice_arr_e21 *matrix_A,
  const Eurydice_arr_ee *s_as_ntt,
  const Eurydice_arr_ee *error_as_ntt
)
{
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    const Eurydice_arr_ee *row = &matrix_A->data[i0];
    Eurydice_arr_13 uu____0 = libcrux_ml_kem_polynomial_ZERO_d6_84();
    t_as_ntt->data[i0] = uu____0;
    KRML_MAYBE_FOR2(i1,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i1;
      const Eurydice_arr_13 *matrix_element = &row->data[j];
      Eurydice_arr_13
      product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(matrix_element, &s_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_16(&t_as_ntt->data[i0], &product););
    libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_84(&t_as_ntt->data[i0],
      &error_as_ntt->data[i0]););
}

/**
 This function implements most of <strong>Algorithm 12</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE key generation algorithm.

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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab(
  Eurydice_borrow_slice_u8 key_generation_seed,
  Eurydice_arr_ee *private_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 *public_key
)
{
  Eurydice_arr_c7 hashed = libcrux_ml_kem_variant_cpa_keygen_seed_39_b1(key_generation_seed);
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      (size_t)32U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_A = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_for_secret_and_error = uu____0.snd;
  Eurydice_arr_e21 *uu____1 = &public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue0 = libcrux_ml_kem_utils_into_padded_array_de(seed_for_A);
  libcrux_ml_kem_matrix_sample_matrix_A_28(uu____1, &lvalue0, true);
  Eurydice_arr_fa
  prf_input = libcrux_ml_kem_utils_into_padded_array_29(seed_for_secret_and_error);
  uint8_t
  domain_separator =
    libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d6(private_key,
      &prf_input,
      0U);
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_ab(&lvalue,
        i););
  Eurydice_arr_ee error_as_ntt = arr_struct;
  libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d6(&error_as_ntt,
    &prf_input,
    domain_separator);
  libcrux_ml_kem_matrix_compute_As_plus_e_16(&public_key->t_as_ntt,
    &public_key->A,
    &private_key[0U],
    &error_as_ntt);
  Eurydice_arr_ec arr;
  memcpy(arr.data, seed_for_A.ptr, (size_t)32U * sizeof (uint8_t));
  Eurydice_arr_ec
  uu____2 =
    core_result_unwrap_26_39(core_result_Result_07_s(core_result_Ok,
        &core_result_Result_07_s::U::case_Ok,
        arr));
  public_key->seed_for_A = uu____2;
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a.closure.call_mut_b4
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_16(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize), [libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]; K]> for libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a.call_mut_22
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ee
libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_16(void **_, size_t tupled_args)
{
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_16(&lvalue,
        i););
  return arr_struct;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_e21
libcrux_ml_kem_ind_cca_unpacked_transpose_a_16(Eurydice_arr_e21 ind_cpa_a)
{
  Eurydice_arr_e21 arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_16(&lvalue, i););
  Eurydice_arr_e21 A = arr_struct;
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i1 = i0;
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i;
      Eurydice_arr_13 uu____0 = libcrux_ml_kem_polynomial_clone_c1_84(&ind_cpa_a.data[j].data[i1]);
      A.data[i1].data[j] = uu____0;););
  return A;
}

/**
 Generate Unpacked Keys
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *out
)
{
  Eurydice_borrow_slice_u8
  ind_cpa_keypair_randomness =
    Eurydice_array_to_subslice_shared_d47(&randomness,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      ));
  Eurydice_borrow_slice_u8
  implicit_rejection_value =
    Eurydice_array_to_subslice_from_shared_5f1(&randomness,
      LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab(ind_cpa_keypair_randomness,
    &out->private_key.ind_cpa_private_key,
    &out->public_key.ind_cpa_public_key);
  Eurydice_arr_e21
  A = libcrux_ml_kem_ind_cca_unpacked_transpose_a_16(out->public_key.ind_cpa_public_key.A);
  out->public_key.ind_cpa_public_key.A = A;
  Eurydice_arr_03
  pk_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_ce(&out->public_key.ind_cpa_public_key.t_as_ntt,
      Eurydice_array_to_slice_shared_01(&out->public_key.ind_cpa_public_key.seed_for_A));
  Eurydice_arr_ec
  uu____0 =
    libcrux_ml_kem_hash_functions_avx2_H_41_af(Eurydice_array_to_slice_shared_3b(&pk_serialized));
  out->public_key.public_key_hash = uu____0;
  Eurydice_arr_ec arr;
  memcpy(arr.data, implicit_rejection_value.ptr, (size_t)32U * sizeof (uint8_t));
  Eurydice_arr_ec
  uu____1 =
    core_result_unwrap_26_39(core_result_Result_07_s(core_result_Ok,
        &core_result_Result_07_s::U::case_Ok,
        arr));
  out->private_key.implicit_rejection_value = uu____1;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b8(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db(randomness, out);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b8(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b8(randomness, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encaps_prepare
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_c7
libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_b1(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_borrow_slice_u8 pk_hash
)
{
  Eurydice_arr_c7 to_hash = libcrux_ml_kem_utils_into_padded_array_c9(randomness);
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f1(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE),
    pk_hash,
    uint8_t);
  return libcrux_ml_kem_hash_functions_avx2_G_41_af(Eurydice_array_to_slice_shared_17(&to_hash));
}

/**
A monomorphic instance of n-tuple
with types Eurydice_arr_ee, libcrux_ml_kem_polynomial_PolynomialRingElement_f6

*/
typedef struct tuple_91_s
{
  Eurydice_arr_ee fst;
  Eurydice_arr_13 snd;
}
tuple_91;

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure<Vector, Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_f1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- C1_LEN= 640
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_78(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure#1<Vector, Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- C1_LEN= 640
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_78(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN
with const generics
- K= 2
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_f3
libcrux_ml_kem_hash_functions_avx2_PRFxN_d50(const Eurydice_arr_4d0 *input)
{
  Eurydice_arr_f3 out = { { { { 0U } }, { { 0U } } } };
  Eurydice_arr_89 out0 = { { 0U } };
  Eurydice_arr_89 out1 = { { 0U } };
  Eurydice_arr_89 out2 = { { 0U } };
  Eurydice_arr_89 out3 = { { 0U } };
  libcrux_sha3_avx2_x4_shake256(Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_shared_b5(&input->data[1U]),
    Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_shared_b5(input->data),
    Eurydice_array_to_slice_mut_78(&out0),
    Eurydice_array_to_slice_mut_78(&out1),
    Eurydice_array_to_slice_mut_78(&out2),
    Eurydice_array_to_slice_mut_78(&out3));
  out.data[0U] = out0;
  out.data[1U] = out1;
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRFxN_41
with const generics
- K= 2
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_f3
libcrux_ml_kem_hash_functions_avx2_PRFxN_41_d50(const Eurydice_arr_4d0 *input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRFxN_d50(input);
}

/**
 Sample a vector of ring elements from a centered binomial distribution.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_ring_element_cbd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- ETA2_RANDOMNESS_SIZE= 128
- ETA2= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_d6(
  const Eurydice_arr_fa *prf_input,
  uint8_t domain_separator,
  Eurydice_arr_ee *error_1
)
{
  Eurydice_arr_4d0 prf_inputs;
  Eurydice_arr_fa repeat_expression[2U];
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    repeat_expression[i] =
      core_array__core__clone__Clone_for__T__N___clone((size_t)33U,
        prf_input,
        uint8_t,
        Eurydice_arr_fa););
  memcpy(prf_inputs.data, repeat_expression, (size_t)2U * sizeof (Eurydice_arr_fa));
  domain_separator = libcrux_ml_kem_utils_prf_input_inc_af(&prf_inputs, domain_separator);
  Eurydice_arr_f3 prf_outputs = libcrux_ml_kem_hash_functions_avx2_PRFxN_41_d50(&prf_inputs);
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    uu____0 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    error_1->data[i0] = uu____0;);
  return domain_separator;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 2
- LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_89
libcrux_ml_kem_hash_functions_avx2_PRF_41_d50(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRF_ec(input);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::matrix::compute_vector_u::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u.call_mut_a8
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_16(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_montgomery
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_16(Eurydice_arr_13 *re)
{
  size_t zeta_i = LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)2U;
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_84(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)4U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)5U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)6U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_84(&zeta_i, re, (size_t)7U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_84(re);
}

/**
 Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ee
libcrux_ml_kem_matrix_compute_vector_u_16(
  const Eurydice_arr_e21 *a_as_ntt,
  const Eurydice_arr_ee *r_as_ntt,
  const Eurydice_arr_ee *error_1
)
{
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_16(&lvalue, i););
  Eurydice_arr_ee result = arr_struct;
  KRML_MAYBE_FOR2(i0,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i1 = i0;
    const Eurydice_arr_ee *row = &a_as_ntt->data[i1];
    KRML_MAYBE_FOR2(i,
      (size_t)0U,
      (size_t)2U,
      (size_t)1U,
      size_t j = i;
      const Eurydice_arr_13 *a_element = &row->data[j];
      Eurydice_arr_13
      product = libcrux_ml_kem_polynomial_ntt_multiply_d6_84(a_element, &r_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_16(&result.data[i1], &product););
    libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_16(&result.data[i1]);
    libcrux_ml_kem_polynomial_add_error_reduce_d6_84(&result.data[i1], &error_1->data[i1]););
  return result;
}

/**
 Call [`compress_then_serialize_ring_element_u`] on each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.compress_then_serialize_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- OUT_LEN= 640
- COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_compress_then_serialize_u_4e(
  Eurydice_arr_ee input,
  Eurydice_mut_borrow_slice_u8 out
)
{
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13 re = input.data[i0];
    Eurydice_mut_borrow_slice_u8
    uu____0 =
      Eurydice_slice_subslice_mut_c8(out,
        (
          core_ops_range_Range_87{
            i0 * ((size_t)640U / (size_t)2U),
            (i0 + (size_t)1U) * ((size_t)640U / (size_t)2U)
          }
        ));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b0
    lvalue = libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_81(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_56(&lvalue), uint8_t););
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- C1_LEN= 640
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_91
libcrux_ml_kem_ind_cpa_encrypt_c1_78(
  Eurydice_borrow_slice_u8 randomness,
  const Eurydice_arr_e21 *matrix,
  Eurydice_mut_borrow_slice_u8 ciphertext
)
{
  Eurydice_arr_fa prf_input = libcrux_ml_kem_utils_into_padded_array_29(randomness);
  Eurydice_arr_ee arr_struct0;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct0.data[i] = libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_78(&lvalue, i););
  Eurydice_arr_ee r_as_ntt = arr_struct0;
  uint8_t
  domain_separator0 =
    libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_d6(&r_as_ntt,
      &prf_input,
      0U);
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_78(&lvalue, i););
  Eurydice_arr_ee error_1 = arr_struct;
  uint8_t
  domain_separator =
    libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_d6(&prf_input,
      domain_separator0,
      &error_1);
  prf_input.data[32U] = domain_separator;
  Eurydice_arr_89
  prf_output =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_d50(Eurydice_array_to_slice_shared_b5(&prf_input));
  Eurydice_arr_13
  error_2 =
    libcrux_ml_kem_sampling_sample_from_binomial_distribution_16(Eurydice_array_to_slice_shared_78(&prf_output));
  Eurydice_arr_ee u = libcrux_ml_kem_matrix_compute_vector_u_16(matrix, &r_as_ntt, &error_1);
  libcrux_ml_kem_ind_cpa_compress_then_serialize_u_4e(u, ciphertext);
  return (tuple_91{ r_as_ntt, error_2 });
}

/**
 Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_matrix_compute_ring_element_v_16(
  const Eurydice_arr_ee *t_as_ntt,
  const Eurydice_arr_ee *r_as_ntt,
  const Eurydice_arr_13 *error_2,
  const Eurydice_arr_13 *message
)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    product =
      libcrux_ml_kem_polynomial_ntt_multiply_d6_84(&t_as_ntt->data[i0],
        &r_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_16(&result, &product););
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_16(&result);
  return libcrux_ml_kem_polynomial_add_message_error_reduce_d6_84(error_2, message, result);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- COMPRESSION_FACTOR= 4
- OUT_LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_e5(
  Eurydice_arr_13 re,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_ml_kem_serialize_compress_then_serialize_4_84(re, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c2
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- V_COMPRESSION_FACTOR= 4
- C2_LEN= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_encrypt_c2_e5(
  const Eurydice_arr_ee *t_as_ntt,
  const Eurydice_arr_ee *r_as_ntt,
  const Eurydice_arr_13 *error_2,
  const Eurydice_arr_ec *message,
  Eurydice_mut_borrow_slice_u8 ciphertext
)
{
  Eurydice_arr_13
  message_as_ring_element =
    libcrux_ml_kem_serialize_deserialize_then_decompress_message_84(message);
  Eurydice_arr_13
  v =
    libcrux_ml_kem_matrix_compute_ring_element_v_16(t_as_ntt,
      r_as_ntt,
      error_2,
      &message_as_ring_element);
  libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_e5(v, ciphertext);
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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_LEN= 640
- C2_LEN= 128
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
- BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d2
libcrux_ml_kem_ind_cpa_encrypt_unpacked_28(
  const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *message,
  Eurydice_borrow_slice_u8 randomness
)
{
  Eurydice_arr_d2 ciphertext = { { 0U } };
  tuple_91
  uu____0 =
    libcrux_ml_kem_ind_cpa_encrypt_c1_78(randomness,
      &public_key->A,
      Eurydice_array_to_subslice_mut_d414(&ciphertext,
        (core_ops_range_Range_87{ (size_t)0U, (size_t)640U })));
  Eurydice_arr_ee r_as_ntt = uu____0.fst;
  Eurydice_arr_13 error_2 = uu____0.snd;
  libcrux_ml_kem_ind_cpa_encrypt_c2_e5(&public_key->t_as_ntt,
    &r_as_ntt,
    &error_2,
    message,
    Eurydice_array_to_subslice_from_mut_5f3(&ciphertext, (size_t)640U));
  return ciphertext;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_ab
libcrux_ml_kem_ind_cca_unpacked_encapsulate_a8(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_b1(Eurydice_array_to_slice_shared_01(randomness),
      Eurydice_array_to_slice_shared_01(&public_key->public_key_hash));
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____0.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____0.snd;
  Eurydice_arr_d2
  ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_unpacked_28(&public_key->ind_cpa_public_key,
      randomness,
      pseudorandomness);
  Eurydice_arr_ec shared_secret_array = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&shared_secret_array),
    shared_secret,
    uint8_t);
  return (tuple_ab{ libcrux_ml_kem_types_from_19_80(ciphertext), shared_secret_array });
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a8(public_key, randomness);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_80(public_key,
      randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cpa::deserialize_then_decompress_u::closure<Vector, K, CIPHERTEXT_SIZE, U_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u.call_mut_35
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_e5(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 Call [`deserialize_then_decompress_ring_element_u`] on each ring element
 in the `ciphertext`.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- U_COMPRESSION_FACTOR= 10
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ee
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_e5(const Eurydice_arr_d2 *ciphertext)
{
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_e5(&lvalue,
        i););
  Eurydice_arr_ee u_as_ntt = arr_struct;
  for
  (size_t
    i = (size_t)0U;
    i <
      (size_t)768U /
        (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U);
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    u_bytes =
      Eurydice_array_to_subslice_shared_d45(ciphertext,
        (
          core_ops_range_Range_87{
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U),
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U)
            + LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)10U / (size_t)8U
          }
        ));
    u_as_ntt.data[i0] =
      libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_d0(u_bytes);
    libcrux_ml_kem_ntt_ntt_vector_u_d0(&u_as_ntt.data[i0]);
  }
  return u_as_ntt;
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_v
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_ce(
  Eurydice_borrow_slice_u8 serialized
)
{
  return libcrux_ml_kem_serialize_deserialize_then_decompress_4_84(serialized);
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
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_13
libcrux_ml_kem_matrix_compute_message_16(
  const Eurydice_arr_13 *v,
  const Eurydice_arr_ee *secret_as_ntt,
  const Eurydice_arr_ee *u_as_ntt
)
{
  Eurydice_arr_13 result = libcrux_ml_kem_polynomial_ZERO_d6_84();
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    size_t i0 = i;
    Eurydice_arr_13
    product =
      libcrux_ml_kem_polynomial_ntt_multiply_d6_84(&secret_as_ntt->data[i0],
        &u_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_16(&result, &product););
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_16(&result);
  return libcrux_ml_kem_polynomial_subtract_reduce_d6_84(v, result);
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
- K= 2
- CIPHERTEXT_SIZE= 768
- VECTOR_U_ENCODED_SIZE= 640
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_unpacked_a4(
  const Eurydice_arr_ee *secret_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  Eurydice_arr_ee u_as_ntt = libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_e5(ciphertext);
  Eurydice_arr_13
  v =
    libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_ce(Eurydice_array_to_subslice_from_shared_5f0(ciphertext,
        (size_t)640U));
  Eurydice_arr_13 message = libcrux_ml_kem_matrix_compute_message_16(&v, secret_key, &u_as_ntt);
  return libcrux_ml_kem_serialize_compress_then_serialize_message_84(message);
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::avx2::Simd256Hash}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.avx2.PRF_41
with const generics
- K= 2
- LEN= 32
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_hash_functions_avx2_PRF_41_d5(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_kem_hash_functions_avx2_PRF_ce(input);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_unpacked_decapsulate_d9(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
)
{
  Eurydice_arr_ec
  decrypted =
    libcrux_ml_kem_ind_cpa_decrypt_unpacked_a4(&key_pair->private_key.ind_cpa_private_key,
      ciphertext);
  Eurydice_arr_c7
  to_hash0 =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_from_mut_5f1(&to_hash0,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____0,
    Eurydice_array_to_slice_shared_01(&key_pair->public_key.public_key_hash),
    uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_af(Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_03
  to_hash =
    libcrux_ml_kem_utils_into_padded_array_df(Eurydice_array_to_slice_shared_01(&key_pair->private_key.implicit_rejection_value));
  Eurydice_mut_borrow_slice_u8
  uu____2 =
    Eurydice_array_to_subslice_from_mut_5f2(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_80(ciphertext), uint8_t);
  Eurydice_arr_ec
  implicit_rejection_shared_secret =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_d5(Eurydice_array_to_slice_shared_3b(&to_hash));
  Eurydice_arr_d2
  expected_ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_unpacked_28(&key_pair->public_key.ind_cpa_public_key,
      &decrypted,
      pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 = libcrux_ml_kem_types_as_ref_c1_80(ciphertext);
  uint8_t
  selector =
    libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(uu____3,
      Eurydice_array_to_slice_shared_27(&expected_ciphertext));
  return
    libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(shared_secret,
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret),
      selector);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_37(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_d9(key_pair, ciphertext);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_37(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_37(key_pair,
      ciphertext);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::serialize::deserialize_ring_elements_reduced_out::closure<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out.call_mut_0b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_16(
  void **_,
  size_t tupled_args
)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
 This function deserializes ring elements and reduces the result by the field
 modulus.

 This function MUST NOT be used on secret inputs.
*/
/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ee
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_16(
  Eurydice_borrow_slice_u8 public_key
)
{
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
      libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_16(&lvalue,
        i););
  Eurydice_arr_ee deserialized_pk = arr_struct;
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_16(public_key, &deserialized_pk);
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
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_public_key_ce(const Eurydice_arr_03 *public_key)
{
  Eurydice_arr_ee
  deserialized_pk =
    libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_16(Eurydice_array_to_subslice_to_shared_212(public_key,
        libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)2U)));
  Eurydice_arr_03
  public_key_serialized =
    libcrux_ml_kem_ind_cpa_serialize_public_key_ce(&deserialized_pk,
      Eurydice_array_to_subslice_from_shared_5f2(public_key,
        libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)2U)));
  return Eurydice_array_eq((size_t)800U, public_key, &public_key_serialized, uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_d5(
  const Eurydice_arr_03 *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_ce(public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_d5(
  const Eurydice_arr_03 *public_key
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_d5(public_key);
}

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key_only
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_private_key_only_37(const Eurydice_arr_ab0 *private_key)
{
  Eurydice_arr_ec
  t =
    libcrux_ml_kem_hash_functions_avx2_H_41_af(Eurydice_array_to_subslice_shared_d48(private_key,
        (
          core_ops_range_Range_87{
            (size_t)384U * (size_t)2U,
            (size_t)768U * (size_t)2U + (size_t)32U
          }
        )));
  Eurydice_borrow_slice_u8
  expected =
    Eurydice_array_to_subslice_shared_d48(private_key,
      (
        core_ops_range_Range_87{
          (size_t)768U * (size_t)2U + (size_t)32U,
          (size_t)768U * (size_t)2U + (size_t)64U
        }
      ));
  return Eurydice_array_eq_slice_shared((size_t)32U, &t, &expected, uint8_t, bool);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_d5(
  const Eurydice_arr_ab0 *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_37(private_key);
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
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_validate_private_key_85(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *_ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_37(private_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_25(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_85(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_25(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_25(private_key,
      ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- PRIVATE_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_utils_extraction_helper_Keypair512
libcrux_ml_kem_ind_cpa_generate_keypair_cc(Eurydice_borrow_slice_u8 key_generation_seed)
{
  Eurydice_arr_ee private_key = libcrux_ml_kem_ind_cpa_unpacked_default_70_16();
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7
  public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_16();
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_ab(key_generation_seed,
    &private_key,
    &public_key);
  return libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_e5(&public_key, &private_key);
}

/**
 Serialize the secret key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key_mut
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- SERIALIZED_KEY_LEN= 1632
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_37(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value,
  Eurydice_arr_ab0 *serialized
)
{
  size_t pointer = (size_t)0U;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d416(serialized,
      (core_ops_range_Range_87{ pointer, pointer + private_key.meta })),
    private_key,
    uint8_t);
  pointer += private_key.meta;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d416(serialized,
      (core_ops_range_Range_87{ pointer, pointer + public_key.meta })),
    public_key,
    uint8_t);
  pointer += public_key.meta;
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_mut_d416(serialized,
      (core_ops_range_Range_87{ pointer, pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE }));
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec lvalue = libcrux_ml_kem_hash_functions_avx2_H_41_af(public_key);
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue), uint8_t);
  pointer += LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d416(serialized,
      (core_ops_range_Range_87{ pointer, pointer + implicit_rejection_value.meta })),
    implicit_rejection_value,
    uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- SERIALIZED_KEY_LEN= 1632
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ab0
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_37(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value
)
{
  Eurydice_arr_ab0 out = { { 0U } };
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_37(private_key,
    public_key,
    implicit_rejection_value,
    &out);
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
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_generate_keypair_db(const Eurydice_arr_c7 *randomness)
{
  Eurydice_borrow_slice_u8
  ind_cpa_keypair_randomness =
    Eurydice_array_to_subslice_shared_d47(randomness,
      (
        core_ops_range_Range_87{
          (size_t)0U,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE
        }
      ));
  Eurydice_borrow_slice_u8
  implicit_rejection_value =
    Eurydice_array_to_subslice_from_shared_5f1(randomness,
      LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_utils_extraction_helper_Keypair512
  uu____0 = libcrux_ml_kem_ind_cpa_generate_keypair_cc(ind_cpa_keypair_randomness);
  Eurydice_arr_d2 ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_03 public_key = uu____0.snd;
  Eurydice_arr_ab0
  secret_key_serialized =
    libcrux_ml_kem_ind_cca_serialize_kem_secret_key_37(Eurydice_array_to_slice_shared_27(&ind_cpa_private_key),
      Eurydice_array_to_slice_shared_3b(&public_key),
      implicit_rejection_value);
  Eurydice_arr_ab0 private_key = libcrux_ml_kem_types_from_b2_be(secret_key_serialized);
  return
    libcrux_ml_kem_types_from_17_d6(private_key,
      libcrux_ml_kem_types_from_51_df(public_key));
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b8(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_db(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b8(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b8(randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.entropy_preprocess_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_entropy_preprocess_39_b1(Eurydice_borrow_slice_u8 randomness)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), randomness, uint8_t);
  return out;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_91(
  Eurydice_borrow_slice_u8 public_key,
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_16(Eurydice_slice_subslice_to_shared_72(public_key,
      (size_t)768U),
    &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8
  seed = Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)768U);
  Eurydice_arr_e21 *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_28(uu____0, &lvalue, false);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_91(Eurydice_borrow_slice_u8 public_key)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7
  unpacked_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_16();
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_91(public_key, &unpacked_public_key);
  return unpacked_public_key;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_LEN= 640
- C2_LEN= 128
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
- BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_d2
libcrux_ml_kem_ind_cpa_encrypt_28(
  Eurydice_borrow_slice_u8 public_key,
  const Eurydice_arr_ec *message,
  Eurydice_borrow_slice_u8 randomness
)
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_c7
  unpacked_public_key = libcrux_ml_kem_ind_cpa_build_unpacked_public_key_91(public_key);
  return libcrux_ml_kem_ind_cpa_encrypt_unpacked_28(&unpacked_public_key, message, randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.kdf_39
with types libcrux_ml_kem_hash_functions_avx2_Simd256Hash
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_kdf_39_37(
  Eurydice_borrow_slice_u8 shared_secret,
  const Eurydice_arr_d2 *_
)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), shared_secret, uint8_t);
  return out;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.encapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE tuple_ab
libcrux_ml_kem_ind_cca_encapsulate_a1(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  Eurydice_arr_ec
  randomness0 =
    libcrux_ml_kem_variant_entropy_preprocess_39_b1(Eurydice_array_to_slice_shared_01(randomness));
  Eurydice_arr_c7
  to_hash =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&randomness0));
  Eurydice_mut_borrow_slice_u8
  uu____0 =
    Eurydice_array_to_subslice_from_mut_5f1(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE);
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec
  lvalue =
    libcrux_ml_kem_hash_functions_avx2_H_41_af(Eurydice_array_to_slice_shared_3b(libcrux_ml_kem_types_as_slice_e6_df(public_key)));
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue), uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_af(Eurydice_array_to_slice_shared_17(&to_hash));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_d2
  ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_28(Eurydice_array_to_slice_shared_3b(libcrux_ml_kem_types_as_slice_e6_df(public_key)),
      &randomness0,
      pseudorandomness);
  Eurydice_arr_d2 uu____2 = libcrux_ml_kem_types_from_19_80(ciphertext);
  return (tuple_ab{ uu____2, libcrux_ml_kem_variant_kdf_39_37(shared_secret, &ciphertext) });
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_80(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_a1(public_key, randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_80(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_80(public_key, randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize), libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0, TraitClause@1]> for libcrux_ml_kem::ind_cpa::decrypt::closure<Vector, K, CIPHERTEXT_SIZE, VECTOR_U_ENCODED_SIZE, U_COMPRESSION_FACTOR, V_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt.call_mut_0b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- VECTOR_U_ENCODED_SIZE= 640
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_13
libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_a4(void **_, size_t tupled_args)
{
  return libcrux_ml_kem_polynomial_ZERO_d6_84();
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- VECTOR_U_ENCODED_SIZE= 640
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_a4(
  Eurydice_borrow_slice_u8 secret_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  Eurydice_arr_ee arr_struct;
  KRML_MAYBE_FOR2(i,
    (size_t)0U,
    (size_t)2U,
    (size_t)1U,
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] = libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_a4(&lvalue, i););
  Eurydice_arr_ee secret_key_unpacked = arr_struct;
  libcrux_ml_kem_ind_cpa_deserialize_vector_16(secret_key, &secret_key_unpacked);
  return libcrux_ml_kem_ind_cpa_decrypt_unpacked_a4(&secret_key_unpacked, ciphertext);
}

/**
 This code verifies on some machines, runs out of memory on others
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.decapsulate
with types libcrux_ml_kem_vector_avx2_SIMD256Vector, libcrux_ml_kem_hash_functions_avx2_Simd256Hash, libcrux_ml_kem_variant_MlKem
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_decapsulate_66(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  Eurydice_borrow_slice_u8_x4
  uu____0 =
    libcrux_ml_kem_types_unpack_private_key_e0(Eurydice_array_to_slice_shared_99(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  Eurydice_arr_ec decrypted = libcrux_ml_kem_ind_cpa_decrypt_a4(ind_cpa_secret_key, ciphertext);
  Eurydice_arr_c7
  to_hash0 =
    libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f1(&to_hash0,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE),
    ind_cpa_public_key_hash,
    uint8_t);
  Eurydice_arr_c7
  hashed =
    libcrux_ml_kem_hash_functions_avx2_G_41_af(Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret0 = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_03 to_hash = libcrux_ml_kem_utils_into_padded_array_df(implicit_rejection_value);
  Eurydice_mut_borrow_slice_u8
  uu____2 =
    Eurydice_array_to_subslice_from_mut_5f2(&to_hash,
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_80(ciphertext), uint8_t);
  Eurydice_arr_ec
  implicit_rejection_shared_secret =
    libcrux_ml_kem_hash_functions_avx2_PRF_41_d5(Eurydice_array_to_slice_shared_3b(&to_hash));
  Eurydice_arr_d2
  expected_ciphertext =
    libcrux_ml_kem_ind_cpa_encrypt_28(ind_cpa_public_key,
      &decrypted,
      pseudorandomness);
  Eurydice_borrow_slice_u8
  uu____3 = Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret);
  Eurydice_arr_ec
  implicit_rejection_shared_secret0 =
    libcrux_ml_kem_variant_kdf_39_37(uu____3,
      libcrux_ml_kem_types_as_slice_a9_80(ciphertext));
  Eurydice_arr_ec
  shared_secret =
    libcrux_ml_kem_variant_kdf_39_37(shared_secret0,
      libcrux_ml_kem_types_as_slice_a9_80(ciphertext));
  Eurydice_borrow_slice_u8 uu____4 = libcrux_ml_kem_types_as_ref_c1_80(ciphertext);
  return
    libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(uu____4,
      Eurydice_array_to_slice_shared_27(&expected_ciphertext),
      Eurydice_array_to_slice_shared_01(&shared_secret),
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret0));
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_37(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_66(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_37(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_37(private_key, ciphertext);
}


#define libcrux_mlkem_avx2_H_DEFINED
#endif /* libcrux_mlkem_avx2_H */
