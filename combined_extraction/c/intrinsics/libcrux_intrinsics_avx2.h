/*
 * SPDX-FileCopyrightText: 2024-2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#ifndef LIBCRUX_HEADER_LIBCRUX_INTRINSICS_AVX2_H
#define LIBCRUX_HEADER_LIBCRUX_INTRINSICS_AVX2_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "../eurydice_glue.h"
#include "immintrin.h"

typedef __m128i core_core_arch_x86___m128i;
typedef __m256i core_core_arch_x86___m256i;

// Cast and Convert

#define libcrux_intrinsics_avx2_mm256_castsi256_si128(a) \
  (_mm256_castsi256_si128(a))

#define libcrux_intrinsics_avx2_mm256_castsi256_ps(a) (_mm256_castsi256_ps(a))

#define libcrux_intrinsics_avx2_mm256_cvtepi16_epi32(a) \
  (_mm256_cvtepi16_epi32(a))

#define libcrux_intrinsics_avx2_mm256_castsi128_si256(a) \
  (_mm256_castsi128_si256(a))

// Initialize, Load, Store

#define libcrux_intrinsics_avx2_mm256_setzero_si256() (_mm256_setzero_si256())

#define libcrux_intrinsics_avx2_mm256_set1_epi16(a) (_mm256_set1_epi16(a))

#define libcrux_intrinsics_avx2_mm256_set1_epi32(a) (_mm256_set1_epi32(a))

#define libcrux_intrinsics_avx2_mm256_set1_epi64x(a) (_mm256_set1_epi64x(a))

#define libcrux_intrinsics_avx2_mm_set1_epi16(a) (_mm_set1_epi16(a))

#define libcrux_intrinsics_avx2_mm256_set_epi16(                           \
    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15)  \
  (_mm256_set_epi16(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, \
                    x13, x14, x15))

#define libcrux_intrinsics_avx2_mm256_set_epi8(                                \
    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, \
    x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31) \
  (_mm256_set_epi8(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, \
                   x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, \
                   x26, x27, x28, x29, x30, x31))

#define libcrux_intrinsics_avx2_mm_set_epi8(                                \
    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15)   \
  (_mm_set_epi8(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, \
                x14, x15))

#define libcrux_intrinsics_avx2_mm256_set_epi32(x0, x1, x2, x3, x4, x5, x6, \
                                                x7)                         \
  (_mm256_set_epi32(x0, x1, x2, x3, x4, x5, x6, x7))

#define libcrux_intrinsics_avx2_mm256_set_epi64x(x0, x1, x2, x3) \
  (_mm256_set_epi64x(x0, x1, x2, x3))

#define libcrux_intrinsics_avx2_mm256_set_m128i(h, l) (_mm256_set_m128i(h, l))

#define libcrux_intrinsics_avx2_mm_set_epi32(x0, x1, x2, x3) \
  (_mm_set_epi32(x0, x1, x2, x3))

#define libcrux_intrinsics_avx2_mm256_loadu_si256_i16(a) \
  (_mm256_loadu_si256((const __m256i *)a.ptr))

#define libcrux_intrinsics_avx2_mm256_loadu_si256_u8(a) \
  (_mm256_loadu_si256((const __m256i *)a.ptr))

#define libcrux_intrinsics_avx2_mm256_loadu_si256_i32(a) \
  (_mm256_loadu_si256((const __m256i *)a.ptr))

#define libcrux_intrinsics_avx2_mm_loadu_si128(a) \
  (_mm_loadu_si128((const __m128i *)a.ptr))

#define libcrux_intrinsics_avx2_mm_storeu_bytes_si128(a, b) \
  (_mm_storeu_si128((__m128i *)a.ptr, b))

#define libcrux_intrinsics_avx2_mm256_storeu_si256_i16(a, b) \
  (_mm256_storeu_si256((__m256i *)a.ptr, b))

#define libcrux_intrinsics_avx2_mm256_storeu_si256_u8(a, b) \
  (_mm256_storeu_si256((__m256i *)a.ptr, b))

#define libcrux_intrinsics_avx2_mm_storeu_si128(a, b) \
  (_mm_storeu_si128((__m128i *)a.ptr, b))

#define libcrux_intrinsics_avx2_mm256_storeu_si256_i32(a, b) \
  (_mm256_storeu_si256((__m256i *)a.ptr, b))

#define libcrux_intrinsics_avx2_mm_storeu_si128_i32(a, b) \
  (_mm_storeu_si128((__m128i *)a.ptr, b))

// Arithmetic: Add, Sub

#define libcrux_intrinsics_avx2_mm256_add_epi16(a, b) (_mm256_add_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_add_epi32(a, b) (_mm256_add_epi32(a, b))

#define libcrux_intrinsics_avx2_mm256_add_epi64(a, b) (_mm256_add_epi64(a, b))

#define libcrux_intrinsics_avx2_mm_add_epi16(a, b) (_mm_add_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_sub_epi16(a, b) (_mm256_sub_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_sub_epi32(a, b) (_mm256_sub_epi32(a, b))

#define libcrux_intrinsics_avx2_mm_sub_epi16(a, b) (_mm_sub_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_abs_epi32(a) (_mm256_abs_epi32(a))

// Arithmetic: Mul low and high, Mul-Add combinations

#define libcrux_intrinsics_avx2_mm256_mullo_epi16(a, b) \
  (_mm256_mullo_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_mulhi_epi16(a, b) \
  (_mm256_mulhi_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_mul_epu32(a, b) (_mm256_mul_epu32(a, b))

#define libcrux_intrinsics_avx2_mm256_mul_epi32(a, b) (_mm256_mul_epi32(a, b))

#define libcrux_intrinsics_avx2_mm256_mullo_epi32(a, b) \
  (_mm256_mullo_epi32(a, b))

#define libcrux_intrinsics_avx2_mm_mullo_epi16(a, b) (_mm_mullo_epi16(a, b))

#define libcrux_intrinsics_avx2_mm_mulhi_epi16(a, b) (_mm_mulhi_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_madd_epi16(a, b) (_mm256_madd_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_sign_epi32(a, b) (_mm256_sign_epi32(a, b))

// Comparison

#define libcrux_intrinsics_avx2_mm256_cmpgt_epi16(a, b) \
  (_mm256_cmpgt_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_cmpgt_epi32(a, b) \
  (_mm256_cmpgt_epi32(a, b))

#define libcrux_intrinsics_avx2_mm256_testz_si256(a, b) \
  (_mm256_testz_si256(a, b))

#define libcrux_intrinsics_avx2_mm256_cmpeq_epi32(a, b) \
  (_mm256_cmpeq_epi32(a, b))

// Bitwise operations

#define libcrux_intrinsics_avx2_mm256_and_si256(a, b) (_mm256_and_si256(a, b))

#define libcrux_intrinsics_avx2_mm256_andnot_si256(a, b) \
  (_mm256_andnot_si256(a, b))

#define libcrux_intrinsics_avx2_mm256_xor_si256(a, b) (_mm256_xor_si256(a, b))

#define libcrux_intrinsics_avx2_mm_movemask_epi8(a) (_mm_movemask_epi8(a))

#define libcrux_intrinsics_avx2_mm256_movemask_ps(a) (_mm256_movemask_ps(a))

#define libcrux_intrinsics_avx2_mm256_or_si256(a, b) (_mm256_or_si256(a, b))

// Shift operations
#define libcrux_intrinsics_avx2_mm256_srai_epi16(a, b, _) \
  (_mm256_srai_epi16(b, a))

#define libcrux_intrinsics_avx2_mm256_srli_epi16(a, b, _) \
  (_mm256_srli_epi16(b, a))

#define libcrux_intrinsics_avx2_mm256_slli_epi16(a, b, _) \
  (_mm256_slli_epi16(b, a))

#define libcrux_intrinsics_avx2_mm256_slli_epi32(a, b, _) \
  (_mm256_slli_epi32(b, a))

#define libcrux_intrinsics_avx2_mm256_slli_epi64_(a, b) \
  (_mm256_slli_epi64(b, a))

#define libcrux_intrinsics_avx2_mm256_slli_epi64(a, b, c) \
  (libcrux_intrinsics_avx2_mm256_slli_epi64_(a, b))

#define libcrux_intrinsics_avx2_mm256_srai_epi32(a, b, _) \
  (_mm256_srai_epi32(b, a))

#define libcrux_intrinsics_avx2_mm256_srli_epi32(a, b, _) \
  (_mm256_srli_epi32(b, a))

#define libcrux_intrinsics_avx2_mm256_sllv_epi32(a, b) (_mm256_sllv_epi32(a, b))

#define libcrux_intrinsics_avx2_mm_sllv_epi32(a, b) (_mm_sllv_epi32(a, b))

#define libcrux_intrinsics_avx2_mm_srli_epi64(a, b, _t) (_mm_srli_epi64(b, a))

#define libcrux_intrinsics_avx2_mm256_srli_epi64_(a, b) \
  (_mm256_srli_epi64(b, a))

#define libcrux_intrinsics_avx2_mm256_srli_epi64(a, b, c) \
  (libcrux_intrinsics_avx2_mm256_srli_epi64_(a, b))

#define libcrux_intrinsics_avx2_mm256_srlv_epi32(a, b) (_mm256_srlv_epi32(a, b))

#define libcrux_intrinsics_avx2_mm256_bsrli_epi128(a, b, _t) \
  (_mm256_bsrli_epi128(b, a))

#define libcrux_intrinsics_avx2_mm256_srlv_epi64(a, b) (_mm256_srlv_epi64(a, b))

// Shuffle and Vector Interleaving

#define libcrux_intrinsics_avx2_mm256_unpacklo_epi32(a, b) \
  (_mm256_unpacklo_epi32(a, b))

#define libcrux_intrinsics_avx2_mm256_unpacklo_epi64(a, b) \
  (_mm256_unpacklo_epi64(a, b))

#define libcrux_intrinsics_avx2_mm256_unpackhi_epi32(a, b) \
  (_mm256_unpackhi_epi32(a, b))

#define libcrux_intrinsics_avx2_mm256_unpackhi_epi64(a, b) \
  (_mm256_unpackhi_epi64(a, b))

#define libcrux_intrinsics_avx2_mm256_packs_epi32(a, b) \
  (_mm256_packs_epi32(a, b))

#define libcrux_intrinsics_avx2_mm_packs_epi16(a, b) (_mm_packs_epi16(a, b))

#define libcrux_intrinsics_avx2_mm256_shuffle_epi32(a, b, _) \
  (_mm256_shuffle_epi32(b, a))

#define libcrux_intrinsics_avx2_mm256_extracti128_si256(a, b, _) \
  (_mm256_extracti128_si256(b, a))

#define libcrux_intrinsics_avx2_mm256_permute4x64_epi64(a, b, _) \
  (_mm256_permute4x64_epi64(b, a))

#define libcrux_intrinsics_avx2_mm256_permute2x128_si256(a, b, c, d) \
  (_mm256_permute2x128_si256(b, c, a))

#define libcrux_intrinsics_avx2_mm256_inserti128_si256(a, b, c, _) \
  (_mm256_inserti128_si256(b, c, a))

#define libcrux_intrinsics_avx2_mm256_blend_epi16(a, b, c, _) \
  (_mm256_blend_epi16(b, c, a))

#define libcrux_intrinsics_avx2_mm256_blend_epi32(a, b, c, _) \
  (_mm256_blend_epi32(b, c, a))

#define libcrux_intrinsics_avx2_vec256_blendv_epi32(a, b, mask) \
  (_mm256_castps_si256(_mm256_blendv_ps(_mm256_castsi256_ps(a), \
                                        _mm256_castsi256_ps(b), \
                                        _mm256_castsi256_ps(mask))))

#define libcrux_intrinsics_avx2_mm256_shuffle_epi8(a, b) \
  (_mm256_shuffle_epi8(a, b))

#define libcrux_intrinsics_avx2_mm256_permutevar8x32_epi32(a, b) \
  (_mm256_permutevar8x32_epi32(a, b))

#define libcrux_intrinsics_avx2_mm_shuffle_epi8(a, b) (_mm_shuffle_epi8(a, b))

#if defined(__cplusplus)
}
#endif

#define LIBCRUX_HEADER_LIBCRUX_INTRINSICS_AVX2_H_DEFINED
#endif /* LIBCRUX_HEADER_LIBCRUX_INTRINSICS_AVX2_H */
