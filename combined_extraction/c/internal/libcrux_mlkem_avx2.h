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


#ifndef internal_libcrux_mlkem_avx2_H
#define internal_libcrux_mlkem_avx2_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_mlkem_core.h"
#include "combined_core.h"
#include "../libcrux_mlkem_avx2.h"

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_d3(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
const
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
*libcrux_ml_kem_ind_cca_unpacked_public_key_11_e3(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self
);

/**
This function found in impl {core::clone::Clone for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.clone_d7
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cca_unpacked_clone_d7_e3(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *self
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_79(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *self,
  Eurydice_arr_5f *serialized
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_79(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self,
  Eurydice_arr_5f *serialized
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
Eurydice_arr_5f
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_79(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self
);

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_d4(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self,
  Eurydice_arr_7d *serialized
);

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
Eurydice_arr_7d
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_d4(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *self
);

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_71(
  const Eurydice_arr_7d *private_key,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
);

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef
libcrux_ml_kem_ind_cca_unpacked_default_30_e3(void);

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 3
*/
libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_e3(void);

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_e9(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out
);

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate
with const generics
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
tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_26(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *randomness
);

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate
with const generics
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
Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_19(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_3b(
  const Eurydice_arr_5f *public_key
);

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_3b(
  const Eurydice_arr_7d *private_key
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_d3(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_e9(
  const Eurydice_arr_c7 *randomness
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate
with const generics
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
tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_26(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate
with const generics
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
Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_19(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
);

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_43(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *unpacked_public_key
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_74(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *self,
  Eurydice_arr_d1 *serialized
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_74(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self,
  Eurydice_arr_d1 *serialized
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
Eurydice_arr_d1
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_74(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self
);

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_f8(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self,
  Eurydice_arr_a8 *serialized
);

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
Eurydice_arr_a8
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_f8(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *self
);

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_39(
  const Eurydice_arr_a8 *private_key,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair
);

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4
libcrux_ml_kem_ind_cca_unpacked_default_30_5b(void);

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 4
*/
libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_5b(void);

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b3(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *out
);

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate
with const generics
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
tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_07(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *randomness
);

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate
with const generics
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
Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_85(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_f5(
  const Eurydice_arr_d1 *public_key
);

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_f5(
  const Eurydice_arr_a8 *private_key
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_43(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b3(
  const Eurydice_arr_c7 *randomness
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate
with const generics
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
tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_07(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate
with const generics
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
Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_85(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
);

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_25(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_mut_dd
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_ce(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *self,
  Eurydice_arr_03 *serialized
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_ce(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self,
  Eurydice_arr_03 *serialized
);

/**
 Get the serialized public key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_public_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
Eurydice_arr_03
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_ce(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self
);

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_mut_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
*/
void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_4e(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self,
  Eurydice_arr_ab0 *serialized
);

/**
 Get the serialized private key.
*/
/**
This function found in impl {libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_private_key_11
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
*/
Eurydice_arr_ab0
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_4e(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *self
);

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.keypair_from_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_c3(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
);

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_30
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7
libcrux_ml_kem_ind_cca_unpacked_default_30_16(void);

/**
This function found in impl {core::default::Default for libcrux_ml_kem::ind_cca::unpacked::MlKemKeyPairUnpacked<Vector, K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.default_7b
with types libcrux_ml_kem_vector_avx2_SIMD256Vector
with const generics
- K= 2
*/
libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked
libcrux_ml_kem_ind_cca_unpacked_default_7b_16(void);

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b8(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *out
);

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *randomness
);

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_37(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_d5(
  const Eurydice_arr_03 *public_key
);

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_d5(
  const Eurydice_arr_ab0 *private_key
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_25(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b8(
  const Eurydice_arr_c7 *randomness
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- VECTOR_U_BLOCK_LEN= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_80(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
);

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- CIPHERTEXT_SIZE= 768
- T_AS_NTT_ENCODED_SIZE= 768
- C1_SIZE= 640
- C2_SIZE= 128
- VECTOR_U_COMPRESSION_FACTOR= 10
- VECTOR_V_COMPRESSION_FACTOR= 4
- C1_BLOCK_SIZE= 320
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
- IMPLICIT_REJECTION_HASH_INPUT_SIZE= 800
*/
Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_37(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem_avx2_H_DEFINED
#endif /* internal_libcrux_mlkem_avx2_H */
