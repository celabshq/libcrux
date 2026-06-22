/*
 *    Copyright 2023 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <gtest/gtest.h>

#include <fstream>
#include <nlohmann/json.hpp>

#include "libcrux_sha3_avx2.h"
// #include "libcrux_mlkem1024.h"
#include "libcrux_core.h"
#include "libcrux_mlkem1024_avx2.h"

using namespace std;

#include "util.h"

TEST(MlKem1024TestAvx2, ConsistencyTest) {
  uint8_t randomness[64] = {0};
  for (int i = 0; i < 64; i++) {
    randomness[i] = 13;
  }
  auto key_pair = libcrux_ml_kem_mlkem1024_avx2_generate_key_pair(randomness);

  auto ctxt =
      libcrux_ml_kem_mlkem1024_avx2_encapsulate(&key_pair.pk, randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE];
  libcrux_ml_kem_mlkem1024_avx2_decapsulate(&key_pair.sk, &ctxt.fst,
                                            sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

#ifdef LIBCRUX_UNPACKED
TEST(MlKem1024TestAvx2Unpacked, ConsistencyTest) {
  uint8_t randomness[64] = {0};
  generate_random(randomness, 64);
  auto key_pair =
      libcrux_ml_kem_mlkem1024_avx2_generate_key_pair_unpacked(randomness);

  uint8_t randomness2[32] = {0};
  generate_random(randomness2, 32);
  auto ctxt = libcrux_ml_kem_mlkem1024_avx2_encapsulate_unpacked(
      &key_pair.public_key, randomness2);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
  libcrux_ml_kem_mlkem1024_avx2_decapsulate_unpacked(&key_pair, &ctxt.fst,
                                                     sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}
#endif  // #ifdef LIBCRUX_UNPACKED

TEST(MlKem1024TestAvx2, ModifiedCiphertextTest) {
  uint8_t randomness[64] = {0};
  generate_random(randomness, 64);
  auto key_pair = libcrux_ml_kem_mlkem1024_avx2_generate_key_pair(randomness);

  generate_random(randomness, 32);
  auto ctxt =
      libcrux_ml_kem_mlkem1024_avx2_encapsulate(&key_pair.pk, randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE];
  modify_ciphertext(ctxt.fst.value,
                    LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE);
  libcrux_ml_kem_mlkem1024_avx2_decapsulate(&key_pair.sk, &ctxt.fst,
                                            sharedSecret2);

  EXPECT_NE(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

  uint8_t *implicitRejectionSharedSecret =
      compute_implicit_rejection_shared_secret(
          ctxt.fst.value, LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE,
          key_pair.sk.value, LIBCRUX_ML_KEM_MLKEM1024_SECRET_KEY_SIZE);

  EXPECT_EQ(0, memcmp(implicitRejectionSharedSecret, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  delete[] implicitRejectionSharedSecret;
}

TEST(MlKem1024TestAvx2, ModifiedSecretKeyTest) {
  uint8_t randomness[64] = {0};
  generate_random(randomness, 64);
  auto key_pair = libcrux_ml_kem_mlkem1024_avx2_generate_key_pair(randomness);

  generate_random(randomness, 32);
  auto ctxt =
      libcrux_ml_kem_mlkem1024_avx2_encapsulate(&key_pair.pk, randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE];
  modify_secret_key(key_pair.sk.value, LIBCRUX_ML_KEM_MLKEM1024_SECRET_KEY_SIZE,
                    false);
  libcrux_ml_kem_mlkem1024_avx2_decapsulate(&key_pair.sk, &ctxt.fst,
                                            sharedSecret2);

  EXPECT_NE(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

  modify_secret_key(ctxt.snd, LIBCRUX_ML_KEM_MLKEM1024_SECRET_KEY_SIZE, true);
  libcrux_ml_kem_mlkem1024_avx2_decapsulate(&key_pair.sk, &ctxt.fst,
                                            sharedSecret2);

  uint8_t *implicitRejectionSharedSecret =
      compute_implicit_rejection_shared_secret(
          ctxt.fst.value, LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE,
          key_pair.sk.value, LIBCRUX_ML_KEM_MLKEM1024_SECRET_KEY_SIZE);
  EXPECT_EQ(0, memcmp(implicitRejectionSharedSecret, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  delete[] implicitRejectionSharedSecret;
}

TEST(MlKem1024TestAvx2, NISTKnownAnswerTest) {
  // XXX: This should be done in a avx2 way.
  auto kats = read_kats("tests/mlkem1024_nistkats.json");

  for (auto kat : kats) {
    auto key_pair = libcrux_ml_kem_mlkem1024_avx2_generate_key_pair(
        kat.key_generation_seed.data());

    uint8_t pk_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(key_pair.pk.value,
                 LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_PUBLIC_KEY_SIZE),
        pk_hash);
    EXPECT_EQ(0, memcmp(pk_hash, kat.sha3_256_hash_of_public_key.data(), 32));

    uint8_t sk_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(key_pair.sk.value, LIBCRUX_ML_KEM_MLKEM1024_SECRET_KEY_SIZE),
        sk_hash);
    EXPECT_EQ(0, memcmp(sk_hash, kat.sha3_256_hash_of_secret_key.data(), 32));

    auto ctxt = libcrux_ml_kem_mlkem1024_avx2_encapsulate(
        &key_pair.pk, kat.encapsulation_seed.data());
    uint8_t ct_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(ctxt.fst.value,
                 LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE),
        ct_hash);
    EXPECT_EQ(0, memcmp(ct_hash, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
    libcrux_ml_kem_mlkem1024_avx2_decapsulate(&key_pair.sk, &ctxt.fst,
                                              sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}

#ifdef LIBCRUX_UNPACKED
TEST(MlKem1024TestAvx2Unpacked, NISTKnownAnswerTest) {
  // XXX: This should be done in a avx2 way.
  auto kats = read_kats("tests/mlkem1024_nistkats.json");

  for (auto kat : kats) {
    auto key_pair = libcrux_ml_kem_mlkem1024_avx2_generate_key_pair_unpacked(
        kat.key_generation_seed.data());

    // We can't check the keys because we don't really have them.

    auto ctxt = libcrux_ml_kem_mlkem1024_avx2_encapsulate_unpacked(
        &key_pair.public_key, kat.encapsulation_seed.data());

    uint8_t ct_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(ctxt.fst.value,
                 LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE),
        ct_hash);
    EXPECT_EQ(0, memcmp(ct_hash, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
    libcrux_ml_kem_mlkem1024_avx2_decapsulate_unpacked(&key_pair, &ctxt.fst,
                                                       sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // #ifdef LIBCRUX_UNPACKED
