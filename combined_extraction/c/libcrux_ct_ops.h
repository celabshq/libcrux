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


#ifndef libcrux_ct_ops_H
#define libcrux_ct_ops_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "combined_core.h"

/**
 Return 1 if `value` is not zero and 0 otherwise.
*/
uint8_t libcrux_ml_kem_constant_time_ops_inz(uint8_t value);

uint8_t libcrux_ml_kem_constant_time_ops_is_non_zero(uint8_t value);

/**
 Return 1 if the bytes of `lhs` and `rhs` do not exactly
 match and 0 otherwise.
*/
uint8_t
libcrux_ml_kem_constant_time_ops_compare(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs
);

uint8_t
libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs
);

/**
 If `selector` is not zero, return the bytes in `rhs`; return the bytes in
 `lhs` otherwise.
*/
Eurydice_arr_ec
libcrux_ml_kem_constant_time_ops_select_ct(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs,
  uint8_t selector
);

Eurydice_arr_ec
libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs,
  uint8_t selector
);

Eurydice_arr_ec
libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(
  Eurydice_borrow_slice_u8 lhs_c,
  Eurydice_borrow_slice_u8 rhs_c,
  Eurydice_borrow_slice_u8 lhs_s,
  Eurydice_borrow_slice_u8 rhs_s
);

#if defined(__cplusplus)
}
#endif

#define libcrux_ct_ops_H_DEFINED
#endif /* libcrux_ct_ops_H */
