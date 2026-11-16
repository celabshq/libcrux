// Makes basic seed corpora for other fuzzers
//
// Will write to ./pubkey_corpus (for valid_fuzz and enc_fuzz) and
// to ./ciphertext_corpus (for dec_fuzz)

#include <err.h>
#include <errno.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "libcrux_mlkem768_portable.h"

#define NSEEDS 1000

void write_blob(const char *path, int n, const char *suffix, const void *p,
                size_t l) {
  char name[256];
  FILE *f;

  snprintf(name, sizeof(name), "%s/%06d.%s", path, n, suffix);
  if ((f = fopen(name, "wb+")) == NULL) {
    err(1, "fopen %s", name);
  }
  if (fwrite(p, l, 1, f) != 1) {
    err(1, "write %s", name);
  }
  fclose(f);
}

int main(void) {
  int i;
  uint8_t rnd[64];
  libcrux_ml_kem_mlkem768_MlKem768KeyPair kp;
  tuple_3c enc;

  if (mkdir("pubkey_corpus", 0777) != 0 && errno != EEXIST)
    err(1, "mkdir pubkey_corpus");
  if (mkdir("ciphertext_corpus", 0777) != 0 && errno != EEXIST)
    err(1, "mkdir ciphertext_corpus");

  for (i = 0; i < NSEEDS; i++) {
    if (i == 0) {
      memset(rnd, 0, sizeof(rnd));
    } else {
      (void)getentropy(rnd, sizeof(rnd));
    }
    kp = libcrux_ml_kem_mlkem768_portable_kyber_generate_key_pair(rnd);
    write_blob("pubkey_corpus", i, "pk", kp.pk.value, sizeof(kp.pk.value));

    if (i == 0) {
      memset(rnd, 0, sizeof(rnd));
    } else {
      (void)getentropy(rnd, sizeof(rnd));
    }
    enc = libcrux_ml_kem_mlkem768_portable_encapsulate(&kp.pk, rnd);
    write_blob("ciphertext_corpus", i, "ct", enc.fst.value,
               sizeof(enc.fst.value));
  }
  return 0;
}
