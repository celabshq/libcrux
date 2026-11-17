# ctgrind-test

Constant-time (CT) tests using [Valgrind memcheck](https://valgrind.org/docs/manual/mc-manual.html).

Each binary marks secret data as "undefined" with `crabgrind::memcheck::mark_memory`, runs a
cryptographic operation, then marks the output as defined. Valgrind flags any use of a value
containing undefined bits which would result in observable differences in program behaviour,
indicating a potential timing side channel.

For libcrux-ml-dsa, appropriate valgrind requests are already issued in the crate itself, if compiled
with the `valgrind_ct_test` cfg enabled.

## Binaries

| Binary         | What is tested                                          | Expected result                   |
| -------------- | ----------------------------------------------------    | --------------------------------- |
| `ctgrind-test` | Insecure vs. secure byte comparison (demo)              | Insecure comparison flagged       |
| `sha3`         | SHA-256, SHA-512, SHAKE-256 over a secret message       | Clean                             |
| `mlkem`        | ML-KEM-512 `decapsulate` with an undefined private key  | Clean                             |
| `mldsa`        | ML-DSA-{44, 65, 87} `sign` with an undefined signing key| Clean                             |
| `hacl_masks`   | `fstar::uint{8,64}` `eq_mask`/`gte_mask` from libcrux-hacl-rs, as select and accumulate + compare | Clean |
| `chacha20poly1305` | Poly1305 `mac`, ChaCha20-Poly1305 `decrypt` (valid and invalid tag) with an undefined key; the tag comparison | Clean |
| `curve25519`   | X25519 `secret_to_public` and hacl `ecdh` with an undefined secret key | Clean |
| `ed25519`      | Ed25519 `secret_to_public` and `sign` with an undefined private key | Clean |
| `p256`         | P-256 `validate_private_key`, `dh_initiator`, `dh_responder`, `ecdsa_sign_p256_sha2` with an undefined private key and nonce | Clean |
| `rsa`          | RSA-PSS 2048 hacl `rsapss_skey_sign` and the private key validation with an undefined `d` | Clean |
| `sha2`         | SHA-224, SHA-256, SHA-384, SHA-512 over a secret message | Clean |
| `blake2`       | Keyed BLAKE2b and BLAKE2s with an undefined key and message | Clean |
| `hmac`         | HMAC with SHA-2 and SHA-3 with an undefined key and message | Clean |
| `hkdf`         | HKDF extract and expand with SHA-2 and SHA-3 with undefined input keying material | Clean |
| `kmac`         | KMAC128 and KMAC256 with an undefined key and message | Clean |
| `nist_kdf`     | NIST SP 800-56C two-step KDF and SP 800-108 feedback KDF with an undefined input key | Clean |
| `hmac_drbg`    | HMAC-DRBG with SHA-256, SHA-384, SHA-512: instantiate, generate and reseed with undefined entropy | Clean |
| `aes`          | AES-GCM-128/256 and AES-CCM-128/256 encrypt and decrypt with an undefined key, multiplexed and portable | Clean |
| `p384`         | P-384 private key validation, public key derivation and ECDH with an undefined private key | Clean |
| `ecdsa`        | ECDSA P-256 signing and public key derivation with an undefined private key and nonce | Clean |

## Running with Docker

_Required on MacOS (arm)_

Build the image once from the repo root:

```sh
docker build -t valgrind crates/utils/ctgrind-test/
```

Then run each binary from the **repo root**:

```sh
# SHA-3
docker run --rm -v "$PWD":/app -w /app valgrind bash -c \
  "cargo build -p ctgrind-test --profile release-debug --bin sha3 && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/sha3"

# ML-KEM
docker run --rm -v "$PWD":/app -w /app valgrind bash -c \
  "cargo build -p ctgrind-test --profile release-debug --bin mlkem && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/mlkem"

# ML-DSA
docker run --rm -v "$PWD":/app -w /app valgrind bash -c \
  "RUSTFLAGS='--cfg valgrind_ct_test' cargo build -p ctgrind-test --profile release-debug --bin mldsa && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/mldsa"
```

Run all of them in one go:

```sh
docker run --rm -v "$PWD":/app -w /app valgrind bash -c "
  RUSTFLAGS='--cfg valgrind_ct_test' cargo build -p ctgrind-test --profile release-debug &&
  for b in sha3 mlkem mldsa hacl_masks chacha20poly1305 curve25519 ed25519 p256 rsa \\
           sha2 blake2 hmac hkdf kmac nist_kdf hmac_drbg aes p384 ecdsa; do
    echo \"--- \$b ---\" && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/\$b || exit 1
  done
"
```

> **Why Docker?**  
> Valgrind on macOS/Apple Silicon does not support all Apple-proprietary ARM64 system registers
> and aborts with `disInstr: unhandled instruction`. The Docker image runs a Linux environment
> where Valgrind works correctly.

## What is and isn't marked undefined

Only the genuinely secret bytes are marked as undefined. Public data embedded in key structs must stay
clean or Valgrind produces false positives in public sampling routines.

**ML-KEM private key** (`[cpa_sk | pk | H(pk) | z]`, e.g. 1632 bytes for ML-KEM-512):
- Undefined: `cpa_sk` (first 768 bytes) and `z` (last 32 bytes)
- Not Undefined: embedded public key and `H(pk)` — these are used in the FO re-encryption
  step to reconstruct matrix A from the public seed ρ

**ML-DSA signing key** (`[ρ | K | tr | s₁, s₂, t₀]`, e.g. 2560 bytes for ML-DSA-44):
- Undefined: `K` (bytes 32–64) and `s₁, s₂` (bytes 128–896)
- Not Undefined: `ρ` (public seed for matrix A), `tr` (hash of verification key) and `t₀` (𝑑 least significant bits of each coefficient of the uncompressed public-key polynomial 𝐭)

## Declassifications in ML-DSA

The signing operation in ML-DSA includes some operations that
technically depend on secret data, but are in fact safe under the
assumptions of the Dilithium security proof [1][dilithium-round3]. In these cases we
explicitly mark the memory as `MemState::Defined` for valgrind, in
order to avoid false positives. The reasoning for each declassify operation 
is given in the ML-DSA implementation.

## HACL mask functions

The HACL-based crates (ChaCha20-Poly1305, Poly1305, X25519, Ed25519, P-256, RSA) use the
`fstar::uintX::{eq_mask, gte_mask}` functions from libcrux-hacl-rs for constant-time comparisons.
These are plain bit operations without an optimisation barrier, so the compiler could turn them
back into branches after inlining. The binaries above check that it does not.

Some of these comparisons end in a legitimate branch on a public result, such as whether a tag or
a key is valid. Valgrind cannot distinguish that branch from a comparison that was compiled into a
branch. Declassifying the result right before the branch would not help: the valgrind client
request is an optimisation barrier and could hide exactly the optimisation we are looking for. So:

- Where the HACL function returns the validity as a `bool` without branching on it (X25519 `ecdh`,
  the P-256 functions, RSA `rsapss_skey_sign`), the test calls that function directly and
  declassifies the returned `bool`.
- Where HACL branches internally (the tag check in ChaCha20-Poly1305 `decrypt`, the private key
  check in RSA `rsapss_skey_sign`), the comparison is an `#[inline]` helper whose result is
  declassified with `libcrux_secrets::mem_requests::ct_declassify` before the branch. The crate also
  exposes a `ct_test` module (only with `--cfg valgrind_ct_test`) that calls the same helper and
  returns its result, which the test checks without the branch.

## Declassified public decisions

Outside the HACL code, the following results are public by design and are declassified with
`libcrux_secrets::mem_requests::ct_declassify` right before the branch on them:

- AES-GCM and AES-CCM `decrypt`: whether the tag is valid.
- ECDSA P-256: whether a private key or nonce is valid (an invalid nonce is resampled), whether
  signing succeeded, and whether the public key could be derived.
- P-384: the point-at-infinity assertions in public key derivation and ECDH, which never fail.
