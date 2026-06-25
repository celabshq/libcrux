# Sprint Learnings — synthesis of two parallel proof-progress audits

Two independent audits were performed today (2026-04-30) on the
libcrux proof state across ML-KEM (`trait-opacify`), ML-DSA
(`ml-dsa-proofs`), and SHA-3 (`sha3-proofs-focused`). This document
synthesizes both, calling out where they agree, where one filled a
gap the other missed, and where they disagreed.

These learnings are **load-bearing for the next sprint plan** and
are referenced from the per-crate agent prompts at
`proofs/agent-prompt-{ml-kem,ml-dsa,sha3}.md`.

## Agreement (4 learnings — both audits independently arrived at)

### A1. Extraction is the foundational gate

Both audits identified that the proof state is bottlenecked at the
**API surface**, not the math layer. The crypto cores are largely
verified — what's missing is the API extraction:

- SHA-3: `lib.rs` filtered out of hax → 6 top-level digest fns
  (sha224/256/384/512, shake128/256) show as Unverified despite
  `Hacspec_sha3.{Keccak_f, Sponge}` being fully proven below.
- ML-KEM: `mlkem.rs` filtered out → 19+ KEM API fns Unverified
  despite the entire crypto core being verified.
- ML-DSA: `lib.rs` enters the inventory as Unverified on the SHA-3
  branch.

**Sprint action**: fix hax extraction config for `lib.rs` /
`mlkem.rs` BEFORE any new spec/proof work. ~1 session per crate.
Highest LOC-of-impact-per-hour available.

### A2. Tier counts undercount when proofs live outside function-level ensures

The `generate_verification_status.py` script classifies fns by
inspecting their `#[ensures(...)]` text. It misses:

- SHA-3's `keccakf1600` permutation, proven equivalent to
  `Hacspec_sha3.Keccak_f.keccak_f` via separate
  `EquivImplSpec.Keccakf.{Portable, Avx2, ChiFold}.fst` lemmas
  (~6,848 LOC of equivalence proofs invisible to the script).
- ML-KEM's `invert_ntt_montgomery`, whose ensures cites
  `Hacspec_ml_kem.Invert_ntt.ntt_inverse_butterflies` — but body is
  admitted (USER-15). Script reports Lax, but the SPEC is correct;
  only the proof is the gap.
- ML-KEM's body-admit functions where the post is hacspec-shaped
  but the body has `--admit_smt_queries true`.

**Sprint action**: when designing new proofs, prefer **function-level
ensures over external `EquivImplSpec.*` lemmas** unless there's a
structural reason for the external form. Surface existing external
lemmas (e.g., SHA-3's keccakf equivalence) as function-level ensures
so the audit/reporting story matches reality (~1 session per fn).

### A3. Spec module design must precede ensures wiring

ML-DSA cannot be "proven correct vs hacspec" because
`Hacspec_ml_dsa.{Ntt, Encoding}.*` does not exist — there's nothing
to cite. The bounds-only ensures on ml-dsa NTT are not a proof gap;
they're a **spec gap**. Conversely, ML-KEM's progress was unblocked
by `Hacspec_ml_kem.Commute.{Bridges, Chunk}.fst` (3,504 LOC of
hand-written spec) being designed first.

**Sprint action**: dedicate the first ~1 sprint per ml-dsa milestone
*tier* to designing the spec module (`Hacspec_ml_dsa.Ntt.*`,
`Hacspec_ml_dsa.Encoding.*`). No ensures wiring on a function until
the spec it would cite exists.

### A4. `VERIFIED_MODULES` / `ADMIT_MODULES` discipline is undervalued

ML-DSA's audit found 119 functions sitting in admit-by-Makefile-
exclusion that already had proper ensures and just weren't on
`VERIFIED_MODULES`. ML-KEM still has `Ind_cpa.fst` with all 21 fns
Lax for the same reason (whole module on the admit list).

**Sprint action**: every sprint kickoff should audit
`ADMIT_MODULES` / `VERIFIED_MODULES` first. Often dozens of fns
become Verified by Makefile-discipline alone, no proof work needed.

## Complementary insights (each audit caught something the other missed)

### Audit 2 caught (incorporated into the prompts)

**B1. Forward NTT is THE next high-leverage trait-opacify application
(ML-KEM).** Bounds-only ensures already exist on `ntt_at_layer_*`;
the hacspec citations are commented out as TODO. Adding the citation
propagates through Portable + Avx2 backends for free, mirroring the
+27 / +23 hacspec gain the inverse direction got from the same
pattern. Audit 1 had this as priority #4 of many; Audit 2 elevates
it to *the* Week 2-3 deliverable.

**B2. Per-fn tier-transition reports** (lax→PF, PF→math, bounds→
hacspec, etc.). Current script's net deltas can show "Δ Bounds = −9"
when actually 9 fns moved Bounds→Hacspec — the upgrade looks like
regression in the per-tier net. Fix: instrument the script to
preserve per-fn matching across two snapshots and report
transitions, not just aggregated deltas.

**B3. Avoid big-axiom-bridge designs.** The
`Hacspec_ml_kem.Serialize.HacspecBridge` approach was abandoned
after 11 commits on `agent/phase-7c-serialize`. Wave-B took the
per-width `panic_free` strategy and that's what produced the actual
gains (USER-9a/b: widths 5+11 done). Lesson: when tempted to write
a "general" bridge axiom that connects layer N+1 to layer N for all
instances at once, don't. Pick one width / one instantiation, prove
that, generalize only after the second falls into the same shape.

**B4. Cross-pollination inflates per-branch metrics if you sum
naively.** ML-KEM branch's "hand-written F\* of 5,988 LOC" includes
~2,484 LOC of SHA-3 team's specs that landed via merges; ML-DSA
included 1,166 LOC of ML-KEM team's `Commute.Chunk`. Without
filtering, "lines I wrote" gets credited to whoever's branch was
rebased latest. Fix: attribute work by `path × branch-of-origin`,
or restrict the metric to `specs/<crate>/` and
`proofs/fstar/equivalence/` only.

### Audit 1 caught (incorporated into the prompts)

**C1. Rule SD4 — no GLOBAL `reveal_opaque (\`%P) (P)` in loop bodies.**
A single line in `invert_ntt_at_layer_2` caused a 153 s top-1 wall;
removing it dropped the verification to 2.1 s. Same fix on layer_3
dropped the rlimit from 800+context_pruning+split_queries to 400.
Pre-flight grep:
`grep -rn 'reveal_opaque.*%[^)]*) *([A-Z][A-Za-z_.]*) *)$' src/`

**C2. Hint-cache cascades on ensures upgrades.** Strengthening one
fn's ensures shifts line numbers and breaks downstream hint replay.
Saw this on layer_3 (rlimit 200 → 400 needed after layer_2's line
shift) and `Vector.Avx2.fst::impl_3` (had to delete the hint file
entirely). After every milestone closure that strengthens an ensures,
re-record hints for the module + at least one consumer.

**C3. Hax codegen fragility.** Three recurring issues that re-emerge
after every `python3 hax.py extract`:
- Duplicate `noeq` on `Vector.Neon.Vector_type.fsti` (gitignored;
  needs `sed -i '' '7,8d'`).
- Trailing `_` mangling on F\* segments whose Rust ident ends in a
  digit (`ml_dsa_44` → `Libcrux_ml_dsa.Ml_dsa_44_.fst`).
- Ancestor coverage when nested mods extract to multiple `.fst`.

Long-term: invest 1 session in a robust extraction post-processor.

**C4. Cross-crate parallelism is genuine.** Three independent
worktrees with no contention between branches; agents in parallel
sessions can run independently.

## Disagreements / different emphasis

| Topic | Audit 1 | Audit 2 | Resolution |
|---|---|---|---|
| **Sprint kickoff sequence** | Audit existing code (SD4 reveal_opaque sweep, codegen workarounds) | Extraction config first (lib.rs / mlkem.rs), THEN spec/proof work | **Audit 2 wins.** Extraction is foundational and ~1 session per crate; SD4 audit is incremental. Lead with extraction. |
| **"Hacspec = highest tier" is the right metric** | Yes, that's what the script tracks | Net per-tier counts can mask within-tier upgrades; need per-fn transitions | **Audit 2 wins.** Net deltas mislead — Bounds→Hacspec shows as "Δ Bounds = −1, Δ Hacspec = +1", looks like wash. Per-fn transition tracking is the real fix. |
| **Cheapest next milestone** | Layer_2 hacspec upgrade (closed today, commit `4672cc005`) | Surface SHA-3 EquivImplSpec lemmas as function-level ensures; mlkem.rs / lib.rs extraction | **Both right for different sessions.** Layer_2 was cheapest for ml-kem alone; Audit 2's are cheapest cross-crate. |
| **Big-axiom-bridge** | Documented `agent/phase-7c-serialize` as "abandoned" | Drew the forward-looking lesson | **Audit 2 wins.** Codify as an anti-pattern (AP-2 in the prompts). |

## Recommended next-sprint sequence (synthesized)

  - **Day 1**: VERIFIED_MODULES / ADMIT_MODULES audit on ML-KEM
    (Ind_cpa.fst) + ML-DSA. Cheap, mechanical.
  - **Day 2**: Extract `lib.rs` (sha3) and `mlkem.rs` (kem) — the
    biggest mechanical move available. Closes dozens of fns from
    Unverified to at-least-PF.
  - **Day 3-5**: Surface SHA-3's `EquivImplSpec.Keccakf.*.fst`
    lemmas as function-level ensures (3-5 functions). Audit picks up
    the real proof state.
  - **Week 2**: Design `Hacspec_ml_dsa.Ntt.*`. No ensures wiring on
    ML-DSA NTT until this lands.
  - **Week 2-3**: ML-KEM forward NTT trait-opacification (mirror the
    pattern that worked for inverse NTT layers 1, 3).
  - **Week 3**: Instrument `generate_verification_status.py` to
    report per-fn transitions (lax→PF, PF→math, bounds→hacspec)
    instead of just net-per-tier counts.

## Memory-store candidates from this synthesis

Two new feedback memories proposed:

  1. **`feedback_extraction_first`** — Always audit hax extraction
     config (`hax.py`/`hax.sh` `-i` rules) before any spec wiring
     sprint. Filtered-out modules (`lib.rs`, `mlkem.rs`,
     `ind_cca/incremental*.rs`) are worse than admitted modules —
     they have no proof of any kind, including panic-freedom. Fix
     extraction first; the API surface flips from Unverified to
     at-least-PF mechanically.

  2. **`feedback_hacspec_spec_first`** — `Hacspec_<crate>.<Layer>.*`
     spec module design MUST precede function-level ensures citing
     it. Bounds-only ensures are not a proof gap; they're a spec
     gap. Don't add `Hacspec_ml_dsa.Ntt.foo` citations to a function
     before the spec exists. Path: define spec → test it's well-formed
     → wire ensures → prove.

Existing memories that this audit confirmed are still load-bearing:
SD3/SD4, lane split, reveal_opaque targeting, fstar-mcp session
lifecycle, touch-unchanged-`.checked`, no-cache-nuke,
for-loop-param-unshadowing, drive-to-the-top spike.
