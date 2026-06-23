/*
 *    Copyright 2023 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <fstream>
#include <gtest/gtest.h>
#include <nlohmann/json.hpp>

#include "libcrux_sha3_portable.h"
#include "libcrux_mlkem512.h"
#include "libcrux_mlkem512_portable.h"

using namespace std;

typedef vector<uint8_t> bytes;

Eurydice_borrow_slice_u8 mk_slice_u8(uint8_t *x, size_t len)
{
    Eurydice_borrow_slice_u8 s;
    s.ptr = x;
    s.meta = len;
    return s;
}

// Not really random
void generate_random(uint8_t *output, uint32_t output_len)
{
    for (size_t i = 0; i < output_len; i++)
    {
        output[i] = 13;
    }
}

vector<uint8_t>
from_hex(const string &hex)
{
    if (hex.length() % 2 == 1)
    {
        throw invalid_argument("Odd-length hex string");
    }

    int len = static_cast<int>(hex.length()) / 2;
    vector<uint8_t> out(len);
    for (int i = 0; i < len; i += 1)
    {
        string byte = hex.substr(2 * i, 2);
        out[i] = static_cast<uint8_t>(strtol(byte.c_str(), nullptr, 16));
    }

    return out;
}

string
bytes_to_hex(const vector<uint8_t> &data)
{
    stringstream hex(ios_base::out);
    hex.flags(ios::hex);
    for (const auto &byte : data)
    {
        hex << setw(2) << setfill('0') << int(byte);
    }
    return hex.str();
}

class KAT
{
public:
    bytes key_generation_seed;
    bytes sha3_256_hash_of_public_key;
    bytes sha3_256_hash_of_secret_key;
    bytes encapsulation_seed;
    bytes sha3_256_hash_of_ciphertext;
    bytes shared_secret;
};

vector<KAT>
read_kats(string path)
{
    ifstream kat_file(path);
    nlohmann::json kats_raw;
    kat_file >> kats_raw;

    vector<KAT> kats;

    // Read test group
    for (auto &kat_raw : kats_raw.items())
    {
        auto kat_raw_value = kat_raw.value();

        kats.push_back(KAT{
            .key_generation_seed = from_hex(kat_raw_value["key_generation_seed"]),
            .sha3_256_hash_of_public_key =
                from_hex(kat_raw_value["sha3_256_hash_of_public_key"]),
            .sha3_256_hash_of_secret_key =
                from_hex(kat_raw_value["sha3_256_hash_of_secret_key"]),
            .encapsulation_seed = from_hex(kat_raw_value["encapsulation_seed"]),
            .sha3_256_hash_of_ciphertext =
                from_hex(kat_raw_value["sha3_256_hash_of_ciphertext"]),
            .shared_secret = from_hex(kat_raw_value["shared_secret"]),
        });
    }

    return kats;
}

void modify_ciphertext(uint8_t *ciphertext, size_t ciphertext_size)
{
    uint8_t randomness[3];
    generate_random(randomness, 3);

    uint8_t random_byte = randomness[0];
    if (random_byte == 0)
    {
        random_byte += 1;
    }

    uint16_t random_u16 = (randomness[2] << 8) | randomness[1];

    uint16_t random_position = random_u16 % ciphertext_size;

    ciphertext[random_position] ^= random_byte;
}

void modify_secret_key(uint8_t *secret_key,
                       size_t secret_key_size,
                       bool modify_implicit_rejection_value)
{
    uint8_t randomness[3];
    generate_random(randomness, 3);

    uint8_t random_byte = randomness[0];
    if (random_byte == 0)
    {
        random_byte += 1;
    }

    uint16_t random_u16 = (randomness[2] << 8) | randomness[1];

    uint16_t random_position = 0;

    if (modify_implicit_rejection_value == true)
    {
        random_position = (secret_key_size - 32) + (random_u16 % 32);
    }
    else
    {
        random_position = random_u16 % (secret_key_size - 32);
    }

    secret_key[random_position] ^= random_byte;
}

uint8_t *
compute_implicit_rejection_shared_secret(uint8_t *ciphertext,
                                         size_t ciphertext_size,
                                         uint8_t *secret_key,
                                         size_t secret_key_size)
{
    uint8_t *hashInput = new uint8_t[32 + ciphertext_size];
    uint8_t *sharedSecret = new uint8_t[32];
    Eurydice_mut_borrow_slice_u8 ss;
    ss.ptr = sharedSecret;
    ss.meta = 32;

    std::copy(secret_key + (secret_key_size - 32),
              secret_key + secret_key_size,
              hashInput);
    std::copy(ciphertext, ciphertext + ciphertext_size, hashInput + 32);

    libcrux_sha3_portable_shake256(ss, mk_slice_u8(hashInput, 32 + ciphertext_size));

    delete[] hashInput;
    return sharedSecret;
}

typedef Eurydice_arr_c7 libcrux_sha3_Sha3_512Digest;

TEST(MlKem512TestPortable, ConsistencyTest)
{
    libcrux_sha3_Sha3_512Digest randomness;
    for (int i = 0; i < 64; i++)
    {
        randomness.data[i] = 13;
    }
    auto key_pair = libcrux_ml_kem_mlkem512_portable_generate_key_pair(randomness);

    Eurydice_arr_ec randomness32;
    memcpy(randomness32.data, randomness.data, 32);
    auto ctxt = libcrux_ml_kem_mlkem512_portable_encapsulate(&key_pair.pk, randomness32);

    Eurydice_arr_ec sharedSecret2 = libcrux_ml_kem_mlkem512_portable_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_EQ(0,
              memcmp(ctxt.snd.data,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber512TestPortable, ModifiedCiphertextTest)
{
    libcrux_sha3_Sha3_512Digest randomness;
    generate_random(randomness.data, 64);
    auto key_pair = libcrux_ml_kem_mlkem512_portable_generate_key_pair(randomness);

    Eurydice_arr_ec randomness32;
    generate_random(randomness32.data, 32);
    auto ctxt = libcrux_ml_kem_mlkem512_portable_encapsulate(&key_pair.pk, randomness32);

    modify_ciphertext(ctxt.fst.data,
                      LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE);
    auto sharedSecret2 = libcrux_ml_kem_mlkem512_portable_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_NE(0,
              memcmp(ctxt.snd.data,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t *implicitRejectionSharedSecret =
        compute_implicit_rejection_shared_secret(
            ctxt.fst.data,
            LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE,
            key_pair.sk.data,
            LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE);

    EXPECT_EQ(0,
              memcmp(implicitRejectionSharedSecret,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
    delete[] implicitRejectionSharedSecret;
}

TEST(Kyber512TestPortable, ModifiedSecretKeyTest)
{
    libcrux_sha3_Sha3_512Digest randomness;
    generate_random(randomness.data, 64);
    auto key_pair = libcrux_ml_kem_mlkem512_portable_generate_key_pair(randomness);

    Eurydice_arr_ec randomness32;
    generate_random(randomness32.data, 32);
    auto ctxt = libcrux_ml_kem_mlkem512_portable_encapsulate(&key_pair.pk, randomness32);

    modify_secret_key(
        key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE, false);
    auto sharedSecret2 = libcrux_ml_kem_mlkem512_portable_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_NE(0,
              memcmp(ctxt.snd.data,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    modify_secret_key(
        ctxt.snd.data, LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE, true);
    sharedSecret2 = libcrux_ml_kem_mlkem512_portable_decapsulate(&key_pair.sk, &ctxt.fst);

    uint8_t *implicitRejectionSharedSecret =
        compute_implicit_rejection_shared_secret(
            ctxt.fst.data,
            LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE,
            key_pair.sk.data,
            LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE);
    EXPECT_EQ(0,
              memcmp(implicitRejectionSharedSecret,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
    delete[] implicitRejectionSharedSecret;
}

TEST(MlKem512TestPortable, NISTKnownAnswerTest)
{
    auto kats = read_kats("tests/mlkem512_nistkats.json");

    for (auto kat : kats)
    {
        libcrux_sha3_Sha3_512Digest randomness;
        memcpy(randomness.data, kat.key_generation_seed.data(), 64);
        auto key_pair =
            libcrux_ml_kem_mlkem512_portable_generate_key_pair(randomness);

        auto pk_hash =
          libcrux_sha3_sha256(
              mk_slice_u8(key_pair.pk.data,
                       LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_PUBLIC_KEY_SIZE));
        EXPECT_EQ(0, memcmp(pk_hash.data, kat.sha3_256_hash_of_public_key.data(), 32));

        auto sk_hash =
          libcrux_sha3_sha256(
              mk_slice_u8(key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE));
        EXPECT_EQ(0, memcmp(sk_hash.data, kat.sha3_256_hash_of_secret_key.data(), 32));

        Eurydice_arr_ec randomness32;
        memcpy(randomness32.data, kat.encapsulation_seed.data(), 32);
        auto ctxt = libcrux_ml_kem_mlkem512_portable_encapsulate(
            &key_pair.pk, randomness32);
        auto ct_hash =
          libcrux_sha3_sha256(
              mk_slice_u8(ctxt.fst.data,
                       LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE));
        EXPECT_EQ(0, memcmp(ct_hash.data, kat.sha3_256_hash_of_ciphertext.data(), 32));
        EXPECT_EQ(0,
                  memcmp(ctxt.snd.data,
                         kat.shared_secret.data(),
                         LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

        auto sharedSecret2 =
          libcrux_ml_kem_mlkem512_portable_decapsulate(&key_pair.sk, &ctxt.fst);

        EXPECT_EQ(0,
                  memcmp(ctxt.snd.data,
                         sharedSecret2.data,
                         LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
    }
}

#ifdef LIBCRUX_X64
#include "libcrux_mlkem512_avx2.h"

TEST(MlKem512TestAvx2, ConsistencyTest)
{
    libcrux_sha3_Sha3_512Digest randomness;
    for (int i = 0; i < 64; i++)
    {
        randomness.data[i] = 13;
    }
    auto key_pair = libcrux_ml_kem_mlkem512_avx2_generate_key_pair(randomness);
    Eurydice_arr_ec randomness32;
    memcpy(randomness32.data, randomness.data, 32);
    auto ctxt = libcrux_ml_kem_mlkem512_avx2_encapsulate(&key_pair.pk, randomness32);

    auto sharedSecret2 =
      libcrux_ml_kem_mlkem512_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_EQ(0,
              memcmp(ctxt.snd.data,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
}

TEST(Kyber512TestAvx2, ModifiedCiphertextTest)
{
    libcrux_sha3_Sha3_512Digest randomness;
    generate_random(randomness.data, 64);
    auto key_pair = libcrux_ml_kem_mlkem512_avx2_generate_key_pair(randomness);

    Eurydice_arr_ec randomness32;
    generate_random(randomness32.data, 32);
    auto ctxt = libcrux_ml_kem_mlkem512_avx2_encapsulate(&key_pair.pk, randomness32);

    modify_ciphertext(ctxt.fst.data,
                      LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE);
    auto sharedSecret2 =
      libcrux_ml_kem_mlkem512_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_NE(0,
              memcmp(ctxt.snd.data,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    uint8_t *implicitRejectionSharedSecret =
        compute_implicit_rejection_shared_secret(
            ctxt.fst.data,
            LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE,
            key_pair.sk.data,
            LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE);

    EXPECT_EQ(0,
              memcmp(implicitRejectionSharedSecret,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
    delete[] implicitRejectionSharedSecret;
}

TEST(Kyber512TestAvx2, ModifiedSecretKeyTest)
{
    libcrux_sha3_Sha3_512Digest randomness;
    generate_random(randomness.data, 64);
    auto key_pair = libcrux_ml_kem_mlkem512_avx2_generate_key_pair(randomness);

    Eurydice_arr_ec randomness32;
    generate_random(randomness32.data, 32);
    auto ctxt = libcrux_ml_kem_mlkem512_avx2_encapsulate(&key_pair.pk, randomness32);

    modify_secret_key(
        key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE, false);
    auto sharedSecret2 =
      libcrux_ml_kem_mlkem512_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

    EXPECT_NE(0,
              memcmp(ctxt.snd.data,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

    modify_secret_key(
        ctxt.snd.data, LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE, true);
    sharedSecret2 = libcrux_ml_kem_mlkem512_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

    uint8_t *implicitRejectionSharedSecret =
        compute_implicit_rejection_shared_secret(
            ctxt.fst.data,
            LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE,
            key_pair.sk.data,
            LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE);
    EXPECT_EQ(0,
              memcmp(implicitRejectionSharedSecret,
                     sharedSecret2.data,
                     LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
    delete[] implicitRejectionSharedSecret;
}

TEST(MlKem512TestAvx2, NISTKnownAnswerTest)
{
    auto kats = read_kats("tests/mlkem512_nistkats.json");

    for (auto kat : kats)
    {
        libcrux_sha3_Sha3_512Digest randomness;
        memcpy(randomness.data, kat.key_generation_seed.data(), 64);
        auto key_pair = libcrux_ml_kem_mlkem512_avx2_generate_key_pair(randomness);

        auto pk_hash =
          libcrux_sha3_sha256(
              mk_slice_u8(key_pair.pk.data,
                       LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_PUBLIC_KEY_SIZE));
        EXPECT_EQ(0, memcmp(pk_hash.data, kat.sha3_256_hash_of_public_key.data(), 32));

        auto sk_hash =
          libcrux_sha3_sha256(
              mk_slice_u8(key_pair.sk.data, LIBCRUX_ML_KEM_MLKEM512_SECRET_KEY_SIZE));
        EXPECT_EQ(0, memcmp(sk_hash.data, kat.sha3_256_hash_of_secret_key.data(), 32));

        Eurydice_arr_ec randomness32;
        memcpy(randomness32.data, kat.encapsulation_seed.data(), 32);
        auto ctxt = libcrux_ml_kem_mlkem512_avx2_encapsulate(
            &key_pair.pk, randomness32);
        auto ct_hash =
          libcrux_sha3_sha256(
              mk_slice_u8(ctxt.fst.data,
                       LIBCRUX_ML_KEM_MLKEM512_CPA_PKE_CIPHERTEXT_SIZE));
        EXPECT_EQ(0, memcmp(ct_hash.data, kat.sha3_256_hash_of_ciphertext.data(), 32));
        EXPECT_EQ(0,
                  memcmp(ctxt.snd.data,
                         kat.shared_secret.data(),
                         LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));

        auto sharedSecret2 =
          libcrux_ml_kem_mlkem512_avx2_decapsulate(&key_pair.sk, &ctxt.fst);

        EXPECT_EQ(0,
                  memcmp(ctxt.snd.data,
                         sharedSecret2.data,
                         LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE));
    }
}

#endif // LIBCRUX_X64
