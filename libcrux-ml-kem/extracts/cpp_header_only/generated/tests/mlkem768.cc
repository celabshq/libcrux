/*
 *    Copyright 2023 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include "util.h"

void modify_ciphertext(uint8_t *ciphertext, size_t ciphertext_size) {
  uint8_t randomness[3] = {0};
  generate_random(randomness, 3);

  uint8_t random_byte = randomness[0];
  if (random_byte == 0) {
    random_byte += 1;
  }

  uint16_t random_u16 = (randomness[2] << 8) | randomness[1];

  uint16_t random_position = random_u16 % ciphertext_size;

  ciphertext[random_position] ^= random_byte;
}

void modify_secret_key(uint8_t *secret_key, size_t secret_key_size,
                       bool modify_implicit_rejection_value) {
  uint8_t randomness[3] = {0};
  generate_random(randomness, 3);

  uint8_t random_byte = randomness[0];
  if (random_byte == 0) {
    random_byte += 1;
  }

  uint16_t random_u16 = (randomness[2] << 8) | randomness[1];

  uint16_t random_position = 0;

  if (modify_implicit_rejection_value) {
    random_position = (secret_key_size - 32) + (random_u16 % 32);
  } else {
    random_position = random_u16 % (secret_key_size - 32);
  }

  secret_key[random_position] ^= random_byte;
}

uint8_t *compute_implicit_rejection_shared_secret(uint8_t *ciphertext,
                                                  size_t ciphertext_size,
                                                  uint8_t *secret_key,
                                                  size_t secret_key_size) {
  uint8_t *hashInput = new uint8_t[32 + ciphertext_size];
  uint8_t *sharedSecret = new uint8_t[32];
  Eurydice_mut_borrow_slice_u8 ss = {0};
  ss.ptr = (uint8_t *)sharedSecret;
  ss.meta = 32;

  std::copy(secret_key + (secret_key_size - 32), secret_key + secret_key_size,
            hashInput);
  std::copy(ciphertext, ciphertext + ciphertext_size, hashInput + 32);

  libcrux_sha3_portable_shake256(
      ss, mk_borrow_slice_u8(hashInput, 32 + ciphertext_size));

  delete[] hashInput;
  return sharedSecret;
}

TEST(MlKem768TestPortable, ConsistencyTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);
  auto ctxt =
      libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);

  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_portable_decapsulate(&key_pair.sk, &ctxt.fst);

  EXPECT_EQ(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(MlKem768TestPortableUnpacked, ConsistencyTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  // We put this on the heap to avoid blowing the stack.
  libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked *key_pair =
      static_cast<
          libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked
              *>(malloc(sizeof(
          libcrux_ml_kem_mlkem768_portable_unpacked_MlKem768KeyPairUnpacked)));
  libcrux_ml_kem_mlkem768_portable_unpacked_generate_key_pair_mut(keygen_rand,
                                                                  key_pair);

  auto ctxt = libcrux_ml_kem_mlkem768_portable_unpacked_encapsulate(
      &key_pair->public_key, encaps_rand);

  auto sharedSecret2 = libcrux_ml_kem_mlkem768_portable_unpacked_decapsulate(
      key_pair, &ctxt.fst);

  EXPECT_EQ(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber768TestPortable, ModifiedCiphertextTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);
  auto ctxt =
      libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);

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
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  auto key_pair =
      libcrux_ml_kem_mlkem768_portable_generate_key_pair(keygen_rand);
  auto ctxt =
      libcrux_ml_kem_mlkem768_portable_encapsulate(&key_pair.pk, encaps_rand);

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
  Eurydice_arr_600 encaps_rand = {0};

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

#ifdef LIBCRUX_X64
#include "libcrux_mlkem768_avx2.h"

TEST(MlKem768TestAvx2, ConsistencyTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(keygen_rand);
  auto ctxt =
      libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);

  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

  EXPECT_EQ(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(MlKem768TestAvx2Unpacked, ConsistencyTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  libcrux_ml_kem_mlkem768_avx2_unpacked_MlKem768KeyPairUnpacked key_pair =
      libcrux_ml_kem_mlkem768_avx2_unpacked_init_key_pair();
  libcrux_ml_kem_mlkem768_avx2_unpacked_generate_key_pair_mut(keygen_rand,
                                                              &key_pair);

  auto ctxt = libcrux_ml_kem_mlkem768_avx2_unpacked_encapsulate(
      &key_pair.public_key, encaps_rand);

  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_avx2_unpacked_decapsulate(&key_pair, &ctxt.fst);

  EXPECT_EQ(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber768TestAvx2, ModifiedCiphertextTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(keygen_rand);
  auto ctxt =
      libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);

  modify_ciphertext(ctxt.fst.data,
                    LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

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

TEST(Kyber768TestAvx2, ModifiedSecretKeyTest) {
  Eurydice_arr_060 keygen_rand = {0};
  memset(keygen_rand.data, 0x13, 64);
  Eurydice_arr_600 encaps_rand = {0};
  memset(encaps_rand.data, 0x15, 32);

  auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(keygen_rand);
  auto ctxt =
      libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);

  modify_secret_key(key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE,
                    false);
  auto sharedSecret2 =
      libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

  EXPECT_NE(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

  modify_secret_key(ctxt.snd.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE,
                    true);
  auto sharedSecret3 =
      libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

  uint8_t *implicitRejectionSharedSecret =
      compute_implicit_rejection_shared_secret(
          ctxt.fst.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE,
          key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE);
  EXPECT_EQ(0, memcmp(implicitRejectionSharedSecret, sharedSecret3.data,
                      LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  delete[] implicitRejectionSharedSecret;
}

TEST(MlKem768TestAvx2, NISTKnownAnswerTest) {
  // XXX: This should be done in a portable way.
  auto kats = read_kats("tests/mlkem768_nistkats.json");
  Eurydice_arr_060 keygen_rand = {0};
  Eurydice_arr_600 encaps_rand = {0};

  for (auto kat : kats) {
    memcpy(keygen_rand.data, kat.key_generation_seed.data(), 64);
    memcpy(encaps_rand.data, kat.encapsulation_seed.data(), 32);

    auto key_pair = libcrux_ml_kem_mlkem768_avx2_generate_key_pair(keygen_rand);

    auto pk_hash = libcrux_sha3_sha256(mk_borrow_slice_u8(
        key_pair.pk.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE));
    EXPECT_EQ(0,
              memcmp(pk_hash.data, kat.sha3_256_hash_of_public_key.data(), 32));

    auto sk_hash = libcrux_sha3_sha256(mk_borrow_slice_u8(
        key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE));
    EXPECT_EQ(0,
              memcmp(sk_hash.data, kat.sha3_256_hash_of_secret_key.data(), 32));

    auto ctxt =
        libcrux_ml_kem_mlkem768_avx2_encapsulate(&key_pair.pk, encaps_rand);
    auto ct_hash = libcrux_sha3_sha256(mk_borrow_slice_u8(
        ctxt.fst.data, LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE));
    EXPECT_EQ(0,
              memcmp(ct_hash.data, kat.sha3_256_hash_of_ciphertext.data(), 32));
    EXPECT_EQ(0, memcmp(ctxt.snd.data, kat.shared_secret.data(),
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    auto sharedSecret2 =
        libcrux_ml_kem_mlkem768_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_EQ(0, memcmp(ctxt.snd.data, sharedSecret2.data,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // LIBCRUX_X64

#ifdef LIBCRUX_AARCH64
#include "libcrux_mlkem768_neon.h"

TEST(MlKem768TestNeon, ConsistencyTest) {
  uint8_t randomness[64] = {0};
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

    uint8_t sharedSecret2[LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE] = {0};
    libcrux_ml_kem_mlkem768_neon_decapsulate(&key_pair.sk, &ctxt.fst,
                                             sharedSecret2);

    EXPECT_EQ(0, memcmp(ctxt.snd, sharedSecret2,
                        LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
  }
}
#endif  // LIBCRUX_AARCH64
