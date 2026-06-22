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
 * Libcrux: c580de08c2461add5a35427c264aeeacde26bcf5
 */


#ifndef internal_libcrux_mlkem768_portable_H
#define internal_libcrux_mlkem768_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "internal/libcrux_sha3_portable.h"
#include "internal/combined_core.h"
#include "combined_core.h"
#include "../libcrux_mlkem768_portable.h"

int16_t libcrux_ml_kem_polynomial_zeta(size_t i);

#define LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS (1353)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS (3329)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R (62209U)

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.shake128_init_absorb_final_4a
with const generics
- K= 3
*/
Eurydice_arr_1b0
libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_78(
  const Eurydice_arr_810 *input
);

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.shake128_squeeze_first_three_blocks_4a
with const generics
- K= 3
*/
Eurydice_arr_7e
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_78(
  Eurydice_arr_1b0 *self
);

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.shake128_squeeze_next_block_4a
with const generics
- K= 3
*/
Eurydice_arr_2c
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_78(
  Eurydice_arr_1b0 *self
);

/**
 Serialize the secret key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key_mut
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
void
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_52(
  Eurydice_borrow_slice_u8 private_key,
  Eurydice_borrow_slice_u8 public_key,
  Eurydice_borrow_slice_u8 implicit_rejection_value,
  Eurydice_arr_7d *serialized
);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem768_portable_H_DEFINED
#endif /* internal_libcrux_mlkem768_portable_H */
