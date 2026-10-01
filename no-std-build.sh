#!/bin/bash

set -e
set -x

# Check no_std compatibility for crates that support it by default
cargo build \
  -p libcrux-digest \
  -p libcrux-blake2 \
  -p libcrux-chacha20poly1305 \
  -p libcrux-curve25519 \
  -p libcrux-ed25519 \
  -p libcrux-hacl-rs \
  -p libcrux-hkdf \
  -p libcrux-hmac \
  -p libcrux-nist-kdf \
  -p libcrux-intrinsics \
  -p libcrux-sha3 \
  -p libcrux-p256 \
  -p libcrux-poly1305 \
  -p libcrux-rsa \
  -p libcrux-secrets \
  -p libcrux-sha2 \
  -p libcrux-traits \
  -p libcrux-nist-kdf \
  $RUST_TARGET_FLAG

# Check no_std compatibility for default features except std
  cargo build \
  -p libcrux-ecdh \
  -p libcrux-kem \
  -p libcrux-ecdsa -F rand \
  -p libcrux-ml-dsa -F mldsa44,mldsa65,mldsa87 \
  -p libcrux-ml-kem -F default-no-std \
  --no-default-features \
  $RUST_TARGET_FLAG

# Check no_std compatibility of the top-level crate.
# Only build the rlib, since the `staticlib` and `cdylib` crate types require
# a panic handler and global allocator without `std`.
# HPKE (`all-protocols-no-std`) is not included, because `getrandom` needs a
# custom backend on bare-metal targets.
cargo rustc \
  -p libcrux \
  --lib --crate-type rlib \
  --no-default-features \
  -F all-algorithms-no-std,all-primitives-no-std \
  $RUST_TARGET_FLAG
