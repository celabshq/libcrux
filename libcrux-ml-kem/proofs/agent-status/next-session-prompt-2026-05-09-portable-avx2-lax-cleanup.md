# Next-session prompt — flip remaining lax in Portable + Avx2 + Generic-impl layer

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** `1c94eae53` (or later)
**Worktree:** `/Users/karthik/libcrux-trait-opacify`
**Out of scope (don't touch):** Neon backend (entire backend in `ADMIT_MODULES`),
`Ind_cca.Incremental.*` / `Mlkem*.Incremental.*` / multiplexing (above ind_cca),
the rejection-sampling while-loop functions in `sampling.rs`
(`sample_from_uniform_distribution_next`, `sample_from_xof`).

## Goal

Eliminate **22 lax sites** across the impl-side stack at and below `ind_cca`,
plus the Portable and Avx2 vector backends. Each site flips lax (or
`--admit_smt_queries true` / body-`admit ()`) → `panic_free` (or stronger).
"lax" here is the union the script `proofs/generate_verification_status.sh`
uses: explicit `verification_status(lax)`, `--admit_smt_queries true`,
`#[hax_lib::opaque]`, body `admit ()`, and `ADMIT_MODULES` membership.

**Definition of done (per stage acceptance):**

```bash
$ bash libcrux-ml-kem/proofs/generate_verification_status.sh
# verification_status.md table: counts in the rows below should match:
#   Generic/serialize     0 lax
#   Generic/ntt           0 lax
#   Generic/invert_ntt    0 lax
#   Generic/sampling      ≤ 2 lax  (only the rejection-sampling pair stays)
#   Portable/vector       0 lax
#   Avx2/ntt              0 lax
#   Avx2/serialize        0 lax
#   Avx2/vector           0 lax
```

End-state grep:

```
$ grep -rn "verification_status(lax)\|admit_smt_queries true" \
    libcrux-ml-kem/src/serialize.rs libcrux-ml-kem/src/ntt.rs \
    libcrux-ml-kem/src/invert_ntt.rs libcrux-ml-kem/src/vector/portable* \
    libcrux-ml-kem/src/vector/avx2*
# expect 0 hits
$ grep -rn "verification_status(lax)" libcrux-ml-kem/src/sampling.rs | wc -l
# expect 2  (sample_from_uniform_distribution_next + sample_from_xof — left aside)
```

Per-stage build gate: the changed file's `make check/<Module>.fst` returns
`rc=0`. Final gate: full `make` from `proofs/fstar/extraction` rc=0.

## Read first (non-negotiable)

1. **`~/.claude/skills/fstar-for-libcrux/README.md`** — Rules 1–8.
   Pipe `make` to `/tmp/*.log` + grep, never `Read` a full make log.
   Use `--admit_except <fn-fully-qualified>` for fast inner-loop iteration
   on a single function; switch to full-file `make` for stage acceptance.
   `fstar-mcp` for symbol lookups (avoid token waste reading whole `.fsti`s).

2. **`~/.claude/skills/smtprofiling/SKILL.md`** — when a query times out at
   `rlimit 400` or shows `incomplete quantifiers` and the structure is
   non-obvious, run `--log_queries --z3refresh` + `z3 smt.qi.profile=true` to
   identify the dominant quantifier. The 2026-05-05 session diagnosed the
   `is_bounded_poly` equation cascade (66k QI) and the
   `Rust_primitives.Slice.array_from_fn` refinement cascade (24k QI) this way.

3. **`MEMORY.md`** — most relevant entries:
   - `feedback_panic_free_vs_lax` — `panic_free` admits ensures, `lax` admits
     everything. Default new flips target `panic_free` first.
   - `feedback_dual_smtpat_opaque_atom` — multi-pattern `[SMTPat (Seq.index
     arr i); SMTPat (opaque_atom b arr)]` fires bidirectionally; the existing
     `lemma_is_bounded_polynomial_{vector,matrix}_elim` are good models.
   - `feedback_proof_debug_budget` — 30–60 min hard cap per function; mark
     blocker and move on.
   - `feedback_no_manual_edits_extracted` — all changes via Rust source +
     `hax_lib::fstar::{before,after,replace}`. Never edit extracted `.fst`/
     `.fsti` directly.
   - `feedback_grep_make_output` — never `Read` a full F* `make` log.
   - `feedback_rlimit_cap_800` — hard cap 800 (≤400 with `--split_queries
     always`). Bumping is a smell; restructure.
   - `feedback_smtpat_percent_above_trait` — SMTPats with raw `%` leak
     non-linear arithmetic. Keep SMTPat bodies opaque-atom-only or plain-bound.

4. **Recent successful commits** (read the diffs of the parent commit shape;
   the commit messages document the patterns to mirror):
   - `1c94eae53` — `deserialize_then_decompress_u` lax → panic_free. Used
     `for i in 0..K` (not `cloop`+`chunks_exact`), `hacspec_ml_kem::parameters::createi`
     instead of `from_fn`, and a localized `--using_facts_from` band-aid.
     **Stage 0 below builds on the same band-aid; Stage 5 retires it.**
   - `dff9669ff` — `encrypt_unpacked` flipped to `--z3rlimit 400 --split_queries
     always` (drop `--ext context_pruning`). The `update_at_range` Subtyping
     check at the encrypt_c2 call site needed this.
   - `b9eee5838` (Phase E push 3) — `decrypt_unpacked` lax → panic_free
     pattern; `decrypt` body chain. Useful template for delegate-style flips.

## The 22-fn target list

Lines are at-tip-of-`1c94eae53`; verify with `grep -rn` on entry.

### Generic (src/, ind_cca and below) — 7 sites

| File:Line | Function | Mechanism | Stage |
|---|---|---|---|
| `serialize.rs:260` | `compress_then_serialize_11` | `verification_status(lax)` | 1 |
| `serialize.rs:346` | `compress_then_serialize_5` | `verification_status(lax)` | 1 |
| `serialize.rs:425` | `deserialize_then_decompress_11` | `verification_status(lax)` | 1 |
| `serialize.rs:528` | `deserialize_then_decompress_5` | `verification_status(lax)` | 1 |
| `ntt.rs:564` | `ntt_vector_u` | `--admit_smt_queries true` | 3 |
| `invert_ntt.rs:552` | `invert_ntt_at_layer_4_plus` | `--admit_smt_queries true` | 3 |
| `invert_ntt.rs:666` | `invert_ntt_montgomery` | `--admit_smt_queries true` | 3 |

### Portable backend — 4 sites

| File:Line | Function | Mechanism | Stage |
|---|---|---|---|
| `vector/portable.rs:445` | `op_ntt_layer_1_step` | `--admit_smt_queries true` | 2 |
| `vector/portable.rs:684` | `op_inv_ntt_layer_1_step` | `--admit_smt_queries true` | 2 |
| `vector/portable.rs:974` | `Operations::from_bytes` impl | body `admit ()` | 4 |
| `vector/portable.rs:994` | `Operations::to_bytes` impl | body `admit ()` | 4 |

### Avx2 backend — 11 sites

| File:Line | Function | Mechanism | Stage |
|---|---|---|---|
| `vector/avx2/serialize.rs:5` | `serialize_1` | `verification_status(lax)` | 2 |
| `vector/avx2/serialize.rs:352` | `serialize_5` | `verification_status(lax)` | 2 |
| `vector/avx2/serialize.rs:468` | `deserialize_5` | `verification_status(lax)` | 2 |
| `vector/avx2/serialize.rs:694` | `serialize_11` | `verification_status(lax)` | 2 |
| `vector/avx2/serialize.rs:705` | `deserialize_11` | `verification_status(lax)` | 2 |
| `vector/avx2/ntt.rs:200` | `inv_ntt_layer_1_step` | `verification_status(lax)` | 2 |
| `vector/avx2/ntt.rs:336` | `ntt_multiply` | `verification_status(lax)` | 2 |
| `vector/avx2.rs:65` | `to_bytes` | `verification_status(lax)` | 4 |
| `vector/avx2.rs:116` | `op_cond_subtract_3329` | tagged lax (audit) | 4 |
| `vector/avx2.rs:391` | `op_ntt_layer_1_step` | tagged lax (audit) | 4 |
| `vector/avx2.rs:994` | `op_serialize_1` | `verification_status(lax)` | 4 |

## Stages

Recommend tackling in this order — each stage standalone, commits between.

### Stage 0 — `is_bounded_poly` opacification spike (foundational, optional)

The 2026-05-05 session diagnosed that the `is_bounded_poly` equation cascades
~66k QI in any function that traverses its body alongside an `array_from_fn`
result (which leaks the `Tm_refine_8143…` refinement). The clean fix is to
add `[@@ "opaque_to_smt"]` to `is_bounded_poly` in `polynomial.rs` and
provide:

- `lemma_is_bounded_poly_intro` (per-coefficient `is_bounded_vector` forall →
  opaque atom)
- SMTPat'd `lemma_is_bounded_poly_elim` (atom + `Seq.index p.f_coefficients i`
  → per-coefficient bound)
- Update `is_bounded_poly_higher` to add `reveal_opaque` for both bounds.

The spike got `Polynomial.Spec`, `Ind_cpa`, `Serialize`, `Matrix`, `Sampling`,
`Invert_ntt` verifying clean but **broke `Ntt.fst::ntt_at_layer_*`** (~4
functions). The break is mechanical: each captures `_re_init = re.coefficients`,
which defeats the SMTPat trigger pattern (the SMTPat looks for `Seq.index
p.f_coefficients i`, but `_re_init` is already the coefficients array — Z3
can't trigger). Fix per function: add `reveal_opaque (`%is_bounded_poly)
(is_bounded_poly $_initial_coefficient_bound $re)` near the start of the
function, plus an explicit `lemma_is_bounded_poly_intro` at the end before
return. Mechanical change, ~8–10 functions across `ntt.rs` and
`invert_ntt.rs`.

If Stage 0 lands, it retires the `--using_facts_from` band-aid on
`deserialize_then_decompress_u` (commit `1c94eae53`) and likely makes
Stages 1–3 easier (cleaner SMT context for is_bounded_poly facts).

**Recommendation:** Skip Stage 0 unless you find Stages 1–3 fragile. The
band-aid is fine and the opacification can be a separate sprint.

### Stage 1 — serialize.rs `_5` / `_11` polynomial wrappers (4 sites)

**Pattern:** the `_4` sibling at `serialize.rs:484` (`deserialize_then_decompress_4`)
verifies clean under `panic_free` with options `--z3rlimit 400 --ext
context_pruning --split_queries always`. Its body uses
`spec::is_bounded_poly_higher` to widen `0 → 4095` after `ZERO()`, then per
iteration calls `lemma_decompress_post_to_is_bounded_vector(&re.coefficients[i],
4095)` to extract the bound. Same template applies to `_5` (also 4095 bound)
and `_10`/`_11` (3328 bound).

For `compress_then_serialize_5` / `_11`: mirror `compress_then_serialize_10`
(at `serialize.rs:259`-ish) — this one is panic_free. The `_5`/`_11` versions
have the same `Vector::compress_<n>` + `Vector::serialize_<n>` shape, just
different `<n>`.

**Per-fn budget:** 30–45 min. If a single fn exceeds 60 min, mark FOLLOW-UP
and proceed.

**Stage acceptance:** `make check/Libcrux_ml_kem.Serialize.fst rc=0`,
`grep -c "verification_status(lax)" libcrux-ml-kem/src/serialize.rs` → 0.

### Stage 2 — Avx2 vector trait methods (5 sites in `serialize.rs` + 2 in `ntt.rs`)

The trait-method `Vector::serialize_5/_11` and `Vector::deserialize_5/_11`
have **prior BitVec lemma work** in commits `107c76641`, `2deb01199`,
`e5c4a6f49`, `a51ddbfc3`. Read those diffs first — they likely have most of
the heavy lifting done; the remaining work is wiring the lemma into a
panic_free `verification_status` flip + sometimes a small ensures
strengthening.

`Vector::serialize_1` (line 5) and the parallel Portable `op_ntt_layer_1_step`
/ `op_inv_ntt_layer_1_step` follow the standard
`Tactics.GetBit.prove_bit_vector_equality'` pattern (see other proven
`serialize_*` in `vector/avx2/serialize.rs` for templates).

`Avx2::ntt_multiply` and `Avx2::inv_ntt_layer_1_step` are the SIMD per-vector
versions of the polynomial-level NTT layers. They delegate to intrinsics with
known specs in `core_models`.

**Per-fn budget:** 45–60 min for serialize, 60–90 min for the NTT layer
fns (heavier).

**Stage acceptance:**
- `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst rc=0`
- `make check/Libcrux_ml_kem.Vector.Avx2.Ntt.fst rc=0`
- `make check/Libcrux_ml_kem.Vector.Portable.Ntt.fst rc=0`

### Stage 3 — NTT/invert_ntt body admits (3 sites)

Three `--admit_smt_queries true` admits gate the layer-4+ chain. They are
deeply intertwined:

1. `ntt_vector_u` (`ntt.rs:564`) — bound preservation across `ntt_at_layer_*`
   chain. Already has the bound ensures; functional ensures uses
   `Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain ${re}_future ==
   Hacspec_ml_kem.Ntt.ntt (...)`. The body admits because the per-layer
   `to_spec_poly_plain` chaining is unproven at layer 4+.

2. `invert_ntt_at_layer_4_plus` — symmetric to `ntt_at_layer_4_plus` (which
   IS proven). Pattern from the proven side should port over.

3. `invert_ntt_montgomery` — top-level invert NTT driver; admits the body
   while the bridge from Hacspec inverse-NTT spec is being built. See
   `proofs/agent-status/next-session-prompt-2026-05-06-lax-elimination.md`
   for prior diagnosis (USER-14 family).

**Order:** start with `invert_ntt_at_layer_4_plus` (mirror `ntt_at_layer_4_plus`'s
proof, which is in `ntt.rs` and has full ensures). Then `invert_ntt_montgomery`.
Then `ntt_vector_u` last (uses the layer-4+ chain as a building block).

**Per-fn budget:** 90 min — these are the heaviest in the sprint. If a fn
exceeds 90 min, document the specific blocker (Z3 query, missing lemma) in
`agent-status/sprint-2026-05-09-rollup.md` and move on.

**Stage acceptance:**
- `make check/Libcrux_ml_kem.Ntt.fst rc=0`
- `make check/Libcrux_ml_kem.Invert_ntt.fst rc=0`

### Stage 4 — `op_*` glue + `to_bytes` / `from_bytes` (6 sites)

Smaller bodies, mostly:
- `op_cond_subtract_3329`, `op_serialize_1`, `op_ntt_layer_1_step` (Avx2):
  panic-freedom + delegation to proven intrinsics (`mm256_*` from
  `core_models`).
- `to_bytes` (Avx2, line 65): the existing comment cites
  `update_at_range bytes [0..32]` panic-freedom failing at rlimit 200 due to
  an `&mut` slice modeling issue. Try the now-standard `--split_queries
  always` flip (per encrypt_unpacked precedent). If that doesn't resolve, the
  blocker is in hax-lib's `Rust_primitives` modeling — surface to user.
- `Operations::from_bytes`/`to_bytes` (Portable, body `admit ()` at lines 974
  / 994): same `&mut` slice issue likely; treat in parallel with Avx2
  `to_bytes`.

**Per-fn budget:** 30 min. If `to_bytes` / `from_bytes` blocks on the slice-
modeling issue, mark FOLLOW-UP and surface — this is hax-lib level, not a
proof-side fix.

**Stage acceptance:**
- `make check/Libcrux_ml_kem.Vector.Avx2.fst rc=0`
- `make check/Libcrux_ml_kem.Vector.Portable.fst rc=0`

### Stage 5 — retire band-aid + final full-make + perf top-20 refresh

If Stage 0 didn't land, retire the `--using_facts_from` band-aid on
`deserialize_then_decompress_u` here only if a strictly-better local proof
emerges; otherwise leave it.

Run a full `make` from `proofs/fstar/extraction`. Refresh
`agent-status/fstar-perf-top20.md` per `feedback_track_fstar_perf`.

Regenerate verification status: `bash proofs/generate_verification_status.sh`,
diff against this entry's "Definition of done" target counts. Commit:
`agent-mlkem: sprint 2026-05-09 — Portable+Avx2+impl-layer lax cleanup
complete`.

## Time budget

- Stage 1: ~3 hours (4 fns × 45 min average).
- Stage 2: ~6 hours (7 fns × 50 min average + lemma wiring).
- Stage 3: ~5 hours (3 fns × 90 min).
- Stage 4: ~3 hours (6 fns × 30 min) — pessimistic if `to_bytes` blocks.
- Stage 5: ~1 hour (full make + status diff + commit).

**Total: ~18 hours** assuming standard fragility. Likely 1–2 sessions split
across stages 1+2 (one session) and 3+4+5 (one session). Per `feedback_proof_debug_budget`,
no single function gets more than 60–90 min — beyond that, FOLLOW-UP and move
on.

## Commit hygiene

One commit per stage, ideally per file:
- `agent-mlkem: flip serialize.rs _5/_11 lax → panic_free (mirror _4/_10 pattern)`
- `agent-mlkem: flip Avx2 vector serialize_*/deserialize_* lax → panic_free (BitVec lemma wiring)`
- `agent-mlkem: discharge ntt_vector_u + invert_ntt_at_layer_4_plus body admits`
- `agent-mlkem: flip op_* glue fns lax → panic_free`
- `agent-mlkem: sprint 2026-05-09 rollup`

Per stage: ensure `make check/<changed-modules>.fst rc=0` before commit. Do
NOT commit the encrypt_unpacked TEMP `--admit_smt_queries true` from any prior
spike — it has been retired in commit `dff9669ff`.

## Pre-session checklist

- [ ] Working tree at `/Users/karthik/libcrux-trait-opacify` clean? If not,
      `git status --short` — only the 4 untracked agent-status notes plus
      `#Makefile#` should be untracked. Anything else, ask before proceeding.
- [ ] Read `fstar-for-libcrux` skill (or `/skill fstar-for-libcrux`).
- [ ] Read `smtprofiling` skill (or `/skill smtprofiling`).
- [ ] Read `MEMORY.md` entries listed above.
- [ ] Confirm tip: `git -C /Users/karthik/libcrux-trait-opacify log -1 --oneline`
      should show `1c94eae53` or later.
- [ ] Confirm baseline lax counts via `bash proofs/generate_verification_status.sh`
      and skim the table — the 22 sites listed above should be present.
- [ ] Pick ONE stage to start with (Stage 1 if uncertain — smallest, most
      independent).

## Status reports

Per `feedback_agent_status_reports`: every 15 min while working, append a 3-
line status to `agent-status/sprint-2026-05-09-status.md`:
- Sub-task / file / fn currently active.
- Blocker (if any) — specific Z3 query / missing lemma / extraction fail.
- ETA for current sub-task.

End-of-session: write `agent-status/sprint-2026-05-09-rollup.md` summarizing
what landed, what blockers surfaced, and the per-stage acceptance status
(rc=0 vs FOLLOW-UP).

## Key file paths quick reference

- Scripts: `proofs/generate_verification_status.{sh,py}`,
  `proofs/verification_status.md` (regenerated artifact)
- ADMIT_MODULES: `libcrux-ml-kem/proofs/fstar/extraction/Makefile` (line ~5).
  Only Neon backend + Incremental modules listed; do NOT add to this list
  during the sprint.
- `polynomial.rs:45-134` — `is_bounded_poly`, `is_bounded_polynomial_vector`,
  `is_bounded_polynomial_matrix`, plus `_higher` lemmas. The `_intro` and
  SMTPat'd `_elim` lemmas live just below (lines 76-237).
- `sample_vector_cbd_then_ntt` at `ind_cpa.rs:254-283` — the working template
  for `for i in 0..K { ... ZERO assigment + ntt_* in_place }` loops.
- `deserialize_then_decompress_u` at `ind_cpa.rs:1023-1059` — the most-
  recently-flipped function; demonstrates createi + `--using_facts_from`
  band-aid pattern.

## Stretch goals (only if all 5 stages land cleanly + budget remains)

- Eliminate the 2 sampling lax sites by formalizing the rejection-sampling
  while-loop. Requires modeling the while loop's variant + a rejection-rate
  analysis. Likely a separate sprint.
- Tackle Neon backend (currently entire backend in ADMIT_MODULES). Requires
  porting the proven AVX2 intrinsic specs to NEON equivalents. Multi-session.
