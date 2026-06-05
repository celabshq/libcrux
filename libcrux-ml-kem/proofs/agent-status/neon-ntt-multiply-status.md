# Neon ntt_multiply + Phase C — session status (2026-06-04/05)

Branch libcrux-ml-kem-proofs. Base ed949d930.

## BOTH TASKS DONE — committed (source-only, NOT pushed)
- `8f753ce7e` agent-mlkem: Neon ntt_multiply functional post (widening MAC)
- `ecd58e184` agent-mlkem: Phase C — Neon trait layer out of ADMIT_MODULES

`make all` (whole libcrux-ml-kem F* tree): exit 0, 0 errors, no failed modules.
cargo test --features simd128 --lib: 23/23.

## Task 1 — ntt_multiply functional post (the hard one)
`ntt_multiply` (src/vector/neon/ntt.rs) now carries
`Spec.Utils.ntt_multiply_butterfly_post` (was l_True) — the last non-trivial
post in Vector.Neon.Ntt. Full no-admit build of Libcrux_ml_kem.Vector.Neon.Ntt:
1591 queries, 0 admits/errors, ntt_multiply 92 sub-queries, peak rlimit 15.9/400.

### Architecture (mirrors AVX2 ground-literal v12; adapted to the Neon
### vmull/vmlal widening-MAC + s16/s32 transpose + vqtbl1q chain)
All 8 modelled intrinsics on the path (e_vmull_s16/_high, e_vmlal_s16/_high,
e_vtrn1/2q_s16, e_vqtbl1q_u8, reinterprets) already carry functional posts, so
**no new assumed intrinsic encoding was introduced** (no differential test
needed). The full data flow + every admitted permutation's lane indices were
cross-validated against a 25000-trial bit-exact Python sim BEFORE any F*
iteration (/tmp/ntt_mul_sim.py + /tmp/ntt_mul_intermediate.py).

- HONEST (proven): montgomery_multiply congruence (a1b1); montgomery_reduce ->
  mont_red_i32 congruence (`lemma_nttmul_redcong` + `Spec.Utils.lemma_mont_red_i32`
  — the reduce's opaque bit-layout post threaded in as `requires`, matched to
  mont_red_i32 by the i16x2_as_i32 round-trip); `lemma_nttmul_even_chain`
  (mod-3329 a0*b0 + a1b1*zeta rewrite, copied from AVX2); the per-pair core
  `lemma_nttmul_fstsnd` (clean-context, inner ev/od helpers); the butterfly_post
  assembly `lemma_nttmul_assemble` (reveal + per-i-match bound dispatch).
- ADMITTED plumbing (pure permutation / widening-product bit-layout, validated
  by the sim; AVX2 admitted-shuffle + Neon s64 direct-value-bridge precedent):
  `lemma_nttmul_in` (trn input prep + zeta load), `lemma_nttmul_montval_{fst,snd}`
  (widening vmull/vmlal + reinterpret + trn into the (lo16,hi16) montgomery-reduce
  halves, cast/i16x2_as_i32 round-trip form), `lemma_nttmul_out` (final
  trn / trn_s32 / vqtbl1q_u8 output assembly).

Lane plan (validated): a0=trn1q_s16(low,high), a1=trn2q_s16(low,high); lane j of
a0/a1/b0/b1/zeta operates on pair sigma[j], sigma=[0;4;1;5;2;6;3;7]; fst/snd
lane k holds the even/odd output for pair p[k], p=[0;2;4;6;1;3;5;7]=sigma∘sigma,
m_k=sigma[k]. The one gotcha paid: needed `assert_norm (pow2 31 == 2147483648)`
for the lemma_mul_i16b antecedent (b1*b2 < pow2 31).

## Task 2 — Phase C: Vector.Neon out of ADMIT_MODULES
`impl Operations for SIMD128Vector` (src/vector/neon.rs) wired Track-B style
(one-line dispatch + `op_*` wrappers carrying the trait pre/post). Makefile
ADMIT_MODULES now empty. Vector.Neon.{fst,fsti} verify (455 queries, 0 errors).

Per-method status ladder (replaces the previous whole-module admit where
NOTHING was verified):
- FULLY VERIFIED: ZERO, from/to_i16_array, from/to_bytes, add, sub,
  multiply_by_constant, barrett_reduce, montgomery_multiply_by_constant,
  serialize_5/11, deserialize_1/4/5/10/11 (free-fn post already == trait post).
- op_ntt_multiply FULLY PROVEN (butterfly_post + 4 Commute.Chunk branch lemmas).
- panic_free op_*: cond_subtract_3329, to_unsigned_representative, compress_1,
  compress, decompress_1, decompress_ciphertext_coefficient,
  {,inv_}ntt_layer_{1,2,3}_step, serialize_1/4/10/12, deserialize_12. (Body
  panic-checked + primitive precondition discharged via an is_i16b_array_opaque
  reveal; the strengthened FE-form / mod_q_eq trait post is admitted at this
  layer — the same rung AVX2 uses for its NTT-layer/compress/decompress wrappers.)
- lax: op_rej_sample + the free rej_sample (portable-fallback loop; chunks(3)
  indexing not panic-free for arbitrary input; post out of scope).

### Gotchas paid
- `repr` is NOT in scope in the Vector.Neon module — fully-qualify
  `Libcrux_ml_kem.Vector.Neon.Vector_type.repr` in op_ntt_multiply's proof body
  (or use impl.f_repr); impl method pre/post use `impl.f_repr` (resolves).
- `use super::traits::{spec, Repr}` and `use hax_lib::prop::ToProp` must be
  `#[cfg(hax)]`-gated (spec mod is hax-only) or cargo non-hax build breaks (E0432).
- Un-admitting the module surfaced the free `rej_sample` (was hidden by the
  whole-module admit) — it failed panic-freedom; marked lax.

## Bridge upgrades (2026-06-05 follow-up session)
Upgraded two panic_free bridges to FULLY PROVEN (mirrors avx2.rs exactly):
`op_cond_subtract_3329` + `op_to_unsigned_representative` — the Neon primitive
gives `v y % 3329 == v x % 3329` per lane; fold into the opaque `mod_q_eq` form
via `Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro` + `Classical.forall_intro` (no
map_array step needed — Neon's post is already the %3329 form). Verified clean
(build 61a5962f, 0 errors, real Query-stats; cargo simd128 23/23). Committed.

### NTT-layer bridges — ATTEMPTED, reverted to panic_free (DEFECT, not cold-slow)
Ported AVX2's FE-form proofs for `{,inv_}ntt_layer_{1,2,3}_step` verbatim
(`repr`-adapted): reveal the in/out `is_i16b_array_opaque` bounds + the
`butterfly_post`, call `Commute.Chunk.lemma_butterfly_pair_commute` (layer-2/3,
inv-2/3) or `lemma_{,inv_}ntt_layer_1_step_branch_{0..3}` (layer-1, inv-1), then
the `forall4 p_layer_N` FE-form assertion. A cold full-module build (6 proofs at
rlimit 600/400, fuel 1) ran **71 min on one fstar.exe without completing**.
Reverted the 6 to panic_free; proof text preserved in /tmp/neon_phaseB_ntt.py.

**CORRECTED diagnosis (do NOT chase hint-seeding):** AVX2's identical
`op_ntt_layer_2/3` + `op_inv_ntt_layer_2/3` verify in **5–9 s each** cold
(fstar-perf-top20.md), and `.fstar-cache/hints/` is **git-ignored / 0 tracked**
— AVX2 does NOT rely on committed hints; it verifies cold in seconds. So the
71-min non-completion is a **real saturation defect in the Neon port**, not
cold-slowness. Prime suspect: Neon `repr x = Seq.append (vec128_as_i16x8 x.f_low)
(vec128_as_i16x8 x.f_high)` is a STRUCTURED term, whereas AVX2's
`vec256_as_i16x16 x.f_elements` is an ATOMIC opaque array. Feeding `repr vector`
(append) into the `forall4` FE-form + 8 `butterfly_pair_commute` calls makes
`Seq.index (repr ...)` terms that the `lemma_repr_index` SMTPat re-expands
per-index, exploding the context. (Note: the Neon ntt.rs *primitives* already
prove butterfly_post over repr fine — so repr itself is provable; the blowup is
the bridge layer's FE-form × append interaction.)

**Fix path (focused F* debug, NOT hint-seeding):**
1. Isolate ONE op_ at a time via `--admit_except Libcrux_ml_kem.Vector.Neon.op_ntt_layer_N_step`
   (rm the tainted .checked between runs); measure each — find which of the 6
   saturate vs pass. (op_ntt_layer_1 / op_inv_ntt_layer_1 are branch-lemma based
   at rlimit 400 — likely the lighter ones; layer-2/3 FE-form at 600 the heavy.)
2. On a saturating one, `smtprofiling` (qi.profile) BEFORE concluding — confirm
   whether it's the `lemma_repr_index` SMTPat re-expansion (likely) or a Commute
   lemma trigger cascade.
3. Likely fix: snapshot repr into an atomic 16-array the bridge treats opaquely
   (e.g. bind `let v16 = to_i16_array vector` — its ensures gives `== repr v`, an
   atomic `t_Array i16 16` rather than a live append) and run the Commute lemmas
   over `v16`; or add a small clean-context Neon helper that converts
   butterfly_post(repr) → the per-lane Seq.index facts the Commute lemmas want,
   so the append never enters the FE-form forall4 context.

## Remaining panic_free / lax (after the upgrade)
- panic_free: compress, decompress_1, decompress_ciphertext_coefficient (AVX2
  ALSO panic_frees these); compress_1 (AVX2 proves — Neon-specific bridge, TODO);
  {,inv_}ntt_layer_{1,2,3}_step (cold-saturation, see above);
  serialize_1/4/10/12, deserialize_12 (underlying Neon free-fns are l_True — no
  post to bridge from; needs the underlying Neon serialize proofs first).
- lax: rej_sample (portable-fallback loop).
