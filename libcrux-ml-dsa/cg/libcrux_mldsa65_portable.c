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
 * Libcrux: dc3acb91f95df539456687a4c0965566e29e77f8
 */

#include "libcrux_mldsa65_portable.h"

#include "libcrux_mldsa_core.h"
#include "libcrux_sha3_portable.h"

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ct_test.ct_declassify
with types bool

*/
void libcrux_ml_dsa_ct_test_ct_declassify_5f(const bool *val) {}

/**
 Mark memory as secret.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ct_test.ct_classify
with types Eurydice_arr uint8_t[[$32size_t]]

*/
void libcrux_ml_dsa_ct_test_ct_classify_62(const Eurydice_arr_60 *val) {}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ct_test.ct_declassify
with types Eurydice_derefed_slice uint8_t

*/
void libcrux_ml_dsa_ct_test_ct_declassify_45(const uint8_t (*val)[]) {}

/**
 Mark memory as secret.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ct_test.ct_classify
with types Eurydice_derefed_slice uint8_t

*/
void libcrux_ml_dsa_ct_test_ct_classify_45(const uint8_t (*val)[]) {}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ct_test.ct_declassify
with types Eurydice_arr uint8_t[[$48size_t]]

*/
void libcrux_ml_dsa_ct_test_ct_declassify_7d(const Eurydice_arr_5f *val) {}
