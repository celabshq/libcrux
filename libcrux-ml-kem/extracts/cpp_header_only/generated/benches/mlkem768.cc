/*
 *    Copyright 2022 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <benchmark/benchmark.h>

#include <memory>

#include "libcrux_mlkem768_portable.h"

void generate_random(uint8_t *output, uint32_t output_len) {
  for (unsigned int i = 0; i < output_len; i++) output[i] = 13;
}

static void kyber768_key_generation(benchmark::State &state) {
  Eurydice_arr_060 keygen_rand;
  memset(keygen_rand.data, 0x13, 64);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);

  for (auto _ : state) {
    key_pair = libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);
  }
}

static void kyber768_key_generation_unpacked(benchmark::State &state) {
  Eurydice_arr_060 keygen_rand;
  memset(keygen_rand.data, 0x13, 64);

  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_mlkem768_portable_unpacked_init_key_pair();
  libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(keygen_rand,
                                                                  &key_pair);

  for (auto _ : state) {
    libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(keygen_rand,
                                                                    &key_pair);
  }
}

static void kyber768_encapsulation(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);

  for (auto _ : state) {
    ctxt =
        libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);
  }
}

static void kyber768_encapsulation_unpacked(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_mlkem768_portable_unpacked_init_key_pair();
  libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(randomness,
                                                                  &key_pair);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt = libcrux_ml_kem_mlkem768_portable_unpacked_encapsulate(
      &key_pair.public_key, encaps_rand);

  for (auto _ : state) {
    ctxt = libcrux_ml_kem_mlkem768_portable_unpacked_encapsulate(
        &key_pair.public_key, encaps_rand);
  }
}

static void kyber768_decapsulation(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

  for (auto _ : state) {
    sharedSecret2 =
        libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);
  }
}

static void kyber768_decapsulation_unpacked(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  std::unique_ptr<
      libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked>
      key_pair(
          new libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked);
  libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(
      randomness, key_pair.get());

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt = libcrux_ml_kem_mlkem768_portable_unpacked_encapsulate(
      &key_pair->public_key, encaps_rand);
  auto sharedSecret2 = libcrux_ml_kem_mlkem768_portable_unpacked_decapsulate(
      key_pair.get(), &ctxt.fst);

  for (auto _ : state) {
    libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked key_pair =
        libcrux_ml_kem_mlkem768_portable_unpacked_init_key_pair();

    sharedSecret2 = libcrux_ml_kem_mlkem768_portable_unpacked_decapsulate(
        &key_pair, &ctxt.fst);
  }
}

BENCHMARK(kyber768_key_generation);
BENCHMARK(kyber768_key_generation_unpacked);
BENCHMARK(kyber768_encapsulation);
BENCHMARK(kyber768_encapsulation_unpacked);
BENCHMARK(kyber768_decapsulation);
BENCHMARK(kyber768_decapsulation_unpacked);

#ifdef LIBCRUX_AARCH64
#include "libcrux_mlkem768_neon.h"

static void kyber768_key_generation_neon(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(randomness);

  for (auto _ : state) {
    key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(randomness);
  }
}

static void kyber768_encapsulation_neon(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(randomness);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_neon_encapsulate(&key_pair.pk, encaps_rand);

  for (auto _ : state) {
    ctxt = libcrux_ml_kem_mlkem768_neon_encapsulate(&key_pair.pk, encaps_rand);
  }
}

static void kyber768_decapsulation_neon(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(randomness);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_neon_encapsulate(&key_pair.pk, encaps_rand);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_neon_unpacked_decapsulate(&key_pair, &ctxt.fst);

  for (auto _ : state) {
    sharedSecret2 =
        libcrux_ml_kem_mlkem768_neon_decapsulate(&key_pair.sk, &ctxt.fst);
  }
}

BENCHMARK(kyber768_key_generation_neon);
BENCHMARK(kyber768_encapsulation_neon);
BENCHMARK(kyber768_decapsulation_neon);
#endif

#ifdef LIBCRUX_X64
#include "libcrux_mlkem768_avx2.h"

static void kyber768_key_generation_avx2(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(randomness);

  for (auto _ : state) {
    key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(randomness);
  }
}

static void kyber768_key_generation_avx2_unpacked(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair();
  libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(randomness,
                                                              &key_pair);

  for (auto _ : state) {
    libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked key_pair =
        libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair();
    libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(randomness,
                                                                &key_pair);
  }
}

static void kyber768_encapsulation_avx2(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(randomness);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);

  for (auto _ : state) {
    ctxt = libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);
  }
}

static void kyber768_encapsulation_avx2_unpacked(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair();
  libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(randomness,
                                                              &key_pair);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt = libcrux_ml_kem_mlkem768_avx2_unpacked_encapsulate(
      &key_pair.public_key, encaps_rand);

  for (auto _ : state) {
    ctxt = libcrux_ml_kem_mlkem768_avx2_unpacked_encapsulate(
        &key_pair.public_key, encaps_rand);
  }
}

static void kyber768_decapsulation_avx2(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(randomness);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt =
      libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

  for (auto _ : state) {
    sharedSecret2 =
        libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);
  }
}

static void kyber768_decapsulation_avx2_unpacked(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair();
  libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(randomness,
                                                              &key_pair);

  Eurydice_arr_600 encaps_rand;
  memset(encaps_rand.data, 0x15, 32);

  auto ctxt = libcrux_ml_kem_mlkem768_avx2_unpacked_encapsulate(
      &key_pair.public_key, encaps_rand);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_avx2_unpacked_decapsulate(&key_pair, &ctxt.fst);

  for (auto _ : state) {
    sharedSecret2 =
        libcrux_ml_kem_mlkem768_avx2_unpacked_decapsulate(&key_pair, &ctxt.fst);
  }
}

BENCHMARK(kyber768_key_generation_avx2);
BENCHMARK(kyber768_key_generation_avx2_unpacked);
BENCHMARK(kyber768_encapsulation_avx2);
BENCHMARK(kyber768_encapsulation_avx2_unpacked);
BENCHMARK(kyber768_decapsulation_avx2);
BENCHMARK(kyber768_decapsulation_avx2_unpacked);
#endif

#ifdef LIBCRUX_SYMCRYPT
#include "inc/symcrypt.h"

static void symcrypt_kyber768_key_generation(benchmark::State &state) {
  Eurydice_arr_060 randomness;
  memset(randomness.data, 0x13, 64);

  auto pKey = SymCryptMlKemkeyAllocate(SymCryptMlKemParamsDraft203MlKem768);
  SymCryptMlKemkeyGenerate(pKey, 0);

  for (auto _ : state) {
    pKey = SymCryptMlKemkeyAllocate(SymCryptMlKemParamsDraft203MlKem768);
    SymCryptMlKemkeyGenerate(pKey, 0);
  }
}

static void symcrypt_kyber768_encapsulation(benchmark::State &state) {
  uint8_t randomness[64];
  generate_random(randomness, 64);

  auto pKey = SymCryptMlKemkeyAllocate(SymCryptMlKemParamsDraft203MlKem768);
  SymCryptMlKemkeyGenerate(pKey, 0);
  generate_random(randomness, 32);

  BYTE secret[32];
  BYTE cipher[1088];
  SymCryptMlKemEncapsulate(pKey, secret, 32, cipher, 1088);

  for (auto _ : state) {
    SymCryptMlKemEncapsulate(pKey, secret, 32, cipher, 1088);
  }
}

static void symcrypt_kyber768_decapsulation(benchmark::State &state) {
  uint8_t randomness[64];
  generate_random(randomness, 64);

  auto pKey = SymCryptMlKemkeyAllocate(SymCryptMlKemParamsDraft203MlKem768);
  SymCryptMlKemkeyGenerate(pKey, 0);

  generate_random(randomness, 32);
  BYTE secret[32];
  BYTE cipher[1088];
  SymCryptMlKemEncapsulate(pKey, secret, 32, cipher, 1088);

  BYTE sharedSecret2[32];

  for (auto _ : state) {
    SymCryptMlKemDecapsulate(pKey, cipher, 1088, sharedSecret2, 32);
  }
}

BENCHMARK(symcrypt_kyber768_key_generation);
BENCHMARK(symcrypt_kyber768_encapsulation);
BENCHMARK(symcrypt_kyber768_decapsulation);
#endif

BENCHMARK_MAIN();
