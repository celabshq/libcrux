#!/usr/bin/env python3
"""
classify-nospec.py — For each genuinely spec-free T1 wrapper, find call sites
in ml-kem / ml-dsa / sha-3 and report the verification status of the caller.

Output: crates/utils/core-models/proofs/nospec-classification.md
"""
import csv
import re
import sys
from pathlib import Path
from collections import defaultdict

REPO = Path(__file__).resolve().parents[4]

CSV_PATH = REPO / "crates/utils/core-models/proofs/intrinsics-trust-index.csv"

SEARCH_ROOTS = {
    "ml-kem":  REPO / "libcrux-ml-kem/src",
    "ml-dsa":  REPO / "libcrux-ml-dsa/src",
    "sha-3":   REPO / "libcrux-sha3/src",
}

# Annotations that describe the proof status of a Rust function.
# We scan backwards from the call site to the nearest `pub fn` / `fn` and
# collect the annotations on it.
PROOF_STATUS_RE = re.compile(
    r'hax_lib::fstar::verification_status\s*\(\s*(\w+)\s*\)'
    r'|hax_lib::opaque\b'
    r'|hax_lib::requires\b'
    r'|hax_lib::ensures\b'
    r'|panic_free\b'
    r'|admit\s*\(\)'
)

FN_DECL_RE = re.compile(
    r'(?m)^[^\S\n]*(pub(?:\s*\([^)]*\))?\s+(?:unsafe\s+)?fn|fn)\s+(\w+)'
)


def verification_status(src: str, fn_start: int) -> str:
    """Return a brief verification label for the function at fn_start."""
    # Find the start of the function's attribute block by scanning backwards.
    before = src[:fn_start]
    lines = before.split('\n')
    attr_lines = []
    depth = 0
    for line in reversed(lines):
        s = line.strip()
        opens = s.count('[')
        closes = s.count(']')
        if depth > 0:
            attr_lines.append(s)
            depth = max(0, depth + closes - opens)
        elif closes > opens:
            attr_lines.append(s)
            depth = closes - opens
        elif s.startswith('#[') or s.startswith('#!['):
            attr_lines.append(s)
        elif not s or s.startswith('//'):
            continue
        else:
            break
    attrs = ' '.join(attr_lines)

    # Check for explicit verification_status annotation.
    vs = re.search(r'verification_status\s*\(\s*(\w+)\s*\)', attrs)
    if vs:
        return vs.group(1)

    tags = []
    if 'hax_lib::ensures' in attrs:
        tags.append('ensures')
    if 'hax_lib::requires' in attrs:
        tags.append('requires')
    if 'panic_free' in attrs:
        tags.append('panic_free')
    if 'hax_lib::opaque' in attrs:
        tags.append('opaque')
    if tags:
        return '+'.join(tags)
    return 'no-annotation'


def find_containing_fn(src: str, pos: int):
    """Return (fn_name, fn_start_pos) for the innermost fn containing pos."""
    best = None
    for m in FN_DECL_RE.finditer(src):
        if m.start() < pos:
            best = (m.group(2), m.start())
        else:
            break
    return best  # (name, start) or None


def classify_wrapper(wrapper_name: str, arch: str):
    """Return list of (crate, file, fn_name, vstatus) call sites."""
    results = []
    # The Rust call in consumer code uses the wrapper name directly.
    call_re = re.compile(r'\b' + re.escape(wrapper_name) + r'\s*\(')

    for crate, root in SEARCH_ROOTS.items():
        if not root.exists():
            continue
        for rs in root.rglob('*.rs'):
            src = rs.read_text(errors='replace')
            for m in call_re.finditer(src):
                fn_info = find_containing_fn(src, m.start())
                if fn_info is None:
                    continue
                fn_name, fn_start = fn_info
                vstatus = verification_status(src, fn_start)
                rel = rs.relative_to(REPO)
                results.append((crate, str(rel), fn_name, vstatus))
    return results


def main():
    # Load genuinely spec-free wrappers.
    no_spec = []
    with open(CSV_PATH) as f:
        for row in csv.DictReader(f):
            if (row['in_T1'] == 'true'
                    and row['has_extract_ensures'] == 'false'
                    and row['has_specintrinsics_lemma'] == 'false'):
                no_spec.append(row)

    # Group by arch and trust_level for the summary table.
    by_arch_level = defaultdict(int)
    for r in no_spec:
        by_arch_level[(r['arch'], r['trust_level'])] += 1

    # Classify each wrapper.
    rows = []
    for entry in no_spec:
        name = entry['name']
        arch = entry['arch']
        lvl  = entry['trust_level']
        sites = classify_wrapper(name, arch)

        if not sites:
            used_in = '—'
            vstatus = '—'
        else:
            crates = sorted(set(s[0] for s in sites))
            statuses = sorted(set(s[3] for s in sites))
            used_in = ', '.join(crates)
            vstatus = ', '.join(statuses)

        rows.append((name, arch, lvl, used_in, vstatus, sites))

    # ── spec-mechanism survey: for specced wrappers, how are they specified? ──
    # Buckets: extract_ensures_only, specintrinsics_only, both, none
    spec_mechanisms = defaultdict(list)
    with open(CSV_PATH) as f:
        for row in csv.DictReader(f):
            if row['in_T1'] != 'true':
                continue
            has_e = row['has_extract_ensures'] == 'true'
            has_s = row['has_specintrinsics_lemma'] == 'true'
            if has_e and has_s:
                spec_mechanisms['both'].append(row['name'])
            elif has_e:
                spec_mechanisms['extract_ensures_only'].append(row['name'])
            elif has_s:
                spec_mechanisms['specintrinsics_only'].append(row['name'])
            else:
                spec_mechanisms['none'].append(row['name'])

    # Write report.
    out = REPO / "crates/utils/core-models/proofs/nospec-classification.md"
    with open(out, 'w') as f:
        f.write("# No-spec wrapper classification\n\n")
        f.write(f"Generated by `classify-nospec.py` on 2026-05-03.\n")
        f.write(f"Input: {len(no_spec)} genuinely spec-free T1 wrappers ")
        f.write(f"(has_extract_ensures=false AND has_specintrinsics_lemma=false).\n\n")

        f.write("## Summary counts\n\n")
        f.write("| arch | trust_level | count |\n|---|---|---:|\n")
        for (arch, lvl), cnt in sorted(by_arch_level.items()):
            f.write(f"| {arch} | {lvl} | {cnt} |\n")

        # Used vs unused
        unused = [r for r in rows if r[3] == '—']
        used   = [r for r in rows if r[3] != '—']
        f.write(f"\n{len(used)} wrappers have ≥1 call site in ml-kem/ml-dsa/sha-3; ")
        f.write(f"{len(unused)} have none.\n\n")

        f.write("## Spec-mechanism survey (all 193 T1 wrappers)\n\n")
        f.write("How are specced wrappers currently specified?\n\n")
        f.write("| mechanism | count | description |\n|---|---:|---|\n")
        f.write(f"| extract_ensures only | {len(spec_mechanisms['extract_ensures_only'])} | "
                f"`#[hax_lib::ensures]` in `_extract.rs`; no Spec.Intrinsics SMTPat |\n")
        f.write(f"| specintrinsics_only  | {len(spec_mechanisms['specintrinsics_only'])} | "
                f"SMTPat lemma in `Spec.Intrinsics.fsti` only (ml-dsa AVX2 path) |\n")
        f.write(f"| both                 | {len(spec_mechanisms['both'])} | "
                f"Has both an extract ensures AND a Spec.Intrinsics SMTPat |\n")
        f.write(f"| none                 | {len(spec_mechanisms['none'])} | "
                f"Genuinely spec-free (the 63 this report focuses on) |\n\n")

        f.write("## Used wrappers — call-site verification status\n\n")
        f.write("| wrapper | arch | trust | used in | caller vstatus |\n")
        f.write("|---|---|---|---|---|\n")
        for name, arch, lvl, used_in, vstatus, _ in sorted(used, key=lambda r: (r[1], r[0])):
            f.write(f"| `{name}` | {arch} | {lvl} | {used_in} | {vstatus} |\n")

        f.write("\n## Unused wrappers (no call site found)\n\n")
        f.write("| wrapper | arch | trust |\n|---|---|---|\n")
        for name, arch, lvl, _, _, _ in sorted(unused, key=lambda r: (r[1], r[0])):
            f.write(f"| `{name}` | {arch} | {lvl} |\n")

        f.write("\n## Detailed call sites\n\n")
        for name, arch, lvl, used_in, vstatus, sites in sorted(used, key=lambda r: (r[1], r[0])):
            if not sites:
                continue
            f.write(f"### `{name}` ({arch}, {lvl})\n\n")
            f.write("| crate | file | fn | vstatus |\n|---|---|---|---|\n")
            for crate, fpath, fn_name, vs in sorted(sites):
                f.write(f"| {crate} | `{fpath}` | `{fn_name}` | {vs} |\n")
            f.write("\n")

    print(f"Report written to {out}")
    print(f"\nSummary: {len(no_spec)} no-spec wrappers — "
          f"{len(used)} used in codebase, {len(unused)} unused.")

    # Print a compact summary table to stdout for the parent session.
    print("\n=== Used no-spec wrappers — vstatus distribution ===")
    vstatus_dist = defaultdict(int)
    for _, _, _, _, vstatus, sites in used:
        for s in sites:
            vstatus_dist[s[3]] += 1
    for vs, cnt in sorted(vstatus_dist.items(), key=lambda x: -x[1]):
        print(f"  {vs:30s} {cnt:3d} call sites")


if __name__ == '__main__':
    main()
