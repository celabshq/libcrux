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
 * Libcrux: b209b76dfc2244d99eabd57364312ebebde6ad88
 */


#ifndef internal_combined_core_H
#define internal_combined_core_H

#include "eurydice_glue.h"



#include "../combined_core.h"

static inline uint64_t core_num__u64__rotate_left(uint64_t x0, uint32_t x1);

static inline Eurydice_array_u8x8 core_num__u64__to_le_bytes(uint64_t x0);

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
Eurydice_array_to_subslice_mut_d45(Eurydice_arr_c4 *a, core_ops_range_Range_87 r)
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
Eurydice_array_to_subslice_mut_d44(Eurydice_arr_f4 *a, core_ops_range_Range_87 r)
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
Eurydice_array_to_subslice_mut_d43(Eurydice_arr_ab *a, core_ops_range_Range_87 r)
{
  return (Eurydice_mut_borrow_slice_u8{ a->data + r.start, r.end - r.start });
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


#define internal_combined_core_H_DEFINED
#endif /* internal_combined_core_H */
