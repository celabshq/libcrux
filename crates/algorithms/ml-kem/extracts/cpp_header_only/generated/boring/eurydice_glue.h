/*
 * SPDX-FileCopyrightText: 2024 Eurydice Contributors
 * SPDX-FileCopyrightText: 2024 - 2025 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 */

// Note that the Rust code must check, either at runtime or with verification,
// that any arithmetic in this file does overflow, underflow, or run out of
// bounds.
//
// TODO(crbug.com/404286922): Use designated initializers instead of /*name=*/
// comments when switching to C++20.

#ifndef EURYDICE_HEADER_EURYDICE_GLUE_H
#define EURYDICE_HEADER_EURYDICE_GLUE_H

#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <utility>

#ifdef _MSC_VER
// For __popcnt
#include <intrin.h>
#endif

#include "karamel/target.h"

// In order to be compatible with C++17 we need to work around the fact that we
// can not use designated initializers. This is an issue for unions. The
// following defines `KRML_UNION_CONSTRUCTOR` for this purpose.
// This is used in union definitions such as
// ```c
// typedef struct Result_s {
//  uint8_t tag;
//  union U {
//    int16_t case_Ok[16U];
//    uint8_t case_Err;
//  } val;
//  KRML_UNION_CONSTRUCTOR(Result_s)
//} Result_0a;
// ```

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

// GENERAL-PURPOSE STUFF

#define LowStar_Ignore_ignore(e, t, _ret_t) ((void)e)

// SLICES, ARRAYS, ETC.

// We represent a slice as a pair of an (untyped) pointer, along with the length
// of the slice, i.e. the number of elements in the slice (this is NOT the
// number of bytes). This design choice has two important consequences.
// - if you need to use `ptr`, you MUST cast it to a proper type *before*
//   performing pointer arithmetic on it (remember that C desugars pointer
//   arithmetic based on the type of the address)
// - if you need to use `len` for a C style function (e.g. memcpy, memcmp), you
//   need to multiply it by sizeof t, where t is the type of the elements.
typedef struct {
  void *ptr;
  size_t len;
} Eurydice_slice;

// Helper macro to create a slice out of a pointer x, a start index in x
// (included), and an end index in x (excluded). The argument x must be suitably
// cast to something that can decay (see remark above about how pointer
// arithmetic works in C), meaning either pointer or array type.
#define EURYDICE_SLICE(x, start, end) \
  (Eurydice_slice{(void *)(x + start), end - start})

// Slice length
#define Eurydice_slice_len(s, _) (s).len

// This macro is a pain because in case the dereferenced element type is an
// array, you cannot simply write `t x` as it would yield `int[4] x` instead,
// which is NOT correct C syntax, so we add a dedicated phase in Eurydice that
// adds an extra argument to this macro at the last minute so that we have the
// correct type of *pointers* to elements.
#define Eurydice_slice_index(s, i, t, t_ptr_t) (((t_ptr_t)s.ptr)[i])

// The following functions get sub slices from a slice.

// Variant for when the start and end indices are statically known (i.e., the
// range argument `r` is a literal).
#define Eurydice_slice_subslice2(s, start, end, t) \
  EURYDICE_SLICE((t *)s.ptr, (start), (end))

#define Eurydice_slice_subslice_to(s, subslice_end_pos, t, _0, _1) \
  EURYDICE_SLICE((t *)s.ptr, 0, (subslice_end_pos))

#define Eurydice_slice_subslice_from(s, subslice_start_pos, t, _0, _1) \
  EURYDICE_SLICE((t *)s.ptr, (subslice_start_pos), s.len)

#define Eurydice_array_to_slice(end, x, _t)             \
  /* x is already at an array type, no need for cast */ \
  EURYDICE_SLICE(x, 0, end)

// Same as above, variant for when start and end are statically known
#define Eurydice_array_to_subslice2(x, start, end, t) \
  EURYDICE_SLICE((t *)x, (start), (end))

// The following functions convert an array into a slice.

#define Eurydice_array_to_subslice_to(_size, x, r, t, _range_t, _0) \
  EURYDICE_SLICE((t *)x, 0, r)
#define Eurydice_array_to_subslice_from(size, x, r, t, _range_t, _0) \
  EURYDICE_SLICE((t *)x, r, size)

// Copy a slice with memcpy
#define Eurydice_slice_copy(dst, src, t) \
  memcpy(dst.ptr, src.ptr, dst.len * sizeof(t))

#define core_array___Array_T__N__23__as_slice(len_, ptr_, t, _ret_t) \
  Eurydice_slice { ptr_, len_ }

#define core_array___core__clone__Clone_for__Array_T__N___20__clone( \
    len, src, dst, elem_type, _ret_t)                                \
  (memcpy(dst, src, len * sizeof(elem_type)))
#define TryFromSliceError uint8_t

// Compare two arrays with `memcmp`.
#define Eurydice_array_eq(sz, a1, a2, t, _) \
  (memcmp(a1, a2, sz * sizeof(t)) == 0)

// Map Rust core equality functions to `Eurydice_array_eq`.
#define core_array_equality___core__cmp__PartialEq__Array_U__N___for__Array_T__N____eq( \
    sz, a1, a2, t, _, _ret_t)                                                           \
  Eurydice_array_eq(sz, a1, a2, t, _)
#define core_array_equality___core__cmp__PartialEq__0___Slice_U____for__Array_T__N___3__eq( \
    sz, a1, a2, t, _, _ret_t)                                                               \
  Eurydice_array_eq(sz, a1, ((a2)->ptr), t, _)

// Rust `split_at` on arrays and slices to generate two slices.
#define Eurydice_slice_split_at(slice, mid, element_type, ret_t)      \
  ret_t {                                                             \
    /* .fst = */ EURYDICE_SLICE((element_type *)(slice).ptr, 0, mid), \
        /* .snd = */ EURYDICE_SLICE((element_type *)(slice).ptr, mid, \
                                    (slice).len)                      \
  }

// Conversion of slice to an array, rewritten (by Eurydice) to name the
// destination array, since arrays are not values in C.
// N.B.: see note in karamel/lib/Inlining.ml if you change this.
// This is generated from Rust code such as `let array: [T; N] =
// slice[a..b].try_into().unwrap()`, where a slice is copied into a fixed sized
// array. Because this may always fail, `dst` is a result type and the value is
// written into the `dst->val.case_Ok`.
#define Eurydice_slice_to_array2(dst, src, _0, t_arr, _1)                 \
  Eurydice_slice_to_array3(&(dst)->tag, (char *)&(dst)->val.case_Ok, src, \
                           sizeof(t_arr))

static inline void Eurydice_slice_to_array3(uint8_t *dst_tag, char *dst_ok,
                                            Eurydice_slice src, size_t sz) {
  *dst_tag = 0;
  memcpy(dst_ok, src.ptr, sz);
}

// CORE STUFF (conversions, endianness, ...)

static inline void core_num__u64_9__to_le_bytes(uint64_t v, uint8_t buf[8]) {
  CRYPTO_store_u64_le(buf, v);
}
static inline uint64_t core_num__u64_9__from_le_bytes(uint8_t buf[8]) {
  return CRYPTO_load_u64_le(buf);
}

static inline uint32_t core_num__u8_6__count_ones(uint8_t x0) {
#ifdef _MSC_VER
  return __popcnt(x0);
#else
  return __builtin_popcount(x0);
#endif
}

// unsigned overflow wraparound semantics in C
static inline uint16_t core_num__u16_7__wrapping_add(uint16_t x, uint16_t y) {
  return x + y;
}
static inline uint8_t core_num__u8_6__wrapping_sub(uint8_t x, uint8_t y) {
  return x - y;
}
static inline uint64_t core_num__u64_9__rotate_left(uint64_t value,
                                                    uint32_t shift) {
  return CRYPTO_rotl_u64(value, shift);
}

#endif /* EURYDICE_HEADER_EURYDICE_GLUE_H */
