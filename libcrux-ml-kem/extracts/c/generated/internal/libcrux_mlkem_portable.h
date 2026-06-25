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

#ifndef internal_libcrux_mlkem_portable_H
#define internal_libcrux_mlkem_portable_H

#include "eurydice_glue.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include "../libcrux_mlkem_portable.h"
#include "libcrux_core.h"
#include "libcrux_sha3_internal.h"

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_b9_s {
  Eurydice_arr_e2 data[16U];
} Eurydice_arr_b9;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_1d
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_cf_s {
  Eurydice_arr_b9 data[4U];
} Eurydice_arr_cf;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_cf
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_5c_s {
  Eurydice_arr_cf data[4U];
} Eurydice_arr_5c;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_af_s {
  Eurydice_arr_cf t_as_ntt;
  Eurydice_arr_60 seed_for_A;
  Eurydice_arr_5c A;
} libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_af;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af_s {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_af ind_cpa_public_key;
  Eurydice_arr_60 public_key_hash;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af;

/**
 Generate an unpacked key from a serialized key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.unpack_public_key
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]],
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
void libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_30(
    const Eurydice_arr_00 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af
        *unpacked_public_key);

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $4size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_af_s {
  Eurydice_arr_cf ind_cpa_private_key;
  Eurydice_arr_60 implicit_rejection_value;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_af;

typedef struct
    libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked_s {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_af private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af public_key;
} libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked;

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
void libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ff(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af *self,
    Eurydice_arr_00 *serialized);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
void libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_ff(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *self,
    Eurydice_arr_00 *serialized);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
Eurydice_arr_00 libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_ff(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *self);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
void libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_2f(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *self,
    Eurydice_arr_17 *serialized);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
Eurydice_arr_17 libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_2f(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *self);

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.keys_from_private_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
*/
void libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_7d(
    const Eurydice_arr_17 *private_key,
    libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair);

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 4
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af
libcrux_ml_kem_ind_cca_unpacked_default_30_d0(void);

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 4
*/
libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_d0(void);

/**
 Generate Unpacked Keys
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.generate_keypair
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
void libcrux_ml_kem_ind_cca_unpacked_generate_keypair_150(
    Eurydice_arr_060 randomness,
    libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *out);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]] with const
generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- VECTOR_U_BLOCK_LEN= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
tuple_4d libcrux_ml_kem_ind_cca_unpacked_encapsulate_0c0(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_af *public_key,
    const Eurydice_arr_60 *randomness);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.decapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]] with const
generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
Eurydice_arr_60 libcrux_ml_kem_ind_cca_unpacked_decapsulate_510(
    const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_00 *ciphertext);

/**
 Validate an ML-KEM public key.

 This implements the Modulus check in 7.2 2.
 Note that the size check in 7.2 1 is covered by the `PUBLIC_KEY_SIZE` in the
 `public_key` type.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_public_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
bool libcrux_ml_kem_ind_cca_validate_public_key_ff(
    const Eurydice_arr_00 *public_key);

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key_only
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]]
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
*/
bool libcrux_ml_kem_ind_cca_validate_private_key_only_60(
    const Eurydice_arr_17 *private_key);

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
 Note that the size checks in 7.2 1 and 2 are covered by the `SECRET_KEY_SIZE`
 and `CIPHERTEXT_SIZE` in the `private_key` and `ciphertext` types.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]]
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
bool libcrux_ml_kem_ind_cca_validate_private_key_b5(
    const Eurydice_arr_17 *private_key, const Eurydice_arr_00 *_ciphertext);

/**
 Packed API

 Generate a key pair.

 Depending on the `Vector` and `Hasher` used, this requires different hardware
 features
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.generate_keypair
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_generate_keypair_150(const Eurydice_arr_060 *randomness);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.encapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 4
- CIPHERTEXT_SIZE= 1568
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
tuple_4d libcrux_ml_kem_ind_cca_encapsulate_ca0(
    const Eurydice_arr_00 *public_key, const Eurydice_arr_60 *randomness);

/**
 This code verifies on some machines, runs out of memory on others
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.decapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$4size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- CIPHERTEXT_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
- C1_SIZE= 1408
- C2_SIZE= 160
- VECTOR_U_COMPRESSION_FACTOR= 11
- VECTOR_V_COMPRESSION_FACTOR= 5
- C1_BLOCK_SIZE= 352
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1600
*/
Eurydice_arr_60 libcrux_ml_kem_ind_cca_decapsulate_620(
    const Eurydice_arr_17 *private_key, const Eurydice_arr_00 *ciphertext);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_1d
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_1d_s {
  Eurydice_arr_b9 data[3U];
} Eurydice_arr_1d;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_1d
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_dd_s {
  Eurydice_arr_1d data[3U];
} Eurydice_arr_dd;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0_s {
  Eurydice_arr_1d t_as_ntt;
  Eurydice_arr_60 seed_for_A;
  Eurydice_arr_dd A;
} libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0;

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0_s {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 ind_cpa_public_key;
  Eurydice_arr_60 public_key_hash;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0;

/**
 Generate an unpacked key from a serialized key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.unpack_public_key
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
void libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_0a(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
        *unpacked_public_key);

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0_s {
  Eurydice_arr_1d ind_cpa_private_key;
  Eurydice_arr_60 implicit_rejection_value;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0;

typedef struct
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked_s {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 public_key;
} libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked;

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.public_key_11
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *
libcrux_ml_kem_ind_cca_unpacked_public_key_11_1b(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self);

/**
This function found in impl {core::clone::Clone for
libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.clone_d7
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
libcrux_ml_kem_ind_cca_unpacked_clone_d7_1b(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *self);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
void libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_89(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *self,
    Eurydice_arr_74 *serialized);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
void libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_89(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self,
    Eurydice_arr_74 *serialized);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
Eurydice_arr_74 libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_89(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
void libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_43(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self,
    Eurydice_arr_ea *serialized);

/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
Eurydice_arr_ea libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_43(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self);

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.keys_from_private_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
*/
void libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_42(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair);

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
libcrux_ml_kem_ind_cca_unpacked_default_30_1b(void);

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_1b(void);

/**
 Generate Unpacked Keys
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.generate_keypair
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
void libcrux_ml_kem_ind_cca_unpacked_generate_keypair_15(
    Eurydice_arr_060 randomness,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *out);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
tuple_7f libcrux_ml_kem_ind_cca_unpacked_encapsulate_0c(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *public_key,
    const Eurydice_arr_60 *randomness);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.decapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
Eurydice_arr_60 libcrux_ml_kem_ind_cca_unpacked_decapsulate_51(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2c *ciphertext);

/**
 Validate an ML-KEM public key.

 This implements the Modulus check in 7.2 2.
 Note that the size check in 7.2 1 is covered by the `PUBLIC_KEY_SIZE` in the
 `public_key` type.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_public_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
bool libcrux_ml_kem_ind_cca_validate_public_key_89(
    const Eurydice_arr_74 *public_key);

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key_only
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
bool libcrux_ml_kem_ind_cca_validate_private_key_only_d6(
    const Eurydice_arr_ea *private_key);

/**
 Validate an ML-KEM private key.

 This implements the Hash check in 7.3 3.
 Note that the size checks in 7.2 1 and 2 are covered by the `SECRET_KEY_SIZE`
 and `CIPHERTEXT_SIZE` in the `private_key` and `ciphertext` types.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.validate_private_key
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
bool libcrux_ml_kem_ind_cca_validate_private_key_37(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *_ciphertext);

/**
 Packed API

 Generate a key pair.

 Depending on the `Vector` and `Hasher` used, this requires different hardware
 features
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.generate_keypair
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_generate_keypair_15(const Eurydice_arr_060 *randomness);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.encapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
tuple_7f libcrux_ml_kem_ind_cca_encapsulate_ca(
    const Eurydice_arr_74 *public_key, const Eurydice_arr_60 *randomness);

/**
 This code verifies on some machines, runs out of memory on others
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.decapsulate
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_SIZE= 960
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 1120
*/
Eurydice_arr_60 libcrux_ml_kem_ind_cca_decapsulate_62(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem_portable_H_DEFINED
#endif /* internal_libcrux_mlkem_portable_H */
