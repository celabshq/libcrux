#pragma once

typedef vector<uint8_t> bytes;

template <typename T>
Eurydice_slice mk_slice(T *x, size_t len) {
  Eurydice_slice s = {0};
  s.ptr = (void *)x;
  s.len = len;
  return s;
}

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

// Not really random
void generate_random(uint8_t *output, uint32_t output_len) {
  for (size_t i = 0; i < output_len; i++) {
    output[i] = 13;
  }
}

vector<uint8_t> from_hex(const string &hex) {
  if (hex.length() % 2 == 1) {
    throw invalid_argument("Odd-length hex string");
  }

  int len = static_cast<int>(hex.length()) / 2;
  vector<uint8_t> out(len);
  for (int i = 0; i < len; i += 1) {
    string byte = hex.substr(2 * i, 2);
    out[i] = static_cast<uint8_t>(strtol(byte.c_str(), nullptr, 16));
  }

  return out;
}

string bytes_to_hex(const vector<uint8_t> &data) {
  stringstream hex(ios_base::out);
  hex.flags(ios::hex);
  for (const auto &byte : data) {
    hex << setw(2) << setfill('0') << int(byte);
  }
  return hex.str();
}

class KAT {
 public:
  bytes key_generation_seed;
  bytes sha3_256_hash_of_public_key;
  bytes sha3_256_hash_of_secret_key;
  bytes encapsulation_seed;
  bytes sha3_256_hash_of_ciphertext;
  bytes shared_secret;
};

vector<KAT> read_kats(string path) {
  ifstream kat_file(path);
  nlohmann::json kats_raw;
  kat_file >> kats_raw;

  vector<KAT> kats;

  // Read test group
  for (auto &kat_raw : kats_raw.items()) {
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

  if (modify_implicit_rejection_value == true) {
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
