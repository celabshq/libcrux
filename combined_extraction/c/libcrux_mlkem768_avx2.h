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
 * Libcrux: 3687467117fe5c6ddf8cdeb78306adc5d11ead2d
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
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_13_s { __m256i data[16U]; } Eurydice_arr_13;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_f6
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_600_s { Eurydice_arr_13 data[3U]; } Eurydice_arr_600;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_600
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_601_s { Eurydice_arr_600 data[3U]; } Eurydice_arr_601;

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef_s
{
  Eurydice_arr_600 t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_601 A;
}
libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef;

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

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef_s
{
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_ef ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef;

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768PublicKeyUnpacked;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef_s
{
  Eurydice_arr_600 ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
}
libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef;

typedef struct libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked_s
{
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_ef private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef public_key;
}
libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked;

/**
 Decapsulate ML-KEM 768 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type [`MlKem768KeyPairUnpacked`]
 and an [`MlKem768Ciphertext`].
*/
Eurydice_arr_ec
libcrux_ml_kem_mlkem768_avx2_unpacked_decapsulate(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *private_key,
  const Eurydice_arr_2b *ciphertext
);

/**
 Encapsulate ML-KEM 768 (unpacked)

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type [`MlKem768PublicKeyUnpacked`],
 the SHA3-256 hash of this public key, and [`SHARED_SECRET_SIZE`] bytes of `randomness`.
*/
tuple_f4
libcrux_ml_kem_mlkem768_avx2_unpacked_encapsulate(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  Eurydice_arr_ec randomness
);

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
);

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair(Eurydice_arr_c7 randomness);

/**
 Create a new, empty unpacked key.
*/
libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair(void);

/**
 Create a new, empty unpacked public key.
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
libcrux_ml_kem_mlkem768_avx2_unpacked_init_public_key(void);

/**
 Get an unpacked key from a private key.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_from_private_mut(
  const Eurydice_arr_7d *private_key,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
);

/**
 Get the serialized private key.
*/
Eurydice_arr_7d
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_private_key(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
);

/**
 Get the serialized private key.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_private_key_mut(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  Eurydice_arr_7d *serialized
);

/**
 Get the serialized public key.
*/
Eurydice_arr_5f
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_public_key(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
);

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_key_pair_serialized_public_key_mut(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  Eurydice_arr_5f *serialized
);

/**
 Get the unpacked public key.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_public_key(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *pk
);

/**
 Get the serialized public key.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_serialized_public_key(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  Eurydice_arr_5f *serialized
);

/**
 Get the unpacked public key.
*/
void
libcrux_ml_kem_mlkem768_avx2_unpacked_unpacked_public_key(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
);

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem768_avx2_H_DEFINED
#endif /* libcrux_mlkem768_avx2_H */
