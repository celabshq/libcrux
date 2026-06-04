# Resume prompt — SHA-3 proofs, post-revert state (2026-05-23)

## Where we are

The previous session attempted to upgrade SHA-3 proofs to hax 0.3.7 +
F\* `nightly-2026-04-12`. The upgrade surfaced a Z3 e-matching cliff
on `fold_range`'s refinement-typed return at consumer sites; many
follow-on cascade failures were chased with admits/fuel/rlimit bumps
before the user called the scope creep and reverted.

**The branch is now at commit `3b107debb`** ("agent-sha3: fix rem8
trailing-block indexing in avx2/store.rs::store_block"), which is
`bfc37801d` (session start) + only the rem8 bug fix. Everything else
this session attempted is gone. The tag
`session-2026-05-22-cascade-attempt` preserves the in-progress state
if you want to inspect what we tried.

## What is reverted (all of these are gone)

- Cargo.toml hax pin (back to `branch = "integer-lemmas"`).
- `~/hax` checkout back to `integer-lemmas` branch (commit `952bee04`).
- Opam switch `hax` rebuilt against integer-lemmas-branch content (via
  `~/hax/setup-local.sh`).
- All session-added admits in source + equivalence proofs.
- All session-added fuel/rlimit options.
- `--z3rlimit_factor` in Makefiles.
- `_keccak_state_impl_opts` / `_incremental_impl_opts` workaround stubs.
- Stubs/Spec.Utils.fsti, BitVec.Utils.fst, Tactics.Utils.fst V1→V2
  edits.
- Cargo lockfile pin to 0.3.7.

## What remains

- `crates/algorithms/sha3/src/simd/avx2/store.rs` rem8 trailing-block
  fix (the only kept change; reachable only under
  `chunks > 0 && rem % 8 != 0` which doesn't occur in current callers
  but isn't precluded by the precondition).
- Untracked files in worktree: editor backups (`*.~`), agent-prompt-*.md,
  the agent-status notes from this session. Up to you whether to
  garbage-collect.

**Important untracked diagnostic notes to keep**:
- `proofs/agent-status/hax-0.3.7-regression-bug-report.md` — full
  diagnostic chain of the e-matching cliff (proof attempts,
  qi.profile snapshots, F\* version + Core_models bisection
  results — all negative).
- `proofs/agent-status/sha3-toolchain-upgrade-2026-05-2{1,2}.md` —
  per-day session logs.
- `proofs/agent-status/CONTINUATION.md` — earlier mid-session note;
  partly superseded by this one.

## Diagnostic findings worth carrying forward

Don't waste time re-running these — we already did them and they were
negative.

1. **The cascade is at `fold_range`'s refinement-typed return.** Z3
   cannot extract any conjunct (even `len out = len old_out`) from
   the refinement at consumer sites under hax 0.3.7 + F\* nightly.
   Cached hints from sibling worktrees were masking this. None of
   the proof-side fixes worked:
   - Bridge lemma (`requires == ensures`): precondition won't
     discharge from the refinement at the call site (same as the
     function post's failure).
   - Bare named predicate (regular `let`): loop body can't construct
     atom, consumer can't deconstruct atom.
   - Named predicate + intro/elim lemmas with SMTPat: intro
     precondition won't discharge (Z3 can't bridge
     `lemma_index_update_at_range`'s `forall (j:nat). Seq.index` to
     the predicate's `forall (j:usize). out.[j]`).
2. **F\* version is NOT the cause.** v2026.03.24 produces the same
   cascade family with different dominant Tm_arrow hashes; v2026.05.17
   broke for unrelated reasons (FStar.Mul removed).
3. **`unfold` removal on `t_FnOnce` arrow instances in `Core_models.Ops.Function.fst`
   between 0.3.6 and 0.3.7 is NOT the cause.** Patched it locally;
   qi.profile bit-identical with the patch.
4. **`Core_models.fst` include restructure is NOT the proximate cause.**
   Substituting 0.3.6's includes + Iter + Ops files via stubs/ left
   the failure unchanged.
5. **`--ext context_pruning=false` doesn't change the cascade.**
6. **`--z3rlimit_factor N` (tested up to 16): doesn't help the
   structural failures** (Z3 uses <1% of rlimit, indicating
   e-matching cliff). DOES help budget-bound failures in equivalence
   proofs (those used full rlimit).

## Toolchain state to verify

Once the revert is settled, confirm:

```bash
cd ~/hax && git rev-parse --abbrev-ref HEAD          # integer-lemmas
opam list --switch=hax hax-engine                    # 0.3.6 (pinned at ~/hax/engine)
/Users/karthik/.opam/hax/bin/cargo-hax --version     # integer-lemmas-untagged-...
cd /Users/karthik/libcrux-sha3-proofs && git log --oneline -1   # 3b107debb
make -C crates/algorithms/sha3/proofs/fstar          # baseline verifies cleanly
```

If `make` shows the same 2-failure state as before any of this
session's work (and no new ones), the revert is correct.

## Useful artifacts left on disk

- `/Users/karthik/.local/fstar-2026.03.24/` — F\* v2026.03.24 release
  binary (in case you want to test other branches under it).
- `/Users/karthik/.local/fstar-2026.05.17/` — F\* v2026.05.17 release
  (broken for libcrux because FStar.Mul removed; useful for other
  testing).
- `/Users/karthik/hax/setup-local.sh` and `/Users/karthik/hax-evit/setup-local.sh`
  — per-switch installers; route `cargo-hax` + companion binaries
  into the opam switch's `~/.opam/<switch>/bin/` so they stay
  matched with `hax-engine`. Use these instead of `~/cargo/bin/`
  installs going forward.
- opam switch `hax-evit` (parallel session's binary set, unchanged).
- Tag `session-2026-05-22-cascade-attempt` in libcrux-sha3-proofs
  for archaeological access to all the attempts.

## What to do next session

The fundamental finding: **hax 0.3.7 + F\* nightly-2026-04-12 hits an
e-matching cliff on `fold_range` refinement extraction**, and no
proof-side hack we tried fixes it. Cached hints from prior toolchain
state were masking it.

Options when picking this up:

1. **Stay on hax 0.3.6 (integer-lemmas)** indefinitely. SHA-3 verifies
   cleanly. Upgrade only when upstream hax fixes the regression.
   This is the current state.
2. **Upgrade attempt #2 with a focused fix**: file the bug report at
   `proofs/agent-status/hax-0.3.7-regression-bug-report.md` upstream,
   wait for upstream resolution, then retry.
3. **Hand-write a `lemma_fold_range_decode`** in `Rust_primitives.Hax.Folds`
   that takes the result and an instance of the loop invariant
   explicitly as arguments and returns a `Lemma` exposing it on a
   fresh binder. This was suggested but never tested — the diagnostic
   agent thought it should work but the implementer agent's bridge
   experiment (which is morally equivalent) failed at the same step.

## Hard-won lessons (for the agent picking this up)

- **Cached hints mask cliffs.** Don't trust a green build until you've
  verified with no `.hints` files in cache.
- **`qi.profile` is misleading.** A 30,000-count quantifier in the
  profile may be noise from typeclass interpretation axioms that
  the goal doesn't depend on. Look at the actual SMT goal, not
  just the top instantiations.
- **Don't chase cascades.** If your fix unblocks one failure and
  reveals another with the same fingerprint, you're not fixing the
  root cause; you're just rotating where it surfaces. Stop and
  surface to the user.
- **"add admit" requests scope.** "Add body admits in the two failing
  functions" is precise and finite. The agent (me) interpreted this
  as a license to chase every downstream failure with more admits —
  scope creep that resulted in 8+ files modified before user pushback.
  Future agents: don't extend admit-mode beyond the literal request.
- **Per-switch opam installs prevent cross-version contamination.**
  `~/.cargo/bin/cargo-hax` is a global landmine when multiple hax
  versions are in active development simultaneously. Use opam
  switches.
