# Prompt — strengthen the intrinsic + per-byte helpers needed by Arm64 `store_block`

## Mission

Build the **helper layer** that the Arm64 `store_block` discharge will lean on, then hand off to (or directly drive) the discharge in `crates/algorithms/sha3/src/simd/arm64.rs:255 hax_lib::fstar!("admit()")`. This prompt is scoped narrowly to *helpers* — the discharge itself is covered by `proofs/agent-prompt-store-block.md` and should consume what this agent produces.

Concretely, two helper-level gaps block the Arm64 store_block proof. Both are mirrors of work that already landed for `load_block` (commit `abf8b5297`, 2026-04-26):

1. **`_vst1q_bytes_u64` ensures is too weak.** The current ensures says only `future(out).len() == out.len()` (`crates/utils/intrinsics/src/arm64_extract.rs:264-269`). The store_block ensures asserts per-byte equalities into `out0`/`out1`; without a content post on `_vst1q_bytes_u64`, no proof of the loop body can connect the SIMD store to the byte-level forall. Compare the mirror `_vld1q_bytes_u64` at `arm64_extract.rs:225-239`, which already has the strong content post.
2. **No per-byte `store_lane_byte_*` aux lemmas.** `load_block` has `load_lane_u64` factored out as the per-lane bridge. `store_block` needs the inverse: a small set of per-byte helpers that (a) decompose `_vst1q_bytes_u64`'s post into the eight `to_le_bytes()` byte equalities for each lane, and (b) propagate those through `_vtrn1q_u64` / `_vtrn2q_u64` so the loop-invariant byte view of `out_k` advances 16 bytes per iteration without re-saturating Z3.

After this lands, the store_block discharge should be a near-mechanical mirror of the 2026-04-26 load_block closure plus optionally the AVX2 cascade-closure four-fix kit (commits `7bb581f8b`, `8203c9ace`, `28db4222a`, `3b9fc054c`).

## Setup

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b store-block-arm64-helpers \
    /Users/karthik/libcrux-sha3-store-arm64-helpers sha3-proofs-focused
```

Hard constraint: NEVER `cd` into `/Users/karthik/libcrux-sha3-focused`, `/Users/karthik/libcrux-sha3-store-arm64`, `/Users/karthik/libcrux-sha3-store-avx2`, the squeeze2 worktree, or any other sibling. Use `git -C /Users/karthik/libcrux-sha3-store-arm64-helpers ...` and absolute paths.

## Read FIRST (no skipping)

1. **Sprint state**: `crates/algorithms/sha3/proofs/sha3-sprint-todo.md` — items 3 and 4 in §"Suggested sprint order"; the `## AVX2 cascade closure (2026-05-05)` section at the top describes the reusable pattern kit.
2. **Parent prompt**: `crates/algorithms/sha3/proofs/agent-prompt-store-block.md` — the full Arm64+AVX2 discharge plan that consumes this agent's output.
3. **Load_block precedent on Arm64** (this is the structural template):
   ```
   git -C <worktree> log --oneline -p abf8b5297 -- \
       crates/algorithms/sha3/src/simd/arm64.rs \
       crates/utils/intrinsics/src/arm64_extract.rs \
       | head -300
   ```
   In particular: how `load_lane_u64` is defined as a `cfg(hax)` helper in `simd/arm64.rs:50-66`, and how `_vld1q_bytes_u64`'s post propagates through `load_u64x2x2`'s ensures into the loop invariant (`simd/arm64.rs:127-187`).
4. **AVX2 cascade closure precedent** for the lemma + tight-SMTPat shape:
   ```
   git -C <worktree> log --oneline -p 7bb581f8b 8203c9ace 28db4222a 3b9fc054c \
       -- crates/algorithms/sha3 crates/utils/intrinsics specs/sha3 | head -400
   ```
   The `load_lane_u64_lane_extensionality` SMTPat-tagged bridge lemma at the tail of `Libcrux_sha3.Simd.Avx2.fst` is the template for any per-byte SMTPat helper this agent ends up writing.
5. **Skills**: `fstar-mcp`, `fstar-for-libcrux` (especially §1.5.1 "Layered cascades and the danger of fixing one cascade with a new SMTPat" — the dual-trigger trap that bit AVX2 load_block applies symmetrically to anything with `to_le_bytes`), `smtprofiling`.
6. **Memory rules**:
   - `feedback_grep_make_output` — pipe make output to a log + grep, never Read full F* logs.
   - `feedback_use_fstar_mcp` — fstar-mcp for sub-second iteration; full SMT through make.
   - `feedback_fstar_mcp_session_dies_after_make` — recreate fstar-mcp session after each make.
   - `feedback_no_manual_edits_extracted` — for permanent fixes go through Rust source + re-extract; experimental probes during cascade debugging are OK after asking once.
   - `feedback_rlimit_cap_800` — never set rlimit > 800 mono / 400 with `--split_queries always`.
   - `feedback_proof_debug_budget` — 60 min hard cap per helper. After that, write a status doc and stop.
   - `feedback_smtpat_percent_above_trait` — SMTPat bodies that expose `%` raw above the trait boundary leak non-linear arithmetic. Bridge through `Hacspec_sha3.*` or per-byte plain bounds instead.

## Helper 1 — strengthen `_vst1q_bytes_u64` ensures

### Current state
```rust
// crates/utils/intrinsics/src/arm64_extract.rs:264-269
#[inline(always)]
#[hax_lib::ensures(|()| future(out).len() == out.len())]
#[hax_lib::lean::replace_body("()")]
pub fn _vst1q_bytes_u64(out: &mut [u8], v: _uint64x2_t) {
    unimplemented!()
}
```

### Target shape

Mirror `_vld1q_bytes_u64`'s `from_le_bytes` post inverted to `to_le_bytes`. The natural form:

```rust
#[hax_lib::requires(out.len() >= 16)]
#[hax_lib::ensures(|()|
    fstar!("future(out).len() == out.len() /\\
            (forall (i:nat{i < 16}).
              Seq.index (future $out) i ==
              Seq.index
                (Core_models.Num.impl_u64__to_le_bytes
                   (get_lane_u64x2 $v (i / 8))) (i % 8)) /\\
            (forall (i:nat{i >= 16 /\\ i < Seq.length $out}).
              Seq.index (future $out) i == Seq.index $out i)"))]
```

Two clauses:
- **Content** (i < 16): byte i of `future(out)` equals byte (i % 8) of lane (i / 8) of `v` after `to_le_bytes`.
- **Frame** (i >= 16): bytes past the 16-byte store window are unchanged.

The frame clause matters because store_block's loop writes into `out_k[start + 16*i..start + 16*(i+1)]` via slice indexing; the F* extraction lowers that to a slice update on `out_k` and the frame property is what tells Z3 that bytes outside the window survive.

### Construction recipe

1. Write the strengthened ensures in `arm64_extract.rs:264-269`. Keep the body `unimplemented!()` — this is an axiomatic intrinsic.
2. Re-extract: `./hax.py extract` from the repo root (or whatever the project's canonical command is — check `Makefile`/scripts).
3. Verify the extracted `Libcrux_intrinsics.Arm64_extract.fst` shows the new `val` shape.
4. Run `make check/Libcrux_intrinsics.Arm64_extract.fst` in `crates/utils/intrinsics/proofs/fstar/extraction/`. The ensures is on an axiomatic `unimplemented!()`, so it lands as an `assume val` post — should typecheck with no SMT cost.
5. Walk the downstream: every consumer of `_vst1q_bytes_u64` should now have a stronger fact in scope. Verify no consumer regressed:
   ```
   grep -rn 'e_vst1q_bytes_u64\|_vst1q_bytes_u64' \
       /Users/karthik/libcrux-sha3-store-arm64-helpers/crates \
       /Users/karthik/libcrux-sha3-store-arm64-helpers/specs \
       2>&1 | head -40
   ```
   Re-`make` each consumer module the search turns up. Pipe to `/tmp/<mod>.log`, grep for `Error|All verification`.

### Pitfalls

- **Don't write the post over `to_le_bytes` of `get_lane_u64`** (the wrapper) — write it over `get_lane_u64x2` (the underlying lane-projection over `_uint64x2_t`). The former lives in `arm64_extract.rs:259` and is itself an `unimplemented!()` axiom whose post says `result == get_lane_u64x2 vec (v lane)`; introducing it on both sides of the SMTPat creates a needless rewrite step. Stick with `get_lane_u64x2`.
- **Don't introduce SMTPat on this `val`'s ensures.** Functions in the intrinsics layer get their ensures consumed by every call site; an over-eager SMTPat fires on every `_vst1q_bytes_u64` call (16 within store_block's loop body alone) and saturates Z3. The pattern that works is to keep the ensures bare, then add a *targeted* SMTPat lemma at the SHA-3 layer (helper 2 below) that re-exposes the per-byte content only when triggered by `Seq.index out k`-shaped goals.
- **Don't prove per-byte content via `Rust_primitives.Arrays.eq_intro`.** That function lifts elementwise equality to array equality; we need the inverse direction here. Just spell out the forall explicitly.

## Helper 2 — per-byte SMTPat bridge lemma at the SHA-3 layer

The store_block invariant is a `forall j. j < out_k.len() ==> ...` partitioned by `start`, `start + i*16`, etc. Z3 needs to derive each loop-iteration step:

```
out_k[j] == get_lane_u64(s[(j-start)/8], k).to_le_bytes()[(j-start) % 8]
```

for `j` in the freshly-stored window `[start + 16*i, start + 16*(i+1))`. With helper 1 in place, `_vst1q_bytes_u64` says exactly this for the 16-byte slice argument; the gap is reconciling the slice-update view (`Seq.upd` / `Seq.append` chain) with the absolute-index view in the invariant.

### Lemma shape

A single SMTPat-tagged lemma in a new file `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` (or appended near the end of `Libcrux_sha3.Simd.Arm64.fst` if hax co-extraction is awkward — start with the dedicated module, fall back if extraction layout fights you):

```fstar
let store_lane_byte_extensionality
    (out: Seq.seq u8) (out': Seq.seq u8)
    (v: _uint64x2_t)
    (start: usize) (i: usize) (j: usize{v j < Seq.length out})
  : Lemma
    (requires
      v start + 16 <= Seq.length out
      /\ v i + 16 <= Seq.length out  (* or the relevant offset arithmetic *)
      /\ (* slice-update relation: out' = out[start+16*i..start+16*(i+1)] := store(v) *) 
        ...)
    (ensures
      (if v j >= v start + 16 * v i /\ v j < v start + 16 * (v i + 1) then
         Seq.index out' (v j) ==
         Seq.index (impl_u64__to_le_bytes (get_lane_u64x2 v (((v j - v start) - 16 * v i) / 8)))
                   (((v j - v start) - 16 * v i) % 8)
       else
         Seq.index out' (v j) == Seq.index out (v j)))
    [SMTPat (Seq.index out' (v j)); SMTPat (...one more matching the slice-update structure...)]
```

The exact universals depend on how hax extracts the slice update. Inspect `Libcrux_sha3.Simd.Arm64.fst:1030` (the `_vst1q_bytes_u64 (out0.[ ... ])` call site) to read the actual lowered shape, then mirror its quantifiers.

### Trigger discipline (CRITICAL — this is where AVX2 load_block burned 2 days)

Use a **dual SMTPat with symmetric specificity**. From `feedback_smtpat_lane_propagation` and the AVX2 cascade closure:

- Both triggers must share *all* universals of the lemma. If one trigger contains only `start`/`i`/`out'` and the other contains only `v`/`j`, Z3 instantiates the cross-product over the broad trigger's scope (4 M+ instantiations were observed in the AVX2 trap).
- A working dual is: `[SMTPat (Seq.index out' (v j)); SMTPat (e_vst1q_bytes_u64 ... v ...)]` — both fire only when the goal mentions the specific byte index AND a corresponding store call. Cross-products are bounded.
- Bodies that expose raw `%` / `/` above the trait layer are dangerous (`feedback_smtpat_percent_above_trait`). Bound the modular arithmetic inside the lemma body, expose only `to_le_bytes(get_lane_u64x2 …)` and plain bounds in the post.

### Construction recipe

1. Read `Libcrux_sha3.Simd.Arm64.fst:1024-1064` (the inner-loop `_vst1q_bytes_u64` call sites for `out0` and `out1`) to see the exact `Seq.update_slice` / `..[ start + 16*i .. start + 16*(i+1) ] := ...` shape hax emits.
2. Sketch the lemma in fstar-mcp (`typecheck_buffer kind="lax"`) without SMTPat, prove it from `_vst1q_bytes_u64`'s strengthened ensures alone. Should be ~5 lines once helper 1 lands. If it isn't, the helper-1 ensures shape is wrong — iterate on it.
3. Add the dual SMTPat. Verify with `kind="full"` on a synthetic call site (a tiny test lemma `let _ = assert (Seq.index out' j == ...)` after a single `_vst1q_bytes_u64` call) before wiring into store_block.
4. Run `make check/Libcrux_sha3.Simd.Arm64.fst OTHERFLAGS='--admit_except "<helper>"' > /tmp/helper.log 2>&1`, grep errors. Once that's clean, hand off to the consumer.

### Optional sub-helpers

If qi.profile shows the byte-arithmetic (`(j - start) / 8`, `(j - start) % 8`) cascading: factor out a `byte_index_to_lane_byte` lemma that maps `j ∈ [start, start+16)` to the `(lane, byte_in_lane)` pair, opaque-to-SMT, and let the main extensionality lemma cite it. This mirrors the load_block `Hacspec_sha3.createi` opacification (commit `7bb581f8b`).

## Helper 3 — `_vtrn1q_u64` / `_vtrn2q_u64` byte-view propagation (write only if helper 2 isn't enough)

The store loop deinterleaves two state lanes via `_vtrn1q_u64` and `_vtrn2q_u64` before storing. Helpers 1+2 give per-byte equalities for the *output* of these intrinsics, but the loop invariant cites lanes of `s[(j-start)/8]` directly — i.e., the *input* to the trn. The trn intrinsics already have lane posts:

```
get_lane_u64x2 (vtrn1q_u64 a b) 0 == get_lane_u64x2 a 0
get_lane_u64x2 (vtrn1q_u64 a b) 1 == get_lane_u64x2 b 0
get_lane_u64x2 (vtrn2q_u64 a b) 0 == get_lane_u64x2 a 1
get_lane_u64x2 (vtrn2q_u64 a b) 1 == get_lane_u64x2 b 1
```

Composing with helper 1 should already give Z3 enough to discharge the byte-level equalities at the loop body. If qi.profile after helper-2 wiring shows a residual cascade in the trn-composition, add a small `trn_to_le_bytes_eq` lemma that pre-computes the composition. Don't add it speculatively — only on cascade evidence.

## Inner loop

- fstar-mcp for type-shape iteration (sub-sec).
- After every Rust-side ensures change, **re-extract** (`./hax.py extract`) before checking. The fstar-mcp session is on the *extracted* file.
- After every `make`, recreate the fstar-mcp session (`feedback_fstar_mcp_session_dies_after_make`).
- Per-helper validation: `make check/<Module>.fst OTHERFLAGS='--admit_except "<helper>"' > /tmp/<name>.log 2>&1`, then `grep -nE '^\* Error|All verification|TOTAL TIME' /tmp/<name>.log`. Never Read the full make log.

## Status reports every 15 min

Append to `crates/algorithms/sha3/proofs/agent-status/store-block-arm64-helpers-progress.md`:
```
## 2026-05-DD, T+N (sub-task)
- Sub-task: <which helper, which phase>
- Blocker: <if any>
- ETA: <next checkpoint>
```

## File boundaries

This agent owns:
- `crates/utils/intrinsics/src/arm64_extract.rs` (helper 1, ensures-only edit)
- `crates/algorithms/sha3/src/simd/arm64.rs` (only if helper 2/3 needs a `cfg(hax)` definition; prefer the dedicated `.fst` module)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` (new) — for helper 2/3, if a dedicated module is feasible
- `crates/algorithms/sha3/proofs/agent-status/store-block-arm64-helpers-*.md`

Does NOT touch:
- The `hax_lib::fstar!("admit()")` line on `arm64.rs:255` itself — that is the consumer's job (parent prompt). Leave the admit in place; the helper agent's deliverable is "helpers exist and verify standalone, store_block discharge will land in a follow-up commit".
- `EquivImplSpec.*` modules.
- The AVX2 store_block worktree's files.
- Squeeze2 / squeeze4 driver lemmas.
- `crates/utils/intrinsics/src/avx2_extract.rs` — the cascade-closure commits already touched it; further edits only if cross-cutting and surfaced to user first.

If you find a cross-cutting fix is required (e.g., the `_vst1q_bytes_u64` ensures shape needs a new spec-side function in `Hacspec_sha3.*`), STOP and surface to user before editing shared modules. `feedback_develop_locally_upstream_once` — develop the auxiliary in the consumer file first; only graduate to a shared module after shape is final.

## Deliverables

Commit on the `store-block-arm64-helpers` branch (do NOT push):

- **Success**: helpers 1 (and 2, optionally 3) verify. `make` passes for `Libcrux_intrinsics.Arm64_extract.fst`, `Libcrux_sha3.Simd.Arm64.fst` (still with the body admit), and the synthetic test lemma demonstrating the SMTPat fires correctly. The store_block body admit is **untouched**. Commit message states which helpers landed and what gap (if any) remains for the consumer agent.
- **Partial**: status doc at `proofs/agent-status/store-block-arm64-helpers-2026-05-DD.md` with the ensures shape attempted, what made it fail (qi.profile, top quantifier), next-attempt path. Commit progress to worktree branch.

## Final report (≤300 words)

(1) Which helpers landed (1, 1+2, 1+2+3)? (2) For helper 1: exact ensures shape used. (3) For helper 2: lemma signature, dual-SMTPat triggers, max sub-query time. (4) For helper 3 (if needed): cascade evidence that prompted it. (5) Synthetic-test verification status. (6) Files committed and branch SHA. (7) Specific notes for the consumer agent — anything about the helper API that affects how store_block's loop invariant should be phrased.

Don't paste full F* logs — summarize.

## Suggested first 30 minutes

1. `git -C <worktree> log --oneline -p abf8b5297 -- crates/utils/intrinsics/src/arm64_extract.rs crates/algorithms/sha3/src/simd/arm64.rs | head -250` — read the load_block discharge end-to-end. The store side mirrors it.
2. Read `_vld1q_bytes_u64` ensures (`arm64_extract.rs:225-239`) and the existing `_vst1q_bytes_u64` (`:264-269`).
3. Sketch helper 1's ensures shape on paper. Validate that the symmetry with `_vld1q_bytes_u64` holds (lanes 0,1 → bytes 0..7, 8..15).
4. Edit `arm64_extract.rs:264-269`, re-extract, run `make check/Libcrux_intrinsics.Arm64_extract.fst > /tmp/h1.log 2>&1`, grep. Should land in <10 min.
5. First status entry to progress.md.
6. Read `Libcrux_sha3.Simd.Arm64.fst:1024-1064` to plan helper 2.

If by T+45 min helper 1 isn't landing, escalate by surfacing a 1-paragraph blocker note and asking the user before grinding more.
