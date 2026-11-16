# ML-KEM

This folder contains the extracted ML-KEM 768 code for C and C++17. It is
header-only.

## Generating the code

The code is generated from Rust using [Charon], [Eurydice] and [Karamel].
The [`extract.sh`](../extract.sh) script drives the extraction, using the
[`extract.yaml`](../extract.yaml) configuration file. Run it from the
`cpp_header_only` directory:

```bash
cd ..
./extract.sh --no-clean
```

While running the commands separately is possible, it is not recommended
because the script sets all necessary configuration flags. See
[`libcrux-ml-kem/C-CODE.md`](../../../C-CODE.md) for the toolchain setup.

## Build

Make sure to use `CC=clang CXX=clang++` when benchmarking on Linux to get full performance.

```bash
cmake -B build -G "Ninja Multi-Config"
cmake --build build
```

To enable neon builds, set `LIBCRUX_NEON=1`.

### Test

```bash
./build/Debug/ml_kem_test768
./build/Debug/sha3_test
```

## Benchmarks

```bash
LIBCRUX_BENCHMARKS=1 cmake -B build -G "Ninja Multi-Config"
cmake --build build --config Release
./build/Release/ml_kem_bench
```

[Charon]: https://github.com/AeneasVerif/charon/
[Eurydice]: https://github.com/AeneasVerif/eurydice
[Karamel]: https://github.com/FStarLang/karamel
