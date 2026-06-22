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


#ifndef libcrux_mldsa87_avx2_H
#define libcrux_mldsa87_avx2_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "intrinsics/libcrux_intrinsics_avx2.h"

#include "libcrux_mldsa_core.h"
#include "combined_core.h"

/**
 Generate an ML-DSA-87 Key Pair
*/
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_87_MLDSA87KeyPair
libcrux_ml_dsa_ml_dsa_87_avx2_generate_key_pair(Eurydice_arr_ec randomness);

/**
 Generate an ML-DSA-87 Signature

 The parameter `context` is used for domain separation
 and is a byte string of length at most 255 bytes. It
 may also be empty.
*/
core_result_Result_8b
libcrux_ml_dsa_ml_dsa_87_avx2_sign(
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
libcrux_ml_dsa_ml_dsa_87_avx2_sign_mut(
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
libcrux_ml_dsa_ml_dsa_87_avx2_sign_pre_hashed_shake128(
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
libcrux_ml_dsa_ml_dsa_87_avx2_verify(
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
libcrux_ml_dsa_ml_dsa_87_avx2_verify_pre_hashed_shake128(
  const Eurydice_arr_43 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_93 *signature
);

#if defined(__cplusplus)
}
#endif

#define libcrux_mldsa87_avx2_H_DEFINED
#endif /* libcrux_mldsa87_avx2_H */
