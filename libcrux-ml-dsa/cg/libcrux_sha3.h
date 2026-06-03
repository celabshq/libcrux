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

#ifndef libcrux_sha3_H
#define libcrux_sha3_H

#include "eurydice_glue.h"
#include "libcrux_mldsa_core.h"
#include "libcrux_sha3_internal.h"

/**
 A portable SHAKE128 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_shake128(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_37(data, digest);
}

/**
 A portable SHAKE256 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_shake256(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_22(data, digest);
}

/**
 A portable SHA3 224 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_sha224(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_3a(data, digest);
}

/**
 A portable SHA3 256 implementation.
*/
static KRML_MUSTINLINE void libcrux_sha3_portable_sha256(
    Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 data) {
  libcrux_sha3_generic_keccak_portable_keccak1_220(data, digest);
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
  EURYDICE_ASSERT(payload.meta <= CORE_NUM__U32__MAX, "panic!");
  EURYDICE_ASSERT(digest.meta == (size_t)28U, "panic!");
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
  EURYDICE_ASSERT(payload.meta <= CORE_NUM__U32__MAX, "panic!");
  EURYDICE_ASSERT(digest.meta == (size_t)32U, "panic!");
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
  EURYDICE_ASSERT(payload.meta <= CORE_NUM__U32__MAX, "panic!");
  EURYDICE_ASSERT(digest.meta == (size_t)48U, "panic!");
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

#define libcrux_sha3_H_DEFINED
#endif /* libcrux_sha3_H */
