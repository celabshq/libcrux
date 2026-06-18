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

Run all three in one go:

```sh
docker run --rm -v "$PWD":/app -w /app valgrind bash -c "
  RUSTFLAGS='--cfg valgrind_ct_test' cargo build -p ctgrind-test --profile release-debug &&
  echo '--- SHA3 ---'  && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/sha3 &&
  echo '--- ML-KEM ---' && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/mlkem &&
  echo '--- ML-DSA ---' && valgrind --error-exitcode=1 --track-origins=yes ./target/release-debug/mldsa
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
