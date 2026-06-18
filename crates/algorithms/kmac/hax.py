#! /usr/bin/env python3

import os
import argparse
import subprocess
import sys
from glob import glob

def shell(command, expect=0, cwd=None, env={}):
    subprocess_stdout = subprocess.DEVNULL

    print("Env:", env)
    print("Command: ", end="")
    for i, word in enumerate(command):
        if i == 4:
            print("'{}' ".format(word), end="")
        else:
            print("{} ".format(word), end="")

    print("\nDirectory: {}".format(cwd))

    os_env = os.environ
    os_env.update(env)

    ret = subprocess.run(command, cwd=cwd, env=os_env)
    if ret.returncode != expect:
        raise Exception("Error {}. Expected {}.".format(ret, expect))

KMAC = os.path.dirname(os.path.abspath(__file__))
PLATFORM = os.path.normpath(os.path.join(KMAC, "../../sys/platform"))
CORE_MODELS = os.path.normpath(os.path.join(KMAC, "../../utils/core-models"))
INTRINSICS = os.path.normpath(os.path.join(KMAC, "../../utils/intrinsics"))
SECRETS = os.path.normpath(os.path.join(KMAC, "../../utils/secrets"))
SHA3 = os.path.normpath(os.path.join(KMAC, "../sha3"))


def replace_in_extraction(crate_dir, replacements):
    """Apply `(old, new)` string replacements to every .fst/.fsti file in
    `crate_dir`'s extraction directory (the Python equivalent of the `sed`
    calls in crates/algorithms/sha3/hax.sh)."""
    extraction_dir = os.path.join(crate_dir, "proofs", "fstar", "extraction")
    files = glob(os.path.join(extraction_dir, "*.fst")) + glob(
        os.path.join(extraction_dir, "*.fsti")
    )
    for path in files:
        with open(path) as f:
            content = f.read()
        updated = content
        for old, new in replacements:
            updated = updated.replace(old, new)
        if updated != content:
            with open(path, "w") as f:
                f.write(updated)


def rename_core_models_uses(crate_dir):
    """Mirror sha3/hax.sh: any extracted crate may refer to core-models under
    the `Core_models.*` module path; rewrite those references to the
    `Libcrux_core_models.*` modules produced by rename_core_models_files."""
    replace_in_extraction(
        crate_dir,
        [
            ("Core_models.Abstractions", "Libcrux_core_models.Abstractions"),
            ("Core_models.Core_arch", "Libcrux_core_models.Core_arch"),
        ],
    )


def rename_core_models_files(crate_dir):
    """Mirror sha3/hax.sh: rename the core-models crate's own modules from
    `Core_models*` to `Libcrux_core_models*` (both file names and the
    `module ...` headers inside them)."""
    extraction_dir = os.path.join(crate_dir, "proofs", "fstar", "extraction")
    for path in glob(os.path.join(extraction_dir, "Core_models*")):
        dir_path = os.path.dirname(path)
        filename = os.path.basename(path)
        new_filename = "Libcrux_core_models" + filename[len("Core_models"):]
        os.rename(path, os.path.join(dir_path, new_filename))
    replace_in_extraction(
        crate_dir, [("module Core_models", "module Libcrux_core_models")]
    )


def hax_extract(cwd, hax_args):
    """Run `cargo hax <hax_args>` in `cwd` and rewrite core-models uses in the
    resulting extraction, exactly like the `extract` helper in sha3/hax.sh."""
    shell(["cargo", "hax"] + hax_args, cwd=cwd, env={})
    rename_core_models_uses(cwd)


class extractAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        # XXX The order of these extractions is relevant. Ideally, hax would be able
        # to just extract a crate and its dependents, but that doesn't seem to always
        # work (only sometimes...). You must also take care to not extract a crate which
        # has dependents before the dependents, as they will otherwise not be extracted
        # properly due to seemingly a caching bug in hax.

        # --- platform --------------------------------------------------------
        hax_extract(
            PLATFORM,
            [
                "into",
                "-i", "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count",
                "fstar", "--z3rlimit", "80", "--interfaces", "+**",
            ],
        )

        # --- core-models -----------------------------------------------------
        hax_extract(CORE_MODELS, ["into", "fstar"])
        rename_core_models_files(CORE_MODELS)

        # --- intrinsics ------------------------------------------------------
        hax_extract(
            INTRINSICS,
            [
                "into",
                "-i", "-core_models::**",
                "fstar", "--z3rlimit", "80", "--interfaces", "+**",
            ],
        )

        # --- secrets ---------------------------------------------------------
        hax_extract(
            SECRETS,
            ["into", "-i", "+**", "fstar", "--z3rlimit", "80"],
        )

        # --- sha3 ------------------------------------------------------------
        # libcrux_sha3::portable must stay transparent (no F* interface) — see
        # the comment in crates/algorithms/sha3/hax.sh for why.
        hax_extract(
            SHA3,
            [
                "into",
                "-i", "+**",
                "-i", "-**::avx2::**",
                "-i", "-**::arm64::**",
                "-i", "-**::neon::**",
                "-i", "-**::simd128::**",
                "-i", "-**::simd256::**",
                "fstar", "--z3rlimit", "80",
                "--interfaces",
                "+** -**::generic_keccak::constants::** "
                "-**::proof_utils::** -libcrux_sha3::portable::**",
            ],
        )

        hax_extract(
            KMAC,
            [
                "into",
                "-i", "+libcrux_kmac::**",
                "fstar",
                "--z3rlimit", "80",
            ],
        )

        return None


class proveAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        admit_env = {}
        if args.admit:
            admit_env = {"OTHERFLAGS": "--admit_smt_queries true"}
        shell(
            ["make", "-j4", "-C", os.path.join(KMAC, "proofs/fstar/extraction/")],
            env=admit_env,
        )
        return None

class cleanAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        for crate_dir in [KMAC, PLATFORM, CORE_MODELS, INTRINSICS, SECRETS, SHA3]:
            extraction_dir = os.path.join(crate_dir, "proofs/fstar/extraction")
            files = glob(os.path.join(extraction_dir, "*.fst")) + glob(
                os.path.join(extraction_dir, "*.fsti")
            )
            if files:
                shell(["rm"] + files)
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

    clean_parser = subparsers.add_parser(
        "clean", help="Remove generated F* code for this crate."
    )
    clean_parser.add_argument("clean", nargs="*", action=cleanAction)    
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
