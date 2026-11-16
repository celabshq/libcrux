/*
 * SPDX-FileCopyrightText: 2026 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 */

/*
 * Optimization barrier for the constant-time operations in
 * `libcrux_ml_kem::constant_time_ops`.
 *
 * This file is written by hand, it is not generated. In Rust, the barrier is
 * `core::hint::black_box`, which Eurydice translates to a no-op. The
 * `libcrux_ct_barrier` bundle in the `extract.yaml` files therefore keeps
 * Eurydice from extracting `constant_time_ops::barrier` and includes this
 * header instead.
 *
 * The value makes a round-trip through a `volatile` local in a function that
 * is not inlined, so the compiler cannot reason about the result (for
 * example, that it is either 0 or 1) and turn the masking code that uses it
 * into a branch.
 *
 * Keep the copies of this file in all C extractions identical.
 */

#ifndef __libcrux_ct_barrier_H
#define __libcrux_ct_barrier_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "eurydice_glue.h"

static KRML_NOINLINE uint8_t
libcrux_ml_kem_constant_time_ops_barrier_ct_barrier_u8(uint8_t value) {
  volatile uint8_t opaque = value;
  return opaque;
}

#if defined(__cplusplus)
}
#endif

#define __libcrux_ct_barrier_H_DEFINED
#endif /* __libcrux_ct_barrier_H */
