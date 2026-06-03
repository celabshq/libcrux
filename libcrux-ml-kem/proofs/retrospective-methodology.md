# Retrospective Measurement Methodology

How we measure "amount of proof work done" across the three libcrux
proof projects compared to libcrux `main`. Designed for **third-party
reproducibility**: every number in the final report is derived from a
named git command, a script invocation, or a hand-curated log file
with explicit fields. No "judgment call" metric goes into the report
without a documented basis.

## Lineage and time-window definition

All three current proof branches share a fork point at main commit
`70effe9c7` (2026-03-24 12:37 UTC, "Merge pull request #1367 from
cryspen/franziskus/psq-fuzz"). This is the **t₀** for the
retrospective. Per the user's request, the retrospective covers all
proof work *from t₀ onward*, including ancestors that have been
merged into the current branches.

```
            ┌─ rust-spec  (2026-03-10 → 2026-03-23, 27 commits, sibling — never merged)
main 70effe9c7 ──┤
(2026-03-24)     └─ trait-poststrengthen  (235 commits, IS ancestor of both)
                          │
                          ├──── trait-opacify         (current, 399 commits ahead, 31 days span)
                          ├──── ml-dsa-proofs         (current, 404 commits ahead, 31 days span)
                          ├──── ml-dsa-above-trait    (sibling, 336 commits, NEVER merged into ml-dsa-proofs)
                          └──── proofs-cleanup        (sibling, 179 commits, never merged)

main 70effe9c7 ──── sha3-proofs-focused  (sibling, 45 commits)
(2026-03-24)        └── sha3-byteform-migration  (current, 46 commits ahead, 7 days span)
```

For each crate's retrospective:
  - **Core measurement** = `git log main..HEAD` on the current branch
    (captures everything in the lineage that's been merged in).
  - **Sibling-branch addendum** = work on parallel proof branches that
    didn't merge. Counted separately and attributed; NOT included in
    the headline numbers but reported as "parallel work".

Sibling-branch list to enumerate in the final report:
  - `proofs-cleanup`, `proofs-cleanup-backup`, `rust-spec`,
    `ml-dsa-above-trait`, `agent-aa244240-above-trait`, `agent-ab0453d4-above-trait`,
    `franziskus/sha3-cleanup`, `protz/cleanup`, `alex/rust-spec-aeneas`,
    `alex/rust-spec-quick-hax`, `Ind-cca-spec`, `dev-add-serialize-specs-trait`.

---

## Four-axis work-type classification

Per the user's clarification, work splits into four kinds, measured
per-crate independently:

| Axis | Definition | Path / pattern |
|---|---|---|
| **W1 Specs** | Hand-written F\* spec definitions (the "what is correct" layer) | `specs/<crate>/proofs/fstar/extraction/Hacspec_<crate>.*.fst[i]` (definitions; `let` not `let lemma_`) |
| **W2 Proofs** | Hand-written F\* lemmas + equivalence proofs (the "this impl matches the spec" layer) | `specs/<crate>/proofs/fstar/commute/*.fst[i]`; sha-3 also: `crates/algorithms/sha3/proofs/fstar/equivalence/*.fst[i]`; PLUS Rust-embedded `hax_lib::fstar!()` blocks (counted as Rust additions but classified as proof work) |
| **W3 Annotations** | Hax-lib attribute lines added in Rust source (the "specifications attached to functions") | grep for `#[hax_lib::ensures(`, `#[hax_lib::requires(`, `#[hax_lib::fstar::verification_status(`, `#[hax_lib::fstar::options(`, `#[hax_lib::loop_invariant!(` in `<crate>/src/**/*.rs` |
| **W4 Tests** | New test code (cross-checks impl against spec at runtime) | `<crate>/tests/`, `<crate>/benches/`; `#[test]` blocks anywhere in `<crate>/src/`; `cavp/` test vectors |

Each axis has a deterministic measurement (see commands below). Lines
that overlap (e.g., a Rust `hax_lib::fstar!()` block contains F\* proof
text but lives in `.rs`) are credited to W2 (proofs) and noted in
parentheses as a Rust-source overlap.

### W1 — Specs commands

All three crates write the spec in **Rust** at `specs/<crate>/src/`
and hax-extract to `Hacspec_<crate>.*.fst`. The Rust source IS the
spec; counting the extracted F\* would double-count the same work.

(Build-system asymmetry: ml-kem and ml-dsa gitignore the extracted
F\*; sha-3 commits it. This affects D1a totals but not W1.)

```bash
cd <branch worktree>
git diff --shortstat main..HEAD -- 'specs/<crate>/src/'   # <crate> ∈ {ml-kem, ml-dsa, sha3}
```

### W2 — Proofs commands

```bash
# Commute bridges (all three crates):
cd <branch worktree>
git diff --shortstat main..HEAD -- \
  'specs/<crate>/proofs/fstar/commute/*.fst' \
  'specs/<crate>/proofs/fstar/commute/*.fsti'

# sha-3 also has equivalence/:
cd ~/libcrux-sha3-focused
git diff --shortstat main..HEAD -- \
  'crates/algorithms/sha3/proofs/fstar/equivalence/*.fst' \
  'crates/algorithms/sha3/proofs/fstar/equivalence/*.fsti'

# Rust-embedded F* proof blocks (lines added inside hax_lib::fstar! macros):
git log -p main..HEAD -- '<crate>/src/**/*.rs' | \
  awk '/hax_lib::fstar!\(/,/^[+-][[:space:]]*"#/' | grep -c '^+'
# (heuristic — see scripts/measure-progress.py for the exact AST-aware count)

# Lemma count (new lemmas added):
git diff main..HEAD -- 'specs/<crate>/' | grep -cE '^\+let lemma_'
```

### W3 — Annotations commands

```bash
# Per attribute kind, count the `^+#\[hax_lib::...` additions in Rust source:
cd <branch worktree>
for kind in 'ensures' 'requires' 'fstar::verification_status' \
            'fstar::options' 'loop_invariant'; do
  count=$(git log -p main..HEAD -- '<crate>/src/**/*.rs' \
          | grep -cE "^\+.*hax_lib::$kind")
  printf "  %-35s %4d\n" "$kind" "$count"
done
```

### W4 — Tests commands

```bash
# New #[test] / #[cfg(test)] blocks added in src:
git log -p main..HEAD -- '<crate>/src/**/*.rs' \
  | grep -cE '^\+\s*#\[(cfg\(test\)|test)\]'

# New files in tests/ or benches/:
git diff --stat main..HEAD -- '<crate>/tests/' '<crate>/benches/' | tail -1

# CAVP test vectors:
git diff --stat main..HEAD -- 'cavp/' 2>/dev/null | tail -1
```

---

## Five measurement dimensions

| # | Dimension | Quantifies | Reproducibility |
|---|---|---|---|
| **D1** | **Code volume** | Lines of F\* / Rust changed; new lemmas; new functions verified | `git diff --stat`; `grep` for `^let lemma_`; per-tier counts from `verification_status.md` |
| **D2** | **Time** | Calendar days; active session-hours; commit count; session count | `git log --format=%ci`; commits clustered by gap > 4 h = session boundary |
| **D3** | **Attribution** | Claude vs user contribution; user interventions per session | Git author + `Co-Authored-By` trailer; explicit `wall-events.md` log of user-guidance events |
| **D4** | **Difficulty** | Per-milestone effort; iterations per closure; structural walls hit | `--z3rlimit` value per fn; commit count per file; `feedback_*` memory count; `wall-events.md` USER-N entries |
| **D5** | **Quality** | Remaining admits; spec coverage; per-tier proof state | `verification_status.md` report; admit-site audit table |

---

## D1. Code volume — exact commands

### D1a. F\* lines added / deleted (per crate)

```bash
# ALL F*: extracted + hand-written + equivalence proofs.
cd <branch worktree>
git diff --stat main..HEAD -- '*.fst' '*.fsti' | tail -1
# Output: "N files changed, X insertions(+), Y deletions(-)"
```

### D1b. Hand-written F\* (the work-of-mind subset)

Excludes auto-extracted files; restricts to spec / equivalence /
extraction-patches paths. Per Audit 2 lesson B4, also restricts
attribution by `path × branch-of-origin` to avoid cross-pollination
inflation.

```bash
# ml-kem
cd ~/libcrux-trait-opacify
git diff --stat main..HEAD -- \
  'specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*.fst' \
  'specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*.fsti' \
  'specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.*.fst' \
  'fstar-helpers/fstar-bitvec/**/*.fst' \
  'libcrux-ml-kem/proofs/fstar/extraction-patches/' \
  | tail -1

# ml-dsa
cd ~/libcrux-ml-dsa-proofs
git diff --stat main..HEAD -- \
  'specs/ml-dsa/proofs/fstar/extraction/Hacspec_ml_dsa.*.fst' \
  'specs/ml-dsa/proofs/fstar/extraction/Hacspec_ml_dsa.*.fsti' \
  'specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.*.fst' \
  'libcrux-ml-dsa/proofs/fstar/extraction-patches/' \
  | tail -1

# sha-3
cd ~/libcrux-sha3-focused
git diff --stat main..HEAD -- \
  'specs/sha3/proofs/fstar/extraction/Hacspec_sha3.*.fst' \
  'specs/sha3/proofs/fstar/extraction/Hacspec_sha3.*.fsti' \
  'crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.*.fst' \
  'crates/algorithms/sha3/proofs/fstar/extraction-patches/' \
  | tail -1
```

Each crate gets *only* its own paths — sha3 `Hacspec_sha3.*` does NOT
count toward ml-kem's tally, even if it lives on the `trait-opacify`
branch via merge.

### D1c. Rust source changes (where ensures originate)

```bash
git diff --stat main..HEAD -- 'libcrux-ml-kem/src/**/*.rs' | tail -1   # ml-kem
git diff --stat main..HEAD -- 'libcrux-ml-dsa/src/**/*.rs' | tail -1   # ml-dsa
git diff --stat main..HEAD -- 'crates/algorithms/sha3/src/**/*.rs' | tail -1   # sha-3
```

### D1d. New lemmas added (count of `let lemma_*`)

```bash
# Across all hand-written F* directories per crate
git diff main..HEAD -- 'specs/<crate>/' | grep -cE '^\+let lemma_'
```

Excludes deletions (the `^+` anchor). Per `feedback_develop_locally_upstream_once`,
some lemmas live in consumer files (`Commute.Chunk`); both should be
counted via the path filter above.

### D1e. New functions verified / per-tier transitions

The `generate_verification_status.py --diff` mode produces the per-tier
delta. Audit 2 lesson B2: also enumerate per-fn tier transitions
(lax→PF, PF→math, bounds→hacspec, etc.) — see "Tooling additions" below.

---

## D2. Time — exact commands

### D2a. Calendar window

```bash
git log --format='%ci' main..HEAD | tail -1   # first proof commit
git log --format='%ci' main..HEAD | head -1   # last proof commit
```

### D2b. Active session-hours (estimate)

Cluster commits with gap < `SESSION_GAP=4h` into "sessions". A session's
duration = first-commit-of-session → last-commit-of-session, capped at
8 h (working-day cap). Total active hours = Σ session durations.

```bash
# script: scripts/cluster_sessions.py (see "Tooling additions")
python3 scripts/cluster_sessions.py --crate <name> --gap 4h --cap 8h
```

### D2c. Distinct sessions / commits

```bash
git log --oneline main..HEAD | wc -l   # commit count
# session count = output of cluster_sessions.py
```

---

## D3. Attribution — exact methods

### D3a. Claude vs human commit count

```bash
git log --format='%an|%ae' main..HEAD | sort -u
# author = "Karthikeyan Bhargavan" → human
# co-author trailer "Claude Opus 4.7" → Claude-assisted

# Count Claude-assisted commits:
git log --format='%H' main..HEAD | while read sha; do
  git show --format='%(trailers:key=Co-Authored-By)' --no-patch $sha
done | grep -c 'Claude'
```

### D3b. User interventions / course corrections

This is the only metric NOT auto-derivable. Maintain a per-crate
`proofs/wall-events.md` log with structured rows:

```
| Date | Session | Event-type | Description | Outcome |
|---|---|---|---|---|
| 2026-04-29 | trait-opacify-A5 | user-guidance | "use SD3 pattern from b7b49c358" | unblocked USER-13 |
| 2026-04-30 | sha3-S1 | user-correction | "Hacspec_ml_dsa.* exists, don't redesign" | rewrote ml-dsa prompt |
```

Event-types (closed vocabulary):
  - `user-guidance` — user supplied a strategy / pattern that unblocked work.
  - `user-correction` — user corrected a factual error in Claude's plan.
  - `user-rule` — user established a new rule (e.g. `feedback_rlimit_cap_800`).
  - `wall-encountered` — Claude hit a structural wall and filed USER-N.
  - `wall-resolved` — wall closed, with the resolution (Claude / user / both).

The count + breakdown of these events IS the attribution measurement.

### D3c. New `feedback_*` memories generated

Each memory under `~/.claude/projects/-Users-karthik-libcrux/memory/feedback_*.md`
that was added during the proof window is a *codified learning* —
implicitly a wall encountered + resolved. Count = proxy for "places
Claude needed to learn something the user knew (or that no one knew)."

```bash
# Per memory: when was it added, who wrote it
ls -t ~/.claude/projects/-Users-karthik-libcrux/memory/feedback_*.md
```

---

## D4. Difficulty — proxies

### D4a. Per-fn `--z3rlimit` (proxy for SMT effort)

```bash
# Extract the rlimit per fn from the source annotations
grep -nE '#\[hax_lib::fstar::options.*--z3rlimit' libcrux-ml-kem/src/**/*.rs
```

Bands:
  - **Easy**: rlimit ≤ 100 (default tier).
  - **Medium**: 200-400.
  - **Hard**: 800 (the cap per `feedback_rlimit_cap_800`).
  - **Very hard / flake debt**: > 800 (PROHIBITED post `feedback_rlimit_cap_800`;
    any pre-existing > 800 should be reduced; count = "rlimit debt remaining").

### D4b. Iterations per milestone closure

```bash
# Commits touching a specific file
git log --oneline main..HEAD -- libcrux-ml-kem/src/invert_ntt.rs | wc -l
```

A milestone that took 5+ commits to close was harder than one that took 1.

### D4c. USER-N task count

Each `USER-N` ticket in `MLKEM_STATUS.md` / `MLDSA_STATUS.md` is a
named structural wall. Count + status (open / closed):

```bash
grep -E "^### USER-[0-9]+" libcrux-ml-kem/MLKEM_STATUS.md | wc -l
```

### D4d. Difficulty classification — 4-tier scheme

For the retrospective, classify each closed milestone as:

| Tier | Criteria | Example |
|---|---|---|
| **E1 — Mechanical** | Pattern-match an existing closure; per-fn rlimit ≤ 200; no new spec; ≤ 2 commits | `invert_ntt_at_layer_3` (mirror of layer_1) |
| **E2 — Pattern adapt** | Apply a known pattern to a different shape; ≤ 5 commits; rlimit 200-400; possibly 1 helper lemma | Layer_2 hacspec post (today, mirror of layer_3) |
| **E3 — Wall + spike** | Hit a Z3 wall; required ≥ 1 anti-pattern discovery (`feedback_*`); ≥ 5 commits; rlimit 400-800 | USER-13 (SD4 discovered) |
| **E4 — Spec design** | Required defining a new `Hacspec_<crate>.*.fst` module or major bridge restructuring; > 10 commits | `Hacspec_ml_kem.Commute.{Bridges, Chunk}` initial |

---

## D5. Quality — derived from `verification_status.md`

### D5a. Per-tier counts (current state)

```bash
cd <crate-root> && python3 proofs/generate_verification_status.py
grep -A8 "## Summary" proofs/ml_kem_verification_status.md
```

### D5b. Outstanding admits

```bash
grep -A30 "## Body-admit sites" proofs/ml_kem_verification_status.md
grep -B1 "ADMIT_MODULES" proofs/fstar/extraction/Makefile
```

### D5c. Spec coverage

For each `Hacspec_<crate>.*` function, what fraction of impl-side
callers cite it in their `ensures`?

```bash
# enumerate Hacspec functions
grep -hE "^let [a-z_][a-zA-Z0-9_]+" specs/<crate>/proofs/fstar/extraction/Hacspec_<crate>.*.fst | sort -u > /tmp/spec-fns.txt
# enumerate impl-side citations
grep -hE "Hacspec_<crate>\.\w+\.\w+" libcrux-<crate>/src/**/*.rs | sort -u > /tmp/cited.txt
# coverage = |cited| / |spec-fns|
```

---

## Tooling additions (build for the final report)

1. **`scripts/cluster_sessions.py`** — input: a git-log timestamp
   stream; output: list of (start, end, n_commits) sessions clustered
   by gap > 4 h. Reproducible methodology.

2. **`generate_verification_status.py --transitions PREV CURR`** —
   new mode (per Audit 2 B2). Given two snapshots, output the per-fn
   tier transitions (`fn_name | prev_tier | curr_tier`). Aggregate
   into a transitions matrix:

   ```
   from\to     unverified  lax    pf     math   bounds  hacspec
   unverified  N           N      N      ...
   lax         N           N      N      ...
   ...
   ```

   This makes "23 fns moved up" and "5 fns regressed" auditable.

3. **`scripts/measure-progress.py`** — top-level orchestrator. Runs
   D1a-e + D2a-c + D3a + D4a-c + D5a-c on each of the three crates,
   produces the retrospective table. Reads `wall-events.md` from each
   crate for D3b. Outputs `proofs/retrospective-summary.md`.

4. **Per-crate `proofs/wall-events.md`** — initially seeded with the
   USER-N entries from MLKEM_STATUS.md / MLDSA_STATUS.md and the
   feedback_* memories that came from this work; maintained going
   forward. Closed-vocabulary event-types per D3b.

5. **`proofs/initial-retrospective.md`** — populated NOW from current
   git state; the final retrospective overwrites it with the full
   numbers when the three sprints finish.

---

## Initial report (NOW) vs. Final report (sprint completion)

The **initial report** uses what's in git as of today. Three weaknesses
to call out:

  i. `wall-events.md` is sparse — has to be backfilled from
     `feedback_*` memories + commit messages + agent deliverable
     reports. Not every wall is recorded yet; the initial number is
     a lower bound.
  ii. Session-hour estimate uses commit-cluster heuristic (gap < 4 h
     = same session). Underestimates "thinking time" between commits
     in a long session; overestimates if a session was interrupted
     for unrelated work.
  iii. Difficulty classification is judgment-based for the E1-E4 tiers
     even with the criteria. Two reviewers may disagree on edge cases.
     Mitigation: report the criteria + per-milestone classification
     so a third party can re-classify if they disagree.

The **final report** adds:
  - Cumulative retrospective table over the full proof window.
  - Per-fn transition matrices (D1e).
  - Wall-events log with all events from sprint kickoff through final
    commit.
  - Difficulty histogram per crate.
  - "What I'd do differently" narrative — synthesized from the
    `feedback_*` memories created during the sprint.

---

## Acceptance criteria for the methodology

A third party reading the final report should be able to:

  1. Reproduce every number in the table by running the documented
     git command or script invocation.
  2. Re-classify any milestone's difficulty (E1-E4) using the criteria
     in D4d, even if they disagree with the original classification.
  3. Audit the wall-events log against the commit history (each
     USER-N event should be traceable to at least one commit message
     mentioning that USER-N).
  4. Distinguish "Claude wrote it" from "Claude wrote it after user
     guidance" via the wall-events log + commit author trailers.
