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
 * Libcrux: c5f306978fa7d13b590fd71dbc260386f3936d57
 */


#ifndef libcrux_mldsa_portable_H
#define libcrux_mldsa_portable_H

#include "eurydice_glue.h"



#include "libcrux_sha3_portable.h"
#include "libcrux_mlkem_core.h"
#include "libcrux_mldsa_core.h"
#include "combined_core.h"

typedef struct libcrux_ml_dsa_hash_functions_portable_Shake128X4_s
{
  Eurydice_arr_7c state0;
  Eurydice_arr_7c state1;
  Eurydice_arr_7c state2;
  Eurydice_arr_7c state3;
}
libcrux_ml_dsa_hash_functions_portable_Shake128X4;

typedef libcrux_sha3_portable_KeccakState libcrux_ml_dsa_hash_functions_portable_Shake256;

typedef struct libcrux_ml_dsa_hash_functions_portable_Shake256X4_s
{
  Eurydice_arr_7c state0;
  Eurydice_arr_7c state1;
  Eurydice_arr_7c state2;
  Eurydice_arr_7c state3;
}
libcrux_ml_dsa_hash_functions_portable_Shake256X4;

typedef libcrux_sha3_portable_incremental_Shake256Xof
libcrux_ml_dsa_hash_functions_portable_Shake256Xof;

static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake128X4
libcrux_ml_dsa_hash_functions_portable_init_absorb(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
)
{
  Eurydice_arr_7c state0 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state0, input0);
  Eurydice_arr_7c state1 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state1, input1);
  Eurydice_arr_7c state2 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state2, input2);
  Eurydice_arr_7c state3 = libcrux_sha3_portable_incremental_shake128_init();
  libcrux_sha3_portable_incremental_shake128_absorb_final(&state3, input3);
  return (libcrux_ml_dsa_hash_functions_portable_Shake128X4{ state0, state1, state2, state3 });
}

static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_ml_dsa_hash_functions_portable_init_absorb_final_shake256(
  Eurydice_borrow_slice_u8 input
)
{
  Eurydice_arr_7c state = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state, input);
  return state;
}

static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake256X4
libcrux_ml_dsa_hash_functions_portable_init_absorb_x4(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
)
{
  Eurydice_arr_7c state0 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state0, input0);
  Eurydice_arr_7c state1 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state1, input1);
  Eurydice_arr_7c state2 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state2, input2);
  Eurydice_arr_7c state3 = libcrux_sha3_portable_incremental_shake256_init();
  libcrux_sha3_portable_incremental_shake256_absorb_final(&state3, input3);
  return (libcrux_ml_dsa_hash_functions_portable_Shake256X4{ state0, state1, state2, state3 });
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake128(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_sha3_portable_shake128(out, input);
}

static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_shake256(Eurydice_arr_7c *state)
{
  Eurydice_arr_ff out = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(state,
    Eurydice_array_to_slice_mut_58(&out));
  return out;
}

static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4(
  libcrux_ml_dsa_hash_functions_portable_Shake256X4 *state
)
{
  Eurydice_arr_ff out0 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(&state->state0,
    Eurydice_array_to_slice_mut_58(&out0));
  Eurydice_arr_ff out1 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(&state->state1,
    Eurydice_array_to_slice_mut_58(&out1));
  Eurydice_arr_ff out2 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(&state->state2,
    Eurydice_array_to_slice_mut_58(&out2));
  Eurydice_arr_ff out3 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_first_block(&state->state3,
    Eurydice_array_to_slice_mut_58(&out3));
  return (Eurydice_arr_ff_x4{ out0, out1, out2, out3 });
}

static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks(
  libcrux_ml_dsa_hash_functions_portable_Shake128X4 *state,
  Eurydice_arr_d10 *out0,
  Eurydice_arr_d10 *out1,
  Eurydice_arr_d10 *out2,
  Eurydice_arr_d10 *out3
)
{
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(&state->state0,
    Eurydice_array_to_slice_mut_4c(out0));
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(&state->state1,
    Eurydice_array_to_slice_mut_4c(out1));
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(&state->state2,
    Eurydice_array_to_slice_mut_4c(out2));
  libcrux_sha3_portable_incremental_shake128_squeeze_first_five_blocks(&state->state3,
    Eurydice_array_to_slice_mut_4c(out3));
}

static KRML_MUSTINLINE Eurydice_arr_c5_x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block(
  libcrux_ml_dsa_hash_functions_portable_Shake128X4 *state
)
{
  Eurydice_arr_c5 out0 = { { 0U } };
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(&state->state0,
    Eurydice_array_to_slice_mut_2c(&out0));
  Eurydice_arr_c5 out1 = { { 0U } };
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(&state->state1,
    Eurydice_array_to_slice_mut_2c(&out1));
  Eurydice_arr_c5 out2 = { { 0U } };
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(&state->state2,
    Eurydice_array_to_slice_mut_2c(&out2));
  Eurydice_arr_c5 out3 = { { 0U } };
  libcrux_sha3_portable_incremental_shake128_squeeze_next_block(&state->state3,
    Eurydice_array_to_slice_mut_2c(&out3));
  return (Eurydice_arr_c5_x4{ out0, out1, out2, out3 });
}

static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_shake256(Eurydice_arr_7c *state)
{
  Eurydice_arr_ff out = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(state,
    Eurydice_array_to_slice_mut_58(&out));
  return out;
}

static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4(
  libcrux_ml_dsa_hash_functions_portable_Shake256X4 *state
)
{
  Eurydice_arr_ff out0 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(&state->state0,
    Eurydice_array_to_slice_mut_58(&out0));
  Eurydice_arr_ff out1 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(&state->state1,
    Eurydice_array_to_slice_mut_58(&out1));
  Eurydice_arr_ff out2 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(&state->state2,
    Eurydice_array_to_slice_mut_58(&out2));
  Eurydice_arr_ff out3 = { { 0U } };
  libcrux_sha3_portable_incremental_shake256_squeeze_next_block(&state->state3,
    Eurydice_array_to_slice_mut_58(&out3));
  return (Eurydice_arr_ff_x4{ out0, out1, out2, out3 });
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::Xof for libcrux_ml_dsa::hash_functions::portable::Shake128}
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake128_7b(
  Eurydice_borrow_slice_u8 input,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_ml_dsa_hash_functions_portable_shake128(input, out);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake128X4}
*/
static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake128X4
libcrux_ml_dsa_hash_functions_portable_init_absorb_11(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
)
{
  return libcrux_ml_dsa_hash_functions_portable_init_absorb(input0, input1, input2, input3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake128X4}
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks_11(
  libcrux_ml_dsa_hash_functions_portable_Shake128X4 *self,
  Eurydice_arr_d10 *out0,
  Eurydice_arr_d10 *out1,
  Eurydice_arr_d10 *out2,
  Eurydice_arr_d10 *out3
)
{
  libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks(self, out0, out1, out2, out3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake128::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake128X4}
*/
static KRML_MUSTINLINE Eurydice_arr_c5_x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(
  libcrux_ml_dsa_hash_functions_portable_Shake128X4 *self
)
{
  return libcrux_ml_dsa_hash_functions_portable_squeeze_next_block(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
static KRML_MUSTINLINE Eurydice_arr_7c
libcrux_ml_dsa_hash_functions_portable_init_absorb_final_61(Eurydice_borrow_slice_u8 input)
{
  return libcrux_ml_dsa_hash_functions_portable_init_absorb_final_shake256(input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_61(Eurydice_arr_7c *self)
{
  return libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_shake256(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
static KRML_MUSTINLINE Eurydice_arr_ff
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_61(Eurydice_arr_7c *self)
{
  return libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_shake256(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline void
libcrux_ml_dsa_hash_functions_portable_absorb_26(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_borrow_slice_u8 input
)
{
  libcrux_sha3_portable_incremental_absorb_42(self, input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline void
libcrux_ml_dsa_hash_functions_portable_absorb_final_26(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_borrow_slice_u8 input
)
{
  libcrux_sha3_portable_incremental_absorb_final_42(self, input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
libcrux_ml_dsa_hash_functions_portable_init_26(void)
{
  return libcrux_sha3_portable_incremental_new_42();
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::Xof for libcrux_ml_dsa::hash_functions::portable::Shake256Xof}
*/
static inline void
libcrux_ml_dsa_hash_functions_portable_squeeze_26(
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *self,
  Eurydice_mut_borrow_slice_u8 out
)
{
  libcrux_sha3_portable_incremental_squeeze_42(self, out);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
static KRML_MUSTINLINE libcrux_ml_dsa_hash_functions_portable_Shake256X4
libcrux_ml_dsa_hash_functions_portable_init_absorb_x4_9b(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3
)
{
  return libcrux_ml_dsa_hash_functions_portable_init_absorb_x4(input0, input1, input2, input3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4_9b(
  libcrux_ml_dsa_hash_functions_portable_Shake256X4 *self
)
{
  return libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4(self);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
static KRML_MUSTINLINE Eurydice_arr_ff_x4
libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(
  libcrux_ml_dsa_hash_functions_portable_Shake256X4 *self
)
{
  return libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4(self);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_four_error_ring_elements
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_four_error_ring_elements_29(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 seed,
  uint16_t start_index,
  Eurydice_dst_ref_mut_44 re
)
{
  Eurydice_arr_91 seed0 = libcrux_ml_dsa_sample_add_error_domain_separator(seed, start_index);
  Eurydice_arr_91
  seed1 = libcrux_ml_dsa_sample_add_error_domain_separator(seed, (uint32_t)start_index + 1U);
  Eurydice_arr_91
  seed2 = libcrux_ml_dsa_sample_add_error_domain_separator(seed, (uint32_t)start_index + 2U);
  Eurydice_arr_91
  seed3 = libcrux_ml_dsa_sample_add_error_domain_separator(seed, (uint32_t)start_index + 3U);
  libcrux_ml_dsa_hash_functions_portable_Shake256X4
  state =
    libcrux_ml_dsa_hash_functions_portable_init_absorb_x4_9b(Eurydice_array_to_slice_shared_f1(&seed0),
      Eurydice_array_to_slice_shared_f1(&seed1),
      Eurydice_array_to_slice_shared_f1(&seed2),
      Eurydice_array_to_slice_shared_f1(&seed3));
  Eurydice_arr_ff_x4
  randomnesses0 = libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_x4_9b(&state);
  Eurydice_arr_93 out = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  size_t sampled0 = (size_t)0U;
  size_t sampled1 = (size_t)0U;
  size_t sampled2 = (size_t)0U;
  size_t sampled3 = (size_t)0U;
  libcrux_ml_dsa_constants_Eta uu____0 = eta;
  bool
  done0 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____0,
      Eurydice_array_to_slice_shared_58(&randomnesses0.fst),
      &sampled0,
      out.data);
  libcrux_ml_dsa_constants_Eta uu____1 = eta;
  bool
  done1 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____1,
      Eurydice_array_to_slice_shared_58(&randomnesses0.snd),
      &sampled1,
      &out.data[1U]);
  libcrux_ml_dsa_constants_Eta uu____2 = eta;
  bool
  done2 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____2,
      Eurydice_array_to_slice_shared_58(&randomnesses0.thd),
      &sampled2,
      &out.data[2U]);
  libcrux_ml_dsa_constants_Eta uu____3 = eta;
  bool
  done3 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____3,
      Eurydice_array_to_slice_shared_58(&randomnesses0.f3),
      &sampled3,
      &out.data[3U]);
  while (true)
  {
    if (done0)
    {
      if (done1)
      {
        if (done2)
        {
          if (done3)
          {
            break;
          }
          else
          {
            Eurydice_arr_ff_x4
            randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(&state);
            if (!done0)
            {
              libcrux_ml_dsa_constants_Eta uu____4 = eta;
              done0 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____4,
                  Eurydice_array_to_slice_shared_58(&randomnesses.fst),
                  &sampled0,
                  out.data);
            }
            if (!done1)
            {
              libcrux_ml_dsa_constants_Eta uu____5 = eta;
              done1 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____5,
                  Eurydice_array_to_slice_shared_58(&randomnesses.snd),
                  &sampled1,
                  &out.data[1U]);
            }
            if (!done2)
            {
              libcrux_ml_dsa_constants_Eta uu____6 = eta;
              done2 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____6,
                  Eurydice_array_to_slice_shared_58(&randomnesses.thd),
                  &sampled2,
                  &out.data[2U]);
            }
            if (!done3)
            {
              libcrux_ml_dsa_constants_Eta uu____7 = eta;
              done3 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____7,
                  Eurydice_array_to_slice_shared_58(&randomnesses.f3),
                  &sampled3,
                  &out.data[3U]);
            }
          }
        }
        else
        {
          Eurydice_arr_ff_x4
          randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(&state);
          if (!done0)
          {
            libcrux_ml_dsa_constants_Eta uu____8 = eta;
            done0 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____8,
                Eurydice_array_to_slice_shared_58(&randomnesses.fst),
                &sampled0,
                out.data);
          }
          if (!done1)
          {
            libcrux_ml_dsa_constants_Eta uu____9 = eta;
            done1 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____9,
                Eurydice_array_to_slice_shared_58(&randomnesses.snd),
                &sampled1,
                &out.data[1U]);
          }
          if (!done2)
          {
            libcrux_ml_dsa_constants_Eta uu____10 = eta;
            done2 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____10,
                Eurydice_array_to_slice_shared_58(&randomnesses.thd),
                &sampled2,
                &out.data[2U]);
          }
          if (!done3)
          {
            libcrux_ml_dsa_constants_Eta uu____11 = eta;
            done3 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____11,
                Eurydice_array_to_slice_shared_58(&randomnesses.f3),
                &sampled3,
                &out.data[3U]);
          }
        }
      }
      else
      {
        Eurydice_arr_ff_x4
        randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(&state);
        if (!done0)
        {
          libcrux_ml_dsa_constants_Eta uu____12 = eta;
          done0 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____12,
              Eurydice_array_to_slice_shared_58(&randomnesses.fst),
              &sampled0,
              out.data);
        }
        if (!done1)
        {
          libcrux_ml_dsa_constants_Eta uu____13 = eta;
          done1 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____13,
              Eurydice_array_to_slice_shared_58(&randomnesses.snd),
              &sampled1,
              &out.data[1U]);
        }
        if (!done2)
        {
          libcrux_ml_dsa_constants_Eta uu____14 = eta;
          done2 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____14,
              Eurydice_array_to_slice_shared_58(&randomnesses.thd),
              &sampled2,
              &out.data[2U]);
        }
        if (!done3)
        {
          libcrux_ml_dsa_constants_Eta uu____15 = eta;
          done3 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____15,
              Eurydice_array_to_slice_shared_58(&randomnesses.f3),
              &sampled3,
              &out.data[3U]);
        }
      }
    }
    else
    {
      Eurydice_arr_ff_x4
      randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_x4_9b(&state);
      if (!done0)
      {
        libcrux_ml_dsa_constants_Eta uu____16 = eta;
        done0 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____16,
            Eurydice_array_to_slice_shared_58(&randomnesses.fst),
            &sampled0,
            out.data);
      }
      if (!done1)
      {
        libcrux_ml_dsa_constants_Eta uu____17 = eta;
        done1 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____17,
            Eurydice_array_to_slice_shared_58(&randomnesses.snd),
            &sampled1,
            &out.data[1U]);
      }
      if (!done2)
      {
        libcrux_ml_dsa_constants_Eta uu____18 = eta;
        done2 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____18,
            Eurydice_array_to_slice_shared_58(&randomnesses.thd),
            &sampled2,
            &out.data[2U]);
      }
      if (!done3)
      {
        libcrux_ml_dsa_constants_Eta uu____19 = eta;
        done3 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_eta_37(uu____19,
            Eurydice_array_to_slice_shared_58(&randomnesses.f3),
            &sampled3,
            &out.data[3U]);
      }
    }
  }
  size_t max0 = (size_t)(uint32_t)start_index + (size_t)4U;
  size_t max;
  if (re.meta < max0)
  {
    max = re.meta;
  }
  else
  {
    max = max0;
  }
  for (size_t i = (size_t)(uint32_t)start_index; i < max; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_polynomial_from_i32_array_ff_37(Eurydice_array_to_slice_shared_2c0(&out.data[i0
        % (size_t)4U]),
      &re.ptr[i0]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.sample_s1_and_s2
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_samplex4_sample_s1_and_s2_29(
  libcrux_ml_dsa_constants_Eta eta,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_44 s1_s2
)
{
  size_t len = s1_s2.meta;
  for (size_t i = (size_t)0U; i < len / (size_t)4U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_sample_sample_four_error_ring_elements_29(eta,
      seed,
      4U * (uint32_t)(uint16_t)i0,
      s1_s2);
  }
  size_t remainder = len % (size_t)4U;
  if (remainder != (size_t)0U)
  {
    libcrux_ml_dsa_sample_sample_four_error_ring_elements_29(eta,
      seed,
      (uint16_t)(len - remainder),
      s1_s2);
  }
}

/**
 Sample and write out up to four ring elements.

 If i <= `elements_requested`, a field element with domain separated
 seed according to the provided index is generated in
 `tmp_stack[i]`. After successful rejection sampling in
 `tmp_stack[i]`, the ring element is written to `matrix` at the
 provided index in `indices[i]`.
 `rand_stack` is a working buffer that holds initial Shake output.
*/
/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_up_to_four_ring_elements_flat
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake128X4
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_63(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_44 matrix,
  Eurydice_arr_d10 *rand_stack0,
  Eurydice_arr_d10 *rand_stack1,
  Eurydice_arr_d10 *rand_stack2,
  Eurydice_arr_d10 *rand_stack3,
  Eurydice_dst_ref_mut_33 tmp_stack,
  size_t start_index,
  size_t elements_requested
)
{
  Eurydice_arr_31
  seed0 =
    libcrux_ml_dsa_sample_add_domain_separator(seed,
      libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(start_index, columns));
  Eurydice_arr_31
  seed1 =
    libcrux_ml_dsa_sample_add_domain_separator(seed,
      libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(start_index + (size_t)1U,
        columns));
  Eurydice_arr_31
  seed2 =
    libcrux_ml_dsa_sample_add_domain_separator(seed,
      libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(start_index + (size_t)2U,
        columns));
  Eurydice_arr_31
  seed3 =
    libcrux_ml_dsa_sample_add_domain_separator(seed,
      libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_xy(start_index + (size_t)3U,
        columns));
  libcrux_ml_dsa_hash_functions_portable_Shake128X4
  state =
    libcrux_ml_dsa_hash_functions_portable_init_absorb_11(Eurydice_array_to_slice_shared_e9(&seed0),
      Eurydice_array_to_slice_shared_e9(&seed1),
      Eurydice_array_to_slice_shared_e9(&seed2),
      Eurydice_array_to_slice_shared_e9(&seed3));
  libcrux_ml_dsa_hash_functions_portable_squeeze_first_five_blocks_11(&state,
    rand_stack0,
    rand_stack1,
    rand_stack2,
    rand_stack3);
  size_t sampled0 = (size_t)0U;
  size_t sampled1 = (size_t)0U;
  size_t sampled2 = (size_t)0U;
  size_t sampled3 = (size_t)0U;
  bool
  done0 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_4c(rand_stack0),
      &sampled0,
      tmp_stack.ptr);
  bool
  done1 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_4c(rand_stack1),
      &sampled1,
      &tmp_stack.ptr[1U]);
  bool
  done2 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_4c(rand_stack2),
      &sampled2,
      &tmp_stack.ptr[2U]);
  bool
  done3 =
    libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_4c(rand_stack3),
      &sampled3,
      &tmp_stack.ptr[3U]);
  while (true)
  {
    if (done0)
    {
      if (done1)
      {
        if (done2)
        {
          if (done3)
          {
            break;
          }
          else
          {
            Eurydice_arr_c5_x4
            randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(&state);
            if (!done0)
            {
              done0 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
                  &sampled0,
                  tmp_stack.ptr);
            }
            if (!done1)
            {
              done1 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
                  &sampled1,
                  &tmp_stack.ptr[1U]);
            }
            if (!done2)
            {
              done2 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
                  &sampled2,
                  &tmp_stack.ptr[2U]);
            }
            if (!done3)
            {
              done3 =
                libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
                  &sampled3,
                  &tmp_stack.ptr[3U]);
            }
          }
        }
        else
        {
          Eurydice_arr_c5_x4
          randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(&state);
          if (!done0)
          {
            done0 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
                &sampled0,
                tmp_stack.ptr);
          }
          if (!done1)
          {
            done1 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
                &sampled1,
                &tmp_stack.ptr[1U]);
          }
          if (!done2)
          {
            done2 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
                &sampled2,
                &tmp_stack.ptr[2U]);
          }
          if (!done3)
          {
            done3 =
              libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
                &sampled3,
                &tmp_stack.ptr[3U]);
          }
        }
      }
      else
      {
        Eurydice_arr_c5_x4
        randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(&state);
        if (!done0)
        {
          done0 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
              &sampled0,
              tmp_stack.ptr);
        }
        if (!done1)
        {
          done1 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
              &sampled1,
              &tmp_stack.ptr[1U]);
        }
        if (!done2)
        {
          done2 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
              &sampled2,
              &tmp_stack.ptr[2U]);
        }
        if (!done3)
        {
          done3 =
            libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
              &sampled3,
              &tmp_stack.ptr[3U]);
        }
      }
    }
    else
    {
      Eurydice_arr_c5_x4
      randomnesses = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_11(&state);
      if (!done0)
      {
        done0 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.fst),
            &sampled0,
            tmp_stack.ptr);
      }
      if (!done1)
      {
        done1 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.snd),
            &sampled1,
            &tmp_stack.ptr[1U]);
      }
      if (!done2)
      {
        done2 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.thd),
            &sampled2,
            &tmp_stack.ptr[2U]);
      }
      if (!done3)
      {
        done3 =
          libcrux_ml_dsa_sample_rejection_sample_less_than_field_modulus_37(Eurydice_array_to_slice_shared_2c(&randomnesses.f3),
            &sampled3,
            &tmp_stack.ptr[3U]);
      }
    }
  }
  for (size_t i = (size_t)0U; i < elements_requested; i++)
  {
    size_t k = i;
    libcrux_ml_dsa_polynomial_from_i32_array_ff_37(Eurydice_array_to_slice_shared_2c0(&tmp_stack.ptr[k]),
      &matrix.ptr[start_index + k]);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.samplex4.matrix_flat
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake128X4
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_samplex4_matrix_flat_63(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_44 matrix
)
{
  Eurydice_arr_d10 rand_stack0 = { { 0U } };
  Eurydice_arr_d10 rand_stack1 = { { 0U } };
  Eurydice_arr_d10 rand_stack2 = { { 0U } };
  Eurydice_arr_d10 rand_stack3 = { { 0U } };
  Eurydice_arr_93 tmp_stack = { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  for (size_t i = (size_t)0U; i < matrix.meta / (size_t)4U + (size_t)1U; i++)
  {
    size_t start_index = i;
    size_t start_index0 = start_index * (size_t)4U;
    if (start_index0 >= matrix.meta)
    {
      break;
    }
    size_t elements_requested;
    if (start_index0 + (size_t)4U <= matrix.meta)
    {
      elements_requested = (size_t)4U;
    }
    else
    {
      elements_requested = matrix.meta - start_index0;
    }
    libcrux_ml_dsa_sample_sample_up_to_four_ring_elements_flat_63(columns,
      seed,
      matrix,
      &rand_stack0,
      &rand_stack1,
      &rand_stack2,
      &rand_stack3,
      Eurydice_array_to_slice_mut_7e(&tmp_stack),
      start_index0,
      elements_requested);
  }
}

/**
This function found in impl {libcrux_ml_dsa::samplex4::X4Sampler for libcrux_ml_dsa::samplex4::portable::PortableSampler}
*/
/**
A monomorphic instance of libcrux_ml_dsa.samplex4.portable.matrix_flat_a8
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients
with const generics

*/
static inline void
libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(
  size_t columns,
  Eurydice_borrow_slice_u8 seed,
  Eurydice_dst_ref_mut_44 matrix
)
{
  libcrux_ml_dsa_samplex4_matrix_flat_63(columns, seed, matrix);
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256
with const generics
- OUTPUT_LENGTH= 64
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_c9(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_c7 *out
)
{
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_17(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_61
with const generics
- OUTPUT_LENGTH= 64
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_61_c9(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_c7 *out
)
{
  libcrux_ml_dsa_hash_functions_portable_shake256_c9(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.encoding.signing_key.generate_serialized
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake256
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_encoding_signing_key_generate_serialized_2e(
  libcrux_ml_dsa_constants_Eta eta,
  size_t error_ring_element_size,
  Eurydice_borrow_slice_u8 seed_matrix,
  Eurydice_borrow_slice_u8 seed_signing,
  Eurydice_borrow_slice_u8 verification_key,
  Eurydice_dst_ref_shared_44 s1_2,
  Eurydice_dst_ref_shared_44 t0,
  Eurydice_mut_borrow_slice_u8 signing_key_serialized
)
{
  size_t offset = (size_t)0U;
  Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(signing_key_serialized,
      (core_ops_range_Range_87{ offset, offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE })),
    seed_matrix,
    uint8_t);
  offset += LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE;
  Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(signing_key_serialized,
      (core_ops_range_Range_87{ offset, offset + LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE })),
    seed_signing,
    uint8_t);
  offset += LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE;
  Eurydice_arr_c7 verification_key_hash = { { 0U } };
  libcrux_ml_dsa_hash_functions_portable_shake256_61_c9(verification_key,
    &verification_key_hash);
  Eurydice_slice_copy(Eurydice_slice_subslice_mut_c8(signing_key_serialized,
      (
        core_ops_range_Range_87{
          offset,
          offset + LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH
        }
      )),
    Eurydice_array_to_slice_shared_17(&verification_key_hash),
    uint8_t);
  offset += LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH;
  for (size_t i = (size_t)0U; i < s1_2.meta; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_encoding_error_serialize_37(eta,
      &s1_2.ptr[i0],
      Eurydice_slice_subslice_mut_c8(signing_key_serialized,
        (core_ops_range_Range_87{ offset, offset + error_ring_element_size })));
    offset += error_ring_element_size;
  }
  for (size_t i = (size_t)0U; i < t0.meta; i++)
  {
    size_t _cloop_j = i;
    const Eurydice_arr_a3 *ring_element = &t0.ptr[_cloop_j];
    libcrux_ml_dsa_encoding_t0_serialize_37(ring_element,
      Eurydice_slice_subslice_mut_c8(signing_key_serialized,
        (
          core_ops_range_Range_87{
            offset,
            offset + LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE
          }
        )));
    offset += LIBCRUX_ML_DSA_CONSTANTS_RING_ELEMENT_OF_T0S_SIZE;
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.generate_key_pair
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_5a(
  Eurydice_arr_ec randomness,
  Eurydice_mut_borrow_slice_u8 signing_key,
  Eurydice_mut_borrow_slice_u8 verification_key
)
{
  Eurydice_arr_89 seed_expanded0 = { { 0U } };
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
  shake = libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake,
    Eurydice_array_to_slice_shared_01(&randomness));
  /* original Rust expression is not an lvalue in C */
  Eurydice_array_u8x2
  lvalue =
    {
      {
        (uint8_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        (uint8_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A
      }
    };
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(&shake,
    Eurydice_array_to_slice_shared_82(&lvalue));
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(&shake,
    Eurydice_array_to_slice_mut_78(&seed_expanded0));
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_78(&seed_expanded0),
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 seed_expanded = uu____0.snd;
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(seed_expanded,
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_ERROR_VECTORS_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_error_vectors = uu____1.fst;
  Eurydice_borrow_slice_u8 seed_for_signing = uu____1.snd;
  Eurydice_arr_47 s1_s2;
  Eurydice_arr_a3 repeat_expression0[11U];
  for (size_t i = (size_t)0U; i < (size_t)11U; i++)
  {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s1_s2.data, repeat_expression0, (size_t)11U * sizeof (Eurydice_arr_a3));
  libcrux_ml_dsa_samplex4_sample_s1_and_s2_29(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
    seed_for_error_vectors,
    Eurydice_array_to_slice_mut_20(&s1_s2));
  Eurydice_arr_dc1 t0;
  Eurydice_arr_a3 repeat_expression1[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++)
  {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t0.data, repeat_expression1, (size_t)6U * sizeof (Eurydice_arr_a3));
  Eurydice_arr_5a a_as_ntt;
  Eurydice_arr_a3 repeat_expression2[30U];
  for (size_t i = (size_t)0U; i < (size_t)30U; i++)
  {
    repeat_expression2[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(a_as_ntt.data, repeat_expression2, (size_t)30U * sizeof (Eurydice_arr_a3));
  libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
    seed_for_a,
    Eurydice_array_to_slice_mut_200(&a_as_ntt));
  Eurydice_arr_5d s1_ntt;
  Eurydice_arr_a3 repeat_expression3[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++)
  {
    repeat_expression3[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s1_ntt.data, repeat_expression3, (size_t)5U * sizeof (Eurydice_arr_a3));
  Eurydice_slice_copy(Eurydice_array_to_slice_mut_201(&s1_ntt),
    Eurydice_array_to_subslice_shared_25(&s1_s2,
      (core_ops_range_Range_87{ (size_t)0U, LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A })),
    Eurydice_arr_a3);
  for (size_t i = (size_t)0U; i < (size_t)5U; i++)
  {
    size_t i0 = i;
    libcrux_ml_dsa_ntt_ntt_37(&s1_ntt.data[i0]);
  }
  libcrux_ml_dsa_matrix_compute_as1_plus_s2_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
    LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
    Eurydice_array_to_slice_mut_200(&a_as_ntt),
    Eurydice_array_to_slice_shared_20(&s1_ntt),
    Eurydice_array_to_slice_shared_200(&s1_s2),
    Eurydice_array_to_slice_mut_202(&t0));
  Eurydice_arr_dc1 t1;
  Eurydice_arr_a3 repeat_expression[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++)
  {
    repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t1.data, repeat_expression, (size_t)6U * sizeof (Eurydice_arr_a3));
  libcrux_ml_dsa_arithmetic_power2round_vector_37(Eurydice_array_to_slice_mut_202(&t0),
    Eurydice_array_to_slice_mut_202(&t1));
  libcrux_ml_dsa_encoding_verification_key_generate_serialized_37(seed_for_a,
    Eurydice_array_to_slice_shared_201(&t1),
    verification_key);
  libcrux_ml_dsa_encoding_signing_key_generate_serialized_2e(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
    LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
    seed_for_a,
    seed_for_signing,
    (Eurydice_borrow_slice_u8{ verification_key.ptr, verification_key.meta }),
    Eurydice_array_to_slice_shared_200(&s1_s2),
    Eurydice_array_to_slice_shared_201(&t0),
    signing_key);
}

/**
 Generate key pair.
*/
static inline void
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_generate_key_pair(
  Eurydice_arr_ec randomness,
  Eurydice_arr_24 *signing_key,
  Eurydice_arr_29 *verification_key
)
{
  libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_generate_key_pair_5a(randomness,
    Eurydice_array_to_slice_mut_98(signing_key),
    Eurydice_array_to_slice_mut_37(verification_key));
}

/**
 This corresponds to line 6 in algorithm 7 in FIPS 204 (line 7 in algorithm
 8, resp.).

 If `domain_separation_context` is supplied, applies domain
 separation and length encoding to the context string,
 before appending the message (in the regular variant) or the
 pre-hash OID as well as the pre-hashed message digest. Otherwise,
 it is assumed that `message` already contains domain separation
 information.

 In FIPS 204 M' is the concatenation of the domain separated context, any
 potential pre-hash OID and the message (or the message pre-hash). We do not
 explicitely construct the concatenation in memory since it is of statically unknown
 length, but feed its components directly into the incremental XOF.

 Refer to line 10 of Algorithm 2 (and line 5 of Algorithm 3, resp.) in [FIPS
 204](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf#section.5)
 for details on the domain separation for regular ML-DSA. Line
 23 of Algorithm 4 (and line 18 of Algorithm 5,resp.) describe domain separation for the HashMl-DSA
 variant.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.derive_message_representative
with types libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(
  Eurydice_borrow_slice_u8 verification_key_hash,
  const core_option_Option_84 *domain_separation_context,
  Eurydice_borrow_slice_u8 message,
  Eurydice_arr_c7 *message_representative
)
{
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
  shake = libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake, verification_key_hash);
  if (domain_separation_context->tag == core_option_Some)
  {
    const
    libcrux_ml_dsa_pre_hash_DomainSeparationContext
    *domain_separation_context0 = &domain_separation_context->f0;
    libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *uu____0 = &shake;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_82
    lvalue0 =
      {
        {
          (uint8_t)core_option__core__option__Option_T__TraitClause_0___is_some(libcrux_ml_dsa_pre_hash_pre_hash_oid_88(domain_separation_context0),
            Eurydice_arr_c9,
            bool)
        }
      };
    libcrux_ml_dsa_hash_functions_portable_absorb_26(uu____0,
      Eurydice_array_to_slice_shared_79(&lvalue0));
    libcrux_sha3_generic_keccak_xof_KeccakXofState_8d *uu____1 = &shake;
    /* original Rust expression is not an lvalue in C */
    Eurydice_arr_82
    lvalue = { { (uint8_t)libcrux_ml_dsa_pre_hash_context_88(domain_separation_context0).meta } };
    libcrux_ml_dsa_hash_functions_portable_absorb_26(uu____1,
      Eurydice_array_to_slice_shared_79(&lvalue));
    libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake,
      libcrux_ml_dsa_pre_hash_context_88(domain_separation_context0));
    const
    core_option_Option_57
    *uu____2 = libcrux_ml_dsa_pre_hash_pre_hash_oid_88(domain_separation_context0);
    if (uu____2->tag == core_option_Some)
    {
      const Eurydice_arr_c9 *pre_hash_oid = &uu____2->f0;
      libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake,
        Eurydice_array_to_slice_shared_2f(pre_hash_oid));
    }
  }
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(&shake, message);
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(&shake,
    Eurydice_array_to_slice_mut_17(message_representative));
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256
with const generics
- OUTPUT_LENGTH= 576
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_5a(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_220 *out
)
{
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_8a(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_x4_9b
with const generics
- OUT_LEN= 576
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_5a(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_arr_220 *out0,
  Eurydice_arr_220 *out1,
  Eurydice_arr_220 *out2,
  Eurydice_arr_220 *out3
)
{
  libcrux_ml_dsa_hash_functions_portable_shake256_5a(input0, out0);
  libcrux_ml_dsa_hash_functions_portable_shake256_5a(input1, out1);
  libcrux_ml_dsa_hash_functions_portable_shake256_5a(input2, out2);
  libcrux_ml_dsa_hash_functions_portable_shake256_5a(input3, out3);
}

/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256
with const generics
- OUTPUT_LENGTH= 640
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_0e(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_20 *out
)
{
  libcrux_sha3_portable_shake256(Eurydice_array_to_slice_mut_4f(out), input);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::XofX4 for libcrux_ml_dsa::hash_functions::portable::Shake256X4}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_x4_9b
with const generics
- OUT_LEN= 640
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_0e(
  Eurydice_borrow_slice_u8 input0,
  Eurydice_borrow_slice_u8 input1,
  Eurydice_borrow_slice_u8 input2,
  Eurydice_borrow_slice_u8 input3,
  Eurydice_arr_20 *out0,
  Eurydice_arr_20 *out1,
  Eurydice_arr_20 *out2,
  Eurydice_arr_20 *out3
)
{
  libcrux_ml_dsa_hash_functions_portable_shake256_0e(input0, out0);
  libcrux_ml_dsa_hash_functions_portable_shake256_0e(input1, out1);
  libcrux_ml_dsa_hash_functions_portable_shake256_0e(input2, out2);
  libcrux_ml_dsa_hash_functions_portable_shake256_0e(input3, out3);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_61
with const generics
- OUTPUT_LENGTH= 640
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_61_0e(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_20 *out
)
{
  libcrux_ml_dsa_hash_functions_portable_shake256_0e(input, out);
}

/**
This function found in impl {libcrux_ml_dsa::hash_functions::shake256::DsaXof for libcrux_ml_dsa::hash_functions::portable::Shake256}
*/
/**
A monomorphic instance of libcrux_ml_dsa.hash_functions.portable.shake256_61
with const generics
- OUTPUT_LENGTH= 576
*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_hash_functions_portable_shake256_61_5a(
  Eurydice_borrow_slice_u8 input,
  Eurydice_arr_220 *out
)
{
  libcrux_ml_dsa_hash_functions_portable_shake256_5a(input, out);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_ring_element
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake256
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_mask_ring_element_2e(
  const Eurydice_arr_91 *seed,
  Eurydice_arr_a3 *result,
  size_t gamma1_exponent
)
{
  switch (gamma1_exponent)
  {
    case 17U:
      {
        break;
      }
    case 19U:
      {
        Eurydice_arr_20 out = { { 0U } };
        libcrux_ml_dsa_hash_functions_portable_shake256_61_0e(Eurydice_array_to_slice_shared_f1(seed),
          &out);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_4f(&out),
          result);
        return;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
        KRML_HOST_EXIT(255U);
      }
  }
  Eurydice_arr_220 out = { { 0U } };
  libcrux_ml_dsa_hash_functions_portable_shake256_61_5a(Eurydice_array_to_slice_shared_f1(seed),
    &out);
  libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
    Eurydice_array_to_slice_shared_8a(&out),
    result);
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_mask_vector
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_mask_vector_67(
  size_t dimension,
  size_t gamma1_exponent,
  const Eurydice_arr_c7 *seed,
  uint16_t *domain_separator,
  Eurydice_dst_ref_mut_44 mask
)
{
  Eurydice_arr_91
  seed0 =
    libcrux_ml_dsa_sample_add_error_domain_separator(Eurydice_array_to_slice_shared_17(seed),
      domain_separator[0U]);
  Eurydice_arr_91
  seed1 =
    libcrux_ml_dsa_sample_add_error_domain_separator(Eurydice_array_to_slice_shared_17(seed),
      (uint32_t)domain_separator[0U] + 1U);
  Eurydice_arr_91
  seed2 =
    libcrux_ml_dsa_sample_add_error_domain_separator(Eurydice_array_to_slice_shared_17(seed),
      (uint32_t)domain_separator[0U] + 2U);
  Eurydice_arr_91
  seed3 =
    libcrux_ml_dsa_sample_add_error_domain_separator(Eurydice_array_to_slice_shared_17(seed),
      (uint32_t)domain_separator[0U] + 3U);
  domain_separator[0U] = (uint32_t)domain_separator[0U] + 4U;
  switch (gamma1_exponent)
  {
    case 17U:
      {
        Eurydice_arr_220 out0 = { { 0U } };
        Eurydice_arr_220 out1 = { { 0U } };
        Eurydice_arr_220 out2 = { { 0U } };
        Eurydice_arr_220 out3 = { { 0U } };
        libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_5a(Eurydice_array_to_slice_shared_f1(&seed0),
          Eurydice_array_to_slice_shared_f1(&seed1),
          Eurydice_array_to_slice_shared_f1(&seed2),
          Eurydice_array_to_slice_shared_f1(&seed3),
          &out0,
          &out1,
          &out2,
          &out3);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_8a(&out0),
          mask.ptr);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_8a(&out1),
          &mask.ptr[1U]);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_8a(&out2),
          &mask.ptr[2U]);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_8a(&out3),
          &mask.ptr[3U]);
        break;
      }
    case 19U:
      {
        Eurydice_arr_20 out0 = { { 0U } };
        Eurydice_arr_20 out1 = { { 0U } };
        Eurydice_arr_20 out2 = { { 0U } };
        Eurydice_arr_20 out3 = { { 0U } };
        libcrux_ml_dsa_hash_functions_portable_shake256_x4_9b_0e(Eurydice_array_to_slice_shared_f1(&seed0),
          Eurydice_array_to_slice_shared_f1(&seed1),
          Eurydice_array_to_slice_shared_f1(&seed2),
          Eurydice_array_to_slice_shared_f1(&seed3),
          &out0,
          &out1,
          &out2,
          &out3);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_4f(&out0),
          mask.ptr);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_4f(&out1),
          &mask.ptr[1U]);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_4f(&out2),
          &mask.ptr[2U]);
        libcrux_ml_dsa_encoding_gamma1_deserialize_37(gamma1_exponent,
          Eurydice_array_to_slice_shared_4f(&out3),
          &mask.ptr[3U]);
        break;
      }
    default:
      {
        KRML_HOST_EPRINTF("KaRaMeL abort at %s:%d\n%s\n", __FILE__, __LINE__, "panic!");
        KRML_HOST_EXIT(255U);
      }
  }
  for (size_t i = (size_t)4U; i < dimension; i++)
  {
    size_t i0 = i;
    Eurydice_arr_91
    seed4 =
      libcrux_ml_dsa_sample_add_error_domain_separator(Eurydice_array_to_slice_shared_17(seed),
        domain_separator[0U]);
    domain_separator[0U] = (uint32_t)domain_separator[0U] + 1U;
    libcrux_ml_dsa_sample_sample_mask_ring_element_2e(&seed4, &mask.ptr[i0], gamma1_exponent);
  }
}

/**
A monomorphic instance of libcrux_ml_dsa.sample.sample_challenge_ring_element
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_hash_functions_portable_Shake256
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_sample_sample_challenge_ring_element_2e(
  Eurydice_borrow_slice_u8 seed,
  size_t number_of_ones,
  Eurydice_arr_a3 *re
)
{
  Eurydice_arr_7c state = libcrux_ml_dsa_hash_functions_portable_init_absorb_final_61(seed);
  Eurydice_arr_ff
  randomness0 = libcrux_ml_dsa_hash_functions_portable_squeeze_first_block_61(&state);
  Eurydice_array_u8x8 arr;
  memcpy(arr.data,
    Eurydice_array_to_subslice_shared_d40(&randomness0,
      (core_ops_range_Range_87{ (size_t)0U, (size_t)8U })).ptr,
    (size_t)8U * sizeof (uint8_t));
  uint64_t
  signs =
    core_num__u64__from_le_bytes(core_result_unwrap_26_e0(core_result_Result_8e_s(core_result_Ok,
          &core_result_Result_8e_s::U::case_Ok,
          arr)));
  Eurydice_arr_6c result = { { 0U } };
  size_t out_index = (size_t)256U - number_of_ones;
  bool
  done =
    libcrux_ml_dsa_sample_inside_out_shuffle(Eurydice_array_to_subslice_from_shared_5f(&randomness0,
        (size_t)8U),
      &out_index,
      &signs,
      &result);
  while (true)
  {
    if (done)
    {
      break;
    }
    else
    {
      Eurydice_arr_ff
      randomness = libcrux_ml_dsa_hash_functions_portable_squeeze_next_block_61(&state);
      done =
        libcrux_ml_dsa_sample_inside_out_shuffle(Eurydice_array_to_slice_shared_58(&randomness),
          &out_index,
          &signs,
          &result);
    }
  }
  libcrux_ml_dsa_polynomial_from_i32_array_ff_37(Eurydice_array_to_slice_shared_af(&result), re);
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_internal
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_5a(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
)
{
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(signing_key,
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 remaining_serialized0 = uu____0.snd;
  Eurydice_borrow_slice_u8_x2
  uu____1 =
    Eurydice_slice_split_at(remaining_serialized0,
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_SIGNING_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_signing = uu____1.fst;
  Eurydice_borrow_slice_u8 remaining_serialized1 = uu____1.snd;
  Eurydice_borrow_slice_u8_x2
  uu____2 =
    Eurydice_slice_split_at(remaining_serialized1,
      LIBCRUX_ML_DSA_CONSTANTS_BYTES_FOR_VERIFICATION_KEY_HASH,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 verification_key_hash = uu____2.fst;
  Eurydice_borrow_slice_u8 remaining_serialized2 = uu____2.snd;
  Eurydice_borrow_slice_u8_x2
  uu____3 =
    Eurydice_slice_split_at(remaining_serialized2,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE *
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 s1_serialized = uu____3.fst;
  Eurydice_borrow_slice_u8 remaining_serialized = uu____3.snd;
  Eurydice_borrow_slice_u8_x2
  uu____4 =
    Eurydice_slice_split_at(remaining_serialized,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE *
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 s2_serialized = uu____4.fst;
  Eurydice_borrow_slice_u8 t0_serialized = uu____4.snd;
  Eurydice_arr_5d s1_as_ntt;
  Eurydice_arr_a3 repeat_expression0[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++)
  {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s1_as_ntt.data, repeat_expression0, (size_t)5U * sizeof (Eurydice_arr_a3));
  Eurydice_arr_dc1 s2_as_ntt;
  Eurydice_arr_a3 repeat_expression1[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++)
  {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(s2_as_ntt.data, repeat_expression1, (size_t)6U * sizeof (Eurydice_arr_a3));
  Eurydice_arr_dc1 t0_as_ntt;
  Eurydice_arr_a3 repeat_expression2[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++)
  {
    repeat_expression2[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t0_as_ntt.data, repeat_expression2, (size_t)6U * sizeof (Eurydice_arr_a3));
  libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
    LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
    s1_serialized,
    Eurydice_array_to_slice_mut_201(&s1_as_ntt));
  libcrux_ml_dsa_encoding_error_deserialize_to_vector_then_ntt_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ETA,
    LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_ERROR_RING_ELEMENT_SIZE,
    s2_serialized,
    Eurydice_array_to_slice_mut_202(&s2_as_ntt));
  libcrux_ml_dsa_encoding_t0_deserialize_to_vector_then_ntt_37(t0_serialized,
    Eurydice_array_to_slice_mut_202(&t0_as_ntt));
  Eurydice_arr_5a matrix;
  Eurydice_arr_a3 repeat_expression3[30U];
  for (size_t i = (size_t)0U; i < (size_t)30U; i++)
  {
    repeat_expression3[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(matrix.data, repeat_expression3, (size_t)30U * sizeof (Eurydice_arr_a3));
  libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
    seed_for_a,
    Eurydice_array_to_slice_mut_200(&matrix));
  Eurydice_arr_c7 message_representative = { { 0U } };
  libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(verification_key_hash,
    &domain_separation_context,
    message,
    &message_representative);
  Eurydice_arr_c7 mask_seed = { { 0U } };
  libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
  shake0 = libcrux_ml_dsa_hash_functions_portable_init_26();
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake0, seed_for_signing);
  libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake0,
    Eurydice_array_to_slice_shared_01(&randomness));
  libcrux_ml_dsa_hash_functions_portable_absorb_final_26(&shake0,
    Eurydice_array_to_slice_shared_17(&message_representative));
  libcrux_ml_dsa_hash_functions_portable_squeeze_26(&shake0,
    Eurydice_array_to_slice_mut_17(&mask_seed));
  uint16_t domain_separator_for_mask = 0U;
  size_t attempt = (size_t)0U;
  core_option_Option_81 commitment_hash0 = { core_option_None };
  core_option_Option_1e signer_response0 = { core_option_None };
  core_option_Option_05 hint0 = { core_option_None };
  while (attempt < LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN)
  {
    attempt++;
    Eurydice_arr_5d mask;
    Eurydice_arr_a3 repeat_expression4[5U];
    for (size_t i = (size_t)0U; i < (size_t)5U; i++)
    {
      repeat_expression4[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(mask.data, repeat_expression4, (size_t)5U * sizeof (Eurydice_arr_a3));
    Eurydice_arr_dc1 w0;
    Eurydice_arr_a3 repeat_expression5[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++)
    {
      repeat_expression5[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(w0.data, repeat_expression5, (size_t)6U * sizeof (Eurydice_arr_a3));
    Eurydice_arr_dc1 commitment;
    Eurydice_arr_a3 repeat_expression6[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++)
    {
      repeat_expression6[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(commitment.data, repeat_expression6, (size_t)6U * sizeof (Eurydice_arr_a3));
    libcrux_ml_dsa_sample_sample_mask_vector_67(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
      &mask_seed,
      &domain_separator_for_mask,
      Eurydice_array_to_slice_mut_201(&mask));
    Eurydice_arr_dc1 a_x_mask;
    Eurydice_arr_a3 repeat_expression[6U];
    for (size_t i = (size_t)0U; i < (size_t)6U; i++)
    {
      repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
    }
    memcpy(a_x_mask.data, repeat_expression, (size_t)6U * sizeof (Eurydice_arr_a3));
    Eurydice_arr_5d
    mask_ntt =
      core_array__core__clone__Clone_for__T__N___clone((size_t)5U,
        &mask,
        Eurydice_arr_a3,
        Eurydice_arr_5d);
    for (size_t i = (size_t)0U; i < (size_t)5U; i++)
    {
      size_t i0 = i;
      libcrux_ml_dsa_ntt_ntt_37(&mask_ntt.data[i0]);
    }
    libcrux_ml_dsa_matrix_compute_matrix_x_mask_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      Eurydice_array_to_slice_shared_202(&matrix),
      Eurydice_array_to_slice_shared_20(&mask_ntt),
      Eurydice_array_to_slice_mut_202(&a_x_mask));
    libcrux_ml_dsa_arithmetic_decompose_vector_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
      Eurydice_array_to_slice_shared_201(&a_x_mask),
      Eurydice_array_to_slice_mut_202(&w0),
      Eurydice_array_to_slice_mut_202(&commitment));
    Eurydice_arr_65 commitment_hash_candidate = { { 0U } };
    Eurydice_arr_d2 commitment_serialized = { { 0U } };
    libcrux_ml_dsa_encoding_commitment_serialize_vector_37(LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE,
      Eurydice_array_to_slice_shared_201(&commitment),
      Eurydice_array_to_slice_mut_27(&commitment_serialized));
    libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
    shake = libcrux_ml_dsa_hash_functions_portable_init_26();
    libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake,
      Eurydice_array_to_slice_shared_17(&message_representative));
    libcrux_ml_dsa_hash_functions_portable_absorb_final_26(&shake,
      Eurydice_array_to_slice_shared_27(&commitment_serialized));
    libcrux_ml_dsa_hash_functions_portable_squeeze_26(&shake,
      Eurydice_array_to_slice_mut_9f(&commitment_hash_candidate));
    Eurydice_arr_a3 verifier_challenge = libcrux_ml_dsa_polynomial_zero_ff_37();
    libcrux_ml_dsa_sample_sample_challenge_ring_element_2e(Eurydice_array_to_slice_shared_9f(&commitment_hash_candidate),
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE,
      &verifier_challenge);
    libcrux_ml_dsa_ntt_ntt_37(&verifier_challenge);
    Eurydice_arr_5d
    challenge_times_s1 =
      core_array__core__clone__Clone_for__T__N___clone((size_t)5U,
        &s1_as_ntt,
        Eurydice_arr_a3,
        Eurydice_arr_5d);
    Eurydice_arr_dc1
    challenge_times_s2 =
      core_array__core__clone__Clone_for__T__N___clone((size_t)6U,
        &s2_as_ntt,
        Eurydice_arr_a3,
        Eurydice_arr_dc1);
    libcrux_ml_dsa_matrix_vector_times_ring_element_37(Eurydice_array_to_slice_mut_201(&challenge_times_s1),
      &verifier_challenge);
    libcrux_ml_dsa_matrix_vector_times_ring_element_37(Eurydice_array_to_slice_mut_202(&challenge_times_s2),
      &verifier_challenge);
    libcrux_ml_dsa_matrix_add_vectors_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      Eurydice_array_to_slice_mut_201(&mask),
      Eurydice_array_to_slice_shared_20(&challenge_times_s1));
    libcrux_ml_dsa_matrix_subtract_vectors_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      Eurydice_array_to_slice_mut_202(&w0),
      Eurydice_array_to_slice_shared_201(&challenge_times_s2));
    if
    (
      !libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(Eurydice_array_to_slice_shared_20(&mask),
        (int32_t)((uint32_t)1 << (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT) -
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)
    )
    {
      if
      (
        !libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(Eurydice_array_to_slice_shared_201(&w0),
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2 - LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)
      )
      {
        Eurydice_arr_dc1
        challenge_times_t0 =
          core_array__core__clone__Clone_for__T__N___clone((size_t)6U,
            &t0_as_ntt,
            Eurydice_arr_a3,
            Eurydice_arr_dc1);
        libcrux_ml_dsa_matrix_vector_times_ring_element_37(Eurydice_array_to_slice_mut_202(&challenge_times_t0),
          &verifier_challenge);
        if
        (
          !libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(Eurydice_array_to_slice_shared_201(&challenge_times_t0),
            LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2)
        )
        {
          libcrux_ml_dsa_matrix_add_vectors_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
            Eurydice_array_to_slice_mut_202(&w0),
            Eurydice_array_to_slice_shared_201(&challenge_times_t0));
          Eurydice_arr_5d0
          hint_candidate =
            { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
          size_t
          ones_in_hint =
            libcrux_ml_dsa_arithmetic_make_hint_37(Eurydice_array_to_slice_shared_201(&w0),
              Eurydice_array_to_slice_shared_201(&commitment),
              LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
              Eurydice_array_to_slice_mut_86(&hint_candidate));
          if (!(ones_in_hint > LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT))
          {
            attempt = LIBCRUX_ML_DSA_CONSTANTS_REJECTION_SAMPLE_BOUND_SIGN;
            commitment_hash0 =
              (core_option_Option_81{ core_option_Some, commitment_hash_candidate });
            signer_response0 = (core_option_Option_1e{ core_option_Some, mask });
            hint0 = (core_option_Option_05{ core_option_Some, hint_candidate });
          }
        }
      }
    }
  }
  core_result_Result_53 uu____5;
  if (commitment_hash0.tag == core_option_None)
  {
    uu____5 =
      (
        core_result_Result_53{
          core_result_Err,
          libcrux_ml_dsa_types_SigningError_RejectionSamplingError
        }
      );
  }
  else
  {
    Eurydice_arr_65 commitment_hash = commitment_hash0.f0;
    Eurydice_arr_65 commitment_hash1 = commitment_hash;
    if (signer_response0.tag == core_option_None)
    {
      uu____5 =
        (
          core_result_Result_53{
            core_result_Err,
            libcrux_ml_dsa_types_SigningError_RejectionSamplingError
          }
        );
    }
    else
    {
      Eurydice_arr_5d signer_response = signer_response0.f0;
      Eurydice_arr_5d signer_response1 = signer_response;
      if (!(hint0.tag == core_option_None))
      {
        Eurydice_arr_5d0 hint = hint0.f0;
        Eurydice_arr_5d0 hint1 = hint;
        libcrux_ml_dsa_encoding_signature_serialize_37(Eurydice_array_to_slice_shared_9f(&commitment_hash1),
          Eurydice_array_to_slice_shared_20(&signer_response1),
          Eurydice_array_to_slice_shared_86(&hint1),
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE,
          LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,
          Eurydice_array_to_slice_mut_6b(signature));
        return (core_result_Result_53{ core_result_Ok });
      }
      uu____5 =
        (
          core_result_Result_53{
            core_result_Err,
            libcrux_ml_dsa_types_SigningError_RejectionSamplingError
          }
        );
    }
  }
  return uu____5;
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_mut
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_5a(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
)
{
  core_result_Result_a8
  uu____0 = libcrux_ml_dsa_pre_hash_new_88(context, (core_option_Option_57{ core_option_None }));
  if (!(uu____0.tag == core_result_Ok))
  {
    return
      (
        core_result_Result_53{
          core_result_Err,
          libcrux_ml_dsa_types_SigningError_ContextTooLongError
        }
      );
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context = dsc;
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_5a(signing_key,
      message,
      (core_option_Option_84{ core_option_Some, domain_separation_context }),
      randomness,
      signature);
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_portable_Shake256X4
with const generics

*/
static KRML_MUSTINLINE core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_5a(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
)
{
  Eurydice_arr_0c signature = libcrux_ml_dsa_types_zero_c5_5c();
  core_result_Result_53
  uu____0 =
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_5a(signing_key,
      message,
      context,
      randomness,
      &signature);
  core_result_Result_8c uu____1;
  if (uu____0.tag == core_result_Ok)
  {
    uu____1 =
      core_result_Result_8c_s(core_result_Ok,
        &core_result_Result_8c_s::U::case_Ok,
        signature);
  }
  else
  {
    libcrux_ml_dsa_types_SigningError e = uu____0.f0;
    uu____1 = core_result_Result_8c_s(core_result_Err, &core_result_Result_8c_s::U::case_Err, e);
  }
  return uu____1;
}

/**
 Sign.
*/
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_5a(Eurydice_array_to_slice_shared_98(signing_key),
      message,
      context,
      randomness);
}

/**
 Sign.
*/
static inline core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign_mut(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_mut_5a(Eurydice_array_to_slice_shared_98(signing_key),
      message,
      context,
      randomness,
      signature);
}

/**
This function found in impl {libcrux_ml_dsa::pre_hash::PreHash for libcrux_ml_dsa::pre_hash::SHAKE128_PH}
*/
/**
A monomorphic instance of libcrux_ml_dsa.pre_hash.hash_30
with types libcrux_ml_dsa_hash_functions_portable_Shake128
with const generics

*/
static KRML_MUSTINLINE void
libcrux_ml_dsa_pre_hash_hash_30_83(
  Eurydice_borrow_slice_u8 message,
  Eurydice_mut_borrow_slice_u8 output
)
{
  libcrux_ml_dsa_hash_functions_portable_shake128_7b(message, output);
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed_mut
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_portable_Shake256X4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
static KRML_MUSTINLINE core_result_Result_53
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_3f(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness,
  Eurydice_arr_0c *signature
)
{
  if (!(context.meta > LIBCRUX_ML_DSA_CONSTANTS_CONTEXT_MAX_LEN))
  {
    libcrux_ml_dsa_pre_hash_hash_30_83(message, pre_hash_buffer);
    core_result_Result_a8
    uu____0 =
      libcrux_ml_dsa_pre_hash_new_88(context,
        (core_option_Option_57{ core_option_Some, libcrux_ml_dsa_pre_hash_oid_30() }));
    if (!(uu____0.tag == core_result_Ok))
    {
      return
        (
          core_result_Result_53{
            core_result_Err,
            libcrux_ml_dsa_types_SigningError_ContextTooLongError
          }
        );
    }
    libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
    libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context = dsc;
    return
      libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_internal_5a(signing_key,
        (Eurydice_borrow_slice_u8{ pre_hash_buffer.ptr, pre_hash_buffer.meta }),
        (core_option_Option_84{ core_option_Some, domain_separation_context }),
        randomness,
        signature);
  }
  return
    (
      core_result_Result_53{
        core_result_Err,
        libcrux_ml_dsa_types_SigningError_ContextTooLongError
      }
    );
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.sign_pre_hashed
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_hash_functions_portable_Shake256X4, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
static KRML_MUSTINLINE core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_3f(
  Eurydice_borrow_slice_u8 signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
)
{
  Eurydice_arr_0c signature = libcrux_ml_dsa_types_zero_c5_5c();
  core_result_Result_53
  uu____0 =
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_mut_3f(signing_key,
      message,
      context,
      pre_hash_buffer,
      randomness,
      &signature);
  core_result_Result_8c uu____1;
  if (uu____0.tag == core_result_Ok)
  {
    uu____1 =
      core_result_Result_8c_s(core_result_Ok,
        &core_result_Result_8c_s::U::case_Ok,
        signature);
  }
  else
  {
    libcrux_ml_dsa_types_SigningError e = uu____0.f0;
    uu____1 = core_result_Result_8c_s(core_result_Err, &core_result_Result_8c_s::U::case_Err, e);
  }
  return uu____1;
}

/**
 Sign (pre-hashed).
*/
static inline core_result_Result_8c
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_sign_pre_hashed_shake128(
  const Eurydice_arr_24 *signing_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  Eurydice_arr_ec randomness
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_sign_pre_hashed_3f(Eurydice_array_to_slice_shared_98(signing_key),
      message,
      context,
      pre_hash_buffer,
      randomness);
}

/**
 The internal verification API.

 If no `domain_separation_context` is supplied, it is assumed that
 `message` already contains the domain separation.
*/
/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_internal
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_5a(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  core_option_Option_84 domain_separation_context,
  const Eurydice_arr_0c *signature_serialized
)
{
  Eurydice_borrow_slice_u8_x2
  uu____0 =
    Eurydice_slice_split_at(Eurydice_array_to_slice_shared_37(verification_key),
      LIBCRUX_ML_DSA_CONSTANTS_SEED_FOR_A_SIZE,
      uint8_t,
      Eurydice_borrow_slice_u8_x2);
  Eurydice_borrow_slice_u8 seed_for_a = uu____0.fst;
  Eurydice_borrow_slice_u8 t1_serialized = uu____0.snd;
  Eurydice_arr_dc1 t1;
  Eurydice_arr_a3 repeat_expression0[6U];
  for (size_t i = (size_t)0U; i < (size_t)6U; i++)
  {
    repeat_expression0[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(t1.data, repeat_expression0, (size_t)6U * sizeof (Eurydice_arr_a3));
  libcrux_ml_dsa_encoding_verification_key_deserialize_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
    LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_VERIFICATION_KEY_SIZE,
    t1_serialized,
    Eurydice_array_to_slice_mut_202(&t1));
  Eurydice_arr_65 deserialized_commitment_hash = { { 0U } };
  Eurydice_arr_5d deserialized_signer_response;
  Eurydice_arr_a3 repeat_expression1[5U];
  for (size_t i = (size_t)0U; i < (size_t)5U; i++)
  {
    repeat_expression1[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
  }
  memcpy(deserialized_signer_response.data,
    repeat_expression1,
    (size_t)5U * sizeof (Eurydice_arr_a3));
  Eurydice_arr_5d0
  deserialized_hint =
    { { { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } }, { { 0U } } } };
  core_result_Result_41
  uu____1 =
    libcrux_ml_dsa_encoding_signature_deserialize_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COMMITMENT_HASH_SIZE,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_GAMMA1_RING_ELEMENT_SIZE,
      LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_MAX_ONES_IN_HINT,
      LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_SIGNATURE_SIZE,
      Eurydice_array_to_slice_shared_6b(signature_serialized),
      Eurydice_array_to_slice_mut_9f(&deserialized_commitment_hash),
      Eurydice_array_to_slice_mut_201(&deserialized_signer_response),
      Eurydice_array_to_slice_mut_86(&deserialized_hint));
  core_result_Result_41 uu____2;
  if (uu____1.tag == core_result_Ok)
  {
    if
    (
      libcrux_ml_dsa_arithmetic_vector_infinity_norm_exceeds_37(Eurydice_array_to_slice_shared_20(&deserialized_signer_response),
        (int32_t)((uint32_t)1 << (uint32_t)LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA1_EXPONENT) -
          LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_BETA)
    )
    {
      uu____2 =
        (
          core_result_Result_41{
            core_result_Err,
            libcrux_ml_dsa_types_VerificationError_SignerResponseExceedsBoundError
          }
        );
    }
    else
    {
      Eurydice_arr_5a matrix;
      Eurydice_arr_a3 repeat_expression[30U];
      for (size_t i = (size_t)0U; i < (size_t)30U; i++)
      {
        repeat_expression[i] = libcrux_ml_dsa_polynomial_zero_ff_37();
      }
      memcpy(matrix.data, repeat_expression, (size_t)30U * sizeof (Eurydice_arr_a3));
      libcrux_ml_dsa_samplex4_portable_matrix_flat_a8_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        seed_for_a,
        Eurydice_array_to_slice_mut_200(&matrix));
      Eurydice_arr_c7 verification_key_hash = { { 0U } };
      libcrux_ml_dsa_hash_functions_portable_shake256_61_c9(Eurydice_array_to_slice_shared_37(verification_key),
        &verification_key_hash);
      Eurydice_arr_c7 message_representative = { { 0U } };
      libcrux_ml_dsa_ml_dsa_generic_derive_message_representative_43(Eurydice_array_to_slice_shared_17(&verification_key_hash),
        &domain_separation_context,
        message,
        &message_representative);
      Eurydice_arr_a3 verifier_challenge = libcrux_ml_dsa_polynomial_zero_ff_37();
      libcrux_ml_dsa_sample_sample_challenge_ring_element_2e(Eurydice_array_to_slice_shared_9f(&deserialized_commitment_hash),
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ONES_IN_VERIFIER_CHALLENGE,
        &verifier_challenge);
      libcrux_ml_dsa_ntt_ntt_37(&verifier_challenge);
      for (size_t i = (size_t)0U; i < (size_t)5U; i++)
      {
        size_t i0 = i;
        libcrux_ml_dsa_ntt_ntt_37(&deserialized_signer_response.data[i0]);
      }
      libcrux_ml_dsa_matrix_compute_w_approx_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_ROWS_IN_A,
        LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_COLUMNS_IN_A,
        Eurydice_array_to_slice_shared_202(&matrix),
        Eurydice_array_to_slice_shared_20(&deserialized_signer_response),
        &verifier_challenge,
        Eurydice_array_to_slice_mut_202(&t1));
      Eurydice_arr_65 recomputed_commitment_hash = { { 0U } };
      libcrux_ml_dsa_arithmetic_use_hint_37(LIBCRUX_ML_DSA_CONSTANTS_ML_DSA_65_GAMMA2,
        Eurydice_array_to_slice_shared_86(&deserialized_hint),
        Eurydice_array_to_slice_mut_202(&t1));
      Eurydice_arr_d2 commitment_serialized = { { 0U } };
      libcrux_ml_dsa_encoding_commitment_serialize_vector_37(LIBCRUX_ML_DSA_ML_DSA_GENERIC_ML_DSA_65_COMMITMENT_RING_ELEMENT_SIZE,
        Eurydice_array_to_slice_shared_201(&t1),
        Eurydice_array_to_slice_mut_27(&commitment_serialized));
      libcrux_sha3_generic_keccak_xof_KeccakXofState_8d
      shake = libcrux_ml_dsa_hash_functions_portable_init_26();
      libcrux_ml_dsa_hash_functions_portable_absorb_26(&shake,
        Eurydice_array_to_slice_shared_17(&message_representative));
      libcrux_ml_dsa_hash_functions_portable_absorb_final_26(&shake,
        Eurydice_array_to_slice_shared_27(&commitment_serialized));
      libcrux_ml_dsa_hash_functions_portable_squeeze_26(&shake,
        Eurydice_array_to_slice_mut_9f(&recomputed_commitment_hash));
      if
      (
        Eurydice_array_eq((size_t)48U,
          &deserialized_commitment_hash,
          &recomputed_commitment_hash,
          uint8_t)
      )
      {
        uu____2 = (core_result_Result_41{ core_result_Ok });
      }
      else
      {
        uu____2 =
          (
            core_result_Result_41{
              core_result_Err,
              libcrux_ml_dsa_types_VerificationError_CommitmentHashesDontMatchError
            }
          );
      }
    }
  }
  else
  {
    libcrux_ml_dsa_types_VerificationError e = uu____1.f0;
    uu____2 = (core_result_Result_41{ core_result_Err, e });
  }
  return uu____2;
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof
with const generics

*/
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_5a(
  const Eurydice_arr_29 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature_serialized
)
{
  core_result_Result_a8
  uu____0 = libcrux_ml_dsa_pre_hash_new_88(context, (core_option_Option_57{ core_option_None }));
  if (!(uu____0.tag == core_result_Ok))
  {
    return
      (
        core_result_Result_41{
          core_result_Err,
          libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError
        }
      );
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context = dsc;
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_5a(verification_key_serialized,
      message,
      (core_option_Option_84{ core_option_Some, domain_separation_context }),
      signature_serialized);
}

/**
 Verify.
*/
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_verify(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  const Eurydice_arr_0c *signature
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_5a(verification_key,
      message,
      context,
      signature);
}

/**
A monomorphic instance of libcrux_ml_dsa.ml_dsa_generic.ml_dsa_65.verify_pre_hashed
with types libcrux_ml_dsa_simd_portable_vector_type_Coefficients, libcrux_ml_dsa_samplex4_portable_PortableSampler, libcrux_ml_dsa_hash_functions_portable_Shake128, libcrux_ml_dsa_hash_functions_portable_Shake128X4, libcrux_ml_dsa_hash_functions_portable_Shake256, libcrux_ml_dsa_hash_functions_portable_Shake256Xof, libcrux_ml_dsa_pre_hash_SHAKE128_PH
with const generics

*/
static KRML_MUSTINLINE core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_3f(
  const Eurydice_arr_29 *verification_key_serialized,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_0c *signature_serialized
)
{
  libcrux_ml_dsa_pre_hash_hash_30_83(message, pre_hash_buffer);
  core_result_Result_a8
  uu____0 =
    libcrux_ml_dsa_pre_hash_new_88(context,
      (core_option_Option_57{ core_option_Some, libcrux_ml_dsa_pre_hash_oid_30() }));
  if (!(uu____0.tag == core_result_Ok))
  {
    return
      (
        core_result_Result_41{
          core_result_Err,
          libcrux_ml_dsa_types_VerificationError_VerificationContextTooLongError
        }
      );
  }
  libcrux_ml_dsa_pre_hash_DomainSeparationContext dsc = uu____0.val.case_Ok;
  libcrux_ml_dsa_pre_hash_DomainSeparationContext domain_separation_context = dsc;
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_internal_5a(verification_key_serialized,
      (Eurydice_borrow_slice_u8{ pre_hash_buffer.ptr, pre_hash_buffer.meta }),
      (core_option_Option_84{ core_option_Some, domain_separation_context }),
      signature_serialized);
}

/**
 Verify (pre-hashed with SHAKE-128).
*/
static inline core_result_Result_41
libcrux_ml_dsa_ml_dsa_generic_instantiations_portable_ml_dsa_65_verify_pre_hashed_shake128(
  const Eurydice_arr_29 *verification_key,
  Eurydice_borrow_slice_u8 message,
  Eurydice_borrow_slice_u8 context,
  Eurydice_mut_borrow_slice_u8 pre_hash_buffer,
  const Eurydice_arr_0c *signature
)
{
  return
    libcrux_ml_dsa_ml_dsa_generic_ml_dsa_65_verify_pre_hashed_3f(verification_key,
      message,
      context,
      pre_hash_buffer,
      signature);
}


#define libcrux_mldsa_portable_H_DEFINED
#endif /* libcrux_mldsa_portable_H */
