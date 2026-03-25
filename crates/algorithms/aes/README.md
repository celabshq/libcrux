# AES-based AEADs

![pre-verification]

This crate implements AES-GCM 128 and 256, as well as AES-CCM 128 and 256.

It provides 
- a portable, bit-sliced implementation
- an x64 optimised implementation using AES-NI
- an Aarch64 optimised implementation using the AES instructions

[pre-verification]: ../../../.assets/pre_verification-orange.svg
