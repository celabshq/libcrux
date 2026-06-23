/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 6f058254eb741c12e9b388df07adaf7cc8aac8ed
 * Eurydice: fca2e9fbd728e49d677f3fc0da0054b55f3b9973
 * Karamel: 8c19d41458ce5cbfea029ebc03334ba96d149039
 * F*: 70671ffb81fa30aba09b9d6e2af275dfbccaa8f8
 * Libcrux: 03a9dbf07ad389374e301a47b3f0418a247bc6b0
 */


#ifndef libcrux_mldsa_avx2_H
#define libcrux_mldsa_avx2_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "intrinsics/libcrux_intrinsics_avx2.h"

#include "libcrux_sha3_portable.h"
#include "libcrux_sha3_avx2.h"
#include "libcrux_mldsa_core.h"
#include "combined_core.h"

typedef libcrux_sha3_avx2_x4_incremental_KeccakState
libcrux_ml_dsa_hash_functions_simd256_Shake128x4;

typedef libcrux_sha3_portable_KeccakState libcrux_ml_dsa_hash_functions_simd256_Shake256;

typedef libcrux_sha3_avx2_x4_incremental_KeccakState
libcrux_ml_dsa_hash_functions_simd256_Shake256x4;

/**
 Init the state and absorb 4 blocks in parallel.
*/
Eurydice_arr_c40
libcrux_ml_dsa_hash_functions_simd256_init_absorb(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
);

Eurydice_arr_7c
libcrux_ml_dsa_hash_functions_simd256_init_absorb_final_shake256(
  Eurydice_borrow_slice_u8 input
);

Eurydice_arr_c40
libcrux_ml_dsa_hash_functions_simd256_init_absorb_x4(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
);

Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_shake256(Eurydice_arr_7c *state);

Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_x4(Eurydice_arr_c40 *state);

void
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_five_blocks(
  Eurydice_arr_c40 *state,
  Eurydice_arr_d10 *out0,
  Eurydice_arr_d10 *out1,
  Eurydice_arr_d10 *out2,
  Eurydice_arr_d10 *out3
);

Eurydice_arr_c5_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block(Eurydice_arr_c40 *state);

Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_shake256(Eurydice_arr_7c *state);

Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4(Eurydice_arr_c40 *state);

/**
 Init the state and absorb 4 blocks in parallel.
*/
/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake128::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake128x4}
*/
Eurydice_arr_c40
libcrux_ml_dsa_hash_functions_simd256_init_absorb_38(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake128::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake128x4}
*/
void
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_five_blocks_38(
  Eurydice_arr_c40 *self,
  Eurydice_arr_d10 *out0,
  Eurydice_arr_d10 *out1,
  Eurydice_arr_d10 *out2,
  Eurydice_arr_d10 *out3
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake128::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake128x4}
*/
Eurydice_arr_c5_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_38(Eurydice_arr_c40 *self);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
Eurydice_arr_7c
libcrux_ml_dsa_hash_functions_simd256_init_absorb_final_21(Eurydice_borrow_slice_u8 input);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_21(Eurydice_arr_7c *self);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_21(Eurydice_arr_7c *self);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
Eurydice_arr_c40
libcrux_ml_dsa_hash_functions_simd256_init_absorb_x4_39(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_first_block_x4_39(Eurydice_arr_c40 *self);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_simd256_squeeze_next_block_x4_39(Eurydice_arr_c40 *self);

typedef __m256i libcrux_ml_dsa_simd_avx2_vector_type_Vec256;

/**
 Create an all-zero vector coefficient
*/
__m256i libcrux_ml_dsa_simd_avx2_vector_type_zero(void);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
__m256i libcrux_ml_dsa_simd_avx2_zero_9a(void);

/**
 Create a coefficient from an `i32` array
*/
void
libcrux_ml_dsa_simd_avx2_vector_type_from_coefficient_array(
  Eurydice_dst_ref_shared_83 coefficient_array,
  __m256i *out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_from_coefficient_array_9a(
  Eurydice_dst_ref_shared_83 coefficient_array,
  __m256i *out
);

/**
 Write out the coefficient to an `i32` array
*/
void
libcrux_ml_dsa_simd_avx2_vector_type_to_coefficient_array(
  const __m256i *value,
  Eurydice_dst_ref_mut_83 out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_to_coefficient_array_9a(
  const __m256i *value,
  Eurydice_dst_ref_mut_83 out
);

void libcrux_ml_dsa_simd_avx2_arithmetic_add(__m256i *lhs, const __m256i *rhs);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_add_9a(__m256i *lhs, const __m256i *rhs);

void libcrux_ml_dsa_simd_avx2_arithmetic_subtract(__m256i *lhs, const __m256i *rhs);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_subtract_9a(__m256i *lhs, const __m256i *rhs);

bool
libcrux_ml_dsa_simd_avx2_arithmetic_infinity_norm_exceeds(
  const __m256i *simd_unit,
  int32_t bound
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
bool
libcrux_ml_dsa_simd_avx2_infinity_norm_exceeds_9a(const __m256i *simd_unit, int32_t bound);

__m256i libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives_ret(const __m256i *t);

void
libcrux_ml_dsa_simd_avx2_arithmetic_decompose(
  int32_t gamma2,
  const __m256i *r,
  __m256i *r0,
  __m256i *r1
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_decompose_9a(
  int32_t gamma2,
  const __m256i *simd_unit,
  __m256i *low,
  __m256i *high
);

size_t
libcrux_ml_dsa_simd_avx2_arithmetic_compute_hint(
  const __m256i *low,
  const __m256i *high,
  int32_t gamma2,
  __m256i *hint
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
size_t
libcrux_ml_dsa_simd_avx2_compute_hint_9a(
  const __m256i *low,
  const __m256i *high,
  int32_t gamma2,
  __m256i *hint
);

void
libcrux_ml_dsa_simd_avx2_arithmetic_use_hint(int32_t gamma2, const __m256i *r, __m256i *hint);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_use_hint_9a(int32_t gamma2, const __m256i *simd_unit, __m256i *hint);

void
libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_aux(
  __m256i field_modulus,
  __m256i inverse_of_modulus_mod_montgomery_r,
  __m256i *lhs,
  const __m256i *rhs
);

void libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply(__m256i *lhs, const __m256i *rhs);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_montgomery_multiply_9a(__m256i *lhs, const __m256i *rhs);

void libcrux_ml_dsa_simd_avx2_arithmetic_barrett_reduce_simd_unit(__m256i *simd_unit);

void libcrux_ml_dsa_simd_avx2_arithmetic_to_unsigned_representatives(__m256i *t);

void libcrux_ml_dsa_simd_avx2_arithmetic_power2round(__m256i *r0, __m256i *r1);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_power2round_9a(__m256i *t0, __m256i *t1);

#define LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_LESS_THAN_FIELD_MODULUS_BYTESTREAM_TO_POTENTIAL_COEFFICIENTS_COEFFICIENT_MASK ((int32_t)((uint32_t)1 << 23U) - 1)

__m256i
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_bytestream_to_potential_coefficients(
  Eurydice_borrow_slice_u8 serialized
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_REJECTION_SAMPLE_SHUFFLE_TABLE_SHUFFLE_TABLE ((KRML_CLITERAL(Eurydice_arr_a30){ .data = { { .data = { 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U } }, { .data = { 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U } } } }))

size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_sample(
  Eurydice_borrow_slice_u8 input,
  Eurydice_dst_ref_mut_83 output
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_field_modulus_9a(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_DESERIALIZE_TO_UNSIGNED_WHEN_ETA_IS_4_COEFFICIENT_MASK ((int32_t)((uint32_t)1 << 4U) - 1)

__m256i
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned_when_eta_is_4(
  Eurydice_borrow_slice_u8 bytes
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_DESERIALIZE_TO_UNSIGNED_WHEN_ETA_IS_2_COEFFICIENT_MASK ((int32_t)((uint32_t)1 << 3U) - 1)

__m256i
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned_when_eta_is_2(
  Eurydice_borrow_slice_u8 bytes
);

__m256i
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize_to_unsigned(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.shift_interval
with const generics
- ETA= 2
*/
__m256i
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_shift_interval_af(__m256i coefficients);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.sample
with const generics
- ETA= 2
*/
size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_sample_af(
  Eurydice_borrow_slice_u8 input,
  Eurydice_dst_ref_mut_83 output
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_equals_2_9a(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.shift_interval
with const generics
- ETA= 4
*/
__m256i
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_shift_interval_23(__m256i coefficients);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.rejection_sample.less_than_eta.sample
with const generics
- ETA= 4
*/
size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_sample_23(
  Eurydice_borrow_slice_u8 input,
  Eurydice_dst_ref_mut_83 output
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
size_t
libcrux_ml_dsa_simd_avx2_rejection_sample_less_than_eta_equals_4_9a(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_2_POW_19 ((int32_t)((uint32_t)1 << 19U))

__m256i
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_19_aux(
  __m256i simd_unit_shifted
);

void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_2_POW_17 ((int32_t)((uint32_t)1 << 17U))

__m256i
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_17_aux(
  __m256i simd_unit_shifted
);

void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_serialize(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized,
  size_t gamma1_exponent
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_gamma1_serialize_9a(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized,
  size_t gamma1_exponent
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17 ((int32_t)((uint32_t)1 << 17U))

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17_TIMES_2_MASK ((int32_t)((uint32_t)LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_17 << 1U) - 1)

void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17_unsigned(
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19 ((int32_t)((uint32_t)1 << 19U))

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19_TIMES_2_MASK ((int32_t)((uint32_t)LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_GAMMA1_GAMMA1_19 << 1U) - 1)

void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19_unsigned(
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out
);

void
libcrux_ml_dsa_simd_avx2_encoding_gamma1_deserialize(
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out,
  size_t gamma1_exponent
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_gamma1_deserialize_9a(
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out,
  size_t gamma1_exponent
);

__m128i libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize_4(const __m256i *simd_unit);

__m256i libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize_6(const __m256i *simd_unit);

void
libcrux_ml_dsa_simd_avx2_encoding_commitment_serialize(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_commitment_serialize_9a(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_ETA_4 (4)

__m128i
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_4_aux(__m256i simd_unit_shifted);

void
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_4(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_ERROR_ETA_2 (2)

__m128i
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_2_aux(__m256i simd_unit_shifted);

void
libcrux_ml_dsa_simd_avx2_encoding_error_serialize_when_eta_is_2(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

void
libcrux_ml_dsa_simd_avx2_encoding_error_serialize(
  libcrux_ml_dsa_constants_Eta eta,
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_error_serialize_9a(
  libcrux_ml_dsa_constants_Eta eta,
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
);

void
libcrux_ml_dsa_simd_avx2_encoding_error_deserialize(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_error_deserialize_9a(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T0_POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE ((int32_t)((uint32_t)1 << (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T - (size_t)1U)))

__m256i libcrux_ml_dsa_simd_avx2_encoding_t0_change_interval(const __m256i *simd_unit);

__m128i libcrux_ml_dsa_simd_avx2_encoding_t0_serialize_aux(__m256i simd_unit);

void
libcrux_ml_dsa_simd_avx2_encoding_t0_serialize(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_t0_serialize_9a(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T0_DESERIALIZE_UNSIGNED_COEFFICIENT_MASK ((int32_t)((uint32_t)1 << 13U) - 1)

void
libcrux_ml_dsa_simd_avx2_encoding_t0_deserialize_unsigned(
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out
);

void
libcrux_ml_dsa_simd_avx2_encoding_t0_deserialize(
  Eurydice_borrow_slice_u8 serialized,
  __m256i *out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_t0_deserialize_9a(Eurydice_borrow_slice_u8 serialized, __m256i *out);

void
libcrux_ml_dsa_simd_avx2_encoding_t1_serialize(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_t1_serialize_9a(
  const __m256i *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_ENCODING_T1_DESERIALIZE_COEFFICIENT_MASK ((int32_t)((uint32_t)1 << 10U) - 1)

void
libcrux_ml_dsa_simd_avx2_encoding_t1_deserialize(Eurydice_borrow_slice_u8 bytes, __m256i *out);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void
libcrux_ml_dsa_simd_avx2_t1_deserialize_9a(Eurydice_borrow_slice_u8 serialized, __m256i *out);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_60_s { __m256i data[32U]; } Eurydice_arr_60;

void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6_mul(
  Eurydice_arr_60 *re,
  size_t index,
  __m256i zeta,
  size_t step_by,
  __m256i field_modulus,
  __m256i inverse_of_modulus_mod_montgomery_r
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_7 ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_7_AND_6_STEP_BY_6 ((size_t)8U)

/**
 This is equivalent to the pqclean 0 and 1

 This does 32 Montgomery multiplications (192 multiplications).
 This is the same as in pqclean. The only difference is locality of registers.
*/
void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_7_and_6(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.ntt.ntt_at_layer_5_to_3.round
with const generics
- STEP= 32
- STEP_BY= 4
*/
void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_90(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta
);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.ntt.ntt_at_layer_5_to_3.round
with const generics
- STEP= 16
- STEP_BY= 2
*/
void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_8e(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta
);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.ntt.ntt_at_layer_5_to_3.round
with const generics
- STEP= 8
- STEP_BY= 1
*/
void
libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3_round_f4(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta
);

/**
 Layer 5, 4, 3

 Each layer does 16 Montgomery multiplications -> 3*16 = 48 total
 pqclean does 4 * 4 on each layer -> 48 total | plus 4 * 4 shuffles every time (48)
*/
void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_5_to_3(Eurydice_arr_60 *re);

void
libcrux_ml_dsa_simd_avx2_ntt_butterfly_8(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta0,
  int32_t zeta1
);

void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_2(Eurydice_arr_60 *re);

void
libcrux_ml_dsa_simd_avx2_ntt_butterfly_4(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta_a0,
  int32_t zeta_a1,
  int32_t zeta_b0,
  int32_t zeta_b1
);

void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_1(Eurydice_arr_60 *re);

void
libcrux_ml_dsa_simd_avx2_ntt_butterfly_2(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta_a0,
  int32_t zeta_a1,
  int32_t zeta_a2,
  int32_t zeta_a3,
  int32_t zeta_b0,
  int32_t zeta_b1,
  int32_t zeta_b2,
  int32_t zeta_b3
);

void libcrux_ml_dsa_simd_avx2_ntt_ntt_at_layer_0(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_ntt_ntt_avx2_ntt(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_ntt_ntt(Eurydice_arr_60 *re);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_ntt_9a(Eurydice_arr_60 *simd_units);

typedef struct libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2_s
{
  __m256i fst;
  __m256i snd;
}
libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2;

libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2
libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_0(
  __m256i simd_unit0,
  __m256i simd_unit1,
  int32_t zeta00,
  int32_t zeta01,
  int32_t zeta02,
  int32_t zeta03,
  int32_t zeta10,
  int32_t zeta11,
  int32_t zeta12,
  int32_t zeta13
);

void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0_round(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta00,
  int32_t zeta01,
  int32_t zeta02,
  int32_t zeta03,
  int32_t zeta10,
  int32_t zeta11,
  int32_t zeta12,
  int32_t zeta13
);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_0(Eurydice_arr_60 *re);

libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2
libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_1(
  __m256i simd_unit0,
  __m256i simd_unit1,
  int32_t zeta00,
  int32_t zeta01,
  int32_t zeta10,
  int32_t zeta11
);

void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1_round(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta_00,
  int32_t zeta_01,
  int32_t zeta_10,
  int32_t zeta_11
);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_1(Eurydice_arr_60 *re);

libcrux_ml_dsa_simd_avx2_vector_type_Vec256_x2
libcrux_ml_dsa_simd_avx2_invntt_simd_unit_invert_ntt_at_layer_2(
  __m256i simd_unit0,
  __m256i simd_unit1,
  int32_t zeta0,
  int32_t zeta1
);

void
libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2_round(
  Eurydice_arr_60 *re,
  size_t index,
  int32_t zeta1,
  int32_t zeta2
);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_2(Eurydice_arr_60 *re);

__m256i
libcrux_ml_dsa_simd_avx2_arithmetic_montgomery_multiply_by_constant(
  __m256i lhs,
  int32_t constant
);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 1
- ZETA= 280005
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_30(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 2
- STEP_BY= 1
- ZETA= 4010497
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_25(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 1
- ZETA= -19422
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_43(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 6
- STEP_BY= 1
- ZETA= 1757237
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_f4(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 1
- ZETA= -3277672
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_82(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 10
- STEP_BY= 1
- ZETA= -1399561
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_1d(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 1
- ZETA= -3859737
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_ea(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 14
- STEP_BY= 1
- ZETA= -2118186
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_d8(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 1
- ZETA= -2108549
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_42(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 18
- STEP_BY= 1
- ZETA= 2619752
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_60(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 1
- ZETA= -1119584
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_61(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 22
- STEP_BY= 1
- ZETA= -549488
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_29(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 1
- ZETA= 3585928
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 26
- STEP_BY= 1
- ZETA= -1079900
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_9d(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 1
- ZETA= 1024112
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_38(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 30
- STEP_BY= 1
- ZETA= 2725464
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_5f(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_3(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 2
- ZETA= 2680103
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_300(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 2
- ZETA= 3111497
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_430(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 2
- ZETA= -2884855
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_820(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 2
- ZETA= 3119733
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_ea0(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 2
- ZETA= -2091905
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_420(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 2
- ZETA= -359251
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_610(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 2
- ZETA= 2353451
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe0(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 2
- ZETA= 1826347
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_380(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_4(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 4
- ZETA= 466468
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_301(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 4
- ZETA= -876248
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_821(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 4
- ZETA= -777960
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_421(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 4
- ZETA= 237124
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_fe1(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_5(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 8
- ZETA= -518909
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_302(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 8
- ZETA= -2608894
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_422(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_6(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 16
- ZETA= 25847
*/
void libcrux_ml_dsa_simd_avx2_invntt_outer_3_plus_303(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_at_layer_7(Eurydice_arr_60 *re);

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_MONTGOMERY_INV_INNER_FACTOR (41978)

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_montgomery_inv_inner(Eurydice_arr_60 *re);

void libcrux_ml_dsa_simd_avx2_invntt_invert_ntt_montgomery(Eurydice_arr_60 *re);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_invert_ntt_montgomery_9a(Eurydice_arr_60 *simd_units);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
void libcrux_ml_dsa_simd_avx2_barrett_reduce_simd_unit_9a(__m256i *simd_unit);

/**
A monomorphic instance of libcrux_ml_dsa.polynomial.PolynomialRingElement
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256

*/
typedef Eurydice_arr_60 libcrux_ml_dsa_polynomial_PolynomialRingElement_4b;

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.zero_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
Eurydice_arr_60 libcrux_ml_dsa_polynomial_zero_e5_94(void);

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b, size_t

*/
typedef struct Eurydice_dst_ref_mut_ba_s
{
  Eurydice_arr_60 *ptr;
  size_t meta;
}
Eurydice_dst_ref_mut_ba;

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_4b, size_t

*/
typedef struct Eurydice_dst_ref_shared_ba_s
{
  const Eurydice_arr_60 *ptr;
  size_t meta;
}
Eurydice_dst_ref_shared_ba;

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_4
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_94(
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled_coefficients,
  Eurydice_arr_d0 *out
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_2
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_94(
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled_coefficients,
  Eurydice_arr_d0 *out
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_94(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled,
  Eurydice_arr_d0 *out
);

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.from_i32_array_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_polynomial_from_i32_array_e5_94(
  Eurydice_dst_ref_shared_83 array,
  Eurydice_arr_60 *result
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_four_error_ring_elements
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
void
libcrux_ml_dsa_sample_sample_four_error_ring_elements_fc(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 seed,
  uint16_t start_index,
  Eurydice_dst_ref_mut_ba re
);

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.sample_s1_and_s2
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
void
libcrux_ml_dsa_samplex4_sample_s1_and_s2_fc(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_ba s1_s2
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_field_modulus
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
bool
libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_94(
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled_coefficients,
  Eurydice_arr_d0 *out
);

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
A monomorphic instance of libcrux_ml_dsa.sample.sample_up_to_four_ring_elements_flat
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake128x4
with const generics

*/
void
libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_0a(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_ba matrix,
  Eurydice_arr_d10 *rand_stack0,
  Eurydice_arr_d10 *rand_stack1,
  Eurydice_arr_d10 *rand_stack2,
  Eurydice_arr_d10 *rand_stack3,
  Eurydice_dst_ref_mut_33 tmp_stack,
  size_t start_index,
  size_t elements_requested
);

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.matrix_flat
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake128x4
with const generics

*/
void
libcrux_ml_dsa_samplex4_matrix_flat_0a(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_ba matrix
);

/**
This function found in impl {impl libcrux_ml_dsa::samplex4::X4Sampler for libcrux_ml_dsa::samplex4::avx2::AVX2Sampler}
*/
/**
A monomorphic instance of libcrux_ml_dsa.samplex4.avx2.matrix_flat.inner_88
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_samplex4_avx2_matrix_flat_inner_88_94(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_ba matrix
);

/**
This function found in impl {impl libcrux_ml_dsa::samplex4::X4Sampler for libcrux_ml_dsa::samplex4::avx2::AVX2Sampler}
*/
/**
A monomorphic instance of libcrux_ml_dsa.samplex4.avx2.matrix_flat_88
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_samplex4_avx2_matrix_flat_88_94(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_ba matrix
);

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void libcrux_ml_dsa_ntt_ntt_94(Eurydice_arr_60 *re);

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt_multiply_montgomery
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_ntt_ntt_multiply_montgomery_94(Eurydice_arr_60 *lhs, const Eurydice_arr_60 *rhs);

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.add_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void libcrux_ml_dsa_polynomial_add_e5_94(Eurydice_arr_60 *self, const Eurydice_arr_60 *rhs);

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.barrett_reduce_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void libcrux_ml_dsa_polynomial_barrett_reduce_e5_94(Eurydice_arr_60 *self);

/**
A monomorphic instance of libcrux_ml_dsa.ntt.invert_ntt_montgomery
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void libcrux_ml_dsa_ntt_invert_ntt_montgomery_94(Eurydice_arr_60 *re);

/**
 Compute InvertNTT(Â ◦ ŝ₁) + s₂
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_as1_plus_s2
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_matrix_compute_as1_plus_s2_94(
  size_t rows_in_a,
  size_t columns_in_a,
  Eurydice_dst_ref_mut_ba a_as_ntt,
  Eurydice_dst_ref_shared_ba s1_ntt,
  Eurydice_dst_ref_shared_ba s1_s2,
  Eurydice_dst_ref_mut_ba result
);

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.power2round_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_arithmetic_power2round_vector_94(
  Eurydice_dst_ref_mut_ba t,
  Eurydice_dst_ref_mut_ba t1
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_t1_serialize_94(
  const Eurydice_arr_60 *re,
  Eurydice_mut_borrow_slice_u8 serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.verification_key.generate_serialized
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_verification_key_generate_serialized_94(
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_shared_ba t1,
  Eurydice_mut_borrow_slice_u8 verification_key_serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256
with const generics
- OUTPUT_LENGTH= 64
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_c9(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_c7 *out
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_21
with const generics
- OUTPUT_LENGTH= 64
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_21_c9(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_c7 *out
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_error_serialize_94(
  libcrux_ml_dsa_constants_Eta eta,
  const Eurydice_arr_60 *re,
  Eurydice_mut_borrow_slice_u8 serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_t0_serialize_94(
  const Eurydice_arr_60 *re,
  Eurydice_mut_borrow_slice_u8 serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signing_key.generate_serialized
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake256
with const generics

*/
void
libcrux_ml_dsa_encoding_signing_key_generate_serialized_18(
  libcrux_ml_dsa_constants_Eta eta,
  size_t error_ring_element_size,
  Eurydice_borrow_slice_u8 seed_matrix,
  Eurydice_borrow_slice_u8 seed_signing,
  Eurydice_borrow_slice_u8 verification_key,
  Eurydice_dst_ref_shared_ba s1_2,
  Eurydice_dst_ref_shared_ba t0,
  Eurydice_mut_borrow_slice_u8 signing_key_serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.generate_key_pair
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
void
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_generate_key_pair_07(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

/**
 Key Generation.
*/
void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_generate_key_pair__inner(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_generate_key_pair(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_error_deserialize_94(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_60 *result
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.deserialize_to_vector_then_ntt
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_94(
  libcrux_ml_dsa_constants_Eta eta,
  size_t ring_element_size,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_dst_ref_mut_ba ring_elements
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_t0_deserialize_94(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_60 *result
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.deserialize_to_vector_then_ntt
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_94(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_dst_ref_mut_ba ring_elements
);

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4
with const generics
- OUT_LEN= 576
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_5a(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_arr_220 *out0,
  Eurydice_arr_220 *out1,
  Eurydice_arr_220 *out2,
  Eurydice_arr_220 *out3
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4_39
with const generics
- OUT_LEN= 576
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_39_5a(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_arr_220 *out0,
  Eurydice_arr_220 *out1,
  Eurydice_arr_220 *out2,
  Eurydice_arr_220 *out3
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_gamma1_deserialize_94(
  size_t gamma1_exponent,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_60 *result
);

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4
with const generics
- OUT_LEN= 640
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_0e(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_arr_20 *out0,
  Eurydice_arr_20 *out1,
  Eurydice_arr_20 *out2,
  Eurydice_arr_20 *out3
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::simd256::Shake256x4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_x4_39
with const generics
- OUT_LEN= 640
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_x4_39_0e(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_arr_20 *out0,
  Eurydice_arr_20 *out1,
  Eurydice_arr_20 *out2,
  Eurydice_arr_20 *out3
);

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256
with const generics
- OUTPUT_LENGTH= 640
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_0e(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_20 *out
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_21
with const generics
- OUTPUT_LENGTH= 640
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_21_0e(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_20 *out
);

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256
with const generics
- OUTPUT_LENGTH= 576
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_5a(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_220 *out
);

/**
This function found in impl {impl libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::simd256::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.simd256.shake256_21
with const generics
- OUTPUT_LENGTH= 576
*/
void
libcrux_ml_dsa_hash_functions_simd256_shake256_21_5a(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_220 *out
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_ring_element
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake256
with const generics

*/
void
libcrux_ml_dsa_sample_sample_mask_ring_element_18(
  const Eurydice_arr_91 *seed,
  Eurydice_arr_60 *result,
  size_t gamma1_exponent
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
void
libcrux_ml_dsa_sample_sample_mask_vector_f4(
  size_t dimension,
  size_t gamma1_exponent,
  const Eurydice_arr_c7 *seed,
  uint16_t *domain_separator,
  Eurydice_dst_ref_mut_ba mask
);

/**
 Compute InvertNTT(Â ◦ ŷ)
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_matrix_x_mask
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_matrix_compute_matrix_x_mask_94(
  size_t rows_in_a,
  size_t columns_in_a,
  Eurydice_dst_ref_shared_ba matrix,
  Eurydice_dst_ref_shared_ba mask,
  Eurydice_dst_ref_mut_ba result
);

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.decompose_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_arithmetic_decompose_vector_94(
  size_t dimension,
  int32_t gamma2,
  Eurydice_dst_ref_shared_ba t,
  Eurydice_dst_ref_mut_ba low,
  Eurydice_dst_ref_mut_ba high
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_commitment_serialize_94(
  const Eurydice_arr_60 *re,
  Eurydice_mut_borrow_slice_u8 serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize_vector
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_commitment_serialize_vector_94(
  size_t ring_element_size,
  Eurydice_dst_ref_shared_ba vector,
  Eurydice_mut_borrow_slice_u8 serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_challenge_ring_element
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_hash_functions_simd256_Shake256
with const generics

*/
void
libcrux_ml_dsa_sample_sample_challenge_ring_element_18(
  Eurydice_borrow_slice_u8 seed,
  size_t number_of_ones,
  Eurydice_arr_60 *re
);

/**
A monomorphic instance of libcrux_ml_dsa.matrix.vector_times_ring_element
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_matrix_vector_times_ring_element_94(
  Eurydice_dst_ref_mut_ba vector,
  const Eurydice_arr_60 *ring_element
);

/**
A monomorphic instance of libcrux_ml_dsa.matrix.add_vectors
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_matrix_add_vectors_94(
  size_t dimension,
  Eurydice_dst_ref_mut_ba lhs,
  Eurydice_dst_ref_shared_ba rhs
);

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.subtract_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_polynomial_subtract_e5_94(Eurydice_arr_60 *self, const Eurydice_arr_60 *rhs);

/**
A monomorphic instance of libcrux_ml_dsa.matrix.subtract_vectors
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_matrix_subtract_vectors_94(
  size_t dimension,
  Eurydice_dst_ref_mut_ba lhs,
  Eurydice_dst_ref_shared_ba rhs
);

/**
 CAUTION: This function must only be called with inputs for
 which it is safe to leak the index of a violating coefficient.

 For all norm checks during ML-DSA signature generation it is
 safe to leak the index of a violating coefficient.
*/
/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.infinity_norm_exceeds_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
bool
libcrux_ml_dsa_polynomial_infinity_norm_exceeds_e5_94(
  const Eurydice_arr_60 *self,
  int32_t bound
);

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
bool
libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_94(
  Eurydice_dst_ref_shared_ba vector,
  int32_t bound
);

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.to_i32_array_e5
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
Eurydice_arr_6c libcrux_ml_dsa_polynomial_to_i32_array_e5_94(const Eurydice_arr_60 *self);

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.make_hint
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
size_t
libcrux_ml_dsa_arithmetic_make_hint_94(
  Eurydice_dst_ref_shared_ba low,
  Eurydice_dst_ref_shared_ba high,
  int32_t gamma2,
  Eurydice_dst_ref_mut_20 hint
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_gamma1_serialize_94(
  const Eurydice_arr_60 *re,
  Eurydice_mut_borrow_slice_u8 serialized,
  size_t gamma1_exponent
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.serialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_signature_serialize_94(
  Eurydice_borrow_slice_u8 commitment_hash,
  Eurydice_dst_ref_shared_ba signer_response,
  Eurydice_dst_ref_shared_20 hint,
  size_t commitment_hash_size,
  size_t columns_in_a,
  size_t rows_in_a,
  size_t gamma1_exponent,
  size_t gamma1_ring_element_size,
  size_t max_ones_in_hint,
  Eurydice_mut_borrow_slice_u8 signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.sign_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_sign_internal_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_85 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.sign_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_sign_mut_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_85 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.sign
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_48
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_sign_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

core_result_Result_48
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_sign__inner(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Sign.
*/
core_result_Result_48
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_sign(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_sign_mut__inner(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_85 *signature
);

/**
 Sign.
*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_sign_mut(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_85 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.sign_pre_hashed_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_sign_pre_hashed_mut_37(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness,
  Eurydice_arr_85 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.sign_pre_hashed
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_48
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_sign_pre_hashed_37(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

core_result_Result_48
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_sign_pre_hashed_shake128__inner(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

/**
 Sign (pre-hashed).
*/
core_result_Result_48
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_sign_pre_hashed_shake128(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_t1_deserialize_94(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_60 *result
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.verification_key.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_encoding_verification_key_deserialize_94(
  size_t rows_in_a,
  size_t verification_key_size,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_dst_ref_mut_ba t1
);

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.deserialize
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_encoding_signature_deserialize_94(
  size_t columns_in_a,
  size_t rows_in_a,
  size_t commitment_hash_size,
  size_t gamma1_exponent,
  size_t gamma1_ring_element_size,
  size_t max_ones_in_hint,
  size_t signature_size,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_mut_borrow_slice_u8 out_commitment_hash,
  Eurydice_dst_ref_mut_ba out_signer_response,
  Eurydice_dst_ref_mut_20 out_hint
);

/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.arithmetic.shift_left_then_reduce
with const generics
- SHIFT_BY= 13
*/
void libcrux_ml_dsa_simd_avx2_arithmetic_shift_left_then_reduce_84(__m256i *simd_unit);

/**
This function found in impl {impl libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.simd.avx2.shift_left_then_reduce_9a
with const generics
- SHIFT_BY= 13
*/
void libcrux_ml_dsa_simd_avx2_shift_left_then_reduce_9a_84(__m256i *simd_unit);

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.shift_left_then_reduce
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics
- SHIFT_BY= 13
*/
void libcrux_ml_dsa_arithmetic_shift_left_then_reduce_3a(Eurydice_arr_60 *re);

/**
 Compute InvertNTT(Â ◦ ẑ - ĉ ◦ NTT(t₁2ᵈ))
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_w_approx
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_matrix_compute_w_approx_94(
  size_t rows_in_a,
  size_t columns_in_a,
  Eurydice_dst_ref_shared_ba matrix,
  Eurydice_dst_ref_shared_ba signer_response,
  const Eurydice_arr_60 *verifier_challenge_as_ntt,
  Eurydice_dst_ref_mut_ba t1
);

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.use_hint
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256
with const generics

*/
void
libcrux_ml_dsa_arithmetic_use_hint_94(
  int32_t gamma2,
  Eurydice_dst_ref_shared_20 hint,
  Eurydice_dst_ref_mut_ba re_vector
);

/**
 The internal verification API.

 If no `domain_separation_context` is supplied, it is assumed that
 `message` already contains the domain separation.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.verify_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_verify_internal_07(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  const Eurydice_arr_85 *signature_serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.verify
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_verify_07(
  const Eurydice_arr_02 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_85 *signature_serialized
);

core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_verify__inner(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_85 *signature
);

/**
 Verify.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_verify(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_85 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_44.verify_pre_hashed
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_verify_pre_hashed_37(
  const Eurydice_arr_02 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_85 *signature_serialized
);

core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_verify_pre_hashed_shake128__inner(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_85 *signature
);

/**
 Verify (pre-hashed with SHAKE-128).
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_44_verify_pre_hashed_shake128(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_85 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.generate_key_pair
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
void
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_07(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

/**
 Key Generation.
*/
void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair__inner(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign__inner(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Sign.
*/
core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_mut__inner(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
);

/**
 Sign.
*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_mut(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_37(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_37(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_pre_hashed_shake128__inner(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

/**
 Sign (pre-hashed).
*/
core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_pre_hashed_shake128(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

/**
 The internal verification API.

 If no `domain_separation_context` is supplied, it is assumed that
 `message` already contains the domain separation.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_07(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  const Eurydice_arr_0c *signature_serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_07(
  const Eurydice_arr_29 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature_serialized
);

core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify__inner(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature
);

/**
 Verify.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_pre_hashed
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_37(
  const Eurydice_arr_29 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_0c *signature_serialized
);

core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify_pre_hashed_shake128__inner(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_0c *signature
);

/**
 Verify (pre-hashed with SHAKE-128).
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify_pre_hashed_shake128(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_0c *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.generate_key_pair
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
void
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_generate_key_pair_07(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

/**
 Key Generation.
*/
void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_generate_key_pair__inner(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

void
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_generate_key_pair(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.sign_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_sign_internal_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_93 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.sign_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_sign_mut_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_93 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.sign
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4
with const generics

*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_sign_07(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

core_result_Result_8b
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_sign__inner(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Sign.
*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_sign(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_sign_mut__inner(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_93 *signature
);

/**
 Sign.
*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_sign_mut(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_93 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.sign_pre_hashed_mut
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_sign_pre_hashed_mut_37(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness,
  Eurydice_arr_93 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.sign_pre_hashed
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_simd256_Shake256x4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_sign_pre_hashed_37(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

core_result_Result_8b
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_sign_pre_hashed_shake128__inner(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

/**
 Sign (pre-hashed).
*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_sign_pre_hashed_shake128(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
);

/**
 The internal verification API.

 If no `domain_separation_context` is supplied, it is assumed that
 `message` already contains the domain separation.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.verify_internal
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_verify_internal_07(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  const Eurydice_arr_93 *signature_serialized
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.verify
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_verify_07(
  const Eurydice_arr_43 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_93 *signature_serialized
);

core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_verify__inner(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_93 *signature
);

/**
 Verify.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_verify(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_93 *signature
);

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_87.verify_pre_hashed
with types libcrux_ml_dsa_simd_avx2_vector_type_Vec256, libcrux_ml_dsa_samplex4_avx2_AVX2Sampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_simd256_Shake128x4, libcrux_ml_dsa_hash_functions_simd256_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_verify_pre_hashed_37(
  const Eurydice_arr_43 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_93 *signature_serialized
);

core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_verify_pre_hashed_shake128__inner(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_93 *signature
);

/**
 Verify (pre-hashed with SHAKE-128).
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_87_verify_pre_hashed_shake128(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_93 *signature
);

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_3_STEP ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_3_STEP_BY ((size_t)1U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_4_STEP ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_4_STEP_BY ((size_t)2U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_5_STEP ((size_t)32U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_5_STEP_BY ((size_t)4U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_6_STEP ((size_t)64U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_6_STEP_BY ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_7_STEP ((size_t)128U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_INVERT_NTT_AT_LAYER_7_STEP_BY ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_INVNTT_SIMD_UNIT_INVERT_NTT_AT_LAYER_0_SHUFFLE (216)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_BUTTERFLY_2_SHUFFLE (216)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP ((size_t)1U << 5U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_1 ((size_t)1U << 4U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_2 ((size_t)1U << 3U)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_BY (LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP / LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_BY_1 (LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_1 / LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT)

#define LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_BY_2 (LIBCRUX_ML_DSA_SIMD_AVX2_NTT_NTT_AT_LAYER_5_TO_3_STEP_2 / LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT)

bool
libcrux_ml_dsa_simd_avx2_rejection_sample_shuffle_table_is_bit_set(
  size_t number,
  uint8_t bit_position
);

Eurydice_arr_a30
libcrux_ml_dsa_simd_avx2_rejection_sample_shuffle_table_generate_shuffle_table(void);

typedef Eurydice_arr_60 libcrux_ml_dsa_simd_avx2_vector_type_AVX2RingElement;

/**
This function found in impl {impl core::clone::Clone for libcrux_ml_dsa::simd::avx2::vector_type::Vec256}
*/
__m256i libcrux_ml_dsa_simd_avx2_vector_type_clone_12(const __m256i *self);

#if defined(__cplusplus)
}
#endif

#define libcrux_mldsa_avx2_H_DEFINED
#endif /* libcrux_mldsa_avx2_H */
