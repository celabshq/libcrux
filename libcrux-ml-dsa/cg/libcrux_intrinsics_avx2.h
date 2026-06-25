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
