/*
 * SPDX-FileCopyrightText: 2026 CE Labs
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: e656e17bff6ca5efac8ab6919b9b74cb9a8dd8ad
 * Eurydice: aaa9fa657fb6f09802edb890252040d94cd93982
 * Karamel: 8c19d41458ce5cbfea029ebc03334ba96d149039
 * F*: 7b347386330d0e5a331a220535b6f15288903234
 * Libcrux: dirty
 */


#ifndef combined_core_H
#define combined_core_H

#include "eurydice_glue.h"



static inline uint32_t core_num__i32__count_ones(int32_t x0);

static inline uint16_t core_num__u16__wrapping_add(uint16_t x0, uint16_t x1);

static inline uint64_t core_num__u64__from_le_bytes(Eurydice_array_u8x8 x0);

static inline uint64_t core_num__u64__rotate_left(uint64_t x0, uint32_t x1);

static inline Eurydice_array_u8x8 core_num__u64__to_le_bytes(uint64_t x0);

static inline uint32_t core_num__u8__count_ones(uint8_t x0);

static inline uint8_t core_num__u8__wrapping_sub(uint8_t x0, uint8_t x1);

static inline uint8_t
core_ops_bit__core__ops__bit__BitAnd_u8__u8__for__0__u8___bitand(const uint8_t *x0, uint8_t x1);

static inline uint8_t
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
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1568size_t
*/
typedef struct Eurydice_arr_d1_s { uint8_t data[1568U]; } Eurydice_arr_d1;

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1568
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_214(const Eurydice_arr_d1 *a, size_t r)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $3168size_t
*/
typedef struct Eurydice_arr_a8_s { uint8_t data[3168U]; } Eurydice_arr_a8;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 3168
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d412(const Eurydice_arr_a8 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1536size_t
*/
typedef struct Eurydice_arr_df_s { uint8_t data[1536U]; } Eurydice_arr_df;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1536
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_2f0(const Eurydice_arr_df *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1536U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 3168
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d424(Eurydice_arr_a8 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1536
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_2f(Eurydice_arr_df *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1536U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1568
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f8(Eurydice_arr_d1 *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)1568U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1568
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d423(Eurydice_arr_d1 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $352size_t
*/
typedef struct Eurydice_arr_e7_s { uint8_t data[352U]; } Eurydice_arr_e7;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 352
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d422(Eurydice_arr_e7 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 352
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_25(const Eurydice_arr_e7 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)352U;
  return lit;
}

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
- $4size_t
*/
typedef struct Eurydice_arr_3b0_s { Eurydice_arr_89 data[4U]; } Eurydice_arr_3b0;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $33size_t
*/
typedef struct Eurydice_arr_fa_s { uint8_t data[33U]; } Eurydice_arr_fa;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_fa
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_890_s { Eurydice_arr_fa data[4U]; } Eurydice_arr_890;

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $272size_t
*/
typedef struct Eurydice_arr_5b_s { int16_t data[272U]; } Eurydice_arr_5b;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_5b
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_240_s { Eurydice_arr_5b data[4U]; } Eurydice_arr_240;

/**
A monomorphic instance of Eurydice.arr
with types size_t
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_cc_s { size_t data[4U]; } Eurydice_arr_cc;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $504size_t
*/
typedef struct Eurydice_arr_79_s { uint8_t data[504U]; } Eurydice_arr_79;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_79
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_7c0_s { Eurydice_arr_79 data[4U]; } Eurydice_arr_7c0;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $34size_t
*/
typedef struct Eurydice_arr_31_s { uint8_t data[34U]; } Eurydice_arr_31;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_31
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_56_s { Eurydice_arr_31 data[4U]; } Eurydice_arr_56;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1600size_t
*/
typedef struct Eurydice_arr_14_s { uint8_t data[1600U]; } Eurydice_arr_14;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1600
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_720(const Eurydice_arr_14 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1600U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1568
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_b50(const Eurydice_arr_d1 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1568U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1600
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f7(Eurydice_arr_14 *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)1600U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1600
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d421(Eurydice_arr_14 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1568
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f5(const Eurydice_arr_d1 *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)1568U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1568
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d411(const Eurydice_arr_d1 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 3168
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_680(const Eurydice_arr_a8 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3168U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1184size_t
*/
typedef struct Eurydice_arr_5f_s { uint8_t data[1184U]; } Eurydice_arr_5f;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f4(const Eurydice_arr_5f *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)1184U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_213(const Eurydice_arr_5f *a, size_t r)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2400size_t
*/
typedef struct Eurydice_arr_7d_s { uint8_t data[2400U]; } Eurydice_arr_7d;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 2400
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d410(const Eurydice_arr_7d *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_f4(const Eurydice_arr_0e *a)
{
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
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d420(Eurydice_arr_7d *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1152
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f4(Eurydice_arr_0e *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1152U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f6(Eurydice_arr_5f *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)1184U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d419(Eurydice_arr_5f *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1184
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_ff(const Eurydice_arr_5f *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1184U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1088size_t
*/
typedef struct Eurydice_arr_2b_s { uint8_t data[1088U]; } Eurydice_arr_2b;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f5(Eurydice_arr_2b *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)1088U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d418(Eurydice_arr_2b *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_89
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_58_s { Eurydice_arr_89 data[3U]; } Eurydice_arr_58;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_fa
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_801_s { Eurydice_arr_fa data[3U]; } Eurydice_arr_801;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $168size_t
*/
typedef struct Eurydice_arr_c5_s { uint8_t data[168U]; } Eurydice_arr_c5;

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
typedef struct Eurydice_arr_eb0_s { size_t data[3U]; } Eurydice_arr_eb0;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_79
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_7e_s { Eurydice_arr_79 data[3U]; } Eurydice_arr_7e;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_31
with const generics
- $3size_t
*/
typedef struct Eurydice_arr_810_s { Eurydice_arr_31 data[3U]; } Eurydice_arr_810;

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_81(const Eurydice_arr_af *a)
{
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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_06(const Eurydice_arr_2b *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1088U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1120
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f4(Eurydice_arr_af *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)1120U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1120
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d417(Eurydice_arr_af *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f3(const Eurydice_arr_2b *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)1088U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d49(const Eurydice_arr_2b *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2400
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_51(const Eurydice_arr_7d *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2400U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $800size_t
*/
typedef struct Eurydice_arr_03_s { uint8_t data[800U]; } Eurydice_arr_03;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 800
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f2(const Eurydice_arr_03 *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)800U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 800
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_212(const Eurydice_arr_03 *a, size_t r)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1632size_t
*/
typedef struct Eurydice_arr_ab0_s { uint8_t data[1632U]; } Eurydice_arr_ab0;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1632
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d48(const Eurydice_arr_ab0 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1632
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d416(Eurydice_arr_ab0 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_94_s { uint8_t data[24U]; } Eurydice_arr_94;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 24
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_ed(const Eurydice_arr_94 *a)
{
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
typedef struct Eurydice_arr_b20_s { uint8_t data[384U]; } Eurydice_arr_b20;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 384
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d415(Eurydice_arr_b20 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 384
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_a9(const Eurydice_arr_b20 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)384U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_ec_s { uint8_t data[32U]; } Eurydice_arr_ec;

#define core_result_Ok 0
#define core_result_Err 1

typedef uint8_t core_result_Result_07_tags;

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_ec, core_array_TryFromSliceError

*/
typedef struct core_result_Result_07_s
{
  core_result_Result_07_tags tag;
  union U {
    Eurydice_arr_ec case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_07_s)
}
core_result_Result_07;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$32size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_ec core_result_unwrap_26_39(core_result_Result_07 self)
{
  if (self.tag == core_result_Ok)
  {
    return self.val.case_Ok;
  }
  else
  {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $64size_t
*/
typedef struct Eurydice_arr_c7_s { uint8_t data[64U]; } Eurydice_arr_c7;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f1(const Eurydice_arr_c7 *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)64U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d47(const Eurydice_arr_c7 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $768size_t
*/
typedef struct Eurydice_arr_d2_s { uint8_t data[768U]; } Eurydice_arr_d2;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 768
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f3(Eurydice_arr_d2 *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)768U - r });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $10size_t
*/
typedef struct Eurydice_arr_6d_s { uint8_t data[10U]; } Eurydice_arr_6d;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 10
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_30(const Eurydice_arr_6d *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)10U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 768
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d414(Eurydice_arr_d2 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $22size_t
*/
typedef struct Eurydice_arr_80_s { uint8_t data[22U]; } Eurydice_arr_80;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 22
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_980(const Eurydice_arr_80 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)22U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $20size_t
*/
typedef struct Eurydice_arr_fc_s { uint8_t data[20U]; } Eurydice_arr_fc;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 20
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_8f(const Eurydice_arr_fc *a)
{
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
typedef struct Eurydice_arr_b0_s { uint8_t data[320U]; } Eurydice_arr_b0;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 320
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d413(Eurydice_arr_b0 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 320
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_56(const Eurydice_arr_b0 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)320U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_89
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_f3_s { Eurydice_arr_89 data[2U]; } Eurydice_arr_f3;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $192size_t
*/
typedef struct Eurydice_arr_1c_s { uint8_t data[192U]; } Eurydice_arr_1c;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 192
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_d9(const Eurydice_arr_1c *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)192U;
  return lit;
}

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
static inline Eurydice_borrow_slice_i16
Eurydice_array_to_slice_shared_990(const Eurydice_arr_04 *a)
{
  Eurydice_borrow_slice_i16 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)256U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_1c
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_eb_s { Eurydice_arr_1c data[2U]; } Eurydice_arr_eb;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 192
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_d9(Eurydice_arr_1c *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)192U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 33
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_b5(const Eurydice_arr_fa *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)33U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_fa
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_4d0_s { Eurydice_arr_fa data[2U]; } Eurydice_arr_4d0;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 33
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d412(Eurydice_arr_fa *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
static inline Eurydice_borrow_slice_i16
Eurydice_array_to_subslice_shared_e70(const Eurydice_arr_5b *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_i16{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_5b0_s { Eurydice_arr_c5 data[2U]; } Eurydice_arr_5b0;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_5b
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_800_s { Eurydice_arr_5b data[2U]; } Eurydice_arr_800;

/**
A monomorphic instance of Eurydice.arr
with types size_t
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_850_s { size_t data[2U]; } Eurydice_arr_850;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
static inline Eurydice_mut_borrow_slice_i16
Eurydice_array_to_subslice_mut_e7(Eurydice_arr_5b *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_i16{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 504
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d46(const Eurydice_arr_79 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_79
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_b8_s { Eurydice_arr_79 data[2U]; } Eurydice_arr_b8;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 504
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_48(Eurydice_arr_79 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)504U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_31
with const generics
- $2size_t
*/
typedef struct Eurydice_arr_bf_s { Eurydice_arr_31 data[2U]; } Eurydice_arr_bf;

/**
A monomorphic instance of Eurydice.slice_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_borrow_slice_u8
Eurydice_slice_subslice_from_shared_6d(Eurydice_borrow_slice_u8 s, size_t r)
{
  return (Eurydice_borrow_slice_u8{ s.ptr + r, s.meta - r });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 800
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_3b(const Eurydice_arr_03 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)800U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 800
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f2(Eurydice_arr_03 *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)800U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 800
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d411(Eurydice_arr_03 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f1(Eurydice_arr_c7 *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)64U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d410(Eurydice_arr_c7 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 768
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f0(const Eurydice_arr_d2 *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)768U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 768
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d45(const Eurydice_arr_d2 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1632
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_99(const Eurydice_arr_ab0 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1632U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 24
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d44(const Eurydice_arr_94 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 24
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d49(Eurydice_arr_94 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_b2_s { uint8_t data[16U]; } Eurydice_arr_b2;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d48(Eurydice_arr_b2 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d43(const Eurydice_arr_38 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 19
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d47(Eurydice_arr_38 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types int32_t, size_t

*/
typedef struct Eurydice_dst_ref_mut_83_s
{
  int32_t *ptr;
  size_t meta;
}
Eurydice_dst_ref_mut_83;

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t

*/
static inline Eurydice_dst_ref_mut_83
Eurydice_slice_subslice_mut_47(Eurydice_dst_ref_mut_83 s, core_ops_range_Range_87 r)
{
  return (Eurydice_dst_ref_mut_83{ s.ptr + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_b2
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_a30_s { Eurydice_arr_b2 data[16U]; } Eurydice_arr_a30;

/**
A monomorphic instance of Eurydice.array_to_subslice_to_mut
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_to_mut_21(Eurydice_arr_ec *a, size_t r)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $4627size_t
*/
typedef struct Eurydice_arr_93_s { uint8_t data[4627U]; } Eurydice_arr_93;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4627
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_11(const Eurydice_arr_93 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4627U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2592size_t
*/
typedef struct Eurydice_arr_43_s { uint8_t data[2592U]; } Eurydice_arr_43;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2592
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_fc(const Eurydice_arr_43 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2592U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $4896size_t
*/
typedef struct Eurydice_arr_e2_s { uint8_t data[4896U]; } Eurydice_arr_e2;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4896
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_f7(const Eurydice_arr_e2 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4896U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types int32_t
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_6c_s { int32_t data[256U]; } Eurydice_arr_6c;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_6c
with const generics
- $8size_t
*/
typedef struct Eurydice_arr_81_s { Eurydice_arr_6c data[8U]; } Eurydice_arr_81;

#define core_option_None 0
#define core_option_Some 1

typedef uint8_t core_option_Option_45_tags;

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
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_11(Eurydice_arr_93 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4627U;
  return lit;
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types Eurydice_arr_6c, size_t

*/
typedef struct Eurydice_dst_ref_shared_20_s
{
  const Eurydice_arr_6c *ptr;
  size_t meta;
}
Eurydice_dst_ref_shared_20;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 8
*/
static inline Eurydice_dst_ref_shared_20
Eurydice_array_to_slice_shared_861(const Eurydice_arr_81 *a)
{
  Eurydice_dst_ref_shared_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types Eurydice_arr_6c, size_t

*/
typedef struct Eurydice_dst_ref_mut_20_s
{
  Eurydice_arr_6c *ptr;
  size_t meta;
}
Eurydice_dst_ref_mut_20;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 8
*/
static inline Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_861(Eurydice_arr_81 *a)
{
  Eurydice_dst_ref_mut_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$64size_t]]

*/
static KRML_MUSTINLINE void
libcrux_secrets_mem_requests_ct_declassify_56(const Eurydice_arr_c7 *val)
{

}

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_68(const Eurydice_arr_1b *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1024U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1024
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_68(Eurydice_arr_1b *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1024U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 2592
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_fc(Eurydice_arr_43 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2592U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 4896
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f7(Eurydice_arr_e2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4896U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $3309size_t
*/
typedef struct Eurydice_arr_0c_s { uint8_t data[3309U]; } Eurydice_arr_0c;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 3309
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_6b(const Eurydice_arr_0c *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3309U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1952size_t
*/
typedef struct Eurydice_arr_29_s { uint8_t data[1952U]; } Eurydice_arr_29;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1952
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_37(const Eurydice_arr_29 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1952U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $4032size_t
*/
typedef struct Eurydice_arr_24_s { uint8_t data[4032U]; } Eurydice_arr_24;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4032
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_98(const Eurydice_arr_24 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4032U;
  return lit;
}

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
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $48size_t
*/
typedef struct Eurydice_arr_65_s { uint8_t data[48U]; } Eurydice_arr_65;

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
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_6b(Eurydice_arr_0c *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3309U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 6
*/
static inline Eurydice_dst_ref_shared_20
Eurydice_array_to_slice_shared_860(const Eurydice_arr_5d0 *a)
{
  Eurydice_dst_ref_shared_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 6
*/
static inline Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_860(Eurydice_arr_5d0 *a)
{
  Eurydice_dst_ref_mut_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)6U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 48
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_9f0(const Eurydice_arr_65 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)48U;
  return lit;
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$48size_t]]

*/
static KRML_MUSTINLINE void
libcrux_secrets_mem_requests_ct_declassify_69(const Eurydice_arr_65 *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1952
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_37(Eurydice_arr_29 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1952U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 4032
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_98(Eurydice_arr_24 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4032U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2420size_t
*/
typedef struct Eurydice_arr_85_s { uint8_t data[2420U]; } Eurydice_arr_85;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2420
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_0d(const Eurydice_arr_85 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2420U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1312size_t
*/
typedef struct Eurydice_arr_02_s { uint8_t data[1312U]; } Eurydice_arr_02;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1312
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_9f(const Eurydice_arr_02 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1312U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2560size_t
*/
typedef struct Eurydice_arr_10_s { uint8_t data[2560U]; } Eurydice_arr_10;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2560
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_34(const Eurydice_arr_10 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2560U;
  return lit;
}

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
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_0d(Eurydice_arr_85 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2420U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 4
*/
static inline Eurydice_dst_ref_shared_20
Eurydice_array_to_slice_shared_86(const Eurydice_arr_b7 *a)
{
  Eurydice_dst_ref_shared_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 4
*/
static inline Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_86(Eurydice_arr_b7 *a)
{
  Eurydice_dst_ref_mut_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 256
*/
static inline Eurydice_dst_ref_mut_83
Eurydice_array_to_subslice_mut_44(Eurydice_arr_6c *a, core_ops_range_Range_87 r)
{
  return (Eurydice_dst_ref_mut_83{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.dst_ref_shared
with types int32_t, size_t

*/
typedef struct Eurydice_dst_ref_shared_83_s
{
  const int32_t *ptr;
  size_t meta;
}
Eurydice_dst_ref_shared_83;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 256
*/
static inline Eurydice_dst_ref_shared_83
Eurydice_array_to_slice_shared_af(const Eurydice_arr_6c *a)
{
  Eurydice_dst_ref_shared_83 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)256U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $136size_t
*/
typedef struct Eurydice_arr_ff_s { uint8_t data[136U]; } Eurydice_arr_ff;

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f(const Eurydice_arr_ff *a, size_t r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r, (size_t)136U - r });
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$32size_t]]

*/
static KRML_MUSTINLINE void
libcrux_secrets_mem_requests_ct_declassify_4b(const Eurydice_arr_ec *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 768
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_27(const Eurydice_arr_d2 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)768U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 768
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_27(Eurydice_arr_d2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)768U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $640size_t
*/
typedef struct Eurydice_arr_20_s { uint8_t data[640U]; } Eurydice_arr_20;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 640
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_4f(const Eurydice_arr_20 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)640U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 640
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_4f(Eurydice_arr_20 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)640U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $576size_t
*/
typedef struct Eurydice_arr_220_s { uint8_t data[576U]; } Eurydice_arr_220;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 576
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_8a0(const Eurydice_arr_220 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)576U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 576
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_8a0(Eurydice_arr_220 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)576U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $11size_t
*/
typedef struct Eurydice_arr_c9_s { uint8_t data[11U]; } Eurydice_arr_c9;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 11
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_2f(const Eurydice_arr_c9 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_79(const Eurydice_arr_82 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1U;
  return lit;
}

/**
 Mark memory as secret.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_classify
with types Eurydice_derefed_slice uint8_t

*/
static KRML_MUSTINLINE void libcrux_secrets_mem_requests_ct_classify_45(const uint8_t (*val)[])
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1312
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f0(Eurydice_arr_02 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1312U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 2560
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_34(Eurydice_arr_10 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2560U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 64
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_17(const Eurydice_arr_c7 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)64U;
  return lit;
}

/**
A monomorphic instance of Eurydice.arr
with types int32_t
with const generics
- $263size_t
*/
typedef struct Eurydice_arr_d0_s { int32_t data[263U]; } Eurydice_arr_d0;

/**
A monomorphic instance of Eurydice.dst_ref_mut
with types Eurydice_arr_d0, size_t

*/
typedef struct Eurydice_dst_ref_mut_33_s
{
  Eurydice_arr_d0 *ptr;
  size_t meta;
}
Eurydice_dst_ref_mut_33;

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
static inline Eurydice_dst_ref_mut_33 Eurydice_array_to_slice_mut_7e(Eurydice_arr_930 *a)
{
  Eurydice_dst_ref_mut_33 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4U;
  return lit;
}

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
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $840size_t
*/
typedef struct Eurydice_arr_d10_s { uint8_t data[840U]; } Eurydice_arr_d10;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 840
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_4c(const Eurydice_arr_d10 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)840U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 34
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_e9(const Eurydice_arr_31 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)34U;
  return lit;
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_derefed_slice uint8_t

*/
static KRML_MUSTINLINE void
libcrux_secrets_mem_requests_ct_declassify_45(const uint8_t (*val)[])
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 263
*/
static inline Eurydice_dst_ref_shared_83
Eurydice_array_to_slice_shared_2c0(const Eurydice_arr_d0 *a)
{
  Eurydice_dst_ref_shared_83 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)263U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types int32_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 263
*/
static inline Eurydice_dst_ref_mut_83
Eurydice_array_to_subslice_from_mut_11(Eurydice_arr_d0 *a, size_t r)
{
  return (Eurydice_dst_ref_mut_83{ a->data + r, (size_t)263U - r });
}

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $66size_t
*/
typedef struct Eurydice_arr_91_s { uint8_t data[66U]; } Eurydice_arr_91;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 66
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_f1(const Eurydice_arr_91 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)66U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 128
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_78(const Eurydice_arr_89 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)128U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 128
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_78(Eurydice_arr_89 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)128U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_82(const Eurydice_array_u8x2 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 32
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_01(const Eurydice_arr_ec *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)32U;
  return lit;
}

/**
 Mark memory as secret.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_classify
with types Eurydice_arr uint8_t[[$32size_t]]

*/
static KRML_MUSTINLINE void
libcrux_secrets_mem_requests_ct_classify_4b(const Eurydice_arr_ec *val)
{

}

typedef struct Eurydice_arr_c5_x4_s
{
  Eurydice_arr_c5 fst;
  Eurydice_arr_c5 snd;
  Eurydice_arr_c5 thd;
  Eurydice_arr_c5 f3;
}
Eurydice_arr_c5_x4;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 840
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_4c(Eurydice_arr_d10 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)840U;
  return lit;
}

typedef struct Eurydice_arr_ff_x4_s
{
  Eurydice_arr_ff fst;
  Eurydice_arr_ff snd;
  Eurydice_arr_ff thd;
  Eurydice_arr_ff f3;
}
Eurydice_arr_ff_x4;

/**
A monomorphic instance of Eurydice.arr
with types int32_t
with const generics
- $8size_t
*/
typedef struct Eurydice_arr_4d_s { int32_t data[8U]; } Eurydice_arr_4d;

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 8
*/
static inline Eurydice_dst_ref_shared_83
Eurydice_array_to_subslice_shared_44(const Eurydice_arr_4d *a, core_ops_range_Range_87 r)
{
  return (Eurydice_dst_ref_shared_83{ a->data + r.start, r.end - r.start });
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types bool

*/
static KRML_MUSTINLINE void libcrux_secrets_mem_requests_ct_declassify_5f(const bool *val)
{

}

typedef struct int32_t_x2_s
{
  int32_t fst;
  int32_t snd;
}
int32_t_x2;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 8
*/
static inline Eurydice_dst_ref_shared_83
Eurydice_array_to_slice_shared_fd(const Eurydice_arr_4d *a)
{
  Eurydice_dst_ref_shared_83 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t

*/
static inline Eurydice_dst_ref_shared_83
Eurydice_slice_subslice_shared_47(Eurydice_dst_ref_shared_83 s, core_ops_range_Range_87 r)
{
  return (Eurydice_dst_ref_shared_83{ s.ptr + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types int32_t
with const generics
- N= 8
*/
static inline Eurydice_dst_ref_mut_83 Eurydice_array_to_slice_mut_fd(Eurydice_arr_4d *a)
{
  Eurydice_dst_ref_mut_83 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_c9

*/
typedef struct core_option_Option_57_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_c9 f0;
}
core_option_Option_57;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 34
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d46(Eurydice_arr_31 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

typedef struct uint8_t_x2_s
{
  uint8_t fst;
  uint8_t snd;
}
uint8_t_x2;

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 66
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d45(Eurydice_arr_91 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
static inline Eurydice_mut_borrow_slice_i16
Eurydice_slice_subslice_mut_a6(Eurydice_mut_borrow_slice_i16 s, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_i16{ s.ptr + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_29(const Eurydice_arr_b2 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_94, core_array_TryFromSliceError

*/
typedef struct core_result_Result_57_s
{
  core_result_Result_07_tags tag;
  union U {
    Eurydice_arr_94 case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_57_s)
}
core_result_Result_57;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$24size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_94 core_result_unwrap_26_78(core_result_Result_57 self)
{
  if (self.tag == core_result_Ok)
  {
    return self.val.case_Ok;
  }
  else
  {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_d6_s { int16_t data[16U]; } Eurydice_arr_d6;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int16_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_i16
Eurydice_array_to_slice_shared_8a(const Eurydice_arr_d6 *a)
{
  Eurydice_borrow_slice_i16 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_fc, core_array_TryFromSliceError

*/
typedef struct core_result_Result_83_s
{
  core_result_Result_07_tags tag;
  union U {
    Eurydice_arr_fc case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_83_s)
}
core_result_Result_83;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$20size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_fc core_result_unwrap_26_7d(core_result_Result_83 self)
{
  if (self.tag == core_result_Ok)
  {
    return self.val.case_Ok;
  }
  else
  {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_6d, core_array_TryFromSliceError

*/
typedef struct core_result_Result_80_s
{
  core_result_Result_07_tags tag;
  union U {
    Eurydice_arr_6d case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_80_s)
}
core_result_Result_80;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$10size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_6d core_result_unwrap_26_63(core_result_Result_80 self)
{
  if (self.tag == core_result_Ok)
  {
    return self.val.case_Ok;
  }
  else
  {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d42(const Eurydice_arr_b2 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 16
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_29(Eurydice_arr_b2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types int16_t
with const generics
- N= 16
*/
static inline Eurydice_mut_borrow_slice_i16 Eurydice_array_to_slice_mut_8a(Eurydice_arr_d6 *a)
{
  Eurydice_mut_borrow_slice_i16 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

typedef struct int16_t_x2_s
{
  int16_t fst;
  int16_t snd;
}
int16_t_x2;

/**
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types uint8_t
with const generics
- N= 24
*/
static KRML_MUSTINLINE Eurydice_arr_94
libcrux_secrets_int_classify_public_declassify_91_ed(Eurydice_arr_94 self)
{
  return self;
}

typedef struct uint8_t_x3_s
{
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
}
uint8_t_x3;

/**
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types uint8_t
with const generics
- N= 20
*/
static KRML_MUSTINLINE Eurydice_arr_fc
libcrux_secrets_int_classify_public_declassify_91_8f(Eurydice_arr_fc self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types uint8_t
with const generics
- N= 10
*/
static KRML_MUSTINLINE Eurydice_arr_6d
libcrux_secrets_int_classify_public_declassify_91_30(Eurydice_arr_6d self)
{
  return self;
}

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
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types uint8_t
with const generics
- N= 8
*/
static KRML_MUSTINLINE Eurydice_array_u8x8
libcrux_secrets_int_classify_public_declassify_91_6e(Eurydice_array_u8x8 self)
{
  return self;
}

typedef struct uint8_t_x4_s
{
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
  uint8_t f3;
}
uint8_t_x4;

/**
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types uint8_t
with const generics
- N= 2
*/
static KRML_MUSTINLINE Eurydice_array_u8x2
libcrux_secrets_int_classify_public_declassify_91_82(Eurydice_array_u8x2 self)
{
  return self;
}

/**
 Classify a mutable reference to a slice
 We define a separate function for this because hax has limited support for &mut-returning functions

 Note that this function has a different signature than the corresponding `check-secret-independence` one.
 Every call to the secret version of this function compiles with this one, but the reverse is not true.
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_mut_slice
with types Eurydice_dst_ref_mut uint8_t size_t

*/
static KRML_MUSTINLINE Eurydice_mut_borrow_slice_u8
libcrux_secrets_int_classify_public_classify_mut_slice_75(Eurydice_mut_borrow_slice_u8 x)
{
  return x;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_fa
with types int16_t
with const generics
- N= 16
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_secrets_int_classify_public_classify_fa_8a(Eurydice_arr_d6 self)
{
  return self;
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_b2
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_87_s { Eurydice_arr_b2 data[256U]; } Eurydice_arr_87;

/**
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types int16_t
with const generics
- N= 16
*/
static KRML_MUSTINLINE Eurydice_arr_d6
libcrux_secrets_int_classify_public_declassify_91_8a(Eurydice_arr_d6 self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for &'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types uint8_t

*/
static KRML_MUSTINLINE Eurydice_borrow_slice_u8
libcrux_secrets_int_classify_public_classify_ref_6d_90(Eurydice_borrow_slice_u8 self)
{
  return self;
}

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
This function found in impl {libcrux_secrets::traits::Declassify<[T; N]> for [T; N]}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_91
with types uint8_t
with const generics
- N= 22
*/
static KRML_MUSTINLINE Eurydice_arr_80
libcrux_secrets_int_classify_public_declassify_91_98(Eurydice_arr_80 self)
{
  return self;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 16
*/
static inline Eurydice_borrow_slice_i16
Eurydice_array_to_subslice_shared_e7(const Eurydice_arr_d6 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_i16{ a->data + r.start, r.end - r.start });
}

typedef struct uint8_t_x11_s
{
  uint8_t fst;
  uint8_t snd;
  uint8_t thd;
  uint8_t f3;
  uint8_t f4;
  uint8_t f5;
  uint8_t f6;
  uint8_t f7;
  uint8_t f8;
  uint8_t f9;
  uint8_t f10;
}
uint8_t_x11;

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for &'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types int16_t

*/
static KRML_MUSTINLINE Eurydice_borrow_slice_i16
libcrux_secrets_int_classify_public_classify_ref_6d_39(Eurydice_borrow_slice_i16 self)
{
  return self;
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
static inline Eurydice_borrow_slice_i16
Eurydice_slice_subslice_shared_a6(Eurydice_borrow_slice_i16 s, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_i16{ s.ptr + r.start, r.end - r.start });
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_arr_d6, core_array_TryFromSliceError

*/
typedef struct core_result_Result_ec_s
{
  core_result_Result_07_tags tag;
  union U {
    Eurydice_arr_d6 case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_ec_s)
}
core_result_Result_ec;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr int16_t[[$16size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_arr_d6 core_result_unwrap_26_d3(core_result_Result_ec self)
{
  if (self.tag == core_result_Ok)
  {
    return self.val.case_Ok;
  }
  else
  {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $128size_t
*/
typedef struct Eurydice_arr_34_s { int16_t data[128U]; } Eurydice_arr_34;

typedef struct libcrux_ml_kem_utils_extraction_helper_Keypair768_s
{
  Eurydice_arr_0e fst;
  Eurydice_arr_5f snd;
}
libcrux_ml_kem_utils_extraction_helper_Keypair768;

typedef struct libcrux_ml_kem_utils_extraction_helper_Keypair512_s
{
  Eurydice_arr_d2 fst;
  Eurydice_arr_03 snd;
}
libcrux_ml_kem_utils_extraction_helper_Keypair512;

typedef struct libcrux_ml_kem_utils_extraction_helper_Keypair1024_s
{
  Eurydice_arr_df fst;
  Eurydice_arr_d1 snd;
}
libcrux_ml_kem_utils_extraction_helper_Keypair1024;

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_d8
with types uint64_t

*/
static KRML_MUSTINLINE uint64_t
libcrux_secrets_int_classify_public_declassify_d8_49(uint64_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_27
with types uint32_t

*/
static KRML_MUSTINLINE uint32_t
libcrux_secrets_int_classify_public_classify_27_df(uint32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_27
with types uint64_t

*/
static KRML_MUSTINLINE uint64_t
libcrux_secrets_int_classify_public_classify_27_49(uint64_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_d8
with types uint16_t

*/
static KRML_MUSTINLINE uint16_t
libcrux_secrets_int_classify_public_declassify_d8_de(uint16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_27
with types uint16_t

*/
static KRML_MUSTINLINE uint16_t
libcrux_secrets_int_classify_public_classify_27_de(uint16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_d8
with types uint32_t

*/
static KRML_MUSTINLINE uint32_t
libcrux_secrets_int_classify_public_declassify_d8_df(uint32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_d8
with types int32_t

*/
static KRML_MUSTINLINE int32_t
libcrux_secrets_int_classify_public_declassify_d8_a8(int32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_27
with types int32_t

*/
static KRML_MUSTINLINE int32_t libcrux_secrets_int_classify_public_classify_27_a8(int32_t self)
{
  return self;
}

/**
 Construct a public integer (identity)
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.secret
with types int16_t

*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_public_integers_secret_39(int16_t x)
{
  return x;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_d8
with types uint8_t

*/
static KRML_MUSTINLINE uint8_t
libcrux_secrets_int_classify_public_declassify_d8_90(uint8_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_27
with types int16_t

*/
static KRML_MUSTINLINE int16_t libcrux_secrets_int_classify_public_classify_27_39(int16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.declassify_d8
with types int16_t

*/
static KRML_MUSTINLINE int16_t
libcrux_secrets_int_classify_public_declassify_d8_39(int16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_27
with types uint8_t

*/
static KRML_MUSTINLINE uint8_t libcrux_secrets_int_classify_public_classify_27_90(uint8_t self)
{
  return self;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d41(const Eurydice_arr_ec *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

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
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d44(Eurydice_arr_ec *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_211(const Eurydice_arr_c5 *a, size_t r)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 168
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_2c(Eurydice_arr_c5 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)168U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d40(const Eurydice_arr_c5 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f0(Eurydice_arr_c5 *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)168U - r });
}

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_c5
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_88_s { Eurydice_arr_c5 data[1U]; } Eurydice_arr_88;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 64
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_17(Eurydice_arr_c7 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)64U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 48
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f(Eurydice_arr_65 *a)
{
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
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_01(Eurydice_arr_ec *a)
{
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
typedef struct Eurydice_arr_a2_s { uint8_t data[28U]; } Eurydice_arr_a2;

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 28
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_5e(Eurydice_arr_a2 *a)
{
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
typedef struct Eurydice_arr_c4_s { uint8_t data[104U]; } Eurydice_arr_c4;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 104
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_72(const Eurydice_arr_c4 *a)
{
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
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d43(Eurydice_arr_c4 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_38(const Eurydice_arr_f4 *a)
{
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
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d42(Eurydice_arr_f4 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_e2(const Eurydice_arr_ab *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)72U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 72
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d41(Eurydice_arr_ab *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_210(const Eurydice_arr_ff *a, size_t r)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 136
*/
static inline Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_58(Eurydice_arr_ff *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)136U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d4(const Eurydice_arr_ff *a, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.slice_subslice_to_mut
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_slice_subslice_to_mut_72(Eurydice_mut_borrow_slice_u8 s, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ s.ptr, r });
}

/**
A monomorphic instance of Eurydice.slice_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_borrow_slice_u8
Eurydice_slice_subslice_to_shared_72(Eurydice_borrow_slice_u8 s, size_t r)
{
  return (Eurydice_borrow_slice_u8{ s.ptr, r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f(Eurydice_arr_ff *a, size_t r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r, (size_t)136U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 8
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_21(const Eurydice_array_u8x8 *a, size_t r)
{
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
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_6e(const Eurydice_array_u8x8 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_slice_subslice_mut_c8(Eurydice_mut_borrow_slice_u8 s, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ s.ptr + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 136
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_58(const Eurydice_arr_ff *a)
{
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
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d40(Eurydice_arr_ff *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_84_s { uint64_t data[5U]; } Eurydice_arr_84;

typedef struct size_t_x2_s
{
  size_t fst;
  size_t snd;
}
size_t_x2;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_borrow_slice_u8
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_dc_s { Eurydice_borrow_slice_u8 data[1U]; } Eurydice_arr_dc;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 168
*/
static inline Eurydice_borrow_slice_u8
Eurydice_array_to_slice_shared_2c(const Eurydice_arr_c5 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)168U;
  return lit;
}

/**
A monomorphic instance of core.result.Result
with types Eurydice_array_u8x8, core_array_TryFromSliceError

*/
typedef struct core_result_Result_8e_s
{
  core_result_Result_07_tags tag;
  union U {
    Eurydice_array_u8x8 case_Ok;
    core_array_TryFromSliceError case_Err;
  }
  val;
  KRML_UNION_CONSTRUCTOR(core_result_Result_8e_s)
}
core_result_Result_8e;

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$8size_t]], core_array_TryFromSliceError

*/
static inline Eurydice_array_u8x8 core_result_unwrap_26_e0(core_result_Result_8e self)
{
  if (self.tag == core_result_Ok)
  {
    return self.val.case_Ok;
  }
  else
  {
    KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "unwrap not Ok");
    KRML_HOST_EXIT(255U);
  }
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
static inline Eurydice_borrow_slice_u8
Eurydice_slice_subslice_shared_c8(Eurydice_borrow_slice_u8 s, core_ops_range_Range_87 r)
{
  return (Eurydice_borrow_slice_u8{ s.ptr + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
static inline Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d4(Eurydice_arr_c5 *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
}

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_22_s { uint64_t data[24U]; } Eurydice_arr_22;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_ff
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_0b_s { Eurydice_arr_ff data[1U]; } Eurydice_arr_0b;

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $25size_t
*/
typedef struct Eurydice_arr_7c_s { uint64_t data[25U]; } Eurydice_arr_7c;


#define combined_core_H_DEFINED
#endif /* combined_core_H */
