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


#include "internal/libcrux_mlkem512_portable.h"

#include "libcrux_mlkem_core.h"
#include "combined_core.h"
#include "internal/libcrux_mlkem_portable.h"
#include "internal/libcrux_mlkem_common.h"

/**
 Decapsulate ML-KEM 512

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem512PrivateKey`] and an [`MlKem512Ciphertext`].
*/
Eurydice_arr_ec
libcrux_ml_kem_mlkem512_portable_decapsulate(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_37(private_key, ciphertext);
}

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
)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_80(public_key, &randomness);
}

/**
 Generate ML-KEM 512 Key Pair
*/
libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_mlkem512_portable_generate_key_pair(Eurydice_arr_c7 randomness)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_b8(&randomness);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem512_portable_validate_private_key(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_25(private_key,
      ciphertext);
}

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
bool
libcrux_ml_kem_mlkem512_portable_validate_private_key_only(const Eurydice_arr_ab0 *private_key)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_d5(private_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
bool libcrux_ml_kem_mlkem512_portable_validate_public_key(const Eurydice_arr_03 *public_key)
{
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_d5(public_key);
}

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
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_37(private_key,
      ciphertext);
}

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
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_80(public_key,
      &randomness);
}

/**
 Generate ML-KEM 512 Key Pair in "unpacked" form
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_generate_key_pair_mut(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_b8(randomness,
    key_pair);
}

/**
 Generate ML-KEM 512 Key Pair in "unpacked" form.
*/
libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_mlkem512_portable_unpacked_generate_key_pair(Eurydice_arr_c7 randomness)
{
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked
  key_pair = libcrux_ml_kem_ind_cca_unpacked_default_87_66();
  libcrux_ml_kem_mlkem512_portable_unpacked_generate_key_pair_mut(randomness, &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_mlkem512_portable_unpacked_init_key_pair(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_87_66();
}

/**
 Create a new, empty unpacked public key.
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b
libcrux_ml_kem_mlkem512_portable_unpacked_init_public_key(void)
{
  return libcrux_ml_kem_ind_cca_unpacked_default_1d_66();
}

/**
 Get an unpacked key from a private key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_from_private_mut(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_c3(private_key,
    key_pair);
}

/**
 Get the serialized private key.
*/
Eurydice_arr_ab0
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_private_key(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_5b_a3(key_pair);
}

/**
 Get the serialized private key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_private_key_mut(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair,
  Eurydice_arr_ab0 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_5b_a3(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
Eurydice_arr_03
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_public_key(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_5b_53(key_pair);
}

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_key_pair_serialized_public_key_mut(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_5b_53(key_pair, serialized);
}

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_serialized_public_key(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *public_key,
  Eurydice_arr_03 *serialized
)
{
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_86_53(public_key, serialized);
}

/**
 Get the unpacked public key.
*/
void
libcrux_ml_kem_mlkem512_portable_unpacked_unpacked_public_key(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_25(public_key,
    unpacked_public_key);
}

