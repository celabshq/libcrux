/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 377317d6b25702c46ffff072fa00a3e32095e46f
 * Eurydice: b227478b67c6a6e2ff611f978f10d6b7f26472ac
 * Karamel: 4e64d915da3c172d1dfad805b8e1a46beff938bc
 * F*: 32dcc97722b61224e0e9c5b447ca72c4cd9ffcf9
 * Libcrux: d8990345d65f638c9b7e98a68b61e93e2bf9180e
 */

#ifndef libcrux_intrinsics_avx2_H
#define libcrux_intrinsics_avx2_H

#include "eurydice_glue.h"
#include "libcrux_mldsa_core.h"

/**
A monomorphic instance of Eurydice.arr
with types core_core_arch_x86___m256i
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_c0_s {
  __m256i data[5U];
} Eurydice_arr_c0;

/**
A monomorphic instance of Eurydice.arr
with types core_core_arch_x86___m256i
with const generics
- $25size_t
*/
typedef struct Eurydice_arr_05_s {
  __m256i data[25U];
} Eurydice_arr_05;

#define libcrux_intrinsics_avx2_H_DEFINED
#endif /* libcrux_intrinsics_avx2_H */
