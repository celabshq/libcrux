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
 * Libcrux: bdbc514c92784f52ed92097e2dfe82c2533df5d0
 */


#include "libcrux_ct_ops.h"

#include "libcrux_mlkem_core.h"
#include "combined_core.h"
#include "internal/combined_core.h"

/**
 Return 1 if `value` is not zero and 0 otherwise.
*/
KRML_NOINLINE uint8_t libcrux_ml_kem_constant_time_ops_inz(uint8_t value)
{
  uint16_t value0 = (uint16_t)(uint32_t)value;
  uint8_t result = (uint8_t)((uint32_t)core_num__u16__wrapping_add(~value0, 1U) >> 8U & 0xFFFFU);
  return (uint32_t)result & 1U;
}

KRML_NOINLINE uint8_t libcrux_ml_kem_constant_time_ops_is_non_zero(uint8_t value)
{
  return libcrux_ml_kem_constant_time_ops_inz(value);
}

/**
 Return 1 if the bytes of `lhs` and `rhs` do not exactly
 match and 0 otherwise.
*/
KRML_NOINLINE uint8_t
libcrux_ml_kem_constant_time_ops_compare(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs
)
{
  uint8_t r = 0U;
  for (size_t i = (size_t)0U; i < lhs.meta; i++)
  {
    size_t i0 = i;
    uint8_t nr = (uint32_t)r | ((uint32_t)lhs.ptr[i0] ^ (uint32_t)rhs.ptr[i0]);
    r = nr;
  }
  return libcrux_ml_kem_constant_time_ops_is_non_zero(r);
}

KRML_NOINLINE uint8_t
libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs
)
{
  return libcrux_ml_kem_constant_time_ops_compare(lhs, rhs);
}

/**
 If `selector` is not zero, return the bytes in `rhs`; return the bytes in
 `lhs` otherwise.
*/
KRML_NOINLINE Eurydice_arr_ec
libcrux_ml_kem_constant_time_ops_select_ct(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs,
  uint8_t selector
)
{
  uint8_t
  mask = core_num__u8__wrapping_sub(libcrux_ml_kem_constant_time_ops_is_non_zero(selector), 1U);
  Eurydice_arr_ec out = { .data = { 0U } };
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE; i++)
  {
    size_t i0 = i;
    uint8_t
    outi =
      ((uint32_t)lhs.ptr[i0] & (uint32_t)mask) | ((uint32_t)rhs.ptr[i0] & (~(uint32_t)mask & 0xFFU));
    out.data[i0] = outi;
  }
  return out;
}

KRML_NOINLINE Eurydice_arr_ec
libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
  Eurydice_borrow_slice_u8 lhs,
  Eurydice_borrow_slice_u8 rhs,
  uint8_t selector
)
{
  return libcrux_ml_kem_constant_time_ops_select_ct(lhs, rhs, selector);
}

KRML_NOINLINE Eurydice_arr_ec
libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(
  Eurydice_borrow_slice_u8 lhs_c,
  Eurydice_borrow_slice_u8 rhs_c,
  Eurydice_borrow_slice_u8 lhs_s,
  Eurydice_borrow_slice_u8 rhs_s
)
{
  uint8_t
  selector = libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(lhs_c, rhs_c);
  return
    libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(lhs_s,
      rhs_s,
      selector);
}

