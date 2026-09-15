#! /usr/bin/env python3
"""Canonical F* extraction for the `libcrux-core-models` crate.

Single source of truth for how `crates/utils/core-models` is extracted to F*.
The ml-dsa and sha3 algorithm scripts (which use the new core-models intrinsics
mapping) call this instead of inlining `cargo hax into fstar`, so the shared
`crates/utils/core-models/proofs/fstar/extraction` tree cannot flip-flop.

This same extraction also feeds core-models' OWN standalone F* build
(`proofs/fstar/extraction/Makefile`).

The crate is extracted TRANSPARENTLY (no `--interfaces`, i.e. `.fst`
implementations only, no `.fsti`): consumers must see the concrete definitions
(BitVec models, etc.).  Content is INVARIANT to `--cfg pre_core_models`
(empirically verified 2026-07-28).

Note: `Tactics.Circuits.fst` in the extraction dir is hand-written (git-tracked,
no Rust source) — hax does not regenerate it and the stale-.fsti sweep never
touches a `.fst`, so it is preserved.

Idempotent: skips when the sentinel module is present unless `--force`
(or `HAX_FORCE=1`).  Staleness trade-off: a skip does not re-extract after a
`src/` change — pass `--force`.
"""

import argparse
import os
import subprocess
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
EXTRACTION_DIR = os.path.join(SCRIPT_DIR, "proofs", "fstar", "extraction")

# Transparent extraction: implementations only, no interfaces.
CARGO_HAX = ["cargo", "hax", "into", "fstar"]

SENTINEL = "Libcrux_core_models.Abstractions.Bit.fst"


def clean_stale_fsti(directory):
    """core-models is extracted TRANSPARENTLY (0 `.fsti`).  hax never deletes a
    `.fsti` when a module stops emitting an interface, and a leftover `.fsti`
    silently SHADOWS the fresh `.fst` (the stale-.fsti contamination that broke
    the SHA-3 SIMD proofs).  So any `.fsti` here is stale by construction — remove
    them all.  (We do NOT remove `.fst`: `Tactics.Circuits.fst` is a hand-written,
    git-tracked module; the generated `.fst` are overwritten by re-extraction.)"""
    if not os.path.isdir(directory):
        return
    import glob
    for f in glob.glob(os.path.join(directory, "*.fsti")):
        os.remove(f)
        print(f"  [clean] removed stale .fsti shadow: {os.path.basename(f)}")


def extract(force=False):
    force = force or os.environ.get("HAX_FORCE") == "1"
    sentinel_path = os.path.join(EXTRACTION_DIR, SENTINEL)
    if os.path.exists(sentinel_path) and not force:
        print(f"[core-models/hax.py] already extracted ({SENTINEL} present); skipping. "
              f"Use --force to re-extract.")
        return

    print(f"[core-models/hax.py] extracting -> {EXTRACTION_DIR}")
    print("  Command:", " ".join(CARGO_HAX))
    clean_stale_fsti(EXTRACTION_DIR)
    subprocess.run(CARGO_HAX, cwd=SCRIPT_DIR, check=True)
    clean_stale_fsti(EXTRACTION_DIR)
    print("[core-models/hax.py] done")


def main():
    sys.tracebacklimit = 0
    parser = argparse.ArgumentParser(description="Canonical F* extraction for libcrux-core-models.")
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
