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
 * Libcrux: bdbc514c92784f52ed92097e2dfe82c2533df5d0
 */


#ifndef internal_libcrux_mlkem512_portable_H
#define internal_libcrux_mlkem512_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "internal/libcrux_mlkem_portable.h"
#include "libcrux_mlkem_core.h"
#include "combined_core.h"
#include "../libcrux_mlkem512_portable.h"

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b
libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512PublicKeyUnpacked;

/**
 Decapsulate ML-KEM 512 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type [`MlKem512KeyPairUnpacked`]
 and an [`MlKem512Ciphertext`].
*/
Eurydice_arr_ec
libcrux_ml_kem_mlkem512_portable_unpacked_decapsulate(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *private_key,
  const Eurydice_arr_d2 *ciphertext
);

/**
 Encapsulate ML-KEM 512 (unpacked)

 Generates an ([`MlKem512Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type [`MlKem512PublicKeyUnpacked`],
 the SHA3-256 hash of this public key, and [`SHARED_SECRET_SIZE`] bytes of `randomness`.
*/
tuple_ab
libcrux_ml_kem_mlkem512_portable_unpacked_encapsulate(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *public_key,
  Eurydice_arr_ec randomness
);

/**
 Generate ML-KEM 512 Key Pair in "unpacked" form
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_generate_key_pair_mut(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
);

/**
 Generate ML-KEM 512 Key Pair in "unpacked" form.
*/
libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_mlkem512_portable_unpacked_generate_key_pair(Eurydice_arr_c7 randomness);

/**
 Create a new, empty unpacked key.
*/
libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_mlkem512_portable_unpacked_init_key_pair(void);

/**
 Create a new, empty unpacked public key.
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b
libcrux_ml_kem_mlkem512_portable_unpacked_init_public_key(void);

/**
 Get an unpacked key from a private key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_from_private_mut(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
);

/**
 Get the serialized private key.
*/
Eurydice_arr_ab0
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_private_key(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
);

/**
 Get the serialized private key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_private_key_mut(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair,
  Eurydice_arr_ab0 *serialized
);

/**
 Get the serialized public key.
*/
Eurydice_arr_03
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_public_key(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
);

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_public_key_mut(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair,
  Eurydice_arr_03 *serialized
);

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_serialized_public_key(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *public_key,
  Eurydice_arr_03 *serialized
);

/**
 Get the unpacked public key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_unpacked_public_key(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *unpacked_public_key
);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem512_portable_H_DEFINED
#endif /* internal_libcrux_mlkem512_portable_H */
