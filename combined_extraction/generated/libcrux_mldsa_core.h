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


#ifndef libcrux_mldsa_core_H
#define libcrux_mldsa_core_H

#include "eurydice_glue.h"



#include "libcrux_mlkem_core.h"
#include "combined_core.h"

#define libcrux_ml_dsa_constants_Eta_Two 2
#define libcrux_ml_dsa_constants_Eta_Four 4

typedef uint8_t libcrux_ml_dsa_constants_Eta;

#define LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_TRAITS_SIMD_UNITS_IN_RING_ELEMENT ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T ((size_t)13U)

#define LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS_MINUS_ONE_BIT_LENGTH ((size_t)23U)

#define LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_UPPER_PART_OF_T (LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS_MINUS_ONE_BIT_LENGTH - LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T)

#define LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT ((size_t)256U)

#define LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN ((size_t)255U)

#define LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS (8380417)

#define LIBCRUX_ML_DSA_CONSTANTS_GAMMA2_V261_888 (261888)

#define LIBCRUX_ML_DSA_CONSTANTS_GAMMA2_V95_232 (95232)

typedef int32_t libcrux_ml_dsa_constants_Gamma2;

#define LIBCRUX_ML_DSA_CONSTANTS_KEY_GENERATION_RANDOMNESS_SIZE ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_MASK_SEED_SIZE ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_MESSAGE_REPRESENTATIVE_SIZE ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN ((size_t)814U)

#define LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE (LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T * LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE (LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_UPPER_PART_OF_T * LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_ERROR_VECTORS_SIZE ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_SIGNING_RANDOMNESS_SIZE ((size_t)32U)

static inline int32_t
libcrux_ml_dsa_constants_beta(
  size_t ones_in_verifier_challenge,
  libcrux_ml_dsa_constants_Eta eta
)
{
  size_t eta_val;
  switch (eta)
  {
    case libcrux_ml_dsa_constants_Eta_Two:
      {
        eta_val = (size_t)2U;
        break;
      }
    case libcrux_ml_dsa_constants_Eta_Four:
      {
        eta_val = (size_t)4U;
        break;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__, __LINE__);
        KRML_HOST_EXIT(253U);
      }
  }
  return (int32_t)(ones_in_verifier_challenge * eta_val);
}

static inline size_t
libcrux_ml_dsa_constants_commitment_ring_element_size(size_t bits_per_commitment_coefficient)
{
  return
    bits_per_commitment_coefficient * LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT /
      (size_t)8U;
}

static inline size_t
libcrux_ml_dsa_constants_commitment_vector_size(
  size_t bits_per_commitment_coefficient,
  size_t rows_in_a
)
{
  return
    libcrux_ml_dsa_constants_commitment_ring_element_size(bits_per_commitment_coefficient) *
      rows_in_a;
}

static inline size_t
libcrux_ml_dsa_constants_error_ring_element_size(size_t bits_per_error_coefficient)
{
  return
    bits_per_error_coefficient * LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U;
}

static inline size_t
libcrux_ml_dsa_constants_gamma1_ring_element_size(size_t bits_per_gamma1_coefficient)
{
  return
    bits_per_gamma1_coefficient * LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT /
      (size_t)8U;
}

static inline size_t
libcrux_ml_dsa_constants_signature_size(
  size_t rows_in_a,
  size_t columns_in_a,
  size_t max_ones_in_hint,
  size_t commitment_hash_size,
  size_t bits_per_gamma1_coefficient
)
{
  return
    commitment_hash_size +
      columns_in_a * libcrux_ml_dsa_constants_gamma1_ring_element_size(bits_per_gamma1_coefficient)
    + max_ones_in_hint
    + rows_in_a;
}

static inline size_t
libcrux_ml_dsa_constants_signing_key_size(
  size_t rows_in_a,
  size_t columns_in_a,
  size_t error_ring_element_size
)
{
  return
    LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE +
      LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH
    + (rows_in_a + columns_in_a) * error_ring_element_size
    + rows_in_a * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
}

static inline size_t libcrux_ml_dsa_constants_verification_key_size(size_t rows_in_a)
{
  return
    LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE +
      LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * rows_in_a *
        (LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS_MINUS_ONE_BIT_LENGTH -
          LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T)
      / (size_t)8U;
}

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_COMMITMENT_COEFFICIENT ((size_t)4U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_ERROR_COEFFICIENT ((size_t)4U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_GAMMA1_COEFFICIENT ((size_t)20U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A ((size_t)5U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE ((size_t)48U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA (libcrux_ml_dsa_constants_Eta_Four)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT ((size_t)19U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2 ((LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS - 1) / 32)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT ((size_t)55U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE ((size_t)49U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A ((size_t)6U)

/**
This function found in impl {core::clone::Clone for libcrux_ml_dsa::constants::Eta}
*/
static inline libcrux_ml_dsa_constants_Eta
libcrux_ml_dsa_constants_clone_54(const libcrux_ml_dsa_constants_Eta *self)
{
  return self[0U];
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_encoding_error_chunk_size(libcrux_ml_dsa_constants_Eta eta)
{
  switch (eta)
  {
    case libcrux_ml_dsa_constants_Eta_Two:
      {
        break;
      }
    case libcrux_ml_dsa_constants_Eta_Four:
      {
        return (size_t)4U;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__, __LINE__);
        KRML_HOST_EXIT(253U);
      }
  }
  return (size_t)3U;
}

#define libcrux_ml_dsa_types_VerificationError_MalformedHintError 0
#define libcrux_ml_dsa_types_VerificationError_SignerResponseExceedsBoundError 1
#define libcrux_ml_dsa_types_VerificationError_CommitmentHashesDontMatchError 2
#define libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError 3

typedef uint8_t libcrux_ml_dsa_types_VerificationError;

static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_signature_set_hint(
  Eurydice_dst_ref_mut_20 out_hint,
  size_t i,
  size_t j
)
{
  out_hint.ptr[i].data[j] = 1;
}

#define LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT ((size_t)13U)

#define LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW ((size_t)10U)

#define LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT ((size_t)10U)

#define LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE128_BLOCK_SIZE ((size_t)168U)

#define LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE128_FIVE_BLOCKS_SIZE (LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE128_BLOCK_SIZE * (size_t)5U)

#define LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE256_BLOCK_SIZE ((size_t)136U)

static KRML_MUSTINLINE Eurydice_arr_91
libcrux_ml_dsa_sample_add_error_domain_separator(
  Eurydice_borrow_slice_u8 slice,
  uint16_t domain_separator
)
{
  Eurydice_arr_91 out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d4(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
    slice,
    uint8_t);
  out.data[64U] = (uint8_t)(uint32_t)domain_separator;
  out.data[65U] = (uint8_t)((uint32_t)domain_separator >> 8U & 0xFFFFU);
  return out;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE (libcrux_ml_dsa_constants_error_ring_element_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_ERROR_COEFFICIENT))

#define LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS (8380417)

#define LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R (58728449ULL)

static inline uint8_t_x2
libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(size_t index, size_t width)
{
  return (uint8_t_x2{ (uint8_t)(index / width), (uint8_t)(index % width) });
}

static KRML_MUSTINLINE uint16_t libcrux_ml_dsa_sample_generate_domain_separator(uint8_t_x2 _)
{
  uint8_t row = _.fst;
  uint8_t column = _.snd;
  return (uint32_t)(uint16_t)(uint32_t)column | (uint32_t)(uint16_t)(uint32_t)row << 8U;
}

static KRML_MUSTINLINE Eurydice_arr_31
libcrux_ml_dsa_sample_add_domain_separator(Eurydice_borrow_slice_u8 slice, uint8_t_x2 indices)
{
  Eurydice_arr_31 out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d40(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
    slice,
    uint8_t);
  uint16_t domain_separator = libcrux_ml_dsa_sample_generate_domain_separator(indices);
  out.data[32U] = (uint8_t)(uint32_t)domain_separator;
  out.data[33U] = (uint8_t)((uint32_t)domain_separator >> 8U & 0xFFFFU);
  return out;
}

#define libcrux_ml_dsa_types_SigningError_RejectionSamplingError 0
#define libcrux_ml_dsa_types_SigningError_ContextTooLongError 1

typedef uint8_t libcrux_ml_dsa_types_SigningError;

typedef struct libcrux_ml_dsa_pre_hash_DomainSeparationContext_s
{
  Eurydice_borrow_slice_u8 context;
  core_option_Option_57 pre_hash_oid;
}
libcrux_ml_dsa_pre_hash_DomainSeparationContext;

#define libcrux_ml_dsa_pre_hash_DomainSeparationError_ContextTooLongError 0

typedef uint8_t libcrux_ml_dsa_pre_hash_DomainSeparationError;

/**
A monomorphic instance of core.result.Result
with types libcrux_ml_dsa_pre_hash_DomainSeparationContext, libcrux_ml_dsa_pre_hash_DomainSeparationError

*/
typedef struct core_result_Result_a8_s
{
  core_result_Result_57_tags tag;
  union U {
    libcrux_ml_dsa_pre_hash_DomainSeparationContext case_Ok;
    libcrux_ml_dsa_pre_hash_DomainSeparationError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_a8_s)
}
core_result_Result_a8;

/**
 `context` must be at most 255 bytes long.
*/
/**
This function found in impl {libcrux_ml_dsa::pre_hash::DomainSeparationContext<'a>}
*/
static inline core_result_Result_a8
libcrux_ml_dsa_pre_hash_new_88(
  Eurydice_borrow_slice_u8 context,
  core_option_Option_57 pre_hash_oid
)
{
  if (!(context.meta > LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN))
  {
    return
      core_result_Result_a8_s(core_result_Ok,
        &core_result_Result_a8_s::U::case_Ok,
        { context, pre_hash_oid });
  }
  return
    core_result_Result_a8_s(core_result_Err,
      &core_result_Result_a8_s::U::case_Err,
      libcrux_ml_dsa_pre_hash_DomainSeparationError_ContextTooLongError);
}

/**
 Returns the pre-hash OID, if any.
*/
/**
This function found in impl {libcrux_ml_dsa::pre_hash::DomainSeparationContext<'a>}
*/
static inline const
core_option_Option_57
*libcrux_ml_dsa_pre_hash_pre_hash_oid_88(
  const libcrux_ml_dsa_pre_hash_DomainSeparationContext *self
)
{
  return &self->pre_hash_oid;
}

/**
 Returns the context, guaranteed to be at most 255 bytes long.
*/
/**
This function found in impl {libcrux_ml_dsa::pre_hash::DomainSeparationContext<'a>}
*/
static inline Eurydice_borrow_slice_u8
libcrux_ml_dsa_pre_hash_context_88(const libcrux_ml_dsa_pre_hash_DomainSeparationContext *self)
{
  return self->context;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE (libcrux_ml_dsa_constants_commitment_ring_element_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_COMMITMENT_COEFFICIENT))

static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_inside_out_shuffle(
  Eurydice_borrow_slice_u8 randomness,
  size_t *out_index,
  uint64_t *signs,
  Eurydice_arr_6c *result
)
{
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta; i++)
  {
    size_t _cloop_j = i;
    const uint8_t *byte = &randomness.ptr[_cloop_j];
    if (!done)
    {
      size_t sample_at = (size_t)(uint32_t)byte[0U];
      if (sample_at <= out_index[0U])
      {
        result->data[out_index[0U]] = result->data[sample_at];
        out_index[0U]++;
        result->data[sample_at] = 1 - 2 * (int32_t)(signs[0U] & 1ULL);
        signs[0U] >>= 1U;
      }
      done = out_index[0U] == (size_t)256U;
    }
  }
  return done;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA (libcrux_ml_dsa_constants_beta(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA))

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE (libcrux_ml_dsa_constants_gamma1_ring_element_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_GAMMA1_COEFFICIENT))

#define LIBCRUX_ML_DSA_PRE_HASH_SHAKE128_OID ((Eurydice_arr_c9{ { 6U, 9U, 96U, 134U, 72U, 1U, 101U, 3U, 4U, 2U, 11U } }))

/**
This function found in impl {libcrux_ml_dsa::pre_hash::PreHash for libcrux_ml_dsa::pre_hash::SHAKE128_PH}
*/
static inline Eurydice_arr_c9 libcrux_ml_dsa_pre_hash_oid_30(void)
{
  return LIBCRUX_ML_DSA_PRE_HASH_SHAKE128_OID;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_VERIFICATION_KEY_SIZE (libcrux_ml_dsa_constants_verification_key_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A))

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNATURE_SIZE (libcrux_ml_dsa_constants_signature_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_GAMMA1_COEFFICIENT))

typedef Eurydice_arr_4d libcrux_ml_dsa_simd_portable_vector_type_Coefficients;

static KRML_MUSTINLINE Eurydice_arr_4d libcrux_ml_dsa_simd_portable_vector_type_zero(void)
{
  return (Eurydice_arr_4d{ { 0U } });
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline Eurydice_arr_4d libcrux_ml_dsa_simd_portable_zero_65(void)
{
  return libcrux_ml_dsa_simd_portable_vector_type_zero();
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_vector_type_from_coefficient_array(
  Eurydice_dst_ref_shared_83 array,
  Eurydice_arr_4d *out
)
{
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_fd(out),
    Eurydice_slice_subslice_shared_47(array,
      (core_ops_range_Range_87{ (size_t)0U, LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT })),
    int32_t);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_from_coefficient_array_65(
  Eurydice_dst_ref_shared_83 array,
  Eurydice_arr_4d *out
)
{
  libcrux_ml_dsa_simd_portable_vector_type_from_coefficient_array(array, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_vector_type_to_coefficient_array(
  const Eurydice_arr_4d *value,
  Eurydice_dst_ref_mut_83 out
)
{
  Eurydice_slice_copy(out, Eurydice_array_to_slice_shared_fd(value), int32_t);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_to_coefficient_array_65(
  const Eurydice_arr_4d *value,
  Eurydice_dst_ref_mut_83 out
)
{
  libcrux_ml_dsa_simd_portable_vector_type_to_coefficient_array(value, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_add(Eurydice_arr_4d *lhs, const Eurydice_arr_4d *rhs)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs->data[uu____0] += rhs->data[i0];
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_add_65(Eurydice_arr_4d *lhs, const Eurydice_arr_4d *rhs)
{
  libcrux_ml_dsa_simd_portable_arithmetic_add(lhs, rhs);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_subtract(
  Eurydice_arr_4d *lhs,
  const Eurydice_arr_4d *rhs
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs->data[uu____0] -= rhs->data[i0];
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_subtract_65(Eurydice_arr_4d *lhs, const Eurydice_arr_4d *rhs)
{
  libcrux_ml_dsa_simd_portable_arithmetic_subtract(lhs, rhs);
}

static KRML_MUSTINLINE bool
libcrux_ml_dsa_simd_portable_arithmetic_infinity_norm_exceeds(
  const Eurydice_arr_4d *simd_unit,
  int32_t bound
)
{
  bool result = false;
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    int32_t coefficient = simd_unit->data[i0];
    int32_t sign = coefficient >> 31U;
    int32_t normalized = coefficient - (sign & 2 * coefficient);
    bool uu____0;
    if (result)
    {
      uu____0 = true;
    }
    else
    {
      uu____0 = normalized >= bound;
    }
    result = uu____0;
  }
  return result;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline bool
libcrux_ml_dsa_simd_portable_infinity_norm_exceeds_65(
  const Eurydice_arr_4d *simd_unit,
  int32_t bound
)
{
  return libcrux_ml_dsa_simd_portable_arithmetic_infinity_norm_exceeds(simd_unit, bound);
}

static KRML_MUSTINLINE int32_t_x2
libcrux_ml_dsa_simd_portable_arithmetic_decompose_element(int32_t gamma2, int32_t r)
{
  int32_t r0 = r + (r >> 31U & LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  int32_t ceil_of_r_by_128 = (r0 + 127) >> 7U;
  int32_t r1;
  switch (gamma2)
  {
    case 95232:
      {
        int32_t result = (ceil_of_r_by_128 * 11275 + (int32_t)((uint32_t)1 << 23U)) >> 24U;
        int32_t result_0 = (result ^ (43 - result) >> 31U) & result;
        r1 = result_0;
        break;
      }
    case 261888:
      {
        int32_t result = (ceil_of_r_by_128 * 1025 + (int32_t)((uint32_t)1 << 21U)) >> 22U;
        int32_t result_0 = result & 15;
        r1 = result_0;
        break;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
        KRML_HOST_EXIT(255U);
      }
  }
  int32_t alpha = gamma2 * 2;
  int32_t r00 = r0 - r1 * alpha;
  r00 -=
    ((LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS - 1) / 2 - r00) >> 31U &
      LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS;
  return (int32_t_x2{ r00, r1 });
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_decompose(
  int32_t gamma2,
  const Eurydice_arr_4d *simd_unit,
  Eurydice_arr_4d *low,
  Eurydice_arr_4d *high
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    int32_t_x2
    uu____0 =
      libcrux_ml_dsa_simd_portable_arithmetic_decompose_element(gamma2,
        simd_unit->data[i0]);
    int32_t uu____1 = uu____0.snd;
    low->data[i0] = uu____0.fst;
    high->data[i0] = uu____1;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_decompose_65(
  int32_t gamma2,
  const Eurydice_arr_4d *simd_unit,
  Eurydice_arr_4d *low,
  Eurydice_arr_4d *high
)
{
  libcrux_ml_dsa_simd_portable_arithmetic_decompose(gamma2, simd_unit, low, high);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_compute_one_hint(
  int32_t low,
  int32_t high,
  int32_t gamma2
)
{
  int32_t uu____0;
  if (low > gamma2)
  {
    uu____0 = 1;
  }
  else if (low < -gamma2)
  {
    uu____0 = 1;
  }
  else if (low == -gamma2)
  {
    if (high != 0)
    {
      uu____0 = 1;
    }
    else
    {
      uu____0 = 0;
    }
  }
  else
  {
    uu____0 = 0;
  }
  return uu____0;
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_arithmetic_compute_hint(
  const Eurydice_arr_4d *low,
  const Eurydice_arr_4d *high,
  int32_t gamma2,
  Eurydice_arr_4d *hint
)
{
  size_t one_hints_count = (size_t)0U;
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    hint->data[i0] =
      libcrux_ml_dsa_simd_portable_arithmetic_compute_one_hint(low->data[i0],
        high->data[i0],
        gamma2);
    one_hints_count += (size_t)hint->data[i0];
  }
  return one_hints_count;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_compute_hint_65(
  const Eurydice_arr_4d *low,
  const Eurydice_arr_4d *high,
  int32_t gamma2,
  Eurydice_arr_4d *hint
)
{
  return libcrux_ml_dsa_simd_portable_arithmetic_compute_hint(low, high, gamma2, hint);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_use_one_hint(int32_t gamma2, int32_t r, int32_t hint)
{
  int32_t_x2 uu____0 = libcrux_ml_dsa_simd_portable_arithmetic_decompose_element(gamma2, r);
  int32_t r0 = uu____0.fst;
  int32_t r1 = uu____0.snd;
  int32_t uu____1;
  if (!(hint == 0))
  {
    switch (gamma2)
    {
      case 95232:
        {
          if (r0 > 0)
          {
            if (r1 == 43)
            {
              uu____1 = 0;
            }
            else
            {
              uu____1 = r1 + hint;
            }
          }
          else if (r1 == 0)
          {
            uu____1 = 43;
          }
          else
          {
            uu____1 = r1 - hint;
          }
          break;
        }
      case 261888:
        {
          if (r0 > 0)
          {
            uu____1 = (r1 + hint) & 15;
          }
          else
          {
            uu____1 = (r1 - hint) & 15;
          }
          break;
        }
      default:
        {
          KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
          KRML_HOST_EXIT(255U);
        }
    }
    return uu____1;
  }
  return r1;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_use_hint(
  int32_t gamma2,
  const Eurydice_arr_4d *simd_unit,
  Eurydice_arr_4d *hint
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    int32_t
    uu____0 =
      libcrux_ml_dsa_simd_portable_arithmetic_use_one_hint(gamma2,
        simd_unit->data[i0],
        hint->data[i0]);
    hint->data[i0] = uu____0;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_use_hint_65(
  int32_t gamma2,
  const Eurydice_arr_4d *simd_unit,
  Eurydice_arr_4d *hint
)
{
  libcrux_ml_dsa_simd_portable_arithmetic_use_hint(gamma2, simd_unit, hint);
}

static KRML_MUSTINLINE uint64_t
libcrux_ml_dsa_simd_portable_arithmetic_get_n_least_significant_bits(uint8_t n, uint64_t value)
{
  return value & ((1ULL << (uint32_t)n) - 1ULL);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT (32U)

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element(int64_t value)
{
  uint64_t
  t =
    libcrux_ml_dsa_simd_portable_arithmetic_get_n_least_significant_bits(LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT,
      (uint64_t)value)
    * LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
  int32_t
  k =
    (int32_t)libcrux_ml_dsa_simd_portable_arithmetic_get_n_least_significant_bits(LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT,
      t);
  int64_t k_times_modulus = (int64_t)k * (int64_t)LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS;
  int32_t
  c =
    (int32_t)(k_times_modulus >> (uint32_t)LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT);
  int32_t
  value_high =
    (int32_t)(value >> (uint32_t)LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT);
  return value_high - c;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply(
  Eurydice_arr_4d *lhs,
  const Eurydice_arr_4d *rhs
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    lhs->data[i0] =
      libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element((int64_t)lhs->data[i0] *
          (int64_t)rhs->data[i0]);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_montgomery_multiply_65(
  Eurydice_arr_4d *lhs,
  const Eurydice_arr_4d *rhs
)
{
  libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply(lhs, rhs);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_barrett_reduce_element(int32_t fe)
{
  int32_t quotient = (fe + (int32_t)((uint32_t)1 << 22U)) >> 23U;
  return fe - quotient * LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS;
}

static inline void
libcrux_ml_dsa_simd_portable_arithmetic_barrett_reduce_simd_unit(Eurydice_arr_4d *simd_unit)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    simd_unit->data[i0] =
      libcrux_ml_dsa_simd_portable_arithmetic_barrett_reduce_element(simd_unit->data[i0]);
  }
}

static KRML_MUSTINLINE int32_t_x2
libcrux_ml_dsa_simd_portable_arithmetic_power2round_element(int32_t t)
{
  int32_t t2 = t + (t >> 31U & LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  int32_t
  t1 =
    (t2 - 1 +
      (int32_t)((uint32_t)1 <<
        (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T - (size_t)1U)))
    >> (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T;
  int32_t
  t0 = t2 - (int32_t)((uint32_t)t1 << (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T);
  return (int32_t_x2{ t0, t1 });
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_power2round(Eurydice_arr_4d *t0, Eurydice_arr_4d *t1)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    int32_t_x2 uu____0 = libcrux_ml_dsa_simd_portable_arithmetic_power2round_element(t0->data[i0]);
    int32_t uu____1 = uu____0.snd;
    t0->data[i0] = uu____0.fst;
    t1->data[i0] = uu____1;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_power2round_65(Eurydice_arr_4d *t0, Eurydice_arr_4d *t1)
{
  libcrux_ml_dsa_simd_portable_arithmetic_power2round(t0, t1);
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_field_modulus(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
)
{
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)3U; i++)
  {
    size_t i0 = i;
    int32_t b0 = (int32_t)(uint32_t)randomness.ptr[i0 * (size_t)3U];
    int32_t b1 = (int32_t)(uint32_t)randomness.ptr[i0 * (size_t)3U + (size_t)1U];
    int32_t b2 = (int32_t)(uint32_t)randomness.ptr[i0 * (size_t)3U + (size_t)2U];
    int32_t
    coefficient = (((int32_t)((uint32_t)b2 << 16U) | (int32_t)((uint32_t)b1 << 8U)) | b0) & 8388607;
    if (coefficient < LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS)
    {
      out.ptr[sampled] = coefficient;
      sampled++;
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_rejection_sample_less_than_field_modulus_65(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
)
{
  return
    libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_field_modulus(randomness,
      out);
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_2(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
)
{
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < randomness.meta; i++)
  {
    size_t i0 = i;
    uint8_t byte = randomness.ptr[i0];
    uint8_t try_0 = (uint32_t)byte & 15U;
    uint8_t try_1 = (uint32_t)byte >> 4U;
    bool try_0_comp = try_0 < 15U;
    bool try_1_comp = try_1 < 15U;
    if (try_0_comp)
    {
      int32_t try_00 = (int32_t)(uint32_t)try_0;
      int32_t try_0_mod_5 = try_00 - (try_00 * 26 >> 7U) * 5;
      out.ptr[sampled] = 2 - try_0_mod_5;
      sampled++;
    }
    if (try_1_comp)
    {
      int32_t try_10 = (int32_t)(uint32_t)try_1;
      int32_t try_1_mod_5 = try_10 - (try_10 * 26 >> 7U) * 5;
      out.ptr[sampled] = 2 - try_1_mod_5;
      sampled++;
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_2_65(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
)
{
  return
    libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_2(randomness,
      out);
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_4(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
)
{
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < randomness.meta; i++)
  {
    size_t i0 = i;
    uint8_t byte = randomness.ptr[i0];
    uint8_t try_0 = (uint32_t)byte & 15U;
    uint8_t try_1 = (uint32_t)byte >> 4U;
    bool try_0_comp = try_0 < 9U;
    bool try_1_comp = try_1 < 9U;
    if (try_0_comp)
    {
      out.ptr[sampled] = 4 - (int32_t)(uint32_t)try_0;
      sampled++;
    }
    if (try_1_comp)
    {
      out.ptr[sampled] = 4 - (int32_t)(uint32_t)try_1;
      sampled++;
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_4_65(
  Eurydice_borrow_slice_u8 randomness,
  Eurydice_dst_ref_mut_83 out
)
{
  return
    libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_4(randomness,
      out);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 ((int32_t)((uint32_t)1 << 19U))

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U / (size_t)2U; i++)
  {
    size_t i0 = i;
    Eurydice_dst_ref_shared_83
    coefficients =
      Eurydice_array_to_subslice_shared_44(simd_unit,
        (core_ops_range_Range_87{ i0 * (size_t)2U, i0 * (size_t)2U + (size_t)2U }));
    int32_t
    coefficient0 =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        coefficients.ptr[0U];
    int32_t
    coefficient1 =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        coefficients.ptr[1U];
    serialized.ptr[(size_t)5U * i0] = (uint8_t)coefficient0;
    serialized.ptr[(size_t)5U * i0 + (size_t)1U] = (uint8_t)(coefficient0 >> 8U);
    serialized.ptr[(size_t)5U * i0 + (size_t)2U] = (uint8_t)(coefficient0 >> 16U);
    size_t uu____0 = (size_t)5U * i0 + (size_t)2U;
    serialized.ptr[uu____0] =
      (uint32_t)serialized.ptr[uu____0] | (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient1 << 4U);
    serialized.ptr[(size_t)5U * i0 + (size_t)3U] = (uint8_t)(coefficient1 >> 4U);
    serialized.ptr[(size_t)5U * i0 + (size_t)4U] = (uint8_t)(coefficient1 >> 12U);
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 ((int32_t)((uint32_t)1 << 17U))

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U / (size_t)4U; i++)
  {
    size_t i0 = i;
    Eurydice_dst_ref_shared_83
    coefficients =
      Eurydice_array_to_subslice_shared_44(simd_unit,
        (core_ops_range_Range_87{ i0 * (size_t)4U, i0 * (size_t)4U + (size_t)4U }));
    int32_t
    coefficient0 =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficients.ptr[0U];
    int32_t
    coefficient1 =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficients.ptr[1U];
    int32_t
    coefficient2 =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficients.ptr[2U];
    int32_t
    coefficient3 =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficients.ptr[3U];
    serialized.ptr[(size_t)9U * i0] = (uint8_t)coefficient0;
    serialized.ptr[(size_t)9U * i0 + (size_t)1U] = (uint8_t)(coefficient0 >> 8U);
    serialized.ptr[(size_t)9U * i0 + (size_t)2U] = (uint8_t)(coefficient0 >> 16U);
    size_t uu____0 = (size_t)9U * i0 + (size_t)2U;
    serialized.ptr[uu____0] =
      (uint32_t)serialized.ptr[uu____0] | (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient1 << 2U);
    serialized.ptr[(size_t)9U * i0 + (size_t)3U] = (uint8_t)(coefficient1 >> 6U);
    serialized.ptr[(size_t)9U * i0 + (size_t)4U] = (uint8_t)(coefficient1 >> 14U);
    size_t uu____1 = (size_t)9U * i0 + (size_t)4U;
    serialized.ptr[uu____1] =
      (uint32_t)serialized.ptr[uu____1] | (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient2 << 4U);
    serialized.ptr[(size_t)9U * i0 + (size_t)5U] = (uint8_t)(coefficient2 >> 4U);
    serialized.ptr[(size_t)9U * i0 + (size_t)6U] = (uint8_t)(coefficient2 >> 12U);
    size_t uu____2 = (size_t)9U * i0 + (size_t)6U;
    serialized.ptr[uu____2] =
      (uint32_t)serialized.ptr[uu____2] | (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient3 << 6U);
    serialized.ptr[(size_t)9U * i0 + (size_t)7U] = (uint8_t)(coefficient3 >> 2U);
    serialized.ptr[(size_t)9U * i0 + (size_t)8U] = (uint8_t)(coefficient3 >> 10U);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized,
  size_t gamma1_exponent
)
{
  switch (gamma1_exponent)
  {
    case 17U:
      {
        break;
      }
    case 19U:
      {
        libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(simd_unit,
          serialized);
        return;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
        KRML_HOST_EXIT(255U);
      }
  }
  libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(simd_unit,
    serialized);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_gamma1_serialize_65(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized,
  size_t gamma1_exponent
)
{
  libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize(simd_unit, serialized, gamma1_exponent);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 ((int32_t)((uint32_t)1 << 19U))

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1_TIMES_2_BITMASK ((int32_t)((uint32_t)LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 << 1U) - 1)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *simd_unit
)
{
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)5U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)5U, i0 * (size_t)5U + (size_t)5U }));
    int32_t coefficient0 = (int32_t)(uint32_t)bytes.ptr[0U];
    coefficient0 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[1U] << 8U);
    coefficient0 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[2U] << 16U);
    coefficient0 &=
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient1 = (int32_t)(uint32_t)bytes.ptr[2U] >> 4U;
    coefficient1 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[3U] << 4U);
    coefficient1 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[4U] << 12U);
    simd_unit->data[(size_t)2U * i0] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        coefficient0;
    simd_unit->data[(size_t)2U * i0 + (size_t)1U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        coefficient1;
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 ((int32_t)((uint32_t)1 << 17U))

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK ((int32_t)((uint32_t)LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 << 1U) - 1)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *simd_unit
)
{
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)9U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)9U, i0 * (size_t)9U + (size_t)9U }));
    int32_t coefficient0 = (int32_t)(uint32_t)bytes.ptr[0U];
    coefficient0 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[1U] << 8U);
    coefficient0 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[2U] << 16U);
    coefficient0 &=
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient1 = (int32_t)(uint32_t)bytes.ptr[2U] >> 2U;
    coefficient1 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[3U] << 6U);
    coefficient1 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[4U] << 14U);
    coefficient1 &=
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient2 = (int32_t)(uint32_t)bytes.ptr[4U] >> 4U;
    coefficient2 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[5U] << 4U);
    coefficient2 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[6U] << 12U);
    coefficient2 &=
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient3 = (int32_t)(uint32_t)bytes.ptr[6U] >> 6U;
    coefficient3 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[7U] << 2U);
    coefficient3 |= (int32_t)((uint32_t)(int32_t)(uint32_t)bytes.ptr[8U] << 10U);
    coefficient3 &=
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    simd_unit->data[(size_t)4U * i0] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficient0;
    simd_unit->data[(size_t)4U * i0 + (size_t)1U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficient1;
    simd_unit->data[(size_t)4U * i0 + (size_t)2U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficient2;
    simd_unit->data[(size_t)4U * i0 + (size_t)3U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        coefficient3;
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *out,
  size_t gamma1_exponent
)
{
  switch (gamma1_exponent)
  {
    case 17U:
      {
        break;
      }
    case 19U:
      {
        libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19(serialized,
          out);
        return;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
        KRML_HOST_EXIT(255U);
      }
  }
  libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17(serialized,
    out);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_gamma1_deserialize_65(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *out,
  size_t gamma1_exponent
)
{
  libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize(serialized, out, gamma1_exponent);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_4(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  uint8_t coefficient0 = (uint8_t)simd_unit->data[0U];
  uint8_t coefficient1 = (uint8_t)simd_unit->data[1U];
  uint8_t coefficient2 = (uint8_t)simd_unit->data[2U];
  uint8_t coefficient3 = (uint8_t)simd_unit->data[3U];
  uint8_t coefficient4 = (uint8_t)simd_unit->data[4U];
  uint8_t coefficient5 = (uint8_t)simd_unit->data[5U];
  uint8_t coefficient6 = (uint8_t)simd_unit->data[6U];
  uint8_t coefficient7 = (uint8_t)simd_unit->data[7U];
  uint8_t byte0 = (uint32_t)coefficient1 << 4U | (uint32_t)coefficient0;
  uint8_t byte1 = (uint32_t)coefficient3 << 4U | (uint32_t)coefficient2;
  uint8_t byte2 = (uint32_t)coefficient5 << 4U | (uint32_t)coefficient4;
  uint8_t byte3 = (uint32_t)coefficient7 << 4U | (uint32_t)coefficient6;
  serialized.ptr[0U] = byte0;
  serialized.ptr[1U] = byte1;
  serialized.ptr[2U] = byte2;
  serialized.ptr[3U] = byte3;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_6(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  uint8_t coefficient0 = (uint8_t)simd_unit->data[0U];
  uint8_t coefficient1 = (uint8_t)simd_unit->data[1U];
  uint8_t coefficient2 = (uint8_t)simd_unit->data[2U];
  uint8_t coefficient3 = (uint8_t)simd_unit->data[3U];
  uint8_t coefficient4 = (uint8_t)simd_unit->data[4U];
  uint8_t coefficient5 = (uint8_t)simd_unit->data[5U];
  uint8_t coefficient6 = (uint8_t)simd_unit->data[6U];
  uint8_t coefficient7 = (uint8_t)simd_unit->data[7U];
  uint8_t byte0 = (uint32_t)coefficient1 << 6U | (uint32_t)coefficient0;
  uint8_t byte1 = (uint32_t)coefficient2 << 4U | (uint32_t)coefficient1 >> 2U;
  uint8_t byte2 = (uint32_t)coefficient3 << 2U | (uint32_t)coefficient2 >> 4U;
  uint8_t byte3 = (uint32_t)coefficient5 << 6U | (uint32_t)coefficient4;
  uint8_t byte4 = (uint32_t)coefficient6 << 4U | (uint32_t)coefficient5 >> 2U;
  uint8_t byte5 = (uint32_t)coefficient7 << 2U | (uint32_t)coefficient6 >> 4U;
  serialized.ptr[0U] = byte0;
  serialized.ptr[1U] = byte1;
  serialized.ptr[2U] = byte2;
  serialized.ptr[3U] = byte3;
  serialized.ptr[4U] = byte4;
  serialized.ptr[5U] = byte5;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_commitment_serialize(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  switch ((uint32_t)(uint8_t)serialized.meta)
  {
    case 4U:
      {
        libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_4(simd_unit, serialized);
        break;
      }
    case 6U:
      {
        libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_6(simd_unit, serialized);
        break;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
        KRML_HOST_EXIT(255U);
      }
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_commitment_serialize_65(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  libcrux_ml_dsa_simd_portable_encoding_commitment_serialize(simd_unit, serialized);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_4_ETA (4)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_4(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U / (size_t)2U; i++)
  {
    size_t i0 = i;
    Eurydice_dst_ref_shared_83
    coefficients =
      Eurydice_array_to_subslice_shared_44(simd_unit,
        (core_ops_range_Range_87{ i0 * (size_t)2U, i0 * (size_t)2U + (size_t)2U }));
    uint8_t
    coefficient0 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_4_ETA -
        coefficients.ptr[0U]);
    uint8_t
    coefficient1 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_4_ETA -
        coefficients.ptr[1U]);
    serialized.ptr[i0] = (uint32_t)coefficient1 << 4U | (uint32_t)coefficient0;
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA (2)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_2(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  uint8_t
  coefficient0 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[0U]);
  uint8_t
  coefficient1 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[1U]);
  uint8_t
  coefficient2 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[2U]);
  uint8_t
  coefficient3 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[3U]);
  uint8_t
  coefficient4 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[4U]);
  uint8_t
  coefficient5 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[5U]);
  uint8_t
  coefficient6 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[6U]);
  uint8_t
  coefficient7 =
    (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
      simd_unit->data[7U]);
  serialized.ptr[0U] =
    ((uint32_t)coefficient2 << 6U | (uint32_t)coefficient1 << 3U) | (uint32_t)coefficient0;
  serialized.ptr[1U] =
    (((uint32_t)coefficient5 << 7U | (uint32_t)coefficient4 << 4U) | (uint32_t)coefficient3 << 1U)
    | (uint32_t)coefficient2 >> 2U;
  serialized.ptr[2U] =
    ((uint32_t)coefficient7 << 5U | (uint32_t)coefficient6 << 2U) | (uint32_t)coefficient5 >> 1U;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_serialize(
  libcrux_ml_dsa_constants_Eta eta,
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  switch (eta)
  {
    case libcrux_ml_dsa_constants_Eta_Two:
      {
        break;
      }
    case libcrux_ml_dsa_constants_Eta_Four:
      {
        libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_4(simd_unit, serialized);
        return;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__, __LINE__);
        KRML_HOST_EXIT(253U);
      }
  }
  libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_2(simd_unit, serialized);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_error_serialize_65(
  libcrux_ml_dsa_constants_Eta eta,
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  libcrux_ml_dsa_simd_portable_encoding_error_serialize(eta, simd_unit, serialized);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_4_ETA (4)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_4(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *simd_units
)
{
  for (size_t i = (size_t)0U; i < serialized.meta; i++)
  {
    size_t i0 = i;
    const uint8_t *byte = &serialized.ptr[i0];
    uint8_t uu____0 = core_ops_bit__core__ops__bit__BitAnd_u8__u8__for__0__u8___bitand(byte, 15U);
    simd_units->data[(size_t)2U * i0] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_4_ETA -
        (int32_t)(uint32_t)uu____0;
    uint8_t uu____1 = core_ops_bit__core__ops__bit__Shr_i32__u8__for__0__u8___shr(byte, 4);
    simd_units->data[(size_t)2U * i0 + (size_t)1U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_4_ETA -
        (int32_t)(uint32_t)uu____1;
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA (2)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_2(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *simd_unit
)
{
  int32_t byte0 = (int32_t)(uint32_t)serialized.ptr[0U];
  int32_t byte1 = (int32_t)(uint32_t)serialized.ptr[1U];
  int32_t byte2 = (int32_t)(uint32_t)serialized.ptr[2U];
  simd_unit->data[0U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA - (byte0 & 7);
  simd_unit->data[1U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA - (byte0 >> 3U & 7);
  simd_unit->data[2U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      ((byte0 >> 6U | (int32_t)((uint32_t)byte1 << 2U)) & 7);
  simd_unit->data[3U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA - (byte1 >> 1U & 7);
  simd_unit->data[4U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA - (byte1 >> 4U & 7);
  simd_unit->data[5U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      ((byte1 >> 7U | (int32_t)((uint32_t)byte2 << 1U)) & 7);
  simd_unit->data[6U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA - (byte2 >> 2U & 7);
  simd_unit->data[7U] =
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA - (byte2 >> 5U & 7);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_deserialize(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *out
)
{
  switch (eta)
  {
    case libcrux_ml_dsa_constants_Eta_Two:
      {
        break;
      }
    case libcrux_ml_dsa_constants_Eta_Four:
      {
        libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_4(serialized, out);
        return;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__, __LINE__);
        KRML_HOST_EXIT(253U);
      }
  }
  libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_2(serialized, out);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_error_deserialize_65(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *out
)
{
  libcrux_ml_dsa_simd_portable_encoding_error_deserialize(eta, serialized, out);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(int32_t t0)
{
  return
    (int32_t)((uint32_t)1 <<
      (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T - (size_t)1U))
    - t0;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_t0_serialize(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  int32_t
  coefficient0 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[0U]);
  int32_t
  coefficient1 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[1U]);
  int32_t
  coefficient2 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[2U]);
  int32_t
  coefficient3 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[3U]);
  int32_t
  coefficient4 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[4U]);
  int32_t
  coefficient5 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[5U]);
  int32_t
  coefficient6 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[6U]);
  int32_t
  coefficient7 = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(simd_unit->data[7U]);
  serialized.ptr[0U] = (uint8_t)coefficient0;
  serialized.ptr[1U] =
    (uint32_t)(uint8_t)(coefficient0 >> 8U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient1 << 5U);
  serialized.ptr[2U] = (uint8_t)(coefficient1 >> 3U);
  serialized.ptr[3U] =
    (uint32_t)(uint8_t)(coefficient1 >> 11U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient2 << 2U);
  serialized.ptr[4U] =
    (uint32_t)(uint8_t)(coefficient2 >> 6U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient3 << 7U);
  serialized.ptr[5U] = (uint8_t)(coefficient3 >> 1U);
  serialized.ptr[6U] =
    (uint32_t)(uint8_t)(coefficient3 >> 9U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient4 << 4U);
  serialized.ptr[7U] = (uint8_t)(coefficient4 >> 4U);
  serialized.ptr[8U] =
    (uint32_t)(uint8_t)(coefficient4 >> 12U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient5 << 1U);
  serialized.ptr[9U] =
    (uint32_t)(uint8_t)(coefficient5 >> 7U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient6 << 6U);
  serialized.ptr[10U] = (uint8_t)(coefficient6 >> 2U);
  serialized.ptr[11U] =
    (uint32_t)(uint8_t)(coefficient6 >> 10U) |
      (uint32_t)(uint8_t)(int32_t)((uint32_t)coefficient7 << 3U);
  serialized.ptr[12U] = (uint8_t)(coefficient7 >> 5U);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_t0_serialize_65(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_ml_dsa_simd_portable_encoding_t0_serialize(simd_unit, out);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK ((int32_t)((uint32_t)1 << (uint32_t)(int32_t)LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T) - 1)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_t0_deserialize(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *simd_unit
)
{
  int32_t byte0 = (int32_t)(uint32_t)serialized.ptr[0U];
  int32_t byte1 = (int32_t)(uint32_t)serialized.ptr[1U];
  int32_t byte2 = (int32_t)(uint32_t)serialized.ptr[2U];
  int32_t byte3 = (int32_t)(uint32_t)serialized.ptr[3U];
  int32_t byte4 = (int32_t)(uint32_t)serialized.ptr[4U];
  int32_t byte5 = (int32_t)(uint32_t)serialized.ptr[5U];
  int32_t byte6 = (int32_t)(uint32_t)serialized.ptr[6U];
  int32_t byte7 = (int32_t)(uint32_t)serialized.ptr[7U];
  int32_t byte8 = (int32_t)(uint32_t)serialized.ptr[8U];
  int32_t byte9 = (int32_t)(uint32_t)serialized.ptr[9U];
  int32_t byte10 = (int32_t)(uint32_t)serialized.ptr[10U];
  int32_t byte11 = (int32_t)(uint32_t)serialized.ptr[11U];
  int32_t byte12 = (int32_t)(uint32_t)serialized.ptr[12U];
  int32_t coefficient0 = byte0;
  coefficient0 |= (int32_t)((uint32_t)byte1 << 8U);
  coefficient0 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient1 = byte1 >> 5U;
  coefficient1 |= (int32_t)((uint32_t)byte2 << 3U);
  coefficient1 |= (int32_t)((uint32_t)byte3 << 11U);
  coefficient1 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient2 = byte3 >> 2U;
  coefficient2 |= (int32_t)((uint32_t)byte4 << 6U);
  coefficient2 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient3 = byte4 >> 7U;
  coefficient3 |= (int32_t)((uint32_t)byte5 << 1U);
  coefficient3 |= (int32_t)((uint32_t)byte6 << 9U);
  coefficient3 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient4 = byte6 >> 4U;
  coefficient4 |= (int32_t)((uint32_t)byte7 << 4U);
  coefficient4 |= (int32_t)((uint32_t)byte8 << 12U);
  coefficient4 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient5 = byte8 >> 1U;
  coefficient5 |= (int32_t)((uint32_t)byte9 << 7U);
  coefficient5 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient6 = byte9 >> 6U;
  coefficient6 |= (int32_t)((uint32_t)byte10 << 2U);
  coefficient6 |= (int32_t)((uint32_t)byte11 << 10U);
  coefficient6 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient7 = byte11 >> 3U;
  coefficient7 |= (int32_t)((uint32_t)byte12 << 5U);
  coefficient7 &=
    LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  simd_unit->data[0U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient0);
  simd_unit->data[1U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient1);
  simd_unit->data[2U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient2);
  simd_unit->data[3U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient3);
  simd_unit->data[4U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient4);
  simd_unit->data[5U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient5);
  simd_unit->data[6U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient6);
  simd_unit->data[7U] = libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient7);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_t0_deserialize_65(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *out
)
{
  libcrux_ml_dsa_simd_portable_encoding_t0_deserialize(serialized, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_t1_serialize(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U / (size_t)4U; i++)
  {
    size_t i0 = i;
    Eurydice_dst_ref_shared_83
    coefficients =
      Eurydice_array_to_subslice_shared_44(simd_unit,
        (core_ops_range_Range_87{ i0 * (size_t)4U, i0 * (size_t)4U + (size_t)4U }));
    serialized.ptr[(size_t)5U * i0] = (uint8_t)(coefficients.ptr[0U] & 255);
    serialized.ptr[(size_t)5U * i0 + (size_t)1U] =
      (uint32_t)(uint8_t)(coefficients.ptr[1U] & 63) << 2U |
        (uint32_t)(uint8_t)(coefficients.ptr[0U] >> 8U & 3);
    serialized.ptr[(size_t)5U * i0 + (size_t)2U] =
      (uint32_t)(uint8_t)(coefficients.ptr[2U] & 15) << 4U |
        (uint32_t)(uint8_t)(coefficients.ptr[1U] >> 6U & 15);
    serialized.ptr[(size_t)5U * i0 + (size_t)3U] =
      (uint32_t)(uint8_t)(coefficients.ptr[3U] & 3) << 6U |
        (uint32_t)(uint8_t)(coefficients.ptr[2U] >> 4U & 63);
    serialized.ptr[(size_t)5U * i0 + (size_t)4U] = (uint8_t)(coefficients.ptr[3U] >> 2U & 255);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_t1_serialize_65(
  const Eurydice_arr_4d *simd_unit,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_ml_dsa_simd_portable_encoding_t1_serialize(simd_unit, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_t1_deserialize(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *simd_unit
)
{
  int32_t
  mask = (int32_t)((uint32_t)1 << (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_UPPER_PART_OF_T) - 1;
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)5U; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * (size_t)5U, i0 * (size_t)5U + (size_t)5U }));
    int32_t byte0 = (int32_t)(uint32_t)bytes.ptr[0U];
    int32_t byte1 = (int32_t)(uint32_t)bytes.ptr[1U];
    int32_t byte2 = (int32_t)(uint32_t)bytes.ptr[2U];
    int32_t byte3 = (int32_t)(uint32_t)bytes.ptr[3U];
    int32_t byte4 = (int32_t)(uint32_t)bytes.ptr[4U];
    simd_unit->data[(size_t)4U * i0] = (byte0 | (int32_t)((uint32_t)byte1 << 8U)) & mask;
    simd_unit->data[(size_t)4U * i0 + (size_t)1U] =
      (byte1 >> 2U | (int32_t)((uint32_t)byte2 << 6U)) & mask;
    simd_unit->data[(size_t)4U * i0 + (size_t)2U] =
      (byte2 >> 4U | (int32_t)((uint32_t)byte3 << 4U)) & mask;
    simd_unit->data[(size_t)4U * i0 + (size_t)3U] =
      (byte3 >> 6U | (int32_t)((uint32_t)byte4 << 2U)) & mask;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_t1_deserialize_65(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_4d *out
)
{
  libcrux_ml_dsa_simd_portable_encoding_t1_deserialize(serialized, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
  Eurydice_arr_4d *simd_unit,
  int32_t c
)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    simd_unit->data[i0] =
      libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element((int64_t)simd_unit->data[i0]
        * (int64_t)c);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_a3_s { Eurydice_arr_4d data[32U]; } Eurydice_arr_a3;

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(
  Eurydice_arr_a3 *re,
  size_t index,
  size_t step_by,
  int32_t zeta
)
{
  Eurydice_arr_4d tmp = re->data[index + step_by];
  libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&tmp, zeta);
  re->data[index + step_by] = re->data[index];
  libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[index + step_by], &tmp);
  libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[index], &tmp);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 16
- ZETA= 25847
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_30(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)16U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)16U, 25847);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_7(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_30(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 8
- ZETA= -2608894
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_300(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)8U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)8U, -2608894);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 8
- ZETA= -518909
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_42(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)8U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)8U, -518909);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_6(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_300(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_42(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 4
- ZETA= 237124
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_301(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)4U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U, 237124);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 4
- ZETA= -777960
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_82(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)4U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U, -777960);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 4
- ZETA= -876248
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_420(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)4U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U, -876248);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 4
- ZETA= 466468
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_fe(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)4U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U, 466468);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_5(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_301(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_82(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_420(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_fe(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 2
- ZETA= 1826347
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_302(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, 1826347);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 2
- ZETA= 2353451
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_43(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, 2353451);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 2
- ZETA= -359251
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_820(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, -359251);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 2
- ZETA= -2091905
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_ea(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, -2091905);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 2
- ZETA= 3119733
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_421(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, 3119733);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 2
- ZETA= -2884855
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_61(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, -2884855);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 2
- ZETA= 3111497
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_fe0(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, 3111497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 2
- ZETA= 2680103
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_38(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)2U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U, 2680103);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_4(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_302(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_43(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_820(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_ea(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_421(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_61(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_fe0(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_38(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 1
- ZETA= 2725464
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_303(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 2725464);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 2
- STEP_BY= 1
- ZETA= 1024112
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_25(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)2U; i < (size_t)2U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 1024112);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 1
- ZETA= -1079900
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_430(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -1079900);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 6
- STEP_BY= 1
- ZETA= 3585928
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_f4(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)6U; i < (size_t)6U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 3585928);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 1
- ZETA= -549488
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_821(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -549488);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 10
- STEP_BY= 1
- ZETA= -1119584
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_1d(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)10U; i < (size_t)10U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -1119584);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 1
- ZETA= 2619752
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_ea0(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 2619752);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 14
- STEP_BY= 1
- ZETA= -2108549
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d8(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)14U; i < (size_t)14U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -2108549);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 1
- ZETA= -2118186
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_422(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -2118186);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 18
- STEP_BY= 1
- ZETA= -3859737
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_60(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)18U; i < (size_t)18U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -3859737);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 1
- ZETA= -1399561
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_610(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -1399561);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 22
- STEP_BY= 1
- ZETA= -3277672
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_29(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)22U; i < (size_t)22U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -3277672);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 1
- ZETA= 1757237
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_fe1(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 1757237);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 26
- STEP_BY= 1
- ZETA= -19422
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_9d(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)26U; i < (size_t)26U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, -19422);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 1
- ZETA= 4010497
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_380(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 4010497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 30
- STEP_BY= 1
- ZETA= 280005
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_5f(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)30U; i < (size_t)30U + (size_t)1U; i++)
  {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U, 280005);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_3(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_303(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_25(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_430(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_f4(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_821(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_1d(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_ea0(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d8(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_422(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_60(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_610(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_29(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_fe1(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_9d(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_380(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_5f(re);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_fe_by_fer(int32_t fe, int32_t fer)
{
  return
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element((int64_t)fe * (int64_t)fer);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta,
  size_t index,
  size_t step
)
{
  int32_t
  t =
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_fe_by_fer(simd_unit->data[index +
        step],
      zeta);
  simd_unit->data[index + step] = simd_unit->data[index] - t;
  simd_unit->data[index] += t;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_2(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta
)
{
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta, (size_t)0U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta, (size_t)1U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta, (size_t)2U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta, (size_t)3U, (size_t)4U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(
  Eurydice_arr_a3 *re,
  size_t index,
  int32_t zeta
)
{
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_2(&re->data[index], zeta);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)0U, 2706023);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)1U, 95776);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)2U, 3077325);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)3U, 3530437);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)4U, -1661693);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)5U, -3592148);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)6U, -2537516);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)7U, 3915439);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)8U, -3861115);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)9U, -3043716);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)10U, 3574422);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)11U, -2867647);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)12U, 3539968);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)13U, -300467);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)14U, 2348700);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)15U, -539299);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)16U, -1699267);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)17U, -1643818);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)18U, 3505694);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)19U, -3821735);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)20U, 3507263);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)21U, -2140649);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)22U, -1600420);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)23U, 3699596);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)24U, 811944);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)25U, 531354);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)26U, 954230);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)27U, 3881043);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)28U, 3900724);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)29U, -2556880);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)30U, 2071892);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)31U, -2797779);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_1(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta1,
  int32_t zeta2
)
{
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta1, (size_t)0U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta1, (size_t)1U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta2, (size_t)4U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta2, (size_t)5U, (size_t)2U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
  Eurydice_arr_a3 *re,
  size_t index,
  int32_t zeta_0,
  int32_t zeta_1
)
{
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_1(&re->data[index], zeta_0, zeta_1);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)0U, -3930395, -1528703);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)1U, -3677745, -3041255);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)2U, -1452451, 3475950);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)3U, 2176455, -1585221);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)4U, -1257611, 1939314);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)5U, -4083598, -1000202);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)6U, -3190144, -3157330);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)7U, -3632928, 126922);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)8U, 3412210, -983419);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)9U, 2147896, 2715295);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)10U, -2967645, -3693493);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)11U, -411027, -2477047);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)12U, -671102, -1228525);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)13U, -22981, -1308169);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)14U, -381987, 1349076);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)15U, 1852771, -1430430);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)16U, -3343383, 264944);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)17U, 508951, 3097992);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)18U, 44288, -1100098);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)19U, 904516, 3958618);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)20U, -3724342, -8578);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)21U, 1653064, -3249728);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)22U, 2389356, -210977);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)23U, 759969, -1316856);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)24U, 189548, -3553272);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)25U, 3159746, -1851402);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)26U, -2409325, -177440);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)27U, 1315589, 1341330);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)28U, 1285669, -1584928);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)29U, -812732, -1439742);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)30U, -3019102, -3881060);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(re, (size_t)31U, -3628969, 3839961);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_0(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta0,
  int32_t zeta1,
  int32_t zeta2,
  int32_t zeta3
)
{
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta0, (size_t)0U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta1, (size_t)2U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta2, (size_t)4U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta3, (size_t)6U, (size_t)1U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
  Eurydice_arr_a3 *re,
  size_t index,
  int32_t zeta_0,
  int32_t zeta_1,
  int32_t zeta_2,
  int32_t zeta_3
)
{
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_0(&re->data[index],
    zeta_0,
    zeta_1,
    zeta_2,
    zeta_3);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)0U,
    2091667,
    3407706,
    2316500,
    3817976);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)1U,
    -3342478,
    2244091,
    -2446433,
    -3562462);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)2U,
    266997,
    2434439,
    -1235728,
    3513181);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)3U,
    -3520352,
    -3759364,
    -1197226,
    -3193378);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)4U,
    900702,
    1859098,
    909542,
    819034);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)5U,
    495491,
    -1613174,
    -43260,
    -522500);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)6U,
    -655327,
    -3122442,
    2031748,
    3207046);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)7U,
    -3556995,
    -525098,
    -768622,
    -3595838);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)8U,
    342297,
    286988,
    -2437823,
    4108315);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)9U,
    3437287,
    -3342277,
    1735879,
    203044);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)10U,
    2842341,
    2691481,
    -2590150,
    1265009);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)11U,
    4055324,
    1247620,
    2486353,
    1595974);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)12U,
    -3767016,
    1250494,
    2635921,
    -3548272);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)13U,
    -2994039,
    1869119,
    1903435,
    -1050970);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)14U,
    -1333058,
    1237275,
    -3318210,
    -1430225);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)15U,
    -451100,
    1312455,
    3306115,
    -1962642);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)16U,
    -1279661,
    1917081,
    -2546312,
    -1374803);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)17U,
    1500165,
    777191,
    2235880,
    3406031);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)18U,
    -542412,
    -2831860,
    -1671176,
    -1846953);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)19U,
    -2584293,
    -3724270,
    594136,
    -3776993);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)20U,
    -2013608,
    2432395,
    2454455,
    -164721);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)21U,
    1957272,
    3369112,
    185531,
    -1207385);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)22U,
    -3183426,
    162844,
    1616392,
    3014001);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)23U,
    810149,
    1652634,
    -3694233,
    -1799107);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)24U,
    -3038916,
    3523897,
    3866901,
    269760);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)25U,
    2213111,
    -975884,
    1717735,
    472078);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)26U,
    -426683,
    1723600,
    -1803090,
    1910376);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)27U,
    -1667432,
    -1104333,
    -260646,
    -3833893);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)28U,
    -2939036,
    -2235985,
    -420899,
    -2286327);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)29U,
    183443,
    -976891,
    1612842,
    -3545687);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)30U,
    -554416,
    3919660,
    -48306,
    -1362209);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(re,
    (size_t)31U,
    3937738,
    1400424,
    -846154,
    1976782);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_7(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_6(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_5(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_4(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_3(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1(re);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0(re);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_ntt_65(Eurydice_arr_a3 *simd_units)
{
  libcrux_ml_dsa_simd_portable_ntt_ntt(simd_units);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta,
  size_t index,
  size_t step
)
{
  int32_t a_minus_b = simd_unit->data[index + step] - simd_unit->data[index];
  simd_unit->data[index] += simd_unit->data[index + step];
  simd_unit->data[index + step] =
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_fe_by_fer(a_minus_b,
      zeta);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_0(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta0,
  int32_t zeta1,
  int32_t zeta2,
  int32_t zeta3
)
{
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta0,
    (size_t)0U,
    (size_t)1U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta1,
    (size_t)2U,
    (size_t)1U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta2,
    (size_t)4U,
    (size_t)1U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta3,
    (size_t)6U,
    (size_t)1U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
  Eurydice_arr_a3 *re,
  size_t index,
  int32_t zeta0,
  int32_t zeta1,
  int32_t zeta2,
  int32_t zeta3
)
{
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_0(&re->data[index],
    zeta0,
    zeta1,
    zeta2,
    zeta3);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)0U,
    1976782,
    -846154,
    1400424,
    3937738);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)1U,
    -1362209,
    -48306,
    3919660,
    -554416);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)2U,
    -3545687,
    1612842,
    -976891,
    183443);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)3U,
    -2286327,
    -420899,
    -2235985,
    -2939036);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)4U,
    -3833893,
    -260646,
    -1104333,
    -1667432);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)5U,
    1910376,
    -1803090,
    1723600,
    -426683);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)6U,
    472078,
    1717735,
    -975884,
    2213111);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)7U,
    269760,
    3866901,
    3523897,
    -3038916);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)8U,
    -1799107,
    -3694233,
    1652634,
    810149);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)9U,
    3014001,
    1616392,
    162844,
    -3183426);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)10U,
    -1207385,
    185531,
    3369112,
    1957272);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)11U,
    -164721,
    2454455,
    2432395,
    -2013608);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)12U,
    -3776993,
    594136,
    -3724270,
    -2584293);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)13U,
    -1846953,
    -1671176,
    -2831860,
    -542412);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)14U,
    3406031,
    2235880,
    777191,
    1500165);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)15U,
    -1374803,
    -2546312,
    1917081,
    -1279661);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)16U,
    -1962642,
    3306115,
    1312455,
    -451100);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)17U,
    -1430225,
    -3318210,
    1237275,
    -1333058);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)18U,
    -1050970,
    1903435,
    1869119,
    -2994039);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)19U,
    -3548272,
    2635921,
    1250494,
    -3767016);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)20U,
    1595974,
    2486353,
    1247620,
    4055324);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)21U,
    1265009,
    -2590150,
    2691481,
    2842341);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)22U,
    203044,
    1735879,
    -3342277,
    3437287);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)23U,
    4108315,
    -2437823,
    286988,
    342297);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)24U,
    -3595838,
    -768622,
    -525098,
    -3556995);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)25U,
    3207046,
    2031748,
    -3122442,
    -655327);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)26U,
    -522500,
    -43260,
    -1613174,
    495491);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)27U,
    819034,
    909542,
    1859098,
    900702);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)28U,
    -3193378,
    -1197226,
    -3759364,
    -3520352);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)29U,
    3513181,
    -1235728,
    2434439,
    266997);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)30U,
    -3562462,
    -2446433,
    2244091,
    -3342478);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(re,
    (size_t)31U,
    3817976,
    2316500,
    3407706,
    2091667);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_1(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta0,
  int32_t zeta1
)
{
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta0,
    (size_t)0U,
    (size_t)2U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta0,
    (size_t)1U,
    (size_t)2U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta1,
    (size_t)4U,
    (size_t)2U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta1,
    (size_t)5U,
    (size_t)2U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
  Eurydice_arr_a3 *re,
  size_t index,
  int32_t zeta_00,
  int32_t zeta_01
)
{
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_1(&re->data[index],
    zeta_00,
    zeta_01);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)0U,
    3839961,
    -3628969);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)1U,
    -3881060,
    -3019102);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)2U,
    -1439742,
    -812732);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)3U,
    -1584928,
    1285669);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)4U,
    1341330,
    1315589);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)5U,
    -177440,
    -2409325);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)6U,
    -1851402,
    3159746);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)7U,
    -3553272,
    189548);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)8U,
    -1316856,
    759969);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)9U,
    -210977,
    2389356);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)10U,
    -3249728,
    1653064);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)11U,
    -8578,
    -3724342);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)12U,
    3958618,
    904516);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)13U,
    -1100098,
    44288);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)14U,
    3097992,
    508951);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)15U,
    264944,
    -3343383);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)16U,
    -1430430,
    1852771);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)17U,
    1349076,
    -381987);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)18U,
    -1308169,
    -22981);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)19U,
    -1228525,
    -671102);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)20U,
    -2477047,
    -411027);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)21U,
    -3693493,
    -2967645);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)22U,
    2715295,
    2147896);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)23U,
    -983419,
    3412210);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)24U,
    126922,
    -3632928);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)25U,
    -3157330,
    -3190144);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)26U,
    -1000202,
    -4083598);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)27U,
    1939314,
    -1257611);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)28U,
    -1585221,
    2176455);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)29U,
    3475950,
    -1452451);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)30U,
    -3041255,
    -3677745);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(re,
    (size_t)31U,
    -1528703,
    -3930395);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_2(
  Eurydice_arr_4d *simd_unit,
  int32_t zeta
)
{
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta,
    (size_t)0U,
    (size_t)4U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta,
    (size_t)1U,
    (size_t)4U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta,
    (size_t)2U,
    (size_t)4U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(simd_unit,
    zeta,
    (size_t)3U,
    (size_t)4U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
  Eurydice_arr_a3 *re,
  size_t index,
  int32_t zeta1
)
{
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_2(&re->data[index], zeta1);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)0U, -2797779);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)1U, 2071892);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)2U, -2556880);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)3U, 3900724);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)4U, 3881043);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)5U, 954230);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)6U, 531354);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)7U, 811944);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)8U, 3699596);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)9U, -1600420);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)10U, -2140649);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)11U, 3507263);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)12U, -3821735);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)13U, 3505694);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)14U, -1643818);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)15U, -1699267);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)16U, -539299);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)17U, 2348700);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)18U, -300467);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)19U, 3539968);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)20U, -2867647);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)21U, 3574422);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)22U, -3043716);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)23U, -3861115);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)24U, 3915439);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)25U, -2537516);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)26U, -3592148);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)27U, -1661693);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)28U, 3530437);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)29U, 3077325);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)30U, 95776);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(re, (size_t)31U, 2706023);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 1
- ZETA= 280005
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_30(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      280005);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 2
- STEP_BY= 1
- ZETA= 4010497
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_25(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)2U; i < (size_t)2U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      4010497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 1
- ZETA= -19422
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_43(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -19422);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 6
- STEP_BY= 1
- ZETA= 1757237
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_f4(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)6U; i < (size_t)6U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      1757237);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 1
- ZETA= -3277672
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_82(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -3277672);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 10
- STEP_BY= 1
- ZETA= -1399561
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_1d(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)10U; i < (size_t)10U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -1399561);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 1
- ZETA= -3859737
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_ea(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -3859737);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 14
- STEP_BY= 1
- ZETA= -2118186
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d8(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)14U; i < (size_t)14U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -2118186);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 1
- ZETA= -2108549
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_42(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -2108549);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 18
- STEP_BY= 1
- ZETA= 2619752
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_60(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)18U; i < (size_t)18U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      2619752);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 1
- ZETA= -1119584
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_61(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -1119584);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 22
- STEP_BY= 1
- ZETA= -549488
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_29(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)22U; i < (size_t)22U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -549488);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 1
- ZETA= 3585928
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_fe(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      3585928);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 26
- STEP_BY= 1
- ZETA= -1079900
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_9d(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)26U; i < (size_t)26U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      -1079900);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 1
- ZETA= 1024112
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_38(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      1024112);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 30
- STEP_BY= 1
- ZETA= 2725464
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_5f(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)30U; i < (size_t)30U + (size_t)1U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)1U],
      2725464);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_3(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_30(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_25(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_43(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_f4(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_82(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_1d(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_ea(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d8(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_42(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_60(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_61(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_29(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_fe(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_9d(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_38(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_5f(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 2
- ZETA= 2680103
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_300(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      2680103);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 2
- ZETA= 3111497
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_430(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      3111497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 2
- ZETA= -2884855
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_820(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      -2884855);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 2
- ZETA= 3119733
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_ea0(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      3119733);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 2
- ZETA= -2091905
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_420(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      -2091905);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 2
- ZETA= -359251
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_610(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      -359251);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 2
- ZETA= 2353451
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_fe0(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      2353451);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 2
- ZETA= 1826347
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_380(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)2U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)2U],
      1826347);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_4(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_300(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_430(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_820(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_ea0(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_420(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_610(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_fe0(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_380(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 4
- ZETA= 466468
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_301(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)4U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)4U],
      466468);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 4
- ZETA= -876248
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_821(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)4U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)4U],
      -876248);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 4
- ZETA= -777960
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_421(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)4U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)4U],
      -777960);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 4
- ZETA= 237124
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_fe1(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)4U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)4U],
      237124);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_5(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_301(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_821(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_421(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_fe1(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 8
- ZETA= -518909
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_302(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)8U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)8U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)8U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)8U],
      -518909);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 8
- ZETA= -2608894
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_422(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)8U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)8U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)8U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)8U],
      -2608894);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_6(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_302(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_422(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 16
- ZETA= 25847
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_303(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)16U; i++)
  {
    size_t j = i;
    Eurydice_arr_4d rej = re->data[j];
    Eurydice_arr_4d rejs = re->data[j + (size_t)16U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)16U], &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[j +
        (size_t)16U],
      25847);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_7(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_303(re);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_montgomery(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_3(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_4(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_5(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_6(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_7(re);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&re->data[i0], 41978);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_invert_ntt_montgomery_65(Eurydice_arr_a3 *simd_units)
{
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_montgomery(simd_units);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void
libcrux_ml_dsa_simd_portable_barrett_reduce_simd_unit_65(Eurydice_arr_4d *simd_unit)
{
  libcrux_ml_dsa_simd_portable_arithmetic_barrett_reduce_simd_unit(simd_unit);
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_VECTOR_SIZE (libcrux_ml_dsa_constants_commitment_vector_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_COMMITMENT_COEFFICIENT, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A))

/**
A monomorphic instance of libcrux_ml_dsa.types.MLDSASigningKey
with const generics
- $4032size_t
*/
typedef Eurydice_arr_24 libcrux_ml_dsa_types_MLDSASigningKey_8e;

typedef Eurydice_arr_24 libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65SigningKey;

/**
A monomorphic instance of libcrux_ml_dsa.types.MLDSAVerificationKey
with const generics
- $1952size_t
*/
typedef Eurydice_arr_29 libcrux_ml_dsa_types_MLDSAVerificationKey_c8;

typedef Eurydice_arr_29 libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65VerificationKey;

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ROW_COLUMN (LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A + LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A)

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ROW_X_COLUMN (LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A * LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A)

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNING_KEY_SIZE (libcrux_ml_dsa_constants_signing_key_size(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE))

#define LIBCRUX_ML_DSA_PRE_HASH_PRE_HASH_OID_LEN ((size_t)11U)

typedef Eurydice_arr_c9 libcrux_ml_dsa_pre_hash_PreHashOID;

typedef core_result_Result_a8 libcrux_ml_dsa_pre_hash_PreHashResult;

/**
This function found in impl {core::convert::From<libcrux_ml_dsa::pre_hash::DomainSeparationError> for libcrux_ml_dsa::types::SigningError}
*/
static inline libcrux_ml_dsa_types_SigningError
libcrux_ml_dsa_pre_hash_from_96(libcrux_ml_dsa_pre_hash_DomainSeparationError e)
{
  return libcrux_ml_dsa_types_SigningError_ContextTooLongError;
}

/**
This function found in impl {core::convert::From<libcrux_ml_dsa::pre_hash::DomainSeparationError> for libcrux_ml_dsa::types::VerificationError}
*/
static inline libcrux_ml_dsa_types_VerificationError
libcrux_ml_dsa_pre_hash_from_bf(libcrux_ml_dsa_pre_hash_DomainSeparationError e)
{
  return libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError;
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_3_STEP ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_3_STEP_BY ((size_t)1U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_4_STEP ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_4_STEP_BY ((size_t)2U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_5_STEP ((size_t)32U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_5_STEP_BY ((size_t)4U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_6_STEP ((size_t)64U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_6_STEP_BY ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_7_STEP ((size_t)128U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_INVNTT_INVERT_NTT_AT_LAYER_7_STEP_BY ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_3_STEP ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_3_STEP_BY ((size_t)1U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_4_STEP ((size_t)16U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_4_STEP_BY ((size_t)2U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_5_STEP ((size_t)32U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_5_STEP_BY ((size_t)4U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_6_STEP ((size_t)64U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_6_STEP_BY ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_7_STEP ((size_t)128U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_NTT_NTT_AT_LAYER_7_STEP_BY ((size_t)16U)

typedef int32_t libcrux_ml_dsa_simd_portable_vector_type_FieldElement;

/**
This function found in impl {core::clone::Clone for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline Eurydice_arr_4d
libcrux_ml_dsa_simd_portable_vector_type_clone_a5(const Eurydice_arr_4d *self)
{
  return self[0U];
}

typedef int32_t libcrux_ml_dsa_simd_traits_FieldElementTimesMontgomeryR;

typedef Eurydice_arr_0c libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65Signature;

/**
 A reference to the raw byte array.
*/
/**
This function found in impl {libcrux_ml_dsa::types::MLDSASignature<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_dsa.types.as_ref_c5
with const generics
- SIZE= 3309
*/
static inline const
Eurydice_arr_0c
*libcrux_ml_dsa_types_as_ref_c5_5c(const Eurydice_arr_0c *self)
{
  return self;
}

/**
 A reference to the raw byte array.
*/
/**
This function found in impl {libcrux_ml_dsa::types::MLDSAVerificationKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_dsa.types.as_ref_7f
with const generics
- SIZE= 1952
*/
static inline const
Eurydice_arr_29
*libcrux_ml_dsa_types_as_ref_7f_a2(const Eurydice_arr_29 *self)
{
  return self;
}

/**
 A reference to the raw byte array.
*/
/**
This function found in impl {libcrux_ml_dsa::types::MLDSASigningKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_dsa.types.as_ref_9b
with const generics
- SIZE= 4032
*/
static inline const
Eurydice_arr_24
*libcrux_ml_dsa_types_as_ref_9b_e5(const Eurydice_arr_24 *self)
{
  return self;
}

/**
 Build
*/
/**
This function found in impl {libcrux_ml_dsa::types::MLDSAVerificationKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_dsa.types.new_7f
with const generics
- SIZE= 1952
*/
static inline Eurydice_arr_29 libcrux_ml_dsa_types_new_7f_a2(Eurydice_arr_29 value)
{
  return value;
}

/**
 Build
*/
/**
This function found in impl {libcrux_ml_dsa::types::MLDSASigningKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_dsa.types.new_9b
with const generics
- SIZE= 4032
*/
static inline Eurydice_arr_24 libcrux_ml_dsa_types_new_9b_e5(Eurydice_arr_24 value)
{
  return value;
}

/**
A monomorphic instance of libcrux_ml_dsa.polynomial.PolynomialRingElement
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients

*/
typedef Eurydice_arr_a3 libcrux_ml_dsa_polynomial_PolynomialRingElement_e8;

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8, size_t

*/
typedef struct Eurydice_dst_ref_mut_44_s
{
  Eurydice_arr_a3 *ptr;
  size_t meta;
}
Eurydice_dst_ref_mut_44;

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8, size_t

*/
typedef struct Eurydice_dst_ref_shared_44_s
{
  const Eurydice_arr_a3 *ptr;
  size_t meta;
}
Eurydice_dst_ref_shared_44;

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.zero_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline Eurydice_arr_a3 libcrux_ml_dsa_polynomial_zero_ff_37(void)
{
  Eurydice_arr_a3 lit;
  Eurydice_arr_4d repeat_expression[32U];
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    repeat_expression[i] = libcrux_ml_dsa_simd_portable_zero_65();
  }
  memcpy(lit.data, repeat_expression, (size_t)32U * sizeof (Eurydice_arr_4d));
  return lit;
}

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.from_i32_array_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline void
libcrux_ml_dsa_polynomial_from_i32_array_ff_37(
  Eurydice_dst_ref_shared_83 array,
  Eurydice_arr_a3 *result
)
{
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_DSA_SIMD_TRAITS_SIMD_UNITS_IN_RING_ELEMENT; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_from_coefficient_array_65(Eurydice_slice_subslice_shared_47(array,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT
          }
        )),
      &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.use_hint
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_arithmetic_use_hint_37(
  int32_t gamma2,
  Eurydice_dst_ref_shared_20 hint,
  Eurydice_dst_ref_mut_44 re_vector
)
{
  for (size_t i0 = (size_t)0U; i0 < re_vector.meta; i0++)
  {
    size_t i1 = i0;
    Eurydice_arr_a3 tmp = libcrux_ml_dsa_polynomial_zero_ff_37();
    libcrux_ml_dsa_polynomial_from_i32_array_ff_37(Eurydice_array_to_slice_shared_af(&hint.ptr[i1]),
      &tmp);
    for (size_t i = (size_t)0U; i < (size_t)32U; i++)
    {
      size_t j = i;
      libcrux_ml_dsa_simd_portable_use_hint_65(gamma2, &re_vector.ptr[i1].data[j], &tmp.data[j]);
    }
    re_vector.ptr[i1] = tmp;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt_multiply_montgomery
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(Eurydice_arr_a3 *lhs, const Eurydice_arr_a3 *rhs)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_montgomery_multiply_65(&lhs->data[i0], &rhs->data[i0]);
  }
}

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.add_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_polynomial_add_ff_37(Eurydice_arr_a3 *self, const Eurydice_arr_a3 *rhs)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_add_65(&self->data[i0], &rhs->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.arithmetic.shift_left_then_reduce
with const generics
- SHIFT_BY= 13
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_shift_left_then_reduce_84(Eurydice_arr_4d *simd_unit)
{
  for (size_t i = (size_t)0U; i < (size_t)8U; i++)
  {
    size_t i0 = i;
    simd_unit->data[i0] = (int32_t)((uint32_t)simd_unit->data[i0] << (uint32_t)13);
  }
  libcrux_ml_dsa_simd_portable_arithmetic_barrett_reduce_simd_unit(simd_unit);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.shift_left_then_reduce_65
with const generics
- SHIFT_BY= 13
*/
static inline void
libcrux_ml_dsa_simd_portable_shift_left_then_reduce_65_84(Eurydice_arr_4d *simd_unit)
{
  libcrux_ml_dsa_simd_portable_arithmetic_shift_left_then_reduce_84(simd_unit);
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.shift_left_then_reduce
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- SHIFT_BY= 13
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_arithmetic_shift_left_then_reduce_68(Eurydice_arr_a3 *re)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_shift_left_then_reduce_65_84(&re->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_ntt_37(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_ntt_65(re);
}

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.subtract_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_polynomial_subtract_ff_37(Eurydice_arr_a3 *self, const Eurydice_arr_a3 *rhs)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_subtract_65(&self->data[i0], &rhs->data[i0]);
  }
}

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.barrett_reduce_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_polynomial_barrett_reduce_ff_37(Eurydice_arr_a3 *self)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_barrett_reduce_simd_unit_65(&self->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.invert_ntt_montgomery
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(Eurydice_arr_a3 *re)
{
  libcrux_ml_dsa_simd_portable_invert_ntt_montgomery_65(re);
}

/**
 Compute InvertNTT(Â ◦ ẑ - ĉ ◦ NTT(t₁2ᵈ))
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_w_approx
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_matrix_compute_w_approx_37(
  size_t rows_in_a,
  size_t columns_in_a,
  Eurydice_dst_ref_shared_44 matrix,
  Eurydice_dst_ref_shared_44 signer_response,
  const Eurydice_arr_a3 *verifier_challenge_as_ntt,
  Eurydice_dst_ref_mut_44 t1
)
{
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++)
  {
    size_t i1 = i0;
    Eurydice_arr_a3 inner_result = libcrux_ml_dsa_polynomial_zero_ff_37();
    for (size_t i = (size_t)0U; i < columns_in_a; i++)
    {
      size_t j = i;
      Eurydice_arr_a3 product = matrix.ptr[i1 * columns_in_a + j];
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(&product, &signer_response.ptr[j]);
      libcrux_ml_dsa_polynomial_add_ff_37(&inner_result, &product);
    }
    libcrux_ml_dsa_arithmetic_shift_left_then_reduce_68(&t1.ptr[i1]);
    libcrux_ml_dsa_ntt_ntt_37(&t1.ptr[i1]);
    libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(&t1.ptr[i1], verifier_challenge_as_ntt);
    libcrux_ml_dsa_polynomial_subtract_ff_37(&inner_result, &t1.ptr[i1]);
    t1.ptr[i1] = inner_result;
    libcrux_ml_dsa_polynomial_barrett_reduce_ff_37(&t1.ptr[i1]);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(&t1.ptr[i1]);
  }
}

/**
A monomorphic instance of core.result.Result
with types (), libcrux_ml_dsa_types_VerificationError

*/
typedef struct core_result_Result_41_s
{
  core_result_Result_57_tags tag;
  libcrux_ml_dsa_types_VerificationError f0;
}
core_result_Result_41;

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_gamma1_deserialize_37(
  size_t gamma1_exponent,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_a3 *result
)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_gamma1_deserialize_65(Eurydice_slice_subslice_shared_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * (gamma1_exponent + (size_t)1U),
            (i0 + (size_t)1U) * (gamma1_exponent + (size_t)1U)
          }
        )),
      &result->data[i0],
      gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_encoding_signature_deserialize_37(
  size_t columns_in_a,
  size_t rows_in_a,
  size_t commitment_hash_size,
  size_t gamma1_exponent,
  size_t gamma1_ring_element_size,
  size_t max_ones_in_hint,
  size_t signature_size,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_mut_borrow_slice_u8 out_commitment_hash,
  Eurydice_dst_ref_mut_44 out_signer_response,
  Eurydice_dst_ref_mut_20 out_hint
)
{
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(serialized,
      commitment_hash_size,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 commitment_hash = uu____0.fst;
  Eurydice_borrow_slice_u8 rest_of_serialized = uu____0.snd;
  Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(out_commitment_hash,
      (core_ops_range_Range_87{ (size_t)0U, commitment_hash_size })),
    commitment_hash,
    uint8_t);
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(rest_of_serialized,
      gamma1_ring_element_size * columns_in_a,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 signer_response_serialized = uu____1.fst;
  Eurydice_borrow_slice_u8 hint_serialized = uu____1.snd;
  for (size_t i = (size_t)0U; i < columns_in_a; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
      Eurydice_slice_subslice_shared_c8(signer_response_serialized,
        (
          core_ops_range_Range_87{
            i0 * gamma1_ring_element_size,
            (i0 + (size_t)1U) * gamma1_ring_element_size
          }
        )),
      &out_signer_response.ptr[i0]);
  }
  size_t previous_true_hints_seen = (size_t)0U;
  bool malformed_hint = false;
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++)
  {
    size_t i1 = i0;
    size_t current_true_hints_seen = (size_t)(uint32_t)hint_serialized.ptr[max_ones_in_hint + i1];
    if (current_true_hints_seen < previous_true_hints_seen)
    {
      malformed_hint = true;
      break;
    }
    if (current_true_hints_seen > max_ones_in_hint)
    {
      malformed_hint = true;
      break;
    }
    for (size_t i = previous_true_hints_seen; i < current_true_hints_seen; i++)
    {
      size_t j = i;
      if (j > previous_true_hints_seen)
      {
        if (hint_serialized.ptr[j] <= hint_serialized.ptr[j - (size_t)1U])
        {
          malformed_hint = true;
          break;
        }
      }
      libcrux_ml_dsa_encoding_signature_set_hint(out_hint,
        i1,
        (size_t)(uint32_t)hint_serialized.ptr[j]);
    }
    if (malformed_hint)
    {
      break;
    }
    previous_true_hints_seen = current_true_hints_seen;
  }
  for (size_t i = previous_true_hints_seen; i < max_ones_in_hint; i++)
  {
    size_t j = i;
    if (hint_serialized.ptr[j] != 0U)
    {
      malformed_hint = true;
      break;
    }
  }
  core_result_Result_41 uu____2;
  if (malformed_hint)
  {
    uu____2 =
      (
        core_result_Result_41{
          core_result_Err,
          libcrux_ml_dsa_types_VerificationError_MalformedHintError
        }
      );
  }
  else
  {
    uu____2 = (core_result_Result_41{ core_result_Ok });
  }
  return uu____2;
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline void
libcrux_ml_dsa_encoding_t1_deserialize_37(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_a3 *result
)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_t1_deserialize_65(Eurydice_slice_subslice_shared_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW
          }
        )),
      &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.verification_key.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_verification_key_deserialize_37(
  size_t rows_in_a,
  size_t verification_key_size,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_dst_ref_mut_44 t1
)
{
  for (size_t i = (size_t)0U; i < rows_in_a; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_t1_deserialize_37(Eurydice_slice_subslice_shared_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE
          }
        )),
      &t1.ptr[i0]);
  }
}

/**
A monomorphic instance of core.result.Result
with types libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65Signature, libcrux_ml_dsa_types_SigningError

*/
typedef struct core_result_Result_8c_s
{
  core_result_Result_57_tags tag;
  union U {
    Eurydice_arr_0c case_Ok;
    libcrux_ml_dsa_types_SigningError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_8c_s)
}
core_result_Result_8c;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_5d_s { Eurydice_arr_a3 data[5U]; } Eurydice_arr_5d;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_5d

*/
typedef struct core_option_Option_1e_s
{
  core_option_Option_05_tags tag;
  Eurydice_arr_5d f0;
}
core_option_Option_1e;

/**
A monomorphic instance of core.result.Result
with types (), libcrux_ml_dsa_types_SigningError

*/
typedef struct core_result_Result_53_s
{
  core_result_Result_57_tags tag;
  libcrux_ml_dsa_types_SigningError f0;
}
core_result_Result_53;

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_gamma1_serialize_37(
  const Eurydice_arr_a3 *re,
  Eurydice_mut_borrow_slice_u8 serialized,
  size_t gamma1_exponent
)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    const Eurydice_arr_4d *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_gamma1_serialize_65(simd_unit,
      Eurydice_slice_subslice_mut_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * (gamma1_exponent + (size_t)1U),
            (i0 + (size_t)1U) * (gamma1_exponent + (size_t)1U)
          }
        )),
      gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_signature_serialize_37(
  Eurydice_borrow_slice_u8 commitment_hash,
  Eurydice_dst_ref_shared_44 signer_response,
  Eurydice_dst_ref_shared_20 hint,
  size_t commitment_hash_size,
  size_t columns_in_a,
  size_t rows_in_a,
  size_t gamma1_exponent,
  size_t gamma1_ring_element_size,
  size_t max_ones_in_hint,
  Eurydice_mut_borrow_slice_u8 signature
)
{
  size_t offset = (size_t)0U;
  Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(signature,
      (core_ops_range_Range_87{ offset, offset + commitment_hash_size })),
    commitment_hash,
    uint8_t);
  offset += commitment_hash_size;
  for (size_t i = (size_t)0U; i < columns_in_a; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_gamma1_serialize_37(&signer_response.ptr[i0],
      Eurydice_slice_subslice_mut_c8(signature,
        (core_ops_range_Range_87{ offset, offset + gamma1_ring_element_size })),
      gamma1_exponent);
    offset += gamma1_ring_element_size;
  }
  size_t true_hints_seen = (size_t)0U;
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++)
  {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)256U; i++)
    {
      size_t j = i;
      if (hint.ptr[i1].data[j] == 1)
      {
        signature.ptr[offset + true_hints_seen] = (uint8_t)j;
        true_hints_seen++;
      }
    }
    signature.ptr[offset + max_ones_in_hint + i1] = (uint8_t)true_hints_seen;
  }
}

/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.to_i32_array_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline Eurydice_arr_6c
libcrux_ml_dsa_polynomial_to_i32_array_ff_37(const Eurydice_arr_a3 *self)
{
  Eurydice_arr_6c result = { { 0U } };
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_to_coefficient_array_65(&self->data[i0],
      Eurydice_array_to_subslice_mut_44(&result,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT
          }
        )));
  }
  return result;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.make_hint
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE size_t
libcrux_ml_dsa_arithmetic_make_hint_37(
  Eurydice_dst_ref_shared_44 low,
  Eurydice_dst_ref_shared_44 high,
  int32_t gamma2,
  Eurydice_dst_ref_mut_20 hint
)
{
  size_t true_hints = (size_t)0U;
  Eurydice_arr_a3 hint_simd = libcrux_ml_dsa_polynomial_zero_ff_37();
  for (size_t i0 = (size_t)0U; i0 < low.meta; i0++)
  {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)32U; i++)
    {
      size_t j = i;
      size_t
      one_hints_count =
        libcrux_ml_dsa_simd_portable_compute_hint_65(&low.ptr[i1].data[j],
          &high.ptr[i1].data[j],
          gamma2,
          &hint_simd.data[j]);
      true_hints += one_hints_count;
    }
    Eurydice_arr_6c uu____0 = libcrux_ml_dsa_polynomial_to_i32_array_ff_37(&hint_simd);
    hint.ptr[i1] = uu____0;
  }
  return true_hints;
}

/**
 CAUTION: This function must only be called with inputs for
 which it is safe to leak the index of a violating coefficient.

 For all norm checks during ML-DSA signature generation it is
 safe to leak the index of a violating coefficient.
*/
/**
This function found in impl {libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.infinity_norm_exceeds_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_polynomial_infinity_norm_exceeds_ff_37(
  const Eurydice_arr_a3 *self,
  int32_t bound
)
{
  bool result = false;
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    bool
    coeff_exceeds = libcrux_ml_dsa_simd_portable_infinity_norm_exceeds_65(&self->data[i0], bound);
    bool uu____0;
    if (result)
    {
      uu____0 = true;
    }
    else
    {
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
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(
  Eurydice_dst_ref_shared_44 vector,
  int32_t bound
)
{
  bool result = false;
  for (size_t i = (size_t)0U; i < vector.meta; i++)
  {
    size_t i0 = i;
    bool uu____0;
    if (result)
    {
      uu____0 = true;
    }
    else
    {
      uu____0 = libcrux_ml_dsa_polynomial_infinity_norm_exceeds_ff_37(&vector.ptr[i0], bound);
    }
    result = uu____0;
  }
  return result;
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.subtract_vectors
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_matrix_subtract_vectors_37(
  size_t dimension,
  Eurydice_dst_ref_mut_44 lhs,
  Eurydice_dst_ref_shared_44 rhs
)
{
  for (size_t i = (size_t)0U; i < dimension; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_subtract_ff_37(&lhs.ptr[i0], &rhs.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.add_vectors
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_matrix_add_vectors_37(
  size_t dimension,
  Eurydice_dst_ref_mut_44 lhs,
  Eurydice_dst_ref_shared_44 rhs
)
{
  for (size_t i = (size_t)0U; i < dimension; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_add_ff_37(&lhs.ptr[i0], &rhs.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.vector_times_ring_element
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_matrix_vector_times_ring_element_37(
  Eurydice_dst_ref_mut_44 vector,
  const Eurydice_arr_a3 *ring_element
)
{
  for (size_t i = (size_t)0U; i < vector.meta; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(&vector.ptr[i0], ring_element);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(&vector.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_commitment_serialize_37(
  const Eurydice_arr_a3 *re,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  size_t output_bytes_per_simd_unit = serialized.meta / ((size_t)8U * (size_t)4U);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    const Eurydice_arr_4d *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_commitment_serialize_65(simd_unit,
      Eurydice_slice_subslice_mut_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * output_bytes_per_simd_unit,
            (i0 + (size_t)1U) * output_bytes_per_simd_unit
          }
        )));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_commitment_serialize_vector_37(
  size_t ring_element_size,
  Eurydice_dst_ref_shared_44 vector,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  size_t offset = (size_t)0U;
  for (size_t i = (size_t)0U; i < vector.meta; i++)
  {
    size_t _cloop_j = i;
    const Eurydice_arr_a3 *ring_element = &vector.ptr[_cloop_j];
    libcrux_ml_dsa_encoding_commitment_serialize_37(ring_element,
      Eurydice_slice_subslice_mut_c8(serialized,
        (core_ops_range_Range_87{ offset, offset + ring_element_size })));
    offset += ring_element_size;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.decompose_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_arithmetic_decompose_vector_37(
  size_t dimension,
  int32_t gamma2,
  Eurydice_dst_ref_shared_44 t,
  Eurydice_dst_ref_mut_44 low,
  Eurydice_dst_ref_mut_44 high
)
{
  for (size_t i0 = (size_t)0U; i0 < dimension; i0++)
  {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)32U; i++)
    {
      size_t j = i;
      libcrux_ml_dsa_simd_portable_decompose_65(gamma2,
        &t.ptr[i1].data[j],
        &low.ptr[i1].data[j],
        &high.ptr[i1].data[j]);
    }
  }
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $30size_t
*/
typedef struct Eurydice_arr_5a_s { Eurydice_arr_a3 data[30U]; } Eurydice_arr_5a;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 30
*/
static inline Eurydice_dst_ref_shared_44
Eurydice_array_to_slice_shared_202(const Eurydice_arr_5a *a)
{
  Eurydice_dst_ref_shared_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)30U;
  return lit;
}

/**
 Compute InvertNTT(Â ◦ ŷ)
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_matrix_x_mask
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_matrix_compute_matrix_x_mask_37(
  size_t rows_in_a,
  size_t columns_in_a,
  Eurydice_dst_ref_shared_44 matrix,
  Eurydice_dst_ref_shared_44 mask,
  Eurydice_dst_ref_mut_44 result
)
{
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++)
  {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < columns_in_a; i++)
    {
      size_t j = i;
      Eurydice_arr_a3 product = mask.ptr[j];
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(&product, &matrix.ptr[i1 * columns_in_a + j]);
      libcrux_ml_dsa_polynomial_add_ff_37(&result.ptr[i1], &product);
    }
    libcrux_ml_dsa_polynomial_barrett_reduce_ff_37(&result.ptr[i1]);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(&result.ptr[i1]);
  }
}

/**
A monomorphic instance of core.option.Option
with types libcrux_ml_dsa_pre_hash_DomainSeparationContext

*/
typedef struct core_option_Option_84_s
{
  core_option_Option_05_tags tag;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext f0;
}
core_option_Option_84;

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_t0_deserialize_37(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_a3 *result
)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_t0_deserialize_65(Eurydice_slice_subslice_shared_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT
          }
        )),
      &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.deserialize_to_vector_then_ntt
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_37(
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_dst_ref_mut_44 ring_elements
)
{
  for
  (size_t
    i = (size_t)0U;
    i < serialized.meta / LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
    i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE,
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE +
              LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE
          }
        ));
    libcrux_ml_dsa_encoding_t0_deserialize_37(bytes, &ring_elements.ptr[i0]);
    libcrux_ml_dsa_ntt_ntt_37(&ring_elements.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_error_deserialize_37(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_arr_a3 *result
)
{
  size_t chunk_size = libcrux_ml_dsa_encoding_error_chunk_size(eta);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_error_deserialize_65(eta,
      Eurydice_slice_subslice_shared_c8(serialized,
        (core_ops_range_Range_87{ i0 * chunk_size, (i0 + (size_t)1U) * chunk_size })),
      &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.deserialize_to_vector_then_ntt
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_37(
  libcrux_ml_dsa_constants_Eta eta,
  size_t ring_element_size,
  Eurydice_borrow_slice_u8 serialized,
  Eurydice_dst_ref_mut_44 ring_elements
)
{
  for (size_t i = (size_t)0U; i < serialized.meta / ring_element_size; i++)
  {
    size_t i0 = i;
    Eurydice_borrow_slice_u8
    bytes =
      Eurydice_slice_subslice_shared_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * ring_element_size,
            i0 * ring_element_size + ring_element_size
          }
        ));
    libcrux_ml_dsa_encoding_error_deserialize_37(eta, bytes, &ring_elements.ptr[i0]);
    libcrux_ml_dsa_ntt_ntt_37(&ring_elements.ptr[i0]);
  }
}

/**
 Init with zero
*/
/**
This function found in impl {libcrux_ml_dsa::types::MLDSASignature<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_dsa.types.zero_c5
with const generics
- SIZE= 3309
*/
static inline Eurydice_arr_0c libcrux_ml_dsa_types_zero_c5_5c(void)
{
  return (Eurydice_arr_0c{ { 0U } });
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_t0_serialize_37(
  const Eurydice_arr_a3 *re,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    const Eurydice_arr_4d *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_t0_serialize_65(simd_unit,
      Eurydice_slice_subslice_mut_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT
          }
        )));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_error_serialize_37(
  libcrux_ml_dsa_constants_Eta eta,
  const Eurydice_arr_a3 *re,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  size_t output_bytes_per_simd_unit = libcrux_ml_dsa_encoding_error_chunk_size(eta);
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    const Eurydice_arr_4d *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_error_serialize_65(eta,
      simd_unit,
      Eurydice_slice_subslice_mut_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * output_bytes_per_simd_unit,
            (i0 + (size_t)1U) * output_bytes_per_simd_unit
          }
        )));
  }
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $6size_t
*/
typedef struct Eurydice_arr_dc1_s { Eurydice_arr_a3 data[6U]; } Eurydice_arr_dc1;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 6
*/
static inline Eurydice_dst_ref_shared_44
Eurydice_array_to_slice_shared_201(const Eurydice_arr_dc1 *a)
{
  Eurydice_dst_ref_shared_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_t1_serialize_37(
  const Eurydice_arr_a3 *re,
  Eurydice_mut_borrow_slice_u8 serialized
)
{
  for (size_t i = (size_t)0U; i < (size_t)32U; i++)
  {
    size_t i0 = i;
    const Eurydice_arr_4d *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_t1_serialize_65(simd_unit,
      Eurydice_slice_subslice_mut_c8(serialized,
        (
          core_ops_range_Range_87{
            i0 * LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT,
            (i0 + (size_t)1U) * LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT
          }
        )));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.verification_key.generate_serialized
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_verification_key_generate_serialized_37(
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_shared_44 t1,
  Eurydice_mut_borrow_slice_u8 verification_key_serialized
)
{
  Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(verification_key_serialized,
      (core_ops_range_Range_87{ (size_t)0U, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE })),
    seed,
    uint8_t);
  for (size_t i = (size_t)0U; i < t1.meta; i++)
  {
    size_t i0 = i;
    const Eurydice_arr_a3 *ring_element = &t1.ptr[i0];
    size_t
    offset =
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE +
        i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE;
    libcrux_ml_dsa_encoding_t1_serialize_37(ring_element,
      Eurydice_slice_subslice_mut_c8(verification_key_serialized,
        (
          core_ops_range_Range_87{
            offset,
            offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE
          }
        )));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.power2round_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_arithmetic_power2round_vector_37(
  Eurydice_dst_ref_mut_44 t,
  Eurydice_dst_ref_mut_44 t1
)
{
  for (size_t i0 = (size_t)0U; i0 < t.meta; i0++)
  {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)32U; i++)
    {
      size_t j = i;
      libcrux_ml_dsa_simd_portable_power2round_65(&t.ptr[i1].data[j], &t1.ptr[i1].data[j]);
    }
  }
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients[[$6size_t]]

*/
static KRML_MUSTINLINE void
libcrux_secrets_mem_requests_ct_declassify_b2(const Eurydice_arr_dc1 *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 6
*/
static inline Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_202(Eurydice_arr_dc1 *a)
{
  Eurydice_dst_ref_mut_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $11size_t
*/
typedef struct Eurydice_arr_47_s { Eurydice_arr_a3 data[11U]; } Eurydice_arr_47;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 11
*/
static inline Eurydice_dst_ref_shared_44
Eurydice_array_to_slice_shared_200(const Eurydice_arr_47 *a)
{
  Eurydice_dst_ref_shared_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 5
*/
static inline Eurydice_dst_ref_shared_44
Eurydice_array_to_slice_shared_20(const Eurydice_arr_5d *a)
{
  Eurydice_dst_ref_shared_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)5U;
  return lit;
}

/**
 Compute InvertNTT(Â ◦ ŝ₁) + s₂
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_as1_plus_s2
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_matrix_compute_as1_plus_s2_37(
  size_t rows_in_a,
  size_t columns_in_a,
  Eurydice_dst_ref_mut_44 a_as_ntt,
  Eurydice_dst_ref_shared_44 s1_ntt,
  Eurydice_dst_ref_shared_44 s1_s2,
  Eurydice_dst_ref_mut_44 result
)
{
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++)
  {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < columns_in_a; i++)
    {
      size_t j = i;
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(&a_as_ntt.ptr[i1 * columns_in_a + j],
        &s1_ntt.ptr[j]);
      libcrux_ml_dsa_polynomial_add_ff_37(&result.ptr[i1], &a_as_ntt.ptr[i1 * columns_in_a + j]);
    }
  }
  for (size_t i = (size_t)0U; i < result.meta; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_barrett_reduce_ff_37(&result.ptr[i0]);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(&result.ptr[i0]);
    libcrux_ml_dsa_polynomial_add_ff_37(&result.ptr[i0], &s1_s2.ptr[columns_in_a + i0]);
  }
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients, core_ops_range_Range size_t, Eurydice_derefed_slice libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 11
*/
static inline Eurydice_dst_ref_shared_44
Eurydice_array_to_subslice_shared_25(const Eurydice_arr_47 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_dst_ref_shared_44{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 5
*/
static inline Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_201(Eurydice_arr_5d *a)
{
  Eurydice_dst_ref_mut_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)5U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 30
*/
static inline Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_200(Eurydice_arr_5a *a)
{
  Eurydice_dst_ref_mut_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)30U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_field_modulus
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled_coefficients,
  Eurydice_arr_d0 *out
)
{
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)24U; i++)
  {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8
    random_bytes =
      Eurydice_slice_subslice_shared_c8(randomness,
        (core_ops_range_Range_87{ _cloop_i * (size_t)24U, _cloop_i * (size_t)24U + (size_t)24U }));
    if (!done)
    {
      size_t
      sampled =
        libcrux_ml_dsa_simd_portable_rejection_sample_less_than_field_modulus_65(random_bytes,
          Eurydice_array_to_subslice_from_mut_11(out, sampled_coefficients[0U]));
      sampled_coefficients[0U] += sampled;
      if (sampled_coefficients[0U] >= LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        done = true;
      }
    }
  }
  return done;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 11
*/
static inline Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_20(Eurydice_arr_47 *a)
{
  Eurydice_dst_ref_mut_44 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_4
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_37(
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled_coefficients,
  Eurydice_arr_d0 *out
)
{
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)4U; i++)
  {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8
    random_bytes =
      Eurydice_slice_subslice_shared_c8(randomness,
        (core_ops_range_Range_87{ _cloop_i * (size_t)4U, _cloop_i * (size_t)4U + (size_t)4U }));
    if (!done)
    {
      size_t
      sampled =
        libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_4_65(random_bytes,
          Eurydice_array_to_subslice_from_mut_11(out, sampled_coefficients[0U]));
      sampled_coefficients[0U] += sampled;
      if (sampled_coefficients[0U] >= LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        done = true;
      }
    }
  }
  return done;
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_2
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_37(
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled_coefficients,
  Eurydice_arr_d0 *out
)
{
  bool done = false;
  for (size_t i = (size_t)0U; i < randomness.meta / (size_t)4U; i++)
  {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8
    random_bytes =
      Eurydice_slice_subslice_shared_c8(randomness,
        (core_ops_range_Range_87{ _cloop_i * (size_t)4U, _cloop_i * (size_t)4U + (size_t)4U }));
    if (!done)
    {
      size_t
      sampled =
        libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_2_65(random_bytes,
          Eurydice_array_to_subslice_from_mut_11(out, sampled_coefficients[0U]));
      sampled_coefficients[0U] += sampled;
      if (sampled_coefficients[0U] >= LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT)
      {
        done = true;
      }
    }
  }
  return done;
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.rejection_sample_less_than_eta
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 randomness,
  size_t *sampled,
  Eurydice_arr_d0 *out
)
{
  switch (eta)
  {
    case libcrux_ml_dsa_constants_Eta_Two:
      {
        break;
      }
    case libcrux_ml_dsa_constants_Eta_Four:
      {
        return
          libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_37(randomness,
            sampled,
            out);
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__, __LINE__);
        KRML_HOST_EXIT(253U);
      }
  }
  return
    libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_37(randomness,
      sampled,
      out);
}

typedef struct libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65KeyPair_s
{
  Eurydice_arr_24 signing_key;
  Eurydice_arr_29 verification_key;
}
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65KeyPair;


#define libcrux_mldsa_core_H_DEFINED
#endif /* libcrux_mldsa_core_H */
