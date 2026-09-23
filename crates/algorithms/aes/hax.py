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


# Items of `libcrux-aes` that the Lean backend can model today, in the order
# the Lean proofs need them. The list is explicit rather than a wildcard over
# `platform::portable::aes_core` because three items in that module cannot be
# extracted yet: `transpose_u16x8`, and the `AESState` implementation's
# `store_block` and `xor_block`. All three write through a `&mut [u8]` slice,
# for which hax emits an array update that the Lean prelude does not define.
# See proofs/lean/README.md.
LEAN_AES_CORE = "libcrux_aes::platform::portable::aes_core"
LEAN_ITEMS = [
    "new_state",
    "interleave_u8_1",
    "deinterleave_u8_1",
    "interleave_u16_2",
    "interleave_u16_4",
    "interleave_u16_8",
    "transpose_u8x16",
    "xnor",
    "sub_bytes_state",
    "shift_row_u16",
    "shift_rows_state",
    "mix_columns_state",
    "xor_key1_state",
    "aes_enc",
    "aes_enc_last",
    "aes_keygen_assisti",
    "aes_keygen_assist",
    "aes_keygen_assist0",
    "aes_keygen_assist1",
    "key_expand1",
    "key_expansion_step",
]


def lean_include():
    parts = ["-**", "+libcrux_aes::aes::**"]
    parts += ["+{}::{}".format(LEAN_AES_CORE, item) for item in LEAN_ITEMS]
    return " ".join(parts)


class extractAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        if args.target == "lean":
            # The portable AES implementation uses no intrinsics, so the
            # dependency crates that the F* flow extracts are not needed.
            shell(
                ["cargo", "hax", "into", "-i", lean_include(), "lean"],
                cwd=".",
                env={},
            )
            return None

        # Extract platform interfaces
        include_str = "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count"
        interface_include = "+**"
        target = "fstar"
        if args.target is not None:
            target = args.target

        def fstar_interfaces(args):
            if target == "fstar":
                return ["--interfaces", args]
            return []

        cargo_hax_into = [
            "cargo",
            "hax",
            "into",
            "-i",
            include_str,
            target,
        ]
        cargo_hax_into.extend(fstar_interfaces(interface_include))
        hax_env = {}
        shell(
            cargo_hax_into,
            cwd="../../sys/platform",
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
            target,
        ]
        cargo_hax_into.extend(fstar_interfaces(interface_include))
        hax_env = {"RUSTFLAGS": "--cfg pre_core_models"}
        shell(
            cargo_hax_into,
            cwd="../../utils/intrinsics",
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
            target,
        ]
        hax_env = {}
        shell(
            cargo_hax_into,
            cwd="../../utils/secrets",
            env=hax_env,
        )

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
        if args.target == "lean":
            shell(["lake", "build"], cwd="proofs/lean", env={})
            return None
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
    prover_parser.add_argument("--target", help="The target language to check.")
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
