/*
 * SPDX-FileCopyrightText: 2024 Eurydice Contributors
 * SPDX-FileCopyrightText: 2024-2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 */

#pragma once

#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _MSC_VER
// For __popcnt
#include <intrin.h>
#endif

#include "karamel/endianness.h"
#include "karamel/target.h"

// C++ HELPERS

#if defined(__cplusplus)

#include <utility>

#ifndef __cpp_lib_type_identity
template <class T>
struct type_identity {
  using type = T;
};

template <class T>
using type_identity_t = typename type_identity<T>::type;
#else
using std::type_identity_t;
#endif

#define KRML_UNION_CONSTRUCTOR(T)                              \
  template <typename V>                                        \
  constexpr T(int t, V U::*m, type_identity_t<V> v) : tag(t) { \
    val.*m = std::move(v);                                     \
  }                                                            \
  T() = default;

#endif

// GENERAL-PURPOSE STUFF

#define LowStar_Ignore_ignore(e, t, _ret_t) ((void)e)

#define EURYDICE_ASSERT(test, msg)                                            \
  do {                                                                        \
    if (!(test)) {                                                            \
      fprintf(stderr, "assertion \"%s\" failed: file \"%s\", line %d\n", msg, \
              __FILE__, __LINE__);                                            \
    }                                                                         \
  } while (0)

// SLICES, ARRAYS, ETC.

// We represent a slice as a pair of an (untyped) pointer, along with the length
// of the slice, i.e. the number of elements in the slice (this is NOT the
// number of bytes). This design choice has two important consequences.
// - if you need to use `ptr`, you MUST cast it to a proper type *before*
//   performing pointer arithmetic on it (remember that C desugars pointer
//   arithmetic based on the type of the address)
// - if you need to use `len` for a C style function (e.g. memcpy, memcmp), you
//   need to multiply it by sizeof t, where t is the type of the elements.
//
// Empty slices have `len == 0` and `ptr` always needs to be a valid pointer
// that is not NULL (otherwise the construction in EURYDICE_SLICE computes `NULL
// + start`).
typedef struct {
  void *ptr;
  size_t len;
} Eurydice_slice;

typedef struct Eurydice_dst_ref_87_s {
  uint8_t *ptr;
  size_t meta;
} Eurydice_dst_ref_87;

typedef struct Eurydice_dst_ref_9a_s {
  int16_t *ptr;
  size_t meta;
} Eurydice_dst_ref_9a;

#if defined(__cplusplus)
#define KRML_CLITERAL(type) type
#else
#define KRML_CLITERAL(type) (type)
#endif

#if defined(__cplusplus) && defined(__cpp_designated_initializers) || \
    !(defined(__cplusplus))
#define EURYDICE_CFIELD(X) X
#else
#define EURYDICE_CFIELD(X)
#endif

// Helper macro to create a slice out of a pointer x, a start index in x
// (included), and an end index in x (excluded). The argument x must be suitably
// cast to something that can decay (see remark above about how pointer
// arithmetic works in C), meaning either pointer or array type.
#define EURYDICE_SLICE(x, start, end) \
  (KRML_CLITERAL(Eurydice_slice){(void *)(x + start), end - start})

// Slice length
#define EURYDICE_SLICE_LEN(s, _) (s).meta
#define Eurydice_slice_len(s, _) (s).meta

#define Eurydice_slice_index_mut(s, i, t) ((s).ptr[i])
#define Eurydice_slice_index_shared(s, i, t) ((s).ptr[i])

#define Eurydice_slice_index(s, i, t) ((s).ptr[i])

// The following functions get sub slices from a slice.

#define Eurydice_slice_subslice(s, r, t, _0, _1) \
  EURYDICE_SLICE((t *)s.ptr, r.start, r.end)

// Variant for when the start and end indices are statically known (i.e., the
// range argument `r` is a literal).
#define Eurydice_slice_subslice2(s, start, end, t) \
  EURYDICE_SLICE((t *)s.ptr, (start), (end))

// Previous version above does not work when t is an array type (as usual). Will
// be deprecated soon.
#define Eurydice_slice_subslice3(s, start, end, t_ptr) \
  EURYDICE_SLICE((t_ptr)s.ptr, (start), (end))

#define Eurydice_slice_subslice_to(s, subslice_end_pos, t, _0, _1) \
  EURYDICE_SLICE((t *)s.ptr, 0, subslice_end_pos)

#define Eurydice_slice_subslice_from(s, subslice_start_pos, t, _0, _1) \
  EURYDICE_SLICE((t *)s.ptr, subslice_start_pos, s.len)

#define Eurydice_array_to_slice(end, x, t) \
  EURYDICE_SLICE(x, 0,                     \
                 end) /* x is already at an array type, no need for cast */
#define Eurydice_array_to_subslice(_arraylen, x, r, t, _0, _1) \
  EURYDICE_SLICE((t *)x, r.start, r.end)

// Same as above, variant for when start and end are statically known
#define Eurydice_array_to_subslice2(x, start, end, t) \
  EURYDICE_SLICE((t *)x, (start), (end))

// Same as above, variant for when start and end are statically known
#define Eurydice_array_to_subslice3(x, start, end, t_ptr) \
  EURYDICE_SLICE((t_ptr)x, (start), (end))
// The following functions convert an array into a slice.

#define Eurydice_array_to_subslice_to(_size, x, r, t, _range_t, _0) \
  EURYDICE_SLICE((t *)x, 0, r)
#define Eurydice_array_to_subslice_from(size, x, r, t, _range_t, _0) \
  EURYDICE_SLICE((t *)x, r, size)

// Copy a slice with memcopy
#define Eurydice_slice_copy(dst, src, t) \
  memcpy(dst.ptr, src.ptr, dst.meta * sizeof(t))

// Distinguished support for some PartialEq trait implementations
//
#define Eurydice_slice_eq(src1, src2, t, _) \
  ((src1)->len == (src2)->len &&            \
   !memcmp((src1)->ptr, (src2)->ptr, (src1)->len * sizeof(t)))

#define core_array___Array_T__N___as_slice(len_, ptr_, t, ret_t) \
  (KRML_CLITERAL(ret_t){EURYDICE_CFIELD(.ptr =)(ptr_)->data,     \
                        EURYDICE_CFIELD(.meta =) len_})

#define core_array__core__clone__Clone_for__Array_T__N___clone( \
    len, src, elem_type, _ret_t)                                \
  (*src)
#define TryFromSliceError uint8_t

#define Eurydice_array_eq(sz, a1, a2, t) (memcmp(a1, a2, sz * sizeof(t)) == 0)

// core::cmp::PartialEq<&0 (@Slice<U>)> for @Array<T, N>
#define Eurydice_array_eq_slice(sz, a1, s2, t, _) \
  (memcmp(a1, (s2)->ptr, sz * sizeof(t)) == 0)

#define core_array_equality___core__cmp__PartialEq__Array_U__N___for__Array_T__N____eq( \
    sz, a1, a2, t, _, _ret_t)                                                           \
  Eurydice_array_eq(sz, a1, a2, t, _)

#define core_array_equality___core__cmp__PartialEq__0___Slice_U____for__Array_T__N___3__eq( \
    sz, a1, a2, t, _, _ret_t)                                                               \
  Eurydice_array_eq(sz, a1, ((a2)->ptr), t, _)

#define Eurydice_slice_split_at(slice, mid, element_type, ret_t)        \
  KRML_CLITERAL(ret_t) {                                                \
    EURYDICE_CFIELD(.fst =){EURYDICE_CFIELD(.ptr =)((slice).ptr),       \
                            EURYDICE_CFIELD(.meta =) mid},              \
        EURYDICE_CFIELD(.snd =) {                                       \
      EURYDICE_CFIELD(.ptr =)                                           \
      ((slice).ptr + mid), EURYDICE_CFIELD(.meta =)((slice).meta - mid) \
    }                                                                   \
  }

#define Eurydice_slice_split_at_mut(slice, mid, element_type, ret_t)  \
  KRML_CLITERAL(ret_t) {                                              \
    EURYDICE_CFIELD(.fst =)                                           \
    KRML_CLITERAL(Eurydice_slice){EURYDICE_CFIELD(.ptr =)(slice.ptr), \
                                  EURYDICE_CFIELD(.len =) mid},       \
        EURYDICE_CFIELD(.snd =) KRML_CLITERAL(Eurydice_slice) {       \
      EURYDICE_CFIELD(.ptr =)                                         \
      ((char *)slice.ptr + mid * sizeof(element_type)),               \
          EURYDICE_CFIELD(.len =)(slice.len - mid)                    \
    }                                                                 \
  }

// Conversion of slice to an array, rewritten (by Eurydice) to name the
// destination array, since arrays are not values in C.
// N.B.: see note in karamel/lib/Inlining.ml if you change this.
#define Eurydice_slice_to_array2(dst, src, _0, t_arr, _1)                 \
  Eurydice_slice_to_array3(&(dst)->tag, (char *)&(dst)->val.case_Ok, src, \
                           sizeof(t_arr))

static KRML_MUSTINLINE void Eurydice_slice_to_array3(uint8_t *dst_tag,
                                                     char *dst_ok,
                                                     Eurydice_slice src,
                                                     size_t sz) {
  *dst_tag = 0;
  memcpy(dst_ok, src.ptr, sz);
}

// CORE STUFF (conversions, endianness, ...)

typedef struct Eurydice_arr_8b_s {
  uint8_t data[2];
} Eurydice_arr_8b;

// [ u8; 2 ]
typedef struct Eurydice_array_u8x2_s {
  uint8_t data[2];
} Eurydice_array_u8x2;

// [ u8; 8 ]
typedef struct Eurydice_array_u8x8_s {
  uint8_t data[8];
} Eurydice_array_u8x8;

// &mut [u8]
typedef struct Eurydice_mut_borrow_slice_u8_s {
  uint8_t *ptr;
  size_t meta;
} Eurydice_mut_borrow_slice_u8;

// &[u8]
typedef struct Eurydice_borrow_slice_u8_s {
  const uint8_t *ptr;
  size_t meta;
} Eurydice_borrow_slice_u8;

static inline Eurydice_array_u8x8 core_num__u64__to_le_bytes(uint64_t v) {
  Eurydice_array_u8x8 a;
  store64_le(a.data, v);
  return a;
}

static inline uint64_t core_num__u64__from_le_bytes(Eurydice_array_u8x8 buf) {
  return load64_le(buf.data);
}

static KRML_MUSTINLINE uint32_t core_num__u32__from_le_bytes(uint8_t buf[4]) {
  return load32_le(buf);
}

static KRML_MUSTINLINE uint32_t core_num__u8__count_ones(uint8_t x0) {
#ifdef _MSC_VER
  return __popcnt(x0);
#else
  return __builtin_popcount(x0);
#endif
}

static inline uint32_t core_num__i32__count_ones(int32_t x0) {
#ifdef _MSC_VER
  return __popcnt(x0);
#else
  return __builtin_popcount(x0);
#endif
}

static inline uint8_t Eurydice_bitand_pv_u8(const uint8_t *p, uint8_t v) {
  return (*p) & v;
}
static inline uint8_t Eurydice_shr_pv_u8(const uint8_t *p, int32_t v) {
  return (*p) >> v;
}

static inline uint8_t
core_ops_bit__core__ops__bit__BitAnd_u8__u8__for__0__u8___bitand(
    const uint8_t *x0, uint8_t x1) {
  return Eurydice_bitand_pv_u8(x0, x1);
}

static inline uint8_t
core_ops_bit__core__ops__bit__Shr_i32__u8__for__0__u8___shr(const uint8_t *x0,
                                                            int32_t x1) {
  return Eurydice_shr_pv_u8(x0, x1);
}

// unsigned overflow wraparound semantics in C
static KRML_MUSTINLINE uint16_t core_num__u16__wrapping_add(uint16_t x,
                                                            uint16_t y) {
  return x + y;
}
static KRML_MUSTINLINE uint8_t core_num__u8__wrapping_sub(uint8_t x,
                                                          uint8_t y) {
  return x - y;
}

static KRML_MUSTINLINE uint64_t core_num__u64__rotate_left(uint64_t x0,
                                                           uint32_t x1) {
  return (x0 << x1 | x0 >> (64 - x1));
}

// ITERATORS

#define Eurydice_range_iter_next(iter_ptr, t, ret_t)      \
  (((iter_ptr)->start >= (iter_ptr)->end)                 \
       ? (KRML_CLITERAL(ret_t){EURYDICE_CFIELD(.tag =) 0, \
                               EURYDICE_CFIELD(.f0 =) 0}) \
       : (KRML_CLITERAL(ret_t){EURYDICE_CFIELD(.tag =) 1, \
                               EURYDICE_CFIELD(.f0 =)(iter_ptr)->start++}))

#define core_iter_range___core__iter__traits__iterator__Iterator_A__for_core__ops__range__Range_A__TraitClause_0___6__next \
  Eurydice_range_iter_next

// See note in karamel/lib/Inlining.ml if you change this
#define Eurydice_into_iter(x, t, _ret_t, _) (x)
#define core_iter_traits_collect___core__iter__traits__collect__IntoIterator_Clause1_Item__I__for_I__1__into_iter \
  Eurydice_into_iter

// OPTIONS

#define core_option__core__option__Option_T__TraitClause_0___is_some( \
    x, _of_type, _)                                                   \
  x->tag
