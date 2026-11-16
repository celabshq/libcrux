// Basic fuzz test for pubkey validation.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "libcrux_mlkem768_portable.h"

extern "C" {

int LLVMFuzzerTestOneInput(const uint8_t *input, size_t len) {
  uint8_t rnd[32];
  libcrux_ml_kem_mlkem768_MlKem768PublicKey pk;

  memset(rnd, 0, sizeof(rnd));
  memset(&pk, 0, sizeof(pk));
  if (len > sizeof(pk.value)) {
    len = sizeof(pk.value);
  }
  memcpy(pk.value, input, len);

  (void)libcrux_ml_kem_mlkem768_portable_validate_public_key(&pk);
  return 0;
}

}  // extern
