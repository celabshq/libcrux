# NTT functional layers 0-2 session (2026-06-03, evening)

Continuation of the layer-0 chunk bridge (ntt-layer0-chunk-bridge-2026-06-03.md).
Scope: (1) all-32-chunk layer-0 composition, (2) Portable simd_unit_ntt_step /
at_layer_0 functional FE posts, (3) layers 1-2 within-chunk bridges.
ALL THREE STEPS PROVEN + COMMITTED. Full-crate prove in flight (final gate).

## Step 1 — DONE (commit 215f3c8c6)
`lemma_ntt_layer_0_step_to_hacspec_poly` in Commute.Chunk.fst: forall i<256
mod-q congruence to `ntt_layer input 0`, witness fns (b,p)->t/zm, dispatching
to the chunk lemma per i/8. Clean module build 77s, 395 query-stats, max
sub-query 65ms. No new admits (baseline 1 admit at line 639 unchanged).

## Step 2 — DONE (commit a12bde20a)
- FE-form ensures on `simd_unit_ntt_step`: exact butterfly relations
  (new_lo == old_lo + t, new_hi == old_lo - t) with t NAMED as the
  `montgomery_multiply_fe_by_fer` application on the OLD hi lane + mod-q
  congruence (v t) % q == (v hi_old * v zeta * 8265825) % q.
  `reveal_opaque mod_q` at body end converts mmfbf's opaque-mod_q post.
- GOTCHA FOUND: requires had only `v step <= 4` — step=0 was admitted, under
  which the FE conjuncts are FALSE (both updates collapse onto one lane;
  bounds posts didn't care). Failure signature: ONE sub-query (q55)
  "incomplete quantifiers" at 4.9/300 rlimit. Fix: `1 <= v step`.
  All callers pass step in {1,2,4}.
- `simd_unit_ntt_at_layer_0` lifted post: 4 pairs x 3 conjuncts in exactly
  the chunk-lemma-requires shape (t_p named via mmfbf on ORIGINAL lanes;
  frame-chaining through the 4 sequential step calls is automatic from
  modifies2_8 ground equalities). Bounds posts kept.
- Re-extraction byte-identical to the verified .fst (md5 4d76002f...).

## Step 3 — DONE (commit e072c02dc)
Layer-1 (pairs (4h+j, 4h+j+2), zeta v_ZETAS[2b+h+64] one per half) and
layer-2 (pairs (p, p+4), zeta v_ZETAS[b+32] one per chunk) in
Commute.Chunk.fst: reducer + lane-reduction + clean-context per-pair +
per-chunk dispatcher + all-32-chunk poly composition each.
`lemma_layer_0_pair_spec` reused as the layer-agnostic butterfly algebra.

- TRIGGER-COVERAGE GOTCHA (new, generalizable): in a requires
  `forall (b,h,j). butterfly(t b h j) /\ zeta-congruence(zm b h)`, the ONLY
  single-term trigger covering all three binders is `t b h j`. Under
  --split_queries the zeta sub-goals (which never mention t) cannot fire the
  instantiation -> "incomplete quantifiers" on exactly the zeta conjuncts
  (q73/q74). Layer-0 passed because its zm is indexed (b,p) = same arity as
  t. FIX: split the requires into two foralls — butterfly per (b,h,j), zeta
  per (b,h) [resp. per b for layer 2] — so each forall's natural trigger
  matches its goals' terms.
- Benign: layer_1_lane/layer_2_lane monolithic WF VC cancels at the file
  default rlimit 80, then F*'s automatic retry-with-split passes all 33
  sub-queries at <=29/80. Not a failure (build exit 0).

## Gate — GREEN (session complete)
- Combined chain build (Chunk -> Portable.Arithmetic -> Portable.Ntt, all
  stale after the Chunk edit): exit 0, 19.5 min wall, 4412 query-stats,
  no ipc crash, zero unretried failures. Build d26c685e.
- Full-crate `JOBS=2 ./hax.sh prove`: **99 modules (94 CHECK + 5 ADMIT
  pre-existing), 99 verified, 0 F* errors, 0 make failures** (~75 min;
  Portable.Invntt alone took ~45 min cold — hint invalidation from the
  Chunk->Arithmetic dep cascade, passed clean).
- verification_status.md regenerated: IDENTICAL to committed (no tier
  flips; this session strengthened existing ensures + added proof-lib
  lemmas, lax count stays 42).

## Follow-up "A" (driver wiring) — scoping (2026-06-04)
- A2 DONE: `simd_unit_ntt_at_layer_1` + `_2` now carry the FE-form functional
  post (direct analogs of the verified at_layer_0 post; disjoint-pair reads of
  the original lanes). All THREE per-SIMD-unit layer fns now expose the
  butterfly relations the chunk lemmas consume.
- A1 FINDING: there is NO clean forall `v_ZETAS[idx] == zeta idx` bridge —
  `Spec.MLDSA.Ntt.zeta` is a `match` (not normalizer-friendly at symbolic
  idx), so the zeta congruence `(v zm)%q == (v v_ZETAS[idx]*pow2 32)%q` is
  inherently per-CONCRETE-idx (ground `assert_norm`). Confirmed the impl's
  hardcoded round constants ARE `zeta_r(idx)` (round0 layer0 = zeta_r(128..131)
  = 2091667,3407706,2316500,3817976) and `v_ZETAS[idx]==zeta(idx)` for idx>=1
  (v_ZETAS[0]=1 vs zeta(0)=0, unused at L0-2).
- A3 (the remaining big task) = wire `ntt_at_layer_{0,1,2}` (32-chunk drivers)
  functional ensures to `lemma_ntt_layer_{0,1,2}_step_to_hacspec_poly`. Needs:
  (1) add the FE relations to each inner `round` post (so chunk b's relations
  surface, forall32-style like the existing bounds post); (2) snapshot
  `orig_re` + state the driver post relative to it; (3) construct witness
  FUNCTIONS `t:(b->p->i32)`, `zm:(b->i32)` (32-arm lambdas returning the
  per-round literals — the fiddly part); (4) ~32/64/128 ground `assert_norm`
  zeta congruences; (5) one poly-lemma call. RISK: the 32-round sequential
  functional WP (relations relative to a snapshot) — bounds composes
  automatically, but functional-relative-to-orig may need explicit framing.
  Estimated multi-hour; its own focused session.
- A3 REFINED BLOCKER (2026-06-04): the witness `zm b` can be
  `cast (Spec.MLDSA.Ntt.zeta_r (b+32))` — `zeta_r`'s ensures holds at SYMBOLIC
  b, giving `(v (zm b))%q == (zeta(b+32)*pow2 32)%q` for free (no per-b
  assert_norm). BUT the poly lemma's zeta hypothesis is stated against
  `v_ZETAS[b+32]`, so closing it still needs `v_ZETAS[idx] == zeta idx` at
  SYMBOLIC idx. `Spec.MLDSA.Ntt.zeta` is a `match` (no symbolic reduction) and
  there is no shared underlying list, so this table equality is only provable
  per-CONCRETE-idx → a 32/64/128-arm match-dispatch per driver (per-i-match
  recipe; 32 borderline, 128 rough). OPTION to de-risk: change the poly
  lemmas' zeta hypothesis to cite `Spec.MLDSA.Ntt.zeta (idx)` instead of
  `v_ZETAS[idx]`, and prove the within-chunk reducers (`layer_k_lane`) against
  `zeta` once via the SAME table bridge — pushing the 224-arm bridge into ONE
  reusable lemma proven by `introduce forall + match` (still 256 arms but
  written once, not per-driver). Decide bridge-home before starting A3.
- GOOD NEWS on composition: the 32-round functional WP should be NO harder than
  the bounds forall32 — the snapshot-equality fact `re_before_b[b]==orig[b]` is
  ALREADY discharged by F* to type the bounds round-call preconditions; the FE
  relation `FErel(re_future[b], orig[b])` is preserved by later-round modifies
  exactly like the absolute bound. So the risk is the table bridge + witness
  plumbing, not the fold composition.

## Next (layer 3+ / future sessions)
- Cross-chunk layers 3-7 (Commute.Bridges port) + 8-layer compose to
  `== Hacspec_ml_dsa.Ntt.ntt` driver.
- Wiring the poly composition into `ntt_at_layer_0`'s ensures needs the
  zeta_r-discharge at the round level + a v_ZETAS<->Spec.MLDSA.Ntt.zeta
  bridge (v_ZETAS matches zeta for idx>=1; idx 0 differs, unused at L0-2).
- at_layer_1/at_layer_2 impl-post lifts (mirror of a12bde20a's at_layer_0).
