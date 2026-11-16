#! /usr/bin/env python3

from dilithium import Dilithium2, Dilithium3, Dilithium5
from aes256_ctr_drbg import AES256_CTR_DRBG

import json
import hashlib


def generate_nistkats(algorithm):
    kats_formatted = []
    pre_hashed_kats_formatted = []

    entropy_input = bytes([i for i in range(48)])
    rng = AES256_CTR_DRBG(entropy_input)

    print("Generating KATs for ML-DSA-{}{}.".format(algorithm.k, algorithm.l))

    for i in range(100):
        seed = rng.random_bytes(48)

        algorithm.set_drbg_seed(seed)

        vk, sk = algorithm.keygen()

        msg_len = 33 * (i + 1)
        msg = rng.random_bytes(msg_len)
        sig = algorithm.sign(sk, msg)

        kats_formatted.append(
            {
                "key_generation_seed": bytes(algorithm.keygen_seed).hex(),
                "sha3_256_hash_of_verification_key": bytes(
                    hashlib.sha3_256(vk).digest()
                ).hex(),
                "sha3_256_hash_of_signing_key": bytes(
                    hashlib.sha3_256(sk).digest()
                ).hex(),
                "message": bytes(msg).hex(),
                "signing_randomness": bytes(algorithm.signing_randomness).hex(),
                "sha3_256_hash_of_signature": bytes(
                    hashlib.sha3_256(sig).digest()
                ).hex(),
            }
        )
        with open("nistkats-{}{}.json".format(algorithm.k, algorithm.l), "w") as f:
            json.dump(kats_formatted, f, ensure_ascii=False, indent=4)


    for i in range(100):
        seed = rng.random_bytes(48)

        algorithm.set_drbg_seed(seed)

        vk, sk = algorithm.keygen()

        msg_len = 33 * (i + 1)
        msg = rng.random_bytes(msg_len)
        sig_pre_hashed = algorithm.sign_pre_hashed_shake128(sk, msg)

        pre_hashed_kats_formatted.append(
            {
                "key_generation_seed": bytes(algorithm.keygen_seed).hex(),
                "sha3_256_hash_of_verification_key": bytes(
                    hashlib.sha3_256(vk).digest()
                ).hex(),
                "sha3_256_hash_of_signing_key": bytes(
                    hashlib.sha3_256(sk).digest()
                ).hex(),
                "message": bytes(msg).hex(),
                "signing_randomness": bytes(algorithm.signing_randomness).hex(),
                "sha3_256_hash_of_signature": bytes(
                    hashlib.sha3_256(sig_pre_hashed).digest()
                ).hex(),
            }
        )
            
        with open("nistkats_pre_hashed-{}{}.json".format(algorithm.k, algorithm.l), "w") as f:
            json.dump(pre_hashed_kats_formatted, f, ensure_ascii=False, indent=4)

generate_nistkats(Dilithium2)
generate_nistkats(Dilithium3)
generate_nistkats(Dilithium5)
