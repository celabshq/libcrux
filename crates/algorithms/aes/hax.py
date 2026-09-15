#! /usr/bin/env python3

import os
import argparse
import subprocess
import sys
import glob


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
# crates/algorithms/aes -> crates/algorithms -> crates -> repo root
REPO_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, "..", "..", ".."))
AES_EXTRACTION_DIR = os.path.join(SCRIPT_DIR, "proofs", "fstar", "extraction")
# aes carries its OWN copy of the shared intrinsics modules
# (Libcrux_intrinsics.{Avx2,Arm64}), extracted below via `--output-dir` into
# this DEDICATED subdir (not the main `extraction` tree).  FINDLIBS
# (Makefile.generic) auto-includes EVERY workspace crate's
# `proofs/fstar/extraction` on EVERY other crate's path, so putting the local
# intrinsics copy in `extraction/` would make aes's / ml-kem's / ml-dsa's /
# sha3's copies collide on each other's include path.  A
# `proofs/fstar/intrinsics` sibling dir is NOT auto-discovered by FINDLIBS, so
# it is added to ONLY aes's own include path via the extraction Makefile's
# FSTAR_INCLUDE_DIRS_EXTRA (`../intrinsics`).
AES_INTRINSICS_DIR = os.path.join(SCRIPT_DIR, "proofs", "fstar", "intrinsics")


def shell(command, expect=0, cwd=None, env={}):
    print("Env:", env)
    print("Command: ", end="")
    for i, word in enumerate(command):
        if i == 4:
            print("'{}' ".format(word), end="")
        else:
            print("{} ".format(word), end="")

    print("\nDirectory: {}".format(cwd))

    # Copy the environment rather than aliasing os.environ so a per-crate flag
    # never leaks into a subsequent extraction in this same process.
    os_env = dict(os.environ)
    os_env.update(env)

    ret = subprocess.run(command, cwd=cwd, env=os_env)
    if ret.returncode != expect:
        raise Exception("Error {}. Expected {}.".format(ret, expect))


def run_dep_extract(rel_script):
    """Invoke a canonical per-dependency `hax.py extract` (single source of
    truth for that uniform shared dep; idempotent).  Keeps the shared
    platform/core-models/secrets trees from flip-flopping between per-algorithm
    configs."""
    script = os.path.join(REPO_ROOT, rel_script)
    print(f"[aes/hax.py] -> {rel_script} extract")
    subprocess.run([sys.executable, script, "extract"], check=True)


def clean_generated_fstar(directory):
    """Remove generated `.fst`/`.fsti` from an extraction dir BEFORE
    re-extracting.  hax extracts incrementally and NEVER deletes a `.fsti` when
    a module stops emitting one — a leftover `.fsti` then silently SHADOWS the
    fresh `.fst`.  These dirs hold only generated files (a tracked `.gitignore`
    / `Makefile` aside), so removing all `.fst`/`.fsti` is safe."""
    if not os.path.isdir(directory):
        return
    for f in glob.glob(os.path.join(directory, "*.fst")) + glob.glob(
        os.path.join(directory, "*.fsti")
    ):
        os.remove(f)


class extractAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        target = "fstar"
        if args.target is not None:
            target = args.target

        def fstar_interfaces(interfaces):
            if target == "fstar":
                return ["--interfaces", interfaces]
            return []

        # Shared platform dep via its canonical script (single source of truth;
        # idempotent).  platform stays in its own crate dir and is auto-included
        # by Makefile.generic's dependencies().
        run_dep_extract("crates/sys/platform/hax.py")

        # core-models flip (WS B): aes now rests on the differentially-tested
        # core-models intrinsics (BOTH backends).  Extract the core-models crate
        # so its `Libcrux_core_models.*` modules are on the include path; the
        # real `Libcrux_intrinsics.{Avx2,Arm64}` op bodies route through them.
        run_dep_extract("crates/utils/core-models/hax.py")

        # Extract intrinsics into aes's OWN dedicated intrinsics dir
        # (--output-dir), so the shared crates/utils/intrinsics tree is never
        # clobbered.  Dropping `--cfg pre_core_models` makes lib.rs route BOTH
        # backends to the REAL core-models `Libcrux_intrinsics.{Avx2,Arm64}`
        # (not the `{avx2,arm64}_extract` bit_vec stubs).  We exclude
        # re-EMITTING the core-models modules (provided by the dep_extract above;
        # emitting them here as roots would collide, Error 72).  `--interfaces
        # "-**"` keeps BOTH real modules TRANSPARENT (mirror sha3): aes has no
        # proofs that need to `reveal` a wrapper body, but transparency is the
        # known-good setting for the real modules (their consumers just see the
        # op signatures + `ensures`, which is all panic-free typechecking needs).
        intr_include = "-libcrux_core_models::**"
        intr_interfaces = "-**"
        cargo_hax_into = [
            "cargo",
            "hax",
            "-C",
            "--features",
            "simd128,simd256",
            ";",
            "into",
            "-i",
            intr_include,
            "--output-dir",
            AES_INTRINSICS_DIR,
            target,
        ]
        if target == "fstar":
            cargo_hax_into.extend(["--z3rlimit", "80"])
        cargo_hax_into.extend(fstar_interfaces(intr_interfaces))
        # Force a rebuild of the intrinsics crate (touch its sources) so the
        # non-pcm variant is regenerated even if a prior extraction in this
        # working tree built it under a DIFFERENT config: hax reuses the cached
        # THIR when cargo thinks the crate is fresh, so without this touch aes
        # can silently pick up another crate's `Avx2_extract.fst`/`Arm64_extract.fst`
        # (the cross-crate cargo-freshness flip).
        for _src in glob.glob(os.path.join(REPO_ROOT, "crates/utils/intrinsics/src/*.rs")):
            os.utime(_src, None)
        clean_generated_fstar(AES_INTRINSICS_DIR)
        shell(
            cargo_hax_into,
            cwd=os.path.join(REPO_ROOT, "crates/utils/intrinsics"),
            env={},
        )

        # Shared secrets dep via its canonical script (idempotent).
        run_dep_extract("crates/utils/secrets/hax.py")

        # Extract libcrux-aes
        includes = [
            "+**",
            "-libcrux_aes::traits_api::**",
        ]
        include_str = " ".join(includes)
        interface_include = "+**"
        cargo_hax_into = [
            "cargo",
            "hax",
            "-C",
            "--features",
            "simd128,simd256",
            ";",
            "into",
            "-i",
            include_str,
            target,
        ]
        if target == "fstar":
            cargo_hax_into.extend(["--z3rlimit", "80"])
        cargo_hax_into.extend(fstar_interfaces(interface_include))
        clean_generated_fstar(AES_EXTRACTION_DIR)
        shell(
            cargo_hax_into,
            cwd=SCRIPT_DIR,
            env={},
        )
        return None


class proveAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        admit_env = {}
        if args.admit:
            admit_env = {"OTHERFLAGS": "--admit_smt_queries true"}
        shell(["make", "-j4", "-C", "proofs/fstar/extraction/"], env=admit_env)
        return None


def parse_arguments():
    parser = argparse.ArgumentParser(
        description="Libcrux prove script. "
        + "Make sure to separate sub-command arguments with --."
    )
    subparsers = parser.add_subparsers()

    extract_parser = subparsers.add_parser(
        "extract", help="Extract the F* code for the proofs."
    )
    extract_parser.add_argument("extract", nargs="*", action=extractAction)
    extract_parser.add_argument("--target", help="The target language to extract.")

    prover_parser = subparsers.add_parser(
        "prove",
        help="""
        Run F*.

        This typechecks the extracted code.
        To lax-typecheck use --admit.
        """,
    )
    prover_parser.add_argument(
        "--admit",
        help="Admit all smt queries to lax typecheck.",
        action="store_true",
    )
    prover_parser.add_argument(
        "prove",
        nargs="*",
        action=proveAction,
    )

    if len(sys.argv) == 1:
        parser.print_help(sys.stderr)
        sys.exit(1)

    return parser.parse_args()


def main():
    # Don't print unnecessary Python stack traces.
    sys.tracebacklimit = 0
    parse_arguments()


if __name__ == "__main__":
    main()
