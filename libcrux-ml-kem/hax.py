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

        # Extract ml-kem reference spec (hacspec_ml_kem)
        include_str = "+**"
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
            cwd="../specs/ml-kem",
            env=hax_env,
        )

        # Extract ml-kem
        includes = [
            "+**",
            "-libcrux_ml_kem::kem::**",
            "-libcrux_ml_kem::hash_functions::portable::*",
            "-libcrux_ml_kem::hash_functions::avx2::*",
            "-libcrux_ml_kem::hash_functions::neon::*",
            "+:libcrux_ml_kem::hash_functions::*::*",
            # Incremental-API alloc submodules use `Box<dyn Keys>` / `&dyn Any`
            # which hax extracts as F* `dyn`, an unknown identifier.  These are
            # runtime-dispatch helpers and irrelevant for proofs.
            "-libcrux_ml_kem::ind_cca::incremental::**::as_keypair",
            "-libcrux_ml_kem::ind_cca::incremental::**::as_state",
            "-libcrux_ml_kem::ind_cca::incremental::multiplexing::alloc::**",
            "-libcrux_ml_kem::ind_cca::incremental::types::alloc::**",
            "-libcrux_ml_kem::mlkem512::incremental::alloc::**",
            "-libcrux_ml_kem::mlkem768::incremental::alloc::**",
            "-libcrux_ml_kem::mlkem1024::incremental::alloc::**",
        ]
        include_str = " ".join(includes)
        interface_include = "+** -libcrux_ml_kem::vector::traits -libcrux_ml_kem::types -libcrux_ml_kem::constants -libcrux_ml_kem::traits::spec -libcrux_ml_kem::polynomial::spec"
        cargo_hax_into = [
            "cargo",
            "hax",
            "-C",
            "--features",
            "simd128,simd256,incremental",
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

        # Apply post-extraction patches
        import glob
        patches = sorted(glob.glob("proofs/fstar/extraction-patches/*.patch"))
        for patch in patches:
            print(f"\nApplying patch: {patch}")
            shell(["git", "apply", patch], cwd=".")

        # Drop runtime-dispatch alloc-helper modules.  These contain
        # `Box<dyn Keys>` / `&dyn Any` that hax extracts as F* `dyn 1 (...)`,
        # an unknown identifier — and they're irrelevant for proofs.  The
        # `-i` filters on the alloc submodules don't fully prevent this
        # because the parent modules cite them.
        alloc_helpers = [
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem512.Incremental.Alloc.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem512.Incremental.Alloc.fsti",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem768.Incremental.Alloc.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem768.Incremental.Alloc.fsti",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem1024.Incremental.Alloc.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem1024.Incremental.Alloc.fsti",
            "proofs/fstar/extraction/Libcrux_ml_kem.Ind_cca.Incremental.Multiplexing.Alloc.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Ind_cca.Incremental.Multiplexing.Alloc.fsti",
            "proofs/fstar/extraction/Libcrux_ml_kem.Ind_cca.Incremental.Types.Alloc.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Ind_cca.Incremental.Types.Alloc.fsti",
            # The .Incremental.Rand modules call `rng.try_fill_bytes` whose F*
            # model (Rand_core.f_try_fill_bytes) doesn't exist in the hax
            # proof-libs (only `f_fill_bytes` is modeled).  Drop until the lib
            # gains the binding.  The non-incremental .Rand modules use
            # `f_fill_bytes` and extract fine.
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem512.Incremental.Rand.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem512.Incremental.Rand.fsti",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem768.Incremental.Rand.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem768.Incremental.Rand.fsti",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem1024.Incremental.Rand.fst",
            "proofs/fstar/extraction/Libcrux_ml_kem.Mlkem1024.Incremental.Rand.fsti",
        ]
        for f in alloc_helpers:
            if os.path.exists(f):
                os.remove(f)

        return None


class proveAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        import re as regex
        import time

        admit_env = {}
        if args.admit:
            admit_env = {"OTHERFLAGS": "--admit_smt_queries true"}

        output_file = "verification_result.txt"
        os_env = os.environ.copy()
        os_env.update(admit_env)

        print(f"Running F* verification (output saved to {output_file})...")
        print()

        with open(output_file, "w") as f:
            proc = subprocess.Popen(
                ["make", "-k", "-j4", "-C", "proofs/fstar/extraction/"],
                env=os_env,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
            )

            results = {}
            current_module = None
            errors = {}
            start_times = {}

            for line in proc.stdout:
                f.write(line)
                f.flush()

                # Detect [CHECK] or [ADMIT] lines (with ANSI codes stripped)
                clean = regex.sub(r'\x1b\[[0-9;]*m', '', line).strip()

                check_match = regex.match(r'\[(CHECK|ADMIT)\]\s+(\S+)', clean)
                if check_match:
                    kind = check_match.group(1)
                    module = check_match.group(2)
                    current_module = module
                    start_times[module] = time.time()
                    results[module] = {"kind": kind, "status": "running", "time_ms": 0}

                # Detect "Verified" lines
                if "Verified" in line and ("module:" in line or "i'face" in line):
                    verified_match = regex.search(r'(?:Verified\s+(?:module|i\'face \(or impl\+i\'face\)):\s+)(\S+)', clean)
                    if verified_match:
                        mod_name = verified_match.group(1)
                        for m in results:
                            if mod_name in m or m.rstrip('.fst').rstrip('.fsti').replace('.', '_') == mod_name.replace('.', '_'):
                                elapsed = int((time.time() - start_times.get(m, time.time())) * 1000)
                                results[m]["status"] = "ok"
                                results[m]["time_ms"] = elapsed

                # Detect TOTAL TIME lines
                total_match = regex.search(r'TOTAL TIME (\d+) ms', clean)
                if total_match and current_module and results.get(current_module, {}).get("status") == "running":
                    results[current_module]["time_ms"] = int(total_match.group(1))
                    results[current_module]["status"] = "ok"

                # Detect errors
                error_match = regex.match(r'\* Error \d+ at (\S+)', clean)
                if error_match:
                    err_file = error_match.group(1).split('(')[0]
                    errors[err_file] = clean

                # Detect make errors for a module
                make_err = regex.search(r'\*\*\* \[.*?/([^/\s]+)\.checked\]', clean)
                if make_err:
                    mod_file = make_err.group(1)
                    for m in results:
                        if mod_file in m:
                            elapsed = int((time.time() - start_times.get(m, time.time())) * 1000)
                            results[m]["status"] = "FAIL"
                            results[m]["time_ms"] = elapsed

            proc.wait()

        # Print summary
        print()
        print("=" * 70)
        print("  Verification Summary")
        print("=" * 70)

        checked = 0
        admitted = 0
        failed = 0

        for module in sorted(results.keys()):
            r = results[module]
            kind = r["kind"]
            status = r["status"]
            time_ms = r["time_ms"]

            if status == "FAIL":
                tag = "\033[31m[FAILED]\033[0m "
                failed += 1
            elif kind == "ADMIT":
                tag = "\033[33m[Admitted]\033[0m"
                admitted += 1
            else:
                tag = "\033[32m[Checked]\033[0m"
                checked += 1

            print(f"  {tag} {module} ({time_ms} ms)")

        print()
        print(f"  Checked: {checked}  Admitted: {admitted}  Failed: {failed}")

        if errors:
            print()
            print("  Errors:")
            for err_file, err_msg in errors.items():
                print(f"    {err_msg}")

        print("=" * 70)
        print(f"\nFull output saved to {output_file}")

        if failed > 0:
            raise Exception(f"{failed} module(s) failed verification.")

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
