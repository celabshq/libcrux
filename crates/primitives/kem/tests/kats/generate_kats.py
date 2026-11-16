#! /usr/bin/env python3

# This file is a modified version of:
# https://github.com/bwesterb/draft-schwabe-cfrg-kyber/blob/main/kyber_test.py

from kyber import *

import hashlib
import json

import Crypto
from Crypto.Cipher import AES


class NistDRBG:
    """NIST's DRBG used to generate NIST's Known Answer Tests (KATs),
    see PQCgenKAT.c."""

    def __init__(self, seed):
        self.key = b"\0" * 32
        self.v = 0
        assert len(seed) == 48
        self._update(seed)

    def _update(self, seed):
        b = AES.new(self.key, AES.MODE_ECB)
        buf = b""
        for i in range(3):
            self.v += 1
            buf += b.encrypt(self.v.to_bytes(16, "big"))
        if seed is not None:
            buf = bytes([x ^ y for x, y in zip(seed, buf)])
        self.key = buf[:32]
        self.v = int.from_bytes(buf[32:], "big")

    def read(self, length):
        b = AES.new(self.key, AES.MODE_ECB)
        ret = b""
        while len(ret) < length:
            self.v += 1
            block = b.encrypt(self.v.to_bytes(16, "big"))
            ret += block
        self._update(None)
        return ret[:length]


for params in [params512, params768, params1024]:
    kats_formatted = []
    seed = bytes(range(48))
    g = NistDRBG(seed)

    print("Generating KATs for {} parameter set.".format(params))

    for i in range(100):
        seed = g.read(48)
        g2 = NistDRBG(seed)

        kseed = g2.read(32) + g2.read(32)
        eseed = g2.read(32)

        pk, sk = KeyGen(kseed, params)
        ct, ss = Enc(pk, eseed, params)

        Dec(sk, ct, params)

        kats_formatted.append(
            {
                "key_generation_seed": bytes(kseed).hex(),
                "sha3_256_hash_of_public_key": bytes(
                    hashlib.sha3_256(pk).digest()
                ).hex(),
                "sha3_256_hash_of_secret_key": bytes(
                    hashlib.sha3_256(sk).digest()
                ).hex(),
                "encapsulation_seed": bytes(eseed).hex(),
                "sha3_256_hash_of_ciphertext": bytes(
                    hashlib.sha3_256(ct).digest()
                ).hex(),
                "shared_secret": bytes(ss).hex(),
            }
        )

        if params == params512:
            output_suffix = "512"
        elif params == params768:
            output_suffix = "768"
        else:
            output_suffix = "1024"

        with open("nistkats_{}.json".format(output_suffix), "w") as f:
            json.dump(kats_formatted, f, ensure_ascii=False, indent=4)
