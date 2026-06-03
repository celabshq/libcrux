# RESUME — Forward NTT F-B: CLOSED (2026-06-02, session c)

Worktree `/Users/karthik/libcrux-fwd-ntt`, branch `agent/fwd-ntt-mirror-inverse`.
hax 0.3.7 at `~/.opam/hax-0.3.7/bin`. Use the fstar-for-libcrux skill.

## STATUS: forward NTT fully verified, 0 admits, backported + re-extracted clean.

`Libcrux_ml_kem.Ntt.fst` now verifies **admit-free** end-to-end.  `ntt_vector_u`
proves `to_spec_poly_plain re_future == Hacspec_ml_kem.Ntt.ntt (to_spec_poly_plain re)`
with **zero** `assume`/`admit`.  Confirmed by a clean post-extraction full build
(`make check/Libcrux_ml_kem.Ntt.fst`, no `--admit_except`): exit 0, 12.3 min,
"Verified module: Libcrux_ml_kem.Ntt", 1945 real Query-stats lines, 0 errors.
`verification_status.md` refreshed: **ntt (Generic) 1 lax → 0** (12 fns, 0 lax,
5 hacspec); total ml-kem lax 135 → 134.

## What this session closed (F-B = ntt_at_layer_4_plus body + all assumes)

The F-B bridge core was already DONE (commit `7e7e6761e`,
`Hacspec_ml_kem.Commute.Ntt_bridge.fst`).  This session proved the DRIVER body so
the bridge's `lemma_layer_4_plus_to_poly_step` is actually fed, and discharged the
3 small assumes.  All in `Libcrux_ml_kem.Ntt` (src + extracted):

1. **`ntt_step_post`** (opaque, forward Cooley-Tukey butterfly, MONT form):
   `mont(x[i]) == add (mont a[i]) (mul (mont ζ) (mont b[i]))` (._1),
   `mont(y[i]) == sub (mont a[i]) (mul (mont ζ) (mont b[i]))` (._2).
   `ntt_layer_int_vec_step` strengthened to emit it — per-lane via
   `lemma_montgomery_multiply_lane_post_to_mod_q_eq` + `lemma_mod_q_eq_unfold` +
   `lemma_mont_mul_fe_commute_mont_mont` + `lemma_add/sub_fe_commute_mont`
   (NO barrett — simpler than the inverse `inv_ntt_layer_int_vec_step_reduce`).
2. **`lemma_step_keystone_fwd`** (reveal ntt_step_post + f_to_i16_array→f_repr bridge
   → `Ntt_bridge.lemma_cross_vec_from_step_fwd`).
3. **Parameterized scaffold** (mirror of the inverse USER-14 Step B, with the forward
   `cross_vec_hyp_fwd` atom and the bound parameterized by `e_initial_coefficient_bound`):
   `cross_vec_done_at_fwd`, `lemma_cvda_fwd_{intro,reveal,frame1}`,
   `lemma_cross_vec_frame_others_fwd`, `lemma_inner_index_fwd`, `lemma_offset_vec_fwd`,
   `lemma_groups_len_256` (NEW — nonlinear closure `(len(zs)*2)*len==256`),
   `lemma_layer_numeric_facts_fwd` (FORWARD: `e_zeta_i_init == groups-1`; L7 g=1 init=0,
   L6 g=2 init=1, L5 g=4 init=3, L4 g=8 init=7), `lemma_step_keystone_loop_fwd`,
   `outer_inv_fwd`/`inner_inv_fwd` (PENDING=bound/==re_init, DONE=bound+3328/cross_vec_done_at_fwd),
   `lemma_{outer,inner}_inv_fwd_{lookup,init}`, `lemma_inner_step_maintains_fwd`,
   `lemma_inner_to_outer_fwd`, `lemma_postloop_cross_vec_fwd`, `zs_of_fwd`
   (ASCENDING: `mont(zeta(e_zeta_i_init+1+r))`).
4. **`ntt_at_layer_4_plus`** body rewritten with the opaque outer/inner invariants;
   post-loop produces the `cross_vec_hyp_fwd` forall → `Ntt_bridge.lemma_layer_4_plus_to_poly_step`
   → strengthened ensures emits `Ntt_bridge.poly_step re re_future layer`.
5. **`ntt_vector_u`**: 4 `assume poly_step` DROPPED (now from layer_4_plus's post);
   28296 bound via `is_bounded_poly_higher`; barrett-value via the FREE
   `Libcrux_ml_kem.Polynomial.poly_barrett_reduce` (its committed post already carries
   `to_spec_poly_plain result == HP.poly_barrett_reduce (...)`) + `lemma_poly_barrett_reduce_id`.
6. **abs(-1600)** in `ntt_at_layer_7_` via `Spec.Utils.impl_i16__abs_value`.

## Two real issues the FULL build surfaced (hidden by `--admit_except`)

- **Predicate well-formedness**: `e_initial_coefficient_bound +! mk_usize 3328` can
  overflow (param unconstrained).  Fixed by refining every forward-scaffold binder to
  `(e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})`.
- **Nonlinear precondition** of `lemma_layer_4_plus_to_poly_step` (`(len(zs)*2)*len==256`):
  added `lemma_groups_len_256` (clean-context) + `Seq.lemma_init_len` before the call.

## Backport mechanics (for future mirrors)

- `src/polynomial.rs`: made the FREE `poly_barrett_reduce` `pub(crate)` (its strong
  post already existed); the `impl PolynomialRingElement::poly_barrett_reduce` METHOD
  was left bounds-only.  **GOTCHA**: a method's `#[ensures]` cannot use `${self}` in a
  `fstar!` block (rustc E0435 "non-constant value in a constant") — that's why
  `ntt_vector_u` calls the free fn directly instead of the method.
- `src/ntt.rs`: `ntt_step_post` as a `#[cfg(hax)] pub(crate) fn ... -> hax_lib::Prop`
  with `fstar_prop_expr!` + `#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]`
  (mirror `inv_ntt_step_post`); the 569-line scaffold injected as one
  `#[cfg_attr(hax, hax_lib::fstar::before(r#" ... "#))]` on `ntt_at_layer_4_plus`;
  body uses `hax_lib::loop_invariant!` citing the opaque invariants + `hax_lib::fstar!`
  lemma calls (mirror `invert_ntt_at_layer_4_plus`).

## Build discipline reconfirmed
`--admit_except` writes a TAINTED `.checked` AND admits every sibling's own VCs
(incl. predicate well-formedness) — both real issues above only appeared in the full
build.  Always `rm .fstar-cache/checked/<M>.fst.checked` then full build, no admit_except.

## Remaining / not done this session
- NOT committed (left for user review).  `src/ntt.rs`, `src/polynomial.rs`,
  `proofs/fstar/extraction/Libcrux_ml_kem.{Ntt,Polynomial}.{fst,fsti}`,
  `proofs/verification_status.md` are the dirty files.
- A full `make all` (every ml-kem module) was NOT run — only `check/Ntt.fst` (which
  rebuilt Polynomial clean).  The change is isolated (Ntt + a `pub(crate)` whose F*
  `val` signature is unchanged), so no cascade expected, but a CI `make all` is the
  final word.
- Verified-`.fst` backups: `agent-status/Libcrux_ml_kem.Ntt.{fst,fsti}.FB-VERIFIED-20260602`.
