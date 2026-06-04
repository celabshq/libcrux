# Prompt — close `lemma_theta_rho_to_spec` via 5-row-helper factoring

Repo: `/Users/karthik/libcrux-sha3-focused`
Branch: `sha3-byteform-migration`

## Goal

Discharge the last load-bearing admit in the portable Keccak chain:

- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Keccakf.Generic.fst:1309` — `lemma_theta_rho_to_spec`, gated by `#push-options "--admit_smt_queries true"`.

This is item 6 in `crates/algorithms/sha3/proofs/sha3-sprint-todo.md`. After it lands, the portable Keccak chain has zero admits and the SHA-3 portable backend becomes fully verified end-to-end.

## What we already know (do not re-test)

`crates/algorithms/sha3/proofs/proof_milestones.md:182-217` (Note A) and `proofs/sha3-sprint-todo.md` item 6 record SEVEN attempts that all fail on the same Z3 cliff — **lifting 25 in-scope per-index equalities to a forall**:

1. `forall_intro` of `()`-bodied aux (Z3 can't case-split symbolic k under ifuel 1).
2. 25 literal-K asserts before `eq_intro` (293/294 split sub-queries pass <300 ms; eq_intro forall-precondition lift times out at 570 s).
3. 25-branch if-else case-split aux + `forall_intro` (queries 47+ each take 4–6 min; ~20 hour wall extrapolation).
4. `#restart-solver` + `--z3rlimit 800 --z3refresh` (single sub-query, 224 s / 800 rlimit timeout).
5. Replace `eq_intro` with `introduce forall ... with ()` + `eq_intro` (same single sub-query timeout).
6. `introduce forall ... with` + explicit 25-branch `if i = 0 then () else ...` (two sub-queries timeout, 230 s + 226 s).
7. **Weaken post to `forall (i: nat{i < 25}). lhs.[mk_usize i] == rhs.[mk_usize i]`** + move `eq_intro` to the single caller `lemma_one_round_to_spec`. Per-i body sub-queries (#2..#26) all succeeded in 60–2000 ms. The forall consolidation step (#27) failed at 198 s / 800 rlimit. **This is the strongest data point**: it confirms each per-i case is provable in <2 s, but the 25→forall consolidation is the cliff.

Implication for the new approach: any structural fix must avoid producing one giant 25-element forall. Decompose into smaller foralls (5 elements each) so each consolidation step is a 5-element lift Z3 can handle directly.

## Strategy — five row-helpers + per-i dispatcher

The 25 indices are arranged 5×5 in y-major (row `Y = i / 5`, column `X = i % 5`). Each row contains indices `{5Y, 5Y+1, 5Y+2, 5Y+3, 5Y+4}`.

### Step 1 — five row-helpers

For each `Y ∈ {0,1,2,3,4}`, write `lemma_theta_rho_row_Y_to_spec` directly **above** `lemma_theta_rho_to_spec` (insert at line ~1309). Each row helper:

- Same parameter list as `lemma_theta_rho_to_spec` (`v_N`, `lc`, `ks`, `l`).
- Conclusion: a **5-conjunct** statement of the form
  ```
  let s = ks.f_st in
  let ks', d = impl_2__theta v_N #v_T ks in
  let ks'' = impl_2__rho v_N #v_T ks' d in
  let lhs = extract_lane v_N lc ks''.f_st l in
  let rhs = Hacspec_sha3.Keccak_f.rho (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
  lhs.[mk_usize (5*Y + 0)] == rhs.[mk_usize (5*Y + 0)] /\
  lhs.[mk_usize (5*Y + 1)] == rhs.[mk_usize (5*Y + 1)] /\
  lhs.[mk_usize (5*Y + 2)] == rhs.[mk_usize (5*Y + 2)] /\
  lhs.[mk_usize (5*Y + 3)] == rhs.[mk_usize (5*Y + 3)] /\
  lhs.[mk_usize (5*Y + 4)] == rhs.[mk_usize (5*Y + 4)]
  ```
- Body: `lemma_theta_extract_lane v_N lc ks l; lemma_rho_thru_4_extract_lane v_N lc ks' d l; lemma_rho_theta_spec state; Lemmas.lemma_rotate_left_zero ...` — same as the current proof body (helpers populate the 25 conjuncts; F* should pick the 5 relevant ones).
- Pragma: `#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"`. **No** `--split_queries always` (the per-i body work proved fast in attempt #7, ~60 ms per assert; a 5-conjunct VC should close monolithically in ~1–5 s).

For row 0 (which contains `i = 0`), the `lemma_rotate_left_zero` call discharges the `rotl_spec(_, mk_u32 0) == _` rewrite that distinguishes index 0 from the others. Other rows don't need it.

### Step 2 — replace `lemma_theta_rho_to_spec` body

Keep the same signature (array-equality post: `extract_lane ... ks''.f_st l == rho (theta (extract_lane ... s l))`). New body:

```fstar
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_theta_rho_to_spec ... =
  let open Libcrux_sha3.Generic_keccak in
  let s = ks.f_st in
  let ks', d = impl_2__theta v_N #v_T ks in
  let ks'' = impl_2__rho v_N #v_T ks' d in
  let lhs = extract_lane v_N lc ks''.f_st l in
  let rhs = Hacspec_sha3.Keccak_f.rho (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
  lemma_theta_rho_row_0_to_spec v_N lc ks l;
  lemma_theta_rho_row_1_to_spec v_N lc ks l;
  lemma_theta_rho_row_2_to_spec v_N lc ks l;
  lemma_theta_rho_row_3_to_spec v_N lc ks l;
  lemma_theta_rho_row_4_to_spec v_N lc ks l;
  (* Now 25 per-index facts in scope, partitioned into 5 named groups.
     Lift to forall via 5-way case-split on i / 5; each branch has a
     literal Y in scope and the 5 facts from row Y are already known. *)
  introduce forall (i: nat{i < 25}). Seq.index lhs i == Seq.index rhs i with
    (let y = i / 5 in
     if y = 0 then ()
     else if y = 1 then ()
     else if y = 2 then ()
     else if y = 3 then ()
     else ());
  Rust_primitives.Arrays.eq_intro lhs rhs
#pop-options
```

The `()` bodies rely on the row helpers' postconditions giving the 5 facts per row, plus Z3 doing one extra step (column = i % 5) to pick the specific conjunct. If the empty bodies don't close, switch each branch to a 5-way nested `if i = 5*Y then () else if i = 5*Y+1 then () else ...` — but try empty first.

If `eq_intro`'s forall-precondition still cascades, replace the explicit `eq_intro` with `Seq.lemma_eq_intro lhs rhs; Seq.lemma_eq_elim lhs rhs` — sometimes the FStar.Seq versions interact better with the `introduce forall` output than `Rust_primitives.Arrays.eq_intro`.

### Step 3 — verify

```bash
cd /Users/karthik/libcrux-sha3-focused/crates/algorithms/sha3/proofs/fstar/equivalence
make check/EquivImplSpec.Keccakf.Generic.fst > /tmp/sha3-5row.log 2>&1
echo "exit=$?"
grep -E "All verification|^\* Error|TOTAL TIME" /tmp/sha3-5row.log | head
```

For inner-loop iteration on just the new lemmas (avoid the 20-min cold rebuild):
```bash
make OTHERFLAGS="--admit_except 'EquivImplSpec.Keccakf.Generic.lemma_theta_rho_row_0_to_spec'" run/EquivImplSpec.Keccakf.Generic.fst
```
(repeat for each row + the main lemma)

`run/%` does not write `.checked`; safe for iteration. After success, run a real `make check/...` to commit the cache.

## Hard constraints (carried over)

- rlimit cap **800** monolithic, **400** with `--split_queries always` (`feedback_rlimit_cap_800`).
- Never bulk-delete `.checked` files (`feedback_no_cache_nuke`); never touch mtimes (`feedback_no_checked_tampering`). The current `EquivImplSpec.Keccakf.Generic.fst.checked` was DELETED at end of the 2026-05-04 triage session (per user request, since it was unsound from `--admit_except` iterations) — this is fine, the first `make check/...` will rebuild it cleanly.
- `make` output: pipe to log + grep, never Read full log into context (`feedback_grep_make_output`).
- Per-fn debug budget: **60 min wall hard cap**. Each `--admit_except` iteration is 8–10 min; budget for ~5 attempts.
- Edits restricted to `EquivImplSpec.Keccakf.Generic.fst`. No upstream-spec, extraction, or Rust changes.
- After success, run downstream consumers `EquivImplSpec.Sponge.Portable.API.fst` and `Libcrux_sha3.fst` to verify no regression in the chain.

## Reference points

- Lemma being repaired: `EquivImplSpec.Keccakf.Generic.fst:1286-1334` (USER-1 admit comment block + body).
- Spec-side closed form (proven, rlimit 400): `:711-790` (`lemma_rho_theta_spec`).
- Impl-side closed form (proven, rlimit 1600 — separate smell): `:1229-1275` (`lemma_rho_thru_4_extract_lane`).
- Existing column partials (proven, rlimit 800–1600): `lemma_rho_{0..4}_extract_lane` at `:910-1088`; `lemma_rho_thru_{1..3}_extract_lane` at `:1096-1228`. These are column-indexed (column `K = i % 5`), orthogonal to the new row-indexed helpers.
- RHO_OFFSETS lookup table (proven): `:646-690`.
- Single caller of the target: `lemma_one_round_to_spec` at `:1652-1668`.
- Prior reverted attempts (do NOT repeat): `proof_milestones.md:182-217`, `sha3-sprint-todo.md` item 6.

## Verification

- Primary: `make check/EquivImplSpec.Keccakf.Generic.fst.checked` reports zero errors, the `--admit_smt_queries true` push-options is removed, no new rlimit > 800.
- Downstream: `make check/EquivImplSpec.Sponge.Portable.API.fst.checked` and `make check/Libcrux_sha3.fst.checked` both verify with the new `.checked`.
- Update `proof_milestones.md` Note A to record closure (or new failure mode) and prune item 6 from `sha3-sprint-todo.md`.

## Failure exit

If the empty `()` row-dispatch body in step 2 doesn't close:
1. Try the explicit per-i 25-way nested case-split (still 5×5, but inside the outer if-y-then).
2. If still failing, profile: add `--log_queries --z3refresh --query_stats --split_queries always` to step 2 only, then `z3 smt.qi.profile=true queries-EquivImplSpec.Keccakf.Generic-NNN.smt2 2>/tmp/qi.txt` on the failing sub-query. Pipe profile parse to `awk '/\[quantifier_instances\]/ {n=$2; total[n]+=$4} END {for (k in total) printf "%8d %s\n", total[k], k}' /tmp/qi.txt | sort -rn | head -20`.
3. Document the dominant cascading quantifier and stop. `git checkout --` the file. The 60-min budget is the gate.
