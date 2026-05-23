# SHA-3 F* Equivalence Proofs — Sprint Punch List

**Audited**: 2026-05-01 (branch `sha3-proofs-focused`)
**Last status update**: 2026-05-23 — toolchain pinned at F\* 2026.03.24 + hax `integer-lemmas` (0.3.6 engine); 2 e-matching cliff admits added on the AVX2/Arm64 `byte_eq` mirrors of the `extract_lane (load_block …)` consumer pattern; SqueezeBytes rlimit bump landed; see §"2026-05-23 STATUS DELTA" below.
**Prior status update**: 2026-05-05 — AVX2 `load_block` cascade fully closed (commits `7bb581f8b`, `8203c9ace`, `28db4222a`, `3b9fc054c` on `sha3-proofs-focused`); see §"AVX2 cascade closure" below.
**Scope**: `/crates/algorithms/sha3/proofs/fstar/{equivalence,extraction}/` plus consumers in `/crates/algorithms/sha3/src/` and `/specs/sha3/proofs/fstar/extraction/`.

Coverage: 5 load-bearing admits (3 pre-existing + 2 session 2026-05-23), 1 elevated-rlimit lemma (SqueezeBytes 800), 2 hint-replay-flake spots. AVX2 `load_block` is in fast-pass territory after 2026-05-05; Arm64 `store_block` body admit was discharged in commits `83d1a04c2`, `c14f94d2c`, `29424f593`; `Portable.squeeze` and `lemma_theta_rho_to_spec` `--admit_smt_queries` were also removed since the original audit. AVX2 `store_block` body admit remains. The two new session admits are on the `byte_eq` inner helpers of `lemma_load_block_eq_xor_block_into_state_{arm64,avx2}` — same `extract_lane (load_block …)` consumer cliff family as 2026-05-05 but on the equivalence side rather than the impl side.

## 2026-05-23 STATUS DELTA

**Toolchain pinning.** F\* nightly-2026-04-12 introduces 3 regressions on this tree (`FStar.Tactics.V1` namespace removed, two `assert_norm (List.Tot.length …)` failures on `mk_list_*` helpers + `Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS`). Reverted to F\* 2026.03.24, preserved at `~/.local/fstar-2026.03.24/`; nightly install moved to `~/.local/fstar-nightly-2026-04-12/`. The `~/.local/fstar` symlink targets the active version. Active hax remains `integer-lemmas` @ `952bee04` with `hax-engine` 0.3.6 pinned at `~/hax/engine`.

**Build flags.** Baseline build now requires `OTHERFLAGS="--z3rlimit_factor 4"` to clear budget-bound failures. The structural e-matching cliff failures (next item) are NOT budget-bound — they bottom out at the same rlimit-used value regardless of factor (tested 4 and 16).

**Two new session admits (e-matching cliff, mirrored pair):**

- `EquivImplSpec.Sponge.Arm64.fst:124` — `byte_eq` inner `admit ();` inside `lemma_load_block_eq_xor_block_into_state_arm64`. Failing query used 59/1600 rlimit (3.7%, e-matching bound). Same `extract_lane (load_block …)` consumer-site cliff family as the 2026-05-05 `Libcrux_sha3.Simd.Arm64.load_block` cascade; that cascade was fixed at the source (`[@@ "opaque_to_smt"]` on `Hacspec_sha3.createi` etc.) but this is a *downstream* consumer of `load_block`'s ensures, on a different reasoning shape (per-byte `Seq.index lhs i == Seq.index rhs i` instead of per-lane `forall i. load_lane_u64 …`).
- `EquivImplSpec.Sponge.Avx2.fst:125` — exact AVX2 mirror of the Arm64 admit; 73/1600 rlimit (4.6%).

**Fix path** (deferred; not done this session): qi.profile the failing query per `~/.claude/skills/fstar-for-libcrux/SKILL.md §1.5` to identify the dominant quantifier in the `byte_eq` body context. Candidate root causes by analogy to load_block: (a) `KA.arm64_lane` / `KA.avx2_lane` not opaque enough; (b) `Hacspec_sha3.Sponge.xor_block_into_state` body unfolding into ApplyTT chains; (c) `lemma_subslice_bytes_eq`'s SMTPat trigger too broad. Profile work was beyond this session's scope.

**SqueezeBytes rlimit bump.** `EquivImplSpec.Sponge.Portable.SqueezeBytes.lemma_squeeze_step_byte_write` (line 28) was hitting the prior 1600 rlimit budget with `canceled` reason on a subtyping check for `(i *! rate)` overflow-fits-in-usize. Bumped local push-options from `--z3rlimit 400` to `--z3rlimit 800` (over the skill's "rlimit 800 is a smell" line). Resolves at ~2000 rlimit used / 644s wall. Per `~/.claude/skills/fstar-for-libcrux/SKILL.md §7`, the proper fix is to factor the lemma — added as proposed sprint item (§"Suggested sprint order" item 9).

**Hints not recorded.** The session ran with `ENABLE_HINTS="--use_hints"` (no `--record_hints`) so the admit-bearing state didn't pollute the `.fstar-cache/hints/` cache. Future un-admit attempts won't start from spuriously-green hint files.

**Resume note artifact gap.** Session-handoff note at `proofs/agent-status/RESUME-PROMPT-2026-05-23.md` references three diagnostic files (`hax-0.3.7-regression-bug-report.md`, `sha3-toolchain-upgrade-2026-05-2{1,2}.md`, `CONTINUATION.md`) that do not exist anywhere on disk. The resume note itself is the only artifact from the prior cascade-attempt session that survived. If those notes are recoverable from another worktree's git stash, they'd be useful context for the next bug-report-upstream attempt.

---

## AVX2 cascade closure (2026-05-05) — STATUS DELTA

The AVX2 `load_block` proof was previously slow/flaky (q314 12.2 s, q180/q182 ~250 ms, 700+ split sub-queries). It now verifies clean at max sub-query 1.5 s after a 4-layer cascade fix:

1. `[@@ "opaque_to_smt"]` on `specs/sha3/Hacspec_sha3.createi` (commit `7bb581f8b`).
2. `get_lane_u64` ensures-refinement → separate SMTPat lemma in `crates/utils/intrinsics` (commit `8203c9ace`).
3. `--using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'` on the load_block options (commit `28db4222a`).
4. Local `[@@ "opaque_to_smt"]` on `Libcrux_sha3.Simd.Avx2.load_lane_u64` plus a `load_lane_u64_lane_extensionality` SMTPat-tagged bridge lemma with **tight dual-`load_lane_u64`-trigger** pattern (commit `3b9fc054c`).

The trap inside (4): the first lemma drafted used a `[SMTPat (load_lane_u64 ... s1 ...); SMTPat (get_lane_u64 s2 lane)]` multi-pattern. The broad `get_lane_u64` second trigger fired the lemma 3.95 M / 4.58 M times in the failing sub-queries (q693/q797) — 7× larger than `k!61`. Tightening the second trigger to also be `load_lane_u64 ... s2 ...` (sharing all of `(blocks, offset, i, lane)` with the first) cut instantiations by ~3 orders of magnitude. Documented in `~/.claude/skills/fstar-for-libcrux/SKILL.md §1.5.1`.

**Reusable patterns** for the upcoming `store_block` work:
- The `--using_facts_from '... -array_from_fn -rem_euclid'` filter is likely needed for store_block too — same upstream cascade source feeds via Slice/byte refinements.
- Multi-pattern SMTPats need *symmetric trigger specificity*; never mix one tight with one broad.
- `smtprofiling` (qi.profile) is the only tool that finds these — re-profile after every fix; the post-fix dominant quantifier is often the lemma you just added.

---

## TL;DR — load-bearing admit count: 5

| # | Kind | Where | Platform |
|---|---|---|---|
| 1 | `assume val` driver lemma | `EquivImplSpec.Sponge.Arm64.Driver.fst:111` (`lemma_squeeze2_arm64`) | Arm64 |
| 2 | `assume val` driver lemma | `EquivImplSpec.Sponge.Avx2.API.fst:87`  (`lemma_squeeze4_avx2`) | Avx2 |
| 3 | body `admit ()` | `Libcrux_sha3.Simd.Avx2.Store.fst:165` (`store_block`) — set by `src/simd/avx2/store.rs:74 hax_lib::fstar!("admit()")` | Avx2 |
| 4 | inner `byte_eq` `admit ();` (session 2026-05-23) | `EquivImplSpec.Sponge.Arm64.fst:124` (`lemma_load_block_eq_xor_block_into_state_arm64`) | Arm64 |
| 5 | inner `byte_eq` `admit ();` (session 2026-05-23) | `EquivImplSpec.Sponge.Avx2.fst:125` (`lemma_load_block_eq_xor_block_into_state_avx2`) | Avx2 |

**Removed since the 2026-05-01 audit** (no longer load-bearing):

- ~~`Libcrux_sha3.Simd.Arm64.store_block` body admit~~ — **DISCHARGED** in commits `c14f94d2c` (loop body), `29424f593` (tail wrappers), `83d1a04c2` (full/_tail split). Mirror function load_block was discharged on 2026-04-26 (commit `abf8b5297`).
- ~~`Libcrux_sha3.Generic_keccak.Portable.squeeze` `--admit_smt_queries true`~~ — **REMOVED** (date not bisected; verify pre-session-2026-05-21 commit).
- ~~`EquivImplSpec.Keccakf.Generic.lemma_theta_rho_to_spec` `--admit_smt_queries true`~~ — **REMOVED** (date not bisected; verify pre-session-2026-05-21 commit).

Items 1 is Arm64; items 2, 3, 5 are Avx2; item 4 is Arm64 (session). Items 4 and 5 are the *equivalence-side* `byte_eq` cliff mirror pair on `extract_lane (load_block …)` consumers — proof scaffolding around the 2026-05-05 cascade closure didn't reach this consumer-site reasoning shape. There are NO `ADMIT_MODULES` or `SLOW_MODULES` set in any SHA-3 Makefile. There is one elevated-rlimit lemma (`EquivImplSpec.Sponge.Portable.SqueezeBytes.lemma_squeeze_step_byte_write` at `--z3rlimit 800 --split_queries always` — over the skill's hard cap; see §"2026-05-23 STATUS DELTA").

---

## Portable

### Admits

- **`Libcrux_sha3.Generic_keccak.Portable.squeeze` body** — `--admit_smt_queries true` push-options on the function body. Comment cites query 227 crossing 150 s line; root cause is 4 nested `forall_intro` calls on per-byte aux lemmas (`aux_write`, `aux_tail`, `aux_write_step`, `aux_tail_step`) inside a `fold_range` whose 4-clause invariant cites `Hacspec_sha3.Sponge.squeeze_blocks` + `squeeze_state`. Z3 has to compose per-byte forall × block-indexed forall in one sub-query. **Fix**: factor each per-byte aux into a top-level `lemma_squeeze_*_byte_*` lemma proven once, then in-body cites them by name. — `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Portable.fst:482`

- **`EquivImplSpec.Keccakf.Generic.lemma_theta_rho_to_spec`** — `--admit_smt_queries true`. 25-index `eq_intro` post-condition consolidation; 293/294 split sub-queries succeed in <300 ms each, the remaining one is the forall-precondition lift. Three reverted stabilization attempts (commit 80e03e0a5). **Fix**: factor into 5 row-helpers (one per row of 5 indices) using existing `lemma_rho_thru_K_extract_lane` partials, then assemble. ~1 sprint of careful work. — `EquivImplSpec.Keccakf.Generic.fst:1309`

  **2026-05-04 minor-tweak triage (60-min budget, all reverted)** — three additional attempts confirmed the regression is structural, not config:
  1. `#restart-solver` + `--fuel 0 --ifuel 1 --z3rlimit 800 --z3refresh` (no body change): monolithic query failed at 230 s / 800 rlimit; F* auto-split, the final eq_intro consolidation sub-query still failed at 224 s / 800.
  2. Replace `eq_intro` with `introduce forall (i: nat{i<25}). Seq.index lhs i == Seq.index rhs i with ();` then `eq_intro` (rlimit 800 + `--split_queries always`): same failure pattern, single sub-query at 800/800 timeout.
  3. Same as (2) but with explicit 25-branch `if i = 0 then () else if i = 1 then () ...` body inside `with`: failed on TWO sub-queries (the monolithic + one mid-cascade at sub-query 92), 230 s and 226 s respectively at 800/800 — matching the previously-reverted milestone-A attempt #3 cost profile.
  4. **Weakened post to `forall (i: nat{i < 25}). lhs.[mk_usize i] == rhs.[mk_usize i]`** (avoids `eq_intro` inside this lemma; updated the single caller `lemma_one_round_to_spec` at line 1656 to apply `Rust_primitives.Arrays.eq_intro` locally to bridge to array equality). Body: just the 3 helper lemma calls + `lemma_rotate_left_zero` (no `eq_intro`). Result: F* still auto-split into ~26 sub-queries; the per-i body sub-queries succeeded, but the final forall-consolidation sub-query (#27) failed at 198 s / 800 rlimit. The monolithic VC (#1) also failed at 201 s. **The forall consolidation is a structural Z3 limitation, independent of whether it appears as `eq_intro`'s precondition or as the lemma's own postcondition** — moving the eq_intro to the caller doesn't help, and the caller would inherit the same forall→array-equality lift.
  Conclusion: every attempt fails on the same Z3 quantifier-instantiation cascade — lifting 25 in-scope per-index equalities to a `forall i. ...` under `--ifuel 1`. SMT-config knobs (`#restart-solver`, `--z3refresh`, `--split_queries always`, `introduce forall` sugar, manual case-split, weakened post) do not help. **The structural 5-row-helper factoring is the only remaining path.** Profile not collected: each iteration cost 8–10 min wall (`--admit_except` strategy), so the budget closed before profiling could fit.

### Coverage map (per phase)

| Phase | Lemma | Status | File:line |
|---|---|---|---|
| load_block | `portable_sc_load_block` | proven (`let`) | `EquivImplSpec.Sponge.Portable.fst:332` |
| load_last | `portable_sc_load_last` | proven (`let`) | `EquivImplSpec.Sponge.Portable.fst:378` |
| store_block | `portable_sc_store_block` | proven (`let`) | `EquivImplSpec.Sponge.Portable.fst:432` |
| absorb_block | `lemma_absorb_block_portable` | proven (`let`) | `EquivImplSpec.Sponge.Portable.Steps.fst:48` |
| absorb_final | `lemma_absorb_last_portable` | proven (`let`) | `EquivImplSpec.Sponge.Portable.Steps.fst:107` |
| squeeze_block | `lemma_squeeze_block_portable` | proven (`let`) | `EquivImplSpec.Sponge.Portable.Steps.fst:155` |
| squeeze_last | `lemma_squeeze_last_portable` | proven (`let`) | `EquivImplSpec.Sponge.Portable.Steps.fst:203` |
| squeeze_one_step | `lemma_squeeze_one_step_portable` | proven (`let`) | `EquivImplSpec.Sponge.Portable.Steps.fst:268` |
| driver: absorb | `lemma_absorb_portable` | proven | `EquivImplSpec.Sponge.Portable.API.fst:69` |
| driver: squeeze | `lemma_squeeze_portable` | proven | `EquivImplSpec.Sponge.Portable.API.fst:98` |
| driver: keccak1 | `lemma_keccak1_portable` | proven | `EquivImplSpec.Sponge.Portable.API.fst:131` |
| top-level: sha224..shake256 | `lemma_{sha224,sha256,sha384,sha512,shake128,shake256}_portable` | proven | `EquivImplSpec.Sponge.Portable.API.fst:153–212` |

### Slow / flaky lemmas (Portable)

- **`EquivImplSpec.Sponge.Portable.lemma_load_last_equals_load_block_on_padded`** — split into 122 sub-queries; **4 known hint-replay failures** (queries 33, 45, 78, 83) all caught with "incomplete quantifiers" at 80 ms, then succeed without hints in retry. Hint file is fragile (subtyping check fails on `Rust_primitives.Integers.range_t USIZE`). **Fix**: regenerate hints after a clean build, or refactor to lift the offending int → usize coercions out of the proof body. — `EquivImplSpec.Sponge.Portable.fst:179-240`

- **`EquivImplSpec.Sponge.Portable.API.lemma_squeeze_portable`** — at `--z3rlimit 800 --split_queries always`. Per the cascade profile (`proofs/squeeze-cascade-profile.md`), 4 sub-queries (q224, q231, q263, q280) all hit `rlimit 400/400` and CANCEL when run inline (`--z3rlimit 400`). Currently passes only because the call site sits at rlimit 800. **Fix**: factor the post-loop branch (output_rem != 0 path) into a separate lemma, or inline `aux_partial` differently. — `EquivImplSpec.Sponge.Portable.API.fst:97 (#push-options "...rlimit 800 --split_queries always")`

- **`EquivImplSpec.Sponge.Portable.fst::squeeze` (cluster 3, q169-q170)** — q170 used 326/400 rlimit. >80% utilization is "flaky territory" per house policy. — `EquivImplSpec.Sponge.Portable.fst` (loop body at lines 594-649)

---

## Arm64 (NEON, N=2)

### Admits

- **`lemma_squeeze2_arm64`** — `assume val` driver-level lemma. Per-lane equivalence of `Generic_keccak.Simd128.squeeze2` to scalar `Hacspec_sha3.Sponge.squeeze`. Building block lemma `lemma_squeeze_one_step_arm64` (Steps.fst:243) IS proven; it preserves the per-lane invariant for one loop iteration. **Fix path**: replace `assume val` with `let` body that calls `lemma_squeeze_one_step_arm64` 2× per iteration of an inline loop-invariant proof on `Simd128.squeeze2` (per `BRIEF_squeeze_steps.md`). ~1 session of mechanical work. — `EquivImplSpec.Sponge.Arm64.Driver.fst:111`

- **`EquivImplSpec.Sponge.Arm64.lemma_load_block_eq_xor_block_into_state_arm64` `byte_eq` inner admit** (session 2026-05-23) — `admit ();` at start of inner `byte_eq` lemma body. Same e-matching cliff family as 2026-05-05 load_block; see §"2026-05-23 STATUS DELTA" and §"Suggested sprint order" item 4 for fix path. — `EquivImplSpec.Sponge.Arm64.fst:124`

- ~~`Libcrux_sha3.Simd.Arm64.store_block` body~~ — **DISCHARGED** in commits `c14f94d2c` (loop body via `store_u64x2x2` wrapper), `29424f593` (tail wrappers), `83d1a04c2` (`_full`/`_tail` split). No longer an admit; retained here as a closed-item marker.

### Coverage map (per phase)

| Phase | Lemma | Status | File:line |
|---|---|---|---|
| load_block (sc) | `arm64_sc_load_block` | proven | `EquivImplSpec.Sponge.Arm64.fst:150` |
| load_last (sc) | `arm64_sc_load_last` | proven | `EquivImplSpec.Sponge.Arm64.fst:386` |
| store_block (sc) | `arm64_sc_store_block` | proven (in equivalence file) | `EquivImplSpec.Sponge.Arm64.fst:487` |
| store_block (extraction body) | (no admit) | proven (commits `c14f94d2c` / `29424f593` / `83d1a04c2`) | `Libcrux_sha3.Simd.Arm64.fst` |
| absorb_block | `lemma_absorb_block_arm64` | proven | `EquivImplSpec.Sponge.Arm64.Steps.fst:54` |
| absorb_final | `lemma_absorb_last_arm64` | proven | `EquivImplSpec.Sponge.Arm64.Steps.fst:92` |
| squeeze_block | `lemma_squeeze_block_arm64` | proven | `EquivImplSpec.Sponge.Arm64.Steps.fst:137` |
| squeeze_last | `lemma_squeeze_last_arm64` | proven | `EquivImplSpec.Sponge.Arm64.Steps.fst:179` |
| squeeze_one_step | `lemma_squeeze_one_step_arm64` | proven | `EquivImplSpec.Sponge.Arm64.Steps.fst:243` |
| driver: absorb2 | `lemma_absorb2_arm64` | proven | `EquivImplSpec.Sponge.Arm64.Driver.fst:80` |
| driver: squeeze2 | `lemma_squeeze2_arm64` | **ADMITTED** | `EquivImplSpec.Sponge.Arm64.Driver.fst:111` |
| driver: keccak2 | `lemma_keccak2_arm64` | proven (composes squeeze2) | `EquivImplSpec.Sponge.Arm64.Driver.fst:132` |
| top-level: sha224..shake256 | `lemma_{sha224..shake256}_arm64` | proven (transitively trusts squeeze2) | `EquivImplSpec.Sponge.Arm64.API.fst:53-119` |

### Slow / flaky (Arm64)

- **`arm64_sc_load_block` / `arm64_sc_load_last` / `arm64_sc_store_block`** — all at `--z3rlimit 200`. No data from this run because cached, but earlier profile runs were modest; not on the perf hot-list. Watch them after the squeeze2 closure changes the call graph. — `EquivImplSpec.Sponge.Arm64.fst:149/385/486`

- **`lemma_load_last_buffer_eq_padded_arm64`** at `--z3rlimit 400` — `EquivImplSpec.Sponge.Arm64.fst:291`

---

## Avx2 (N=4)

### Admits

- **`lemma_squeeze4_avx2`** — `assume val` driver-level lemma, mirror of `lemma_squeeze2_arm64`. Comment in Avx2.API.fst (line 23) cites `avx2_sc_store_block` admit as a precondition admit, but the actual Sponge.Avx2.fst shows 0 admits there — comment is **stale**. The actual block here is the same per-lane Steps lemma technique as Arm64. **Fix path**: port `BRIEF_squeeze_steps.md` from Arm64 (rename, lift `l < 2` → `l < 4`, mirror Avx2.Steps.fst:243-style). ~1 sprint after Arm64 closure. — `EquivImplSpec.Sponge.Avx2.API.fst:87`

- **`Libcrux_sha3.Simd.Avx2.Store.store_block` body** — `let _:Prims.unit = admit () in` directly before the `fold_range`. Set by Rust `hax_lib::fstar!("admit()")` at `src/simd/avx2/store.rs:74` (relocated from the pre-`f9e915bd8` monolithic `src/simd/avx2.rs:490`; module split commit landed on 2026-05-06). Mirror of (now-discharged) Arm64 `store_block` at N=4 with 8 forall conjuncts (vs 4) in invariant + 12 live arrays (vs 6) in VC. **Per HANDOFF**: AVX2 store_block is harder than Arm64 — needs either (a) `store_block_chunk` opaque helper + `--split_queries always`, (b) recursive predicate over per-lane forall, or (c) hand-prove via `assume val` interface + .fst body. **2026-05-05 update**: the AVX2 `load_block` cascade closure (commits `7bb581f8b`, `8203c9ace`, `28db4222a`, `3b9fc054c`) supplies a working template — the `--using_facts_from '... -array_from_fn -rem_euclid'` filter and the SMTPat extensionality-lemma pattern are likely directly reusable. **2026-05-23 update**: Arm64 `store_block` was discharged via the loop-body / `_full`/`_tail` split pattern (commits `c14f94d2c` / `29424f593` / `83d1a04c2`); that pattern is the recommended starting template now. — `Libcrux_sha3.Simd.Avx2.Store.fst:165` (Rust source: `crates/algorithms/sha3/src/simd/avx2/store.rs:74`)

- **`EquivImplSpec.Sponge.Avx2.lemma_load_block_eq_xor_block_into_state_avx2` `byte_eq` inner admit** (session 2026-05-23) — `admit ();` at start of inner `byte_eq` lemma body. Exact AVX2 mirror of the Arm64 admit at item 4 in the TL;DR. See §"2026-05-23 STATUS DELTA" and §"Suggested sprint order" item 4 for fix path. — `EquivImplSpec.Sponge.Avx2.fst:125`

### Coverage map (per phase)

| Phase | Lemma | Status | File:line |
|---|---|---|---|
| load_block (sc) | `avx2_sc_load_block` | proven | `EquivImplSpec.Sponge.Avx2.fst:149` |
| load_last (sc) | `avx2_sc_load_last` | proven | `EquivImplSpec.Sponge.Avx2.fst:330` |
| store_block (sc) | `avx2_sc_store_block` | proven (in equivalence file) | `EquivImplSpec.Sponge.Avx2.fst:426` |
| store_block (extraction body) | n/a — extraction body admitted | **ADMITTED** | `Libcrux_sha3.Simd.Avx2.Store.fst:165` |
| absorb_block | `lemma_absorb_block_avx2` | proven | `EquivImplSpec.Sponge.Avx2.Steps.fst:54` |
| absorb_final | `lemma_absorb_last_avx2` | proven | `EquivImplSpec.Sponge.Avx2.Steps.fst:92` |
| squeeze_block | `lemma_squeeze_block_avx2` | proven | `EquivImplSpec.Sponge.Avx2.Steps.fst:131` |
| squeeze_last | `lemma_squeeze_last_avx2` | proven | `EquivImplSpec.Sponge.Avx2.Steps.fst:171` |
| squeeze_one_step | **MISSING** (gap vs Arm64) | not present | `EquivImplSpec.Sponge.Avx2.Steps.fst` (no analogue of `lemma_squeeze_one_step_arm64`) |
| driver: absorb4 | `lemma_absorb4_avx2` | proven | `EquivImplSpec.Sponge.Avx2.API.fst:59` |
| driver: squeeze4 | `lemma_squeeze4_avx2` | **ADMITTED** | `EquivImplSpec.Sponge.Avx2.API.fst:87` |
| driver: keccak4 | `lemma_keccak4_avx2` | proven (composes squeeze4) | `EquivImplSpec.Sponge.Avx2.API.fst:124` |
| top-level: shake256_x4 | `lemma_shake256_x4_avx2` | proven (transitively trusts squeeze4) | `EquivImplSpec.Sponge.Avx2.API.fst:162` |
| top-level: sha224..sha512, shake128 | **MISSING** | n/a — Avx2.X4 only exposes shake256 currently | `EquivImplSpec.Sponge.Avx2.API.fst` |

### Slow / flaky (Avx2)

- ~~`Libcrux_sha3.Simd.Avx2.load_u64x4x4` query 314 12.2 s~~ — **CLEARED** (2026-05-05). Today's cascade closure brings load_u64x4x4 max sub-query under 1 s. q306/q310/q314 specifically all dropped from 1-12 s to <50 ms.
- ~~`Libcrux_sha3.Simd.Avx2.load_u64x4` query 1~~ — **CLEARED** alongside load_u64x4x4.
- ~~`Libcrux_sha3.Simd.Avx2.load_block` 700+ split sub-queries~~ — **CLEARED**. After today's fix-stack, load_block has 1361 sub-queries with max 1.5 s (warm hint) / 41 s (cold start). The 700→1361 sub-query growth is from the added per-iteration `hax_lib::fstar!(reveal_opaque)` bodies in load_u64x4x4 / load_u64x4 — they're individually fast. No z3-RPC handle-leak symptoms observed.

- **`avx2_sc_load_block` / `avx2_sc_load_last` / `avx2_sc_store_block`** — all at `--z3rlimit 200`. — `EquivImplSpec.Sponge.Avx2.fst:148/329/425`
- **`lemma_load_last_eq_xor_block_into_state_avx2`** at `--z3rlimit 600` — uses Arm64 `lemma_load_last_buffer_eq_padded_arm64` 4×. Slow elevated rlimit suggests this is near the budget; watch in next build. — `EquivImplSpec.Sponge.Avx2.fst:187`
- **`lemma_sq_lane_avx2_eq_squeeze_state`** at `--z3rlimit 600` — `EquivImplSpec.Sponge.Avx2.fst:375`

---

## Cross-cutting / Generic

### Admits

- **`EquivImplSpec.Keccakf.Generic.lemma_theta_rho_to_spec`** — `--admit_smt_queries true`. Cross-cutting because all three keccakf1600 backends (Portable / Arm64 / Avx2) call this through `lemma_keccakf1600_to_spec`. See Portable section above for fix sketch (5 row-helpers). — `EquivImplSpec.Keccakf.Generic.fst:1309`

### Stale comments (not real admits)

- `EquivImplSpec.Keccakf.Avx2.fst:99` — comment refers to a `[assume val]` for `Core_models.Num.fst:493`. That's an upstream `rotate_left` axiom (intrinsics layer), not a sha3-layer admit. **Note**: comment incorrectly says `lemma_shl_xor_shr_is_rotate_left` is admitted; it has been proven (per HANDOFF 2026-04-26).
- `Libcrux_sha3.Generic_keccak.Simd256.fst:281` — comment string `"remains an assume val"` references the actual `lemma_squeeze4_avx2` admit; not a separate admit.

### Spec-side admits (specs/sha3, NOT sha3 crate)

- ~~`Hacspec_sha3.fst:6,13` — `assume val createi` and `assume val createi_lemma`.~~ **ELIMINATED 2026-05-04**: replaced with `let createi v_N f = Rust_primitives.Arrays.createi v_N f` and `let createi_lemma ... = ()` (post-condition of `Rust_primitives.Arrays.createi` discharges the SMTPat lemma). No remaining spec-side axioms in `specs/sha3`.

### Makefile-level

- **No** `ADMIT_MODULES` or `SLOW_MODULES` set in any of:
  - `crates/algorithms/sha3/proofs/fstar/equivalence/Makefile`
  - `crates/algorithms/sha3/proofs/fstar/extraction/Makefile`
  - `specs/sha3/proofs/fstar/extraction/Makefile`
  - `crates/algorithms/sha3/proofs/fstar/Makefile` (top-level driver)

---

## Top slow-query list (combined across this audit's runs)

Threshold: >2 s wall **or** >50% of rlimit used. Smaller queries omitted.

**2026-05-05 update**: the top 9 entries below (all AVX2) are obsolete — cleared by the cascade closure. Retained for archival reference.

| wall (ms) | used rlimit | rlimit | function | notes |
|----:|----:|----:|---|---|
| ~~**12,242**~~ | ~~80.5~~ | ~~400~~ | ~~`Libcrux_sha3.Simd.Avx2.load_u64x4x4, q314`~~ | **CLEARED 2026-05-05** |
| ~~1,398~~ | ~~12.3~~ | ~~400~~ | ~~`Libcrux_sha3.Simd.Avx2.load_u64x4x4, q310`~~ | **CLEARED 2026-05-05** |
| ~~1,073~~ | ~~10.3~~ | ~~400~~ | ~~`Libcrux_sha3.Simd.Avx2.load_u64x4x4, q306`~~ | **CLEARED 2026-05-05** |
| ~~539~~ | ~~3.1~~ | ~~80~~ | ~~`Libcrux_sha3.Simd.Avx2.load_u64x4, q1`~~ | **CLEARED 2026-05-05** |
| ~~256~~ | ~~2.8~~ | ~~500~~ | ~~`Libcrux_sha3.Simd.Avx2.load_block, q180`~~ | **CLEARED 2026-05-05** (now ~80 ms) |
| ~~243~~ | ~~2.6~~ | ~~500~~ | ~~`Libcrux_sha3.Simd.Avx2.load_block, q182`~~ | **CLEARED 2026-05-05** |
| ~~162~~ | ~~0.5~~ | ~~500~~ | ~~`Libcrux_sha3.Simd.Avx2.load_block, q596`~~ | **CLEARED 2026-05-05** |
| ~~129~~ | ~~0.6~~ | ~~500~~ | ~~`Libcrux_sha3.Simd.Avx2.load_block, q645`~~ | **CLEARED 2026-05-05** |
| ~~113~~ | ~~0.6~~ | ~~500~~ | ~~`Libcrux_sha3.Simd.Avx2.load_block, q708`~~ | **CLEARED 2026-05-05** |
| 118 | 0.15 | 400 | `EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_one_step_portable, q33` | Portable |
| 112 | 0.34 | 200 | `EquivImplSpec.Sponge.Portable.lemma_load_block_eq_xor_block_into_state, q1` | Portable |
| 112 | 0.22 | 100 | `EquivImplSpec.Sponge.Portable.portable_sc_load_last, q1` | Portable |
| 103 | 0.31 | 200 | `EquivImplSpec.Sponge.Portable.lemma_load_last_equals_load_block_on_padded, q96` | Portable |
| 102 | 0.10 | 150 | `EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_last_portable, q1` | Portable |
| 100 | 0.11 | 100 | `EquivImplSpec.Sponge.Portable.portable_sc_store_block, q1` | Portable |
| 96 | 0.10 | 200 | `EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable, q1` | Portable |
| 96 | 0.17 | 400 | `EquivImplSpec.Sponge.Portable.lemma_load_last_eq_xor_block_into_state_padded, q1` | Portable |
| 94 | 0.10 | 300 | `EquivImplSpec.Sponge.Portable.lemma_store_block_eq_squeeze_state, q1` | Portable |
| 89 | 0.09 | 200 | `EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_last_portable, q1` | Portable |
| — | — | 400/400 (CANCEL) | `Libcrux_sha3.Generic_keccak.Portable.squeeze` q224, q231, q263, q280 | Portable; admitted via `--admit_smt_queries true`; cite: `proofs/squeeze-cascade-profile.md` |

### Hint-replay flakes (success-after-retry)

These succeed on `--query_stats` but fail to replay from the hint file. Each retry costs ~80 ms wall + a `Warning 252 Hint-replay failed`. Regenerate hints or fix the int/usize coercion that confuses the replayer.

- `EquivImplSpec.Sponge.Portable.lemma_load_last_equals_load_block_on_padded` queries 33, 45, 78, 83 — "incomplete quantifiers" + "Subtyping check failed: Expected `range_t USIZE`, got `Prims.int`". `EquivImplSpec.Sponge.Portable.fst:179-240` (warnings cite lines 196 and 205)

---

## Suggested sprint order (high → low impact, low → high effort first)

1. **Closure: `lemma_squeeze2_arm64`** — the building-block `lemma_squeeze_one_step_arm64` already exists and is proven. Mechanical replacement of `assume val` → `let` body that uses it 2× per iteration. Closes 1 of the 5 load-bearing admits. Reference: `BRIEF_squeeze_steps.md`. **Effort**: ~1 session.
2. **Port the Steps `squeeze_one_step` lemma to Avx2** — `EquivImplSpec.Sponge.Avx2.Steps.fst` is missing the analogue of `lemma_squeeze_one_step_arm64`. Add it, then close `lemma_squeeze4_avx2` the same way as Arm64. **Effort**: ~1 sprint (the Z3 budget at N=4 is uglier).
3. **Discharge `Simd.Avx2.store_block` admit** — N=4 is harder; needs `[@@ "opaque_to_smt"]` chunk helper or `assume val + .fst body` workaround. Reference: HANDOFF.md "AVX2 load_block — committed but NOT verifying" section. The Arm64 store_block discharge sequence (commits `c14f94d2c`/`29424f593`/`83d1a04c2`) supplies the structural template (loop-body wrapper + `_full`/`_tail` split). **Effort**: 1-2 sprints, may hit z3-RPC handle leak.
4. **Profile and fix the `byte_eq` cliff (session 2026-05-23 admits 4 & 5)** — qi.profile the failing query in `lemma_load_block_eq_xor_block_into_state_{arm64,avx2}` per `~/.claude/skills/fstar-for-libcrux/SKILL.md §1.5`. The 2026-05-05 cascade-closure fix on `Hacspec_sha3.createi` is upstream of this; the cliff is at the *consumer side* (`extract_lane (load_block …)` and per-byte `Seq.index` reasoning) rather than `load_block`'s body. Candidate root-cause shapes to investigate in order: (a) opacity on `KA.{arm64,avx2}_lane`, (b) opacity on `Hacspec_sha3.Sponge.xor_block_into_state`, (c) tightening `lemma_subslice_bytes_eq`'s SMTPat trigger. Closes 2 of the 5 admits. **Effort**: ~1 session for profile + 1 sprint for fix.
5. **Factor `SqueezeBytes.lemma_squeeze_step_byte_write`** — currently passing only at `--z3rlimit 800 --split_queries always` (over skill cap). Factor the proof body so subtyping check on `(i *! rate)` discharges at default budget. Likely candidate: extract the bound-derivation into a top-level lemma so the function-level VC doesn't carry the full `output_initial`/`outlen` context. **Effort**: 1-2 sessions.
6. **Regenerate hints for `Sponge.Portable.fst`** — fix the 4 `lemma_load_last_equals_load_block_on_padded` hint-replay failures. **Effort**: 1 session.
7. **Watch `EquivImplSpec.Sponge.Portable.API.lemma_squeeze_portable`** — currently passing only at `--z3rlimit 800`; cascade profile shows 4 sub-queries cancel at 400. Confirm whether this is still load-bearing now that the Portable.squeeze and `lemma_theta_rho_to_spec` `--admit_smt_queries` were removed; if so, factor the post-loop branch.
8. **Upstream-report the F\* nightly-2026-04-12 regressions** — `FStar.Tactics.V1` namespace removal + the two `assert_norm (List.Tot.length …)` failures. The original "hax-0.3.7-regression-bug-report.md" referenced by RESUME-PROMPT-2026-05-23.md was never committed; recreate from the toolchain-pin notes above. **Effort**: 1 session (after the next nightly stabilises).

---

## Parallel-branch inventory (2026-05-23 audit)

This repo has 14 sibling SHA-3 worktrees in `~/libcrux-sha3-*` and `~/libcrux-{loop-inv-opacify,avx2-store-arm64-transplant}`. Each was a per-sprint branch. Audited 2026-05-23 against the current `sha3-proofs-focused` HEAD (`3b107debb`).

### Active (unique WIP — pick these up when working their target admit)

- **`store-block-avx2-discharge`** (worktree `~/libcrux-sha3-store-avx2-discharge`, HEAD `464a9914a`, 3 commits ahead): The most-complete AVX2 store_block scaffolding. Adds `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` (axiom bridges + per-iter wrapper `store_u64x4x4` + tail wrappers + structural split into `store_block_full` / `store_block_tail` + composer). **Does NOT discharge the admit** — `store_block_full`/`_tail` carry `--admit_smt_queries true`. The 4-buffer 16-conjunct loop_invariant + `s.[(j-start)/8]` indexing cliff was profiled (qi.profile shows `k!61` HasTypeFuel ~1.1M instances) but not closed. **Use when:** picking up §"Suggested sprint order" item 3.

- **`loop-invariant-opacify`** (worktree `~/libcrux-loop-inv-opacify`, HEAD `c62edb033`, 3 commits ahead): Contains `Libcrux_sha3.Simd.Avx2.LoopInv.fst` (170 lines) — defines `byte_inv_full` opaque per-lane byte invariant + `_init`/`_step`/`_after_loop` lemmas, the predicate stack designed to support the AVX2 store_block_full/_tail loop_invariant opacification. Stage A (Arm64 load opacification — `e15202b19`) is already on the current branch via `bb604df94`. Stage B (using LoopInv.fst to actually close the AVX2 store_block admit) was blocked on a pre-existing hax-checkout include drift (`ad110bf` vs `952bee0` Rust_primitives.Integers.fsti) that prevents `Libcrux_intrinsics.Avx2_extract.fsti` from typechecking. **Use when:** picking up §"Suggested sprint order" item 3, after item 0 below (resolve the include-drift blocker) is sorted.

- **`sha3-byteform-migration-squeeze2`** (worktree `~/libcrux-sha3-squeeze2`, HEAD `7979e4371`, 2 commits ahead): Factors squeeze2 multi-block loop into a `squeeze_blocks2` helper. Relevant to §"Suggested sprint order" item 1 (closure of `lemma_squeeze2_arm64`) — the helper provides a finer-grained step than `lemma_squeeze_one_step_arm64` alone. **Use when:** picking up §"Suggested sprint order" item 1.

### Superseded (safe to GC after follow-up confirms)

- **`store-block-arm64-lemma-bridge`** (HEAD `5bacec6b7`, 4 commits ahead): Arm64 store_block split + lemma-bridge approach. Arm64 store_block is now discharged via a different (loop-body wrapper + `_full`/`_tail` split) approach landed in commits `c14f94d2c`/`29424f593`/`83d1a04c2`. The lemma-bridge work is obsolete.
- **`store-block-arm64`** (HEAD `c20b0956c`, 1 commit ahead): Earlier "wrapper + filter, body-admit retained" approach to Arm64 store_block. Superseded by the discharge sequence above.
- **`store-block-arm64-profile-first`** (HEAD `c6e4848c5`, 2 commits ahead): Arm64 store_block profile baselines, no fix. Diagnostic only.
- **`store-block-avx2`** (HEAD `692f3f2b0`, 3 commits ahead): Predecessor of `store-block-avx2-discharge`. Earlier iteration of the same structural work (wrapper extraction, store_u64x4x4 opacification). Superseded.
- **`avx2-store-arm64-transplant`** (HEAD `a7084c916`, 1 commit ahead): Same shape as `95ca5782c` on `loop-invariant-opacify` ("AVX2 store_block structural split + 3 wrappers + helpers"). Probably superseded by `4bbe9b667` on `store-block-avx2-discharge`.
- **`store-block-arm64-helpers`** (HEAD `8c0202a4b`, **0 commits ahead**): Already fully merged. Safe to GC.

### Upstream-prep (not novel work)

- **`sha3-keccakf-upstream`** (HEAD `b3caa47cd`, 84 commits ahead, 11 of those proof-touching): Upstream-clean rebases of work already in the current branch under different SHAs (verified `7b420e1a9 specs/sha3: mark createi opaque_to_smt` matches the 2026-05-05 createi opacification already landed). Awaiting PR submission.
- **`sha3-spec-upstream`** (HEAD `d8f7ed529`, 75 commits ahead, 4 of those proof-touching): Same shape — upstream-clean rebases. Awaiting PR submission.

### Open prerequisite (gating item 3 closure)

- **0. Resolve the hax-checkout include drift** that blocks `Libcrux_intrinsics.Avx2_extract.fsti` from typechecking against the `LoopInv.fst` predicate stack. Was the original Stage B blocker on `loop-invariant-opacify`. Likely requires either bumping Cargo.lock to match `loop-invariant-opacify`'s base commit `95ca5782c`'s hax checkout, or pinning hax to `ad110bf` for the duration of the AVX2 store_block work. **Effort**: 1 session to confirm scope.

---

## Files referenced

- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst:101`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.API.fst:87`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Keccakf.Generic.fst:1309`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst:243` (working `lemma_squeeze_one_step_arm64`)
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.Steps.fst` (gap: no `lemma_squeeze_one_step_avx2`)
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/BRIEF_squeeze_steps.md`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/BRIEF_load_store_block.md`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence/HANDOFF.md`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/squeeze-cascade-profile.md`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/agent-status/portable-perf-profile.md`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Portable.fst:482`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.fst:913`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.fst:1930`
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/src/simd/arm64.rs:255` (Rust-side `hax_lib::fstar!("admit()")`)
- `/Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/src/simd/avx2.rs:420` (Rust-side `hax_lib::fstar!("admit()")`)
