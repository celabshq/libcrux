/*
 *    Copyright 2022 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <benchmark/benchmark.h>

#include "internal/libcrux_core.h"
#include "libcrux_mlkem768.h"
#include "libcrux_mlkem768_portable.h"

void generate_random(uint8_t *output, uint32_t output_len) {
  for (uint32_t i = 0; i < output_len; i++) output[i] = 13;
}

int main(int argc, char const *argv[]) {
  Eurydice_arr_06 randomness = {0};
  memset(randomness.data, 0x13, 64);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness);

  for (size_t i = 0; i < 100000; i++) {
    key_pair = libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness);
  }
  return 0;
}
