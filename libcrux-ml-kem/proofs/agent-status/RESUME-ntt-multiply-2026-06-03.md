# RESUME state — AVX2 ntt_multiply session (2026-06-03, pre-shutdown snapshot)

## Where we are (exact)

- **Merged + verified**: `agent/fwd-ntt-mirror-inverse` → `libcrux-ml-kem-proofs`
  (commit `e83675803`); post-merge build green (`Ntt_bridge`, `Libcrux_ml_kem.Ntt`).
- **Chunk.fst** (handwritten, edits in working tree): 4 `lemma_ntt_multiply_branch_*`
  verified; `lemma_ntt_multiply_n_16_lane` (336 ms), `lemma_mont_fe_neg` (62 ms) verified;
  `lemma_ntt_multiply_lane_bridge` REWRITTEN to 16-way literal `match i` dispatch
  (after symbolic `i/4` saturated) — **needs the Chunk rebuild**:
  `fstar_build check/Hacspec_ml_kem.Commute.Chunk.fst` (no admit flags).
  `lemma_ntt_multiply_chunk_commutes` (replaces former assume val) is after the bridge —
  unverified yet (module errored at the bridge in the last run before the rewrite).
- **Leaf** `Libcrux_ml_kem.Vector.Avx2.Ntt.fst` (hand-edited extracted file, v7):
  architecture = per-lane admitted axioms (quantifier-free, masks INLINE — BitVec
  constants get no SMT equations) + clean-context helpers (ground conjuncts) +
  `lemma_nttmul_main` carrying the WHOLE proof (Lemma = plain-implication encoding;
  large Pure bodies bury requires/unit-let posts behind a fuel-guarded unit-refinement
  chain Z3 can't thread — the session's core discovery) + fn body = original let-spine
  with ONE `lemma_nttmul_main` call at the END.
  Status at snapshot: build `3a0ec611` in flight testing the fn VC after making the
  lemma's zeta-vector terms byte-identical (ascription layers) to the extracted spine.
  Previous run had exactly 1 error: fn-ensures congruence (eq2, sub-query 112).
- **Arithmetic.fsti** (hand-edited): `lane32` opaque_to_smt + `mont_red_i32_lane`
  (unfold, ground triple) + `montgomery_reduce_i32s` with `requires True` and an
  implication ensures (bounds ==> 8 ground triples). Rust mirror in arithmetic.rs is
  ONE REVISION BEHIND (still has ground requires + plain ensures — must be updated to
  the implication form before re-extraction!).
- **Rust edits in working tree**: arithmetic.rs (respec — needs the implication-form
  update), avx2.rs + portable.rs op_ntt_multiply (wrappers rewritten, branch-lemma
  derivation). ntt.rs leaf backport NOT done yet (waits for the .fst to green).
- **fsti** Avx2.Ntt.fsti (hand-edited): ntt_multiply val = zetas + 32 ground per-lane
  `is_i16b 3328 (get_lane …)` bool conjuncts (props don't thread; bools do) + ensures
  (is_i16b_array + butterfly_post).

## Next steps (in order)

1. Check build `3a0ec611` (log at ~/.fstar-mcp-logs/builds/). If green → run
   `--admit_except 'Libcrux_ml_kem.Vector.Avx2.Ntt.lemma_nttmul_main'` to verify the
   main lemma's own body; fix anything; then FULL no-admit module build.
2. Rebuild Chunk.fst (full, no admit) — validates dispatched bridge + chunk_commutes.
3. Update arithmetic.rs ensures to the implication form (match the .fsti exactly).
4. Backport the leaf to ntt.rs: before-block = library (axioms+helpers+chain+main
   lemma) verbatim; fn = remove lax, new requires (32 bools)/ensures; body = single
   `hax_lib::fstar!("lemma_nttmul_main $lhs $rhs zeta0 zeta1 zeta2 zeta3")` at the END.
5. Re-extract (`python3 hax.py extract` in libcrux-ml-kem; cargo-hax 0.3.7 active);
   verify regenerated .fst/.fsti match the hand-edited ones (modulo formatting).
6. Wrapper check: build Libcrux_ml_kem.Vector.Avx2.fst (op_ntt_multiply needs the 32
   ground bools from is_i16b_array_opaque reveal — may need a small dispatch helper or
   16 instantiation asserts; wrapper body is tiny so props thread there).
   Same for portable wrapper.
7. Full no-admit builds: Chunk, Avx2.Ntt, Vector.Avx2, Vector.Portable, Arithmetic.
   Admit-count gate (baseline: leaf had 1 lax; now 12 admitted per-lane axioms — the
   accepted intrinsics-axiom pattern + montgomery_reduce_i32s panic_free).
8. Commit (Rust + Chunk.fst + status docs). Update verification_status.md (regen).

## Key learnings codified
- memory: feedback_ground_literal_simd_proofs (+ MEMORY.md index)
- design doc: ntt-multiply-design-2026-06-03.md (§Session learnings)
- The Python sim at /tmp/ntt_multiply_sim.py (cross-validates all axioms; copy is
  reproducible from the design doc if /tmp is cleared).

## Gotchas for the resuming session
- fstar-proxy must be restarted after reboot (skill §3 bootstrap;
  /Users/karthik/fstar-mcp/target/release/fstar-proxy, port 3002).
- /tmp/fp.sh curl wrapper: rebuild it from skill §3 — and note the fixed version
  must put `id` at the TOP LEVEL of the JSON-RPC payload (not inside params).
- Backups: /tmp/AvxNtt-full-body.fst.keep (v2), /tmp/Arith.fsti.bak (pre-edit),
  /tmp/AvxNtt.fst.bak (original extracted) — /tmp may be wiped on reboot; the
  authoritative current state is the working tree itself.
- DON'T re-extract before doing step 3+4 (would clobber the hand-edited .fst/.fsti).
