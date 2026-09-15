#! /usr/bin/env python3
"""Canonical F* extraction for the `libcrux-secrets` crate.

Single source of truth for how `crates/utils/secrets` is extracted to F*.  All
three algorithm scripts call this instead of inlining `cargo hax`, unifying a
prior drift (sha3 used to omit `--interfaces "-**"`).

secrets is extracted TRANSPARENTLY (`--interfaces "-**"`, i.e. `.fst`
implementations only, no `.fsti`): the abstract `.fsti` (with `true` posts)
would hide the classify-is-identity fact (`classify = fun self -> self`), which
is not cold-provable through the typeclass-method encoding and could only be
replayed from stale hints — breaking any consumer whose hints drifted (e.g.
Vector.Portable.Compress.compress post-merge).  Transparency lets `f_as_*`
reduce to the plain reinterpret cast.  See feedback_postmerge_audit_order.

Content is INVARIANT to `--cfg pre_core_models` (empirically verified
2026-07-28).  `--interfaces` only controls `.fsti` emission, never `.fst`
bodies, so the transparent policy is uniform across consumers.

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

# Transparent extraction: implementations only (`--interfaces "-**"`), all items.
CARGO_HAX = ["cargo", "hax", "into", "-i", "+**", "fstar", "--interfaces", "-**"]

SENTINEL = "Libcrux_secrets.Int.fst"


def clean_generated_fstar(directory):
    """Remove generated `.fst`/`.fsti` BEFORE re-extracting.  hax extracts
    incrementally (unchanged modules keep their old files) and NEVER deletes a
    `.fsti` when a module stops emitting an interface — a leftover `.fsti` then
    silently SHADOWS the fresh `.fst` (the stale-.fsti contamination that broke
    the SHA-3 SIMD proofs).  secrets is extracted transparently (0 `.fsti`), so a
    clean-then-extract guarantees no `.fsti` shadow survives.  This dir contains
    no hand-written `.fst`/`.fsti`, so removing all is safe."""
    if not os.path.isdir(directory):
        return
    import glob
    for f in glob.glob(os.path.join(directory, "*.fst")) + glob.glob(os.path.join(directory, "*.fsti")):
        os.remove(f)


def extract(force=False):
    force = force or os.environ.get("HAX_FORCE") == "1"
    sentinel_path = os.path.join(EXTRACTION_DIR, SENTINEL)
    if os.path.exists(sentinel_path) and not force:
        print(f"[secrets/hax.py] already extracted ({SENTINEL} present); skipping. "
              f"Use --force to re-extract.")
        return

    print(f"[secrets/hax.py] extracting -> {EXTRACTION_DIR}")
    print("  Command:", " ".join(CARGO_HAX))
    clean_generated_fstar(EXTRACTION_DIR)
    subprocess.run(CARGO_HAX, cwd=SCRIPT_DIR, check=True)
    print("[secrets/hax.py] done")


def main():
    sys.tracebacklimit = 0
    parser = argparse.ArgumentParser(description="Canonical F* extraction for libcrux-secrets.")
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
