# Handoff: SHA-3 F* proof sprint

Branch: `sha3-byteform-migration` (tip `d3dc6effe` plus uncommitted scaffolding).

## Read first

- `proofs/sha3-sprint-todo.md` — full audited punch list, every admit + flake with file:line.
- `proofs/squeeze-cascade-profile.md` — qi-profile data on the four cascade queries
  (q224/q231/q263/q280) inside `Generic_keccak.Portable.squeeze`.
- `proofs/agent-prompt-sha3-cascade.md` — superseded by today's session; kept for
  history, but its createi-cascade hypothesis was disproved by the qi profile.

## Branch decision

Stay on `sha3-byteform-migration`. Reverting to `sha3-proofs-focused` (pre-byteform)
costs the −863/+81 spec simplification and the 18 layer-3 wrapper rewires it
already triggered, and buys nothing on the cascade — both styles hit the same
per-byte forall × NL-arith pattern in `Generic_keccak.Portable.squeeze`.

If `Portable.squeeze`'s body proof later proves intractable, fall back by
*locally* swapping just that one function's ensures from byteform `Sponge.squeeze`
to recursive `squeeze_blocks` — much cheaper than reverting the migration.

## Sprint order (from `sha3-sprint-todo.md`)

1. **`lemma_squeeze2_arm64`** — easiest win, ~1 session. The building block
   `lemma_squeeze_one_step_arm64` already exists in
   `EquivImplSpec.Sponge.Arm64.Steps.fst:243`. Replace
   `EquivImplSpec.Sponge.Arm64.Driver.fst:101` `assume val` with a real `let`
   that wires a `fold_range` per-lane loop calling the step lemma.
2. **Add `lemma_squeeze_one_step_avx2`** to `EquivImplSpec.Sponge.Avx2.Steps.fst`
   (mirror Arm64's at N=4 lanes), then close `lemma_squeeze4_avx2`
   (`EquivImplSpec.Sponge.Avx2.API.fst:87`).
3. **`Simd.Arm64.store_block` admit** (`Libcrux_sha3.Simd.Arm64.fst:913`,
   Rust `simd/arm64.rs:255`) — mirror the closed `load_block` / `load_last`
   pattern from commit `abf8b5297`. Per `BRIEF_load_store_block.md`. ~1 session.
4. **`Simd.Avx2.store_block` admit** (`Libcrux_sha3.Simd.Avx2.fst:1930`,
   Rust `simd/avx2.rs:420`) — N=4 trips the AVX2 z3-RPC handle leak; needs the
   opaque chunk helper described in HANDOFF. 1–2 sprints.
5. **`lemma_theta_rho_to_spec` factor** (`EquivImplSpec.Keccakf.Generic.fst:1309`)
   — split into 5 row-helpers. ~1 sprint. Cross-platform.
6. **`Generic_keccak.Portable.squeeze` body** — the cascade-bound admit at
   `Libcrux_sha3.Generic_keccak.Portable.fst:482`. See "Cascade notes" below
   for what was tried and what is still untested.
7. **Regenerate `Sponge.Portable.fst` hints** — 4 hint-replay flakes in
   `lemma_load_last_equals_load_block_on_padded` (qq 33/45/78/83). 1 session.

## Scaffolding kept (uncommitted)

These are verified building blocks left in the working tree for sprint item 6.
Either commit alongside the next attempt or `git checkout` to remove.

- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.SqueezeBytes.fst`
  — two top-level per-byte lemmas (`lemma_squeeze_step_byte_write`,
  `lemma_squeeze_step_byte_tail`) for one-iteration write/tail bytes.
  Verified standalone, hints recorded, ~98 s cold-cache.
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.SqueezeFrame.fst`
  — opaque-to-SMT predicates `prefix_eq` / `tail_eq` with intro / apply /
  to_forall reveal lemmas. Verified standalone in <1 s.
- `crates/algorithms/sha3/proofs/fstar/equivalence/Makefile` — adds those two
  modules to `ROOTS`.

## Cascade notes (saves the next session a day)

The `Portable.squeeze` body cascade is **not** what the prior handoff claimed
(`createi_lemma` SMTPat). qi-profile data (in `squeeze-cascade-profile.md`):

- Top firing quantifier on q224 is `k!61` (Z3-internal Skolem, 1M+ instances)
  followed by `l_quant_interp_84c2c9c541082466b38e1caaa7b29042.1` — that's the
  loop invariant's own per-byte tail forall (170k instances). `createi_lemma`
  is **not in the top 15**.
- The cascade pattern: any proof obligation whose context contains both
  `(v i + 1) * v rate` (NL multiplication) and a per-byte `forall k. ... output[k] ...`
  triggers `l_quant_interp` instantiation cascades.

Things tried in this session that **did not** break the cascade (so don't
repeat them blind):

- Marking `iterate_keccak_f` `[@@"opaque_to_smt"]`. Helped one query (q170);
  did not touch q224/q231/q263/q280; broke the existing step lemma's hints.
  Reverted.
- Rewriting the loop invariant from per-byte `forall` to `Seq.slice ==
  Seq.slice`. Forced changing the step lemma's API. The cascade just relocated
  to the per-byte / slice bridging in the lemma's body. Reverted.
- Lifting `aux_write_step` / `aux_tail_step` to top-level lemmas
  (`SqueezeBytes.fst`). The lemmas verify standalone. Calling them from a
  `forall_intro (fun k -> ... lemma ... )` closure inside the step lemma
  reproduces the cascade because the closure's own VC has the same
  `(i *! rate)` term + per-byte forall in scope. Lifted lemmas are useful as
  building blocks but not sufficient.
- Wrapping the step lemma with opaque `prefix_eq` / `tail_eq` predicates
  (`SqueezeFrame.fst`). Same cascade, one level out: the wrapper's call to
  `lemma_prefix_eq_intro ((v i + 1) * v rate) ...` cascades because the intro
  lemma's precondition is a per-byte forall under the reveal.

Things **not yet** tried that the next session could attempt:

- Wrap NL multiplication in a typed helper, e.g.
  `let mul_off (i: usize) (rate: usize{...}) : usize{v _ == v i * v rate} = i *! rate`
  per smtprofiling Technique 5 (Kruskal `adj_weight` case study). The hope is
  Z3 sees a refined function call instead of a raw machine-int multiplication
  in the cascade-trigger context.
- Replace the per-byte forall in the invariant with a `Seq.equal s1 s2`
  predicate where `s1` and `s2` are `Seq.slice` extractions. `Seq.equal` is
  reflexive on slice projection and may avoid quantifier instantiation that
  the explicit `forall` triggers. Did **not** finish prototyping this today.
- Set `--z3smtopt '(set-option :smt.qi.eager_threshold 100)'` per
  smtprofiling skill — only useful if the cascade is being cut off; here it's
  the opposite (over-firing), so unlikely to help.

## Quick repo state

- All audited admits & flakes: see `proofs/sha3-sprint-todo.md`.
- No `ADMIT_MODULES` / `SLOW_MODULES` in any of the four sha3-related Makefiles.
- End-to-end `Libcrux_sha3.fst` verifies (with USER-2 admit retained on
  `Generic_keccak.Portable.squeeze` body and `--admit_smt_queries true` on
  `lemma_theta_rho_to_spec`).
- Untracked `Libcrux_sha3.Avx2.X4*.fst`, `Generic_keccak.Simd256.fst`,
  `Simd.Avx2.fst*` are parallel-AVX2 work; uncached at audit time and they
  grind through the 700+ split-query `Simd.Avx2.load_block` proof. See
  `agent-status/portable-perf-profile.md` for cold-build timings.

Suggested first move: tackle item 1 (`lemma_squeeze2_arm64`). It's a clean
mechanical wire-up and confirms the byteform step-lemma plumbing is solid
before touching anything cascade-related.
