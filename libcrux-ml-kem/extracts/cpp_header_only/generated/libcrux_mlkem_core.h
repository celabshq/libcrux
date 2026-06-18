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
 * Libcrux: ae092f4921933e45dec31610a1465191171e5b3f
 */

#ifndef libcrux_mlkem_core_H
#define libcrux_mlkem_core_H

#include "eurydice_glue.h"

#define None 0
#define Some 1

typedef uint8_t Option_87_tags;

/**
A monomorphic instance of core.option.Option
with types size_t

*/
typedef struct Option_87_s {
  Option_87_tags tag;
  size_t f0;
} Option_87;

static inline uint16_t core_num__u16__wrapping_add(uint16_t x0, uint16_t x1);

static inline uint64_t core_num__u64__from_le_bytes(Eurydice_array_u8x8 x0);

static inline uint64_t core_num__u64__rotate_left(uint64_t x0, uint32_t x1);

static inline Eurydice_array_u8x8 core_num__u64__to_le_bytes(uint64_t x0);

static inline uint32_t core_num__u8__count_ones(uint8_t x0);

static inline uint8_t core_num__u8__wrapping_sub(uint8_t x0, uint8_t x1);

/**
A monomorphic instance of core.ops.range.Range
with types size_t

*/
typedef struct core_ops_range_Range_87_s {
  size_t start;
  size_t end;
} core_ops_range_Range_87;

#define LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE ((size_t)32U)

#define LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT ((size_t)12U)

#define LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT ((size_t)256U)

#define LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT \
  (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * (size_t)12U)

#define LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT \
  (LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE ((size_t)32U)

#define LIBCRUX_ML_KEM_CONSTANTS_G_DIGEST_SIZE ((size_t)64U)

#define LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE ((size_t)32U)

/**
 K * BITS_PER_RING_ELEMENT / 8

 [eurydice] Note that we can't use const generics here because that breaks
            C extraction with eurydice.
*/
static inline size_t libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
    size_t rank) {
  return rank * LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint8_t

*/
static KRML_MUSTINLINE uint8_t
libcrux_secrets_int_public_integers_classify_27_90(uint8_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types int16_t

*/
static KRML_MUSTINLINE int16_t
libcrux_secrets_int_public_integers_declassify_d8_39(int16_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE uint8_t libcrux_secrets_int_as_u8_f5(int16_t self) {
  return libcrux_secrets_int_public_integers_classify_27_90(
      (uint8_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types int16_t

*/
static KRML_MUSTINLINE int16_t
libcrux_secrets_int_public_integers_classify_27_39(int16_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint8_t

*/
static KRML_MUSTINLINE uint8_t
libcrux_secrets_int_public_integers_declassify_d8_90(uint8_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u8}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_59(uint8_t self) {
  return libcrux_secrets_int_public_integers_classify_27_39(
      (int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_90(
          self));
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types int32_t

*/
static KRML_MUSTINLINE int32_t
libcrux_secrets_int_public_integers_classify_27_a8(int32_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE int32_t libcrux_secrets_int_as_i32_f5(int16_t self) {
  return libcrux_secrets_int_public_integers_classify_27_a8(
      (int32_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types int32_t

*/
static KRML_MUSTINLINE int32_t
libcrux_secrets_int_public_integers_declassify_d8_a8(int32_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i32}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_36(int32_t self) {
  return libcrux_secrets_int_public_integers_classify_27_39(
      (int16_t)libcrux_secrets_int_public_integers_declassify_d8_a8(self));
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint32_t

*/
static KRML_MUSTINLINE uint32_t
libcrux_secrets_int_public_integers_declassify_d8_df(uint32_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u32}
*/
static KRML_MUSTINLINE int32_t libcrux_secrets_int_as_i32_b8(uint32_t self) {
  return libcrux_secrets_int_public_integers_classify_27_a8(
      (int32_t)libcrux_secrets_int_public_integers_declassify_d8_df(self));
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint16_t

*/
static KRML_MUSTINLINE uint16_t
libcrux_secrets_int_public_integers_classify_27_de(uint16_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE uint16_t libcrux_secrets_int_as_u16_f5(int16_t self) {
  return libcrux_secrets_int_public_integers_classify_27_de(
      (uint16_t)libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint16_t

*/
static KRML_MUSTINLINE uint16_t
libcrux_secrets_int_public_integers_declassify_d8_de(uint16_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u16}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_ca(uint16_t self) {
  return libcrux_secrets_int_public_integers_classify_27_39(
      (int16_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_de(
          self));
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint64_t

*/
static KRML_MUSTINLINE uint64_t
libcrux_secrets_int_public_integers_classify_27_49(uint64_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u16}
*/
static KRML_MUSTINLINE uint64_t libcrux_secrets_int_as_u64_ca(uint16_t self) {
  return libcrux_secrets_int_public_integers_classify_27_49(
      (uint64_t)(uint32_t)libcrux_secrets_int_public_integers_declassify_d8_de(
          self));
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint32_t

*/
static KRML_MUSTINLINE uint32_t
libcrux_secrets_int_public_integers_classify_27_df(uint32_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint64_t

*/
static KRML_MUSTINLINE uint64_t
libcrux_secrets_int_public_integers_declassify_d8_49(uint64_t self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u64}
*/
static KRML_MUSTINLINE uint32_t libcrux_secrets_int_as_u32_a3(uint64_t self) {
  return libcrux_secrets_int_public_integers_classify_27_df(
      (uint32_t)libcrux_secrets_int_public_integers_declassify_d8_49(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for u32}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_b8(uint32_t self) {
  return libcrux_secrets_int_public_integers_classify_27_39(
      (int16_t)libcrux_secrets_int_public_integers_declassify_d8_df(self));
}

/**
This function found in impl {libcrux_secrets::int::CastOps for i16}
*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_as_i16_f5(int16_t self) {
  return libcrux_secrets_int_public_integers_classify_27_39(
      libcrux_secrets_int_public_integers_declassify_d8_39(self));
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1152size_t
*/
typedef struct Eurydice_arr_0e_s {
  uint8_t data[1152U];
} Eurydice_arr_0e;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1184size_t
*/
typedef struct Eurydice_arr_5f_s {
  uint8_t data[1184U];
} Eurydice_arr_5f;

typedef struct libcrux_ml_kem_utils_extraction_helper_Keypair768_s {
  Eurydice_arr_0e fst;
  Eurydice_arr_5f snd;
} libcrux_ml_kem_utils_extraction_helper_Keypair768;

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
static inline Eurydice_mut_borrow_slice_i16 Eurydice_slice_subslice_mut_a6(
    Eurydice_mut_borrow_slice_i16 s, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_i16{s.ptr + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_b20_s {
  uint8_t data[16U];
} Eurydice_arr_b20;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_29(
    const Eurydice_arr_b20 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_b20
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_e1_s {
  Eurydice_arr_b20 data[256U];
} Eurydice_arr_e1;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_94_s {
  uint8_t data[24U];
} Eurydice_arr_94;

#define Ok 0
#define Err 1

typedef uint8_t Result_57_tags;

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_94, core_array_TryFromSliceError

*/
typedef struct Result_57_s {
  Result_57_tags tag;
  union U {
    Eurydice_arr_94 case_Ok;
    TryFromSliceError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_57_s)
} Result_57;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$24size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_94 unwrap_26_78(Result_57 self) {
  if (self.tag == Ok) {
    return self.val.case_Ok;
  } else {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                      "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_d6_s {
  int16_t data[16U];
} Eurydice_arr_d6;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $20size_t
*/
typedef struct Eurydice_arr_fc_s {
  uint8_t data[20U];
} Eurydice_arr_fc;

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_fc, core_array_TryFromSliceError

*/
typedef struct Result_83_s {
  Result_57_tags tag;
  union U {
    Eurydice_arr_fc case_Ok;
    TryFromSliceError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_83_s)
} Result_83;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$20size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_fc unwrap_26_7d(Result_83 self) {
  if (self.tag == Ok) {
    return self.val.case_Ok;
  } else {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                      "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d45(
    const Eurydice_arr_b20 *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 16
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_29(
    Eurydice_arr_b20 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $168size_t
*/
typedef struct Eurydice_arr_c5_s {
  uint8_t data[168U];
} Eurydice_arr_c5;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_9c_s {
  Eurydice_arr_c5 data[4U];
} Eurydice_arr_9c;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $136size_t
*/
typedef struct Eurydice_arr_ff_s {
  uint8_t data[136U];
} Eurydice_arr_ff;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_ff
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_dc0_s {
  Eurydice_arr_ff data[4U];
} Eurydice_arr_dc0;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_borrow_slice_u8
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_68_s {
  Eurydice_borrow_slice_u8 data[4U];
} Eurydice_arr_68;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_ec_s {
  uint8_t data[32U];
} Eurydice_arr_ec;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d44(
    Eurydice_arr_ec *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
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
libcrux_ml_kem_utils_into_padded_array_ce(Eurydice_borrow_slice_u8 slice) {
  Eurydice_arr_ec out = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d44(
          &out, (core_ops_range_Range_87{(size_t)0U, slice.meta})),
      slice, uint8_t);
  return out;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2400size_t
*/
typedef struct Eurydice_arr_7d_s {
  uint8_t data[2400U];
} Eurydice_arr_7d;

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.default_d3
with const generics
- SIZE= 2400
*/
static inline Eurydice_arr_7d libcrux_ml_kem_types_default_d3_79(void) {
  return (Eurydice_arr_7d{{0U}});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 1184
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f1(const Eurydice_arr_5f *a, size_t r) {
  return (Eurydice_borrow_slice_u8{a->data + r, (size_t)1184U - r});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 1184
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_to_shared_210(
    const Eurydice_arr_5f *a, size_t r) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 2400
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d44(
    const Eurydice_arr_7d *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
This function found in impl {core::convert::From<[u8; SIZE]> for
libcrux_ml_kem::types::MlKemPublicKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_51
with const generics
- SIZE= 1184
*/
static inline Eurydice_arr_5f libcrux_ml_kem_types_from_51_3d(
    Eurydice_arr_5f value) {
  return value;
}

typedef struct libcrux_ml_kem_mlkem768_MlKem768KeyPair_s {
  Eurydice_arr_7d sk;
  Eurydice_arr_5f pk;
} libcrux_ml_kem_mlkem768_MlKem768KeyPair;

/**
 Create a new [`MlKemKeyPair`] from the secret and public key.
*/
/**
This function found in impl
{libcrux_ml_kem::types::MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_17
with const generics
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
*/
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_types_from_17_bc(Eurydice_arr_7d sk, Eurydice_arr_5f pk) {
  return (libcrux_ml_kem_mlkem768_MlKem768KeyPair{sk, pk});
}

/**
This function found in impl {core::convert::From<[u8; SIZE]> for
libcrux_ml_kem::types::MlKemPrivateKey<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_b2
with const generics
- SIZE= 2400
*/
static inline Eurydice_arr_7d libcrux_ml_kem_types_from_b2_79(
    Eurydice_arr_7d value) {
  return value;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1152
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f4(
    const Eurydice_arr_0e *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1152U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 2400
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d413(
    Eurydice_arr_7d *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1152
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f4(
    Eurydice_arr_0e *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1152U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 1184
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f2(Eurydice_arr_5f *a, size_t r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r, (size_t)1184U - r});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d412(
    Eurydice_arr_5f *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 24
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_ed(
    const Eurydice_arr_94 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)24U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $384size_t
*/
typedef struct Eurydice_arr_b2_s {
  uint8_t data[384U];
} Eurydice_arr_b2;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 384
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d411(
    Eurydice_arr_b2 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 384
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_a9(
    const Eurydice_arr_b2 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)384U;
  return lit;
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_ec, core_array_TryFromSliceError

*/
typedef struct Result_07_s {
  Result_57_tags tag;
  union U {
    Eurydice_arr_ec case_Ok;
    TryFromSliceError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_07_s)
} Result_07;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$32size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_ec unwrap_26_39(Result_07 self) {
  if (self.tag == Ok) {
    return self.val.case_Ok;
  } else {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                      "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $64size_t
*/
typedef struct Eurydice_arr_c7_s {
  uint8_t data[64U];
} Eurydice_arr_c7;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 64
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f0(const Eurydice_arr_c7 *a, size_t r) {
  return (Eurydice_borrow_slice_u8{a->data + r, (size_t)64U - r});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d43(
    const Eurydice_arr_c7 *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1088size_t
*/
typedef struct Eurydice_arr_2b_s {
  uint8_t data[1088U];
} Eurydice_arr_2b;

/**
A monomorphic instance of n-tuple
with types libcrux_ml_kem_mlkem768_MlKem768Ciphertext, Eurydice_arr_ec

*/
typedef struct tuple_f4_s {
  Eurydice_arr_2b fst;
  Eurydice_arr_ec snd;
} tuple_f4;

/**
This function found in impl {core::convert::From<[u8; SIZE]> for
libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.from_19
with const generics
- SIZE= 1088
*/
static inline Eurydice_arr_2b libcrux_ml_kem_types_from_19_52(
    Eurydice_arr_2b value) {
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
static inline const Eurydice_arr_5f *libcrux_ml_kem_types_as_slice_e6_3d(
    const Eurydice_arr_5f *self) {
  return self;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_ff(
    const Eurydice_arr_5f *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1184U;
  return lit;
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
static inline const Eurydice_arr_2b *libcrux_ml_kem_types_as_slice_a9_52(
    const Eurydice_arr_2b *self) {
  return self;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 1088
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f1(Eurydice_arr_2b *a, size_t r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r, (size_t)1088U - r});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d42(
    const Eurydice_arr_ec *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d410(
    Eurydice_arr_2b *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 20
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_8f(
    const Eurydice_arr_fc *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)20U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $320size_t
*/
typedef struct Eurydice_arr_b0_s {
  uint8_t data[320U];
} Eurydice_arr_b0;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 320
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d49(
    Eurydice_arr_b0 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 320
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_56(
    const Eurydice_arr_b0 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)320U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $128size_t
*/
typedef struct Eurydice_arr_89_s {
  uint8_t data[128U];
} Eurydice_arr_89;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 128
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_78(
    const Eurydice_arr_89 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)128U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_04_s {
  int16_t data[256U];
} Eurydice_arr_04;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int16_t
with const generics
- N= 256
*/
static inline Eurydice_borrow_slice_i16 Eurydice_array_to_slice_shared_99(
    const Eurydice_arr_04 *a) {
  Eurydice_borrow_slice_i16 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)256U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_89
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_58_s {
  Eurydice_arr_89 data[3U];
} Eurydice_arr_58;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $33size_t
*/
typedef struct Eurydice_arr_fa_s {
  uint8_t data[33U];
} Eurydice_arr_fa;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 33
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_b5(
    const Eurydice_arr_fa *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)33U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 128
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_78(
    Eurydice_arr_89 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)128U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_fa
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_800_s {
  Eurydice_arr_fa data[3U];
} Eurydice_arr_800;

/**
A monomorphic instance of libcrux_ml_kem.utils.prf_input_inc
with const generics
- K= 3
*/
static KRML_MUSTINLINE uint8_t libcrux_ml_kem_utils_prf_input_inc_78(
    Eurydice_arr_800 *prf_inputs, uint8_t domain_separator) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    prf_inputs->data[i0].data[32U] = domain_separator;
    domain_separator = (uint32_t)domain_separator + 1U;
  }
  return domain_separator;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 33
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d48(
    Eurydice_arr_fa *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
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
libcrux_ml_kem_utils_into_padded_array_29(Eurydice_borrow_slice_u8 slice) {
  Eurydice_arr_fa out = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d48(
          &out, (core_ops_range_Range_87{(size_t)0U, slice.meta})),
      slice, uint8_t);
  return out;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $34size_t
*/
typedef struct Eurydice_arr_31_s {
  uint8_t data[34U];
} Eurydice_arr_31;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 34
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d47(
    Eurydice_arr_31 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
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
libcrux_ml_kem_utils_into_padded_array_de(Eurydice_borrow_slice_u8 slice) {
  Eurydice_arr_31 out = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d47(
          &out, (core_ops_range_Range_87{(size_t)0U, slice.meta})),
      slice, uint8_t);
  return out;
}

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $272size_t
*/
typedef struct Eurydice_arr_5b_s {
  int16_t data[272U];
} Eurydice_arr_5b;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
static inline Eurydice_borrow_slice_i16 Eurydice_array_to_subslice_shared_e70(
    const Eurydice_arr_5b *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_i16{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d41(
    const Eurydice_arr_c5 *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_2c_s {
  Eurydice_arr_c5 data[3U];
} Eurydice_arr_2c;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 168
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_2c(
    Eurydice_arr_c5 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)168U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_5b
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_b1_s {
  Eurydice_arr_5b data[3U];
} Eurydice_arr_b1;

/**
A monomorphic instance of Eurydice.arr
with types size_t
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_eb_s {
  size_t data[3U];
} Eurydice_arr_eb;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
static inline Eurydice_mut_borrow_slice_i16 Eurydice_array_to_subslice_mut_e7(
    Eurydice_arr_5b *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_i16{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $504size_t
*/
typedef struct Eurydice_arr_79_s {
  uint8_t data[504U];
} Eurydice_arr_79;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 504
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d40(
    const Eurydice_arr_79 *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_79
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_7e_s {
  Eurydice_arr_79 data[3U];
} Eurydice_arr_7e;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 504
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_48(
    Eurydice_arr_79 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)504U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_31
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_81_s {
  Eurydice_arr_31 data[3U];
} Eurydice_arr_81;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 34
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e9(
    const Eurydice_arr_31 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)34U;
  return lit;
}

/**
A monomorphic instance of Eurydice.slice_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t

*/
static inline Eurydice_borrow_slice_u8 Eurydice_slice_subslice_from_shared_6d(
    Eurydice_borrow_slice_u8 s, size_t r) {
  return (Eurydice_borrow_slice_u8{s.ptr + r, s.meta - r});
}

/**
A monomorphic instance of Eurydice.slice_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice
uint8_t

*/
static inline Eurydice_borrow_slice_u8 Eurydice_slice_subslice_to_shared_72(
    Eurydice_borrow_slice_u8 s, size_t r) {
  return (Eurydice_borrow_slice_u8{s.ptr, r});
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1120size_t
*/
typedef struct Eurydice_arr_af_s {
  uint8_t data[1120U];
} Eurydice_arr_af;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1120
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_81(
    const Eurydice_arr_af *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1120U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_06(
    const Eurydice_arr_2b *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1088U;
  return lit;
}

/**
This function found in impl {core::convert::AsRef<[u8]> for
libcrux_ml_kem::types::MlKemCiphertext<SIZE>}
*/
/**
A monomorphic instance of libcrux_ml_kem.types.as_ref_c1
with const generics
- SIZE= 1088
*/
static inline Eurydice_borrow_slice_u8 libcrux_ml_kem_types_as_ref_c1_52(
    const Eurydice_arr_2b *self) {
  return Eurydice_array_to_slice_shared_06(self);
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 1120
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f0(Eurydice_arr_af *a, size_t r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r, (size_t)1120U - r});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1120
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d46(
    Eurydice_arr_af *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
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
libcrux_ml_kem_utils_into_padded_array_66(Eurydice_borrow_slice_u8 slice) {
  Eurydice_arr_af out = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d46(
          &out, (core_ops_range_Range_87{(size_t)0U, slice.meta})),
      slice, uint8_t);
  return out;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 64
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_17(
    const Eurydice_arr_c7 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)64U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 64
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f(Eurydice_arr_c7 *a, size_t r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r, (size_t)64U - r});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 32
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_01(
    const Eurydice_arr_ec *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)32U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d45(
    Eurydice_arr_c7 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
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
libcrux_ml_kem_utils_into_padded_array_c9(Eurydice_borrow_slice_u8 slice) {
  Eurydice_arr_c7 out = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d45(
          &out, (core_ops_range_Range_87{(size_t)0U, slice.meta})),
      slice, uint8_t);
  return out;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_82(
    const Eurydice_array_u8x2 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 1088
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f(const Eurydice_arr_2b *a, size_t r) {
  return (Eurydice_borrow_slice_u8{a->data + r, (size_t)1088U - r});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_shared_d4(
    const Eurydice_arr_2b *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2400
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_51(
    const Eurydice_arr_7d *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2400U;
  return lit;
}

typedef struct Eurydice_borrow_slice_u8_x4_s {
  Eurydice_borrow_slice_u8 fst;
  Eurydice_borrow_slice_u8 snd;
  Eurydice_borrow_slice_u8 thd;
  Eurydice_borrow_slice_u8 f3;
} Eurydice_borrow_slice_u8_x4;

typedef struct Eurydice_borrow_slice_u8_x2_s {
  Eurydice_borrow_slice_u8 fst;
  Eurydice_borrow_slice_u8 snd;
} Eurydice_borrow_slice_u8_x2;

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
libcrux_ml_kem_types_unpack_private_key_64(
    Eurydice_borrow_slice_u8 private_key) {
  Eurydice_borrow_slice_u8_x2 uu____0 = Eurydice_slice_split_at(
      private_key, (size_t)1152U, uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 secret_key0 = uu____0.snd;
  Eurydice_borrow_slice_u8_x2 uu____1 = Eurydice_slice_split_at(
      secret_key0, (size_t)1184U, uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____1.fst;
  Eurydice_borrow_slice_u8 secret_key = uu____1.snd;
  Eurydice_borrow_slice_u8_x2 uu____2 = Eurydice_slice_split_at(
      secret_key, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____2.snd;
  return (Eurydice_borrow_slice_u8_x4{ind_cpa_secret_key, ind_cpa_public_key,
                                      ind_cpa_public_key_hash,
                                      implicit_rejection_value});
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$24size_t]]

*/
static KRML_MUSTINLINE Eurydice_arr_94
libcrux_secrets_int_public_integers_declassify_d8_40(Eurydice_arr_94 self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$20size_t]]

*/
static KRML_MUSTINLINE Eurydice_arr_fc
libcrux_secrets_int_public_integers_declassify_d8_2b(Eurydice_arr_fc self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$8size_t]]

*/
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_secrets_int_public_integers_declassify_d8_52(Eurydice_array_u8x8 self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$2size_t]]

*/
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_secrets_int_public_integers_declassify_d8_75(Eurydice_array_u8x2 self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types Eurydice_arr int16_t[[$16size_t]]

*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_secrets_int_public_integers_classify_27_4b(Eurydice_arr_d6 self) {
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for
&'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types uint8_t

*/
static KRML_MUSTINLINE Eurydice_borrow_slice_u8
libcrux_secrets_int_classify_public_classify_ref_6d_90(
    Eurydice_borrow_slice_u8 self) {
  return self;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_i16 Eurydice_array_to_subslice_shared_e7(
    const Eurydice_arr_d6 *a, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_i16{a->data + r.start, r.end - r.start});
}

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for
&'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types int16_t

*/
static KRML_MUSTINLINE Eurydice_borrow_slice_i16
libcrux_secrets_int_classify_public_classify_ref_6d_39(
    Eurydice_borrow_slice_i16 self) {
  return self;
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
static inline Eurydice_borrow_slice_i16 Eurydice_slice_subslice_shared_a6(
    Eurydice_borrow_slice_i16 s, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_i16{s.ptr + r.start, r.end - r.start});
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_d6, core_array_TryFromSliceError

*/
typedef struct Result_ec_s {
  Result_57_tags tag;
  union U {
    Eurydice_arr_d6 case_Ok;
    TryFromSliceError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_ec_s)
} Result_ec;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr int16_t[[$16size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_d6 unwrap_26_d3(Result_ec self) {
  if (self.tag == Ok) {
    return self.val.case_Ok;
  } else {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                      "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $128size_t
*/
typedef struct Eurydice_arr_34_s {
  int16_t data[128U];
} Eurydice_arr_34;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_ff
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_0b_s {
  Eurydice_arr_ff data[1U];
} Eurydice_arr_0b;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_88_s {
  Eurydice_arr_c5 data[1U];
} Eurydice_arr_88;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 64
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_17(
    Eurydice_arr_c7 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)64U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $48size_t
*/
typedef struct Eurydice_arr_65_s {
  uint8_t data[48U];
} Eurydice_arr_65;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 48
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f(
    Eurydice_arr_65 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)48U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 32
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_01(
    Eurydice_arr_ec *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)32U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $28size_t
*/
typedef struct Eurydice_arr_a2_s {
  uint8_t data[28U];
} Eurydice_arr_a2;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 28
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_5e(
    Eurydice_arr_a2 *a) {
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)28U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $104size_t
*/
typedef struct Eurydice_arr_c4_s {
  uint8_t data[104U];
} Eurydice_arr_c4;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 104
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_72(
    const Eurydice_arr_c4 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)104U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 104
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d43(
    Eurydice_arr_c4 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $144size_t
*/
typedef struct Eurydice_arr_f4_s {
  uint8_t data[144U];
} Eurydice_arr_f4;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 144
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_38(
    const Eurydice_arr_f4 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)144U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 144
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d42(
    Eurydice_arr_f4 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 168
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_2c(
    const Eurydice_arr_c5 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)168U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d41(
    Eurydice_arr_c5 *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 136
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_58(
    const Eurydice_arr_ff *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)136U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d40(
    Eurydice_arr_ff *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice
uint8_t with const generics
- N= 8
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_subslice_to_shared_21(
    const Eurydice_array_u8x8 *a, size_t r) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 8
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_6e(
    const Eurydice_array_u8x8 *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_slice_subslice_mut_c8(
    Eurydice_mut_borrow_slice_u8 s, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{s.ptr + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_84_s {
  uint64_t data[5U];
} Eurydice_arr_84;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_borrow_slice_u8
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_dc_s {
  Eurydice_borrow_slice_u8 data[1U];
} Eurydice_arr_dc;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $72size_t
*/
typedef struct Eurydice_arr_ab_s {
  uint8_t data[72U];
} Eurydice_arr_ab;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 72
*/
static inline Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e2(
    const Eurydice_arr_ab *a) {
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)72U;
  return lit;
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_array_u8x8, core_array_TryFromSliceError

*/
typedef struct Result_8e_s {
  Result_57_tags tag;
  union U {
    Eurydice_array_u8x8 case_Ok;
    TryFromSliceError case_Err;
  } val;
  KRML_UNION_CONSTRUCTOR(Result_8e_s)
} Result_8e;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$8size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_array_u8x8 unwrap_26_e0(Result_8e self) {
  if (self.tag == Ok) {
    return self.val.case_Ok;
  } else {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                      "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_borrow_slice_u8 Eurydice_slice_subslice_shared_c8(
    Eurydice_borrow_slice_u8 s, core_ops_range_Range_87 r) {
  return (Eurydice_borrow_slice_u8{s.ptr + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 72
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_mut_d4(
    Eurydice_arr_ab *a, core_ops_range_Range_87 r) {
  return (Eurydice_mut_borrow_slice_u8{a->data + r.start, r.end - r.start});
}

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $25size_t
*/
typedef struct Eurydice_arr_7c_s {
  uint64_t data[25U];
} Eurydice_arr_7c;

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_22_s {
  uint64_t data[24U];
} Eurydice_arr_22;

#define libcrux_mlkem_core_H_DEFINED
#endif /* libcrux_mlkem_core_H */
