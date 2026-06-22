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
 * Libcrux: c580de08c2461add5a35427c264aeeacde26bcf5
 */


#ifndef libcrux_mldsa44_portable_H
#define libcrux_mldsa44_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_mldsa_portable.h"
#include "libcrux_mldsa_core.h"
#include "combined_core.h"

/**
 Generate an ML-DSA-44 Key Pair
*/
static inline libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_MLDSA44KeyPair
libcrux_ml_dsa_ml_dsa_44_portable_generate_key_pair(Eurydice_arr_ec randomness)
{
  Eurydice_arr_10 signing_key = { .data = { 0U } };
  Eurydice_arr_02 verification_key = { .data = { 0U } };
  libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_44_generate_key_pair(randomness,
    &signing_key,
    &verification_key);
  return
    (
      KRML_CLITERAL(libcrux_ml_dsa_ml_dsa_generic_ml_dsa_44_MLDSA44KeyPair){
        .signing_key = libcrux_ml_dsa_types_new_9b_ab(signing_key),
        .verification_key = libcrux_ml_dsa_types_new_7f_7d(verification_key)
      }
    );
}

/**
 Generate an ML-DSA-44 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline core_result_Result_48
libcrux_ml_dsa_ml_dsa_44_portable_sign(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_44_sign(libcrux_ml_dsa_types_as_ref_9b_ab(signing_key),
      message,
      context,
      randomness);
}

/**
 Generate an ML-DSA-44 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline core_result_Result_53
libcrux_ml_dsa_ml_dsa_44_portable_sign_mut(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_85 *signature
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_44_sign_mut(libcrux_ml_dsa_types_as_ref_9b_ab(signing_key),
      message,
      context,
      randomness,
      signature);
}

/**
 Generate a HashML-DSA-44 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline core_result_Result_48
libcrux_ml_dsa_ml_dsa_44_portable_sign_pre_hashed_shake128(
  const Eurydice_arr_10 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
)
{
  Eurydice_arr_ec pre_hash_buffer = { .data = { 0U } };
  const Eurydice_arr_10 *uu____0 = libcrux_ml_dsa_types_as_ref_9b_ab(signing_key);
  return
    libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_44_sign_pre_hashed_shake128(uu____0,
      message,
      context,
      Eurydice_array_to_slice_mut_01(&pre_hash_buffer),
      randomness);
}

/**
 Verify an ML-DSA-44 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_44_portable_verify(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_85 *signature
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_44_verify(libcrux_ml_dsa_types_as_ref_7f_7d(verification_key),
      message,
      context,
      libcrux_ml_dsa_types_as_ref_c5_37(signature));
}

/**
 Verify a HashML-DSA-44 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_44_portable_verify_pre_hashed_shake128(
  const Eurydice_arr_02 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_85 *signature
)
{
  Eurydice_arr_ec pre_hash_buffer = { .data = { 0U } };
  const Eurydice_arr_02 *uu____0 = libcrux_ml_dsa_types_as_ref_7f_7d(verification_key);
  Eurydice_borrow_slice_u8 uu____1 = message;
  Eurydice_borrow_slice_u8 uu____2 = context;
  Eurydice_mut_borrow_slice_u8 uu____3 = Eurydice_array_to_slice_mut_01(&pre_hash_buffer);
  return
    libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_44_verify_pre_hashed_shake128(uu____0,
      uu____1,
      uu____2,
      uu____3,
      libcrux_ml_dsa_types_as_ref_c5_37(signature));
}

#if defined(__cplusplus)
}
#endif

#define libcrux_mldsa44_portable_H_DEFINED
#endif /* libcrux_mldsa44_portable_H */
