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

#ifndef libcrux_mlkem768_portable_H
#define libcrux_mlkem768_portable_H

#include "eurydice_glue.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_core.h"
#include "libcrux_sha3_internal.h"

/**
 Decapsulate ML-KEM 768

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem768PrivateKey`] and an
 [`MlKem768Ciphertext`].
*/
Eurydice_arr_60 libcrux_ml_kem_mlkem768_portable_decapsulate(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext);

/**
 Encapsulate ML-KEM 768

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
tuple_7f libcrux_ml_kem_mlkem768_portable_encapsulate(
    const Eurydice_arr_74 *public_key, Eurydice_arr_60 randomness);

/**
 Generate ML-KEM 768 Key Pair
*/
libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_mlkem768_portable_generate_key_pair(Eurydice_arr_060 randomness);

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem768_portable_validate_private_key(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext);

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem768_portable_validate_private_key_only(
    const Eurydice_arr_ea *private_key);

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem768_portable_validate_public_key(
    const Eurydice_arr_74 *public_key);

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem768_portable_H_DEFINED
#endif /* libcrux_mlkem768_portable_H */
