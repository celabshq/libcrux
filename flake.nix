#
#       Disclaimer: This nix environment is provided as-is.
#       None of this is officially supported and use is at your own risk.
#       We do not maintain or support nix environments.
#
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    # Keep this revision in sync with EURYDICE_REV in .docker/c/Dockerfile,
    # which is what CI uses for the extraction. charon and karamel follow
    # eurydice transitively, so pinning eurydice pins all three.
    eurydice.url = "github:aeneasverif/eurydice/aaa9fa657fb6f09802edb890252040d94cd93982";
    eurydice.inputs.karamel.inputs.fstar.follows = "fstar-pinned";
    # Pin hax 0.3.6, update if hax-lib is updated in Cargo.toml
    hax.url = "github:cryspen/hax/87ba96831ecfeb7dbb54efcf97036fbc5f25bc71";
    # Pin F* to the version CI uses for hax lax/prove (see .github/workflows/mlkem-hax.yml).
    fstar-pinned.url = "github:FStarLang/FStar/v2025.10.06";
    googletest = {
      url = "github:google/googletest/release-1.11.0";
      flake = false;
    };
    benchmark = {
      url = "github:google/benchmark/v1.8.4";
      flake = false;
    };
    json = {
      url = "github:nlohmann/json/v3.10.3";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      eurydice,
      fstar-pinned,
      hax,
      googletest,
      benchmark,
      json,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        charon = eurydice.inputs.charon;
        crane = charon.inputs.crane;
        # Use the overridden package exported by the eurydice flake.
        karamel = eurydice.packages.${system}.karamel;
        fstar = fstar-pinned;

        tools-environment = {
          CHARON_HOME = charon.packages.${system}.charon;
          EURYDICE_HOME = pkgs.runCommand "eurydice-home" { } ''
            mkdir -p $out
            cp -r ${eurydice.packages.${system}.default}/bin/eurydice $out
            cp -r ${eurydice}/include $out
          '';
          FSTAR_HOME = fstar.packages.${system}.default;
          HAX_HOME = hax;
          KRML_HOME = karamel;

          CHARON_REV = charon.rev or "dirty";
          EURYDICE_REV = eurydice.rev or "dirty";
          KRML_REV = karamel.version;
          FSTAR_REV = fstar.rev or "dirty";
          LIBCRUX_REV = self.rev or "dirty";
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ "aarch64-unknown-linux-gnu" ];
        };
        rustNightlyWithMiri = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.default.override {
            extensions = [
              "miri"
              "rust-src"
            ];
          }
        );
        # Wrapper so `cargo miri ...` uses the nightly toolchain while the
        # default `cargo`/`rustc` in the shell remain stable. Stable cargo
        # exports CARGO/RUSTC pointing at its own binaries when invoking
        # subcommands, so we override them to the nightly equivalents.
        cargoMiri = pkgs.writeShellScriptBin "cargo-miri" ''
          export PATH=${rustNightlyWithMiri}/bin:$PATH
          export CARGO=${rustNightlyWithMiri}/bin/cargo
          export RUSTC=${rustNightlyWithMiri}/bin/rustc
          exec ${rustNightlyWithMiri}/bin/cargo-miri "$@"
        '';
        # `cargo fuzz` needs nightly rustc (`-Zsanitizer=address`), so unlike
        # miri above it can't be wrapped: `devShells.fuzz` ships this as the
        # shell's only cargo/rustc. `llvm-tools-preview` is for
        # `cargo fuzz coverage`, which needs an llvm-profdata matching rustc.
        rustNightlyForFuzz = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.default.override {
            extensions = [
              "rust-src"
              "llvm-tools-preview"
            ];
          }
        );
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        # Cargo.lock is gitignored, so it isn't part of the flake's source
        # tree. Read it from the invocation directory; requires `--impure`
        # and that nix is invoked from the repo root. When the lockfile is
        # absent (fresh checkout), fall back to an error-derivation so that
        # the devShell still evaluates — its shellHook will then generate
        # the lockfile via `cargo generate-lockfile`.
        pwd = builtins.getEnv "PWD";
        hasCargoLock = pwd != "" && builtins.pathExists "${pwd}/Cargo.lock";
        defaultCargoLock =
          if hasCargoLock then
            builtins.path {
              path = "${pwd}/Cargo.lock";
              name = "Cargo.lock";
            }
          else
            pkgs.runCommand "cargo-lock-missing" { } ''
              cat >&2 <<'EOF'
              error: Cargo.lock not found.

              Cargo.lock is intentionally not committed to this repo. To
              bootstrap it, enter the dev shell once (the shellHook runs
              `cargo generate-lockfile` automatically):

                nix develop --impure

              Or run the dedicated app:

                nix run --impure .#gen-lockfile

              Then re-run your `nix build` command.
              EOF
              exit 1
            '';

        # Construct a copy of the current directory with the given `Cargo.lock` added.
        build_src =
          cargoLock:
          let
            src = builtins.filterSource (name: _: !(pkgs.lib.hasSuffix "flake.nix" name)) ./.;
          in
          pkgs.runCommand "libcrux-src" { } ''
            cp -r ${src} $out
            chmod u+w $out
            rm -f $out/Cargo.lock
            cp ${cargoLock} $out/Cargo.lock
          '';

        ml-kem =
          pkgs.callPackage
            (
              {
                lib,
                clang-tools_18,
                cmake,
                mold-wrapped,
                ninja,
                git,
                python3,
                craneLib,
                hax,
                googletest,
                benchmark,
                json,
                tools-environment,
                cargoLock ? defaultCargoLock,
                checkHax ? true,
                runBenchmarks ? true,
              }:
              let
                src = build_src cargoLock;
                cargoArtifacts = craneLib.buildDepsOnly { inherit src; };
              in
              craneLib.buildPackage (
                tools-environment
                // {
                  name = "ml-kem";
                  inherit src cargoArtifacts;

                  nativeBuildInputs = [
                    clang-tools_18
                    # Alias `clang_format` to `clang-format-18`
                    (pkgs.writeShellScriptBin "clang-format-18" ''exec ${clang-tools_18}/bin/clang-format "$@"'')
                    cmake
                    mold-wrapped
                    ninja
                    git
                    python3
                    fstar.packages.${system}.default
                  ]
                  ++ lib.optional checkHax [
                    hax
                  ];
                  buildPhase = ''
                    cd crates/algorithms/ml-kem
                    patchShebangs ./.
                    ${lib.optionalString checkHax ''
                      python hax.py extract
                    ''}
                    ./c.sh
                    cd c
                    ${lib.optionalString runBenchmarks "LIBCRUX_BENCHMARKS=1"} \
                      cmake \
                      -DFETCHCONTENT_SOURCE_DIR_GOOGLETEST=${googletest} \
                      -DFETCHCONTENT_SOURCE_DIR_BENCHMARK=${benchmark} \
                      -DFETCHCONTENT_SOURCE_DIR_JSON=${json} \
                      -DCMAKE_EXE_LINKER_FLAGS="-fuse-ld=mold" \
                      -DCMAKE_SHARED_LINKER_FLAGS="-fuse-ld=mold" \
                      -G "Ninja Multi-Config" -B build
                    cmake --build build --config Release
                    rm -rf build/_deps
                  '';
                  checkPhase = ''
                    build/Release/ml_kem_test
                    build/Release/sha3_test
                  ''
                  + lib.optionalString runBenchmarks ''
                    build/Release/ml_kem_bench
                  '';
                  installPhase = ''
                    cd ./..
                    cp -r . $out
                  '';
                }
              )
            )
            {
              inherit
                googletest
                benchmark
                json
                craneLib
                tools-environment
                ;
              hax = hax.packages.${system}.default;
              clang-tools_18 = pkgs.llvmPackages_18.clang-tools;
            };

        ml-dsa =
          pkgs.callPackage
            (
              {
                lib,
                clang-tools_18,
                cmake,
                mold-wrapped,
                ninja,
                git,
                python3,
                perl,
                craneLib,
                hax,
                googletest,
                benchmark,
                json,
                tools-environment,
                cargoLock ? defaultCargoLock,
                checkHax ? true,
              }:
              let
                src = build_src cargoLock;
                cargoArtifacts = craneLib.buildDepsOnly { inherit src; };
              in
              craneLib.buildPackage (
                tools-environment
                // {
                  name = "ml-dsa";
                  inherit src cargoArtifacts;

                  nativeBuildInputs = [
                    clang-tools_18
                    # Alias `clang_format` to `clang-format-18`
                    (pkgs.writeShellScriptBin "clang-format-18" ''exec ${clang-tools_18}/bin/clang-format "$@"'')
                    cmake
                    mold-wrapped
                    ninja
                    git
                    python3
                    fstar.packages.${system}.default
                    perl
                  ]
                  ++ lib.optional checkHax [
                    hax
                  ];
                  buildPhase = ''
                    cd crates/algorithms/ml-dsa
                    patchShebangs ./.
                    ${lib.optionalString checkHax ''
                      ./hax.sh extract
                    ''}
                    ./boring.sh --no-clean
                    cd cg
                    cmake \
                      -DFETCHCONTENT_SOURCE_DIR_GOOGLETEST=${googletest} \
                      -DFETCHCONTENT_SOURCE_DIR_BENCHMARK=${benchmark} \
                      -DFETCHCONTENT_SOURCE_DIR_JSON=${json} \
                      -DCMAKE_EXE_LINKER_FLAGS="-fuse-ld=mold" \
                      -DCMAKE_SHARED_LINKER_FLAGS="-fuse-ld=mold" \
                      -G "Ninja Multi-Config" -B build
                    cmake --build build --config Release
                    rm -rf build/_deps
                  '';
                  checkPhase = ''
                    build/Release/ml_dsa_test
                  '';
                  installPhase = ''
                    cd ./..
                    cp -r . $out
                  '';
                }
              )
            )
            {
              inherit
                googletest
                benchmark
                json
                craneLib
                tools-environment
                ;
              hax = hax.packages.${system}.default;
              clang-tools_18 = pkgs.llvmPackages_18.clang-tools;
            };

        clang-format-18-wrapper = pkgs.writeShellScriptBin "clang-format-18" ''
          exec ${pkgs.llvmPackages_18.clang-tools}/bin/clang-format "$@"
        '';

        # Env vars from `tools-environment` coerced to strings so they're
        # safe to pass through `writeShellApplication`'s `runtimeEnv`
        # (which JSON-encodes anything that's still an attrset).
        tools-environment-strings = builtins.mapAttrs (_: toString) tools-environment;

        ml-kem-extract-app = pkgs.writeShellApplication {
          name = "ml-kem-extract";
          runtimeInputs = [
            rustToolchain
            pkgs.llvmPackages_18.clang-tools
            clang-format-18-wrapper
            pkgs.git
            pkgs.python3
            fstar.packages.${system}.default
          ];
          runtimeEnv = tools-environment-strings;
          # there is a mismatch between the version of karamel installed to $KRML_HOME by the flake.nix
          # and what CI expects the layout to be from a repo checkout. In the installed version,
          # the include path is include/krml/krml whereas it is include/krml in the repo
          # Use --no-karamel_include for now to work around that
          text = ''
            root=$(git rev-parse --show-toplevel)
            cd "$root/crates/algorithms/ml-kem/extracts"
            ./extract-all.sh --no-karamel_include
          '';
        };

        ml-dsa-extract-app = pkgs.writeShellApplication {
          name = "ml-dsa-extract";
          runtimeInputs = [
            rustToolchain
            pkgs.llvmPackages_18.clang-tools
            clang-format-18-wrapper
            pkgs.git
            pkgs.python3
            pkgs.perl
            fstar.packages.${system}.default
          ];
          runtimeEnv = tools-environment-strings;
          text = ''
            root=$(git rev-parse --show-toplevel)
            cd "$root/crates/algorithms/ml-dsa"
            ./boring.sh --no-clean
          '';
        };

        combined-extract-app = pkgs.writeShellApplication {
          name = "combined-extract";
          runtimeInputs = [
            rustToolchain
            pkgs.llvmPackages_18.clang-tools
            clang-format-18-wrapper
            pkgs.git
          ];
          runtimeEnv = tools-environment-strings;
          text = ''
            root=$(git rev-parse --show-toplevel)
            cd "$root/combined_extraction"
            ./extract.sh "$@"
          '';
        };

        gen-lockfile-app = pkgs.writeShellApplication {
          name = "gen-lockfile";
          runtimeInputs = [ rustToolchain ];
          text = ''
            if [ -f Cargo.lock ]; then
              echo "Cargo.lock already exists at $PWD/Cargo.lock"
              exit 0
            fi
            if [ ! -f Cargo.toml ]; then
              echo "error: no Cargo.toml in $PWD — run this from the repo root" >&2
              exit 1
            fi
            cargo generate-lockfile
          '';
        };
      in
      rec {
        packages = {
          inherit ml-kem ml-dsa;
        };
        apps = {
          ml-kem-extract = {
            type = "app";
            program = "${ml-kem-extract-app}/bin/ml-kem-extract";
          };
          ml-dsa-extract = {
            type = "app";
            program = "${ml-dsa-extract-app}/bin/ml-dsa-extract";
          };
          combined-extract = {
            type = "app";
            program = "${combined-extract-app}/bin/combined-extract";
          };
          gen-lockfile = {
            type = "app";
            program = "${gen-lockfile-app}/bin/gen-lockfile";
          };
        };
        devShells.default = craneLib.devShell (
          tools-environment
          // {
            # Build inputs are listed inline rather than pulled in via
            # `inputsFrom = [ packages.ml-kem ]`. That reference would force
            # crane to realize `cargoArtifacts` (and therefore Cargo.lock)
            # just to enter the shell, which breaks a fresh checkout where
            # the lockfile is generated by the shellHook below.
            packages = [
              pkgs.clang_18
              pkgs.openssl
              pkgs.pkg-config
              pkgs.jq
              pkgs.valgrind
              pkgs.libclang
              rustToolchain
              cargoMiri
              hax.packages.${system}.default
              fstar.packages.${system}.default
              pkgs.qemu
              pkgs.llvmPackages_18.clang-tools
              clang-format-18-wrapper
              pkgs.cmake
              pkgs.mold-wrapped
              pkgs.ninja
              pkgs.git
              pkgs.python3
              pkgs.perl
            ];
            RUST_SRC_PATH = "${rustToolchain.outPath}/lib/rustlib/src/rust/library";
            LIBCLANG_PATH = "${pkgs.llvmPackages_18.libclang.lib}/lib";
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc";
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";
            CC_aarch64_unknown_linux_gnu = "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc";
            shellHook = ''
              if [ ! -f Cargo.lock ] && [ -f Cargo.toml ]; then
                echo "[flake] Cargo.lock missing — running 'cargo generate-lockfile'..."
                cargo generate-lockfile
              fi
            '';
          }
        );
        # `nix develop --impure .#fuzz`, then e.g.
        #   cd crates/algorithms/hmac-drbg && cargo fuzz run generate
        # clangStdenv so that libfuzzer-sys builds its bundled libFuzzer C++
        # sources with clang rather than the default gcc.
        devShells.fuzz = (pkgs.mkShell.override { stdenv = pkgs.clangStdenv; }) {
          packages = [
            rustNightlyForFuzz
            pkgs.cargo-fuzz
            pkgs.pkg-config
            pkgs.git
          ];
          RUST_SRC_PATH = "${rustNightlyForFuzz}/lib/rustlib/src/rust/library";
          ASAN_SYMBOLIZER_PATH = "${pkgs.llvm_18}/bin/llvm-symbolizer";
          # `cargo fuzz` builds with the release profile, whose workspace
          # settings are hostile to fuzzing: fat LTO drops the sanitizer
          # instrumentation, and stripping loses the symbols needed to read a
          # crash report. (CI works around the former the same way, see
          # CARGO_PROFILE_RELEASE_LTO in .github/workflows/aes.yml.)
          # See https://github.com/rust-fuzz/cargo-fuzz/issues/384
          CARGO_PROFILE_RELEASE_LTO = "false";
          CARGO_PROFILE_RELEASE_STRIP = "false";
          CARGO_PROFILE_RELEASE_DEBUG = "1";
          shellHook = ''
            root=$(git rev-parse --show-toplevel 2>/dev/null || echo .)
            if [ ! -f "$root/Cargo.lock" ] && [ -f "$root/Cargo.toml" ]; then
              echo "[flake] Cargo.lock missing — running 'cargo generate-lockfile'..."
              (cd "$root" && cargo generate-lockfile)
            fi
            echo "[flake] fuzz shell: $(rustc --version), $(cargo fuzz --version)"
          '';
        };
      }
    );
}
