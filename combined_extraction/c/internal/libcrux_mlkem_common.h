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


#ifndef internal_libcrux_mlkem_common_H
#define internal_libcrux_mlkem_common_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "internal/libcrux_mlkem_portable.h"
#include "internal/libcrux_mlkem_avx2.h"
#include "libcrux_mlkem_core.h"
#include "combined_core.h"

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.unpack_public_key
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_43(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_29(public_key, unpacked_public_key);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.keypair_from_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
- T_AS_NTT_ENCODED_SIZE= 1536
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_39(
  const Eurydice_arr_a8 *private_key,
  libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_38(private_key, key_pair);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.generate_keypair
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_b3(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_b81(randomness, out);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.encapsulate
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
static KRML_MUSTINLINE tuple_25
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_07(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_94 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a71(public_key, randomness);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.decapsulate
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_85(
  const libcrux_ml_kem_mlkem1024_portable_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_0c1(key_pair, ciphertext);
}

/**
 Public key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_public_key
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_f5(
  const Eurydice_arr_d1 *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_1c(public_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key_only
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_f5(
  const Eurydice_arr_a8 *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_4c(private_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_43(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_79(private_key, ciphertext);
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.generate_keypair
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_b3(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_b81(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.encapsulate
with const generics
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
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_07(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_991(public_key, randomness);
}

/**
 Portable decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.decapsulate
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_85(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_fd1(private_key, ciphertext);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_d3(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a1(public_key, unpacked_public_key);
}

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
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_d3(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_d3(public_key,
    unpacked_public_key);
}

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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_71(
  const Eurydice_arr_7d *private_key,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_3e(private_key, key_pair);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_e9(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db1(randomness, out);
}

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
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_e9(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_e9(randomness, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_26(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a81(public_key, randomness);
}

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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_26(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_ef *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_26(public_key,
      randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_19(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_d91(key_pair, ciphertext);
}

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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_19(
  const libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_19(key_pair,
      ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_3b(
  const Eurydice_arr_5f *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_79(public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_3b(
  const Eurydice_arr_5f *public_key
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_3b(public_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_3b(
  const Eurydice_arr_7d *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_a4(private_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_d3(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_d50(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_d3(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_d3(private_key,
      ciphertext);
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_e9(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_db1(randomness);
}

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
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_e9(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_e9(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_26(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_a11(public_key, randomness);
}

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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_26(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_26(public_key, randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_19(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_661(private_key, ciphertext);
}

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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_19(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_19(private_key, ciphertext);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2
with const generics
- K= 4
- T_AS_NTT_ENCODED_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_43(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a0(public_key, unpacked_public_key);
}

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
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_43(
  const Eurydice_arr_d1 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_43(public_key,
    unpacked_public_key);
}

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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_39(
  const Eurydice_arr_a8 *private_key,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_b2(private_key, key_pair);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b3(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db0(randomness, out);
}

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
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b3(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b3(randomness, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2
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
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_07(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a80(public_key, randomness);
}

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
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_07(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_d4 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_07(public_key,
      randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_85(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_d90(key_pair, ciphertext);
}

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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_85(
  const libcrux_ml_kem_mlkem1024_avx2_unpacked_MlKem1024KeyPairUnpacked *key_pair,
  const Eurydice_arr_d1 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_85(key_pair,
      ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_f5(
  const Eurydice_arr_d1 *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_74(public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 4
- PUBLIC_KEY_SIZE= 1568
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_f5(
  const Eurydice_arr_d1 *public_key
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_f5(public_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_f5(
  const Eurydice_arr_a8 *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_f8(private_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_43(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_b3(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 4
- SECRET_KEY_SIZE= 3168
- CIPHERTEXT_SIZE= 1568
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_43(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_43(private_key,
      ciphertext);
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2
with const generics
- K= 4
- CPA_PRIVATE_KEY_SIZE= 1536
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b3(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_db0(randomness);
}

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
static inline libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b3(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b3(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2
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
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_07(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_a10(public_key, randomness);
}

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
static inline tuple_25
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_07(
  const Eurydice_arr_d1 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_07(public_key, randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_85(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_660(private_key, ciphertext);
}

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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_85(
  const Eurydice_arr_a8 *private_key,
  const Eurydice_arr_d1 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_85(private_key, ciphertext);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.unpack_public_key
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_25(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_e0(public_key, unpacked_public_key);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.keypair_from_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
- T_AS_NTT_ENCODED_SIZE= 768
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_c3(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_71(private_key, key_pair);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.generate_keypair
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_b8(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_b80(randomness, out);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.encapsulate
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
static KRML_MUSTINLINE tuple_ab
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_3b *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a70(public_key, randomness);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.decapsulate
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_37(
  const libcrux_ml_kem_mlkem512_portable_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_0c0(key_pair, ciphertext);
}

/**
 Public key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_public_key
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_d5(
  const Eurydice_arr_03 *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_53(public_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key_only
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_d5(
  const Eurydice_arr_ab0 *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_e2(private_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_25(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_d5(private_key, ciphertext);
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.generate_keypair
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
static inline libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_b8(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_b80(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.encapsulate
with const generics
- K= 2
- CIPHERTEXT_SIZE= 768
- PUBLIC_KEY_SIZE= 800
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
*/
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_80(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_990(public_key, randomness);
}

/**
 Portable decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.decapsulate
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_37(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_fd0(private_key, ciphertext);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.unpack_public_key
with const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_d3(
  const Eurydice_arr_5f *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_22(public_key, unpacked_public_key);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.keypair_from_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- T_AS_NTT_ENCODED_SIZE= 1152
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_71(
  const Eurydice_arr_7d *private_key,
  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_01(private_key, key_pair);
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.generate_keypair
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_e9(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_b8(randomness, out);
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.encapsulate
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
static KRML_MUSTINLINE tuple_f4
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_26(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a7(public_key, randomness);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.decapsulate
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_19(
  const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *key_pair,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_0c(key_pair, ciphertext);
}

/**
 Public key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_public_key
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_3b(
  const Eurydice_arr_5f *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_b6(public_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key_only
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_3b(
  const Eurydice_arr_7d *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_52(private_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key
with const generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_d3(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_ba(private_key, ciphertext);
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.generate_keypair
with const generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_e9(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_b8(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.encapsulate
with const generics
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
static inline tuple_f4
libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_26(
  const Eurydice_arr_5f *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_99(public_key, randomness);
}

/**
 Portable decapsulate
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.portable.decapsulate
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_19(
  const Eurydice_arr_7d *private_key,
  const Eurydice_arr_2b *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_fd(private_key, ciphertext);
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.unpack_public_key_avx2
with const generics
- K= 2
- T_AS_NTT_ENCODED_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_25(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_2a(public_key, unpacked_public_key);
}

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
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_25(
  const Eurydice_arr_03 *public_key,
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *unpacked_public_key
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_unpack_public_key_avx2_25(public_key,
    unpacked_public_key);
}

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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_keypair_from_private_key_c3(
  const Eurydice_arr_ab0 *private_key,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair
)
{
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_a4(private_key, key_pair);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.generate_keypair_avx2
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b8(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_db(randomness, out);
}

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
static inline void
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_b8(
  Eurydice_arr_c7 randomness,
  libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *out
)
{
  libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_generate_keypair_avx2_b8(randomness, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.encapsulate_avx2
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
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a8(public_key, randomness);
}

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
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_80(
  const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_c7 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_encapsulate_avx2_80(public_key,
      randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.unpacked.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_37(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_d9(key_pair, ciphertext);
}

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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_37(
  const libcrux_ml_kem_mlkem512_avx2_unpacked_MlKem512KeyPairUnpacked *key_pair,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_unpacked_decapsulate_avx2_37(key_pair,
      ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key_avx2
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_d5(
  const Eurydice_arr_03 *public_key
)
{
  return libcrux_ml_kem_ind_cca_validate_public_key_ce(public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_public_key
with const generics
- K= 2
- PUBLIC_KEY_SIZE= 800
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_d5(
  const Eurydice_arr_03 *public_key
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_validate_public_key_avx2_d5(public_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_only
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_only_d5(
  const Eurydice_arr_ab0 *private_key
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_only_37(private_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key_avx2
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_25(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_validate_private_key_85(private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.validate_private_key
with const generics
- K= 2
- SECRET_KEY_SIZE= 1632
- CIPHERTEXT_SIZE= 768
*/
static inline bool
libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_25(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return
    libcrux_ml_kem_ind_cca_instantiations_avx2_validate_private_key_avx2_25(private_key,
      ciphertext);
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.generate_keypair_avx2
with const generics
- K= 2
- CPA_PRIVATE_KEY_SIZE= 768
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
- ETA1= 3
- ETA1_RANDOMNESS_SIZE= 192
*/
static inline libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b8(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_generate_keypair_db(randomness);
}

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
static inline libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_b8(
  const Eurydice_arr_c7 *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_generate_keypair_avx2_b8(randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.encapsulate_avx2
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
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_80(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_encapsulate_a1(public_key, randomness);
}

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
static inline tuple_ab
libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_80(
  const Eurydice_arr_03 *public_key,
  const Eurydice_arr_ec *randomness
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_encapsulate_avx2_80(public_key, randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.instantiations.avx2.decapsulate_avx2
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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_37(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_decapsulate_66(private_key, ciphertext);
}

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
static inline Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_37(
  const Eurydice_arr_ab0 *private_key,
  const Eurydice_arr_d2 *ciphertext
)
{
  return libcrux_ml_kem_ind_cca_instantiations_avx2_decapsulate_avx2_37(private_key, ciphertext);
}

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem_common_H_DEFINED
#endif /* internal_libcrux_mlkem_common_H */
