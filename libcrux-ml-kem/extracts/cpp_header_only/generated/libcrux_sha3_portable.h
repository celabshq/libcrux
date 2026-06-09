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

#ifndef libcrux_sha3_portable_H
#define libcrux_sha3_portable_H

#include "eurydice_glue.h"
#include "libcrux_mlkem_core.h"

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
static KRML_MUSTINLINE uint64_t libcrux_sha3_simd_portable_zero_d2(void) {
  return 0ULL;
}

static KRML_MUSTINLINE uint64_t libcrux_sha3_simd_portable__veor5q_u64(
    uint64_t a, uint64_t b, uint64_t c, uint64_t d, uint64_t e) {
  return (((a ^ b) ^ c) ^ d) ^ e;
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
static KRML_MUSTINLINE uint64_t libcrux_sha3_simd_portable_xor5_d2(
    uint64_t a, uint64_t b, uint64_t c, uint64_t d, uint64_t e) {
  return libcrux_sha3_simd_portable__veor5q_u64(a, b, c, d, e);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 1
- RIGHT= 63
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_76(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)1);
}

static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vrax1q_u64(uint64_t a, uint64_t b) {
  return a ^ libcrux_sha3_simd_portable_rotate_left_76(b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left1_and_xor_d2(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vrax1q_u64(a, b);
}

static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vbcaxq_u64(uint64_t a, uint64_t b, uint64_t c) {
  return a ^ (b & ~c);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_and_not_xor_d2(uint64_t a, uint64_t b, uint64_t c) {
  return libcrux_sha3_simd_portable__vbcaxq_u64(a, b, c);
}

static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__veorq_n_u64(uint64_t a, uint64_t c) {
  return a ^ c;
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_constant_d2(uint64_t a, uint64_t c) {
  return libcrux_sha3_simd_portable__veorq_n_u64(a, c);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
static KRML_MUSTINLINE uint64_t libcrux_sha3_simd_portable_xor_d2(uint64_t a,
                                                                  uint64_t b) {
  return a ^ b;
}

#define LIBCRUX_SHA3_GENERIC_KECCAK_CONSTANTS_ROUNDCONSTANTS \
  ((Eurydice_arr_22{{1ULL,                                   \
                     32898ULL,                               \
                     9223372036854808714ULL,                 \
                     9223372039002292224ULL,                 \
                     32907ULL,                               \
                     2147483649ULL,                          \
                     9223372039002292353ULL,                 \
                     9223372036854808585ULL,                 \
                     138ULL,                                 \
                     136ULL,                                 \
                     2147516425ULL,                          \
                     2147483658ULL,                          \
                     2147516555ULL,                          \
                     9223372036854775947ULL,                 \
                     9223372036854808713ULL,                 \
                     9223372036854808579ULL,                 \
                     9223372036854808578ULL,                 \
                     9223372036854775936ULL,                 \
                     32778ULL,                               \
                     9223372039002259466ULL,                 \
                     9223372039002292353ULL,                 \
                     9223372036854808704ULL,                 \
                     2147483649ULL,                          \
                     9223372039002292232ULL}}))

typedef struct size_t_x2_s {
  size_t fst;
  size_t snd;
} size_t_x2;

/**
A monomorphic instance of libcrux_sha3.generic_keccak.KeccakState
with types uint64_t
with const generics
- $1size_t
*/
typedef Eurydice_arr_7c libcrux_sha3_generic_keccak_KeccakState_f3;

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.new_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_sha3_generic_keccak_new_80_71(void) {
  Eurydice_arr_7c lit;
  uint64_t repeat_expression[25U];
  for (size_t i = (size_t)0U; i < (size_t)25U; i++) {
    repeat_expression[i] = libcrux_sha3_simd_portable_zero_d2();
  }
  memcpy(lit.data, repeat_expression, (size_t)25U * sizeof(uint64_t));
  return lit;
}

/**
A monomorphic instance of libcrux_sha3.traits.get_ij
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE const uint64_t *libcrux_sha3_traits_get_ij_71(
    const Eurydice_arr_7c *arr, size_t i, size_t j) {
  return &arr->data[(size_t)5U * j + i];
}

/**
A monomorphic instance of libcrux_sha3.traits.set_ij
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_traits_set_ij_71(Eurydice_arr_7c *arr,
                                                          size_t i, size_t j,
                                                          uint64_t value) {
  arr->data[(size_t)5U * j + i] = value;
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 72
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_block_c6(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start) {
  Eurydice_arr_7c state_flat = {{0U}};
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, (size_t)72U / (size_t)8U}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      for (size_t i = (size_t)0U; i < (size_t)72U / (size_t)8U; i++) {
        size_t i0 = i;
        libcrux_sha3_traits_set_ij_71(
            state, i0 / (size_t)5U, i0 % (size_t)5U,
            libcrux_sha3_traits_get_ij_71(state, i0 / (size_t)5U,
                                          i0 % (size_t)5U)[0U] ^
                state_flat.data[i0]);
      }
      return;
    }
    size_t i = uu____0.f0;
    size_t offset = start + (size_t)8U * i;
    Eurydice_array_u8x8 arr;
    memcpy(arr.data,
           Eurydice_slice_subslice_shared_c8(
               blocks, (core_ops_range_Range_87{offset, offset + (size_t)8U}))
               .ptr,
           (size_t)8U * sizeof(uint8_t));
    Eurydice_array_u8x8 uu____1 =
        unwrap_26_e0(Result_8e_s(Ok, &Result_8e_s::U::case_Ok, arr));
    state_flat.data[i] = core_num__u64__from_le_bytes(uu____1);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 72
- DELIMITER= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_last_dc(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start,
    size_t len) {
  Eurydice_arr_ab buffer = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d4(
          &buffer, (core_ops_range_Range_87{(size_t)0U, len})),
      Eurydice_slice_subslice_shared_c8(
          blocks, (core_ops_range_Range_87{start, start + len})),
      uint8_t);
  buffer.data[len] = 6U;
  size_t uu____0 = (size_t)72U - (size_t)1U;
  buffer.data[uu____0] = (uint32_t)buffer.data[uu____0] | 128U;
  libcrux_sha3_simd_portable_load_block_c6(
      state, Eurydice_array_to_slice_shared_e2(&buffer), (size_t)0U);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_a1
with const generics
- RATE= 72
- DELIMITER= 6
*/
static inline void libcrux_sha3_simd_portable_load_last_a1_dc(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_dc(self, input->data[0U], start, len);
}

/**
This function found in impl {core::ops::index::Index<(usize, usize), T> for
libcrux_sha3::generic_keccak::KeccakState<T, N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.index_c2
with types uint64_t
with const generics
- N= 1
*/
static inline const uint64_t *libcrux_sha3_generic_keccak_index_c2_71(
    const Eurydice_arr_7c *self, size_t_x2 index) {
  return libcrux_sha3_traits_get_ij_71(self, index.fst, index.snd);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.theta_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE Eurydice_arr_84
libcrux_sha3_generic_keccak_theta_80_71(Eurydice_arr_7c *self) {
  Eurydice_arr_84 c = {
      {libcrux_sha3_simd_portable_xor5_d2(
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)0U, (size_t)0U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)1U, (size_t)0U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)2U, (size_t)0U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)3U, (size_t)0U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)4U, (size_t)0U}))[0U]),
       libcrux_sha3_simd_portable_xor5_d2(
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)0U, (size_t)1U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)1U, (size_t)1U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)2U, (size_t)1U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)3U, (size_t)1U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)4U, (size_t)1U}))[0U]),
       libcrux_sha3_simd_portable_xor5_d2(
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)0U, (size_t)2U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)1U, (size_t)2U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)2U, (size_t)2U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)3U, (size_t)2U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)4U, (size_t)2U}))[0U]),
       libcrux_sha3_simd_portable_xor5_d2(
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)0U, (size_t)3U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)1U, (size_t)3U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)2U, (size_t)3U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)3U, (size_t)3U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)4U, (size_t)3U}))[0U]),
       libcrux_sha3_simd_portable_xor5_d2(
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)0U, (size_t)4U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)1U, (size_t)4U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)2U, (size_t)4U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)3U, (size_t)4U}))[0U],
           libcrux_sha3_generic_keccak_index_c2_71(
               self, (size_t_x2{(size_t)4U, (size_t)4U}))[0U])}};
  return (
      Eurydice_arr_84{{libcrux_sha3_simd_portable_rotate_left1_and_xor_d2(
                           c.data[((size_t)0U + (size_t)4U) % (size_t)5U],
                           c.data[((size_t)0U + (size_t)1U) % (size_t)5U]),
                       libcrux_sha3_simd_portable_rotate_left1_and_xor_d2(
                           c.data[((size_t)1U + (size_t)4U) % (size_t)5U],
                           c.data[((size_t)1U + (size_t)1U) % (size_t)5U]),
                       libcrux_sha3_simd_portable_rotate_left1_and_xor_d2(
                           c.data[((size_t)2U + (size_t)4U) % (size_t)5U],
                           c.data[((size_t)2U + (size_t)1U) % (size_t)5U]),
                       libcrux_sha3_simd_portable_rotate_left1_and_xor_d2(
                           c.data[((size_t)3U + (size_t)4U) % (size_t)5U],
                           c.data[((size_t)3U + (size_t)1U) % (size_t)5U]),
                       libcrux_sha3_simd_portable_rotate_left1_and_xor_d2(
                           c.data[((size_t)4U + (size_t)4U) % (size_t)5U],
                           c.data[((size_t)4U + (size_t)1U) % (size_t)5U])}});
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.set_80
with types uint64_t
with const generics
- N= 1
*/
static inline void libcrux_sha3_generic_keccak_set_80_71(Eurydice_arr_7c *self,
                                                         size_t i, size_t j,
                                                         uint64_t v) {
  libcrux_sha3_traits_set_ij_71(self, i, j, v);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 36
- RIGHT= 28
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_02(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)36);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 36
- RIGHT= 28
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_02(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_02(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 36
- RIGHT= 28
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_02(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_02(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 3
- RIGHT= 61
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_ac(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)3);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 3
- RIGHT= 61
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_ac(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_ac(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 3
- RIGHT= 61
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_ac(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_ac(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 41
- RIGHT= 23
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_020(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)41);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 41
- RIGHT= 23
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_020(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_020(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 41
- RIGHT= 23
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_020(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_020(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 18
- RIGHT= 46
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_a9(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)18);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 18
- RIGHT= 46
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_a9(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_a9(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 18
- RIGHT= 46
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_a9(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_a9(a, b);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_0_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_rho_0_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_84 t) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)0U,
      libcrux_sha3_simd_portable_xor_d2(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)0U, (size_t)0U}))[0U],
          t.data[0U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)0U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_02(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)1U, (size_t)0U}))[0U],
          t.data[0U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)0U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_ac(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)2U, (size_t)0U}))[0U],
          t.data[0U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)0U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_020(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)3U, (size_t)0U}))[0U],
          t.data[0U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)0U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_a9(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)4U, (size_t)0U}))[0U],
          t.data[0U]));
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 1
- RIGHT= 63
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_76(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_76(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 1
- RIGHT= 63
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_76(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_76(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 44
- RIGHT= 20
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_58(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)44);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 44
- RIGHT= 20
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_58(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_58(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 44
- RIGHT= 20
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_58(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_58(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 10
- RIGHT= 54
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_e0(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)10);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 10
- RIGHT= 54
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_e0(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_e0(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 10
- RIGHT= 54
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_e0(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_e0(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 45
- RIGHT= 19
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_63(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)45);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 45
- RIGHT= 19
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_63(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_63(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 45
- RIGHT= 19
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_63(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_63(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 2
- RIGHT= 62
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_6a(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)2);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 2
- RIGHT= 62
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_6a(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_6a(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 2
- RIGHT= 62
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_6a(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_6a(a, b);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_1_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_rho_1_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_84 t) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)1U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_76(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)0U, (size_t)1U}))[0U],
          t.data[1U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)1U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_58(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)1U, (size_t)1U}))[0U],
          t.data[1U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)1U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_e0(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)2U, (size_t)1U}))[0U],
          t.data[1U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)1U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_63(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)3U, (size_t)1U}))[0U],
          t.data[1U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)1U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_6a(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)4U, (size_t)1U}))[0U],
          t.data[1U]));
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 62
- RIGHT= 2
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_ab(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)62);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 62
- RIGHT= 2
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_ab(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_ab(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 62
- RIGHT= 2
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_ab(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_ab(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 6
- RIGHT= 58
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_5b(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)6);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 6
- RIGHT= 58
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_5b(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_5b(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 6
- RIGHT= 58
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_5b(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_5b(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 43
- RIGHT= 21
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_6f(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)43);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 43
- RIGHT= 21
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_6f(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_6f(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 43
- RIGHT= 21
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_6f(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_6f(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 15
- RIGHT= 49
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_62(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)15);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 15
- RIGHT= 49
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_62(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_62(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 15
- RIGHT= 49
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_62(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_62(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 61
- RIGHT= 3
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_23(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)61);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 61
- RIGHT= 3
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_23(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_23(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 61
- RIGHT= 3
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_23(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_23(a, b);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_2_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_rho_2_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_84 t) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)2U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_ab(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)0U, (size_t)2U}))[0U],
          t.data[2U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)2U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_5b(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)1U, (size_t)2U}))[0U],
          t.data[2U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)2U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_6f(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)2U, (size_t)2U}))[0U],
          t.data[2U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)2U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_62(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)3U, (size_t)2U}))[0U],
          t.data[2U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)2U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_23(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)4U, (size_t)2U}))[0U],
          t.data[2U]));
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 28
- RIGHT= 36
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_37(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)28);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 28
- RIGHT= 36
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_37(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_37(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 28
- RIGHT= 36
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_37(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_37(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 55
- RIGHT= 9
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_bb(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)55);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 55
- RIGHT= 9
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_bb(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_bb(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 55
- RIGHT= 9
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_bb(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_bb(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 25
- RIGHT= 39
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_b9(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)25);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 25
- RIGHT= 39
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_b9(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_b9(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 25
- RIGHT= 39
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_b9(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_b9(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 21
- RIGHT= 43
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_54(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)21);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 21
- RIGHT= 43
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_54(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_54(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 21
- RIGHT= 43
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_54(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_54(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 56
- RIGHT= 8
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_4c(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)56);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 56
- RIGHT= 8
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_4c(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_4c(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 56
- RIGHT= 8
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_4c(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_4c(a, b);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_3_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_rho_3_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_84 t) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)3U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_37(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)0U, (size_t)3U}))[0U],
          t.data[3U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)3U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_bb(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)1U, (size_t)3U}))[0U],
          t.data[3U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)3U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_b9(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)2U, (size_t)3U}))[0U],
          t.data[3U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)3U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_54(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)3U, (size_t)3U}))[0U],
          t.data[3U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)3U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_4c(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)4U, (size_t)3U}))[0U],
          t.data[3U]));
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 27
- RIGHT= 37
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_ce(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)27);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 27
- RIGHT= 37
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_ce(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_ce(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 27
- RIGHT= 37
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_ce(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_ce(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 20
- RIGHT= 44
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_77(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)20);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 20
- RIGHT= 44
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_77(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_77(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 20
- RIGHT= 44
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_77(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_77(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 39
- RIGHT= 25
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_25(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)39);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 39
- RIGHT= 25
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_25(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_25(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 39
- RIGHT= 25
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_25(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_25(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 8
- RIGHT= 56
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_af(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)8);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 8
- RIGHT= 56
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_af(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_af(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 8
- RIGHT= 56
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_af(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_af(a, b);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 14
- RIGHT= 50
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_rotate_left_fd(uint64_t x) {
  return core_num__u64__rotate_left(x, (uint32_t)14);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 14
- RIGHT= 50
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable__vxarq_u64_fd(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable_rotate_left_fd(a ^ b);
}

/**
This function found in impl {libcrux_sha3::traits::KeccakItem<1usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d2
with const generics
- LEFT= 14
- RIGHT= 50
*/
static KRML_MUSTINLINE uint64_t
libcrux_sha3_simd_portable_xor_and_rotate_d2_fd(uint64_t a, uint64_t b) {
  return libcrux_sha3_simd_portable__vxarq_u64_fd(a, b);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_4_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_rho_4_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_84 t) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)4U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_ce(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)0U, (size_t)4U}))[0U],
          t.data[4U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)4U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_77(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)1U, (size_t)4U}))[0U],
          t.data[4U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)4U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_25(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)2U, (size_t)4U}))[0U],
          t.data[4U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)4U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_af(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)3U, (size_t)4U}))[0U],
          t.data[4U]));
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)4U,
      libcrux_sha3_simd_portable_xor_and_rotate_d2_fd(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)4U, (size_t)4U}))[0U],
          t.data[4U]));
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_rho_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_84 t) {
  libcrux_sha3_generic_keccak_rho_0_80_71(self, t);
  libcrux_sha3_generic_keccak_rho_1_80_71(self, t);
  libcrux_sha3_generic_keccak_rho_2_80_71(self, t);
  libcrux_sha3_generic_keccak_rho_3_80_71(self, t);
  libcrux_sha3_generic_keccak_rho_4_80_71(self, t);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_0_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_pi_0_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_7c old) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)0U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)0U, (size_t)3U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)0U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)0U, (size_t)1U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)0U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)0U, (size_t)4U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)0U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)0U, (size_t)2U}))[0U]);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_1_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_pi_1_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_7c old) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)1U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)1U, (size_t)1U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)1U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)1U, (size_t)4U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)1U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)1U, (size_t)2U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)1U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)1U, (size_t)0U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)1U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)1U, (size_t)3U}))[0U]);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_2_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_pi_2_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_7c old) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)2U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)2U, (size_t)2U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)2U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)2U, (size_t)0U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)2U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)2U, (size_t)3U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)2U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)2U, (size_t)1U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)2U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)2U, (size_t)4U}))[0U]);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_3_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_pi_3_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_7c old) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)3U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)3U, (size_t)3U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)3U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)3U, (size_t)1U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)3U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)3U, (size_t)4U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)3U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)3U, (size_t)2U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)3U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)3U, (size_t)0U}))[0U]);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_4_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_pi_4_80_71(
    Eurydice_arr_7c *self, Eurydice_arr_7c old) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)4U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)4U, (size_t)4U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)1U, (size_t)4U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)4U, (size_t)2U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)2U, (size_t)4U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)4U, (size_t)0U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)3U, (size_t)4U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)4U, (size_t)3U}))[0U]);
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)4U, (size_t)4U,
      libcrux_sha3_generic_keccak_index_c2_71(
          &old, (size_t_x2{(size_t)4U, (size_t)1U}))[0U]);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_pi_80_71(
    Eurydice_arr_7c *self) {
  Eurydice_arr_7c old = self[0U];
  libcrux_sha3_generic_keccak_pi_0_80_71(self, old);
  libcrux_sha3_generic_keccak_pi_1_80_71(self, old);
  libcrux_sha3_generic_keccak_pi_2_80_71(self, old);
  libcrux_sha3_generic_keccak_pi_3_80_71(self, old);
  libcrux_sha3_generic_keccak_pi_4_80_71(self, old);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.chi_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_chi_80_71(
    Eurydice_arr_7c *self) {
  Eurydice_arr_7c old = self[0U];
  for (size_t i0 = (size_t)0U; i0 < (size_t)5U; i0++) {
    size_t i1 = i0;
    for (size_t i = (size_t)0U; i < (size_t)5U; i++) {
      size_t j = i;
      libcrux_sha3_generic_keccak_set_80_71(
          self, i1, j,
          libcrux_sha3_simd_portable_and_not_xor_d2(
              libcrux_sha3_generic_keccak_index_c2_71(self,
                                                      (size_t_x2{i1, j}))[0U],
              libcrux_sha3_generic_keccak_index_c2_71(
                  &old, (size_t_x2{i1, (j + (size_t)2U) % (size_t)5U}))[0U],
              libcrux_sha3_generic_keccak_index_c2_71(
                  &old, (size_t_x2{i1, (j + (size_t)1U) % (size_t)5U}))[0U]));
    }
  }
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.iota_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_iota_80_71(
    Eurydice_arr_7c *self, size_t i) {
  libcrux_sha3_generic_keccak_set_80_71(
      self, (size_t)0U, (size_t)0U,
      libcrux_sha3_simd_portable_xor_constant_d2(
          libcrux_sha3_generic_keccak_index_c2_71(
              self, (size_t_x2{(size_t)0U, (size_t)0U}))[0U],
          LIBCRUX_SHA3_GENERIC_KECCAK_CONSTANTS_ROUNDCONSTANTS.data[i]));
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.keccakf1600_80
with types uint64_t
with const generics
- N= 1
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_keccakf1600_80_71(
    Eurydice_arr_7c *self) {
  for (size_t i = (size_t)0U; i < (size_t)24U; i++) {
    size_t i0 = i;
    Eurydice_arr_84 t = libcrux_sha3_generic_keccak_theta_80_71(self);
    libcrux_sha3_generic_keccak_rho_80_71(self, t);
    libcrux_sha3_generic_keccak_pi_80_71(self);
    libcrux_sha3_generic_keccak_chi_80_71(self);
    libcrux_sha3_generic_keccak_iota_80_71(self, i0);
  }
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_80
with types uint64_t
with const generics
- N= 1
- RATE= 72
- DELIM= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_a1_dc(self, input, start, len);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 72
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_store_block_c6(
    const Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  size_t octets = len / (size_t)8U;
  for (size_t i = (size_t)0U; i < octets; i++) {
    size_t i0 = i;
    Eurydice_array_u8x8 bytes = core_num__u64__to_le_bytes(
        libcrux_sha3_traits_get_ij_71(s, i0 / (size_t)5U, i0 % (size_t)5U)[0U]);
    size_t out_pos = start + (size_t)8U * i0;
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_c8(
            out, (core_ops_range_Range_87{out_pos, out_pos + (size_t)8U})),
        Eurydice_array_to_slice_shared_6e(&bytes), uint8_t);
  }
  size_t remaining = len % (size_t)8U;
  if (remaining > (size_t)0U) {
    Eurydice_array_u8x8 bytes =
        core_num__u64__to_le_bytes(libcrux_sha3_traits_get_ij_71(
            s, octets / (size_t)5U, octets % (size_t)5U)[0U]);
    size_t out_pos = start + len - remaining;
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (core_ops_range_Range_87{out_pos, out_pos + remaining}));
    Eurydice_slice_copy(
        uu____0, Eurydice_array_to_subslice_to_shared_21(&bytes, remaining),
        uint8_t);
  }
}

/**
This function found in impl {libcrux_sha3::traits::Squeeze<u64> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_9b
with const generics
- RATE= 72
*/
static inline void libcrux_sha3_simd_portable_squeeze_9b_c6(
    const Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_store_block_c6(self, out, start, len);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_a1
with const generics
- RATE= 72
*/
static inline void libcrux_sha3_simd_portable_load_block_a1_c6(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_c6(self, input->data[0U], start);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_80
with types uint64_t
with const generics
- N= 1
- RATE= 72
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_block_80_e9(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_a1_c6(self, input, start);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 72
- DELIM= 6
*/
static inline void libcrux_sha3_generic_keccak_portable_keccak1_dc(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 output) {
  Eurydice_arr_7c s = libcrux_sha3_generic_keccak_new_80_71();
  size_t input_len = input.meta;
  size_t input_blocks = input_len / (size_t)72U;
  size_t input_rem = input_len % (size_t)72U;
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, input_blocks}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd(
          &s, &lvalue, input_len - input_rem, input_rem);
      size_t output_len = output.meta;
      size_t output_blocks = output_len / (size_t)72U;
      size_t output_rem = output_len % (size_t)72U;
      if (output_blocks == (size_t)0U) {
        libcrux_sha3_simd_portable_squeeze_9b_c6(&s, output, (size_t)0U,
                                                 output_len);
      } else {
        libcrux_sha3_simd_portable_squeeze_9b_c6(&s, output, (size_t)0U,
                                                 (size_t)72U);
        for (size_t i = (size_t)1U; i < output_blocks; i++) {
          size_t i0 = i;
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_c6(&s, output, i0 * (size_t)72U,
                                                   (size_t)72U);
        }
        if (output_rem != (size_t)0U) {
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_c6(
              &s, output, output_len - output_rem, output_rem);
        }
      }
      return;
    }
    size_t i = uu____0.f0;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_dc lvalue = {{input}};
    libcrux_sha3_generic_keccak_absorb_block_80_e9(&s, &lvalue,
                                                   i * (size_t)72U);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
 A portable SHA3 512 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_sha512(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_dc(data, digest);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 136
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_block_b2(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start) {
  Eurydice_arr_7c state_flat = {{0U}};
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, (size_t)136U / (size_t)8U}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      for (size_t i = (size_t)0U; i < (size_t)136U / (size_t)8U; i++) {
        size_t i0 = i;
        libcrux_sha3_traits_set_ij_71(
            state, i0 / (size_t)5U, i0 % (size_t)5U,
            libcrux_sha3_traits_get_ij_71(state, i0 / (size_t)5U,
                                          i0 % (size_t)5U)[0U] ^
                state_flat.data[i0]);
      }
      return;
    }
    size_t i = uu____0.f0;
    size_t offset = start + (size_t)8U * i;
    Eurydice_array_u8x8 arr;
    memcpy(arr.data,
           Eurydice_slice_subslice_shared_c8(
               blocks, (core_ops_range_Range_87{offset, offset + (size_t)8U}))
               .ptr,
           (size_t)8U * sizeof(uint8_t));
    Eurydice_array_u8x8 uu____1 =
        unwrap_26_e0(Result_8e_s(Ok, &Result_8e_s::U::case_Ok, arr));
    state_flat.data[i] = core_num__u64__from_le_bytes(uu____1);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 136
- DELIMITER= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_last_22(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start,
    size_t len) {
  Eurydice_arr_ff buffer = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d40(
          &buffer, (core_ops_range_Range_87{(size_t)0U, len})),
      Eurydice_slice_subslice_shared_c8(
          blocks, (core_ops_range_Range_87{start, start + len})),
      uint8_t);
  buffer.data[len] = 6U;
  size_t uu____0 = (size_t)136U - (size_t)1U;
  buffer.data[uu____0] = (uint32_t)buffer.data[uu____0] | 128U;
  libcrux_sha3_simd_portable_load_block_b2(
      state, Eurydice_array_to_slice_shared_58(&buffer), (size_t)0U);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_a1
with const generics
- RATE= 136
- DELIMITER= 6
*/
static inline void libcrux_sha3_simd_portable_load_last_a1_22(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_22(self, input->data[0U], start, len);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_80
with types uint64_t
with const generics
- N= 1
- RATE= 136
- DELIM= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd0(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_a1_22(self, input, start, len);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 136
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_store_block_b2(
    const Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  size_t octets = len / (size_t)8U;
  for (size_t i = (size_t)0U; i < octets; i++) {
    size_t i0 = i;
    Eurydice_array_u8x8 bytes = core_num__u64__to_le_bytes(
        libcrux_sha3_traits_get_ij_71(s, i0 / (size_t)5U, i0 % (size_t)5U)[0U]);
    size_t out_pos = start + (size_t)8U * i0;
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_c8(
            out, (core_ops_range_Range_87{out_pos, out_pos + (size_t)8U})),
        Eurydice_array_to_slice_shared_6e(&bytes), uint8_t);
  }
  size_t remaining = len % (size_t)8U;
  if (remaining > (size_t)0U) {
    Eurydice_array_u8x8 bytes =
        core_num__u64__to_le_bytes(libcrux_sha3_traits_get_ij_71(
            s, octets / (size_t)5U, octets % (size_t)5U)[0U]);
    size_t out_pos = start + len - remaining;
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (core_ops_range_Range_87{out_pos, out_pos + remaining}));
    Eurydice_slice_copy(
        uu____0, Eurydice_array_to_subslice_to_shared_21(&bytes, remaining),
        uint8_t);
  }
}

/**
This function found in impl {libcrux_sha3::traits::Squeeze<u64> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_9b
with const generics
- RATE= 136
*/
static inline void libcrux_sha3_simd_portable_squeeze_9b_b2(
    const Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_store_block_b2(self, out, start, len);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_a1
with const generics
- RATE= 136
*/
static inline void libcrux_sha3_simd_portable_load_block_a1_b2(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_b2(self, input->data[0U], start);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_80
with types uint64_t
with const generics
- N= 1
- RATE= 136
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_block_80_e90(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_a1_b2(self, input, start);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 136
- DELIM= 6
*/
static inline void libcrux_sha3_generic_keccak_portable_keccak1_22(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 output) {
  Eurydice_arr_7c s = libcrux_sha3_generic_keccak_new_80_71();
  size_t input_len = input.meta;
  size_t input_blocks = input_len / (size_t)136U;
  size_t input_rem = input_len % (size_t)136U;
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, input_blocks}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd0(
          &s, &lvalue, input_len - input_rem, input_rem);
      size_t output_len = output.meta;
      size_t output_blocks = output_len / (size_t)136U;
      size_t output_rem = output_len % (size_t)136U;
      if (output_blocks == (size_t)0U) {
        libcrux_sha3_simd_portable_squeeze_9b_b2(&s, output, (size_t)0U,
                                                 output_len);
      } else {
        libcrux_sha3_simd_portable_squeeze_9b_b2(&s, output, (size_t)0U,
                                                 (size_t)136U);
        for (size_t i = (size_t)1U; i < output_blocks; i++) {
          size_t i0 = i;
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_b2(
              &s, output, i0 * (size_t)136U, (size_t)136U);
        }
        if (output_rem != (size_t)0U) {
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_b2(
              &s, output, output_len - output_rem, output_rem);
        }
      }
      return;
    }
    size_t i = uu____0.f0;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_dc lvalue = {{input}};
    libcrux_sha3_generic_keccak_absorb_block_80_e90(&s, &lvalue,
                                                    i * (size_t)136U);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
 A portable SHA3 256 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_sha256(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_22(data, digest);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 136
- DELIMITER= 31
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_last_220(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start,
    size_t len) {
  Eurydice_arr_ff buffer = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d40(
          &buffer, (core_ops_range_Range_87{(size_t)0U, len})),
      Eurydice_slice_subslice_shared_c8(
          blocks, (core_ops_range_Range_87{start, start + len})),
      uint8_t);
  buffer.data[len] = 31U;
  size_t uu____0 = (size_t)136U - (size_t)1U;
  buffer.data[uu____0] = (uint32_t)buffer.data[uu____0] | 128U;
  libcrux_sha3_simd_portable_load_block_b2(
      state, Eurydice_array_to_slice_shared_58(&buffer), (size_t)0U);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_a1
with const generics
- RATE= 136
- DELIMITER= 31
*/
static inline void libcrux_sha3_simd_portable_load_last_a1_220(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_220(self, input->data[0U], start, len);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_80
with types uint64_t
with const generics
- N= 1
- RATE= 136
- DELIM= 31
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd1(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_a1_220(self, input, start, len);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 136
- DELIM= 31
*/
static inline void libcrux_sha3_generic_keccak_portable_keccak1_220(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 output) {
  Eurydice_arr_7c s = libcrux_sha3_generic_keccak_new_80_71();
  size_t input_len = input.meta;
  size_t input_blocks = input_len / (size_t)136U;
  size_t input_rem = input_len % (size_t)136U;
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, input_blocks}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd1(
          &s, &lvalue, input_len - input_rem, input_rem);
      size_t output_len = output.meta;
      size_t output_blocks = output_len / (size_t)136U;
      size_t output_rem = output_len % (size_t)136U;
      if (output_blocks == (size_t)0U) {
        libcrux_sha3_simd_portable_squeeze_9b_b2(&s, output, (size_t)0U,
                                                 output_len);
      } else {
        libcrux_sha3_simd_portable_squeeze_9b_b2(&s, output, (size_t)0U,
                                                 (size_t)136U);
        for (size_t i = (size_t)1U; i < output_blocks; i++) {
          size_t i0 = i;
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_b2(
              &s, output, i0 * (size_t)136U, (size_t)136U);
        }
        if (output_rem != (size_t)0U) {
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_b2(
              &s, output, output_len - output_rem, output_rem);
        }
      }
      return;
    }
    size_t i = uu____0.f0;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_dc lvalue = {{input}};
    libcrux_sha3_generic_keccak_absorb_block_80_e90(&s, &lvalue,
                                                    i * (size_t)136U);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
 A portable SHAKE256 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_shake256(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_220(data, digest);
}

typedef libcrux_sha3_generic_keccak_KeccakState_f3
    libcrux_sha3_portable_KeccakState;

/**
 Create a new SHAKE-128 state object.
*/
static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_sha3_portable_incremental_shake128_init(void) {
  return libcrux_sha3_generic_keccak_new_80_71();
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 168
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_block_60(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start) {
  Eurydice_arr_7c state_flat = {{0U}};
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, (size_t)168U / (size_t)8U}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      for (size_t i = (size_t)0U; i < (size_t)168U / (size_t)8U; i++) {
        size_t i0 = i;
        libcrux_sha3_traits_set_ij_71(
            state, i0 / (size_t)5U, i0 % (size_t)5U,
            libcrux_sha3_traits_get_ij_71(state, i0 / (size_t)5U,
                                          i0 % (size_t)5U)[0U] ^
                state_flat.data[i0]);
      }
      return;
    }
    size_t i = uu____0.f0;
    size_t offset = start + (size_t)8U * i;
    Eurydice_array_u8x8 arr;
    memcpy(arr.data,
           Eurydice_slice_subslice_shared_c8(
               blocks, (core_ops_range_Range_87{offset, offset + (size_t)8U}))
               .ptr,
           (size_t)8U * sizeof(uint8_t));
    Eurydice_array_u8x8 uu____1 =
        unwrap_26_e0(Result_8e_s(Ok, &Result_8e_s::U::case_Ok, arr));
    state_flat.data[i] = core_num__u64__from_le_bytes(uu____1);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 168
- DELIMITER= 31
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_last_37(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start,
    size_t len) {
  Eurydice_arr_c5 buffer = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d41(
          &buffer, (core_ops_range_Range_87{(size_t)0U, len})),
      Eurydice_slice_subslice_shared_c8(
          blocks, (core_ops_range_Range_87{start, start + len})),
      uint8_t);
  buffer.data[len] = 31U;
  size_t uu____0 = (size_t)168U - (size_t)1U;
  buffer.data[uu____0] = (uint32_t)buffer.data[uu____0] | 128U;
  libcrux_sha3_simd_portable_load_block_60(
      state, Eurydice_array_to_slice_shared_2c(&buffer), (size_t)0U);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_a1
with const generics
- RATE= 168
- DELIMITER= 31
*/
static inline void libcrux_sha3_simd_portable_load_last_a1_37(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_37(self, input->data[0U], start, len);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_80
with types uint64_t
with const generics
- N= 1
- RATE= 168
- DELIM= 31
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd2(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_a1_37(self, input, start, len);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
 Absorb
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake128_absorb_final(
    Eurydice_arr_7c *s, Eurydice_borrow_slice_u8 data0) {
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_dc lvalue = {{data0}};
  libcrux_sha3_generic_keccak_absorb_final_80_bd2(s, &lvalue, (size_t)0U,
                                                  data0.meta);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 168
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_store_block_60(
    const Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  size_t octets = len / (size_t)8U;
  for (size_t i = (size_t)0U; i < octets; i++) {
    size_t i0 = i;
    Eurydice_array_u8x8 bytes = core_num__u64__to_le_bytes(
        libcrux_sha3_traits_get_ij_71(s, i0 / (size_t)5U, i0 % (size_t)5U)[0U]);
    size_t out_pos = start + (size_t)8U * i0;
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_c8(
            out, (core_ops_range_Range_87{out_pos, out_pos + (size_t)8U})),
        Eurydice_array_to_slice_shared_6e(&bytes), uint8_t);
  }
  size_t remaining = len % (size_t)8U;
  if (remaining > (size_t)0U) {
    Eurydice_array_u8x8 bytes =
        core_num__u64__to_le_bytes(libcrux_sha3_traits_get_ij_71(
            s, octets / (size_t)5U, octets % (size_t)5U)[0U]);
    size_t out_pos = start + len - remaining;
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (core_ops_range_Range_87{out_pos, out_pos + remaining}));
    Eurydice_slice_copy(
        uu____0, Eurydice_array_to_subslice_to_shared_21(&bytes, remaining),
        uint8_t);
  }
}

/**
This function found in impl {libcrux_sha3::traits::Squeeze<u64> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_9b
with const generics
- RATE= 168
*/
static inline void libcrux_sha3_simd_portable_squeeze_9b_60(
    const Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_store_block_60(self, out, start, len);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64,
1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of
libcrux_sha3.generic_keccak.portable.squeeze_first_three_blocks_b4 with const
generics
- RATE= 168
*/
static KRML_MUSTINLINE void
libcrux_sha3_generic_keccak_portable_squeeze_first_three_blocks_b4_60(
    Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)0U, (size_t)168U);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)168U,
                                           (size_t)168U);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)2U * (size_t)168U,
                                           (size_t)168U);
}

/**
 Squeeze three blocks
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake128_squeeze_first_three_blocks(
    Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out0) {
  libcrux_sha3_generic_keccak_portable_squeeze_first_three_blocks_b4_60(s,
                                                                        out0);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64,
1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of
libcrux_sha3.generic_keccak.portable.squeeze_next_block_b4 with const generics
- RATE= 168
*/
static KRML_MUSTINLINE void
libcrux_sha3_generic_keccak_portable_squeeze_next_block_b4_60(
    Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start) {
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, start, (size_t)168U);
}

/**
 Squeeze another block
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
    Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out0) {
  libcrux_sha3_generic_keccak_portable_squeeze_next_block_b4_60(s, out0,
                                                                (size_t)0U);
}

#define libcrux_sha3_Algorithm_Sha224 1
#define libcrux_sha3_Algorithm_Sha256 2
#define libcrux_sha3_Algorithm_Sha384 3
#define libcrux_sha3_Algorithm_Sha512 4

typedef uint8_t libcrux_sha3_Algorithm;

#define LIBCRUX_SHA3_SHA3_224_DIGEST_SIZE ((size_t)28U)

#define LIBCRUX_SHA3_SHA3_256_DIGEST_SIZE ((size_t)32U)

#define LIBCRUX_SHA3_SHA3_384_DIGEST_SIZE ((size_t)48U)

#define LIBCRUX_SHA3_SHA3_512_DIGEST_SIZE ((size_t)64U)

/**
 Returns the output size of a digest.
*/
static inline size_t libcrux_sha3_digest_size(libcrux_sha3_Algorithm mode) {
  switch (mode) {
    case libcrux_sha3_Algorithm_Sha224: {
      break;
    }
    case libcrux_sha3_Algorithm_Sha256: {
      return LIBCRUX_SHA3_SHA3_256_DIGEST_SIZE;
    }
    case libcrux_sha3_Algorithm_Sha384: {
      return LIBCRUX_SHA3_SHA3_384_DIGEST_SIZE;
    }
    case libcrux_sha3_Algorithm_Sha512: {
      return LIBCRUX_SHA3_SHA3_512_DIGEST_SIZE;
    }
    default: {
      KRML_HOST_EPRINTF("KaRaMeL incomplete match at %s:%d\n", __FILE__,
                        __LINE__);
      KRML_HOST_EXIT(253U);
    }
  }
  return LIBCRUX_SHA3_SHA3_224_DIGEST_SIZE;
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 144
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_block_9e(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start) {
  Eurydice_arr_7c state_flat = {{0U}};
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, (size_t)144U / (size_t)8U}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      for (size_t i = (size_t)0U; i < (size_t)144U / (size_t)8U; i++) {
        size_t i0 = i;
        libcrux_sha3_traits_set_ij_71(
            state, i0 / (size_t)5U, i0 % (size_t)5U,
            libcrux_sha3_traits_get_ij_71(state, i0 / (size_t)5U,
                                          i0 % (size_t)5U)[0U] ^
                state_flat.data[i0]);
      }
      return;
    }
    size_t i = uu____0.f0;
    size_t offset = start + (size_t)8U * i;
    Eurydice_array_u8x8 arr;
    memcpy(arr.data,
           Eurydice_slice_subslice_shared_c8(
               blocks, (core_ops_range_Range_87{offset, offset + (size_t)8U}))
               .ptr,
           (size_t)8U * sizeof(uint8_t));
    Eurydice_array_u8x8 uu____1 =
        unwrap_26_e0(Result_8e_s(Ok, &Result_8e_s::U::case_Ok, arr));
    state_flat.data[i] = core_num__u64__from_le_bytes(uu____1);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 144
- DELIMITER= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_last_3a(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start,
    size_t len) {
  Eurydice_arr_f4 buffer = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d42(
          &buffer, (core_ops_range_Range_87{(size_t)0U, len})),
      Eurydice_slice_subslice_shared_c8(
          blocks, (core_ops_range_Range_87{start, start + len})),
      uint8_t);
  buffer.data[len] = 6U;
  size_t uu____0 = (size_t)144U - (size_t)1U;
  buffer.data[uu____0] = (uint32_t)buffer.data[uu____0] | 128U;
  libcrux_sha3_simd_portable_load_block_9e(
      state, Eurydice_array_to_slice_shared_38(&buffer), (size_t)0U);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_a1
with const generics
- RATE= 144
- DELIMITER= 6
*/
static inline void libcrux_sha3_simd_portable_load_last_a1_3a(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_3a(self, input->data[0U], start, len);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_80
with types uint64_t
with const generics
- N= 1
- RATE= 144
- DELIM= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd3(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_a1_3a(self, input, start, len);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 144
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_store_block_9e(
    const Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  size_t octets = len / (size_t)8U;
  for (size_t i = (size_t)0U; i < octets; i++) {
    size_t i0 = i;
    Eurydice_array_u8x8 bytes = core_num__u64__to_le_bytes(
        libcrux_sha3_traits_get_ij_71(s, i0 / (size_t)5U, i0 % (size_t)5U)[0U]);
    size_t out_pos = start + (size_t)8U * i0;
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_c8(
            out, (core_ops_range_Range_87{out_pos, out_pos + (size_t)8U})),
        Eurydice_array_to_slice_shared_6e(&bytes), uint8_t);
  }
  size_t remaining = len % (size_t)8U;
  if (remaining > (size_t)0U) {
    Eurydice_array_u8x8 bytes =
        core_num__u64__to_le_bytes(libcrux_sha3_traits_get_ij_71(
            s, octets / (size_t)5U, octets % (size_t)5U)[0U]);
    size_t out_pos = start + len - remaining;
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (core_ops_range_Range_87{out_pos, out_pos + remaining}));
    Eurydice_slice_copy(
        uu____0, Eurydice_array_to_subslice_to_shared_21(&bytes, remaining),
        uint8_t);
  }
}

/**
This function found in impl {libcrux_sha3::traits::Squeeze<u64> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_9b
with const generics
- RATE= 144
*/
static inline void libcrux_sha3_simd_portable_squeeze_9b_9e(
    const Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_store_block_9e(self, out, start, len);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_a1
with const generics
- RATE= 144
*/
static inline void libcrux_sha3_simd_portable_load_block_a1_9e(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_9e(self, input->data[0U], start);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_80
with types uint64_t
with const generics
- N= 1
- RATE= 144
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_block_80_e91(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_a1_9e(self, input, start);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 144
- DELIM= 6
*/
static inline void libcrux_sha3_generic_keccak_portable_keccak1_3a(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 output) {
  Eurydice_arr_7c s = libcrux_sha3_generic_keccak_new_80_71();
  size_t input_len = input.meta;
  size_t input_blocks = input_len / (size_t)144U;
  size_t input_rem = input_len % (size_t)144U;
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, input_blocks}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd3(
          &s, &lvalue, input_len - input_rem, input_rem);
      size_t output_len = output.meta;
      size_t output_blocks = output_len / (size_t)144U;
      size_t output_rem = output_len % (size_t)144U;
      if (output_blocks == (size_t)0U) {
        libcrux_sha3_simd_portable_squeeze_9b_9e(&s, output, (size_t)0U,
                                                 output_len);
      } else {
        libcrux_sha3_simd_portable_squeeze_9b_9e(&s, output, (size_t)0U,
                                                 (size_t)144U);
        for (size_t i = (size_t)1U; i < output_blocks; i++) {
          size_t i0 = i;
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_9e(
              &s, output, i0 * (size_t)144U, (size_t)144U);
        }
        if (output_rem != (size_t)0U) {
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_9e(
              &s, output, output_len - output_rem, output_rem);
        }
      }
      return;
    }
    size_t i = uu____0.f0;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_dc lvalue = {{input}};
    libcrux_sha3_generic_keccak_absorb_block_80_e91(&s, &lvalue,
                                                    i * (size_t)144U);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
 A portable SHA3 224 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_sha224(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_3a(data, digest);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 104
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_block_53(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start) {
  Eurydice_arr_7c state_flat = {{0U}};
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, (size_t)104U / (size_t)8U}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      for (size_t i = (size_t)0U; i < (size_t)104U / (size_t)8U; i++) {
        size_t i0 = i;
        libcrux_sha3_traits_set_ij_71(
            state, i0 / (size_t)5U, i0 % (size_t)5U,
            libcrux_sha3_traits_get_ij_71(state, i0 / (size_t)5U,
                                          i0 % (size_t)5U)[0U] ^
                state_flat.data[i0]);
      }
      return;
    }
    size_t i = uu____0.f0;
    size_t offset = start + (size_t)8U * i;
    Eurydice_array_u8x8 arr;
    memcpy(arr.data,
           Eurydice_slice_subslice_shared_c8(
               blocks, (core_ops_range_Range_87{offset, offset + (size_t)8U}))
               .ptr,
           (size_t)8U * sizeof(uint8_t));
    Eurydice_array_u8x8 uu____1 =
        unwrap_26_e0(Result_8e_s(Ok, &Result_8e_s::U::case_Ok, arr));
    state_flat.data[i] = core_num__u64__from_le_bytes(uu____1);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 104
- DELIMITER= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_load_last_dc0(
    Eurydice_arr_7c *state, Eurydice_borrow_slice_u8 blocks, size_t start,
    size_t len) {
  Eurydice_arr_c4 buffer = {{0U}};
  Eurydice_slice_copy(
      Eurydice_array_to_subslice_mut_d43(
          &buffer, (core_ops_range_Range_87{(size_t)0U, len})),
      Eurydice_slice_subslice_shared_c8(
          blocks, (core_ops_range_Range_87{start, start + len})),
      uint8_t);
  buffer.data[len] = 6U;
  size_t uu____0 = (size_t)104U - (size_t)1U;
  buffer.data[uu____0] = (uint32_t)buffer.data[uu____0] | 128U;
  libcrux_sha3_simd_portable_load_block_53(
      state, Eurydice_array_to_slice_shared_72(&buffer), (size_t)0U);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_a1
with const generics
- RATE= 104
- DELIMITER= 6
*/
static inline void libcrux_sha3_simd_portable_load_last_a1_dc0(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_dc0(self, input->data[0U], start, len);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_80
with types uint64_t
with const generics
- N= 1
- RATE= 104
- DELIM= 6
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd4(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_load_last_a1_dc0(self, input, start, len);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 104
*/
static KRML_MUSTINLINE void libcrux_sha3_simd_portable_store_block_53(
    const Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  size_t octets = len / (size_t)8U;
  for (size_t i = (size_t)0U; i < octets; i++) {
    size_t i0 = i;
    Eurydice_array_u8x8 bytes = core_num__u64__to_le_bytes(
        libcrux_sha3_traits_get_ij_71(s, i0 / (size_t)5U, i0 % (size_t)5U)[0U]);
    size_t out_pos = start + (size_t)8U * i0;
    Eurydice_slice_copy(
        Eurydice_slice_subslice_mut_c8(
            out, (core_ops_range_Range_87{out_pos, out_pos + (size_t)8U})),
        Eurydice_array_to_slice_shared_6e(&bytes), uint8_t);
  }
  size_t remaining = len % (size_t)8U;
  if (remaining > (size_t)0U) {
    Eurydice_array_u8x8 bytes =
        core_num__u64__to_le_bytes(libcrux_sha3_traits_get_ij_71(
            s, octets / (size_t)5U, octets % (size_t)5U)[0U]);
    size_t out_pos = start + len - remaining;
    Eurydice_mut_borrow_slice_u8 uu____0 = Eurydice_slice_subslice_mut_c8(
        out, (core_ops_range_Range_87{out_pos, out_pos + remaining}));
    Eurydice_slice_copy(
        uu____0, Eurydice_array_to_subslice_to_shared_21(&bytes, remaining),
        uint8_t);
  }
}

/**
This function found in impl {libcrux_sha3::traits::Squeeze<u64> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_9b
with const generics
- RATE= 104
*/
static inline void libcrux_sha3_simd_portable_squeeze_9b_53(
    const Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start,
    size_t len) {
  libcrux_sha3_simd_portable_store_block_53(self, out, start, len);
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_a1
with const generics
- RATE= 104
*/
static inline void libcrux_sha3_simd_portable_load_block_a1_53(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_53(self, input->data[0U], start);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_80
with types uint64_t
with const generics
- N= 1
- RATE= 104
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_block_80_e92(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_a1_53(self, input, start);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 104
- DELIM= 6
*/
static inline void libcrux_sha3_generic_keccak_portable_keccak1_dc0(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 output) {
  Eurydice_arr_7c s = libcrux_sha3_generic_keccak_new_80_71();
  size_t input_len = input.meta;
  size_t input_blocks = input_len / (size_t)104U;
  size_t input_rem = input_len % (size_t)104U;
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, input_blocks}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd4(
          &s, &lvalue, input_len - input_rem, input_rem);
      size_t output_len = output.meta;
      size_t output_blocks = output_len / (size_t)104U;
      size_t output_rem = output_len % (size_t)104U;
      if (output_blocks == (size_t)0U) {
        libcrux_sha3_simd_portable_squeeze_9b_53(&s, output, (size_t)0U,
                                                 output_len);
      } else {
        libcrux_sha3_simd_portable_squeeze_9b_53(&s, output, (size_t)0U,
                                                 (size_t)104U);
        for (size_t i = (size_t)1U; i < output_blocks; i++) {
          size_t i0 = i;
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_53(
              &s, output, i0 * (size_t)104U, (size_t)104U);
        }
        if (output_rem != (size_t)0U) {
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_53(
              &s, output, output_len - output_rem, output_rem);
        }
      }
      return;
    }
    size_t i = uu____0.f0;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_dc lvalue = {{input}};
    libcrux_sha3_generic_keccak_absorb_block_80_e92(&s, &lvalue,
                                                    i * (size_t)104U);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
 A portable SHA3 384 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_sha384(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_dc0(data, digest);
}

/**
 SHA3 224

 Preconditions:
 - `digest.len() == 28`
*/
static inline void libcrux_sha3_sha224_ema(Eurydice_mut_borrow_slice_u8 digest,
                                           Eurydice_borrow_slice_u8 payload) {
  libcrux_sha3_portable_sha224(digest, payload);
}

/**
 SHA3 224
*/
static inline Eurydice_arr_a2 libcrux_sha3_sha224(
    Eurydice_borrow_slice_u8 data) {
  Eurydice_arr_a2 out = {{0U}};
  libcrux_sha3_sha224_ema(Eurydice_array_to_slice_mut_5e(&out), data);
  return out;
}

/**
 SHA3 256
*/
static inline void libcrux_sha3_sha256_ema(Eurydice_mut_borrow_slice_u8 digest,
                                           Eurydice_borrow_slice_u8 payload) {
  libcrux_sha3_portable_sha256(digest, payload);
}

/**
 SHA3 256
*/
static inline Eurydice_arr_ec libcrux_sha3_sha256(
    Eurydice_borrow_slice_u8 data) {
  Eurydice_arr_ec out = {{0U}};
  libcrux_sha3_sha256_ema(Eurydice_array_to_slice_mut_01(&out), data);
  return out;
}

/**
 SHA3 384
*/
static inline void libcrux_sha3_sha384_ema(Eurydice_mut_borrow_slice_u8 digest,
                                           Eurydice_borrow_slice_u8 payload) {
  libcrux_sha3_portable_sha384(digest, payload);
}

/**
 SHA3 384
*/
static inline Eurydice_arr_65 libcrux_sha3_sha384(
    Eurydice_borrow_slice_u8 data) {
  Eurydice_arr_65 out = {{0U}};
  libcrux_sha3_sha384_ema(Eurydice_array_to_slice_mut_9f(&out), data);
  return out;
}

/**
 SHA3 512
*/
static inline void libcrux_sha3_sha512_ema(Eurydice_mut_borrow_slice_u8 digest,
                                           Eurydice_borrow_slice_u8 payload) {
  libcrux_sha3_portable_sha512(digest, payload);
}

/**
 SHA3 512
*/
static inline Eurydice_arr_c7 libcrux_sha3_sha512(
    Eurydice_borrow_slice_u8 data) {
  Eurydice_arr_c7 out = {{0U}};
  libcrux_sha3_sha512_ema(Eurydice_array_to_slice_mut_17(&out), data);
  return out;
}

/**
This function found in impl {libcrux_sha3::traits::Absorb<1usize> for
libcrux_sha3::generic_keccak::KeccakState<u64, 1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_a1
with const generics
- RATE= 168
*/
static inline void libcrux_sha3_simd_portable_load_block_a1_60(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_60(self, input->data[0U], start);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T,
N>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_80
with types uint64_t
with const generics
- N= 1
- RATE= 168
*/
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_block_80_e93(
    Eurydice_arr_7c *self, const Eurydice_arr_dc *input, size_t start) {
  libcrux_sha3_simd_portable_load_block_a1_60(self, input, start);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 168
- DELIM= 31
*/
static inline void libcrux_sha3_generic_keccak_portable_keccak1_37(
    Eurydice_borrow_slice_u8 input, Eurydice_mut_borrow_slice_u8 output) {
  Eurydice_arr_7c s = libcrux_sha3_generic_keccak_new_80_71();
  size_t input_len = input.meta;
  size_t input_blocks = input_len / (size_t)168U;
  size_t input_rem = input_len % (size_t)168U;
  core_ops_range_Range_87 iter =
      core_iter_traits_collect__core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__into_iter(
          (core_ops_range_Range_87{(size_t)0U, input_blocks}),
          core_ops_range_Range_87, size_t, core_ops_range_Range_87);
  while (true) {
    Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, Option_87);
    if (uu____0.tag == None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd2(
          &s, &lvalue, input_len - input_rem, input_rem);
      size_t output_len = output.meta;
      size_t output_blocks = output_len / (size_t)168U;
      size_t output_rem = output_len % (size_t)168U;
      if (output_blocks == (size_t)0U) {
        libcrux_sha3_simd_portable_squeeze_9b_60(&s, output, (size_t)0U,
                                                 output_len);
      } else {
        libcrux_sha3_simd_portable_squeeze_9b_60(&s, output, (size_t)0U,
                                                 (size_t)168U);
        for (size_t i = (size_t)1U; i < output_blocks; i++) {
          size_t i0 = i;
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_60(
              &s, output, i0 * (size_t)168U, (size_t)168U);
        }
        if (output_rem != (size_t)0U) {
          libcrux_sha3_generic_keccak_keccakf1600_80_71(&s);
          libcrux_sha3_simd_portable_squeeze_9b_60(
              &s, output, output_len - output_rem, output_rem);
        }
      }
      return;
    }
    size_t i = uu____0.f0;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_dc lvalue = {{input}};
    libcrux_sha3_generic_keccak_absorb_block_80_e93(&s, &lvalue,
                                                    i * (size_t)168U);
  }
  KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__,
                    "panic!");
  KRML_HOST_EXIT(255U);
}

/**
 A portable SHAKE128 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_shake128(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_37(data, digest);
}

/**
 SHAKE 128

 Writes `out.len()` bytes.
*/
static inline void libcrux_sha3_shake128_ema(Eurydice_mut_borrow_slice_u8 out,
                                             Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_portable_shake128(out, data);
}

/**
 SHAKE 256

 Writes `out.len()` bytes.
*/
static inline void libcrux_sha3_shake256_ema(Eurydice_mut_borrow_slice_u8 out,
                                             Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_portable_shake256(out, data);
}

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.KeccakXofState
with types uint64_t
with const generics
- $1size_t
- $168size_t
*/
typedef struct libcrux_sha3_generic_keccak_xof_KeccakXofState_55_s {
  Eurydice_arr_7c inner;
  Eurydice_arr_88 buf;
  size_t buf_len;
  bool sponge;
} libcrux_sha3_generic_keccak_xof_KeccakXofState_55;

typedef libcrux_sha3_generic_keccak_xof_KeccakXofState_55
    libcrux_sha3_portable_incremental_Shake128Xof;

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.KeccakXofState
with types uint64_t
with const generics
- $1size_t
- $136size_t
*/
typedef struct libcrux_sha3_generic_keccak_xof_KeccakXofState_8d_s {
  Eurydice_arr_7c inner;
  Eurydice_arr_0b buf;
  size_t buf_len;
  bool sponge;
} libcrux_sha3_generic_keccak_xof_KeccakXofState_8d;

typedef libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
    libcrux_sha3_portable_incremental_Shake256Xof;

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64,
1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of
libcrux_sha3.generic_keccak.portable.squeeze_first_five_blocks_b4 with const
generics
- RATE= 168
*/
static KRML_MUSTINLINE void
libcrux_sha3_generic_keccak_portable_squeeze_first_five_blocks_b4_60(
    Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)0U, (size_t)168U);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)168U,
                                           (size_t)168U);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)2U * (size_t)168U,
                                           (size_t)168U);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)3U * (size_t)168U,
                                           (size_t)168U);
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_60(self, out, (size_t)4U * (size_t)168U,
                                           (size_t)168U);
}

/**
 Squeeze five blocks
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(
    Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out0) {
  libcrux_sha3_generic_keccak_portable_squeeze_first_five_blocks_b4_60(s, out0);
}

/**
 Absorb some data for SHAKE-256 for the last time
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake256_absorb_final(
    Eurydice_arr_7c *s, Eurydice_borrow_slice_u8 data) {
  /* original Rust expression is not an lvalue in C */
  Eurydice_arr_dc lvalue = {{data}};
  libcrux_sha3_generic_keccak_absorb_final_80_bd1(s, &lvalue, (size_t)0U,
                                                  data.meta);
}

/**
 Create a new SHAKE-256 state object.
*/
static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_sha3_portable_incremental_shake256_init(void) {
  return libcrux_sha3_generic_keccak_new_80_71();
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64,
1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of
libcrux_sha3.generic_keccak.portable.squeeze_first_block_b4 with const generics
- RATE= 136
*/
static KRML_MUSTINLINE void
libcrux_sha3_generic_keccak_portable_squeeze_first_block_b4_b2(
    const Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_simd_portable_squeeze_9b_b2(self, out, (size_t)0U, (size_t)136U);
}

/**
 Squeeze the first SHAKE-256 block
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
    Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_generic_keccak_portable_squeeze_first_block_b4_b2(&s[0U], out);
}

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64,
1usize>[core::marker::Sized<u64>,
libcrux_sha3::simd::portable::{libcrux_sha3::traits::KeccakItem<1usize> for
u64}]}
*/
/**
A monomorphic instance of
libcrux_sha3.generic_keccak.portable.squeeze_next_block_b4 with const generics
- RATE= 136
*/
static KRML_MUSTINLINE void
libcrux_sha3_generic_keccak_portable_squeeze_next_block_b4_b2(
    Eurydice_arr_7c *self, Eurydice_mut_borrow_slice_u8 out, size_t start) {
  libcrux_sha3_generic_keccak_keccakf1600_80_71(self);
  libcrux_sha3_simd_portable_squeeze_9b_b2(self, out, start, (size_t)136U);
}

/**
 Squeeze the next SHAKE-256 block
*/
static KRML_MUSTINLINE void
libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
    Eurydice_arr_7c *s, Eurydice_mut_borrow_slice_u8 out) {
  libcrux_sha3_generic_keccak_portable_squeeze_next_block_b4_b2(s, out,
                                                                (size_t)0U);
}

/**
A monomorphic instance of Eurydice.arr
with types libcrux_sha3_portable_KeccakState
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_1b_s {
  Eurydice_arr_7c data[3U];
} Eurydice_arr_1b;

#define libcrux_sha3_portable_H_DEFINED
#endif /* libcrux_sha3_portable_H */
