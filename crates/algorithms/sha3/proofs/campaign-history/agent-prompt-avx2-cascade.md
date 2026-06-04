# Prompt — AVX2 load_block third cascade source

## Setup (do BEFORE starting this prompt)

Pre-conditions on the parent /Users/karthik/libcrux-sha3-focused tree
(user runs once before spawning either agent):

1. Two upstream fixes need to be on `sha3-byteform-migration` so both
   worktrees inherit them:
   - `specs/sha3/src/lib.rs` — `[@@ "opaque_to_smt"]` on `createi` +
     `reveal_opaque` in `createi_lemma`. Commit `7bb581f8b`.
   - `crates/utils/intrinsics/src/avx2_extract.rs::get_lane_u64` —
     refinement-on-result replaced with separate SMTPat lemma
     `get_lane_u64_post` declared in `.fsti` via
     `fstar::after(interface, ...)` and body via `fstar::after(...)`.
   Verify both via:
   ```
   git log -p -1 -- specs/sha3/src/lib.rs
   git log -p -1 -- crates/utils/intrinsics/src/avx2_extract.rs
   ```

2. Worktree this AVX2 session:
   ```
   cd /Users/karthik/libcrux-sha3-focused
   git worktree add -b avx2-cascade /Users/karthik/libcrux-sha3-avx2 sha3-byteform-migration
   ```
   The `avx2-cascade` feature branch isolates this work from the
   parallel squeeze2 agent (which runs in
   `/Users/karthik/libcrux-sha3-squeeze2` on its own feature branch).

## Repo

Repo: `/Users/karthik/libcrux-sha3-avx2`  (FRESH GIT WORKTREE,
       branch `avx2-cascade` off `sha3-byteform-migration`).

A parallel agent is working concurrently in
`/Users/karthik/libcrux-sha3-squeeze2` on the Arm64 squeeze2 body
proof. That worktree is fully isolated from yours (separate src/,
proofs/, .fstar-cache/, branch, staging area). The two of you must
NOT touch each other's domain.

## Mission

Close the AVX2 load_block forall-25 cascade so
`make check/Libcrux_sha3.Simd.Avx2.fst` passes clean from
`crates/algorithms/sha3/proofs/fstar/extraction/`.

## Read FIRST (no skipping)

- skills `fstar-mcp`, `fstar-for-libcrux` (especially §1.5 about
  regressions vs always-broken proofs), `smtprofiling`
- `/Users/karthik/.claude/projects/-Users-karthik-libcrux/memory/MEMORY.md`
- `proofs/agent-status/avx2-load-block-2026-05-05.md` — Phase 1 status
  doc from prior agent
- `proofs/agent-status/sha3-arm64-squeeze2-status-2026-05-04.md` — for
  the analogous Arm64 cascade fix and the smtprofiling workflow

## What's already known and fixed (DO NOT REDO)

Two cascade sources are fixed via the commits inherited from
`sha3-byteform-migration`. Verify they're in your worktree:
```
grep -B1 '^let createi\b' specs/sha3/proofs/fstar/extraction/Hacspec_sha3.fst
grep -A4 'get_lane_u64_post' crates/utils/intrinsics/proofs/fstar/extraction/Libcrux_intrinsics.Avx2_extract.fsti
```

If those don't show the `opaque_to_smt` / `get_lane_u64_post` lemma,
stop and surface to user — the prerequisite commits weren't made.

Both fixes individually reduce instantiation counts but together they
do NOT close the AVX2 cliff — there is a third cascade source.

## Current cliff (what's left)

- `make check/Libcrux_sha3.Simd.Avx2.fst` with
  `--admit_except 'Libcrux_sha3.Simd.Avx2.load_block'` produces 4
  errors at unique source lines around 1091, 1164, 1249, 1349 — the
  per-iteration `Hax_lib.v_assert` calls in
  `crates/algorithms/sha3/src/simd/avx2.rs::load_block`.
- Each failing sub-query cancels at 400/400 rlimit, ~100s wall.
- z3 qi.profile on the failing `.smt2` shows the dominant quantifier
  is **`k!61` at ~1.1M instantiations**, anonymous (no qid). Even with
  both above fixes plus
  `--using_facts_from '* -Rust_primitives.Slice.array_from_fn'` filter
  on load_block options, the cliff persists.

## Your task

1. **Profile**: re-add log_queries to load_block options:
   ```rust
   #[hax_lib::fstar::options("--z3rlimit 400 --split_queries always \
     --using_facts_from '* -Rust_primitives.Slice.array_from_fn' \
     --log_queries --z3refresh --query_stats")]
   ```
   Re-extract via `bash crates/algorithms/sha3/hax.sh extract`.

2. **Run**: from `crates/algorithms/sha3/proofs/fstar/extraction/`:
   ```
   make OTHERFLAGS="--admit_except 'Libcrux_sha3.Simd.Avx2.load_block'" \
     check/Libcrux_sha3.Simd.Avx2.fst > /tmp/avx2-prof.log 2>&1
   ```
   Identify first failing query's `@queries-...-N.smt2` filename.

3. **z3 qi.profile** that .smt2 directly (skill: smtprofiling):
   ```
   cd /tmp && timeout 200 z3-4.13.3 smt.qi.profile=true \
     <full-path>/queries-Libcrux_sha3.Simd.Avx2-<N>.smt2 \
     > /tmp/q-out.txt 2> /tmp/q-qi.txt
   awk '/^\[quantifier_instances\]/ {total[$2] += $4}
        END {for (n in total) printf "%10d  %s\n", total[n], n}' \
       /tmp/q-qi.txt | sort -rn | head -25
   ```

4. **Identify the third cascade source.** The dominant `k!N` is
   anonymous; find it by inspecting the .smt2 for refinements that
   declare an inner `forall` without qid/pattern. Most likely
   candidates after the known fixes:
   - Another intrinsic with `Pure T (ensures result == ...)` shape
     in `Libcrux_intrinsics.Avx2_extract.fsti` (mirror of the
     get_lane_u64 issue). Check
     `Libcrux_intrinsics.Avx2_extract.get_lane_u64x4` (the underlying
     spec function), `set_lane_u64x4`, vstore variants, etc.
   - A different `Rust_primitives.Slice` definition besides
     `array_from_fn` that has a similar refinement-interpretation
     pattern.
   - A `Hacspec_sha3.*` definition that's transitively reachable and
     not yet opaque.

5. **Apply the appropriate fix at the source** (NOT in load_block):
   a. **SMTPat replacement** (mirror of get_lane_u64 fix): drop the
      ensures-refinement, add a separate lemma with SMTPat. Use:
      ```rust
      #[hax_lib::fstar::after(
        interface,
        r#"val lemma_name (...) : Lemma (...) [SMTPat (target_term ...)]"#)]
      #[hax_lib::fstar::after(
        r#"let lemma_name (...) : Lemma (...) [SMTPat (target_term ...)] = admit ()"#)]
      ```
      Trust footprint preserved if the original ensures was already
      axiomatic via `unimplemented!()` or similar.
   b. **Opacity** (mirror of createi fix): `[@@ "opaque_to_smt"]`
      plus a companion lemma with `reveal_opaque` body and SMTPat.
   c. Local `--using_facts_from` filter on load_block options as
      a workaround if (a) and (b) aren't viable.

6. **Verify**:
   - Remove `--log_queries --z3refresh --query_stats` debug flags
     from load_block options (CI will fail with them on).
   - `make check/Libcrux_sha3.Simd.Avx2.fst` clean (0 errors, max
     load_block sub-query under ~10s).
   - `cd ../equivalence && make` — full chain passes (squeeze2 may
     still have its body admit; that's the parallel agent's territory
     and not your concern unless your changes broke it).

7. **Commit your work** on the `avx2-cascade` branch (do NOT push,
   user merges later):
   ```
   git -C /Users/karthik/libcrux-sha3-avx2 add -p
   git -C /Users/karthik/libcrux-sha3-avx2 commit -m "..."
   ```

## Hard constraints — file boundaries

This worktree is the AVX2 sandbox. The parallel squeeze2 agent owns
these files; you must NOT modify them:
```
crates/algorithms/sha3/src/generic_keccak/simd128.rs
crates/algorithms/sha3/src/neon.rs
crates/algorithms/sha3/src/simd/arm64.rs
crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst
crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst
```

You DO own:
```
crates/algorithms/sha3/src/simd/avx2.rs
crates/algorithms/sha3/src/avx2.rs
crates/utils/intrinsics/src/avx2_extract.rs (already touched by
  the inherited commit; further edits OK if needed)
crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.* (if needed)
```

Shared / coordinate via the inherited commit (already in place):
```
specs/sha3/src/lib.rs (createi opacity)
crates/utils/intrinsics/src/avx2_extract.rs::get_lane_u64
```

If you find another upstream lib (e.g. hax-lib's own .fsti) is the
cascade source, surface to user — modifying hax-lib in worktree is
out of scope.

## Hard constraints — proof discipline

- rlimit cap **800 mono / 400 with --split_queries**. Never bump above.
- Per-fn debug budget **60 min wall**.
- Do NOT modify the AVX2 load_block proof itself — it was working
  before; the regression is upstream. Per `fstar-for-libcrux` §1.5,
  fix at the cascade source.
- Pipe make output to log + grep, never Read full F* logs into context
  (memory `feedback_grep_make_output`).
- `--log_queries --z3refresh --query_stats` debug flags must be
  removed from load_block options before declaring done. They generate
  large `.smt2` files in CI.
- Do not run `bash crates/algorithms/sha3/hax.sh extract` while the
  squeeze2 agent might be running it concurrently in the other
  worktree. Each worktree's extract is independent at the filesystem
  level (different src/, different proofs/), but if both run at once
  they'll compete for cargo locks. If you see cargo metadata errors,
  retry; or coordinate timing if you can detect concurrent activity
  (e.g., look for `fstar.exe` processes whose paths point at the
  squeeze2 worktree).
- Worktree hygiene: never `cd` into `/Users/karthik/libcrux-sha3-focused`
  or `/Users/karthik/libcrux-sha3-squeeze2`.

## Deliverables

- Either `make check/Libcrux_sha3.Simd.Avx2.fst` passes clean AND
  full `make` from `proofs/fstar/equivalence/` clean — surface
  results.
- Or status doc at
  `proofs/agent-status/avx2-load-block-cascade3-2026-05-05.md`
  documenting:
  - the third cascade source (qid, refinement hash, F* construct,
    file:line)
  - what fix you tried and why it didn't close
  - the next attempt path
  - confirmation that the createi-opacity and `get_lane_u64` SMTPat
    fixes from the inherited commit are still intact

Report back with: (1) what the third cascade source was, (2) what
fix landed, (3) verification status (0 errors? max query time?),
(4) which files you committed on `avx2-cascade`.
