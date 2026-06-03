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

#ifndef libcrux_mlkem768_portable_H
#define libcrux_mlkem768_portable_H

#include "eurydice_glue.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_ct_ops.h"
#include "libcrux_mlkem_core.h"
#include "libcrux_sha3_portable.h"

static inline Eurydice_arr_c7 libcrux_ml_kem_hash_functions_portable_G(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_c7 digest = {.data = {0U}};
  libcrux_sha3_portable_sha512(Eurydice_array_to_slice_mut_17(&digest), input);
  return digest;
}

static inline Eurydice_arr_ec libcrux_ml_kem_hash_functions_portable_H(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_ec digest = {.data = {0U}};
  libcrux_sha3_portable_sha256(Eurydice_array_to_slice_mut_01(&digest), input);
  return digest;
}

#define LIBCRUX_ML_KEM_POLYNOMIAL_ZETAS_TIMES_MONTGOMERY_R                    \
  ((KRML_CLITERAL(Eurydice_arr_34){                                           \
      .data = {-1044, -758,  -359,  -1517, 1493,  1422,  287,   202,   -171,  \
               622,   1577,  182,   962,   -1202, -1474, 1468,  573,   -1325, \
               264,   383,   -829,  1458,  -1602, -130,  -681,  1017,  732,   \
               608,   -1542, 411,   -205,  -1571, 1223,  652,   -552,  1015,  \
               -1293, 1491,  -282,  -1544, 516,   -8,    -320,  -666,  -1618, \
               -1162, 126,   1469,  -853,  -90,   -271,  830,   107,   -1421, \
               -247,  -951,  -398,  961,   -1508, -725,  448,   -1065, 677,   \
               -1275, -1103, 430,   555,   843,   -1251, 871,   1550,  105,   \
               422,   587,   177,   -235,  -291,  -460,  1574,  1653,  -246,  \
               778,   1159,  -147,  -777,  1483,  -602,  1119,  -1590, 644,   \
               -872,  349,   418,   329,   -156,  -75,   817,   1097,  603,   \
               610,   1322,  -1285, -1465, 384,   -1215, -136,  1218,  -1335, \
               -874,  220,   -1187, -1659, -1185, -1530, -1278, 794,   -1510, \
               -854,  -870,  478,   -108,  -308,  996,   991,   958,   -1460, \
               1522,  1628}}))

static KRML_MUSTINLINE int16_t libcrux_ml_kem_polynomial_zeta(size_t i) {
  return LIBCRUX_ML_KEM_POLYNOMIAL_ZETAS_TIMES_MONTGOMERY_R.data[i];
}

#define LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS \
  (1353)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS (3329)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R \
  (62209U)

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_vector_type_from_i16_array(
    Eurydice_borrow_slice_i16 array) {
  Eurydice_arr_d6 arr;
  memcpy(arr.data,
         Eurydice_slice_subslice_shared_a6(
             array, (KRML_CLITERAL(core_ops_range_Range_87){
                        .start = (size_t)0U, .end = (size_t)16U}))
             .ptr,
         (size_t)16U * sizeof(int16_t));
  return unwrap_26_d3(
      (KRML_CLITERAL(Result_ec){.tag = Ok, .val = {.case_Ok = arr}}));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_from_i16_array_b8(
    Eurydice_borrow_slice_i16 array) {
  return libcrux_ml_kem_vector_portable_vector_type_from_i16_array(
      libcrux_secrets_int_classify_public_classify_ref_6d_39(array));
}

typedef struct int16_t_x8_s {
  int16_t fst;
  int16_t snd;
  int16_t thd;
  int16_t f3;
  int16_t f4;
  int16_t f5;
  int16_t f6;
  int16_t f7;
} int16_t_x8;

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_vector_type_zero(void) {
  return libcrux_secrets_int_public_integers_classify_27_4b(
      (KRML_CLITERAL(Eurydice_arr_d6){.data = {0U}}));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_ZERO_b8(void) {
  return libcrux_ml_kem_vector_portable_vector_type_zero();
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_add(Eurydice_arr_d6 lhs,
                                              const Eurydice_arr_d6 *rhs) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs.data[uu____0] += rhs->data[i0];
  }
  return lhs;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_add_b8(
    Eurydice_arr_d6 lhs, const Eurydice_arr_d6 *rhs) {
  return libcrux_ml_kem_vector_portable_arithmetic_add(lhs, rhs);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_sub(Eurydice_arr_d6 lhs,
                                              const Eurydice_arr_d6 *rhs) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs.data[uu____0] -= rhs->data[i0];
  }
  return lhs;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_sub_b8(
    Eurydice_arr_d6 lhs, const Eurydice_arr_d6 *rhs) {
  return libcrux_ml_kem_vector_portable_arithmetic_sub(lhs, rhs);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_multiply_by_constant(
    Eurydice_arr_d6 vec, int16_t c) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    vec.data[uu____0] *= c;
  }
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_multiply_by_constant_b8(Eurydice_arr_d6 vec,
                                                       int16_t c) {
  return libcrux_ml_kem_vector_portable_arithmetic_multiply_by_constant(vec, c);
}

/**
 Note: This function is not secret independent
 Only use with public values.
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_cond_subtract_3329(
    Eurydice_arr_d6 vec) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    if (libcrux_secrets_int_public_integers_declassify_d8_39(vec.data[i0]) >=
        3329) {
      size_t uu____0 = i0;
      vec.data[uu____0] -= 3329;
    }
  }
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_cond_subtract_3329_b8(Eurydice_arr_d6 v) {
  return libcrux_ml_kem_vector_portable_arithmetic_cond_subtract_3329(v);
}

#define LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_BARRETT_MULTIPLIER (20159)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT (26)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_R \
  (                                            \
      (int32_t)((uint32_t)1                    \
                << (uint32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT))

/**
 Signed Barrett Reduction

 Given an input `value`, `barrett_reduce` outputs a representative `result`
 such that:

 - result ≡ value (mod FIELD_MODULUS)
 - the absolute value of `result` is bound as follows:

 `|result| ≤ FIELD_MODULUS / 2 · (|value|/BARRETT_R + 1)

 Note: The input bound is 28296 to prevent overflow in the multiplication of
 quotient by FIELD_MODULUS

*/
static inline int16_t
libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce_element(
    int16_t value) {
  int32_t t = libcrux_secrets_int_as_i32_f5(value) *
                  LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_BARRETT_MULTIPLIER +
              (LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_R >> 1U);
  int16_t quotient = libcrux_secrets_int_as_i16_36(
      t >> (uint32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT);
  return value - quotient * LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS;
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce(Eurydice_arr_d6 vec) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int16_t vi =
        libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce_element(
            vec.data[i0]);
    vec.data[i0] = vi;
  }
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_barrett_reduce_b8(
    Eurydice_arr_d6 vector) {
  return libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce(vector);
}

#define LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT (16U)

/**
 Signed Montgomery Reduction

 Given an input `value`, `montgomery_reduce` outputs a representative `o`
 such that:

 - o ≡ value · MONTGOMERY_R^(-1) (mod FIELD_MODULUS)
 - the absolute value of `o` is bound as follows:

 `|result| ≤ ceil(|value| / MONTGOMERY_R) + 1665

 In particular, if `|value| ≤ FIELD_MODULUS-1 * FIELD_MODULUS-1`, then `|o| <=
 FIELD_MODULUS-1`. And, if `|value| ≤ pow2 16 * FIELD_MODULUS-1`, then `|o| <=
 FIELD_MODULUS + 1664

*/
static inline int16_t
libcrux_ml_kem_vector_portable_arithmetic_montgomery_reduce_element(
    int32_t value) {
  int32_t k =
      libcrux_secrets_int_as_i32_f5(libcrux_secrets_int_as_i16_36(value)) *
      libcrux_secrets_int_as_i32_b8(
          libcrux_secrets_int_public_integers_classify_27_df(
              LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R));
  int32_t k_times_modulus =
      libcrux_secrets_int_as_i32_f5(libcrux_secrets_int_as_i16_36(k)) *
      libcrux_secrets_int_as_i32_f5(
          libcrux_secrets_int_public_integers_classify_27_39(
              LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
  int16_t c = libcrux_secrets_int_as_i16_36(
      k_times_modulus >>
      (uint32_t)LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT);
  int16_t value_high = libcrux_secrets_int_as_i16_36(
      value >>
      (uint32_t)LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_MONTGOMERY_SHIFT);
  return value_high - c;
}

/**
 If `fe` is some field element 'x' of the Kyber field and `fer` is congruent to
 `y · MONTGOMERY_R`, this procedure outputs a value that is congruent to
 `x · y`, as follows:

    `fe · fer ≡ x · y · MONTGOMERY_R (mod FIELD_MODULUS)`

 `montgomery_reduce` takes the value `x · y · MONTGOMERY_R` and outputs a
 representative `x · y · MONTGOMERY_R * MONTGOMERY_R^{-1} ≡ x · y (mod
 FIELD_MODULUS)`.
*/
static KRML_MUSTINLINE int16_t
libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_fe_by_fer(
    int16_t fe, int16_t fer) {
  int32_t product =
      libcrux_secrets_int_as_i32_f5(fe) * libcrux_secrets_int_as_i32_f5(fer);
  return libcrux_ml_kem_vector_portable_arithmetic_montgomery_reduce_element(
      product);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_by_constant(
    Eurydice_arr_d6 vec, int16_t c) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    vec.data[i0] =
        libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_fe_by_fer(
            vec.data[i0], c);
  }
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
    Eurydice_arr_d6 vector, int16_t constant) {
  return libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_by_constant(
      vector, libcrux_secrets_int_public_integers_classify_27_39(constant));
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(
    Eurydice_arr_d6 vec, int16_t c) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    vec.data[uu____0] &= c;
  }
  return vec;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.portable.arithmetic.shift_right
with const generics
- SHIFT_BY= 15
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_shift_right_ef(Eurydice_arr_d6 vec) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    vec.data[i0] >>= (uint32_t)15;
  }
  return vec;
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_arithmetic_to_unsigned_representative(
    Eurydice_arr_d6 a) {
  Eurydice_arr_d6 t =
      libcrux_ml_kem_vector_portable_arithmetic_shift_right_ef(a);
  Eurydice_arr_d6 fm =
      libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(
          t, LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  return libcrux_ml_kem_vector_portable_arithmetic_add(a, &fm);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_to_unsigned_representative_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_arithmetic_to_unsigned_representative(
      a);
}

/**
 The `compress_*` functions implement the `Compress` function specified in the
 NIST FIPS 203 standard (Page 18, Expression 4.5), which is defined as:

 ```plaintext
 Compress_d: ℤq -> ℤ_{2ᵈ}
 Compress_d(x) = ⌈(2ᵈ/q)·x⌋
 ```

 Since `⌈x⌋ = ⌊x + 1/2⌋` we have:

 ```plaintext
 Compress_d(x) = ⌊(2ᵈ/q)·x + 1/2⌋
               = ⌊(2^{d+1}·x + q) / 2q⌋
 ```

 For further information about the function implementations, consult the
 `implementation_notes.pdf` document in this directory.

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
static inline uint8_t
libcrux_ml_kem_vector_portable_compress_compress_message_coefficient(
    uint16_t fe) {
  int16_t shifted = libcrux_secrets_int_public_integers_classify_27_39(1664) -
                    libcrux_secrets_int_as_i16_ca(fe);
  int16_t mask = shifted >> 15U;
  int16_t shifted_to_positive = mask ^ shifted;
  int16_t shifted_positive_in_range = shifted_to_positive - 832;
  int16_t r0 = shifted_positive_in_range >> 15U;
  int16_t r1 = r0 & 1;
  return libcrux_secrets_int_as_u8_f5(r1);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_compress_compress_1(Eurydice_arr_d6 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    a.data[i0] = libcrux_secrets_int_as_i16_59(
        libcrux_ml_kem_vector_portable_compress_compress_message_coefficient(
            libcrux_secrets_int_as_u16_f5(a.data[i0])));
  }
  return a;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_compress_1_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_compress_compress_1(a);
}

static KRML_MUSTINLINE uint32_t
libcrux_ml_kem_vector_portable_arithmetic_get_n_least_significant_bits(
    uint8_t n, uint32_t value) {
  return value & ((1U << (uint32_t)n) - 1U);
}

static inline int16_t
libcrux_ml_kem_vector_portable_compress_compress_ciphertext_coefficient(
    uint8_t coefficient_bits, uint16_t fe) {
  uint64_t compressed = libcrux_secrets_int_as_u64_ca(fe)
                        << (uint32_t)coefficient_bits;
  compressed += 1664ULL;
  compressed *= 10321340ULL;
  compressed >>= 35U;
  return libcrux_secrets_int_as_i16_b8(
      libcrux_ml_kem_vector_portable_arithmetic_get_n_least_significant_bits(
          coefficient_bits, libcrux_secrets_int_as_u32_a3(compressed)));
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_compress_decompress_1(Eurydice_arr_d6 a) {
  Eurydice_arr_d6 z = libcrux_ml_kem_vector_portable_vector_type_zero();
  Eurydice_arr_d6 s = libcrux_ml_kem_vector_portable_arithmetic_sub(z, &a);
  Eurydice_arr_d6 res =
      libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(s,
                                                                          1665);
  return res;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_decompress_1_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_compress_decompress_1(a);
}

static KRML_MUSTINLINE void libcrux_ml_kem_vector_portable_ntt_ntt_step(
    Eurydice_arr_d6 *vec, int16_t zeta, size_t i, size_t j) {
  int16_t t =
      libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_fe_by_fer(
          vec->data[j],
          libcrux_secrets_int_public_integers_classify_27_39(zeta));
  int16_t a_minus_t = vec->data[i] - t;
  int16_t a_plus_t = vec->data[i] + t;
  vec->data[j] = a_minus_t;
  vec->data[i] = a_plus_t;
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_layer_1_step(Eurydice_arr_d6 vec,
                                                    int16_t zeta0,
                                                    int16_t zeta1,
                                                    int16_t zeta2,
                                                    int16_t zeta3) {
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta0, (size_t)0U,
                                              (size_t)2U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta0, (size_t)1U,
                                              (size_t)3U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta1, (size_t)4U,
                                              (size_t)6U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta1, (size_t)5U,
                                              (size_t)7U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta2, (size_t)8U,
                                              (size_t)10U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta2, (size_t)9U,
                                              (size_t)11U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta3, (size_t)12U,
                                              (size_t)14U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta3, (size_t)13U,
                                              (size_t)15U);
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_layer_1_step_b8(Eurydice_arr_d6 a,
                                                   int16_t zeta0, int16_t zeta1,
                                                   int16_t zeta2,
                                                   int16_t zeta3) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_layer_1_step(a, zeta0, zeta1,
                                                             zeta2, zeta3);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_layer_2_step(Eurydice_arr_d6 vec,
                                                    int16_t zeta0,
                                                    int16_t zeta1) {
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta0, (size_t)0U,
                                              (size_t)4U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta0, (size_t)1U,
                                              (size_t)5U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta0, (size_t)2U,
                                              (size_t)6U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta0, (size_t)3U,
                                              (size_t)7U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta1, (size_t)8U,
                                              (size_t)12U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta1, (size_t)9U,
                                              (size_t)13U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta1, (size_t)10U,
                                              (size_t)14U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta1, (size_t)11U,
                                              (size_t)15U);
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_layer_2_step_b8(Eurydice_arr_d6 a,
                                                   int16_t zeta0,
                                                   int16_t zeta1) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_layer_2_step(a, zeta0, zeta1);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_layer_3_step(Eurydice_arr_d6 vec,
                                                    int16_t zeta) {
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)0U,
                                              (size_t)8U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)1U,
                                              (size_t)9U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)2U,
                                              (size_t)10U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)3U,
                                              (size_t)11U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)4U,
                                              (size_t)12U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)5U,
                                              (size_t)13U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)6U,
                                              (size_t)14U);
  libcrux_ml_kem_vector_portable_ntt_ntt_step(&vec, zeta, (size_t)7U,
                                              (size_t)15U);
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_layer_3_step_b8(Eurydice_arr_d6 a,
                                                   int16_t zeta) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_layer_3_step(a, zeta);
}

static KRML_MUSTINLINE void libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(
    Eurydice_arr_d6 *vec, int16_t zeta, size_t i, size_t j) {
  int16_t a_minus_b = vec->data[j] - vec->data[i];
  int16_t a_plus_b = vec->data[j] + vec->data[i];
  int16_t o0 = libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce_element(
      a_plus_b);
  int16_t o1 =
      libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_fe_by_fer(
          a_minus_b, libcrux_secrets_int_public_integers_classify_27_39(zeta));
  vec->data[i] = o0;
  vec->data[j] = o1;
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_1_step(Eurydice_arr_d6 vec,
                                                        int16_t zeta0,
                                                        int16_t zeta1,
                                                        int16_t zeta2,
                                                        int16_t zeta3) {
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta0, (size_t)0U,
                                                  (size_t)2U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta0, (size_t)1U,
                                                  (size_t)3U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta1, (size_t)4U,
                                                  (size_t)6U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta1, (size_t)5U,
                                                  (size_t)7U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta2, (size_t)8U,
                                                  (size_t)10U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta2, (size_t)9U,
                                                  (size_t)11U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta3, (size_t)12U,
                                                  (size_t)14U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta3, (size_t)13U,
                                                  (size_t)15U);
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_inv_ntt_layer_1_step_b8(Eurydice_arr_d6 a,
                                                       int16_t zeta0,
                                                       int16_t zeta1,
                                                       int16_t zeta2,
                                                       int16_t zeta3) {
  return libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_1_step(
      a, zeta0, zeta1, zeta2, zeta3);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_2_step(Eurydice_arr_d6 vec,
                                                        int16_t zeta0,
                                                        int16_t zeta1) {
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta0, (size_t)0U,
                                                  (size_t)4U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta0, (size_t)1U,
                                                  (size_t)5U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta0, (size_t)2U,
                                                  (size_t)6U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta0, (size_t)3U,
                                                  (size_t)7U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta1, (size_t)8U,
                                                  (size_t)12U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta1, (size_t)9U,
                                                  (size_t)13U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta1, (size_t)10U,
                                                  (size_t)14U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta1, (size_t)11U,
                                                  (size_t)15U);
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_inv_ntt_layer_2_step_b8(Eurydice_arr_d6 a,
                                                       int16_t zeta0,
                                                       int16_t zeta1) {
  return libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_2_step(a, zeta0,
                                                                 zeta1);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_3_step(Eurydice_arr_d6 vec,
                                                        int16_t zeta) {
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)0U,
                                                  (size_t)8U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)1U,
                                                  (size_t)9U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)2U,
                                                  (size_t)10U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)3U,
                                                  (size_t)11U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)4U,
                                                  (size_t)12U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)5U,
                                                  (size_t)13U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)6U,
                                                  (size_t)14U);
  libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(&vec, zeta, (size_t)7U,
                                                  (size_t)15U);
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_inv_ntt_layer_3_step_b8(Eurydice_arr_d6 a,
                                                       int16_t zeta) {
  return libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_3_step(a, zeta);
}

/**
 Compute the product of two Kyber binomials with respect to the
 modulus `X² - zeta`.

 This function almost implements <strong>Algorithm 11</strong> of the
 NIST FIPS 203 standard, which is reproduced below:

 ```plaintext
 Input:  a₀, a₁, b₀, b₁ ∈ ℤq.
 Input: γ ∈ ℤq.
 Output: c₀, c₁ ∈ ℤq.

 c₀ ← a₀·b₀ + a₁·b₁·γ
 c₁ ← a₀·b₁ + a₁·b₀
 return c₀, c₁
 ```
 We say "almost" because the coefficients output by this function are in
 the Montgomery domain (unlike in the specification).

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
    const Eurydice_arr_d6 *a, const Eurydice_arr_d6 *b, int16_t zeta, size_t i,
    Eurydice_arr_d6 *out) {
  int16_t ai = a->data[(size_t)2U * i];
  int16_t bi = b->data[(size_t)2U * i];
  int16_t aj = a->data[(size_t)2U * i + (size_t)1U];
  int16_t bj = b->data[(size_t)2U * i + (size_t)1U];
  int32_t ai_bi =
      libcrux_secrets_int_as_i32_f5(ai) * libcrux_secrets_int_as_i32_f5(bi);
  int32_t aj_bj_ =
      libcrux_secrets_int_as_i32_f5(aj) * libcrux_secrets_int_as_i32_f5(bj);
  int16_t aj_bj =
      libcrux_ml_kem_vector_portable_arithmetic_montgomery_reduce_element(
          aj_bj_);
  int32_t aj_bj_zeta = libcrux_secrets_int_as_i32_f5(aj_bj) *
                       libcrux_secrets_int_as_i32_f5(zeta);
  int32_t ai_bi_aj_bj = ai_bi + aj_bj_zeta;
  int16_t o0 =
      libcrux_ml_kem_vector_portable_arithmetic_montgomery_reduce_element(
          ai_bi_aj_bj);
  int32_t ai_bj =
      libcrux_secrets_int_as_i32_f5(ai) * libcrux_secrets_int_as_i32_f5(bj);
  int32_t aj_bi =
      libcrux_secrets_int_as_i32_f5(aj) * libcrux_secrets_int_as_i32_f5(bi);
  int32_t ai_bj_aj_bi = ai_bj + aj_bi;
  int16_t o1 =
      libcrux_ml_kem_vector_portable_arithmetic_montgomery_reduce_element(
          ai_bj_aj_bi);
  out->data[(size_t)2U * i] = o0;
  out->data[(size_t)2U * i + (size_t)1U] = o1;
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_ntt_ntt_multiply(const Eurydice_arr_d6 *lhs,
                                                const Eurydice_arr_d6 *rhs,
                                                int16_t zeta0, int16_t zeta1,
                                                int16_t zeta2, int16_t zeta3) {
  int16_t nzeta0 = -zeta0;
  int16_t nzeta1 = -zeta1;
  int16_t nzeta2 = -zeta2;
  int16_t nzeta3 = -zeta3;
  Eurydice_arr_d6 out = libcrux_ml_kem_vector_portable_vector_type_zero();
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(zeta0),
      (size_t)0U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(nzeta0),
      (size_t)1U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(zeta1),
      (size_t)2U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(nzeta1),
      (size_t)3U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(zeta2),
      (size_t)4U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(nzeta2),
      (size_t)5U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(zeta3),
      (size_t)6U, &out);
  libcrux_ml_kem_vector_portable_ntt_ntt_multiply_binomials(
      lhs, rhs, libcrux_secrets_int_public_integers_classify_27_39(nzeta3),
      (size_t)7U, &out);
  return out;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_ntt_multiply_b8(
    const Eurydice_arr_d6 *lhs, const Eurydice_arr_d6 *rhs, int16_t zeta0,
    int16_t zeta1, int16_t zeta2, int16_t zeta3) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_multiply(lhs, rhs, zeta0, zeta1,
                                                         zeta2, zeta3);
}

static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_portable_serialize_serialize_1(Eurydice_arr_d6 v) {
  uint8_t result0 =
      (((((((uint32_t)libcrux_secrets_int_as_u8_f5(v.data[0U]) |
            (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[1U]) << 1U) |
           (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[2U]) << 2U) |
          (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[3U]) << 3U) |
         (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[4U]) << 4U) |
        (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[5U]) << 5U) |
       (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[6U]) << 6U) |
      (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[7U]) << 7U;
  uint8_t result1 =
      (((((((uint32_t)libcrux_secrets_int_as_u8_f5(v.data[8U]) |
            (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[9U]) << 1U) |
           (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[10U]) << 2U) |
          (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[11U]) << 3U) |
         (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[12U]) << 4U) |
        (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[13U]) << 5U) |
       (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[14U]) << 6U) |
      (uint32_t)libcrux_secrets_int_as_u8_f5(v.data[15U]) << 7U;
  return (KRML_CLITERAL(Eurydice_array_u8x2){.data = {result0, result1}});
}

static inline Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_1(
    Eurydice_arr_d6 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_75(
      libcrux_ml_kem_vector_portable_serialize_serialize_1(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_1_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_serialize_1(a);
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_1(
    Eurydice_borrow_slice_u8 v) {
  int16_t result0 = libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] & 1U);
  int16_t result1 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 1U & 1U);
  int16_t result2 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 2U & 1U);
  int16_t result3 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 3U & 1U);
  int16_t result4 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 4U & 1U);
  int16_t result5 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 5U & 1U);
  int16_t result6 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 6U & 1U);
  int16_t result7 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[0U] >> 7U & 1U);
  int16_t result8 = libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] & 1U);
  int16_t result9 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 1U & 1U);
  int16_t result10 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 2U & 1U);
  int16_t result11 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 3U & 1U);
  int16_t result12 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 4U & 1U);
  int16_t result13 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 5U & 1U);
  int16_t result14 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 6U & 1U);
  int16_t result15 =
      libcrux_secrets_int_as_i16_59((uint32_t)v.ptr[1U] >> 7U & 1U);
  return (KRML_CLITERAL(Eurydice_arr_d6){
      .data = {result0, result1, result2, result3, result4, result5, result6,
               result7, result8, result9, result10, result11, result12,
               result13, result14, result15}});
}

static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_1(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_1(
      libcrux_secrets_int_classify_public_classify_ref_6d_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_1_b8(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_deserialize_1(a);
}

typedef struct uint8_t_x4_s {
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
  uint8_t f3;
} uint8_t_x4;

static KRML_MUSTINLINE uint8_t_x4
libcrux_ml_kem_vector_portable_serialize_serialize_4_int(
    Eurydice_borrow_slice_i16 v) {
  uint8_t result0 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[1U]) << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[0U]);
  uint8_t result1 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[3U]) << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[2U]);
  uint8_t result2 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[5U]) << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[4U]);
  uint8_t result3 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[7U]) << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[6U]);
  return (KRML_CLITERAL(uint8_t_x4){
      .fst = result0, .snd = result1, .thd = result2, .f3 = result3});
}

static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_portable_serialize_serialize_4(Eurydice_arr_d6 v) {
  uint8_t_x4 result0_3 =
      libcrux_ml_kem_vector_portable_serialize_serialize_4_int(
          Eurydice_array_to_subslice_shared_e7(
              &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                          .end = (size_t)8U})));
  uint8_t_x4 result4_7 =
      libcrux_ml_kem_vector_portable_serialize_serialize_4_int(
          Eurydice_array_to_subslice_shared_e7(
              &v, (KRML_CLITERAL(core_ops_range_Range_87){
                      .start = (size_t)8U, .end = (size_t)16U})));
  return (KRML_CLITERAL(Eurydice_array_u8x8){
      .data = {result0_3.fst, result0_3.snd, result0_3.thd, result0_3.f3,
               result4_7.fst, result4_7.snd, result4_7.thd, result4_7.f3}});
}

static inline Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_4(
    Eurydice_arr_d6 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_52(
      libcrux_ml_kem_vector_portable_serialize_serialize_4(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_4_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_serialize_4(a);
}

static KRML_MUSTINLINE int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t v0 = libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[0U] & 15U);
  int16_t v1 =
      libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[0U] >> 4U & 15U);
  int16_t v2 = libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[1U] & 15U);
  int16_t v3 =
      libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[1U] >> 4U & 15U);
  int16_t v4 = libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[2U] & 15U);
  int16_t v5 =
      libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[2U] >> 4U & 15U);
  int16_t v6 = libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[3U] & 15U);
  int16_t v7 =
      libcrux_secrets_int_as_i16_59((uint32_t)bytes.ptr[3U] >> 4U & 15U);
  return (KRML_CLITERAL(int16_t_x8){.fst = v0,
                                    .snd = v1,
                                    .thd = v2,
                                    .f3 = v3,
                                    .f4 = v4,
                                    .f5 = v5,
                                    .f6 = v6,
                                    .f7 = v7});
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_4(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t_x8 v0_7 = libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                         .end = (size_t)4U})));
  int16_t_x8 v8_15 = libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)4U,
                                                         .end = (size_t)8U})));
  return (KRML_CLITERAL(Eurydice_arr_d6){
      .data = {v0_7.fst, v0_7.snd, v0_7.thd, v0_7.f3, v0_7.f4, v0_7.f5, v0_7.f6,
               v0_7.f7, v8_15.fst, v8_15.snd, v8_15.thd, v8_15.f3, v8_15.f4,
               v8_15.f5, v8_15.f6, v8_15.f7}});
}

static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_4(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_4(
      libcrux_secrets_int_classify_public_classify_ref_6d_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_4_b8(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_deserialize_4(a);
}

typedef struct uint8_t_x5_s {
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
  uint8_t f3;
  uint8_t f4;
} uint8_t_x5;

static KRML_MUSTINLINE uint8_t_x5
libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
    Eurydice_borrow_slice_i16 v) {
  uint8_t r0 = libcrux_secrets_int_as_u8_f5(v.ptr[0U] & 255);
  uint8_t r1 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[1U] & 63) << 2U |
               (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[0U] >> 8U & 3);
  uint8_t r2 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[2U] & 15) << 4U |
               (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[1U] >> 6U & 15);
  uint8_t r3 = (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[3U] & 3) << 6U |
               (uint32_t)libcrux_secrets_int_as_u8_f5(v.ptr[2U] >> 4U & 63);
  uint8_t r4 = libcrux_secrets_int_as_u8_f5(v.ptr[3U] >> 2U & 255);
  return (KRML_CLITERAL(uint8_t_x5){
      .fst = r0, .snd = r1, .thd = r2, .f3 = r3, .f4 = r4});
}

static KRML_MUSTINLINE Eurydice_arr_fc
libcrux_ml_kem_vector_portable_serialize_serialize_10(Eurydice_arr_d6 v) {
  uint8_t_x5 r0_4 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                      .end = (size_t)4U})));
  uint8_t_x5 r5_9 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)4U,
                                                      .end = (size_t)8U})));
  uint8_t_x5 r10_14 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)8U,
                                                      .end = (size_t)12U})));
  uint8_t_x5 r15_19 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)12U,
                                                      .end = (size_t)16U})));
  return (KRML_CLITERAL(Eurydice_arr_fc){
      .data = {r0_4.fst,   r0_4.snd,   r0_4.thd,   r0_4.f3,   r0_4.f4,
               r5_9.fst,   r5_9.snd,   r5_9.thd,   r5_9.f3,   r5_9.f4,
               r10_14.fst, r10_14.snd, r10_14.thd, r10_14.f3, r10_14.f4,
               r15_19.fst, r15_19.snd, r15_19.thd, r15_19.f3, r15_19.f4}});
}

static inline Eurydice_arr_fc libcrux_ml_kem_vector_portable_serialize_10(
    Eurydice_arr_d6 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_2b(
      libcrux_ml_kem_vector_portable_serialize_serialize_10(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_fc libcrux_ml_kem_vector_portable_serialize_10_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_serialize_10(a);
}

static KRML_MUSTINLINE int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t r0 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)(libcrux_secrets_int_as_i16_59(bytes.ptr[1U]) & 3)
                << 8U) |
      (libcrux_secrets_int_as_i16_59(bytes.ptr[0U]) & 255));
  int16_t r1 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)(libcrux_secrets_int_as_i16_59(bytes.ptr[2U]) & 15)
                << 6U) |
      libcrux_secrets_int_as_i16_59(bytes.ptr[1U]) >> 2U);
  int16_t r2 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)(libcrux_secrets_int_as_i16_59(bytes.ptr[3U]) & 63)
                << 4U) |
      libcrux_secrets_int_as_i16_59(bytes.ptr[2U]) >> 4U);
  int16_t r3 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)libcrux_secrets_int_as_i16_59(bytes.ptr[4U]) << 2U) |
      libcrux_secrets_int_as_i16_59(bytes.ptr[3U]) >> 6U);
  int16_t r4 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)(libcrux_secrets_int_as_i16_59(bytes.ptr[6U]) & 3)
                << 8U) |
      (libcrux_secrets_int_as_i16_59(bytes.ptr[5U]) & 255));
  int16_t r5 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)(libcrux_secrets_int_as_i16_59(bytes.ptr[7U]) & 15)
                << 6U) |
      libcrux_secrets_int_as_i16_59(bytes.ptr[6U]) >> 2U);
  int16_t r6 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)(libcrux_secrets_int_as_i16_59(bytes.ptr[8U]) & 63)
                << 4U) |
      libcrux_secrets_int_as_i16_59(bytes.ptr[7U]) >> 4U);
  int16_t r7 = libcrux_secrets_int_as_i16_f5(
      (int16_t)((uint32_t)libcrux_secrets_int_as_i16_59(bytes.ptr[9U]) << 2U) |
      libcrux_secrets_int_as_i16_59(bytes.ptr[8U]) >> 6U);
  return (KRML_CLITERAL(int16_t_x8){.fst = r0,
                                    .snd = r1,
                                    .thd = r2,
                                    .f3 = r3,
                                    .f4 = r4,
                                    .f5 = r5,
                                    .f6 = r6,
                                    .f7 = r7});
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_10(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t_x8 v0_7 = libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                         .end = (size_t)10U})));
  int16_t_x8 v8_15 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(
          Eurydice_slice_subslice_shared_c8(
              bytes, (KRML_CLITERAL(core_ops_range_Range_87){
                         .start = (size_t)10U, .end = (size_t)20U})));
  return (KRML_CLITERAL(Eurydice_arr_d6){
      .data = {v0_7.fst, v0_7.snd, v0_7.thd, v0_7.f3, v0_7.f4, v0_7.f5, v0_7.f6,
               v0_7.f7, v8_15.fst, v8_15.snd, v8_15.thd, v8_15.f3, v8_15.f4,
               v8_15.f5, v8_15.f6, v8_15.f7}});
}

static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_10(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_10(
      libcrux_secrets_int_classify_public_classify_ref_6d_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_10_b8(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_deserialize_10(a);
}

typedef struct uint8_t_x3_s {
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
} uint8_t_x3;

static KRML_MUSTINLINE uint8_t_x3
libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
    Eurydice_borrow_slice_i16 v) {
  uint8_t r0 = libcrux_secrets_int_as_u8_f5(v.ptr[0U] & 255);
  uint8_t r1 = libcrux_secrets_int_as_u8_f5(
      v.ptr[0U] >> 8U | (int16_t)((uint32_t)(v.ptr[1U] & 15) << 4U));
  uint8_t r2 = libcrux_secrets_int_as_u8_f5(v.ptr[1U] >> 4U & 255);
  return (KRML_CLITERAL(uint8_t_x3){.fst = r0, .snd = r1, .thd = r2});
}

static KRML_MUSTINLINE Eurydice_arr_94
libcrux_ml_kem_vector_portable_serialize_serialize_12(Eurydice_arr_d6 v) {
  uint8_t_x3 r0_2 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                      .end = (size_t)2U})));
  uint8_t_x3 r3_5 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)2U,
                                                      .end = (size_t)4U})));
  uint8_t_x3 r6_8 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)4U,
                                                      .end = (size_t)6U})));
  uint8_t_x3 r9_11 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)6U,
                                                      .end = (size_t)8U})));
  uint8_t_x3 r12_14 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)8U,
                                                      .end = (size_t)10U})));
  uint8_t_x3 r15_17 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)10U,
                                                      .end = (size_t)12U})));
  uint8_t_x3 r18_20 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)12U,
                                                      .end = (size_t)14U})));
  uint8_t_x3 r21_23 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_e7(
          &v, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)14U,
                                                      .end = (size_t)16U})));
  return (KRML_CLITERAL(Eurydice_arr_94){
      .data = {r0_2.fst,   r0_2.snd,   r0_2.thd,   r3_5.fst,   r3_5.snd,
               r3_5.thd,   r6_8.fst,   r6_8.snd,   r6_8.thd,   r9_11.fst,
               r9_11.snd,  r9_11.thd,  r12_14.fst, r12_14.snd, r12_14.thd,
               r15_17.fst, r15_17.snd, r15_17.thd, r18_20.fst, r18_20.snd,
               r18_20.thd, r21_23.fst, r21_23.snd, r21_23.thd}});
}

static inline Eurydice_arr_94 libcrux_ml_kem_vector_portable_serialize_12(
    Eurydice_arr_d6 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_40(
      libcrux_ml_kem_vector_portable_serialize_serialize_12(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_94 libcrux_ml_kem_vector_portable_serialize_12_b8(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_serialize_12(a);
}

typedef struct int16_t_x2_s {
  int16_t fst;
  int16_t snd;
} int16_t_x2;

static KRML_MUSTINLINE int16_t_x2
libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t byte0 = libcrux_secrets_int_as_i16_59(bytes.ptr[0U]);
  int16_t byte1 = libcrux_secrets_int_as_i16_59(bytes.ptr[1U]);
  int16_t byte2 = libcrux_secrets_int_as_i16_59(bytes.ptr[2U]);
  int16_t r0 = (int16_t)((uint32_t)(byte1 & 15) << 8U) | (byte0 & 255);
  int16_t r1 = (int16_t)((uint32_t)byte2 << 4U) | (byte1 >> 4U & 15);
  return (KRML_CLITERAL(int16_t_x2){.fst = r0, .snd = r1});
}

static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_serialize_deserialize_12(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t_x2 v0_1 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                         .end = (size_t)3U})));
  int16_t_x2 v2_3 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)3U,
                                                         .end = (size_t)6U})));
  int16_t_x2 v4_5 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)6U,
                                                         .end = (size_t)9U})));
  int16_t_x2 v6_7 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)9U,
                                                         .end = (size_t)12U})));
  int16_t_x2 v8_9 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_c8(
          bytes, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)12U,
                                                         .end = (size_t)15U})));
  int16_t_x2 v10_11 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
          Eurydice_slice_subslice_shared_c8(
              bytes, (KRML_CLITERAL(core_ops_range_Range_87){
                         .start = (size_t)15U, .end = (size_t)18U})));
  int16_t_x2 v12_13 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
          Eurydice_slice_subslice_shared_c8(
              bytes, (KRML_CLITERAL(core_ops_range_Range_87){
                         .start = (size_t)18U, .end = (size_t)21U})));
  int16_t_x2 v14_15 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
          Eurydice_slice_subslice_shared_c8(
              bytes, (KRML_CLITERAL(core_ops_range_Range_87){
                         .start = (size_t)21U, .end = (size_t)24U})));
  return (KRML_CLITERAL(Eurydice_arr_d6){
      .data = {v0_1.fst, v0_1.snd, v2_3.fst, v2_3.snd, v4_5.fst, v4_5.snd,
               v6_7.fst, v6_7.snd, v8_9.fst, v8_9.snd, v10_11.fst, v10_11.snd,
               v12_13.fst, v12_13.snd, v14_15.fst, v14_15.snd}});
}

static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_12(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_12(
      libcrux_secrets_int_classify_public_classify_ref_6d_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_deserialize_12_b8(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_deserialize_12(a);
}

static KRML_MUSTINLINE size_t
libcrux_ml_kem_vector_portable_sampling_rej_sample(
    Eurydice_borrow_slice_u8 a, Eurydice_mut_borrow_slice_i16 result) {
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < a.meta / (size_t)3U; i++) {
    size_t i0 = i;
    int16_t b1 = (int16_t)(uint32_t)a.ptr[i0 * (size_t)3U + (size_t)0U];
    int16_t b2 = (int16_t)(uint32_t)a.ptr[i0 * (size_t)3U + (size_t)1U];
    int16_t b3 = (int16_t)(uint32_t)a.ptr[i0 * (size_t)3U + (size_t)2U];
    int16_t d1 = (int16_t)((uint32_t)(b2 & 15) << 8U) | b1;
    int16_t d2 = (int16_t)((uint32_t)b3 << 4U) | b2 >> 4U;
    if (d1 < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS) {
      if (sampled < (size_t)16U) {
        result.ptr[sampled] = d1;
        sampled++;
      }
    }
    if (d2 < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS) {
      if (sampled < (size_t)16U) {
        result.ptr[sampled] = d2;
        sampled++;
      }
    }
  }
  return sampled;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline size_t libcrux_ml_kem_vector_portable_rej_sample_b8(
    Eurydice_borrow_slice_u8 a, Eurydice_mut_borrow_slice_i16 out) {
  return libcrux_ml_kem_vector_portable_sampling_rej_sample(a, out);
}

#define LIBCRUX_ML_KEM_MLKEM768_VECTOR_U_COMPRESSION_FACTOR ((size_t)10U)

#define LIBCRUX_ML_KEM_MLKEM768_C1_BLOCK_SIZE              \
  (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * \
   LIBCRUX_ML_KEM_MLKEM768_VECTOR_U_COMPRESSION_FACTOR / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_RANK ((size_t)3U)

#define LIBCRUX_ML_KEM_MLKEM768_C1_SIZE \
  (LIBCRUX_ML_KEM_MLKEM768_C1_BLOCK_SIZE * LIBCRUX_ML_KEM_MLKEM768_RANK)

#define LIBCRUX_ML_KEM_MLKEM768_VECTOR_V_COMPRESSION_FACTOR ((size_t)4U)

#define LIBCRUX_ML_KEM_MLKEM768_C2_SIZE                    \
  (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * \
   LIBCRUX_ML_KEM_MLKEM768_VECTOR_V_COMPRESSION_FACTOR / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE \
  (LIBCRUX_ML_KEM_MLKEM768_C1_SIZE + LIBCRUX_ML_KEM_MLKEM768_C2_SIZE)

#define LIBCRUX_ML_KEM_MLKEM768_T_AS_NTT_ENCODED_SIZE      \
  (LIBCRUX_ML_KEM_MLKEM768_RANK *                          \
   LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * \
   LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE \
  (LIBCRUX_ML_KEM_MLKEM768_T_AS_NTT_ENCODED_SIZE + (size_t)32U)

#define LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_SECRET_KEY_SIZE    \
  (LIBCRUX_ML_KEM_MLKEM768_RANK *                          \
   LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * \
   LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA1 ((size_t)2U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA1_RANDOMNESS_SIZE \
  (LIBCRUX_ML_KEM_MLKEM768_ETA1 * (size_t)64U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA2 ((size_t)2U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA2_RANDOMNESS_SIZE \
  (LIBCRUX_ML_KEM_MLKEM768_ETA2 * (size_t)64U)

#define LIBCRUX_ML_KEM_MLKEM768_IMPLICIT_REJECTION_HASH_INPUT_SIZE \
  (LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE +                   \
   LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE)

typedef Eurydice_arr_7d libcrux_ml_kem_mlkem768_MlKem768PrivateKey;

typedef Eurydice_arr_5f libcrux_ml_kem_mlkem768_MlKem768PublicKey;

#define LIBCRUX_ML_KEM_MLKEM768_RANKED_BYTES_PER_RING_ELEMENT \
  (LIBCRUX_ML_KEM_MLKEM768_RANK *                             \
   LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE      \
  (LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_SECRET_KEY_SIZE + \
   LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE + \
   LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE +          \
   LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE)

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_9e_s {
  Eurydice_arr_d6 data[16U];
} Eurydice_arr_9e;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement_1d
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_bb_s {
  Eurydice_arr_9e data[3U];
} Eurydice_arr_bb;

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static inline Eurydice_arr_9e libcrux_ml_kem_polynomial_ZERO_d6_ea(void) {
  Eurydice_arr_9e lit;
  Eurydice_arr_d6 repeat_expression[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    repeat_expression[i] = libcrux_ml_kem_vector_portable_ZERO_b8();
  }
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(Eurydice_arr_d6));
  return lit;
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for libcrux_ml_kem::ind_cpa::decrypt::closure<Vector, K,
CIPHERTEXT_SIZE, VECTOR_U_ENCODED_SIZE, U_COMPRESSION_FACTOR,
V_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt.call_mut_0b
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- VECTOR_U_ENCODED_SIZE= 960
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
static inline Eurydice_arr_9e libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_01(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_to_uncompressed_ring_element with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_9e re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_c8(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_87){
            .start = i0 * (size_t)24U, .end = i0 * (size_t)24U + (size_t)24U}));
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_deserialize_12_b8(bytes);
    re.data[i0] = uu____0;
  }
  return re;
}

/**
 Call [`deserialize_to_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_vector
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_deserialize_vector_68(
    Eurydice_borrow_slice_u8 secret_key, Eurydice_arr_bb *secret_as_ntt) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e uu____0 =
        libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_ea(
            Eurydice_slice_subslice_shared_c8(
                secret_key,
                (KRML_CLITERAL(core_ops_range_Range_87){
                    .start =
                        i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                    .end = (i0 + (size_t)1U) *
                           LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT})));
    secret_as_ntt->data[i0] = uu____0;
  }
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for
libcrux_ml_kem::ind_cpa::deserialize_then_decompress_u::closure<Vector, K,
CIPHERTEXT_SIZE, U_COMPRESSION_FACTOR>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u.call_mut_35 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- U_COMPRESSION_FACTOR= 10
*/
static inline Eurydice_arr_9e
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_30(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.portable.compress.decompress_ciphertext_coefficient with
const generics
- COEFFICIENT_BITS= 10
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_ef(
    Eurydice_arr_d6 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int32_t decompressed =
        libcrux_secrets_int_as_i32_f5(a.data[i0]) *
        libcrux_secrets_int_as_i32_f5(
            libcrux_secrets_int_public_integers_classify_27_39(
                LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
    decompressed = (int32_t)((uint32_t)decompressed << 1U) +
                   (int32_t)((uint32_t)1 << (uint32_t)10);
    decompressed >>= (uint32_t)(10 + 1);
    a.data[i0] = libcrux_secrets_int_as_i16_36(decompressed);
  }
  return a;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
/**
A monomorphic instance of
libcrux_ml_kem.vector.portable.decompress_ciphertext_coefficient_b8 with const
generics
- COEFFICIENT_BITS= 10
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_decompress_ciphertext_coefficient_b8_ef(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_ef(
      a);
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_10 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_then_decompress_10_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_9e re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)20U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_c8(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_87){
            .start = i0 * (size_t)20U, .end = i0 * (size_t)20U + (size_t)20U}));
    Eurydice_arr_d6 coefficient =
        libcrux_ml_kem_vector_portable_deserialize_10_b8(bytes);
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_decompress_ciphertext_coefficient_b8_ef(
            coefficient);
    re.data[i0] = uu____0;
  }
  return re;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_u with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- COMPRESSION_FACTOR= 10
*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_f7(
    Eurydice_borrow_slice_u8 serialized) {
  return libcrux_ml_kem_serialize_deserialize_then_decompress_10_ea(serialized);
}

typedef struct libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2_s {
  Eurydice_arr_d6 fst;
  Eurydice_arr_d6 snd;
} libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2;

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_layer_int_vec_step
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE
    libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2
    libcrux_ml_kem_ntt_ntt_layer_int_vec_step_ea(Eurydice_arr_d6 a,
                                                 Eurydice_arr_d6 b,
                                                 int16_t zeta_r) {
  Eurydice_arr_d6 t =
      libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(b,
                                                                        zeta_r);
  b = libcrux_ml_kem_vector_portable_sub_b8(a, &t);
  a = libcrux_ml_kem_vector_portable_add_b8(a, &t);
  return (KRML_CLITERAL(
      libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2){.fst = a,
                                                                    .snd = b});
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(
    size_t *zeta_i, Eurydice_arr_9e *re, size_t layer,
    size_t _initial_coefficient_bound) {
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++) {
    size_t round = i0;
    zeta_i[0U]++;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec = offset / (size_t)16U;
    size_t step_vec = step / (size_t)16U;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++) {
      size_t j = i;
      libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2 uu____0 =
          libcrux_ml_kem_ntt_ntt_layer_int_vec_step_ea(
              re->data[j], re->data[j + step_vec],
              libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
      Eurydice_arr_d6 x = uu____0.fst;
      Eurydice_arr_d6 y = uu____0.snd;
      re->data[j] = x;
      re->data[j + step_vec] = y;
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_3
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_3_ea(
    size_t *zeta_i, Eurydice_arr_9e *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U]++;
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_ntt_layer_3_step_b8(
            re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
    re->data[round] = uu____0;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_2
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_2_ea(
    size_t *zeta_i, Eurydice_arr_9e *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U]++;
    re->data[round] = libcrux_ml_kem_vector_portable_ntt_layer_2_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U));
    zeta_i[0U]++;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_1_ea(
    size_t *zeta_i, Eurydice_arr_9e *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U]++;
    re->data[round] = libcrux_ml_kem_vector_portable_ntt_layer_1_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)3U));
    zeta_i[0U] += (size_t)3U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_poly_barrett_reduce_ea(
    Eurydice_arr_9e *myself) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_barrett_reduce_b8(myself->data[i0]);
    myself->data[i0] = uu____0;
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_ea(
    Eurydice_arr_9e *self) {
  libcrux_ml_kem_polynomial_poly_barrett_reduce_ea(self);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_vector_u
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- VECTOR_U_COMPRESSION_FACTOR= 10
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_vector_u_f7(
    Eurydice_arr_9e *re) {
  size_t zeta_i = (size_t)0U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(&zeta_i, re, (size_t)7U,
                                            (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(&zeta_i, re, (size_t)6U,
                                            (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(&zeta_i, re, (size_t)5U,
                                            (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(&zeta_i, re, (size_t)4U,
                                            (size_t)4U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_3_ea(&zeta_i, re, (size_t)5U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_2_ea(&zeta_i, re, (size_t)6U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_1_ea(&zeta_i, re, (size_t)7U * (size_t)3328U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_ea(re);
}

/**
 Call [`deserialize_then_decompress_ring_element_u`] on each ring element
 in the `ciphertext`.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.deserialize_then_decompress_u
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- U_COMPRESSION_FACTOR= 10
*/
static KRML_MUSTINLINE Eurydice_arr_bb
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_30(
    const Eurydice_arr_2b *ciphertext) {
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_30(
            &lvalue, i);
  }
  Eurydice_arr_bb u_as_ntt = arr_struct;
  for (size_t i = (size_t)0U;
       i <
       (size_t)1088U / (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                        (size_t)10U / (size_t)8U);
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 u_bytes = Eurydice_array_to_subslice_shared_d4(
        ciphertext,
        (KRML_CLITERAL(core_ops_range_Range_87){
            .start =
                i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                      (size_t)10U / (size_t)8U),
            .end = i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                         (size_t)10U / (size_t)8U) +
                   LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                       (size_t)10U / (size_t)8U}));
    u_as_ntt.data[i0] =
        libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_f7(
            u_bytes);
    libcrux_ml_kem_ntt_ntt_vector_u_f7(&u_as_ntt.data[i0]);
  }
  return u_as_ntt;
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.portable.compress.decompress_ciphertext_coefficient with
const generics
- COEFFICIENT_BITS= 4
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_d1(
    Eurydice_arr_d6 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int32_t decompressed =
        libcrux_secrets_int_as_i32_f5(a.data[i0]) *
        libcrux_secrets_int_as_i32_f5(
            libcrux_secrets_int_public_integers_classify_27_39(
                LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
    decompressed = (int32_t)((uint32_t)decompressed << 1U) +
                   (int32_t)((uint32_t)1 << (uint32_t)4);
    decompressed >>= (uint32_t)(4 + 1);
    a.data[i0] = libcrux_secrets_int_as_i16_36(decompressed);
  }
  return a;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
/**
A monomorphic instance of
libcrux_ml_kem.vector.portable.decompress_ciphertext_coefficient_b8 with const
generics
- COEFFICIENT_BITS= 4
*/
static inline Eurydice_arr_d6
libcrux_ml_kem_vector_portable_decompress_ciphertext_coefficient_b8_d1(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_d1(
      a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_4
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_then_decompress_4_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_9e re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)8U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_c8(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_87){
            .start = i0 * (size_t)8U, .end = i0 * (size_t)8U + (size_t)8U}));
    Eurydice_arr_d6 coefficient =
        libcrux_ml_kem_vector_portable_deserialize_4_b8(bytes);
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_decompress_ciphertext_coefficient_b8_d1(
            coefficient);
    re.data[i0] = uu____0;
  }
  return re;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_ring_element_v with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- COMPRESSION_FACTOR= 4
*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_b6(
    Eurydice_borrow_slice_u8 serialized) {
  return libcrux_ml_kem_serialize_deserialize_then_decompress_4_ea(serialized);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static inline Eurydice_arr_9e libcrux_ml_kem_polynomial_ZERO_ea(void) {
  Eurydice_arr_9e lit;
  Eurydice_arr_d6 repeat_expression[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    repeat_expression[i] = libcrux_ml_kem_vector_portable_ZERO_b8();
  }
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(Eurydice_arr_d6));
  return lit;
}

/**
 Given two `KyberPolynomialRingElement`s in their NTT representations,
 compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
 the `iᵗʰ` coefficient of the product `k̂` is determined by the calculation:

 ```plaintext
 ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X²
 - ζ^(2·BitRev₇(i) + 1))
 ```

 This function almost implements <strong>Algorithm 10</strong> of the
 NIST FIPS 203 standard, which is reproduced below:

 ```plaintext
 Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
 Output: An array ĥ ∈ ℤq.

 for(i ← 0; i < 128; i++)
     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1],
 ζ^(2·BitRev₇(i) + 1)) end for return ĥ
 ```
 We say "almost" because the coefficients of the ring element output by
 this function are in the Montgomery domain.

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ntt_multiply
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_ntt_multiply_ea(const Eurydice_arr_9e *myself,
                                          const Eurydice_arr_9e *rhs) {
  Eurydice_arr_9e out = libcrux_ml_kem_polynomial_ZERO_ea();
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 uu____0 = libcrux_ml_kem_vector_portable_ntt_multiply_b8(
        &myself->data[i0], &rhs->data[i0],
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 +
                                       (size_t)1U),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 +
                                       (size_t)2U),
        libcrux_ml_kem_polynomial_zeta((size_t)64U + (size_t)4U * i0 +
                                       (size_t)3U));
    out.data[i0] = uu____0;
  }
  return out;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.ntt_multiply_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(const Eurydice_arr_9e *self,
                                             const Eurydice_arr_9e *rhs) {
  return libcrux_ml_kem_polynomial_ntt_multiply_ea(self, rhs);
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_to_ring_element_68(
    Eurydice_arr_9e *myself, const Eurydice_arr_9e *rhs) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_add_b8(myself->data[i0], &rhs->data[i0]);
    myself->data[i0] = uu____0;
  }
}

/**
 Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
 sum of their constituent coefficients.
*/
/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_to_ring_element_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_to_ring_element_d6_68(
    Eurydice_arr_9e *self, const Eurydice_arr_9e *rhs) {
  libcrux_ml_kem_polynomial_add_to_ring_element_68(self, rhs);
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_ea(
    size_t *zeta_i, Eurydice_arr_9e *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U]--;
    re->data[round] = libcrux_ml_kem_vector_portable_inv_ntt_layer_1_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)3U));
    zeta_i[0U] -= (size_t)3U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_2
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_ea(
    size_t *zeta_i, Eurydice_arr_9e *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U]--;
    re->data[round] = libcrux_ml_kem_vector_portable_inv_ntt_layer_2_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U));
    zeta_i[0U]--;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_3
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_ea(
    size_t *zeta_i, Eurydice_arr_9e *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U]--;
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_inv_ntt_layer_3_step_b8(
            re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
    re->data[round] = uu____0;
  }
}

/**
A monomorphic instance of
libcrux_ml_kem.invert_ntt.inv_ntt_layer_int_vec_step_reduce with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE
    libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2
    libcrux_ml_kem_invert_ntt_inv_ntt_layer_int_vec_step_reduce_ea(
        Eurydice_arr_d6 a, Eurydice_arr_d6 b, int16_t zeta_r) {
  Eurydice_arr_d6 a_minus_b = libcrux_ml_kem_vector_portable_sub_b8(b, &a);
  a = libcrux_ml_kem_vector_portable_barrett_reduce_b8(
      libcrux_ml_kem_vector_portable_add_b8(a, &b));
  b = libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
      a_minus_b, zeta_r);
  return (KRML_CLITERAL(
      libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2){.fst = a,
                                                                    .snd = b});
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_ea(size_t *zeta_i,
                                                        Eurydice_arr_9e *re,
                                                        size_t layer) {
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++) {
    size_t round = i0;
    zeta_i[0U]--;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec =
        offset / LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR;
    size_t step_vec =
        step / LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++) {
      size_t j = i;
      libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2 uu____0 =
          libcrux_ml_kem_invert_ntt_inv_ntt_layer_int_vec_step_reduce_ea(
              re->data[j], re->data[j + step_vec],
              libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
      Eurydice_arr_d6 x = uu____0.fst;
      Eurydice_arr_d6 y = uu____0.snd;
      re->data[j] = x;
      re->data[j + step_vec] = y;
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_montgomery
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_68(
    Eurydice_arr_9e *re) {
  size_t zeta_i =
      LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT / (size_t)2U;
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_ea(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_ea(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_ea(&zeta_i, re);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_ea(&zeta_i, re,
                                                          (size_t)4U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_ea(&zeta_i, re,
                                                          (size_t)5U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_ea(&zeta_i, re,
                                                          (size_t)6U);
  libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_ea(&zeta_i, re,
                                                          (size_t)7U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_ea(re);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.subtract_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_subtract_reduce_ea(const Eurydice_arr_9e *myself,
                                             Eurydice_arr_9e b) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient_normal_form =
        libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
            b.data[i0], 1441);
    Eurydice_arr_d6 diff = libcrux_ml_kem_vector_portable_sub_b8(
        myself->data[i0], &coefficient_normal_form);
    Eurydice_arr_d6 red =
        libcrux_ml_kem_vector_portable_barrett_reduce_b8(diff);
    b.data[i0] = red;
  }
  return b;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.subtract_reduce_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_subtract_reduce_d6_ea(const Eurydice_arr_9e *self,
                                                Eurydice_arr_9e b) {
  return libcrux_ml_kem_polynomial_subtract_reduce_ea(self, b);
}

/**
 The following functions compute various expressions involving
 vectors and matrices. The computation of these expressions has been
 abstracted away into these functions in order to save on loop iterations.
 Compute v − InverseNTT(sᵀ ◦ NTT(u))
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_message
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_9e libcrux_ml_kem_matrix_compute_message_68(
    const Eurydice_arr_9e *v, const Eurydice_arr_bb *secret_as_ntt,
    const Eurydice_arr_bb *u_as_ntt) {
  Eurydice_arr_9e result = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
        &secret_as_ntt->data[i0], &u_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_68(&result, &product);
  }
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_68(&result);
  return libcrux_ml_kem_polynomial_subtract_reduce_d6_ea(v, result);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.to_unsigned_field_modulus
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_to_unsigned_representative_b8(a);
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_message with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_serialize_compress_then_serialize_message_ea(
    Eurydice_arr_9e re) {
  Eurydice_arr_ec serialized = {.data = {0U}};
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient =
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re.data[i0]);
    Eurydice_arr_d6 coefficient_compressed =
        libcrux_ml_kem_vector_portable_compress_1_b8(coefficient);
    Eurydice_array_u8x2 bytes =
        libcrux_ml_kem_vector_portable_serialize_1_b8(coefficient_compressed);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_d44(
            &serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                             .start = (size_t)2U * i0,
                             .end = (size_t)2U * i0 + (size_t)2U})),
        Eurydice_array_to_slice_shared_82(&bytes), uint8_t);
  }
  return serialized;
}

/**
 This function implements <strong>Algorithm 14</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE decryption algorithm.

 Algorithm 14 is reproduced below:

 ```plaintext
 Input: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
 Input: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
 Output: message m ∈ 𝔹^{32}.

 c₁ ← c[0 : 32dᵤk]
 c₂ ← c[32dᵤk : 32(dᵤk + dᵥ)]
 u ← Decompress_{dᵤ}(ByteDecode_{dᵤ}(c₁))
 v ← Decompress_{dᵥ}(ByteDecode_{dᵥ}(c₂))
 ŝ ← ByteDecode₁₂(dkₚₖₑ)
 w ← v - NTT-¹(ŝᵀ ◦ NTT(u))
 m ← ByteEncode₁(Compress₁(w))
 return m
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt_unpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- VECTOR_U_ENCODED_SIZE= 960
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cpa_decrypt_unpacked_01(const Eurydice_arr_bb *secret_key,
                                           const Eurydice_arr_2b *ciphertext) {
  Eurydice_arr_bb u_as_ntt =
      libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_30(ciphertext);
  Eurydice_arr_9e v =
      libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_b6(
          Eurydice_array_to_subslice_from_shared_5f(ciphertext, (size_t)960U));
  Eurydice_arr_9e message =
      libcrux_ml_kem_matrix_compute_message_68(&v, secret_key, &u_as_ntt);
  return libcrux_ml_kem_serialize_compress_then_serialize_message_ea(message);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.decrypt
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- VECTOR_U_ENCODED_SIZE= 960
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
*/
static KRML_MUSTINLINE Eurydice_arr_ec libcrux_ml_kem_ind_cpa_decrypt_01(
    Eurydice_borrow_slice_u8 secret_key, const Eurydice_arr_2b *ciphertext) {
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_01(&lvalue, i);
  }
  Eurydice_arr_bb secret_key_unpacked = arr_struct;
  libcrux_ml_kem_ind_cpa_deserialize_vector_68(secret_key,
                                               &secret_key_unpacked);
  return libcrux_ml_kem_ind_cpa_decrypt_unpacked_01(&secret_key_unpacked,
                                                    ciphertext);
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.G_4a
with const generics
- K= 3
*/
static inline Eurydice_arr_c7 libcrux_ml_kem_hash_functions_portable_G_4a_78(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_G(input);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRF
with const generics
- LEN= 32
*/
static inline Eurydice_arr_ec libcrux_ml_kem_hash_functions_portable_PRF_ce(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_ec digest = {.data = {0U}};
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_01(&digest),
                                 input);
  return digest;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRF_4a
with const generics
- K= 3
- LEN= 32
*/
static inline Eurydice_arr_ec libcrux_ml_kem_hash_functions_portable_PRF_4a_3b(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_PRF_ce(input);
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_bb
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_1d_s {
  Eurydice_arr_bb data[3U];
} Eurydice_arr_1d;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51_s {
  Eurydice_arr_bb t_as_ntt;
  Eurydice_arr_ec seed_for_A;
  Eurydice_arr_1d A;
} libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51;

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_8b
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
libcrux_ml_kem_ind_cpa_unpacked_default_8b_68(void) {
  Eurydice_arr_bb uu____0;
  Eurydice_arr_9e repeat_expression0[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression0[i] = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  }
  memcpy(uu____0.data, repeat_expression0,
         (size_t)3U * sizeof(Eurydice_arr_9e));
  Eurydice_arr_ec uu____1 = {.data = {0U}};
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 lit0;
  lit0.t_as_ntt = uu____0;
  lit0.seed_for_A = uu____1;
  Eurydice_arr_bb repeat_expression1[3U];
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    Eurydice_arr_bb lit;
    Eurydice_arr_9e repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_ea();
    }
    memcpy(lit.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_9e));
    repeat_expression1[i0] = lit;
  }
  memcpy(lit0.A.data, repeat_expression1, (size_t)3U * sizeof(Eurydice_arr_bb));
  return lit0;
}

/**
 Only use with public values.

 This MUST NOT be used with secret inputs, like its caller
 `deserialize_ring_elements_reduced`.
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_to_reduced_ring_element with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_9e re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < serialized.meta / (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_c8(
        serialized,
        (KRML_CLITERAL(core_ops_range_Range_87){
            .start = i0 * (size_t)24U, .end = i0 * (size_t)24U + (size_t)24U}));
    Eurydice_arr_d6 coefficient =
        libcrux_ml_kem_vector_portable_deserialize_12_b8(bytes);
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_cond_subtract_3329_b8(coefficient);
    re.data[i0] = uu____0;
  }
  return re;
}

/**
 See [deserialize_ring_elements_reduced_out].
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_ring_elements_reduced with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_68(
    Eurydice_borrow_slice_u8 public_key, Eurydice_arr_bb *deserialized_pk) {
  for (size_t i = (size_t)0U;
       i < public_key.meta / LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT;
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 ring_element = Eurydice_slice_subslice_shared_c8(
        public_key,
        (KRML_CLITERAL(core_ops_range_Range_87){
            .start = i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
            .end = i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT +
                   LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT}));
    Eurydice_arr_9e uu____0 =
        libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_ea(
            ring_element);
    deserialized_pk->data[i0] = uu____0;
  }
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_init_absorb_final with const
generics
- K= 3
*/
static inline Eurydice_arr_1b
libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_78(
    const Eurydice_arr_81 *input) {
  Eurydice_arr_1b shake128_state;
  Eurydice_arr_7c repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = libcrux_sha3_portable_incremental_shake128_init();
  }
  memcpy(shake128_state.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_7c));
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_incremental_shake128_absorb_final(
        &shake128_state.data[i0],
        Eurydice_array_to_slice_shared_e9(&input->data[i0]));
  }
  return shake128_state;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_init_absorb_final_4a with const
generics
- K= 3
*/
static inline Eurydice_arr_1b
libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_78(
    const Eurydice_arr_81 *input) {
  return libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_78(
      input);
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_squeeze_first_three_blocks with
const generics
- K= 3
*/
static inline Eurydice_arr_7e
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_78(
    Eurydice_arr_1b *st) {
  Eurydice_arr_7e out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_incremental_shake128_squeeze_first_three_blocks(
        &st->data[i0], Eurydice_array_to_slice_mut_48(&out.data[i0]));
  }
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_squeeze_first_three_blocks_4a
with const generics
- K= 3
*/
static inline Eurydice_arr_7e
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_78(
    Eurydice_arr_1b *self) {
  return libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_78(
      self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT
 representation of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm
 6</strong> of the NIST FIPS 203 standard, We say "partially" because this
 implementation only accepts a finite set of bytes as input and returns an error
 if the set is not enough; Algorithm 6 of the FIPS 203 standard on the other
 hand samples from an infinite stream of bytes until the ring element is filled.
 Algorithm 6 is reproduced below:

 ```plaintext
 Input: byte stream B ∈ 𝔹*.
 Output: array â ∈ ℤ₂₅₆.

 i ← 0
 j ← 0
 while j < 256 do
     d₁ ← B[i] + 256·(B[i+1] mod 16)
     d₂ ← ⌊B[i+1]/16⌋ + 16·B[i+2]
     if d₁ < q then
         â[j] ← d₁
         j ← j + 1
     end if
     if d₂ < q and j < 256 then
         â[j] ← d₂
         j ← j + 1
     end if
     i ← i + 3
 end while
 return â
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_uniform_distribution_next with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- N= 504
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_b6(
    const Eurydice_arr_7e *randomness, Eurydice_arr_eb *sampled_coefficients,
    Eurydice_arr_b1 *out) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)504U / (size_t)24U; i++) {
      size_t r = i;
      if (sampled_coefficients->data[i1] <
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        size_t sampled = libcrux_ml_kem_vector_portable_rej_sample_b8(
            Eurydice_array_to_subslice_shared_d40(
                &randomness->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_87){
                    .start = r * (size_t)24U,
                    .end = r * (size_t)24U + (size_t)24U})),
            Eurydice_array_to_subslice_mut_e7(
                &out->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_87){
                    .start = sampled_coefficients->data[i1],
                    .end = sampled_coefficients->data[i1] + (size_t)16U})));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    }
  }
  bool done = true;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >=
        LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
      sampled_coefficients->data[i0] =
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    } else {
      done = false;
    }
  }
  return done;
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_squeeze_next_block with const
generics
- K= 3
*/
static inline Eurydice_arr_2c
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_78(
    Eurydice_arr_1b *st) {
  Eurydice_arr_2c out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
        &st->data[i0], Eurydice_array_to_slice_mut_2c(&out.data[i0]));
  }
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_squeeze_next_block_4a with const
generics
- K= 3
*/
static inline Eurydice_arr_2c
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_78(
    Eurydice_arr_1b *self) {
  return libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_78(
      self);
}

/**
 If `bytes` contains a set of uniformly random bytes, this function
 uniformly samples a ring element `â` that is treated as being the NTT
 representation of the corresponding polynomial `a`.

 Since rejection sampling is used, it is possible the supplied bytes are
 not enough to sample the element, in which case an `Err` is returned and the
 caller must try again with a fresh set of bytes.

 This function <strong>partially</strong> implements <strong>Algorithm
 6</strong> of the NIST FIPS 203 standard, We say "partially" because this
 implementation only accepts a finite set of bytes as input and returns an error
 if the set is not enough; Algorithm 6 of the FIPS 203 standard on the other
 hand samples from an infinite stream of bytes until the ring element is filled.
 Algorithm 6 is reproduced below:

 ```plaintext
 Input: byte stream B ∈ 𝔹*.
 Output: array â ∈ ℤ₂₅₆.

 i ← 0
 j ← 0
 while j < 256 do
     d₁ ← B[i] + 256·(B[i+1] mod 16)
     d₂ ← ⌊B[i+1]/16⌋ + 16·B[i+2]
     if d₁ < q then
         â[j] ← d₁
         j ← j + 1
     end if
     if d₂ < q and j < 256 then
         â[j] ← d₂
         j ← j + 1
     end if
     i ← i + 3
 end while
 return â
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_uniform_distribution_next with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- N= 168
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_b60(
    const Eurydice_arr_2c *randomness, Eurydice_arr_eb *sampled_coefficients,
    Eurydice_arr_b1 *out) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)24U; i++) {
      size_t r = i;
      if (sampled_coefficients->data[i1] <
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        size_t sampled = libcrux_ml_kem_vector_portable_rej_sample_b8(
            Eurydice_array_to_subslice_shared_d41(
                &randomness->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_87){
                    .start = r * (size_t)24U,
                    .end = r * (size_t)24U + (size_t)24U})),
            Eurydice_array_to_subslice_mut_e7(
                &out->data[i1],
                (KRML_CLITERAL(core_ops_range_Range_87){
                    .start = sampled_coefficients->data[i1],
                    .end = sampled_coefficients->data[i1] + (size_t)16U})));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] += sampled;
      }
    }
  }
  bool done = true;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    if (sampled_coefficients->data[i0] >=
        LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
      sampled_coefficients->data[i0] =
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT;
    } else {
      done = false;
    }
  }
  return done;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.from_i16_array
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_from_i16_array_ea(Eurydice_borrow_slice_i16 a) {
  Eurydice_arr_9e result = libcrux_ml_kem_polynomial_ZERO_ea();
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 uu____0 = libcrux_ml_kem_vector_portable_from_i16_array_b8(
        Eurydice_slice_subslice_shared_a6(
            a, (KRML_CLITERAL(core_ops_range_Range_87){
                   .start = i0 * (size_t)16U,
                   .end = (i0 + (size_t)1U) * (size_t)16U})));
    result.data[i0] = uu____0;
  }
  return result;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.from_i16_array_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_from_i16_array_d6_ea(Eurydice_borrow_slice_i16 a) {
  return libcrux_ml_kem_polynomial_from_i16_array_ea(a);
}

/**
This function found in impl {core::ops::function::FnMut<([i16; 272usize]),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector,
Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_0a
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
static inline Eurydice_arr_9e
libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_91(
    void **_, Eurydice_arr_5b tupled_args) {
  Eurydice_arr_5b s = tupled_args;
  return libcrux_ml_kem_polynomial_from_i16_array_d6_ea(
      Eurydice_array_to_subslice_shared_e70(
          &s, (KRML_CLITERAL(core_ops_range_Range_87){.start = (size_t)0U,
                                                      .end = (size_t)256U})));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_bb
libcrux_ml_kem_sampling_sample_from_xof_91(const Eurydice_arr_81 *seeds) {
  Eurydice_arr_eb sampled_coefficients = {.data = {0U}};
  Eurydice_arr_b1 out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  Eurydice_arr_1b xof_state =
      libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_78(
          seeds);
  Eurydice_arr_7e randomness0 =
      libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_78(
          &xof_state);
  bool done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_b6(
      &randomness0, &sampled_coefficients, &out);
  while (true) {
    if (done) {
      break;
    } else {
      Eurydice_arr_2c randomness =
          libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_78(
              &xof_state);
      done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_b60(
          &randomness, &sampled_coefficients, &out);
    }
  }
  Eurydice_arr_bb arr_mapped_str;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
        libcrux_ml_kem_sampling_sample_from_xof_call_mut_0a_91(&lvalue,
                                                               out.data[i]);
  }
  return arr_mapped_str;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_sample_matrix_A_91(
    Eurydice_arr_1d *A_transpose, const Eurydice_arr_31 *seed, bool transpose) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    Eurydice_arr_81 seeds;
    Eurydice_arr_31 repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] = core_array__core__clone__Clone_for__T__N___clone(
          (size_t)34U, seed, uint8_t, Eurydice_arr_31);
    }
    memcpy(seeds.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_31));
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;
    }
    Eurydice_arr_bb sampled =
        libcrux_ml_kem_sampling_sample_from_xof_91(&seeds);
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      Eurydice_arr_9e sample = sampled.data[j];
      if (transpose) {
        A_transpose->data[j].data[i1] = sample;
      } else {
        A_transpose->data[i1].data[j] = sample;
      }
    }
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key_mut
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_05(
    Eurydice_borrow_slice_u8 public_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
        *unpacked_public_key) {
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_68(
      Eurydice_slice_subslice_to_shared_72(public_key, (size_t)1152U),
      &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8 seed =
      Eurydice_slice_subslice_from_shared_6d(public_key, (size_t)1152U);
  Eurydice_arr_1d *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_91(uu____0, &lvalue, false);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.build_unpacked_public_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
*/
static KRML_MUSTINLINE
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
    libcrux_ml_kem_ind_cpa_build_unpacked_public_key_05(
        Eurydice_borrow_slice_u8 public_key) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
      unpacked_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_68();
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_05(public_key,
                                                          &unpacked_public_key);
  return unpacked_public_key;
}

/**
A monomorphic instance of n-tuple
with types Eurydice_arr_bb, libcrux_ml_kem_polynomial_PolynomialRingElement_1d

*/
typedef struct tuple_5b_s {
  Eurydice_arr_bb fst;
  Eurydice_arr_9e snd;
} tuple_5b;

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure<Vector, Hasher,
K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE, ETA2,
ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2,
TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_f1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- C1_LEN= 960
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
static inline Eurydice_arr_9e libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_87(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRFxN
with const generics
- K= 3
- LEN= 128
*/
static inline Eurydice_arr_58 libcrux_ml_kem_hash_functions_portable_PRFxN_3b(
    const Eurydice_arr_800 *input) {
  Eurydice_arr_58 out = {
      .data = {{.data = {0U}}, {.data = {0U}}, {.data = {0U}}}};
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_shake256(
        Eurydice_array_to_slice_mut_78(&out.data[i0]),
        Eurydice_array_to_slice_shared_b5(&input->data[i0]));
  }
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRFxN_4a
with const generics
- K= 3
- LEN= 128
*/
static inline Eurydice_arr_58
libcrux_ml_kem_hash_functions_portable_PRFxN_4a_3b(
    const Eurydice_arr_800 *input) {
  return libcrux_ml_kem_hash_functions_portable_PRFxN_3b(input);
}

/**
 Given a series of uniformly random bytes in `randomness`, for some number
 `eta`, the `sample_from_binomial_distribution_{eta}` functions sample a ring
 element from a binomial distribution centered at 0 that uses two sets of `eta`
 coin flips. If, for example, `eta = ETA`, each ring coefficient is a value `v`
 such such that `v ∈ {-ETA, -ETA + 1, ..., 0, ..., ETA + 1, ETA}` and:

 ```plaintext
 - If v < 0, Pr[v] = Pr[-v]
 - If v >= 0, Pr[v] = BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2 ^ (2 * ETA)
 ```

 The values `v < 0` are mapped to the appropriate `KyberFieldElement`.

 The expected value is:

 ```plaintext
 E[X] = (-ETA)Pr[-ETA] + (-(ETA - 1))Pr[-(ETA - 1)] + ... + (ETA - 1)Pr[ETA - 1]
 + (ETA)Pr[ETA] = 0 since Pr[-v] = Pr[v] when v < 0.
 ```

 And the variance is:

 ```plaintext
 Var(X) = E[(X - E[X])^2]
        = E[X^2]
        = sum_(v=-ETA to ETA)v^2 * (BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) /
 2^(2 * ETA)) = ETA / 2
 ```

 This function implements <strong>Algorithm 7</strong> of the NIST FIPS 203
 standard, which is reproduced below:

 ```plaintext
 Input: byte array B ∈ 𝔹^{64η}.
 Output: array f ∈ ℤ₂₅₆.

 b ← BytesToBits(B)
 for (i ← 0; i < 256; i++)
     x ← ∑(j=0 to η - 1) b[2iη + j]
     y ← ∑(j=0 to η - 1) b[2iη + η + j]
     f[i] ← x−y mod q
 end for
 return f
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_binomial_distribution_2 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_ea(
    Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_04 sampled_i16s = {.data = {0U}};
  for (size_t i0 = (size_t)0U; i0 < randomness.meta / (size_t)4U; i0++) {
    size_t chunk_number = i0;
    Eurydice_borrow_slice_u8 byte_chunk = Eurydice_slice_subslice_shared_c8(
        randomness, (KRML_CLITERAL(core_ops_range_Range_87){
                        .start = chunk_number * (size_t)4U,
                        .end = chunk_number * (size_t)4U + (size_t)4U}));
    uint32_t random_bits_as_u32 =
        (((uint32_t)byte_chunk.ptr[0U] | (uint32_t)byte_chunk.ptr[1U] << 8U) |
         (uint32_t)byte_chunk.ptr[2U] << 16U) |
        (uint32_t)byte_chunk.ptr[3U] << 24U;
    uint32_t even_bits = random_bits_as_u32 & 1431655765U;
    uint32_t odd_bits = random_bits_as_u32 >> 1U & 1431655765U;
    uint32_t coin_toss_outcomes = even_bits + odd_bits;
    for (uint32_t i = 0U; i < 32U / 4U; i++) {
      uint32_t outcome_set = i;
      uint32_t outcome_set0 = outcome_set * 4U;
      int16_t outcome_1 =
          (int16_t)(coin_toss_outcomes >> (uint32_t)outcome_set0 & 3U);
      int16_t outcome_2 =
          (int16_t)(coin_toss_outcomes >> (uint32_t)(outcome_set0 + 2U) & 3U);
      size_t offset = (size_t)(outcome_set0 >> 2U);
      sampled_i16s.data[(size_t)8U * chunk_number + offset] =
          outcome_1 - outcome_2;
    }
  }
  return libcrux_ml_kem_polynomial_from_i16_array_d6_ea(
      Eurydice_array_to_slice_shared_99(&sampled_i16s));
}

/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_binomial_distribution with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- ETA= 2
*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_sampling_sample_from_binomial_distribution_66(
    Eurydice_borrow_slice_u8 randomness) {
  return libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_ea(
      randomness);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_7
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_7_ea(
    Eurydice_arr_9e *re) {
  size_t step = LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT / (size_t)2U;
  for (size_t i = (size_t)0U; i < step; i++) {
    size_t j = i;
    Eurydice_arr_d6 t = libcrux_ml_kem_vector_portable_multiply_by_constant_b8(
        re->data[j + step], -1600);
    re->data[j + step] = libcrux_ml_kem_vector_portable_sub_b8(re->data[j], &t);
    Eurydice_arr_d6 uu____1 =
        libcrux_ml_kem_vector_portable_add_b8(re->data[j], &t);
    re->data[j] = uu____1;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_binomially_sampled_ring_element
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_ea(Eurydice_arr_9e *re) {
  libcrux_ml_kem_ntt_ntt_at_layer_7_ea(re);
  size_t zeta_i = (size_t)1U;
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(&zeta_i, re, (size_t)6U,
                                            (size_t)11207U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(&zeta_i, re, (size_t)5U,
                                            (size_t)11207U + (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(
      &zeta_i, re, (size_t)4U, (size_t)11207U + (size_t)2U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_3_ea(
      &zeta_i, re, (size_t)11207U + (size_t)3U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_2_ea(
      &zeta_i, re, (size_t)11207U + (size_t)4U * (size_t)3328U);
  libcrux_ml_kem_ntt_ntt_at_layer_1_ea(
      &zeta_i, re, (size_t)11207U + (size_t)5U * (size_t)3328U);
  libcrux_ml_kem_polynomial_poly_barrett_reduce_d6_ea(re);
}

/**
 Sample a vector of ring elements from a centered binomial distribution and
 convert them into their NTT representations.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_vector_cbd_then_ntt
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- ETA= 2
- ETA_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_bf(
    Eurydice_arr_bb *re_as_ntt, const Eurydice_arr_fa *prf_input,
    uint8_t domain_separator) {
  Eurydice_arr_800 prf_inputs;
  Eurydice_arr_fa repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = core_array__core__clone__Clone_for__T__N___clone(
        (size_t)33U, prf_input, uint8_t, Eurydice_arr_fa);
  }
  memcpy(prf_inputs.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_fa));
  domain_separator =
      libcrux_ml_kem_utils_prf_input_inc_78(&prf_inputs, domain_separator);
  Eurydice_arr_58 prf_outputs =
      libcrux_ml_kem_hash_functions_portable_PRFxN_4a_3b(&prf_inputs);
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e uu____0 =
        libcrux_ml_kem_sampling_sample_from_binomial_distribution_66(
            Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    re_as_ntt->data[i0] = uu____0;
    libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_ea(
        &re_as_ntt->data[i0]);
  }
  return domain_separator;
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::ind_cpa::encrypt_c1::closure#1<Vector,
Hasher, K, C1_LEN, U_COMPRESSION_FACTOR, BLOCK_LEN, ETA1, ETA1_RANDOMNESS_SIZE,
ETA2, ETA2_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1, TraitClause@2,
TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1.call_mut_dd
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- C1_LEN= 960
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
static inline Eurydice_arr_9e libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_87(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
 Sample a vector of ring elements from a centered binomial distribution.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.sample_ring_element_cbd
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- ETA2_RANDOMNESS_SIZE= 128
- ETA2= 2
*/
static KRML_MUSTINLINE uint8_t
libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_bf(
    const Eurydice_arr_fa *prf_input, uint8_t domain_separator,
    Eurydice_arr_bb *error_1) {
  Eurydice_arr_800 prf_inputs;
  Eurydice_arr_fa repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = core_array__core__clone__Clone_for__T__N___clone(
        (size_t)33U, prf_input, uint8_t, Eurydice_arr_fa);
  }
  memcpy(prf_inputs.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_fa));
  domain_separator =
      libcrux_ml_kem_utils_prf_input_inc_78(&prf_inputs, domain_separator);
  Eurydice_arr_58 prf_outputs =
      libcrux_ml_kem_hash_functions_portable_PRFxN_4a_3b(&prf_inputs);
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e uu____0 =
        libcrux_ml_kem_sampling_sample_from_binomial_distribution_66(
            Eurydice_array_to_slice_shared_78(&prf_outputs.data[i0]));
    error_1->data[i0] = uu____0;
  }
  return domain_separator;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRF
with const generics
- LEN= 128
*/
static inline Eurydice_arr_89 libcrux_ml_kem_hash_functions_portable_PRF_ec(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_89 digest = {.data = {0U}};
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_78(&digest),
                                 input);
  return digest;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRF_4a
with const generics
- K= 3
- LEN= 128
*/
static inline Eurydice_arr_89 libcrux_ml_kem_hash_functions_portable_PRF_4a_3b0(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_PRF_ec(input);
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for libcrux_ml_kem::matrix::compute_vector_u::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u.call_mut_a8
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static inline Eurydice_arr_9e
libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_68(void **_,
                                                      size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_error_reduce_ea(
    Eurydice_arr_9e *myself, const Eurydice_arr_9e *error) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t j = i;
    Eurydice_arr_d6 coefficient_normal_form =
        libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
            myself->data[j], 1441);
    Eurydice_arr_d6 sum = libcrux_ml_kem_vector_portable_add_b8(
        coefficient_normal_form, &error->data[j]);
    Eurydice_arr_d6 red = libcrux_ml_kem_vector_portable_barrett_reduce_b8(sum);
    myself->data[j] = red;
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_error_reduce_d6_ea(
    Eurydice_arr_9e *self, const Eurydice_arr_9e *error) {
  libcrux_ml_kem_polynomial_add_error_reduce_ea(self, error);
}

/**
 Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_vector_u
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_bb
libcrux_ml_kem_matrix_compute_vector_u_68(const Eurydice_arr_1d *a_as_ntt,
                                          const Eurydice_arr_bb *r_as_ntt,
                                          const Eurydice_arr_bb *error_1) {
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_68(&lvalue, i);
  }
  Eurydice_arr_bb result = arr_struct;
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    const Eurydice_arr_bb *row = &a_as_ntt->data[i1];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      const Eurydice_arr_9e *a_element = &row->data[j];
      Eurydice_arr_9e product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
          a_element, &r_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_68(&result.data[i1],
                                                          &product);
    }
    libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_68(&result.data[i1]);
    libcrux_ml_kem_polynomial_add_error_reduce_d6_ea(&result.data[i1],
                                                     &error_1->data[i1]);
  }
  return result;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.portable.compress.compress
with const generics
- COEFFICIENT_BITS= 10
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_compress_compress_ef(Eurydice_arr_d6 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int16_t uu____0 = libcrux_secrets_int_as_i16_f5(
        libcrux_ml_kem_vector_portable_compress_compress_ciphertext_coefficient(
            (uint8_t)10, libcrux_secrets_int_as_u16_f5(a.data[i0])));
    a.data[i0] = uu____0;
  }
  return a;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.portable.compress_b8
with const generics
- COEFFICIENT_BITS= 10
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_compress_b8_ef(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_compress_compress_ef(a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_10
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- OUT_LEN= 320
*/
static KRML_MUSTINLINE Eurydice_arr_b0
libcrux_ml_kem_serialize_compress_then_serialize_10_e1(
    const Eurydice_arr_9e *re) {
  Eurydice_arr_b0 serialized = {.data = {0U}};
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient = libcrux_ml_kem_vector_portable_compress_b8_ef(
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re->data[i0]));
    Eurydice_arr_fc bytes =
        libcrux_ml_kem_vector_portable_serialize_10_b8(coefficient);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_d49(
            &serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                             .start = (size_t)20U * i0,
                             .end = (size_t)20U * i0 + (size_t)20U})),
        Eurydice_array_to_slice_shared_8f(&bytes), uint8_t);
  }
  return serialized;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_ring_element_u with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- COMPRESSION_FACTOR= 10
- OUT_LEN= 320
*/
static KRML_MUSTINLINE Eurydice_arr_b0
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_f7(
    const Eurydice_arr_9e *re) {
  return libcrux_ml_kem_serialize_compress_then_serialize_10_e1(re);
}

/**
 Call [`compress_then_serialize_ring_element_u`] on each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.compress_then_serialize_u
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- OUT_LEN= 960
- COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_compress_then_serialize_u_21(
    Eurydice_arr_bb input, Eurydice_mut_borrow_slice_u8 out) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e re = input.data[i0];
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (KRML_CLITERAL(core_ops_range_Range_87){
                 .start = i0 * ((size_t)960U / (size_t)3U),
                 .end = (i0 + (size_t)1U) * ((size_t)960U / (size_t)3U)}));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b0 lvalue =
        libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_f7(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_56(&lvalue),
                        uint8_t);
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- C1_LEN= 960
- U_COMPRESSION_FACTOR= 10
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE tuple_5b libcrux_ml_kem_ind_cpa_encrypt_c1_87(
    Eurydice_borrow_slice_u8 randomness, const Eurydice_arr_1d *matrix,
    Eurydice_mut_borrow_slice_u8 ciphertext) {
  Eurydice_arr_fa prf_input =
      libcrux_ml_kem_utils_into_padded_array_29(randomness);
  Eurydice_arr_bb arr_struct0;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct0.data[i] =
        libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_87(&lvalue, i);
  }
  Eurydice_arr_bb r_as_ntt = arr_struct0;
  uint8_t domain_separator0 =
      libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_bf(&r_as_ntt,
                                                           &prf_input, 0U);
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_87(&lvalue, i);
  }
  Eurydice_arr_bb error_1 = arr_struct;
  uint8_t domain_separator = libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_bf(
      &prf_input, domain_separator0, &error_1);
  prf_input.data[32U] = domain_separator;
  Eurydice_arr_89 prf_output =
      libcrux_ml_kem_hash_functions_portable_PRF_4a_3b0(
          Eurydice_array_to_slice_shared_b5(&prf_input));
  Eurydice_arr_9e error_2 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_66(
          Eurydice_array_to_slice_shared_78(&prf_output));
  Eurydice_arr_bb u =
      libcrux_ml_kem_matrix_compute_vector_u_68(matrix, &r_as_ntt, &error_1);
  libcrux_ml_kem_ind_cpa_compress_then_serialize_u_21(u, ciphertext);
  return (KRML_CLITERAL(tuple_5b){.fst = r_as_ntt, .snd = error_2});
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_message with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_then_decompress_message_ea(
    const Eurydice_arr_ec *serialized) {
  Eurydice_arr_9e re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient_compressed =
        libcrux_ml_kem_vector_portable_deserialize_1_b8(
            Eurydice_array_to_subslice_shared_d42(
                serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                                .start = (size_t)2U * i0,
                                .end = (size_t)2U * i0 + (size_t)2U})));
    Eurydice_arr_d6 uu____0 =
        libcrux_ml_kem_vector_portable_decompress_1_b8(coefficient_compressed);
    re.data[i0] = uu____0;
  }
  return re;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_message_error_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_add_message_error_reduce_ea(
    const Eurydice_arr_9e *myself, const Eurydice_arr_9e *message,
    Eurydice_arr_9e result) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient_normal_form =
        libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
            result.data[i0], 1441);
    Eurydice_arr_d6 sum1 = libcrux_ml_kem_vector_portable_add_b8(
        myself->data[i0], &message->data[i0]);
    Eurydice_arr_d6 sum2 =
        libcrux_ml_kem_vector_portable_add_b8(coefficient_normal_form, &sum1);
    Eurydice_arr_d6 red =
        libcrux_ml_kem_vector_portable_barrett_reduce_b8(sum2);
    result.data[i0] = red;
  }
  return result;
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_message_error_reduce_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_polynomial_add_message_error_reduce_d6_ea(
    const Eurydice_arr_9e *self, const Eurydice_arr_9e *message,
    Eurydice_arr_9e result) {
  return libcrux_ml_kem_polynomial_add_message_error_reduce_ea(self, message,
                                                               result);
}

/**
 Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_ring_element_v
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_9e
libcrux_ml_kem_matrix_compute_ring_element_v_68(
    const Eurydice_arr_bb *t_as_ntt, const Eurydice_arr_bb *r_as_ntt,
    const Eurydice_arr_9e *error_2, const Eurydice_arr_9e *message) {
  Eurydice_arr_9e result = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
        &t_as_ntt->data[i0], &r_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_68(&result, &product);
  }
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_68(&result);
  return libcrux_ml_kem_polynomial_add_message_error_reduce_d6_ea(
      error_2, message, result);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.portable.compress.compress
with const generics
- COEFFICIENT_BITS= 4
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_vector_portable_compress_compress_d1(Eurydice_arr_d6 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int16_t uu____0 = libcrux_secrets_int_as_i16_f5(
        libcrux_ml_kem_vector_portable_compress_compress_ciphertext_coefficient(
            (uint8_t)4, libcrux_secrets_int_as_u16_f5(a.data[i0])));
    a.data[i0] = uu____0;
  }
  return a;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
/**
A monomorphic instance of libcrux_ml_kem.vector.portable.compress_b8
with const generics
- COEFFICIENT_BITS= 4
*/
static inline Eurydice_arr_d6 libcrux_ml_kem_vector_portable_compress_b8_d1(
    Eurydice_arr_d6 a) {
  return libcrux_ml_kem_vector_portable_compress_compress_d1(a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_4
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_4_ea(
    Eurydice_arr_9e re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient = libcrux_ml_kem_vector_portable_compress_b8_d1(
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re.data[i0]));
    Eurydice_array_u8x8 bytes =
        libcrux_ml_kem_vector_portable_serialize_4_b8(coefficient);
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_c8(
            serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                            .start = (size_t)8U * i0,
                            .end = (size_t)8U * i0 + (size_t)8U})),
        Eurydice_array_to_slice_shared_6e(&bytes), uint8_t);
  }
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_ring_element_v with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
- COMPRESSION_FACTOR= 4
- OUT_LEN= 128
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_30(
    Eurydice_arr_9e re, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_ml_kem_serialize_compress_then_serialize_4_ea(re, out);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_c2
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- V_COMPRESSION_FACTOR= 4
- C2_LEN= 128
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_encrypt_c2_30(
    const Eurydice_arr_bb *t_as_ntt, const Eurydice_arr_bb *r_as_ntt,
    const Eurydice_arr_9e *error_2, const Eurydice_arr_ec *message,
    Eurydice_mut_borrow_slice_u8 ciphertext) {
  Eurydice_arr_9e message_as_ring_element =
      libcrux_ml_kem_serialize_deserialize_then_decompress_message_ea(message);
  Eurydice_arr_9e v = libcrux_ml_kem_matrix_compute_ring_element_v_68(
      t_as_ntt, r_as_ntt, error_2, &message_as_ring_element);
  libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_30(
      v, ciphertext);
}

/**
 This function implements <strong>Algorithm 13</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE encryption algorithm.

 Algorithm 13 is reproduced below:

 ```plaintext
 Input: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
 Input: message m ∈ 𝔹^{32}.
 Input: encryption randomness r ∈ 𝔹^{32}.
 Output: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.

 N ← 0
 t̂ ← ByteDecode₁₂(ekₚₖₑ[0:384k])
 ρ ← ekₚₖₑ[384k: 384k + 32]
 for (i ← 0; i < k; i++)
     for(j ← 0; j < k; j++)
         Â[i,j] ← SampleNTT(XOF(ρ, i, j))
     end for
 end for
 for(i ← 0; i < k; i++)
     r[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(r,N))
     N ← N + 1
 end for
 for(i ← 0; i < k; i++)
     e₁[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
     N ← N + 1
 end for
 e₂ ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
 r̂ ← NTT(r)
 u ← NTT-¹(Âᵀ ◦ r̂) + e₁
 μ ← Decompress₁(ByteDecode₁(m)))
 v ← NTT-¹(t̂ᵀ ◦ rˆ) + e₂ + μ
 c₁ ← ByteEncode_{dᵤ}(Compress_{dᵤ}(u))
 c₂ ← ByteEncode_{dᵥ}(Compress_{dᵥ}(v))
 return c ← (c₁ ‖ c₂)
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt_unpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_LEN= 960
- C2_LEN= 128
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE Eurydice_arr_2b
libcrux_ml_kem_ind_cpa_encrypt_unpacked_d5(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
        *public_key,
    const Eurydice_arr_ec *message, Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_2b ciphertext = {.data = {0U}};
  tuple_5b uu____0 = libcrux_ml_kem_ind_cpa_encrypt_c1_87(
      randomness, &public_key->A,
      Eurydice_array_to_subslice_mut_d410(
          &ciphertext, (KRML_CLITERAL(core_ops_range_Range_87){
                           .start = (size_t)0U, .end = (size_t)960U})));
  Eurydice_arr_bb r_as_ntt = uu____0.fst;
  Eurydice_arr_9e error_2 = uu____0.snd;
  libcrux_ml_kem_ind_cpa_encrypt_c2_30(
      &public_key->t_as_ntt, &r_as_ntt, &error_2, message,
      Eurydice_array_to_subslice_from_mut_5f1(&ciphertext, (size_t)960U));
  return ciphertext;
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.encrypt
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
- CIPHERTEXT_SIZE= 1088
- T_AS_NTT_ENCODED_SIZE= 1152
- C1_LEN= 960
- C2_LEN= 128
- U_COMPRESSION_FACTOR= 10
- V_COMPRESSION_FACTOR= 4
- BLOCK_LEN= 320
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
- ETA2= 2
- ETA2_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE Eurydice_arr_2b libcrux_ml_kem_ind_cpa_encrypt_d5(
    Eurydice_borrow_slice_u8 public_key, const Eurydice_arr_ec *message,
    Eurydice_borrow_slice_u8 randomness) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
      unpacked_public_key =
          libcrux_ml_kem_ind_cpa_build_unpacked_public_key_05(public_key);
  return libcrux_ml_kem_ind_cpa_encrypt_unpacked_d5(&unpacked_public_key,
                                                    message, randomness);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for
libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.kdf_39
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- CIPHERTEXT_SIZE= 1088
*/
static KRML_MUSTINLINE Eurydice_arr_ec libcrux_ml_kem_variant_kdf_39_52(
    Eurydice_borrow_slice_u8 shared_secret, const Eurydice_arr_2b *_) {
  Eurydice_arr_ec out = {.data = {0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), shared_secret,
                      uint8_t);
  return out;
}

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
static KRML_MUSTINLINE Eurydice_arr_ec libcrux_ml_kem_ind_cca_decapsulate_fd(
    const Eurydice_arr_7d *private_key, const Eurydice_arr_2b *ciphertext) {
  Eurydice_borrow_slice_u8_x4 uu____0 =
      libcrux_ml_kem_types_unpack_private_key_64(
          Eurydice_array_to_slice_shared_51(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  Eurydice_arr_ec decrypted =
      libcrux_ml_kem_ind_cpa_decrypt_01(ind_cpa_secret_key, ciphertext);
  Eurydice_arr_c7 to_hash0 = libcrux_ml_kem_utils_into_padded_array_c9(
      Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_from_mut_5f(
          &to_hash0, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE),
      ind_cpa_public_key_hash, uint8_t);
  Eurydice_arr_c7 hashed = libcrux_ml_kem_hash_functions_portable_G_4a_78(
      Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2 uu____1 =
      Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
                              LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
                              uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret0 = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_af to_hash =
      libcrux_ml_kem_utils_into_padded_array_66(implicit_rejection_value);
  Eurydice_mut_borrow_slice_u8 uu____2 =
      Eurydice_array_to_subslice_from_mut_5f0(
          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_52(ciphertext),
                      uint8_t);
  Eurydice_arr_ec implicit_rejection_shared_secret =
      libcrux_ml_kem_hash_functions_portable_PRF_4a_3b(
          Eurydice_array_to_slice_shared_81(&to_hash));
  Eurydice_arr_2b expected_ciphertext = libcrux_ml_kem_ind_cpa_encrypt_d5(
      ind_cpa_public_key, &decrypted, pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 =
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret);
  Eurydice_arr_ec implicit_rejection_shared_secret0 =
      libcrux_ml_kem_variant_kdf_39_52(
          uu____3, libcrux_ml_kem_types_as_slice_a9_52(ciphertext));
  Eurydice_arr_ec shared_secret = libcrux_ml_kem_variant_kdf_39_52(
      shared_secret0, libcrux_ml_kem_types_as_slice_a9_52(ciphertext));
  Eurydice_borrow_slice_u8 uu____4 =
      libcrux_ml_kem_types_as_ref_c1_52(ciphertext);
  return libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(
      uu____4, Eurydice_array_to_slice_shared_06(&expected_ciphertext),
      Eurydice_array_to_slice_shared_01(&shared_secret),
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret0));
}

/**
 Portable decapsulate
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.decapsulate with const generics
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
    const Eurydice_arr_7d *private_key, const Eurydice_arr_2b *ciphertext) {
  return libcrux_ml_kem_ind_cca_decapsulate_fd(private_key, ciphertext);
}

/**
 Decapsulate ML-KEM 768

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem768PrivateKey`] and an
 [`MlKem768Ciphertext`].
*/
static inline Eurydice_arr_ec libcrux_ml_kem_mlkem768_portable_decapsulate(
    const Eurydice_arr_7d *private_key, const Eurydice_arr_2b *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_19(
      private_key, ciphertext);
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for
libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.entropy_preprocess_39
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_variant_entropy_preprocess_39_13(
    Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_ec out = {.data = {0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&out), randomness,
                      uint8_t);
  return out;
}

/**
This function found in impl {libcrux_ml_kem::hash_functions::Hash<K> for
libcrux_ml_kem::hash_functions::portable::PortableHash<K>}
*/
/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.H_4a
with const generics
- K= 3
*/
static inline Eurydice_arr_ec libcrux_ml_kem_hash_functions_portable_H_4a_78(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_H(input);
}

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
static KRML_MUSTINLINE tuple_f4 libcrux_ml_kem_ind_cca_encapsulate_99(
    const Eurydice_arr_5f *public_key, const Eurydice_arr_ec *randomness) {
  Eurydice_arr_ec randomness0 = libcrux_ml_kem_variant_entropy_preprocess_39_13(
      Eurydice_array_to_slice_shared_01(randomness));
  Eurydice_arr_c7 to_hash = libcrux_ml_kem_utils_into_padded_array_c9(
      Eurydice_array_to_slice_shared_01(&randomness0));
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_from_mut_5f(
      &to_hash, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE);
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec lvalue = libcrux_ml_kem_hash_functions_portable_H_4a_78(
      Eurydice_array_to_slice_shared_ff(
          libcrux_ml_kem_types_as_slice_e6_3d(public_key)));
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue),
                      uint8_t);
  Eurydice_arr_c7 hashed = libcrux_ml_kem_hash_functions_portable_G_4a_78(
      Eurydice_array_to_slice_shared_17(&to_hash));
  Eurydice_borrow_slice_u8_x2 uu____1 =
      Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
                              LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
                              uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_2b ciphertext = libcrux_ml_kem_ind_cpa_encrypt_d5(
      Eurydice_array_to_slice_shared_ff(
          libcrux_ml_kem_types_as_slice_e6_3d(public_key)),
      &randomness0, pseudorandomness);
  Eurydice_arr_2b uu____2 = libcrux_ml_kem_types_from_19_52(ciphertext);
  return (KRML_CLITERAL(tuple_f4){
      .fst = uu____2,
      .snd = libcrux_ml_kem_variant_kdf_39_52(shared_secret, &ciphertext)});
}

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.encapsulate with const generics
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
    const Eurydice_arr_5f *public_key, const Eurydice_arr_ec *randomness) {
  return libcrux_ml_kem_ind_cca_encapsulate_99(public_key, randomness);
}

/**
 Encapsulate ML-KEM 768

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
static inline tuple_f4 libcrux_ml_kem_mlkem768_portable_encapsulate(
    const Eurydice_arr_5f *public_key, Eurydice_arr_ec randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_26(
      public_key, &randomness);
}

/**
This function found in impl {core::default::Default for
libcrux_ml_kem::ind_cpa::unpacked::IndCpaPrivateKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.default_70
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static inline Eurydice_arr_bb libcrux_ml_kem_ind_cpa_unpacked_default_70_68(
    void) {
  Eurydice_arr_bb lit;
  Eurydice_arr_9e repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  }
  memcpy(lit.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_9e));
  return lit;
}

/**
This function found in impl {libcrux_ml_kem::variant::Variant for
libcrux_ml_kem::variant::MlKem}
*/
/**
A monomorphic instance of libcrux_ml_kem.variant.cpa_keygen_seed_39
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_c7
libcrux_ml_kem_variant_cpa_keygen_seed_39_13(
    Eurydice_borrow_slice_u8 key_generation_seed) {
  Eurydice_arr_fa seed = {.data = {0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d48(
          &seed,
          (KRML_CLITERAL(core_ops_range_Range_87){
              .start = (size_t)0U,
              .end =
                  LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE})),
      key_generation_seed, uint8_t);
  seed.data[LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE] =
      (uint8_t)(size_t)3U;
  return libcrux_ml_kem_hash_functions_portable_G_4a_78(
      Eurydice_array_to_slice_shared_b5(&seed));
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@3]> for
libcrux_ml_kem::ind_cpa::generate_keypair_unpacked::closure<Vector, Hasher,
Scheme, K, ETA1, ETA1_RANDOMNESS_SIZE>[TraitClause@0, TraitClause@1,
TraitClause@2, TraitClause@3, TraitClause@4, TraitClause@5]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.generate_keypair_unpacked.call_mut_73 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline Eurydice_arr_9e
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_39(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.to_standard_domain
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_ml_kem_polynomial_to_standard_domain_ea(Eurydice_arr_d6 vector) {
  return libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
      vector,
      LIBCRUX_ML_KEM_VECTOR_TRAITS_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_standard_error_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_standard_error_reduce_ea(
    Eurydice_arr_9e *myself, const Eurydice_arr_9e *error) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t j = i;
    Eurydice_arr_d6 coefficient_normal_form =
        libcrux_ml_kem_polynomial_to_standard_domain_ea(myself->data[j]);
    Eurydice_arr_d6 sum = libcrux_ml_kem_vector_portable_add_b8(
        coefficient_normal_form, &error->data[j]);
    Eurydice_arr_d6 red = libcrux_ml_kem_vector_portable_barrett_reduce_b8(sum);
    myself->data[j] = red;
  }
}

/**
This function found in impl
{libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_standard_error_reduce_d6
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_ea(
    Eurydice_arr_9e *self, const Eurydice_arr_9e *error) {
  libcrux_ml_kem_polynomial_add_standard_error_reduce_ea(self, error);
}

/**
 Compute Â ◦ ŝ + ê
*/
/**
A monomorphic instance of libcrux_ml_kem.matrix.compute_As_plus_e
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_compute_As_plus_e_68(
    Eurydice_arr_bb *t_as_ntt, const Eurydice_arr_1d *matrix_A,
    const Eurydice_arr_bb *s_as_ntt, const Eurydice_arr_bb *error_as_ntt) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    const Eurydice_arr_bb *row = &matrix_A->data[i0];
    Eurydice_arr_9e uu____0 = libcrux_ml_kem_polynomial_ZERO_d6_ea();
    t_as_ntt->data[i0] = uu____0;
    for (size_t i1 = (size_t)0U; i1 < (size_t)3U; i1++) {
      size_t j = i1;
      const Eurydice_arr_9e *matrix_element = &row->data[j];
      Eurydice_arr_9e product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
          matrix_element, &s_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_68(&t_as_ntt->data[i0],
                                                          &product);
    }
    libcrux_ml_kem_polynomial_add_standard_error_reduce_d6_ea(
        &t_as_ntt->data[i0], &error_as_ntt->data[i0]);
  }
}

/**
 This function implements most of <strong>Algorithm 12</strong> of the
 NIST FIPS 203 specification; this is the Kyber CPA-PKE key generation
 algorithm.

 We say "most of" since Algorithm 12 samples the required randomness within
 the function itself, whereas this implementation expects it to be provided
 through the `key_generation_seed` parameter.

 Algorithm 12 is reproduced below:

 ```plaintext
 Output: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
 Output: decryption key dkₚₖₑ ∈ 𝔹^{384k}.

 d ←$ B
 (ρ,σ) ← G(d)
 N ← 0
 for (i ← 0; i < k; i++)
     for(j ← 0; j < k; j++)
         Â[i,j] ← SampleNTT(XOF(ρ, i, j))
     end for
 end for
 for(i ← 0; i < k; i++)
     s[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
     N ← N + 1
 end for
 for(i ← 0; i < k; i++)
     e[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(σ,N))
     N ← N + 1
 end for
 ŝ ← NTT(s)
 ê ← NTT(e)
 t̂ ← Â◦ŝ + ê
 ekₚₖₑ ← ByteEncode₁₂(t̂) ‖ ρ
 dkₚₖₑ ← ByteEncode₁₂(ŝ)
 ```

 The NIST FIPS 203 standard can be found at
 <https://csrc.nist.gov/pubs/fips/203/ipd>.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair_unpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_39(
    Eurydice_borrow_slice_u8 key_generation_seed, Eurydice_arr_bb *private_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 *public_key) {
  Eurydice_arr_c7 hashed =
      libcrux_ml_kem_variant_cpa_keygen_seed_39_13(key_generation_seed);
  Eurydice_borrow_slice_u8_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_17(&hashed), (size_t)32U, uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_A = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_for_secret_and_error = uu____0.snd;
  Eurydice_arr_1d *uu____1 = &public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue0 =
      libcrux_ml_kem_utils_into_padded_array_de(seed_for_A);
  libcrux_ml_kem_matrix_sample_matrix_A_91(uu____1, &lvalue0, true);
  Eurydice_arr_fa prf_input =
      libcrux_ml_kem_utils_into_padded_array_29(seed_for_secret_and_error);
  uint8_t domain_separator =
      libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_bf(private_key,
                                                           &prf_input, 0U);
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_39(&lvalue,
                                                                        i);
  }
  Eurydice_arr_bb error_as_ntt = arr_struct;
  libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_bf(
      &error_as_ntt, &prf_input, domain_separator);
  libcrux_ml_kem_matrix_compute_As_plus_e_68(
      &public_key->t_as_ntt, &public_key->A, &private_key[0U], &error_as_ntt);
  Eurydice_arr_ec arr;
  memcpy(arr.data, seed_for_A.ptr, (size_t)32U * sizeof(uint8_t));
  Eurydice_arr_ec uu____2 = unwrap_26_39(
      (KRML_CLITERAL(Result_07){.tag = Ok, .val = {.case_Ok = arr}}));
  public_key->seed_for_A = uu____2;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.serialize_uncompressed_ring_element with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_b2
libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_ea(
    const Eurydice_arr_9e *re) {
  Eurydice_arr_b2 serialized = {.data = {0U}};
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_d6 coefficient =
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re->data[i0]);
    Eurydice_arr_94 bytes =
        libcrux_ml_kem_vector_portable_serialize_12_b8(coefficient);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_d411(
            &serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                             .start = (size_t)24U * i0,
                             .end = (size_t)24U * i0 + (size_t)24U})),
        Eurydice_array_to_slice_shared_ed(&bytes), uint8_t);
  }
  return serialized;
}

/**
 Call [`serialize_uncompressed_ring_element`] for each ring element.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_vector
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_serialize_vector_68(
    const Eurydice_arr_bb *key, Eurydice_mut_borrow_slice_u8 out) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_9e re = key->data[i0];
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (KRML_CLITERAL(core_ops_range_Range_87){
                 .start = i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                 .end = (i0 + (size_t)1U) *
                        LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT}));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b2 lvalue =
        libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_ea(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_a9(&lvalue),
                        uint8_t);
  }
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key_mut
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_serialize_public_key_mut_b6(
    const Eurydice_arr_bb *t_as_ntt, Eurydice_borrow_slice_u8 seed_for_a,
    Eurydice_arr_5f *serialized) {
  libcrux_ml_kem_ind_cpa_serialize_vector_68(
      t_as_ntt,
      Eurydice_array_to_subslice_mut_d412(
          serialized,
          (KRML_CLITERAL(core_ops_range_Range_87){
              .start = (size_t)0U,
              .end = libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)})));
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_from_mut_5f2(
          serialized,
          libcrux_ml_kem_constants_ranked_bytes_per_ring_element((size_t)3U)),
      seed_for_a, uint8_t);
}

/**
 Concatenate `t` and `ρ` into the public key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_public_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE Eurydice_arr_5f
libcrux_ml_kem_ind_cpa_serialize_public_key_b6(
    const Eurydice_arr_bb *t_as_ntt, Eurydice_borrow_slice_u8 seed_for_a) {
  Eurydice_arr_5f public_key_serialized = {.data = {0U}};
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_b6(t_as_ntt, seed_for_a,
                                                     &public_key_serialized);
  return public_key_serialized;
}

/**
 Serialize the secret key from the unpacked key pair generation.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.serialize_unpacked_secret_key
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- PRIVATE_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
static inline libcrux_ml_kem_utils_extraction_helper_Keypair768
libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_30(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
        *public_key,
    const Eurydice_arr_bb *private_key) {
  Eurydice_arr_5f public_key_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_b6(
          &public_key->t_as_ntt,
          Eurydice_array_to_slice_shared_01(&public_key->seed_for_A));
  Eurydice_arr_0e secret_key_serialized = {.data = {0U}};
  libcrux_ml_kem_ind_cpa_serialize_vector_68(
      private_key, Eurydice_array_to_slice_mut_f4(&secret_key_serialized));
  return (KRML_CLITERAL(libcrux_ml_kem_utils_extraction_helper_Keypair768){
      .fst = secret_key_serialized, .snd = public_key_serialized});
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.generate_keypair
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]],
libcrux_ml_kem_variant_MlKem with const generics
- K= 3
- PRIVATE_KEY_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static KRML_MUSTINLINE libcrux_ml_kem_utils_extraction_helper_Keypair768
libcrux_ml_kem_ind_cpa_generate_keypair_30(
    Eurydice_borrow_slice_u8 key_generation_seed) {
  Eurydice_arr_bb private_key = libcrux_ml_kem_ind_cpa_unpacked_default_70_68();
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 public_key =
      libcrux_ml_kem_ind_cpa_unpacked_default_8b_68();
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_39(
      key_generation_seed, &private_key, &public_key);
  return libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_30(&public_key,
                                                                 &private_key);
}

/**
 Serialize the secret key.
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key_mut
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_52(
    Eurydice_borrow_slice_u8 private_key, Eurydice_borrow_slice_u8 public_key,
    Eurydice_borrow_slice_u8 implicit_rejection_value,
    Eurydice_arr_7d *serialized) {
  size_t pointer = (size_t)0U;
  Eurydice_slice_copy(Eurydice_array_to_subslice_mut_d413(
                          serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                                          .start = pointer,
                                          .end = pointer + private_key.meta})),
                      private_key, uint8_t);
  pointer += private_key.meta;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d413(
          serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                          .start = pointer, .end = pointer + public_key.meta})),
      public_key, uint8_t);
  pointer += public_key.meta;
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_mut_d413(
      serialized,
      (KRML_CLITERAL(core_ops_range_Range_87){
          .start = pointer,
          .end = pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE}));
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_ec lvalue =
      libcrux_ml_kem_hash_functions_portable_H_4a_78(public_key);
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_01(&lvalue),
                      uint8_t);
  pointer += LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d413(
          serialized, (KRML_CLITERAL(core_ops_range_Range_87){
                          .start = pointer,
                          .end = pointer + implicit_rejection_value.meta})),
      implicit_rejection_value, uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
static KRML_MUSTINLINE Eurydice_arr_7d
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_52(
    Eurydice_borrow_slice_u8 private_key, Eurydice_borrow_slice_u8 public_key,
    Eurydice_borrow_slice_u8 implicit_rejection_value) {
  Eurydice_arr_7d out = {.data = {0U}};
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_52(
      private_key, public_key, implicit_rejection_value, &out);
  return out;
}

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
static KRML_MUSTINLINE libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_generate_keypair_b8(const Eurydice_arr_c7 *randomness) {
  Eurydice_borrow_slice_u8 ind_cpa_keypair_randomness =
      Eurydice_array_to_subslice_shared_d43(
          randomness,
          (KRML_CLITERAL(core_ops_range_Range_87){
              .start = (size_t)0U,
              .end =
                  LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE}));
  Eurydice_borrow_slice_u8 implicit_rejection_value =
      Eurydice_array_to_subslice_from_shared_5f0(
          randomness,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_utils_extraction_helper_Keypair768 uu____0 =
      libcrux_ml_kem_ind_cpa_generate_keypair_30(ind_cpa_keypair_randomness);
  Eurydice_arr_0e ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_5f public_key = uu____0.snd;
  Eurydice_arr_7d secret_key_serialized =
      libcrux_ml_kem_ind_cca_serialize_kem_secret_key_52(
          Eurydice_array_to_slice_shared_f4(&ind_cpa_private_key),
          Eurydice_array_to_slice_shared_ff(&public_key),
          implicit_rejection_value);
  Eurydice_arr_7d private_key =
      libcrux_ml_kem_types_from_b2_79(secret_key_serialized);
  return libcrux_ml_kem_types_from_17_bc(
      private_key, libcrux_ml_kem_types_from_51_3d(public_key));
}

/**
 Portable generate key pair.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.generate_keypair with const
generics
- K= 3
- CPA_PRIVATE_KEY_SIZE= 1152
- PRIVATE_KEY_SIZE= 2400
- PUBLIC_KEY_SIZE= 1184
- ETA1= 2
- ETA1_RANDOMNESS_SIZE= 128
*/
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_e9(
    const Eurydice_arr_c7 *randomness) {
  return libcrux_ml_kem_ind_cca_generate_keypair_b8(randomness);
}

/**
 Generate ML-KEM 768 Key Pair
*/
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_mlkem768_portable_generate_key_pair(Eurydice_arr_c7 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_e9(
      &randomness);
}

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
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_private_key_only_52(
    const Eurydice_arr_7d *private_key) {
  Eurydice_arr_ec t = libcrux_ml_kem_hash_functions_portable_H_4a_78(
      Eurydice_array_to_subslice_shared_d44(
          private_key, (KRML_CLITERAL(core_ops_range_Range_87){
                           .start = (size_t)384U * (size_t)3U,
                           .end = (size_t)768U * (size_t)3U + (size_t)32U})));
  Eurydice_borrow_slice_u8 expected = Eurydice_array_to_subslice_shared_d44(
      private_key, (KRML_CLITERAL(core_ops_range_Range_87){
                       .start = (size_t)768U * (size_t)3U + (size_t)32U,
                       .end = (size_t)768U * (size_t)3U + (size_t)64U}));
  return Eurydice_array_eq_slice_shared((size_t)32U, &t, &expected, uint8_t,
                                        bool);
}

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
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_private_key_ba(
    const Eurydice_arr_7d *private_key, const Eurydice_arr_2b *_ciphertext) {
  return libcrux_ml_kem_ind_cca_validate_private_key_only_52(private_key);
}

/**
 Private key validation
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key with const
generics
- K= 3
- SECRET_KEY_SIZE= 2400
- CIPHERTEXT_SIZE= 1088
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_d3(
    const Eurydice_arr_7d *private_key, const Eurydice_arr_2b *ciphertext) {
  return libcrux_ml_kem_ind_cca_validate_private_key_ba(private_key,
                                                        ciphertext);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool libcrux_ml_kem_mlkem768_portable_validate_private_key(
    const Eurydice_arr_7d *private_key, const Eurydice_arr_2b *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_d3(
      private_key, ciphertext);
}

/**
 Private key validation
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.validate_private_key_only with
const generics
- K= 3
- SECRET_KEY_SIZE= 2400
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_3b(
    const Eurydice_arr_7d *private_key) {
  return libcrux_ml_kem_ind_cca_validate_private_key_only_52(private_key);
}

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool libcrux_ml_kem_mlkem768_portable_validate_private_key_only(
    const Eurydice_arr_7d *private_key) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_3b(
      private_key);
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for
libcrux_ml_kem::serialize::deserialize_ring_elements_reduced_out::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out.call_mut_0b with
types libcrux_ml_kem_vector_portable_vector_type_PortableVector with const
generics
- K= 3
*/
static inline Eurydice_arr_9e
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_68(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
 This function deserializes ring elements and reduces the result by the field
 modulus.

 This function MUST NOT be used on secret inputs.
*/
/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_ring_elements_reduced_out with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_bb
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_68(
    Eurydice_borrow_slice_u8 public_key) {
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_68(
            &lvalue, i);
  }
  Eurydice_arr_bb deserialized_pk = arr_struct;
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_68(
      public_key, &deserialized_pk);
  return deserialized_pk;
}

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
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_public_key_b6(
    const Eurydice_arr_5f *public_key) {
  Eurydice_arr_bb deserialized_pk =
      libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_68(
          Eurydice_array_to_subslice_to_shared_210(
              public_key,
              libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)));
  Eurydice_arr_5f public_key_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_b6(
          &deserialized_pk,
          Eurydice_array_to_subslice_from_shared_5f1(
              public_key,
              libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)));
  return Eurydice_array_eq((size_t)1184U, public_key, &public_key_serialized,
                           uint8_t);
}

/**
 Public key validation
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.validate_public_key with const
generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE bool
libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_3b(
    const Eurydice_arr_5f *public_key) {
  return libcrux_ml_kem_ind_cca_validate_public_key_b6(public_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool libcrux_ml_kem_mlkem768_portable_validate_public_key(
    const Eurydice_arr_5f *public_key) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_3b(
      public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51_s {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 ind_cpa_public_key;
  Eurydice_arr_ec public_key_hash;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51;

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768PublicKeyUnpacked;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51_s {
  Eurydice_arr_bb ind_cpa_private_key;
  Eurydice_arr_ec implicit_rejection_value;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51;

typedef struct
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked_s {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 public_key;
} libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked;

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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_unpacked_decapsulate_0c(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2b *ciphertext) {
  Eurydice_arr_ec decrypted = libcrux_ml_kem_ind_cpa_decrypt_unpacked_01(
      &key_pair->private_key.ind_cpa_private_key, ciphertext);
  Eurydice_arr_c7 to_hash0 = libcrux_ml_kem_utils_into_padded_array_c9(
      Eurydice_array_to_slice_shared_01(&decrypted));
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_from_mut_5f(
      &to_hash0, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(
      uu____0,
      Eurydice_array_to_slice_shared_01(&key_pair->public_key.public_key_hash),
      uint8_t);
  Eurydice_arr_c7 hashed = libcrux_ml_kem_hash_functions_portable_G_4a_78(
      Eurydice_array_to_slice_shared_17(&to_hash0));
  Eurydice_borrow_slice_u8_x2 uu____1 =
      Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
                              LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
                              uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_af to_hash = libcrux_ml_kem_utils_into_padded_array_66(
      Eurydice_array_to_slice_shared_01(
          &key_pair->private_key.implicit_rejection_value));
  Eurydice_mut_borrow_slice_u8 uu____2 =
      Eurydice_array_to_subslice_from_mut_5f0(
          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_c1_52(ciphertext),
                      uint8_t);
  Eurydice_arr_ec implicit_rejection_shared_secret =
      libcrux_ml_kem_hash_functions_portable_PRF_4a_3b(
          Eurydice_array_to_slice_shared_81(&to_hash));
  Eurydice_arr_2b expected_ciphertext =
      libcrux_ml_kem_ind_cpa_encrypt_unpacked_d5(
          &key_pair->public_key.ind_cpa_public_key, &decrypted,
          pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 =
      libcrux_ml_kem_types_as_ref_c1_52(ciphertext);
  uint8_t selector =
      libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
          uu____3, Eurydice_array_to_slice_shared_06(&expected_ciphertext));
  return libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
      shared_secret,
      Eurydice_array_to_slice_shared_01(&implicit_rejection_shared_secret),
      selector);
}

/**
 Unpacked decapsulate
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.decapsulate with const
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
static KRML_MUSTINLINE Eurydice_arr_ec
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_19(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2b *ciphertext) {
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_0c(key_pair, ciphertext);
}

/**
 Decapsulate ML-KEM 768 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type
 [`MlKem768KeyPairUnpacked`] and an [`MlKem768Ciphertext`].
*/
static inline Eurydice_arr_ec
libcrux_ml_kem_mlkem768_portable_unpacked_decapsulate(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *private_key,
    const Eurydice_arr_2b *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_19(
      private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encaps_prepare
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
static inline Eurydice_arr_c7 libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_13(
    Eurydice_borrow_slice_u8 randomness, Eurydice_borrow_slice_u8 pk_hash) {
  Eurydice_arr_c7 to_hash =
      libcrux_ml_kem_utils_into_padded_array_c9(randomness);
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_5f(
                          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE),
                      pk_hash, uint8_t);
  return libcrux_ml_kem_hash_functions_portable_G_4a_78(
      Eurydice_array_to_slice_shared_17(&to_hash));
}

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
static KRML_MUSTINLINE tuple_f4 libcrux_ml_kem_ind_cca_unpacked_encapsulate_a7(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *public_key,
    const Eurydice_arr_ec *randomness) {
  Eurydice_arr_c7 hashed = libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_13(
      Eurydice_array_to_slice_shared_01(randomness),
      Eurydice_array_to_slice_shared_01(&public_key->public_key_hash));
  Eurydice_borrow_slice_u8_x2 uu____0 =
      Eurydice_slice_split_at(Eurydice_array_to_slice_shared_17(&hashed),
                              LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE,
                              uint8_t, Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____0.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____0.snd;
  Eurydice_arr_2b ciphertext = libcrux_ml_kem_ind_cpa_encrypt_unpacked_d5(
      &public_key->ind_cpa_public_key, randomness, pseudorandomness);
  Eurydice_arr_ec shared_secret_array = {.data = {0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(&shared_secret_array),
                      shared_secret, uint8_t);
  return (KRML_CLITERAL(tuple_f4){
      .fst = libcrux_ml_kem_types_from_19_52(ciphertext),
      .snd = shared_secret_array});
}

/**
 Unpacked encapsulate
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.encapsulate with const
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
static KRML_MUSTINLINE tuple_f4
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_26(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *public_key,
    const Eurydice_arr_ec *randomness) {
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_a7(public_key, randomness);
}

/**
 Encapsulate ML-KEM 768 (unpacked)

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type
 [`MlKem768PublicKeyUnpacked`], the SHA3-256 hash of this public key, and
 [`SHARED_SECRET_SIZE`] bytes of `randomness`.
*/
static inline tuple_f4 libcrux_ml_kem_mlkem768_portable_unpacked_encapsulate(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *public_key,
    Eurydice_arr_ec randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_26(
      public_key, &randomness);
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]> for
libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.transpose_a.closure.call_mut_b4 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
*/
static inline Eurydice_arr_9e
libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_68(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
[libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1]; K]> for
libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.transpose_a.call_mut_22 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
*/
static inline Eurydice_arr_bb
libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_68(void **_,
                                                           size_t tupled_args) {
  Eurydice_arr_bb arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_68(
            &lvalue, i);
  }
  return arr_struct;
}

/**
This function found in impl {core::clone::Clone for
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.polynomial.clone_c1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static inline Eurydice_arr_9e libcrux_ml_kem_polynomial_clone_c1_ea(
    const Eurydice_arr_9e *self) {
  return core_array__core__clone__Clone_for__T__N___clone(
      (size_t)16U, self, Eurydice_arr_d6, Eurydice_arr_9e);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static inline Eurydice_arr_1d libcrux_ml_kem_ind_cca_unpacked_transpose_a_68(
    Eurydice_arr_1d ind_cpa_a) {
  Eurydice_arr_1d arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_22_68(&lvalue, i);
  }
  Eurydice_arr_1d A = arr_struct;
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      Eurydice_arr_9e uu____0 =
          libcrux_ml_kem_polynomial_clone_c1_ea(&ind_cpa_a.data[j].data[i1]);
      A.data[i1].data[j] = uu____0;
    }
  }
  return A;
}

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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cca_unpacked_generate_keypair_b8(
    Eurydice_arr_c7 randomness,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *out) {
  Eurydice_borrow_slice_u8 ind_cpa_keypair_randomness =
      Eurydice_array_to_subslice_shared_d43(
          &randomness,
          (KRML_CLITERAL(core_ops_range_Range_87){
              .start = (size_t)0U,
              .end =
                  LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE}));
  Eurydice_borrow_slice_u8 implicit_rejection_value =
      Eurydice_array_to_subslice_from_shared_5f0(
          &randomness,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_39(
      ind_cpa_keypair_randomness, &out->private_key.ind_cpa_private_key,
      &out->public_key.ind_cpa_public_key);
  Eurydice_arr_1d A = libcrux_ml_kem_ind_cca_unpacked_transpose_a_68(
      out->public_key.ind_cpa_public_key.A);
  out->public_key.ind_cpa_public_key.A = A;
  Eurydice_arr_5f pk_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_b6(
          &out->public_key.ind_cpa_public_key.t_as_ntt,
          Eurydice_array_to_slice_shared_01(
              &out->public_key.ind_cpa_public_key.seed_for_A));
  Eurydice_arr_ec uu____0 = libcrux_ml_kem_hash_functions_portable_H_4a_78(
      Eurydice_array_to_slice_shared_ff(&pk_serialized));
  out->public_key.public_key_hash = uu____0;
  Eurydice_arr_ec arr;
  memcpy(arr.data, implicit_rejection_value.ptr, (size_t)32U * sizeof(uint8_t));
  Eurydice_arr_ec uu____1 = unwrap_26_39(
      (KRML_CLITERAL(Result_07){.tag = Ok, .val = {.case_Ok = arr}}));
  out->private_key.implicit_rejection_value = uu____1;
}

/**
 Generate a key pair
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.generate_keypair with
const generics
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
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *out) {
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_b8(randomness, out);
}

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(
    Eurydice_arr_c7 randomness,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_e9(
      randomness, key_pair);
}

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
static KRML_MUSTINLINE libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
libcrux_ml_kem_ind_cca_unpacked_default_30_68(void) {
  return (
      KRML_CLITERAL(libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51){
          .ind_cpa_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_68(),
          .public_key_hash = {.data = {0U}}});
}

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
static KRML_MUSTINLINE
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
    libcrux_ml_kem_ind_cca_unpacked_default_7b_68(void) {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_51 uu____0 = {
      .ind_cpa_private_key = libcrux_ml_kem_ind_cpa_unpacked_default_70_68(),
      .implicit_rejection_value = {.data = {0U}}};
  return (KRML_CLITERAL(
      libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked){
      .private_key = uu____0,
      .public_key = libcrux_ml_kem_ind_cca_unpacked_default_30_68()});
}

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
static inline libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair(
    Eurydice_arr_c7 randomness) {
  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_ind_cca_unpacked_default_7b_68();
  libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(randomness,
                                                                  &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
static inline libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_portable_unpacked_init_key_pair(void) {
  return libcrux_ml_kem_ind_cca_unpacked_default_7b_68();
}

/**
 Create a new, empty unpacked public key.
*/
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
libcrux_ml_kem_mlkem768_portable_unpacked_init_public_key(void) {
  return libcrux_ml_kem_ind_cca_unpacked_default_30_68();
}

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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_01(
    const Eurydice_arr_7d *private_key,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  Eurydice_borrow_slice_u8_x4 uu____0 =
      libcrux_ml_kem_types_unpack_private_key_64(
          Eurydice_array_to_slice_shared_51(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  libcrux_ml_kem_ind_cpa_deserialize_vector_68(
      ind_cpa_secret_key, &key_pair->private_key.ind_cpa_private_key);
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_05(
      ind_cpa_public_key, &key_pair->public_key.ind_cpa_public_key);
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_01(&key_pair->public_key.public_key_hash),
      ind_cpa_public_key_hash, uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_01(
                          &key_pair->private_key.implicit_rejection_value),
                      implicit_rejection_value, uint8_t);
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_01(
          &key_pair->public_key.ind_cpa_public_key.seed_for_A),
      Eurydice_slice_subslice_from_shared_6d(ind_cpa_public_key, (size_t)1152U),
      uint8_t);
}

/**
 Take a serialized private key and generate an unpacked key pair from it.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.keypair_from_private_key
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
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_01(private_key,
                                                           key_pair);
}

/**
 Get an unpacked key from a private key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_from_private_mut(
    const Eurydice_arr_7d *private_key,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_71(
      private_key, key_pair);
}

/**
 Get the serialized private key.
*/
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_21(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self,
    Eurydice_arr_7d *serialized) {
  libcrux_ml_kem_utils_extraction_helper_Keypair768 uu____0 =
      libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_30(
          &self->public_key.ind_cpa_public_key,
          &self->private_key.ind_cpa_private_key);
  Eurydice_arr_0e ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_5f ind_cpa_public_key = uu____0.snd;
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_52(
      Eurydice_array_to_slice_shared_f4(&ind_cpa_private_key),
      Eurydice_array_to_slice_shared_ff(&ind_cpa_public_key),
      Eurydice_array_to_slice_shared_01(
          &self->private_key.implicit_rejection_value),
      serialized);
}

/**
 Get the serialized private key.
*/
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
static KRML_MUSTINLINE Eurydice_arr_7d
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_21(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self) {
  Eurydice_arr_7d sk = libcrux_ml_kem_types_default_d3_79();
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_21(self, &sk);
  return sk;
}

/**
 Get the serialized private key.
*/
static inline Eurydice_arr_7d
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_private_key(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_21(key_pair);
}

/**
 Get the serialized private key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_private_key_mut(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    Eurydice_arr_7d *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_21(key_pair,
                                                                   serialized);
}

/**
 Get the serialized public key.
*/
/**
This function found in impl
{libcrux_ml_kem::ind_cca::unpacked::MlKemPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.serialized_dd
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE Eurydice_arr_5f
libcrux_ml_kem_ind_cca_unpacked_serialized_dd_b6(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *self) {
  return libcrux_ml_kem_types_from_51_3d(
      libcrux_ml_kem_ind_cpa_serialize_public_key_b6(
          &self->ind_cpa_public_key.t_as_ntt,
          Eurydice_array_to_slice_shared_01(
              &self->ind_cpa_public_key.seed_for_A)));
}

/**
 Get the serialized public key.
*/
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
static KRML_MUSTINLINE Eurydice_arr_5f
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_b6(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_dd_b6(&self->public_key);
}

/**
 Get the serialized public key.
*/
static inline Eurydice_arr_5f
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_public_key(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_b6(key_pair);
}

/**
 Get the serialized public key.
*/
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_b6(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *self,
    Eurydice_arr_5f *serialized) {
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_b6(
      &self->ind_cpa_public_key.t_as_ntt,
      Eurydice_array_to_slice_shared_01(&self->ind_cpa_public_key.seed_for_A),
      serialized);
}

/**
 Get the serialized public key.
*/
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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_b6(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self,
    Eurydice_arr_5f *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_b6(&self->public_key,
                                                       serialized);
}

/**
 Get the serialized public key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_public_key_mut(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    Eurydice_arr_5f *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_b6(key_pair,
                                                                  serialized);
}

/**
This function found in impl {core::clone::Clone for
libcrux_ml_kem::ind_cpa::unpacked::IndCpaPublicKeyUnpacked<Vector,
K>[TraitClause@0, TraitClause@2]}
*/
/**
A monomorphic instance of libcrux_ml_kem.ind_cpa.unpacked.clone_91
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51
libcrux_ml_kem_ind_cpa_unpacked_clone_91_68(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 *self) {
  Eurydice_arr_bb uu____0 = core_array__core__clone__Clone_for__T__N___clone(
      (size_t)3U, &self->t_as_ntt, Eurydice_arr_9e, Eurydice_arr_bb);
  Eurydice_arr_ec uu____1 = core_array__core__clone__Clone_for__T__N___clone(
      (size_t)32U, &self->seed_for_A, uint8_t, Eurydice_arr_ec);
  return (
      KRML_CLITERAL(libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51){
          .t_as_ntt = uu____0,
          .seed_for_A = uu____1,
          .A = core_array__core__clone__Clone_for__T__N___clone(
              (size_t)3U, &self->A, Eurydice_arr_bb, Eurydice_arr_1d)});
}

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
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
libcrux_ml_kem_ind_cca_unpacked_clone_d7_68(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *self) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_51 uu____0 =
      libcrux_ml_kem_ind_cpa_unpacked_clone_91_68(&self->ind_cpa_public_key);
  return (
      KRML_CLITERAL(libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51){
          .ind_cpa_public_key = uu____0,
          .public_key_hash = core_array__core__clone__Clone_for__T__N___clone(
              (size_t)32U, &self->public_key_hash, uint8_t, Eurydice_arr_ec)});
}

/**
 Get the serialized public key.
*/
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
static KRML_MUSTINLINE const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *
libcrux_ml_kem_ind_cca_unpacked_public_key_11_68(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self) {
  return &self->public_key;
}

/**
 Get the unpacked public key.
*/
static inline void libcrux_ml_kem_mlkem768_portable_unpacked_public_key(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *pk) {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 uu____0 =
      libcrux_ml_kem_ind_cca_unpacked_clone_d7_68(
          libcrux_ml_kem_ind_cca_unpacked_public_key_11_68(key_pair));
  pk[0U] = uu____0;
}

/**
 Get the serialized public key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_serialized_public_key(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51 *public_key,
    Eurydice_arr_5f *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_b6(public_key, serialized);
}

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
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_22(
    const Eurydice_arr_5f *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
        *unpacked_public_key) {
  Eurydice_borrow_slice_u8 uu____0 =
      Eurydice_array_to_subslice_to_shared_210(public_key, (size_t)1152U);
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_68(
      uu____0, &unpacked_public_key->ind_cpa_public_key.t_as_ntt);
  unpacked_public_key->ind_cpa_public_key.seed_for_A =
      libcrux_ml_kem_utils_into_padded_array_ce(
          Eurydice_array_to_subslice_from_shared_5f1(public_key,
                                                     (size_t)1152U));
  Eurydice_arr_1d *uu____2 = &unpacked_public_key->ind_cpa_public_key.A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_31 lvalue = libcrux_ml_kem_utils_into_padded_array_de(
      Eurydice_array_to_subslice_from_shared_5f1(public_key, (size_t)1152U));
  libcrux_ml_kem_matrix_sample_matrix_A_91(uu____2, &lvalue, false);
  Eurydice_arr_ec uu____3 = libcrux_ml_kem_hash_functions_portable_H_4a_78(
      Eurydice_array_to_slice_shared_ff(
          libcrux_ml_kem_types_as_slice_e6_3d(public_key)));
  unpacked_public_key->public_key_hash = uu____3;
}

/**
 Get the unpacked public key.
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.instantiations.portable.unpacked.unpack_public_key with
const generics
- K= 3
- T_AS_NTT_ENCODED_SIZE= 1152
- PUBLIC_KEY_SIZE= 1184
*/
static KRML_MUSTINLINE void
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_d3(
    const Eurydice_arr_5f *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_22(public_key,
                                                       unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_unpacked_public_key(
    const Eurydice_arr_5f *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_51
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_d3(
      public_key, unpacked_public_key);
}

#if defined(__cplusplus)
}
#endif

#define libcrux_mlkem768_portable_H_DEFINED
#endif /* libcrux_mlkem768_portable_H */
