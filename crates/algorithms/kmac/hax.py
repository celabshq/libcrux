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
REPO_ROOT = os.path.normpath(os.path.join(KMAC, "../../.."))
SHA3 = os.path.normpath(os.path.join(KMAC, "../sha3"))


def run_dep_extract(rel_script):
    """Invoke a canonical per-dependency `hax.py extract` (single source of
    truth for that uniform shared dep; idempotent — skips if already
    extracted).  Keeps the shared platform/core-models/secrets trees from
    flip-flopping between per-algorithm configs."""
    script = os.path.join(REPO_ROOT, rel_script)
    print(f"[kmac/hax.py] -> {rel_script} extract")
    subprocess.run([sys.executable, script, "extract"], check=True)


def run_sha3_extract():
    """Invoke sha3's own driver.  kmac verifies against
    `Libcrux_sha3.Portable.Incremental.*`, and that tree's single writer is
    `crates/algorithms/sha3/hax.sh` — re-emitting sha3 here under kmac's
    backend flags would make its content depend on which crate extracted
    last."""
    print("[kmac/hax.py] -> crates/algorithms/sha3/hax.sh extract")
    subprocess.run([os.path.join(SHA3, "hax.sh"), "extract"], cwd=SHA3, check=True)


def hax_extract(cwd, hax_args):
    """Run `cargo hax <hax_args>` in `cwd`."""
    shell(["cargo", "hax"] + hax_args, cwd=cwd, env={})


class extractAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        # Every dependency tree kmac verifies against has a canonical owner, and
        # that owner is its only writer.  Re-emitting them here under kmac's
        # backend flags would give each a different `--interfaces`/`--z3rlimit`
        # module header, so the shared trees would depend on which crate
        # extracted last and every cross-crate extraction would invalidate the
        # others' `.checked` for nothing.
        run_dep_extract("crates/sys/platform/hax.py")
        run_dep_extract("crates/utils/core-models/hax.py")
        run_dep_extract("crates/utils/secrets/hax.py")
        run_sha3_extract()

        # kmac's own sweep names only `libcrux_kmac::**`, so it pulls in no
        # dependency module and needs no exclusions.
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
        # Only kmac's own tree: the dependency trees belong to their canonical
        # owners, and clearing them here would delete another driver's output.
        extraction_dir = os.path.join(KMAC, "proofs/fstar/extraction")
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
