#! /usr/bin/env python3
"""Canonical F* extraction for the `libcrux-platform` crate.

This is the SINGLE source of truth for how `crates/sys/platform` is extracted
to F*.  The three algorithm scripts (libcrux-ml-kem/hax.py,
libcrux-ml-dsa/hax.sh, crates/algorithms/sha3/hax.sh) call this script instead
of inlining their own `cargo hax` invocation, so the shared
`crates/sys/platform/proofs/fstar/extraction` tree can never flip-flop between
per-algorithm configs (the extraction-contamination bug —
feedback_shared_coremodels_extraction_contamination).

The platform crate's extracted content is INVARIANT to the per-algorithm
`--cfg pre_core_models` flag (empirically verified 2026-07-28), so a single
canonical config serves all consumers.  The interface (`.fsti`) is emitted for
every module (`--interfaces "+**"`); the CPUID init helpers are dropped because
they extract as unmodellable inline asm.

Idempotent: re-extraction is skipped when the sentinel module is already
present, unless `--force` is passed (or `HAX_FORCE=1` is set).  A multi-crate
build therefore extracts this uniform dep at most once.  Trade-off: a skip does
NOT re-extract after the crate's `src/` changes — pass `--force` (or delete the
extraction dir) after editing the Rust source.
"""

import argparse
import os
import subprocess
import sys
import time

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
EXTRACTION_DIR = os.path.join(SCRIPT_DIR, "proofs", "fstar", "extraction")

# The canonical `cargo hax into ... fstar ...` argument vector for this crate.
# `-i` (include namespaces) and `--output-dir` are `into` options; everything
# after `fstar` is the backend's.  We do NOT pass `--z3rlimit` here: the
# platform module is trivial and verifies at F*'s default rlimit (15); the
# historical `--z3rlimit 80` on some callers only diverged the module header.
CARGO_HAX = [
    "cargo",
    "hax",
    "into",
    "-i",
    "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count",
    "fstar",
    "--interfaces",
    "+**",
]

# Presence of this module ⇒ the crate is already extracted (skip unless --force).
SENTINEL = "Libcrux_platform.Platform.fst"


def clean_generated_fstar(directory):
    """Remove generated `.fst`/`.fsti` BEFORE re-extracting.  hax extracts
    incrementally (unchanged modules keep their old files) and NEVER deletes a
    `.fsti` when a module stops emitting an interface — a leftover `.fsti` then
    silently SHADOWS the fresh `.fst` (the stale-.fsti contamination that broke
    the SHA-3 SIMD proofs).  A clean-then-extract guarantees the dir holds
    exactly what the current config produces.  This dir contains no hand-written
    `.fst`/`.fsti` (only a tracked `Makefile`), so removing all is safe."""
    if not os.path.isdir(directory):
        return
    import glob
    for f in glob.glob(os.path.join(directory, "*.fst")) + glob.glob(os.path.join(directory, "*.fsti")):
        os.remove(f)


def extract(force=False):
    force = force or os.environ.get("HAX_FORCE") == "1"
    sentinel_path = os.path.join(EXTRACTION_DIR, SENTINEL)
    if os.path.exists(sentinel_path) and not force:
        print(f"[platform/hax.py] already extracted ({SENTINEL} present); skipping. "
              f"Use --force to re-extract.")
        return

    print(f"[platform/hax.py] extracting -> {EXTRACTION_DIR}")
    print("  Command:", " ".join(CARGO_HAX))
    clean_generated_fstar(EXTRACTION_DIR)
    subprocess.run(CARGO_HAX, cwd=SCRIPT_DIR, check=True)
    print("[platform/hax.py] done")


def main():
    sys.tracebacklimit = 0
    parser = argparse.ArgumentParser(description="Canonical F* extraction for libcrux-platform.")
    sub = parser.add_subparsers(dest="command")
    ep = sub.add_parser("extract", help="Extract the F* code for the proofs.")
    ep.add_argument("--force", action="store_true", help="Re-extract even if already present.")
    if len(sys.argv) == 1:
        parser.print_help(sys.stderr)
        sys.exit(1)
    args = parser.parse_args()
    if args.command == "extract":
        extract(force=args.force)


if __name__ == "__main__":
    main()
