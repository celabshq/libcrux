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


#ifndef libcrux_mlkem512_portable_H
#define libcrux_mlkem512_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_mlkem_portable.h"
#include "libcrux_mlkem_core.h"
#include "combined_core.h"

/**
 Decapsulate ML-KEM 512

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem512PrivateKey`] and an [`MlKem512Ciphertext`].
*/
Eurydice_arr_ec
libcrux_ml_kem_mlkem512_portable_decapsulate(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
);

/**
 Encapsulate ML-KEM 512

 Generates an ([`MlKem512Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem512PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
tuple_ab
libcrux_ml_kem_mlkem512_portable_encapsulate(
  const Eurydice_arr_03 *public_key,
  Eurydice_arr_ec randomness
);

/**
 Generate ML-KEM 512 Key Pair
*/
libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_mlkem512_portable_generate_key_pair(Eurydice_arr_c7 randomness);

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem512_portable_validate_private_key(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
);

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem512_portable_validate_private_key_only(const Eurydice_arr_ab0 *private_key);

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem512_portable_validate_public_key(const Eurydice_arr_03 *public_key);

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

#define libcrux_mlkem512_portable_H_DEFINED
#endif /* libcrux_mlkem512_portable_H */
