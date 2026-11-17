/*
 * SPDX-FileCopyrightText: 2026 CE Labs
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: e656e17bff6ca5efac8ab6919b9b74cb9a8dd8ad
 * Eurydice: aaa9fa657fb6f09802edb890252040d94cd93982
 * Karamel: 8c19d41458ce5cbfea029ebc03334ba96d149039
 * F*: 7b347386330d0e5a331a220535b6f15288903234
 * Libcrux: dirty
 */


#ifndef libcrux_mlkem_portable_H
#define libcrux_mlkem_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "combined_core.h"

Eurydice_arr_c7 libcrux_ml_kem_hash_functions_portable_G(Eurydice_borrow_slice_u8 input);

Eurydice_arr_ec libcrux_ml_kem_hash_functions_portable_H(Eurydice_borrow_slice_u8 input);

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS (1353)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS (3329)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R (62209U)

typedef Eurydice_arr_d6 libcrux_ml_kem_vector_portable_vector_type_PortableVector;

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_vector_type_from_i16_array(Eurydice_borrow_slice_i16 array);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_from_i16_array_b8(Eurydice_borrow_slice_i16 array);

uint8_t_x11
libcrux_ml_kem_vector_portable_serialize_serialize_11_int(Eurydice_borrow_slice_i16 v);

Eurydice_arr_80 libcrux_ml_kem_vector_portable_serialize_serialize_11(Eurydice_arr_d6 v);

Eurydice_arr_80 libcrux_ml_kem_vector_portable_serialize_11(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_80 libcrux_ml_kem_vector_portable_serialize_11_b8(Eurydice_arr_d6 a);

int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_11_int(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_11(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_11(Eurydice_borrow_slice_u8 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_11_b8(Eurydice_borrow_slice_u8 a);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_vector_type_to_i16_array(Eurydice_arr_d6 x);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_to_i16_array_b8(Eurydice_arr_d6 x);

#define LIBCRUX_ML_KEM_VECTOR_REJ_SAMPLE_TABLE_REJECTION_SAMPLE_SHUFFLE_TABLE ((KRML_CLITERAL(Eurydice_arr_87){ .data = { { .data = { 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 255U, 255U, 255U, 255U } }, { .data = { 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 255U, 255U, 255U, 255U } }, { .data = { 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 255U, 255U } }, { .data = { 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 14U, 15U, 255U, 255U } }, { .data = { 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 12U, 13U, 14U, 15U, 255U, 255U } }, { .data = { 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U } }, { .data = { 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U } }, { .data = { 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U } }, { .data = { 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U, 255U, 255U } }, { .data = { 0U, 1U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U } }, { .data = { 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U, 255U, 255U } }, { .data = { 0U, 1U, 2U, 3U, 4U, 5U, 6U, 7U, 8U, 9U, 10U, 11U, 12U, 13U, 14U, 15U } } } }))

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_vector_type_zero(void);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_ZERO_b8(void);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_vector_type_from_bytes(Eurydice_borrow_slice_u8 array);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_from_bytes_b8(Eurydice_borrow_slice_u8 array);

void
libcrux_ml_kem_vector_portable_vector_type_to_bytes(
  Eurydice_arr_d6 x,
  Eurydice_mut_borrow_slice_u8 bytes
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
void
libcrux_ml_kem_vector_portable_to_bytes_b8(
  Eurydice_arr_d6 x,
  Eurydice_mut_borrow_slice_u8 bytes
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_add(Eurydice_arr_d6 lhs, const Eurydice_arr_d6 *rhs);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_add_b8(Eurydice_arr_d6 lhs, const Eurydice_arr_d6 *rhs);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_sub(Eurydice_arr_d6 lhs, const Eurydice_arr_d6 *rhs);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_sub_b8(Eurydice_arr_d6 lhs, const Eurydice_arr_d6 *rhs);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_multiply_by_constant(Eurydice_arr_d6 vec, int16_t c);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_multiply_by_constant_b8(Eurydice_arr_d6 vec, int16_t c);

/**
 Note: This function is not secret independent
 Only use with public values.
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_cond_subtract_3329(Eurydice_arr_d6 vec);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_cond_subtract_3329_b8(Eurydice_arr_d6 v);

#define LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_BARRETT_MULTIPLIER (20159)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT (26)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_R ((int32_t)((uint32_t)1 << (uint32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT))

/**
 Signed Barrett Reduction

 Given an input `value`, `barrett_reduce` outputs a representative `result`
 such that:

 - result ≡ value (mod FIELD_MODULUS)
 - the absolute value of `result` is bound as follows:

 `|result| ≤ FIELD_MODULUS / 2 · (|value|/BARRETT_R + 1)

 Note: The input bound is 28296 to prevent overflow in the multiplication of quotient by FIELD_MODULUS
*/
int16_t libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce_element(int16_t value);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce(Eurydice_arr_d6 vec);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_barrett_reduce_b8(Eurydice_arr_d6 vector);

#define LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT (16U)

/**
 Signed Montgomery Reduction

 Given an input `value`, `montgomery_reduce` outputs a representative `o`
 such that:

 - o ≡ value · MONTGOMERY_R^(-1) (mod FIELD_MODULUS)
 - the absolute value of `o` is bound as follows:

 `|result| ≤ ceil(|value| / MONTGOMERY_R) + 1665

 In particular, if `|value| ≤ FIELD_MODULUS-1 * FIELD_MODULUS-1`, then `|o| <= FIELD_MODULUS-1`.
 And, if `|value| ≤ pow2 16 * FIELD_MODULUS-1`, then `|o| <= FIELD_MODULUS + 1664
*/
int16_t libcrux_ml_kem_vector_portable_arithmetic_montgomery_reduce_element(int32_t value);

/**
 If `fe` is some field element 'x' of the Kyber field and `fer` is congruent to
 `y · MONTGOMERY_R`, this procedure outputs a value that is congruent to
 `x · y`, as follows:

    `fe · fer ≡ x · y · MONTGOMERY_R (mod FIELD_MODULUS)`

 `montgomery_reduce` takes the value `x · y · MONTGOMERY_R` and outputs a representative
 `x · y · MONTGOMERY_R * MONTGOMERY_R^{-1} ≡ x · y (mod FIELD_MODULUS)`.
*/
int16_t
libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_fe_by_fer(
  int16_t fe,
  int16_t fer
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_by_constant(
  Eurydice_arr_d6 vec,
  int16_t c
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
  Eurydice_arr_d6 vector,
  int16_t constant
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(
  Eurydice_arr_d6 vec,
  int16_t c
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_to_unsigned_representative(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_to_unsigned_representative_b8(Eurydice_arr_d6 a);

/**
 The `compress_*` functions implement the `Compress` function specified in the NIST FIPS
 203 standard (Page 18, Expression 4.5), which is defined as:

 ```plaintext
 Compress_d: ℤq -> ℤ_{2ᵈ}
 Compress_d(x) = ⌈(2ᵈ/q)·x⌋
 ```

 Since `⌈x⌋ = ⌊x + 1/2⌋` we have:

 ```plaintext
 Compress_d(x) = ⌊(2ᵈ/q)·x + 1/2⌋
               = ⌊(2^{d+1}·x + q) / 2q⌋
 ```

 For further information about the function implementations, consult the
 `implementation_notes.pdf` document in this directory.

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
uint8_t libcrux_ml_kem_vector_portable_compress_compress_message_coefficient(uint16_t fe);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_compress_compress_1(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_compress_1_b8(Eurydice_arr_d6 a);

uint32_t
libcrux_ml_kem_vector_portable_arithmetic_get_n_least_significant_bits(
  uint8_t n,
  uint32_t value
);

int16_t
libcrux_ml_kem_vector_portable_compress_compress_ciphertext_coefficient(
  uint8_t coefficient_bits,
  uint16_t fe
);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_compress_decompress_1(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_decompress_1_b8(Eurydice_arr_d6 a);

void
libcrux_ml_kem_vector_portable_ntt_ntt_step(
  Eurydice_arr_d6 *vec,
  int16_t zeta,
  size_t i,
  size_t j
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_layer_1_step(
  Eurydice_arr_d6 vec,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_layer_1_step_b8(
  Eurydice_arr_d6 a,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_layer_2_step(
  Eurydice_arr_d6 vec,
  int16_t zeta0,
  int16_t zeta1
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_layer_2_step_b8(
  Eurydice_arr_d6 a,
  int16_t zeta0,
  int16_t zeta1
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_layer_3_step(Eurydice_arr_d6 vec, int16_t zeta);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_layer_3_step_b8(Eurydice_arr_d6 a, int16_t zeta);

void
libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(
  Eurydice_arr_d6 *vec,
  int16_t zeta,
  size_t i,
  size_t j
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_1_step(
  Eurydice_arr_d6 vec,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_inv_ntt_layer_1_step_b8(
  Eurydice_arr_d6 a,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_2_step(
  Eurydice_arr_d6 vec,
  int16_t zeta0,
  int16_t zeta1
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_inv_ntt_layer_2_step_b8(
  Eurydice_arr_d6 a,
  int16_t zeta0,
  int16_t zeta1
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_3_step(Eurydice_arr_d6 vec, int16_t zeta);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_inv_ntt_layer_3_step_b8(Eurydice_arr_d6 a, int16_t zeta);

/**
 Compute the product of two Kyber binomials with respect to the
 modulus `X² - zeta`.

 This function almost implements <strong>Algorithm 11</strong> of the
 NIST FIPS 203 standard, which is reproduced below:

 ```plaintext
 Input:  a₀, a₁, b₀, b₁ ∈ ℤq.
 Input: γ ∈ ℤq.
 Output: c₀, c₁ ∈ ℤq.

 c₀ ← a₀·b₀ + a₁·b₁·γ
 c₁ ← a₀·b₁ + a₁·b₀
 return c₀, c₁
 ```
 We say "almost" because the coefficients output by this function are in
 the Montgomery domain (unlike in the specification).

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
void
libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
  const Eurydice_arr_d6 *a,
  const Eurydice_arr_d6 *b,
  int16_t zeta,
  size_t i,
  Eurydice_arr_d6 *out
);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_multiply(
  const Eurydice_arr_d6 *lhs,
  const Eurydice_arr_d6 *rhs,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_multiply_b8(
  const Eurydice_arr_d6 *lhs,
  const Eurydice_arr_d6 *rhs,
  int16_t zeta0,
  int16_t zeta1,
  int16_t zeta2,
  int16_t zeta3
);

Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_serialize_1(Eurydice_arr_d6 v);

Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_1(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_1_b8(Eurydice_arr_d6 a);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_1(Eurydice_borrow_slice_u8 v);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_1(Eurydice_borrow_slice_u8 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_1_b8(Eurydice_borrow_slice_u8 a);

uint8_t_x4
libcrux_ml_kem_vector_portable_serialize_serialize_4_int(Eurydice_borrow_slice_i16 v);

Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_serialize_4(Eurydice_arr_d6 v);

Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_4(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_4_b8(Eurydice_arr_d6 a);

int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_4(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_4(Eurydice_borrow_slice_u8 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_4_b8(Eurydice_borrow_slice_u8 a);

uint8_t_x5
libcrux_ml_kem_vector_portable_serialize_serialize_5_int(Eurydice_borrow_slice_i16 v);

Eurydice_arr_6d libcrux_ml_kem_vector_portable_serialize_serialize_5(Eurydice_arr_d6 v);

Eurydice_arr_6d libcrux_ml_kem_vector_portable_serialize_5(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_6d libcrux_ml_kem_vector_portable_serialize_5_b8(Eurydice_arr_d6 a);

int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_5_int(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_5(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_5(Eurydice_borrow_slice_u8 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_5_b8(Eurydice_borrow_slice_u8 a);

uint8_t_x5
libcrux_ml_kem_vector_portable_serialize_serialize_10_int(Eurydice_borrow_slice_i16 v);

Eurydice_arr_fc libcrux_ml_kem_vector_portable_serialize_serialize_10(Eurydice_arr_d6 v);

Eurydice_arr_fc libcrux_ml_kem_vector_portable_serialize_10(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_fc libcrux_ml_kem_vector_portable_serialize_10_b8(Eurydice_arr_d6 a);

int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_10(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_10(Eurydice_borrow_slice_u8 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_10_b8(Eurydice_borrow_slice_u8 a);

uint8_t_x3
libcrux_ml_kem_vector_portable_serialize_serialize_12_int(Eurydice_borrow_slice_i16 v);

Eurydice_arr_94 libcrux_ml_kem_vector_portable_serialize_serialize_12(Eurydice_arr_d6 v);

Eurydice_arr_94 libcrux_ml_kem_vector_portable_serialize_12(Eurydice_arr_d6 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_94 libcrux_ml_kem_vector_portable_serialize_12_b8(Eurydice_arr_d6 a);

int16_t_x2
libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_12(Eurydice_borrow_slice_u8 bytes);

Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_12(Eurydice_borrow_slice_u8 a);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_12_b8(Eurydice_borrow_slice_u8 a);

size_t
libcrux_ml_kem_vector_portable_sampling_rej_sample(
  Eurydice_borrow_slice_u8 a,
  Eurydice_mut_borrow_slice_i16 result
);

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
size_t
libcrux_ml_kem_vector_portable_rej_sample_b8(
  Eurydice_borrow_slice_u8 a,
  Eurydice_mut_borrow_slice_i16 out
);

typedef int16_t libcrux_ml_kem_vector_portable_arithmetic_FieldElementTimesMontgomeryR;

typedef int16_t libcrux_ml_kem_vector_portable_arithmetic_MontgomeryFieldElement;

typedef int16_t libcrux_ml_kem_vector_portable_vector_type_FieldElement;

/**
This function found in impl {core::clone::Clone for libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
Eurydice_arr_d6
libcrux_ml_kem_vector_portable_vector_type_clone_9c(const Eurydice_arr_d6 *self);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_9e_s { Eurydice_arr_d6 data[16U]; } Eurydice_arr_9e;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_1d
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_d20_s { Eurydice_arr_9e data[4U]; } Eurydice_arr_d20;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_d20
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_602_s { Eurydice_arr_d20 data[4U]; } Eurydice_arr_602;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_94_s
{
  Eurydice_arr_d20 t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_602 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_94;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_94 ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_94_s
{
  Eurydice_arr_d20 ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_94;

typedef struct libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_94 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 public_key;
}
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_1d
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_1e_s { Eurydice_arr_9e data[2U]; } Eurydice_arr_1e;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_1e
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_df0_s { Eurydice_arr_1e data[2U]; } Eurydice_arr_df0;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $2size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_3b_s
{
  Eurydice_arr_1e t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_df0 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_3b;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $2size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_3b ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $2size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_3b_s
{
  Eurydice_arr_1e ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_3b;

typedef struct libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_3b private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b public_key;
}
libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_1d
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_bb0_s { Eurydice_arr_9e data[3U]; } Eurydice_arr_bb0;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_bb0
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_c10_s { Eurydice_arr_bb0 data[3U]; } Eurydice_arr_c10;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51_s
{
  Eurydice_arr_bb0 t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_c10 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51_s
{
  Eurydice_arr_bb0 ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51;

typedef struct libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 public_key;
}
libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked;

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem_portable_H_DEFINED
#endif /* libcrux_mlkem_portable_H */
