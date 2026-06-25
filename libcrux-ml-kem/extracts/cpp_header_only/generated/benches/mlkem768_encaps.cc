/*
 *    Copyright 2022 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <benchmark/benchmark.h>

#include "libcrux_mlkem768_portable.h"

void generate_random(uint8_t *output, uint32_t output_len) {
  for (unsigned int i = 0; i < output_len; i++) output[i] = 13;
}

int main(int argc, char const *argv[]) {
  Eurydice_arr_060 keygen_rand;
  memset(keygen_rand.data, 0x13, 64);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);

  for (size_t i = 0; i < 100000; i++) {
    ctxt =
        libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);
  }

  return 0;
}
