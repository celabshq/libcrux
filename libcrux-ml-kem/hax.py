#! /usr/bin/env python3

import os
import argparse
import subprocess
import sys


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


class extractAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        # Extract platform interfaces
        include_str = "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count"
        interface_include = "+**"
        cargo_hax_into = [
            "cargo",
            "hax",
            "into",
            "-i",
            include_str,
            "fstar",
            "--interfaces",
            interface_include,
        ]
        hax_env = {}
        shell(
            cargo_hax_into,
            cwd="../crates/sys/platform",
            env=hax_env,
        )

        # Extract intrinsics interfaces
        include_str = "+:**"
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
            "fstar",
            "--interfaces",
            interface_include,
        ]
        hax_env = {
            'RUSTFLAGS': "--cfg pre_core_models"
        }
        shell(
            cargo_hax_into,
            cwd="../crates/utils/intrinsics",
            env=hax_env,
        )

        # Extract libcrux-secrets
        include_str = "+**"
        interface_include = ""
        cargo_hax_into = [
            "cargo",
            "hax",
            "into",
            "-i",
            include_str,
            "fstar",
        ]
        hax_env = {}
        shell(
            cargo_hax_into,
            cwd="../crates/utils/secrets",
            env=hax_env,
        )

        # Extract ml-kem
        includes = [
            "+**",
            "-libcrux_ml_kem::types::index_impls::**",
            "-libcrux_ml_kem::kem::**",
            "-libcrux_ml_kem::hash_functions::portable::*",
            "-libcrux_ml_kem::hash_functions::avx2::*",
            "-libcrux_ml_kem::hash_functions::neon::*",
            "+:libcrux_ml_kem::hash_functions::*::*",
        ]
        include_str = " ".join(includes)
        interface_include = "+** -libcrux_ml_kem::vector::traits -libcrux_ml_kem::types -libcrux_ml_kem::constants"
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
            "fstar",
            "--z3rlimit",
            "80",
            "--interfaces",
            interface_include,
        ]
        hax_env = {}
        shell(
            cargo_hax_into,
            cwd=".",
            env=hax_env,
        )
        return None


class proveAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        admit_env = {}
        if args.admit:
            admit_env = {"OTHERFLAGS": "--admit_smt_queries true"}
        os_env = os.environ.copy()
        os_env.update(admit_env)
        # `-k`: keep going past failures so a single run reports every failing
        # module (not just the first). Capture output to print an F* error
        # summary at the end, so failures don't have to be grepped out of the log.
        cmd = ["make", "-k", "-j4", "-C", "proofs/fstar/extraction/"]
        print("Command: {}".format(" ".join(cmd)))
        proc = subprocess.Popen(
            cmd, env=os_env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
        )
        captured = []
        for line in proc.stdout:
            sys.stdout.write(line)
            captured.append(line)
        proc.wait()
        errors = [
            line.rstrip("\n")
            for line in captured
            if "* Error " in line
            or ("*** [" in line and "Error" in line)
            or "failed {reason-unknown" in line
        ]
        if errors:
            print("\n================ F* ERROR SUMMARY ================")
            for line in errors:
                print(line)
            print("================ {} error line(s) ================".format(len(errors)))
        if proc.returncode != 0:
            sys.exit(proc.returncode)
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
