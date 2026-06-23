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


#ifndef libcrux_mlkem1024_avx2_H
#define libcrux_mlkem1024_avx2_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "intrinsics/libcrux_intrinsics_avx2.h"

#include "libcrux_mlkem_core.h"
#include "combined_core.h"

/**
 Decapsulate ML-KEM 1024

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem1024PrivateKey`] and an [`MlKem1024Ciphertext`].
*/
Eurydice_arr_ec
libcrux_ml_kem_mlkem1024_avx2_decapsulate(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
);

/**
 Encapsulate ML-KEM 1024

 Generates an ([`MlKem1024Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem1024PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
tuple_25
libcrux_ml_kem_mlkem1024_avx2_encapsulate(
  const Eurydice_arr_d1 *public_key,
  Eurydice_arr_ec randomness
);

/**
 Generate ML-KEM 1024 Key Pair
*/
libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_mlkem1024_avx2_generate_key_pair(Eurydice_arr_c7 randomness);

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem1024_avx2_validate_private_key(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
);

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem1024_avx2_validate_private_key_only(const Eurydice_arr_a8 *private_key);

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem1024_avx2_validate_public_key(const Eurydice_arr_d1 *public_key);

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem1024_avx2_H_DEFINED
#endif /* libcrux_mlkem1024_avx2_H */
