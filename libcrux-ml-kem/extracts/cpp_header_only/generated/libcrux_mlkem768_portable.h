/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 146b7dce58cb11ca8010b1c947c3437a959dcd88
 * Eurydice: cdf02f9d8ed0d73f88c0a495c5b79359a51398fc
 * Karamel: 8e7262955105599e91f3a99c9ab3d3387f7046f2
 * F*: 89901492c020c74b82d811d27f3149c222d9b8b5
 * Libcrux: 4faeb5fdd7bdf29c1f30136c8f32fe24c06cfab0
 */

#ifndef libcrux_mlkem768_portable_H
#define libcrux_mlkem768_portable_H

#include "eurydice_glue.h"
#include "libcrux_ct_ops.h"
#include "libcrux_mlkem_core.h"
#include "libcrux_sha3_portable.h"

static inline Eurydice_arr_06 libcrux_ml_kem_hash_functions_portable_G(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_06 digest = {{0U}};
  libcrux_sha3_portable_sha512(Eurydice_array_to_slice_mut_d8(&digest), input);
  return digest;
}

static inline Eurydice_arr_600 libcrux_ml_kem_hash_functions_portable_H(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_600 digest = {{0U}};
  libcrux_sha3_portable_sha256(Eurydice_array_to_slice_mut_6e(&digest), input);
  return digest;
}

#define LIBCRUX_ML_KEM_POLYNOMIAL_ZETAS_TIMES_MONTGOMERY_R                     \
  ((Eurydice_arr_49{                                                           \
      {(int16_t) - 1044, (int16_t) - 758,  (int16_t) - 359,  (int16_t) - 1517, \
       (int16_t)1493,    (int16_t)1422,    (int16_t)287,     (int16_t)202,     \
       (int16_t) - 171,  (int16_t)622,     (int16_t)1577,    (int16_t)182,     \
       (int16_t)962,     (int16_t) - 1202, (int16_t) - 1474, (int16_t)1468,    \
       (int16_t)573,     (int16_t) - 1325, (int16_t)264,     (int16_t)383,     \
       (int16_t) - 829,  (int16_t)1458,    (int16_t) - 1602, (int16_t) - 130,  \
       (int16_t) - 681,  (int16_t)1017,    (int16_t)732,     (int16_t)608,     \
       (int16_t) - 1542, (int16_t)411,     (int16_t) - 205,  (int16_t) - 1571, \
       (int16_t)1223,    (int16_t)652,     (int16_t) - 552,  (int16_t)1015,    \
       (int16_t) - 1293, (int16_t)1491,    (int16_t) - 282,  (int16_t) - 1544, \
       (int16_t)516,     (int16_t) - 8,    (int16_t) - 320,  (int16_t) - 666,  \
       (int16_t) - 1618, (int16_t) - 1162, (int16_t)126,     (int16_t)1469,    \
       (int16_t) - 853,  (int16_t) - 90,   (int16_t) - 271,  (int16_t)830,     \
       (int16_t)107,     (int16_t) - 1421, (int16_t) - 247,  (int16_t) - 951,  \
       (int16_t) - 398,  (int16_t)961,     (int16_t) - 1508, (int16_t) - 725,  \
       (int16_t)448,     (int16_t) - 1065, (int16_t)677,     (int16_t) - 1275, \
       (int16_t) - 1103, (int16_t)430,     (int16_t)555,     (int16_t)843,     \
       (int16_t) - 1251, (int16_t)871,     (int16_t)1550,    (int16_t)105,     \
       (int16_t)422,     (int16_t)587,     (int16_t)177,     (int16_t) - 235,  \
       (int16_t) - 291,  (int16_t) - 460,  (int16_t)1574,    (int16_t)1653,    \
       (int16_t) - 246,  (int16_t)778,     (int16_t)1159,    (int16_t) - 147,  \
       (int16_t) - 777,  (int16_t)1483,    (int16_t) - 602,  (int16_t)1119,    \
       (int16_t) - 1590, (int16_t)644,     (int16_t) - 872,  (int16_t)349,     \
       (int16_t)418,     (int16_t)329,     (int16_t) - 156,  (int16_t) - 75,   \
       (int16_t)817,     (int16_t)1097,    (int16_t)603,     (int16_t)610,     \
       (int16_t)1322,    (int16_t) - 1285, (int16_t) - 1465, (int16_t)384,     \
       (int16_t) - 1215, (int16_t) - 136,  (int16_t)1218,    (int16_t) - 1335, \
       (int16_t) - 874,  (int16_t)220,     (int16_t) - 1187, (int16_t) - 1659, \
       (int16_t) - 1185, (int16_t) - 1530, (int16_t) - 1278, (int16_t)794,     \
       (int16_t) - 1510, (int16_t) - 854,  (int16_t) - 870,  (int16_t)478,     \
       (int16_t) - 108,  (int16_t) - 308,  (int16_t)996,     (int16_t)991,     \
       (int16_t)958,     (int16_t) - 1460, (int16_t)1522,    (int16_t)1628}}))

static KRML_MUSTINLINE int16_t libcrux_ml_kem_polynomial_zeta(size_t i) {
  return LIBCRUX_ML_KEM_POLYNOMIAL_ZETAS_TIMES_MONTGOMERY_R.data[i];
}

#define LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR ((size_t)16U)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS \
  ((int16_t)1353)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS ((int16_t)3329)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R \
  (62209U)

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_vector_type_from_i16_array(
    Eurydice_borrow_slice_i16 array) {
  Eurydice_arr_e2 arr;
  memcpy(arr.data,
         Eurydice_slice_subslice_shared_76(
             array, (core_ops_range_Range_08{(size_t)0U, (size_t)16U}))
             .ptr,
         (size_t)16U * sizeof(int16_t));
  return unwrap_26_0e(Result_20_s(Ok, &Result_20_s::U::case_Ok, arr));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_from_i16_array_b8(
    Eurydice_borrow_slice_i16 array) {
  return libcrux_ml_kem_vector_portable_vector_type_from_i16_array(
      libcrux_secrets_int_classify_public_classify_ref_9b_39(array));
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

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_vector_type_zero(void) {
  return libcrux_secrets_int_public_integers_classify_27_3a(
      (Eurydice_arr_e2{{0U}}));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_ZERO_b8(void) {
  return libcrux_ml_kem_vector_portable_vector_type_zero();
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_add(Eurydice_arr_e2 lhs,
                                              const Eurydice_arr_e2 *rhs) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs.data[uu____0] = lhs.data[uu____0] + rhs->data[i0];
  }
  return lhs;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_add_b8(
    Eurydice_arr_e2 lhs, const Eurydice_arr_e2 *rhs) {
  return libcrux_ml_kem_vector_portable_arithmetic_add(lhs, rhs);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_sub(Eurydice_arr_e2 lhs,
                                              const Eurydice_arr_e2 *rhs) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    lhs.data[uu____0] = lhs.data[uu____0] - rhs->data[i0];
  }
  return lhs;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_sub_b8(
    Eurydice_arr_e2 lhs, const Eurydice_arr_e2 *rhs) {
  return libcrux_ml_kem_vector_portable_arithmetic_sub(lhs, rhs);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_multiply_by_constant(
    Eurydice_arr_e2 vec, int16_t c) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    vec.data[uu____0] = vec.data[uu____0] * c;
  }
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_multiply_by_constant_b8(Eurydice_arr_e2 vec,
                                                       int16_t c) {
  return libcrux_ml_kem_vector_portable_arithmetic_multiply_by_constant(vec, c);
}

/**
 Note: This function is not secret independent
 Only use with public values.
*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_cond_subtract_3329(
    Eurydice_arr_e2 vec) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    if (libcrux_secrets_int_public_integers_declassify_d8_39(vec.data[i0]) >=
        (int16_t)3329) {
      size_t uu____0 = i0;
      vec.data[uu____0] = vec.data[uu____0] - (int16_t)3329;
    }
  }
  return vec;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_cond_subtract_3329_b8(Eurydice_arr_e2 v) {
  return libcrux_ml_kem_vector_portable_arithmetic_cond_subtract_3329(v);
}

#define LIBCRUX_ML_KEM_VECTOR_PORTABLE_ARITHMETIC_BARRETT_MULTIPLIER \
  ((int32_t)20159)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT ((int32_t)26)

#define LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_R \
  ((int32_t)1 << (uint32_t)LIBCRUX_ML_KEM_VECTOR_TRAITS_BARRETT_SHIFT)

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

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_barrett_reduce(Eurydice_arr_e2 vec) {
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
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_barrett_reduce_b8(
    Eurydice_arr_e2 vector) {
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

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_by_constant(
    Eurydice_arr_e2 vec, int16_t c) {
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
    Eurydice_arr_e2 vector, int16_t constant) {
  return libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_by_constant(
      vector, libcrux_secrets_int_public_integers_classify_27_39(constant));
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(
    Eurydice_arr_e2 vec, int16_t c) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    size_t uu____0 = i0;
    vec.data[uu____0] = vec.data[uu____0] & c;
  }
  return vec;
}

/**
A monomorphic instance of libcrux_ml_kem.vector.portable.arithmetic.shift_right
with const generics
- SHIFT_BY= 15
*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_shift_right_ef(Eurydice_arr_e2 vec) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    vec.data[i0] = vec.data[i0] >> (uint32_t)(int32_t)15;
  }
  return vec;
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_arithmetic_to_unsigned_representative(
    Eurydice_arr_e2 a) {
  Eurydice_arr_e2 t =
      libcrux_ml_kem_vector_portable_arithmetic_shift_right_ef(a);
  Eurydice_arr_e2 fm =
      libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(
          t, LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS);
  return libcrux_ml_kem_vector_portable_arithmetic_add(a, &fm);
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_to_unsigned_representative_b8(
    Eurydice_arr_e2 a) {
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
  int16_t shifted =
      libcrux_secrets_int_public_integers_classify_27_39((int16_t)1664) -
      libcrux_secrets_int_as_i16_ca(fe);
  int16_t mask = shifted >> 15U;
  int16_t shifted_to_positive = mask ^ shifted;
  int16_t shifted_positive_in_range = shifted_to_positive - (int16_t)832;
  int16_t r0 = shifted_positive_in_range >> 15U;
  int16_t r1 = r0 & (int16_t)1;
  return libcrux_secrets_int_as_u8_f5(r1);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_compress_compress_1(Eurydice_arr_e2 a) {
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
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_compress_1_b8(
    Eurydice_arr_e2 a) {
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
  compressed = compressed + 1664ULL;
  compressed = compressed * 10321340ULL;
  compressed = compressed >> 35U;
  return libcrux_secrets_int_as_i16_b8(
      libcrux_ml_kem_vector_portable_arithmetic_get_n_least_significant_bits(
          coefficient_bits, libcrux_secrets_int_as_u32_a3(compressed)));
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_compress_decompress_1(Eurydice_arr_e2 a) {
  Eurydice_arr_e2 z = libcrux_ml_kem_vector_portable_vector_type_zero();
  Eurydice_arr_e2 s = libcrux_ml_kem_vector_portable_arithmetic_sub(z, &a);
  Eurydice_arr_e2 res =
      libcrux_ml_kem_vector_portable_arithmetic_bitwise_and_with_constant(
          s, (int16_t)1665);
  return res;
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_decompress_1_b8(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_compress_decompress_1(a);
}

static KRML_MUSTINLINE void libcrux_ml_kem_vector_portable_ntt_ntt_step(
    Eurydice_arr_e2 *vec, int16_t zeta, size_t i, size_t j) {
  int16_t t =
      libcrux_ml_kem_vector_portable_arithmetic_montgomery_multiply_fe_by_fer(
          vec->data[j],
          libcrux_secrets_int_public_integers_classify_27_39(zeta));
  int16_t a_minus_t = vec->data[i] - t;
  int16_t a_plus_t = vec->data[i] + t;
  vec->data[j] = a_minus_t;
  vec->data[i] = a_plus_t;
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_ntt_layer_1_step(Eurydice_arr_e2 vec,
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_layer_1_step_b8(Eurydice_arr_e2 a,
                                                   int16_t zeta0, int16_t zeta1,
                                                   int16_t zeta2,
                                                   int16_t zeta3) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_layer_1_step(a, zeta0, zeta1,
                                                             zeta2, zeta3);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_ntt_layer_2_step(Eurydice_arr_e2 vec,
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_layer_2_step_b8(Eurydice_arr_e2 a,
                                                   int16_t zeta0,
                                                   int16_t zeta1) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_layer_2_step(a, zeta0, zeta1);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_ntt_layer_3_step(Eurydice_arr_e2 vec,
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_layer_3_step_b8(Eurydice_arr_e2 a,
                                                   int16_t zeta) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_layer_3_step(a, zeta);
}

static KRML_MUSTINLINE void libcrux_ml_kem_vector_portable_ntt_inv_ntt_step(
    Eurydice_arr_e2 *vec, int16_t zeta, size_t i, size_t j) {
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

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_1_step(Eurydice_arr_e2 vec,
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_inv_ntt_layer_1_step_b8(Eurydice_arr_e2 a,
                                                       int16_t zeta0,
                                                       int16_t zeta1,
                                                       int16_t zeta2,
                                                       int16_t zeta3) {
  return libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_1_step(
      a, zeta0, zeta1, zeta2, zeta3);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_2_step(Eurydice_arr_e2 vec,
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_inv_ntt_layer_2_step_b8(Eurydice_arr_e2 a,
                                                       int16_t zeta0,
                                                       int16_t zeta1) {
  return libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_2_step(a, zeta0,
                                                                 zeta1);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_inv_ntt_layer_3_step(Eurydice_arr_e2 vec,
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_inv_ntt_layer_3_step_b8(Eurydice_arr_e2 a,
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
    const Eurydice_arr_e2 *a, const Eurydice_arr_e2 *b, int16_t zeta, size_t i,
    Eurydice_arr_e2 *out) {
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

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_ntt_ntt_multiply(const Eurydice_arr_e2 *lhs,
                                                const Eurydice_arr_e2 *rhs,
                                                int16_t zeta0, int16_t zeta1,
                                                int16_t zeta2, int16_t zeta3) {
  int16_t nzeta0 = -zeta0;
  int16_t nzeta1 = -zeta1;
  int16_t nzeta2 = -zeta2;
  int16_t nzeta3 = -zeta3;
  Eurydice_arr_e2 out = libcrux_ml_kem_vector_portable_vector_type_zero();
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
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_ntt_multiply_b8(
    const Eurydice_arr_e2 *lhs, const Eurydice_arr_e2 *rhs, int16_t zeta0,
    int16_t zeta1, int16_t zeta2, int16_t zeta3) {
  return libcrux_ml_kem_vector_portable_ntt_ntt_multiply(lhs, rhs, zeta0, zeta1,
                                                         zeta2, zeta3);
}

static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_ml_kem_vector_portable_serialize_serialize_1(Eurydice_arr_e2 v) {
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
  return (Eurydice_array_u8x2{{result0, result1}});
}

static inline Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_1(
    Eurydice_arr_e2 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_ee(
      libcrux_ml_kem_vector_portable_serialize_serialize_1(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_array_u8x2 libcrux_ml_kem_vector_portable_serialize_1_b8(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_serialize_1(a);
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_serialize_deserialize_1(
    Eurydice_borrow_slice_u8 v) {
  int16_t result0 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) & 1U);
  int16_t result1 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 1U & 1U);
  int16_t result2 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 2U & 1U);
  int16_t result3 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 3U & 1U);
  int16_t result4 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 4U & 1U);
  int16_t result5 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 5U & 1U);
  int16_t result6 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 6U & 1U);
  int16_t result7 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)0U, uint8_t) >> 7U & 1U);
  int16_t result8 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) & 1U);
  int16_t result9 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 1U & 1U);
  int16_t result10 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 2U & 1U);
  int16_t result11 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 3U & 1U);
  int16_t result12 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 4U & 1U);
  int16_t result13 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 5U & 1U);
  int16_t result14 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 6U & 1U);
  int16_t result15 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(v, (size_t)1U, uint8_t) >> 7U & 1U);
  return (Eurydice_arr_e2{{result0, result1, result2, result3, result4, result5,
                           result6, result7, result8, result9, result10,
                           result11, result12, result13, result14, result15}});
}

static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_1(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_1(
      libcrux_secrets_int_classify_public_classify_ref_9b_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_1_b8(
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
  uint8_t result0 = (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)1U, int16_t))
                        << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)0U, int16_t));
  uint8_t result1 = (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)3U, int16_t))
                        << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)2U, int16_t));
  uint8_t result2 = (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)5U, int16_t))
                        << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)4U, int16_t));
  uint8_t result3 = (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)7U, int16_t))
                        << 4U |
                    (uint32_t)libcrux_secrets_int_as_u8_f5(
                        Eurydice_slice_index_shared(v, (size_t)6U, int16_t));
  return (uint8_t_x4{result0, result1, result2, result3});
}

static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_ml_kem_vector_portable_serialize_serialize_4(Eurydice_arr_e2 v) {
  uint8_t_x4 result0_3 =
      libcrux_ml_kem_vector_portable_serialize_serialize_4_int(
          Eurydice_array_to_subslice_shared_85(
              &v, (core_ops_range_Range_08{(size_t)0U, (size_t)8U})));
  uint8_t_x4 result4_7 =
      libcrux_ml_kem_vector_portable_serialize_serialize_4_int(
          Eurydice_array_to_subslice_shared_85(
              &v, (core_ops_range_Range_08{(size_t)8U, (size_t)16U})));
  return (Eurydice_array_u8x8{{result0_3.fst, result0_3.snd, result0_3.thd,
                               result0_3.f3, result4_7.fst, result4_7.snd,
                               result4_7.thd, result4_7.f3}});
}

static inline Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_4(
    Eurydice_arr_e2 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_36(
      libcrux_ml_kem_vector_portable_serialize_serialize_4(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_array_u8x8 libcrux_ml_kem_vector_portable_serialize_4_b8(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_serialize_4(a);
}

static KRML_MUSTINLINE int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t v0 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t) & 15U);
  int16_t v1 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t) >> 4U &
      15U);
  int16_t v2 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t) & 15U);
  int16_t v3 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t) >> 4U &
      15U);
  int16_t v4 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t) & 15U);
  int16_t v5 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t) >> 4U &
      15U);
  int16_t v6 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t) & 15U);
  int16_t v7 = libcrux_secrets_int_as_i16_59(
      (uint32_t)Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t) >> 4U &
      15U);
  return (int16_t_x8{v0, v1, v2, v3, v4, v5, v6, v7});
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_serialize_deserialize_4(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t_x8 v0_7 = libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)0U, (size_t)4U})));
  int16_t_x8 v8_15 = libcrux_ml_kem_vector_portable_serialize_deserialize_4_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)4U, (size_t)8U})));
  return (
      Eurydice_arr_e2{{v0_7.fst, v0_7.snd, v0_7.thd, v0_7.f3, v0_7.f4, v0_7.f5,
                       v0_7.f6, v0_7.f7, v8_15.fst, v8_15.snd, v8_15.thd,
                       v8_15.f3, v8_15.f4, v8_15.f5, v8_15.f6, v8_15.f7}});
}

static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_4(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_4(
      libcrux_secrets_int_classify_public_classify_ref_9b_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_4_b8(
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
  uint8_t r0 = libcrux_secrets_int_as_u8_f5(
      Eurydice_slice_index_shared(v, (size_t)0U, int16_t) & (int16_t)255);
  uint8_t r1 =
      (uint32_t)libcrux_secrets_int_as_u8_f5(
          Eurydice_slice_index_shared(v, (size_t)1U, int16_t) & (int16_t)63)
          << 2U |
      (uint32_t)libcrux_secrets_int_as_u8_f5(
          Eurydice_slice_index_shared(v, (size_t)0U, int16_t) >> 8U &
          (int16_t)3);
  uint8_t r2 =
      (uint32_t)libcrux_secrets_int_as_u8_f5(
          Eurydice_slice_index_shared(v, (size_t)2U, int16_t) & (int16_t)15)
          << 4U |
      (uint32_t)libcrux_secrets_int_as_u8_f5(
          Eurydice_slice_index_shared(v, (size_t)1U, int16_t) >> 6U &
          (int16_t)15);
  uint8_t r3 =
      (uint32_t)libcrux_secrets_int_as_u8_f5(
          Eurydice_slice_index_shared(v, (size_t)3U, int16_t) & (int16_t)3)
          << 6U |
      (uint32_t)libcrux_secrets_int_as_u8_f5(
          Eurydice_slice_index_shared(v, (size_t)2U, int16_t) >> 4U &
          (int16_t)63);
  uint8_t r4 = libcrux_secrets_int_as_u8_f5(
      Eurydice_slice_index_shared(v, (size_t)3U, int16_t) >> 2U & (int16_t)255);
  return (uint8_t_x5{r0, r1, r2, r3, r4});
}

static KRML_MUSTINLINE Eurydice_arr_dc
libcrux_ml_kem_vector_portable_serialize_serialize_10(Eurydice_arr_e2 v) {
  uint8_t_x5 r0_4 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)0U, (size_t)4U})));
  uint8_t_x5 r5_9 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)4U, (size_t)8U})));
  uint8_t_x5 r10_14 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)8U, (size_t)12U})));
  uint8_t_x5 r15_19 = libcrux_ml_kem_vector_portable_serialize_serialize_10_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)12U, (size_t)16U})));
  return (Eurydice_arr_dc{{r0_4.fst,   r0_4.snd,   r0_4.thd,   r0_4.f3,
                           r0_4.f4,    r5_9.fst,   r5_9.snd,   r5_9.thd,
                           r5_9.f3,    r5_9.f4,    r10_14.fst, r10_14.snd,
                           r10_14.thd, r10_14.f3,  r10_14.f4,  r15_19.fst,
                           r15_19.snd, r15_19.thd, r15_19.f3,  r15_19.f4}});
}

static inline Eurydice_arr_dc libcrux_ml_kem_vector_portable_serialize_10(
    Eurydice_arr_e2 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_89(
      libcrux_ml_kem_vector_portable_serialize_serialize_10(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_dc libcrux_ml_kem_vector_portable_serialize_10_b8(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_serialize_10(a);
}

static KRML_MUSTINLINE int16_t_x8
libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t r0 = libcrux_secrets_int_as_i16_f5(
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t)) &
       (int16_t)3)
          << 8U |
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t)) &
       (int16_t)255));
  int16_t r1 = libcrux_secrets_int_as_i16_f5(
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t)) &
       (int16_t)15)
          << 6U |
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t)) >>
          2U);
  int16_t r2 = libcrux_secrets_int_as_i16_f5(
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t)) &
       (int16_t)63)
          << 4U |
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t)) >>
          4U);
  int16_t r3 = libcrux_secrets_int_as_i16_f5(
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)4U, uint8_t))
          << 2U |
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)3U, uint8_t)) >>
          6U);
  int16_t r4 = libcrux_secrets_int_as_i16_f5(
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)6U, uint8_t)) &
       (int16_t)3)
          << 8U |
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)5U, uint8_t)) &
       (int16_t)255));
  int16_t r5 = libcrux_secrets_int_as_i16_f5(
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)7U, uint8_t)) &
       (int16_t)15)
          << 6U |
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)6U, uint8_t)) >>
          2U);
  int16_t r6 = libcrux_secrets_int_as_i16_f5(
      (libcrux_secrets_int_as_i16_59(
           Eurydice_slice_index_shared(bytes, (size_t)8U, uint8_t)) &
       (int16_t)63)
          << 4U |
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)7U, uint8_t)) >>
          4U);
  int16_t r7 = libcrux_secrets_int_as_i16_f5(
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)9U, uint8_t))
          << 2U |
      libcrux_secrets_int_as_i16_59(
          Eurydice_slice_index_shared(bytes, (size_t)8U, uint8_t)) >>
          6U);
  return (int16_t_x8{r0, r1, r2, r3, r4, r5, r6, r7});
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_serialize_deserialize_10(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t_x8 v0_7 = libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)0U, (size_t)10U})));
  int16_t_x8 v8_15 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_10_int(
          Eurydice_slice_subslice_shared_7e(
              bytes, (core_ops_range_Range_08{(size_t)10U, (size_t)20U})));
  return (
      Eurydice_arr_e2{{v0_7.fst, v0_7.snd, v0_7.thd, v0_7.f3, v0_7.f4, v0_7.f5,
                       v0_7.f6, v0_7.f7, v8_15.fst, v8_15.snd, v8_15.thd,
                       v8_15.f3, v8_15.f4, v8_15.f5, v8_15.f6, v8_15.f7}});
}

static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_10(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_10(
      libcrux_secrets_int_classify_public_classify_ref_9b_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_10_b8(
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
  uint8_t r0 = libcrux_secrets_int_as_u8_f5(
      Eurydice_slice_index_shared(v, (size_t)0U, int16_t) & (int16_t)255);
  uint8_t r1 = libcrux_secrets_int_as_u8_f5(
      Eurydice_slice_index_shared(v, (size_t)0U, int16_t) >> 8U |
      (Eurydice_slice_index_shared(v, (size_t)1U, int16_t) & (int16_t)15)
          << 4U);
  uint8_t r2 = libcrux_secrets_int_as_u8_f5(
      Eurydice_slice_index_shared(v, (size_t)1U, int16_t) >> 4U & (int16_t)255);
  return (uint8_t_x3{r0, r1, r2});
}

static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_ml_kem_vector_portable_serialize_serialize_12(Eurydice_arr_e2 v) {
  uint8_t_x3 r0_2 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)0U, (size_t)2U})));
  uint8_t_x3 r3_5 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)2U, (size_t)4U})));
  uint8_t_x3 r6_8 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)4U, (size_t)6U})));
  uint8_t_x3 r9_11 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)6U, (size_t)8U})));
  uint8_t_x3 r12_14 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)8U, (size_t)10U})));
  uint8_t_x3 r15_17 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)10U, (size_t)12U})));
  uint8_t_x3 r18_20 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)12U, (size_t)14U})));
  uint8_t_x3 r21_23 = libcrux_ml_kem_vector_portable_serialize_serialize_12_int(
      Eurydice_array_to_subslice_shared_85(
          &v, (core_ops_range_Range_08{(size_t)14U, (size_t)16U})));
  return (Eurydice_arr_6d{{r0_2.fst,   r0_2.snd,   r0_2.thd,   r3_5.fst,
                           r3_5.snd,   r3_5.thd,   r6_8.fst,   r6_8.snd,
                           r6_8.thd,   r9_11.fst,  r9_11.snd,  r9_11.thd,
                           r12_14.fst, r12_14.snd, r12_14.thd, r15_17.fst,
                           r15_17.snd, r15_17.thd, r18_20.fst, r18_20.snd,
                           r18_20.thd, r21_23.fst, r21_23.snd, r21_23.thd}});
}

static inline Eurydice_arr_6d libcrux_ml_kem_vector_portable_serialize_12(
    Eurydice_arr_e2 a) {
  return libcrux_secrets_int_public_integers_declassify_d8_bd(
      libcrux_ml_kem_vector_portable_serialize_serialize_12(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_6d libcrux_ml_kem_vector_portable_serialize_12_b8(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_serialize_12(a);
}

typedef struct int16_t_x2_s {
  int16_t fst;
  int16_t snd;
} int16_t_x2;

static KRML_MUSTINLINE int16_t_x2
libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t byte0 = libcrux_secrets_int_as_i16_59(
      Eurydice_slice_index_shared(bytes, (size_t)0U, uint8_t));
  int16_t byte1 = libcrux_secrets_int_as_i16_59(
      Eurydice_slice_index_shared(bytes, (size_t)1U, uint8_t));
  int16_t byte2 = libcrux_secrets_int_as_i16_59(
      Eurydice_slice_index_shared(bytes, (size_t)2U, uint8_t));
  int16_t r0 = (byte1 & (int16_t)15) << 8U | (byte0 & (int16_t)255);
  int16_t r1 = byte2 << 4U | (byte1 >> 4U & (int16_t)15);
  return (int16_t_x2{r0, r1});
}

static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_serialize_deserialize_12(
    Eurydice_borrow_slice_u8 bytes) {
  int16_t_x2 v0_1 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)0U, (size_t)3U})));
  int16_t_x2 v2_3 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)3U, (size_t)6U})));
  int16_t_x2 v4_5 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)6U, (size_t)9U})));
  int16_t_x2 v6_7 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)9U, (size_t)12U})));
  int16_t_x2 v8_9 = libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
      Eurydice_slice_subslice_shared_7e(
          bytes, (core_ops_range_Range_08{(size_t)12U, (size_t)15U})));
  int16_t_x2 v10_11 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
          Eurydice_slice_subslice_shared_7e(
              bytes, (core_ops_range_Range_08{(size_t)15U, (size_t)18U})));
  int16_t_x2 v12_13 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
          Eurydice_slice_subslice_shared_7e(
              bytes, (core_ops_range_Range_08{(size_t)18U, (size_t)21U})));
  int16_t_x2 v14_15 =
      libcrux_ml_kem_vector_portable_serialize_deserialize_12_int(
          Eurydice_slice_subslice_shared_7e(
              bytes, (core_ops_range_Range_08{(size_t)21U, (size_t)24U})));
  return (Eurydice_arr_e2{{v0_1.fst, v0_1.snd, v2_3.fst, v2_3.snd, v4_5.fst,
                           v4_5.snd, v6_7.fst, v6_7.snd, v8_9.fst, v8_9.snd,
                           v10_11.fst, v10_11.snd, v12_13.fst, v12_13.snd,
                           v14_15.fst, v14_15.snd}});
}

static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_12(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_serialize_deserialize_12(
      libcrux_secrets_int_classify_public_classify_ref_9b_90(a));
}

/**
This function found in impl {libcrux_ml_kem::vector::traits::Operations for
libcrux_ml_kem::vector::portable::vector_type::PortableVector}
*/
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_deserialize_12_b8(
    Eurydice_borrow_slice_u8 a) {
  return libcrux_ml_kem_vector_portable_deserialize_12(a);
}

static KRML_MUSTINLINE size_t
libcrux_ml_kem_vector_portable_sampling_rej_sample(
    Eurydice_borrow_slice_u8 a, Eurydice_mut_borrow_slice_i16 result) {
  size_t sampled = (size_t)0U;
  for (size_t i = (size_t)0U; i < Eurydice_slice_len(a, uint8_t) / (size_t)3U;
       i++) {
    size_t i0 = i;
    int16_t b1 = (int16_t)Eurydice_slice_index_shared(
        a, i0 * (size_t)3U + (size_t)0U, uint8_t);
    int16_t b2 = (int16_t)Eurydice_slice_index_shared(
        a, i0 * (size_t)3U + (size_t)1U, uint8_t);
    int16_t b3 = (int16_t)Eurydice_slice_index_shared(
        a, i0 * (size_t)3U + (size_t)2U, uint8_t);
    int16_t d1 = (b2 & (int16_t)15) << 8U | b1;
    int16_t d2 = b3 << 4U | b2 >> 4U;
    if (d1 < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS) {
      if (sampled < (size_t)16U) {
        Eurydice_slice_index_mut(result, sampled, int16_t) = d1;
        sampled++;
      }
    }
    if (d2 < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS) {
      if (sampled < (size_t)16U) {
        Eurydice_slice_index_mut(result, sampled, int16_t) = d2;
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

typedef Eurydice_arr_ea libcrux_ml_kem_mlkem768_MlKem768PrivateKey;

typedef Eurydice_arr_74 libcrux_ml_kem_mlkem768_MlKem768PublicKey;

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
typedef struct Eurydice_arr_b9_s {
  Eurydice_arr_e2 data[16U];
} Eurydice_arr_b9;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct Eurydice_arr_c40_s {
  Eurydice_arr_b9 data[3U];
} Eurydice_arr_c40;

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
static inline Eurydice_arr_b9 libcrux_ml_kem_polynomial_ZERO_d6_ea(void) {
  Eurydice_arr_b9 lit;
  Eurydice_arr_e2 repeat_expression[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    repeat_expression[i] = libcrux_ml_kem_vector_portable_ZERO_b8();
  }
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(Eurydice_arr_e2));
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
static inline Eurydice_arr_b9 libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_42(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_to_uncompressed_ring_element with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_b9 re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)24U,
                                             i0 * (size_t)24U + (size_t)24U}));
    Eurydice_arr_e2 uu____0 =
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_deserialize_vector_1b(
    Eurydice_borrow_slice_u8 secret_key, Eurydice_arr_c40 *secret_as_ntt) {
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_b9 uu____0 =
        libcrux_ml_kem_serialize_deserialize_to_uncompressed_ring_element_ea(
            Eurydice_slice_subslice_shared_7e(
                secret_key,
                (core_ops_range_Range_08{
                    i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                    (i0 + (size_t)1U) *
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
static inline Eurydice_arr_b9
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_6c(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.portable.compress.decompress_ciphertext_coefficient with
const generics
- COEFFICIENT_BITS= 10
*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_ef(
    Eurydice_arr_e2 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int32_t decompressed =
        libcrux_secrets_int_as_i32_f5(a.data[i0]) *
        libcrux_secrets_int_as_i32_f5(
            libcrux_secrets_int_public_integers_classify_27_39(
                LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
    decompressed = (decompressed << 1U) + ((int32_t)1 << (uint32_t)(int32_t)10);
    decompressed = decompressed >> (uint32_t)((int32_t)10 + (int32_t)1);
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_decompress_ciphertext_coefficient_b8_ef(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_ef(
      a);
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_10 with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_then_decompress_10_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_b9 re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)20U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)20U,
                                             i0 * (size_t)20U + (size_t)20U}));
    Eurydice_arr_e2 coefficient =
        libcrux_ml_kem_vector_portable_deserialize_10_b8(bytes);
    Eurydice_arr_e2 uu____0 =
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_0a(
    Eurydice_borrow_slice_u8 serialized) {
  return libcrux_ml_kem_serialize_deserialize_then_decompress_10_ea(serialized);
}

typedef struct libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2_s {
  Eurydice_arr_e2 fst;
  Eurydice_arr_e2 snd;
} libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2;

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_layer_int_vec_step
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE
    libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2
    libcrux_ml_kem_ntt_ntt_layer_int_vec_step_ea(Eurydice_arr_e2 a,
                                                 Eurydice_arr_e2 b,
                                                 int16_t zeta_r) {
  Eurydice_arr_e2 t =
      libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(b,
                                                                        zeta_r);
  b = libcrux_ml_kem_vector_portable_sub_b8(a, &t);
  a = libcrux_ml_kem_vector_portable_add_b8(a, &t);
  return (libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2{a, b});
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_4_plus_ea(
    size_t *zeta_i, Eurydice_arr_b9 *re, size_t layer,
    size_t _initial_coefficient_bound) {
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++) {
    size_t round = i0;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    size_t offset = round * step * (size_t)2U;
    size_t offset_vec = offset / (size_t)16U;
    size_t step_vec = step / (size_t)16U;
    for (size_t i = offset_vec; i < offset_vec + step_vec; i++) {
      size_t j = i;
      libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2 uu____0 =
          libcrux_ml_kem_ntt_ntt_layer_int_vec_step_ea(
              re->data[j], re->data[j + step_vec],
              libcrux_ml_kem_polynomial_zeta(zeta_i[0U]));
      Eurydice_arr_e2 x = uu____0.fst;
      Eurydice_arr_e2 y = uu____0.snd;
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
    size_t *zeta_i, Eurydice_arr_b9 *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    Eurydice_arr_e2 uu____0 =
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
    size_t *zeta_i, Eurydice_arr_b9 *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_portable_ntt_layer_2_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U));
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_at_layer_1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_at_layer_1_ea(
    size_t *zeta_i, Eurydice_arr_b9 *re, size_t _initial_coefficient_bound) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] + (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_portable_ntt_layer_1_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] + (size_t)3U));
    zeta_i[0U] = zeta_i[0U] + (size_t)3U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.poly_barrett_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_poly_barrett_reduce_ea(
    Eurydice_arr_b9 *myself) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 uu____0 =
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
    Eurydice_arr_b9 *self) {
  libcrux_ml_kem_polynomial_poly_barrett_reduce_ea(self);
}

/**
A monomorphic instance of libcrux_ml_kem.ntt.ntt_vector_u
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- VECTOR_U_COMPRESSION_FACTOR= 10
*/
static KRML_MUSTINLINE void libcrux_ml_kem_ntt_ntt_vector_u_0a(
    Eurydice_arr_b9 *re) {
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
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_6c(
    const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_call_mut_35_6c(
            &lvalue, i);
  }
  Eurydice_arr_c40 u_as_ntt = arr_struct;
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_42(ciphertext),
                              uint8_t) /
               (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                (size_t)10U / (size_t)8U);
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 u_bytes = Eurydice_array_to_subslice_shared_36(
        ciphertext,
        (core_ops_range_Range_08{
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                  (size_t)10U / (size_t)8U),
            i0 * (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                  (size_t)10U / (size_t)8U) +
                LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *
                    (size_t)10U / (size_t)8U}));
    u_as_ntt.data[i0] =
        libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_u_0a(
            u_bytes);
    libcrux_ml_kem_ntt_ntt_vector_u_0a(&u_as_ntt.data[i0]);
  }
  return u_as_ntt;
}

/**
A monomorphic instance of
libcrux_ml_kem.vector.portable.compress.decompress_ciphertext_coefficient with
const generics
- COEFFICIENT_BITS= 4
*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_d1(
    Eurydice_arr_e2 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int32_t decompressed =
        libcrux_secrets_int_as_i32_f5(a.data[i0]) *
        libcrux_secrets_int_as_i32_f5(
            libcrux_secrets_int_public_integers_classify_27_39(
                LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_MODULUS));
    decompressed = (decompressed << 1U) + ((int32_t)1 << (uint32_t)(int32_t)4);
    decompressed = decompressed >> (uint32_t)((int32_t)4 + (int32_t)1);
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
static inline Eurydice_arr_e2
libcrux_ml_kem_vector_portable_decompress_ciphertext_coefficient_b8_d1(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_compress_decompress_ciphertext_coefficient_d1(
      a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.deserialize_then_decompress_4
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_then_decompress_4_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_b9 re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)8U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)8U,
                                             i0 * (size_t)8U + (size_t)8U}));
    Eurydice_arr_e2 coefficient =
        libcrux_ml_kem_vector_portable_deserialize_4_b8(bytes);
    Eurydice_arr_e2 uu____0 =
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_89(
    Eurydice_borrow_slice_u8 serialized) {
  return libcrux_ml_kem_serialize_deserialize_then_decompress_4_ea(serialized);
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.ZERO
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static inline Eurydice_arr_b9 libcrux_ml_kem_polynomial_ZERO_ea(void) {
  Eurydice_arr_b9 lit;
  Eurydice_arr_e2 repeat_expression[16U];
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    repeat_expression[i] = libcrux_ml_kem_vector_portable_ZERO_b8();
  }
  memcpy(lit.data, repeat_expression, (size_t)16U * sizeof(Eurydice_arr_e2));
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_ntt_multiply_ea(const Eurydice_arr_b9 *myself,
                                          const Eurydice_arr_b9 *rhs) {
  Eurydice_arr_b9 out = libcrux_ml_kem_polynomial_ZERO_ea();
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 uu____0 = libcrux_ml_kem_vector_portable_ntt_multiply_b8(
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(const Eurydice_arr_b9 *self,
                                             const Eurydice_arr_b9 *rhs) {
  return libcrux_ml_kem_polynomial_ntt_multiply_ea(self, rhs);
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector, size_t

*/
typedef struct Eurydice_dst_ref_shared_f3_s {
  const Eurydice_arr_e2 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_f3;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- N= 16
*/
static inline Eurydice_dst_ref_shared_f3 Eurydice_array_to_slice_shared_24(
    const Eurydice_arr_b9 *a) {
  Eurydice_dst_ref_shared_f3 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
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
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_to_ring_element_1b(
    Eurydice_arr_b9 *myself, const Eurydice_arr_b9 *rhs) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_24(myself),
                              Eurydice_arr_e2);
       i++) {
    size_t i0 = i;
    Eurydice_arr_e2 uu____0 =
        libcrux_ml_kem_vector_portable_add_b8(myself->data[i0], &rhs->data[i0]);
    myself->data[i0] = uu____0;
  }
}

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
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_to_ring_element_d6_1b(
    Eurydice_arr_b9 *self, const Eurydice_arr_b9 *rhs) {
  libcrux_ml_kem_polynomial_add_to_ring_element_1b(self, rhs);
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_1
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_1_ea(
    size_t *zeta_i, Eurydice_arr_b9 *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_portable_inv_ntt_layer_1_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)2U),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)3U));
    zeta_i[0U] = zeta_i[0U] - (size_t)3U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_2
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_2_ea(
    size_t *zeta_i, Eurydice_arr_b9 *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    re->data[round] = libcrux_ml_kem_vector_portable_inv_ntt_layer_2_step_b8(
        re->data[round], libcrux_ml_kem_polynomial_zeta(zeta_i[0U]),
        libcrux_ml_kem_polynomial_zeta(zeta_i[0U] - (size_t)1U));
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
  }
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_3
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_3_ea(
    size_t *zeta_i, Eurydice_arr_b9 *re) {
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t round = i;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
    Eurydice_arr_e2 uu____0 =
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
        Eurydice_arr_e2 a, Eurydice_arr_e2 b, int16_t zeta_r) {
  Eurydice_arr_e2 a_minus_b = libcrux_ml_kem_vector_portable_sub_b8(b, &a);
  a = libcrux_ml_kem_vector_portable_barrett_reduce_b8(
      libcrux_ml_kem_vector_portable_add_b8(a, &b));
  b = libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
      a_minus_b, zeta_r);
  return (libcrux_ml_kem_vector_portable_vector_type_PortableVector_x2{a, b});
}

/**
A monomorphic instance of libcrux_ml_kem.invert_ntt.invert_ntt_at_layer_4_plus
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_invert_ntt_invert_ntt_at_layer_4_plus_ea(size_t *zeta_i,
                                                        Eurydice_arr_b9 *re,
                                                        size_t layer) {
  size_t step = (size_t)1U << (uint32_t)layer;
  for (size_t i0 = (size_t)0U; i0 < (size_t)128U >> (uint32_t)layer; i0++) {
    size_t round = i0;
    zeta_i[0U] = zeta_i[0U] - (size_t)1U;
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
      Eurydice_arr_e2 x = uu____0.fst;
      Eurydice_arr_e2 y = uu____0.snd;
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
static KRML_MUSTINLINE void libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_1b(
    Eurydice_arr_b9 *re) {
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_subtract_reduce_ea(const Eurydice_arr_b9 *myself,
                                             Eurydice_arr_b9 b) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient_normal_form =
        libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
            b.data[i0], (int16_t)1441);
    Eurydice_arr_e2 diff = libcrux_ml_kem_vector_portable_sub_b8(
        myself->data[i0], &coefficient_normal_form);
    Eurydice_arr_e2 red =
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_subtract_reduce_d6_ea(const Eurydice_arr_b9 *self,
                                                Eurydice_arr_b9 b) {
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
static KRML_MUSTINLINE Eurydice_arr_b9 libcrux_ml_kem_matrix_compute_message_1b(
    const Eurydice_arr_b9 *v, const Eurydice_arr_c40 *secret_as_ntt,
    const Eurydice_arr_c40 *u_as_ntt) {
  Eurydice_arr_b9 result = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_b9 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
        &secret_as_ntt->data[i0], &u_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_1b(&result, &product);
  }
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_1b(&result);
  return libcrux_ml_kem_polynomial_subtract_reduce_d6_ea(v, result);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.to_unsigned_field_modulus
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_to_unsigned_representative_b8(a);
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.compress_then_serialize_message with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_serialize_compress_then_serialize_message_ea(
    Eurydice_arr_b9 re) {
  Eurydice_arr_600 serialized = {{0U}};
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient =
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re.data[i0]);
    Eurydice_arr_e2 coefficient_compressed =
        libcrux_ml_kem_vector_portable_compress_1_b8(coefficient);
    Eurydice_array_u8x2 bytes =
        libcrux_ml_kem_vector_portable_serialize_1_b8(coefficient_compressed);
    Eurydice_slice_copy(
        Eurydice_array_to_subslice_mut_364(
            &serialized, (core_ops_range_Range_08{
                             (size_t)2U * i0, (size_t)2U * i0 + (size_t)2U})),
        Eurydice_array_to_slice_shared_26(&bytes), uint8_t);
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
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_ind_cpa_decrypt_unpacked_42(const Eurydice_arr_c40 *secret_key,
                                           const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_c40 u_as_ntt =
      libcrux_ml_kem_ind_cpa_deserialize_then_decompress_u_6c(ciphertext);
  Eurydice_arr_b9 v =
      libcrux_ml_kem_serialize_deserialize_then_decompress_ring_element_v_89(
          Eurydice_array_to_subslice_from_shared_8c(ciphertext, (size_t)960U));
  Eurydice_arr_b9 message =
      libcrux_ml_kem_matrix_compute_message_1b(&v, secret_key, &u_as_ntt);
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
static KRML_MUSTINLINE Eurydice_arr_600 libcrux_ml_kem_ind_cpa_decrypt_42(
    Eurydice_borrow_slice_u8 secret_key, const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_decrypt_call_mut_0b_42(&lvalue, i);
  }
  Eurydice_arr_c40 secret_key_unpacked = arr_struct;
  libcrux_ml_kem_ind_cpa_deserialize_vector_1b(secret_key,
                                               &secret_key_unpacked);
  return libcrux_ml_kem_ind_cpa_decrypt_unpacked_42(&secret_key_unpacked,
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
static inline Eurydice_arr_06 libcrux_ml_kem_hash_functions_portable_G_4a_e0(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_G(input);
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRF
with const generics
- LEN= 32
*/
static inline Eurydice_arr_600 libcrux_ml_kem_hash_functions_portable_PRF_9e(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_600 digest = {{0U}};
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_6e(&digest),
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
static inline Eurydice_arr_600 libcrux_ml_kem_hash_functions_portable_PRF_4a_41(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_PRF_9e(input);
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector[[$3size_t]] with const
generics
- $3size_t
*/
typedef struct Eurydice_arr_aa_s {
  Eurydice_arr_c40 data[3U];
} Eurydice_arr_aa;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cpa.unpacked.IndCpaPublicKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0_s {
  Eurydice_arr_c40 t_as_ntt;
  Eurydice_arr_600 seed_for_A;
  Eurydice_arr_aa A;
} libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0;

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
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
libcrux_ml_kem_ind_cpa_unpacked_default_8b_1b(void) {
  Eurydice_arr_c40 uu____0;
  Eurydice_arr_b9 repeat_expression0[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression0[i] = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  }
  memcpy(uu____0.data, repeat_expression0,
         (size_t)3U * sizeof(Eurydice_arr_b9));
  Eurydice_arr_600 uu____1 = {{0U}};
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 lit0;
  lit0.t_as_ntt = uu____0;
  lit0.seed_for_A = uu____1;
  Eurydice_arr_c40 repeat_expression1[3U];
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    Eurydice_arr_c40 lit;
    Eurydice_arr_b9 repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_ea();
    }
    memcpy(lit.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_b9));
    repeat_expression1[i0] = lit;
  }
  memcpy(lit0.A.data, repeat_expression1,
         (size_t)3U * sizeof(Eurydice_arr_c40));
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_to_reduced_ring_element_ea(
    Eurydice_borrow_slice_u8 serialized) {
  Eurydice_arr_b9 re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(serialized, uint8_t) / (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 bytes = Eurydice_slice_subslice_shared_7e(
        serialized, (core_ops_range_Range_08{i0 * (size_t)24U,
                                             i0 * (size_t)24U + (size_t)24U}));
    Eurydice_arr_e2 coefficient =
        libcrux_ml_kem_vector_portable_deserialize_12_b8(bytes);
    Eurydice_arr_e2 uu____0 =
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
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_1b(
    Eurydice_borrow_slice_u8 public_key, Eurydice_arr_c40 *deserialized_pk) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(public_key, uint8_t) /
               LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT;
       i++) {
    size_t i0 = i;
    Eurydice_borrow_slice_u8 ring_element = Eurydice_slice_subslice_shared_7e(
        public_key, (core_ops_range_Range_08{
                        i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                        i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT +
                            LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT}));
    Eurydice_arr_b9 uu____0 =
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
static inline Eurydice_arr_e4
libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_e0(
    const Eurydice_arr_84 *input) {
  Eurydice_arr_e4 shake128_state;
  Eurydice_arr_26 repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = libcrux_sha3_portable_incremental_shake128_init();
  }
  memcpy(shake128_state.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_26));
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_incremental_shake128_absorb_final(
        &shake128_state.data[i0],
        Eurydice_array_to_slice_shared_8d(&input->data[i0]));
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
static inline Eurydice_arr_e4
libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_e0(
    const Eurydice_arr_84 *input) {
  return libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_e0(
      input);
}

/**
A monomorphic instance of
libcrux_ml_kem.hash_functions.portable.shake128_squeeze_first_three_blocks with
const generics
- K= 3
*/
static inline Eurydice_arr_35
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_e0(
    Eurydice_arr_e4 *st) {
  Eurydice_arr_35 out = {{{{0U}}, {{0U}}, {{0U}}}};
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_incremental_shake128_squeeze_first_three_blocks(
        &st->data[i0], Eurydice_array_to_slice_mut_85(&out.data[i0]));
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
static inline Eurydice_arr_35
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_e0(
    Eurydice_arr_e4 *self) {
  return libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_e0(
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
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_89(
    const Eurydice_arr_35 *randomness, Eurydice_arr_c8 *sampled_coefficients,
    Eurydice_arr_d4 *out) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)504U / (size_t)24U; i++) {
      size_t r = i;
      if (sampled_coefficients->data[i1] <
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        size_t sampled = libcrux_ml_kem_vector_portable_rej_sample_b8(
            Eurydice_array_to_subslice_shared_360(
                &randomness->data[i1],
                (core_ops_range_Range_08{r * (size_t)24U,
                                         r * (size_t)24U + (size_t)24U})),
            Eurydice_array_to_subslice_mut_85(
                &out->data[i1],
                (core_ops_range_Range_08{
                    sampled_coefficients->data[i1],
                    sampled_coefficients->data[i1] + (size_t)16U})));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] =
            sampled_coefficients->data[uu____0] + sampled;
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
static inline Eurydice_arr_d6
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_e0(
    Eurydice_arr_e4 *st) {
  Eurydice_arr_d6 out = {{{{0U}}, {{0U}}, {{0U}}}};
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
        &st->data[i0], Eurydice_array_to_slice_mut_7b(&out.data[i0]));
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
static inline Eurydice_arr_d6
libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_e0(
    Eurydice_arr_e4 *self) {
  return libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_e0(
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
libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_890(
    const Eurydice_arr_d6 *randomness, Eurydice_arr_c8 *sampled_coefficients,
    Eurydice_arr_d4 *out) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)24U; i++) {
      size_t r = i;
      if (sampled_coefficients->data[i1] <
          LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT) {
        size_t sampled = libcrux_ml_kem_vector_portable_rej_sample_b8(
            Eurydice_array_to_subslice_shared_361(
                &randomness->data[i1],
                (core_ops_range_Range_08{r * (size_t)24U,
                                         r * (size_t)24U + (size_t)24U})),
            Eurydice_array_to_subslice_mut_85(
                &out->data[i1],
                (core_ops_range_Range_08{
                    sampled_coefficients->data[i1],
                    sampled_coefficients->data[i1] + (size_t)16U})));
        size_t uu____0 = i1;
        sampled_coefficients->data[uu____0] =
            sampled_coefficients->data[uu____0] + sampled;
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_from_i16_array_ea(Eurydice_borrow_slice_i16 a) {
  Eurydice_arr_b9 result = libcrux_ml_kem_polynomial_ZERO_ea();
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 uu____0 = libcrux_ml_kem_vector_portable_from_i16_array_b8(
        Eurydice_slice_subslice_shared_76(
            a, (core_ops_range_Range_08{i0 * (size_t)16U,
                                        (i0 + (size_t)1U) * (size_t)16U})));
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_from_i16_array_d6_ea(Eurydice_borrow_slice_i16 a) {
  return libcrux_ml_kem_polynomial_from_i16_array_ea(a);
}

/**
This function found in impl {core::ops::function::FnMut<(@Array<i16, 272usize>),
libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@2]> for libcrux_ml_kem::sampling::sample_from_xof::closure<Vector,
Hasher, K>[TraitClause@0, TraitClause@1, TraitClause@2, TraitClause@3]}
*/
/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof.call_mut_e7
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
static inline Eurydice_arr_b9
libcrux_ml_kem_sampling_sample_from_xof_call_mut_e7_2b(
    void **_, Eurydice_arr_a00 tupled_args) {
  Eurydice_arr_a00 s = tupled_args;
  return libcrux_ml_kem_polynomial_from_i16_array_d6_ea(
      Eurydice_array_to_subslice_shared_850(
          &s, (core_ops_range_Range_08{(size_t)0U, (size_t)256U})));
}

/**
A monomorphic instance of libcrux_ml_kem.sampling.sample_from_xof
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_sampling_sample_from_xof_2b(const Eurydice_arr_84 *seeds) {
  Eurydice_arr_c8 sampled_coefficients = {{0U}};
  Eurydice_arr_d4 out = {{{{0U}}, {{0U}}, {{0U}}}};
  Eurydice_arr_e4 xof_state =
      libcrux_ml_kem_hash_functions_portable_shake128_init_absorb_final_4a_e0(
          seeds);
  Eurydice_arr_35 randomness0 =
      libcrux_ml_kem_hash_functions_portable_shake128_squeeze_first_three_blocks_4a_e0(
          &xof_state);
  bool done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_89(
      &randomness0, &sampled_coefficients, &out);
  while (true) {
    if (done) {
      break;
    } else {
      Eurydice_arr_d6 randomness =
          libcrux_ml_kem_hash_functions_portable_shake128_squeeze_next_block_4a_e0(
              &xof_state);
      done = libcrux_ml_kem_sampling_sample_from_uniform_distribution_next_890(
          &randomness, &sampled_coefficients, &out);
    }
  }
  Eurydice_arr_c40 arr_mapped_str;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_mapped_str.data[i] =
        libcrux_ml_kem_sampling_sample_from_xof_call_mut_e7_2b(&lvalue,
                                                               out.data[i]);
  }
  return arr_mapped_str;
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector, size_t

*/
typedef struct Eurydice_dst_ref_shared_30_s {
  const Eurydice_arr_b9 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_30;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- N= 3
*/
static inline Eurydice_dst_ref_shared_30 Eurydice_array_to_slice_shared_cf(
    const Eurydice_arr_c40 *a) {
  Eurydice_dst_ref_shared_30 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_kem.matrix.sample_matrix_A
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector,
libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]] with const
generics
- K= 3
*/
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_sample_matrix_A_2b(
    Eurydice_arr_aa *A_transpose, const Eurydice_arr_48 *seed, bool transpose) {
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    Eurydice_arr_84 seeds;
    Eurydice_arr_48 repeat_expression[3U];
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      repeat_expression[i] =
          core_array__core__clone__Clone_for__Array_T__N___clone(
              (size_t)34U, seed, uint8_t, Eurydice_arr_48);
    }
    memcpy(seeds.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_48));
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      seeds.data[j].data[32U] = (uint8_t)i1;
      seeds.data[j].data[33U] = (uint8_t)j;
    }
    Eurydice_arr_c40 sampled =
        libcrux_ml_kem_sampling_sample_from_xof_2b(&seeds);
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf(&sampled),
                                Eurydice_arr_b9);
         i++) {
      size_t j = i;
      Eurydice_arr_b9 sample = sampled.data[j];
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
libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_3f(
    Eurydice_borrow_slice_u8 public_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
        *unpacked_public_key) {
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_1b(
      Eurydice_slice_subslice_to_shared_c6(public_key, (size_t)1152U),
      &unpacked_public_key->t_as_ntt);
  Eurydice_borrow_slice_u8 seed =
      Eurydice_slice_subslice_from_shared_6b(public_key, (size_t)1152U);
  Eurydice_arr_aa *uu____0 = &unpacked_public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue = libcrux_ml_kem_utils_into_padded_array_b6(seed);
  libcrux_ml_kem_matrix_sample_matrix_A_2b(uu____0, &lvalue, false);
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
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
    libcrux_ml_kem_ind_cpa_build_unpacked_public_key_3f(
        Eurydice_borrow_slice_u8 public_key) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
      unpacked_public_key = libcrux_ml_kem_ind_cpa_unpacked_default_8b_1b();
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_3f(public_key,
                                                          &unpacked_public_key);
  return unpacked_public_key;
}

/**
A monomorphic instance of K.
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector[[$3size_t]],
libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector

*/
typedef struct tuple_88_s {
  Eurydice_arr_c40 fst;
  Eurydice_arr_b9 snd;
} tuple_88;

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
static inline Eurydice_arr_b9 libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_85(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRFxN
with const generics
- K= 3
- LEN= 128
*/
static inline Eurydice_arr_db libcrux_ml_kem_hash_functions_portable_PRFxN_41(
    const Eurydice_arr_46 *input) {
  Eurydice_arr_db out = {{{{0U}}, {{0U}}, {{0U}}}};
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    libcrux_sha3_portable_shake256(
        Eurydice_array_to_slice_mut_18(&out.data[i0]),
        Eurydice_array_to_slice_shared_61(&input->data[i0]));
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
static inline Eurydice_arr_db
libcrux_ml_kem_hash_functions_portable_PRFxN_4a_41(
    const Eurydice_arr_46 *input) {
  return libcrux_ml_kem_hash_functions_portable_PRFxN_41(input);
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_sampling_sample_from_binomial_distribution_2_ea(
    Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_c1 sampled_i16s = {{0U}};
  for (size_t i0 = (size_t)0U;
       i0 < Eurydice_slice_len(randomness, uint8_t) / (size_t)4U; i0++) {
    size_t chunk_number = i0;
    Eurydice_borrow_slice_u8 byte_chunk = Eurydice_slice_subslice_shared_7e(
        randomness,
        (core_ops_range_Range_08{chunk_number * (size_t)4U,
                                 chunk_number * (size_t)4U + (size_t)4U}));
    uint32_t random_bits_as_u32 =
        (((uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)0U,
                                                uint8_t) |
          (uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)1U, uint8_t)
              << 8U) |
         (uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)2U, uint8_t)
             << 16U) |
        (uint32_t)Eurydice_slice_index_shared(byte_chunk, (size_t)3U, uint8_t)
            << 24U;
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
      Eurydice_array_to_slice_shared_1a(&sampled_i16s));
}

/**
A monomorphic instance of
libcrux_ml_kem.sampling.sample_from_binomial_distribution with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- ETA= 2
*/
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_sampling_sample_from_binomial_distribution_a0(
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
    Eurydice_arr_b9 *re) {
  size_t step = LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT / (size_t)2U;
  for (size_t i = (size_t)0U; i < step; i++) {
    size_t j = i;
    Eurydice_arr_e2 t = libcrux_ml_kem_vector_portable_multiply_by_constant_b8(
        re->data[j + step], (int16_t)-1600);
    re->data[j + step] = libcrux_ml_kem_vector_portable_sub_b8(re->data[j], &t);
    Eurydice_arr_e2 uu____1 =
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
libcrux_ml_kem_ntt_ntt_binomially_sampled_ring_element_ea(Eurydice_arr_b9 *re) {
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
libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_3b(
    Eurydice_arr_c40 *re_as_ntt, const Eurydice_arr_3e *prf_input,
    uint8_t domain_separator) {
  Eurydice_arr_46 prf_inputs;
  Eurydice_arr_3e repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)33U, prf_input, uint8_t, Eurydice_arr_3e);
  }
  memcpy(prf_inputs.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_3e));
  domain_separator =
      libcrux_ml_kem_utils_prf_input_inc_e0(&prf_inputs, domain_separator);
  Eurydice_arr_db prf_outputs =
      libcrux_ml_kem_hash_functions_portable_PRFxN_4a_41(&prf_inputs);
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_b9 uu____0 =
        libcrux_ml_kem_sampling_sample_from_binomial_distribution_a0(
            Eurydice_array_to_slice_shared_18(&prf_outputs.data[i0]));
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
static inline Eurydice_arr_b9 libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_85(
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
libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_3b(
    const Eurydice_arr_3e *prf_input, uint8_t domain_separator,
    Eurydice_arr_c40 *error_1) {
  Eurydice_arr_46 prf_inputs;
  Eurydice_arr_3e repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] =
        core_array__core__clone__Clone_for__Array_T__N___clone(
            (size_t)33U, prf_input, uint8_t, Eurydice_arr_3e);
  }
  memcpy(prf_inputs.data, repeat_expression,
         (size_t)3U * sizeof(Eurydice_arr_3e));
  domain_separator =
      libcrux_ml_kem_utils_prf_input_inc_e0(&prf_inputs, domain_separator);
  Eurydice_arr_db prf_outputs =
      libcrux_ml_kem_hash_functions_portable_PRFxN_4a_41(&prf_inputs);
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_b9 uu____0 =
        libcrux_ml_kem_sampling_sample_from_binomial_distribution_a0(
            Eurydice_array_to_slice_shared_18(&prf_outputs.data[i0]));
    error_1->data[i0] = uu____0;
  }
  return domain_separator;
}

/**
A monomorphic instance of libcrux_ml_kem.hash_functions.portable.PRF
with const generics
- LEN= 128
*/
static inline Eurydice_arr_d1 libcrux_ml_kem_hash_functions_portable_PRF_a6(
    Eurydice_borrow_slice_u8 input) {
  Eurydice_arr_d1 digest = {{0U}};
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_18(&digest),
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
static inline Eurydice_arr_d1 libcrux_ml_kem_hash_functions_portable_PRF_4a_410(
    Eurydice_borrow_slice_u8 input) {
  return libcrux_ml_kem_hash_functions_portable_PRF_a6(input);
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
static inline Eurydice_arr_b9
libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_1b(void **_,
                                                      size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector[[$3size_t]], size_t

*/
typedef struct Eurydice_dst_ref_shared_94_s {
  const Eurydice_arr_c40 *ptr;
  size_t meta;
} Eurydice_dst_ref_shared_94;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr libcrux_ml_kem_polynomial_PolynomialRingElement
libcrux_ml_kem_vector_portable_vector_type_PortableVector[[$3size_t]] with const
generics
- N= 3
*/
static inline Eurydice_dst_ref_shared_94 Eurydice_array_to_slice_shared_b5(
    const Eurydice_arr_aa *a) {
  Eurydice_dst_ref_shared_94 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3U;
  return lit;
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.add_error_reduce
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void libcrux_ml_kem_polynomial_add_error_reduce_ea(
    Eurydice_arr_b9 *myself, const Eurydice_arr_b9 *error) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t j = i;
    Eurydice_arr_e2 coefficient_normal_form =
        libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
            myself->data[j], (int16_t)1441);
    Eurydice_arr_e2 sum = libcrux_ml_kem_vector_portable_add_b8(
        coefficient_normal_form, &error->data[j]);
    Eurydice_arr_e2 red = libcrux_ml_kem_vector_portable_barrett_reduce_b8(sum);
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
    Eurydice_arr_b9 *self, const Eurydice_arr_b9 *error) {
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
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_matrix_compute_vector_u_1b(const Eurydice_arr_aa *a_as_ntt,
                                          const Eurydice_arr_c40 *r_as_ntt,
                                          const Eurydice_arr_c40 *error_1) {
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_matrix_compute_vector_u_call_mut_a8_1b(&lvalue, i);
  }
  Eurydice_arr_c40 result = arr_struct;
  for (size_t i0 = (size_t)0U;
       i0 < Eurydice_slice_len(Eurydice_array_to_slice_shared_b5(a_as_ntt),
                               Eurydice_arr_c40);
       i0++) {
    size_t i1 = i0;
    const Eurydice_arr_c40 *row = &a_as_ntt->data[i1];
    for (size_t i = (size_t)0U;
         i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf(row),
                                Eurydice_arr_b9);
         i++) {
      size_t j = i;
      const Eurydice_arr_b9 *a_element = &row->data[j];
      Eurydice_arr_b9 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
          a_element, &r_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_1b(&result.data[i1],
                                                          &product);
    }
    libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_1b(&result.data[i1]);
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
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_compress_compress_ef(Eurydice_arr_e2 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int16_t uu____0 = libcrux_secrets_int_as_i16_f5(
        libcrux_ml_kem_vector_portable_compress_compress_ciphertext_coefficient(
            (uint8_t)(int32_t)10, libcrux_secrets_int_as_u16_f5(a.data[i0])));
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
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_compress_b8_ef(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_compress_compress_ef(a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_10
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- OUT_LEN= 320
*/
static KRML_MUSTINLINE Eurydice_arr_b7
libcrux_ml_kem_serialize_compress_then_serialize_10_ff(
    const Eurydice_arr_b9 *re) {
  Eurydice_arr_b7 serialized = {{0U}};
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient = libcrux_ml_kem_vector_portable_compress_b8_ef(
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re->data[i0]));
    Eurydice_arr_dc bytes =
        libcrux_ml_kem_vector_portable_serialize_10_b8(coefficient);
    Eurydice_slice_copy(Eurydice_array_to_subslice_mut_369(
                            &serialized, (core_ops_range_Range_08{
                                             (size_t)20U * i0,
                                             (size_t)20U * i0 + (size_t)20U})),
                        Eurydice_array_to_slice_shared_c2(&bytes), uint8_t);
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
static KRML_MUSTINLINE Eurydice_arr_b7
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_fe(
    const Eurydice_arr_b9 *re) {
  return libcrux_ml_kem_serialize_compress_then_serialize_10_ff(re);
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_compress_then_serialize_u_43(
    Eurydice_arr_c40 input, Eurydice_mut_borrow_slice_u8 out) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf(&input),
                              Eurydice_arr_b9);
       i++) {
    size_t i0 = i;
    Eurydice_arr_b9 re = input.data[i0];
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_7e(
        out, (core_ops_range_Range_08{
                 i0 * ((size_t)960U / (size_t)3U),
                 (i0 + (size_t)1U) * ((size_t)960U / (size_t)3U)}));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_b7 lvalue =
        libcrux_ml_kem_serialize_compress_then_serialize_ring_element_u_fe(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_d3(&lvalue),
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
static KRML_MUSTINLINE tuple_88 libcrux_ml_kem_ind_cpa_encrypt_c1_85(
    Eurydice_borrow_slice_u8 randomness, const Eurydice_arr_aa *matrix,
    Eurydice_mut_borrow_slice_u8 ciphertext) {
  Eurydice_arr_3e prf_input =
      libcrux_ml_kem_utils_into_padded_array_c8(randomness);
  Eurydice_arr_c40 arr_struct0;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct0.data[i] =
        libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_f1_85(&lvalue, i);
  }
  Eurydice_arr_c40 r_as_ntt = arr_struct0;
  uint8_t domain_separator0 =
      libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_3b(&r_as_ntt,
                                                           &prf_input, 0U);
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_encrypt_c1_call_mut_dd_85(&lvalue, i);
  }
  Eurydice_arr_c40 error_1 = arr_struct;
  uint8_t domain_separator = libcrux_ml_kem_ind_cpa_sample_ring_element_cbd_3b(
      &prf_input, domain_separator0, &error_1);
  prf_input.data[32U] = domain_separator;
  Eurydice_arr_d1 prf_output =
      libcrux_ml_kem_hash_functions_portable_PRF_4a_410(
          Eurydice_array_to_slice_shared_61(&prf_input));
  Eurydice_arr_b9 error_2 =
      libcrux_ml_kem_sampling_sample_from_binomial_distribution_a0(
          Eurydice_array_to_slice_shared_18(&prf_output));
  Eurydice_arr_c40 u =
      libcrux_ml_kem_matrix_compute_vector_u_1b(matrix, &r_as_ntt, &error_1);
  libcrux_ml_kem_ind_cpa_compress_then_serialize_u_43(u, ciphertext);
  return (tuple_88{r_as_ntt, error_2});
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.deserialize_then_decompress_message with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_then_decompress_message_ea(
    const Eurydice_arr_600 *serialized) {
  Eurydice_arr_b9 re = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < (size_t)16U; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient_compressed =
        libcrux_ml_kem_vector_portable_deserialize_1_b8(
            Eurydice_array_to_subslice_shared_362(
                serialized,
                (core_ops_range_Range_08{(size_t)2U * i0,
                                         (size_t)2U * i0 + (size_t)2U})));
    Eurydice_arr_e2 uu____0 =
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_add_message_error_reduce_ea(
    const Eurydice_arr_b9 *myself, const Eurydice_arr_b9 *message,
    Eurydice_arr_b9 result) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient_normal_form =
        libcrux_ml_kem_vector_portable_montgomery_multiply_by_constant_b8(
            result.data[i0], (int16_t)1441);
    Eurydice_arr_e2 sum1 = libcrux_ml_kem_vector_portable_add_b8(
        myself->data[i0], &message->data[i0]);
    Eurydice_arr_e2 sum2 =
        libcrux_ml_kem_vector_portable_add_b8(coefficient_normal_form, &sum1);
    Eurydice_arr_e2 red =
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_polynomial_add_message_error_reduce_d6_ea(
    const Eurydice_arr_b9 *self, const Eurydice_arr_b9 *message,
    Eurydice_arr_b9 result) {
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
static KRML_MUSTINLINE Eurydice_arr_b9
libcrux_ml_kem_matrix_compute_ring_element_v_1b(
    const Eurydice_arr_c40 *t_as_ntt, const Eurydice_arr_c40 *r_as_ntt,
    const Eurydice_arr_b9 *error_2, const Eurydice_arr_b9 *message) {
  Eurydice_arr_b9 result = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    size_t i0 = i;
    Eurydice_arr_b9 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
        &t_as_ntt->data[i0], &r_as_ntt->data[i0]);
    libcrux_ml_kem_polynomial_add_to_ring_element_d6_1b(&result, &product);
  }
  libcrux_ml_kem_invert_ntt_invert_ntt_montgomery_1b(&result);
  return libcrux_ml_kem_polynomial_add_message_error_reduce_d6_ea(
      error_2, message, result);
}

/**
A monomorphic instance of libcrux_ml_kem.vector.portable.compress.compress
with const generics
- COEFFICIENT_BITS= 4
*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_vector_portable_compress_compress_d1(Eurydice_arr_e2 a) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_VECTOR_TRAITS_FIELD_ELEMENTS_IN_VECTOR; i++) {
    size_t i0 = i;
    int16_t uu____0 = libcrux_secrets_int_as_i16_f5(
        libcrux_ml_kem_vector_portable_compress_compress_ciphertext_coefficient(
            (uint8_t)(int32_t)4, libcrux_secrets_int_as_u16_f5(a.data[i0])));
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
static inline Eurydice_arr_e2 libcrux_ml_kem_vector_portable_compress_b8_d1(
    Eurydice_arr_e2 a) {
  return libcrux_ml_kem_vector_portable_compress_compress_d1(a);
}

/**
A monomorphic instance of libcrux_ml_kem.serialize.compress_then_serialize_4
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_kem_serialize_compress_then_serialize_4_ea(
    Eurydice_arr_b9 re, Eurydice_mut_borrow_slice_u8 serialized) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient = libcrux_ml_kem_vector_portable_compress_b8_d1(
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re.data[i0]));
    Eurydice_array_u8x8 bytes =
        libcrux_ml_kem_vector_portable_serialize_4_b8(coefficient);
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_7e(
            serialized, (core_ops_range_Range_08{
                            (size_t)8U * i0, (size_t)8U * i0 + (size_t)8U})),
        Eurydice_array_to_slice_shared_41(&bytes), uint8_t);
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
libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_6c(
    Eurydice_arr_b9 re, Eurydice_mut_borrow_slice_u8 out) {
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_encrypt_c2_6c(
    const Eurydice_arr_c40 *t_as_ntt, const Eurydice_arr_c40 *r_as_ntt,
    const Eurydice_arr_b9 *error_2, const Eurydice_arr_600 *message,
    Eurydice_mut_borrow_slice_u8 ciphertext) {
  Eurydice_arr_b9 message_as_ring_element =
      libcrux_ml_kem_serialize_deserialize_then_decompress_message_ea(message);
  Eurydice_arr_b9 v = libcrux_ml_kem_matrix_compute_ring_element_v_1b(
      t_as_ntt, r_as_ntt, error_2, &message_as_ring_element);
  libcrux_ml_kem_serialize_compress_then_serialize_ring_element_v_6c(
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
static KRML_MUSTINLINE Eurydice_arr_2c
libcrux_ml_kem_ind_cpa_encrypt_unpacked_2a(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
        *public_key,
    const Eurydice_arr_600 *message, Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_2c ciphertext = {{0U}};
  tuple_88 uu____0 = libcrux_ml_kem_ind_cpa_encrypt_c1_85(
      randomness, &public_key->A,
      Eurydice_array_to_subslice_mut_3610(
          &ciphertext, (core_ops_range_Range_08{(size_t)0U, (size_t)960U})));
  Eurydice_arr_c40 r_as_ntt = uu____0.fst;
  Eurydice_arr_b9 error_2 = uu____0.snd;
  libcrux_ml_kem_ind_cpa_encrypt_c2_6c(
      &public_key->t_as_ntt, &r_as_ntt, &error_2, message,
      Eurydice_array_to_subslice_from_mut_8c1(&ciphertext, (size_t)960U));
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
static KRML_MUSTINLINE Eurydice_arr_2c libcrux_ml_kem_ind_cpa_encrypt_2a(
    Eurydice_borrow_slice_u8 public_key, const Eurydice_arr_600 *message,
    Eurydice_borrow_slice_u8 randomness) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
      unpacked_public_key =
          libcrux_ml_kem_ind_cpa_build_unpacked_public_key_3f(public_key);
  return libcrux_ml_kem_ind_cpa_encrypt_unpacked_2a(&unpacked_public_key,
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
static KRML_MUSTINLINE Eurydice_arr_600 libcrux_ml_kem_variant_kdf_39_d6(
    Eurydice_borrow_slice_u8 shared_secret, const Eurydice_arr_2c *_) {
  Eurydice_arr_600 out = {{0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(&out), shared_secret,
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
static KRML_MUSTINLINE Eurydice_arr_600 libcrux_ml_kem_ind_cca_decapsulate_62(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x4 uu____0 =
      libcrux_ml_kem_types_unpack_private_key_b4(
          Eurydice_array_to_slice_shared_ec(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  Eurydice_arr_600 decrypted =
      libcrux_ml_kem_ind_cpa_decrypt_42(ind_cpa_secret_key, ciphertext);
  Eurydice_arr_06 to_hash0 = libcrux_ml_kem_utils_into_padded_array_24(
      Eurydice_array_to_slice_shared_6e(&decrypted));
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_from_mut_8c(
          &to_hash0, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE),
      ind_cpa_public_key_hash, uint8_t);
  Eurydice_arr_06 hashed = libcrux_ml_kem_hash_functions_portable_G_4a_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash0));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret0 = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_480 to_hash =
      libcrux_ml_kem_utils_into_padded_array_15(implicit_rejection_value);
  Eurydice_mut_borrow_slice_u8 uu____2 =
      Eurydice_array_to_subslice_from_mut_8c0(
          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_d3_80(ciphertext),
                      uint8_t);
  Eurydice_arr_600 implicit_rejection_shared_secret =
      libcrux_ml_kem_hash_functions_portable_PRF_4a_41(
          Eurydice_array_to_slice_shared_74(&to_hash));
  Eurydice_arr_2c expected_ciphertext = libcrux_ml_kem_ind_cpa_encrypt_2a(
      ind_cpa_public_key, &decrypted, pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 =
      Eurydice_array_to_slice_shared_6e(&implicit_rejection_shared_secret);
  Eurydice_arr_600 implicit_rejection_shared_secret0 =
      libcrux_ml_kem_variant_kdf_39_d6(
          uu____3, libcrux_ml_kem_types_as_slice_a9_80(ciphertext));
  Eurydice_arr_600 shared_secret = libcrux_ml_kem_variant_kdf_39_d6(
      shared_secret0, libcrux_ml_kem_types_as_slice_a9_80(ciphertext));
  Eurydice_borrow_slice_u8 uu____4 =
      libcrux_ml_kem_types_as_ref_d3_80(ciphertext);
  return libcrux_ml_kem_constant_time_ops_compare_ciphertexts_select_shared_secret_in_constant_time(
      uu____4, Eurydice_array_to_slice_shared_42(&expected_ciphertext),
      Eurydice_array_to_slice_shared_6e(&shared_secret),
      Eurydice_array_to_slice_shared_6e(&implicit_rejection_shared_secret0));
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
static inline Eurydice_arr_600
libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_35(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_decapsulate_62(private_key, ciphertext);
}

/**
 Decapsulate ML-KEM 768

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an [`MlKem768PrivateKey`] and an
 [`MlKem768Ciphertext`].
*/
static inline Eurydice_arr_600 libcrux_ml_kem_mlkem768_portable_decapsulate(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_decapsulate_35(
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
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_variant_entropy_preprocess_39_9c(
    Eurydice_borrow_slice_u8 randomness) {
  Eurydice_arr_600 out = {{0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(&out), randomness,
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
static inline Eurydice_arr_600 libcrux_ml_kem_hash_functions_portable_H_4a_e0(
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
static KRML_MUSTINLINE tuple_56 libcrux_ml_kem_ind_cca_encapsulate_ca(
    const Eurydice_arr_74 *public_key, const Eurydice_arr_600 *randomness) {
  Eurydice_arr_600 randomness0 =
      libcrux_ml_kem_variant_entropy_preprocess_39_9c(
          Eurydice_array_to_slice_shared_6e(randomness));
  Eurydice_arr_06 to_hash = libcrux_ml_kem_utils_into_padded_array_24(
      Eurydice_array_to_slice_shared_6e(&randomness0));
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_from_mut_8c(
      &to_hash, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE);
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_600 lvalue = libcrux_ml_kem_hash_functions_portable_H_4a_e0(
      Eurydice_array_to_slice_shared_45(
          libcrux_ml_kem_types_as_slice_e6_d0(public_key)));
  Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_6e(&lvalue),
                      uint8_t);
  Eurydice_arr_06 hashed = libcrux_ml_kem_hash_functions_portable_G_4a_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_2c ciphertext = libcrux_ml_kem_ind_cpa_encrypt_2a(
      Eurydice_array_to_slice_shared_45(
          libcrux_ml_kem_types_as_slice_e6_d0(public_key)),
      &randomness0, pseudorandomness);
  Eurydice_arr_2c uu____2 = libcrux_ml_kem_types_from_e0_80(ciphertext);
  return (tuple_56{
      uu____2, libcrux_ml_kem_variant_kdf_39_d6(shared_secret, &ciphertext)});
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
static inline tuple_56
libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_cd(
    const Eurydice_arr_74 *public_key, const Eurydice_arr_600 *randomness) {
  return libcrux_ml_kem_ind_cca_encapsulate_ca(public_key, randomness);
}

/**
 Encapsulate ML-KEM 768

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
 bytes of `randomness`.
*/
static inline tuple_56 libcrux_ml_kem_mlkem768_portable_encapsulate(
    const Eurydice_arr_74 *public_key, Eurydice_arr_600 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_encapsulate_cd(
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
static inline Eurydice_arr_c40 libcrux_ml_kem_ind_cpa_unpacked_default_70_1b(
    void) {
  Eurydice_arr_c40 lit;
  Eurydice_arr_b9 repeat_expression[3U];
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    repeat_expression[i] = libcrux_ml_kem_polynomial_ZERO_d6_ea();
  }
  memcpy(lit.data, repeat_expression, (size_t)3U * sizeof(Eurydice_arr_b9));
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
static KRML_MUSTINLINE Eurydice_arr_06
libcrux_ml_kem_variant_cpa_keygen_seed_39_9c(
    Eurydice_borrow_slice_u8 key_generation_seed) {
  Eurydice_arr_3e seed = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_368(
          &seed,
          (core_ops_range_Range_08{
              (size_t)0U,
              LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE})),
      key_generation_seed, uint8_t);
  seed.data[LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE] =
      (uint8_t)(size_t)3U;
  return libcrux_ml_kem_hash_functions_portable_G_4a_e0(
      Eurydice_array_to_slice_shared_61(&seed));
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
static inline Eurydice_arr_b9
libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_1c(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
A monomorphic instance of libcrux_ml_kem.polynomial.to_standard_domain
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_e2
libcrux_ml_kem_polynomial_to_standard_domain_ea(Eurydice_arr_e2 vector) {
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
    Eurydice_arr_b9 *myself, const Eurydice_arr_b9 *error) {
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t j = i;
    Eurydice_arr_e2 coefficient_normal_form =
        libcrux_ml_kem_polynomial_to_standard_domain_ea(myself->data[j]);
    Eurydice_arr_e2 sum = libcrux_ml_kem_vector_portable_add_b8(
        coefficient_normal_form, &error->data[j]);
    Eurydice_arr_e2 red = libcrux_ml_kem_vector_portable_barrett_reduce_b8(sum);
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
    Eurydice_arr_b9 *self, const Eurydice_arr_b9 *error) {
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
static KRML_MUSTINLINE void libcrux_ml_kem_matrix_compute_As_plus_e_1b(
    Eurydice_arr_c40 *t_as_ntt, const Eurydice_arr_aa *matrix_A,
    const Eurydice_arr_c40 *s_as_ntt, const Eurydice_arr_c40 *error_as_ntt) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_b5(matrix_A),
                              Eurydice_arr_c40);
       i++) {
    size_t i0 = i;
    const Eurydice_arr_c40 *row = &matrix_A->data[i0];
    Eurydice_arr_b9 uu____0 = libcrux_ml_kem_polynomial_ZERO_d6_ea();
    t_as_ntt->data[i0] = uu____0;
    for (size_t i1 = (size_t)0U;
         i1 < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf(row),
                                 Eurydice_arr_b9);
         i1++) {
      size_t j = i1;
      const Eurydice_arr_b9 *matrix_element = &row->data[j];
      Eurydice_arr_b9 product = libcrux_ml_kem_polynomial_ntt_multiply_d6_ea(
          matrix_element, &s_as_ntt->data[j]);
      libcrux_ml_kem_polynomial_add_to_ring_element_d6_1b(&t_as_ntt->data[i0],
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_1c(
    Eurydice_borrow_slice_u8 key_generation_seed, Eurydice_arr_c40 *private_key,
    libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 *public_key) {
  Eurydice_arr_06 hashed =
      libcrux_ml_kem_variant_cpa_keygen_seed_39_9c(key_generation_seed);
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed), (size_t)32U, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 seed_for_A = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_for_secret_and_error = uu____0.snd;
  Eurydice_arr_aa *uu____1 = &public_key->A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue0 =
      libcrux_ml_kem_utils_into_padded_array_b6(seed_for_A);
  libcrux_ml_kem_matrix_sample_matrix_A_2b(uu____1, &lvalue0, true);
  Eurydice_arr_3e prf_input =
      libcrux_ml_kem_utils_into_padded_array_c8(seed_for_secret_and_error);
  uint8_t domain_separator =
      libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_3b(private_key,
                                                           &prf_input, 0U);
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_call_mut_73_1c(&lvalue,
                                                                        i);
  }
  Eurydice_arr_c40 error_as_ntt = arr_struct;
  libcrux_ml_kem_ind_cpa_sample_vector_cbd_then_ntt_3b(
      &error_as_ntt, &prf_input, domain_separator);
  libcrux_ml_kem_matrix_compute_As_plus_e_1b(
      &public_key->t_as_ntt, &public_key->A, private_key, &error_as_ntt);
  Eurydice_arr_600 arr;
  memcpy(arr.data, seed_for_A.ptr, (size_t)32U * sizeof(uint8_t));
  Eurydice_arr_600 uu____2 =
      unwrap_26_07(Result_95_s(Ok, &Result_95_s::U::case_Ok, arr));
  public_key->seed_for_A = uu____2;
}

/**
A monomorphic instance of
libcrux_ml_kem.serialize.serialize_uncompressed_ring_element with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics

*/
static KRML_MUSTINLINE Eurydice_arr_cc
libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_ea(
    const Eurydice_arr_b9 *re) {
  Eurydice_arr_cc serialized = {{0U}};
  for (size_t i = (size_t)0U;
       i < LIBCRUX_ML_KEM_POLYNOMIAL_VECTORS_IN_RING_ELEMENT; i++) {
    size_t i0 = i;
    Eurydice_arr_e2 coefficient =
        libcrux_ml_kem_serialize_to_unsigned_field_modulus_ea(re->data[i0]);
    Eurydice_arr_6d bytes =
        libcrux_ml_kem_vector_portable_serialize_12_b8(coefficient);
    Eurydice_slice_copy(Eurydice_array_to_subslice_mut_3611(
                            &serialized, (core_ops_range_Range_08{
                                             (size_t)24U * i0,
                                             (size_t)24U * i0 + (size_t)24U})),
                        Eurydice_array_to_slice_shared_0b(&bytes), uint8_t);
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_serialize_vector_1b(
    const Eurydice_arr_c40 *key, Eurydice_mut_borrow_slice_u8 out) {
  for (size_t i = (size_t)0U;
       i < Eurydice_slice_len(Eurydice_array_to_slice_shared_cf(key),
                              Eurydice_arr_b9);
       i++) {
    size_t i0 = i;
    Eurydice_arr_b9 re = key->data[i0];
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_7e(
        out, (core_ops_range_Range_08{
                 i0 * LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT,
                 (i0 + (size_t)1U) *
                     LIBCRUX_ML_KEM_CONSTANTS_BYTES_PER_RING_ELEMENT}));
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_cc lvalue =
        libcrux_ml_kem_serialize_serialize_uncompressed_ring_element_ea(&re);
    Eurydice_slice_copy(uu____0, Eurydice_array_to_slice_shared_fe(&lvalue),
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cpa_serialize_public_key_mut_89(
    const Eurydice_arr_c40 *t_as_ntt, Eurydice_borrow_slice_u8 seed_for_a,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cpa_serialize_vector_1b(
      t_as_ntt, Eurydice_array_to_subslice_mut_3612(
                    serialized,
                    (core_ops_range_Range_08{
                        (size_t)0U,
                        libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                            (size_t)3U)})));
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_from_mut_8c2(
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
static KRML_MUSTINLINE Eurydice_arr_74
libcrux_ml_kem_ind_cpa_serialize_public_key_89(
    const Eurydice_arr_c40 *t_as_ntt, Eurydice_borrow_slice_u8 seed_for_a) {
  Eurydice_arr_74 public_key_serialized = {{0U}};
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_89(t_as_ntt, seed_for_a,
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
libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_6c(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
        *public_key,
    const Eurydice_arr_c40 *private_key) {
  Eurydice_arr_74 public_key_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_89(
          &public_key->t_as_ntt,
          Eurydice_array_to_slice_shared_6e(&public_key->seed_for_A));
  Eurydice_arr_60 secret_key_serialized = {{0U}};
  libcrux_ml_kem_ind_cpa_serialize_vector_1b(
      private_key, Eurydice_array_to_slice_mut_06(&secret_key_serialized));
  return (libcrux_ml_kem_utils_extraction_helper_Keypair768{
      secret_key_serialized, public_key_serialized});
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
libcrux_ml_kem_ind_cpa_generate_keypair_ea(
    Eurydice_borrow_slice_u8 key_generation_seed) {
  Eurydice_arr_c40 private_key =
      libcrux_ml_kem_ind_cpa_unpacked_default_70_1b();
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 public_key =
      libcrux_ml_kem_ind_cpa_unpacked_default_8b_1b();
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_1c(
      key_generation_seed, &private_key, &public_key);
  return libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_6c(&public_key,
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
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_d6(
    Eurydice_borrow_slice_u8 private_key, Eurydice_borrow_slice_u8 public_key,
    Eurydice_borrow_slice_u8 implicit_rejection_value,
    Eurydice_arr_ea *serialized) {
  size_t pointer = (size_t)0U;
  Eurydice_arr_ea *uu____0 = serialized;
  size_t uu____1 = pointer;
  size_t uu____2 = pointer;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_3613(
          uu____0,
          (core_ops_range_Range_08{
              uu____1, uu____2 + Eurydice_slice_len(private_key, uint8_t)})),
      private_key, uint8_t);
  pointer = pointer + Eurydice_slice_len(private_key, uint8_t);
  Eurydice_arr_ea *uu____3 = serialized;
  size_t uu____4 = pointer;
  size_t uu____5 = pointer;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_3613(
          uu____3,
          (core_ops_range_Range_08{
              uu____4, uu____5 + Eurydice_slice_len(public_key, uint8_t)})),
      public_key, uint8_t);
  pointer = pointer + Eurydice_slice_len(public_key, uint8_t);
  Eurydice_mut_borrow_slice_u8 uu____6 = Eurydice_array_to_subslice_mut_3613(
      serialized,
      (core_ops_range_Range_08{
          pointer, pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE}));
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_600 lvalue =
      libcrux_ml_kem_hash_functions_portable_H_4a_e0(public_key);
  Eurydice_slice_copy(uu____6, Eurydice_array_to_slice_shared_6e(&lvalue),
                      uint8_t);
  pointer = pointer + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE;
  Eurydice_arr_ea *uu____7 = serialized;
  size_t uu____8 = pointer;
  size_t uu____9 = pointer;
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_3613(
          uu____7,
          (core_ops_range_Range_08{
              uu____8, uu____9 + Eurydice_slice_len(implicit_rejection_value,
                                                    uint8_t)})),
      implicit_rejection_value, uint8_t);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.serialize_kem_secret_key
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
- SERIALIZED_KEY_LEN= 2400
*/
static KRML_MUSTINLINE Eurydice_arr_ea
libcrux_ml_kem_ind_cca_serialize_kem_secret_key_d6(
    Eurydice_borrow_slice_u8 private_key, Eurydice_borrow_slice_u8 public_key,
    Eurydice_borrow_slice_u8 implicit_rejection_value) {
  Eurydice_arr_ea out = {{0U}};
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_d6(
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
libcrux_ml_kem_ind_cca_generate_keypair_15(const Eurydice_arr_06 *randomness) {
  Eurydice_borrow_slice_u8 ind_cpa_keypair_randomness =
      Eurydice_array_to_subslice_shared_363(
          randomness,
          (core_ops_range_Range_08{
              (size_t)0U,
              LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE}));
  Eurydice_borrow_slice_u8 implicit_rejection_value =
      Eurydice_array_to_subslice_from_shared_8c0(
          randomness,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_utils_extraction_helper_Keypair768 uu____0 =
      libcrux_ml_kem_ind_cpa_generate_keypair_ea(ind_cpa_keypair_randomness);
  Eurydice_arr_60 ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_74 public_key = uu____0.snd;
  Eurydice_arr_ea secret_key_serialized =
      libcrux_ml_kem_ind_cca_serialize_kem_secret_key_d6(
          Eurydice_array_to_slice_shared_06(&ind_cpa_private_key),
          Eurydice_array_to_slice_shared_45(&public_key),
          implicit_rejection_value);
  Eurydice_arr_ea private_key =
      libcrux_ml_kem_types_from_77_28(secret_key_serialized);
  return libcrux_ml_kem_types_from_17_74(
      private_key, libcrux_ml_kem_types_from_fd_d0(public_key));
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
libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_ce(
    const Eurydice_arr_06 *randomness) {
  return libcrux_ml_kem_ind_cca_generate_keypair_15(randomness);
}

/**
 Generate ML-KEM 768 Key Pair
*/
static inline libcrux_ml_kem_mlkem768_MlKem768KeyPair
libcrux_ml_kem_mlkem768_portable_generate_key_pair(Eurydice_arr_06 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_generate_keypair_ce(
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
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_private_key_only_d6(
    const Eurydice_arr_ea *private_key) {
  Eurydice_arr_600 t = libcrux_ml_kem_hash_functions_portable_H_4a_e0(
      Eurydice_array_to_subslice_shared_364(
          private_key,
          (core_ops_range_Range_08{(size_t)384U * (size_t)3U,
                                   (size_t)768U * (size_t)3U + (size_t)32U})));
  Eurydice_borrow_slice_u8 expected = Eurydice_array_to_subslice_shared_364(
      private_key,
      (core_ops_range_Range_08{(size_t)768U * (size_t)3U + (size_t)32U,
                               (size_t)768U * (size_t)3U + (size_t)64U}));
  return Eurydice_array_eq_slice((size_t)32U, &t, &expected, uint8_t, bool);
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
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_private_key_37(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *_ciphertext) {
  return libcrux_ml_kem_ind_cca_validate_private_key_only_d6(private_key);
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
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_31(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_validate_private_key_37(private_key,
                                                        ciphertext);
}

/**
 Validate a private key.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool libcrux_ml_kem_mlkem768_portable_validate_private_key(
    const Eurydice_arr_ea *private_key, const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_31(
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
libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_41(
    const Eurydice_arr_ea *private_key) {
  return libcrux_ml_kem_ind_cca_validate_private_key_only_d6(private_key);
}

/**
 Validate the private key only.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool libcrux_ml_kem_mlkem768_portable_validate_private_key_only(
    const Eurydice_arr_ea *private_key) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_private_key_only_41(
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
static inline Eurydice_arr_b9
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_1b(
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
static KRML_MUSTINLINE Eurydice_arr_c40
libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_1b(
    Eurydice_borrow_slice_u8 public_key) {
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_call_mut_0b_1b(
            &lvalue, i);
  }
  Eurydice_arr_c40 deserialized_pk = arr_struct;
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_1b(
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
static KRML_MUSTINLINE bool libcrux_ml_kem_ind_cca_validate_public_key_89(
    const Eurydice_arr_74 *public_key) {
  Eurydice_arr_c40 deserialized_pk =
      libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_out_1b(
          Eurydice_array_to_subslice_to_shared_6e0(
              public_key,
              libcrux_ml_kem_constants_ranked_bytes_per_ring_element(
                  (size_t)3U)));
  Eurydice_arr_74 public_key_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_89(
          &deserialized_pk,
          Eurydice_array_to_subslice_from_shared_8c1(
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
libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_41(
    const Eurydice_arr_74 *public_key) {
  return libcrux_ml_kem_ind_cca_validate_public_key_89(public_key);
}

/**
 Validate a public key.

 Returns `true` if valid, and `false` otherwise.
*/
static inline bool libcrux_ml_kem_mlkem768_portable_validate_public_key(
    const Eurydice_arr_74 *public_key) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_validate_public_key_41(
      public_key);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.MlKemPublicKeyUnpacked
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0_s {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 ind_cpa_public_key;
  Eurydice_arr_600 public_key_hash;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0;

typedef libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768PublicKeyUnpacked;

/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.MlKemPrivateKeyUnpacked with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- $3size_t
*/
typedef struct libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0_s {
  Eurydice_arr_c40 ind_cpa_private_key;
  Eurydice_arr_600 implicit_rejection_value;
} libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0;

typedef struct
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked_s {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0 private_key;
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 public_key;
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
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_ind_cca_unpacked_decapsulate_51(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2c *ciphertext) {
  Eurydice_arr_600 decrypted = libcrux_ml_kem_ind_cpa_decrypt_unpacked_42(
      &key_pair->private_key.ind_cpa_private_key, ciphertext);
  Eurydice_arr_06 to_hash0 = libcrux_ml_kem_utils_into_padded_array_24(
      Eurydice_array_to_slice_shared_6e(&decrypted));
  Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_array_to_subslice_from_mut_8c(
      &to_hash0, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(
      uu____0,
      Eurydice_array_to_slice_shared_6e(&key_pair->public_key.public_key_hash),
      uint8_t);
  Eurydice_arr_06 hashed = libcrux_ml_kem_hash_functions_portable_G_4a_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash0));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____1 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____1.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____1.snd;
  Eurydice_arr_480 to_hash = libcrux_ml_kem_utils_into_padded_array_15(
      Eurydice_array_to_slice_shared_6e(
          &key_pair->private_key.implicit_rejection_value));
  Eurydice_mut_borrow_slice_u8 uu____2 =
      Eurydice_array_to_subslice_from_mut_8c0(
          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  Eurydice_slice_copy(uu____2, libcrux_ml_kem_types_as_ref_d3_80(ciphertext),
                      uint8_t);
  Eurydice_arr_600 implicit_rejection_shared_secret =
      libcrux_ml_kem_hash_functions_portable_PRF_4a_41(
          Eurydice_array_to_slice_shared_74(&to_hash));
  Eurydice_arr_2c expected_ciphertext =
      libcrux_ml_kem_ind_cpa_encrypt_unpacked_2a(
          &key_pair->public_key.ind_cpa_public_key, &decrypted,
          pseudorandomness);
  Eurydice_borrow_slice_u8 uu____3 =
      libcrux_ml_kem_types_as_ref_d3_80(ciphertext);
  uint8_t selector =
      libcrux_ml_kem_constant_time_ops_compare_ciphertexts_in_constant_time(
          uu____3, Eurydice_array_to_slice_shared_42(&expected_ciphertext));
  return libcrux_ml_kem_constant_time_ops_select_shared_secret_in_constant_time(
      shared_secret,
      Eurydice_array_to_slice_shared_6e(&implicit_rejection_shared_secret),
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
static KRML_MUSTINLINE Eurydice_arr_600
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_35(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_unpacked_decapsulate_51(key_pair, ciphertext);
}

/**
 Decapsulate ML-KEM 768 (unpacked)

 Generates an [`MlKemSharedSecret`].
 The input is a reference to an unpacked key pair of type
 [`MlKem768KeyPairUnpacked`] and an [`MlKem768Ciphertext`].
*/
static inline Eurydice_arr_600
libcrux_ml_kem_mlkem768_portable_unpacked_decapsulate(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *private_key,
    const Eurydice_arr_2c *ciphertext) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_decapsulate_35(
      private_key, ciphertext);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.encaps_prepare
with types libcrux_ml_kem_hash_functions_portable_PortableHash[[$3size_t]]
with const generics
- K= 3
*/
static inline Eurydice_arr_06 libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_9c(
    Eurydice_borrow_slice_u8 randomness, Eurydice_borrow_slice_u8 pk_hash) {
  Eurydice_arr_06 to_hash =
      libcrux_ml_kem_utils_into_padded_array_24(randomness);
  Eurydice_slice_copy(Eurydice_array_to_subslice_from_mut_8c(
                          &to_hash, LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE),
                      pk_hash, uint8_t);
  return libcrux_ml_kem_hash_functions_portable_G_4a_e0(
      Eurydice_array_to_slice_shared_d8(&to_hash));
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
static KRML_MUSTINLINE tuple_56 libcrux_ml_kem_ind_cca_unpacked_encapsulate_0c(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *public_key,
    const Eurydice_arr_600 *randomness) {
  Eurydice_arr_06 hashed = libcrux_ml_kem_ind_cca_unpacked_encaps_prepare_9c(
      Eurydice_array_to_slice_shared_6e(randomness),
      Eurydice_array_to_slice_shared_6e(&public_key->public_key_hash));
  Eurydice_dst_ref_shared_uint8_t_size_t_x2 uu____0 = Eurydice_slice_split_at(
      Eurydice_array_to_slice_shared_d8(&hashed),
      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE, uint8_t,
      Eurydice_dst_ref_shared_uint8_t_size_t_x2);
  Eurydice_borrow_slice_u8 shared_secret = uu____0.fst;
  Eurydice_borrow_slice_u8 pseudorandomness = uu____0.snd;
  Eurydice_arr_2c ciphertext = libcrux_ml_kem_ind_cpa_encrypt_unpacked_2a(
      &public_key->ind_cpa_public_key, randomness, pseudorandomness);
  Eurydice_arr_600 shared_secret_array = {{0U}};
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(&shared_secret_array),
                      shared_secret, uint8_t);
  return (tuple_56{libcrux_ml_kem_types_from_e0_80(ciphertext),
                   shared_secret_array});
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
static KRML_MUSTINLINE tuple_56
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_cd(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *public_key,
    const Eurydice_arr_600 *randomness) {
  return libcrux_ml_kem_ind_cca_unpacked_encapsulate_0c(public_key, randomness);
}

/**
 Encapsulate ML-KEM 768 (unpacked)

 Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
 The input is a reference to an unpacked public key of type
 [`MlKem768PublicKeyUnpacked`], the SHA3-256 hash of this public key, and
 [`SHARED_SECRET_SIZE`] bytes of `randomness`.
*/
static inline tuple_56 libcrux_ml_kem_mlkem768_portable_unpacked_encapsulate(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *public_key,
    Eurydice_arr_600 randomness) {
  return libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_encapsulate_cd(
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
static inline Eurydice_arr_b9
libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_1b(
    void **_, size_t tupled_args) {
  return libcrux_ml_kem_polynomial_ZERO_d6_ea();
}

/**
This function found in impl {core::ops::function::FnMut<(usize),
@Array<libcrux_ml_kem::polynomial::PolynomialRingElement<Vector>[TraitClause@0,
TraitClause@1], K>> for
libcrux_ml_kem::ind_cca::unpacked::transpose_a::closure<Vector,
K>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of
libcrux_ml_kem.ind_cca.unpacked.transpose_a.call_mut_7b with types
libcrux_ml_kem_vector_portable_vector_type_PortableVector with const generics
- K= 3
*/
static inline Eurydice_arr_c40
libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_7b_1b(void **_,
                                                           size_t tupled_args) {
  Eurydice_arr_c40 arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cca_unpacked_transpose_a_closure_call_mut_b4_1b(
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
static inline Eurydice_arr_b9 libcrux_ml_kem_polynomial_clone_c1_ea(
    const Eurydice_arr_b9 *self) {
  return core_array__core__clone__Clone_for__Array_T__N___clone(
      (size_t)16U, self, Eurydice_arr_e2, Eurydice_arr_b9);
}

/**
A monomorphic instance of libcrux_ml_kem.ind_cca.unpacked.transpose_a
with types libcrux_ml_kem_vector_portable_vector_type_PortableVector
with const generics
- K= 3
*/
static inline Eurydice_arr_aa libcrux_ml_kem_ind_cca_unpacked_transpose_a_1b(
    Eurydice_arr_aa ind_cpa_a) {
  Eurydice_arr_aa arr_struct;
  for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
    /* original Rust expression is not an lvalue in C */
    void *lvalue = (void *)0U;
    arr_struct.data[i] =
        libcrux_ml_kem_ind_cca_unpacked_transpose_a_call_mut_7b_1b(&lvalue, i);
  }
  Eurydice_arr_aa A = arr_struct;
  for (size_t i0 = (size_t)0U; i0 < (size_t)3U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)3U; i++) {
      size_t j = i;
      Eurydice_arr_b9 uu____0 =
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
static KRML_MUSTINLINE void libcrux_ml_kem_ind_cca_unpacked_generate_keypair_15(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *out) {
  Eurydice_borrow_slice_u8 ind_cpa_keypair_randomness =
      Eurydice_array_to_subslice_shared_363(
          &randomness,
          (core_ops_range_Range_08{
              (size_t)0U,
              LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE}));
  Eurydice_borrow_slice_u8 implicit_rejection_value =
      Eurydice_array_to_subslice_from_shared_8c0(
          &randomness,
          LIBCRUX_ML_KEM_CONSTANTS_CPA_PKE_KEY_GENERATION_SEED_SIZE);
  libcrux_ml_kem_ind_cpa_generate_keypair_unpacked_1c(
      ind_cpa_keypair_randomness, &out->private_key.ind_cpa_private_key,
      &out->public_key.ind_cpa_public_key);
  Eurydice_arr_aa A = libcrux_ml_kem_ind_cca_unpacked_transpose_a_1b(
      out->public_key.ind_cpa_public_key.A);
  out->public_key.ind_cpa_public_key.A = A;
  Eurydice_arr_74 pk_serialized =
      libcrux_ml_kem_ind_cpa_serialize_public_key_89(
          &out->public_key.ind_cpa_public_key.t_as_ntt,
          Eurydice_array_to_slice_shared_6e(
              &out->public_key.ind_cpa_public_key.seed_for_A));
  Eurydice_arr_600 uu____0 = libcrux_ml_kem_hash_functions_portable_H_4a_e0(
      Eurydice_array_to_slice_shared_45(&pk_serialized));
  out->public_key.public_key_hash = uu____0;
  Eurydice_arr_600 arr;
  memcpy(arr.data, implicit_rejection_value.ptr, (size_t)32U * sizeof(uint8_t));
  Eurydice_arr_600 uu____1 =
      unwrap_26_07(Result_95_s(Ok, &Result_95_s::U::case_Ok, arr));
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
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_ce(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *out) {
  libcrux_ml_kem_ind_cca_unpacked_generate_keypair_15(randomness, out);
}

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(
    Eurydice_arr_06 randomness,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_generate_keypair_ce(
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
static KRML_MUSTINLINE libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
libcrux_ml_kem_ind_cca_unpacked_default_30_1b(void) {
  return (libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0{
      libcrux_ml_kem_ind_cpa_unpacked_default_8b_1b(), {{0U}}});
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
    libcrux_ml_kem_ind_cca_unpacked_default_7b_1b(void) {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPrivateKeyUnpacked_a0 uu____0 = {
      libcrux_ml_kem_ind_cpa_unpacked_default_70_1b(), {{0U}}};
  return (libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked{
      uu____0, libcrux_ml_kem_ind_cca_unpacked_default_30_1b()});
}

/**
 Generate ML-KEM 768 Key Pair in "unpacked" form.
*/
static inline libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair(
    Eurydice_arr_06 randomness) {
  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_ind_cca_unpacked_default_7b_1b();
  libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(randomness,
                                                                  &key_pair);
  return key_pair;
}

/**
 Create a new, empty unpacked key.
*/
static inline libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
libcrux_ml_kem_mlkem768_portable_unpacked_init_key_pair(void) {
  return libcrux_ml_kem_ind_cca_unpacked_default_7b_1b();
}

/**
 Create a new, empty unpacked public key.
*/
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
libcrux_ml_kem_mlkem768_portable_unpacked_init_public_key(void) {
  return libcrux_ml_kem_ind_cca_unpacked_default_30_1b();
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
libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_42(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  Eurydice_dst_ref_shared_uint8_t_size_t_x4 uu____0 =
      libcrux_ml_kem_types_unpack_private_key_b4(
          Eurydice_array_to_slice_shared_ec(private_key));
  Eurydice_borrow_slice_u8 ind_cpa_secret_key = uu____0.fst;
  Eurydice_borrow_slice_u8 ind_cpa_public_key = uu____0.snd;
  Eurydice_borrow_slice_u8 ind_cpa_public_key_hash = uu____0.thd;
  Eurydice_borrow_slice_u8 implicit_rejection_value = uu____0.f3;
  libcrux_ml_kem_ind_cpa_deserialize_vector_1b(
      ind_cpa_secret_key, &key_pair->private_key.ind_cpa_private_key);
  libcrux_ml_kem_ind_cpa_build_unpacked_public_key_mut_3f(
      ind_cpa_public_key, &key_pair->public_key.ind_cpa_public_key);
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_6e(&key_pair->public_key.public_key_hash),
      ind_cpa_public_key_hash, uint8_t);
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_6e(
                          &key_pair->private_key.implicit_rejection_value),
                      implicit_rejection_value, uint8_t);
  Eurydice_slice_copy(
      Eurydice_array_to_slice_mut_6e(
          &key_pair->public_key.ind_cpa_public_key.seed_for_A),
      Eurydice_slice_subslice_from_shared_6b(ind_cpa_public_key, (size_t)1152U),
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
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_fd(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  libcrux_ml_kem_ind_cca_unpacked_keys_from_private_key_42(private_key,
                                                           key_pair);
}

/**
 Get an unpacked key from a private key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_from_private_mut(
    const Eurydice_arr_ea *private_key,
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_keypair_from_private_key_fd(
      private_key, key_pair);
}

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
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_43(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self,
    Eurydice_arr_ea *serialized) {
  libcrux_ml_kem_utils_extraction_helper_Keypair768 uu____0 =
      libcrux_ml_kem_ind_cpa_serialize_unpacked_secret_key_6c(
          &self->public_key.ind_cpa_public_key,
          &self->private_key.ind_cpa_private_key);
  Eurydice_arr_60 ind_cpa_private_key = uu____0.fst;
  Eurydice_arr_74 ind_cpa_public_key = uu____0.snd;
  libcrux_ml_kem_ind_cca_serialize_kem_secret_key_mut_d6(
      Eurydice_array_to_slice_shared_06(&ind_cpa_private_key),
      Eurydice_array_to_slice_shared_45(&ind_cpa_public_key),
      Eurydice_array_to_slice_shared_6e(
          &self->private_key.implicit_rejection_value),
      serialized);
}

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
static KRML_MUSTINLINE Eurydice_arr_ea
libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_43(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self) {
  Eurydice_arr_ea sk = libcrux_ml_kem_types_default_d3_28();
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_43(self, &sk);
  return sk;
}

/**
 Get the serialized private key.
*/
static inline Eurydice_arr_ea
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_private_key(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_11_43(key_pair);
}

/**
 Get the serialized private key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_private_key_mut(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    Eurydice_arr_ea *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_private_key_mut_11_43(key_pair,
                                                                   serialized);
}

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
static KRML_MUSTINLINE Eurydice_arr_74
libcrux_ml_kem_ind_cca_unpacked_serialized_dd_89(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *self) {
  return libcrux_ml_kem_types_from_fd_d0(
      libcrux_ml_kem_ind_cpa_serialize_public_key_89(
          &self->ind_cpa_public_key.t_as_ntt,
          Eurydice_array_to_slice_shared_6e(
              &self->ind_cpa_public_key.seed_for_A)));
}

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
static KRML_MUSTINLINE Eurydice_arr_74
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_89(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_dd_89(&self->public_key);
}

/**
 Get the serialized public key.
*/
static inline Eurydice_arr_74
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_public_key(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair) {
  return libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_11_89(key_pair);
}

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
libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_89(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *self,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cpa_serialize_public_key_mut_89(
      &self->ind_cpa_public_key.t_as_ntt,
      Eurydice_array_to_slice_shared_6e(&self->ind_cpa_public_key.seed_for_A),
      serialized);
}

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
libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_89(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *self,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_89(&self->public_key,
                                                       serialized);
}

/**
 Get the serialized public key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_key_pair_serialized_public_key_mut(
    const libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
        *key_pair,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_public_key_mut_11_89(key_pair,
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
static inline libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0
libcrux_ml_kem_ind_cpa_unpacked_clone_91_1b(
    const libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 *self) {
  Eurydice_arr_c40 uu____0 =
      core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)3U, &self->t_as_ntt, Eurydice_arr_b9, Eurydice_arr_c40);
  Eurydice_arr_600 uu____1 =
      core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)32U, &self->seed_for_A, uint8_t, Eurydice_arr_600);
  return (libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0{
      uu____0, uu____1,
      core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)3U, &self->A, Eurydice_arr_c40, Eurydice_arr_aa)});
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
static inline libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
libcrux_ml_kem_ind_cca_unpacked_clone_d7_1b(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *self) {
  libcrux_ml_kem_ind_cpa_unpacked_IndCpaPublicKeyUnpacked_a0 uu____0 =
      libcrux_ml_kem_ind_cpa_unpacked_clone_91_1b(&self->ind_cpa_public_key);
  return (libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0{
      uu____0,
      core_array__core__clone__Clone_for__Array_T__N___clone(
          (size_t)32U, &self->public_key_hash, uint8_t, Eurydice_arr_600)});
}

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
static KRML_MUSTINLINE const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *
libcrux_ml_kem_ind_cca_unpacked_public_key_11_1b(
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
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *pk) {
  libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 uu____0 =
      libcrux_ml_kem_ind_cca_unpacked_clone_d7_1b(
          libcrux_ml_kem_ind_cca_unpacked_public_key_11_1b(key_pair));
  pk[0U] = uu____0;
}

/**
 Get the serialized public key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_serialized_public_key(
    const libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0 *public_key,
    Eurydice_arr_74 *serialized) {
  libcrux_ml_kem_ind_cca_unpacked_serialized_mut_dd_89(public_key, serialized);
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
libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_0a(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
        *unpacked_public_key) {
  Eurydice_borrow_slice_u8 uu____0 =
      Eurydice_array_to_subslice_to_shared_6e0(public_key, (size_t)1152U);
  libcrux_ml_kem_serialize_deserialize_ring_elements_reduced_1b(
      uu____0, &unpacked_public_key->ind_cpa_public_key.t_as_ntt);
  unpacked_public_key->ind_cpa_public_key.seed_for_A =
      libcrux_ml_kem_utils_into_padded_array_9e(
          Eurydice_array_to_subslice_from_shared_8c1(public_key,
                                                     (size_t)1152U));
  Eurydice_arr_aa *uu____2 = &unpacked_public_key->ind_cpa_public_key.A;
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_48 lvalue = libcrux_ml_kem_utils_into_padded_array_b6(
      Eurydice_array_to_subslice_from_shared_8c1(public_key, (size_t)1152U));
  libcrux_ml_kem_matrix_sample_matrix_A_2b(uu____2, &lvalue, false);
  Eurydice_arr_600 uu____3 = libcrux_ml_kem_hash_functions_portable_H_4a_e0(
      Eurydice_array_to_slice_shared_45(
          libcrux_ml_kem_types_as_slice_e6_d0(public_key)));
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
libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_31(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_unpacked_unpack_public_key_0a(public_key,
                                                       unpacked_public_key);
}

/**
 Get the unpacked public key.
*/
static inline void
libcrux_ml_kem_mlkem768_portable_unpacked_unpacked_public_key(
    const Eurydice_arr_74 *public_key,
    libcrux_ml_kem_ind_cca_unpacked_MlKemPublicKeyUnpacked_a0
        *unpacked_public_key) {
  libcrux_ml_kem_ind_cca_instantiations_portable_unpacked_unpack_public_key_31(
      public_key, unpacked_public_key);
}

#define libcrux_mlkem768_portable_H_DEFINED
#endif /* libcrux_mlkem768_portable_H */
