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


#include "internal/combined_core.h"

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
Eurydice_mut_borrow_slice_i16
Eurydice_slice_subslice_mut_a6(Eurydice_mut_borrow_slice_i16 s, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_i16){
        .ptr = s.ptr + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$24size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_94 core_result_unwrap_26_78(core_result_Result_57 self)
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
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$20size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_fc core_result_unwrap_26_7d(core_result_Result_83 self)
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
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f2(const Eurydice_arr_5f *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)1184U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_to_shared_210(const Eurydice_arr_5f *a, size_t r)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 2400
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d48(const Eurydice_arr_7d *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1152
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f4(const Eurydice_arr_0e *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d417(Eurydice_arr_7d *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1152
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f4(Eurydice_arr_0e *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f4(Eurydice_arr_5f *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)1184U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1184
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d416(Eurydice_arr_5f *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 24
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_ed(const Eurydice_arr_94 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)24U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 384
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d415(Eurydice_arr_b20 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 384
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_a9(const Eurydice_arr_b20 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)384U;
  return lit;
}

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$32size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_ec core_result_unwrap_26_39(core_result_Result_07 self)
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
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f1(const Eurydice_arr_c7 *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)64U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d47(const Eurydice_arr_c7 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1184
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_ff(const Eurydice_arr_5f *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1184U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f3(Eurydice_arr_2b *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)1088U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d414(Eurydice_arr_2b *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 20
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_8f(const Eurydice_arr_fc *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)20U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 320
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d413(Eurydice_arr_b0 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 320
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_56(const Eurydice_arr_b0 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)320U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int16_t
with const generics
- N= 256
*/
Eurydice_borrow_slice_i16 Eurydice_array_to_slice_shared_99(const Eurydice_arr_04 *a)
{
  Eurydice_borrow_slice_i16 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)256U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 33
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_b5(const Eurydice_arr_fa0 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)33U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 33
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d412(Eurydice_arr_fa0 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
Eurydice_borrow_slice_i16
Eurydice_array_to_subslice_shared_e70(const Eurydice_arr_5b *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_i16){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d46(const Eurydice_arr_c5 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 272
*/
Eurydice_mut_borrow_slice_i16
Eurydice_array_to_subslice_mut_e7(Eurydice_arr_5b *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_i16){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 504
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d45(const Eurydice_arr_79 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 504
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_48(Eurydice_arr_79 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)504U;
  return lit;
}

/**
A monomorphic instance of Eurydice.slice_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_borrow_slice_u8
Eurydice_slice_subslice_from_shared_6d(Eurydice_borrow_slice_u8 s, size_t r)
{
  return (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = s.ptr + r, .meta = s.meta - r });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1120
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_81(const Eurydice_arr_af *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_06(const Eurydice_arr_2b *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f2(Eurydice_arr_af *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)1120U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1120
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d411(Eurydice_arr_af *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f1(Eurydice_arr_c7 *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)64U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 64
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d410(Eurydice_arr_c7 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f0(const Eurydice_arr_2b *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)1088U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 1088
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d44(const Eurydice_arr_2b *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2400
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_51(const Eurydice_arr_7d *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2400U;
  return lit;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$24size_t]]

*/
Eurydice_arr_94 libcrux_secrets_int_public_integers_declassify_d8_40(Eurydice_arr_94 self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$20size_t]]

*/
Eurydice_arr_fc libcrux_secrets_int_public_integers_declassify_d8_2b(Eurydice_arr_fc self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$8size_t]]

*/
Eurydice_array_u8x8
libcrux_secrets_int_public_integers_declassify_d8_52(Eurydice_array_u8x8 self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types Eurydice_arr uint8_t[[$2size_t]]

*/
Eurydice_array_u8x2
libcrux_secrets_int_public_integers_declassify_d8_75(Eurydice_array_u8x2 self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types Eurydice_arr int16_t[[$16size_t]]

*/
Eurydice_arr_d6 libcrux_secrets_int_public_integers_classify_27_4b(Eurydice_arr_d6 self)
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
Eurydice_borrow_slice_u8
libcrux_secrets_int_classify_public_classify_ref_6d_90(Eurydice_borrow_slice_u8 self)
{
  return self;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t
with const generics
- N= 16
*/
Eurydice_borrow_slice_i16
Eurydice_array_to_subslice_shared_e7(const Eurydice_arr_d6 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_i16){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
This function found in impl {libcrux_secrets::traits::ClassifyRef<&'a ([T])> for &'a ([T])}
*/
/**
A monomorphic instance of libcrux_secrets.int.classify_public.classify_ref_6d
with types int16_t

*/
Eurydice_borrow_slice_i16
libcrux_secrets_int_classify_public_classify_ref_6d_39(Eurydice_borrow_slice_i16 self)
{
  return self;
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types int16_t, core_ops_range_Range size_t, Eurydice_derefed_slice int16_t

*/
Eurydice_borrow_slice_i16
Eurydice_slice_subslice_shared_a6(Eurydice_borrow_slice_i16 s, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_i16){ .ptr = s.ptr + r.start, .meta = r.end - r.start });
}

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr int16_t[[$16size_t]], core_array_TryFromSliceError

*/
Eurydice_arr_d6 core_result_unwrap_26_d3(core_result_Result_ec self)
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
- N= 24
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d43(const Eurydice_arr_94 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 24
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d49(Eurydice_arr_94 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 16
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_29(Eurydice_arr_b2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d42(const Eurydice_arr_b2 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 16
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d48(Eurydice_arr_b2 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 19
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d41(const Eurydice_arr_38 *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 19
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d47(Eurydice_arr_38 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.slice_subslice_mut
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t

*/
Eurydice_dst_ref_mut_83
Eurydice_slice_subslice_mut_47(Eurydice_dst_ref_mut_83 s, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_dst_ref_mut_83){ .ptr = s.ptr + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 16
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_29(const Eurydice_arr_b2 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)16U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_mut
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_subslice_to_mut_21(Eurydice_arr_ec *a, size_t r)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = r;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4627
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_11(const Eurydice_arr_93 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4627U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2592
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_fc(const Eurydice_arr_43 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2592U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4896
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f7(const Eurydice_arr_e2 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4896U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 4627
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_11(Eurydice_arr_93 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4627U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_20 Eurydice_array_to_slice_shared_861(const Eurydice_arr_81 *a)
{
  Eurydice_dst_ref_shared_20 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$256size_t]]
with const generics
- N= 8
*/
Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_861(Eurydice_arr_81 *a)
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
void libcrux_secrets_mem_requests_ct_declassify_56(const Eurydice_arr_c7 *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1024
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_68(const Eurydice_arr_1b *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_68(Eurydice_arr_1b *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_fc(Eurydice_arr_43 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_f7(Eurydice_arr_e2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4896U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 3309
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_6b(const Eurydice_arr_0c *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)3309U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1952
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_37(const Eurydice_arr_29 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1952U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 4032
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_98(const Eurydice_arr_24 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4032U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 3309
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_6b(Eurydice_arr_0c *a)
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
Eurydice_dst_ref_shared_20 Eurydice_array_to_slice_shared_860(const Eurydice_arr_5d0 *a)
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
Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_860(Eurydice_arr_5d0 *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_9f0(const Eurydice_arr_65 *a)
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
void libcrux_secrets_mem_requests_ct_declassify_69(const Eurydice_arr_65 *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1952
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_37(Eurydice_arr_29 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_98(Eurydice_arr_24 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4032U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2420
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_0d(const Eurydice_arr_85 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2420U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1312
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_9f(const Eurydice_arr_02 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)1312U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 2560
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_34(const Eurydice_arr_10 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)2560U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 2420
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_0d(Eurydice_arr_85 *a)
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
Eurydice_dst_ref_shared_20 Eurydice_array_to_slice_shared_86(const Eurydice_arr_b7 *a)
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
Eurydice_dst_ref_mut_20 Eurydice_array_to_slice_mut_86(Eurydice_arr_b7 *a)
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
Eurydice_dst_ref_mut_83
Eurydice_array_to_subslice_mut_44(Eurydice_arr_6c *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_dst_ref_mut_83){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 256
*/
Eurydice_dst_ref_shared_83 Eurydice_array_to_slice_shared_af(const Eurydice_arr_6c *a)
{
  Eurydice_dst_ref_shared_83 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)256U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_shared
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_from_shared_5f(const Eurydice_arr_ff *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)136U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d40(const Eurydice_arr_ff *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr uint8_t[[$32size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_4b(const Eurydice_arr_ec *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 768
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_27(const Eurydice_arr_d2 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_27(Eurydice_arr_d2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)768U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 640
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_4f(const Eurydice_arr_20 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_4f(Eurydice_arr_20 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)640U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 576
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_8a(const Eurydice_arr_220 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_8a(Eurydice_arr_220 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)576U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 11
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_2f(const Eurydice_arr_c9 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)11U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 1
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_79(const Eurydice_arr_82 *a)
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
void libcrux_secrets_mem_requests_ct_classify_45(const uint8_t (*val)[])
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 1312
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f0(Eurydice_arr_02 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_34(Eurydice_arr_10 *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_17(const Eurydice_arr_c7 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)64U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types Eurydice_arr int32_t[[$263size_t]]
with const generics
- N= 4
*/
Eurydice_dst_ref_mut_33 Eurydice_array_to_slice_mut_7e(Eurydice_arr_930 *a)
{
  Eurydice_dst_ref_mut_33 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)4U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 840
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_4c(const Eurydice_arr_d10 *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e9(const Eurydice_arr_31 *a)
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
void libcrux_secrets_mem_requests_ct_declassify_45(const uint8_t (*val)[])
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 263
*/
Eurydice_dst_ref_shared_83 Eurydice_array_to_slice_shared_2c0(const Eurydice_arr_d0 *a)
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
Eurydice_dst_ref_mut_83 Eurydice_array_to_subslice_from_mut_11(Eurydice_arr_d0 *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_dst_ref_mut_83){ .ptr = a->data + r, .meta = (size_t)263U - r });
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 66
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_f1(const Eurydice_arr_91 *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_78(const Eurydice_arr_89 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_78(Eurydice_arr_89 *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_82(const Eurydice_array_u8x2 *a)
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_01(const Eurydice_arr_ec *a)
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
void libcrux_secrets_mem_requests_ct_classify_4b(const Eurydice_arr_ec *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 168
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_2c(Eurydice_arr_c5 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)168U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 840
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_4c(Eurydice_arr_d10 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)840U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 136
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_58(Eurydice_arr_ff *a)
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
- N= 32
*/
Eurydice_borrow_slice_u8
Eurydice_array_to_subslice_shared_d4(const Eurydice_arr_ec *a, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = a->data + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 32
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d46(Eurydice_arr_ec *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f0(Eurydice_arr_c5 *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)168U - r });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 64
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_17(Eurydice_arr_c7 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_9f(Eurydice_arr_65 *a)
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
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_01(Eurydice_arr_ec *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)32U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types uint8_t
with const generics
- N= 28
*/
Eurydice_mut_borrow_slice_u8 Eurydice_array_to_slice_mut_5e(Eurydice_arr_a2 *a)
{
  Eurydice_mut_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)28U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 104
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_72(const Eurydice_arr_c4 *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d45(Eurydice_arr_c4 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 144
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_38(const Eurydice_arr_f4 *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d44(Eurydice_arr_f4 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 72
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_e2(const Eurydice_arr_ab *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d43(Eurydice_arr_ab *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.slice_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_borrow_slice_u8
Eurydice_slice_subslice_to_shared_72(Eurydice_borrow_slice_u8 s, size_t r)
{
  return (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = s.ptr, .meta = r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_from_mut
with types uint8_t, core_ops_range_RangeFrom size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 136
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_from_mut_5f(Eurydice_arr_ff *a, size_t r)
{
  return
    (KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = a->data + r, .meta = (size_t)136U - r });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_to_shared
with types uint8_t, core_ops_range_RangeTo size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 8
*/
Eurydice_borrow_slice_u8
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
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_6e(const Eurydice_array_u8x8 *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_slice_subslice_mut_c8(Eurydice_mut_borrow_slice_u8 s, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){ .ptr = s.ptr + r.start, .meta = r.end - r.start }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 136
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_58(const Eurydice_arr_ff *a)
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
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d42(Eurydice_arr_ff *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types uint8_t
with const generics
- N= 168
*/
Eurydice_borrow_slice_u8 Eurydice_array_to_slice_shared_2c(const Eurydice_arr_c5 *a)
{
  Eurydice_borrow_slice_u8 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)168U;
  return lit;
}

/**
This function found in impl {core::result::Result<T, E>[TraitClause@0, TraitClause@1]}
*/
/**
A monomorphic instance of core.result.unwrap_26
with types Eurydice_arr uint8_t[[$8size_t]], core_array_TryFromSliceError

*/
Eurydice_array_u8x8 core_result_unwrap_26_e0(core_result_Result_8e self)
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
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 168
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d41(Eurydice_arr_c5 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.slice_subslice_shared
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t

*/
Eurydice_borrow_slice_u8
Eurydice_slice_subslice_shared_c8(Eurydice_borrow_slice_u8 s, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_borrow_slice_u8){ .ptr = s.ptr + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types int32_t, core_ops_range_Range size_t, Eurydice_derefed_slice int32_t
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_83
Eurydice_array_to_subslice_shared_44(const Eurydice_arr_4d *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_dst_ref_shared_83){ .ptr = a->data + r.start, .meta = r.end - r.start }
    );
}

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types bool

*/
void libcrux_secrets_mem_requests_ct_declassify_5f(const bool *val)
{

}

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types int32_t
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_83 Eurydice_array_to_slice_shared_fd(const Eurydice_arr_4d *a)
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
Eurydice_dst_ref_shared_83
Eurydice_slice_subslice_shared_47(Eurydice_dst_ref_shared_83 s, core_ops_range_Range_87 r)
{
  return
    (KRML_CLITERAL(Eurydice_dst_ref_shared_83){ .ptr = s.ptr + r.start, .meta = r.end - r.start });
}

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types int32_t
with const generics
- N= 8
*/
Eurydice_dst_ref_mut_83 Eurydice_array_to_slice_mut_fd(Eurydice_arr_4d *a)
{
  Eurydice_dst_ref_mut_83 lit;
  lit.ptr = a->data;
  lit.meta = (size_t)8U;
  return lit;
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 34
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d40(Eurydice_arr_31 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
A monomorphic instance of Eurydice.array_to_subslice_mut
with types uint8_t, core_ops_range_Range size_t, Eurydice_derefed_slice uint8_t
with const generics
- N= 66
*/
Eurydice_mut_borrow_slice_u8
Eurydice_array_to_subslice_mut_d4(Eurydice_arr_91 *a, core_ops_range_Range_87 r)
{
  return
    (
      KRML_CLITERAL(Eurydice_mut_borrow_slice_u8){
        .ptr = a->data + r.start,
        .meta = r.end - r.start
      }
    );
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint64_t

*/
uint64_t libcrux_secrets_int_public_integers_declassify_d8_49(uint64_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint32_t

*/
uint32_t libcrux_secrets_int_public_integers_classify_27_df(uint32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint64_t

*/
uint64_t libcrux_secrets_int_public_integers_classify_27_49(uint64_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint16_t

*/
uint16_t libcrux_secrets_int_public_integers_declassify_d8_de(uint16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint16_t

*/
uint16_t libcrux_secrets_int_public_integers_classify_27_de(uint16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint32_t

*/
uint32_t libcrux_secrets_int_public_integers_declassify_d8_df(uint32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types int32_t

*/
int32_t libcrux_secrets_int_public_integers_declassify_d8_a8(int32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types int32_t

*/
int32_t libcrux_secrets_int_public_integers_classify_27_a8(int32_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types uint8_t

*/
uint8_t libcrux_secrets_int_public_integers_declassify_d8_90(uint8_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types int16_t

*/
int16_t libcrux_secrets_int_public_integers_classify_27_39(int16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Declassify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.declassify_d8
with types int16_t

*/
int16_t libcrux_secrets_int_public_integers_declassify_d8_39(int16_t self)
{
  return self;
}

/**
This function found in impl {libcrux_secrets::traits::Classify<T> for T}
*/
/**
A monomorphic instance of libcrux_secrets.int.public_integers.classify_27
with types uint8_t

*/
uint8_t libcrux_secrets_int_public_integers_classify_27_90(uint8_t self)
{
  return self;
}

