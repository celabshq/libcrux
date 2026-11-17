# C code in libcrux

Parts of libcrux are translated from Rust into C (and C++-compatible C). This
document lists every C extraction in the workspace, explains how they are
produced, and describes how to build and test them.

## Toolchain

The pipeline is [Charon] → [Eurydice] → [KaRaMeL]:

1. **Charon** compiles a crate with `rustc` and dumps it as LLBC (Low-Level
   Borrow Calculus) into a `<crate>.llbc` file at the repository root.
2. **Eurydice** reads one or more `.llbc` files, monomorphizes generics,
   applies the bundling rules from a YAML config, and lowers the result to
   KaRaMeL's AST.
3. **KaRaMeL** (used as a library by Eurydice) prints the final C.

The Rust code sees `--cfg eurydice` during extraction, so code paths that the
tools cannot handle can be switched out with `#[cfg(eurydice)]`.

### Tool versions

`EURYDICE_REV` in [`.docker/c/Dockerfile`](.docker/c/Dockerfile)
specifies the Eurydice commit that we currently use for the
extraction. Eurydice pins the required versions of Charon and KaRaMel
itself.

### Getting the tools

**Docker (as used on CI).** The image `ghcr.io/cryspen/libcrux-c:latest` is
built from [`.docker/c/`](.docker/c/) and has all tools installed:

```bash
./c.sh --docker          # extracts ML-KEM (all three) and ML-DSA inside the container
```

**Nix.** [`flake.nix`](flake.nix) provides the pinned tools and sets
`CHARON_HOME`, `EURYDICE_HOME`, `KRML_HOME` and the `*_REV` variables following the Dockerfile. It is
community-maintained and unsupported.

```bash
nix develop --impure                     # shell with all tools; then run the scripts below
nix run --impure .#ml-kem-extract        # all ML-KEM extractions
nix run --impure .#ml-dsa-extract        # ML-DSA (cg)
nix run --impure .#combined-extract      # combined extraction, arguments are passed through
```

**Manual.** Mirror [`.docker/c/install.sh`](.docker/c/install.sh) and
[`.docker/c/ext-tools.sh`](.docker/c/ext-tools.sh): an OCaml 5.1.1 opam
switch with the packages listed in `install.sh`, then

```bash
git clone https://github.com/AeneasVerif/eurydice.git $HOME/eurydice
cd $HOME/eurydice
git checkout <EURYDICE_REV from .docker/c/Dockerfile>
make setup-karamel
make setup-charon
make

export EURYDICE_HOME=$HOME/eurydice
export KRML_HOME=$HOME/eurydice/karamel
export CHARON_HOME=$HOME/eurydice/charon
```

All extraction scripts refuse to run unless `CHARON_HOME`, `EURYDICE_HOME`
and `KRML_HOME` are set. Formatting needs `clang-format-18` on the `PATH`.

## Overview of all extractions

| Output directory                                    | Algorithms                            | Layout                        | Script                     | Config                  |
|-----------------------------------------------------|---------------------------------------|-------------------------------|----------------------------|-------------------------|
| `libcrux-ml-kem/extracts/c/generated`               | ML-KEM 768, 1024 (portable only)      | `.c` + `.h`                   | `extract.sh`               | `extract.yaml`          |
| `libcrux-ml-kem/extracts/c_header_only/generated`   | ML-KEM 768 (portable + AVX2)          | header-only, C / C++20        | `extract.sh`               | `extract.yaml`          |
| `libcrux-ml-kem/extracts/cpp_header_only/generated` | ML-KEM 768 (portable + AVX2)          | header-only, C++17-compatible | `extract.sh`               | `extract.yaml`          |
| `libcrux-ml-dsa/cg`                                 | ML-DSA 65 (portable + AVX2)           | header-only, C++17-compatible | `boring.sh`                | `cg.yaml`               |
| `combined_extraction/c`                             | ML-KEM 512/768/1024 + ML-DSA 44/65/87 | `.c` + `.h`                   | `extract.sh`               | `extract-c.yaml`        |
| `combined_extraction/c-header-only`                 | ML-KEM 512/768/1024 + ML-DSA 44/65/87 | header-only, C++17-compatible | `extract.sh --header-only` | `extract-c-header.yaml` |

The generated code in all of these directories is committed to the
repository, so users can consume it without installing the toolchain.
SHA-3 and the `libcrux-secrets` helpers do not have their own extraction.
They are pulled in by each of the extractions above (for example
`libcrux_sha3_portable.h`). `crates/algorithms/sha3/c.sh` is a stub that
only points to the ML-KEM scripts.

CI treats all of these extractions the same way, see [CI](#ci).

## Combined extraction (`combined_extraction/`)

The combined extraction produces one library containing all ML-KEM and ML-DSA
parameter sets, sharing a single copy of SHA-3 and the common code. See
[`combined_extraction/README.md`](combined_extraction/README.md) for details.

```bash
cd combined_extraction
./extract.sh                  # both outputs: Charon once, Eurydice twice
./extract.sh --c-only         # only c/
./extract.sh --header-only    # only c-header-only/
./extract.sh --no-charon      # reuse the .llbc files at the repository root
./extract.sh --portable       # no SIMD
./extract.sh --clean          # cargo clean and delete old output first
```

Unlike the per-crate scripts, `extract.sh` runs Charon separately in
`crates/utils/secrets`, `crates/algorithms/sha3`, `libcrux-ml-kem` and
`libcrux-ml-dsa`, then passes all four `.llbc` files to Eurydice at once. It
does not run clang-format, and does not copy the glue or KaRaMeL headers.
`c-header-only/` shares `tests/`, `intrinsics/` and `karamel/` with `c/` via
symlinks.

```bash
cd combined_extraction/c      # or c-header-only
cmake -B build -G "Ninja Multi-Config"
cmake --build build
ctest --test-dir build -C Debug --output-on-failure
```

`c/` builds a `combined_static` library that the tests link against.
`c-header-only/` has no library target. Use `-DENABLE_SANITIZERS=ON` for
ASan/UBSan, and `LIBCRUX_KYBER=1` for the Kyber 768 test.

## Convenience scripts

- [`c.sh`](c.sh) at the repository root runs `libcrux-ml-kem/extracts/extract-all.sh`
  and `libcrux-ml-dsa/boring.sh`. Use `--docker` to run in the CI image,
  `--no-extract` to skip extraction, and `--build` / `--test` to build and
  test the three ML-KEM extractions.
- `libcrux-ml-kem/extracts/extract-all.sh` runs all three ML-KEM extractions
  and forwards its arguments to each `extract.sh`.

## CI

Every extraction is checked by the reusable workflow
[`c-extraction.yml`](.github/workflows/c-extraction.yml), so they are all
treated the same:

- **Pull requests:** the committed C code is built and tested.
- **Merge queue, pushes to `main`/`dev`, manual runs:** the code is
  re-extracted in the CI Docker image. The job fails if the result differs
  from the committed code, ignoring the tool revisions in the file headers
  (see `libcrux-ml-kem/extracts/common/headers_kill_revs.sh`). The freshly
  extracted code is then built and tested.
- **Changes to `.docker/c/`:** the `latest` image doesn't contain them yet, so
  the regular workflows only build the committed code, and
  `docker-c-build-test.yml` re-extracts and checks everything with the newly
  built image instead.

Building and testing runs on Linux (clang), macOS and Windows with
`cmake -B build -DCMAKE_BUILD_TYPE=Debug`, `cmake --build build --config Debug`
and `ctest --test-dir build -C Debug --output-on-failure` in each output
directory. Every `CMakeLists.txt` registers its tests with `ctest`, running
them from the source directory so the KAT files are found.

| Workflow                    | Extractions                                                    |
|-----------------------------|----------------------------------------------------------------|
| `mlkem-c.yml`               | ML-KEM `c`, `c_header_only`, `cpp_header_only`                 |
| `mldsa-c.yml`               | ML-DSA `cg`                                                    |
| `combined-extraction-c.yml` | `combined_extraction/c` and `c-header-only` (one extraction run) |
| `docker-c-build-test.yml`   | All of the above, with a freshly built Docker image            |

When you change Rust code that is extracted, re-extract and commit the result.
Otherwise the diff jobs fail in the merge queue.

## Side-Channel Resistance and Zeroization

To quote the workspace `Readme.md` with respect to the Rust code:
> Executables compiled from the code in this repository are *not* verified to be side-channel resistant, although we try to enforce that the source code is secret-independent (also sometimes called “constant-time”).

>At this moment, we do also not give guarantees on the zeroization of secret values on the stack or heap. We are working on best-effort implementations of zeroization.

The same holds for the C extractions provided here. In the case of ML-KEM we configure the extraction such that operations which should happen in constant time are extracted to separate files `libcrux_ct_ops.{c,h}`. This makes it easier for consumers to provide their own implementations of these operations, independently from us.


## Troubleshooting

- **Charon does not produce an `.llbc` file.** This is a known Charon issue
  with stale build artefacts. Run `cargo clean` (or `rm -rf target`) and try
  again. The scripts print a hint when this happens.
- **The output differs only in the header comments.** The revisions in
  `code_gen.txt` differ. CI ignores them, but keep them accurate by extracting
  with the pinned toolchain.
- **`clang-format-18: command not found`.** Install clang-format 18 or put a
  wrapper with that name on the `PATH`. The Nix shell provides one.

[Charon]: https://github.com/AeneasVerif/charon/
[Eurydice]: https://github.com/AeneasVerif/eurydice
[KaRaMeL]: https://github.com/FStarLang/karamel
