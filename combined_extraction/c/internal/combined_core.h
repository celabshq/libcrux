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
 * Libcrux: 9206402bb781ceb075738adf111bd86f9f767cb1
 */


#ifndef internal_combined_core_H
#define internal_combined_core_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "../combined_core.h"

static inline uint32_t core_num__i32__count_ones(int32_t x0);

static inline uint16_t core_num__u16__wrapping_add(uint16_t x0, uint16_t x1);

static inline uint64_t core_num__u64__from_le_bytes(Eurydice_array_u8x8 x0);

static inline uint64_t core_num__u64__rotate_left(uint64_t x0, uint32_t x1);

static inline Eurydice_array_u8x8 core_num__u64__to_le_bytes(uint64_t x0);

static inline uint32_t core_num__u8__count_ones(uint8_t x0);

static inline uint8_t core_num__u8__wrapping_sub(uint8_t x0, uint8_t x1);

extern uint8_t
core_ops_bit__core__ops__bit__BitAnd_u8__u8__for__0__u8___bitand(const uint8_t *x0, uint8_t x1);

extern uint8_t
core_ops_bit__core__ops__bit__Shr_i32__u8__for__0__u8___shr(const uint8_t *x0, int32_t x1);

/**
A monomorphic instance of core.ops.range.Range
with types size_t

*/
typedef struct core_ops_range_Range_87_s
{
  size_t start;
  size_t end;
}
core_ops_range_Range_87;

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
Eurydice_mut_borrow_slice_i16
Eurydice_slice_subslice_mut_a6(Eurydice_mut_borrow_slice_i16 s, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_b2
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_87_s { Eurydice_arr_b2 data[256U]; } Eurydice_arr_87;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_94_s { uint8_t data[24U]; } Eurydice_arr_94;

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_94, core_array_TryFromSliceError

*/
typedef struct core_result_Result_57_s
{
  core_result_Result_57_tags tag;
  union {
    Eurydice_arr_94 case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
}
core_result_Result_57;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$24size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_94 core_result_unwrap_26_78(core_result_Result_57 self);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $20size_t
*/
typedef struct Eurydice_arr_fc_s { uint8_t data[20U]; } Eurydice_arr_fc;

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_fc, core_array_TryFromSliceError

*/
typedef struct core_result_Result_83_s
{
  core_result_Result_57_tags tag;
  union {
    Eurydice_arr_fc case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
}
core_result_Result_83;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$20size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_fc core_result_unwrap_26_7d(core_result_Result_83 self);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f2(const Eurydice_arr_5f *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_210(const Eurydice_arr_5f *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 2400
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d48(const Eurydice_arr_7d *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1152size_t
*/
typedef struct Eurydice_arr_0e_s { uint8_t data[1152U]; } Eurydice_arr_0e;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1152
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f4(const Eurydice_arr_0e *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 2400
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d417(Eurydice_arr_7d *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1152
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f4(Eurydice_arr_0e *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f4(Eurydice_arr_5f *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d416(Eurydice_arr_5f *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 24
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_ed(const Eurydice_arr_94 *a);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $384size_t
*/
typedef struct Eurydice_arr_b20_s { uint8_t data[384U]; } Eurydice_arr_b20;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 384
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d415(Eurydice_arr_b20 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 384
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_a9(const Eurydice_arr_b20 *a);

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_ec, core_array_TryFromSliceError

*/
typedef struct core_result_Result_07_s
{
  core_result_Result_57_tags tag;
  union {
    Eurydice_arr_ec case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
}
core_result_Result_07;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$32size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_ec core_result_unwrap_26_39(core_result_Result_07 self);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f1(const Eurydice_arr_c7 *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d47(const Eurydice_arr_c7 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1184
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_ff(const Eurydice_arr_5f *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f3(Eurydice_arr_2b *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d414(Eurydice_arr_2b *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 20
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_8f(const Eurydice_arr_fc *a);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $320size_t
*/
typedef struct Eurydice_arr_b0_s { uint8_t data[320U]; } Eurydice_arr_b0;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 320
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d413(Eurydice_arr_b0 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 320
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_56(const Eurydice_arr_b0 *a);

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_04_s { int16_t data[256U]; } Eurydice_arr_04;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int16_t
with const generics
- N= 256
*/
Eurydice_borrow_slice_i16 Eurydice_array_to_slice_shared_99(const Eurydice_arr_04 *a);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $128size_t
*/
typedef struct Eurydice_arr_89_s { uint8_t data[128U]; } Eurydice_arr_89;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_89
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_58_s { Eurydice_arr_89 data[3U]; } Eurydice_arr_58;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $33size_t
*/
typedef struct Eurydice_arr_fa0_s { uint8_t data[33U]; } Eurydice_arr_fa0;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 33
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_b5(const Eurydice_arr_fa0 *a);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_fa0
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_fd_s { Eurydice_arr_fa0 data[3U]; } Eurydice_arr_fd;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 33
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d412(Eurydice_arr_fa0 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $272size_t
*/
typedef struct Eurydice_arr_5b_s { int16_t data[272U]; } Eurydice_arr_5b;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
Eurydice_borrow_slice_i16
Eurydice_array_to_subslice_shared_e70(const Eurydice_arr_5b *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d46(const Eurydice_arr_c5 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_2c_s { Eurydice_arr_c5 data[3U]; } Eurydice_arr_2c;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_5b
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_b1_s { Eurydice_arr_5b data[3U]; } Eurydice_arr_b1;

/**
A monomorphic instance of Eurydice.arr
with types size_t
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_eb_s { size_t data[3U]; } Eurydice_arr_eb;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
Eurydice_mut_borrow_slice_i16
Eurydice_array_to_subslice_mut_e7(Eurydice_arr_5b *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $504size_t
*/
typedef struct Eurydice_arr_79_s { uint8_t data[504U]; } Eurydice_arr_79;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 504
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d45(const Eurydice_arr_79 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_79
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_7e_s { Eurydice_arr_79 data[3U]; } Eurydice_arr_7e;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 504
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_48(Eurydice_arr_79 *a);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_31
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_810_s { Eurydice_arr_31 data[3U]; } Eurydice_arr_810;

/**
A monomorphic instance of Eurydice.slice_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_borrow_slice_u8
Eurydice_slice_subslice_from_shared_6d(Eurydice_borrow_slice_u8 s, size_t r);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1120size_t
*/
typedef struct Eurydice_arr_af_s { uint8_t data[1120U]; } Eurydice_arr_af;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1120
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_81(const Eurydice_arr_af *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1088
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_06(const Eurydice_arr_2b *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1120
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f2(Eurydice_arr_af *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1120
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d411(Eurydice_arr_af *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f1(Eurydice_arr_c7 *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d410(Eurydice_arr_c7 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f0(const Eurydice_arr_2b *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d44(const Eurydice_arr_2b *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2400
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_51(const Eurydice_arr_7d *a);

typedef struct int16_t_x2_s
{
  int16_t fst;
  int16_t snd;
}
int16_t_x2;

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$24size_t]]

*/
Eurydice_arr_94 libcrux_secrets_int_public_integers_declassify_d8_40(Eurydice_arr_94 self);

typedef struct uint8_t_x3_s
{
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
}
uint8_t_x3;

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$20size_t]]

*/
Eurydice_arr_fc libcrux_secrets_int_public_integers_declassify_d8_2b(Eurydice_arr_fc self);

typedef struct uint8_t_x5_s
{
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
  uint8_t f3;
  uint8_t f4;
}
uint8_t_x5;

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$8size_t]]

*/
Eurydice_array_u8x8
libcrux_secrets_int_public_integers_declassify_d8_52(Eurydice_array_u8x8 self);

typedef struct uint8_t_x4_s
{
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
  uint8_t f3;
}
uint8_t_x4;

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$2size_t]]

*/
Eurydice_array_u8x2
libcrux_secrets_int_public_integers_declassify_d8_75(Eurydice_array_u8x2 self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types Eurydice_arr int16_t[[$16size_t]]

*/
Eurydice_arr_d6 libcrux_secrets_int_public_integers_classify_27_4b(Eurydice_arr_d6 self);

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for &'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types uint8_t

*/
Eurydice_borrow_slice_u8
libcrux_secrets_int_classify_public_classify_ref_6d_90(Eurydice_borrow_slice_u8 self);

typedef struct int16_t_x8_s
{
  int16_t fst;
  int16_t snd;
  int16_t thd;
  int16_t f3;
  int16_t f4;
  int16_t f5;
  int16_t f6;
  int16_t f7;
}
int16_t_x8;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 16
*/
Eurydice_borrow_slice_i16
Eurydice_array_to_subslice_shared_e7(const Eurydice_arr_d6 *a, core_ops_range_Range_87 r);

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for &'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types int16_t

*/
Eurydice_borrow_slice_i16
libcrux_secrets_int_classify_public_classify_ref_6d_39(Eurydice_borrow_slice_i16 self);

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
Eurydice_borrow_slice_i16
Eurydice_slice_subslice_shared_a6(Eurydice_borrow_slice_i16 s, core_ops_range_Range_87 r);

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_d6, core_array_TryFromSliceError

*/
typedef struct core_result_Result_ec_s
{
  core_result_Result_57_tags tag;
  union {
    Eurydice_arr_d6 case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
}
core_result_Result_ec;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr int16_t[[$16size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_d6 core_result_unwrap_26_d3(core_result_Result_ec self);

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $128size_t
*/
typedef struct Eurydice_arr_34_s { int16_t data[128U]; } Eurydice_arr_34;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 24
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d43(const Eurydice_arr_94 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 24
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d49(Eurydice_arr_94 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 16
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_29(Eurydice_arr_b2 *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d42(const Eurydice_arr_b2 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d48(Eurydice_arr_b2 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $19size_t
*/
typedef struct Eurydice_arr_38_s { uint8_t data[19U]; } Eurydice_arr_38;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 19
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d41(const Eurydice_arr_38 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 19
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d47(Eurydice_arr_38 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t

*/
Eurydice_dst_ref_mut_83
Eurydice_slice_subslice_mut_47(Eurydice_dst_ref_mut_83 s, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 16
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_29(const Eurydice_arr_b2 *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_to_mut
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_to_mut_21(Eurydice_arr_ec *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4627
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_11(const Eurydice_arr_93 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2592
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_fc(const Eurydice_arr_43 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4896
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f7(const Eurydice_arr_e2 *a);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_6c
with const generics
- $8size_t
*/
typedef struct Eurydice_arr_81_s { Eurydice_arr_6c data[8U]; } Eurydice_arr_81;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_81

*/
typedef struct core_option_Option_45_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_81 f0;
}
core_option_Option_45;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_c7

*/
typedef struct core_option_Option_b2_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_c7 f0;
}
core_option_Option_b2;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 4627
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_11(Eurydice_arr_93 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_20 Eurydice_array_to_slice_shared_861(const Eurydice_arr_81 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 8
*/
Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_861(Eurydice_arr_81 *a);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$64size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_56(const Eurydice_arr_c7 *val);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1024size_t
*/
typedef struct Eurydice_arr_1b_s { uint8_t data[1024U]; } Eurydice_arr_1b;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1024
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_68(const Eurydice_arr_1b *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1024
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_68(Eurydice_arr_1b *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 2592
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_fc(Eurydice_arr_43 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 4896
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f7(Eurydice_arr_e2 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 3309
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_6b(const Eurydice_arr_0c *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1952
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_37(const Eurydice_arr_29 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4032
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_98(const Eurydice_arr_24 *a);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_6c
with const generics
- $6size_t
*/
typedef struct Eurydice_arr_5d0_s { Eurydice_arr_6c data[6U]; } Eurydice_arr_5d0;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_5d0

*/
typedef struct core_option_Option_05_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_5d0 f0;
}
core_option_Option_05;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_65

*/
typedef struct core_option_Option_81_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_65 f0;
}
core_option_Option_81;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 3309
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_6b(Eurydice_arr_0c *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 6
*/
Eurydice_dst_ref_shared_20 Eurydice_array_to_slice_shared_860(const Eurydice_arr_5d0 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 6
*/
Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_860(Eurydice_arr_5d0 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 48
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_9f0(const Eurydice_arr_65 *a);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$48size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_69(const Eurydice_arr_65 *val);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1952
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_37(Eurydice_arr_29 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 4032
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_98(Eurydice_arr_24 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2420
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_0d(const Eurydice_arr_85 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1312
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_9f(const Eurydice_arr_02 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2560
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_34(const Eurydice_arr_10 *a);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_6c
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_b7_s { Eurydice_arr_6c data[4U]; } Eurydice_arr_b7;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_b7

*/
typedef struct core_option_Option_51_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_b7 f0;
}
core_option_Option_51;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_ec

*/
typedef struct core_option_Option_14_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_ec f0;
}
core_option_Option_14;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 2420
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_0d(Eurydice_arr_85 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 4
*/
Eurydice_dst_ref_shared_20 Eurydice_array_to_slice_shared_86(const Eurydice_arr_b7 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 4
*/
Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_86(Eurydice_arr_b7 *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 256
*/
Eurydice_dst_ref_mut_83
Eurydice_array_to_subslice_mut_44(Eurydice_arr_6c *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 256
*/
Eurydice_dst_ref_shared_83 Eurydice_array_to_slice_shared_af(const Eurydice_arr_6c *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f(const Eurydice_arr_ff *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d40(const Eurydice_arr_ff *a, core_ops_range_Range_87 r);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$32size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_4b(const Eurydice_arr_ec *val);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $768size_t
*/
typedef struct Eurydice_arr_d2_s { uint8_t data[768U]; } Eurydice_arr_d2;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 768
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_27(const Eurydice_arr_d2 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 768
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_27(Eurydice_arr_d2 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 640
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_4f(const Eurydice_arr_20 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 640
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_4f(Eurydice_arr_20 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 576
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_8a(const Eurydice_arr_220 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 576
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_8a(Eurydice_arr_220 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 11
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_2f(const Eurydice_arr_c9 *a);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_82_s { uint8_t data[1U]; } Eurydice_arr_82;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_79(const Eurydice_arr_82 *a);

/**
 Mark memory as secret.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_classify
with types Eurydice_derefed_slice uint8_t

*/
void libcrux_secrets_mem_requests_ct_classify_45(const uint8_t (*val)[]);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1312
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f0(Eurydice_arr_02 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 2560
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_34(Eurydice_arr_10 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 64
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_17(const Eurydice_arr_c7 *a);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_d0
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_930_s { Eurydice_arr_d0 data[4U]; } Eurydice_arr_930;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$263size_t]]
with const generics
- N= 4
*/
Eurydice_dst_ref_mut_33 Eurydice_array_to_slice_mut_7e(Eurydice_arr_930 *a);

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types Eurydice_arr_d0, size_t

*/
typedef struct Eurydice_dst_ref_shared_33_s
{
  const Eurydice_arr_d0 *ptr;
  size_t meta;
}
Eurydice_dst_ref_shared_33;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 840
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_4c(const Eurydice_arr_d10 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 34
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e9(const Eurydice_arr_31 *a);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_derefed_slice uint8_t

*/
void libcrux_secrets_mem_requests_ct_declassify_45(const uint8_t (*val)[]);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 263
*/
Eurydice_dst_ref_shared_83 Eurydice_array_to_slice_shared_2c0(const Eurydice_arr_d0 *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types int32_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 263
*/
Eurydice_dst_ref_mut_83 Eurydice_array_to_subslice_from_mut_11(Eurydice_arr_d0 *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 66
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f1(const Eurydice_arr_91 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 128
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_78(const Eurydice_arr_89 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 128
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_78(Eurydice_arr_89 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_82(const Eurydice_array_u8x2 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 32
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_01(const Eurydice_arr_ec *a);

/**
 Mark memory as secret.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_classify
with types Eurydice_arr uint8_t[[$32size_t]]

*/
void libcrux_secrets_mem_requests_ct_classify_4b(const Eurydice_arr_ec *val);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 168
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_2c(Eurydice_arr_c5 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 840
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_4c(Eurydice_arr_d10 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 136
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_58(Eurydice_arr_ff *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d4(const Eurydice_arr_ec *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_ff
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_dc0_s { Eurydice_arr_ff data[4U]; } Eurydice_arr_dc0;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_9c_s { Eurydice_arr_c5 data[4U]; } Eurydice_arr_9c;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_borrow_slice_u8
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_68_s { Eurydice_borrow_slice_u8 data[4U]; } Eurydice_arr_68;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d46(Eurydice_arr_ec *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f0(Eurydice_arr_c5 *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 64
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_17(Eurydice_arr_c7 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 48
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f(Eurydice_arr_65 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 32
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_01(Eurydice_arr_ec *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 28
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_5e(Eurydice_arr_a2 *a);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $104size_t
*/
typedef struct Eurydice_arr_c4_s { uint8_t data[104U]; } Eurydice_arr_c4;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 104
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_72(const Eurydice_arr_c4 *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 104
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d45(Eurydice_arr_c4 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $144size_t
*/
typedef struct Eurydice_arr_f4_s { uint8_t data[144U]; } Eurydice_arr_f4;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 144
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_38(const Eurydice_arr_f4 *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 144
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d44(Eurydice_arr_f4 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $72size_t
*/
typedef struct Eurydice_arr_ab_s { uint8_t data[72U]; } Eurydice_arr_ab;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 72
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e2(const Eurydice_arr_ab *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 72
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d43(Eurydice_arr_ab *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.slice_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_borrow_slice_u8
Eurydice_slice_subslice_to_shared_72(Eurydice_borrow_slice_u8 s, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f(Eurydice_arr_ff *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 8
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_21(const Eurydice_array_u8x8 *a, size_t r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 8
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_6e(const Eurydice_array_u8x8 *a);

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_mut_borrow_slice_u8
Eurydice_slice_subslice_mut_c8(Eurydice_mut_borrow_slice_u8 s, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 136
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_58(const Eurydice_arr_ff *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d42(Eurydice_arr_ff *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 168
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_2c(const Eurydice_arr_c5 *a);

/**
A monomorphic instance of core.result.Result
with types Eurydice_array_u8x8, core_array_TryFromSliceError

*/
typedef struct core_result_Result_8e_s
{
  core_result_Result_57_tags tag;
  union {
    Eurydice_array_u8x8 case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
}
core_result_Result_8e;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$8size_t]], core_array_TryFromSliceError

*/
Eurydice_array_u8x8 core_result_unwrap_26_e0(core_result_Result_8e self);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d41(Eurydice_arr_c5 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_borrow_slice_u8
Eurydice_slice_subslice_shared_c8(Eurydice_borrow_slice_u8 s, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_83
Eurydice_array_to_subslice_shared_44(const Eurydice_arr_4d *a, core_ops_range_Range_87 r);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types bool

*/
void libcrux_secrets_mem_requests_ct_declassify_5f(const bool *val);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_83 Eurydice_array_to_slice_shared_fd(const Eurydice_arr_4d *a);

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t

*/
Eurydice_dst_ref_shared_83
Eurydice_slice_subslice_shared_47(Eurydice_dst_ref_shared_83 s, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types int32_t
with const generics
- N= 8
*/
Eurydice_dst_ref_mut_83 Eurydice_array_to_slice_mut_fd(Eurydice_arr_4d *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 34
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d40(Eurydice_arr_31 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 66
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d4(Eurydice_arr_91 *a, core_ops_range_Range_87 r);

typedef struct libcrux_ml_kem_utils_extraction_helper_Keypair768_s
{
  Eurydice_arr_0e fst;
  Eurydice_arr_5f snd;
}
libcrux_ml_kem_utils_extraction_helper_Keypair768;

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint64_t

*/
uint64_t libcrux_secrets_int_public_integers_declassify_d8_49(uint64_t self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint32_t

*/
uint32_t libcrux_secrets_int_public_integers_classify_27_df(uint32_t self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint64_t

*/
uint64_t libcrux_secrets_int_public_integers_classify_27_49(uint64_t self);

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint16_t

*/
uint16_t libcrux_secrets_int_public_integers_declassify_d8_de(uint16_t self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint16_t

*/
uint16_t libcrux_secrets_int_public_integers_classify_27_de(uint16_t self);

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint32_t

*/
uint32_t libcrux_secrets_int_public_integers_declassify_d8_df(uint32_t self);

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types int32_t

*/
int32_t libcrux_secrets_int_public_integers_declassify_d8_a8(int32_t self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types int32_t

*/
int32_t libcrux_secrets_int_public_integers_classify_27_a8(int32_t self);

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint8_t

*/
uint8_t libcrux_secrets_int_public_integers_declassify_d8_90(uint8_t self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types int16_t

*/
int16_t libcrux_secrets_int_public_integers_classify_27_39(int16_t self);

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types int16_t

*/
int16_t libcrux_secrets_int_public_integers_declassify_d8_39(int16_t self);

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint8_t

*/
uint8_t libcrux_secrets_int_public_integers_classify_27_90(uint8_t self);

#if defined(__cplusplus)
}
#endif

#define internal_combined_core_H_DEFINED
#endif /* internal_combined_core_H */
