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
 * Libcrux: c580de08c2461add5a35427c264aeeacde26bcf5
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
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
uint8_t libcrux_secrets_int_as_u8_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_90((uint8_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u8}
*/
int16_t libcrux_secrets_int_as_i16_59(uint8_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_90(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
int32_t libcrux_secrets_int_as_i32_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_a8((int32_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i32}
*/
int16_t libcrux_secrets_int_as_i16_36(int32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)libcrux_secrets_int_public_integers_declassify_d8_a8(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u32}
*/
int32_t libcrux_secrets_int_as_i32_b8(uint32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_a8((int32_t)libcrux_secrets_int_public_integers_declassify_d8_df(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
uint16_t libcrux_secrets_int_as_u16_f5(int16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_de((uint16_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u16}
*/
int16_t libcrux_secrets_int_as_i16_ca(uint16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_de(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u16}
*/
uint64_t libcrux_secrets_int_as_u64_ca(uint16_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_49((uint64_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_de(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u64}
*/
uint32_t libcrux_secrets_int_as_u32_a3(uint64_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_df((uint32_t)libcrux_secrets_int_public_integers_declassify_d8_49(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u32}
*/
int16_t libcrux_secrets_int_as_i16_b8(uint32_t self)
{
  return
    libcrux_secrets_int_public_integers_classify_27_39((int16_t)libcrux_secrets_int_public_integers_declassify_d8_df(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
int16_t libcrux_secrets_int_as_i16_f5(int16_t self)
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
Eurydice_arr_ec libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_borrow_slice_u8 slice)
{
  Eurydice_arr_ec out = { .data = { 0U } };
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d46(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
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
Eurydice_arr_7d libcrux_ml_kem_types_default_d3_79(void)
{
  return (KRML_CLITERAL(Eurydice_arr_7d){ .data = { 0U } });
}

/**
This function found in impl {core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_51
with const generics
- SIZE= 1184
*/
Eurydice_arr_5f libcrux_ml_kem_types_from_51_3d(Eurydice_arr_5f value)
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
This function found in impl {core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_b2
with const generics
- SIZE= 2400
*/
Eurydice_arr_7d libcrux_ml_kem_types_from_b2_79(Eurydice_arr_7d value)
{
  return value;
}

/**
This function found in impl {core::convert::From<[u8; SIZE]> for libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_19
with const generics
- SIZE= 1088
*/
Eurydice_arr_2b libcrux_ml_kem_types_from_19_52(Eurydice_arr_2b value)
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
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d40(&out,
      (KRML_CLITERAL(core_ops_range_Range_87){ .start = (size_t)0U, .end = slice.meta })),
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
Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_c1_52(const Eurydice_arr_2b *self)
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

