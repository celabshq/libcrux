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


#ifndef libcrux_mlkem512_avx2_H
#define libcrux_mlkem512_avx2_H

#include "eurydice_glue.h"



#include "libcrux_mlkem_avx2.h"

#include "libcrux_mlkem_core.h"
#include "libcrux_mlkem_avx2.h"
#include "combined_core.h"

/**
 Decapsulate ML-KEM 512

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem512PrivateKey`] and an [`MlKem512Ciphertext`].
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_mlkem512_avx2_decapsulate(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_37(private_key, ciphertext);
}

/**
 Encapsulate ML-KEM 512

 Generates an ([`MlKem512Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem512PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_ab
libcrux_ml_kem_mlkem512_avx2_encapsulate(
  const Eurydice_arr_03 *public_key,
  Eurydice_arr_ec randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_80(public_key, &randomness);
}

/**
 Generate ML-KEM 512 Key Pair
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_mlkem512_avx2_generate_key_pair(Eurydice_arr_c7 randomness)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b8(&randomness);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_mlkem512_avx2_validate_private_key(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_25(private_key,
      ciphertext);
}

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_mlkem512_avx2_validate_private_key_only(const Eurydice_arr_ab0 *private_key)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_d5(private_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline bool
libcrux_ml_kem_mlkem512_avx2_validate_public_key(const Eurydice_arr_03 *public_key)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_d5(public_key);
}

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7
libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512PublicKeyUnpacked;

/**
 Decapsulate ML-KEM 512 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type [`MlKem512KeyPairUnpacked`]
 and an [`MlKem512Ciphertext`].
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ec
libcrux_ml_kem_mlkem512_avx2_unpacked_decapsulate(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_37(private_key,
      ciphertext);
}

/**
 Encapsulate ML-KEM 512 (unpacked)

 Generates an ([`MlKem512Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type [`MlKem512PublicKeyUnpacked`],
 the SHA3-256 hash of this public key, and [`SHARED_SECRET_SIZE`] bytes of `randomness`.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline tuple_ab
libcrux_ml_kem_mlkem512_avx2_unpacked_encapsulate(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  Eurydice_arr_ec randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_80(public_key,
      &randomness);
}

/**
 Generate ML-KEM 512 Key Pair in "unpacked" form
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem512_avx2_unpacked_generate_key_pair_mut(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b8(randomness, key_pair);
}

/**
 Generate ML-KEM 512 Key Pair in "unpacked" form.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_mlkem512_avx2_unpacked_generate_key_pair(Eurydice_arr_c7 randomness)
{
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked
  key_pair = libcrux_ml_kem_ind_cca_unpacked_default_7b_16();
  libcrux_ml_kem_mlkem512_avx2_unpacked_generate_key_pair_mut(randomness, &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_mlkem512_avx2_unpacked_init_key_pair(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_7b_16();
}

/**
 Create a new, empty unpacked public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7
libcrux_ml_kem_mlkem512_avx2_unpacked_init_public_key(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_30_16();
}

/**
 Get an unpacked key from a private key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem512_avx2_unpacked_key_pair_from_private_mut(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_c3(private_key,
    key_pair);
}

/**
 Get the serialized private key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_ab0
libcrux_ml_kem_mlkem512_avx2_unpacked_key_pair_serialized_private_key(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_4e(key_pair);
}

/**
 Get the serialized private key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem512_avx2_unpacked_key_pair_serialized_private_key_mut(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  Eurydice_arr_ab0 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_4e(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline Eurydice_arr_03
libcrux_ml_kem_mlkem512_avx2_unpacked_key_pair_serialized_public_key(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_ce(key_pair);
}

/**
 Get the serialized public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem512_avx2_unpacked_key_pair_serialized_public_key_mut(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_ce(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem512_avx2_unpacked_serialized_public_key(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ce(public_key, serialized);
}

/**
 Get the unpacked public key.
*/
KRML_ATTRIBUTE_TARGET("avx2")
static inline void
libcrux_ml_kem_mlkem512_avx2_unpacked_unpacked_public_key(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_25(public_key,
    unpacked_public_key);
}


#define libcrux_mlkem512_avx2_H_DEFINED
#endif /* libcrux_mlkem512_avx2_H */
