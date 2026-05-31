# Session prompt — USER-15: close `invert_ntt_montgomery` functional post

Use the **fstar-for-libcrux** skill. Worktree `/Users/karthik/libcrux-user14-bridge`, branch
`agent/user-14-layer4-bridge` (or a fresh worktree off its tip). A sha3 agent may also be building in
`/Users/karthik/libcrux-sha3-proofs` — never touch its processes; cap your own fstar+z3 at ≤4 and watch RSS.

## Goal
Discharge the functional postcondition of `invert_ntt_montgomery` (the 7-layer inverse-NTT driver) in
`libcrux-ml-kem/proofs/fstar/extraction/Libcrux_ml_kem.Invert_ntt.fst`:

```
to_spec_poly_mont #v_Vector re_future == ntt_inverse_butterflies (to_spec_poly_mont #v_Vector re)
```

and **remove the `admit () (* Panic freedom *)`** that currently ends the function body. The bounds
conjunct (`is_bounded_poly 3328 re_future`) already flows from the last layer's post. Then backport to
`invert_ntt.rs` (the driver already calls the 7 layers; add the per-step + chain `fstar!` asserts and any
new Bridges lemma), re-extract, per-stage clean build.

## Why this is unblocked NOW
USER-14 closed `invert_ntt_at_layer_4_plus` (verified, no assume — see
`agent-status/user14-stepB-topdown-2026-05-31.md`, verified artifact at git commit `55cdf0a2d` /
`agent-status/Invert_ntt.fst.stepB-topdown-VERIFIED`). Its post is already in the exact polynomial form
the chain needs.

## The shape — this is a COMPOSITION proof, NOT loop-threading
`ntt_inverse_butterflies` (verified spec, `specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.Invert_ntt.fst:167`):
```
ntt_inverse_butterflies p = ntt_inverse_layer (ntt_inverse_layer (… (ntt_inverse_layer p 1) …) 6) 7
```
`invert_ntt_montgomery` runs `layer_1; layer_2; layer_3; layer_4_plus(4); …(5); …(6); …(7)`, threading
`re_0 → re_1 → … → re_7`. Prove each step in polynomial form
`to_spec_poly_mont re_i == ntt_inverse_layer (to_spec_poly_mont re_{i-1}) i`, then chain the 7 steps.

### Step 1 — layers 4–7 are FREE
`invert_ntt_at_layer_4_plus`'s post is *already*
`to_spec_poly_mont re_i == ntt_inverse_layer (to_spec_poly_mont re_{i-1}) layer` for layer∈{4,5,6,7}.
Just sequence those four steps — no new lemma.

### Step 2 — the ONE new lemma: layers 1–3 per-vector → polynomial bridge
`invert_ntt_at_layer_{1,2,3}` posts are PER-16-VECTOR (intra-vector), shape (for k<16):
```
mont_i16_to_spec_array 16 (f_repr re_i.f_coefficients.[k]) ==
  ntt_inverse_layer_n 16 (mont_i16_to_spec_array 16 (f_repr re_{i-1}.f_coefficients.[k])) len_i (zetas_… …)
```
with len = 2 (layer 1), 4 (layer 2), 8 (layer 3) — all < 16, so the butterfly stays WITHIN each 16-vector.
Write a Bridges lemma (place in `Hacspec_ml_kem.Commute.Bridges.fst`):
```
lemma_intra_vec_layer_to_poly (#vV) {| iop |} (re_in re_out: VV.t_PolynomialRingElement vV) (layer: usize)
  : Lemma (requires (v layer == 1 \/ v layer == 2 \/ v layer == 3) /\
                    <the 16 per-vector ntt_inverse_layer_n 16 … (2^layer) … equalities>)
          (ensures to_spec_poly_mont re_out == ntt_inverse_layer (to_spec_poly_mont re_in) layer)
```
Building blocks ALREADY in Bridges (reuse, don't re-derive):
- `lemma_ntt_inverse_layer_n_16_{2,4,8}_lane` (lines 41/463/308) — the intra-vector per-len lemmas.
- `lemma_ntt_inverse_layer_n_256_compose` + `lemma_ntt_inverse_layer_unfold` (the per-coeff→256 +
  table→explicit plumbing from USER-14).
- `lemma_zeta_eq_vzetas` (axiom) — bridges the impl `zetas_{4,2,1}_ (zeta …)` to `ntt_inverse_layer`'s
  internal `v_ZETAS` table (same trick USER-14 used; the per-layer zeta indices are
  `127-4k / 126-4k / …` for layer 1, `63-2k / 62-2k` layer 2, `31-k` layer 3).
- `lemma_to_spec_poly_mont_unfold` — `to_spec_poly_mont re == to_spec_poly_mont_arr re.f_coefficients`,
  the per-vector ↔ 256-array assembly (same as `lemma_layer_4_plus_per_coeff` uses).
Key fact to encode: for len<16, `ntt_inverse_layer p layer` applied to the 256-poly equals applying
`ntt_inverse_layer_n 16 vec_k (2^layer)` to each of the 16 vectors independently (the cross-vector
`ntt_inverse_layer_n 256` from USER-14 is for len≥16; here it stays intra-vector). Mirror the
`lemma_layer_4_plus_per_coeff` → `lemma_ntt_inverse_layer_unfold` structure but with the `_16_*_lane`
intra-vector decomposition.

### Step 3 — chain in `invert_ntt_montgomery`
After each layer call, assert its polynomial-form step (Step 1 for 4–7, `lemma_intra_vec_layer_to_poly`
for 1–3), then a 7-fold `calc (==)` / sequential rewrite composes:
```
to_spec_poly_mont re_7
  == ntt_inverse_layer (to_spec_poly_mont re_6) 7
  == ntt_inverse_layer (ntt_inverse_layer (to_spec_poly_mont re_5) 6) 7
  == … == ntt_inverse_butterflies (to_spec_poly_mont re_0)
```
The RHS literally unfolds `ntt_inverse_butterflies` (its def is the 7-fold `let`). Mechanical once the
7 steps hold. NOTE: the body has `is_bounded_poly_higher` coercions between layers — they don't change
`to_spec_poly_mont`, but you may need a `to_spec_poly_mont`-invariance assert across the coercion (it's a
bound-widening identity on the coefficients).

### Step 4 — remove `admit`, backport, gate
Remove the panic-freedom `admit`. Backport to `invert_ntt.rs::invert_ntt_montgomery`: the 7 layer calls
exist; add (a) the new `lemma_intra_vec_layer_to_poly` (in the commute dir — Bridges is hand-maintained,
git-tracked, NOT extracted, so edit it directly there), (b) per-step + chain `fstar!` asserts in the driver
body. Re-extract (`cd libcrux-ml-kem && python3 hax.py extract`); per-stage clean build
(`rm` the module's `.checked` + hints first; `fstar_build check/Libcrux_ml_kem.Invert_ntt.fst`, NO
`--admit_except`).

## Reuse summary
- FREE: layer_4_plus posts (USER-14), all the `_256_compose`/`unfold`/`zeta_eq_vzetas`/`to_spec_poly_mont`
  lemmas.
- NEW: `lemma_intra_vec_layer_to_poly` (Bridges) + ~7 chain asserts in the driver.
- This is NOT a Z3-wall task — it's composition of already-verified posts. Est. lane A (Bridges lemma)
  ~30–90 min, lane B (driver chain) ~30–45 min.

## Env / workflow (same as USER-14 session)
- All F* via the `fstar-proxy` MCP (`mcp__fstar__*`) or curl helpers `/tmp/fp.sh <tool> <args-json>`
  (build/status) and `/tmp/ftc.sh <session_id> <file> <kind> [<to_line>]` (typecheck — injects buffer via
  `jq --rawfile`). NEVER shell out to `make`/`fstar.exe`.
- `fstar_open` timeout: use 3600–7200 s (240 s timed out mid-edit-loop). Session recipe / include list:
  derive from `cd …/extraction && make --dry-run check/Libcrux_ml_kem.Invert_ntt.fst | grep fstar.exe`
  (17 includes incl. `../spec` + `specs/ml-kem/proofs/fstar/commute`); fstar_exe `/Users/karthik/.local/bin/fstar.exe`.
- Bridges.fst is in `specs/ml-kem/proofs/fstar/commute/` (git-tracked, hand-maintained, NOT extracted —
  edit directly; iterate with `fstar_build check/Hacspec_ml_kem.Commute.Bridges.fst`).
- Per-stage clean rebuild (delete touched `.checked` + hints before each build); verify the NEW lemma in
  isolation (Bridges build) BEFORE wiring it into the driver.
- Verify-to-position on this proxy re-verifies from line 1 (doesn't honor a prior lax checkpoint) — so a
  "function verify" is effectively a partial build (~3–4 min) and surfaces sibling WF VCs. Use
  `--admit_except` for fast single-fn isolation; ALWAYS finish with a full build (no `--admit_except`).
- `.fst/.fsti` are gitignored; back up to `agent-status/` after each milestone (commit durable artifacts).
- Cap ≤4 fstar+z3 procs; watch RSS (≤24 GB); never kill the sha3 build.

## Done =
Full `fstar_build check/Libcrux_ml_kem.Invert_ntt.fst` green (NO `--admit_except`), `assume`=0,
`[@@admitted]`=0, NO `admit ()` in `invert_ntt_montgomery`; then backport to `invert_ntt.rs` +
`Bridges.fst`, re-extract, per-stage clean build green.
```
```
