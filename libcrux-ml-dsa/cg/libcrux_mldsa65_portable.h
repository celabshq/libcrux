/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 146b7dce58cb11ca8010b1c947c3437a959dcd88
 * Eurydice: cdf02f9d8ed0d73f88c0a495c5b79359a51398fc
 * Karamel: 8e7262955105599e91f3a99c9ab3d3387f7046f2
 * F*: 4b3fc11774003a6ff7c09500ecb5f0145ca6d862
 * Libcrux: 191ac203df9eaaf55c1a5b3559419f99e1127e2d
 */

#ifndef libcrux_mldsa65_portable_H
#define libcrux_mldsa65_portable_H

#include "eurydice_glue.h"
#include "libcrux_mldsa_core.h"
#include "libcrux_sha3_portable.h"

#define libcrux_ml_dsa_constants_Eta_Two 2
#define libcrux_ml_dsa_constants_Eta_Four 4

typedef uint8_t libcrux_ml_dsa_constants_Eta;

#define LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT ((size_t)8U)

#define LIBCRUX_ML_DSA_SIMD_TRAITS_SIMD_UNITS_IN_RING_ELEMENT ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T ((size_t)13U)

#define LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS_MINUS_ONE_BIT_LENGTH \
  ((size_t)23U)

#define LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_UPPER_PART_OF_T         \
  (LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS_MINUS_ONE_BIT_LENGTH - \
   LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T)

#define LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT ((size_t)256U)

#define LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN ((size_t)255U)

#define LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS ((int32_t)8380417)

#define LIBCRUX_ML_DSA_CONSTANTS_GAMMA2_V261_888 ((int32_t)261888)

#define LIBCRUX_ML_DSA_CONSTANTS_GAMMA2_V95_232 ((int32_t)95232)

typedef int32_t libcrux_ml_dsa_constants_Gamma2;

#define LIBCRUX_ML_DSA_CONSTANTS_KEY_GENERATION_RANDOMNESS_SIZE ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_MASK_SEED_SIZE ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_MESSAGE_REPRESENTATIVE_SIZE ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN ((size_t)814U)

#define LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE \
  (LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T *     \
   LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE \
  (LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_UPPER_PART_OF_T *     \
   LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_ERROR_VECTORS_SIZE ((size_t)64U)

#define LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE ((size_t)32U)

#define LIBCRUX_ML_DSA_CONSTANTS_SIGNING_RANDOMNESS_SIZE ((size_t)32U)

static inline int32_t libcrux_ml_dsa_constants_beta(
    size_t ones_in_verifier_challenge, libcrux_ml_dsa_constants_Eta eta) {
  size_t eta_val;
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      eta_val = (size_t)2U;
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      eta_val = (size_t)4U;
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  return (int32_t)(ones_in_verifier_challenge * eta_val);
}

static inline size_t libcrux_ml_dsa_constants_commitment_ring_element_size(
    size_t bits_per_commitment_coefficient) {
  return bits_per_commitment_coefficient *
         LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U;
}

static inline size_t libcrux_ml_dsa_constants_commitment_vector_size(
    size_t bits_per_commitment_coefficient, size_t rows_in_a) {
  return libcrux_ml_dsa_constants_commitment_ring_element_size(
             bits_per_commitment_coefficient) *
         rows_in_a;
}

static inline size_t libcrux_ml_dsa_constants_error_ring_element_size(
    size_t bits_per_error_coefficient) {
  return bits_per_error_coefficient *
         LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U;
}

static inline size_t libcrux_ml_dsa_constants_gamma1_ring_element_size(
    size_t bits_per_gamma1_coefficient) {
  return bits_per_gamma1_coefficient *
         LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)8U;
}

static inline size_t libcrux_ml_dsa_constants_signature_size(
    size_t rows_in_a, size_t columns_in_a, size_t max_ones_in_hint,
    size_t commitment_hash_size, size_t bits_per_gamma1_coefficient) {
  return commitment_hash_size +
         columns_in_a * libcrux_ml_dsa_constants_gamma1_ring_element_size(
                            bits_per_gamma1_coefficient) +
         max_ones_in_hint + rows_in_a;
}

static inline size_t libcrux_ml_dsa_constants_signing_key_size(
    size_t rows_in_a, size_t columns_in_a, size_t error_ring_element_size) {
  return LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE +
         LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE +
         LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH +
         (rows_in_a + columns_in_a) * error_ring_element_size +
         rows_in_a * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
}

static inline size_t libcrux_ml_dsa_constants_verification_key_size(
    size_t rows_in_a) {
  return LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE +
         LIBCRUX_ML_DSA_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * rows_in_a *
             (LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS_MINUS_ONE_BIT_LENGTH -
              LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T) /
             (size_t)8U;
}

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_COMMITMENT_COEFFICIENT \
  ((size_t)4U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_ERROR_COEFFICIENT \
  ((size_t)4U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_GAMMA1_COEFFICIENT \
  ((size_t)20U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A ((size_t)5U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE ((size_t)48U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA \
  (libcrux_ml_dsa_constants_Eta_Four)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT ((size_t)19U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2 \
  ((LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS - (int32_t)1) / (int32_t)32)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT ((size_t)55U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE \
  ((size_t)49U)

#define LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A ((size_t)6U)

/**
This function found in impl {core::clone::Clone for
libcrux_ml_dsa::constants::Eta}
*/
static inline libcrux_ml_dsa_constants_Eta libcrux_ml_dsa_constants_clone_54(
    const libcrux_ml_dsa_constants_Eta *self) {
  return self[0U];
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_encoding_error_chunk_size(libcrux_ml_dsa_constants_Eta eta) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      return (size_t)4U;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  return (size_t)3U;
}

static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_signature_set_hint(
    Eurydice_dst_ref_mut_59 out_hint, size_t i, size_t j) {
  Eurydice_slice_index_mut(out_hint, i, Eurydice_arr_c3).data[j] = (int32_t)1;
}

#define LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT ((size_t)13U)

#define LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW ((size_t)10U)

#define LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT \
  ((size_t)10U)

typedef struct libcrux_ml_dsa_hash_functions_portable_Shake128X4_s {
  Eurydice_arr_26 state0;
  Eurydice_arr_26 state1;
  Eurydice_arr_26 state2;
  Eurydice_arr_26 state3;
} libcrux_ml_dsa_hash_functions_portable_Shake128X4;

static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake128X4
libcrux_ml_dsa_hash_functions_portable_init_absorb(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  Eurydice_arr_26 state0 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state0, input0);
  Eurydice_arr_26 state1 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state1, input1);
  Eurydice_arr_26 state2 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state2, input2);
  Eurydice_arr_26 state3 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state3, input3);
  return (libcrux_ml_dsa_hash_functions_portable_Shake128X4{state0, state1,
                                                            state2, state3});
}

typedef libcrux_sha3_portable_KeccakState
    libcrux_ml_dsa_hash_functions_portable_Shake256;

static KRML_MUSTINLINE Eurydice_arr_26
libcrux_ml_dsa_hash_functions_portable_init_absorb_final_shake256(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_26 state = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state, input);
  return state;
}

typedef struct libcrux_ml_dsa_hash_functions_portable_Shake256X4_s {
  Eurydice_arr_26 state0;
  Eurydice_arr_26 state1;
  Eurydice_arr_26 state2;
  Eurydice_arr_26 state3;
} libcrux_ml_dsa_hash_functions_portable_Shake256X4;

static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake256X4
libcrux_ml_dsa_hash_functions_portable_init_absorb_x4(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  Eurydice_arr_26 state0 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state0, input0);
  Eurydice_arr_26 state1 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state1, input1);
  Eurydice_arr_26 state2 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state2, input2);
  Eurydice_arr_26 state3 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state3, input3);
  return (libcrux_ml_dsa_hash_functions_portable_Shake256X4{state0, state1,
                                                            state2, state3});
}

static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_portable_shake128(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_portable_shake128(out, input);
}

static KRML_MUSTINLINE Eurydice_arr_3d
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_shake256(
    Eurydice_arr_26 *state) {
  Eurydice_arr_3d out = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
      state, Eurydice_array_to_slice_mut_d4(&out));
  return out;
}

typedef struct Eurydice_arr_uint8_t___136size_t___x4_s {
  Eurydice_arr_3d fst;
  Eurydice_arr_3d snd;
  Eurydice_arr_3d thd;
  Eurydice_arr_3d f3;
} Eurydice_arr_uint8_t___136size_t___x4;

static KRML_MUSTINLINE Eurydice_arr_uint8_t___136size_t___x4
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4(
    libcrux_ml_dsa_hash_functions_portable_Shake256X4 *state) {
  Eurydice_arr_3d out0 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
      &state->state0, Eurydice_array_to_slice_mut_d4(&out0));
  Eurydice_arr_3d out1 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
      &state->state1, Eurydice_array_to_slice_mut_d4(&out1));
  Eurydice_arr_3d out2 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
      &state->state2, Eurydice_array_to_slice_mut_d4(&out2));
  Eurydice_arr_3d out3 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
      &state->state3, Eurydice_array_to_slice_mut_d4(&out3));
  return (Eurydice_arr_uint8_t___136size_t___x4{out0, out1, out2, out3});
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks(
    libcrux_ml_dsa_hash_functions_portable_Shake128X4 *state,
    Eurydice_arr_12 *out0, Eurydice_arr_12 *out1, Eurydice_arr_12 *out2,
    Eurydice_arr_12 *out3) {
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(
      &state->state0, Eurydice_array_to_slice_mut_a8(out0));
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(
      &state->state1, Eurydice_array_to_slice_mut_a8(out1));
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(
      &state->state2, Eurydice_array_to_slice_mut_a8(out2));
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(
      &state->state3, Eurydice_array_to_slice_mut_a8(out3));
}

typedef struct Eurydice_arr_uint8_t___168size_t___x4_s {
  Eurydice_arr_27 fst;
  Eurydice_arr_27 snd;
  Eurydice_arr_27 thd;
  Eurydice_arr_27 f3;
} Eurydice_arr_uint8_t___168size_t___x4;

static KRML_MUSTINLINE Eurydice_arr_uint8_t___168size_t___x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block(
    libcrux_ml_dsa_hash_functions_portable_Shake128X4 *state) {
  Eurydice_arr_27 out0 = {{0U}};
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
      &state->state0, Eurydice_array_to_slice_mut_7b(&out0));
  Eurydice_arr_27 out1 = {{0U}};
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
      &state->state1, Eurydice_array_to_slice_mut_7b(&out1));
  Eurydice_arr_27 out2 = {{0U}};
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
      &state->state2, Eurydice_array_to_slice_mut_7b(&out2));
  Eurydice_arr_27 out3 = {{0U}};
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
      &state->state3, Eurydice_array_to_slice_mut_7b(&out3));
  return (Eurydice_arr_uint8_t___168size_t___x4{out0, out1, out2, out3});
}

static KRML_MUSTINLINE Eurydice_arr_3d
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_shake256(
    Eurydice_arr_26 *state) {
  Eurydice_arr_3d out = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
      state, Eurydice_array_to_slice_mut_d4(&out));
  return out;
}

static KRML_MUSTINLINE Eurydice_arr_uint8_t___136size_t___x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4(
    libcrux_ml_dsa_hash_functions_portable_Shake256X4 *state) {
  Eurydice_arr_3d out0 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
      &state->state0, Eurydice_array_to_slice_mut_d4(&out0));
  Eurydice_arr_3d out1 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
      &state->state1, Eurydice_array_to_slice_mut_d4(&out1));
  Eurydice_arr_3d out2 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
      &state->state2, Eurydice_array_to_slice_mut_d4(&out2));
  Eurydice_arr_3d out3 = {{0U}};
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
      &state->state3, Eurydice_array_to_slice_mut_d4(&out3));
  return (Eurydice_arr_uint8_t___136size_t___x4{out0, out1, out2, out3});
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::Xof for
libcrux_ml_dsa::hash_functions::portable::Shake128}
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_portable_shake128_7b(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_dsa_hash_functions_portable_shake128(input, out);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake128X4}
*/
static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake128X4
libcrux_ml_dsa_hash_functions_portable_init_absorb_11(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  return libcrux_ml_dsa_hash_functions_portable_init_absorb(input0, input1,
                                                            input2, input3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake128X4}
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks_11(
    libcrux_ml_dsa_hash_functions_portable_Shake128X4 *self,
    Eurydice_arr_12 *out0, Eurydice_arr_12 *out1, Eurydice_arr_12 *out2,
    Eurydice_arr_12 *out3) {
  libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks(
      self, out0, out1, out2, out3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake128X4}
*/
static KRML_MUSTINLINE Eurydice_arr_uint8_t___168size_t___x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(
    libcrux_ml_dsa_hash_functions_portable_Shake128X4 *self) {
  return libcrux_ml_dsa_hash_functions_portable_squeeze_next_block(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
static KRML_MUSTINLINE Eurydice_arr_26
libcrux_ml_dsa_hash_functions_portable_init_absorb_final_61(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_dsa_hash_functions_portable_init_absorb_final_shake256(
      input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
static KRML_MUSTINLINE Eurydice_arr_3d
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_61(
    Eurydice_arr_26 *self) {
  return libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_shake256(
      self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
static KRML_MUSTINLINE Eurydice_arr_3d
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_61(
    Eurydice_arr_26 *self) {
  return libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_shake256(
      self);
}

typedef libcrux_sha3_portable_incremental_Shake256Xof
    libcrux_ml_dsa_hash_functions_portable_Shake256Xof;

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for
libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline void libcrux_ml_dsa_hash_functions_portable_absorb_26(
    libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 *self,
    Eurydice_borrow_slice_u8 input) {
  libcrux_sha3_portable_incremental_absorb_42(self, input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for
libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline void libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
    libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 *self,
    Eurydice_borrow_slice_u8 input) {
  libcrux_sha3_portable_incremental_absorb_final_42(self, input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for
libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline libcrux_sha3_generic_keccak_xof_KeccakXofState_e2
libcrux_ml_dsa_hash_functions_portable_init_26(void) {
  return libcrux_sha3_portable_incremental_new_42();
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for
libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline void libcrux_ml_dsa_hash_functions_portable_squeeze_26(
    libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 *self,
    Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_portable_incremental_squeeze_42(self, out);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake256X4
libcrux_ml_dsa_hash_functions_portable_init_absorb_x4_9b(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3) {
  return libcrux_ml_dsa_hash_functions_portable_init_absorb_x4(input0, input1,
                                                               input2, input3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
static KRML_MUSTINLINE Eurydice_arr_uint8_t___136size_t___x4
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4_9b(
    libcrux_ml_dsa_hash_functions_portable_Shake256X4 *self) {
  return libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
static KRML_MUSTINLINE Eurydice_arr_uint8_t___136size_t___x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(
    libcrux_ml_dsa_hash_functions_portable_Shake256X4 *self) {
  return libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4(self);
}

#define LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE128_BLOCK_SIZE ((size_t)168U)

#define LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE128_FIVE_BLOCKS_SIZE \
  (LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE128_BLOCK_SIZE * (size_t)5U)

#define LIBCRUX_ML_DSA_HASH_FUNCTIONS_SHAKE256_BLOCK_SIZE ((size_t)136U)

static KRML_MUSTINLINE Eurydice_arr_a2
libcrux_ml_dsa_sample_add_error_domain_separator(Eurydice_borrow_slice_u8 slice,
                                                 uint16_t domain_separator) {
  Eurydice_arr_a2 out = {{0U}};
  Eurydice_arr_a2 *uu____0 = &out;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_365(
          uu____0, (core_ops_range_Range_08{
                       (size_t)0U, Eurydice_slice_len(slice, uint8_t)})),
      slice, uint8_t);
  out.data[64U] = (uint8_t)domain_separator;
  out.data[65U] = (uint8_t)((uint32_t)domain_separator >> 8U);
  return out;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE \
  (libcrux_ml_dsa_constants_error_ring_element_size(                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_ERROR_COEFFICIENT))

#define LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS ((int32_t)8380417)

#define LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R \
  (58728449ULL)

typedef struct uint8_t_x2_s {
  uint8_t fst;
  uint8_t snd;
} uint8_t_x2;

static inline uint8_t_x2
libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(size_t index,
                                                              size_t width) {
  return (uint8_t_x2{(uint8_t)(index / width), (uint8_t)(index % width)});
}

static KRML_MUSTINLINE uint16_t
libcrux_ml_dsa_sample_generate_domain_separator(uint8_t_x2 _) {
  uint8_t row = _.fst;
  uint8_t column = _.snd;
  return (uint32_t)(uint16_t)column | (uint32_t)(uint16_t)row << 8U;
}

static KRML_MUSTINLINE Eurydice_arr_48
libcrux_ml_dsa_sample_add_domain_separator(Eurydice_borrow_slice_u8 slice,
                                           uint8_t_x2 indices) {
  Eurydice_arr_48 out = {{0U}};
  Eurydice_arr_48 *uu____0 = &out;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_366(
          uu____0, (core_ops_range_Range_08{
                       (size_t)0U, Eurydice_slice_len(slice, uint8_t)})),
      slice, uint8_t);
  uint16_t domain_separator =
      libcrux_ml_dsa_sample_generate_domain_separator(indices);
  out.data[32U] = (uint8_t)domain_separator;
  out.data[33U] = (uint8_t)((uint32_t)domain_separator >> 8U);
  return out;
}

#define None 0
#define Some 1

typedef uint8_t Option_3b_tags;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr uint8_t[[$11size_t]]

*/
typedef struct Option_3b_s {
  Option_3b_tags tag;
  Eurydice_arr_cb f0;
} Option_3b;

typedef struct libcrux_ml_dsa_pre_hash_DomainSeparationContext_s {
  Eurydice_borrow_slice_u8 context;
  Option_3b pre_hash_oid;
} libcrux_ml_dsa_pre_hash_DomainSeparationContext;

#define libcrux_ml_dsa_pre_hash_DomainSeparationError_ContextTooLongError 0

typedef uint8_t libcrux_ml_dsa_pre_hash_DomainSeparationError;

/**
A monomorphic instance of core.result.Result
with types libcrux_ml_dsa_pre_hash_DomainSeparationContext,
libcrux_ml_dsa_pre_hash_DomainSeparationError

*/
typedef struct Result_a8_s {
  Result_a4_tags tag;
  union U {
    libcrux_ml_dsa_pre_hash_DomainSeparationContext case_Ok;
    libcrux_ml_dsa_pre_hash_DomainSeparationError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_a8_s)
} Result_a8;

/**
 `context` must be at most 255 bytes long.
*/
/**
This function found in impl
{libcrux_ml_dsa::pre_hash::DomainSeparationContext<'a>}
*/
static inline Result_a8 libcrux_ml_dsa_pre_hash_new_88(
    Eurydice_borrow_slice_u8 context, Option_3b pre_hash_oid) {
  if (!(Eurydice_slice_len(context, uint8_t) >
        LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN)) {
    return Result_a8_s(Ok, &Result_a8_s::U::case_Ok, {context, pre_hash_oid});
  }
  return Result_a8_s(
      Err, &Result_a8_s::U::case_Err,
      libcrux_ml_dsa_pre_hash_DomainSeparationError_ContextTooLongError);
}

/**
 Returns the pre-hash OID, if any.
*/
/**
This function found in impl
{libcrux_ml_dsa::pre_hash::DomainSeparationContext<'a>}
*/
static inline const Option_3b *libcrux_ml_dsa_pre_hash_pre_hash_oid_88(
    const libcrux_ml_dsa_pre_hash_DomainSeparationContext *self) {
  return &self->pre_hash_oid;
}

/**
 Returns the context, guaranteed to be at most 255 bytes long.
*/
/**
This function found in impl
{libcrux_ml_dsa::pre_hash::DomainSeparationContext<'a>}
*/
static inline Eurydice_borrow_slice_u8 libcrux_ml_dsa_pre_hash_context_88(
    const libcrux_ml_dsa_pre_hash_DomainSeparationContext *self) {
  return self->context;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE \
  (libcrux_ml_dsa_constants_commitment_ring_element_size(                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_COMMITMENT_COEFFICIENT))

static KRML_MUSTINLINE bool libcrux_ml_dsa_sample_inside_out_shuffle(
    Eurydice_borrow_slice_u8 randomness, size_t *out_index, uint64_t *signs,
    Eurydice_arr_c3 *result) {
  bool done = false;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(randomness, uint8_t);
       i++) {
    size_t _cloop_j = i;
    const uint8_t *byte =
        &Eurydice_slice_index_shared(randomness, _cloop_j, uint8_t);
    if (!done) {
      size_t sample_at = (size_t)byte[0U];
      if (sample_at <= out_index[0U]) {
        result->data[out_index[0U]] = result->data[sample_at];
        out_index[0U] = out_index[0U] + (size_t)1U;
        result->data[sample_at] =
            (int32_t)1 - (int32_t)2 * (int32_t)(signs[0U] & 1ULL);
        signs[0U] = signs[0U] >> 1U;
      }
      size_t uu____0 = out_index[0U];
      done = uu____0 == Eurydice_slice_len(
                            Eurydice_array_to_slice_shared_20(result), int32_t);
    }
  }
  return done;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA                 \
  (libcrux_ml_dsa_constants_beta(                                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE, \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA))

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE \
  (libcrux_ml_dsa_constants_gamma1_ring_element_size(                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_GAMMA1_COEFFICIENT))

#define LIBCRUX_ML_DSA_PRE_HASH_SHAKE128_OID \
  ((Eurydice_arr_cb{{6U, 9U, 96U, 134U, 72U, 1U, 101U, 3U, 4U, 2U, 11U}}))

/**
This function found in impl {libcrux_ml_dsa::pre_hash::PreHash for
libcrux_ml_dsa::pre_hash::SHAKE128_PH}
*/
static inline Eurydice_arr_cb libcrux_ml_dsa_pre_hash_oid_30(void) {
  return LIBCRUX_ML_DSA_PRE_HASH_SHAKE128_OID;
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_VERIFICATION_KEY_SIZE \
  (libcrux_ml_dsa_constants_verification_key_size(                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A))

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNATURE_SIZE \
  (libcrux_ml_dsa_constants_signature_size(                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,            \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,         \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,     \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE, \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_GAMMA1_COEFFICIENT))

typedef Eurydice_arr_d4 libcrux_ml_dsa_simd_portable_vector_type_Coefficients;

static KRML_MUSTINLINE Eurydice_arr_d4
libcrux_ml_dsa_simd_portable_vector_type_zero(void) {
  return (Eurydice_arr_d4{{0U}});
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline Eurydice_arr_d4 libcrux_ml_dsa_simd_portable_zero_65(void) {
  return libcrux_ml_dsa_simd_portable_vector_type_zero();
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_vector_type_from_coefficient_array(
    Eurydice_dst_ref_shared_fc array, Eurydice_arr_d4 *out) {
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_a7(out),
      Eurydice_slice_subslice_shared_46(
          array, (core_ops_range_Range_08{
                     (size_t)0U,
                     LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT})),
      int32_t);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_from_coefficient_array_65(
    Eurydice_dst_ref_shared_fc array, Eurydice_arr_d4 *out) {
  libcrux_ml_dsa_simd_portable_vector_type_from_coefficient_array(array, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_vector_type_to_coefficient_array(
    const Eurydice_arr_d4 *value, Eurydice_dst_ref_mut_fc out) {
  Eurydice_slice_copy(out, Eurydice_array_to_slice_shared_a7(value), int32_t);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_to_coefficient_array_65(
    const Eurydice_arr_d4 *value, Eurydice_dst_ref_mut_fc out) {
  libcrux_ml_dsa_simd_portable_vector_type_to_coefficient_array(value, out);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_arithmetic_add(
    Eurydice_arr_d4 *lhs, const Eurydice_arr_d4 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(lhs), int32_t);
       i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs->data[uu____0] = lhs->data[uu____0] + rhs->data[i0];
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_add_65(
    Eurydice_arr_d4 *lhs, const Eurydice_arr_d4 *rhs) {
  libcrux_ml_dsa_simd_portable_arithmetic_add(lhs, rhs);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_arithmetic_subtract(
    Eurydice_arr_d4 *lhs, const Eurydice_arr_d4 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(lhs), int32_t);
       i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs->data[uu____0] = lhs->data[uu____0] - rhs->data[i0];
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_subtract_65(
    Eurydice_arr_d4 *lhs, const Eurydice_arr_d4 *rhs) {
  libcrux_ml_dsa_simd_portable_arithmetic_subtract(lhs, rhs);
}

static KRML_MUSTINLINE bool
libcrux_ml_dsa_simd_portable_arithmetic_infinity_norm_exceeds(
    const Eurydice_arr_d4 *simd_unit, int32_t bound) {
  bool result = false;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t);
       i++) {
    size_t i0 = i;
    int32_t coefficient = simd_unit->data[i0];
    int32_t sign = coefficient >> 31U;
    int32_t normalized = coefficient - (sign & (int32_t)2 * coefficient);
    bool uu____0;
    if (result) {
      uu____0 = true;
    } else {
      uu____0 = normalized >= bound;
    }
    result = uu____0;
  }
  return result;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline bool libcrux_ml_dsa_simd_portable_infinity_norm_exceeds_65(
    const Eurydice_arr_d4 *simd_unit, int32_t bound) {
  return libcrux_ml_dsa_simd_portable_arithmetic_infinity_norm_exceeds(
      simd_unit, bound);
}

typedef struct int32_t_x2_s {
  int32_t fst;
  int32_t snd;
} int32_t_x2;

static KRML_MUSTINLINE int32_t_x2
libcrux_ml_dsa_simd_portable_arithmetic_decompose_element(int32_t gamma2,
                                                          int32_t r) {
  int32_t r0 = r + (r >> 31U & LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  int32_t ceil_of_r_by_128 = (r0 + (int32_t)127) >> 7U;
  int32_t r1;
  switch (gamma2) {
    case 95232: {
      int32_t result =
          (ceil_of_r_by_128 * (int32_t)11275 + ((int32_t)1 << 23U)) >> 24U;
      int32_t result_0 = (result ^ ((int32_t)43 - result) >> 31U) & result;
      r1 = result_0;
      break;
    }
    case 261888: {
      int32_t result =
          (ceil_of_r_by_128 * (int32_t)1025 + ((int32_t)1 << 21U)) >> 22U;
      int32_t result_0 = result & (int32_t)15;
      r1 = result_0;
      break;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  int32_t alpha = gamma2 * (int32_t)2;
  int32_t r00 = r0 - r1 * alpha;
  r00 = r00 -
        (((LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS - (int32_t)1) / (int32_t)2 -
          r00) >>
             31U &
         LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  return (int32_t_x2{r00, r1});
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_arithmetic_decompose(
    int32_t gamma2, const Eurydice_arr_d4 *simd_unit, Eurydice_arr_d4 *low,
    Eurydice_arr_d4 *high) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(low), int32_t);
       i++) {
    size_t i0 = i;
    int32_t_x2 uu____0 =
        libcrux_ml_dsa_simd_portable_arithmetic_decompose_element(
            gamma2, simd_unit->data[i0]);
    int32_t uu____1 = uu____0.snd;
    low->data[i0] = uu____0.fst;
    high->data[i0] = uu____1;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_decompose_65(
    int32_t gamma2, const Eurydice_arr_d4 *simd_unit, Eurydice_arr_d4 *low,
    Eurydice_arr_d4 *high) {
  libcrux_ml_dsa_simd_portable_arithmetic_decompose(gamma2, simd_unit, low,
                                                    high);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_compute_one_hint(int32_t low,
                                                         int32_t high,
                                                         int32_t gamma2) {
  int32_t uu____0;
  if (low > gamma2) {
    uu____0 = (int32_t)1;
  } else if (low < -gamma2) {
    uu____0 = (int32_t)1;
  } else if (low == -gamma2) {
    if (high != (int32_t)0) {
      uu____0 = (int32_t)1;
    } else {
      uu____0 = (int32_t)0;
    }
  } else {
    uu____0 = (int32_t)0;
  }
  return uu____0;
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_arithmetic_compute_hint(
    const Eurydice_arr_d4 *low, const Eurydice_arr_d4 *high, int32_t gamma2,
    Eurydice_arr_d4 *hint) {
  size_t one_hints_count = (size_t)0U;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(hint), int32_t);
       i++) {
    size_t i0 = i;
    hint->data[i0] = libcrux_ml_dsa_simd_portable_arithmetic_compute_one_hint(
        low->data[i0], high->data[i0], gamma2);
    one_hints_count = one_hints_count + (size_t)hint->data[i0];
  }
  return one_hints_count;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t libcrux_ml_dsa_simd_portable_compute_hint_65(
    const Eurydice_arr_d4 *low, const Eurydice_arr_d4 *high, int32_t gamma2,
    Eurydice_arr_d4 *hint) {
  return libcrux_ml_dsa_simd_portable_arithmetic_compute_hint(low, high, gamma2,
                                                              hint);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_use_one_hint(int32_t gamma2, int32_t r,
                                                     int32_t hint) {
  int32_t_x2 uu____0 =
      libcrux_ml_dsa_simd_portable_arithmetic_decompose_element(gamma2, r);
  int32_t r0 = uu____0.fst;
  int32_t r1 = uu____0.snd;
  int32_t uu____1;
  if (!(hint == (int32_t)0)) {
    switch (gamma2) {
      case 95232: {
        if (r0 > (int32_t)0) {
          if (r1 == (int32_t)43) {
            uu____1 = (int32_t)0;
          } else {
            uu____1 = r1 + hint;
          }
        } else if (r1 == (int32_t)0) {
          uu____1 = (int32_t)43;
        } else {
          uu____1 = r1 - hint;
        }
        break;
      }
      case 261888: {
        if (r0 > (int32_t)0) {
          uu____1 = (r1 + hint) & (int32_t)15;
        } else {
          uu____1 = (r1 - hint) & (int32_t)15;
        }
        break;
      }
      default: {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                          "panic!");
        KRML_HOST_EXIT(255U);
      }
    }
    return uu____1;
  }
  return r1;
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_arithmetic_use_hint(
    int32_t gamma2, const Eurydice_arr_d4 *simd_unit, Eurydice_arr_d4 *hint) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(hint), int32_t);
       i++) {
    size_t i0 = i;
    int32_t uu____0 = libcrux_ml_dsa_simd_portable_arithmetic_use_one_hint(
        gamma2, simd_unit->data[i0], hint->data[i0]);
    hint->data[i0] = uu____0;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_use_hint_65(
    int32_t gamma2, const Eurydice_arr_d4 *simd_unit, Eurydice_arr_d4 *hint) {
  libcrux_ml_dsa_simd_portable_arithmetic_use_hint(gamma2, simd_unit, hint);
}

static KRML_MUSTINLINE uint64_t
libcrux_ml_dsa_simd_portable_arithmetic_get_n_least_significant_bits(
    uint8_t n, uint64_t value) {
  return value & ((1ULL << (uint32_t)n) - 1ULL);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT (32U)

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element(
    int64_t value) {
  uint64_t t =
      libcrux_ml_dsa_simd_portable_arithmetic_get_n_least_significant_bits(
          LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT,
          (uint64_t)value) *
      LIBCRUX_ML_DSA_SIMD_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
  int32_t k = (int32_t)
      libcrux_ml_dsa_simd_portable_arithmetic_get_n_least_significant_bits(
          LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT, t);
  int64_t k_times_modulus =
      (int64_t)k * (int64_t)LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS;
  int32_t c =
      (int32_t)(k_times_modulus >>
                (uint32_t)
                    LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT);
  int32_t value_high =
      (int32_t)(value >>
                (uint32_t)
                    LIBCRUX_ML_DSA_SIMD_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT);
  return value_high - c;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply(
    Eurydice_arr_d4 *lhs, const Eurydice_arr_d4 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(lhs), int32_t);
       i++) {
    size_t i0 = i;
    lhs->data[i0] =
        libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element(
            (int64_t)lhs->data[i0] * (int64_t)rhs->data[i0]);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_montgomery_multiply_65(
    Eurydice_arr_d4 *lhs, const Eurydice_arr_d4 *rhs) {
  libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply(lhs, rhs);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_reduce_element(int32_t fe) {
  int32_t quotient = (fe + ((int32_t)1 << 22U)) >> 23U;
  return fe - quotient * LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS;
}

static KRML_MUSTINLINE int32_t_x2
libcrux_ml_dsa_simd_portable_arithmetic_power2round_element(int32_t t) {
  int32_t t2 = t + (t >> 31U & LIBCRUX_ML_DSA_SIMD_TRAITS_FIELD_MODULUS);
  int32_t t1 =
      (t2 - (int32_t)1 +
       ((int32_t)1
        << (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T -
                      (size_t)1U))) >>
      (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T;
  int32_t t0 =
      t2 - (t1 << (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T);
  return (int32_t_x2{t0, t1});
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_arithmetic_power2round(
    Eurydice_arr_d4 *t0, Eurydice_arr_d4 *t1) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(t0), int32_t);
       i++) {
    size_t i0 = i;
    int32_t_x2 uu____0 =
        libcrux_ml_dsa_simd_portable_arithmetic_power2round_element(
            t0->data[i0]);
    int32_t uu____1 = uu____0.snd;
    t0->data[i0] = uu____0.fst;
    t1->data[i0] = uu____1;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_power2round_65(
    Eurydice_arr_d4 *t0, Eurydice_arr_d4 *t1) {
  libcrux_ml_dsa_simd_portable_arithmetic_power2round(t0, t1);
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_field_modulus(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_fc out) {
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(randomness, uint8_t) / (size_t)3U; i++) {
    size_t i0 = i;
    int32_t b0 = (int32_t)Eurydice_slice_index_shared(randomness,
                                                      i0 * (size_t)3U, uint8_t);
    int32_t b1 = (int32_t)Eurydice_slice_index_shared(
        randomness, i0 * (size_t)3U + (size_t)1U, uint8_t);
    int32_t b2 = (int32_t)Eurydice_slice_index_shared(
        randomness, i0 * (size_t)3U + (size_t)2U, uint8_t);
    int32_t coefficient = ((b2 << 16U | b1 << 8U) | b0) & (int32_t)8388607;
    if (coefficient < LIBCRUX_ML_DSA_CONSTANTS_FIELD_MODULUS) {
      Eurydice_slice_index_mut(out, sampled, int32_t) = coefficient;
      sampled++;
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_rejection_sample_less_than_field_modulus_65(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_fc out) {
  return libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_field_modulus(
      randomness, out);
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_2(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_fc out) {
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(randomness, uint8_t);
       i++) {
    size_t i0 = i;
    uint8_t byte = Eurydice_slice_index_shared(randomness, i0, uint8_t);
    uint8_t try_0 = (uint32_t)byte & 15U;
    uint8_t try_1 = (uint32_t)byte >> 4U;
    if (try_0 < 15U) {
      int32_t try_00 = (int32_t)try_0;
      int32_t try_0_mod_5 = try_00 - (try_00 * (int32_t)26 >> 7U) * (int32_t)5;
      Eurydice_slice_index_mut(out, sampled, int32_t) =
          (int32_t)2 - try_0_mod_5;
      sampled++;
    }
    if (try_1 < 15U) {
      int32_t try_10 = (int32_t)try_1;
      int32_t try_1_mod_5 = try_10 - (try_10 * (int32_t)26 >> 7U) * (int32_t)5;
      Eurydice_slice_index_mut(out, sampled, int32_t) =
          (int32_t)2 - try_1_mod_5;
      sampled++;
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_2_65(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_fc out) {
  return libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_2(
      randomness, out);
}

static KRML_MUSTINLINE size_t
libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_4(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_fc out) {
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(randomness, uint8_t);
       i++) {
    size_t i0 = i;
    uint8_t byte = Eurydice_slice_index_shared(randomness, i0, uint8_t);
    uint8_t try_0 = (uint32_t)byte & 15U;
    uint8_t try_1 = (uint32_t)byte >> 4U;
    if (try_0 < 9U) {
      Eurydice_slice_index_mut(out, sampled, int32_t) =
          (int32_t)4 - (int32_t)try_0;
      sampled++;
    }
    if (try_1 < 9U) {
      Eurydice_slice_index_mut(out, sampled, int32_t) =
          (int32_t)4 - (int32_t)try_1;
      sampled++;
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline size_t
libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_4_65(
    Eurydice_borrow_slice_u8 randomness, Eurydice_dst_ref_mut_fc out) {
  return libcrux_ml_dsa_simd_portable_sample_rejection_sample_less_than_eta_equals_4(
      randomness, out);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 \
  ((int32_t)1 << 19U)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t) /
               (size_t)2U;
       i++) {
    size_t i0 = i;
    Eurydice_dst_ref_shared_fc coefficients =
        Eurydice_array_to_subslice_shared_7f(
            simd_unit, (core_ops_range_Range_08{i0 * (size_t)2U,
                                                i0 * (size_t)2U + (size_t)2U}));
    int32_t coefficient0 =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        Eurydice_slice_index_shared(coefficients, (size_t)0U, int32_t);
    int32_t coefficient1 =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        Eurydice_slice_index_shared(coefficients, (size_t)1U, int32_t);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0, uint8_t) =
        (uint8_t)coefficient0;
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)1U,
                             uint8_t) = (uint8_t)(coefficient0 >> 8U);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)2U,
                             uint8_t) = (uint8_t)(coefficient0 >> 16U);
    size_t uu____0 = (size_t)5U * i0 + (size_t)2U;
    Eurydice_slice_index_mut(serialized, uu____0, uint8_t) =
        (uint32_t)Eurydice_slice_index_shared(
            (Eurydice_borrow_slice_u8{serialized.ptr, serialized.meta}),
            uu____0, uint8_t) |
        (uint32_t)(uint8_t)(coefficient1 << 4U);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)3U,
                             uint8_t) = (uint8_t)(coefficient1 >> 4U);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)4U,
                             uint8_t) = (uint8_t)(coefficient1 >> 12U);
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 \
  ((int32_t)1 << 17U)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t) /
               (size_t)4U;
       i++) {
    size_t i0 = i;
    Eurydice_dst_ref_shared_fc coefficients =
        Eurydice_array_to_subslice_shared_7f(
            simd_unit, (core_ops_range_Range_08{i0 * (size_t)4U,
                                                i0 * (size_t)4U + (size_t)4U}));
    int32_t coefficient0 =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        Eurydice_slice_index_shared(coefficients, (size_t)0U, int32_t);
    int32_t coefficient1 =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        Eurydice_slice_index_shared(coefficients, (size_t)1U, int32_t);
    int32_t coefficient2 =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        Eurydice_slice_index_shared(coefficients, (size_t)2U, int32_t);
    int32_t coefficient3 =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_SERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 -
        Eurydice_slice_index_shared(coefficients, (size_t)3U, int32_t);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0, uint8_t) =
        (uint8_t)coefficient0;
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)1U,
                             uint8_t) = (uint8_t)(coefficient0 >> 8U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)2U,
                             uint8_t) = (uint8_t)(coefficient0 >> 16U);
    size_t uu____0 = (size_t)9U * i0 + (size_t)2U;
    Eurydice_slice_index_mut(serialized, uu____0, uint8_t) =
        (uint32_t)Eurydice_slice_index_shared(
            (Eurydice_borrow_slice_u8{serialized.ptr, serialized.meta}),
            uu____0, uint8_t) |
        (uint32_t)(uint8_t)(coefficient1 << 2U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)3U,
                             uint8_t) = (uint8_t)(coefficient1 >> 6U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)4U,
                             uint8_t) = (uint8_t)(coefficient1 >> 14U);
    size_t uu____1 = (size_t)9U * i0 + (size_t)4U;
    Eurydice_slice_index_mut(serialized, uu____1, uint8_t) =
        (uint32_t)Eurydice_slice_index_shared(
            (Eurydice_borrow_slice_u8{serialized.ptr, serialized.meta}),
            uu____1, uint8_t) |
        (uint32_t)(uint8_t)(coefficient2 << 4U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)5U,
                             uint8_t) = (uint8_t)(coefficient2 >> 4U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)6U,
                             uint8_t) = (uint8_t)(coefficient2 >> 12U);
    size_t uu____2 = (size_t)9U * i0 + (size_t)6U;
    Eurydice_slice_index_mut(serialized, uu____2, uint8_t) =
        (uint32_t)Eurydice_slice_index_shared(
            (Eurydice_borrow_slice_u8{serialized.ptr, serialized.meta}),
            uu____2, uint8_t) |
        (uint32_t)(uint8_t)(coefficient3 << 6U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)7U,
                             uint8_t) = (uint8_t)(coefficient3 >> 2U);
    Eurydice_slice_index_mut(serialized, (size_t)9U * i0 + (size_t)8U,
                             uint8_t) = (uint8_t)(coefficient3 >> 10U);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized,
    size_t gamma1_exponent) {
  switch (gamma1_exponent) {
    case 17U: {
      break;
    }
    case 19U: {
      libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_19(
          simd_unit, serialized);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize_when_gamma1_is_2_pow_17(
      simd_unit, serialized);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_gamma1_serialize_65(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized,
    size_t gamma1_exponent) {
  libcrux_ml_dsa_simd_portable_encoding_gamma1_serialize(simd_unit, serialized,
                                                         gamma1_exponent);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 \
  ((int32_t)1 << 19U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1_TIMES_2_BITMASK \
  ((LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1                     \
    << 1U) -                                                                                                    \
   (int32_t)1)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *simd_unit) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)5U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)5U,
                                             i0 * (size_t)5U + (size_t)5U}));
    int32_t coefficient0 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t);
    coefficient0 =
        coefficient0 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t) << 8U;
    coefficient0 =
        coefficient0 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t) << 16U;
    coefficient0 =
        coefficient0 &
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient1 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t) >> 4U;
    coefficient1 =
        coefficient1 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t) << 4U;
    coefficient1 =
        coefficient1 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)4U, uint8_t) << 12U;
    simd_unit->data[(size_t)2U * i0] =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        coefficient0;
    simd_unit->data[(size_t)2U * i0 + (size_t)1U] =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_19_GAMMA1 -
        coefficient1;
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1 \
  ((int32_t)1 << 17U)

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK \
  ((LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1                     \
    << 1U) -                                                                                                    \
   (int32_t)1)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *simd_unit) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)9U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)9U,
                                             i0 * (size_t)9U + (size_t)9U}));
    int32_t coefficient0 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t);
    coefficient0 =
        coefficient0 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t) << 8U;
    coefficient0 =
        coefficient0 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t) << 16U;
    coefficient0 =
        coefficient0 &
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient1 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t) >> 2U;
    coefficient1 =
        coefficient1 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t) << 6U;
    coefficient1 =
        coefficient1 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)4U, uint8_t) << 14U;
    coefficient1 =
        coefficient1 &
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient2 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)4U, uint8_t) >> 4U;
    coefficient2 =
        coefficient2 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)5U, uint8_t) << 4U;
    coefficient2 =
        coefficient2 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)6U, uint8_t) << 12U;
    coefficient2 =
        coefficient2 &
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_GAMMA1_DESERIALIZE_WHEN_GAMMA1_IS_2_POW_17_GAMMA1_TIMES_2_BITMASK;
    int32_t coefficient3 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)6U, uint8_t) >> 6U;
    coefficient3 =
        coefficient3 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)7U, uint8_t) << 2U;
    coefficient3 =
        coefficient3 |
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)8U, uint8_t) << 10U;
    coefficient3 =
        coefficient3 &
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
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *out,
    size_t gamma1_exponent) {
  switch (gamma1_exponent) {
    case 17U: {
      break;
    }
    case 19U: {
      libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_19(
          serialized, out);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize_when_gamma1_is_2_pow_17(
      serialized, out);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_gamma1_deserialize_65(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *out,
    size_t gamma1_exponent) {
  libcrux_ml_dsa_simd_portable_encoding_gamma1_deserialize(serialized, out,
                                                           gamma1_exponent);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_4(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
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
  Eurydice_slice_index_mut(serialized, (size_t)0U, uint8_t) = byte0;
  Eurydice_slice_index_mut(serialized, (size_t)1U, uint8_t) = byte1;
  Eurydice_slice_index_mut(serialized, (size_t)2U, uint8_t) = byte2;
  Eurydice_slice_index_mut(serialized, (size_t)3U, uint8_t) = byte3;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_6(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
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
  Eurydice_slice_index_mut(serialized, (size_t)0U, uint8_t) = byte0;
  Eurydice_slice_index_mut(serialized, (size_t)1U, uint8_t) = byte1;
  Eurydice_slice_index_mut(serialized, (size_t)2U, uint8_t) = byte2;
  Eurydice_slice_index_mut(serialized, (size_t)3U, uint8_t) = byte3;
  Eurydice_slice_index_mut(serialized, (size_t)4U, uint8_t) = byte4;
  Eurydice_slice_index_mut(serialized, (size_t)5U, uint8_t) = byte5;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_commitment_serialize(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  switch ((uint8_t)Eurydice_slice_len(
      (Eurydice_borrow_slice_u8{serialized.ptr, serialized.meta}), uint8_t)) {
    case 4U: {
      libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_4(simd_unit,
                                                                   serialized);
      break;
    }
    case 6U: {
      libcrux_ml_dsa_simd_portable_encoding_commitment_serialize_6(simd_unit,
                                                                   serialized);
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
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_commitment_serialize_65(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  libcrux_ml_dsa_simd_portable_encoding_commitment_serialize(simd_unit,
                                                             serialized);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_4_ETA \
  ((int32_t)4)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_4(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t) /
               (size_t)2U;
       i++) {
    size_t i0 = i;
    Eurydice_dst_ref_shared_fc coefficients =
        Eurydice_array_to_subslice_shared_7f(
            simd_unit, (core_ops_range_Range_08{i0 * (size_t)2U,
                                                i0 * (size_t)2U + (size_t)2U}));
    uint8_t coefficient0 =
        (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_4_ETA -
                  Eurydice_slice_index_shared(coefficients, (size_t)0U,
                                              int32_t));
    uint8_t coefficient1 =
        (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_4_ETA -
                  Eurydice_slice_index_shared(coefficients, (size_t)1U,
                                              int32_t));
    Eurydice_slice_index_mut(serialized, i0, uint8_t) =
        (uint32_t)coefficient1 << 4U | (uint32_t)coefficient0;
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA \
  ((int32_t)2)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_2(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  uint8_t coefficient0 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[0U]);
  uint8_t coefficient1 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[1U]);
  uint8_t coefficient2 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[2U]);
  uint8_t coefficient3 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[3U]);
  uint8_t coefficient4 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[4U]);
  uint8_t coefficient5 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[5U]);
  uint8_t coefficient6 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[6U]);
  uint8_t coefficient7 =
      (uint8_t)(LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_SERIALIZE_WHEN_ETA_IS_2_ETA -
                simd_unit->data[7U]);
  Eurydice_slice_index_mut(serialized, (size_t)0U, uint8_t) =
      ((uint32_t)coefficient2 << 6U | (uint32_t)coefficient1 << 3U) |
      (uint32_t)coefficient0;
  Eurydice_slice_index_mut(serialized, (size_t)1U, uint8_t) =
      (((uint32_t)coefficient5 << 7U | (uint32_t)coefficient4 << 4U) |
       (uint32_t)coefficient3 << 1U) |
      (uint32_t)coefficient2 >> 2U;
  Eurydice_slice_index_mut(serialized, (size_t)2U, uint8_t) =
      ((uint32_t)coefficient7 << 5U | (uint32_t)coefficient6 << 2U) |
      (uint32_t)coefficient5 >> 1U;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_serialize(
    libcrux_ml_dsa_constants_Eta eta, const Eurydice_arr_d4 *simd_unit,
    Eurydice_mut_borrow_slice_u8 serialized) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_4(
          simd_unit, serialized);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  libcrux_ml_dsa_simd_portable_encoding_error_serialize_when_eta_is_2(
      simd_unit, serialized);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_error_serialize_65(
    libcrux_ml_dsa_constants_Eta eta, const Eurydice_arr_d4 *simd_unit,
    Eurydice_mut_borrow_slice_u8 serialized) {
  libcrux_ml_dsa_simd_portable_encoding_error_serialize(eta, simd_unit,
                                                        serialized);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_4_ETA \
  ((int32_t)4)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_4(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *simd_units) {
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(serialized, uint8_t);
       i++) {
    size_t i0 = i;
    const uint8_t *byte = &Eurydice_slice_index_shared(serialized, i0, uint8_t);
    uint8_t uu____0 =
        core_ops_bit__core__ops__bit__BitAnd_u8__u8__for__0__u8___bitand(byte,
                                                                         15U);
    simd_units->data[(size_t)2U * i0] =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_4_ETA -
        (int32_t)uu____0;
    uint8_t uu____1 =
        core_ops_bit__core__ops__bit__Shr_i32__u8__for__0__u8___shr(byte,
                                                                    (int32_t)4);
    simd_units->data[(size_t)2U * i0 + (size_t)1U] =
        LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_4_ETA -
        (int32_t)uu____1;
  }
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA \
  ((int32_t)2)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_2(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *simd_unit) {
  int32_t byte0 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)0U, uint8_t);
  int32_t byte1 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)1U, uint8_t);
  int32_t byte2 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)2U, uint8_t);
  simd_unit->data[0U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      (byte0 & (int32_t)7);
  simd_unit->data[1U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      (byte0 >> 3U & (int32_t)7);
  simd_unit->data[2U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      ((byte0 >> 6U | byte1 << 2U) & (int32_t)7);
  simd_unit->data[3U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      (byte1 >> 1U & (int32_t)7);
  simd_unit->data[4U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      (byte1 >> 4U & (int32_t)7);
  simd_unit->data[5U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      ((byte1 >> 7U | byte2 << 1U) & (int32_t)7);
  simd_unit->data[6U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      (byte2 >> 2U & (int32_t)7);
  simd_unit->data[7U] =
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_ERROR_DESERIALIZE_WHEN_ETA_IS_2_ETA -
      (byte2 >> 5U & (int32_t)7);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_error_deserialize(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized,
    Eurydice_arr_d4 *out) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_4(
          serialized, out);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  libcrux_ml_dsa_simd_portable_encoding_error_deserialize_when_eta_is_2(
      serialized, out);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_error_deserialize_65(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized,
    Eurydice_arr_d4 *out) {
  libcrux_ml_dsa_simd_portable_encoding_error_deserialize(eta, serialized, out);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(int32_t t0) {
  return ((int32_t)1
          << (uint32_t)(LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T -
                        (size_t)1U)) -
         t0;
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_encoding_t0_serialize(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  int32_t coefficient0 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[0U]);
  int32_t coefficient1 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[1U]);
  int32_t coefficient2 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[2U]);
  int32_t coefficient3 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[3U]);
  int32_t coefficient4 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[4U]);
  int32_t coefficient5 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[5U]);
  int32_t coefficient6 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[6U]);
  int32_t coefficient7 =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(
          simd_unit->data[7U]);
  Eurydice_slice_index_mut(serialized, (size_t)0U, uint8_t) =
      (uint8_t)coefficient0;
  Eurydice_slice_index_mut(serialized, (size_t)1U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient0 >> 8U) |
      (uint32_t)(uint8_t)(coefficient1 << 5U);
  Eurydice_slice_index_mut(serialized, (size_t)2U, uint8_t) =
      (uint8_t)(coefficient1 >> 3U);
  Eurydice_slice_index_mut(serialized, (size_t)3U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient1 >> 11U) |
      (uint32_t)(uint8_t)(coefficient2 << 2U);
  Eurydice_slice_index_mut(serialized, (size_t)4U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient2 >> 6U) |
      (uint32_t)(uint8_t)(coefficient3 << 7U);
  Eurydice_slice_index_mut(serialized, (size_t)5U, uint8_t) =
      (uint8_t)(coefficient3 >> 1U);
  Eurydice_slice_index_mut(serialized, (size_t)6U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient3 >> 9U) |
      (uint32_t)(uint8_t)(coefficient4 << 4U);
  Eurydice_slice_index_mut(serialized, (size_t)7U, uint8_t) =
      (uint8_t)(coefficient4 >> 4U);
  Eurydice_slice_index_mut(serialized, (size_t)8U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient4 >> 12U) |
      (uint32_t)(uint8_t)(coefficient5 << 1U);
  Eurydice_slice_index_mut(serialized, (size_t)9U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient5 >> 7U) |
      (uint32_t)(uint8_t)(coefficient6 << 6U);
  Eurydice_slice_index_mut(serialized, (size_t)10U, uint8_t) =
      (uint8_t)(coefficient6 >> 2U);
  Eurydice_slice_index_mut(serialized, (size_t)11U, uint8_t) =
      (uint32_t)(uint8_t)(coefficient6 >> 10U) |
      (uint32_t)(uint8_t)(coefficient7 << 3U);
  Eurydice_slice_index_mut(serialized, (size_t)12U, uint8_t) =
      (uint8_t)(coefficient7 >> 5U);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_t0_serialize_65(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_dsa_simd_portable_encoding_t0_serialize(simd_unit, out);
}

#define LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK \
  (((int32_t)1 << (uint32_t)(int32_t)                                                     \
        LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_LOWER_PART_OF_T) -                               \
   (int32_t)1)

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_t0_deserialize(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *simd_unit) {
  int32_t byte0 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)0U, uint8_t);
  int32_t byte1 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)1U, uint8_t);
  int32_t byte2 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)2U, uint8_t);
  int32_t byte3 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)3U, uint8_t);
  int32_t byte4 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)4U, uint8_t);
  int32_t byte5 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)5U, uint8_t);
  int32_t byte6 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)6U, uint8_t);
  int32_t byte7 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)7U, uint8_t);
  int32_t byte8 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)8U, uint8_t);
  int32_t byte9 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)9U, uint8_t);
  int32_t byte10 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)10U, uint8_t);
  int32_t byte11 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)11U, uint8_t);
  int32_t byte12 =
      (int32_t)Eurydice_slice_index_shared(serialized, (size_t)12U, uint8_t);
  int32_t coefficient0 = byte0;
  coefficient0 = coefficient0 | byte1 << 8U;
  coefficient0 =
      coefficient0 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient1 = byte1 >> 5U;
  coefficient1 = coefficient1 | byte2 << 3U;
  coefficient1 = coefficient1 | byte3 << 11U;
  coefficient1 =
      coefficient1 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient2 = byte3 >> 2U;
  coefficient2 = coefficient2 | byte4 << 6U;
  coefficient2 =
      coefficient2 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient3 = byte4 >> 7U;
  coefficient3 = coefficient3 | byte5 << 1U;
  coefficient3 = coefficient3 | byte6 << 9U;
  coefficient3 =
      coefficient3 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient4 = byte6 >> 4U;
  coefficient4 = coefficient4 | byte7 << 4U;
  coefficient4 = coefficient4 | byte8 << 12U;
  coefficient4 =
      coefficient4 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient5 = byte8 >> 1U;
  coefficient5 = coefficient5 | byte9 << 7U;
  coefficient5 =
      coefficient5 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient6 = byte9 >> 6U;
  coefficient6 = coefficient6 | byte10 << 2U;
  coefficient6 = coefficient6 | byte11 << 10U;
  coefficient6 =
      coefficient6 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  int32_t coefficient7 = byte11 >> 3U;
  coefficient7 = coefficient7 | byte12 << 5U;
  coefficient7 =
      coefficient7 &
      LIBCRUX_ML_DSA_SIMD_PORTABLE_ENCODING_T0_DESERIALIZE_BITS_IN_LOWER_PART_OF_T_MASK;
  simd_unit->data[0U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient0);
  simd_unit->data[1U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient1);
  simd_unit->data[2U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient2);
  simd_unit->data[3U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient3);
  simd_unit->data[4U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient4);
  simd_unit->data[5U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient5);
  simd_unit->data[6U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient6);
  simd_unit->data[7U] =
      libcrux_ml_dsa_simd_portable_encoding_t0_change_t0_interval(coefficient7);
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_t0_deserialize_65(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *out) {
  libcrux_ml_dsa_simd_portable_encoding_t0_deserialize(serialized, out);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_encoding_t1_serialize(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t) /
               (size_t)4U;
       i++) {
    size_t i0 = i;
    Eurydice_dst_ref_shared_fc coefficients =
        Eurydice_array_to_subslice_shared_7f(
            simd_unit, (core_ops_range_Range_08{i0 * (size_t)4U,
                                                i0 * (size_t)4U + (size_t)4U}));
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0, uint8_t) =
        (uint8_t)(Eurydice_slice_index_shared(coefficients, (size_t)0U,
                                              int32_t) &
                  (int32_t)255);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)1U,
                             uint8_t) =
        (uint32_t)(uint8_t)(Eurydice_slice_index_shared(coefficients,
                                                        (size_t)1U, int32_t) &
                            (int32_t)63)
            << 2U |
        (uint32_t)(uint8_t)(Eurydice_slice_index_shared(coefficients,
                                                        (size_t)0U, int32_t) >>
                                8U &
                            (int32_t)3);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)2U,
                             uint8_t) =
        (uint32_t)(uint8_t)(Eurydice_slice_index_shared(coefficients,
                                                        (size_t)2U, int32_t) &
                            (int32_t)15)
            << 4U |
        (uint32_t)(uint8_t)(Eurydice_slice_index_shared(coefficients,
                                                        (size_t)1U, int32_t) >>
                                6U &
                            (int32_t)15);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)3U,
                             uint8_t) =
        (uint32_t)(uint8_t)(Eurydice_slice_index_shared(coefficients,
                                                        (size_t)3U, int32_t) &
                            (int32_t)3)
            << 6U |
        (uint32_t)(uint8_t)(Eurydice_slice_index_shared(coefficients,
                                                        (size_t)2U, int32_t) >>
                                4U &
                            (int32_t)63);
    Eurydice_slice_index_mut(serialized, (size_t)5U * i0 + (size_t)4U,
                             uint8_t) =
        (uint8_t)(Eurydice_slice_index_shared(coefficients, (size_t)3U,
                                              int32_t) >>
                      2U &
                  (int32_t)255);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_t1_serialize_65(
    const Eurydice_arr_d4 *simd_unit, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_dsa_simd_portable_encoding_t1_serialize(simd_unit, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_encoding_t1_deserialize(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *simd_unit) {
  int32_t mask = ((int32_t)1 << (uint32_t)
                      LIBCRUX_ML_DSA_CONSTANTS_BITS_IN_UPPER_PART_OF_T) -
                 (int32_t)1;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)5U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)5U,
                                             i0 * (size_t)5U + (size_t)5U}));
    int32_t byte0 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t);
    int32_t byte1 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t);
    int32_t byte2 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t);
    int32_t byte3 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t);
    int32_t byte4 =
        (int32_t)Eurydice_slice_index_shared(bytes, (size_t)4U, uint8_t);
    simd_unit->data[(size_t)4U * i0] = (byte0 | byte1 << 8U) & mask;
    simd_unit->data[(size_t)4U * i0 + (size_t)1U] =
        (byte1 >> 2U | byte2 << 6U) & mask;
    simd_unit->data[(size_t)4U * i0 + (size_t)2U] =
        (byte2 >> 4U | byte3 << 4U) & mask;
    simd_unit->data[(size_t)4U * i0 + (size_t)3U] =
        (byte3 >> 6U | byte4 << 2U) & mask;
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_t1_deserialize_65(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_d4 *out) {
  libcrux_ml_dsa_simd_portable_encoding_t1_deserialize(serialized, out);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
    Eurydice_arr_d4 *simd_unit, int32_t c) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t);
       i++) {
    size_t i0 = i;
    simd_unit->data[i0] =
        libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element(
            (int64_t)simd_unit->data[i0] * (int64_t)c);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_a4_s {
  Eurydice_arr_d4 data[32U];
} Eurydice_arr_a4;

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(
    Eurydice_arr_a4 *re, size_t index, size_t step_by, int32_t zeta) {
  Eurydice_arr_d4 tmp = re->data[index + step_by];
  libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(&tmp,
                                                                          zeta);
  re->data[index + step_by] = re->data[index];
  libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[index + step_by],
                                                   &tmp);
  libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[index], &tmp);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 16
- ZETA= 25847
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_99(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)16U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)16U,
                                                        (int32_t)25847);
  }
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_7(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_99(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 8
- ZETA= -2608894
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_990(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)8U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)8U,
                                                        (int32_t)-2608894);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 8
- ZETA= -518909
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)8U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)8U,
                                                        (int32_t)-518909);
  }
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_6(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_990(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 4
- ZETA= 237124
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_991(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)4U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U,
                                                        (int32_t)237124);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 4
- ZETA= -777960
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a8(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)4U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U,
                                                        (int32_t)-777960);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 4
- ZETA= -876248
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a0(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)4U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U,
                                                        (int32_t)-876248);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 4
- ZETA= 466468
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d9(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)4U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)4U,
                                                        (int32_t)466468);
  }
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_5(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_991(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a8(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a0(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d9(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 2
- ZETA= 1826347
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_992(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)1826347);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 2
- ZETA= 2353451
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_6b(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)2353451);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 2
- ZETA= -359251
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a80(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)-359251);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 2
- ZETA= -2091905
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_95(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)-2091905);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 2
- ZETA= 3119733
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a1(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)3119733);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 2
- ZETA= -2884855
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_de(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)-2884855);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 2
- ZETA= 3111497
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d90(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)3111497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 2
- ZETA= 2680103
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3b(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)2U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)2U,
                                                        (int32_t)2680103);
  }
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_4(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_992(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_6b(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a80(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_95(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a1(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_de(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d90(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3b(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 1
- ZETA= 2725464
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_993(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)2725464);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 2
- STEP_BY= 1
- ZETA= 1024112
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_1c(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)2U; i < (size_t)2U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)1024112);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 1
- ZETA= -1079900
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_6b0(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-1079900);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 6
- STEP_BY= 1
- ZETA= 3585928
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_44(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)6U; i < (size_t)6U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)3585928);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 1
- ZETA= -549488
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a81(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-549488);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 10
- STEP_BY= 1
- ZETA= -1119584
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_1f(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)10U; i < (size_t)10U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-1119584);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 1
- ZETA= 2619752
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_950(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)2619752);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 14
- STEP_BY= 1
- ZETA= -2108549
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3b0(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)14U; i < (size_t)14U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-2108549);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 1
- ZETA= -2118186
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a2(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-2118186);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 18
- STEP_BY= 1
- ZETA= -3859737
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_e4(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)18U; i < (size_t)18U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-3859737);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 1
- ZETA= -1399561
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_de0(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-1399561);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 22
- STEP_BY= 1
- ZETA= -3277672
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_05(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)22U; i < (size_t)22U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-3277672);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 1
- ZETA= 1757237
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d91(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)1757237);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 26
- STEP_BY= 1
- ZETA= -19422
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3a(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)26U; i < (size_t)26U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)-19422);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 28
- STEP_BY= 1
- ZETA= 4010497
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3b1(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)4010497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.ntt.outer_3_plus
with const generics
- OFFSET= 30
- STEP_BY= 1
- ZETA= 280005
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a0(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)30U; i < (size_t)30U + (size_t)1U; i++) {
    size_t j = i;
    libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_round(re, j, (size_t)1U,
                                                        (int32_t)280005);
  }
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_3(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_993(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_1c(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_6b0(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_44(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a81(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_1f(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_950(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3b0(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_7a2(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_e4(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_de0(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_05(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_d91(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3a(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_3b1(re);
  libcrux_ml_dsa_simd_portable_ntt_outer_3_plus_a0(re);
}

static KRML_MUSTINLINE int32_t
libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_fe_by_fer(
    int32_t fe, int32_t fer) {
  return libcrux_ml_dsa_simd_portable_arithmetic_montgomery_reduce_element(
      (int64_t)fe * (int64_t)fer);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(
    Eurydice_arr_d4 *simd_unit, int32_t zeta, size_t index, size_t step) {
  int32_t t =
      libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_fe_by_fer(
          simd_unit->data[index + step], zeta);
  simd_unit->data[index + step] = simd_unit->data[index] - t;
  simd_unit->data[index] = simd_unit->data[index] + t;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_2(
    Eurydice_arr_d4 *simd_unit, int32_t zeta) {
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta,
                                                      (size_t)0U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta,
                                                      (size_t)1U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta,
                                                      (size_t)2U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta,
                                                      (size_t)3U, (size_t)4U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(Eurydice_arr_a4 *re,
                                                      size_t index,
                                                      int32_t zeta) {
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_2(&re->data[index],
                                                            zeta);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)0U,
                                                        (int32_t)2706023);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)1U,
                                                        (int32_t)95776);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)2U,
                                                        (int32_t)3077325);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)3U,
                                                        (int32_t)3530437);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)4U,
                                                        (int32_t)-1661693);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)5U,
                                                        (int32_t)-3592148);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)6U,
                                                        (int32_t)-2537516);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)7U,
                                                        (int32_t)3915439);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)8U,
                                                        (int32_t)-3861115);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)9U,
                                                        (int32_t)-3043716);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)10U,
                                                        (int32_t)3574422);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)11U,
                                                        (int32_t)-2867647);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)12U,
                                                        (int32_t)3539968);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)13U,
                                                        (int32_t)-300467);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)14U,
                                                        (int32_t)2348700);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)15U,
                                                        (int32_t)-539299);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)16U,
                                                        (int32_t)-1699267);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)17U,
                                                        (int32_t)-1643818);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)18U,
                                                        (int32_t)3505694);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)19U,
                                                        (int32_t)-3821735);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)20U,
                                                        (int32_t)3507263);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)21U,
                                                        (int32_t)-2140649);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)22U,
                                                        (int32_t)-1600420);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)23U,
                                                        (int32_t)3699596);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)24U,
                                                        (int32_t)811944);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)25U,
                                                        (int32_t)531354);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)26U,
                                                        (int32_t)954230);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)27U,
                                                        (int32_t)3881043);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)28U,
                                                        (int32_t)3900724);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)29U,
                                                        (int32_t)-2556880);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)30U,
                                                        (int32_t)2071892);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_2_round(re, (size_t)31U,
                                                        (int32_t)-2797779);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_1(
    Eurydice_arr_d4 *simd_unit, int32_t zeta1, int32_t zeta2) {
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta1,
                                                      (size_t)0U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta1,
                                                      (size_t)1U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta2,
                                                      (size_t)4U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta2,
                                                      (size_t)5U, (size_t)2U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(Eurydice_arr_a4 *re,
                                                      size_t index,
                                                      int32_t zeta_0,
                                                      int32_t zeta_1) {
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_1(&re->data[index],
                                                            zeta_0, zeta_1);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)0U, (int32_t)-3930395, (int32_t)-1528703);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)1U, (int32_t)-3677745, (int32_t)-3041255);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)2U, (int32_t)-1452451, (int32_t)3475950);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)3U, (int32_t)2176455, (int32_t)-1585221);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)4U, (int32_t)-1257611, (int32_t)1939314);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)5U, (int32_t)-4083598, (int32_t)-1000202);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)6U, (int32_t)-3190144, (int32_t)-3157330);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)7U, (int32_t)-3632928, (int32_t)126922);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)8U, (int32_t)3412210, (int32_t)-983419);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)9U, (int32_t)2147896, (int32_t)2715295);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)10U, (int32_t)-2967645, (int32_t)-3693493);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)11U, (int32_t)-411027, (int32_t)-2477047);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)12U, (int32_t)-671102, (int32_t)-1228525);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)13U, (int32_t)-22981, (int32_t)-1308169);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)14U, (int32_t)-381987, (int32_t)1349076);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)15U, (int32_t)1852771, (int32_t)-1430430);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)16U, (int32_t)-3343383, (int32_t)264944);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)17U, (int32_t)508951, (int32_t)3097992);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)18U, (int32_t)44288, (int32_t)-1100098);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)19U, (int32_t)904516, (int32_t)3958618);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)20U, (int32_t)-3724342, (int32_t)-8578);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)21U, (int32_t)1653064, (int32_t)-3249728);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)22U, (int32_t)2389356, (int32_t)-210977);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)23U, (int32_t)759969, (int32_t)-1316856);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)24U, (int32_t)189548, (int32_t)-3553272);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)25U, (int32_t)3159746, (int32_t)-1851402);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)26U, (int32_t)-2409325, (int32_t)-177440);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)27U, (int32_t)1315589, (int32_t)1341330);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)28U, (int32_t)1285669, (int32_t)-1584928);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)29U, (int32_t)-812732, (int32_t)-1439742);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)30U, (int32_t)-3019102, (int32_t)-3881060);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_1_round(
      re, (size_t)31U, (int32_t)-3628969, (int32_t)3839961);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_0(
    Eurydice_arr_d4 *simd_unit, int32_t zeta0, int32_t zeta1, int32_t zeta2,
    int32_t zeta3) {
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta0,
                                                      (size_t)0U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta1,
                                                      (size_t)2U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta2,
                                                      (size_t)4U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_step(simd_unit, zeta3,
                                                      (size_t)6U, (size_t)1U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
    Eurydice_arr_a4 *re, size_t index, int32_t zeta_0, int32_t zeta_1,
    int32_t zeta_2, int32_t zeta_3) {
  libcrux_ml_dsa_simd_portable_ntt_simd_unit_ntt_at_layer_0(
      &re->data[index], zeta_0, zeta_1, zeta_2, zeta_3);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)0U, (int32_t)2091667, (int32_t)3407706, (int32_t)2316500,
      (int32_t)3817976);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)1U, (int32_t)-3342478, (int32_t)2244091, (int32_t)-2446433,
      (int32_t)-3562462);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)2U, (int32_t)266997, (int32_t)2434439, (int32_t)-1235728,
      (int32_t)3513181);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)3U, (int32_t)-3520352, (int32_t)-3759364, (int32_t)-1197226,
      (int32_t)-3193378);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)4U, (int32_t)900702, (int32_t)1859098, (int32_t)909542,
      (int32_t)819034);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)5U, (int32_t)495491, (int32_t)-1613174, (int32_t)-43260,
      (int32_t)-522500);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)6U, (int32_t)-655327, (int32_t)-3122442, (int32_t)2031748,
      (int32_t)3207046);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)7U, (int32_t)-3556995, (int32_t)-525098, (int32_t)-768622,
      (int32_t)-3595838);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)8U, (int32_t)342297, (int32_t)286988, (int32_t)-2437823,
      (int32_t)4108315);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)9U, (int32_t)3437287, (int32_t)-3342277, (int32_t)1735879,
      (int32_t)203044);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)10U, (int32_t)2842341, (int32_t)2691481, (int32_t)-2590150,
      (int32_t)1265009);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)11U, (int32_t)4055324, (int32_t)1247620, (int32_t)2486353,
      (int32_t)1595974);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)12U, (int32_t)-3767016, (int32_t)1250494, (int32_t)2635921,
      (int32_t)-3548272);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)13U, (int32_t)-2994039, (int32_t)1869119, (int32_t)1903435,
      (int32_t)-1050970);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)14U, (int32_t)-1333058, (int32_t)1237275, (int32_t)-3318210,
      (int32_t)-1430225);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)15U, (int32_t)-451100, (int32_t)1312455, (int32_t)3306115,
      (int32_t)-1962642);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)16U, (int32_t)-1279661, (int32_t)1917081, (int32_t)-2546312,
      (int32_t)-1374803);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)17U, (int32_t)1500165, (int32_t)777191, (int32_t)2235880,
      (int32_t)3406031);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)18U, (int32_t)-542412, (int32_t)-2831860, (int32_t)-1671176,
      (int32_t)-1846953);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)19U, (int32_t)-2584293, (int32_t)-3724270, (int32_t)594136,
      (int32_t)-3776993);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)20U, (int32_t)-2013608, (int32_t)2432395, (int32_t)2454455,
      (int32_t)-164721);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)21U, (int32_t)1957272, (int32_t)3369112, (int32_t)185531,
      (int32_t)-1207385);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)22U, (int32_t)-3183426, (int32_t)162844, (int32_t)1616392,
      (int32_t)3014001);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)23U, (int32_t)810149, (int32_t)1652634, (int32_t)-3694233,
      (int32_t)-1799107);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)24U, (int32_t)-3038916, (int32_t)3523897, (int32_t)3866901,
      (int32_t)269760);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)25U, (int32_t)2213111, (int32_t)-975884, (int32_t)1717735,
      (int32_t)472078);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)26U, (int32_t)-426683, (int32_t)1723600, (int32_t)-1803090,
      (int32_t)1910376);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)27U, (int32_t)-1667432, (int32_t)-1104333, (int32_t)-260646,
      (int32_t)-3833893);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)28U, (int32_t)-2939036, (int32_t)-2235985, (int32_t)-420899,
      (int32_t)-2286327);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)29U, (int32_t)183443, (int32_t)-976891, (int32_t)1612842,
      (int32_t)-3545687);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)30U, (int32_t)-554416, (int32_t)3919660, (int32_t)-48306,
      (int32_t)-1362209);
  libcrux_ml_dsa_simd_portable_ntt_ntt_at_layer_0_round(
      re, (size_t)31U, (int32_t)3937738, (int32_t)1400424, (int32_t)-846154,
      (int32_t)1976782);
}

static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_ntt_ntt(
    Eurydice_arr_a4 *re) {
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
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_ntt_65(
    Eurydice_arr_a4 *simd_units) {
  libcrux_ml_dsa_simd_portable_ntt_ntt(simd_units);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
    Eurydice_arr_d4 *simd_unit, int32_t zeta, size_t index, size_t step) {
  int32_t a_minus_b = simd_unit->data[index + step] - simd_unit->data[index];
  simd_unit->data[index] =
      simd_unit->data[index] + simd_unit->data[index + step];
  simd_unit->data[index + step] =
      libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_fe_by_fer(
          a_minus_b, zeta);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_0(
    Eurydice_arr_d4 *simd_unit, int32_t zeta0, int32_t zeta1, int32_t zeta2,
    int32_t zeta3) {
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta0, (size_t)0U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta1, (size_t)2U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta2, (size_t)4U, (size_t)1U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta3, (size_t)6U, (size_t)1U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
    Eurydice_arr_a4 *re, size_t index, int32_t zeta0, int32_t zeta1,
    int32_t zeta2, int32_t zeta3) {
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_0(
      &re->data[index], zeta0, zeta1, zeta2, zeta3);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)0U, (int32_t)1976782, (int32_t)-846154, (int32_t)1400424,
      (int32_t)3937738);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)1U, (int32_t)-1362209, (int32_t)-48306, (int32_t)3919660,
      (int32_t)-554416);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)2U, (int32_t)-3545687, (int32_t)1612842, (int32_t)-976891,
      (int32_t)183443);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)3U, (int32_t)-2286327, (int32_t)-420899, (int32_t)-2235985,
      (int32_t)-2939036);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)4U, (int32_t)-3833893, (int32_t)-260646, (int32_t)-1104333,
      (int32_t)-1667432);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)5U, (int32_t)1910376, (int32_t)-1803090, (int32_t)1723600,
      (int32_t)-426683);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)6U, (int32_t)472078, (int32_t)1717735, (int32_t)-975884,
      (int32_t)2213111);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)7U, (int32_t)269760, (int32_t)3866901, (int32_t)3523897,
      (int32_t)-3038916);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)8U, (int32_t)-1799107, (int32_t)-3694233, (int32_t)1652634,
      (int32_t)810149);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)9U, (int32_t)3014001, (int32_t)1616392, (int32_t)162844,
      (int32_t)-3183426);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)10U, (int32_t)-1207385, (int32_t)185531, (int32_t)3369112,
      (int32_t)1957272);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)11U, (int32_t)-164721, (int32_t)2454455, (int32_t)2432395,
      (int32_t)-2013608);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)12U, (int32_t)-3776993, (int32_t)594136, (int32_t)-3724270,
      (int32_t)-2584293);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)13U, (int32_t)-1846953, (int32_t)-1671176, (int32_t)-2831860,
      (int32_t)-542412);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)14U, (int32_t)3406031, (int32_t)2235880, (int32_t)777191,
      (int32_t)1500165);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)15U, (int32_t)-1374803, (int32_t)-2546312, (int32_t)1917081,
      (int32_t)-1279661);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)16U, (int32_t)-1962642, (int32_t)3306115, (int32_t)1312455,
      (int32_t)-451100);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)17U, (int32_t)-1430225, (int32_t)-3318210, (int32_t)1237275,
      (int32_t)-1333058);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)18U, (int32_t)-1050970, (int32_t)1903435, (int32_t)1869119,
      (int32_t)-2994039);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)19U, (int32_t)-3548272, (int32_t)2635921, (int32_t)1250494,
      (int32_t)-3767016);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)20U, (int32_t)1595974, (int32_t)2486353, (int32_t)1247620,
      (int32_t)4055324);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)21U, (int32_t)1265009, (int32_t)-2590150, (int32_t)2691481,
      (int32_t)2842341);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)22U, (int32_t)203044, (int32_t)1735879, (int32_t)-3342277,
      (int32_t)3437287);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)23U, (int32_t)4108315, (int32_t)-2437823, (int32_t)286988,
      (int32_t)342297);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)24U, (int32_t)-3595838, (int32_t)-768622, (int32_t)-525098,
      (int32_t)-3556995);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)25U, (int32_t)3207046, (int32_t)2031748, (int32_t)-3122442,
      (int32_t)-655327);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)26U, (int32_t)-522500, (int32_t)-43260, (int32_t)-1613174,
      (int32_t)495491);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)27U, (int32_t)819034, (int32_t)909542, (int32_t)1859098,
      (int32_t)900702);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)28U, (int32_t)-3193378, (int32_t)-1197226, (int32_t)-3759364,
      (int32_t)-3520352);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)29U, (int32_t)3513181, (int32_t)-1235728, (int32_t)2434439,
      (int32_t)266997);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)30U, (int32_t)-3562462, (int32_t)-2446433, (int32_t)2244091,
      (int32_t)-3342478);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0_round(
      re, (size_t)31U, (int32_t)3817976, (int32_t)2316500, (int32_t)3407706,
      (int32_t)2091667);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_1(
    Eurydice_arr_d4 *simd_unit, int32_t zeta0, int32_t zeta1) {
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta0, (size_t)0U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta0, (size_t)1U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta1, (size_t)4U, (size_t)2U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta1, (size_t)5U, (size_t)2U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
    Eurydice_arr_a4 *re, size_t index, int32_t zeta_00, int32_t zeta_01) {
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_1(
      &re->data[index], zeta_00, zeta_01);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)0U, (int32_t)3839961, (int32_t)-3628969);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)1U, (int32_t)-3881060, (int32_t)-3019102);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)2U, (int32_t)-1439742, (int32_t)-812732);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)3U, (int32_t)-1584928, (int32_t)1285669);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)4U, (int32_t)1341330, (int32_t)1315589);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)5U, (int32_t)-177440, (int32_t)-2409325);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)6U, (int32_t)-1851402, (int32_t)3159746);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)7U, (int32_t)-3553272, (int32_t)189548);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)8U, (int32_t)-1316856, (int32_t)759969);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)9U, (int32_t)-210977, (int32_t)2389356);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)10U, (int32_t)-3249728, (int32_t)1653064);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)11U, (int32_t)-8578, (int32_t)-3724342);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)12U, (int32_t)3958618, (int32_t)904516);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)13U, (int32_t)-1100098, (int32_t)44288);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)14U, (int32_t)3097992, (int32_t)508951);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)15U, (int32_t)264944, (int32_t)-3343383);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)16U, (int32_t)-1430430, (int32_t)1852771);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)17U, (int32_t)1349076, (int32_t)-381987);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)18U, (int32_t)-1308169, (int32_t)-22981);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)19U, (int32_t)-1228525, (int32_t)-671102);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)20U, (int32_t)-2477047, (int32_t)-411027);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)21U, (int32_t)-3693493, (int32_t)-2967645);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)22U, (int32_t)2715295, (int32_t)2147896);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)23U, (int32_t)-983419, (int32_t)3412210);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)24U, (int32_t)126922, (int32_t)-3632928);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)25U, (int32_t)-3157330, (int32_t)-3190144);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)26U, (int32_t)-1000202, (int32_t)-4083598);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)27U, (int32_t)1939314, (int32_t)-1257611);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)28U, (int32_t)-1585221, (int32_t)2176455);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)29U, (int32_t)3475950, (int32_t)-1452451);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)30U, (int32_t)-3041255, (int32_t)-3677745);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1_round(
      re, (size_t)31U, (int32_t)-1528703, (int32_t)-3930395);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_2(
    Eurydice_arr_d4 *simd_unit, int32_t zeta) {
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta, (size_t)0U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta, (size_t)1U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta, (size_t)2U, (size_t)4U);
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_inv_ntt_step(
      simd_unit, zeta, (size_t)3U, (size_t)4U);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
    Eurydice_arr_a4 *re, size_t index, int32_t zeta1) {
  libcrux_ml_dsa_simd_portable_invntt_simd_unit_invert_ntt_at_layer_2(
      &re->data[index], zeta1);
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)0U, (int32_t)-2797779);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)1U, (int32_t)2071892);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)2U, (int32_t)-2556880);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)3U, (int32_t)3900724);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)4U, (int32_t)3881043);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)5U, (int32_t)954230);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)6U, (int32_t)531354);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)7U, (int32_t)811944);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)8U, (int32_t)3699596);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)9U, (int32_t)-1600420);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)10U, (int32_t)-2140649);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)11U, (int32_t)3507263);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)12U, (int32_t)-3821735);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)13U, (int32_t)3505694);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)14U, (int32_t)-1643818);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)15U, (int32_t)-1699267);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)16U, (int32_t)-539299);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)17U, (int32_t)2348700);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)18U, (int32_t)-300467);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)19U, (int32_t)3539968);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)20U, (int32_t)-2867647);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)21U, (int32_t)3574422);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)22U, (int32_t)-3043716);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)23U, (int32_t)-3861115);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)24U, (int32_t)3915439);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)25U, (int32_t)-2537516);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)26U, (int32_t)-3592148);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)27U, (int32_t)-1661693);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)28U, (int32_t)3530437);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)29U, (int32_t)3077325);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)30U, (int32_t)95776);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2_round(
      re, (size_t)31U, (int32_t)2706023);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 1
- ZETA= 280005
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_99(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)280005);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 2
- STEP_BY= 1
- ZETA= 4010497
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_1c(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)2U; i < (size_t)2U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)4010497);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 4
- STEP_BY= 1
- ZETA= -19422
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_6b(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-19422);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 6
- STEP_BY= 1
- ZETA= 1757237
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_44(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)6U; i < (size_t)6U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)1757237);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 8
- STEP_BY= 1
- ZETA= -3277672
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a8(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-3277672);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 10
- STEP_BY= 1
- ZETA= -1399561
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_1f(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)10U; i < (size_t)10U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-1399561);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 12
- STEP_BY= 1
- ZETA= -3859737
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_95(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-3859737);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 14
- STEP_BY= 1
- ZETA= -2118186
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3b(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)14U; i < (size_t)14U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-2118186);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 16
- STEP_BY= 1
- ZETA= -2108549
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-2108549);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 18
- STEP_BY= 1
- ZETA= 2619752
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_e4(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)18U; i < (size_t)18U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)2619752);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 20
- STEP_BY= 1
- ZETA= -1119584
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_de(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-1119584);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 22
- STEP_BY= 1
- ZETA= -549488
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_05(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)22U; i < (size_t)22U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-549488);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 24
- STEP_BY= 1
- ZETA= 3585928
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d9(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)3585928);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 26
- STEP_BY= 1
- ZETA= -1079900
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3a(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)26U; i < (size_t)26U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)-1079900);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3b0(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)1024112);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 30
- STEP_BY= 1
- ZETA= 2725464
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a0(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)30U; i < (size_t)30U + (size_t)1U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)1U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)1U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)1U], (int32_t)2725464);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_3(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_99(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_1c(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_6b(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_44(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a8(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_1f(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_95(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3b(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_e4(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_de(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_05(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d9(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3a(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3b0(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a0(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 2
- ZETA= 2680103
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_990(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)2680103);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_6b0(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)4U; i < (size_t)4U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)3111497);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a80(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)-2884855);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_950(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)12U; i < (size_t)12U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)3119733);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a0(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)-2091905);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_de0(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)20U; i < (size_t)20U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)-359251);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d90(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)2353451);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3b1(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)28U; i < (size_t)28U + (size_t)2U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)2U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)2U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)2U], (int32_t)1826347);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_4(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_990(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_6b0(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a80(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_950(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a0(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_de0(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d90(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_3b1(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 4
- ZETA= 466468
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_991(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)4U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)4U], (int32_t)466468);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a81(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)8U; i < (size_t)8U + (size_t)4U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)4U], (int32_t)-876248);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a1(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)4U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)4U], (int32_t)-777960);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d91(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)24U; i < (size_t)24U + (size_t)4U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)4U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)4U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)4U], (int32_t)237124);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_5(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_991(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_a81(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a1(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_d91(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 8
- ZETA= -518909
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_992(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)8U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)8U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)8U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)8U], (int32_t)-518909);
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
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a2(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)16U; i < (size_t)16U + (size_t)8U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)8U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)8U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)8U], (int32_t)-2608894);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_6(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_992(re);
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_7a2(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.invntt.outer_3_plus
with const generics
- OFFSET= 0
- STEP_BY= 16
- ZETA= 25847
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_993(Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U; i < (size_t)0U + (size_t)16U; i++) {
    size_t j = i;
    Eurydice_arr_d4 rej = re->data[j];
    Eurydice_arr_d4 rejs = re->data[j + (size_t)16U];
    libcrux_ml_dsa_simd_portable_arithmetic_add(&re->data[j], &rejs);
    libcrux_ml_dsa_simd_portable_arithmetic_subtract(&re->data[j + (size_t)16U],
                                                     &rej);
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[j + (size_t)16U], (int32_t)25847);
  }
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_7(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_outer_3_plus_993(re);
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, size_t

*/
typedef struct Eurydice_dst_ref_shared_b7_s {
  const Eurydice_arr_d4 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_b7;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 32
*/
static inline Eurydice_dst_ref_shared_b7 Eurydice_array_to_slice_shared_a2(
    const Eurydice_arr_a4 *a) {
  Eurydice_dst_ref_shared_b7 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)32U;
  return lit;
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_invntt_invert_ntt_montgomery(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_0(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_1(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_2(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_3(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_4(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_5(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_6(re);
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_at_layer_7(re);
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_arithmetic_montgomery_multiply_by_constant(
        &re->data[i0], (int32_t)41978);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_invert_ntt_montgomery_65(
    Eurydice_arr_a4 *simd_units) {
  libcrux_ml_dsa_simd_portable_invntt_invert_ntt_montgomery(simd_units);
}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.portable.arithmetic.shift_left_then_reduce with const
generics
- SHIFT_BY= 0
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_shift_left_then_reduce_c3(
    Eurydice_arr_d4 *simd_unit) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t);
       i++) {
    size_t i0 = i;
    simd_unit->data[i0] =
        libcrux_ml_dsa_simd_portable_arithmetic_reduce_element(
            simd_unit->data[i0] << (uint32_t)(int32_t)0);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline void libcrux_ml_dsa_simd_portable_reduce_65(
    Eurydice_arr_a4 *simd_units) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(simd_units),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_arithmetic_shift_left_then_reduce_c3(
        &simd_units->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.polynomial.PolynomialRingElement
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients

*/
typedef Eurydice_arr_a4 libcrux_ml_dsa_polynomial_PolynomialRingElement_e8;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- $11size_t
*/
typedef struct Eurydice_arr_93_s {
  Eurydice_arr_a4 data[11U];
} Eurydice_arr_93;

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.zero_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline Eurydice_arr_a4 libcrux_ml_dsa_polynomial_zero_ff_37(void) {
  Eurydice_arr_a4 lit;
  Eurydice_arr_d4 repeat_expression[32U];
  for (size_t i = (size_t)0U; i < (size_t)32U; i++) {
    repeat_expression[i] = libcrux_ml_dsa_simd_portable_zero_65();
  }
  memcpy(lit.data, repeat_expression, (size_t)32U * sizeof(Eurydice_arr_d4));
  return lit;
}

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients, size_t

*/
typedef struct Eurydice_dst_ref_mut_f3_s {
  Eurydice_arr_a4 *ptr;
  size_t meta;
} Eurydice_dst_ref_mut_f3;

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients, size_t

*/
typedef struct Eurydice_dst_ref_shared_f3_s {
  const Eurydice_arr_a4 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_f3;

/**
A monomorphic instance of
libcrux_ml_dsa.sample.rejection_sample_less_than_eta_equals_4 with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_37(
    Eurydice_borrow_slice_u8 randomness, size_t *sampled_coefficients,
    Eurydice_arr_13 *out) {
  bool done = false;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(randomness, uint8_t) / (size_t)4U; i++) {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8 random_bytes = Eurydice_slice_subslice_shared_7e(
        randomness,
        (core_ops_range_Range_08{_cloop_i * (size_t)4U,
                                 _cloop_i * (size_t)4U + (size_t)4U}));
    if (!done) {
      size_t sampled =
          libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_4_65(
              random_bytes, Eurydice_array_to_subslice_from_mut_96(
                                out, sampled_coefficients[0U]));
      sampled_coefficients[0U] = sampled_coefficients[0U] + sampled;
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
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_37(
    Eurydice_borrow_slice_u8 randomness, size_t *sampled_coefficients,
    Eurydice_arr_13 *out) {
  bool done = false;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(randomness, uint8_t) / (size_t)4U; i++) {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8 random_bytes = Eurydice_slice_subslice_shared_7e(
        randomness,
        (core_ops_range_Range_08{_cloop_i * (size_t)4U,
                                 _cloop_i * (size_t)4U + (size_t)4U}));
    if (!done) {
      size_t sampled =
          libcrux_ml_dsa_simd_portable_rejection_sample_less_than_eta_equals_2_65(
              random_bytes, Eurydice_array_to_subslice_from_mut_96(
                                out, sampled_coefficients[0U]));
      sampled_coefficients[0U] = sampled_coefficients[0U] + sampled;
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
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 randomness,
    size_t *sampled, Eurydice_arr_13 *out) {
  switch (eta) {
    case libcrux_ml_dsa_constants_Eta_Two: {
      break;
    }
    case libcrux_ml_dsa_constants_Eta_Four: {
      return libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_4_37(
          randomness, sampled, out);
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  return libcrux_ml_dsa_sample_rejection_sample_less_than_eta_equals_2_37(
      randomness, sampled, out);
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.from_i32_array_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline void libcrux_ml_dsa_polynomial_from_i32_array_ff_37(
    Eurydice_dst_ref_shared_fc array, Eurydice_arr_a4 *result) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_DSA_SIMD_TRAITS_SIMD_UNITS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_from_coefficient_array_65(
        Eurydice_slice_subslice_shared_46(
            array,
            (core_ops_range_Range_08{
                i0 * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_four_error_ring_elements
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_four_error_ring_elements_29(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 seed,
    uint16_t start_index, Eurydice_dst_ref_mut_f3 re) {
  Eurydice_arr_a2 seed0 =
      libcrux_ml_dsa_sample_add_error_domain_separator(seed, start_index);
  Eurydice_arr_a2 seed1 = libcrux_ml_dsa_sample_add_error_domain_separator(
      seed, (uint32_t)start_index + 1U);
  Eurydice_arr_a2 seed2 = libcrux_ml_dsa_sample_add_error_domain_separator(
      seed, (uint32_t)start_index + 2U);
  Eurydice_arr_a2 seed3 = libcrux_ml_dsa_sample_add_error_domain_separator(
      seed, (uint32_t)start_index + 3U);
  libcrux_ml_dsa_hash_functions_portable_Shake256X4 state =
      libcrux_ml_dsa_hash_functions_portable_init_absorb_x4_9b(
          Eurydice_array_to_slice_shared_39(&seed0),
          Eurydice_array_to_slice_shared_39(&seed1),
          Eurydice_array_to_slice_shared_39(&seed2),
          Eurydice_array_to_slice_shared_39(&seed3));
  Eurydice_arr_uint8_t___136size_t___x4 randomnesses0 =
      libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4_9b(&state);
  Eurydice_arr_46 out = {{{{0U}}, {{0U}}, {{0U}}, {{0U}}}};
  size_t sampled0 = (size_t)0U;
  size_t sampled1 = (size_t)0U;
  size_t sampled2 = (size_t)0U;
  size_t sampled3 = (size_t)0U;
  libcrux_ml_dsa_constants_Eta uu____0 = eta;
  bool done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
      uu____0, Eurydice_array_to_slice_shared_d4(&randomnesses0.fst), &sampled0,
      out.data);
  libcrux_ml_dsa_constants_Eta uu____1 = eta;
  bool done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
      uu____1, Eurydice_array_to_slice_shared_d4(&randomnesses0.snd), &sampled1,
      &out.data[1U]);
  libcrux_ml_dsa_constants_Eta uu____2 = eta;
  bool done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
      uu____2, Eurydice_array_to_slice_shared_d4(&randomnesses0.thd), &sampled2,
      &out.data[2U]);
  libcrux_ml_dsa_constants_Eta uu____3 = eta;
  bool done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
      uu____3, Eurydice_array_to_slice_shared_d4(&randomnesses0.f3), &sampled3,
      &out.data[3U]);
  while (true) {
    if (done0) {
      if (done1) {
        if (done2) {
          if (done3) {
            break;
          } else {
            Eurydice_arr_uint8_t___136size_t___x4 randomnesses =
                libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(
                    &state);
            if (!done0) {
              libcrux_ml_dsa_constants_Eta uu____4 = eta;
              done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                  uu____4, Eurydice_array_to_slice_shared_d4(&randomnesses.fst),
                  &sampled0, out.data);
            }
            if (!done1) {
              libcrux_ml_dsa_constants_Eta uu____5 = eta;
              done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                  uu____5, Eurydice_array_to_slice_shared_d4(&randomnesses.snd),
                  &sampled1, &out.data[1U]);
            }
            if (!done2) {
              libcrux_ml_dsa_constants_Eta uu____6 = eta;
              done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                  uu____6, Eurydice_array_to_slice_shared_d4(&randomnesses.thd),
                  &sampled2, &out.data[2U]);
            }
            if (!done3) {
              libcrux_ml_dsa_constants_Eta uu____7 = eta;
              done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                  uu____7, Eurydice_array_to_slice_shared_d4(&randomnesses.f3),
                  &sampled3, &out.data[3U]);
            }
          }
        } else {
          Eurydice_arr_uint8_t___136size_t___x4 randomnesses =
              libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(
                  &state);
          if (!done0) {
            libcrux_ml_dsa_constants_Eta uu____8 = eta;
            done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                uu____8, Eurydice_array_to_slice_shared_d4(&randomnesses.fst),
                &sampled0, out.data);
          }
          if (!done1) {
            libcrux_ml_dsa_constants_Eta uu____9 = eta;
            done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                uu____9, Eurydice_array_to_slice_shared_d4(&randomnesses.snd),
                &sampled1, &out.data[1U]);
          }
          if (!done2) {
            libcrux_ml_dsa_constants_Eta uu____10 = eta;
            done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                uu____10, Eurydice_array_to_slice_shared_d4(&randomnesses.thd),
                &sampled2, &out.data[2U]);
          }
          if (!done3) {
            libcrux_ml_dsa_constants_Eta uu____11 = eta;
            done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
                uu____11, Eurydice_array_to_slice_shared_d4(&randomnesses.f3),
                &sampled3, &out.data[3U]);
          }
        }
      } else {
        Eurydice_arr_uint8_t___136size_t___x4 randomnesses =
            libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(
                &state);
        if (!done0) {
          libcrux_ml_dsa_constants_Eta uu____12 = eta;
          done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
              uu____12, Eurydice_array_to_slice_shared_d4(&randomnesses.fst),
              &sampled0, out.data);
        }
        if (!done1) {
          libcrux_ml_dsa_constants_Eta uu____13 = eta;
          done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
              uu____13, Eurydice_array_to_slice_shared_d4(&randomnesses.snd),
              &sampled1, &out.data[1U]);
        }
        if (!done2) {
          libcrux_ml_dsa_constants_Eta uu____14 = eta;
          done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
              uu____14, Eurydice_array_to_slice_shared_d4(&randomnesses.thd),
              &sampled2, &out.data[2U]);
        }
        if (!done3) {
          libcrux_ml_dsa_constants_Eta uu____15 = eta;
          done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
              uu____15, Eurydice_array_to_slice_shared_d4(&randomnesses.f3),
              &sampled3, &out.data[3U]);
        }
      }
    } else {
      Eurydice_arr_uint8_t___136size_t___x4 randomnesses =
          libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(
              &state);
      if (!done0) {
        libcrux_ml_dsa_constants_Eta uu____16 = eta;
        done0 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
            uu____16, Eurydice_array_to_slice_shared_d4(&randomnesses.fst),
            &sampled0, out.data);
      }
      if (!done1) {
        libcrux_ml_dsa_constants_Eta uu____17 = eta;
        done1 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
            uu____17, Eurydice_array_to_slice_shared_d4(&randomnesses.snd),
            &sampled1, &out.data[1U]);
      }
      if (!done2) {
        libcrux_ml_dsa_constants_Eta uu____18 = eta;
        done2 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
            uu____18, Eurydice_array_to_slice_shared_d4(&randomnesses.thd),
            &sampled2, &out.data[2U]);
      }
      if (!done3) {
        libcrux_ml_dsa_constants_Eta uu____19 = eta;
        done3 = libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(
            uu____19, Eurydice_array_to_slice_shared_d4(&randomnesses.f3),
            &sampled3, &out.data[3U]);
      }
    }
  }
  size_t max0 = (size_t)start_index + (size_t)4U;
  size_t max;
  if (Eurydice_slice_len((Eurydice_dst_ref_shared_f3{re.ptr, re.meta}),
                         Eurydice_arr_a4) < max0) {
    max = Eurydice_slice_len((Eurydice_dst_ref_shared_f3{re.ptr, re.meta}),
                             Eurydice_arr_a4);
  } else {
    max = max0;
  }
  for (size_t i = (size_t)start_index; i < max; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_from_i32_array_ff_37(
        Eurydice_array_to_slice_shared_200(&out.data[i0 % (size_t)4U]),
        &Eurydice_slice_index_mut(re, i0, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.sample_s1_and_s2
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_samplex4_sample_s1_and_s2_29(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_f3 s1_s2) {
  size_t len = Eurydice_slice_len(
      (Eurydice_dst_ref_shared_f3{s1_s2.ptr, s1_s2.meta}), Eurydice_arr_a4);
  for (size_t i = (size_t)0U; i < len / (size_t)4U; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_sample_sample_four_error_ring_elements_29(
        eta, seed, 4U * (uint32_t)(uint16_t)i0, s1_s2);
  }
  size_t remainder = len % (size_t)4U;
  if (remainder != (size_t)0U) {
    libcrux_ml_dsa_sample_sample_four_error_ring_elements_29(
        eta, seed, (uint16_t)(len - remainder), s1_s2);
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 11
*/
static inline Eurydice_dst_ref_mut_f3 Eurydice_array_to_slice_mut_71(
    Eurydice_arr_93 *a) {
  Eurydice_dst_ref_mut_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- $6size_t
*/
typedef struct Eurydice_arr_47_s {
  Eurydice_arr_a4 data[6U];
} Eurydice_arr_47;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- $30size_t
*/
typedef struct Eurydice_arr_c9_s {
  Eurydice_arr_a4 data[30U];
} Eurydice_arr_c9;

/**
A monomorphic instance of
libcrux_ml_dsa.sample.rejection_sample_less_than_field_modulus with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
    Eurydice_borrow_slice_u8 randomness, size_t *sampled_coefficients,
    Eurydice_arr_13 *out) {
  bool done = false;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(randomness, uint8_t) / (size_t)24U; i++) {
    size_t _cloop_i = i;
    Eurydice_borrow_slice_u8 random_bytes = Eurydice_slice_subslice_shared_7e(
        randomness,
        (core_ops_range_Range_08{_cloop_i * (size_t)24U,
                                 _cloop_i * (size_t)24U + (size_t)24U}));
    if (!done) {
      size_t sampled =
          libcrux_ml_dsa_simd_portable_rejection_sample_less_than_field_modulus_65(
              random_bytes, Eurydice_array_to_subslice_from_mut_96(
                                out, sampled_coefficients[0U]));
      sampled_coefficients[0U] = sampled_coefficients[0U] + sampled;
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
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake128X4 with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_63(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_f3 matrix, Eurydice_arr_12 *rand_stack0,
    Eurydice_arr_12 *rand_stack1, Eurydice_arr_12 *rand_stack2,
    Eurydice_arr_12 *rand_stack3, Eurydice_dst_ref_mut_2c tmp_stack,
    size_t start_index, size_t elements_requested) {
  Eurydice_arr_48 seed0 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index, columns));
  Eurydice_arr_48 seed1 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index + (size_t)1U, columns));
  Eurydice_arr_48 seed2 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index + (size_t)2U, columns));
  Eurydice_arr_48 seed3 = libcrux_ml_dsa_sample_add_domain_separator(
      seed, libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(
                start_index + (size_t)3U, columns));
  libcrux_ml_dsa_hash_functions_portable_Shake128X4 state =
      libcrux_ml_dsa_hash_functions_portable_init_absorb_11(
          Eurydice_array_to_slice_shared_8d(&seed0),
          Eurydice_array_to_slice_shared_8d(&seed1),
          Eurydice_array_to_slice_shared_8d(&seed2),
          Eurydice_array_to_slice_shared_8d(&seed3));
  libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks_11(
      &state, rand_stack0, rand_stack1, rand_stack2, rand_stack3);
  size_t sampled0 = (size_t)0U;
  size_t sampled1 = (size_t)0U;
  size_t sampled2 = (size_t)0U;
  size_t sampled3 = (size_t)0U;
  bool done0 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
          Eurydice_array_to_slice_shared_a8(rand_stack0), &sampled0,
          &Eurydice_slice_index_mut(tmp_stack, (size_t)0U, Eurydice_arr_13));
  bool done1 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
          Eurydice_array_to_slice_shared_a8(rand_stack1), &sampled1,
          &Eurydice_slice_index_mut(tmp_stack, (size_t)1U, Eurydice_arr_13));
  bool done2 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
          Eurydice_array_to_slice_shared_a8(rand_stack2), &sampled2,
          &Eurydice_slice_index_mut(tmp_stack, (size_t)2U, Eurydice_arr_13));
  bool done3 =
      libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
          Eurydice_array_to_slice_shared_a8(rand_stack3), &sampled3,
          &Eurydice_slice_index_mut(tmp_stack, (size_t)3U, Eurydice_arr_13));
  while (true) {
    if (done0) {
      if (done1) {
        if (done2) {
          if (done3) {
            break;
          } else {
            Eurydice_arr_uint8_t___168size_t___x4 randomnesses =
                libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(
                    &state);
            if (!done0) {
              done0 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                      Eurydice_array_to_slice_shared_7b(&randomnesses.fst),
                      &sampled0,
                      &Eurydice_slice_index_mut(tmp_stack, (size_t)0U,
                                                Eurydice_arr_13));
            }
            if (!done1) {
              done1 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                      Eurydice_array_to_slice_shared_7b(&randomnesses.snd),
                      &sampled1,
                      &Eurydice_slice_index_mut(tmp_stack, (size_t)1U,
                                                Eurydice_arr_13));
            }
            if (!done2) {
              done2 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                      Eurydice_array_to_slice_shared_7b(&randomnesses.thd),
                      &sampled2,
                      &Eurydice_slice_index_mut(tmp_stack, (size_t)2U,
                                                Eurydice_arr_13));
            }
            if (!done3) {
              done3 =
                  libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                      Eurydice_array_to_slice_shared_7b(&randomnesses.f3),
                      &sampled3,
                      &Eurydice_slice_index_mut(tmp_stack, (size_t)3U,
                                                Eurydice_arr_13));
            }
          }
        } else {
          Eurydice_arr_uint8_t___168size_t___x4 randomnesses =
              libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(
                  &state);
          if (!done0) {
            done0 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                    Eurydice_array_to_slice_shared_7b(&randomnesses.fst),
                    &sampled0,
                    &Eurydice_slice_index_mut(tmp_stack, (size_t)0U,
                                              Eurydice_arr_13));
          }
          if (!done1) {
            done1 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                    Eurydice_array_to_slice_shared_7b(&randomnesses.snd),
                    &sampled1,
                    &Eurydice_slice_index_mut(tmp_stack, (size_t)1U,
                                              Eurydice_arr_13));
          }
          if (!done2) {
            done2 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                    Eurydice_array_to_slice_shared_7b(&randomnesses.thd),
                    &sampled2,
                    &Eurydice_slice_index_mut(tmp_stack, (size_t)2U,
                                              Eurydice_arr_13));
          }
          if (!done3) {
            done3 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                    Eurydice_array_to_slice_shared_7b(&randomnesses.f3),
                    &sampled3,
                    &Eurydice_slice_index_mut(tmp_stack, (size_t)3U,
                                              Eurydice_arr_13));
          }
        }
      } else {
        Eurydice_arr_uint8_t___168size_t___x4 randomnesses =
            libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(
                &state);
        if (!done0) {
          done0 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                  Eurydice_array_to_slice_shared_7b(&randomnesses.fst),
                  &sampled0,
                  &Eurydice_slice_index_mut(tmp_stack, (size_t)0U,
                                            Eurydice_arr_13));
        }
        if (!done1) {
          done1 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                  Eurydice_array_to_slice_shared_7b(&randomnesses.snd),
                  &sampled1,
                  &Eurydice_slice_index_mut(tmp_stack, (size_t)1U,
                                            Eurydice_arr_13));
        }
        if (!done2) {
          done2 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                  Eurydice_array_to_slice_shared_7b(&randomnesses.thd),
                  &sampled2,
                  &Eurydice_slice_index_mut(tmp_stack, (size_t)2U,
                                            Eurydice_arr_13));
        }
        if (!done3) {
          done3 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                  Eurydice_array_to_slice_shared_7b(&randomnesses.f3),
                  &sampled3,
                  &Eurydice_slice_index_mut(tmp_stack, (size_t)3U,
                                            Eurydice_arr_13));
        }
      }
    } else {
      Eurydice_arr_uint8_t___168size_t___x4 randomnesses =
          libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(&state);
      if (!done0) {
        done0 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                Eurydice_array_to_slice_shared_7b(&randomnesses.fst), &sampled0,
                &Eurydice_slice_index_mut(tmp_stack, (size_t)0U,
                                          Eurydice_arr_13));
      }
      if (!done1) {
        done1 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                Eurydice_array_to_slice_shared_7b(&randomnesses.snd), &sampled1,
                &Eurydice_slice_index_mut(tmp_stack, (size_t)1U,
                                          Eurydice_arr_13));
      }
      if (!done2) {
        done2 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                Eurydice_array_to_slice_shared_7b(&randomnesses.thd), &sampled2,
                &Eurydice_slice_index_mut(tmp_stack, (size_t)2U,
                                          Eurydice_arr_13));
      }
      if (!done3) {
        done3 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(
                Eurydice_array_to_slice_shared_7b(&randomnesses.f3), &sampled3,
                &Eurydice_slice_index_mut(tmp_stack, (size_t)3U,
                                          Eurydice_arr_13));
      }
    }
  }
  for (size_t i = (size_t)0U; i < elements_requested; i++) {
    size_t k = i;
    libcrux_ml_dsa_polynomial_from_i32_array_ff_37(
        Eurydice_array_to_slice_shared_200(&Eurydice_slice_index_shared(
            (Eurydice_dst_ref_shared_2c{tmp_stack.ptr, tmp_stack.meta}), k,
            Eurydice_arr_13)),
        &Eurydice_slice_index_mut(matrix, start_index + k, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.matrix_flat
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake128X4 with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_samplex4_matrix_flat_63(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_f3 matrix) {
  Eurydice_arr_12 rand_stack0 = {{0U}};
  Eurydice_arr_12 rand_stack1 = {{0U}};
  Eurydice_arr_12 rand_stack2 = {{0U}};
  Eurydice_arr_12 rand_stack3 = {{0U}};
  Eurydice_arr_46 tmp_stack = {{{{0U}}, {{0U}}, {{0U}}, {{0U}}}};
  for (size_t i = (size_t)0U;
       i <
       Eurydice_slice_len((Eurydice_dst_ref_shared_f3{matrix.ptr, matrix.meta}),
                          Eurydice_arr_a4) /
               (size_t)4U +
           (size_t)1U;
       i++) {
    size_t start_index = i;
    size_t start_index0 = start_index * (size_t)4U;
    size_t uu____0 = start_index0;
    if (!(uu____0 >= Eurydice_slice_len(
                         (Eurydice_dst_ref_shared_f3{matrix.ptr, matrix.meta}),
                         Eurydice_arr_a4))) {
      size_t uu____1 = start_index0 + (size_t)4U;
      size_t elements_requested;
      if (uu____1 <= Eurydice_slice_len(
                         (Eurydice_dst_ref_shared_f3{matrix.ptr, matrix.meta}),
                         Eurydice_arr_a4)) {
        elements_requested = (size_t)4U;
      } else {
        elements_requested =
            Eurydice_slice_len(
                (Eurydice_dst_ref_shared_f3{matrix.ptr, matrix.meta}),
                Eurydice_arr_a4) -
            start_index0;
      }
      libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_63(
          columns, seed, matrix, &rand_stack0, &rand_stack1, &rand_stack2,
          &rand_stack3, Eurydice_array_to_slice_mut_f6(&tmp_stack),
          start_index0, elements_requested);
      continue;
    }
    break;
  }
}

/**
This function found in impl {libcrux_ml_dsa::samplex4::X4Sampler for
libcrux_ml_dsa::samplex4::portable::PortableSampler}
*/
/**
A monomorphic instance of libcrux_ml_dsa.samplex4.portable.matrix_flat_a8
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline void libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(
    size_t columns, Eurydice_borrow_slice_u8 seed,
    Eurydice_dst_ref_mut_f3 matrix) {
  libcrux_ml_dsa_samplex4_matrix_flat_63(columns, seed, matrix);
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 30
*/
static inline Eurydice_dst_ref_mut_f3 Eurydice_array_to_slice_mut_710(
    Eurydice_arr_c9 *a) {
  Eurydice_dst_ref_mut_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)30U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- $5size_t
*/
typedef struct Eurydice_arr_40_s {
  Eurydice_arr_a4 data[5U];
} Eurydice_arr_40;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 5
*/
static inline Eurydice_dst_ref_mut_f3 Eurydice_array_to_slice_mut_711(
    Eurydice_arr_40 *a) {
  Eurydice_dst_ref_mut_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)5U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients, core_ops_range_Range
size_t, Eurydice_derefed_slice libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 11
*/
static inline Eurydice_dst_ref_shared_f3 Eurydice_array_to_subslice_shared_c3(
    const Eurydice_arr_93 *a, core_ops_range_Range_08 r) {
  return (Eurydice_dst_ref_shared_f3{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 5
*/
static inline Eurydice_dst_ref_shared_f3 Eurydice_array_to_slice_shared_71(
    const Eurydice_arr_40 *a) {
  Eurydice_dst_ref_shared_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)5U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_ntt_37(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_ntt_65(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.ntt_multiply_montgomery
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(
    Eurydice_arr_a4 *lhs, const Eurydice_arr_a4 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(lhs),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_montgomery_multiply_65(&lhs->data[i0],
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
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_polynomial_add_ff_37(
    Eurydice_arr_a4 *self, const Eurydice_arr_a4 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(self),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_add_65(&self->data[i0], &rhs->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.reduce
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_reduce_37(Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_reduce_65(re);
}

/**
A monomorphic instance of libcrux_ml_dsa.ntt.invert_ntt_montgomery
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(
    Eurydice_arr_a4 *re) {
  libcrux_ml_dsa_simd_portable_invert_ntt_montgomery_65(re);
}

/**
 Compute InvertNTT(Â ◦ ŝ₁) + s₂
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_as1_plus_s2
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_compute_as1_plus_s2_37(
    size_t rows_in_a, size_t columns_in_a, Eurydice_dst_ref_mut_f3 a_as_ntt,
    Eurydice_dst_ref_shared_f3 s1_ntt, Eurydice_dst_ref_shared_f3 s1_s2,
    Eurydice_dst_ref_mut_f3 result) {
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < columns_in_a; i++) {
      size_t j = i;
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(
          &Eurydice_slice_index_mut(a_as_ntt, i1 * columns_in_a + j,
                                    Eurydice_arr_a4),
          &Eurydice_slice_index_shared(s1_ntt, j, Eurydice_arr_a4));
      libcrux_ml_dsa_polynomial_add_ff_37(
          &Eurydice_slice_index_mut(result, i1, Eurydice_arr_a4),
          &Eurydice_slice_index_mut(a_as_ntt, i1 * columns_in_a + j,
                                    Eurydice_arr_a4));
    }
  }
  for (size_t i = (size_t)0U;
       i <
       Eurydice_slice_len((Eurydice_dst_ref_shared_f3{result.ptr, result.meta}),
                          Eurydice_arr_a4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_reduce_37(
        &Eurydice_slice_index_mut(result, i0, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(
        &Eurydice_slice_index_mut(result, i0, Eurydice_arr_a4));
    libcrux_ml_dsa_polynomial_add_ff_37(
        &Eurydice_slice_index_mut(result, i0, Eurydice_arr_a4),
        &Eurydice_slice_index_shared(s1_s2, columns_in_a + i0,
                                     Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 11
*/
static inline Eurydice_dst_ref_shared_f3 Eurydice_array_to_slice_shared_710(
    const Eurydice_arr_93 *a) {
  Eurydice_dst_ref_shared_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 6
*/
static inline Eurydice_dst_ref_mut_f3 Eurydice_array_to_slice_mut_712(
    Eurydice_arr_47 *a) {
  Eurydice_dst_ref_mut_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.power2round_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_power2round_vector_37(
    Eurydice_dst_ref_mut_f3 t, Eurydice_dst_ref_mut_f3 t1) {
  for (size_t i0 = (size_t)0U;
       i0 < Eurydice_slice_len((Eurydice_dst_ref_shared_f3{t.ptr, t.meta}),
                               Eurydice_arr_a4);
       i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(
                 Eurydice_array_to_slice_shared_a2(&Eurydice_slice_index_shared(
                     (Eurydice_dst_ref_shared_f3{t.ptr, t.meta}), i1,
                     Eurydice_arr_a4)),
                 Eurydice_arr_d4);
         i++) {
      size_t j = i;
      libcrux_ml_dsa_simd_portable_power2round_65(
          &Eurydice_slice_index_mut(t, i1, Eurydice_arr_a4).data[j],
          &Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4).data[j]);
    }
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_t1_serialize_37(
    const Eurydice_arr_a4 *re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_d4 *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_t1_serialize_65(
        simd_unit,
        Eurydice_slice_subslice_mut_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 *
                    LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T1_SERIALIZE_OUTPUT_BYTES_PER_SIMD_UNIT})));
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.verification_key.generate_serialized with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_verification_key_generate_serialized_37(
    Eurydice_borrow_slice_u8 seed, Eurydice_dst_ref_shared_f3 t1,
    Eurydice_mut_borrow_slice_u8 verification_key_serialized) {
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_7e(
          verification_key_serialized,
          (core_ops_range_Range_08{(size_t)0U,
                                   LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE})),
      seed, uint8_t);
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(t1, Eurydice_arr_a4);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_a4 *ring_element =
        &Eurydice_slice_index_shared(t1, i0, Eurydice_arr_a4);
    size_t offset = LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE +
                    i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE;
    libcrux_ml_dsa_encoding_t1_serialize_37(
        ring_element,
        Eurydice_slice_subslice_mut_7e(
            verification_key_serialized,
            (core_ops_range_Range_08{
                offset,
                offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE})));
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 6
*/
static inline Eurydice_dst_ref_shared_f3 Eurydice_array_to_slice_shared_711(
    const Eurydice_arr_47 *a) {
  Eurydice_dst_ref_shared_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256
with const generics
- OUTPUT_LENGTH= 64
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_portable_shake256_24(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_06 *out) {
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_d8(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_61
with const generics
- OUTPUT_LENGTH= 64
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_61_24(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_06 *out) {
  libcrux_ml_dsa_hash_functions_portable_shake256_24(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_error_serialize_37(
    libcrux_ml_dsa_constants_Eta eta, const Eurydice_arr_a4 *re,
    Eurydice_mut_borrow_slice_u8 serialized) {
  size_t output_bytes_per_simd_unit =
      libcrux_ml_dsa_encoding_error_chunk_size(eta);
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_d4 *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_error_serialize_65(
        eta, simd_unit,
        Eurydice_slice_subslice_mut_7e(
            serialized, (core_ops_range_Range_08{
                            i0 * output_bytes_per_simd_unit,
                            (i0 + (size_t)1U) * output_bytes_per_simd_unit})));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_t0_serialize_37(
    const Eurydice_arr_a4 *re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_d4 *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_t0_serialize_65(
        simd_unit,
        Eurydice_slice_subslice_mut_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT})));
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.signing_key.generate_serialized with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake256 with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_signing_key_generate_serialized_2e(
    libcrux_ml_dsa_constants_Eta eta, size_t error_ring_element_size,
    Eurydice_borrow_slice_u8 seed_matrix, Eurydice_borrow_slice_u8 seed_signing,
    Eurydice_borrow_slice_u8 verification_key, Eurydice_dst_ref_shared_f3 s1_2,
    Eurydice_dst_ref_shared_f3 t0,
    Eurydice_mut_borrow_slice_u8 signing_key_serialized) {
  size_t offset = (size_t)0U;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_7e(
          signing_key_serialized,
          (core_ops_range_Range_08{
              offset, offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE})),
      seed_matrix, uint8_t);
  offset = offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_7e(
          signing_key_serialized,
          (core_ops_range_Range_08{
              offset,
              offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE})),
      seed_signing, uint8_t);
  offset = offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE;
  Eurydice_arr_06 verification_key_hash = {{0U}};
  libcrux_ml_dsa_hash_functions_portable_shake256_61_24(verification_key,
                                                        &verification_key_hash);
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_7e(
          signing_key_serialized,
          (core_ops_range_Range_08{
              offset,
              offset +
                  LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH})),
      Eurydice_array_to_slice_shared_d8(&verification_key_hash), uint8_t);
  offset = offset + LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(s1_2, Eurydice_arr_a4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_error_serialize_37(
        eta, &Eurydice_slice_index_shared(s1_2, i0, Eurydice_arr_a4),
        Eurydice_slice_subslice_mut_7e(
            signing_key_serialized,
            (core_ops_range_Range_08{offset,
                                     offset + error_ring_element_size})));
    offset = offset + error_ring_element_size;
  }
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(t0, Eurydice_arr_a4);
       i++) {
    size_t _cloop_j = i;
    const Eurydice_arr_a4 *ring_element =
        &Eurydice_slice_index_shared(t0, _cloop_j, Eurydice_arr_a4);
    libcrux_ml_dsa_encoding_t0_serialize_37(
        ring_element,
        Eurydice_slice_subslice_mut_7e(
            signing_key_serialized,
            (core_ops_range_Range_08{
                offset,
                offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE})));
    offset = offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.generate_key_pair with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_5a(
    Eurydice_arr_60 randomness, Eurydice_mut_borrow_slice_u8 signing_key,
    Eurydice_mut_borrow_slice_u8 verification_key) {
  Eurydice_arr_d1 seed_expanded0 = {{0U}};
  libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 shake =
      libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(
      &shake, Eurydice_array_to_slice_shared_6e(&randomness));
  /* original Rust expression is not an lvalue in C */
  Eurydice_array_u8x2 lvalue = {
      {(uint8_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
       (uint8_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A}};
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
      &shake, Eurydice_array_to_slice_shared_26(&lvalue));
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(
      &shake, Eurydice_array_to_slice_mut_18(&seed_expanded0));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_18(&seed_expanded0),
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_expanded = uu____0.snd;
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      seed_expanded, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_ERROR_VECTORS_SIZE,
      uint8_t, Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_error_vectors = uu____1.fst;
  Eurydice_borrow_slice_u8 seed_for_signing = uu____1.snd;
  Eurydice_arr_93 s1_s2;
  Eurydice_arr_a4 repeat_expression0[11U];
  for (size_t i = (size_t)0U; i < (size_t)11U; i++) {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s1_s2.data, repeat_expression0, (size_t)11U * sizeof(Eurydice_arr_a4));
  libcrux_ml_dsa_samplex4_sample_s1_and_s2_29(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA, seed_for_error_vectors,
      Eurydice_array_to_slice_mut_71(&s1_s2));
  Eurydice_arr_47 t0;
  Eurydice_arr_a4 repeat_expression1[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t0.data, repeat_expression1, (size_t)6U * sizeof(Eurydice_arr_a4));
  Eurydice_arr_c9 a_as_ntt;
  Eurydice_arr_a4 repeat_expression2[30U];
  for (size_t i = (size_t)0U; i < (size_t)30U; i++) {
    repeat_expression2[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(a_as_ntt.data, repeat_expression2,
         (size_t)30U * sizeof(Eurydice_arr_a4));
  libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, seed_for_a,
      Eurydice_array_to_slice_mut_710(&a_as_ntt));
  Eurydice_arr_40 s1_ntt;
  Eurydice_arr_a4 repeat_expression3[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    repeat_expression3[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s1_ntt.data, repeat_expression3, (size_t)5U * sizeof(Eurydice_arr_a4));
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_711(&s1_ntt),
      Eurydice_array_to_subslice_shared_c3(
          &s1_s2,
          (core_ops_range_Range_08{
              (size_t)0U, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A})),
      Eurydice_arr_a4);
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_71(&s1_ntt),
                              Eurydice_arr_a4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_ntt_37(&s1_ntt.data[i0]);
  }
  libcrux_ml_dsa_matrix_compute_as1_plus_s2_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      Eurydice_array_to_slice_mut_710(&a_as_ntt),
      Eurydice_array_to_slice_shared_71(&s1_ntt),
      Eurydice_array_to_slice_shared_710(&s1_s2),
      Eurydice_array_to_slice_mut_712(&t0));
  Eurydice_arr_47 t1;
  Eurydice_arr_a4 repeat_expression[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t1.data, repeat_expression, (size_t)6U * sizeof(Eurydice_arr_a4));
  libcrux_ml_dsa_arithmetic_power2round_vector_37(
      Eurydice_array_to_slice_mut_712(&t0),
      Eurydice_array_to_slice_mut_712(&t1));
  libcrux_ml_dsa_encoding_verification_key_generate_serialized_37(
      seed_for_a, Eurydice_array_to_slice_shared_711(&t1), verification_key);
  libcrux_ml_dsa_encoding_signing_key_generate_serialized_2e(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
      seed_for_a, seed_for_signing,
      (Eurydice_borrow_slice_u8{verification_key.ptr, verification_key.meta}),
      Eurydice_array_to_slice_shared_710(&s1_s2),
      Eurydice_array_to_slice_shared_711(&t0), signing_key);
}

/**
 Generate key pair.
*/
static inline void
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_generate_key_pair(
    Eurydice_arr_60 randomness, Eurydice_arr_d10 *signing_key,
    Eurydice_arr_4a *verification_key) {
  libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_5a(
      randomness, Eurydice_array_to_slice_mut_ef(signing_key),
      Eurydice_array_to_slice_mut_5b(verification_key));
}

/**
 Generate an ML-DSA-65 Key Pair
*/
static inline libcrux_ml_dsa_types_MLDSAKeyPair_06
libcrux_ml_dsa_ml_dsa_65_portable_generate_key_pair(
    Eurydice_arr_60 randomness) {
  Eurydice_arr_d10 signing_key = {{0U}};
  Eurydice_arr_4a verification_key = {{0U}};
  libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_generate_key_pair(
      randomness, &signing_key, &verification_key);
  return (libcrux_ml_dsa_types_MLDSAKeyPair_06{
      libcrux_ml_dsa_types_new_9b_09(signing_key),
      libcrux_ml_dsa_types_new_7f_97(verification_key)});
}

/**
 Generate an ML-DSA-65 Key Pair
*/
static inline void libcrux_ml_dsa_ml_dsa_65_portable_generate_key_pair_mut(
    Eurydice_arr_60 randomness, Eurydice_arr_d10 *signing_key,
    Eurydice_arr_4a *verification_key) {
  libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_generate_key_pair(
      randomness, signing_key, verification_key);
}

/**
A monomorphic instance of core.result.Result
with types (), libcrux_ml_dsa_types_SigningError

*/
typedef struct Result_53_s {
  Result_a4_tags tag;
  libcrux_ml_dsa_types_SigningError f0;
} Result_53;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr uint8_t[[$48size_t]]

*/
typedef struct Option_91_s {
  Option_3b_tags tag;
  Eurydice_arr_5f f0;
} Option_91;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr Eurydice_arr int32_t[[$256size_t]][[$6size_t]]

*/
typedef struct Option_c9_s {
  Option_3b_tags tag;
  Eurydice_arr_e6 f0;
} Option_c9;

/**
A monomorphic instance of core.result.Result
with types libcrux_ml_dsa_types_MLDSASignature[[$3309size_t]],
libcrux_ml_dsa_types_SigningError

*/
typedef struct Result_2e_s {
  Result_a4_tags tag;
  union U {
    Eurydice_arr_96 case_Ok;
    libcrux_ml_dsa_types_SigningError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_2e_s)
} Result_2e;

/**
A monomorphic instance of core.option.Option
with types libcrux_ml_dsa_pre_hash_DomainSeparationContext

*/
typedef struct Option_84_s {
  Option_3b_tags tag;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext f0;
} Option_84;

/**
A monomorphic instance of libcrux_ml_dsa.encoding.error.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_error_deserialize_37(
    libcrux_ml_dsa_constants_Eta eta, Eurydice_borrow_slice_u8 serialized,
    Eurydice_arr_a4 *result) {
  size_t chunk_size = libcrux_ml_dsa_encoding_error_chunk_size(eta);
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(result),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_error_deserialize_65(
        eta,
        Eurydice_slice_subslice_shared_7e(
            serialized, (core_ops_range_Range_08{
                            i0 * chunk_size, (i0 + (size_t)1U) * chunk_size})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.error.deserialize_to_vector_then_ntt with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_37(
    libcrux_ml_dsa_constants_Eta eta, size_t ring_element_size,
    Eurydice_borrow_slice_u8 serialized,
    Eurydice_dst_ref_mut_f3 ring_elements) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / ring_element_size; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized,
        (core_ops_range_Range_08{i0 * ring_element_size,
                                 i0 * ring_element_size + ring_element_size}));
    libcrux_ml_dsa_encoding_error_deserialize_37(
        eta, bytes,
        &Eurydice_slice_index_mut(ring_elements, i0, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_ntt_37(
        &Eurydice_slice_index_mut(ring_elements, i0, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t0.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_t0_deserialize_37(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_a4 *result) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(result),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_t0_deserialize_65(
        Eurydice_slice_subslice_shared_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 * LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T0_OUTPUT_BYTES_PER_SIMD_UNIT})),
        &result->data[i0]);
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.encoding.t0.deserialize_to_vector_then_ntt with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_37(
    Eurydice_borrow_slice_u8 serialized,
    Eurydice_dst_ref_mut_f3 ring_elements) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) /
               LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized,
        (core_ops_range_Range_08{
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE,
            i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE +
                LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE}));
    libcrux_ml_dsa_encoding_t0_deserialize_37(
        bytes, &Eurydice_slice_index_mut(ring_elements, i0, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_ntt_37(
        &Eurydice_slice_index_mut(ring_elements, i0, Eurydice_arr_a4));
  }
}

/**
 This corresponds to line 6 in algorithm 7 in FIPS 204 (line 7 in algorithm
 8, resp.).

 If `domain_separation_context` is supplied, applies domain
 separation and length encoding to the context string,
 before appending the message (in the regular variant) or the
 pre-hash OID as well as the pre-hashed message digest. Otherwise,
 it is assumed that `message` already contains domain separation
 information.

 In FIPS 204 M' is the concatenation of the domain separated context, any
 potential pre-hash OID and the message (or the message pre-hash). We do not
 explicitely construct the concatenation in memory since it is of statically
 unknown length, but feed its components directly into the incremental XOF.

 Refer to line 10 of Algorithm 2 (and line 5 of Algorithm 3, resp.) in [FIPS
 204](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf#section.5)
 for details on the domain separation for regular ML-DSA. Line
 23 of Algorithm 4 (and line 18 of Algorithm 5,resp.) describe domain separation
 for the HashMl-DSA variant.
*/
/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.derive_message_representative with types
libcrux_ml_dsa_hash_functions_portable_Shake256Xof with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(
    Eurydice_borrow_slice_u8 verification_key_hash,
    const Option_84 *domain_separation_context,
    Eurydice_borrow_slice_u8 message, Eurydice_arr_06 *message_representative) {
  libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 shake =
      libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake,
                                                   verification_key_hash);
  if (domain_separation_context->tag == Some) {
    const libcrux_ml_dsa_pre_hash_DomainSeparationContext
        *domain_separation_context0 = &domain_separation_context->f0;
    libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 *uu____0 = &shake;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_f10 lvalue0 = {
        {(uint8_t)core_option__core__option__Option_T__TraitClause_0___is_some(
            libcrux_ml_dsa_pre_hash_pre_hash_oid_88(domain_separation_context0),
            Eurydice_arr_cb, bool)}};
    libcrux_ml_dsa_hash_functions_portable_absorb_26(
        uu____0, Eurydice_array_to_slice_shared_07(&lvalue0));
    libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 *uu____1 = &shake;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_f10 lvalue = {{(uint8_t)Eurydice_slice_len(
        libcrux_ml_dsa_pre_hash_context_88(domain_separation_context0),
        uint8_t)}};
    libcrux_ml_dsa_hash_functions_portable_absorb_26(
        uu____1, Eurydice_array_to_slice_shared_07(&lvalue));
    libcrux_ml_dsa_hash_functions_portable_absorb_26(
        &shake, libcrux_ml_dsa_pre_hash_context_88(domain_separation_context0));
    const Option_3b *uu____2 =
        libcrux_ml_dsa_pre_hash_pre_hash_oid_88(domain_separation_context0);
    if (uu____2->tag == Some) {
      const Eurydice_arr_cb *pre_hash_oid = &uu____2->f0;
      libcrux_ml_dsa_hash_functions_portable_absorb_26(
          &shake, Eurydice_array_to_slice_shared_da(pre_hash_oid));
    }
  }
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(&shake, message);
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(
      &shake, Eurydice_array_to_slice_mut_d8(message_representative));
}

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients[[$5size_t]]

*/
typedef struct Option_e7_s {
  Option_3b_tags tag;
  Eurydice_arr_40 f0;
} Option_e7;

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256
with const generics
- OUTPUT_LENGTH= 576
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_portable_shake256_1b(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_5f0 *out) {
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_fa(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_x4_9b
with const generics
- OUT_LEN= 576
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_1b(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3,
    Eurydice_arr_5f0 *out0, Eurydice_arr_5f0 *out1, Eurydice_arr_5f0 *out2,
    Eurydice_arr_5f0 *out3) {
  libcrux_ml_dsa_hash_functions_portable_shake256_1b(input0, out0);
  libcrux_ml_dsa_hash_functions_portable_shake256_1b(input1, out1);
  libcrux_ml_dsa_hash_functions_portable_shake256_1b(input2, out2);
  libcrux_ml_dsa_hash_functions_portable_shake256_1b(input3, out3);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_gamma1_deserialize_37(
    size_t gamma1_exponent, Eurydice_borrow_slice_u8 serialized,
    Eurydice_arr_a4 *result) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(result),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_gamma1_deserialize_65(
        Eurydice_slice_subslice_shared_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 * (gamma1_exponent + (size_t)1U),
                (i0 + (size_t)1U) * (gamma1_exponent + (size_t)1U)})),
        &result->data[i0], gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256
with const generics
- OUTPUT_LENGTH= 640
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_hash_functions_portable_shake256_c8(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_c30 *out) {
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_7d(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for
libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_x4_9b
with const generics
- OUT_LEN= 640
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_c8(
    Eurydice_borrow_slice_u8 input0, Eurydice_borrow_slice_u8 input1,
    Eurydice_borrow_slice_u8 input2, Eurydice_borrow_slice_u8 input3,
    Eurydice_arr_c30 *out0, Eurydice_arr_c30 *out1, Eurydice_arr_c30 *out2,
    Eurydice_arr_c30 *out3) {
  libcrux_ml_dsa_hash_functions_portable_shake256_c8(input0, out0);
  libcrux_ml_dsa_hash_functions_portable_shake256_c8(input1, out1);
  libcrux_ml_dsa_hash_functions_portable_shake256_c8(input2, out2);
  libcrux_ml_dsa_hash_functions_portable_shake256_c8(input3, out3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_61
with const generics
- OUTPUT_LENGTH= 640
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_61_c8(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_c30 *out) {
  libcrux_ml_dsa_hash_functions_portable_shake256_c8(input, out);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof
for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_61
with const generics
- OUTPUT_LENGTH= 576
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_61_1b(
    Eurydice_borrow_slice_u8 input, Eurydice_arr_5f0 *out) {
  libcrux_ml_dsa_hash_functions_portable_shake256_1b(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_ring_element
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake256 with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_sample_sample_mask_ring_element_2e(
    const Eurydice_arr_a2 *seed, Eurydice_arr_a4 *result,
    size_t gamma1_exponent) {
  switch (gamma1_exponent) {
    case 17U: {
      break;
    }
    case 19U: {
      Eurydice_arr_c30 out = {{0U}};
      libcrux_ml_dsa_hash_functions_portable_shake256_61_c8(
          Eurydice_array_to_slice_shared_39(seed), &out);
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_7d0(&out), result);
      return;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                        "panic!");
      KRML_HOST_EXIT(255U);
    }
  }
  Eurydice_arr_5f0 out = {{0U}};
  libcrux_ml_dsa_hash_functions_portable_shake256_61_1b(
      Eurydice_array_to_slice_shared_39(seed), &out);
  libcrux_ml_dsa_encoding_gamma1_deserialize_37(
      gamma1_exponent, Eurydice_array_to_slice_shared_fa(&out), result);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_sample_sample_mask_vector_67(
    size_t dimension, size_t gamma1_exponent, const Eurydice_arr_06 *seed,
    uint16_t *domain_separator, Eurydice_dst_ref_mut_f3 mask) {
  Eurydice_arr_a2 seed0 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_d8(seed), domain_separator[0U]);
  Eurydice_arr_a2 seed1 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_d8(seed),
      (uint32_t)domain_separator[0U] + 1U);
  Eurydice_arr_a2 seed2 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_d8(seed),
      (uint32_t)domain_separator[0U] + 2U);
  Eurydice_arr_a2 seed3 = libcrux_ml_dsa_sample_add_error_domain_separator(
      Eurydice_array_to_slice_shared_d8(seed),
      (uint32_t)domain_separator[0U] + 3U);
  domain_separator[0U] = (uint32_t)domain_separator[0U] + 4U;
  switch (gamma1_exponent) {
    case 17U: {
      Eurydice_arr_5f0 out0 = {{0U}};
      Eurydice_arr_5f0 out1 = {{0U}};
      Eurydice_arr_5f0 out2 = {{0U}};
      Eurydice_arr_5f0 out3 = {{0U}};
      libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_1b(
          Eurydice_array_to_slice_shared_39(&seed0),
          Eurydice_array_to_slice_shared_39(&seed1),
          Eurydice_array_to_slice_shared_39(&seed2),
          Eurydice_array_to_slice_shared_39(&seed3), &out0, &out1, &out2,
          &out3);
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_fa(&out0),
          &Eurydice_slice_index_mut(mask, (size_t)0U, Eurydice_arr_a4));
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_fa(&out1),
          &Eurydice_slice_index_mut(mask, (size_t)1U, Eurydice_arr_a4));
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_fa(&out2),
          &Eurydice_slice_index_mut(mask, (size_t)2U, Eurydice_arr_a4));
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_fa(&out3),
          &Eurydice_slice_index_mut(mask, (size_t)3U, Eurydice_arr_a4));
      break;
    }
    case 19U: {
      Eurydice_arr_c30 out0 = {{0U}};
      Eurydice_arr_c30 out1 = {{0U}};
      Eurydice_arr_c30 out2 = {{0U}};
      Eurydice_arr_c30 out3 = {{0U}};
      libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_c8(
          Eurydice_array_to_slice_shared_39(&seed0),
          Eurydice_array_to_slice_shared_39(&seed1),
          Eurydice_array_to_slice_shared_39(&seed2),
          Eurydice_array_to_slice_shared_39(&seed3), &out0, &out1, &out2,
          &out3);
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_7d0(&out0),
          &Eurydice_slice_index_mut(mask, (size_t)0U, Eurydice_arr_a4));
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_7d0(&out1),
          &Eurydice_slice_index_mut(mask, (size_t)1U, Eurydice_arr_a4));
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_7d0(&out2),
          &Eurydice_slice_index_mut(mask, (size_t)2U, Eurydice_arr_a4));
      libcrux_ml_dsa_encoding_gamma1_deserialize_37(
          gamma1_exponent, Eurydice_array_to_slice_shared_7d0(&out3),
          &Eurydice_slice_index_mut(mask, (size_t)3U, Eurydice_arr_a4));
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
    Eurydice_arr_a2 seed4 = libcrux_ml_dsa_sample_add_error_domain_separator(
        Eurydice_array_to_slice_shared_d8(seed), domain_separator[0U]);
    domain_separator[0U] = (uint32_t)domain_separator[0U] + 1U;
    libcrux_ml_dsa_sample_sample_mask_ring_element_2e(
        &seed4, &Eurydice_slice_index_mut(mask, i0, Eurydice_arr_a4),
        gamma1_exponent);
  }
}

/**
 Compute InvertNTT(Â ◦ ŷ)
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_matrix_x_mask
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_compute_matrix_x_mask_37(
    size_t rows_in_a, size_t columns_in_a, Eurydice_dst_ref_shared_f3 matrix,
    Eurydice_dst_ref_shared_f3 mask, Eurydice_dst_ref_mut_f3 result) {
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < columns_in_a; i++) {
      size_t j = i;
      Eurydice_arr_a4 product =
          Eurydice_slice_index_shared(mask, j, Eurydice_arr_a4);
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(
          &product, &Eurydice_slice_index_shared(matrix, i1 * columns_in_a + j,
                                                 Eurydice_arr_a4));
      libcrux_ml_dsa_polynomial_add_ff_37(
          &Eurydice_slice_index_mut(result, i1, Eurydice_arr_a4), &product);
    }
    libcrux_ml_dsa_ntt_reduce_37(
        &Eurydice_slice_index_mut(result, i1, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(
        &Eurydice_slice_index_mut(result, i1, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement
libcrux_ml_dsa_simd_portable_vector_type_Coefficients with const generics
- N= 30
*/
static inline Eurydice_dst_ref_shared_f3 Eurydice_array_to_slice_shared_712(
    const Eurydice_arr_c9 *a) {
  Eurydice_dst_ref_shared_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)30U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.decompose_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_decompose_vector_37(
    size_t dimension, int32_t gamma2, Eurydice_dst_ref_shared_f3 t,
    Eurydice_dst_ref_mut_f3 low, Eurydice_dst_ref_mut_f3 high) {
  for (size_t i0 = (size_t)0U; i0 < dimension; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(
                 Eurydice_array_to_slice_shared_a2(&Eurydice_slice_index_shared(
                     (Eurydice_dst_ref_shared_f3{low.ptr, low.meta}),
                     (size_t)0U, Eurydice_arr_a4)),
                 Eurydice_arr_d4);
         i++) {
      size_t j = i;
      libcrux_ml_dsa_simd_portable_decompose_65(
          gamma2, &Eurydice_slice_index_shared(t, i1, Eurydice_arr_a4).data[j],
          &Eurydice_slice_index_mut(low, i1, Eurydice_arr_a4).data[j],
          &Eurydice_slice_index_mut(high, i1, Eurydice_arr_a4).data[j]);
    }
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_commitment_serialize_37(
    const Eurydice_arr_a4 *re, Eurydice_mut_borrow_slice_u8 serialized) {
  size_t output_bytes_per_simd_unit =
      Eurydice_slice_len(
          (Eurydice_borrow_slice_u8{serialized.ptr, serialized.meta}),
          uint8_t) /
      ((size_t)8U * (size_t)4U);
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_d4 *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_commitment_serialize_65(
        simd_unit,
        Eurydice_slice_subslice_mut_7e(
            serialized, (core_ops_range_Range_08{
                            i0 * output_bytes_per_simd_unit,
                            (i0 + (size_t)1U) * output_bytes_per_simd_unit})));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.commitment.serialize_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_commitment_serialize_vector_37(
    size_t ring_element_size, Eurydice_dst_ref_shared_f3 vector,
    Eurydice_mut_borrow_slice_u8 serialized) {
  size_t offset = (size_t)0U;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(vector, Eurydice_arr_a4);
       i++) {
    size_t _cloop_j = i;
    const Eurydice_arr_a4 *ring_element =
        &Eurydice_slice_index_shared(vector, _cloop_j, Eurydice_arr_a4);
    libcrux_ml_dsa_encoding_commitment_serialize_37(
        ring_element,
        Eurydice_slice_subslice_mut_7e(
            serialized,
            (core_ops_range_Range_08{offset, offset + ring_element_size})));
    offset = offset + ring_element_size;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_challenge_ring_element
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_hash_functions_portable_Shake256 with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_challenge_ring_element_2e(
    Eurydice_borrow_slice_u8 seed, size_t number_of_ones, Eurydice_arr_a4 *re) {
  Eurydice_arr_26 state =
      libcrux_ml_dsa_hash_functions_portable_init_absorb_final_61(seed);
  Eurydice_arr_3d randomness0 =
      libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_61(&state);
  Eurydice_array_u8x8 arr;
  memcpy(arr.data,
         Eurydice_array_to_subslice_shared_360(
             &randomness0, (core_ops_range_Range_08{(size_t)0U, (size_t)8U}))
             .ptr,
         (size_t)8U * sizeof(uint8_t));
  uint64_t signs = core_num__u64__from_le_bytes(
      unwrap_26_ab(Result_a4_s(Ok, &Result_a4_s::U::case_Ok, arr)));
  Eurydice_arr_c3 result = {{0U}};
  size_t out_index =
      Eurydice_slice_len(Eurydice_array_to_slice_shared_20(&result), int32_t) -
      number_of_ones;
  bool done = libcrux_ml_dsa_sample_inside_out_shuffle(
      Eurydice_array_to_subslice_from_shared_8c(&randomness0, (size_t)8U),
      &out_index, &signs, &result);
  while (true) {
    if (done) {
      break;
    } else {
      Eurydice_arr_3d randomness =
          libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_61(&state);
      done = libcrux_ml_dsa_sample_inside_out_shuffle(
          Eurydice_array_to_slice_shared_d4(&randomness), &out_index, &signs,
          &result);
    }
  }
  libcrux_ml_dsa_polynomial_from_i32_array_ff_37(
      Eurydice_array_to_slice_shared_20(&result), re);
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.vector_times_ring_element
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_vector_times_ring_element_37(
    Eurydice_dst_ref_mut_f3 vector, const Eurydice_arr_a4 *ring_element) {
  for (size_t i = (size_t)0U;
       i <
       Eurydice_slice_len((Eurydice_dst_ref_shared_f3{vector.ptr, vector.meta}),
                          Eurydice_arr_a4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(
        &Eurydice_slice_index_mut(vector, i0, Eurydice_arr_a4), ring_element);
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(
        &Eurydice_slice_index_mut(vector, i0, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.add_vectors
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_add_vectors_37(
    size_t dimension, Eurydice_dst_ref_mut_f3 lhs,
    Eurydice_dst_ref_shared_f3 rhs) {
  for (size_t i = (size_t)0U; i < dimension; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_add_ff_37(
        &Eurydice_slice_index_mut(lhs, i0, Eurydice_arr_a4),
        &Eurydice_slice_index_shared(rhs, i0, Eurydice_arr_a4));
  }
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.subtract_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_polynomial_subtract_ff_37(
    Eurydice_arr_a4 *self, const Eurydice_arr_a4 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(self),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_subtract_65(&self->data[i0], &rhs->data[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.matrix.subtract_vectors
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_subtract_vectors_37(
    size_t dimension, Eurydice_dst_ref_mut_f3 lhs,
    Eurydice_dst_ref_shared_f3 rhs) {
  for (size_t i = (size_t)0U; i < dimension; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_subtract_ff_37(
        &Eurydice_slice_index_mut(lhs, i0, Eurydice_arr_a4),
        &Eurydice_slice_index_shared(rhs, i0, Eurydice_arr_a4));
  }
}

/**
This function found in impl
{libcrux_ml_dsa::polynomial::PolynomialRingElement<SIMDUnit>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_dsa.polynomial.infinity_norm_exceeds_ff
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_polynomial_infinity_norm_exceeds_ff_37(
    const Eurydice_arr_a4 *self, int32_t bound) {
  bool result = false;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(self),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    bool uu____0;
    if (result) {
      uu____0 = true;
    } else {
      uu____0 = libcrux_ml_dsa_simd_portable_infinity_norm_exceeds_65(
          &self->data[i0], bound);
    }
    result = uu____0;
  }
  return result;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.vector_infinity_norm_exceeds
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE bool
libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(
    Eurydice_dst_ref_shared_f3 vector, int32_t bound) {
  bool result = false;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(vector, Eurydice_arr_a4);
       i++) {
    size_t i0 = i;
    bool uu____0;
    if (result) {
      uu____0 = true;
    } else {
      uu____0 = libcrux_ml_dsa_polynomial_infinity_norm_exceeds_ff_37(
          &Eurydice_slice_index_shared(vector, i0, Eurydice_arr_a4), bound);
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
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline Eurydice_arr_c3 libcrux_ml_dsa_polynomial_to_i32_array_ff_37(
    const Eurydice_arr_a4 *self) {
  Eurydice_arr_c3 result = {{0U}};
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(self),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_to_coefficient_array_65(
        &self->data[i0],
        Eurydice_array_to_subslice_mut_7f(
            &result,
            (core_ops_range_Range_08{
                i0 * LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_SIMD_TRAITS_COEFFICIENTS_IN_SIMD_UNIT})));
  }
  return result;
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.make_hint
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE size_t libcrux_ml_dsa_arithmetic_make_hint_37(
    Eurydice_dst_ref_shared_f3 low, Eurydice_dst_ref_shared_f3 high,
    int32_t gamma2, Eurydice_dst_ref_mut_59 hint) {
  size_t true_hints = (size_t)0U;
  Eurydice_arr_a4 hint_simd = libcrux_ml_dsa_polynomial_zero_ff_37();
  for (size_t i0 = (size_t)0U; i0 < Eurydice_slice_len(low, Eurydice_arr_a4);
       i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(&hint_simd),
                                Eurydice_arr_d4);
         i++) {
      size_t j = i;
      size_t one_hints_count = libcrux_ml_dsa_simd_portable_compute_hint_65(
          &Eurydice_slice_index_shared(low, i1, Eurydice_arr_a4).data[j],
          &Eurydice_slice_index_shared(high, i1, Eurydice_arr_a4).data[j],
          gamma2, &hint_simd.data[j]);
      true_hints = true_hints + one_hints_count;
    }
    Eurydice_arr_c3 uu____0 =
        libcrux_ml_dsa_polynomial_to_i32_array_ff_37(&hint_simd);
    Eurydice_slice_index_mut(hint, i1, Eurydice_arr_c3) = uu____0;
  }
  return true_hints;
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.gamma1.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_gamma1_serialize_37(
    const Eurydice_arr_a4 *re, Eurydice_mut_borrow_slice_u8 serialized,
    size_t gamma1_exponent) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_d4 *simd_unit = &re->data[i0];
    libcrux_ml_dsa_simd_portable_gamma1_serialize_65(
        simd_unit,
        Eurydice_slice_subslice_mut_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 * (gamma1_exponent + (size_t)1U),
                (i0 + (size_t)1U) * (gamma1_exponent + (size_t)1U)})),
        gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.serialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_encoding_signature_serialize_37(
    Eurydice_borrow_slice_u8 commitment_hash,
    Eurydice_dst_ref_shared_f3 signer_response, Eurydice_dst_ref_shared_59 hint,
    size_t commitment_hash_size, size_t columns_in_a, size_t rows_in_a,
    size_t gamma1_exponent, size_t gamma1_ring_element_size,
    size_t max_ones_in_hint, Eurydice_mut_borrow_slice_u8 signature) {
  size_t offset = (size_t)0U;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_7e(
          signature,
          (core_ops_range_Range_08{offset, offset + commitment_hash_size})),
      commitment_hash, uint8_t);
  offset = offset + commitment_hash_size;
  for (size_t i = (size_t)0U; i < columns_in_a; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_gamma1_serialize_37(
        &Eurydice_slice_index_shared(signer_response, i0, Eurydice_arr_a4),
        Eurydice_slice_subslice_mut_7e(
            signature, (core_ops_range_Range_08{
                           offset, offset + gamma1_ring_element_size})),
        gamma1_exponent);
    offset = offset + gamma1_ring_element_size;
  }
  size_t true_hints_seen = (size_t)0U;
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(
                 Eurydice_array_to_slice_shared_20(
                     &Eurydice_slice_index_shared(hint, i1, Eurydice_arr_c3)),
                 int32_t);
         i++) {
      size_t j = i;
      if (Eurydice_slice_index_shared(hint, i1, Eurydice_arr_c3).data[j] ==
          (int32_t)1) {
        Eurydice_slice_index_mut(signature, offset + true_hints_seen, uint8_t) =
            (uint8_t)j;
        true_hints_seen++;
      }
    }
    Eurydice_slice_index_mut(signature, offset + max_ones_in_hint + i1,
                             uint8_t) = (uint8_t)true_hints_seen;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_internal
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_5a(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Option_84 domain_separation_context, Eurydice_arr_60 randomness,
    Eurydice_arr_96 *signature) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      signing_key, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 remaining_serialized0 = uu____0.snd;
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      remaining_serialized0, LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE,
      uint8_t, Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_signing = uu____1.fst;
  Eurydice_borrow_slice_u8 remaining_serialized1 = uu____1.snd;
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____2 = Eurydice_slice_split_at(
      remaining_serialized1,
      LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 verification_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 remaining_serialized2 = uu____2.snd;
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____3 = Eurydice_slice_split_at(
      remaining_serialized2,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE *
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      uint8_t, Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 s1_serialized = uu____3.fst;
  Eurydice_borrow_slice_u8 remaining_serialized = uu____3.snd;
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____4 = Eurydice_slice_split_at(
      remaining_serialized,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE *
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      uint8_t, Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 s2_serialized = uu____4.fst;
  Eurydice_borrow_slice_u8 t0_serialized = uu____4.snd;
  Eurydice_arr_40 s1_as_ntt;
  Eurydice_arr_a4 repeat_expression0[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s1_as_ntt.data, repeat_expression0,
         (size_t)5U * sizeof(Eurydice_arr_a4));
  Eurydice_arr_47 s2_as_ntt;
  Eurydice_arr_a4 repeat_expression1[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s2_as_ntt.data, repeat_expression1,
         (size_t)6U * sizeof(Eurydice_arr_a4));
  Eurydice_arr_47 t0_as_ntt;
  Eurydice_arr_a4 repeat_expression2[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression2[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t0_as_ntt.data, repeat_expression2,
         (size_t)6U * sizeof(Eurydice_arr_a4));
  libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
      s1_serialized, Eurydice_array_to_slice_mut_711(&s1_as_ntt));
  libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
      s2_serialized, Eurydice_array_to_slice_mut_712(&s2_as_ntt));
  libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_37(
      t0_serialized, Eurydice_array_to_slice_mut_712(&t0_as_ntt));
  Eurydice_arr_c9 matrix;
  Eurydice_arr_a4 repeat_expression3[30U];
  for (size_t i = (size_t)0U; i < (size_t)30U; i++) {
    repeat_expression3[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(matrix.data, repeat_expression3,
         (size_t)30U * sizeof(Eurydice_arr_a4));
  libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, seed_for_a,
      Eurydice_array_to_slice_mut_710(&matrix));
  Eurydice_arr_06 message_representative = {{0U}};
  libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(
      verification_key_hash, &domain_separation_context, message,
      &message_representative);
  Eurydice_arr_06 mask_seed = {{0U}};
  libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 shake0 =
      libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake0, seed_for_signing);
  libcrux_ml_dsa_hash_functions_portable_absorb_26(
      &shake0, Eurydice_array_to_slice_shared_6e(&randomness));
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
      &shake0, Eurydice_array_to_slice_shared_d8(&message_representative));
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(
      &shake0, Eurydice_array_to_slice_mut_d8(&mask_seed));
  uint16_t domain_separator_for_mask = 0U;
  size_t attempt = (size_t)0U;
  Option_91 commitment_hash0 = {None};
  Option_e7 signer_response0 = {None};
  Option_c9 hint0 = {None};
  while (attempt < LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN) {
    attempt++;
    Eurydice_arr_40 mask;
    Eurydice_arr_a4 repeat_expression4[5U];
    for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
      repeat_expression4[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(mask.data, repeat_expression4, (size_t)5U * sizeof(Eurydice_arr_a4));
    Eurydice_arr_47 w0;
    Eurydice_arr_a4 repeat_expression5[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
      repeat_expression5[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(w0.data, repeat_expression5, (size_t)6U * sizeof(Eurydice_arr_a4));
    Eurydice_arr_47 commitment;
    Eurydice_arr_a4 repeat_expression6[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
      repeat_expression6[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(commitment.data, repeat_expression6,
           (size_t)6U * sizeof(Eurydice_arr_a4));
    libcrux_ml_dsa_sample_sample_mask_vector_67(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT, &mask_seed,
        &domain_separator_for_mask, Eurydice_array_to_slice_mut_711(&mask));
    Eurydice_arr_47 a_x_mask;
    Eurydice_arr_a4 repeat_expression[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
      repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(a_x_mask.data, repeat_expression,
           (size_t)6U * sizeof(Eurydice_arr_a4));
    Eurydice_arr_40 mask_ntt =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)5U, &mask, Eurydice_arr_a4, Eurydice_arr_40);
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_71(&mask_ntt),
                                Eurydice_arr_a4);
         i++) {
      size_t i0 = i;
      libcrux_ml_dsa_ntt_ntt_37(&mask_ntt.data[i0]);
    }
    libcrux_ml_dsa_matrix_compute_matrix_x_mask_37(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        Eurydice_array_to_slice_shared_712(&matrix),
        Eurydice_array_to_slice_shared_71(&mask_ntt),
        Eurydice_array_to_slice_mut_712(&a_x_mask));
    libcrux_ml_dsa_arithmetic_decompose_vector_37(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
        Eurydice_array_to_slice_shared_711(&a_x_mask),
        Eurydice_array_to_slice_mut_712(&w0),
        Eurydice_array_to_slice_mut_712(&commitment));
    Eurydice_arr_5f commitment_hash_candidate = {{0U}};
    Eurydice_arr_56 commitment_serialized = {{0U}};
    libcrux_ml_dsa_encoding_commitment_serialize_vector_37(
        LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE,
        Eurydice_array_to_slice_shared_711(&commitment),
        Eurydice_array_to_slice_mut_ee(&commitment_serialized));
    libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 shake =
        libcrux_ml_dsa_hash_functions_portable_init_26();
    libcrux_ml_dsa_hash_functions_portable_absorb_26(
        &shake, Eurydice_array_to_slice_shared_d8(&message_representative));
    libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
        &shake, Eurydice_array_to_slice_shared_ee(&commitment_serialized));
    libcrux_ml_dsa_hash_functions_portable_squeeze_26(
        &shake, Eurydice_array_to_slice_mut_95(&commitment_hash_candidate));
    Eurydice_arr_a4 verifier_challenge = libcrux_ml_dsa_polynomial_zero_ff_37();
    libcrux_ml_dsa_sample_sample_challenge_ring_element_2e(
        Eurydice_array_to_slice_shared_95(&commitment_hash_candidate),
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE,
        &verifier_challenge);
    libcrux_ml_dsa_ntt_ntt_37(&verifier_challenge);
    Eurydice_arr_40 challenge_times_s1 =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)5U, &s1_as_ntt, Eurydice_arr_a4, Eurydice_arr_40);
    Eurydice_arr_47 challenge_times_s2 =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)6U, &s2_as_ntt, Eurydice_arr_a4, Eurydice_arr_47);
    libcrux_ml_dsa_matrix_vector_times_ring_element_37(
        Eurydice_array_to_slice_mut_711(&challenge_times_s1),
        &verifier_challenge);
    libcrux_ml_dsa_matrix_vector_times_ring_element_37(
        Eurydice_array_to_slice_mut_712(&challenge_times_s2),
        &verifier_challenge);
    libcrux_ml_dsa_matrix_add_vectors_37(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        Eurydice_array_to_slice_mut_711(&mask),
        Eurydice_array_to_slice_shared_71(&challenge_times_s1));
    libcrux_ml_dsa_matrix_subtract_vectors_37(
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        Eurydice_array_to_slice_mut_712(&w0),
        Eurydice_array_to_slice_shared_711(&challenge_times_s2));
    if (!libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(
            Eurydice_array_to_slice_shared_71(&mask),
            ((int32_t)1 << (uint32_t)
                 LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT) -
                LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)) {
      if (!libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(
              Eurydice_array_to_slice_shared_711(&w0),
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2 -
                  LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)) {
        Eurydice_arr_47 challenge_times_t0 =
            core_array__core__clone__Clone_for__Array_T__N___clone(
                (size_t)6U, &t0_as_ntt, Eurydice_arr_a4, Eurydice_arr_47);
        libcrux_ml_dsa_matrix_vector_times_ring_element_37(
            Eurydice_array_to_slice_mut_712(&challenge_times_t0),
            &verifier_challenge);
        if (!libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(
                Eurydice_array_to_slice_shared_711(&challenge_times_t0),
                LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2)) {
          libcrux_ml_dsa_matrix_add_vectors_37(
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
              Eurydice_array_to_slice_mut_712(&w0),
              Eurydice_array_to_slice_shared_711(&challenge_times_t0));
          Eurydice_arr_e6 hint_candidate = {
              {{{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}}};
          size_t ones_in_hint = libcrux_ml_dsa_arithmetic_make_hint_37(
              Eurydice_array_to_slice_shared_711(&w0),
              Eurydice_array_to_slice_shared_711(&commitment),
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
              Eurydice_array_to_slice_mut_6d(&hint_candidate));
          if (!(ones_in_hint >
                LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT)) {
            attempt = LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN;
            commitment_hash0 = (Option_91{Some, commitment_hash_candidate});
            signer_response0 = (Option_e7{Some, mask});
            hint0 = (Option_c9{Some, hint_candidate});
          }
        }
      }
    }
  }
  Result_53 uu____5;
  if (commitment_hash0.tag == None) {
    uu____5 = (Result_53{
        Err, libcrux_ml_dsa_types_SigningError_RejectionSamplingError});
  } else {
    Eurydice_arr_5f commitment_hash = commitment_hash0.f0;
    Eurydice_arr_5f commitment_hash1 = commitment_hash;
    if (signer_response0.tag == None) {
      uu____5 = (Result_53{
          Err, libcrux_ml_dsa_types_SigningError_RejectionSamplingError});
    } else {
      Eurydice_arr_40 signer_response = signer_response0.f0;
      Eurydice_arr_40 signer_response1 = signer_response;
      if (!(hint0.tag == None)) {
        Eurydice_arr_e6 hint = hint0.f0;
        Eurydice_arr_e6 hint1 = hint;
        libcrux_ml_dsa_encoding_signature_serialize_37(
            Eurydice_array_to_slice_shared_95(&commitment_hash1),
            Eurydice_array_to_slice_shared_71(&signer_response1),
            Eurydice_array_to_slice_shared_6d(&hint1),
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
            LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE,
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,
            Eurydice_array_to_slice_mut_ee0(signature));
        return (Result_53{Ok});
      }
      uu____5 = (Result_53{
          Err, libcrux_ml_dsa_types_SigningError_RejectionSamplingError});
    }
  }
  return uu____5;
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_mut
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_5a(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness,
    Eurydice_arr_96 *signature) {
  Result_a8 uu____0 =
      libcrux_ml_dsa_pre_hash_new_88(context, (Option_3b{None}));
  if (!(uu____0.tag == Ok)) {
    return (
        Result_53{Err, libcrux_ml_dsa_types_SigningError_ContextTooLongError});
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
      dsc;
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_5a(
      signing_key, message, (Option_84{Some, domain_separation_context}),
      randomness, signature);
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_portable_Shake256X4 with const generics

*/
static KRML_MUSTINLINE Result_2e
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_5a(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness) {
  Eurydice_arr_96 signature = libcrux_ml_dsa_types_zero_c5_fa();
  Result_53 uu____0 = libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_5a(
      signing_key, message, context, randomness, &signature);
  Result_2e uu____1;
  if (uu____0.tag == Ok) {
    uu____1 = Result_2e_s(Ok, &Result_2e_s::U::case_Ok, signature);
  } else {
    libcrux_ml_dsa_types_SigningError e = uu____0.f0;
    uu____1 = Result_2e_s(Err, &Result_2e_s::U::case_Err, e);
  }
  return uu____1;
}

/**
 Sign.
*/
static inline Result_2e
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign(
    const Eurydice_arr_d10 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_5a(
      Eurydice_array_to_slice_shared_ef(signing_key), message, context,
      randomness);
}

/**
 Generate an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline Result_2e libcrux_ml_dsa_ml_dsa_65_portable_sign(
    const Eurydice_arr_d10 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign(
      libcrux_ml_dsa_types_as_ref_9b_09(signing_key), message, context,
      randomness);
}

/**
 Sign.
*/
static inline Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign_mut(
    const Eurydice_arr_d10 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness,
    Eurydice_arr_96 *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_5a(
      Eurydice_array_to_slice_shared_ef(signing_key), message, context,
      randomness, signature);
}

/**
 Generate an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline Result_53 libcrux_ml_dsa_ml_dsa_65_portable_sign_mut(
    const Eurydice_arr_d10 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness,
    Eurydice_arr_96 *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign_mut(
      signing_key, message, context, randomness, signature);
}

/**
This function found in impl {libcrux_ml_dsa::pre_hash::PreHash for
libcrux_ml_dsa::pre_hash::SHAKE128_PH}
*/
/**
A monomorphic instance of libcrux_ml_dsa.pre_hash.hash_30
with types libcrux_ml_dsa_hash_functions_portable_Shake128
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_pre_hash_hash_30_83(
    Eurydice_borrow_slice_u8 message, Eurydice_mut_borrow_slice_u8 output) {
  libcrux_ml_dsa_hash_functions_portable_shake128_7b(message, output);
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed_mut with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_portable_Shake256X4,
libcrux_ml_dsa_pre_hash_SHAKE128_PH with const generics

*/
static KRML_MUSTINLINE Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_3f(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_60 randomness,
    Eurydice_arr_96 *signature) {
  if (!(Eurydice_slice_len(context, uint8_t) >
        LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN)) {
    libcrux_ml_dsa_pre_hash_hash_30_83(message, pre_hash_buffer);
    Result_a8 uu____0 = libcrux_ml_dsa_pre_hash_new_88(
        context, (Option_3b{Some, libcrux_ml_dsa_pre_hash_oid_30()}));
    if (!(uu____0.tag == Ok)) {
      return (Result_53{Err,
                        libcrux_ml_dsa_types_SigningError_ContextTooLongError});
    }
    libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
    libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
        dsc;
    return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_5a(
        signing_key,
        (Eurydice_borrow_slice_u8{pre_hash_buffer.ptr, pre_hash_buffer.meta}),
        (Option_84{Some, domain_separation_context}), randomness, signature);
  }
  return (
      Result_53{Err, libcrux_ml_dsa_types_SigningError_ContextTooLongError});
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_hash_functions_portable_Shake256X4,
libcrux_ml_dsa_pre_hash_SHAKE128_PH with const generics

*/
static KRML_MUSTINLINE Result_2e
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_3f(
    Eurydice_borrow_slice_u8 signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_60 randomness) {
  Eurydice_arr_96 signature = libcrux_ml_dsa_types_zero_c5_fa();
  Result_53 uu____0 =
      libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_3f(
          signing_key, message, context, pre_hash_buffer, randomness,
          &signature);
  Result_2e uu____1;
  if (uu____0.tag == Ok) {
    uu____1 = Result_2e_s(Ok, &Result_2e_s::U::case_Ok, signature);
  } else {
    libcrux_ml_dsa_types_SigningError e = uu____0.f0;
    uu____1 = Result_2e_s(Err, &Result_2e_s::U::case_Err, e);
  }
  return uu____1;
}

/**
 Sign (pre-hashed).
*/
static inline Result_2e
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign_pre_hashed_shake128(
    const Eurydice_arr_d10 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer, Eurydice_arr_60 randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_3f(
      Eurydice_array_to_slice_shared_ef(signing_key), message, context,
      pre_hash_buffer, randomness);
}

/**
 Generate a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline Result_2e
libcrux_ml_dsa_ml_dsa_65_portable_sign_pre_hashed_shake128(
    const Eurydice_arr_d10 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_60 randomness) {
  Eurydice_arr_60 pre_hash_buffer = {{0U}};
  const Eurydice_arr_d10 *uu____0 =
      libcrux_ml_dsa_types_as_ref_9b_09(signing_key);
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign_pre_hashed_shake128(
      uu____0, message, context,
      Eurydice_array_to_slice_mut_6e(&pre_hash_buffer), randomness);
}

/**
A monomorphic instance of core.result.Result
with types (), libcrux_ml_dsa_types_VerificationError

*/
typedef struct Result_41_s {
  Result_a4_tags tag;
  libcrux_ml_dsa_types_VerificationError f0;
} Result_41;

/**
A monomorphic instance of libcrux_ml_dsa.encoding.t1.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline void libcrux_ml_dsa_encoding_t1_deserialize_37(
    Eurydice_borrow_slice_u8 serialized, Eurydice_arr_a4 *result) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(result),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_t1_deserialize_65(
        Eurydice_slice_subslice_shared_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 * LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_ENCODING_T1_DESERIALIZE_WINDOW})),
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
    size_t rows_in_a, size_t verification_key_size,
    Eurydice_borrow_slice_u8 serialized, Eurydice_dst_ref_mut_f3 t1) {
  for (size_t i = (size_t)0U; i < rows_in_a; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_t1_deserialize_37(
        Eurydice_slice_subslice_shared_7e(
            serialized,
            (core_ops_range_Range_08{
                i0 * LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE,
                (i0 + (size_t)1U) *
                    LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T1S_SIZE})),
        &Eurydice_slice_index_mut(t1, i0, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signature.deserialize
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE Result_41
libcrux_ml_dsa_encoding_signature_deserialize_37(
    size_t columns_in_a, size_t rows_in_a, size_t commitment_hash_size,
    size_t gamma1_exponent, size_t gamma1_ring_element_size,
    size_t max_ones_in_hint, size_t signature_size,
    Eurydice_borrow_slice_u8 serialized,
    Eurydice_mut_borrow_slice_u8 out_commitment_hash,
    Eurydice_dst_ref_mut_f3 out_signer_response,
    Eurydice_dst_ref_mut_59 out_hint) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 =
      Eurydice_slice_split_at(serialized, commitment_hash_size, uint8_t,
                              Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 commitment_hash = uu____0.fst;
  Eurydice_borrow_slice_u8 rest_of_serialized = uu____0.snd;
  Eurydice_slice_copy(
      Eurydice_slice_subslice_mut_7e(
          out_commitment_hash,
          (core_ops_range_Range_08{(size_t)0U, commitment_hash_size})),
      commitment_hash, uint8_t);
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      rest_of_serialized, gamma1_ring_element_size * columns_in_a, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 signer_response_serialized = uu____1.fst;
  Eurydice_borrow_slice_u8 hint_serialized = uu____1.snd;
  for (size_t i = (size_t)0U; i < columns_in_a; i++) {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_gamma1_deserialize_37(
        gamma1_exponent,
        Eurydice_slice_subslice_shared_7e(
            signer_response_serialized,
            (core_ops_range_Range_08{
                i0 * gamma1_ring_element_size,
                (i0 + (size_t)1U) * gamma1_ring_element_size})),
        &Eurydice_slice_index_mut(out_signer_response, i0, Eurydice_arr_a4));
  }
  size_t previous_true_hints_seen = (size_t)0U;
  bool malformed_hint = false;
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    size_t current_true_hints_seen = (size_t)Eurydice_slice_index_shared(
        hint_serialized, max_ones_in_hint + i1, uint8_t);
    if (current_true_hints_seen < previous_true_hints_seen) {
      malformed_hint = true;
    } else if (current_true_hints_seen > max_ones_in_hint) {
      malformed_hint = true;
    } else {
      for (size_t i = previous_true_hints_seen; i < current_true_hints_seen;
           i++) {
        size_t j = i;
        if (j > previous_true_hints_seen) {
          if (Eurydice_slice_index_shared(hint_serialized, j, uint8_t) <=
              Eurydice_slice_index_shared(hint_serialized, j - (size_t)1U,
                                          uint8_t)) {
            malformed_hint = true;
            break;
          }
        }
        libcrux_ml_dsa_encoding_signature_set_hint(
            out_hint, i1,
            (size_t)Eurydice_slice_index_shared(hint_serialized, j, uint8_t));
      }
      if (!malformed_hint) {
        previous_true_hints_seen = current_true_hints_seen;
        continue;
      }
    }
    break;
  }
  for (size_t i = previous_true_hints_seen; i < max_ones_in_hint; i++) {
    size_t j = i;
    if (!(Eurydice_slice_index_shared(hint_serialized, j, uint8_t) != 0U)) {
      continue;
    }
    malformed_hint = true;
    break;
  }
  Result_41 uu____2;
  if (malformed_hint) {
    uu____2 = (Result_41{
        Err, libcrux_ml_dsa_types_VerificationError_MalformedHintError});
  } else {
    uu____2 = (Result_41{Ok});
  }
  return uu____2;
}

/**
A monomorphic instance of
libcrux_ml_dsa.simd.portable.arithmetic.shift_left_then_reduce with const
generics
- SHIFT_BY= 13
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_simd_portable_arithmetic_shift_left_then_reduce_84(
    Eurydice_arr_d4 *simd_unit) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a7(simd_unit),
                              int32_t);
       i++) {
    size_t i0 = i;
    simd_unit->data[i0] =
        libcrux_ml_dsa_simd_portable_arithmetic_reduce_element(
            simd_unit->data[i0] << (uint32_t)(int32_t)13);
  }
}

/**
This function found in impl {libcrux_ml_dsa::simd::traits::Operations for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
/**
A monomorphic instance of libcrux_ml_dsa.simd.portable.shift_left_then_reduce_65
with const generics
- SHIFT_BY= 13
*/
static inline void libcrux_ml_dsa_simd_portable_shift_left_then_reduce_65_84(
    Eurydice_arr_d4 *simd_unit) {
  libcrux_ml_dsa_simd_portable_arithmetic_shift_left_then_reduce_84(simd_unit);
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.shift_left_then_reduce
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- SHIFT_BY= 13
*/
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_shift_left_then_reduce_68(
    Eurydice_arr_a4 *re) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_a2(re),
                              Eurydice_arr_d4);
       i++) {
    size_t i0 = i;
    libcrux_ml_dsa_simd_portable_shift_left_then_reduce_65_84(&re->data[i0]);
  }
}

/**
 Compute InvertNTT(Â ◦ ẑ - ĉ ◦ NTT(t₁2ᵈ))
*/
/**
A monomorphic instance of libcrux_ml_dsa.matrix.compute_w_approx
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_matrix_compute_w_approx_37(
    size_t rows_in_a, size_t columns_in_a, Eurydice_dst_ref_shared_f3 matrix,
    Eurydice_dst_ref_shared_f3 signer_response,
    const Eurydice_arr_a4 *verifier_challenge_as_ntt,
    Eurydice_dst_ref_mut_f3 t1) {
  for (size_t i0 = (size_t)0U; i0 < rows_in_a; i0++) {
    size_t i1 = i0;
    Eurydice_arr_a4 inner_result = libcrux_ml_dsa_polynomial_zero_ff_37();
    for (size_t i = (size_t)0U; i < columns_in_a; i++) {
      size_t j = i;
      Eurydice_arr_a4 product = Eurydice_slice_index_shared(
          matrix, i1 * columns_in_a + j, Eurydice_arr_a4);
      libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(
          &product,
          &Eurydice_slice_index_shared(signer_response, j, Eurydice_arr_a4));
      libcrux_ml_dsa_polynomial_add_ff_37(&inner_result, &product);
    }
    libcrux_ml_dsa_arithmetic_shift_left_then_reduce_68(
        &Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_ntt_37(
        &Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_ntt_multiply_montgomery_37(
        &Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4),
        verifier_challenge_as_ntt);
    libcrux_ml_dsa_polynomial_subtract_ff_37(
        &inner_result, &Eurydice_slice_index_shared(
                           (Eurydice_dst_ref_shared_f3{t1.ptr, t1.meta}), i1,
                           Eurydice_arr_a4));
    Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4) = inner_result;
    libcrux_ml_dsa_ntt_reduce_37(
        &Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4));
    libcrux_ml_dsa_ntt_invert_ntt_montgomery_37(
        &Eurydice_slice_index_mut(t1, i1, Eurydice_arr_a4));
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.arithmetic.use_hint
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_dsa_arithmetic_use_hint_37(
    int32_t gamma2, Eurydice_dst_ref_shared_59 hint,
    Eurydice_dst_ref_mut_f3 re_vector) {
  for (size_t i0 = (size_t)0U;
       i0 < Eurydice_slice_len(
                (Eurydice_dst_ref_shared_f3{re_vector.ptr, re_vector.meta}),
                Eurydice_arr_a4);
       i0++) {
    size_t i1 = i0;
    Eurydice_arr_a4 tmp = libcrux_ml_dsa_polynomial_zero_ff_37();
    libcrux_ml_dsa_polynomial_from_i32_array_ff_37(
        Eurydice_array_to_slice_shared_20(
            &Eurydice_slice_index_shared(hint, i1, Eurydice_arr_c3)),
        &tmp);
    for (size_t i = (size_t)0U;
         i <
         Eurydice_slice_len(
             Eurydice_array_to_slice_shared_a2(&Eurydice_slice_index_shared(
                 (Eurydice_dst_ref_shared_f3{re_vector.ptr, re_vector.meta}),
                 (size_t)0U, Eurydice_arr_a4)),
             Eurydice_arr_d4);
         i++) {
      size_t j = i;
      libcrux_ml_dsa_simd_portable_use_hint_65(
          gamma2,
          &Eurydice_slice_index_shared(
               (Eurydice_dst_ref_shared_f3{re_vector.ptr, re_vector.meta}), i1,
               Eurydice_arr_a4)
               .data[j],
          &tmp.data[j]);
    }
    Eurydice_slice_index_mut(re_vector, i1, Eurydice_arr_a4) = tmp;
  }
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_internal with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof with const generics

*/
static KRML_MUSTINLINE Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_5a(
    const Eurydice_arr_4a *verification_key, Eurydice_borrow_slice_u8 message,
    Option_84 domain_separation_context,
    const Eurydice_arr_96 *signature_serialized) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_5b(verification_key),
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 t1_serialized = uu____0.snd;
  Eurydice_arr_47 t1;
  Eurydice_arr_a4 repeat_expression0[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++) {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t1.data, repeat_expression0, (size_t)6U * sizeof(Eurydice_arr_a4));
  libcrux_ml_dsa_encoding_verification_key_deserialize_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_VERIFICATION_KEY_SIZE,
      t1_serialized, Eurydice_array_to_slice_mut_712(&t1));
  Eurydice_arr_5f deserialized_commitment_hash = {{0U}};
  Eurydice_arr_40 deserialized_signer_response;
  Eurydice_arr_a4 repeat_expression1[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(deserialized_signer_response.data, repeat_expression1,
         (size_t)5U * sizeof(Eurydice_arr_a4));
  Eurydice_arr_e6 deserialized_hint = {
      {{{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}, {{0U}}}};
  Result_41 uu____1 = libcrux_ml_dsa_encoding_signature_deserialize_37(
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNATURE_SIZE,
      Eurydice_array_to_slice_shared_ee0(signature_serialized),
      Eurydice_array_to_slice_mut_95(&deserialized_commitment_hash),
      Eurydice_array_to_slice_mut_711(&deserialized_signer_response),
      Eurydice_array_to_slice_mut_6d(&deserialized_hint));
  Result_41 uu____2;
  if (uu____1.tag == Ok) {
    if (libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(
            Eurydice_array_to_slice_shared_71(&deserialized_signer_response),
            ((int32_t)1 << (uint32_t)
                 LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT) -
                LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)) {
      uu____2 = (Result_41{
          Err,
          libcrux_ml_dsa_types_VerificationError_SignerResponseExceedsBoundError});
    } else {
      Eurydice_arr_c9 matrix;
      Eurydice_arr_a4 repeat_expression[30U];
      for (size_t i = (size_t)0U; i < (size_t)30U; i++) {
        repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
      }
      memcpy(matrix.data, repeat_expression,
             (size_t)30U * sizeof(Eurydice_arr_a4));
      libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A, seed_for_a,
          Eurydice_array_to_slice_mut_710(&matrix));
      Eurydice_arr_06 verification_key_hash = {{0U}};
      libcrux_ml_dsa_hash_functions_portable_shake256_61_24(
          Eurydice_array_to_slice_shared_5b(verification_key),
          &verification_key_hash);
      Eurydice_arr_06 message_representative = {{0U}};
      libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(
          Eurydice_array_to_slice_shared_d8(&verification_key_hash),
          &domain_separation_context, message, &message_representative);
      Eurydice_arr_a4 verifier_challenge =
          libcrux_ml_dsa_polynomial_zero_ff_37();
      libcrux_ml_dsa_sample_sample_challenge_ring_element_2e(
          Eurydice_array_to_slice_shared_95(&deserialized_commitment_hash),
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE,
          &verifier_challenge);
      libcrux_ml_dsa_ntt_ntt_37(&verifier_challenge);
      for (size_t i = (size_t)0U;
           i < Eurydice_slice_len(Eurydice_array_to_slice_shared_71(
                                      &deserialized_signer_response),
                                  Eurydice_arr_a4);
           i++) {
        size_t i0 = i;
        libcrux_ml_dsa_ntt_ntt_37(&deserialized_signer_response.data[i0]);
      }
      libcrux_ml_dsa_matrix_compute_w_approx_37(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
          Eurydice_array_to_slice_shared_712(&matrix),
          Eurydice_array_to_slice_shared_71(&deserialized_signer_response),
          &verifier_challenge, Eurydice_array_to_slice_mut_712(&t1));
      Eurydice_arr_5f recomputed_commitment_hash = {{0U}};
      libcrux_ml_dsa_arithmetic_use_hint_37(
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
          Eurydice_array_to_slice_shared_6d(&deserialized_hint),
          Eurydice_array_to_slice_mut_712(&t1));
      Eurydice_arr_56 commitment_serialized = {{0U}};
      libcrux_ml_dsa_encoding_commitment_serialize_vector_37(
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE,
          Eurydice_array_to_slice_shared_711(&t1),
          Eurydice_array_to_slice_mut_ee(&commitment_serialized));
      libcrux_sha3_generic_keccak_xof_KeccakXofState_e2 shake =
          libcrux_ml_dsa_hash_functions_portable_init_26();
      libcrux_ml_dsa_hash_functions_portable_absorb_26(
          &shake, Eurydice_array_to_slice_shared_d8(&message_representative));
      libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
          &shake, Eurydice_array_to_slice_shared_ee(&commitment_serialized));
      libcrux_ml_dsa_hash_functions_portable_squeeze_26(
          &shake, Eurydice_array_to_slice_mut_95(&recomputed_commitment_hash));
      if (Eurydice_array_eq((size_t)48U, &deserialized_commitment_hash,
                            &recomputed_commitment_hash, uint8_t)) {
        uu____2 = (Result_41{Ok});
      } else {
        uu____2 = (Result_41{
            Err,
            libcrux_ml_dsa_types_VerificationError_CommitmentHashesDontMatchError});
      }
    }
  } else {
    libcrux_ml_dsa_types_VerificationError e = uu____1.f0;
    uu____2 = (Result_41{Err, e});
  }
  return uu____2;
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof with const generics

*/
static KRML_MUSTINLINE Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_5a(
    const Eurydice_arr_4a *verification_key_serialized,
    Eurydice_borrow_slice_u8 message, Eurydice_borrow_slice_u8 context,
    const Eurydice_arr_96 *signature_serialized) {
  Result_a8 uu____0 =
      libcrux_ml_dsa_pre_hash_new_88(context, (Option_3b{None}));
  if (!(uu____0.tag == Ok)) {
    return (Result_41{
        Err,
        libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError});
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
      dsc;
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_5a(
      verification_key_serialized, message,
      (Option_84{Some, domain_separation_context}), signature_serialized);
}

/**
 Verify.
*/
static inline Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_verify(
    const Eurydice_arr_4a *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_96 *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_5a(
      verification_key, message, context, signature);
}

/**
 Verify an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline Result_41 libcrux_ml_dsa_ml_dsa_65_portable_verify(
    const Eurydice_arr_4a *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_96 *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_verify(
      libcrux_ml_dsa_types_as_ref_7f_97(verification_key), message, context,
      libcrux_ml_dsa_types_as_ref_c5_fa(signature));
}

/**
A monomorphic instance of
libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_pre_hashed with types
libcrux_ml_dsa_simd_portable_vector_type_Coefficients,
libcrux_ml_dsa_samplex4_portable_PortableSampler,
libcrux_ml_dsa_hash_functions_portable_Shake128,
libcrux_ml_dsa_hash_functions_portable_Shake128X4,
libcrux_ml_dsa_hash_functions_portable_Shake256,
libcrux_ml_dsa_hash_functions_portable_Shake256Xof,
libcrux_ml_dsa_pre_hash_SHAKE128_PH with const generics

*/
static KRML_MUSTINLINE Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_3f(
    const Eurydice_arr_4a *verification_key_serialized,
    Eurydice_borrow_slice_u8 message, Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
    const Eurydice_arr_96 *signature_serialized) {
  libcrux_ml_dsa_pre_hash_hash_30_83(message, pre_hash_buffer);
  Result_a8 uu____0 = libcrux_ml_dsa_pre_hash_new_88(
      context, (Option_3b{Some, libcrux_ml_dsa_pre_hash_oid_30()}));
  if (!(uu____0.tag == Ok)) {
    return (Result_41{
        Err,
        libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError});
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context =
      dsc;
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_5a(
      verification_key_serialized,
      (Eurydice_borrow_slice_u8{pre_hash_buffer.ptr, pre_hash_buffer.meta}),
      (Option_84{Some, domain_separation_context}), signature_serialized);
}

/**
 Verify (pre-hashed with SHAKE-128).
*/
static inline Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_verify_pre_hashed_shake128(
    const Eurydice_arr_4a *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context,
    Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
    const Eurydice_arr_96 *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_3f(
      verification_key, message, context, pre_hash_buffer, signature);
}

/**
 Verify a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline Result_41
libcrux_ml_dsa_ml_dsa_65_portable_verify_pre_hashed_shake128(
    const Eurydice_arr_4a *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_96 *signature) {
  Eurydice_arr_60 pre_hash_buffer = {{0U}};
  const Eurydice_arr_4a *uu____0 =
      libcrux_ml_dsa_types_as_ref_7f_97(verification_key);
  Eurydice_borrow_slice_u8 uu____1 = message;
  Eurydice_borrow_slice_u8 uu____2 = context;
  Eurydice_mut_borrow_slice_u8 uu____3 =
      Eurydice_array_to_slice_mut_6e(&pre_hash_buffer);
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_verify_pre_hashed_shake128(
      uu____0, uu____1, uu____2, uu____3,
      libcrux_ml_dsa_types_as_ref_c5_fa(signature));
}

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_VECTOR_SIZE    \
  (libcrux_ml_dsa_constants_commitment_vector_size(                       \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_BITS_PER_COMMITMENT_COEFFICIENT, \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A))

typedef libcrux_ml_dsa_types_MLDSAKeyPair_06
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65KeyPair;

typedef Eurydice_arr_96
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65Signature;

typedef Eurydice_arr_d10
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65SigningKey;

typedef Eurydice_arr_4a
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65VerificationKey;

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ROW_COLUMN \
  (LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A +          \
   LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A)

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ROW_X_COLUMN \
  (LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A *            \
   LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A)

#define LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNING_KEY_SIZE \
  (libcrux_ml_dsa_constants_signing_key_size(                    \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,              \
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,           \
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE))

#define LIBCRUX_ML_DSA_PRE_HASH_PRE_HASH_OID_LEN ((size_t)11U)

typedef Eurydice_arr_cb libcrux_ml_dsa_pre_hash_PreHashOID;

/**
This function found in impl
{core::convert::From<libcrux_ml_dsa::pre_hash::DomainSeparationError> for
libcrux_ml_dsa::types::SigningError}
*/
static inline libcrux_ml_dsa_types_SigningError libcrux_ml_dsa_pre_hash_from_96(
    libcrux_ml_dsa_pre_hash_DomainSeparationError e) {
  return libcrux_ml_dsa_types_SigningError_ContextTooLongError;
}

/**
This function found in impl
{core::convert::From<libcrux_ml_dsa::pre_hash::DomainSeparationError> for
libcrux_ml_dsa::types::VerificationError}
*/
static inline libcrux_ml_dsa_types_VerificationError
libcrux_ml_dsa_pre_hash_from_bf(
    libcrux_ml_dsa_pre_hash_DomainSeparationError e) {
  return libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError;
}

/**
This function found in impl {core::clone::Clone for
libcrux_ml_dsa::simd::portable::vector_type::Coefficients}
*/
static inline Eurydice_arr_d4 libcrux_ml_dsa_simd_portable_vector_type_clone_a5(
    const Eurydice_arr_d4 *self) {
  return self[0U];
}

typedef int32_t libcrux_ml_dsa_simd_traits_FieldElementTimesMontgomeryR;

typedef int32_t libcrux_ml_dsa_simd_portable_vector_type_FieldElement;

typedef Result_a8 libcrux_ml_dsa_pre_hash_PreHashResult;

#define libcrux_mldsa65_portable_H_DEFINED
#endif /* libcrux_mldsa65_portable_H */
