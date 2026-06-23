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


#include "internal/libcrux_mlkem_core.h"

#include "combined_core.h"
#include "internal/combined_core.h"

/**
 K * BITS_PER_RING_ELEMENT / 8

 [eurydice] Note that we can't use const generics here because that breaks
            C extraction with eurydice.
*/
size_t libcrux_ml_kem_constants_ranked_bytes_per_ring_element(size_t rank)
{
  return rank * LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U;
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
uint8_t libcrux_secrets_int_as_u8_e5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_90((uint8_t)libcrux_secrets_int_public_integers_declassify_22_39(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u8}
*/
int16_t libcrux_secrets_int_as_i16_c3(uint8_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_39((int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_22_90(self));
}

int16_t libcrux_secrets_int_I16(int16_t v)
{
  return libcrux_secrets_int_public_integers_secret_39(v);
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
int32_t libcrux_secrets_int_as_i32_e5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_a8((int32_t)libcrux_secrets_int_public_integers_declassify_22_39(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i32}
*/
int16_t libcrux_secrets_int_as_i16_06(int32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_39((int16_t)libcrux_secrets_int_public_integers_declassify_22_a8(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u32}
*/
int32_t libcrux_secrets_int_as_i32_c6(uint32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_a8((int32_t)libcrux_secrets_int_public_integers_declassify_22_df(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
uint16_t libcrux_secrets_int_as_u16_e5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_de((uint16_t)libcrux_secrets_int_public_integers_declassify_22_39(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u16}
*/
int16_t libcrux_secrets_int_as_i16_80(uint16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_39((int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_22_de(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u16}
*/
uint64_t libcrux_secrets_int_as_u64_80(uint16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_49((uint64_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_22_de(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u64}
*/
uint32_t libcrux_secrets_int_as_u32_11(uint64_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_df((uint32_t)libcrux_secrets_int_public_integers_declassify_22_49(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for u32}
*/
int16_t libcrux_secrets_int_as_i16_c6(uint32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_39((int16_t)libcrux_secrets_int_public_integers_declassify_22_df(self));
}

/**
This function found in impl {impl libcrux_secrets::int::CastOps for i16}
*/
int16_t libcrux_secrets_int_as_i16_e5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_f9_39(libcrux_secrets_int_public_integers_declassify_22_39(self));
}

/**
This function found in impl {impl core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_43
with const generics
- SIZE= 3168
*/
Eurydice_arr_a8 libcrux_ml_kem_types_default_43_0e(void)
{
  return (KRML_CLITERAL(Eurydice_arr_a8){ .data = { 0U } });
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_bd
with const generics
- SIZE= 1568
*/
Eurydice_arr_d1 libcrux_ml_kem_types_from_bd_d9(Eurydice_arr_d1 value)
{
  return value;
}

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
libcrux_ml_kem_types_from_17_70(Eurydice_arr_a8 sk, Eurydice_arr_d1 pk)
{
  return (KRML_CLITERAL(libcrux_ml_kem_mlkem1024_MlKem1024KeyPair){ .sk = sk, .pk = pk });
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_3b
with const generics
- SIZE= 3168
*/
Eurydice_arr_a8 libcrux_ml_kem_types_from_3b_0e(Eurydice_arr_a8 value)
{
  return value;
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_63
with const generics
- SIZE= 1568
*/
Eurydice_arr_d1 libcrux_ml_kem_types_from_63_d9(Eurydice_arr_d1 value)
{
  return value;
}

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
const Eurydice_arr_d1 *libcrux_ml_kem_types_as_slice_e6_d9(const Eurydice_arr_d1 *self)
{
  return self;
}

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
const Eurydice_arr_d1 *libcrux_ml_kem_types_as_slice_a9_d9(const Eurydice_arr_d1 *self)
{
  return self;
}

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 4
*/
uint8_t
libcrux_ml_kem_utils_prf_input_inc_23(Eurydice_arr_d20 *prf_inputs, uint8_t domain_separator)
{
  for (size_t i = (size_t)0U; i < (size_t)4U; i++)
  {
    size_t i0 = i;
    prf_inputs->data[i0].data[32U] = domain_separator;
    domain_separator = (uint32_t)domain_separator + 1U;
  }
  return domain_separator;
}

/**
This function found in impl {impl core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_17
with const generics
- SIZE= 1568
*/
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_17_d9(const Eurydice_arr_d1 *self)
{
  return Eurydice_array_to_slice_shared_b50(self);
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 1600
*/
Eurydice_arr_14 libcrux_ml_kem_utils_into_padded_array_49(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_14 out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d421(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

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
libcrux_ml_kem_types_unpack_private_key_e3(Eurydice_borrow_slice_u8 private_key)
{
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(private_key,
      (size_t)1536U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 secret_key0 = uu____0.snd;
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(secret_key0,
      (size_t)1568U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____1.fst;
  Eurydice_borrow_slice_u8 secret_key = uu____1.snd;
  Eurydice_borrow_slice_u8_x2
  uu____2 =
    Eurydice_slice_split_at(secret_key,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____2.snd;
  return
    (
      KRML_CLITERAL(Eurydice_borrow_slice_u8_x4){
        .fst = ind_cpa_secret_key,
        .snd = ind_cpa_public_key,
        .thd = ind_cpa_public_key_hash,
        .f3 = implicit_rejection_value
      }
    );
}

/**
This function found in impl {impl core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_43
with const generics
- SIZE= 2400
*/
Eurydice_arr_7d libcrux_ml_kem_types_default_43_79(void)
{
  return (KRML_CLITERAL(Eurydice_arr_7d){ .data = { 0U } });
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_bd
with const generics
- SIZE= 1184
*/
Eurydice_arr_5f libcrux_ml_kem_types_from_bd_3d(Eurydice_arr_5f value)
{
  return value;
}

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
libcrux_ml_kem_types_from_17_bc(Eurydice_arr_7d sk, Eurydice_arr_5f pk)
{
  return (KRML_CLITERAL(libcrux_ml_kem_mlkem768_MlKem768KeyPair){ .sk = sk, .pk = pk });
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_3b
with const generics
- SIZE= 2400
*/
Eurydice_arr_7d libcrux_ml_kem_types_from_3b_79(Eurydice_arr_7d value)
{
  return value;
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_63
with const generics
- SIZE= 1088
*/
Eurydice_arr_2b libcrux_ml_kem_types_from_63_52(Eurydice_arr_2b value)
{
  return value;
}

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
const Eurydice_arr_5f *libcrux_ml_kem_types_as_slice_e6_3d(const Eurydice_arr_5f *self)
{
  return self;
}

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
const Eurydice_arr_2b *libcrux_ml_kem_types_as_slice_a9_52(const Eurydice_arr_2b *self)
{
  return self;
}

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 3
*/
uint8_t
libcrux_ml_kem_utils_prf_input_inc_78(Eurydice_arr_fd *prf_inputs, uint8_t domain_separator)
{
  for (size_t i = (size_t)0U; i < (size_t)3U; i++)
  {
    size_t i0 = i;
    prf_inputs->data[i0].data[32U] = domain_separator;
    domain_separator = (uint32_t)domain_separator + 1U;
  }
  return domain_separator;
}

/**
This function found in impl {impl core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_17
with const generics
- SIZE= 1088
*/
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_17_52(const Eurydice_arr_2b *self)
{
  return Eurydice_array_to_slice_shared_06(self);
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 1120
*/
Eurydice_arr_af libcrux_ml_kem_utils_into_padded_array_66(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_af out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d417(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

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
libcrux_ml_kem_types_unpack_private_key_64(Eurydice_borrow_slice_u8 private_key)
{
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(private_key,
      (size_t)1152U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 secret_key0 = uu____0.snd;
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(secret_key0,
      (size_t)1184U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____1.fst;
  Eurydice_borrow_slice_u8 secret_key = uu____1.snd;
  Eurydice_borrow_slice_u8_x2
  uu____2 =
    Eurydice_slice_split_at(secret_key,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____2.snd;
  return
    (
      KRML_CLITERAL(Eurydice_borrow_slice_u8_x4){
        .fst = ind_cpa_secret_key,
        .snd = ind_cpa_public_key,
        .thd = ind_cpa_public_key_hash,
        .f3 = implicit_rejection_value
      }
    );
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 32
*/
Eurydice_arr_ec libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_ec out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d44(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

/**
This function found in impl {impl core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_43
with const generics
- SIZE= 1632
*/
Eurydice_arr_ab0 libcrux_ml_kem_types_default_43_be(void)
{
  return (KRML_CLITERAL(Eurydice_arr_ab0){ .data = { 0U } });
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_bd
with const generics
- SIZE= 800
*/
Eurydice_arr_03 libcrux_ml_kem_types_from_bd_df(Eurydice_arr_03 value)
{
  return value;
}

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
libcrux_ml_kem_types_from_17_d6(Eurydice_arr_ab0 sk, Eurydice_arr_03 pk)
{
  return (KRML_CLITERAL(libcrux_ml_kem_types_MlKemKeyPair_0d){ .sk = sk, .pk = pk });
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_3b
with const generics
- SIZE= 1632
*/
Eurydice_arr_ab0 libcrux_ml_kem_types_from_3b_be(Eurydice_arr_ab0 value)
{
  return value;
}

/**
This function found in impl {impl core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_63
with const generics
- SIZE= 768
*/
Eurydice_arr_d2 libcrux_ml_kem_types_from_63_80(Eurydice_arr_d2 value)
{
  return value;
}

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
const Eurydice_arr_03 *libcrux_ml_kem_types_as_slice_e6_df(const Eurydice_arr_03 *self)
{
  return self;
}

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
const Eurydice_arr_d2 *libcrux_ml_kem_types_as_slice_a9_80(const Eurydice_arr_d2 *self)
{
  return self;
}

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 2
*/
uint8_t
libcrux_ml_kem_utils_prf_input_inc_af(Eurydice_arr_1b0 *prf_inputs, uint8_t domain_separator)
{
  for (size_t i = (size_t)0U; i < (size_t)2U; i++)
  {
    size_t i0 = i;
    prf_inputs->data[i0].data[32U] = domain_separator;
    domain_separator = (uint32_t)domain_separator + 1U;
  }
  return domain_separator;
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 33
*/
Eurydice_arr_fa0 libcrux_ml_kem_utils_into_padded_array_29(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_fa0 out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d412(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 34
*/
Eurydice_arr_31 libcrux_ml_kem_utils_into_padded_array_de(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_31 out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d46(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

/**
This function found in impl {impl core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_17
with const generics
- SIZE= 768
*/
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_17_80(const Eurydice_arr_d2 *self)
{
  return Eurydice_array_to_slice_shared_27(self);
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 800
*/
Eurydice_arr_03 libcrux_ml_kem_utils_into_padded_array_df(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_03 out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d411(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 64
*/
Eurydice_arr_c7 libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_c7 out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d410(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
    slice,
    uint8_t);
  return out;
}

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
libcrux_ml_kem_types_unpack_private_key_e0(Eurydice_borrow_slice_u8 private_key)
{
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(private_key,
      (size_t)768U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 secret_key0 = uu____0.snd;
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(secret_key0,
      (size_t)800U,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____1.fst;
  Eurydice_borrow_slice_u8 secret_key = uu____1.snd;
  Eurydice_borrow_slice_u8_x2
  uu____2 =
    Eurydice_slice_split_at(secret_key,
      LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____2.snd;
  return
    (
      KRML_CLITERAL(Eurydice_borrow_slice_u8_x4){
        .fst = ind_cpa_secret_key,
        .snd = ind_cpa_public_key,
        .thd = ind_cpa_public_key_hash,
        .f3 = implicit_rejection_value
      }
    );
}

