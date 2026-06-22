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
 * Libcrux: 3687467117fe5c6ddf8cdeb78306adc5d11ead2d
 */


#ifndef internal_libcrux_mldsa_core_H
#define internal_libcrux_mldsa_core_H

#include "eurydice_glue.h"


#if defined(__cplusplus)
extern "C" {
#endif

#include "internal/combined_core.h"
#include "combined_core.h"
#include "../libcrux_mldsa_core.h"

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $7size_t
*/
typedef struct Eurydice_arr_bb_s { Eurydice_arr_a3 data[7U]; } Eurydice_arr_bb;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_bb

*/
typedef struct core_option_Option_2d_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_bb f0;
}
core_option_Option_2d;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $56size_t
*/
typedef struct Eurydice_arr_0f_s { Eurydice_arr_a3 data[56U]; } Eurydice_arr_0f;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 56
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_208(const Eurydice_arr_0f *a);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $8size_t
*/
typedef struct Eurydice_arr_8f_s { Eurydice_arr_a3 data[8U]; } Eurydice_arr_8f;

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients[[$8size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_6a(const Eurydice_arr_8f *val);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $15size_t
*/
typedef struct Eurydice_arr_92_s { Eurydice_arr_a3 data[15U]; } Eurydice_arr_92;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 15
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_207(const Eurydice_arr_92 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 7
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_206(const Eurydice_arr_bb *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients, core_ops_range_Range size_t, Eurydice_derefed_slice libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 15
*/
Eurydice_dst_ref_shared_44
Eurydice_array_to_subslice_shared_251(const Eurydice_arr_92 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 7
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_208(Eurydice_arr_bb *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 56
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_207(Eurydice_arr_0f *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 15
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_206(Eurydice_arr_92 *a);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $5size_t
*/
typedef struct Eurydice_arr_5d_s { Eurydice_arr_a3 data[5U]; } Eurydice_arr_5d;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_5d

*/
typedef struct core_option_Option_1e_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_5d f0;
}
core_option_Option_1e;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $30size_t
*/
typedef struct Eurydice_arr_5a_s { Eurydice_arr_a3 data[30U]; } Eurydice_arr_5a;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 30
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_205(const Eurydice_arr_5a *a);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $6size_t
*/
typedef struct Eurydice_arr_dc1_s { Eurydice_arr_a3 data[6U]; } Eurydice_arr_dc1;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 6
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_204(const Eurydice_arr_dc1 *a);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients[[$6size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_b2(const Eurydice_arr_dc1 *val);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 6
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_205(Eurydice_arr_dc1 *a);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $11size_t
*/
typedef struct Eurydice_arr_47_s { Eurydice_arr_a3 data[11U]; } Eurydice_arr_47;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 11
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_203(const Eurydice_arr_47 *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 5
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_202(const Eurydice_arr_5d *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients, core_ops_range_Range size_t, Eurydice_derefed_slice libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 11
*/
Eurydice_dst_ref_shared_44
Eurydice_array_to_subslice_shared_250(const Eurydice_arr_47 *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 5
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_204(Eurydice_arr_5d *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 30
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_203(Eurydice_arr_5a *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 11
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_202(Eurydice_arr_47 *a);

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $4size_t
*/
typedef struct Eurydice_arr_9d_s { Eurydice_arr_a3 data[4U]; } Eurydice_arr_9d;

/**
A monomorphic instance of core.option.Option
with types Eurydice_arr_9d

*/
typedef struct core_option_Option_d9_s
{
  core_option_Option_45_tags tag;
  Eurydice_arr_9d f0;
}
core_option_Option_d9;

/**
A monomorphic instance of Eurydice.arr
with types libcrux_ml_dsa_polynomial_PolynomialRingElement_e8
with const generics
- $16size_t
*/
typedef struct Eurydice_arr_2f_s { Eurydice_arr_a3 data[16U]; } Eurydice_arr_2f;

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 16
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_201(const Eurydice_arr_2f *a);

/**
 Declassify secret memory.

 No-op if `valgrind_ct_test` cfg is not enabled.
*/
/**
A monomorphic instance of libcrux_secrets.mem_requests.ct_declassify
with types Eurydice_arr libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients[[$4size_t]]

*/
void libcrux_secrets_mem_requests_ct_declassify_f5(const Eurydice_arr_9d *val);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_200(const Eurydice_arr_8f *a);

/**
A monomorphic instance of Eurydice.array_to_slice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 4
*/
Eurydice_dst_ref_shared_44 Eurydice_array_to_slice_shared_20(const Eurydice_arr_9d *a);

/**
A monomorphic instance of Eurydice.array_to_subslice_shared
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients, core_ops_range_Range size_t, Eurydice_derefed_slice libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 8
*/
Eurydice_dst_ref_shared_44
Eurydice_array_to_subslice_shared_25(const Eurydice_arr_8f *a, core_ops_range_Range_87 r);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 4
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_201(Eurydice_arr_9d *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 16
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_200(Eurydice_arr_2f *a);

/**
A monomorphic instance of Eurydice.array_to_slice_mut
with types libcrux_ml_dsa_polynomial_PolynomialRingElement libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics
- N= 8
*/
Eurydice_dst_ref_mut_44 Eurydice_array_to_slice_mut_20(Eurydice_arr_8f *a);

#if defined(__cplusplus)
}
#endif

#define internal_libcrux_mldsa_core_H_DEFINED
#endif /* internal_libcrux_mldsa_core_H */
