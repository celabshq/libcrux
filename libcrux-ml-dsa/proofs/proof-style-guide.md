# ML-DSA Proof Style Guide

This guide is forked from `libcrux-ml-kem/proofs/proof-style-guide.md`
and adapted for ML-DSA (FIPS 204). The phase structure and patterns are
identical; the constants and module names differ.

## 1. Spec hierarchy convention

| Tier | ML-DSA | Status |
|---|---|---|
| ✅ Canonical | `Hacspec_ml_dsa.*` (in `specs/ml-dsa/proofs/fstar/extraction/`) | Cite this |
| ⚠️ Obsolete (deletion-pending) | `Spec.MLDSA.Math`, `Spec.MLDSA.Ntt`, `Spec.Intrinsics` (in `proofs/fstar/spec/`) | DO NOT extend. Banner-marked. |
| 🔄 Temporary scaffolding | `Spec.Utils.*` (shared with ML-KEM) | Pieces we use will move to `Proof_utils.*` |

## 1.1 ML-DSA-specific naming gotchas

- `Hacspec_ml_dsa.Arithmetic.uuse_hint` (note the typo: `uuse`, not
  `use`). It is canonical — do not "fix" it without a coordinated rename
  across the spec, all proofs, and the extraction Makefile.
- `Hacspec_ml_dsa.Arithmetic.make_hint` returns `bool`; the SIMD impl
  returns `i32` (0 or 1). The trait post relating them must convert
  bool ↔ {0, 1}.
- `Hacspec_ml_dsa.Encoding.coeff_from_three_bytes` /
  `coeff_from_half_byte` are the per-byte rejection-sampling step
  helpers. Trait posts on `rejection_sample_*` cite these.

## 2. Architectural pattern: trait at the bottom + opacity

ML-DSA's SIMD trait (`Simd::Traits::Operations`) provides a uniform
8-element interface (vs. ML-KEM's 16-element). Above it sit modules
(Polynomial, Ntt, Matrix, Encoding) that compose per-vector operations
into per-polynomial / per-vector algorithms.

Trait posts must give STRONG semantic guarantees (not just bounds) so
above-trait modules can lift to spec correspondence. Wrap the FE-equation
portion in opaque predicates so the iteration structure (`forall8` /
`forall32` / `forall4`) stays transparent but the hacspec-internal body
is hidden.

### The forall-of-opaque-atoms shape

```
bound_predicate
  /\ Spec.Utils.forall8 (fun i -> X_lane_post v.[i] r.[i])
```

Where `forall8` is `unfold let` (transparent) and `X_lane_post` is
`[@@ "opaque_to_smt"]` (hidden body).

Predicate granularity:
- **Per-lane** for compress / decompress / unary / decompose / power2round
  (one atom per lane, each carrying a small equation referencing one
  i32 pair).
- **Per-branch** for NTT-layer / butterfly ops (one atom per zeta-group;
  ML-DSA NTT uses radix-2 Cooley-Tukey, so the branching pattern is
  similar to ML-KEM).

## 3. Specific F* / Z3 techniques

### 3.1 Opaque predicate definition

```fstar
[@@ "opaque_to_smt"]
let X_lane_post (input result: i32) : prop =
  i32_to_spec_fe result == hacspec_fn (i32_to_spec_fe input)
```

### 3.2 Dual-trigger SMTPat for predicate unfolding

```fstar
let lemma_X_lookup (lo hi: i32) (x: t_Slice i32) (i: nat)
    : Lemma (requires bounded x lo hi /\ i < Seq.length x)
            (ensures v lo <= v (Seq.index x i) /\ v (Seq.index x i) <= v hi)
            [SMTPat (Seq.index x i); SMTPat (bounded x lo hi)] =
  reveal_opaque (`%bounded) (bounded x lo hi)
```

**Anti-pattern**: bidirectional SMTPat with the SAME trigger. Use the
named-intro pattern (3.3) for the produce direction.

### 3.3 Named intro lemma (no SMTPat)

```fstar
let lemma_X_intro (lo hi: i32) (x: t_Slice i32)
    : Lemma (requires forall (i: nat). i < Seq.length x ==>
                       v lo <= v (Seq.index x i) /\ v (Seq.index x i) <= v hi)
            (ensures bounded x lo hi) =
  reveal_opaque (`%bounded) (bounded x lo hi)
```

### 3.4 `Classical.forall_intro` for collapsing N manual aux invocations

```rust
hax_lib::fstar!(r#"
  let aux (j: nat{j < 8}) : Lemma (X_lane_post a.[j] r.[j]) =
    lemma_per_lane_commute (Seq.index a j) (Seq.index r j)
  in
  Classical.forall_intro aux
"#);
```

ML-KEM measured 35–50% Z3-time speedup over manual `aux 0; aux 1; ...`
unrolling. The same pattern applies to ML-DSA's 8-lane vectors.

### 3.5 Per-element commute lemmas

Each per-element transformation gets a "fe_commute" lemma in
`specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
(new file in Phase 2F):

```fstar
let lemma_X_fe_commute (a result: i32) :
  Lemma (requires <integer-form post from primitive>)
        (ensures hacspec_fn (i32_to_spec_fe a) == i32_to_spec_fe result)
  = (* ~5 lines of integer arithmetic + i32_to_spec_fe refinement *) ()
```

Targets for ML-DSA: per-lane `decompose`, `power2round`, `make_hint`,
`use_hint`, `montgomery_multiply`, `shift_left_then_reduce`,
butterfly_pair (NTT), inv_butterfly_pair (INVNTT), Barrett (reduce).

### 3.6 Tier-N composition lemmas

- **Tier 1 (poly-level)**: `Classical.forall_intro` lifts a per-lane
  commute lemma to a per-vector statement. Iterate over 32 SIMD units
  to get a polynomial-level statement.
- **Tier 2 (NTT layer-step)**: Composes butterfly_pair_commute calls
  into the hacspec layer's `forall_n` form. One lemma per layer.
- **Tier 3 (full NTT)**: Chains 8 layer-step lemmas (vs. ML-KEM's 7),
  matching the BitRev₈ zeta-table ordering. Hard — likely USER lane.
- **Tier 4 (matrix/vector)**: `vector_add`, `vector_sub`,
  `multiply_matrix_by_column` built from per-poly composition.

### 3.7 The post-only invariant

Every change in the migration phase satisfies:
- **Pre-conditions**: never touched.
- **Post-conditions**: only conjunctive additions.

## 4. ML-DSA-specific constants

| Constant | Value | Used for |
|---|---|---|
| `Q` | 8380417 | Field modulus |
| `D` | 13 | Power2Round split bit |
| `R` | 2³² | Montgomery |
| `R⁻¹` mod Q | 8265825 | Montgomery |
| `Q⁻¹` mod R | 58728449 | Montgomery |
| `256⁻¹·R²` mod Q | 41978 | INVNTT post-scale (libcrux's Montgomery-aware variant of FIPS f=8347681) |
| `γ₂` | (Q-1)/88 = 95232 or (Q-1)/32 = 261888 | Decompose / hints |
| `γ₁` | 2¹⁷ or 2¹⁹ | Mask sampling |
| `η` | 2 or 4 | Error sampling |
| `τ` | 39, 49, 60 | SampleInBall (per param set 44/65/87) |
| `β` | τ·η | Bound checks |
| `ω` | 80, 55, 75 | Hint count cap (per param set 44/65/87) |

## 5. The 20-minute rule

Every individual proof attempt has a hard wall-clock budget of **20
minutes**. If the proof does not close in that window:

1. Mark the function as admitted via either:
   - `#[hax_lib::fstar::verification_status::panic_free]` on the Rust function, OR
   - `hax_lib::fstar!("admit()")` at the top of the function body
2. Document in `proofs/outstanding-admits.md`:
   - File and line range.
   - Diagnosis (Z3 timeout? quantifier blowup? missing lemma?).
   - Suggested USER-lane mitigation.
3. Move on.

The objective is **maximum easy-proof coverage** in the sprint window.

## 6. Phase methodology

- **Phase 1**: Strengthen trait posts (single-agent serial — every method
  post lives in `traits.rs` / `traits/specs.rs`).
- **Phase 2**: Portable Operations impls (waves 2A–2G; 2A–2E parallel,
  2F→2G sequential because they share commute lemmas).
- **Phase 3**: AVX2 Operations impls (waves 3A–3E; 3A and 3B parallel,
  3D→3E sequential).
- **Phase 4**: Migration (4A serial, 4B depends 4A, 4C parallel with 4A,
  4D last).

See `proofs/sprint-plan.md` for the full phase table with parallelism.

## 7. User-vs-agent task split

### User lane (manual, math-heavy, Z3-blocked)
- Standalone integer-arithmetic proofs (Barrett 4-case, Montgomery identity).
- SMT-blocked SIMD proofs (AVX2 4-zeta-parallel layer where Z3 can't handle the wide context).
- Tier-3 layer compositions with subtle indexing (BitRev₈).
- Anything that hits "incomplete quantifiers" repeatedly.

### Agent lane (mechanical, pattern-following)
- Tier-1 per-poly composition lemmas via `Classical.forall_intro`.
- Tier-2 NTT layer-step commute lemmas (with templates).
- Tier-4 matrix/vector composition lemmas.
- Adding hacspec citations to module posts.
- Re-rooting obsolete spec citations to canonical.
- Phase 6 impl-admit drops.

### Cut-off (escalate to user)
- A single Z3 query times out at rlimit ≥ 800 with `--split_queries always`.
- Lemma needs new mathematical reasoning we don't have a pattern for.
- Refactor would change > ~50 lines of existing proof code.
- Verification time on a single module exceeds 10 minutes pure F* CPU.

## 8. Tooling

- **`fstar-mcp` skill** — incremental F* typechecking via the fstar-mcp server. Default for proof iteration.
- **`proofdebugging` skill** — systematic Z3 failure triage.
- **`hax.sh extract` + `make -C proofs/fstar/extraction`** — full pipeline.
- **Hint files** in `.fstar-cache/hints/` — speed up Z3 selection. Independent of `.checked` files; preserve both. See §9.1.

## 8.1 Reading verification output — what's not a failure

F*'s logs show transient "failed" messages that look alarming but
are **normal recovery behavior**. Don't conclude the proof is
broken just because you see one. Patterns:

### Hint replay miss → retry without hint succeeds

```
(Module.fn_name, 22) failed ... (with hint)   ← hint replay miss
(Module.fn_name, 22) succeeded in 18 ms        ← retry without hint
```

Meaning: the recorded hint went stale (Z3 update, tiny proof
context shift, etc.), F* retried the same query without it, and
Z3 found a fresh proof. The query is healthy. The new proof
overwrites the stale hint via `--record_hints`. Nothing to do.

### Single-query failure → split queries succeeds

A query reported as failed at the function level may then succeed
once F* retries with `--split_queries`:

```
(Module.fn_name, 5) failed
(Module.fn_name, 5.1) succeeded
(Module.fn_name, 5.2) succeeded
...
```

The function still verifies; F* just needed to break the VC into
sub-queries. Module ends "verified". No action needed.

### Decision rule

Only treat a verification line as a real failure if **the module
ends without "Verified module: ..." and a non-zero exit propagates
to make.** Per-query "failed" lines without a corresponding module-
level success are the real signal; per-query "failed" lines that
are followed by a "succeeded" (with or without hint, with or without
split) for the same query ID are recovery noise.

When tail-grepping a log to triage, check for `\* Error [0-9]+ at`
(F*'s typecheck-error format) and `\*\*\* \[.*\.checked\] Error`
(make's signal of a failed target). Those are the real failure
signals; "failed" alone is not.

## 9. Cache-invalidation lessons

Every change to `traits.rs`'s `fstar::before` block invalidates
`Simd.Traits.Spec.fst`'s digest, cascading to ~15+ Simd.* modules and
forcing a long re-verify. Mitigation: factor helpers (`bounded_i32_array`,
lookup, intro lemmas) into a separate hand-written F* file (e.g.,
`proofs/fstar/spec/Libcrux_ml_dsa.Simd.Bounds.fst`) included via
`FSTAR_INCLUDE_DIRS_EXTRA`. Helper additions then invalidate only that
file + its direct importers.

## 9.1 Maximize caching — pattern and anti-pattern

F* keeps two independent caches that **must both be preserved**:

- `.fstar-cache/checked/*.checked` — per-module verification cache.
  If a source's mtime is unchanged, F* skips the module entirely.
  This is the dominant cost saver.
- `.fstar-cache/hints/*.hints` — per-query SMT replay cache. Hints
  make individual queries faster but don't avoid module re-check.

### Pattern: targeted-make iteration

1. Edit file(s).
2. Run `make $TARGET.fst.checked` from
   `libcrux-ml-dsa/proofs/fstar/extraction/`. Make walks back
   through deps and re-checks only what's stale. Warm cache is
   typically ~1-30 sec; "cold but only your touched files
   invalidated" is usually <2 min.
3. Iterate — only step 2 each cycle.
4. **Once per cohesive piece of work, before commit**:
   `JOBS=2 ./hax.sh prove 2>&1 | tail -22`. With a warm cache this
   is 2-3 min (the 56 ADMIT-mode modules always re-emit their
   tag; the 42 CHECK-mode modules are cache hits).

### Anti-pattern: blanket cache wipes

Do **not** run `rm -rf .fstar-cache/checked/*` or
`find .fstar-cache -delete` (or the same with broad globs like
`Libcrux_ml_dsa.*` or `Hacspec_ml_dsa.*`). It looks innocuous but
forces a 10-15 min cold re-prove. F*'s caching is reliable; if
you genuinely suspect corruption (rare), delete only the one or
two specific `.checked` files you suspect. Never delete `.hints`
files — they replay correctly even after source edits, and a fresh
hint can be substantially worse than a recorded one until you
re-converge.

### When you legitimately need to invalidate

- Make says "nothing to do" but you know a transitive dep
  changed: `touch path/to/file.fst` to bump mtime, then
  re-make.
- You changed an `.fsti` interface and want a fresh re-check:
  `rm` only that one specific `.checked` file (and the `.checked`
  files of any sources you can clearly identify as needing fresh
  digests).

### Anti-pattern that has bitten this project

Two prior agent sessions cleared `Libcrux_ml_dsa.*` and
`Hacspec_ml_dsa.*` from the cache before re-proving "to be safe".
Each cost ~12 min of wall-clock for zero added confidence —
F* would have re-checked exactly the same modules from a warm
cache, just without the wait. Targeted `make $TARGET.checked`
is the right cycle; full `./hax.sh prove` with warm cache is the
right pre-commit gate.

## 9.2 Develop locally, upstream once (binding workflow)

Touching a low-level shared module (`Libcrux_ml_dsa.Simd.Traits.Specs`,
`Spec.MLDSA.Math`, `Spec.Intrinsics`, `Hacspec_ml_dsa.Parameters`)
cascades a re-verify through every transitive dependent.  In Step 9.3
a single change to `shift_left_then_reduce_lane_post`'s body forced
re-verification of `Simd.Portable.Ntt`, `Simd.Portable.Invntt`, the
AVX2 NTT/Invntt counterparts, and every encoding module — ~30+ min
of build for a 5-line spec relax.

**Binding rule: develop new lemmas standalone first; upstream after
they verify.**

### Iteration loop (use during exploration)

1. **Start the lemma in the consumer file**, not in the shared spec.
   - New per-lane bridge lemma → write it inline in
     `Hacspec_ml_dsa.Commute.Chunk.fst` with a *local* relaxed-form
     `prop` (don't touch `Specs.fst` yet).
   - New `Spec.MLDSA.Math.*` helper → write it inline at the top of
     the file that uses it (e.g. inside the impl-side `arithmetic.rs`'s
     `hax_lib::fstar!(...)` block, or in a private F* aux).
   - For very experimental lemmas, create a sibling file
     `<Module>.Sandbox.fst` that imports just what's needed; nothing
     depends on it, so it costs only the fresh-file verification.

2. **Iterate with `fstar-mcp`'s `typecheck_buffer`** (sub-second
   feedback) instead of `make` (50-100s+ even when warm-cached, since
   F* still type-loads the world).  The fstar-mcp binary lives at
   `/Users/karthik/fstar-mcp/target/release/fstar-mcp`; the project's
   `.mcp.json` registers it on port 3001.  See the `fstar-mcp` skill.

3. **Confirm against the consumer once** with a targeted
   `make $CONSUMER.fst.checked` — typically <2 min when only one or
   two impl files are stale.

4. **Upstream in one batch** — once the lemma shape is final, move
   the `prop` definition (and its `_lookup`/`_intro` SMTPats) into
   the canonical home (`Specs.fst`, `Spec.MLDSA.Math.fst`, etc.), pay
   the cascade cost ONCE, and commit.

### Anti-pattern: edit-the-spec-then-iterate

Editing `Specs.fst` *before* knowing the final lemma shape forces
a 30-min cascade per edit.  In Step 9.3 the right move would have
been to:
  - Inline the relaxed `shift_left_then_reduce_lane_post` shape as a
    local `prop` inside `Commute.Chunk.fst`'s commute lemma's
    `(ensures ...)`, develop with `typecheck_buffer`, confirm against
    the impl files (~3 min),
  - Then upstream the relaxed `prop` definition + `_lookup`/`_intro`
    lemmas into `Specs.fst` in one commit.

The instinct to "fix the spec first, then iterate" is wrong when the
spec lives in a high-fan-in module — fix the consumers and pin down
the lemma shape first, then move the shape upstream.

### Why this matters

Cache-warm `make` of a single impl file is ~30s.  Cache-warm
`make` after touching `Specs.fst` is ~30 min.  The 60× factor
isn't iteration friction — it's the difference between a productive
day and a stuck-on-cascade day.  Use `typecheck_buffer` and local
specs as the default development surface; treat the upstream spec
modules as commit targets, not edit targets.

## 10. Documentation cadence

`MLDSA_STATUS.md` and `proofs/outstanding-admits.md` MUST be kept in sync
after each meaningful step (new commit or wave complete). The session
may close at any time — both files together are the resume point.

## 11. Branch / commit conventions

- Each wave is one commit. Commit message: imperative, names the wave
  and the dropped admit (if any).
- `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
  on agent-authored commits.
