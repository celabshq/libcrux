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


#ifndef libcrux_mlkem_core_H
#define libcrux_mlkem_core_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "combined_core.h"

#define LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE ((size_t)32U)

#define LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT ((size_t)12U)

#define LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT ((size_t)256U)

#define LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)12U)

#define LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT (LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE ((size_t)32U)

#define LIBCRUX_ML_KEM_CONSTANTS_G_DIGEST_SIZE ((size_t)64U)

#define LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE ((size_t)32U)

/**
 K * BITS_PER_RING_ELEMENT / 8

 [eurydice] Note that we can't use const generics here because that breaks
            C extraction with eurydice.
*/
size_t libcrux_ml_kem_constants_ranked_bytes_per_ring_element(size_t rank);

typedef struct libcrux_ml_kem_mlkem1024_MlKem1024KeyPair_s
{
  Eurydice_arr_a8 sk;
  Eurydice_arr_d1 pk;
}
libcrux_ml_kem_mlkem1024_MlKem1024KeyPair;

/**
A monomorphic instance of n-tuple
with types libcrux_ml_kem_mlkem1024_MlKem1024Ciphertext, Eurydice_arr_ec

*/
typedef struct tuple_25_s
{
  Eurydice_arr_d1 fst;
  Eurydice_arr_ec snd;
}
tuple_25;

typedef struct libcrux_ml_kem_mlkem768_MlKem768KeyPair_s
{
  Eurydice_arr_7d sk;
  Eurydice_arr_5f pk;
}
libcrux_ml_kem_mlkem768_MlKem768KeyPair;

/**
A monomorphic instance of n-tuple
with types libcrux_ml_kem_mlkem768_MlKem768Ciphertext, Eurydice_arr_ec

*/
typedef struct tuple_f4_s
{
  Eurydice_arr_2b fst;
  Eurydice_arr_ec snd;
}
tuple_f4;

/**
A monomorphic instance of libcrux_ml_kem.types.MlKemKeyPair
with const generics
- $1632size_t
- $800size_t
*/
typedef struct libcrux_ml_kem_types_MlKemKeyPair_0d_s
{
  Eurydice_arr_ab0 sk;
  Eurydice_arr_03 pk;
}
libcrux_ml_kem_types_MlKemKeyPair_0d;

/**
A monomorphic instance of n-tuple
with types libcrux_ml_kem_types_MlKemCiphertext_6e, Eurydice_arr_ec

*/
typedef struct tuple_ab_s
{
  Eurydice_arr_d2 fst;
  Eurydice_arr_ec snd;
}
tuple_ab;

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem_core_H_DEFINED
#endif /* libcrux_mlkem_core_H */
