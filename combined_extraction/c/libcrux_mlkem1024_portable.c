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


#include "internal/libcrux_mlkem1024_portable.h"

#include "libcrux_mlkem_core.h"
#include "combined_core.h"
#include "internal/libcrux_mlkem_portable.h"
#include "internal/libcrux_mlkem_common.h"

/**
 Decapsulate ML-KEM 1024

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem1024PrivateKey`] and an [`MlKem1024Ciphertext`].
*/
Eurydice_arr_ec
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
tuple_25
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
libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_mlkem1024_portable_generate_key_pair(Eurydice_arr_c7 randomness)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_b3(&randomness);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
bool
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
bool
libcrux_ml_kem_mlkem1024_portable_validate_private_key_only(const Eurydice_arr_a8 *private_key)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_f5(private_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem1024_portable_validate_public_key(const Eurydice_arr_d1 *public_key)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_f5(public_key);
}

/**
 Decapsulate ML-KEM 1024 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type [`MlKem1024KeyPairUnpacked`]
 and an [`MlKem1024Ciphertext`].
*/
Eurydice_arr_ec
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
tuple_25
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
void
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
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair(Eurydice_arr_c7 randomness)
{
  libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
  key_pair = libcrux_ml_kem_ind_cca_unpacked_default_87_ee();
  libcrux_ml_kem_mlkem1024_portable_unpacked_generate_key_pair_mut(randomness, &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_mlkem1024_portable_unpacked_init_key_pair(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_87_ee();
}

/**
 Create a new, empty unpacked public key.
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94
libcrux_ml_kem_mlkem1024_portable_unpacked_init_public_key(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_1d_ee();
}

/**
 Get an unpacked key from a private key.
*/
void
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
Eurydice_arr_a8
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_private_key(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_5b_2e(key_pair);
}

/**
 Get the serialized private key.
*/
void
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_private_key_mut(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  Eurydice_arr_a8 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_5b_2e(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
Eurydice_arr_d1
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_public_key(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_5b_1c(key_pair);
}

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem1024_portable_unpacked_key_pair_serialized_public_key_mut(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_5b_1c(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem1024_portable_unpacked_serialized_public_key(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *public_key,
  Eurydice_arr_d1 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_86_1c(public_key, serialized);
}

/**
 Get the unpacked public key.
*/
void
libcrux_ml_kem_mlkem1024_portable_unpacked_unpacked_public_key(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_43(public_key,
    unpacked_public_key);
}

