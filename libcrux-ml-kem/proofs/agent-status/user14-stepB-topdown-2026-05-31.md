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

## Backport progress (Task #6, IN PROGRESS)
### DONE — opacity fix in `invert_ntt.rs` (working tree, untested pending full extract)
- Added `#[cfg(hax)] #[fstar::before([@@ "opaque_to_smt"])] pub(crate) fn inv_ntt_step_post<Vector>(a,b,r0,r1: &Vector, zeta_r: i16) -> hax_lib::Prop` with the two per-lane foralls as `fstar_prop_expr!`, placed before `inv_ntt_layer_int_vec_step_reduce`.
- `inv_ntt_layer_int_vec_step_reduce` `#[ensures]`: replaced the raw foralls with `fstar!(r#"inv_ntt_step_post #$:Vector ${a} ${b} ${r0} ${r1} ${zeta_r}"#)`.
- Body: appended `hax_lib::fstar!(reveal_opaque (\`%inv_ntt_step_post) (inv_ntt_step_post #$:Vector ${_a_in} ${_b_in} ${r0} ${r1} ${zeta_r}))`.

### REMAINING — confirmed mechanism: RAW `fstar::before` injection (NO type reshape)
The verified `.fst` lemmas use native-F* `pos`/`nat`/`t_Array`/`Seq` (don't round-trip through Rust
types), so inject them VERBATIM (they're standalone, no `${}` antiquotation) and have the function
reference them via `fstar!` with `v`-projected args. Steps:
1. On `invert_ntt_at_layer_4_plus`, add ONE `#[cfg_attr(hax, hax_lib::fstar::before(r#" <verbatim F*> "#))]`
   containing, IN ORDER (copy from the committed verified `.fst`, lines ~434–984): `lemma_step_keystone`,
   `cross_vec_done_at` + `lemma_cvda_intro/reveal/frame1`, `lemma_cross_vec_frame_others`, `lemma_inner_index`,
   `lemma_offset_vec`, `lemma_layer_numeric_facts`, `lemma_step_keystone_loop`, `outer_inv`, `inner_inv`,
   `lemma_outer_inv_lookup`, `lemma_inner_inv_lookup`, `lemma_outer_inv_init`, `lemma_inner_inv_init`,
   `lemma_inner_step_maintains`, `lemma_inner_to_outer`, `lemma_postloop_cross_vec`, PLUS a new
   `let zs_of (layer e_zeta_i_init: usize) : t_Slice Hacspec_ml_kem.Parameters.t_FieldElement =
   Seq.init (v (mk_usize 128 >>! layer)) (fun (r:nat{r < v (mk_usize 128>>!layer)}) ->
   mont_i16_to_spec_fe (zeta (e_zeta_i_init -! mk_usize 1 -! mk_usize r)))`.
   (The raw F* embeds cleanly in `r#"..."#` — backticks/`#push-options "..."` are fine; no `"#` sequence appears.)
2. Remove `#[hax_lib::fstar::options("--admit_smt_queries true")]`; restore the real options
   `("--z3rlimit 400 --ext context_pruning --split_queries always")`.
3. Function body (mirror the committed verified `.fst` fn + layer_1's idiom):
   - `#[cfg(hax)] let _re_init = re.coefficients;` and `#[cfg(hax)] let _re0 = ...` (or just use _re_init —
     `lemma_layer_4_plus_post_from_cross_vec` needs re_in.f_coefficients == _re_init).
   - `let step = 1 << layer;`, `let groups = 128 >> layer;`, `let step_vec_n = step / FIELD_ELEMENTS_IN_VECTOR;`
     (usize; pass `(v ${step_vec_n})` for the nat/pos arg — F* proves pos from the numeric facts).
   - `hax_lib::fstar!(r#"lemma_layer_numeric_facts ${layer} ${_zeta_i_init}"#);` before the loop.
   - `hax_lib::fstar!(r#"lemma_outer_inv_init #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n}) (zs_of ${layer} ${_zeta_i_init}) ${step}"#);` (outer-fold init).
   - outer `loop_invariant!(|round| { (*zeta_i == _zeta_i_init - round).to_prop() & fstar!(r#"outer_inv #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n}) (zs_of ${layer} ${_zeta_i_init}) ${round} ${step}"#) })`.
   - outer body: before inner loop `fstar!(lemma_inner_inv_init … round step offset_vec step_vec)`; inner
     `loop_invariant!(|j| fstar!(r#"inner_inv #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n}) (zs_of …) ${offset_vec} ${step_vec} ${j}"#))`.
   - inner body: BEFORE the step, `fstar!(lemma_inner_inv_lookup … j j; lemma_inner_inv_lookup … j (j+!step_vec))`
     (expose the 4*3328 bounds for the step precondition); after the two `re.coefficients[..] = ` updates,
     `fstar!(lemma_offset_vec …; FStar.Seq.Base.init_index_ …; lemma_inner_step_maintains …)`.
   - after inner loop: `fstar!(lemma_offset_vec …; lemma_inner_to_outer … (round+!1) (offset_vec+!step_vec))`.
   - epilogue: the zeta-table `aux` forall, `lemma_offset_vec (v groups) …`, the `is_bounded_poly 3328`
     `auxb` forall via `lemma_outer_inv_lookup … (sz i)`, `lemma_postloop_cross_vec …`,
     `lemma_layer_4_plus_post_from_cross_vec _re0 re layer step (v step_vec_n) (zs_of …)`.
4. `cargo check` (won't validate cfg(hax) bodies); then re-extract; then per-stage clean build of
   `check/Libcrux_ml_kem.Invert_ntt.fst` (delete its .checked + hints first). Iterate: the extracted
   function from loop_invariant!/fstar! must verify — compare to the committed verified `.fst` (commit
   `55cdf0a2d`) and adjust the Rust until the extracted module verifies with NO admit/assume.
5. SAFETY: before re-extracting (it OVERWRITES the on-disk `.fst`/`.fsti`), the verified versions are at
   commit `55cdf0a2d` (`agent-status/Invert_ntt.{fst,fsti}.stepB-topdown-VERIFIED`), `/tmp/Invert_ntt.{fst,fsti}.VERIFIED.bak`,
   and `/Users/karthik/user14-backups/*.tgz`. If extraction breaks the module, restore the scratch `.fst`/`.fsti`
   to rebuild-verify, and iterate the Rust.

## (original) Backport plan (Task #6, pending .fst green)
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
