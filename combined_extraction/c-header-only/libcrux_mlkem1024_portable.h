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


#ifndef libcrux_mlkem1024_portable_H
#define libcrux_mlkem1024_portable_H

#include "eurydice_glue.h"



#include "libcrux_mlkem_portable.h"
#include "libcrux_mlkem_core.h"
#include "libcrux_mlkem_common.h"
#include "combined_core.h"

/**
 Decapsulate ML-KEM 1024

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem1024PrivateKey`] and an [`MlKem1024Ciphertext`].
*/
static inline Eurydice_arr_ec
libcrux_ml_kem_mlkem1024_portable_decapsulate(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_85(private_key, ciphertext);
}

/**
 Encapsulate ML-KEM 1024

 Generates an ([`MlKem1024Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem1024PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
static inline tuple_25
libcrux_ml_kem_mlkem1024_portable_encapsulate(
  const Eurydice_arr_d1 *public_key,
  Eurydice_arr_ec randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_07(public_key, &randomness);
}

/**
 Generate ML-KEM 1024 Key Pair
*/
static inline libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_mlkem1024_portable_generate_key_pair(Eurydice_arr_c7 randomness)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_b3(&randomness);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool
libcrux_ml_kem_mlkem1024_portable_validate_private_key(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_43(private_key,
      ciphertext);
}

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool
libcrux_ml_kem_mlkem1024_portable_validate_private_key_only(const Eurydice_arr_a8 *private_key)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_f5(private_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool
libcrux_ml_kem_mlkem1024_portable_validate_public_key(const Eurydice_arr_d1 *public_key)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_f5(public_key);
}

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024PublicKeyUnpacked;

/**
 Decapsulate ML-KEM 1024 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type [`MlKem1024KeyPairUnpacked`]
 and an [`MlKem1024Ciphertext`].
*/
static inline Eurydice_arr_ec
libcrux_ml_kem_mlkem1024_portable_unpacked_decapsulate(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_85(private_key,
      ciphertext);
}

/**
 Encapsulate ML-KEM 1024 (unpacked)

 Generates an ([`MlKem1024Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type [`MlKem1024PublicKeyUnpacked`],
 the SHA3-256 hash of this public key, and [`SHARED_SECRET_SIZE`] bytes of `randomness`.
 TODO: The F* prefix opens required modules, it should go away when the following issue is resolved:
 <https://github.com/hacspec/hax/issues/770>
*/
static inline tuple_25
libcrux_ml_kem_mlkem1024_portable_unpacked_encapsulate(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *public_key,
  Eurydice_arr_ec randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_07(public_key,
      &randomness);
}

/**
 Generate ML-KEM 1024 Key Pair in "unpacked" form
*/
static inline void
libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair_mut(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_b3(randomness,
    key_pair);
}

/**
 Generate ML-KEM 1024 Key Pair in "unpacked" form.
*/
static inline libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair(Eurydice_arr_c7 randomness)
{
  libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
  key_pair = libcrux_ml_kem_ind_cca_unpacked_default_7b_ee();
  libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair_mut(randomness, &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
static inline libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_mlkem1024_portable_unpacked_init_key_pair(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_7b_ee();
}

/**
 Create a new, empty unpacked public key.
*/
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94
libcrux_ml_kem_mlkem1024_portable_unpacked_init_public_key(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_30_ee();
}

/**
 Get an unpacked key from a private key.
*/
static inline void
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_from_private_mut(
  const Eurydice_arr_a8 *private_key,
  libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_39(private_key,
    key_pair);
}

/**
 Get the serialized private key.
*/
static inline Eurydice_arr_a8
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_private_key(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_2e(key_pair);
}

/**
 Get the serialized private key.
*/
static inline void
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_private_key_mut(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  Eurydice_arr_a8 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_2e(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
static inline Eurydice_arr_d1
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_public_key(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_1c(key_pair);
}

/**
 Get the serialized public key.
*/
static inline void
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_public_key_mut(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_1c(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
static inline void
libcrux_ml_kem_mlkem1024_portable_unpacked_serialized_public_key(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *public_key,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_1c(public_key, serialized);
}

/**
 Get the unpacked public key.
*/
static inline void
libcrux_ml_kem_mlkem1024_portable_unpacked_unpacked_public_key(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_43(public_key,
    unpacked_public_key);
}


#define libcrux_mlkem1024_portable_H_DEFINED
#endif /* libcrux_mlkem1024_portable_H */
