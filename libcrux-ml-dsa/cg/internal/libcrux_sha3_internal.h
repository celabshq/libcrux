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

#ifndef internal_libcrux_sha3_internal_H
#define internal_libcrux_sha3_internal_H

#include "../libcrux_sha3_internal.h"
#include "eurydice_glue.h"
#include "internal/libcrux_mldsa_core.h"
#include "libcrux_mldsa_core.h"

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.closure
with const generics
- $1size_t
- $136size_t
*/
typedef const Eurydice_arr_0b
    *libcrux_sha3_generic_keccak_xof_buf_to_slices_closure_94;

/**
This function found in impl {core::ops::function::FnOnce<(usize), &'_ ([u8])>
for libcrux_sha3::generic_keccak::xof::buf_to_slices::closure<0, PARALLEL_LANES,
RATE>}
*/
/**
A monomorphic instance of
libcrux_sha3.generic_keccak.xof.buf_to_slices.call_once_fa with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
static inline Eurydice_borrow_slice_u8
libcrux_sha3_generic_keccak_xof_buf_to_slices_call_once_fa_81(
    const Eurydice_arr_0b *_, size_t _0) {
  return libcrux_sha3_generic_keccak_xof_buf_to_slices_call_mut_2a_81(&_, _0);
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
    core_option_Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, core_option_Option_87);
    if (uu____0.tag == core_option_None) {
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
        core_result_unwrap_26_e0(core_result_Result_8e_s(
            core_result_Ok, &core_result_Result_8e_s::U::case_Ok, arr));
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
      Eurydice_array_to_subslice_mut_d43(
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
static KRML_MUSTINLINE void libcrux_sha3_generic_keccak_absorb_final_80_bd1(
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
    core_option_Option_87 uu____0 =
        core_iter_range__core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___next(
            &iter, size_t, core_option_Option_87);
    if (uu____0.tag == core_option_None) {
      /* original Rust expression is not an lvalue in C */
      Eurydice_arr_dc lvalue = {{input}};
      libcrux_sha3_generic_keccak_absorb_final_80_bd1(
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
 SHA3 512
*/
static inline void libcrux_sha3_sha512_ema(Eurydice_mut_borrow_slice_u8 digest,
                                           Eurydice_borrow_slice_u8 payload) {
  EURYDICE_ASSERT(payload.meta <= CORE_NUM__U32__MAX, "panic!");
  EURYDICE_ASSERT(digest.meta == (size_t)64U, "panic!");
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

#define internal_libcrux_sha3_internal_H_DEFINED
#endif /* internal_libcrux_sha3_internal_H */
