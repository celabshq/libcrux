# USER-14 Step B — top-down closure (2026-05-31, session 2)

## Goal
Replace the `cross_vec_hyp` assume in `invert_ntt_at_layer_4_plus`
(`Libcrux_ml_kem.Invert_ntt.fst`) with real loop-invariant threading so the
strengthened post `to_spec_poly_mont re_future == ntt_inverse_layer (to_spec_poly_mont re) layer`
verifies with **NO assume**, then backport to `invert_ntt.rs` + Phase-8 gate.

## Approach taken — store_block top-down recipe (FULLY out-of-function maintenance)
Prior session diagnosis: the proof STRUCTURE was correct (8 helper lemmas + opaque
`cross_vec_done_at` all verified standalone) but the function-level VC was globally
unstable because the maintenance was done INLINE in the fold bodies (13.7 MB context,
9-min flaky → 28-min runaway). This session factors ALL maintenance OUT of the function:

### New opaque NAMED invariant predicates (so each fold carries ONE atom)
- `outer_inv re_init coeffs step_vec_n zs round step` `[@@opaque_to_smt]` — the outer
  fold's `forall i. if i>=threshold(round) then PENDING(==re_init,4*3328) else DONE(3328,cross_vec_done_at)`.
- `inner_inv re_init coeffs step_vec_n zs offset_vec step_vec j` `[@@opaque_to_smt]` — the
  inner fold's `forall i. if (i>=j && i<ofs+sv)||i>=j+sv then PENDING else DONE`.

### 5 standalone maintenance/init/bridge lemmas (verified in CLEAN context)
1. `lemma_outer_inv_init` — outer fold init at round=0 (threshold=0, all PENDING).
2. `lemma_inner_inv_init` — outer_inv(round) ⟹ inner_inv at j=offset_vec (inner disjunction
   collapses to i>=offset_vec == outer threshold).
3. `lemma_inner_step_maintains` — THE CORE: inner_inv(cb,j) + one butterfly step
   (cf=cb[j:=x][j+sv:=y], step facts) ⟹ inner_inv(cf,j+1). Per-i aux: {j,j+sv} become DONE
   via `lemma_step_keystone_loop`; all other i keep their branch (PENDING_j and PENDING_{j+1}
   differ exactly by {j,j+sv}) and content via Seq.upd frame + `lemma_cvda_frame1`.
4. `lemma_inner_to_outer` — inner_inv at j=offset_vec+step_vec ⟹ outer_inv(round+1)
   (threshold(round+1)==offset_vec+2*step_vec via `lemma_offset_vec`).
5. `lemma_postloop_cross_vec` — outer_inv(round=groups) (threshold==16, all DONE) ⟹
   `forall m l. cross_vec_hyp re_init coeffs step_vec_n zs m l` (the function post input).

### Function rewrite
Fold invariants now call `outer_inv`/`inner_inv` (one opaque atom each). Inner-fold body
maintenance = ONE `lemma_inner_step_maintains` call (+ the existing `lemma_offset_vec` &
`init_index_` to supply offset_vec/zs facts). Init/bridge via lemmas above. Epilogue
`lemma_postloop_cross_vec` replaces the inline aux2; existing
`lemma_layer_4_plus_post_from_cross_vec` discharges the post.

### Supporting edits
- `lemma_layer_numeric_facts`: added ensures `2*groups*(step/16)==16` (per-layer assert_norm)
  — needed by `lemma_postloop_cross_vec` for threshold(groups)==16.
- `lemma_cvda_frame1`: added `m < 16` to requires (was missing — the `Seq.index cout1 m` WF
  VC + the Bridges `lemma_cross_vec_frame` call both need it; only passed before by Z3 luck
  / lax-prelude. Surfaced by the from-scratch verify).

## Verification status — ✅ CLOSED IN .fst (build fa852745, exit 0, ~9.9 min wall)
- Full module `fstar_build check/Libcrux_ml_kem.Invert_ntt.fst` (NO --admit_except): **GREEN**.
- Phase-8 regression gate: `[@@ admitted]` = 0; `assume` = **0** (cross_vec_hyp assume ELIMINATED —
  this was the USER-14 Step B goal); only `admit ()` left = the pre-existing panic-freedom admit
  in `invert_ntt_montgomery` (USER-15, NOT this task). No stray probes.
- Verified artifact saved: `agent-status/Invert_ntt.fst.stepB-topdown-VERIFIED` (+ .fsti).

### Extra fixes the from-scratch build surfaced (all real bugs hidden before by lax/Z3-luck)
1. `lemma_cvda_frame1` requires was missing `m < 16` (needed for both the `Seq.index cout1 m` WF
   and the Bridges `lemma_cross_vec_frame` call). Added.
2. **Opaque predicates must NOT do usize arithmetic on free params** — `outer_inv`'s `round *! step`
   and `inner_inv`'s `offset_vec +! step_vec` / `j +! step_vec` generated unprovable no-overflow
   WF VCs (`v round * v step < 2^64` for arbitrary round/step). Reformulated ALL index/threshold
   arithmetic in both predicates AND the lemma `requires` to **`v`-level (total int) arithmetic**.
   `lemma_inner_to_outer` additionally takes `rnext`/`jend` params (the `round+!1` / `offset_vec+!step_vec`
   are computed in the bounded function context and passed in) to avoid free-param `+!` overflow.
3. **Opaque-predicate forall instantiation**: a bare `assert (Seq.index coeffs (v j) == ..)` after
   `reveal_opaque inner_inv` does NOT carry the forall's auto-selected `coeffs.[i]` trigger →
   incomplete quantifiers. Added `lemma_outer_inv_lookup` / `lemma_inner_inv_lookup` whose ENSURES
   contains the trigger terms, so instantiation at a specific `i` is goal-driven & reliable. Used
   for: the step-call precondition (expose `is_bounded_vector 4*3328` on j & j+step_vec BEFORE the
   step), inside `lemma_inner_step_maintains`, in `lemma_postloop_cross_vec`, and to establish the
   function's `is_bounded_poly 3328` post (per-index DONE bound).

## Backup
- `agent-status/Invert_ntt.fst.stepB-topdown-wip` — current on-disk .fst (this session).
- `agent-status/Invert_ntt.fst.stepB-cvda-backup` — prior session base (inline maintenance).

## Backport plan (Task #6, pending .fst green)
The .fst is scratch ahead of Rust. Backport to `invert_ntt.rs`:
1. **Opacity fix**: `inv_ntt_step_post` opaque pred (currently .fsti-scratch) → inject via
   `#[hax_lib::fstar::before(interface, r#"[@@ "opaque_to_smt"] let inv_ntt_step_post ... "#)]`;
   rewrite `inv_ntt_layer_int_vec_step_reduce`'s ensures to cite it instead of the raw foralls.
2. **Predicates + 14 lemmas**: inject via `#[hax_lib::fstar::before(...)]` (non-interface) on
   `invert_ntt_at_layer_4_plus`. They are standalone F* (no Rust antiquotation) → raw-text inject.
   NOTE: function-scope F* bindings `zs = Seq.init ...` and `step_vec_n : pos = v step/16` have
   no Rust equivalent. Cleanest: lift them into injected helper F* fns `zs_of layer ezi` /
   `step_vec_n_of step` so predicates/invariants are functions of Rust-available values only
   (layer, _zeta_i_init, step, round, re.coefficients, _re_init). (Re-verify .fst after this
   refactor.)
3. Remove `--admit_smt_queries true`; strengthen both `loop_invariant!`s to `fstar!` calls of
   `outer_inv`/`inner_inv`; add `#[cfg(hax)] let _re_init = re.coefficients`; add the
   per-body + post-loop lemma-call `fstar!` blocks (mirror layer_1's idiom in this file).
4. Re-extract (`hax.py extract`), per-stage clean build (delete touched .checked, re-make).

## Env notes
- fstar-proxy MCP tools ARE registered this session (mcp__fstar__*). Curl helpers
  /tmp/fp.sh (build/status) and /tmp/ftc.sh (typecheck, injects buffer via jq --rawfile)
  also set up.
- fstar_open session timeout: use 3600s (240s timed out mid-edit-loop last time).
- verify-to-position on this proxy re-verifies from line 1 (does NOT honor a prior
  lax-to-position checkpoint) — so a "function verify" is effectively a partial build and
  surfaces sibling WF VCs (good for catching bugs, ~3-4 min).
- Delete `.fstar-cache/hints/Libcrux_ml_kem.Invert_ntt.fst.hints` before each build.
