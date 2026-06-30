# Proofs-branch cleanup and upstreaming plan

This file tracks the outstanding work on the `proofs` branch: completing the
remaining proofs, slicing them into upstream pull requests, and the
build-infrastructure cleanup. For the current verification status, see
[`PROOFS.md`](PROOFS.md) and each crate's `proofs/verification_status.md`.

Status is recorded with `[x]` (done), `[~]` (partial), `[ ]` (open).

## 1. Upstream slicing

The `proofs` branch assembles all three Rust-verified crates onto a shared
intrinsics + spec surface. Landing it upstream is a linear SHA-3 PR stack plus
per-crate spec reconciliation.

SHA-3 PR stack:
- [x] PR1 — hax-lib 0.3.7 + F\* upgrade (#1478)
- [x] PR2 — intrinsics + core-models SIMD models and differential tests (#1481)
- [~] PR3 — `specs/sha3` Hacspec spec and its own F\* verification (#1399); part
  of this has already landed on `main` (the `get_ij/set_ij` index fix and the
  `createi → array_from_fn` rename), so the step may collapse into a rebase.
- [ ] PR4 — SHA-3 Rust impl refactor (SIMD load/store/wrappers split). Keep
  `crates/algorithms/sha3/proofs` in place.
- [ ] PR5 — Keccak-f1600 portable proofs (#1408; currently parked).
- [ ] PR6 — sponge proofs, including AVX2/Neon squeeze and finalization.

Per-crate spec reconciliation to canonical upstream:
- [ ] sha3 spec → `main`
- [ ] ml-dsa spec → #1488
- [ ] ml-kem spec → #1480 (large)

## 2. Remaining proof obligations

- [x] SHA-3 — 0 lax / 0 unverified across all backends.
- [~] ML-KEM — SIMD backends 0 lax; `Generic` carries 3 lax / 3 unverified.
  Four non-axiom items remain:
  - [ ] `Sampling.fst` `sample_from_xof` (`--admit_smt_queries`; unbounded
    rejection loop, lax by design)
  - [ ] `Ind_cca.Incremental.Types.fst` — two panic-freedom `admit ()`
  - [ ] `Ind_cca.Incremental.fst` — one semantic `assume` (kp/sk equality;
    likely provable from the packed-keypair construction)
  - [ ] TRUST.md + hygiene pass for the assumed intrinsic axioms
- [~] ML-DSA — SIMD layer 0 lax; above-trait `Generic` carries 19 lax / 6
  unverified (panic-freedom is the target there).
  - [ ] Close / classify the remaining above-trait lax + unverified.
  - [ ] InvNTT `invert_ntt_at_layer_3_` tractability: abstract companion `.fsti`
    so clean builds do not cold-prove ~40 min (see
    `project_mldsa_avx2_ntt_companion_fsti`).
- [ ] KMAC — runtime safety only today; functional correctness against a spec is
  not attempted (would compose the SHA-3 spec). Decide whether to pursue.

## 3. Build-infrastructure consistency

- [ ] Per-crate F\* `--cache_dir` / `--hint_dir`. sha3 and ml-kem define
  different modules under the same name `Spec.Utils` but share one
  `.fstar-cache`; the two clobber one another's `Spec.Utils.fst.checked`, which
  blocks `.checked`/hint writes for whichever crate built second (the green
  `libcrux-sha3-proofs` worktree avoids this by using a separate tree). A
  per-crate cache directory removes the collision without a source rename.
- [ ] Reconcile the three extraction drivers: `crates/algorithms/sha3/hax.sh`
  and `libcrux-ml-dsa/hax.sh` (bash) vs `libcrux-ml-kem/hax.py` (python). They
  extract the shared `crates/utils/intrinsics` with different `-i` filters and
  `--interfaces` flags.
- [ ] Have each driver extract its own spec. `libcrux-ml-kem/hax.py` extracts
  `specs/ml-kem`; `sha3/hax.sh` and `ml-dsa/hax.sh` do not extract `specs/sha3`
  / `specs/ml-dsa`, so a clean `extract` does not reproduce the full proof
  closure.
- [ ] Apply the ignore-generated policy uniformly. ml-kem's spec F\* is
  untracked and regenerated (intended); `specs/sha3`'s spec F\* is committed
  (four tracked files), which is inconsistent.
- [ ] Resolve the `Libcrux_intrinsics.Arm64_extract.fst` ↔ `.fsti` mismatch (the
  `.fst` has more declarations than the interface; a latent intrinsics-hygiene
  item for the PR).

## 4. Branch hygiene

- [ ] Reconcile `proofs` with `main` (currently 42 behind / 60 ahead, diverged
  2026-06-16). Among verified crates only SHA-3 source changed on `main`
  (`get_ij/set_ij` fix, `createi → array_from_fn` rename, `sha3-spec-upstream`
  cross-spec tests); re-validate the SHA-3 proofs against main's source on
  reconciliation. ML-KEM / ML-DSA / intrinsics source are not affected by
  main-side drift.
- [x] `origin` URL updated `cryspen/libcrux` → `celabshq/libcrux`.
