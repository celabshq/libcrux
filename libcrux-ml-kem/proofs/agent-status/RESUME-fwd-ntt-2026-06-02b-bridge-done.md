# RESUME — Forward NTT F-B (2026-06-02, session b): bridge core DONE

Worktree `/Users/karthik/libcrux-fwd-ntt`, branch `agent/fwd-ntt-mirror-inverse`.
Cache warm. hax 0.3.7 at `~/.opam/hax-0.3.7/bin` (checkout `d8b5b3d`).
Predecessor handoff: `RESUME-fwd-ntt-2026-06-02.md` (architecture + per-lemma status).
Use the fstar-for-libcrux skill.

## What this session closed (committed `7e7e6761e`)

`Hacspec_ml_kem.Commute.Ntt_bridge.fst` now verifies **admit-free** — real full build
296 s, "All verification conditions discharged", 0 `admit`/`assume`/`admit_smt`.
(The only assumed fact it leans on is `Bridges.lemma_zeta_eq_vzetas`, the by-design
user-approved zeta-correspondence axiom, identical to the inverse.)

Two pieces landed:

1. **TEMP-ADMIT-UNFOLD restored** (`lemma_ntt_layer_unfold_lo` / `lemma_ntt_layer_unfold`).
   Reconstructed (never committed before). Forward is *simpler* than the inverse
   template: `N.ntt_layer` slices `v_ZETAS[groups..2*groups]` directly (no reversed
   `createi` table), so the body is FACT1 (`norm [delta_only[%N.ntt_layer]; iota; zeta;
   primops]; trefl`) + FACT2 (point-wise `lemma_index_slice` → `tbl_slice == zs`).
   Gotchas hit & fixed: (a) bound `v groups` (`<=64` lo / `<=8` hi) BEFORE the
   `mk_usize 2 *! groups` overflow check; (b) establish `len' == len` from `v`-equality
   (`lemma_shift_pow2_lo` for 1-3 / inline `assert_norm` case-split for 4-7).

2. **Section 7 — forward layer-4+ cross-vector keystone (F-B bridge family)**, mirror of
   the Bridges USER-14 inverse keystone for the Cooley-Tukey butterfly, stated so the
   per-coefficient bridge yields **PLAIN** form directly via the Section-1 mont→plain
   cancellation cores (NO global ntt homogeneity). All verified:
   - `cross_vec_hyp_fwd` (opaque, MONT, forward `N.butterfly`, partners m ± step_vec):
     low half `cout[m] == (butterfly zs[block] cin[m] cin[m+sv])._1`,
     high half `cout[m] == (butterfly zs[block] cin[m-sv] cin[m])._2`.
   - `lemma_layer_4_plus_per_coeff_fwd` (cross_vec_hyp_fwd forall → PLAIN per-coeff
     butterfly relations vs `to_spec_poly_plain_arr`; reveal + `lemma_cross_idx` +
     `lemma_partner_idx_{add,sub}` + `tspp_arr_lane` + `lemma_mont_to_plain_butterfly_{plus,minus}`; 21 s).
   - `lemma_layer_4_plus_cross_vector_fwd` (→ `N.ntt_layer_n 256` plain, via
     `lemma_div_128_prod` + `lemma_ntt_layer_n_256_compose`).
   - **`lemma_layer_4_plus_to_poly_step`** (the headline: cross_vec_hyp_fwd forall +
     ascending-zeta requires → `poly_step re_in re_out layer` for layer∈{4,5,6,7};
     via `lemma_to_spec_poly_plain_unfold` + cross_vector_fwd + `lemma_ntt_layer_unfold`).
   - `lemma_cross_vec_from_step_fwd` (generic keystone helper: raw per-lane forward
     butterfly relations for vectors j and j+sv → cross_vec_hyp_fwd for both).
   - `lemma_cross_vec_frame_fwd` (frame across `cout` updates).
   Reuses direction-agnostic Bridges helpers (open'd): `lemma_cross_idx`,
   `lemma_partner_idx_{add,sub}`, `lemma_div_128_prod`, `lemma_vec_partner_hi`.

## STATE of the proof (unchanged from predecessor except the bridge)

`ntt_vector_u` (Libcrux_ml_kem.Ntt.fst) still PROVES
`to_spec_poly_plain re_future == N.ntt (to_spec_poly_plain re)` modulo scoped admits:
- 4 `assume (Ntt_bridge.poly_step re_{k} re_{k+1} (mk_usize {7,6,5,4}))` (lines 675/685/695/705)
  — **F-B body**, the real remaining work.
- 1 `assume (is_bounded_poly bnd28296 re)` (line 741).
- 1 barrett-value `assume (to_spec_poly_plain (barrett_reduce re) == to_spec_poly_plain re7)` (line 744).
- `assume (v (impl_i16__abs (mk_i16 (-1600))) == 1600)` in `ntt_at_layer_7_` (line 531).
Layers 1-3 bridges + compose_7 are REAL. The bridge file is now fully real too.

## REMAINING WORK — F-B body scaffold (the big one)

Removing the 4 `assume poly_step` requires PROVING `ntt_at_layer_4_plus`'s body so it
produces the `cross_vec_hyp_fwd` forall, then converting to `poly_step` via the (now
verified) `lemma_layer_4_plus_to_poly_step`. This is a port of the inverse scaffold
`Libcrux_ml_kem.Invert_ntt.fst` lines **363–1270** (≈900 lines). It is mechanical in
structure but has **three real adaptations** (NOT a blind copy):

1. **Forward `ntt_step_post`** (opaque, analog of `inv_ntt_step_post` @ Invert_ntt.fst:365).
   From `ntt_layer_int_vec_step a b ζ_r bound` (Ntt.fst:360): `t=mont_mul(b,ζ_r)`,
   `x=add_bounded a t` (=`a+ζ_r·b`), `y=sub_bounded a t` (=`a−ζ_r·b`); x stored at j, y at j+sv.
   Post (per lane i<16):
   `mont(x[i]) == add (mont a[i]) (mul (mont ζ_r) (mont b[i]))`  (butterfly._1)
   `mont(y[i]) == sub (mont a[i]) (mul (mont ζ_r) (mont b[i]))`  (butterfly._2)
   Per-lane proof (NO barrett, simpler than inverse): `lemma_mont_mul_fe_commute_mont_mont
   ζ_r b[i] t[i]` (needs montgomery_multiply lane post `v t % q == (v b * v ζ_r * 169)%q`)
   + `lemma_add_fe_commute_mont a[i] t[i] x[i]` + `lemma_sub_fe_commute_mont a[i] t[i] y[i]`
   (Chunk lemmas; add/sub lane posts give `v x==v a + v t`, `v y==v a − v t`).
   Strengthen `ntt_layer_int_vec_step` ensures to emit `ntt_step_post` (mirror the
   `aux0`/`aux1` + `reveal_opaque` pattern at Invert_ntt.fst:439-486, dropping the barrett `aux0`).

2. **`lemma_step_keystone_fwd`** (analog of `lemma_step_keystone` @ Invert_ntt.fst:498):
   reveal `ntt_step_post`, get the raw per-lane butterfly relations, call
   `Ntt_bridge.lemma_cross_vec_from_step_fwd` → cross_vec_hyp_fwd for j and j+sv.
   Lives in the Ntt module (cites the module-local `ntt_step_post`).

3. **`outer_inv`/`inner_inv` PARAMETERIZED by `e_initial_coefficient_bound`** (≠ inverse,
   which hardcoded 4·3328 PENDING / 3328 DONE). Forward polarity is OPPOSITE and variable:
   - PENDING (i ≥ threshold): `is_bounded_vector e_initial_coefficient_bound coeffs[i]
     /\ coeffs[i] == re_init[i]`
   - DONE (i < threshold): `is_bounded_vector (e_initial_coefficient_bound + 3328) coeffs[i]
     /\ cross_vec_done_at_fwd re_init coeffs step_vec_n zs (v i)`
   (`e_initial_coefficient_bound` is an extra param of both invs.) The body invariant
   shape is already in the extracted `ntt_at_layer_4_plus` (Ntt.fst:404-448) — match it.
   `cross_vec_done_at_fwd m` = `forall l<16. cross_vec_hyp_fwd re_init coeffs sv zs m l`
   (analog of inverse `cross_vec_done_at`, ~Invert_ntt.fst:543).

Plus the direction-agnostic scaffold (copy + rename inv→fwd, swap butterfly atom):
`cross_vec_done_at_fwd`, lookups (`lemma_outer/inner_inv_lookup`), init
(`lemma_outer/inner_inv_init`), maintenance (`lemma_inner_step_maintains` — uses
`lemma_step_keystone_fwd` + `lemma_cross_vec_frame_fwd`), `lemma_inner_to_outer`,
`lemma_postloop_cross_vec`, and the numeric helpers `lemma_inner_index` / `lemma_offset_vec`
(copy as-is). **`lemma_layer_numeric_facts`**: zeta_i **INCREMENTS** in forward; the
relation is `zeta_i_init == groups − 1` per layer (L7 groups=1 init=0; L6 g=2 init=1;
L5 g=4 init=3; L4 g=8 init=7). `zs_of_fwd groups zeta_i_init =
Seq.init groups (fun r -> mont(zeta(zeta_i_init + 1 + r)))` (ASCENDING); the
`lemma_zeta_eq_vzetas (zeta_i_init+1+r)` + `zeta_i_init+1==groups` gives
`zs[r] == v_ZETAS[groups+r]` (matches `lemma_layer_4_plus_to_poly_step`'s requires).

Body wiring: mirror `invert_ntt_at_layer_4_plus` (Invert_ntt.fst:1053-1268) but the
post-loop ends with `Ntt_bridge.lemma_layer_4_plus_to_poly_step` (PLAIN poly_step),
NOT `post_from_cross_vec` (MONT). Strengthen `ntt_at_layer_4_plus` ensures to emit
`poly_step` (opaque) — then `ntt_vector_u` calls `lemma_poly_step_intro` (or directly
consumes it) to drop the 4 assumes.

### The 3 small assumes (all in Libcrux_ml_kem.Ntt.fst; bundle into the same build)
- **abs(-1600)** (line 531): NOT computable (`abs_i16` is an abstract `val`). Replace the
  local `assume` with `Spec.Utils.impl_i16__abs_value (mk_i16 (-1600))` (gives
  `v (impl_i16__abs x) == Prims.abs (v x)`) — relocates to a pre-existing accepted library
  axiom. Confirm `Spec.Utils` is in scope / imported; check the lemma's `requires`
  (`x >. i16::MIN`, holds for −1600).
- **28296 bound** (line 741): factor `is_bounded_poly_higher re (8·3328=26624) 28296`
  into a clean-context helper (the inverse calls `is_bounded_poly_higher` directly,
  Invert_ntt.fst:1283+); the inline call saturates under post-layer quantifier pollution.
- **barrett-value** (line 744): strengthen `Libcrux_ml_kem.Polynomial.impl__poly_barrett_reduce`
  post (shared Polynomial.fst — cascade risk, do last) to give `barrett_reduce_post` per
  chunk, then `Chunk.lemma_poly_barrett_reduce_commute` + `lemma_poly_barrett_reduce_id`
  (plain identity, so `to_spec_poly_plain` survives the reduce).

### F-D backport + finish
After the .fst-direct edits verify: port all body wiring + ensures to `src/ntt.rs`
(F-A pattern already there for layers 1-3; mirror it for layer_4_plus: ensures emits
poly_step, post-loop calls the keystone), re-extract (hax 0.3.7), full build of
`check/Libcrux_ml_kem.Ntt.fst` (no `--admit_except`), confirm ntt module 1 lax → 0,
refresh `verification_status.md`.

## Build discipline (re-confirmed this session)
- `--admit_except <fn>` writes a TAINTED `.checked`; a later run can load it & "pass" in
  <6 s with NO real query stats. After a green `--admit_except` sequence, ALWAYS
  `rm .fstar-cache/checked/<Module>.fst.checked` then run a full build (no `--admit_except`)
  and require a real multi-minute wall + "All verification conditions discharged".
  (`touch` of the .fst alone did NOT reliably invalidate; rm the .checked.)
- Bridge full build ≈ 5 min. Driver `Libcrux_ml_kem.Ntt.fst` full build ≈ 5-7 min.
  Use `--admit_except` per-fn for the inner loop. Curl `/tmp/fp.sh fstar_build ... wait_secs:580`
  to beat the 60 s MCP cap. Max 4 own fstar/z3; serialize builds.
- 3 idle mldsa scratch sessions were parked on the proxy (13 h idle, harmless).
