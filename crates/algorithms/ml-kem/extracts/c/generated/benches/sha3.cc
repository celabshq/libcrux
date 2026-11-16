/*
 *    Copyright 2022 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <benchmark/benchmark.h>

// TODO: FIXME: why is the macro definition in
// karamel/include/krml/internal/target.h not working?
// This is only broken in C++
#define KRML_HOST_EPRINTF(...) fprintf(stderr, __VA_ARGS__)

#include "internal/libcrux_core.h"
#include "libcrux_mlkem768.h"
#include "libcrux_sha3_portable.h"

Eurydice_borrow_slice_u8 mk_borrow_slice_u8(const uint8_t *x, size_t len) {
  Eurydice_borrow_slice_u8 s = {0};
  s.ptr = x;
  s.meta = len;
  return s;
}

Eurydice_mut_borrow_slice_u8 mk_mut_borrow_slice_u8(uint8_t *x, size_t len) {
  Eurydice_mut_borrow_slice_u8 s = {0};
  s.ptr = x;
  s.meta = len;
  return s;
}

void generate_random(uint8_t *output, uint32_t output_len) {
  for (uint32_t i = 0; i < output_len; i++) output[i] = 13;
}

static void sha3_256_1184(benchmark::State &state) {
  uint8_t digest[32] = {0};
  uint8_t input[1184] = {0};
  generate_random(input, 1184);

  libcrux_sha3_portable_sha256(mk_mut_borrow_slice_u8(digest, 32),
                               mk_borrow_slice_u8(input, 32));

  for (auto _ : state) {
    libcrux_sha3_portable_sha256(mk_mut_borrow_slice_u8(digest, 32),
                                 mk_borrow_slice_u8(input, 32));
  }
}

static void sha3_512_64(benchmark::State &state) {
  uint8_t digest[64] = {0};
  uint8_t input[64] = {0};
  generate_random(input, 64);

  libcrux_sha3_portable_sha512(mk_mut_borrow_slice_u8(digest, 64),
                               mk_borrow_slice_u8(input, 64));

  for (auto _ : state) {
    libcrux_sha3_portable_sha512(mk_mut_borrow_slice_u8(digest, 64),
                                 mk_borrow_slice_u8(input, 64));
  }
}

BENCHMARK(sha3_256_1184);
BENCHMARK(sha3_512_64);

BENCHMARK_MAIN();
