# Stream status — arm64 squeeze2 driver, Path A (opaque squeezed_upto)

**Date:** 2026-05-30  **Goal:** close the admit at `src/generic_keccak/simd128.rs:120`.

## Decision: Path A (in-body loop invariant), confirmed cycle-safe
- `absorb2` in the SAME module (`simd128.rs:86`) already cites
  `EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_last_arm64` → Steps does NOT
  depend on Simd128, so citing Steps from squeeze2 does not cycle.
- Portable `squeeze` (portable.rs:356) is the live template: it carries the
  per-byte prefix forall **inline** in the loop invariant at N=1.
- N=2 differentiator: two-lane × all-bytes forall saturates Z3 → wrap the
  forall in an opaque `squeezed_upto` predicate so loop-step VC sees only
  the opaque atom (memory `feedback_opaque_predicate_store_proof`).

## Plan
1. Helper module `EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst` (manual, untracked):
   - `squeezed_upto (out spec_out: Seq.seq u8) (hi:nat) : prop` (opaque), param
     over the COMPUTED spec array (sidesteps `squeeze`'s Pure precondition).
   - `lemma_squeezed_upto_full` (close to full Seq equality).
   - `lemma_squeeze_first_step_arm64` (offset 0, len rate, NO keccakf).
   - `lemma_squeeze_mid_step_arm64`  (restated `lemma_squeeze_one_step_arm64`).
   - `lemma_squeeze_tail_step_arm64` (keccakf, partial trailing block).
   Each step lemma confines `reveal_opaque squeezed_upto` to its own body.
2. Strengthen driver squeeze2 loop invariant (two opaque squeezed_upto + per-lane
   iterate_keccak_f), backport to Rust `loop_invariant!`/`fstar!`.
3. Remove `--admit_smt_queries true`; full build; admit-count net −1; cargo test.

## Bridge facts established
- `sq_lane_arm64 rate state outputs start len l` ≡ project `f_squeeze2 rate {f_st=state}
  outputs[0] outputs[1] start len` to lane l (Arm64.fst:44). Driver bridges
  f_squeeze2→sq_lane by packaging outputs=[|out0;out1|].
- `arm64_sc_store_block` : sq_lane ≡ squeeze_state (no keccakf bridge).
- `lemma_squeeze_one_step_arm64` (Steps.fst:243): full per-iter engine, raw forall.
- `lemma_squeeze_last_arm64` (Steps.fst:179): partial trailing block, squeeze_state.
- spec `squeeze outlen st rate`: byte k = to_le_bytes((iterate_keccak_f (k/rate) st)[(k%rate)/8])[(k%rate)%8].
- `iterate_keccak_f 0 st = st` (fuel 1).

## Status: building helper module.

## UPDATE 1: helper module GREEN (9 lemmas)
EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst fully verifies (full typecheck, no admit):
squeezed_upto + lemma_squeezed_upto_full + first/mid/tail_step + lemma_sq_lane_is_f_squeeze2
bridge + first/mid/tail_driver wrappers.

Gotcha resolved (via localized assert-walk per user steer): a "Subtyping check failed:
Expected nat got int" at the wrapper ENSURES `((v i+1)*v rate)` was MISLEADING — the body
asserts of every ensures conjunct PASSED; only the `hi:nat` refinement of the squeezed_upto
arg failed (nonlinear nat*nat>=0 dropped by Z3 in the heavy ensures context). Fix: changed
`squeezed_upto ... (hi: nat)` -> `(hi: int)` (k<hi stays vacuous for hi<0). Structural fix,
no rlimit bump needed.

## NEXT: Rust driver squeeze2 (Path A in-body), extract, integrate build.

## UPDATE 2 (2026-05-31): helper DONE; driver integration cliff

### DONE & verified
- Helper module `EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst` — 9 lemmas, FULLY
  verified (the mathematical core): opaque `squeezed_upto` (hi:int) + `_full` +
  first/mid/tail `_step` + `f_squeeze2`↔`sq_lane` bridge + first/mid/tail `_driver`
  wrappers. This is the hard part and it's solid.
- Driver `squeeze2` split (Rust) into `squeeze2_blocks` (loop engine, opaque
  `squeezed_upto` ensures) + thin `squeeze2` (consumes it, does the `Seq`-equality
  close). Base case fixed: `first_driver` precond `RATE<=outlen` via `small_div`
  contrapositive (avoids the Z3 4.13.3 `lar_solver.cpp:1066` LP crash that
  `lemma_mul_succ_le` triggered).

### BLOCKER (cliff)
`squeeze2_blocks` sub-queries **41 & 42 saturate** (`used rlimit 400.000`, ~50-60s,
canceled) = the two-lane `squeezed_upto` POST. This is function-level WP composition
over `first_block → fold → tail → (blocks==0|else) merge`. NOT fixed by: opaque
predicate, the squeeze2/squeeze2_blocks split, or `--using_facts_from '*
-Hacspec_sha3.Sponge.squeeze'` (so it's not `squeeze` unfolding). Matches skill §7
"Sequential folds collapse the function-level WP — split functions".

### Remaining plan (next session)
1. Aggressive per-phase split: `squeeze2_first` (no fold) / `squeeze2_loop` (the one
   fold) / `squeeze2_tail` (no fold), each with `squeezed_upto`-based ensures, +
   `squeeze2` thin sequencer that closes. One fold per function = small WPs.
   OR move `blocks==0` out of `squeeze2_blocks` into `squeeze2` first (smaller change).
2. If still saturates: `smtprofiling` query 41 `.smt2` to pinpoint the cascade.

### Side issue (pre-existing, project-wide)
`Core_models.Num.fst.checked` has a digest mismatch → F* refuses to WRITE any
downstream `.checked` (Warning 247), incl. SqueezeDriver/Simd128. Verification still
runs; only caching is blocked. Needs a clean rebuild of that hax-lib dep.

## UPDATE 3 (2026-05-31): top-down + Portable reshape; down to 1 query, precise diagnosis

### Progress
- **Top-down validated**: `--admit_except ...squeeze2` → exit 0. The composer `squeeze2`
  verifies for real from `squeeze2_blocks`' opaque `squeezed_upto` ensures (closes per
  branch). Architecture is sound.
- **Reshaped to Portable's `squeeze`/`squeeze_blocks` shape** (Rust): `squeeze2_blocks`
  now requires `blocks>0`, does **first-block + loop ONLY** (no blocks==0, no tail),
  ensures per-lane `iterate_keccak_f (blocks-1)` state + `squeezed_upto` at `blocks*RATE`.
  `squeeze2` handles blocks==0 + the trailing block + closes within each branch.
  This cut saturation from **2 queries (q41/42) → 1 (q62)**.

### Remaining cliff (precise)
`squeeze2_blocks` POST: 2 of 4 conjuncts saturate (rlimit 400.000). Localized via 4
explicit asserts inserted before the return IN LOOP-FORM (`s_init_st`): **all 4 asserts
PASS**, but the function POST (parameter-`$s.st` form) still saturates. So the loop-form
facts are proven; the gap is the WP bridge loop-form→POST-form (`s_init_st ≡ $s.st`, the
function-entry let) over the transparent recursive `iterate_keccak_f` in the **state**
conjuncts. Excluding `squeeze` + `extract_lane` from facts did NOT help (confirming it's
the `iterate_keccak_f` state conjuncts, not the `squeezed_upto`/`squeeze` ones). This is
cascade pollution (skill §7): trivial-by-congruence facts saturate under the ambient WP.

### Concrete next step (clean handoff)
Standalone bridge lemma (skill §7 "iter-start snapshot + standalone bridge lemma"):
prove `s_init_st == s_param /\ <state+squeezed_upto in s_init_st> ==> <same in s_param>`
in CLEAN context (pure congruence, no ambient pollution), cite it at the end of
`squeeze2_blocks`. Alternatively exclude `iterate_keccak_f` in `squeeze2_blocks` + supply
`iterate_keccak_f 0 == id` for the base case via a lemma. NOTE: the 4 localizing asserts
currently live ONLY in the extracted `.fst` (scratch); Rust source has the clean reshape.

### Refined fix (free-params trick, skill §7 "leaf producers take SIMD words as free params")
Give `squeeze2_blocks` GHOST params `lane0 lane1: t_Array u64 25` and `spec0 spec1: Seq u8`
with `requires`: `extract_lane $s.st l == lane_l /\ spec_l == squeeze outlen lane_l rate`.
State its `ensures` PURELY in `lane*/spec*` terms (no `$s.st`, no `squeeze`, no
`extract_lane`): `extract_lane s_future l == iterate_keccak_f (blocks-1) lane_l /\
squeezed_upto out{l}_future spec_l (blocks*rate)`. The loop invariant likewise uses
`lane*/spec*`. Then the saturating `s_init_st≡$s.st` congruence over `iterate`/`squeeze`
moves to the CALLER's `requires`-discharge (direct equalities, cheap) instead of the
polluted function POST. `squeeze2` passes `lane_l = extract_lane s.st l`,
`spec_l = squeeze outlen lane_l rate` and discharges the links trivially.

## ============ RESUME HERE (machine restart 2026-05-31) ============

### State of source files (the source of truth — all SAVED on disk)
- `src/generic_keccak/simd128.rs`: squeeze2 SPLIT into `squeeze2_blocks` (first+loop,
  Portable shape) + `squeeze2` (blocks==0 + tail + per-branch close). Latest attempt:
  **both fns at `--fuel 0`** + `--using_facts_from '* -Hacspec_sha3.Sponge.squeeze
  -EquivImplSpec.Keccakf.Generic.extract_lane'`, and `squeeze2_blocks` base case cites
  `lemma_iterate_keccak_f_zero` ×2 lanes. THIS FUEL-0 ATTEMPT IS APPLIED BUT NOT YET
  TESTED (the cache rebuild was blocking when we stopped).
- `proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst`: 9 verified
  lemmas + NEW `lemma_iterate_keccak_f_zero` (fuel-1 proof of `iterate_keccak_f 0 == id`).
  Manual .fst (untracked). The 9 original lemmas were FULLY verified earlier; the new
  lemma is trivial.
- `proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Simd128.fst`: STALE — contains
  pre-fuel-0 structure + scratch localization asserts. **Will be regenerated by
  `./hax.sh extract`; ignore its current content.**

### Cache note
`Core_models.Num.fst.checked` was DELETED (it was digest-stale, blocking all downstream
`.checked` writes). A rebuild was in progress at restart. First build after restart will
rebuild stale deps incrementally (slow once, then warm). This was the fix for the
project-wide Warning 247 "checked file not written".

### Exact resume steps
1. `cd crates/algorithms/sha3 && ./hax.sh extract`  (regenerates Simd128.fst from Rust)
2. `fstar_build check/EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst`  (helper + new lemma;
   confirms .checked now WRITES post-cache-fix)
3. `fstar_build check/Libcrux_sha3.Generic_keccak.Simd128.fst`  (the driver)
4. Grep the build log for `Simd128.squeeze2_blocks` + `failed`:
   - If NO failed queries → **DONE** (admit at simd128.rs:120 closed; run cargo test; the
     `lemma_squeeze2_arm64` reader in EquivImplSpec.Sponge.Arm64.Driver.fst already
     consumes the driver ensures, so the whole arm64 chain closes).
   - If `squeeze2_blocks` query still saturates (`used rlimit 400.000`) → fuel-0 wasn't
     enough; fall back to the **opaque state predicate** (UPDATE 3 / "hide behind a
     predicate"): wrap `extract_lane … == iterate_keccak_f (blocks-1) …` in an opaque
     `state_advanced` predicate (mirror `squeezed_upto`), so the POST is all-opaque atoms.

### What's solid regardless
Helper module (the hard math) fully verified; top-down `--admit_except …squeeze2` = exit 0
(composer proven). Only the `squeeze2_blocks` loop-engine POST remains (1 saturating query
as of last full build; fuel-0 attempt targets it). NOTHING committed to git.

### CACHE-FIX UPDATE (post-restart-prep)
Deleting `Core_models.Num.fst.checked` was NOT enough: rebuild produced another 0-byte
`.checked` (exit 0, no failed modules) → the digest-stale dep is DEEPER in the hax-lib
proof-libs / core-models layer (something Core_models.Num imports is itself digest-stale).
This is a project-wide ENVIRONMENTAL issue, independent of the squeeze2 proof. KEY: builds
still **exit 0 with no Error 19** → verification runs correctly; only `.checked` WRITING is
blocked (Warning 247). So judge proof correctness by **exit code + absence of `Error 19`**,
NOT by whether `.checked` is non-empty. A clean fix for the cache (separate task) would be a
fuller rebuild of the hax-lib proof-libs `.checked` set, or matching the cached digests to
the current `952bee0` hax-lib checkout.

CONFIRMED at restart-prep: `fstar_build check/EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst`
→ exit 0, failed_modules=[] (helper module re-verifies; the 0-byte .checked is only the
caching issue above).
