// Basic fuzz test for depcapsulate operation,

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "libcrux_mlkem768_portable.h"

extern "C" {

void privkeys(libcrux_ml_kem_types_MlKemPrivateKey_55 *zero_sk,
              libcrux_ml_kem_types_MlKemPrivateKey_55 *rnd_sk) {
  uint8_t rnd[64];
  memset(rnd, 0, sizeof(rnd));
  auto kp = libcrux_ml_kem_mlkem768_portable_kyber_generate_key_pair(rnd);
  *zero_sk = kp.sk;
  (void)getentropy(rnd, sizeof(rnd));
  kp = libcrux_ml_kem_mlkem768_portable_kyber_generate_key_pair(rnd);
  *rnd_sk = kp.sk;
}

int LLVMFuzzerTestOneInput(const uint8_t *input, size_t len) {
  static bool once;
  uint8_t ret[32];
  static libcrux_ml_kem_types_MlKemPrivateKey_55 zero_sk, rnd_sk;
  libcrux_ml_kem_mlkem768_MlKem768Ciphertext ct;

  if (!once) {
    privkeys(&zero_sk, &rnd_sk);
    once = true;
  }

  memset(&ct, 0, sizeof(ct));
  if (len > sizeof(ct.value)) {
    len = sizeof(ct.value);
  }
  memcpy(ct.value, input, len);

  libcrux_ml_kem_mlkem768_portable_decapsulate(&zero_sk, &ct, ret);
  libcrux_ml_kem_mlkem768_portable_decapsulate(&rnd_sk, &ct, ret);
  return 0;
}

}  // extern
