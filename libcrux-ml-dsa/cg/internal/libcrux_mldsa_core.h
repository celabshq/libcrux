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

#ifndef internal_libcrux_mldsa_core_H
#define internal_libcrux_mldsa_core_H

#include "../libcrux_mldsa_core.h"
#include "eurydice_glue.h"

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_88_s {
  Eurydice_arr_c5 data[1U];
} Eurydice_arr_88;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $72size_t
*/
typedef struct Eurydice_arr_ab_s {
  uint8_t data[72U];
} Eurydice_arr_ab;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 72
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e2(
    const Eurydice_arr_ab *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)72U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 72
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d43(
    Eurydice_arr_ab *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

#define internal_libcrux_mldsa_core_H_DEFINED
#endif /* internal_libcrux_mldsa_core_H */
