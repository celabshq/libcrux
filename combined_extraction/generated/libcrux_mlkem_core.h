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
 * Libcrux: c5f306978fa7d13b590fd71dbc260386f3936d57
 */


#ifndef libcrux_mlkem_core_H
#define libcrux_mlkem_core_H

#include "eurydice_glue.h"



#include "combined_core.h"

#define LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE ((size_t)32U)

#define LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT ((size_t)12U)

#define LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT ((size_t)256U)

#define LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)12U)

#define LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT (LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE ((size_t)32U)

#define LIBCRUX_ML_KEM_CONSTANTS_G_DIGEST_SIZE ((size_t)64U)

#define LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE ((size_t)32U)

/**
 K * BITS_PER_RING_ELEMENT / 8

 [eurydice] Note that we can't use const generics here because that breaks
            C extraction with eurydice.
*/
static inline size_t libcrux_ml_kem_constants_ranked_bytes_per_ring_element(size_t rank)
{
  return rank * LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE uint8_t libcrux_secrets_int_as_u8_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_90((uint8_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u8}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_59(uint8_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_90(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE int32_t libcrux_secrets_int_as_i32_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_a8((int32_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i32}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_36(int32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)libcrux_secrets_int_public_integers_declassify_d8_a8(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u32}
*/
static KRML_MUSTINLINE int32_t libcrux_secrets_int_as_i32_b8(uint32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_a8((int32_t)libcrux_secrets_int_public_integers_declassify_d8_df(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE uint16_t libcrux_secrets_int_as_u16_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_de((uint16_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u16}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_ca(uint16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_de(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u16}
*/
static KRML_MUSTINLINE uint64_t libcrux_secrets_int_as_u64_ca(uint16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_49((uint64_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_de(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u64}
*/
static KRML_MUSTINLINE uint32_t libcrux_secrets_int_as_u32_a3(uint64_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_df((uint32_t)libcrux_secrets_int_public_integers_declassify_d8_49(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u32}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_b8(uint32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)libcrux_secrets_int_public_integers_declassify_d8_df(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39(libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 32
*/
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_ec out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d46(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
    slice,
    uint8_t);
  return out;
}

/**
This function found in impl {core::default::Default for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_d3
with const generics
- SIZE= 2400
*/
static inline Eurydice_arr_7d libcrux_ml_kem_types_default_d3_79(void)
{
  return (Eurydice_arr_7d{ { 0U } });
}

/**
This function found in impl {core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_51
with const generics
- SIZE= 1184
*/
static inline Eurydice_arr_5f libcrux_ml_kem_types_from_51_3d(Eurydice_arr_5f value)
{
  return value;
}

typedef struct libcrux_ml_kem_mlkem768_MlKem768KeyPair_s
{
  Eurydice_arr_7d sk;
  Eurydice_arr_5f pk;
}
libcrux_ml_kem_mlkem768_MlKem768KeyPair;

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
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_types_from_17_bc(Eurydice_arr_7d sk, Eurydice_arr_5f pk)
{
  return (libcrux_ml_kem_mlkem768_MlKem768KeyPair{ sk, pk });
}

/**
This function found in impl {core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_b2
with const generics
- SIZE= 2400
*/
static inline Eurydice_arr_7d libcrux_ml_kem_types_from_b2_79(Eurydice_arr_7d value)
{
  return value;
}

/**
A monomorphic instance of n-tuple
with types libcrux_ml_kem_mlkem768_MlKem768Ciphertext, Eurydice_arr_ec

*/
typedef struct tuple_f4_s
{
  Eurydice_arr_2b fst;
  Eurydice_arr_ec snd;
}
tuple_f4;

/**
This function found in impl {core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_19
with const generics
- SIZE= 1088
*/
static inline Eurydice_arr_2b libcrux_ml_kem_types_from_19_52(Eurydice_arr_2b value)
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
static inline const
Eurydice_arr_5f
*libcrux_ml_kem_types_as_slice_e6_3d(const Eurydice_arr_5f *self)
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
static inline const
Eurydice_arr_2b
*libcrux_ml_kem_types_as_slice_a9_52(const Eurydice_arr_2b *self)
{
  return self;
}

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 3
*/
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_utils_prf_input_inc_78(Eurydice_arr_800 *prf_inputs, uint8_t domain_separator)
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
 Pad the `slice` with `0`s at the end.
*/
/**
A monomorphic instance of libcrux_ml_kem.utils.into_padded_array
with const generics
- LEN= 33
*/
static KRML_MUSTINLINE Eurydice_arr_fa
libcrux_ml_kem_utils_into_padded_array_29(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_fa out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d412(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
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
static KRML_MUSTINLINE Eurydice_arr_31
libcrux_ml_kem_utils_into_padded_array_de(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_31 out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d40(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
    slice,
    uint8_t);
  return out;
}

/**
This function found in impl {core::convert::AsRef<[u8]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_c1
with const generics
- SIZE= 1088
*/
static inline Eurydice_borrow_slice_u8
libcrux_ml_kem_types_as_ref_c1_52(const Eurydice_arr_2b *self)
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
static KRML_MUSTINLINE Eurydice_arr_af
libcrux_ml_kem_utils_into_padded_array_66(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_af out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d411(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
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
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_c7 out = { { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d410(&out,
      (core_ops_range_Range_87{ (size_t)0U, slice.meta })),
    slice,
    uint8_t);
  return out;
}

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
- CPA_SECRET_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
static inline Eurydice_borrow_slice_u8_x4
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
      Eurydice_borrow_slice_u8_x4{
        ind_cpa_secret_key,
        ind_cpa_public_key,
        ind_cpa_public_key_hash,
        implicit_rejection_value
      }
    );
}


#define libcrux_mlkem_core_H_DEFINED
#endif /* libcrux_mlkem_core_H */
