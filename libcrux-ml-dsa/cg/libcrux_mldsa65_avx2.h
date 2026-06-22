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
 * Libcrux: c5f306978fa7d13b590fd71dbc260386f3936d57
 */

#ifndef libcrux_mldsa65_avx2_H
#define libcrux_mldsa65_avx2_H

#include "eurydice_glue.h"
#include "intrinsics/libcrux_intrinsics_avx2.h"
#include "libcrux_mldsa_avx2.h"
#include "libcrux_mldsa_core.h"

/**
 Generate an ML-DSA-65 Key Pair
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65KeyPair
libcrux_ml_dsa_ml_dsa_65_avx2_generate_key_pair(Eurydice_arr_ec randomness) {
  Eurydice_arr_24 signing_key = {{0U}};
  Eurydice_arr_29 verification_key = {{0U}};
  libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair(
      randomness, Eurydice_array_to_slice_mut_98(&signing_key),
      Eurydice_array_to_slice_mut_37(&verification_key));
  return (libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65KeyPair{
      libcrux_ml_dsa_types_new_9b_e5(signing_key),
      libcrux_ml_dsa_types_new_7f_a2(verification_key)});
}

/**
 Generate an ML-DSA-65 Key Pair
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void libcrux_ml_dsa_ml_dsa_65_avx2_generate_key_pair_mut(
    Eurydice_arr_ec randomness, Eurydice_arr_24 *signing_key,
    Eurydice_arr_29 *verification_key) {
  libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_generate_key_pair(
      randomness, Eurydice_array_to_slice_mut_98(signing_key),
      Eurydice_array_to_slice_mut_37(verification_key));
}

/**
 Generate an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_8c libcrux_ml_dsa_ml_dsa_65_avx2_sign(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign(
      libcrux_ml_dsa_types_as_ref_9b_e5(signing_key), message, context,
      randomness);
}

/**
 Generate an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_53 libcrux_ml_dsa_ml_dsa_65_avx2_sign_mut(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness,
    Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_mut(
      signing_key, message, context, randomness, signature);
}

/**
 Generate a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_65_avx2_sign_pre_hashed_shake128(
    const Eurydice_arr_24 *signing_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, Eurydice_arr_ec randomness) {
  Eurydice_arr_ec pre_hash_buffer = {{0U}};
  const Eurydice_arr_24 *uu____0 =
      libcrux_ml_dsa_types_as_ref_9b_e5(signing_key);
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_sign_pre_hashed_shake128(
      uu____0, message, context,
      Eurydice_array_to_slice_mut_01(&pre_hash_buffer), randomness);
}

/**
 Verify an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_41 libcrux_ml_dsa_ml_dsa_65_avx2_verify(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_0c *signature) {
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify(
      libcrux_ml_dsa_types_as_ref_7f_a2(verification_key), message, context,
      libcrux_ml_dsa_types_as_ref_c5_5c(signature));
}

/**
 Verify a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_65_avx2_verify_pre_hashed_shake128(
    const Eurydice_arr_29 *verification_key, Eurydice_borrow_slice_u8 message,
    Eurydice_borrow_slice_u8 context, const Eurydice_arr_0c *signature) {
  Eurydice_arr_ec pre_hash_buffer = {{0U}};
  const Eurydice_arr_29 *uu____0 =
      libcrux_ml_dsa_types_as_ref_7f_a2(verification_key);
  Eurydice_borrow_slice_u8 uu____1 = message;
  Eurydice_borrow_slice_u8 uu____2 = context;
  Eurydice_mut_borrow_slice_u8 uu____3 =
      Eurydice_array_to_slice_mut_01(&pre_hash_buffer);
  return libcrux_ml_dsa_ml_dsa_generic_instantiations_avx2_ml_dsa_65_verify_pre_hashed_shake128(
      uu____0, uu____1, uu____2, uu____3,
      libcrux_ml_dsa_types_as_ref_c5_5c(signature));
}

#define libcrux_mldsa65_avx2_H_DEFINED
#endif /* libcrux_mldsa65_avx2_H */
