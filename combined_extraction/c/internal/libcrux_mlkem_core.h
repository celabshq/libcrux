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


#ifndef internal_libcrux_mlkem_core_H
#define internal_libcrux_mlkem_core_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "internal/combined_core.h"
#include "combined_core.h"
#include "../libcrux_mlkem_core.h"

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
uint8_t libcrux_secrets_int_as_u8_e5(int16_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u8}
*/
int16_t libcrux_secrets_int_as_i16_c3(uint8_t self);

int16_t libcrux_secrets_int_I16(int16_t v);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
int32_t libcrux_secrets_int_as_i32_e5(int16_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i32}
*/
int16_t libcrux_secrets_int_as_i16_06(int32_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u32}
*/
int32_t libcrux_secrets_int_as_i32_c6(uint32_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
uint16_t libcrux_secrets_int_as_u16_e5(int16_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u16}
*/
int16_t libcrux_secrets_int_as_i16_80(uint16_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u16}
*/
uint64_t libcrux_secrets_int_as_u64_80(uint16_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u64}
*/
uint32_t libcrux_secrets_int_as_u32_11(uint64_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u32}
*/
int16_t libcrux_secrets_int_as_i16_c6(uint32_t self);

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
int16_t libcrux_secrets_int_as_i16_e5(int16_t self);

/**
This function found in impl {impl core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_43
with const generics
- SIZE= 3168
*/
Eurydice_arr_a8 libcrux_ml_kem_types_default_43_0e(void);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_bd
with const generics
- SIZE= 1568
*/
Eurydice_arr_d1 libcrux_ml_kem_types_from_bd_d9(Eurydice_arr_d1 value);

/**
 Create a new [`MlKemKeyPair`] from the secret and public key.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_17
with const generics
- PRIVATE_KEY_SIZE= 3168
- PUBLIC_KEY_SIZE= 1568
*/
libcrux_ml_kem_mlkem1024_MlKem1024KeyPair
libcrux_ml_kem_types_from_17_70(Eurydice_arr_a8 sk, Eurydice_arr_d1 pk);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_3b
with const generics
- SIZE= 3168
*/
Eurydice_arr_a8 libcrux_ml_kem_types_from_3b_0e(Eurydice_arr_a8 value);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_63
with const generics
- SIZE= 1568
*/
Eurydice_arr_d1 libcrux_ml_kem_types_from_63_d9(Eurydice_arr_d1 value);

/**
 A reference to the raw byte slice.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_slice_e6
with const generics
- SIZE= 1568
*/
const Eurydice_arr_d1 *libcrux_ml_kem_types_as_slice_e6_d9(const Eurydice_arr_d1 *self);

/**
 A reference to the raw byte slice.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_slice_a9
with const generics
- SIZE= 1568
*/
const Eurydice_arr_d1 *libcrux_ml_kem_types_as_slice_a9_d9(const Eurydice_arr_d1 *self);

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 4
*/
uint8_t
libcrux_ml_kem_utils_prf_input_inc_23(Eurydice_arr_d20 *prf_inputs, uint8_t domain_separator);

/**
This function found in impl {impl core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_17
with const generics
- SIZE= 1568
*/
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_17_d9(const Eurydice_arr_d1 *self);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 1600
*/
Eurydice_arr_14 libcrux_ml_kem_utils_into_padded_array_49(Eurydice_borrow_slice_u8 slice);

typedef struct Eurydice_borrow_slice_u8_x4_s
{
  Eurydice_borrow_slice_u8 fst;
  Eurydice_borrow_slice_u8 snd;
  Eurydice_borrow_slice_u8 thd;
  Eurydice_borrow_slice_u8 f3;
}
Eurydice_borrow_slice_u8_x4;

typedef struct Eurydice_borrow_slice_u8_x2_s
{
  Eurydice_borrow_slice_u8 fst;
  Eurydice_borrow_slice_u8 snd;
}
Eurydice_borrow_slice_u8_x2;

/**
 Unpack an incoming private key into it's different parts.

 We have this here in types to extract into a common core for C.
*/
/**
A monomorphic instance of libcrux_ml_kem.types.unpack_private_key
with const generics
- CPA_SECRET_KEY_SIZE= 1536
- PUBLIC_KEY_SIZE= 1568
*/
Eurydice_borrow_slice_u8_x4
libcrux_ml_kem_types_unpack_private_key_e3(Eurydice_borrow_slice_u8 private_key);

/**
This function found in impl {impl core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_43
with const generics
- SIZE= 2400
*/
Eurydice_arr_7d libcrux_ml_kem_types_default_43_79(void);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_bd
with const generics
- SIZE= 1184
*/
Eurydice_arr_5f libcrux_ml_kem_types_from_bd_3d(Eurydice_arr_5f value);

/**
 Create a new [`MlKemKeyPair`] from the secret and public key.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_17
with const generics
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_types_from_17_bc(Eurydice_arr_7d sk, Eurydice_arr_5f pk);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_3b
with const generics
- SIZE= 2400
*/
Eurydice_arr_7d libcrux_ml_kem_types_from_3b_79(Eurydice_arr_7d value);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_63
with const generics
- SIZE= 1088
*/
Eurydice_arr_2b libcrux_ml_kem_types_from_63_52(Eurydice_arr_2b value);

/**
 A reference to the raw byte slice.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_slice_e6
with const generics
- SIZE= 1184
*/
const Eurydice_arr_5f *libcrux_ml_kem_types_as_slice_e6_3d(const Eurydice_arr_5f *self);

/**
 A reference to the raw byte slice.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_slice_a9
with const generics
- SIZE= 1088
*/
const Eurydice_arr_2b *libcrux_ml_kem_types_as_slice_a9_52(const Eurydice_arr_2b *self);

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 3
*/
uint8_t
libcrux_ml_kem_utils_prf_input_inc_78(Eurydice_arr_fd *prf_inputs, uint8_t domain_separator);

/**
This function found in impl {impl core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_17
with const generics
- SIZE= 1088
*/
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_17_52(const Eurydice_arr_2b *self);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 1120
*/
Eurydice_arr_af libcrux_ml_kem_utils_into_padded_array_66(Eurydice_borrow_slice_u8 slice);

/**
 Unpack an incoming private key into it's different parts.

 We have this here in types to extract into a common core for C.
*/
/**
A monomorphic instance of libcrux_ml_kem.types.unpack_private_key
with const generics
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
Eurydice_borrow_slice_u8_x4
libcrux_ml_kem_types_unpack_private_key_64(Eurydice_borrow_slice_u8 private_key);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 32
*/
Eurydice_arr_ec libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_borrow_slice_u8 slice);

/**
This function found in impl {impl core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_43
with const generics
- SIZE= 1632
*/
Eurydice_arr_ab0 libcrux_ml_kem_types_default_43_be(void);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_bd
with const generics
- SIZE= 800
*/
Eurydice_arr_03 libcrux_ml_kem_types_from_bd_df(Eurydice_arr_03 value);

/**
 Create a new [`MlKemKeyPair`] from the secret and public key.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_17
with const generics
- PRIVATE_KEY_SIZE= 1632
- PUBLIC_KEY_SIZE= 800
*/
libcrux_ml_kem_types_MlKemKeyPair_0d
libcrux_ml_kem_types_from_17_d6(Eurydice_arr_ab0 sk, Eurydice_arr_03 pk);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_3b
with const generics
- SIZE= 1632
*/
Eurydice_arr_ab0 libcrux_ml_kem_types_from_3b_be(Eurydice_arr_ab0 value);

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_63
with const generics
- SIZE= 768
*/
Eurydice_arr_d2 libcrux_ml_kem_types_from_63_80(Eurydice_arr_d2 value);

/**
 A reference to the raw byte slice.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_slice_e6
with const generics
- SIZE= 800
*/
const Eurydice_arr_03 *libcrux_ml_kem_types_as_slice_e6_df(const Eurydice_arr_03 *self);

/**
 A reference to the raw byte slice.
*/
/**
This function found in impl {libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_slice_a9
with const generics
- SIZE= 768
*/
const Eurydice_arr_d2 *libcrux_ml_kem_types_as_slice_a9_80(const Eurydice_arr_d2 *self);

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 2
*/
uint8_t
libcrux_ml_kem_utils_prf_input_inc_af(Eurydice_arr_1b0 *prf_inputs, uint8_t domain_separator);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 33
*/
Eurydice_arr_fa0 libcrux_ml_kem_utils_into_padded_array_29(Eurydice_borrow_slice_u8 slice);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 34
*/
Eurydice_arr_31 libcrux_ml_kem_utils_into_padded_array_de(Eurydice_borrow_slice_u8 slice);

/**
This function found in impl {impl core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_17
with const generics
- SIZE= 768
*/
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_17_80(const Eurydice_arr_d2 *self);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 800
*/
Eurydice_arr_03 libcrux_ml_kem_utils_into_padded_array_df(Eurydice_borrow_slice_u8 slice);

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 64
*/
Eurydice_arr_c7 libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_borrow_slice_u8 slice);

/**
 Unpack an incoming private key into it's different parts.

 We have this here in types to extract into a common core for C.
*/
/**
A monomorphic instance of libcrux_ml_kem.types.unpack_private_key
with const generics
- CPA_SECRET_KEY_SIZE= 768
- PUBLIC_KEY_SIZE= 800
*/
Eurydice_borrow_slice_u8_x4
libcrux_ml_kem_types_unpack_private_key_e0(Eurydice_borrow_slice_u8 private_key);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mlkem_core_H_DEFINED
#endif /* internal_libcrux_mlkem_core_H */
