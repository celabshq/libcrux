# AVX2 ntt_multiply — session design (2026-06-03)

## Lane semantics derived (verified by hand against Intel semantics)

Inputs `lhs/rhs` = 16 i16 lanes; binomial j (0..7) = lanes (2j, 2j+1); zeta for
binomial j = `+z_{j/2}` (j even) / `−z_{j/2}` (j odd) — matches
`Spec.Utils.ntt_multiply_butterfly_post` ordering `[z0,−z0,z1,−z1,z2,−z2,z3,−z3]`.

- shuffle mask1 per half: out half-lane k ← in σ(k), σ = [0,2,4,6,1,3,5,7]
  (evens in low 64b, odds in high 64b of each half).
- permute4x64 0xD8: i16 lanes [0..3]→[0..3], [4..7]→[8..11], [8..11]→[4..7], [12..15]→[12..15].
- Composition: low 128 = lhs evens (0,2,..,14) in order; high 128 = odds (1,3,..,15).
- cvtepi16_epi32: i32 lane j = sign-extend(get_lane128 x j).
- left = mullo_epi32(evens32): i32 lane j = a_{2j}·b_{2j} (exact, < 2^31).
- right0 = mullo_epi32(odds32): a_{2j+1}·b_{2j+1}; reduce → r_j ≡ ·169 (mod q), |r_j| ≤ 3328,
  i32 lane = sign-extended r_j.
- zetas_vec = set_epi32(−z3,z3,−z2,z2,−z1,z1,−z0,z0): i32 lane j = ±z_{j/2} (− for odd j).
- right = mullo_epi32(right0_reduced, zetas_vec) = r_j·(±z) (exact ≤ 3328·1664).
- products_left = reduce(add_epi32(left, right)): even i16 lane 2j = pl_j,
  v pl_j % q = ((a_{2j}b_{2j} + r_j·(±z))·169) % q → mod-chain to spec form.
- mask2 shuffle: out lane k = rhs lane (k xor 1).
- products_right = reduce(madd_epi16(lhs, swapped)): pr_j; madd i32 lane j =
  a_{2j}·b_{2j+1} + a_{2j+1}·b_{2j} (exact ≤ 2·3328²). Residue = spec odd form directly.
- slli_epi32<16>: i16 out[2j] = 0, out[2j+1] = in[2j].
- blend 0xAA: out[2j] = left[2j] = pl_j; out[2j+1] = right_shifted[2j+1] = pr_j. ✓

## Key findings

- **No i32 accessor in Avx2_extract.fsti** (shared crate — DO NOT touch). Define local
  `lane32 vv j = (v (get_lane vv (2j)) % 65536) + 65536 * v (get_lane vv (2j+1))`
  in Libcrux_ml_kem.Vector.Avx2.Arithmetic (fstar::before(interface,…)).
- **montgomery_reduce_i32s is panic_free with a WRONG/vacuous spec** (i16-view bounds
  3328·2^16 > 2^15 are trivially true; odd-lane residue claim false in general).
  Used ONLY by ntt_multiply (3 sites) → safe to respec:
  requires: ∀j<8. is_intb (3328·2^15) (lane32 vec j)
  ensures:  ∀j<8. let r16 = get_lane result (2j) in is_i16b 3328 r16 /\
            lane32 result j == v r16 /\ v r16 % 3329 == (lane32 vec j · 169) % 3329
  (lane32 result == v r16 is needed because reduce output feeds mullo_epi32.)
  Keep panic_free for now; flag follow-up to prove body (mirror m128i_by_constants +
  srli/srai/sub cross-lane axioms).
- Trait post (`spec::ntt_multiply_post`) = is_i16b_array_opaque 3328 + opaque
  `Spec.Utils.ntt_multiply_butterfly_post` + forall4 opaque `ntt_multiply_branch_post`
  (FE-equation form, traits.rs:579).
- Chunk.fst already has `lemma_base_case_mult_pair_commute` (residues → FE eqs, lanes 2k/2k+1)
  + zetas_4_lane + mont_array_lane + the layer template
  `lemma_ntt_layer_1_step_to_hacspec` (per-lane bridge + forall_intro + lemma_eq_intro).
- N.ntt_multiply_n extraction lambda: group = i/4, zeta = zs[group] or FE-neg, even/odd
  base_case_multiply. Need `lemma_ntt_multiply_n_16_lane` via P.createi_lemma (mirror
  lemma_ntt_layer_n_16_2_lane) + `lemma_mont_fe_neg` (mont lift commutes with FE neg).
- lemma_ntt_multiply_chunk_commutes has NO consumer yet; close as stated.
- Portable leaf post = is_i16b_array 3328 + butterfly_post (+ portable-local forall8).
  AVX2 leaf post := is_i16b_array 3328 + butterfly_post. Wrapper reveals opaques and
  calls 4 new `lemma_ntt_multiply_branch_{0..3}` (Chunk.fst), each =
  reveal branch_post + 2× pair_commute (k=2b zeta=zb; k=2b+1 zeta=neg_i16 zb).

## New axioms needed (admit, accepted pattern, ntt.rs before-block)

lemma_shuffle8_pairs_grouping (mask1, per-lane σ), lemma_shuffle8_adjacent_swap (mask2, xor 1),
lemma_permute4x64_d8, lemma_cvtepi16_epi32 (lane32 = get_lane128), lemma_mullo_epi32
(lane32 wrap @% 2^32), lemma_add_epi32, lemma_madd_epi16, lemma_set_epi32_lanes,
lemma_slli_epi32_16 (i16-lane form), lemma_blend_170.
(castsi256_si128 / extracti128_si256_1 axioms already exist.)

## Mod-q chain helper (even lanes)

lemma: requires r % q == (ab·169) % q
       ensures ((p + r·z)·169) % q == ((p + ab·169·z)·169) % q
(lemma_mod_mul_distr_l/r + lemma_mod_add_distr; int-level, define in ntt.rs before-block.)

## Order

1. Post-merge build gate (fwd-ntt) — in flight.
2. Chunk.fst: branch lemmas + n_16_lane + fe_neg + lane bridge + close chunk lemma
   (replaces assume val; needs only trait interface). → fstar_build Chunk.
3. arithmetic.rs respec + ntt.rs axioms + leaf post (body temporarily admitted) +
   avx2.rs op_ntt_multiply (drop panic_free) → re-extract → build wrapper. Validates shapes.
4. Discharge leaf body (iterate on extracted .fst, backport).
5. Full no-admit builds: Chunk, Avx2.Ntt, Vector.Avx2, Arithmetic + admit-count gate.

## Session learnings (updated mid-session)

1. **Vector-level forall axioms saturate.** First leaf attempt stated intrinsic
   axioms as per-vector foralls with ITE-indexed conclusions; 10/108 sub-queries
   canceled at FULL rlimit 400 (saturation, one churned 22 min before cancel),
   including trivially-true `cast` range obligations (cascade pollution). 98
   passed in 47-211 ms. Z3 auto-patterns on ITE-indexed quantifiers are the
   cascade source.
2. **Ground-literal architecture (v2).** lane32 → opaque_to_smt (atomic terms);
   montgomery_reduce_i32s spec = 8 ground `mont_red_i32_lane` triples (no
   quantifier!); axioms per-lane index-parameterized (quantifier-FREE); helpers
   emit 8/16 ground conjuncts via explicit literal-index axiom calls; the single
   symbolic forall (final is_i16b_array) assembled by 16-way literal dispatch in
   a clean-context helper. Pattern-matching/e-matching dependence: zero.
3. **Chunk lane bridge: symbolic `i/4` saturates too.** lemma_ntt_multiply_n_16_lane
   (createi unfold) passed in 336 ms (!), mont_fe_neg 62 ms, branch lemmas pass
   (split-retry). But the lane bridge's final ensures (connecting branch-post
   lanes 4b.. to lane i via symbolic div/mod) canceled at full rlimit ×4.
   Fix: 16-way literal `match i with` dispatch (§7 per-i match recipe),
   each branch ground (reveal at literal b, partner lanes literal).
4. **2*j-pattern ≠ literal matching.** E-matching cannot unify pattern `2*j`
   with literal index terms (0, 2, 14...) — any quantified fact keyed on
   `2*j`-shaped indexes is unusable at ground call sites and vice versa.
   Ground-conjunct specs sidestep the whole class.
5. mm256 lane semantics + corrected reduce spec cross-validated by bit-exact
   Python sim, 2000 random trials (/tmp/ntt_multiply_sim.py).

## FINAL architecture (v12/v13 — both isolation checks GREEN)

6. **Pure-fn hypothesis threading is the leaf-glue wall.** Large Pure bodies bury
   requires + unit-let posts behind a fuel-guarded unit-refinement chain
   (`Tm_refine_*` iff-quantifiers) that Z3 cannot thread ("incomplete quantifiers"
   at rlimit 7/400). Bool conjuncts (zeta bounds) thread; `Valid`-squashed props
   (is_i16b_array) and distant let-equations don't. → the WHOLE proof lives in
   `lemma_nttmul_main` (Lemma = plain-implication encoding).
7. **BitVec closure terms admit no first-order congruence.** Two textually
   identical `mm256_set_epi8 …` applications (or terms differing only by a
   let-bound vs inline mask) are distinct closure symbols to Z3; assert_norm
   can't see local let definitions either. → thread BOTH shuffle masks through
   the main lemma as FREE parameters (`m`, `ms`) with `m == <inline mask>` as
   requires (hypothesis to propagate, never re-derive); the function let-binds
   both masks (`shuffle_with`, new `swap_with`) and instantiates, so every
   lemma conclusion lands on the function's own spine terms.
8. **Unshadowed locals** (lhs_grouped, odd_products_reduced, products_left_raw,
   products_right_reduced, …) — rename-only Rust change, established precedent.
9. montgomery_reduce_i32s: `requires True` + implication ensures (bounds ==>
   8 ground mont_red_i32_lane triples) — fn callers carry zero obligations;
   the main lemma derives the antecedents internally.
10. fn ensures discharge: bind `result`, single fstar! block = lemma call +
    re-assert of the two ensures conjuncts (WP-final `ApplyTT post result`
    closes from depth-1 facts).

Backported: ntt.rs (library before-block + renames + swap_with + result binding),
arithmetic.rs (lane32 opaque + mont_red_i32_lane + implication ensures).
Remaining: full no-admit gate (in flight) → re-extract → wrapper + Chunk + full builds → commit.
