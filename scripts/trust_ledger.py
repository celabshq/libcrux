#!/usr/bin/env python3
"""Trust-ledger reconciler (V7) — the trust campaign's ground-truth centerpiece.

Computes the OBSERVED trust surface of each hax-verified crate directly from build
artifacts (via scripts/trust_scan.py), stores it as a committed per-crate baseline
JSON, and fails CI on any *regression* — a new unproven obligation, a module that
silently stopped extracting, or a grown ADMIT_MODULES list. The ledger is computed
EXCLUSIVELY from the observed side, so no source marker can shrink the reported
surface (plan requirement).

Four observed planes (see trust_scan.py):
  fstar       admit ()/magic ()/assume/assume val/--admit_smt_queries true obligations
  extraction  the set of extracted F* modules (coverage)
  makefile    SLOW_MODULES / ADMIT_MODULES declared in the F* Makefile
  patches     post-extraction *.patch files (count + digest)

Baselines: <crate>/proofs/trust-ledger.baseline.json (git-tracked).

Usage:
  trust_ledger.py [--repo-root PATH] [--crate ml-dsa|ml-kem|sha3] [--json]
  trust_ledger.py --check           # default: compare observed vs baseline, exit 1 on regression
  trust_ledger.py --update-baseline # rewrite baselines from the current observed surface

MARKER RECONCILIATION (G1+): the observed baseline above is ground truth; the Rust
trust markers are CLAIMS about WHY each obligation is trusted. `reconcile_markers()`
(G1 first cut) checks the CLAIMS side for internal soundness — every fn body carrying
a `trusted_admit!` / `trusted_assume!` must also carry the matching-kind
`#[libcrux_macros::trusted(inline-*)]` label and vice-versa, and no raw
`proof!("admit ()")` / `proof!(assume …)` may bypass the wrappers. The full
obligation↔marker NAME mapping (resolving each extracted F* `admit`/`assume` back to
its Rust body marker via hax's deterministic decl-name mangling, so an *unmarked* body
obligation hard-fails) remains a scoped follow-up; module-level coverage + kind
matching in the observed baseline covers the near-term risk. See the plan's V7 section.
"""

import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import trust_scan as ts

# ---------------------------------------------------------------------------
# Crate registry. Paths are relative to the repo root.
# ---------------------------------------------------------------------------
CRATES = {
    "ml-dsa": {
        "root": "libcrux-ml-dsa",
        "prefix": "Libcrux_ml_dsa.",
        "snake": "libcrux_ml_dsa",  # hax `-i` exclusion prefix (V6)
    },
    "ml-kem": {
        "root": "libcrux-ml-kem",
        "prefix": "Libcrux_ml_kem.",
        "snake": "libcrux_ml_kem",
    },
    "sha3": {
        "root": "crates/algorithms/sha3",
        "prefix": "Libcrux_sha3.",
        # no `snake`: G3 module/config mirrors are scoped to ml-kem + ml-dsa.
    },
}

BASELINE_NAME = os.path.join("proofs", "trust-ledger.baseline.json")


def _abs(repo_root, *parts):
    return os.path.join(repo_root, *parts)


# ===========================================================================
# Observed-surface computation
# ===========================================================================

def observe(repo_root, crate_name):
    """Compute the observed 4-plane trust surface for one crate."""
    spec = CRATES[crate_name]
    crate_root = _abs(repo_root, spec["root"])
    fstar_root = os.path.join(crate_root, "proofs", "fstar")
    extraction_dir = os.path.join(fstar_root, "extraction")
    makefile = os.path.join(extraction_dir, "Makefile")

    obl = ts.scan_obligations(fstar_root)
    return {
        "crate": crate_name,
        "planes": {
            "fstar": {
                "total": obl["total"],
                "scanned_files": obl["scanned_files"],
                "by_kind": dict(sorted(obl["by_kind"].items())),
                "by_module": dict(sorted(obl["by_file"].items())),
            },
            "extraction": {
                "modules": ts.list_extracted_module_names(extraction_dir, spec["prefix"]),
            },
            "makefile": {
                "slow_modules": ts.parse_makefile_module_list(makefile, "SLOW_MODULES"),
                "admit_modules": ts.parse_makefile_module_list(makefile, "ADMIT_MODULES"),
            },
            "patches": ts.list_fstar_patches(crate_root),
        },
    }


# ===========================================================================
# Baseline I/O
# ===========================================================================

def baseline_path(repo_root, crate_name):
    return _abs(repo_root, CRATES[crate_name]["root"], BASELINE_NAME)


def load_baseline(repo_root, crate_name):
    p = baseline_path(repo_root, crate_name)
    if not os.path.isfile(p):
        return None
    with open(p) as f:
        text = f.read()
    # The writer prepends a `//` header comment; strip comment lines before parsing.
    lines = [ln for ln in text.splitlines() if not ln.lstrip().startswith("//")]
    return json.loads("\n".join(lines))


def write_baseline(repo_root, crate_name, observed):
    p = baseline_path(repo_root, crate_name)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    header = (
        "// AUTO-GENERATED observed-side trust ledger baseline "
        "(scripts/trust_ledger.py). Do not edit by hand.\n"
    )
    with open(p, "w") as f:
        # A leading // comment keeps the intent visible; strip it before json.load.
        f.write(header)
        json.dump(observed, f, indent=2, sort_keys=True)
        f.write("\n")
    return p


def _load_json_with_comment(path):
    with open(path) as f:
        text = f.read()
    lines = [ln for ln in text.splitlines() if not ln.lstrip().startswith("//")]
    return json.loads("\n".join(lines))


# ===========================================================================
# Regression gate (observed vs baseline)
# ===========================================================================

def reconcile(observed, baseline):
    """Compare an observed surface against its baseline.

    Returns (regressions, notes). `regressions` are hard failures (the trust
    surface grew); `notes` are non-failing changes worth a human glance
    (reductions — rebaseline! — new coverage, SLOW/patch churn)."""
    regressions, notes = [], []
    op, bp = observed["planes"], baseline["planes"]

    # ---- plane 1: F* obligations ----------------------------------------
    ot, bt = op["fstar"]["total"], bp["fstar"]["total"]
    if ot > bt:
        regressions.append(f"[fstar] total obligations {bt} -> {ot} (+{ot - bt})")
    elif ot < bt:
        notes.append(f"[fstar] total obligations {bt} -> {ot} ({ot - bt}); rebaseline to lock the win")

    obm, bbm = op["fstar"]["by_module"], bp["fstar"]["by_module"]
    for mod in sorted(set(obm) | set(bbm)):
        o, b = obm.get(mod, 0), bbm.get(mod, 0)
        if o > b:
            what = "NEW module with obligations" if b == 0 else f"{b} -> {o}"
            regressions.append(f"[fstar] {mod}: {what} (+{o - b})")

    obk, bbk = op["fstar"]["by_kind"], bp["fstar"]["by_kind"]
    for kind in sorted(set(obk) - set(bbk)):
        regressions.append(f"[fstar] new obligation kind '{kind}' x{obk[kind]}")

    # ---- plane 2: extraction coverage -----------------------------------
    oe, be = set(op["extraction"]["modules"]), set(bp["extraction"]["modules"])
    for mod in sorted(be - oe):
        regressions.append(f"[extraction] module no longer extracted: {mod}")
    for mod in sorted(oe - be):
        notes.append(f"[extraction] newly extracted module: {mod}")

    # ---- plane 3: Makefile SLOW / ADMIT ---------------------------------
    oa, ba = set(op["makefile"]["admit_modules"]), set(bp["makefile"]["admit_modules"])
    for mod in sorted(oa - ba):
        regressions.append(f"[makefile] ADMIT_MODULES grew (ratchet is empty): +{mod}")
    for mod in sorted(ba - oa):
        notes.append(f"[makefile] ADMIT_MODULES shrank: -{mod}")
    os_, bs_ = set(op["makefile"]["slow_modules"]), set(bp["makefile"]["slow_modules"])
    for mod in sorted(os_ - bs_):
        regressions.append(f"[makefile] SLOW_MODULES grew (verified-on-cadence trust): +{mod}")
    for mod in sorted(bs_ - os_):
        notes.append(f"[makefile] SLOW_MODULES shrank: -{mod}")

    # ---- plane 4: patches (count-only regression, digest churn is a note)
    opatch = {d["path"]: d["sha256"] for d in op["patches"]}
    bpatch = {d["path"]: d["sha256"] for d in bp["patches"]}
    for path in sorted(set(opatch) - set(bpatch)):
        regressions.append(f"[patches] new post-extraction patch: {path}")
    for path in sorted(set(bpatch) - set(opatch)):
        notes.append(f"[patches] patch removed: {path}")
    for path in sorted(set(opatch) & set(bpatch)):
        if opatch[path] != bpatch[path]:
            notes.append(f"[patches] patch content changed: {path}")

    return regressions, notes


def reconcile_markers(repo_root, crate_name):
    """Marker DIRECTION of the V7 reconciler (G1 first cut).

    Scans the crate's Rust source for trust markers and checks the CLAIMS side
    for internal soundness (independent of the observed-side numbers, which stay
    baseline-locked). Returns (regressions, notes):
      * missing/stale fn-level label vs body macro  -> regression (soundness)
      * raw proof!("admit ()")/proof!(assume …) outside the wrappers -> regression
      * reason without a category prefix            -> note (annotation_lint --strict enforces)
      * inventory of inline-admit/-assume sites + labels -> note

    NOT YET IMPLEMENTED (scoped follow-up, see module docstring): resolving each
    extracted F* obligation back to a Rust body marker via hax decl-name mangling.
    """
    src_root = _abs(repo_root, CRATES[crate_name]["root"], "src")
    if not os.path.isdir(src_root):
        return [], []
    markers = ts.scan_rust_trust_markers(src_root, repo_root)
    regressions, notes = [], []

    missing, stale, raw = ts.marker_soundness(markers)
    for f, fn, k in missing:
        regressions.append(f"[markers] {f} fn {fn or '<unknown fn>'}: body {k} without fn-level label")
    for f, fn, k in stale:
        regressions.append(f"[markers] {f} fn {fn or '<unknown fn>'}: stale fn-level {k} label")
    for f, line, k in raw:
        regressions.append(f"[markers] {f}:{line} raw proof!({k} …) obligation not wrapped")

    for b in markers["body"] + markers["attr"]:
        if not ts.reason_ok(b["reason"]):
            notes.append(f"[markers] {b['file']}:{b['line']} reason lacks category prefix")

    n_admit = sum(1 for b in markers["body"] if b["kind"] == "inline-admit")
    n_assume = sum(1 for b in markers["body"] if b["kind"] == "inline-assume")
    attr_by_kind = {}
    for a in markers["attr"]:
        attr_by_kind[a["kind"]] = attr_by_kind.get(a["kind"], 0) + 1
    if markers["body"] or markers["labels"]:
        notes.append(
            f"[markers] {n_admit} inline-admit + {n_assume} inline-assume body site(s), "
            f"{len(markers['labels'])} fn label(s); obligation↔marker name mapping is a "
            "scoped follow-up"
        )
    if markers["attr"]:
        breakdown = ", ".join(f"{k}={attr_by_kind[k]}" for k in sorted(attr_by_kind))
        notes.append(
            f"[markers] {len(markers['attr'])} whole-function trust wrapper(s) ({breakdown}); "
            "each emits its hax mechanism (byte-identical extraction) + a category+reason"
        )
    return regressions, notes


def check_companion_tags(repo_root, crate_name):
    """V4 — companion-axiom tags. Every hand-written companion AXIOM (an F* obligation
    in a git-tracked `proofs/fstar/spec/` module) must carry exactly one
    `[@@ "trusted: <category>: <reason>"]` tag whose reason passes reason_ok. Per file
    the bijection is #tags == #obligations. These are the CLAIMS side of the F* plane
    for the git-tracked companions; they run on the committed tree (no extraction
    needed). Returns (regressions, notes)."""
    spec_dir = _abs(repo_root, CRATES[crate_name]["root"], "proofs", "fstar", "spec")
    if not os.path.isdir(spec_dir):
        return [], []
    regressions, notes = [], []
    n_ax = n_tag = 0
    for fn in sorted(os.listdir(spec_dir)):
        if not (fn.endswith(".fst") or fn.endswith(".fsti")):
            continue
        p = os.path.join(spec_dir, fn)
        obl = ts.scan_file_obligations(p)
        tags = ts.scan_fstar_trusted_tags(p)
        n_ax += len(obl)
        n_tag += len(tags)
        if len(obl) != len(tags):
            regressions.append(
                f"[companion-tags V4] {crate_name}/{fn}: {len(obl)} companion axiom(s) "
                f'but {len(tags)} `[@@ "trusted:…"]` tag(s) (need exactly one per axiom)')
        for t in tags:
            if not ts.reason_ok(t["reason"]):
                regressions.append(
                    f"[companion-tags V4] {crate_name}/{fn}:{t['line']} tag reason lacks a "
                    f"valid category prefix: {t['reason'][:60]!r}")
    if n_ax:
        notes.append(f"[companion-tags V4] {crate_name}: {n_tag}/{n_ax} companion axioms tagged")
    return regressions, notes


def check_module_mirrors(repo_root, crate_name):
    """V5 + V6 — module/config trust mirrors.

    V5: every Makefile `SLOW_MODULES` / `ADMIT_MODULES` entry carries a
        `# trusted-module: <module> : <reason>` annotation (reason_ok), the bijection
        {SLOW∪ADMIT} == {annotated modules} holds, and ADMIT_MODULES does not grow
        beyond the committed baseline (ratchet; empty remains the eventual target,
        but a documented deferral recorded in the baseline is tolerated).
    V6: every `-<crate_snake>::…` hax extraction-exclusion token carries a
        `# trusted-module: <token> : <reason>` annotation (bijection + reason_ok).

    Both mirrors live in git-tracked files (Makefile, hax.py/hax.sh), so they run on
    the committed tree without extraction. Returns (regressions, notes)."""
    spec = CRATES[crate_name]
    crate_root = _abs(repo_root, spec["root"])
    regressions, notes = [], []

    # ---- V5: Makefile SLOW/ADMIT ↔ annotation bijection + ADMIT empty-ratchet ----
    makefile = os.path.join(crate_root, "proofs", "fstar", "extraction", "Makefile")
    if os.path.isfile(makefile):
        with open(makefile) as f:
            mtext = f.read()
        slow = ts.parse_makefile_module_list(makefile, "SLOW_MODULES")
        admit = ts.parse_makefile_module_list(makefile, "ADMIT_MODULES")
        anns = ts.scan_trusted_module_annotations(mtext)
        ann = {a["name"].removesuffix(".fst").removesuffix(".fsti"): a for a in anns}
        want = set(slow) | set(admit)
        for mod in sorted(want):
            if mod not in ann:
                regressions.append(f"[module-mirror V5] {crate_name}: SLOW/ADMIT module "
                                   f"{mod} has no `# trusted-module:` reason")
            elif not ts.reason_ok(ann[mod]["reason"]):
                regressions.append(f"[module-mirror V5] {crate_name}: {mod} reason lacks a "
                                   f"category prefix: {ann[mod]['reason'][:50]!r}")
        for name, a in sorted(ann.items()):
            if name not in want:
                regressions.append(f"[module-mirror V5] {crate_name}: stray `# trusted-module: "
                                   f"{a['name']}` names no SLOW/ADMIT module")
        if admit:
            # Ratchet, not absolute zero: a documented deferral may live in the
            # committed baseline (e.g. the 2026-07 sha3 SIMD store_block
            # deferral); anything BEYOND the baseline is a regression.
            baseline = load_baseline(repo_root, crate_name)
            allowed = set(((baseline or {}).get("planes", {})
                           .get("makefile", {}) or {}).get("admit_modules", []))
            extra = sorted(set(admit) - allowed)
            if extra:
                regressions.append(f"[module-mirror V5] {crate_name}: ADMIT_MODULES beyond the "
                                   f"committed baseline (ratchet is non-increasing): {extra}")
            else:
                notes.append(f"[module-mirror V5] {crate_name}: ADMIT_MODULES non-empty but "
                             f"baseline-covered (documented deferral): {sorted(admit)}")
        if want:
            notes.append(f"[module-mirror V5] {crate_name}: {len(want)} SLOW/ADMIT module(s) mirrored")

    # ---- V6: hax `-i` extraction-exclusion tokens ↔ annotation bijection ----
    snake = spec.get("snake")
    if snake:
        hax_script = next((os.path.join(crate_root, c)
                           for c in ("hax.py", "hax.sh")
                           if os.path.isfile(os.path.join(crate_root, c))), None)
        if hax_script:
            with open(hax_script) as f:
                htext = f.read()
            tokens = set(ts.scan_hax_exclusion_tokens(htext, snake))
            ann = {a["name"]: a for a in ts.scan_trusted_module_annotations(htext)}
            for tok in sorted(tokens):
                if tok not in ann:
                    regressions.append(f"[module-mirror V6] {crate_name}: extraction-exclusion "
                                       f"{tok} has no `# trusted-module:` reason")
                elif not ts.reason_ok(ann[tok]["reason"]):
                    regressions.append(f"[module-mirror V6] {crate_name}: {tok} reason lacks a "
                                       f"category prefix: {ann[tok]['reason'][:50]!r}")
            for name in sorted(ann):
                if name.startswith("-" + snake + "::") and name not in tokens:
                    regressions.append(f"[module-mirror V6] {crate_name}: stray `# trusted-module: "
                                       f"{name}` names no active `-i` exclusion filter")
            if tokens:
                notes.append(f"[module-mirror V6] {crate_name}: {len(tokens)} extraction-exclusion(s) mirrored")

    return regressions, notes


# ===========================================================================
# V8 — fstar::replace bijection (repo-global CLAIMS-side lint)
#
# `#[hax_lib::fstar::replace(...)]` / `#[fstar::replace_body(...)]` DISCARDS the F*
# hax extracts from a Rust body and substitutes hand-written F*. F* then verifies the
# SUBSTITUTE, not the code — a trust surface AS BAD OR WORSE than an assume val, and
# INVISIBLE to the observed F* plane (the substitute `let` looks like a real definition;
# `trust_scan`'s plane-1 tokenizer never sees the .rs attribute). So the only way to
# force a REASON at each site and fail CI on a NEW unmarked block is this CLAIMS-side
# bijection: every replace site carries exactly one `#[libcrux_macros::trusted(replace,
# "<category>: <reason>")]` marker (reason_ok), counted per .rs file (a struct that
# stacks an `interface` + impl replace pair thus needs two markers). Mirrors the V4
# companion-tag check, but repo-global (replace blocks live in intrinsics/core-models/
# specs too, not just the three registered crates' src).
# ===========================================================================

# Roots that can contain `fstar::replace` sites, scoped to the WORKSPACE crates the
# trust ledger governs. Walked once, _SKIP_DIRS pruned.
#
# `specs/*` is deliberately OUT of scope: the Hacspec spec crates are standalone
# (outside the main workspace, pinned to a different git hax-lib and extracted
# independently), so they cannot take the `libcrux-macros` dep the marker needs
# without risk. They carry 3 `assume val createi` replace blocks (one per spec crate:
# specs/ml-dsa, specs/ml-kem, specs/sha3) — a known SPEC-LAYER trusted primitive
# (array-from-fn), a distinct trust surface from this intrinsics campaign. Revisit if
# the spec crates are folded into the workspace.
REPLACE_SCAN_ROOTS = ("crates", "libcrux-ml-dsa", "libcrux-ml-kem")


def check_replace_markers(repo_root):
    """V8 — every fstar::replace / replace_body SITE carries exactly one
    `#[libcrux_macros::trusted(replace, …)]` MARKER (reason_ok), per .rs file.
    Returns (regressions, notes). Repo-global; runs on the committed tree."""
    regressions, notes = [], []
    n_site = n_mark = 0
    for root in REPLACE_SCAN_ROOTS:
        aroot = _abs(repo_root, root)
        if not os.path.isdir(aroot):
            continue
        for dirpath, dirnames, filenames in os.walk(aroot):
            dirnames[:] = [d for d in dirnames if d not in ts._SKIP_DIRS]
            for fn in sorted(filenames):
                if not fn.endswith(".rs") or fn in ts._MARKER_SKIP_FILES:
                    continue
                p = os.path.join(dirpath, fn)
                sites = ts.scan_file_replace_sites(p, repo_root)
                markers = [a for a in ts.scan_file_trust_markers(p, repo_root)["attr"]
                           if a["kind"] == "replace"]
                if not sites and not markers:
                    continue
                n_site += len(sites)
                n_mark += len(markers)
                rel = os.path.relpath(p, repo_root)
                if len(sites) != len(markers):
                    regressions.append(
                        f"[replace-bijection V8] {rel}: {len(sites)} fstar::replace site(s) "
                        f"but {len(markers)} `trusted(replace,…)` marker(s) "
                        "(need exactly one per site)")
                for mk in markers:
                    if not ts.reason_ok(mk["reason"]):
                        regressions.append(
                            f"[replace-bijection V8] {rel}:{mk['line']} marker reason lacks a "
                            f"valid category prefix: {mk['reason'][:60]!r}")
    if n_site or n_mark:
        notes.append(f"[replace-bijection V8] {n_mark}/{n_site} fstar::replace site(s) marked")
    return regressions, notes


# ===========================================================================
# Reporting
# ===========================================================================

def summarize(observed):
    p = observed["planes"]
    return (
        f"{observed['crate']:>7}: "
        f"fstar={p['fstar']['total']:<4} "
        f"kinds={p['fstar']['by_kind']} "
        f"extracted={len(p['extraction']['modules'])} "
        f"slow={len(p['makefile']['slow_modules'])} "
        f"admit={len(p['makefile']['admit_modules'])} "
        f"patches={len(p['patches'])}"
    )


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--repo-root", default=None,
                    help="Repo root (default: parent of scripts/)")
    ap.add_argument("--crate", choices=sorted(CRATES), action="append",
                    help="Restrict to one crate (repeatable). Default: all.")
    ap.add_argument("--check", action="store_true",
                    help="Compare observed vs baseline; exit 1 on regression (default action).")
    ap.add_argument("--warn-only", action="store_true",
                    help="Report regressions but always exit 0 (soft CI gate until parity is "
                         "confirmed against the CI hax version; flip off to make it blocking).")
    ap.add_argument("--update-baseline", action="store_true",
                    help="Rewrite baselines from the current observed surface.")
    ap.add_argument("--json", action="store_true",
                    help="Print the observed surface as JSON instead of the summary.")
    args = ap.parse_args()

    repo_root = os.path.abspath(
        args.repo_root or os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    )
    crates = args.crate or sorted(CRATES)

    if args.json:
        out = {c: observe(repo_root, c) for c in crates}
        print(json.dumps(out, indent=2, sort_keys=True))
        return 0

    if args.update_baseline:
        for c in crates:
            observed = observe(repo_root, c)
            p = write_baseline(repo_root, c, observed)
            print(f"wrote {os.path.relpath(p, repo_root)}")
            print("  " + summarize(observed))
        return 0

    # default: --check
    exit_code = 0
    # V8 replace-bijection is repo-global (replace blocks live outside the three
    # registered crates), so it runs ONCE, not per-crate.
    rreg, rnotes = check_replace_markers(repo_root)
    for n in rnotes:
        print(f"  note: {n}")
    for r in rreg:
        print(f"  REGRESSION: {r}")
    if rreg:
        exit_code = 1
    for c in crates:
        observed = observe(repo_root, c)
        print(summarize(observed))
        bpath = baseline_path(repo_root, c)
        if not os.path.isfile(bpath):
            print(f"  no baseline yet ({os.path.relpath(bpath, repo_root)}); run --update-baseline")
            continue
        baseline = _load_json_with_comment(bpath)
        regressions, notes = reconcile(observed, baseline)
        mreg, mnotes = reconcile_markers(repo_root, c)
        regressions += mreg
        notes += mnotes
        # G3 CLAIMS-side lints (V4 companion-axiom tags, V5/V6 module/config mirrors).
        # These run on git-tracked files, so they are correct even on a non-freshly-
        # extracted tree (unlike the observed planes above).
        creg, cnotes = check_companion_tags(repo_root, c)
        vreg, vnotes = check_module_mirrors(repo_root, c)
        regressions += creg + vreg
        notes += cnotes + vnotes
        for n in notes:
            print(f"  note: {n}")
        for r in regressions:
            print(f"  REGRESSION: {r}")
        if regressions:
            exit_code = 1
    if exit_code:
        print("\ntrust-ledger: REGRESSION — the trust surface grew. "
              "Prove the new obligation away, or justify + rebaseline deliberately.")
        if args.warn_only:
            print("(--warn-only: not failing the build. Run on a freshly-extracted tree; "
                  "stale local artifacts read as regressions.)")
            return 0
    else:
        print("\ntrust-ledger: clean (observed surface within baseline).")
    return exit_code


if __name__ == "__main__":
    sys.exit(main())
