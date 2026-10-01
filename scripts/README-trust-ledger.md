# Trust-ledger tooling (`trust_scan.py` + `trust_ledger.py`)

A mechanically-computed, git-tracked ledger of the **unproven trust surface** of
the hax-verified crates (`libcrux-ml-kem`, `libcrux-ml-dsa`, `crates/algorithms/sha3`),
with a CI regression gate. It is the ground-truth side of the trust annotations:
the reported surface is computed **only** from build artifacts, so no source
marker can shrink it.

## What it measures — four observed planes

| Plane | What | Source |
|---|---|---|
| `fstar` | unproven obligations: `admit ()`, `magic ()`, bare `assume`, `assume val`, `--admit_smt_queries true` | scan of extracted `.fst`/`.fsti` + hand-written `spec/` companions |
| `extraction` | the set of extracted F\* modules (coverage) | `proofs/fstar/extraction/*.fst{,i}` matching the crate prefix |
| `makefile` | `SLOW_MODULES` / `ADMIT_MODULES` — modules verified-on-cadence or admitted in the default build | the F\* extraction `Makefile` |
| `patches` | post-extraction `*.patch` files (count + sha256) | `proofs/fstar/**/*.patch` |

`trust_scan.py` plane 1 reproduces the `fstar_admits` MCP tool **exactly** (validated
per-file for both large crates: ml-dsa 103, ml-kem 63) but with zero proxy/`make`
dependency, so it runs in plain CI.

## Usage

```bash
python3 scripts/trust_ledger.py --check            # observed vs baseline; exit 1 on regression
python3 scripts/trust_ledger.py --update-baseline  # rebaseline (deliberate, reviewed)
python3 scripts/trust_ledger.py --json             # dump the raw observed surface
python3 scripts/trust_ledger.py --crate ml-dsa --check
```

Baselines live at `<crate>/proofs/trust-ledger.baseline.json` (git-tracked,
auto-generated — do not hand-edit).

## The regression gate

`--check` fails (exit 1) when the trust surface **grows**:
- a new/increased F\* obligation in any module (a new *trusted assumption*),
- a new obligation *kind*,
- a baseline-extracted module that stopped extracting (silent coverage loss),
- `ADMIT_MODULES` grew (the ratchet target is empty) or `SLOW_MODULES` grew,
- a new post-extraction patch.

Surface *reductions* are reported as notes ("rebaseline to lock the win"), never
failures. So the ledger is monotonically non-increasing by construction — proving
an obligation away and rebaselining is the only way the numbers move down, and
nothing can move them up without a red CI.

## The gate runs AFTER extraction (CI-only)

Most obligations live in the *generated* F\* tree — the extracted `.fst`/`.fsti` are
gitignored (`*.fst` + `!`-exceptions for the hand-written companions), so ~94% of the
ml-dsa surface and most of ml-kem's are not present in a plain checkout. The plan's
V7 model therefore runs this **in CI, immediately after `hax extract`**, against a
clean generated tree. The committed baselines are that canonical post-extract surface:

| Crate | Baseline obligations | Note |
|---|---|---|
| ml-dsa | 103 | matches `fstar_admits`; `.fst` gitignored, 6 in tracked `spec/` companions |
| ml-kem | 63 | matches `fstar_admits` |
| sha3 | 6 | all `assume_val` — hax derive/trait stubs (trusted base); 0 actionable admits |

Consequence: **do not read a `--check` diff on a worktree that hasn't been freshly
re-extracted.** Stale leftover `.fst` from an older extraction (e.g. pre-relocation
module names) show up as spurious regressions — that is a "dirty tree", not a real
surface change. Re-extract, then check. The three baselines above were captured from
one consistent re-extraction of all three crates.

## Marker reconciliation — the CLAIMS side

The observed planes above are ground truth; the Rust **trust markers** are *claims*
about **why** each obligation is trusted. The body-level markers provide a
fn-level summary label, plus their reconciliation (`trust_ledger.reconcile_markers`,
run in `--check`) and the V2/V2b/V3 source lints (`annotation_lint.py`):

| Marker | Kind | Wraps |
|---|---|---|
| `trusted_admit!("<cat>: reason")` | in-crate `macro_rules!` (body) | `proof!("admit ()")` |
| `trusted_assume!("<cat>: reason", r#"assume (…)"#)` | in-crate `macro_rules!` (body) | `proof!(assume …)` |
| `#[libcrux_macros::trusted(inline-admit\|inline-assume)]` | proc-macro attr (fn-level) | mandatory summary label per body site |

Category prefixes (the plan's vocabulary): `unprovable-termination:`,
`hax-limitation:`, `trusted-extern:`, `validated-axiom:`, `pending-proof(<ref>):`,
`slow-proof:`. The reason is a **Rust-only** one-line summary (dropped from
extraction — every wrapper is byte-identical to the raw mechanism); long prose stays
as an ordinary source comment.

`reconcile_markers()` checks the claims side for **internal soundness**:
- a fn body carrying `trusted_admit!`/`trusted_assume!` **must** also carry the
  matching-kind `#[…trusted(inline-*)]` label, and vice-versa (missing/stale = regression);
- no raw `proof!("admit ()")`/`proof!(assume …)` may bypass the wrappers (V3).

**Scoped follow-up (not yet implemented):** the full obligation↔marker *name* mapping —
resolving each extracted F\* `admit`/`assume` back to its Rust body marker via hax's
deterministic decl-name mangling, so an *unmarked* body obligation hard-fails. Module-
level coverage + kind matching in the observed baseline covers the near-term risk.

Markers only *annotate* observed entries with categories/reasons — consistent with the
whole design, they can never shrink the reported surface.

## Companion-axiom tags + module/config mirrors

Companion-axiom tags extend the CLAIMS side from Rust body/function markers to the two remaining trust
surfaces, with three more lints wired into both `annotation_lint.py` and
`trust_ledger.py --check`. Unlike the observed planes, these run on the **git-tracked**
tree (companion `spec/` modules, the Makefile, the hax scripts), so they are correct
even without a fresh re-extraction:

| Lint | Surface | Marker | Check |
|---|---|---|---|
| V4 | hand-written companion **axioms** (`assume val` / `admit ()` in `proofs/fstar/spec/`) | `[@@ "trusted: <cat>: <reason>"]` above the decl | per-file bijection `#tags == #obligations` + `reason_ok` |
| V5 | `SLOW_MODULES` / `ADMIT_MODULES` (verified-on-cadence / admitted) | `# trusted-module: <module> : <reason>` in the F\* `Makefile` | bijection + `reason_ok` + ADMIT empty-ratchet |
| V6 | `-<crate>::…` hax `-i` extraction exclusions (dropped modules) | `# trusted-module: <token> : <reason>` in `hax.py` / `hax.sh` | bijection + `reason_ok` |

Counts on the current tree: V4 = 27 tagged axioms (ml-dsa 6, ml-kem 21); V5 = 3 ml-kem
SLOW modules (0 ADMIT in both crates); V6 = 14 exclusions (ml-kem 11, ml-dsa 3).

**Pollution-trap note.** The companion tags are F\* string-literal attributes, and the
plane-1 scanner deliberately does not mask string literals (it mirrors `fstar_admits`).
Reasons are therefore kept token-safe (no bare `assume` / `admit ()` / `magic ()` /
`assume val` / `admit_smt_queries true`), and `trust_scan.mask_trusted_reason_strings`
blanks `"trusted: …"` interiors as a backstop so a future non-token-safe reason can
never grow the surface.
