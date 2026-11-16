# ML-KEM C code

The subdirectories of [`extracts/`](extracts/) contain C code extracted from
`libcrux-ml-kem` with [Charon], [Eurydice] and [KaRaMeL].

This page is a quick reference for ML-KEM. For more information on C extraction in libcrux in general, please see
[`C-CODE.md`](../C-CODE.md) at the repository root.

## Extractions

| Directory                                                                  | Parameter sets | SIMD            | Layout                    |
|----------------------------------------------------------------------------|----------------|-----------------|---------------------------|
| [`extracts/c/generated`](extracts/c/generated)                             | 768, 1024      | none (portable) | `.c` + `.h`               |
| [`extracts/c_header_only/generated`](extracts/c_header_only/generated)     | 768            | AVX2            | header-only, C11 or C++20 |
| [`extracts/cpp_header_only/generated`](extracts/cpp_header_only/generated) | 768            | AVX2            | header-only, C11 or C++17 |

All three also contain the SHA-3 code ML-KEM needs, and the
`intrinsics/` headers.

## Extracting

Set `CHARON_HOME`, `EURYDICE_HOME` and `KRML_HOME` to tool checkouts at the
revisions pinned by `EURYDICE_REV` in
[`.docker/c/Dockerfile`](../.docker/c/Dockerfile). Alternatively, use the
Docker image or `nix run --impure .#ml-kem-extract` (see the root
`C-CODE.md`). Then:

```bash
cd extracts/c_header_only    # or c, cpp_header_only
./extract.sh

# or all three at once
extracts/extract-all.sh
```

Each `extract.sh` calls
 [`extracts/common/c.sh`](extracts/common/c.sh), using the
 `extract.yaml` in the same directory.  `cpp_header_only/extract.sh`
 runs `cargo clean` first unless you pass `--no-clean`, and copies the
 result into BoringSSL if `BORINGSSL_HOME` is set.

## Building and testing

The CMake project for each extraction is in subdirectory `generated/`:

```bash
cd extracts/c_header_only/generated    # or c/generated, cpp_header_only/generated
CC=clang CXX=clang++ cmake -B build -G "Ninja Multi-Config"
cmake --build build
./build/Debug/ml_kem_test768
./build/Debug/sha3_test
```

The `c` extraction also has `ml_kem_test1024`.

Environment variables read at configure time:

| Variable               | Effect                                                                  |
|------------------------|-------------------------------------------------------------------------|
| `LIBCRUX_BENCHMARKS=1` | Build `ml_kem_bench`, `ml_kem_keygen`, `ml_kem_encaps` and `sha3_bench` |
| `LIBCRUX_NEON=1`       | Enable the arm64 code paths                                             |
| `LIBCRUX_KYBER=1`      | Build `kyber_test` (header-only extractions)                            |
| `LIBCRUX_UNPACKED=1`   | Also test the unpacked API (`c` extraction)                             |
| `SYMCRYPT_PATH=<path>` | Benchmark against SymCrypt as well                                      |

## Benchmarks

```bash
cd extracts/c_header_only/generated
LIBCRUX_BENCHMARKS=1 CC=clang CXX=clang++ cmake -B build -G "Ninja Multi-Config"
cmake --build build --config Release
./build/Release/ml_kem_bench
```

### Comparing against SymCrypt

Build SymCrypt (needs `pip install pyelftools`):

```bash
git clone https://github.com/microsoft/symcrypt
cd symcrypt
git checkout b070a5d236a4d40aa90524cb5b492463c5452b40
scripts/build.py cmake build --config Release
```

Then configure the benchmarks with `SYMCRYPT_PATH` set:

```bash
SYMCRYPT_PATH=<path-to-symcrypt> LIBCRUX_BENCHMARKS=1 CC=clang CXX=clang++ \
  cmake -B build -G "Ninja Multi-Config"
cmake --build build --config Release
./build/Release/ml_kem_bench
```

On Windows with MSVC:

1. Build SymCrypt with `cmake -S . -B bin -DCMAKE_BUILD_TYPE=Release` and
   `cmake --build bin --config Release`.
2. Apply `extracts/c/generated/fix-symcrypt-includes.patch` in the SymCrypt
   checkout (`git apply <path>/fix-symcrypt-includes.patch`).
3. Configure and build the benchmarks as above.
4. Copy `symcrypt.dll` (from `bin/exe/` in the SymCrypt checkout) into
   `build/Release/`.
5. Run `build/Release/ml_kem_bench`.

[Charon]: https://github.com/AeneasVerif/charon/
[Eurydice]: https://github.com/AeneasVerif/eurydice
[KaRaMeL]: https://github.com/FStarLang/karamel
