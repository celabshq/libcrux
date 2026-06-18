# libcrux - a high-assurance cryptographic library in Rust

libcrux is a cryptographic library written in Rust. It uses formally verified
code generated from the [HACL* project](https://github.com/hacl-star/hacl-star) and includes Rust code that is
directly verified for runtime safety and functional correctness using the
[hax toolchain](https://github.com/cryspen/hax). For more details, see below.

libcrux is in pre-release (all of its crates are versioned < `0.1`). If you wish to use any of these crates
in production, get in touch with the [maintainers](mailto:info@cryspen.com) and we can advise you on whether
libcrux is a good fit for your use-case.

## Which crate to use?

Libcrux is organized in different sub-crates which implement cryptographic algorithms, primitives and protocols.

| **Primitive**                                 | **Algorithms**                                                                                          | **sub-crate**                                                                                                 |
|-----------------------------------------------|---------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|
| KEM                                           | ML-KEM                                                                                                  | [`libcrux-ml-kem`](https://github.com/cryspen/libcrux/tree/main/libcrux-ml-kem)                               |
|                                               | P256 DHKEM                                                                                              | [`libcrux-p256`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/p256)                         |
|                                               | X25519 ECDH                                                                                             | [`libcrux-curve25519`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/curve25519)             |
|                                               | [X25519MlKem768Draft00](https://www.ietf.org/archive/id/draft-westerbaan-cfrg-hpke-xyber768d00-00.html) | [`libcrux-kem`](https://github.com/cryspen/libcrux/tree/main/libcrux-kem)                                     |
|                                               | [XWing](https://www.ietf.org/archive/id/draft-connolly-cfrg-xwing-kem-06.html)                          | [`libcrux-kem`](https://github.com/cryspen/libcrux/tree/main/libcrux-kem)                                     |
| AEAD                                          | AES-GCM 128/256                                                                                         | [`libcrux-aes`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/aes)                           |
|                                               | AES-CCM 128/256                                                                                         | [`libcrux-aes`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/aes)                           |
|                                               | ChaCha20Poly1305                                                                                        | [`libcrux-chacha20poly1305`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/chacha20poly1305) |
|                                               | XChaCha20Poly1305                                                                                       | [`libcrux-chacha20poly1305`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/chacha20poly1305) |
| Hash                                          | BLAKE2                                                                                                  | [`libcrux-blake2`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/blake2)                     |
|                                               | SHA2                                                                                                    | [`libcrux-sha2`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/sha2)                         |
|                                               | SHA3                                                                                                    | [`libcrux-sha3`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/sha3)                         |
| Signature                                     | P256 ECDSA                                                                                              | [`libcrux-ecdsa`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/ecdsa)                       |
|                                               | ML-DSA                                                                                                  | [`libcrux-ml-dsa`](https://github.com/cryspen/libcrux/tree/main/libcrux-ml-dsa)                               |
|                                               | Ed25519                                                                                                 | [`libcrux-ed25519`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/ed25519)                   |
|                                               | RSA-PSS                                                                                                 | [`libcrux-rsa`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/rsa)                           |
| KDF                                           | HKDF                                                                                                    | [`libcrux-hkdf`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/hkdf)                         |
| MAC                                           | HMAC                                                                                                    | [`libcrux-hmac`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/hmac)                         |
|                                               | Poly1305                                                                                                | [`libcrux-poly1305`](https://github.com/cryspen/libcrux/tree/main/crates/algorithms/poly1305)                 |
| **Protocol**                                  |                                                                                                         | **sub-crate**                                                                                                 |
| [PSQ](https://cryspen.com/post/psq-announce/) |                                                                                                         | [`libcrux-ml-kem`](https://github.com/cryspen/libcrux/tree/main/libcrux-ml-kem)                               |

This repository also contains a `libcrux` crate, which is a simple re-exporter of the functionality offered by the individual sub-crates and is useful for us to ensure sub-crate versions remain compatible with each other. We recommend you use the individual sub-crates that provide the functionality you need instead of the main `libcrux` crate.



## Minimum Supported Rust Version (MSRV)

The default feature set has a MSRV of `1.89.0`. `no_std` environments
are supported starting from Rust version `1.89.0`.

## `no_std` support
`libcrux` and the individual primitive crates it depends on support
`no_std` environments given a global allocator for the target
platform.

## Verification status

As a quick indicator of overall verification status, subcrates in this workspace include the following badges:

- ![pre-verification] to indicate that most (or all) of the code that
  is contained in default features of that crate is not (yet)
  verified.
- ![verified-hacl] to indicate that algorithms in a crate have been verified and extracted to Rust as part of the HACL* project. The source F* code in HACL* is verified for memory safety, functional correctness against a high-level spec, and secret independence (for details, see [these](https://eprint.iacr.org/2017/536) [papers](https://arxiv.org/abs/1703.00053).) Top-level Rust APIs in these crates accessing the code from HACL* may not be verified.

- ![verified] to indicate that most (or all) of the Rust code that is contained in the default feature set is verified using the [hax toolchain](https://github.com/cryspen/hax). Where indicated, the code is verified to be panic free and functionally correct against a mathematical spec (for an overview of hax, see [this paper](https://eprint.iacr.org/2025/142))

Importantly, executables compiled from the code in this repository are *not* verified to be side-channel resistant, although we try to enforce that the source code is secret-independent (also sometimes called “constant-time”).

In every case, please refer to the more detailed notes on verification in each sub-crate to learn more about what has (or has not) been verified in the particular case, and [reach out to us](mailto:info@cryspen.com) if you have any questions.

## Publications

Libcrux was introduced in this article at RWC 2023. The verification methodology used to build libcrux is described in [this paper](https://eprint.iacr.org/2025/142).

[architecture]: ./Architecture.md
[hacspec]: https://hacspec.org
[formal verification]: ./formal_verification

[verified-hacl]: ./.assets/verified-hacl-rs-brightgreen.svg
[verified]: ./.assets/verified-brightgreen.svg
[pre-verification]: ./.assets/pre_verification-orange.svg
