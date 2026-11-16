# Incremental API

This module implements an incremental API for ML-KEM that allows incremental
encapsulation.

## Keys

The `generate_keypair` function generates an "unpacked" key pair, which is not
serialized according to FIP 203.
Instead, it keeps the unserialized values, which makes decapsulation significantly
faster.
The unpacked key pair is defined as follows:

- private key
  - secret in ntt (vector of rank)
  - implicit rejection value (32 bytes)
- public key
  - t in NTT (vector of rank)
  - seed for matrix A (32 bytes)
  - matrix A (rank x rank)
  - public key hash (32 bytes)

A vector of rank is of the size `512*rank` where rank is 2, 3 or 4, for ML-KEM 512, 768, 1024.

For the incremental API, we split this key up as follows:

- Public key 1
  - seed for matrix A (32 byte)
  - public key hash (32 bytes)
- Public key 2
  - serialized t (rank * 384)
- private key (unpacked)
- matrix A (rank x rank)

### Compressed key bytes

When space is more important than speed, the key pair can be stored with a serialized
private key, which is `s | (t | ⍴) | H(ek) | z` where `s = dk_PKE` and `(t | ⍴) = ek`.
This is `2 * ((rank * 3072) / 8) + 3 * 32` bytes, which is 1632, 2400, 3168 bytes for ML-KEM 512, 768, 1024.

## Encapsulation

With these keys, we can now encapsulate in two steps.

### Encapsulate 1

First, the initiator sends the public key 1 to the other party.
Using this, the other party can generate the first part of the ciphertext c1.

### Encapsulate 2

After receiving the second part of the public key, the other party can generate
ciphertext c2.

Before performing the second step of the encapsulation, the caller should ensure
that the public key is valid.
Because the public key is sent in two steps, `validate_pk` should be used for to
ensure that the two parts are consistent with each other.

## Decapsulation

After the initiator received both ciphertexts c1 and c2, it can decapsulate
the shared secret, using the private key.
