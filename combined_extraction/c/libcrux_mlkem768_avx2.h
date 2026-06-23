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


#ifndef libcrux_mlkem768_avx2_H
#define libcrux_mlkem768_avx2_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "intrinsics/libcrux_intrinsics_avx2.h"

#include "libcrux_mlkem_core.h"
#include "combined_core.h"

/**
 Decapsulate ML-KEM 768

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem768PrivateKey`] and an [`MlKem768Ciphertext`].
*/
Eurydice_arr_ec
libcrux_ml_kem_mlkem768_avx2_decapsulate(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
);

/**
 Encapsulate ML-KEM 768

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
tuple_f4
libcrux_ml_kem_mlkem768_avx2_encapsulate(
  const Eurydice_arr_5f *public_key,
  Eurydice_arr_ec randomness
);

/**
 Generate ML-KEM 768 Key Pair
*/
libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_mlkem768_avx2_generate_key_pair(Eurydice_arr_c7 randomness);

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem768_avx2_validate_private_key(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
);

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem768_avx2_validate_private_key_only(const Eurydice_arr_7d *private_key);

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem768_avx2_validate_public_key(const Eurydice_arr_5f *public_key);

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem768_avx2_H_DEFINED
#endif /* libcrux_mlkem768_avx2_H */
