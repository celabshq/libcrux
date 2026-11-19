# NIST KDFs

This crate implements a subset of the KDFs defined in [SP 800-108r1-upd1][108]
and of [SP 800-56Cr2][56].

The implemented subset is:
- A [SP 800-108r1-upd1][108] [KDF in feedback mode][`feedback::kdf`] using HMAC as the PRF
  and a 32-bit big endian counter.
- A [SP 800-56Cr2][56] [two-step KDF][`two_step::kdf`] using the [`feedback::kdf`] for
  the expansion.

[108]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-108r1-upd1.pdf
[56]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Cr2.pdf

# Zeroization

The key derivation key of the two-step KDF, every intermediate `K(i)` and the
keyed HMAC states are wiped before returning. The derived key written to
`k_out` is the caller's to wipe.

# `no_std`

This crate can be used in `no_std` context.
