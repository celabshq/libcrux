# Initial Retrospective — proof work since libcrux main fork

**Snapshot date**: 2026-05-01
**Fork point (t₀)**: main commit `70effe9c7` (2026-03-24 12:37 UTC, "Merge pull request #1367 from cryspen/franziskus/psq-fuzz")
**Methodology**: see `proofs/retrospective-methodology.md`

This is a snapshot mid-sprint. The final retrospective will be written
when the three sprints complete and will overwrite the relevant
sections. Numbers below are reproducible by re-running the documented
commands.

## Lineage

```
main 70effe9c7 (2026-03-24)
  ├── trait-poststrengthen (235 commits, 2026-03-24 → 2026-04-27)
  │     ├── trait-opacify (current ml-kem branch, 399 commits, 2026-03-31 → 2026-05-01, 31 days)
  │     └── ml-dsa-proofs (current ml-dsa branch, 404 commits, 2026-03-31 → 2026-05-01, 31 days)
  │
  └── sha3-byteform-migration (current sha-3 branch, 46 commits, 2026-04-24 → 2026-05-01, 7 days)
        └── (sha3-proofs-focused: sibling, 45 commits, did not merge into byteform-migration via fast-forward; cousin)

Sibling branches (NOT merged into any current branch — counted separately):
  rust-spec, proofs-cleanup, proofs-cleanup-backup, ml-dsa-above-trait,
  agent-aa244240-above-trait, agent-ab0453d4-above-trait, franziskus/sha3-cleanup,
  protz/cleanup, alex/rust-spec-aeneas, alex/rust-spec-quick-hax, Ind-cca-spec,
  dev-add-serialize-specs-trait
```

## Spec source convention (uniform across crates — corrected 2026-05-01)

All three crates write the authoritative Hacspec in **Rust** under
`specs/<crate>/src/*.rs` and hax-extract it to `Hacspec_<crate>.*.fst`.

Build-system asymmetry only: ml-kem and ml-dsa **gitignore** their
extracted `Hacspec_<crate>.*.fst` (regenerated on every
`python3 hax.py extract`); sha-3 **commits** them. The proof work is
the same — the Rust spec source is what gets written by hand.

For W1 measurement, count Rust lines only at `specs/<crate>/src/`.
Counting the committed F\* extraction in sha-3 would double-count the
same source.

## Headline numbers

| Metric | ML-KEM (trait-opacify) | ML-DSA (ml-dsa-proofs) | SHA-3 (sha3-byteform-migration) |
|---|---:|---:|---:|
| Calendar span | 31 days | 31 days | 7 days |
| Total commits | 399 | 404 | 46 |
| Claude-coauthored | 334 (84%) | 360 (89%) | 38 (83%) |
| **W1 Specs added (lines)** | 3,341 (`specs/ml-kem/src/`) | 1,854 (`specs/ml-dsa/src/`) | 656 (`specs/sha3/src/`) |
| **W2a Commute bridges (lines)** | 4,490 (in `specs/ml-kem/proofs/fstar/commute/`) | 884 (in `specs/ml-dsa/proofs/fstar/commute/`) | (none — uses equivalence/ instead) |
| **W2b EquivImplSpec proofs (sha3)** | n/a | n/a | 8,701 (`crates/algorithms/sha3/proofs/fstar/equivalence/`) |
| **W2c Total proof lines (W2a + W2b)** | 4,490 | 884 | 8,701 |
| **W2d New `let lemma_*` definitions** | 147 | 24 | 0 (sha3 uses `: Lemma` typed `let` form, not `lemma_` prefix) |
| **W3 Annotation additions in src/ Rust:** | | | |
|   `#[hax_lib::ensures(`               | 173 | 103 |  67 |
|   `#[hax_lib::requires(`              | 173 |  92 |  71 |
|   `verification_status(`              |  51 |  27 |   0 |
|   `fstar::options(`                   |  81 |  14 |  25 |
|   `loop_invariant!(`                  |  29 |  53 |  14 |
|   **Subtotal W3**                     | **507** | **289** | **177** |
| **W4 Tests:** | | | |
|   `#[test]` / `#[cfg(test)]` adds | 24 | 0 | 27 |
|   New lines in `tests/` + `benches/` | 477 | 1,602 | 456 |
|   `cavp/` lines | (not measured separately) | (not measured) | (not measured) |
| **D1a Total F\* line changes (extracted + handwritten)** | +26,152 / −22,578 (183 files) | +23,735 / −22,583 (178 files) | +14,978 / −1 (48 files) |
| **D1c Total Rust source line changes** | (cumulative across W3+W4 + body changes; need scripts/measure-progress.py to disaggregate) | | |

## Per-axis observations

### Specs (W1)

ml-kem leads in spec source (3,341 LOC), reflecting a near-complete
hax-source `Hacspec_ml_kem` covering Compress, Ind_cca, Ind_cpa,
Invert_ntt, Matrix, Ntt, Polynomial, Sampling, Parameters, etc.
(11 modules visible in the extracted dir).

ml-dsa has 1,854 spec lines and corresponds to the 4,318 LOC
extracted spec layer noted in the ml-dsa milestones doc — confirming
the cross-audit's "spec already exists" correction.

sha-3 has 656 spec-Rust lines (4 files: `lib.rs`, `keccak_f.rs`,
`sponge.rs`, `sha3.rs`) — a small spec module by design, because
the FIPS 202 spec is concise (`keccak_f`, `sponge.{absorb,squeeze}`,
top-level digest fns).

### Proofs (W2)

The story is heaviest in **sha-3 equivalence proofs (8,701 LOC)** and
**ml-kem commute bridges (4,490 LOC)**. ml-dsa has 884 lines of
commute bridges, reflecting that ml-dsa's per-lane bridge layer
(`Commute.Chunk.fst`) is in place but the per-layer NTT/encoding
bridges haven't been written yet (per ml-dsa milestone doc rows 1-7).

The 147 new `let lemma_*` definitions on ml-kem vs 24 on ml-dsa is
striking and consistent with ml-kem's far-greater proof activity over
the 31-day window. sha-3 reports 0 because its proof style uses the
typed `let foo : Lemma (...)` form rather than the `lemma_*` prefix
convention.

### Annotations (W3)

The annotation count tells the same story: ml-kem 507 hax-lib attribute
additions, ml-dsa 289, sha-3 177. These are roughly proportional to
the calendar span (31:31:7 days) — actually sha-3 punches well above
its weight at 177 / 7 = 25.3 annotations/day vs ml-kem's 507 / 31 =
16.4 annotations/day. The sha-3 sprint is denser per-day.

### Tests (W4)

ml-dsa leads in test bytes (1,602 lines new in tests/), driven by
NIST KAT and Wycheproof test additions. ml-kem has 477 new lines in
tests/. sha-3 has the most `#[test]` additions in src (27) — reflects
inline conformance testing. None of the three crates use `cavp/`
extensively for the proof window.

### Authorship (D3)

Roughly 84-89% of commits are Claude-coauthored across all three
projects. The remaining 11-16% are commits from human authors
without Claude attribution — typically merges, infrastructure setup,
and the user's manual interventions.

The number of distinct user-correction events (D3b in the methodology)
is NOT yet logged in `wall-events.md` for any of the three crates.
Backfilling this from the `feedback_*` memories created during the
sprint + commit messages mentioning user corrections is part of the
final retrospective. Visible candidates so far:

  - User correction on ml-dsa Hacspec inventory (the prompt I wrote
    falsely claimed `Hacspec_ml_dsa.*` didn't exist; user corrected
    via the ml-dsa parallel agent's audit; resulted in the prompt
    rewrite).
  - User mandate `feedback_avoid_spec_mlkem` (ml-kem trait-opacify
    sprint mid-stream).
  - User mandate `feedback_rlimit_cap_800` (mid-sprint reaffirmation
    of rlimit cap; resulted in remediation ladder + AP-8 codification).
  - User SD4 codification (after USER-13 closure showed the
    reveal_opaque pattern was load-bearing).

## Difficulty distribution (preliminary)

Per the E1-E4 classification in the methodology. This is *preliminary*
— the final report will catalog every closed milestone. The numbers
below are based on the closed milestones I'm aware of.

| Tier | Count | Examples |
|---|---:|---|
| **E1 Mechanical** | ~10 | layer_3 inverse NTT mirror; per-fn lax→PF lift; constants admit-to-verified |
| **E2 Pattern adapt** | ~5 | layer_2 hacspec post (mirror of layer_3); sha3 SqueezeAPI per-block lemmas |
| **E3 Wall + spike** | ~3 | USER-13 (SD4 discovery); sha3 keccakf1600 AVX2/Neon backends; ml-dsa Encoding.Verification_key body discharge |
| **E4 Spec design** | ~2 | initial `Hacspec_ml_kem.Commute.{Bridges, Chunk}` design (predates the 31-day window); sha3 EquivImplSpec.* layer (8,701 LOC of equivalence proofs in 7 days — the major sha3 outcome) |

## Sibling-branch addendum

These branches contain proof work that was NOT merged into the
current three. They're separate proof projects and should be reported
separately if the goal is "all proof work since fork":

| Branch | Commits ahead of main | Span | Notes |
|---|---:|---|---|
| `proofs-cleanup` | 179 | 2026-03-24 → 2026-04-24 | Independent proof-cleanup effort; never merged. |
| `rust-spec` | 27 | 2026-03-10 → 2026-03-23 | Pre-fork proof effort; spec scaffolding. |
| `ml-dsa-above-trait` | 336 | 2026-03-24 → 2026-04-29 | Parallel ml-dsa proof effort; never merged into `ml-dsa-proofs`. |
| `sha3-proofs-focused` | 45 | (sha3-byteform-migration's older cousin) | Most work likely shared; needs cherry-pick attribution. |
| `trait-poststrengthen` | 235 | 2026-03-24 → 2026-04-27 | Common ancestor of `trait-opacify` + `ml-dsa-proofs` — already counted in their numbers above. |

The 336 commits on `ml-dsa-above-trait` are the most striking — a
parallel proof effort that didn't make it into the merged branch. A
final retrospective should reconcile: which fns/proofs are unique to
that branch vs. duplicated/superseded by `ml-dsa-proofs`.

## Limitations of this snapshot

  1. `wall-events.md` doesn't exist yet for any crate; user-intervention
     count is therefore approximated from `feedback_*` memory creation
     dates + visible course-corrections only. Backfill is part of the
     final report.
  2. The `assert_norm` / per-fn lax / opaque-wrapper effort isn't
     reflected in the W3 line count because it's mostly *removals* of
     existing annotations + reshapes (which `git diff` counts as
     deletions+insertions of similar size).
  3. Per-fn tier transitions (e.g., bounds → hacspec) need
     `--transitions` mode on the verification-status script — not yet
     implemented (see methodology "Tooling additions" #2).
  4. Difficulty classification (E1-E4) needs a per-milestone catalog,
     not the rough buckets above.
  5. The sibling-branch addendum is best-effort listing; the final
     report should compute git-cherry to find which sibling commits
     are duplicates of merged commits (so we don't double-count).

## Tooling status

  - ✅ `proofs/retrospective-methodology.md` — methodology written.
  - ✅ `proofs/initial-retrospective.md` — this file.
  - ❌ `scripts/cluster_sessions.py` — not yet implemented (D2b).
  - ❌ `generate_verification_status.py --transitions` — not yet
    implemented (per-fn tier transitions, D1e + Audit 2 lesson B2).
  - ❌ `scripts/measure-progress.py` — not yet implemented
    (orchestrator that produces the final report from D1-D5 commands +
    `wall-events.md`).
  - ❌ Per-crate `proofs/wall-events.md` — not yet seeded.

These four tooling items are the gap between this initial snapshot
and the final report. They should land before the three sprints
finish so the final report is fully reproducible.
