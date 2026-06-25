/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 377317d6b25702c46ffff072fa00a3e32095e46f
 * Eurydice: b227478b67c6a6e2ff611f978f10d6b7f26472ac
 * Karamel: 4e64d915da3c172d1dfad805b8e1a46beff938bc
 * F*: 89901492c020c74b82d811d27f3149c222d9b8b5
 * Libcrux: a53e03cfd7b424560bdfefc9d483f87faacd3122
 */

#ifndef libcrux_core_H
#define libcrux_core_H

#include "eurydice_glue.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_sha3_internal.h"

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $3168size_t
*/
typedef struct Eurydice_arr_17_s {
  uint8_t data[3168U];
} Eurydice_arr_17;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1568size_t
*/
typedef struct Eurydice_arr_00_s {
  uint8_t data[1568U];
} Eurydice_arr_00;

typedef struct libcrux_ml_kem_mlkem1024_MlKem1024KeyPair_s {
  Eurydice_arr_17 sk;
  Eurydice_arr_00 pk;
} libcrux_ml_kem_mlkem1024_MlKem1024KeyPair;

/**
A monomorphic instance of K.
with types libcrux_ml_kem_mlkem1024_MlKem1024Ciphertext, Eurydice_arr_60

*/
typedef struct tuple_4d_s {
  Eurydice_arr_00 fst;
  Eurydice_arr_60 snd;
} tuple_4d;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $168size_t
*/
typedef struct Eurydice_arr_27_s {
  uint8_t data[168U];
} Eurydice_arr_27;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $2400size_t
*/
typedef struct Eurydice_arr_ea_s {
  uint8_t data[2400U];
} Eurydice_arr_ea;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1184size_t
*/
typedef struct Eurydice_arr_74_s {
  uint8_t data[1184U];
} Eurydice_arr_74;

typedef struct libcrux_ml_kem_mlkem768_MlKem768KeyPair_s {
  Eurydice_arr_ea sk;
  Eurydice_arr_74 pk;
} libcrux_ml_kem_mlkem768_MlKem768KeyPair;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_6d_s {
  uint8_t data[24U];
} Eurydice_arr_6d;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $64size_t
*/
typedef struct Eurydice_arr_060_s {
  uint8_t data[64U];
} Eurydice_arr_060;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $1088size_t
*/
typedef struct Eurydice_arr_2c_s {
  uint8_t data[1088U];
} Eurydice_arr_2c;

/**
A monomorphic instance of K.
with types libcrux_ml_kem_mlkem768_MlKem768Ciphertext, Eurydice_arr_60

*/
typedef struct tuple_7f_s {
  Eurydice_arr_2c fst;
  Eurydice_arr_60 snd;
} tuple_7f;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $10size_t
*/
typedef struct Eurydice_arr_77_s {
  uint8_t data[10U];
} Eurydice_arr_77;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $22size_t
*/
typedef struct Eurydice_arr_f3_s {
  uint8_t data[22U];
} Eurydice_arr_f3;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $20size_t
*/
typedef struct Eurydice_arr_dc_s {
  uint8_t data[20U];
} Eurydice_arr_dc;

/**
A monomorphic instance of Eurydice.arr
with types int16_t
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_e2_s {
  int16_t data[16U];
} Eurydice_arr_e2;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $136size_t
*/
typedef struct Eurydice_arr_3d_s {
  uint8_t data[136U];
} Eurydice_arr_3d;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_3d
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_3e_s {
  Eurydice_arr_3d data[1U];
} Eurydice_arr_3e;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_arr_27
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_3a_s {
  Eurydice_arr_27 data[1U];
} Eurydice_arr_3a;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $48size_t
*/
typedef struct Eurydice_arr_5f_s {
  uint8_t data[48U];
} Eurydice_arr_5f;

/**
A monomorphic instance of Eurydice.arr
with types uint8_t
with const generics
- $28size_t
*/
typedef struct Eurydice_arr_f1_s {
  uint8_t data[28U];
} Eurydice_arr_f1;

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_a5_s {
  uint64_t data[5U];
} Eurydice_arr_a5;

/**
A monomorphic instance of Eurydice.arr
with types Eurydice_borrow_slice_u8
with const generics
- $1size_t
*/
typedef struct Eurydice_arr_06_s {
  Eurydice_borrow_slice_u8 data[1U];
} Eurydice_arr_06;

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $25size_t
*/
typedef struct Eurydice_arr_26_s {
  uint64_t data[25U];
} Eurydice_arr_26;

/**
A monomorphic instance of Eurydice.arr
with types uint64_t
with const generics
- $24size_t
*/
typedef struct Eurydice_arr_a7_s {
  uint64_t data[24U];
} Eurydice_arr_a7;

#if defined(__cplusplus)
}
#endif

#define libcrux_core_H_DEFINED
#endif /* libcrux_core_H */
