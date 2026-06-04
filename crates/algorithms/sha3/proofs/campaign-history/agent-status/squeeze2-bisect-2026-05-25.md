# squeeze2 admit-bisect + workaround findings — 2026-05-25

Worktree: `/Users/karthik/libcrux-sha3-proofs` (branch `sha3-proofs-focused`)
HEAD: `4fc269a51` (status note for prior squeeze2 attempt)

## Setup

Restored the `1d35f933c` draft of:
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Simd128.fst` — `squeeze_blocks2` helper + strong-ensures `squeeze2`.
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst` — 3 helper lemmas; `lemma_squeeze_final_reconcile_arm64` body admitted (`--admit_smt_queries true`).

Applied `assert (valid_rate v_RATE); assert (v v_RATE <= 200)` in `squeeze_blocks2`'s aux0/aux1 — required for clean squeeze_blocks2 verification.

## Part 1 — Bisection (where is the LP-solver crash?)

| Position | Where in body | Result |
|---|---|---|
| A | After `let s_init_st`; entire body admitted | ok |
| B'1 (ELSE admitted) | THEN branch real | ok — THEN branch (blocks==0) doesn't trigger |
| B'2 (THEN admitted) | ELSE branch real | **LP-solver crash** |
| C (after squeeze_blocks2) | Admit immediately after `squeeze_blocks2` call | ok |
| D (after trailing if/else) | Admit after trailing-block if-else | ok |
| E (after final_reconcile calls) | Admit BEFORE both final asserts | DIFFERENT failure — regular "Assertion failed" at function-level post (NOT LP-solver) |
| F (between final asserts) | 1st assert real, 2nd assert admitted | **LP-solver crash** |

**Conclusion of bisection:** The crash trigger is the first per-lane byteform `assert` at lines 597-601 of the restored file. Not in helpers' preconditions or call sites — specifically in the byteform-equality assert itself. Confirmed `lar_solver.cpp:1066`, same bug HANDOFF.md docs for `lemma_absorb_rec_step`.

## Part 2 — Workaround sweep

| WA | Approach | Result |
|---|---|---|
| **#1** | `assert_norm` on the 2 final asserts | LP crash dodged but regular "Assertion failed" at function-level post (normalizer can't reduce `out0`/`squeeze`). **No progress.** |
| **#2** | `--using_facts_from '* -Hacspec_sha3.Sponge.squeeze_state'` wrapping squeeze2 | LP crash dodged but same regular "Assertion failed". **No progress.** |
| **#3** | Let-bind `spec0_l`, then `assert (out0 == spec0_l)` | LP crash dodged but same regular "Assertion failed" (F* doesn't unfold spec0_l for the function-level post). **No progress.** |
| **#4** | Standalone bridge lemma `lemma_squeeze2_final_bridge_arm64` in `Steps.fst`, body admitted — squeeze2 calls it instead of doing the 2 asserts | **WORKS** with bounds-only precondition (`valid_rate`, `outlen < MAX - 200`, `out1_final.len == out0_final.len`). Fails with strong arithmetic precondition (the LP bug returns). |
| **#5** | SMTPat-tagged variant of `final_reconcile` | Structurally infeasible — pattern can't cover all 7 universally-quantified variables (Z3 warns "pattern does not contain all quantified variables"). |

## Why WA4 works

The LP-solver bug is in the discharge of the byteform equality combined with the arithmetic about `outlen == blocks * rate + output_rem` etc. By:

1. Moving the byteform equality conclusion into a separate lemma's `ensures`, and
2. Requiring only bounds (no `outlen == blocks * rate + output_rem` decomposition),

we sidestep both triggers. The lemma body is admitted, so the proof obligation moves entirely into the helper — and the caller's discharge is clean.

## Net admit count

|  | Baseline | After WA4 |
|---|---|---|
| `lemma_squeeze2_arm64` | `assume val` (1 admit) | real `let`, 1-liner (0 admits) |
| `Simd128.squeeze2` body | Weak ensures, verified clean | Strong byteform ensures, verified clean via bridge call |
| `lemma_squeeze_final_reconcile_arm64` | (didn't exist) | admitted body (1 admit) |
| `lemma_squeeze2_final_bridge_arm64` | (didn't exist) | admitted body, bounds-only pre (1 admit) |
| `squeeze_blocks2` | (didn't exist) | verified clean (with valid_rate asserts fix) |
| **Total** | **1** | **2** |

**Net: +1 admit but structurally much better placed.** The 2 admits are on small internal helpers with bounds-only requires + clean per-lane post-conditions. Closing them is a focused next-task; closing `assume val lemma_squeeze2_arm64` directly required the entire squeeze_blocks2 + squeeze2 infrastructure first.

To make this *real* progress (net -1 admit), close `lemma_squeeze2_final_bridge_arm64`'s body by calling `lemma_squeeze_final_reconcile_arm64 × 2` — but that requires `final_reconcile`'s strong forall preconditions to be discharged at the bridge's call site, which re-introduces the LP-bug-triggering arithmetic. So `final_reconcile`'s admit is the actual blocker; closing it would unlock both.

## Recommendations for next session

1. **Close `lemma_squeeze_final_reconcile_arm64` without admit.** The earlier attempt failed due to Z3 not bridging `array_of_list 2 [out_l; out_l]` lookups with caller hypotheses (see prior bisect). The fix path is to refactor `lemma_squeeze_prefix_preserved_arm64` and `lemma_squeeze_trailing_byteform_arm64` to take `out_l: t_Slice u8` directly instead of the 2-element array. This eliminates the array-indirection that breaks Z3 instantiation.

2. **Alternative**: change F* version / Z3 version. The `lar_solver.cpp:1066` bug is specific to Z3 4.13.3. If Z3 4.12.x or 4.14.x avoids it, we could use that workaround at file-level (no body refactor needed).

3. **Alternative**: write the final-reconcile reasoning in `assert_norm`-friendly form, with explicit `Math.Lemmas` calls factoring all arithmetic out of the byteform-equality step. Tedious but precise.

## Failure mode confirmation

LP-solver crash error verbatim:
```
ASSERTION VIOLATION
File: ../src/math/lp/lar_solver.cpp
Line: 1066
Failed to verify: m_columns_with_changed_bounds.empty()
Z3 4.13.3.0
```

Same as HANDOFF.md's `lemma_absorb_rec_step` symptom.

## Tree state at end

Reverted to HEAD via `git checkout --` (no committed changes). `git status` confirms no modified files under `crates/algorithms/sha3/proofs/fstar/`.

## Session bookkeeping

- F* proxy MCP used throughout (curl via `/tmp/fp.sh` to `localhost:3002`).
- Multiple sessions opened (timed out / closed as needed); WA4 + WA5 ran on session `ea4a5dbc-96f7-4b32-a4fd-0fb0baf2765b` / `69b97a68-9728-4e8b-b7c1-4c159eeaa50d`.
- One `fstar.exe` process at peak, RSS <100 MB during workaround TCs.
- Total wall: ~45 minutes for bisect + 5 workarounds.
