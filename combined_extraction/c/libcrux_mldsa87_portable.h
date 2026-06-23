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


#ifndef libcrux_mldsa87_portable_H
#define libcrux_mldsa87_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_mldsa_core.h"
#include "combined_core.h"

/**
 Generate an ML-DSA-87 Key Pair
*/
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_MLDSA87KeyPair
libcrux_ml_dsa_ml_dsa_87_portable_generate_key_pair(Eurydice_arr_ec randomness);

/**
 Generate an ML-DSA-87 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_87_portable_sign(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Generate an ML-DSA-87 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_53
libcrux_ml_dsa_ml_dsa_87_portable_sign_mut(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_93 *signature
);

/**
 Generate a HashML-DSA-87 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_87_portable_sign_pre_hashed_shake128(
  const Eurydice_arr_e2 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
);

/**
 Verify an ML-DSA-87 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_87_portable_verify(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_93 *signature
);

/**
 Verify a HashML-DSA-87 Signature, with a SHAKE128 pre-hashing

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_41
libcrux_ml_dsa_ml_dsa_87_portable_verify_pre_hashed_shake128(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_93 *signature
);

#if defined(__cplusplus)
}
#endif

#define libcrux_mldsa87_portable_H_DEFINED
#endif /* libcrux_mldsa87_portable_H */
