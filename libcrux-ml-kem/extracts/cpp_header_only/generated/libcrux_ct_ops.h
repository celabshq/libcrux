/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 377317d6b25702c46ffff072fa00a3e32095e46f
 * Eurydice: b227478b67c6a6e2ff611f978f10d6b7f26472ac
 * Karamel: 4e64d915da3c172d1dfad805b8e1a46beff938bc
 * F*: 89901492c020c74b82d811d27f3149c222d9b8b5
 * Libcrux: a53e03cfd7b424560bdfefc9d483f87faacd3122
 */

#ifndef libcrux_ct_ops_H
#define libcrux_ct_ops_H

#include "eurydice_glue.h"
#include "libcrux_mlkem_core.h"

/**
 Return 1 if `value` is not zero and 0 otherwise.
*/
static KRML_NOINLINE uint8_t
libcrux_ml_kem_constant_time_ops_inz(uint8_t value) {
  uint16_t value0 = (uint16_t)value;
  uint8_t result =
      (uint8_t)((uint32_t)core_num__u16__wrapping_add(~value0, 1U) >> 8U);
  return (uint32_t)result & 1U;
}

static KRML_NOINLINE uint8_t
libcrux_ml_kem_constant_time_ops_is_non_zero(uint8_t value) {
  return libcrux_ml_kem_constant_time_ops_inz(value);
}

/**
 Return 1 if the bytes of `lhs` and `rhs` do not exactly
 match and 0 otherwise.
*/
static KRML_NOINLINE uint8_t libcrux_ml_kem_constant_time_ops_compare(
    Eurydice_borrow_slice_u8 lhs, Eurydice_borrow_slice_u8 rhs) {
  uint8_t r = 0U;
  for (size_t i = (size_t)0U; i < lhs.meta; i++) {
    size_t i0 = i;
    uint8_t nr = (uint32_t)r | ((uint32_t)lhs.ptr[i0] ^ (uint32_t)rhs.ptr[i0]);
    r = nr;
  }
  return libcrux_ml_kem_constant_time_ops_is_non_zero(r);
}

static KRML_NOINLINE uint8_t
libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
    Eurydice_borrow_slice_u8 lhs, Eurydice_borrow_slice_u8 rhs) {
  return libcrux_ml_kem_constant_time_ops_compare(lhs, rhs);
}

/**
 If `selector` is not zero, return the bytes in `rhs`; return the bytes in
 `lhs` otherwise.
*/
static KRML_NOINLINE Eurydice_arr_600
libcrux_ml_kem_constant_time_ops_select_ct(Eurydice_borrow_slice_u8 lhs,
                                           Eurydice_borrow_slice_u8 rhs,
                                           uint8_t selector) {
  uint8_t mask = core_num__u8__wrapping_sub(
      libcrux_ml_kem_constant_time_ops_is_non_zero(selector), 1U);
  Eurydice_arr_600 out = {{0U}};
  for (size_t i = (size_t)0U; i < LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE;
       i++) {
    size_t i0 = i;
    uint8_t outi = ((uint32_t)lhs.ptr[i0] & (uint32_t)mask) |
                   ((uint32_t)rhs.ptr[i0] & (uint32_t)~mask);
    out.data[i0] = outi;
  }
  return out;
}

static KRML_NOINLINE Eurydice_arr_600
libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
    Eurydice_borrow_slice_u8 lhs, Eurydice_borrow_slice_u8 rhs,
    uint8_t selector) {
  return libcrux_ml_kem_constant_time_ops_select_ct(lhs, rhs, selector);
}

static KRML_NOINLINE Eurydice_arr_600
libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(
    Eurydice_borrow_slice_u8 lhs_c, Eurydice_borrow_slice_u8 rhs_c,
    Eurydice_borrow_slice_u8 lhs_s, Eurydice_borrow_slice_u8 rhs_s) {
  uint8_t selector =
      libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
          lhs_c, rhs_c);
  return libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
      lhs_s, rhs_s, selector);
}

#define libcrux_ct_ops_H_DEFINED
#endif /* libcrux_ct_ops_H */
