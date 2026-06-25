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

#include "internal/libcrux_core.h"
#include "libcrux_mlkem768.h"
#include "libcrux_mlkem768_portable.h"
#include "libcrux_sha3_portable.h"

using namespace std;

#include "util.h"

TEST(MlKem768TestPortable, ConsistencyTest) {
  Eurydice_arr_060 r = {0};
  memset(r.data, 0x13, 64);
  auto key_pair = libcrux_ml_kem_mlkem768_portable_generate_key_pair(r);

  Eurydice_arr_60 r2 = {0};
  memset(r2.data, 0x15, 32);
  auto ctxt = libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, r2);

  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

  auto cmp = memcmp(ctxt.snd.data, sharedSecret2.data,
                    LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE);
  EXPECT_EQ(0, cmp);
}

#ifdef LIBCRUX_UNPACKED
TEST(MlKem768TestPortableUnpacked, ConsistencyTest) {
  uint8_t randomness[64] = {0};
  generate_random(randomness, 64);
  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair_unpacked(randomness);

  uint8_t randomness2[32] = {0};
  generate_random(randomness2, 32);
  auto ctxt = libcrux_ml_kem_mlkem768_portable_encapsulate_unpacked(
      &key_pair.public_key, randomness2);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
  libcrux_ml_kem_mlkem768_portable_decapsulate_unpacked(&key_pair, &ctxt.fst,
                                                        sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}
#endif  // #ifdef LIBCRUX_UNPACKED

TEST(Kyber768TestPortable, ModifiedCiphertextTest) {
  Eurydice_arr_060 randomness1 = {0};
  memset(randomness1.data, 0x13, 64);
  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness1);

  Eurydice_arr_60 r2 = {0};
  memset(r2.data, 0x15, 32);
  auto ctxt = libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, r2);

  modify_ciphertext(ctxt.fst.data,
                    LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

  EXPECT_NE(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

  uint8_t *implicitRejectionSharedSecret =
      compute_implicit_rejection_shared_secret(
          ctxt.fst.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE,
          key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE);

  EXPECT_EQ(0, memcmp(implicitRejectionSharedSecret, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  delete[] implicitRejectionSharedSecret;
}

TEST(Kyber768TestPortable, ModifiedSecretKeyTest) {
  Eurydice_arr_060 randomness1 = {0};
  memset(randomness1.data, 0x13, 64);
  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness1);

  Eurydice_arr_60 r2 = {0};
  memset(r2.data, 0x15, 32);
  auto ctxt = libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, r2);

  modify_secret_key(key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE,
                    false);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

  EXPECT_NE(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

  modify_secret_key(ctxt.snd.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE,
                    true);
  auto sharedSecret3 =
      libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

  uint8_t *implicitRejectionSharedSecret =
      compute_implicit_rejection_shared_secret(
          ctxt.fst.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE,
          key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE);
  EXPECT_EQ(0, memcmp(implicitRejectionSharedSecret, sharedSecret3.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  delete[] implicitRejectionSharedSecret;
}

TEST(MlKem768TestPortable, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/mlkem768_nistkats.json");
  Eurydice_arr_060 keygen_rand = {0};
  Eurydice_arr_60 encaps_rand = {0};

  for (auto kat : kats) {
    memcpy(keygen_rand.data, kat.key_generation_seed.data(), 64);
    memcpy(encaps_rand.data, kat.encapsulation_seed.data(), 32);

    auto key_pair =
        libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);

    auto pk_hash = libcrux_sha3_sha256(mk_borrow_slice_u8(
        key_pair.pk.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE));
    EXPECT_EQ(0,
              memcmp(pk_hash.data, kat.sha3_256_hash_of_public_key.data(), 32));

    auto sk_hash = libcrux_sha3_sha256(mk_borrow_slice_u8(
        key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE));
    EXPECT_EQ(0,
              memcmp(sk_hash.data, kat.sha3_256_hash_of_secret_key.data(), 32));

    auto ctxt =
        libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);
    auto ct_hash = libcrux_sha3_sha256(mk_borrow_slice_u8(
        ctxt.fst.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE));
    EXPECT_EQ(0,
              memcmp(ct_hash.data, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd.data, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    auto sharedSecret2 =
        libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_EQ(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}

#ifdef LIBCRUX_UNPACKED
TEST(MlKem768TestPortableUnpacked, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/mlkem768_nistkats.json");

  for (auto kat : kats) {
    auto key_pair = libcrux_ml_kem_mlkem768_portable_generate_key_pair_unpacked(
        kat.key_generation_seed.data());

    // We can't check the keys because we don't really have them.

    auto ctxt = libcrux_ml_kem_mlkem768_portable_encapsulate_unpacked(
        &key_pair.public_key, kat.encapsulation_seed.data());

    uint8_t ct_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(ctxt.fst.value,
                 LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE),
        ct_hash);
    EXPECT_EQ(0, memcmp(ct_hash, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE];
    libcrux_ml_kem_mlkem768_portable_decapsulate_unpacked(&key_pair, &ctxt.fst,
                                                          sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // #ifdef LIBCRUX_UNPACKED

#ifdef LIBCRUX_AARCH64
#include "libcrux_mlkem768_neon.h"

TEST(MlKem768TestNeon, ConsistencyTest) {
  uint8_t randomness[64];
  for (int i = 0; i < 64; i++) randomness[i] = 13;
  auto key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(randomness);
  auto ctxt =
      libcrux_ml_kem_mlkem768_neon_encapsulate(&key_pair.pk, randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE];
  libcrux_ml_kem_mlkem768_neon_decapsulate(&key_pair.sk, &ctxt.fst,
                                           sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(MlKem768TestNeon, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/mlkem768_nistkats.json");

  for (auto kat : kats) {
    auto key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(
        kat.key_generation_seed.data());

    uint8_t pk_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(key_pair.pk.value,
                 LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE),
        pk_hash);
    EXPECT_EQ(0, memcmp(pk_hash, kat.sha3_256_hash_of_public_key.data(), 32));

    uint8_t sk_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(key_pair.sk.value, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE),
        sk_hash);
    EXPECT_EQ(0, memcmp(sk_hash, kat.sha3_256_hash_of_secret_key.data(), 32));

    auto ctxt = libcrux_ml_kem_mlkem768_neon_encapsulate(
        &key_pair.pk, kat.encapsulation_seed.data());
    uint8_t ct_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(ctxt.fst.value,
                 LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE),
        ct_hash);
    EXPECT_EQ(0, memcmp(ct_hash, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE];
    libcrux_ml_kem_mlkem768_neon_decapsulate(&key_pair.sk, &ctxt.fst,
                                             sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // LIBCRUX_AARCH64
