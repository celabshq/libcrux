# Prompt — port the new Portable squeeze proof to Arm64 squeeze2 (and the same idea to store_block)

Repo: `/Users/karthik/libcrux-sha3-focused`
Branch: `sha3-byteform-migration` (or successor)
Skills: read `fstar-mcp` and `fstar-for-libcrux` before writing any F*.

## Goal

Replace the `assume val lemma_squeeze2_arm64` driver lemma at
`crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst:101`
with a real `let` body. The Portable backend was recently re-verified
end-to-end with **zero admits**; the proof structure used there is
substantially different from the pre-2026-05-04 form. Your job is to
study the *current* Portable squeeze proof and port the **same idea** to
Arm64 squeeze2 — not to reuse the existing Arm64 scaffolding by reflex.

After this lands, only the Avx2 N=4 mirror (`lemma_squeeze4_avx2`) and
two `store_block` body admits remain in the SHA-3 equivalence chain.

## Step 0 — warmup: close `Simd.Arm64.load_block:658` with `forall25`

Before any squeeze work, fix the cascade currently breaking the full
`make` in this directory:

- File: `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.fst`
- Failing lemma: `load_block`, sub-query 301
- Failing assertion: a `forall j: usize. j < 25 ==> …` subtyping check on
  the loop accumulator at line 658, times out at rlimit 800/800 in 157 s
  on cold cache. Sub-queries 1–300 succeed in 30–74 ms each; only the
  forall-consolidation step is the cliff.

This is the same shape `forall25` was built to skip. The fix is
mechanical: enumerate the 25 per-index facts in scope (likely already
populated by prior loop body work), then call `forall25 (fun j -> …)` to
lift to the quantified invariant. Because the file is *extracted* (not
hand-written), the actual edit lives in the Rust source —
`crates/algorithms/sha3/src/simd/arm64.rs::load_block` — likely in an
`hax_lib::loop_invariant!` body or a `hax_lib::fstar!` block. Make the
smallest possible edit there, re-extract via `./hax.py extract`, then
re-run `make check/Libcrux_sha3.Simd.Arm64.fst`.

Verifying this warmup gives you (a) a working `forall25` precedent in a
non-trivial real proof, (b) an unblocked `make` chain, (c) confidence
that the technique generalises before you commit to the squeeze rewrite.
If `forall25` doesn't close it in 30 minutes, surface to the user — that
result reshapes the squeeze plan.

## Reusable primitive: `forall25` (and friends)

Before you write *any* lift-25-facts-to-a-forall step, look at
`EquivImplSpec.Keccakf.Generic.fst:1441`:

```fstar
let forall25 (p:(i:nat{i < 25} -> Type0)):
  Lemma (requires (p 0 /\ p 1 /\ p 2 /\ p 3 /\ p 4 /\
                   p 5 /\ p 6 /\ p 7 /\ p 8 /\ p 9 /\
                   p 10 /\ p 11 /\ p 12 /\ p 13 /\ p 14 /\
                   p 15 /\ p 16 /\ p 17 /\ p 18 /\ p 19 /\
                   p 20 /\ p 21 /\ p 22 /\ p 23 /\ p 24))
        (ensures (forall (i:nat{i < 25}). p i)) = ()
```

This is a generic finite-domain forall-introduction. Body is `()`
because Z3 instantiates `p` at 25 ground points trivially. It collapses
the "lift N specific facts to `forall i. p i`" step — a recurring Z3
quantifier-cascade hot spot in this codebase — into a single near-free
lemma call.

**You will almost certainly want this (or an arity-N variant) at:**

- The `forall j: usize. j < 25 ==> …` invariant inside
  `Libcrux_sha3.Simd.Arm64.load_block` (line ~658). Sub-query 301 of that
  loop currently times out at rlimit 800 because Z3 is consolidating 25
  per-index facts into the forall. A single `forall25 (fun j -> …)` after
  enumerating the 25 facts (or just letting prior helpers populate them)
  should kill that cascade.
- Any `Steps.fst` per-iteration invariant in the new Arm64 squeeze
  proof that quantifies over `j: nat{j < 25}` for state lanes, or
  `l: nat{l < 2}` for output lanes (define a `forall2` analogue
  trivially: requires `p 0 /\ p 1`, ensures `forall l. l < 2 ==> p l`).
- The Avx2 mirrors when those land — `forall4` for output lanes,
  `forall25` for state.
- The `store_block` followup: subbyte-loop invariants of the same
  shape as `load_block` will benefit identically.

If you find yourself writing `introduce forall (i: nat{i < N}) ... with`
and a case-split body, stop and ask: "would `forallN` close this in one
line?" Almost always yes. Add `forall2`, `forall4`, `forall8` etc. to
`Proof_Utils.Lemmas.fst` (or wherever fits) the first time you need them
— the bodies are all `()`.

The shape that *requires* a real proof body is when the predicate `p`
has internal preconditions that aren't trivially true at every ground
point — but for the equality-of-array-indices and bounds-style
predicates you'll be lifting in this work, `()` suffices.

## Read this first — be willing to throw away existing infrastructure

The pre-existing Arm64 squeeze scaffolding —
`lemma_squeeze_one_step_arm64`, `lemma_squeeze_block_arm64`,
`Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_tail`,
`BRIEF_squeeze_steps.md`, the per-byte aux structure, etc. — was designed
against a Portable proof shape that **no longer exists**. The Portable
side has been restructured (likely a different invariant, different
recursion form, possibly different spec-side helpers). Your priorities,
in order:

1. **First: read the *current* Portable squeeze proof end-to-end.** Do
   not skim. Trace from the API one-liner down through the Rust-side
   ensures and into whichever lemmas the new proof actually leans on. If
   any of the old names appear in the new proof, that's coincidence, not
   reuse — verify by reading them.
2. **Then: design the Arm64 proof from the Portable shape, not from the
   old Arm64 shape.** It is fine — expected, even — to **delete**
   pre-existing Arm64 lemmas (`lemma_squeeze_one_step_arm64`,
   `lemma_squeeze_block_arm64`, etc.) if they don't fit the new shape.
   Removing dead scaffolding is part of the work, not a last resort.
3. **Same applies to `store_block`** (the next followup): when you
   eventually port the Avx2/Arm64 `store_block` body admits, expect to
   throw away the pre-existing per-byte aux infrastructure. Whatever the
   *current* Portable `store_block` proof does is the model.

If the new Portable proof turns out to be a one-liner discharge against a
strong Rust-side ensures clause (likely), the per-iteration Steps lemmas
in the existing Arm64 file are simply obsolete. Don't keep them around
"in case they're useful." Delete and re-derive.

## Phase 0 — study the current Portable squeeze (do not skip)

Trace these in order. After reading, you should be able to answer in
two sentences: "the Portable squeeze closes because (1) the Rust fn has
ensures clause X, (2) the body proves it via mechanism Y." If you can't,
keep reading.

1. `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.API.fst`
   — the driver lemma `lemma_squeeze_portable`. Note the rlimit/split
   options on the push, the body shape, the dispatch.
2. `crates/algorithms/sha3/src/generic_keccak/portable.rs` — the
   `squeeze` impl. Look at the **current** `#[hax_lib::ensures(...)]`
   clause, any `#[hax_lib::requires(...)]`, and any
   `hax_lib::loop_invariant!` body. Trace the spec-side reference it
   bridges to (likely `Hacspec_sha3.Sponge.squeeze` or successor).
3. Whatever lemmas the API one-liner / Rust-side ensures pull in.
   Probably **not** the ones you'd expect from `BRIEF_squeeze_steps.md`.
   Read them; understand how they compose.
4. `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Portable.fst`
   — the extracted version of the Rust file, to see exactly what
   ensures form lands at the F* level.
5. `git log -p crates/algorithms/sha3/src/generic_keccak/portable.rs
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.API.fst`
   for the last 2-3 weeks — there will be commits that explain *why*
   the proof shape changed. Read those commit messages; they will save
   you hours of guessing.

## Phase 1 — Arm64 inventory (read, then plan deletions)

Skim the current state. Do not assume anything is "load-bearing" without
verifying via `git blame`/lemma-callgraph:

- Driver `assume val` to replace:
  `EquivImplSpec.Sponge.Arm64.Driver.fst:101` (`lemma_squeeze2_arm64`)
- Pre-existing scaffolding in
  `EquivImplSpec.Sponge.Arm64.Steps.fst`: enumerate every lemma. For
  each, ask: "would the Portable-shape proof need this?" If no, mark
  for deletion. (You may discover the entire file shrinks dramatically.)
- Pre-existing helpers in `EquivImplSpec.Sponge.Arm64.fst` (the
  `arm64_sc_*` family). These are scalar-side lane bridges; likely still
  needed regardless of squeeze proof shape, but verify.
- `BRIEF_squeeze_steps.md` and any other agent-prompt files in this
  directory: stale guidance — read once for context, then ignore.

## Phase 2 — port the Portable shape to Arm64

The exact transformation depends on what you discover in Phase 0. The
template is roughly:

1. **Mirror the Rust-side ensures + body** from `portable.rs::squeeze`
   onto `simd128.rs::squeeze2` (Arm64 N=2 driver in
   `crates/algorithms/sha3/src/generic_keccak/simd128.rs`). Lift any
   single-lane facts to per-lane forall over `l: nat{l < 2}`. The
   bridge from per-lane spec to single-lane spec is
   `EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
   KA.lc_arm64 _ l`.
2. **Replace `assume val lemma_squeeze2_arm64`** with a `let` body
   that mirrors the Portable driver one-liner — typically just
   "call the function so its ensures lands in context, then dispatch
   per lane".
3. **Delete obsolete scaffolding** in
   `EquivImplSpec.Sponge.Arm64.Steps.fst` and adjacent files. The
   commit should both add the new structure AND remove the dead one;
   don't leave both in.

## Phase 3 — verify

Per `feedback_use_fstar_mcp`: inner-loop edits go through fstar-mcp lax
+ `--admit_except` for fast iteration; only batch `make` invocations
for final validation.

```bash
cd /Users/karthik/libcrux-sha3-focused

# After each Rust-side edit, re-extract:
./hax.py extract --crate libcrux-sha3   # or the project's exact command

cd crates/algorithms/sha3/proofs/fstar/equivalence

# Verify squeeze2 alone (Rust ensures wiring):
make OTHERFLAGS="--admit_except 'Libcrux_sha3.Generic_keccak.Simd128.squeeze2'" \
     run/Libcrux_sha3.Generic_keccak.Simd128.fst > /tmp/sq2-step1.log 2>&1
grep -nE '^\* Error|All verification|TOTAL TIME [0-9]+ ms' /tmp/sq2-step1.log | head

# Verify driver lemma alone:
make OTHERFLAGS="--admit_except 'EquivImplSpec.Sponge.Arm64.Driver.lemma_squeeze2_arm64'" \
     run/EquivImplSpec.Sponge.Arm64.Driver.fst > /tmp/sq2-driver.log 2>&1
grep -nE '^\* Error|All verification' /tmp/sq2-driver.log | head

# Final: real make check + downstream
make check/Libcrux_sha3.Generic_keccak.Simd128.fst > /tmp/sq2-make.log 2>&1
make check/EquivImplSpec.Sponge.Arm64.Driver.fst > /tmp/sq2-driver-make.log 2>&1
make check/EquivImplSpec.Sponge.Arm64.API.fst > /tmp/sq2-api.log 2>&1
make > /tmp/sq2-fullchain.log 2>&1   # full equivalence
```

After every `--admit_except` run, the resulting `.checked` is unsound
(written by F*'s `--cache_checked_modules` even with admits). The user's
2026-05-04 clarification: deletion of stale `.checked` files **is**
allowed when invalidated; touching mtimes is not. Per
`feedback_no_cache_nuke` only delete the specific file you contaminated.

## Hard constraints

- **rlimit cap 800** monolithic, **400** with `--split_queries always`
  (`feedback_rlimit_cap_800`). Do not bump above for any new annotation.
- Pipe `make` output to a log + grep, never `Read` the full log into
  context (`feedback_grep_make_output`).
- Per-fn debug budget **30-60 min wall** (`feedback_proof_debug_budget`).
  If Phase 2 stretches past that, document the blocker in
  `proofs/agent-status/` and surface to user.
- Do not silently apply `admit ()` / `--admit_smt_queries true` / `lax`
  to F* errors (`feedback_fstar_errors_ask_user`). Surface to user
  unless fix is mechanical (rlimit bump within cap, naming fix).
- Edits expected:
  - `crates/algorithms/sha3/src/generic_keccak/simd128.rs` (Rust-side
    ensures + body for `squeeze2`)
  - `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst`
    (replace `assume val` with `let`)
  - `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst`
    (delete obsolete scaffolding)
  - **Do not manually edit** the extracted
    `Libcrux_sha3.Generic_keccak.Simd128.fst`; re-extract via hax.

## Verification

- Primary: `make check/EquivImplSpec.Sponge.Arm64.Driver.fst.checked`
  passes, the `assume val` is gone.
- Downstream: `EquivImplSpec.Sponge.Arm64.API.fst` and the full
  `make` pass.
- Update `proofs/sha3-sprint-todo.md`: prune the Arm64 squeeze2 item;
  drop the load-bearing admit count.

## Followup — store_block (same framing)

The two remaining `admit ()` bodies on `Simd.{Arm64,Avx2}.store_block`
(set by `hax_lib::fstar!("admit()")` in `src/simd/{arm64.rs,avx2.rs}`)
should follow the **same study-the-current-Portable-shape-first**
discipline. The Portable `store_block` proof has likely been
restructured along with the squeeze proof. When you (or the next agent)
take that work on:

1. Read the *current* Portable `store_block` proof end-to-end.
2. Design the Arm64 / Avx2 versions to mirror it.
3. **Be willing to delete** any pre-existing per-byte aux scaffolding
   (`load_block`-mirror lemmas, byte-stream invariants, etc.) that no
   longer matches the Portable shape.
4. The Avx2 version is harder than Arm64 (8 forall conjuncts × 12 live
   arrays); per the old `BRIEF_load_store_block.md` it may need an
   opaque `load_block_chunk` helper. That advice may also be obsolete —
   re-derive it from whatever the current Portable proof does.

## Failure exit

1. If after Phase 0 you cannot articulate the Portable proof shape in
   two sentences, stop and ask the user — don't guess.
2. If the Rust-side `hax_lib::ensures` mirror fails type-check, the
   `simd128.rs` file likely uses a different `KA.lc_*` / `extract_lane`
   namespace than `portable.rs`. Fix mechanically.
3. If quantifier-cascade reappears at the per-lane forall layer:
   `[@@ "opaque_to_smt"]` on `Hacspec_sha3.Sponge.squeeze` for the
   duration of the proof, then unfold via lemma at the call site.
4. Document the dominant cascading quantifier and stop. The 60-min
   budget is the gate.
