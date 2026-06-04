# Neon NTT functional posts — session status (2026-06-04)

Branch: libcrux-ml-kem-proofs, base e401d0bf0.

## MILESTONE 2 DONE — full no-admit module build CLEAN (2026-06-04)
`make check/Libcrux_ml_kem.Vector.Neon.Ntt.fst` (build 89d3cd7d):
`exit_code 0`, `all_vcs_discharged: true`, `error_count 0`, **query_count 790**,
failed_modules []. Wall 5 min (down from 19.5). The 4 "failed_queries" are the
benign .fsti-boundary stale-hint replays (used_hint:true, rlimit ~0.01, all
retried+succeeded). cargo test --features simd128 --lib: 23/23.
Two s64-class call-site saturations (forward-2 res values, inverse-2 bres
congruence) were each closed by a direct-value bridge lemma (see below); peak
real rlimit now inv-2 = 221 (was saturating 400), fwd-2 = 101.

## VERIFIED in F* (.fst), saved to agent-status/neon-ntt-VERIFIED.{fst,fsti}.txt
All 6 layer functions + the two-vector montgomery foundation carry real
functional posts and verify in the full no-admit module build (above):
- `montgomery_multiply_int16x8_t` (Neon.Arithmetic): opaque_to_smt + IMPLICATION
  post (l_True pre). Mirrors by-constant worker, vector multiplier. CONFIRMED.
- `ntt_layer_1_step`  -> ntt_layer_1_butterfly_post (s32 transpose). CONFIRMED.
- `ntt_layer_2_step`  -> ntt_layer_2_butterfly_post (s64). CONFIRMED, peak 101
  rlimit via lemma_fwd_l2_resultv (direct res values, t symbolic).
- `ntt_layer_3_step`  -> AVX2 inline-forall post. CONFIRMED.
- `inv_ntt_layer_1_step` -> inv_ntt_layer_1_butterfly_post (s32 + barrett). CONFIRMED.
- `inv_ntt_layer_2_step` -> inv_ntt_layer_2_butterfly_post (s64). CONFIRMED, peak
  221 rlimit via lemma_inv_l2_bdiff (direct b_minus_a diffs, bres symbolic).
- `inv_ntt_layer_3_step` -> AVX2 inline-forall post. CONFIRMED.

### s64 direct-value bridge lemmas (the fix for both layer-2 cliffs)
Both s64 layer-2 functions saturated Z3 at the full rlimit 400 at the
post-helper CALL SITE: the per-lane VALUE/congruence obligation composes the
admitted s64 transpose lane-permutation with the lane add/sub (and, for inv-2,
montgomery's %3329).  Fix = an admitted direct-value bridge that hands the call
site ground equalities so the obligation becomes substitution (memory:
"equality beats strict-ineq in composer arith"):
- `lemma_fwd_l2_resultv` (admit): res lane VALUES = iv_j +/- t (t = montgomery
  output, kept SYMBOLIC); lemma_neon_fwd_l2_post then does the t congruence.
- `lemma_inv_l2_bdiff` (admit): the PRE-montgomery diff b_minus_a lane VALUES =
  iv_{k+4} - iv_k (bres = montgomery output, kept SYMBOLIC); montgomery's own
  PROVEN congruence does the %3329, lemma_neon_inv_l2_post reveals the post.
Both are the admitted lane permutation (cf. lemma_trn{1,2}_s64_reinterpret,
lemma_trn_s64_bound) composed with an EXACT integer add/sub (each |.| < 2^15),
mirroring the AVX2 admitted-shuffle precedent.  Keeping the montgomery output
symbolic (not folding %3329 into the admit) preserves the real reduction proof.

### Recipe
- repr lanes 0-7 = f_low, 8-15 = f_high (lemma_repr_index SMTPat).
- transpose dance tracked by 4 ADMITTED lane-permutation lemmas
  (lemma_trn{1,2}_s{32,64}_reinterpret) — bit-layout facts, mirrors AVX2's
  admitted shuffle lemmas; a wrong permutation is caught by the butterfly post.
- forward layers: clean-context post-helper (lemma_neon_fwd_l{1,2}_post) does the
  16 modadd/modsub OUT of the function WP (one split sub-query saturated when
  inlined — the helper fixed it).
- inverse layers: add/sub BEFORE barrett/montgomery, so butterfly conjuncts ARE
  the barrett(sum)/montgomery(residue) congruences — helper is a clean reveal.
- montgomery/barrett implication-posts: caller asserts the antecedent
  (forall i<8. is_i16b 1664 (zeta lane) / is_i16b 28296 (sum lane)).

## Rust port status (commit)
- arithmetic.rs montgomery: PORTED (rename v->a, implication post). VERIFIED.
- ntt.rs: ALL 6 layer functions PORTED + the two s64 direct-value bridge lemmas
  (lemma_fwd_l2_resultv, lemma_inv_l2_bdiff).  Re-extracted byte-faithful; full
  no-admit module build CLEAN (build 89d3cd7d, 790 queries, 5 min); cargo simd128
  --lib 23/23.  COMMITTED as milestone 2 (agent-mlkem, source-only).
- ntt_multiply: untouched (l_True). The hard one — widening MAC; FOLLOW-UP.

## inverse-2 (s64 raw-sum) — the one that fought back
Unlike layer-1 (barrett supplies the asum bound/congruence via its post) and the
forward layers (helper takes VALUE equations, mod-reasoning stays inside), the
first inv-2 helper took mod-3329 CONGRUENCES — forcing the call site to push the
s64 transpose chain THROUGH the %3329, which saturated Z3 at the full rlimit 400.
Three-part fix:
  1. lemma_trn_s64_bound (ADMIT) — gives `is_i16b b (aa/bb i)` for all i without
     the per-lane transpose->repr_index->input chain (that chain saturates). It's
     a consequence of the already-admitted lane permutation; mirrors AVX2
     lemma_shuffle_preserves_bound.
  2. lemma_vadd_bound (PROVEN, clean context) — `summ=vadd aa bb` + aa/bb bounds
     => `is_i16b (2b) (summ i)` AND `v(summ i)=v(aa i)+v(bb i)`, via forall8.
  3. inv-l2 helper takes asum VALUE equations (not congruences) + the input bound,
     and does the %3329 + output bound INTERNALLY (clean reveal).  forall8 (unfold)
     instead of a symbolic forall is load-bearing for the per-lane bound.

## Perf note (for Phase C / CI)
After the s64 direct-value bridges, the Neon.Ntt module does a cold full
no-admit verify in ~5 min (790 queries, all --split_queries always).  Slowest
real sub-queries: inv-2 = 221 / fwd-1 helper = 33 / fwd-2 helper = 21 / inv-1 =
43 rlimit — all comfortably under the 400 split cap (was inv-2 saturating 400
before lemma_inv_l2_bdiff).  Hot spot remains inv-2's s64 lane bridge but with
healthy margin now.

## Gotcha paid
- `#[cfg_attr(hax, hax_lib::fstar::before(...))]` -> when converting to bare
  `#[hax_lib::fstar::before(...)]` the trailing `))]` must become `)]` (one fewer
  paren). The extra paren gives "unexpected closing delimiter" at extraction.
- ml-dsa parallel session ran a 7GB / 27min z3 query — crippled all shared F*
  builds; used `hax.py`/`make -j2` per user instruction + per-fn --admit_except.

## Next session
1. ntt_multiply functional post (ntt_multiply_butterfly_post; widening MAC) — the
   last l_True in Neon.Ntt; hardest (widening multiply-accumulate).
2. Phase C: op_* wrappers + remove Vector.Neon.{fst,fsti} from ADMIT_MODULES.
