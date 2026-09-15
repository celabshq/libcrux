#!/usr/bin/env python3
"""enumerate-trust (WS5) — the ONE command that prints the COMPLETE, DEDUPED,
SOURCE-BACKED trusted base of the hax-verified crates, with the *silent* surfaces
made visible.

The 2026-08-12 trust audit (§3b/§3d) found trust spread across FIVE surface forms,
with no single command giving a trustworthy count. Two of the five say neither
`assume` nor `admit` in Rust and so a naive grep MISSES them entirely:

  (S1) F* `assume val` / `assume` / `admit ()` / `magic ()` / `--admit_smt_queries`
       — the observed F* plane (trust_scan.py plane 1).
  (S2) `mk_lift_lemma!(...)`  → an opaque `[@@ LIFT_LEMMA] assume val` axiom. SILENT.
  (S3) `#[hax_lib::opaque] { unimplemented!() }` → an uninterpreted F* `val`. SILENT.
  (S4) `#[libcrux_macros::trusted(lax|panic_free|opaque|exclude|replace, …)]` markers.
  (S5) raw `#[hax_lib::fstar::verification_status(...)]` / `proof!("admit ()")`.

This tool enumerates all five, across all crates AND the shared core-models base,
and folds them into the audit's §2 decomposition (hardware symbols / generated lifts
/ hand-written axioms / foundation). It fixes the three counting bugs the audit named:

  * S2/S3 made first-class (were invisible),                       [deliverable 1]
  * byte-identical F* modules extracted into >1 crate deduped,     [deliverable 2]
  * source-less stale `*_extract` `.fst` modules excluded.         [deliverable 3]

The core-models Rust-source surfaces (S2/S3/S4-replace) are git-tracked, so they are
ALWAYS visible — even on a plain checkout where the generated `.fst` tree is gitignored
and absent. The F* observed plane (S1) degrades gracefully and says so when the tree
is not present; run this in CI right after `hax extract` for the full observed count.

Usage:
  python3 scripts/enumerate_trust.py            # human-readable inventory
  python3 scripts/enumerate_trust.py --json     # machine-readable, same data
  python3 scripts/enumerate_trust.py --repo-root PATH
"""

import argparse
import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import trust_scan as ts
import trust_ledger as tl


# ---------------------------------------------------------------------------
# Crate registry. Extends trust_ledger.CRATES (ml-kem/ml-dsa/sha3) with the two
# other workspace crates that carry a trusted surface: the shared core-models
# base and kmac (which re-extracts the same Libcrux_core_models.* tree).
# ---------------------------------------------------------------------------
CRATE_ROOTS = {
    "ml-kem":      "libcrux-ml-kem",
    "ml-dsa":      "libcrux-ml-dsa",
    "sha3":        "crates/algorithms/sha3",
    "kmac":        "crates/algorithms/kmac",
    "core-models": "crates/utils/core-models",
}

# Rust `src/` roots scanned for the S2/S3/S4/S5 markers.
SRC_ROOTS = {
    "core-models": "crates/utils/core-models/src",
    "ml-kem":      "libcrux-ml-kem/src",
    "ml-dsa":      "libcrux-ml-dsa/src",
    "sha3":        "crates/algorithms/sha3/src",
    "kmac":        "crates/algorithms/kmac/src",
}

# F* module-name prefix -> crate src dir, for stale/source-backing resolution.
MODULE_PREFIX_SRC = {
    "Libcrux_ml_kem.":      "libcrux-ml-kem/src",
    "Libcrux_ml_dsa.":      "libcrux-ml-dsa/src",
    "Libcrux_sha3.":        "crates/algorithms/sha3/src",
    "Libcrux_kmac.":        "crates/algorithms/kmac/src",
    "Libcrux_core_models.": "crates/utils/core-models/src",
    "Libcrux_intrinsics.":  "crates/utils/intrinsics/src",
}

# A stale build-leftover module: its last dotted segment is `Extract` or ends
# `_extract` (case-insensitive) — a `*_extract` module re-extracted under an old
# (e.g. pre-relocation) name with no Rust definition behind it. `(?:^|_)` keeps
# a camelCase word that merely ends in "extract" (no separator) from matching.
_STALE_MODULE_RE = re.compile(r"(?:^|_)extract$", re.I)


def _abs(repo_root, *p):
    return os.path.join(repo_root, *p)


def _arch_of(path):
    if "/x86" in path or "x86" in os.path.basename(path):
        return "x86"
    if "/arm" in path or "arm" in os.path.basename(path):
        return "arm"
    return "other"


# ===========================================================================
# Core-models base — the SILENT surfaces (S2/S3) + hand-written F* (S4-replace)
# ===========================================================================

def _walk_rs(root):
    for dp, dn, fns in os.walk(root):
        dn[:] = [d for d in dn if d not in ts._SKIP_DIRS]
        for fn in sorted(fns):
            if fn.endswith(".rs") and fn not in ts._MARKER_SKIP_FILES:
                yield os.path.join(dp, fn)


def enumerate_core_models(repo_root):
    """S2 (mk_lift_lemma! generated lifts) + S3 (opaque intrinsic hardware symbols)
    + the core-models hand-written F* `replace` blocks + foundation primitives.
    All from git-tracked Rust source (visible without the generated F* tree)."""
    cm_src = _abs(repo_root, SRC_ROOTS["core-models"])
    result = {
        "present": os.path.isdir(cm_src),
        "lift_lemmas": [],       # S2
        "opaque_intrinsics": [], # S3
        "replace_sites": [],     # hand-written F* substituted for extracted body
        "foundation_opaque": [], # opaque primitives in abstractions/
    }
    if not result["present"]:
        return result

    for path in _walk_rs(cm_src):
        rel = os.path.relpath(path, repo_root)
        arch = _arch_of(rel)
        for r in ts.scan_file_mk_lift_lemmas(path, repo_root):
            r["arch"] = arch
            result["lift_lemmas"].append(r)
        is_abstractions = f"{os.sep}abstractions{os.sep}" in path
        for r in ts.scan_file_opaque_intrinsics(path, repo_root):
            r["arch"] = arch
            (result["foundation_opaque"] if is_abstractions
             else result["opaque_intrinsics"]).append(r)
        for r in ts.scan_file_replace_sites(path, repo_root):
            result["replace_sites"].append(r)

    # Cross-link S3 <-> S2: an opaque intrinsic that some mk_lift_lemma! names has a
    # (differentially-tested) int-vec model reachable through a LIFT axiom; one that
    # NO lift names is uninterpreted with no F* semantics at all ("no-spec").
    lifted_names = {r["name"] for r in result["lift_lemmas"]}
    for r in result["opaque_intrinsics"]:
        r["has_lift"] = r["name"] in lifted_names
    return result


# ===========================================================================
# Rust trust markers (S4 whole-fn wrappers + inline admit/assume, S5 raw)
# ===========================================================================

def enumerate_rust_markers(repo_root):
    """Aggregate the S4/S5 Rust markers across every crate src/ (comment-masked).
    Returns per-root and total breakdowns."""
    per_root = {}
    totals = {"attr_by_kind": {}, "inline_admit": 0, "inline_assume": 0,
              "replace_sites": 0, "raw": 0}
    for name, rel in sorted(SRC_ROOTS.items()):
        root = _abs(repo_root, rel)
        if not os.path.isdir(root):
            continue
        mk = ts.scan_rust_trust_markers(root, repo_root)
        # replace SITES (observed) live across the whole src, count them too.
        replace_sites = 0
        for path in _walk_rs(root):
            replace_sites += len(ts.scan_file_replace_sites(path, repo_root))
        attr_by_kind = {}
        for a in mk["attr"]:
            attr_by_kind[a["kind"]] = attr_by_kind.get(a["kind"], 0) + 1
        n_admit = sum(1 for b in mk["body"] if b["kind"] == "inline-admit")
        n_assume = sum(1 for b in mk["body"] if b["kind"] == "inline-assume")
        n_raw = len(mk["raw_admit"]) + len(mk["raw_assume"])
        per_root[name] = {
            "attr_by_kind": dict(sorted(attr_by_kind.items())),
            "inline_admit": n_admit, "inline_assume": n_assume,
            "replace_sites": replace_sites, "raw": n_raw,
        }
        for k, v in attr_by_kind.items():
            totals["attr_by_kind"][k] = totals["attr_by_kind"].get(k, 0) + v
        totals["inline_admit"] += n_admit
        totals["inline_assume"] += n_assume
        totals["replace_sites"] += replace_sites
        totals["raw"] += n_raw
    totals["attr_by_kind"] = dict(sorted(totals["attr_by_kind"].items()))
    return {"per_root": per_root, "totals": totals}


# ===========================================================================
# F* observed plane (S1) — DEDUPED by content hash + stale-excluded
# ===========================================================================

def _git_tracked_fstar(repo_root):
    try:
        out = subprocess.run(
            ["git", "-C", repo_root, "ls-files", "*.fst", "*.fsti"],
            capture_output=True, text=True, check=True).stdout
    except Exception:
        return set()
    return {os.path.normpath(_abs(repo_root, line)) for line in out.split("\n") if line}


def _resolve_module_source(module_name, repo_root):
    """Best-effort map an extracted F* module name to a backing Rust file, using
    hax's `Crate.A.B` <-> `src/a/b.rs` convention (submodules may be collapsed, so
    every prefix chain is tried). Returns a path or None."""
    for prefix, src_rel in MODULE_PREFIX_SRC.items():
        if module_name.startswith(prefix):
            src = _abs(repo_root, src_rel)
            if not os.path.isdir(src):
                return None
            segs = [s.lower() for s in module_name[len(prefix):].split(".") if s]
            # Try progressively shorter chains (hax collapses nested modules into
            # one file), and both `a/b.rs` and `a/b/mod.rs`.
            for k in range(len(segs), 0, -1):
                chain = segs[:k]
                cand = os.path.join(src, *chain) + ".rs"
                if os.path.isfile(cand):
                    return cand
                cand = os.path.join(src, *chain, "mod.rs")
                if os.path.isfile(cand):
                    return cand
            # top-level crate module (lib.rs / the crate root) always backs it
            return src if segs else None
    return None  # non-crate module (Spec.*, Tactics.*, EquivImplSpec.*, …)


def _source_backing(module_name, path, repo_root, tracked):
    """Classify a scanned F* module's source backing:
      'tracked'    git-tracked companion — backed by definition.
      'stale'      untracked + name matches the `*_extract` build-leftover pattern
                   — DROP (source-less, inflates the count).
      'resolved'   untracked + resolves to a Rust source file — backed.
      'unresolved' untracked + a crate module we could not resolve — KEPT but flagged
                   (conservative: over-count beats hiding trust; audit the note).
      'external'   non-crate module (Spec.*, Tactics.*, …) — kept, not a crate axiom.
    """
    if os.path.normpath(path) in tracked:
        return "tracked"
    last = module_name.split(".")[-1]
    if _STALE_MODULE_RE.search(last):
        return "stale"
    is_crate = any(module_name.startswith(p) for p in MODULE_PREFIX_SRC)
    if not is_crate:
        return "external"
    return "resolved" if _resolve_module_source(module_name, repo_root) else "unresolved"


def enumerate_fstar_observed(repo_root):
    """Walk every crate's proofs/fstar, scan S1 obligations per file, DEDUP
    byte-identical modules (same name + same sha256) extracted into >1 crate, and
    drop source-less stale modules. Degrades to 'tree absent' gracefully."""
    tracked = _git_tracked_fstar(repo_root)
    # Collect per (module, sha256): obligation count + which crates carry it.
    by_content = {}   # (module, sha256) -> {"count", "crates":set, "path", "backing"}
    per_crate_raw = {}
    stale, unresolved = [], []
    any_generated = False

    for crate, rel in sorted(CRATE_ROOTS.items()):
        fstar_root = _abs(repo_root, rel, "proofs", "fstar")
        if not os.path.isdir(fstar_root):
            continue
        raw_count = 0
        for dp, dn, fns in os.walk(fstar_root):
            dn[:] = [d for d in dn if d not in ts._SKIP_DIRS]
            if os.path.basename(dp) == "extraction":
                # "generated tree present" == there is a freshly-extracted (UNTRACKED)
                # .fst here. Git-tracked companions under extraction/ do NOT count, so
                # a plain checkout reads as ABSENT (matching how trust_ledger warns).
                any_generated = any_generated or any(
                    (f.endswith(".fst") or f.endswith(".fsti"))
                    and os.path.normpath(os.path.join(dp, f)) not in tracked
                    for f in fns)
            for fn in sorted(fns):
                if not (fn.endswith(".fst") or fn.endswith(".fsti")):
                    continue
                path = os.path.join(dp, fn)
                obl = ts.scan_file_obligations(path)
                if not obl:
                    continue
                module = ts.module_name_of(path)
                backing = _source_backing(module, path, repo_root, tracked)
                if backing == "stale":
                    stale.append({"crate": crate, "module": module,
                                  "obligations": len(obl)})
                    continue
                if backing == "unresolved":
                    unresolved.append({"crate": crate, "module": module,
                                       "obligations": len(obl)})
                raw_count += len(obl)
                key = (module, ts.file_sha256(path))
                slot = by_content.setdefault(
                    key, {"count": len(obl), "crates": set(),
                          "module": module, "backing": backing})
                slot["crates"].add(crate)
        if raw_count:
            per_crate_raw[crate] = raw_count

    deduped_total = sum(v["count"] for v in by_content.values())
    raw_total = sum(v["count"] * len(v["crates"]) for v in by_content.values())
    shared = [{"module": v["module"], "obligations": v["count"],
               "crates": sorted(v["crates"])}
              for v in by_content.values() if len(v["crates"]) > 1]
    shared.sort(key=lambda d: (-len(d["crates"]), d["module"]))
    return {
        "tree_present": any_generated or bool(by_content),
        "generated_present": any_generated,
        "per_crate_raw": dict(sorted(per_crate_raw.items())),
        "raw_total": raw_total,
        "deduped_total": deduped_total,
        "shared_modules": shared,
        "duplicate_obligations_collapsed": raw_total - deduped_total,
        "stale_excluded": sorted(stale, key=lambda d: (d["crate"], d["module"])),
        "unresolved_backing": sorted(unresolved, key=lambda d: (d["crate"], d["module"])),
    }


# ===========================================================================
# Assemble the full inventory + §2 decomposition
# ===========================================================================

def build_inventory(repo_root):
    cm = enumerate_core_models(repo_root)
    markers = enumerate_rust_markers(repo_root)
    fstar = enumerate_fstar_observed(repo_root)

    n_lift = len(cm["lift_lemmas"])
    hw = cm["opaque_intrinsics"]
    hw_lifted = sum(1 for r in hw if r.get("has_lift"))
    hw_nospec = sum(1 for r in hw if not r.get("has_lift"))
    hw_nospec_stub = sum(1 for r in hw if not r.get("has_lift") and r["stub_body"])
    hw_marked = sum(1 for r in hw if r.get("marked"))
    hw_unmarked_names = sorted(r["name"] for r in hw if not r.get("marked"))
    n_foundation = len(cm["foundation_opaque"]) + len(cm["replace_sites"])

    # §2 decomposition (audit 2026-08-12). Source-side (always available, dedup-free).
    #   hardware symbols  = opaque intrinsic vals (S3)
    #   generated lifts   = mk_lift_lemma! axioms (S2)
    #   hand-written      = core-models fstar::replace blocks + inline admit/assume + raw
    #   foundation        = opaque primitives in the abstractions layer
    handwritten = (len(cm["replace_sites"])
                   + markers["totals"]["inline_admit"]
                   + markers["totals"]["inline_assume"]
                   + markers["totals"]["raw"])
    decomposition = {
        "hardware_symbols": len(hw),
        "generated_lifts": n_lift,
        "handwritten_axioms": handwritten,
        "foundation": len(cm["foundation_opaque"]),
    }
    decomposition["source_side_total"] = sum(decomposition.values())

    return {
        "repo_root": repo_root,
        "core_models": {
            "present": cm["present"],
            "hardware_symbols": {
                "total": len(hw),
                "by_arch": _arch_hist(hw),
                "lifted": hw_lifted,
                "nospec": hw_nospec,
                "nospec_by_arch": _arch_hist([r for r in hw if not r.get("has_lift")]),
                "nospec_stub_body": hw_nospec_stub,
                "nospec_names": sorted(r["name"] for r in hw if not r.get("has_lift")),
                "marked": hw_marked,
                "unmarked_names": hw_unmarked_names,
            },
            "generated_lifts": {
                "total": n_lift,
                "by_arch": _arch_hist(cm["lift_lemmas"]),
                "unique_names": len({r["name"] for r in cm["lift_lemmas"]}),
            },
            "handwritten_fstar_replace": len(cm["replace_sites"]),
            "foundation_opaque": len(cm["foundation_opaque"]),
        },
        "rust_markers": markers,
        "fstar_observed": fstar,
        "decomposition": decomposition,
    }


def _arch_hist(records):
    h = {}
    for r in records:
        h[r.get("arch", "other")] = h.get(r.get("arch", "other"), 0) + 1
    return dict(sorted(h.items()))


# ===========================================================================
# Human-readable report
# ===========================================================================

def _fmt_hist(h):
    return " | ".join(f"{k} {v}" for k, v in h.items()) if h else "-"


def render(inv):
    L = []
    a = L.append
    a("=" * 78)
    a("libcrux TRUSTED-BASE INVENTORY  (enumerate-trust, WS5)")
    a("=" * 78)
    a(f"repo: {inv['repo_root']}")

    fo = inv["fstar_observed"]
    if fo["generated_present"]:
        a("generated F* tree: PRESENT (full observed plane below)")
    else:
        a("generated F* tree: ABSENT on this checkout (gitignored). The F* OBSERVED")
        a("  section is partial (git-tracked companions only); the CORE-MODELS and")
        a("  RUST-MARKER sections below are git-tracked and AUTHORITATIVE. Run in CI")
        a("  right after `hax extract` for the full deduped observed count.")
    a("")

    # ---- core-models silent surfaces --------------------------------------
    cm = inv["core_models"]
    a("-" * 78)
    a("CORE-MODELS BASE  (Rust source; git-tracked, always visible)")
    a("-" * 78)
    hs = cm["hardware_symbols"]
    a(f"  S3  hardware symbols  (opaque intrinsic -> uninterpreted F* val)          "
      f"   {hs['total']:>4}   [{_fmt_hist(hs['by_arch'])}]")
    a(f"          carry #[trusted(opaque,…)] loud marker                           "
      f"   {hs['marked']:>4}   (rest = non-intrinsic opaque helpers)")
    a(f"          lifted  (a mk_lift_lemma! gives a tested int-vec model)          "
      f"   {hs['lifted']:>4}")
    a(f"          NO-SPEC (uninterpreted; NO F* semantics)                         "
      f"   {hs['nospec']:>4}   [{_fmt_hist(hs['nospec_by_arch'])}]   <== riskiest")
    a(f"            of which empty body (unimplemented!/todo!)                      "
      f"   {hs['nospec_stub_body']:>4}")
    if hs["nospec_names"]:
        preview = ", ".join(hs["nospec_names"][:8])
        more = "" if len(hs["nospec_names"]) <= 8 else f", +{len(hs['nospec_names']) - 8} more"
        a(f"            names: {preview}{more}")
    gl = cm["generated_lifts"]
    a(f"  S2  generated lifts   (mk_lift_lemma! -> [@@ LIFT_LEMMA] assume val)      "
      f"   {gl['total']:>4}   [{_fmt_hist(gl['by_arch'])}]  ({gl['unique_names']} unique)")
    a(f"      hand-written F*   (fstar::replace: models/type-aliases substituted)   "
      f"   {cm['handwritten_fstar_replace']:>4}")
    a(f"      foundation opaque (abstractions/* trusted primitives)                 "
      f"   {cm['foundation_opaque']:>4}")
    a("")

    # ---- rust markers -----------------------------------------------------
    a("-" * 78)
    a("RUST TRUST MARKERS  (S4 whole-fn wrappers + inline admit/assume; S5 raw)")
    a("-" * 78)
    t = inv["rust_markers"]["totals"]
    a(f"  #[trusted(...)] whole-fn wrappers by kind: {t['attr_by_kind'] or '-'}")
    a(f"  inline-admit body sites: {t['inline_admit']}    inline-assume body sites: {t['inline_assume']}")
    a(f"  fstar::replace sites: {t['replace_sites']}    raw proof!(admit/assume): {t['raw']}")
    for root, d in inv["rust_markers"]["per_root"].items():
        bits = []
        if d["attr_by_kind"]:
            bits.append("wrappers " + ",".join(f"{k}={v}" for k, v in d["attr_by_kind"].items()))
        if d["inline_admit"] or d["inline_assume"]:
            bits.append(f"inline a/A={d['inline_admit']}/{d['inline_assume']}")
        if d["replace_sites"]:
            bits.append(f"replace={d['replace_sites']}")
        if d["raw"]:
            bits.append(f"raw={d['raw']}")
        if bits:
            a(f"    {root:>12}: " + "; ".join(bits))
    a("")

    # ---- F* observed ------------------------------------------------------
    a("-" * 78)
    a("F* OBSERVED OBLIGATIONS  (S1: assume val/assume/admit/magic) — DEDUPED + SOURCE-BACKED")
    a("-" * 78)
    if not fo["per_crate_raw"] and not fo["generated_present"]:
        a("  (no obligations scanned — generated tree absent; git-tracked companions clean)")
    else:
        for crate, n in fo["per_crate_raw"].items():
            a(f"    {crate:>12}: {n} raw obligation(s) scanned")
        a(f"  raw (sum over crates):            {fo['raw_total']}")
        a(f"  DEDUPED (byte-identical modules counted once): {fo['deduped_total']}")
        a(f"  duplicate obligations collapsed:  {fo['duplicate_obligations_collapsed']}"
          f"  (shared core-models modules across crates)")
        if fo["shared_modules"]:
            a(f"  shared modules ({len(fo['shared_modules'])}), e.g.:")
            for s in fo["shared_modules"][:6]:
                a(f"      {s['module']}  x{len(s['crates'])} crates  ({s['obligations']} obl)")
    if fo["stale_excluded"]:
        a(f"  STALE excluded (source-less *_extract leftovers): {len(fo['stale_excluded'])}")
        for s in fo["stale_excluded"][:8]:
            a(f"      DROP {s['crate']}/{s['module']}  ({s['obligations']} obl)")
    else:
        a("  stale (source-less) modules excluded: 0")
    if fo["unresolved_backing"]:
        a(f"  NOTE unresolved source backing (kept, audit): {len(fo['unresolved_backing'])}")
        for s in fo["unresolved_backing"][:8]:
            a(f"      ? {s['crate']}/{s['module']}  ({s['obligations']} obl)")
    a("")

    # ---- §2 decomposition -------------------------------------------------
    d = inv["decomposition"]
    a("-" * 78)
    a("§2 DECOMPOSITION  (audit 2026-08-12; source-side, dedup-free, always available)")
    a("-" * 78)
    a(f"  hardware symbols    {d['hardware_symbols']:>5}")
    a(f"  generated lifts     {d['generated_lifts']:>5}")
    a(f"  hand-written axioms {d['handwritten_axioms']:>5}")
    a(f"  foundation          {d['foundation']:>5}")
    a(f"  {'-' * 22}")
    a(f"  TRUSTED-BASE TOTAL  {d['source_side_total']:>5}   (core-models source-side)")
    a("")
    a("Note: the F* OBSERVED plane is the *extraction* of these same source-side")
    a("axioms (the shared Libcrux_core_models.* modules), so the two views measure")
    a("the SAME trust on different planes — they are not additive.")
    return "\n".join(L)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--repo-root", default=None,
                    help="Repo root (default: parent of scripts/)")
    ap.add_argument("--json", action="store_true",
                    help="Emit the full structured inventory as JSON.")
    args = ap.parse_args()
    repo_root = os.path.abspath(
        args.repo_root or os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    inv = build_inventory(repo_root)
    if args.json:
        print(json.dumps(inv, indent=2, sort_keys=True, default=lambda o: sorted(o)
                         if isinstance(o, set) else o))
    else:
        print(render(inv))
    return 0


if __name__ == "__main__":
    sys.exit(main())
