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

    # Copy the environment rather than aliasing os.environ: `os_env.update(env)`
    # would otherwise mutate the real process env in place, so a per-crate flag
    # like `--cfg pre_core_models` (set for the intrinsics extraction) would leak
    # into every subsequent extraction in this same process.
    os_env = dict(os.environ)
    os_env.update(env)

    ret = subprocess.run(command, cwd=cwd, env=os_env)
    if ret.returncode != expect:
        raise Exception("Error {}. Expected {}.".format(ret, expect))


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, ".."))
ML_KEM_EXTRACTION_DIR = os.path.join(SCRIPT_DIR, "proofs", "fstar", "extraction")
# ml-kem carries its OWN copy of the shared intrinsics modules
# (Libcrux_intrinsics.{Avx2_extract,Arm64_extract}), extracted below via
# `--output-dir` into this DEDICATED subdir (not the main `extraction` tree).
# Rationale: FINDLIBS (Makefile.generic) auto-includes EVERY workspace crate's
# `proofs/fstar/extraction` on EVERY other crate's path, so putting the local
# intrinsics copy in `extraction/` would make ml-kem's / ml-dsa's / sha3's
# copies collide on each other's include path (F* silently picks the
# alphabetically-last one → wrong variant).  A `proofs/fstar/intrinsics` sibling
# dir is NOT auto-discovered by FINDLIBS (it only looks at `.../extraction`), so
# it is added to ONLY ml-kem's own include path via FSTAR_INCLUDE_DIRS_EXTRA
# (`../intrinsics`) — exactly like the existing `../spec` / `../commute` dirs.
# The shared crates/utils/intrinsics tree is likewise excluded from the include
# path in Makefile.generic, so this local copy is the one that is used.
ML_KEM_INTRINSICS_DIR = os.path.join(SCRIPT_DIR, "proofs", "fstar", "intrinsics")


def run_dep_extract(rel_script):
    """Invoke a canonical per-dependency `hax.py extract` (single source of
    truth for that uniform shared dep; idempotent — skips if already
    extracted).  Keeps the shared platform/secrets trees from flip-flopping
    between per-algorithm configs."""
    script = os.path.join(REPO_ROOT, rel_script)
    print(f"[ml-kem/hax.py] -> {rel_script} extract")
    subprocess.run([sys.executable, script, "extract"], check=True)


def clean_generated_fstar(directory):
    """Remove generated `.fst`/`.fsti` from an extraction dir BEFORE re-extracting.
    hax extracts incrementally (unchanged modules keep their old files) and NEVER
    deletes a `.fsti` when a module stops emitting an interface — a leftover
    `.fsti` then silently SHADOWS the fresh `.fst` (the stale-.fsti contamination
    that broke the SHA-3 SIMD proofs).  A clean-then-extract guarantees the dir
    holds exactly what the current config produces.  These dirs contain no
    hand-written `.fst`/`.fsti` (only a tracked `Makefile`), so this is safe."""
    if not os.path.isdir(directory):
        return
    import glob
    for f in glob.glob(os.path.join(directory, "*.fst")) + glob.glob(os.path.join(directory, "*.fsti")):
        os.remove(f)


class extractAction(argparse.Action):

    def __call__(self, parser, args, values, option_string=None) -> None:
        # Extract the uniform shared platform dep via its canonical script
        # (single source of truth; idempotent).  platform stays in its own
        # crate dir and is auto-included by Makefile.generic's dependencies().
        run_dep_extract("crates/sys/platform/hax.py")

        # MIGRATION (2026-07-28): ml-kem AVX2 now rests on the tested core-models
        # intrinsics (mirror ml-dsa hax.sh).  Extract the core-models crate so its
        # `Libcrux_core_models.*` modules are on the shared include path; the real
        # `Libcrux_intrinsics.Avx2` op bodies route through them.  Idempotent.
        run_dep_extract("crates/utils/core-models/hax.py")

        # Extract intrinsics into ml-kem's OWN extraction dir (--output-dir), so
        # the shared crates/utils/intrinsics tree is never clobbered by ml-kem's
        # `pre_core_models` config (which routes avx2 -> Avx2_extract, the
        # bit_vec stub — the cross-crate flip vs ml-dsa's real Avx2).  We exclude
        # `libcrux_core_models::**`: under pre_core_models ml-kem references ZERO
        # `Libcrux_core_models.*` (it uses the hax `Core_models` proof-lib +
        # BitVec.Intrinsics), so `+:**` would only emit vestigial core-models
        # signature modules that, as roots in this dir, would COLLIDE with the
        # core-models crate's extraction tree (Error 72 — the shared-core-models
        # contamination this refactor eliminates).
        # MIGRATION (2026-07-28): non-pcm core-models path (mirror ml-dsa hax.sh's
        # intrinsics step).  Exclude re-EMITTING the core-models modules (they are
        # provided by the `dep_extract` above from the shared `../` dir — emitting
        # them here as roots would collide, Error 72).  Drop `--cfg pre_core_models`
        # so lib.rs routes avx2 -> the REAL `Libcrux_intrinsics.Avx2` (core-models),
        # not the `Avx2_extract` bit_vec stub.
        #
        # `--interfaces "-**"`: everything extracts TRANSPARENT (no `.fsti`).
        # Avx2 (migrated to core-models) MUST stay transparent — its consumers
        # (the Spec.Avx2Lanes companion) need the `.fst` op BODIES so
        # `reveal_opaque` can unfold `mm256_OP = e_mm256_OP`; a `.fsti` would hide
        # those bodies and regress the (green) AVX2 cone.
        # (Historically `-** +libcrux_intrinsics::arm64_extract::**`, to give the
        # hand-written `arm64_extract.rs` bit_vec stub its own `.fsti`.  That stub
        # was DELETED in the core-models migration, so the allowlist is vestigial
        # — plain "-**", matching ml-dsa hax.sh.)
        include_str = "-libcrux_core_models::**"
        interfaces_str = "-**"
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
            "--output-dir",
            ML_KEM_INTRINSICS_DIR,
            "fstar",
            "--z3rlimit",
            "80",
            "--interfaces",
            interfaces_str,
        ]
        hax_env = {}
        # Force a rebuild of the intrinsics crate (touch its sources) so the
        # pre_core_models variant is regenerated even if a prior extraction in
        # this working tree built it under a DIFFERENT config (e.g. ml-dsa's
        # non-pcm real `Avx2`): hax reuses the cached THIR when cargo thinks the
        # crate is fresh, so without this touch ml-kem can silently pick up
        # ml-dsa's `Avx2.fst` instead of its own `Avx2_extract.fst` (the
        # cross-crate cargo-freshness flip). Harmless in single-crate CI.
        import glob as _glob
        for _src in _glob.glob(os.path.join(REPO_ROOT, "crates/utils/intrinsics/src/*.rs")):
            os.utime(_src, None)
        clean_generated_fstar(ML_KEM_INTRINSICS_DIR)
        shell(
            cargo_hax_into,
            cwd=os.path.join(REPO_ROOT, "crates/utils/intrinsics"),
            env=hax_env,
        )

        # Extract the uniform shared secrets dep via its canonical script
        # (transparent `--interfaces "-**"`; single source of truth; idempotent).
        run_dep_extract("crates/utils/secrets/hax.py")

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
            cwd=os.path.join(REPO_ROOT, "specs/ml-kem"),
            env=hax_env,
        )

        # Extract ml-kem
        includes = [
            "+**",
            "-libcrux_ml_kem::kem::**",
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
        # G3 module-trust mirror (annotation_lint V6 / trust_ledger --check): every
        # `-libcrux_ml_kem::…` module dropped from F* extraction above carries a
        # machine-readable reason here (an absent module is worse than an admitted
        # one). Reasons use the shared category vocabulary (reason_ok); the bijection
        # {exclusion tokens} == {annotations} is enforced by the V6 lint.
        # trusted-module: -libcrux_ml_kem::kem::** : hax-limitation: top-level kem API/dispatch glue, not extracted (verified via the generic + incremental paths)
        # trusted-module: -libcrux_ml_kem::ind_cca::incremental::**::as_keypair : hax-limitation: runtime-dispatch helper (Box<dyn Keys> / &dyn Any) has no F* model
        # trusted-module: -libcrux_ml_kem::ind_cca::incremental::**::as_state : hax-limitation: runtime-dispatch helper (Box<dyn Keys> / &dyn Any) has no F* model
        # trusted-module: -libcrux_ml_kem::ind_cca::incremental::multiplexing::alloc::** : hax-limitation: alloc/runtime-dispatch submodule (Box<dyn> / dyn Any) has no F* model
        # trusted-module: -libcrux_ml_kem::ind_cca::incremental::types::alloc::** : hax-limitation: alloc/runtime-dispatch submodule (Box<dyn> / dyn Any) has no F* model
        # trusted-module: -libcrux_ml_kem::mlkem512::incremental::alloc::** : hax-limitation: alloc/runtime-dispatch submodule (Box<dyn> / dyn Any) has no F* model
        # trusted-module: -libcrux_ml_kem::mlkem768::incremental::alloc::** : hax-limitation: alloc/runtime-dispatch submodule (Box<dyn> / dyn Any) has no F* model
        # trusted-module: -libcrux_ml_kem::mlkem1024::incremental::alloc::** : hax-limitation: alloc/runtime-dispatch submodule (Box<dyn> / dyn Any) has no F* model
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
        # MIGRATION (2026-07-28): ml-kem AVX2 now rests on the core-models
        # intrinsics (mirror ml-dsa).  Dropping `--cfg pre_core_models` makes
        # lib.rs route avx2 -> the real `Libcrux_intrinsics.Avx2`, so ml-kem's
        # AVX2 contracts cite `Avx2` (+ the Spec.Avx2Lanes lane-view companion)
        # instead of the `Avx2_extract` bit_vec stub.  NEON/Portable unaffected
        # (pre_core_models only gates the avx2 module in lib.rs).
        hax_env = {}
        clean_generated_fstar(ML_KEM_EXTRACTION_DIR)
        shell(
            cargo_hax_into,
            cwd=SCRIPT_DIR,
            env=hax_env,
        )

        # Apply post-extraction patches
        import glob
        patches = sorted(glob.glob(os.path.join(SCRIPT_DIR, "proofs/fstar/extraction-patches/*.patch")))
        for patch in patches:
            print(f"\nApplying patch: {patch}")
            shell(["git", "apply", patch], cwd=SCRIPT_DIR)

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
            f = os.path.join(SCRIPT_DIR, f)
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

        # Always run `make` in ml-kem's OWN extraction dir (absolute path), never
        # the process CWD.  This previously used `-C proofs/fstar/extraction/`
        # relative to wherever the script was invoked from, so running
        # `python3 libcrux-ml-kem/hax.py prove` from the repo root built the
        # (near-empty) repo-root `proofs/fstar/extraction` tree and reported
        # "Checked: 0".  The log likewise lands next to the crate (as ml-dsa's
        # hax.sh does), not in the caller's CWD.
        output_file = os.path.join(SCRIPT_DIR, "verification_result.txt")
        os_env = os.environ.copy()
        os_env.update(admit_env)

        # Parallel-job count: honor a `JOBS` env var (like the shell drivers),
        # defaulting to 4.
        jobs = os.environ.get("JOBS", "4")

        print(f"Running F* verification (output saved to {output_file})...")
        print()

        with open(output_file, "w") as f:
            proc = subprocess.Popen(
                ["make", "-k", f"-j{jobs}"],
                cwd=ML_KEM_EXTRACTION_DIR,
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
