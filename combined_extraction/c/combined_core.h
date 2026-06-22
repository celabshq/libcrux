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


#ifndef combined_core_H
#define combined_core_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_b2_s { uint8_t data[16U]; } Eurydice_arr_b2;

#define core_result_Ok 0
#define core_result_Err 1

typedef uint8_t core_result_Result_57_tags;

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_d6_s { int16_t data[16U]; } Eurydice_arr_d6;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1184size_t
*/
typedef struct Eurydice_arr_5f_s { uint8_t data[1184U]; } Eurydice_arr_5f;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2400size_t
*/
typedef struct Eurydice_arr_7d_s { uint8_t data[2400U]; } Eurydice_arr_7d;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $32size_t
*/
typedef struct Eurydice_arr_ec_s { uint8_t data[32U]; } Eurydice_arr_ec;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $64size_t
*/
typedef struct Eurydice_arr_c7_s { uint8_t data[64U]; } Eurydice_arr_c7;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1088size_t
*/
typedef struct Eurydice_arr_2b_s { uint8_t data[1088U]; } Eurydice_arr_2b;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $168size_t
*/
typedef struct Eurydice_arr_c5_s { uint8_t data[168U]; } Eurydice_arr_c5;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $34size_t
*/
typedef struct Eurydice_arr_31_s { uint8_t data[34U]; } Eurydice_arr_31;

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
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_b2
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_a30_s { Eurydice_arr_b2 data[16U]; } Eurydice_arr_a30;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $4627size_t
*/
typedef struct Eurydice_arr_93_s { uint8_t data[4627U]; } Eurydice_arr_93;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2592size_t
*/
typedef struct Eurydice_arr_43_s { uint8_t data[2592U]; } Eurydice_arr_43;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $4896size_t
*/
typedef struct Eurydice_arr_e2_s { uint8_t data[4896U]; } Eurydice_arr_e2;

/**
A monomorphic instance of Eurydice.arr
with types int32_t
with const generics
- $256size_t
*/
typedef struct Eurydice_arr_6c_s { int32_t data[256U]; } Eurydice_arr_6c;

#define core_option_None 0
#define core_option_Some 1

typedef uint8_t core_option_Option_45_tags;

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
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $3309size_t
*/
typedef struct Eurydice_arr_0c_s { uint8_t data[3309U]; } Eurydice_arr_0c;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1952size_t
*/
typedef struct Eurydice_arr_29_s { uint8_t data[1952U]; } Eurydice_arr_29;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $4032size_t
*/
typedef struct Eurydice_arr_24_s { uint8_t data[4032U]; } Eurydice_arr_24;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $48size_t
*/
typedef struct Eurydice_arr_65_s { uint8_t data[48U]; } Eurydice_arr_65;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2420size_t
*/
typedef struct Eurydice_arr_85_s { uint8_t data[2420U]; } Eurydice_arr_85;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1312size_t
*/
typedef struct Eurydice_arr_02_s { uint8_t data[1312U]; } Eurydice_arr_02;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2560size_t
*/
typedef struct Eurydice_arr_10_s { uint8_t data[2560U]; } Eurydice_arr_10;

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
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $136size_t
*/
typedef struct Eurydice_arr_ff_s { uint8_t data[136U]; } Eurydice_arr_ff;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $640size_t
*/
typedef struct Eurydice_arr_20_s { uint8_t data[640U]; } Eurydice_arr_20;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $576size_t
*/
typedef struct Eurydice_arr_220_s { uint8_t data[576U]; } Eurydice_arr_220;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $11size_t
*/
typedef struct Eurydice_arr_c9_s { uint8_t data[11U]; } Eurydice_arr_c9;

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
with types uint8_t
with const generics
- $840size_t
*/
typedef struct Eurydice_arr_d10_s { uint8_t data[840U]; } Eurydice_arr_d10;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $66size_t
*/
typedef struct Eurydice_arr_91_s { uint8_t data[66U]; } Eurydice_arr_91;

typedef struct Eurydice_arr_c5_x4_s
{
  Eurydice_arr_c5 fst;
  Eurydice_arr_c5 snd;
  Eurydice_arr_c5 thd;
  Eurydice_arr_c5 f3;
}
Eurydice_arr_c5_x4;

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
with types Eurydice_arr_c5
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_88_s { Eurydice_arr_c5 data[1U]; } Eurydice_arr_88;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $28size_t
*/
typedef struct Eurydice_arr_a2_s { uint8_t data[28U]; } Eurydice_arr_a2;

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

/**
A monomorphic instance of Eurydice.arr
with types int32_t
with const generics
- $8size_t
*/
typedef struct Eurydice_arr_4d_s { int32_t data[8U]; } Eurydice_arr_4d;

typedef struct int32_t_x2_s
{
  int32_t fst;
  int32_t snd;
}
int32_t_x2;

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

typedef struct uint8_t_x2_s
{
  uint8_t fst;
  uint8_t snd;
}
uint8_t_x2;

#if defined(__cplusplus)
}
#endif

#define combined_core_H_DEFINED
#endif /* combined_core_H */
