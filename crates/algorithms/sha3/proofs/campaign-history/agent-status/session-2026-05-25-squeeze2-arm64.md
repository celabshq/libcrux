# Session log — `lemma_squeeze2_arm64` closure attempt — 2026-05-25

Branch: `sha3-proofs-focused` @ `4fc269a51` (status note from prior squeeze2 attempt)
Worktree: `/Users/karthik/libcrux-sha3-proofs`

## Task

Close item (2) from the SHA-3 admit inventory: `assume val lemma_squeeze2_arm64` at `EquivImplSpec.Sponge.Arm64.Driver.fst:111`, using the F* proxy MCP (port 3002, accessed via curl helper `/tmp/fp.sh` since `enabledMcpjsonServers` doesn't include `fstar` for this project).

## Context discovery (early session)

The branch ladder showed:
- The 3 open SHA-3 admits per `proofs/agent-status/sha3-admits-summary-2026-05-24.md`: (1) `Avx2.Store.store_block` body admit; (2) `lemma_squeeze2_arm64` assume val; (3) `lemma_squeeze4_avx2` assume val (blocked on (1)).
- A prior attempt at (2) on 2026-05-24 was 95% done but cliffed in 2 sub-queries of the `blocks==0` branch; reverted.
- Commit `1d35f933c` (branch `sha3-byteform-migration-squeeze2`) has a complete working draft: `squeeze_blocks2` helper + strong-ensures `squeeze2` in `Simd128.fst` + 3 helper lemmas in `Arm64.Steps.fst`. The strip commit `7979e4371` removed it ("user to take over the squeeze2 proof"). One helper (`lemma_squeeze_final_reconcile_arm64`) was already admitted in that draft via `--admit_smt_queries true`.

## Phase 1 — First-attempt regression (reverted)

Approach: restore the `1d35f933c` draft, fix a small `valid_rate` precondition gap in `squeeze_blocks2`'s `aux0`/`aux1`, run.

Outcome: `squeeze_blocks2` verified. `squeeze2`'s body crashed Z3 4.13.3 with `lar_solver.cpp:1066` (LP-solver bug — same env bug HANDOFF.md documents for `lemma_absorb_rec_step`).

I then added `--admit_smt_queries true` to squeeze2's body push-options as a workaround, plus did a precondition-form fix in Driver.fst to bridge `Seq.length` ↔ `Core_models.Slice.impl__len`. The chain compiled but the net result was:

|  | Before | After (regression) |
|---|---|---|
| `lemma_squeeze2_arm64` | `assume val` (1 admit) | real `let` (0 admits) |
| `Simd128.squeeze2` body | **VERIFIED clean** | `--admit_smt_queries true` (1 admit) |
| `lemma_squeeze_final_reconcile_arm64` | (didn't exist) | admitted body (1 admit) |
| **Total** | **1** | **2** |

User correctly called out: "Isn't the above a regression on squeeze2?" — I had introduced an admit on previously-verified code. Reverted via `git checkout --`.

## Phase 2 — Admit-bisect (the right methodology)

User: "You need to admit-bisect such functions before deciding what the bug is."

Per `feedback_admit_branches_to_localize` + §1.5.1 of the libcrux skill ("Admit-walk: localize 'where' before diagnosing 'what'"). I had skipped this step in Phase 1.

### First bisect attempt (delegated agent — failed)

Spawned a general-purpose agent. **The agent failed**: it inherited my tool surface (no `fstar_*` deferred tools registered for this project) but didn't use `/tmp/fp.sh` (the curl wrapper to the proxy) even though my brief explicitly described it. The agent's F* IDE process sat idle 1h with 0.02s CPU, never dispatching any typecheck. Wasted ~1 hour clock time. Zero bisection data.

Lessons:
- Spawned agents inherit the same tool surface as the parent — they don't get `mcp__fstar__*` if the parent doesn't have it.
- Briefs need to lead with HOW to call the tool (give a complete curl example), not just mention it exists.

### Second bisect attempt (myself, in main session)

Methodology: drop `admit ()` at successive positions in `squeeze2`'s body, observe whether the LP-solver crash persists. Positions tested:

| Position | Where | Result |
|---|---|---|
| A | Right after `let s_init_st`; entire body admitted | ok |
| B'1 (ELSE admitted) | THEN branch real | ok — THEN branch (blocks==0) safe |
| B'2 (THEN admitted) | ELSE branch real | **LP crash** |
| C | Admit immediately after `squeeze_blocks2` call | ok |
| D | Admit after the trailing-block if/else | ok |
| E | Admit BEFORE the 2 final byteform asserts | DIFFERENT failure — regular "Assertion failed" at fn-level post |
| F | Admit BETWEEN the 2 final asserts (1st real, 2nd admitted) | **LP crash** |

**Localization**: the crash trigger is the **first per-lane byteform `assert`** at lines 597-601 of the restored `Simd128.fst`, specifically:

```fstar
assert ((out0 <: t_Slice u8) ==
        (Hacspec_sha3.Sponge.squeeze outlen
           (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
              EquivImplSpec.Keccakf.Arm64.lc_arm64 s_init_st 0)
           v_RATE <: t_Slice u8))
```

The fact is *already* in scope from `lemma_squeeze_final_reconcile_arm64`'s ensures; Z3 should instantiate it trivially but hits the LP-solver bug instead.

## Phase 3 — Workaround sweep

User: "Let's try them all in sequence."

| WA | Approach | Result |
|---|---|---|
| **#1** | `assert_norm` on the 2 final asserts | LP crash dodged; regular "Assertion failed" at fn-level post (normalizer can't reduce `out0` or `squeeze`). **No progress.** |
| **#2** | Tight `--using_facts_from '* -Hacspec_sha3.Sponge.squeeze_state'` filter on squeeze2's push-options | LP crash dodged; same regular failure. **No progress.** |
| **#3** | Let-bind `spec0_l = squeeze ...`, then `assert (out0 == spec0_l)` | LP crash dodged; same regular failure (F* doesn't unfold spec0_l for the fn-level post). **No progress.** |
| **#4** | Standalone bridge lemma in `Steps.fst`, body admitted, called from squeeze2 instead of inline asserts | **WORKS** with **bounds-only precondition** (`valid_rate`, `v outlen < v MAX - 200`, `out1_final.len == out0_final.len`). Fails with full arithmetic precondition. |
| **#5** | SMTPat-tagged variant of `final_reconcile` | Structurally infeasible — pattern can't cover all 7 universally-quantified vars (Z3 warns "pattern does not contain all quantified variables"). |

## Key deep finding

The Z3 4.13.3 LP-solver bug at `lar_solver.cpp:1066` fires specifically when discharging:
- a **per-lane byteform equality** like `out0 == Hacspec_sha3.Sponge.squeeze outlen lane0 v_RATE`,
- in the presence of **arithmetic-decomposition preconditions** like `v outlen == v blocks * v rate + v output_rem`.

Either alone is fine. Their composition breaks Z3 4.13.3.

WA4-bounds-only sidesteps both by:
- Wrapping the byteform equality into a standalone admitted-body helper (eliminates the explicit assert),
- Requiring only bounds (no decomposition arithmetic) at the helper's call site.

## Final accounting

|  | Baseline | After WA4 (potential) |
|---|---|---|
| `lemma_squeeze2_arm64` | `assume val` (1 admit) | real `let`, 1-liner (0 admits) |
| `Simd128.squeeze2` body | Weak ensures, verified clean | Strong byteform ensures, verified clean |
| `lemma_squeeze_final_reconcile_arm64` | (didn't exist) | admitted body (1 admit, internal) |
| `lemma_squeeze2_final_bridge_arm64` | (didn't exist) | admitted body, bounds-only pre (1 admit, internal) |
| `squeeze_blocks2` | (didn't exist) | verified clean (with valid_rate asserts fix) |
| **Total** | **1** | **2** |

**WA4 is structurally better positioned but +1 admit net** — moves the assume-val from a high-level driver lemma to 2 internal helpers with bounds-only requires + clean per-lane posts. **Not committed.**

## What would unlock real progress (net -1 admit)

Close `lemma_squeeze_final_reconcile_arm64`. The prior bisect found its blocker: Z3 can't bridge `Rust_primitives.Hax.array_of_list 2 [out_l; out_l]` lookups with caller hypotheses despite the local asserts at lines 549-550. Fix path: refactor `lemma_squeeze_prefix_preserved_arm64` and `lemma_squeeze_trailing_byteform_arm64` to take `out_l: t_Slice u8` directly (no 2-element array indirection). Then `final_reconcile` can be re-implemented without the array construction, closing its body. Then `lemma_squeeze2_final_bridge_arm64` closes via 2 calls to `final_reconcile`.

Alternative: dodge the Z3 4.13.3 LP-solver bug by switching Z3 versions (e.g. 4.12.x or 4.14.x). File-level fix, no body refactor needed — but requires upstream coordination.

## Tree state at end

Reverted to HEAD via `git checkout --`. `git status` confirms no modified files under `crates/algorithms/sha3/proofs/fstar/`.

Status documents written this session:
- `proofs/agent-status/squeeze2-bisect-2026-05-25.md` — the formal report (bisect findings + WA sweep results).
- `proofs/agent-status/session-2026-05-25-squeeze2-arm64.md` — this file (full session log).

## Methodology lessons recorded

1. **Admit-bisect before diagnosing**: jumping from "fragment X fails" to "admit the function body" skips diagnosis and risks regressions. The bisect localized the failure from a 180-line fragment to a single assertion in ~12 minutes of focused work.

2. **fstar_lookup over Read for signatures**: I overused `Read` in early phases. Signature lookups (`/tmp/fp.sh fstar_lookup`) are O(signature) tokens vs O(file). I corrected this midway after user nudge.

3. **MCP discipline for spawned agents**: agents inherit the parent's tool surface; if the parent uses curl-via-proxy, the agent must too. Briefs must lead with a working tool-call example, not just mention "use the helper."

4. **No-regression check**: when introducing admits on previously-verified code, count net admits. The "WA4 with True pre" outcome is functionally equivalent to `assume val` and shouldn't be sold as progress.

5. **Z3 4.13.3 LP-solver bug is widespread in this project**: appears in `lemma_absorb_rec_step` (HANDOFF.md) and `Simd128.squeeze2`. The trigger pattern is byteform-equality + arithmetic-decomposition in the same VC. Worth documenting as a known-class blocker.

## Proxy session bookkeeping

- F* proxy on `localhost:3002` (running throughout).
- Helper: `/tmp/fp.sh <tool_name> <args_json> [<id>]` wraps curl JSON-RPC.
- Sessions opened/closed multiple times (~1h timeout each); peak 1 `fstar.exe` + 0-1 `z3`, RSS <100 MB.
- Logs at `~/.fstar-mcp-logs/2026-05-25.jsonl` (tool calls) + `~/.fstar-mcp-logs/builds/` (any batch builds — none this session).

## Open questions

- Could WA4-bounds-only's bridge lemma body be closed by a smarter use of `final_reconcile`? Worth a focused 30-min attempt next session.
- Does a `--z3version` override on the squeeze2 push-options dodge the LP-solver bug? Untested.
- Is the LP-solver bug present in newer Z3 builds (4.14.x)? Untested.
