# Prompt — discharge the two `store_block` body-`admit ()` blocks

## Mission

Close the two body-`admit()` blocks on `Libcrux_sha3.Simd.{Arm64,Avx2}.store_block`:

- `crates/algorithms/sha3/src/simd/arm64.rs:255` → `Libcrux_sha3.Simd.Arm64.fst:913`
- `crates/algorithms/sha3/src/simd/avx2.rs:490`  → `Libcrux_sha3.Simd.Avx2.fst:1874`

Each is a single `hax_lib::fstar!("admit()");` line that suppresses verification of the function's `fold_range` body against its `ensures` post (per-byte equalities into `out_k` buffers).

## Setup (user runs once before spawning)

Two parallel worktrees so the agents can't trample each other:

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b store-block-arm64 /Users/karthik/libcrux-sha3-store-arm64 sha3-proofs-focused
git worktree add -b store-block-avx2  /Users/karthik/libcrux-sha3-store-avx2  sha3-proofs-focused
```

Spawn one agent per worktree. The Arm64 target is easier (N=2, 4 forall conjuncts, 6 live arrays); the AVX2 target is harder (N=4, 8 conjuncts, 12 arrays). Start with Arm64 — its solution often informs AVX2.

## Repo

Each agent works ONLY in its own worktree:

- Arm64 agent → `/Users/karthik/libcrux-sha3-store-arm64`, branch `store-block-arm64`
- AVX2 agent → `/Users/karthik/libcrux-sha3-store-avx2`, branch `store-block-avx2`

Hard constraint: NEVER `cd` into `/Users/karthik/libcrux-sha3-focused`, the other store-block worktree, the squeeze2 worktree, or any other sibling. Use `git -C <worktree-path> ...` and absolute paths.

## Read FIRST (no skipping)

1. **Sprint state**: `crates/algorithms/sha3/proofs/sha3-sprint-todo.md` — items 3 (Arm64 store_block) and 4 (AVX2 store_block) in §"Suggested sprint order"; the `## AVX2 cascade closure (2026-05-05)` section at the top has the four-fix recipe.
2. **Recent landed pattern (Arm64 `load_block` discharge, 2026-04-26 commit `abf8b5297`)** — Arm64 store_block should mirror this. Run:
   ```
   git -C <worktree> log --oneline -p abf8b5297 -- crates/algorithms/sha3/src/simd/arm64.rs | head -200
   ```
3. **Recent landed pattern (AVX2 cascade closure)** — for AVX2 store_block, the four commits `7bb581f8b`, `8203c9ace`, `28db4222a`, `3b9fc054c` on `sha3-proofs-focused`. The lemma + tight-SMTPat shape in `Libcrux_sha3.Simd.Avx2.load_lane_u64_lane_extensionality` is the template.
4. **Skills**: `fstar-mcp`, `fstar-for-libcrux` (especially §1.5.1 "Layered cascades and the danger of fixing one cascade with a new SMTPat" and §8 "Debugging proofs in F* directly"), `smtprofiling`.
5. **Memory rules** (in `~/.claude/projects/-Users-karthik-libcrux/memory/MEMORY.md`):
   - `feedback_grep_make_output` — pipe make output to a log + grep, never Read full F* logs.
   - `feedback_use_fstar_mcp` — fstar-mcp for sub-second iteration; full SMT through make.
   - `feedback_fstar_mcp_session_dies_after_make` — recreate fstar-mcp session after each make.
   - `feedback_no_manual_edits_extracted` — surface to user before editing extracted `.fst` directly. **For experimental probes during cascade debugging, ask once per session and the user usually approves.** Permanent fixes go through Rust source + re-extract.
   - `feedback_rlimit_cap_800` — never set rlimit > 800 mono / 400 with `--split_queries always`.
   - `feedback_proof_debug_budget` — 60 min hard cap per function. After that, write a status doc and stop.

## Function shape

### Arm64 (N=2, 2 output buffers `out0`, `out1`)

```rust
pub(crate) fn store_block<const RATE: usize>(
    s: &[uint64x2_t; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    start: usize,
    len: usize,
)
```

Body (after the admit): a `for i in 0..len/16` loop that uses `_vtrn1q_u64` / `_vtrn2q_u64` to interleave two state lanes, then `_vst1q_bytes_u64` to write 16 bytes per output buffer per iteration. Then a tail handling `len % 16` (split into ">8" and ">0" cases).

Post (8 conjuncts, 4 per output buffer):
```rust
forall i. if i < out_k.len() {
  if i < start { out_k[i] == future(out_k)[i] }
  else if i < start + len { future(out_k)[i] == get_lane_u64(s[(i-start)/8], k).to_le_bytes()[(i-start) % 8] }
  else { out_k[i] == future(out_k)[i] }
}
```
for k ∈ {0, 1}.

### AVX2 (N=4, 4 output buffers)

```rust
pub(crate) fn store_block<const RATE: usize>(
    s: &[Vec256; 25],
    out0: &mut [u8], out1: &mut [u8], out2: &mut [u8], out3: &mut [u8],
    start: usize, len: usize,
)
```

Same pattern at N=4: outer `for i in 0..chunks` loop with `mm256_permute2x128_si256` + `mm256_unpacklo/hi_epi64` to deinterleave 4 lanes from 4 state slots, then 4 `mm256_storeu_si256_u8` writes (32 bytes per buffer per iteration). Tail at `len % 32` with sub-loop over 8-byte chunks plus ragged last.

Post: 16 forall conjuncts (4 per output × 4 output buffers).

## Approach

### Arm64 (do this first)

Mirror the 2026-04-26 `load_block` close on Arm64. Likely sequence:

1. Remove the `hax_lib::fstar!("admit()");` line.
2. Add a `loop_invariant!` if missing (the existing one at lines 258-281 should suffice — it's the per-iteration-prefix invariant that the asserts/ensures derive from).
3. Run `make check/Libcrux_sha3.Simd.Arm64.fst OTHERFLAGS='--admit_except "Libcrux_sha3.Simd.Arm64.store_block"' > /tmp/arm64-store-baseline.log 2>&1` to get the failure surface.
4. If clean: done. If not:
   - **Profile first** — re-run with `--log_queries --query_stats --z3refresh` on the store_block options block, identify failing sub-queries' `.smt2` files, run `z3-4.13.3 smt.qi.profile=true <file>`, identify dominant quantifiers (memory: `feedback_grep_make_output`, skill: `smtprofiling`).
   - Inherit AVX2's filter: `--using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'` on the function options.
   - Per-byte aux lemmas (`store_lane_byte_*`) factored to top level if the cascade is in the byte-by-byte invariant (mirror the Portable.squeeze §"Fix" sketch in sprint-todo).
   - SMTPat extensionality / set_lane preservation lemma if the cascade is in the per-iteration write composition.

### AVX2 (after Arm64 closes, or in parallel if you have budget)

Same recipe, harder Z3 budget. Steps:

1. Inherit the load_block options filter on store_block from the start (don't even try without it — same upstream `array_from_fn` cascade source feeds via byte slices).
2. Remove the `hax_lib::fstar!("admit()");`.
3. Profile what fails. Likely candidates given the load_block experience:
   - `set_ij` / `Seq.upd` projection chain (the inverse of load_block's set_ij chain — here the state is read, not written, but the per-iteration write into 4 output buffers is structurally analogous).
   - Per-byte refinement cascade through `to_le_bytes` and `Seq.index`.
4. Apply the same fix kit: filter, opacify intermediate helpers, add tight-SMTPat bridge lemmas, factor per-byte auxes if needed.

### Hard rules from today's load_block experience

- After every fix, **re-`qi.profile`**. The lemma you just added may now be the top instantiator (today's trap). Sprint-todo §"AVX2 cascade closure (2026-05-05)" details the dual-`load_lane_u64`-trigger fix that resolved the lemma's own cascade.
- Multi-pattern SMTPats: both triggers need *symmetric specificity*. Don't mix one tight with one broad. If you write `[SMTPat (foo a b c d); SMTPat (bar c)]` and `bar` has many in-scope occurrences, the lemma will fire for the cross-product. Pair with another `foo`-shaped trigger that shares as many universals as possible.
- Don't bump rlimit > 800 mono / 400 with `--split_queries always`.
- Don't manually edit extracted `.fst` files for **permanent** fixes. For experimental cascade probing (~5–10 min/cycle vs ~10–15 min for hax-extract roundtrip), edit the `.fst` directly *with user approval*, validate, then port back to Rust source. Skill §8.

## Inner loop

- fstar-mcp `lookup_symbol` for type signatures, `typecheck_buffer kind="lax"` for syntax, `kind="full"` for SMT (sub-sec to ~30 s for non-cliff queries).
- `make check/<Module>.fst OTHERFLAGS='--admit_except "<fn>"'` for the per-fn outer loop (~1–3 min on store_block once cliffs are gone).
- `make check/<Module>.fst` for end-of-task validation.
- Pipe everything to `/tmp/<name>.log`, grep for errors. Never `Read` the full make log into context.

## Memory rules: status reports every 15 min

Append to `crates/algorithms/sha3/proofs/agent-status/store-block-{arm64,avx2}-progress.md`:
```
## 2026-05-DD, T+N (sub-task)
- Sub-task: <what>
- Blocker: <if any>
- ETA: <next checkpoint>
```

The parent uses these to detect stalls.

## Hard constraints — file boundaries

Each agent owns ONLY its target's files plus shared options:

**Arm64 agent owns:**
- `crates/algorithms/sha3/src/simd/arm64.rs`
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.fst` (regenerated by hax extract; OK to edit-then-port-back during probing per skill §8)
- `crates/algorithms/sha3/proofs/agent-status/store-block-arm64-*.md`

**AVX2 agent owns:**
- `crates/algorithms/sha3/src/simd/avx2.rs`
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.fst`
- `crates/algorithms/sha3/proofs/agent-status/store-block-avx2-*.md`

**Neither agent touches:**
- `EquivImplSpec.Sponge.*` (equivalence layer; out of scope)
- The other store-block target's files
- `specs/sha3/src/lib.rs`, `crates/utils/intrinsics/src/avx2_extract.rs` (already touched by inherited cascade-closure commits; further edits OK only if discovered necessary AND surfaced to user first)
- `crates/algorithms/sha3/src/simd/portable.rs` (out of scope)
- `crates/algorithms/sha3/src/{generic_keccak.rs,generic_keccak/*.rs}` (squeeze2 agent's domain)

If you find a cross-cutting fix is required (e.g., a new lemma in `EquivImplSpec.Keccakf.*`), STOP and surface to user.

## Deliverables

Each agent commits to its own branch (do NOT push):

- **Success**: `make check/Libcrux_sha3.Simd.{Arm64,Avx2}.fst` passes clean, `cd ../equivalence && make` (whatever subset doesn't depend on squeeze2's not-yet-landed work) passes, no body admits remaining in the target function. Commit on the worktree branch with message describing the fix kit applied and the qi.profile-validated cascade-source story (per skill §1.5).
- **Partial**: status doc at `proofs/agent-status/store-block-{arm64,avx2}-2026-05-DD.md` with the cascade source identified, fixes attempted (and why each didn't close), next-attempt path, profile data. Commit progress to worktree branch.

## Final report (≤400 words)

(1) Which target (Arm64 / AVX2 / both)? (2) What fix kit landed (filter? lemma + SMTPat? per-byte aux?)? (3) Cascade-source diagnosis (top quantifier in qi.profile, instantiation count)? (4) Verification status (max sub-query time, any remaining slow tail)? (5) Files committed and branch SHA(s)?

Don't paste full F* logs — summarize.

## Suggested first 30 minutes

1. `git -C <worktree> log --oneline -p abf8b5297 -- crates/algorithms/sha3/src/simd/arm64.rs | head -200` (Arm64 agent only) — read the load_block discharge pattern.
2. Read the relevant Rust source range around `store_block`.
3. Remove the `admit()` line.
4. Run `make check/<Module>.fst OTHERFLAGS='--admit_except "<fn>"'` with the inherited `--using_facts_from` filter pre-installed, expect failures, count them by (line, type).
5. First status entry to progress.md.
6. Begin smtprofiling on the first failure.

If by T+45 min the kit isn't closing, escalate by surfacing a 1-paragraph blocker note and asking the user before grinding more.
