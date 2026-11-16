/*
 *    Copyright 2023 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include "util.h"

TEST(Kyber768TestPortable, ConsistencyTest) {
  uint8_t randomness[64] = {0};
  for (int i = 0; i < 64; i++) {
    randomness[i] = 13;
  }
  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(randomness);

  auto ctxt = libcrux_ml_kem_mlkem768_portable_kyber_encapsulate(&key_pair.pk,
                                                                 randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
  libcrux_ml_kem_mlkem768_portable_kyber_decapsulate(&key_pair.sk, &ctxt.fst,
                                                     sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber768TestPortable, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/nistkats_kyber_768.json");

  for (auto kat : kats) {
    auto key_pair = libcrux_ml_kem_mlkem768_portable_generate_key_pair(
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

    auto ctxt = libcrux_ml_kem_mlkem768_portable_kyber_encapsulate(
        &key_pair.pk, kat.encapsulation_seed.data());
    uint8_t ct_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(ctxt.fst.value,
                 LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE),
        ct_hash);
    EXPECT_EQ(0, memcmp(ct_hash, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
    libcrux_ml_kem_mlkem768_portable_kyber_decapsulate(&key_pair.sk, &ctxt.fst,
                                                       sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}

#ifdef LIBCRUX_X64
#include "libcrux_mlkem768_avx2.h"

TEST(Kyber768TestAvx2, ConsistencyTest) {
  uint8_t randomness[64] = {0};
  for (int i = 0; i < 64; i++) randomness[i] = 13;
  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(randomness);
  auto ctxt =
      libcrux_ml_kem_mlkem768_avx2_kyber_encapsulate(&key_pair.pk, randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
  libcrux_ml_kem_mlkem768_avx2_kyber_decapsulate(&key_pair.sk, &ctxt.fst,
                                                 sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber768TestAvx2, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/nistkats_kyber_768.json");

  for (auto kat : kats) {
    auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(
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

    auto ctxt = libcrux_ml_kem_mlkem768_avx2_kyber_encapsulate(
        &key_pair.pk, kat.encapsulation_seed.data());
    uint8_t ct_hash[32] = {0};
    libcrux_sha3_sha256(
        mk_slice(ctxt.fst.value,
                 LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE),
        ct_hash);
    EXPECT_EQ(0, memcmp(ct_hash, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
    libcrux_ml_kem_mlkem768_avx2_kyber_decapsulate(&key_pair.sk, &ctxt.fst,
                                                   sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // LIBCRUX_X64

#ifdef LIBCRUX_AARCH64
#include "libcrux_mlkem768_neon.h"

TEST(Kyber768TestNeon, ConsistencyTest) {
  uint8_t randomness[64] = {0};
  for (int i = 0; i < 64; i++) randomness[i] = 13;
  auto key_pair = libcrux_ml_kem_mlkem768_neon_generate_key_pair(randomness);
  auto ctxt =
      libcrux_ml_kem_mlkem768_neon_encapsulate(&key_pair.pk, randomness);

  uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
  libcrux_ml_kem_mlkem768_neon_decapsulate(&key_pair.sk, &ctxt.fst,
                                           sharedSecret2);

  EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber768TestNeon, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/nistkats_kyber_768.json");

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

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
    libcrux_ml_kem_mlkem768_neon_decapsulate(&key_pair.sk, &ctxt.fst,
                                             sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // LIBCRUX_AARCH64
