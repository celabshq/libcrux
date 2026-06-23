/*
 * SPDX-FileCopyrightText: 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 6f058254eb741c12e9b388df07adaf7cc8aac8ed
 * Eurydice: fca2e9fbd728e49d677f3fc0da0054b55f3b9973
 * Karamel: 8c19d41458ce5cbfea029ebc03334ba96d149039
 * F*: 70671ffb81fa30aba09b9d6e2af275dfbccaa8f8
 * Libcrux: bdbc514c92784f52ed92097e2dfe82c2533df5d0
 */


#ifndef libcrux_sha3_portable_H
#define libcrux_sha3_portable_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "combined_core.h"

/**
A monomorphic instance of libcrux_sha3.generic_keccak.KeccakState
with types uint64_t
with const generics
- $1size_t
*/
typedef Eurydice_arr_7c libcrux_sha3_generic_keccak_KeccakState_f3;

typedef libcrux_sha3_generic_keccak_KeccakState_f3 libcrux_sha3_portable_KeccakState;

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.KeccakXofState
with types uint64_t
with const generics
- $1size_t
- $136size_t
*/
typedef struct libcrux_sha3_generic_keccak_xof_KeccakXofState_8d_s
{
  Eurydice_arr_7c inner;
  Eurydice_arr_0b buf;
  size_t buf_len;
  bool sponge;
}
libcrux_sha3_generic_keccak_xof_KeccakXofState_8d;

typedef libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
libcrux_sha3_portable_incremental_Shake256Xof;

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
uint64_t libcrux_sha3_simd_portable_zero_d1(void);

uint64_t
libcrux_sha3_simd_portable__veor5q_u64(
  uint64_t a,
  uint64_t b,
  uint64_t c,
  uint64_t d,
  uint64_t e
);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
uint64_t
libcrux_sha3_simd_portable_xor5_d1(uint64_t a, uint64_t b, uint64_t c, uint64_t d, uint64_t e);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 1
- RIGHT= 63
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_76(uint64_t x);

uint64_t libcrux_sha3_simd_portable__vrax1q_u64(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
uint64_t libcrux_sha3_simd_portable_rotate_left1_and_xor_d1(uint64_t a, uint64_t b);

uint64_t libcrux_sha3_simd_portable__vbcaxq_u64(uint64_t a, uint64_t b, uint64_t c);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
uint64_t libcrux_sha3_simd_portable_and_not_xor_d1(uint64_t a, uint64_t b, uint64_t c);

uint64_t libcrux_sha3_simd_portable__veorq_n_u64(uint64_t a, uint64_t c);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
uint64_t libcrux_sha3_simd_portable_xor_constant_d1(uint64_t a, uint64_t c);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
uint64_t libcrux_sha3_simd_portable_xor_d1(uint64_t a, uint64_t b);

/**
 Create a new Shake128 x4 state.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.new_26
with types uint64_t
with const generics
- N= 1
*/
Eurydice_arr_7c libcrux_sha3_generic_keccak_new_26_71(void);

/**
 Create a new SHAKE-128 state object.
*/
Eurydice_arr_7c libcrux_sha3_portable_incremental_shake128_init(void);

#define LIBCRUX_SHA3_GENERIC_KECCAK_CONSTANTS_ROUNDCONSTANTS ((KRML_CLITERAL(Eurydice_arr_22){ .data = { 1ULL, 32898ULL, 9223372036854808714ULL, 9223372039002292224ULL, 32907ULL, 2147483649ULL, 9223372039002292353ULL, 9223372036854808585ULL, 138ULL, 136ULL, 2147516425ULL, 2147483658ULL, 2147516555ULL, 9223372036854775947ULL, 9223372036854808713ULL, 9223372036854808579ULL, 9223372036854808578ULL, 9223372036854775936ULL, 32778ULL, 9223372039002259466ULL, 9223372039002292353ULL, 9223372036854808704ULL, 2147483649ULL, 9223372039002292232ULL } }))

/**
A monomorphic instance of libcrux_sha3.traits.get_ij
with types uint64_t
with const generics
- N= 1
*/
const uint64_t *libcrux_sha3_traits_get_ij_71(const Eurydice_arr_7c *arr, size_t i, size_t j);

/**
A monomorphic instance of libcrux_sha3.traits.set_ij
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_traits_set_ij_71(Eurydice_arr_7c *arr, size_t i, size_t j, uint64_t value);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 168
*/
void
libcrux_sha3_simd_portable_load_block_60(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 168
- DELIMITER= 31
*/
void
libcrux_sha3_simd_portable_load_last_37(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_0f
with const generics
- RATE= 168
- DELIMITER= 31
*/
void
libcrux_sha3_simd_portable_load_last_0f_37(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
 Get element `[i, j]`.
*/
/**
This function found in impl {impl core::ops::index::Index<(usize, usize), T> for libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.index_6a
with types uint64_t
with const generics
- N= 1
*/
const
uint64_t
*libcrux_sha3_generic_keccak_index_6a_71(const Eurydice_arr_7c *self, size_t_x2 index);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.theta_26
with types uint64_t
with const generics
- N= 1
*/
Eurydice_arr_84 libcrux_sha3_generic_keccak_theta_26_71(Eurydice_arr_7c *self);

/**
 Set element `[i, j] = v`.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.set_26
with types uint64_t
with const generics
- N= 1
*/
void
libcrux_sha3_generic_keccak_set_26_71(Eurydice_arr_7c *self, size_t i, size_t j, uint64_t v);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 36
- RIGHT= 28
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_02(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 36
- RIGHT= 28
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_02(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 36
- RIGHT= 28
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_02(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 3
- RIGHT= 61
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_ac(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 3
- RIGHT= 61
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_ac(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 3
- RIGHT= 61
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_ac(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 41
- RIGHT= 23
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_020(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 41
- RIGHT= 23
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_020(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 41
- RIGHT= 23
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_020(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 18
- RIGHT= 46
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_a9(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 18
- RIGHT= 46
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_a9(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 18
- RIGHT= 46
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_a9(uint64_t a, uint64_t b);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_0_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_rho_0_26_71(Eurydice_arr_7c *self, Eurydice_arr_84 t);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 1
- RIGHT= 63
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_76(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 1
- RIGHT= 63
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_76(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 44
- RIGHT= 20
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_58(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 44
- RIGHT= 20
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_58(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 44
- RIGHT= 20
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_58(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 10
- RIGHT= 54
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_e0(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 10
- RIGHT= 54
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_e0(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 10
- RIGHT= 54
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_e0(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 45
- RIGHT= 19
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_63(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 45
- RIGHT= 19
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_63(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 45
- RIGHT= 19
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_63(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 2
- RIGHT= 62
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_6a(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 2
- RIGHT= 62
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_6a(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 2
- RIGHT= 62
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_6a(uint64_t a, uint64_t b);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_1_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_rho_1_26_71(Eurydice_arr_7c *self, Eurydice_arr_84 t);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 62
- RIGHT= 2
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_ab(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 62
- RIGHT= 2
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_ab(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 62
- RIGHT= 2
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_ab(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 6
- RIGHT= 58
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_5b(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 6
- RIGHT= 58
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_5b(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 6
- RIGHT= 58
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_5b(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 43
- RIGHT= 21
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_6f(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 43
- RIGHT= 21
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_6f(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 43
- RIGHT= 21
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_6f(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 15
- RIGHT= 49
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_62(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 15
- RIGHT= 49
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_62(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 15
- RIGHT= 49
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_62(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 61
- RIGHT= 3
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_23(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 61
- RIGHT= 3
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_23(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 61
- RIGHT= 3
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_23(uint64_t a, uint64_t b);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_2_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_rho_2_26_71(Eurydice_arr_7c *self, Eurydice_arr_84 t);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 28
- RIGHT= 36
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_37(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 28
- RIGHT= 36
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_37(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 28
- RIGHT= 36
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_37(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 55
- RIGHT= 9
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_bb(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 55
- RIGHT= 9
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_bb(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 55
- RIGHT= 9
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_bb(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 25
- RIGHT= 39
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_b9(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 25
- RIGHT= 39
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_b9(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 25
- RIGHT= 39
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_b9(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 21
- RIGHT= 43
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_54(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 21
- RIGHT= 43
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_54(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 21
- RIGHT= 43
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_54(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 56
- RIGHT= 8
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_4c(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 56
- RIGHT= 8
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_4c(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 56
- RIGHT= 8
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_4c(uint64_t a, uint64_t b);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_3_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_rho_3_26_71(Eurydice_arr_7c *self, Eurydice_arr_84 t);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 27
- RIGHT= 37
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_ce(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 27
- RIGHT= 37
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_ce(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 27
- RIGHT= 37
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_ce(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 20
- RIGHT= 44
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_77(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 20
- RIGHT= 44
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_77(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 20
- RIGHT= 44
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_77(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 39
- RIGHT= 25
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_25(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 39
- RIGHT= 25
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_25(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 39
- RIGHT= 25
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_25(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 8
- RIGHT= 56
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_af(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 8
- RIGHT= 56
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_af(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 8
- RIGHT= 56
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_af(uint64_t a, uint64_t b);

/**
A monomorphic instance of libcrux_sha3.simd.portable.rotate_left
with const generics
- LEFT= 14
- RIGHT= 50
*/
uint64_t libcrux_sha3_simd_portable_rotate_left_fd(uint64_t x);

/**
A monomorphic instance of libcrux_sha3.simd.portable._vxarq_u64
with const generics
- LEFT= 14
- RIGHT= 50
*/
uint64_t libcrux_sha3_simd_portable__vxarq_u64_fd(uint64_t a, uint64_t b);

/**
This function found in impl {impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.xor_and_rotate_d1
with const generics
- LEFT= 14
- RIGHT= 50
*/
uint64_t libcrux_sha3_simd_portable_xor_and_rotate_d1_fd(uint64_t a, uint64_t b);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_4_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_rho_4_26_71(Eurydice_arr_7c *self, Eurydice_arr_84 t);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.rho_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_rho_26_71(Eurydice_arr_7c *self, Eurydice_arr_84 t);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_0_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_pi_0_26_71(Eurydice_arr_7c *self, Eurydice_arr_7c old);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_1_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_pi_1_26_71(Eurydice_arr_7c *self, Eurydice_arr_7c old);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_2_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_pi_2_26_71(Eurydice_arr_7c *self, Eurydice_arr_7c old);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_3_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_pi_3_26_71(Eurydice_arr_7c *self, Eurydice_arr_7c old);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_4_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_pi_4_26_71(Eurydice_arr_7c *self, Eurydice_arr_7c old);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.pi_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_pi_26_71(Eurydice_arr_7c *self);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.chi_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_chi_26_71(Eurydice_arr_7c *self);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.iota_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_iota_26_71(Eurydice_arr_7c *self, size_t i);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.keccakf1600_26
with types uint64_t
with const generics
- N= 1
*/
void libcrux_sha3_generic_keccak_keccakf1600_26_71(Eurydice_arr_7c *self);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_26
with types uint64_t
with const generics
- N= 1
- RATE= 168
- DELIM= 31
*/
void
libcrux_sha3_generic_keccak_absorb_final_26_bd(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
 Absorb
*/
void
libcrux_sha3_portable_incremental_shake128_absorb_final(
  Eurydice_arr_7c *s,
  Eurydice_borrow_slice_u8 data0
);

/**
 Create a new SHAKE-256 state object.
*/
Eurydice_arr_7c libcrux_sha3_portable_incremental_shake256_init(void);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 136
*/
void
libcrux_sha3_simd_portable_load_block_b2(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 136
- DELIMITER= 31
*/
void
libcrux_sha3_simd_portable_load_last_22(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_0f
with const generics
- RATE= 136
- DELIMITER= 31
*/
void
libcrux_sha3_simd_portable_load_last_0f_22(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_26
with types uint64_t
with const generics
- N= 1
- RATE= 136
- DELIM= 31
*/
void
libcrux_sha3_generic_keccak_absorb_final_26_bd0(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
 Absorb some data for SHAKE-256 for the last time
*/
void
libcrux_sha3_portable_incremental_shake256_absorb_final(
  Eurydice_arr_7c *s,
  Eurydice_borrow_slice_u8 data
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_0f
with const generics
- RATE= 168
*/
void
libcrux_sha3_simd_portable_load_block_0f_60(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_26
with types uint64_t
with const generics
- N= 1
- RATE= 168
*/
void
libcrux_sha3_generic_keccak_absorb_block_26_e9(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 168
*/
void
libcrux_sha3_simd_portable_store_block_60(
  const Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Squeeze<u64> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_84
with const generics
- RATE= 168
*/
void
libcrux_sha3_simd_portable_squeeze_84_60(
  const Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 168
- DELIM= 31
*/
void
libcrux_sha3_generic_keccak_portable_keccak1_37(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 output
);

/**
 A portable SHAKE128 implementation.
*/
void
libcrux_sha3_portable_shake128(
  Eurydice_mut_borrow_slice_u8 digest,
  Eurydice_borrow_slice_u8 data
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_0f
with const generics
- RATE= 136
*/
void
libcrux_sha3_simd_portable_load_block_0f_b2(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_26
with types uint64_t
with const generics
- N= 1
- RATE= 136
*/
void
libcrux_sha3_generic_keccak_absorb_block_26_e90(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 136
*/
void
libcrux_sha3_simd_portable_store_block_b2(
  const Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Squeeze<u64> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_84
with const generics
- RATE= 136
*/
void
libcrux_sha3_simd_portable_squeeze_84_b2(
  const Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 136
- DELIM= 31
*/
void
libcrux_sha3_generic_keccak_portable_keccak1_22(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 output
);

/**
 A portable SHAKE256 implementation.
*/
void
libcrux_sha3_portable_shake256(
  Eurydice_mut_borrow_slice_u8 digest,
  Eurydice_borrow_slice_u8 data
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.squeeze_first_block_fd
with const generics
- RATE= 136
*/
void
libcrux_sha3_generic_keccak_portable_squeeze_first_block_fd_b2(
  const Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
 Squeeze the first SHAKE-256 block
*/
void
libcrux_sha3_portable_incremental_shake256_squeeze_first_block(
  Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.squeeze_first_five_blocks_fd
with const generics
- RATE= 168
*/
void
libcrux_sha3_generic_keccak_portable_squeeze_first_five_blocks_fd_60(
  Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
 Squeeze five blocks
*/
void
libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(
  Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out0
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.squeeze_next_block_fd
with const generics
- RATE= 168
*/
void
libcrux_sha3_generic_keccak_portable_squeeze_next_block_fd_60(
  Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start
);

/**
 Squeeze another block
*/
void
libcrux_sha3_portable_incremental_shake128_squeeze_next_block(
  Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out0
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.squeeze_next_block_fd
with const generics
- RATE= 136
*/
void
libcrux_sha3_generic_keccak_portable_squeeze_next_block_fd_b2(
  Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start
);

/**
 Squeeze the next SHAKE-256 block
*/
void
libcrux_sha3_portable_incremental_shake256_squeeze_next_block(
  Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out
);

/**
 Try to complete the internal partial buffer by consuming the minimum required
 number of bytes from the provided `inputs` so that `self.buf` becomes exactly
 one full block of size `RATE`.

 Behaviour:
 - If `self.buf_len` is 0 (no buffered bytes) or already equal to `RATE`
   (already a full block), or if the combined available bytes in `inputs` are
   not enough to reach `RATE`, the function does nothing and returns 0.
 - If `0 < self.buf_len < RATE` and `inputs[..]` contain at least
   `RATE - self.buf_len` bytes, the function copies exactly
   `consumed = RATE - self.buf_len` bytes from each lane `inputs[i]` into
   `self.buf[i]` starting at the current `self.buf_len` offset, sets
   `self.buf_len = RATE`, and returns `consumed`.

 Returns the `consumed` bytes from `inputs` if there's enough buffered
 content to consume, and `0` otherwise.
 If `consumed > 0` is returned, `self.buf` contains a full block to be
 loaded.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.fill_buffer_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
size_t
libcrux_sha3_generic_keccak_xof_fill_buffer_da_e9(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  const Eurydice_arr_dc *inputs
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.closure
with const generics
- $1size_t
- $136size_t
*/
typedef const Eurydice_arr_0b *libcrux_sha3_generic_keccak_xof_buf_to_slices_closure_94;

/**
This function found in impl {impl core::ops::function::FnMut<(usize,), &'_ [u8]> for libcrux_sha3::generic_keccak::xof::buf_to_slices::closure<'_0, PARALLEL_LANES, RATE>}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.call_mut_89
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
Eurydice_borrow_slice_u8
libcrux_sha3_generic_keccak_xof_buf_to_slices_call_mut_89_81(
  const Eurydice_arr_0b **_,
  size_t tupled_args
);

/**
This function found in impl {impl core::ops::function::FnOnce<(usize,), &'_ [u8]> for libcrux_sha3::generic_keccak::xof::buf_to_slices::closure<'_0, PARALLEL_LANES, RATE>}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.call_once_9c
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
Eurydice_borrow_slice_u8
libcrux_sha3_generic_keccak_xof_buf_to_slices_call_once_9c_81(
  const Eurydice_arr_0b *_,
  size_t _0
);

/**
 Note: This function exists to work around a hax bug where `core::array::from_fn`
 is extracted with an incorrect explicit type parameter `#(usize -> t_Slice u8)`
 instead of using the typeclass-based implicit parameter `#v_F` from
 `Core_models.Array.from_fn`.
 See: https://github.com/cryspen/hax/issues/1920
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
Eurydice_arr_dc libcrux_sha3_generic_keccak_xof_buf_to_slices_81(const Eurydice_arr_0b *buf);

/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.absorb_full_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
size_t
libcrux_sha3_generic_keccak_xof_absorb_full_da_e9(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  const Eurydice_arr_dc *inputs
);

/**
 Absorb

 This function takes any number of bytes to absorb and buffers if it's not enough.
 The function assumes that all input slices in `inputs` have the same length.

 Only a multiple of `RATE` blocks are absorbed.
 For the remaining bytes [`absorb_final`] needs to be called.

 This works best with relatively small `inputs`.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.absorb_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
void
libcrux_sha3_generic_keccak_xof_absorb_da_e9(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  const Eurydice_arr_dc *inputs
);

/**
 Shake256 absorb
*/
/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<136 : usize> for libcrux_sha3::portable::incremental::Shake256Xof}
*/
void
libcrux_sha3_portable_incremental_absorb_6d(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_borrow_slice_u8 input
);

/**
 Absorb a final block.

 The `inputs` block may be empty. Everything in the `inputs` block beyond
 `RATE` bytes is ignored.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.absorb_final_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 136
- DELIMITER= 31
*/
void
libcrux_sha3_generic_keccak_xof_absorb_final_da_bd(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  const Eurydice_arr_dc *inputs
);

/**
 Shake256 absorb final
*/
/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<136 : usize> for libcrux_sha3::portable::incremental::Shake256Xof}
*/
void
libcrux_sha3_portable_incremental_absorb_final_6d(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_borrow_slice_u8 input
);

/**
 An all zero block
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.zero_block_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
Eurydice_arr_ff libcrux_sha3_generic_keccak_xof_zero_block_da_e9(void);

/**
 Generate a new keccak xof state.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.new_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 136
*/
libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
libcrux_sha3_generic_keccak_xof_new_da_e9(void);

/**
 Shake256 new state
*/
/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<136 : usize> for libcrux_sha3::portable::incremental::Shake256Xof}
*/
libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
libcrux_sha3_portable_incremental_new_6d(void);

/**
 Squeeze `N` x `LEN` bytes. Only `N = 1` for now.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, 1 : usize, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.squeeze_27
with types uint64_t
with const generics
- RATE= 136
*/
void
libcrux_sha3_generic_keccak_xof_squeeze_27_76(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
 Shake256 squeeze
*/
/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<136 : usize> for libcrux_sha3::portable::incremental::Shake256Xof}
*/
void
libcrux_sha3_portable_incremental_squeeze_6d(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 72
*/
void
libcrux_sha3_simd_portable_load_block_c6(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_0f
with const generics
- RATE= 72
*/
void
libcrux_sha3_simd_portable_load_block_0f_c6(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_26
with types uint64_t
with const generics
- N= 1
- RATE= 72
*/
void
libcrux_sha3_generic_keccak_absorb_block_26_e91(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 72
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_dc(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_0f
with const generics
- RATE= 72
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_0f_dc(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_26
with types uint64_t
with const generics
- N= 1
- RATE= 72
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_absorb_final_26_bd1(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 72
*/
void
libcrux_sha3_simd_portable_store_block_c6(
  const Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Squeeze<u64> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_84
with const generics
- RATE= 72
*/
void
libcrux_sha3_simd_portable_squeeze_84_c6(
  const Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 72
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_portable_keccak1_dc(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 output
);

/**
 A portable SHA3 512 implementation.
*/
void
libcrux_sha3_portable_sha512(
  Eurydice_mut_borrow_slice_u8 digest,
  Eurydice_borrow_slice_u8 data
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 136
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_220(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_0f
with const generics
- RATE= 136
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_0f_220(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_26
with types uint64_t
with const generics
- N= 1
- RATE= 136
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_absorb_final_26_bd2(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 136
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_portable_keccak1_220(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 output
);

/**
 A portable SHA3 256 implementation.
*/
void
libcrux_sha3_portable_sha256(
  Eurydice_mut_borrow_slice_u8 digest,
  Eurydice_borrow_slice_u8 data
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.squeeze_first_three_blocks_fd
with const generics
- RATE= 168
*/
void
libcrux_sha3_generic_keccak_portable_squeeze_first_three_blocks_fd_60(
  Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
 Squeeze three blocks
*/
void
libcrux_sha3_portable_incremental_shake128_squeeze_first_three_blocks(
  Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out0
);

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
size_t libcrux_sha3_digest_size(libcrux_sha3_Algorithm mode);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 144
*/
void
libcrux_sha3_simd_portable_load_block_9e(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_0f
with const generics
- RATE= 144
*/
void
libcrux_sha3_simd_portable_load_block_0f_9e(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_26
with types uint64_t
with const generics
- N= 1
- RATE= 144
*/
void
libcrux_sha3_generic_keccak_absorb_block_26_e92(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 144
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_3a(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_0f
with const generics
- RATE= 144
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_0f_3a(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_26
with types uint64_t
with const generics
- N= 1
- RATE= 144
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_absorb_final_26_bd3(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 144
*/
void
libcrux_sha3_simd_portable_store_block_9e(
  const Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Squeeze<u64> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_84
with const generics
- RATE= 144
*/
void
libcrux_sha3_simd_portable_squeeze_84_9e(
  const Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 144
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_portable_keccak1_3a(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 output
);

/**
 A portable SHA3 224 implementation.
*/
void
libcrux_sha3_portable_sha224(
  Eurydice_mut_borrow_slice_u8 digest,
  Eurydice_borrow_slice_u8 data
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block
with const generics
- RATE= 104
*/
void
libcrux_sha3_simd_portable_load_block_53(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_block_0f
with const generics
- RATE= 104
*/
void
libcrux_sha3_simd_portable_load_block_0f_53(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_block_26
with types uint64_t
with const generics
- N= 1
- RATE= 104
*/
void
libcrux_sha3_generic_keccak_absorb_block_26_e93(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last
with const generics
- RATE= 104
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_dc0(
  Eurydice_arr_7c *state,
  Eurydice_borrow_slice_u8 blocks,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Absorb<1 : usize> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.load_last_0f
with const generics
- RATE= 104
- DELIMITER= 6
*/
void
libcrux_sha3_simd_portable_load_last_0f_dc0(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
This function found in impl {libcrux_sha3::generic_keccak::KeccakState<T, N>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.absorb_final_26
with types uint64_t
with const generics
- N= 1
- RATE= 104
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_absorb_final_26_bd4(
  Eurydice_arr_7c *self,
  const Eurydice_arr_dc *input,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.simd.portable.store_block
with const generics
- RATE= 104
*/
void
libcrux_sha3_simd_portable_store_block_53(
  const Eurydice_arr_7c *s,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
This function found in impl {impl libcrux_sha3::traits::Squeeze<u64> for libcrux_sha3::generic_keccak::KeccakState<u64, 1 : usize>[{built_in impl core::marker::Sized for u64}, libcrux_sha3::simd::portable::{impl libcrux_sha3::traits::KeccakItem<1 : usize> for u64}]}
*/
/**
A monomorphic instance of libcrux_sha3.simd.portable.squeeze_84
with const generics
- RATE= 104
*/
void
libcrux_sha3_simd_portable_squeeze_84_53(
  const Eurydice_arr_7c *self,
  Eurydice_mut_borrow_slice_u8 out,
  size_t start,
  size_t len
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.portable.keccak1
with const generics
- RATE= 104
- DELIM= 6
*/
void
libcrux_sha3_generic_keccak_portable_keccak1_dc0(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 output
);

/**
 A portable SHA3 384 implementation.
*/
void
libcrux_sha3_portable_sha384(
  Eurydice_mut_borrow_slice_u8 digest,
  Eurydice_borrow_slice_u8 data
);

/**
 SHA3 224

 Preconditions:
 - `digest.len() == 28`
*/
void
libcrux_sha3_sha224_ema(Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 payload);

/**
 SHA3 224
*/
Eurydice_arr_a2 libcrux_sha3_sha224(Eurydice_borrow_slice_u8 data);

/**
 SHA3 256
*/
void
libcrux_sha3_sha256_ema(Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 payload);

/**
 SHA3 256
*/
Eurydice_arr_ec libcrux_sha3_sha256(Eurydice_borrow_slice_u8 data);

/**
 SHA3 384
*/
void
libcrux_sha3_sha384_ema(Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 payload);

/**
 SHA3 384
*/
Eurydice_arr_65 libcrux_sha3_sha384(Eurydice_borrow_slice_u8 data);

/**
 SHA3 512
*/
void
libcrux_sha3_sha512_ema(Eurydice_mut_borrow_slice_u8 digest, Eurydice_borrow_slice_u8 payload);

/**
 SHA3 512
*/
Eurydice_arr_c7 libcrux_sha3_sha512(Eurydice_borrow_slice_u8 data);

/**
 SHAKE 128

 Writes `out.len()` bytes.
*/
void
libcrux_sha3_shake128_ema(Eurydice_mut_borrow_slice_u8 out, Eurydice_borrow_slice_u8 data);

/**
 SHAKE 256

 Writes `out.len()` bytes.
*/
void
libcrux_sha3_shake256_ema(Eurydice_mut_borrow_slice_u8 out, Eurydice_borrow_slice_u8 data);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.KeccakXofState
with types uint64_t
with const generics
- $1size_t
- $168size_t
*/
typedef struct libcrux_sha3_generic_keccak_xof_KeccakXofState_55_s
{
  Eurydice_arr_7c inner;
  Eurydice_arr_88 buf;
  size_t buf_len;
  bool sponge;
}
libcrux_sha3_generic_keccak_xof_KeccakXofState_55;

typedef libcrux_sha3_generic_keccak_xof_KeccakXofState_55
libcrux_sha3_portable_incremental_Shake128Xof;

/**
 Try to complete the internal partial buffer by consuming the minimum required
 number of bytes from the provided `inputs` so that `self.buf` becomes exactly
 one full block of size `RATE`.

 Behaviour:
 - If `self.buf_len` is 0 (no buffered bytes) or already equal to `RATE`
   (already a full block), or if the combined available bytes in `inputs` are
   not enough to reach `RATE`, the function does nothing and returns 0.
 - If `0 < self.buf_len < RATE` and `inputs[..]` contain at least
   `RATE - self.buf_len` bytes, the function copies exactly
   `consumed = RATE - self.buf_len` bytes from each lane `inputs[i]` into
   `self.buf[i]` starting at the current `self.buf_len` offset, sets
   `self.buf_len = RATE`, and returns `consumed`.

 Returns the `consumed` bytes from `inputs` if there's enough buffered
 content to consume, and `0` otherwise.
 If `consumed > 0` is returned, `self.buf` contains a full block to be
 loaded.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.fill_buffer_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
size_t
libcrux_sha3_generic_keccak_xof_fill_buffer_da_e90(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  const Eurydice_arr_dc *inputs
);

/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.closure
with const generics
- $1size_t
- $168size_t
*/
typedef const Eurydice_arr_88 *libcrux_sha3_generic_keccak_xof_buf_to_slices_closure_48;

/**
This function found in impl {impl core::ops::function::FnMut<(usize,), &'_ [u8]> for libcrux_sha3::generic_keccak::xof::buf_to_slices::closure<'_0, PARALLEL_LANES, RATE>}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.call_mut_89
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
Eurydice_borrow_slice_u8
libcrux_sha3_generic_keccak_xof_buf_to_slices_call_mut_89_810(
  const Eurydice_arr_88 **_,
  size_t tupled_args
);

/**
This function found in impl {impl core::ops::function::FnOnce<(usize,), &'_ [u8]> for libcrux_sha3::generic_keccak::xof::buf_to_slices::closure<'_0, PARALLEL_LANES, RATE>}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices.call_once_9c
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
Eurydice_borrow_slice_u8
libcrux_sha3_generic_keccak_xof_buf_to_slices_call_once_9c_810(
  const Eurydice_arr_88 *_,
  size_t _0
);

/**
 Note: This function exists to work around a hax bug where `core::array::from_fn`
 is extracted with an incorrect explicit type parameter `#(usize -> t_Slice u8)`
 instead of using the typeclass-based implicit parameter `#v_F` from
 `Core_models.Array.from_fn`.
 See: https://github.com/cryspen/hax/issues/1920
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.buf_to_slices
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
Eurydice_arr_dc libcrux_sha3_generic_keccak_xof_buf_to_slices_810(const Eurydice_arr_88 *buf);

/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.absorb_full_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
size_t
libcrux_sha3_generic_keccak_xof_absorb_full_da_e90(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  const Eurydice_arr_dc *inputs
);

/**
 Absorb

 This function takes any number of bytes to absorb and buffers if it's not enough.
 The function assumes that all input slices in `inputs` have the same length.

 Only a multiple of `RATE` blocks are absorbed.
 For the remaining bytes [`absorb_final`] needs to be called.

 This works best with relatively small `inputs`.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.absorb_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
void
libcrux_sha3_generic_keccak_xof_absorb_da_e90(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  const Eurydice_arr_dc *inputs
);

/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<168 : usize> for libcrux_sha3::portable::incremental::Shake128Xof}
*/
void
libcrux_sha3_portable_incremental_absorb_5f(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  Eurydice_borrow_slice_u8 input
);

/**
 Absorb a final block.

 The `inputs` block may be empty. Everything in the `inputs` block beyond
 `RATE` bytes is ignored.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.absorb_final_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 168
- DELIMITER= 31
*/
void
libcrux_sha3_generic_keccak_xof_absorb_final_da_bd0(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  const Eurydice_arr_dc *inputs
);

/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<168 : usize> for libcrux_sha3::portable::incremental::Shake128Xof}
*/
void
libcrux_sha3_portable_incremental_absorb_final_5f(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  Eurydice_borrow_slice_u8 input
);

/**
 An all zero block
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.zero_block_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
Eurydice_arr_c5 libcrux_sha3_generic_keccak_xof_zero_block_da_e90(void);

/**
 Generate a new keccak xof state.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, PARALLEL_LANES, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.new_da
with types uint64_t
with const generics
- PARALLEL_LANES= 1
- RATE= 168
*/
libcrux_sha3_generic_keccak_xof_KeccakXofState_55
libcrux_sha3_generic_keccak_xof_new_da_e90(void);

/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<168 : usize> for libcrux_sha3::portable::incremental::Shake128Xof}
*/
libcrux_sha3_generic_keccak_xof_KeccakXofState_55
libcrux_sha3_portable_incremental_new_5f(void);

/**
 Squeeze `N` x `LEN` bytes. Only `N = 1` for now.
*/
/**
This function found in impl {libcrux_sha3::generic_keccak::xof::KeccakXofState<STATE, 1 : usize, RATE>[@TraitClause0, @TraitClause1]}
*/
/**
A monomorphic instance of libcrux_sha3.generic_keccak.xof.squeeze_27
with types uint64_t
with const generics
- RATE= 168
*/
void
libcrux_sha3_generic_keccak_xof_squeeze_27_2a(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
 Shake128 squeeze
*/
/**
This function found in impl {impl libcrux_sha3::portable::incremental::Xof<168 : usize> for libcrux_sha3::portable::incremental::Shake128Xof}
*/
void
libcrux_sha3_portable_incremental_squeeze_5f(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_55 *self,
  Eurydice_mut_borrow_slice_u8 out
);

/**
This function found in impl {impl core::clone::Clone for libcrux_sha3::portable::KeccakState}
*/
Eurydice_arr_7c libcrux_sha3_portable_clone_5a(const Eurydice_arr_7c *self);

/**
This function found in impl {impl core::clone::Clone for libcrux_sha3::Algorithm}
*/
libcrux_sha3_Algorithm libcrux_sha3_clone_8c(const libcrux_sha3_Algorithm *self);

/**
This function found in impl {impl core::convert::From<libcrux_sha3::Algorithm> for u32}
*/
uint32_t libcrux_sha3_from_83(libcrux_sha3_Algorithm v);

#if defined(__cplusplus)
}
#endif

#define libcrux_sha3_portable_H_DEFINED
#endif /* libcrux_sha3_portable_H */
