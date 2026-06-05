# Neon NTT-layer trait bridges: panic_free → PROVEN (all 6)

## Outcome
All six Neon `op_{,inv_}ntt_layer_{1,2,3}_step` trait wrappers in
`libcrux-ml-kem/src/vector/neon.rs` are now **fully proven** (functional posts,
no admit), matching AVX2/portable. Full `check/Libcrux_ml_kem.Vector.Neon.fst`
verifies clean from Rust source: `all_vcs_discharged: true`, 780 queries, 0
errors, ~137 s (build 40852c0e). Previously a single attempt at all 6 ran 71 min
without completing.

## Root cause (NOT what the brief predicted)
The brief's "prime suspect" (the `lemma_repr_index` SMTPat over Neon's
`repr = Seq.append`) was a *contributor* but not the whole story. The real
blockers, found by isolation + `fstar_build_status`:

1. **layer_2 / inv_2 (FE-form)** — their Neon *primitives* expose the opaque
   `*_butterfly_post` (ground conjuncts), so the bridge proof works; it was just
   the `lemma_repr_index` SMTPat polluting the `forall4` FE-form over `repr`.
   Fix: a clean-context helper lemma over **atom params** (see below).

2. **layer_3 / inv_3 (direct-forall)** — STRUCTURAL, not perf. Their Neon
   primitives ensured a raw `forall (i:nat{i<8}). … (Seq.index (repr result) i) …`.
   Because Neon `repr` is a *transparent* `Seq.append`, F* delta-unfolds it
   inside the quantifier, so the auto-trigger becomes `Seq.index (Seq.append…) i`
   and **never matches** any consumer's folded `Seq.index (repr v) j` term. No
   bridge-side trick (seed asserts, `--using_facts_from`, fuel) could instantiate
   it. AVX2's identical layer verifies only because `vec256_as_i16x16` is an
   atomic `val` (no unfold). Confirmed: a single seed `assert (… Seq.index (repr
   result) 0 …)` fails `incomplete quantifiers` at rlimit 5.

## The fix (committed)
**A. Primitives → opaque `butterfly_post` (uniform with layer_1/2, inv_1/2).**
In `src/vector/neon/ntt.rs`, `ntt_layer_3_step` and `inv_ntt_layer_3_step` now
ensure `Spec.Utils.{,inv_}ntt_layer_3_butterfly_post (repr vec) (repr result)
zeta_c` instead of the raw `forall`. Proof changed from `introduce forall …`
to an explicit **ground unroll** (literal-index `lemma_modadd/modsub` /
`b_minus_a` asserts, i=0..7) + `reveal_opaque … butterfly_post` — no quantifier,
so no trigger problem (peak rlimit ~18). This is the same ground shape `forall8`
produces; `butterfly_post` keeps all six NTT primitives uniform.

**B. Bridges → trivial helper calls + clean-context helper lemmas.** A
`#[hax_lib::fstar::before(r#"…"#)]` block on `op_ntt_layer_2_step` defines four
helpers `lemma_neon_{,inv_}ntt_layer_{2,3}_post (vec out: t_Array i16 (mk_usize
16)) …` that take **atom array params** (no `repr`), `requires` the relevant
`butterfly_post`, and `ensures forall4 (… step_branch_post …)`. Each helper does
the 8 `lemma_butterfly_pair_commute` + `forall4 p_layer_N` work over the atom
params — exactly AVX2's structure — verifying clean at peak rlimit ~50–60. The
bridge bodies become: reveal `is_i16b_array_opaque`, call the primitive, call the
helper on `(repr vector) (repr result)`. Options `--z3rlimit 200 --fuel 0 --ifuel
1 --split_queries always --using_facts_from '* -…lemma_repr_index'`. layer_1/inv_1
keep their existing branch-lemma bodies (already proven).

## Files
- `src/vector/neon/ntt.rs` — layer_3/inv_3 primitive ensures + ground-unroll proofs.
- `src/vector/neon.rs` — 4 helper lemmas (before-block) + 4 trivial bridges.
- Splicer used for the bridge backport: `proofs/agent-status/neon-ntt-layer-helper-backport.py`.

## Gotchas hit
- The `neon-ntt-layer-bridge-port.py` brief splicer's AVX2-mirror bodies do NOT
  verify for layer_3/inv_3 (raw-forall primitive) — needed the primitive change.
- Backport splicer line-range bug: `op_ntt_multiply`'s options/requires/ensures
  precede its `#[inline]`, so an `#[inline]`-keyed boundary dropped them (caused
  op_ntt_multiply to fall back to rlimit 80 / l_True). Restored manually.
- ml-dsa session contention (z3 at 4.7 GB) intermittently swap-throttled builds
  to 5–15× wall and OOM-killed one isolation run — not a proof issue.
