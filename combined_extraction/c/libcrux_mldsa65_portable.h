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
 * Libcrux: 3687467117fe5c6ddf8cdeb78306adc5d11ead2d
 */


#ifndef libcrux_mldsa65_portable_H
#define libcrux_mldsa65_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_mldsa_core.h"
#include "combined_core.h"

/**
 Generate an ML-DSA-65 Key Pair
*/
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_MLDSA65KeyPair
libcrux_ml_dsa_ml_dsa_65_portable_generate_key_pair(Eurydice_arr_ec randomness);

/**
 Generate an ML-DSA-65 Key Pair
*/
void
libcrux_ml_dsa_ml_dsa_65_portable_generate_key_pair_mut(
  Eurydice_arr_ec randomness,
  Eurydice_arr_24 *signing_key,
  Eurydice_arr_29 *verification_key
);

/**
 Generate an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_8c
libcrux_ml_dsa_ml_dsa_65_portable_sign(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Generate an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_65_portable_sign_mut(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
);

/**
 Generate a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_8c
libcrux_ml_dsa_ml_dsa_65_portable_sign_pre_hashed_shake128(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Verify an ML-DSA-65 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_65_portable_verify(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature
);

/**
 Verify a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_65_portable_verify_pre_hashed_shake128(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature
);

#if defined(__cplusplus)
}
#endif

#define libcrux_mldsa65_portable_H_DEFINED
#endif /* libcrux_mldsa65_portable_H */
