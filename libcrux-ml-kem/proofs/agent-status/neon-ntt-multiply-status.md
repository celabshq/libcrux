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

## Follow-up (not blocking; Neon now matches AVX2 for the hard methods)
Upgrade the panic_free bridges (to_unsigned/cond_subtract via mod_q_eq fold;
{,inv_}ntt_layer via the FE-form Commute lemmas; compress/decompress/serialize
FE-form) to fully-proven — the Neon mirror of portable's C4f work. AVX2 itself
proves to_unsigned/cond_subtract and panic_frees the NTT-layer/compress ones, so
matching AVX2 exactly means proving those two and leaving the rest panic_free.
