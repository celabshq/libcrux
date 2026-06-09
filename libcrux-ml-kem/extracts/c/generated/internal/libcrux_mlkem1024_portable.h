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

#ifndef internal_libcrux_mlkem1024_portable_H
#define internal_libcrux_mlkem1024_portable_H

#include "eurydice_glue.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include "../libcrux_mlkem1024_portable.h"
#include "internal/libcrux_mlkem_portable.h"
#include "libcrux_core.h"

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94
    libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024PublicKeyUnpacked;

/**
 Decapsulate ML-KEM 1024 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type
 [`MlKem1024KeyPairUnpacked`] and an [`MlKem1024Ciphertext`].
*/
Eurydice_arr_ec libcrux_ml_kem_mlkem1024_portable_unpacked_decapsulate(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *private_key,
    const Eurydice_arr_d1 *ciphertext);

/**
 Encapsulate ML-KEM 1024 (unpacked)

 Generates an ([`MlKem1024Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type
 [`MlKem1024PublicKeyUnpacked`], the SHA3-256 hash of this public key, and
 [`SHARED_SECRET_SIZE`] bytes of `randomness`.
 TODO: The F* prefix opens required modules, it should go away when the
 following issue is resolved: <https://github.com/hacspec/hax/issues/770>
*/
tuple_25 libcrux_ml_kem_mlkem1024_portable_unpacked_encapsulate(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *public_key,
    Eurydice_arr_ec randomness);

/**
 Generate ML-KEM 1024 Key Pair in "unpacked" form
*/
void libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair_mut(
    Eurydice_arr_c7 randomness,
    libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair);

/**
 Generate ML-KEM 1024 Key Pair in "unpacked" form.
*/
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair(
    Eurydice_arr_c7 randomness);

/**
 Create a new, empty unpacked key.
*/
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_mlkem1024_portable_unpacked_init_key_pair(void);

/**
 Create a new, empty unpacked public key.
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94
libcrux_ml_kem_mlkem1024_portable_unpacked_init_public_key(void);

/**
 Get an unpacked key from a private key.
*/
void libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_from_private_mut(
    const Eurydice_arr_a8 *private_key,
    libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair);

/**
 Get the serialized private key.
*/
Eurydice_arr_a8
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_private_key(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair);

/**
 Get the serialized private key.
*/
void libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_private_key_mut(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair,
    Eurydice_arr_a8 *serialized);

/**
 Get the serialized public key.
*/
Eurydice_arr_d1
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_public_key(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair);

/**
 Get the serialized public key.
*/
void libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_public_key_mut(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair,
    Eurydice_arr_d1 *serialized);

/**
 Get the serialized public key.
*/
void libcrux_ml_kem_mlkem1024_portable_unpacked_serialized_public_key(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *public_key,
    Eurydice_arr_d1 *serialized);

/**
 Get the unpacked public key.
*/
void libcrux_ml_kem_mlkem1024_portable_unpacked_unpacked_public_key(
    const Eurydice_arr_d1 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94
        *unpacked_public_key);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem1024_portable_H_DEFINED
#endif /* internal_libcrux_mlkem1024_portable_H */
