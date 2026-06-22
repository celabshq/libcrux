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


#ifndef libcrux_sha3_avx2_H
#define libcrux_sha3_avx2_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "intrinsics/libcrux_intrinsics_avx2.h"

/**
A monomorphic instance of Eurydice.arr
with types core_core_arch_x86___m256i
with const generics
- $25size_t
*/
typedef struct Eurydice_arr_c40_s { __m256i data[25U]; } Eurydice_arr_c40;

/**
A monomorphic instance of libcrux_sha3.generic_keccak.KeccakState
with types core_core_arch_x86___m256i
with const generics
- $4size_t
*/
typedef Eurydice_arr_c40 libcrux_sha3_generic_keccak_KeccakState_7d;

typedef libcrux_sha3_generic_keccak_KeccakState_7d
libcrux_sha3_avx2_x4_incremental_KeccakState;

/**
 Initialise the [`KeccakState`].
*/
Eurydice_arr_c40 libcrux_sha3_avx2_x4_incremental_init(void);

/**
 Absorb
*/
void
libcrux_sha3_avx2_x4_incremental_shake128_absorb_final(
  Eurydice_arr_c40 *s,
  Eurydice_borrow_slice_u8 data0,
  Eurydice_borrow_slice_u8 data1,
  Eurydice_borrow_slice_u8 data2,
  Eurydice_borrow_slice_u8 data3
);

/**
 Absorb
*/
void
libcrux_sha3_avx2_x4_incremental_shake256_absorb_final(
  Eurydice_arr_c40 *s,
  Eurydice_borrow_slice_u8 data0,
  Eurydice_borrow_slice_u8 data1,
  Eurydice_borrow_slice_u8 data2,
  Eurydice_borrow_slice_u8 data3
);

/**
 Perform 4 SHAKE256 operations in parallel
*/
void
libcrux_sha3_avx2_x4_shake256(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_mut_borrow_slice_u8 out0,
  Eurydice_mut_borrow_slice_u8 out1,
  Eurydice_mut_borrow_slice_u8 out2,
  Eurydice_mut_borrow_slice_u8 out3
);

/**
 Squeeze block
*/
void
libcrux_sha3_avx2_x4_incremental_shake256_squeeze_first_block(
  Eurydice_arr_c40 *s,
  Eurydice_mut_borrow_slice_u8 out0,
  Eurydice_mut_borrow_slice_u8 out1,
  Eurydice_mut_borrow_slice_u8 out2,
  Eurydice_mut_borrow_slice_u8 out3
);

/**
 Squeeze five blocks
*/
void
libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_five_blocks(
  Eurydice_arr_c40 *s,
  Eurydice_mut_borrow_slice_u8 out0,
  Eurydice_mut_borrow_slice_u8 out1,
  Eurydice_mut_borrow_slice_u8 out2,
  Eurydice_mut_borrow_slice_u8 out3
);

/**
 Squeeze another block
*/
void
libcrux_sha3_avx2_x4_incremental_shake128_squeeze_next_block(
  Eurydice_arr_c40 *s,
  Eurydice_mut_borrow_slice_u8 out0,
  Eurydice_mut_borrow_slice_u8 out1,
  Eurydice_mut_borrow_slice_u8 out2,
  Eurydice_mut_borrow_slice_u8 out3
);

/**
 Squeeze next block
*/
void
libcrux_sha3_avx2_x4_incremental_shake256_squeeze_next_block(
  Eurydice_arr_c40 *s,
  Eurydice_mut_borrow_slice_u8 out0,
  Eurydice_mut_borrow_slice_u8 out1,
  Eurydice_mut_borrow_slice_u8 out2,
  Eurydice_mut_borrow_slice_u8 out3
);

/**
 Squeeze three blocks
*/
void
libcrux_sha3_avx2_x4_incremental_shake128_squeeze_first_three_blocks(
  Eurydice_arr_c40 *s,
  Eurydice_mut_borrow_slice_u8 out0,
  Eurydice_mut_borrow_slice_u8 out1,
  Eurydice_mut_borrow_slice_u8 out2,
  Eurydice_mut_borrow_slice_u8 out3
);

#if defined(__cplusplus)
}
#endif

#define libcrux_sha3_avx2_H_DEFINED
#endif /* libcrux_sha3_avx2_H */
